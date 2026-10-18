use std::path::PathBuf;
use std::sync::Arc;

use async_fs;
use bytes::Bytes;
use futures::io::AsyncReadExt as _;
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::r#async::SpawnedFutureHandle;
use warpui::elements::{
    Border, CacheOption, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult,
    Element, EventHandler, Flex, Image, MainAxisAlignment, MainAxisSize, ParentElement, Shrinkable,
    Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;
use warpui::{AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle};

use crate::appearance::Appearance;
use crate::editor::{EditorView, Event as EditorEvent, SingleLineEditorOptions};
use crate::local_workspace::model::{OrganizationId, ProjectId};

static IMAGE_CROP_LOCK: std::sync::OnceLock<futures::lock::Mutex<()>> = std::sync::OnceLock::new();

fn image_crop_lock() -> &'static futures::lock::Mutex<()> {
    IMAGE_CROP_LOCK.get_or_init(|| futures::lock::Mutex::new(()))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CreateDialogKind {
    CreateOrganization,
    EditOrganization(OrganizationId),
    RemoveOrganization(OrganizationId),
    CreateProject(OrganizationId),
    EditProject(ProjectId),
    RemoveProject(ProjectId),
}

pub(crate) enum CreateDialogEvent {
    Close,
    PickProjectPath,
    PickOrganizationImage,
    Submit {
        kind: CreateDialogKind,
        name: String,
        path: Option<PathBuf>,
        accent_color: String,
        image_data: Option<Vec<u8>>,
        project_validation_generation: u64,
    },
}

fn submit_blocking_status(
    image_job_active: bool,
    image_path_pending: bool,
    crop_preview_pending: bool,
    project_validation_pending: bool,
    image_selection_error: bool,
) -> Option<&'static str> {
    if image_job_active || image_path_pending || crop_preview_pending {
        Some("Aguarde o processamento da imagem terminar.")
    } else if project_validation_pending {
        Some("Aguarde a verificacao da pasta terminar.")
    } else if image_selection_error {
        Some("Remova a imagem com erro ou escolha outra antes de salvar.")
    } else {
        None
    }
}

fn next_image_generation(generation: u64) -> u64 {
    generation.wrapping_add(1)
}

pub(crate) fn validate_project_path_metadata(
    metadata_result: Result<bool, std::io::ErrorKind>,
) -> Result<(), &'static str> {
    match metadata_result {
        Ok(true) => Ok(()),
        Ok(false) => Err("O caminho selecionado nao e uma pasta."),
        Err(std::io::ErrorKind::NotFound) => Err("A pasta selecionada nao foi encontrada."),
        Err(std::io::ErrorKind::NotADirectory) => Err("O caminho selecionado nao e uma pasta."),
        Err(std::io::ErrorKind::PermissionDenied) => Err("Sem acesso a pasta selecionada."),
        Err(_) => Err("Nao foi possivel verificar a pasta selecionada."),
    }
}

#[derive(Clone, Debug)]
pub(crate) enum CreateDialogAction {
    Submit,
    Cancel,
    PickProjectPath,
    PickOrganizationImage,
    SelectAccent(String),
    ZoomIn,
    ZoomOut,
    PanCropTo(f32, f32),
    EndCropPan,
    DiscardImage,
}

pub(crate) struct CreateDialog {
    kind: CreateDialogKind,
    name_editor: ViewHandle<EditorView>,
    project_path: Option<PathBuf>,
    accent_color: String,
    image_data: Option<Vec<u8>>,
    source_image_data: Option<Vec<u8>>,
    source_dimensions: Option<(u32, u32)>,
    crop_zoom: f32,
    crop_viewport: super::image_cropper::CropViewport,
    crop_drag_position: Option<(f32, f32)>,
    status: Option<String>,
    error: Option<String>,
    active: bool,
    image_generation: u64,
    image_job: Option<SpawnedFutureHandle>,
    image_job_is_load: bool,
    pending_image_path: Option<PathBuf>,
    crop_preview_pending: bool,
    project_validation_generation: u64,
    project_validation_pending: bool,
    image_selection_error: bool,
}

impl CreateDialog {
    pub(crate) fn new(
        kind: CreateDialogKind,
        name: Option<String>,
        project_path: Option<PathBuf>,
        accent_color: Option<String>,
        image_data: Option<Vec<u8>>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let name_editor = ctx.add_typed_action_view(|ctx| {
            let mut editor = EditorView::single_line(SingleLineEditorOptions::default(), ctx);
            editor.set_placeholder_text("Nome", ctx);
            editor
        });
        name_editor.update(ctx, |editor, ctx| {
            if let Some(name) = name {
                editor.set_buffer_text_ignoring_undo(&name, ctx);
            }
        });
        ctx.subscribe_to_view(&name_editor, |me, _, event, ctx| match event {
            EditorEvent::Enter => me.try_submit(ctx),
            EditorEvent::Escape => ctx.emit(CreateDialogEvent::Close),
            EditorEvent::Edited(_) => {
                me.error = None;
                ctx.notify();
            }
            _ => {}
        });
        Self {
            kind,
            name_editor,
            project_path,
            accent_color: accent_color.unwrap_or_else(|| "blue".to_owned()),
            source_image_data: image_data.clone(),
            source_dimensions: image_data.as_ref().map(|_| (128, 128)),
            image_data,
            crop_zoom: 1.,
            crop_viewport: super::image_cropper::CropViewport {
                width: 128.,
                height: 128.,
                zoom: 1.,
                offset_x: 0.,
                offset_y: 0.,
            },
            crop_drag_position: None,
            status: None,
            error: None,
            active: true,
            image_generation: 0,
            image_job: None,
            image_job_is_load: false,
            pending_image_path: None,
            crop_preview_pending: false,
            project_validation_generation: 0,
            project_validation_pending: false,
            image_selection_error: false,
        }
    }

    pub(crate) fn set_project_path(&mut self, path: PathBuf, ctx: &mut ViewContext<Self>) {
        if !self.active {
            return;
        }
        if self
            .name_editor
            .as_ref(ctx)
            .buffer_text(ctx)
            .trim()
            .is_empty()
            && let Some(name) = path.file_name().and_then(|name| name.to_str())
        {
            self.name_editor.update(ctx, |editor, ctx| {
                editor.set_buffer_text_ignoring_undo(name, ctx)
            });
        }
        self.project_path = Some(path);
        self.project_validation_generation = self.project_validation_generation.wrapping_add(1);
        self.project_validation_pending = false;
        self.error = None;
        self.status = None;
        ctx.notify();
    }

    pub(crate) fn begin_image_load(&mut self, path: PathBuf, ctx: &mut ViewContext<Self>) {
        if !self.active {
            return;
        }
        self.image_generation = next_image_generation(self.image_generation);
        self.pending_image_path = None;
        self.crop_preview_pending = false;
        let generation = self.image_generation;
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "svg") {
            self.set_image_error("Use PNG, JPG ou SVG.".to_owned(), ctx);
            return;
        }
        if self.image_job.is_some() {
            self.pending_image_path = Some(path);
            self.crop_preview_pending = false;
            self.status = Some("Aguardando processamento atual...".to_owned());
            ctx.notify();
            return;
        }
        self.status = Some("Carregando imagem…".to_owned());
        self.error = None;
        ctx.notify();
        self.image_job_is_load = true;
        self.image_job = Some(ctx.spawn(
            async move {
                let metadata = async_fs::metadata(&path).await.map_err(|_| "read")?;
                if !metadata.is_file() {
                    return Err("read");
                }
                if metadata.len() > super::image_cropper::MAX_UPLOAD_BYTES as u64 {
                    return Err("size");
                }
                let mut file = async_fs::File::open(path).await.map_err(|_| "read")?;
                let mut source = Vec::new();
                (&mut file)
                    .take((super::image_cropper::MAX_UPLOAD_BYTES + 1) as u64)
                    .read_to_end(&mut source)
                    .await
                    .map_err(|_| "read")?;
                if source.len() > super::image_cropper::MAX_UPLOAD_BYTES {
                    return Err("size");
                }
                let _guard = image_crop_lock().lock().await;
                let crop = super::image_cropper::crop_image_to_png_with_view_and_dimensions(
                    &source, 1., 0., 0.,
                )
                .map_err(|error| match error {
                    super::image_cropper::ImageCropError::TooLarge => "size",
                    super::image_cropper::ImageCropError::UnsupportedFormat => "format",
                    super::image_cropper::ImageCropError::InvalidImage
                    | super::image_cropper::ImageCropError::DimensionsTooLarge => "process",
                })?;
                Ok::<_, &'static str>((source, crop))
            },
            move |dialog, result, ctx| {
                dialog.image_job = None;
                dialog.image_job_is_load = false;
                if !dialog.active {
                    return;
                }
                if let Some(path) = dialog.pending_image_path.take() {
                    dialog.begin_image_load(path, ctx);
                    return;
                }
                if dialog.image_generation != generation {
                    if dialog.crop_preview_pending {
                        dialog.crop_preview_pending = false;
                        dialog.request_crop_preview(ctx);
                    }
                    return;
                }
                match result {
                    Ok((source, crop)) => dialog.set_image_data(source, crop, ctx),
                    Err("size") => {
                        dialog.set_image_error("A imagem deve ter até 5 MB.".to_owned(), ctx)
                    }
                    Err("format") => dialog
                        .set_image_error("O formato da imagem não é compatível.".to_owned(), ctx),
                    Err("read") => {
                        dialog.set_image_error("Não foi possível ler a imagem.".to_owned(), ctx)
                    }
                    Err(_) => dialog
                        .set_image_error("Não foi possível processar a imagem.".to_owned(), ctx),
                }
            },
        ));
    }

    pub(crate) fn set_image_data(
        &mut self,
        source_bytes: Vec<u8>,
        cropped: super::image_cropper::CroppedImage,
        ctx: &mut ViewContext<Self>,
    ) {
        self.source_image_data = Some(source_bytes);
        self.source_dimensions = Some((cropped.source_width, cropped.source_height));
        self.image_data = Some(cropped.png);
        self.image_selection_error = false;
        self.crop_zoom = 1.;
        self.crop_viewport = super::image_cropper::CropViewport {
            width: 128.,
            height: 128.,
            zoom: 1.,
            offset_x: 0.,
            offset_y: 0.,
        };
        self.error = None;
        self.status = None;
        ctx.notify();
    }

    pub(crate) fn deactivate(&mut self, ctx: &mut ViewContext<Self>) {
        self.active = false;
        self.image_generation = self.image_generation.wrapping_add(1);
        if let Some(job) = self.image_job.take() {
            job.abort();
        }
        self.image_job_is_load = false;
        self.pending_image_path = None;
        self.crop_preview_pending = false;
        self.project_validation_generation = self.project_validation_generation.wrapping_add(1);
        self.project_validation_pending = false;
        ctx.notify();
    }

    pub(crate) fn finish_project_path_validation(
        &mut self,
        generation: u64,
        result: Result<(), String>,
        ctx: &mut ViewContext<Self>,
    ) -> bool {
        if !self.active
            || !self.project_validation_pending
            || self.project_validation_generation != generation
        {
            return false;
        }
        self.project_validation_pending = false;
        match result {
            Ok(()) => {
                self.status = None;
                self.error = None;
                ctx.notify();
                true
            }
            Err(error) => {
                self.set_error(error, ctx);
                false
            }
        }
    }

    pub(crate) fn set_error(&mut self, message: String, ctx: &mut ViewContext<Self>) {
        if !self.active {
            return;
        }
        self.error = Some(message);
        self.status = None;
        ctx.notify();
    }

    fn set_image_error(&mut self, message: String, ctx: &mut ViewContext<Self>) {
        if !self.active {
            return;
        }
        self.image_selection_error = true;
        self.set_error(message, ctx);
    }

    fn discard_image(&mut self, ctx: &mut ViewContext<Self>) {
        self.image_generation = self.image_generation.wrapping_add(1);
        self.image_data = None;
        self.source_image_data = None;
        self.source_dimensions = None;
        self.crop_zoom = 1.;
        self.image_selection_error = false;
        self.error = None;
        self.status = None;
        ctx.notify();
    }

    pub(crate) fn set_status(&mut self, message: String, ctx: &mut ViewContext<Self>) {
        if !self.active {
            return;
        }
        self.status = Some(message);
        self.error = None;
        ctx.notify();
    }

    fn try_submit(&mut self, ctx: &mut ViewContext<Self>) {
        if let Some(status) = submit_blocking_status(
            self.image_job.is_some(),
            self.pending_image_path.is_some(),
            self.crop_preview_pending,
            self.project_validation_pending,
            self.image_selection_error,
        ) {
            self.status = Some(status.to_owned());
            ctx.notify();
            return;
        }
        let name = self
            .name_editor
            .as_ref(ctx)
            .buffer_text(ctx)
            .trim()
            .to_owned();
        if !matches!(
            &self.kind,
            CreateDialogKind::RemoveOrganization(_) | CreateDialogKind::RemoveProject(_)
        ) && name.is_empty()
        {
            self.error = Some("Informe um nome.".to_owned());
            ctx.notify();
            return;
        }
        if matches!(&self.kind, CreateDialogKind::CreateProject(_)) && self.project_path.is_none() {
            self.error = Some("Selecione uma pasta para o projeto.".to_owned());
            ctx.notify();
            return;
        }
        if matches!(&self.kind, CreateDialogKind::CreateProject(_)) {
            self.project_validation_generation = self.project_validation_generation.wrapping_add(1);
            self.project_validation_pending = true;
            self.status = Some("Verificando pasta...".to_owned());
            self.error = None;
            ctx.notify();
        }
        ctx.emit(CreateDialogEvent::Submit {
            kind: self.kind.clone(),
            name,
            path: self.project_path.clone(),
            accent_color: self.accent_color.clone(),
            image_data: self.image_data.clone(),
            project_validation_generation: self.project_validation_generation,
        });
    }

    fn button(
        label: &str,
        action: CreateDialogAction,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let element = Container::new(
            Text::new_inline(label.to_owned(), appearance.ui_font_family(), 12.)
                .with_color(theme.font_color(theme.surface_2()).into())
                .finish(),
        )
        .with_background(theme.surface_2())
        .with_horizontal_padding(12.)
        .with_vertical_padding(7.)
        .with_corner_radius(warpui::elements::CornerRadius::with_all(
            warpui::elements::Radius::Pixels(4.),
        ))
        .finish();
        EventHandler::new(element)
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    }
}

impl Entity for CreateDialog {
    type Event = CreateDialogEvent;
}

impl View for CreateDialog {
    fn ui_name() -> &'static str {
        "LocalWorkspaceDialog"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let confirmation = matches!(
            &self.kind,
            CreateDialogKind::RemoveOrganization(_) | CreateDialogKind::RemoveProject(_)
        );
        let title = match &self.kind {
            CreateDialogKind::CreateOrganization => "Nova organização",
            CreateDialogKind::EditOrganization(_) => "Editar organização",
            CreateDialogKind::RemoveOrganization(_) => "Remover organização",
            CreateDialogKind::CreateProject(_) => "Novo projeto",
            CreateDialogKind::EditProject(_) => "Editar projeto",
            CreateDialogKind::RemoveProject(_) => "Remover projeto",
        };
        let mut body = Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(12.);
        body.add_child(
            Text::new_inline(title, appearance.ui_font_family(), 18.)
                .with_color(theme.font_color(theme.background()).into())
                .with_style(Properties::default().weight(Weight::Bold))
                .finish(),
        );
        if confirmation {
            let message = match &self.kind {
                CreateDialogKind::RemoveOrganization(_) => {
                    "Os projetos, pastas e sessões continuarão no disco. Apenas a organização será removida deste terminal."
                }
                CreateDialogKind::RemoveProject(_) => {
                    "A pasta continuará no disco. Os terminais ficarão acessíveis em Início, sem projeto."
                }
                _ => "",
            };
            body.add_child(
                Text::new_inline(message.to_owned(), appearance.ui_font_family(), 12.)
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
            );
        } else {
            body.add_child(
                TextInput::new(
                    self.name_editor.clone(),
                    UiComponentStyles::default()
                        .set_border_radius(warpui::elements::CornerRadius::with_all(
                            warpui::elements::Radius::Pixels(4.),
                        ))
                        .set_height(36.),
                )
                .build()
                .finish(),
            );
        }
        if matches!(
            &self.kind,
            CreateDialogKind::CreateOrganization | CreateDialogKind::EditOrganization(_)
        ) {
            let mut colors = Flex::row().with_spacing(8.);
            for color in ["blue", "green", "magenta", "yellow", "cyan", "red"] {
                let action = CreateDialogAction::SelectAccent(color.to_owned());
                let selected = self.accent_color == color;
                let swatch = ConstrainedBox::new(
                    Container::new(
                        Text::new_inline(" ".to_owned(), appearance.ui_font_family(), 1.).finish(),
                    )
                    .with_background(organization_accent(color, theme.accent()))
                    .with_border(if selected {
                        Border::all(2.).with_border_fill(theme.font_color(theme.background()))
                    } else {
                        Border::all(1.).with_border_fill(theme.outline())
                    })
                    .with_corner_radius(warpui::elements::CornerRadius::with_all(
                        warpui::elements::Radius::Pixels(999.),
                    ))
                    .finish(),
                )
                .with_width(20.)
                .with_height(20.)
                .finish();
                colors.add_child(
                    EventHandler::new(swatch)
                        .on_left_mouse_up(move |ctx, _, _| {
                            ctx.dispatch_typed_action(action.clone());
                            DispatchEventResult::StopPropagation
                        })
                        .finish(),
                );
            }
            body.add_child(colors.finish());
            body.add_child(Self::button(
                "Escolher imagem...",
                CreateDialogAction::PickOrganizationImage,
                appearance,
            ));
            if self.image_selection_error {
                body.add_child(Self::button(
                    "Remover imagem",
                    CreateDialogAction::DiscardImage,
                    appearance,
                ));
            }
            if let Some(bytes) = &self.image_data {
                let mut crop_preview = Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(12.);
                crop_preview.add_child(
                    ConstrainedBox::new(
                        EventHandler::new(
                            Image::new(image_preview_source(bytes.clone()), CacheOption::BySize)
                                .finish(),
                        )
                        .on_mouse_dragged(|ctx, _, position| {
                            ctx.dispatch_typed_action(CreateDialogAction::PanCropTo(
                                position.x(),
                                position.y(),
                            ));
                            DispatchEventResult::StopPropagation
                        })
                        .on_left_mouse_up(|ctx, _, _| {
                            ctx.dispatch_typed_action(CreateDialogAction::EndCropPan);
                            DispatchEventResult::StopPropagation
                        })
                        .finish(),
                    )
                    .with_width(128.)
                    .with_height(128.)
                    .finish(),
                );
                let mut zoom_controls = Flex::column()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(6.);
                zoom_controls.add_child(Self::button("−", CreateDialogAction::ZoomOut, appearance));
                zoom_controls.add_child(
                    Text::new_inline(
                        format!("{:.2}×", self.crop_zoom),
                        appearance.ui_font_family(),
                        11.,
                    )
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
                );
                zoom_controls.add_child(Self::button("+", CreateDialogAction::ZoomIn, appearance));
                crop_preview.add_child(zoom_controls.finish());
                body.add_child(crop_preview.finish());
            }
        }
        if matches!(&self.kind, CreateDialogKind::CreateProject(_)) {
            let path = self
                .project_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "Nenhuma pasta selecionada".to_owned());
            body.add_child(
                Text::new_inline(path, appearance.ui_font_family(), 11.)
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
            );
            body.add_child(Self::button(
                "Escolher pasta...",
                CreateDialogAction::PickProjectPath,
                appearance,
            ));
        }
        if let Some(error) = &self.error {
            body.add_child(
                Text::new_inline(error.clone(), appearance.ui_font_family(), 12.)
                    .with_color(theme.ui_error_color().into())
                    .finish(),
            );
        }
        if let Some(status) = &self.status {
            body.add_child(
                Text::new_inline(status.clone(), appearance.ui_font_family(), 11.)
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
            );
        }
        let mut footer = Flex::row()
            .with_main_axis_alignment(MainAxisAlignment::End)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(8.);
        footer.add_child(Self::button(
            "Cancelar",
            CreateDialogAction::Cancel,
            appearance,
        ));
        footer.add_child(Self::button(
            if confirmation { "Remover" } else { "Salvar" },
            CreateDialogAction::Submit,
            appearance,
        ));
        body.add_child(footer.finish());
        let body = Container::new(Shrinkable::new(1., body.finish()).finish())
            .with_uniform_padding(12.)
            .finish();
        Container::new(body).finish()
    }
}

impl TypedActionView for CreateDialog {
    type Action = CreateDialogAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CreateDialogAction::Submit => self.try_submit(ctx),
            CreateDialogAction::Cancel => ctx.emit(CreateDialogEvent::Close),
            CreateDialogAction::PickProjectPath => ctx.emit(CreateDialogEvent::PickProjectPath),
            CreateDialogAction::PickOrganizationImage => {
                ctx.emit(CreateDialogEvent::PickOrganizationImage)
            }
            CreateDialogAction::SelectAccent(color) => {
                self.accent_color = color.clone();
                ctx.notify();
            }
            CreateDialogAction::ZoomIn | CreateDialogAction::ZoomOut => {
                let zoom = if matches!(action, CreateDialogAction::ZoomIn) {
                    (self.crop_zoom + 0.25).min(4.)
                } else {
                    (self.crop_zoom - 0.25).max(1.)
                };
                if self.source_image_data.is_some() {
                    self.crop_viewport = self.crop_viewport.zoomed(zoom);
                    self.crop_zoom = zoom;
                    self.request_crop_preview(ctx);
                    ctx.notify();
                }
            }
            CreateDialogAction::PanCropTo(x, y) => {
                if let Some((last_x, last_y)) = self.crop_drag_position {
                    let (width, height) = self.source_dimensions.unwrap_or((128, 128));
                    self.crop_viewport = self.crop_viewport.panned(
                        x - last_x,
                        y - last_y,
                        width as f32,
                        height as f32,
                    );
                    if self.source_image_data.is_some() {
                        self.status = Some("Solte para atualizar o recorte.".to_owned());
                    }
                }
                self.crop_drag_position = Some((*x, *y));
                ctx.notify();
            }
            CreateDialogAction::EndCropPan => {
                self.crop_drag_position = None;
                if self.source_image_data.is_some() {
                    self.request_crop_preview(ctx);
                }
            }
            CreateDialogAction::DiscardImage => self.discard_image(ctx),
        }
    }
}

impl CreateDialog {
    fn request_crop_preview(&mut self, ctx: &mut ViewContext<Self>) {
        let Some(source) = self.source_image_data.clone() else {
            return;
        };
        let Some((width, height)) = self.source_dimensions else {
            return;
        };
        let zoom = self.crop_zoom;
        let pan_x = self.crop_viewport.offset_x;
        let pan_y = self.crop_viewport.offset_y;
        if self.image_job.is_some() && self.image_job_is_load {
            return;
        }
        self.image_generation = self.image_generation.wrapping_add(1);
        let generation = self.image_generation;
        if self.image_job.is_some() {
            self.crop_preview_pending = true;
            return;
        }
        self.crop_preview_pending = false;
        self.status = Some("Atualizando recorte...".to_owned());
        self.error = None;
        self.image_job_is_load = false;
        self.image_job = Some(ctx.spawn(
            async move {
                let _guard = image_crop_lock().lock().await;
                super::image_cropper::crop_image_to_png_with_view_and_dimensions(
                    &source, zoom, pan_x, pan_y,
                )
                .map(|crop| (crop.png, width, height))
            },
            move |dialog, result, ctx| {
                dialog.image_job = None;
                dialog.image_job_is_load = false;
                if !dialog.active {
                    return;
                }
                if let Some(path) = dialog.pending_image_path.take() {
                    dialog.begin_image_load(path, ctx);
                    return;
                }
                if dialog.image_generation != generation {
                    if dialog.crop_preview_pending {
                        dialog.crop_preview_pending = false;
                        dialog.request_crop_preview(ctx);
                    }
                    return;
                }
                match result {
                    Ok((png, width, height)) => {
                        dialog.image_data = Some(png);
                        dialog.source_dimensions = Some((width, height));
                        dialog.status = None;
                        dialog.error = None;
                        ctx.notify();
                    }
                    Err(_) => {
                        dialog.set_error("Nao foi possivel atualizar o recorte.".to_owned(), ctx)
                    }
                }
            },
        ));
    }
}

fn organization_accent(
    name: &str,
    fallback: warp_core::ui::theme::Fill,
) -> warp_core::ui::theme::Fill {
    let hex = match name {
        "green" => "#20A874",
        "magenta" => "#CF5B9C",
        "yellow" => "#D8A82E",
        "cyan" => "#29A5B3",
        "red" => "#D45D5D",
        _ => "#4E83D3",
    };
    warp_core::ui::color::hex_color::coloru_from_hex_string(hex)
        .map(warp_core::ui::theme::Fill::Solid)
        .unwrap_or(fallback)
}

struct OrganizationPreviewAsset;

impl AsyncAssetType for OrganizationPreviewAsset {}

fn image_preview_source(bytes: Vec<u8>) -> AssetSource {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&bytes, &mut hasher);
    let key = format!(
        "organization-crop-preview-{}",
        std::hash::Hasher::finish(&hasher)
    );
    AssetSource::Async {
        id: AsyncAssetId::new::<OrganizationPreviewAsset>(key),
        fetch: Arc::new(move || {
            let bytes = Bytes::from(bytes.clone());
            Box::pin(async move { Ok(bytes) })
        }),
    }
}

#[cfg(test)]
#[path = "create_dialog_tests.rs"]
mod tests;
