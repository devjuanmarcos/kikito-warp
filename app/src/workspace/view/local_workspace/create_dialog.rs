use std::path::PathBuf;
use std::sync::Arc;

use async_fs;
use bytes::Bytes;
use futures::io::AsyncReadExt as _;
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::r#async::SpawnedFutureHandle;
use warpui::elements::{
    Align, Border, CacheOption, ChildAnchor, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, Element, EventHandler, Flex, Image, MainAxisAlignment, MainAxisSize,
    OffsetPositioning, ParentAnchor, ParentElement, ParentOffsetBounds, Shrinkable, Stack, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;
use warpui::{
    AppContext, Entity, FocusContext, SingletonEntity, TypedActionView, View, ViewContext,
    ViewHandle,
};

use super::dialog_ui::{
    ButtonVariant, PixelAligned, color_dot, dialog_button, dialog_close, small_naked,
    small_secondary,
};
use super::ws_style::{mouse_state, neutral, radius, sized_icon};
use crate::appearance::Appearance;
use crate::editor::{EditorView, Event as EditorEvent, SingleLineEditorOptions, TextOptions};
use crate::local_workspace::model::{OrganizationId, ProjectId};
use pathfinder_color::ColorU;
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill;
use warpui::elements::Clipped;
use warpui::elements::Hoverable;
use warpui::geometry::vector::vec2f;
use warpui::ui_components::slider::SliderStateHandle;

use super::image_cropper::{CROP_VIEW_SIZE, CropView};
use warpui::platform::Cursor;
use warpui::ui_components::components::Coords;

/// Corner radius of the kit's crop square (22% of the viewport, the org tile's 8/36).
const CROP_SQUARE_RADIUS: f32 = CROP_VIEW_SIZE * 0.22;
/// Thickness of the dimming border; it only has to cover the corners outside the crop square.
const CROP_MASK_WIDTH: f32 = 60.;
const CROP_ZOOM_STEP: f32 = 0.25;
/// Height of the kit's slider (18px thumb), which sets the zoom row height.
const CROP_SLIDER_HEIGHT: f32 = 18.;
const CROP_WHEEL_FACTOR: f32 = 1.08;
/// Kit mono inputs use 13px text, which makes them 37px tall instead of 36px.
const PATH_FONT_SIZE: f32 = 13.;
const PATH_INPUT_HEIGHT: f32 = 37.;
/// Line box of dialog body text (`DialogBody` sets an 18px line height in the kit).
const BODY_LINE_HEIGHT: f32 = 18.;
/// Height of the kit's naked close button, which sets the header row height.
const HEADER_ROW_HEIGHT: f32 = 32.;

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
        Some("Aguarde a verificação da pasta terminar.")
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
        Ok(false) => Err("O caminho selecionado não é uma pasta."),
        Err(std::io::ErrorKind::NotFound) => Err("A pasta selecionada não foi encontrada."),
        Err(std::io::ErrorKind::NotADirectory) => Err("O caminho selecionado não é uma pasta."),
        Err(std::io::ErrorKind::PermissionDenied) => Err("Sem acesso à pasta selecionada."),
        Err(_) => Err("Não foi possível verificar a pasta selecionada."),
    }
}

#[derive(Clone, Debug)]
pub(crate) enum CreateDialogAction {
    Submit,
    Cancel,
    PickProjectPath,
    PickOrganizationImage,
    SelectAccent(String),
    DiscardImage,
    CropDragStart(f32, f32),
    CropDragTo(f32, f32),
    CropDragEnd,
    /// Zoom factor change, positive to zoom in.
    CropZoomBy(f32),
    CropZoomTo(f32),
    ApplyCrop,
    CancelCrop,
}

/// Image state to restore when "Voltar" leaves the crop step.
struct ImageBeforeCrop {
    image_data: Option<Vec<u8>>,
    source_image_data: Option<Vec<u8>>,
    source_dimensions: Option<(u32, u32)>,
}

/// Pointer and image position where the current crop drag started.
#[derive(Clone, Copy)]
struct CropDrag {
    pointer_x: f32,
    pointer_y: f32,
    image_x: f32,
    image_y: f32,
}

pub(crate) struct CreateDialog {
    kind: CreateDialogKind,
    name_editor: ViewHandle<EditorView>,
    path_editor: Option<ViewHandle<EditorView>>,
    organization_name: Option<String>,
    project_path: Option<PathBuf>,
    accent_color: String,
    image_data: Option<Vec<u8>>,
    source_image_data: Option<Vec<u8>>,
    source_dimensions: Option<(u32, u32)>,
    /// `(zoom, pan_x, pan_y)` of the applied crop.
    crop_parameters: (f32, f32, f32),
    /// Present while the "Ajustar imagem" step is shown.
    crop_view: Option<CropView>,
    crop_drag: Option<CropDrag>,
    image_before_crop: Option<ImageBeforeCrop>,
    crop_zoom_slider: SliderStateHandle,
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
        let kind_for_placeholder = kind.clone();
        let ui_text = TextOptions::ui_text(Some(12.), Appearance::as_ref(ctx));
        let name_editor = ctx.add_typed_action_view(|ctx| {
            let mut editor = EditorView::single_line(
                SingleLineEditorOptions {
                    text: ui_text,
                    ..Default::default()
                },
                ctx,
            );
            editor.set_placeholder_text(
                if matches!(
                    kind_for_placeholder,
                    CreateDialogKind::CreateProject(_) | CreateDialogKind::EditProject(_)
                ) {
                    "Ex.: keener-api"
                } else {
                    "Ex.: Keener"
                },
                ctx,
            );
            editor
        });
        name_editor.update(ctx, |editor, ctx| {
            if let Some(name) = name {
                editor.set_buffer_text_ignoring_undo(&name, ctx);
            }
        });
        ctx.subscribe_to_view(&name_editor, |me, _, event, ctx| match event {
            EditorEvent::Enter if me.crop_view.is_none() => me.try_submit(ctx),
            EditorEvent::Escape if me.crop_view.is_some() => me.cancel_crop(ctx),
            EditorEvent::Escape => ctx.emit(CreateDialogEvent::Close),
            EditorEvent::Edited(_) => {
                me.error = None;
                me.refresh_path_placeholder(ctx);
                ctx.notify();
            }
            _ => {}
        });
        let path_editor = matches!(kind, CreateDialogKind::CreateProject(_)).then(|| {
            let mono_text = TextOptions {
                font_size_override: Some(PATH_FONT_SIZE),
                ..Default::default()
            };
            let path_editor = ctx.add_typed_action_view(|ctx| {
                let mut editor = EditorView::single_line(
                    SingleLineEditorOptions {
                        text: mono_text,
                        ..Default::default()
                    },
                    ctx,
                );
                editor.set_placeholder_text(project_path_placeholder(""), ctx);
                editor
            });
            if let Some(path) = &project_path {
                let text = path.display().to_string();
                path_editor.update(ctx, |editor, ctx| {
                    editor.set_buffer_text_ignoring_undo(&text, ctx)
                });
            }
            ctx.subscribe_to_view(&path_editor, |me, _, event, ctx| match event {
                EditorEvent::Enter => me.try_submit(ctx),
                EditorEvent::Escape => ctx.emit(CreateDialogEvent::Close),
                EditorEvent::Edited(_) => {
                    me.error = None;
                    ctx.notify();
                }
                _ => {}
            });
            path_editor
        });
        Self {
            kind,
            name_editor,
            path_editor,
            organization_name: None,
            project_path,
            accent_color: accent_color.unwrap_or_else(|| "yellow".to_owned()),
            source_image_data: image_data.clone(),
            source_dimensions: image_data.as_ref().map(|_| (128, 128)),
            image_data,
            crop_parameters: (1., 0., 0.),
            crop_view: None,
            crop_drag: None,
            image_before_crop: None,
            crop_zoom_slider: SliderStateHandle::default(),
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
        if let Some(path_editor) = &self.path_editor {
            let text = path.display().to_string();
            path_editor.update(ctx, |editor, ctx| {
                editor.set_buffer_text_ignoring_undo(&text, ctx)
            });
        }
        self.project_path = Some(path);
        self.project_validation_generation = self.project_validation_generation.wrapping_add(1);
        self.project_validation_pending = false;
        self.error = None;
        self.status = None;
        ctx.notify();
    }

    /// Organization named in the "Novo projeto em …" title.
    pub(crate) fn set_organization_name(&mut self, name: String, ctx: &mut ViewContext<Self>) {
        self.organization_name = Some(name);
        ctx.notify();
    }

    fn refresh_path_placeholder(&mut self, ctx: &mut ViewContext<Self>) {
        let Some(path_editor) = self.path_editor.clone() else {
            return;
        };
        let name = self.name_editor.as_ref(ctx).buffer_text(ctx);
        path_editor.update(ctx, |editor, ctx| {
            editor.set_placeholder_text(project_path_placeholder(&name), ctx)
        });
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
            self.status = Some("Aguardando o processamento atual…".to_owned());
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
        self.image_before_crop = Some(ImageBeforeCrop {
            image_data: self.image_data.take(),
            source_image_data: self.source_image_data.take(),
            source_dimensions: self.source_dimensions.take(),
        });
        self.source_image_data = Some(source_bytes);
        self.source_dimensions = Some((cropped.source_width, cropped.source_height));
        self.image_data = Some(cropped.png);
        self.image_selection_error = false;
        self.crop_parameters = (1., 0., 0.);
        self.crop_view = Some(CropView::centered(
            cropped.source_width,
            cropped.source_height,
        ));
        self.crop_drag = None;
        self.crop_zoom_slider.reset_offset();
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
        self.crop_parameters = (1., 0., 0.);
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

    fn apply_crop(&mut self, ctx: &mut ViewContext<Self>) {
        let Some(view) = self.crop_view.take() else {
            return;
        };
        self.crop_drag = None;
        self.image_before_crop = None;
        self.crop_parameters = view.crop_parameters();
        self.request_crop_preview(ctx);
        ctx.notify();
    }

    fn cancel_crop(&mut self, ctx: &mut ViewContext<Self>) {
        if self.crop_view.take().is_none() {
            return;
        }
        self.crop_drag = None;
        if let Some(before) = self.image_before_crop.take() {
            self.image_data = before.image_data;
            self.source_image_data = before.source_image_data;
            self.source_dimensions = before.source_dimensions;
        }
        ctx.notify();
    }

    fn update_crop_view(
        &mut self,
        update: impl FnOnce(CropView) -> CropView,
        ctx: &mut ViewContext<Self>,
    ) {
        if let Some(view) = self.crop_view {
            self.crop_view = Some(update(view));
            ctx.notify();
        }
    }

    fn try_submit(&mut self, ctx: &mut ViewContext<Self>) {
        if self.crop_view.is_some() {
            return;
        }
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
        if let Some(path_editor) = &self.path_editor {
            let typed = path_editor.as_ref(ctx).buffer_text(ctx).trim().to_owned();
            let typed = if typed.is_empty() {
                project_path_placeholder(&name)
            } else {
                typed
            };
            self.project_path = Some(expand_home(&typed));
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

impl CreateDialog {
    fn image_field(&self, name: &str, appearance: &Appearance) -> Box<dyn Element> {
        let theme = appearance.theme();
        let accent = super::org_rail::organization_accent(theme, &self.accent_color);
        let initial = name
            .trim()
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string());
        let tile_content: Box<dyn Element> = if let Some(bytes) = &self.image_data {
            Image::new(image_preview_source(bytes.clone()), CacheOption::BySize)
                .with_corner_radius(warpui::elements::CornerRadius::with_all(
                    warpui::elements::Radius::Pixels(12.),
                ))
                .finish()
        } else {
            let glyph = match initial {
                Some(initial) => Text::new_inline(initial, appearance.ui_font_family(), 22.)
                    .with_style(Properties::default().weight(Weight::Bold))
                    .with_color(accent.into())
                    .finish(),
                None => sized_icon(Icon::Image, accent, 20.),
            };
            Align::new(glyph).finish()
        };
        let tile = ConstrainedBox::new(
            Container::new(tile_content)
                .with_background(super::org_rail::organization_accent_with_alpha(
                    theme,
                    &self.accent_color,
                    56,
                ))
                .with_border(Border::all(1.).with_border_fill(
                    super::org_rail::organization_accent_with_alpha(theme, &self.accent_color, 102),
                ))
                .with_corner_radius(warpui::elements::CornerRadius::with_all(
                    warpui::elements::Radius::Pixels(12.),
                ))
                .finish(),
        )
        .with_width(56.)
        .with_height(56.)
        .finish();
        let tile = Hoverable::new(mouse_state("dialog-image-tile", 0), |_| tile)
            .with_cursor(Cursor::PointingHand)
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(CreateDialogAction::PickOrganizationImage)
            })
            .finish();
        let mut actions = Flex::column().with_spacing(6.);
        let mut buttons = Flex::row().with_spacing(6.);
        buttons.add_child(small_secondary(
            theme,
            ("dialog-image-button", 0),
            |ctx| ctx.dispatch_typed_action(CreateDialogAction::PickOrganizationImage),
            Container::new(
                Flex::row()
                    .with_main_axis_size(MainAxisSize::Min)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(2.)
                    .with_child(sized_icon(Icon::Upload, theme.foreground(), 14.))
                    .with_child(
                        Text::new_inline(
                            if self.image_data.is_some() {
                                "Trocar imagem"
                            } else {
                                "Escolher imagem"
                            },
                            appearance.ui_font_family(),
                            12.,
                        )
                        .with_style(Properties::default().weight(Weight::Semibold))
                        .with_color(theme.foreground().into())
                        .finish(),
                    )
                    .finish(),
            )
            .with_padding_left(8.)
            .with_padding_right(8.)
            .with_padding_bottom(2.)
            .finish(),
        ));
        if self.image_data.is_some() {
            buttons.add_child(small_naked(
                theme,
                ("dialog-image-remove", 0),
                |ctx| ctx.dispatch_typed_action(CreateDialogAction::DiscardImage),
                Container::new(
                    Text::new_inline("Remover", appearance.ui_font_family(), 12.)
                        .with_style(Properties::default().weight(Weight::Semibold))
                        .with_color(theme.foreground().into())
                        .finish(),
                )
                .with_padding_left(8.)
                .with_padding_right(8.)
                .with_padding_bottom(2.)
                .finish(),
            ));
        }
        actions.add_child(buttons.finish());
        actions.add_child(
            Text::new_inline(
                "PNG, JPG ou SVG.".to_owned(),
                appearance.ui_font_family(),
                11.,
            )
            .with_line_height_ratio(BODY_LINE_HEIGHT / 11.)
            .with_color(theme.hint_text_color(theme.surface_1()).into())
            .finish(),
        );
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(12.)
            .with_child(tile)
            .with_child(actions.finish())
            .finish()
    }
}

impl CreateDialog {
    /// "Ajustar imagem" step: drag to position, slider or wheel to zoom, 128px output.
    fn crop_step(&self, view: &CropView, appearance: &Appearance) -> Box<dyn Element> {
        let theme = appearance.theme();
        let (width, height) = view.displayed_size();
        let unbounded_at = |x: f32, y: f32| {
            OffsetPositioning::offset_from_parent(
                vec2f(x, y),
                ParentOffsetBounds::Unbounded,
                ParentAnchor::TopLeft,
                ChildAnchor::TopLeft,
            )
        };
        let block = |fill: Fill, width: f32, height: f32| {
            ConstrainedBox::new(
                Container::new(Flex::row().finish())
                    .with_background(fill)
                    .finish(),
            )
            .with_width(width)
            .with_height(height)
            .finish()
        };
        let mut viewport = Stack::new();
        viewport.add_child(block(neutral(theme, 10), CROP_VIEW_SIZE, CROP_VIEW_SIZE));
        if let Some(bytes) = &self.source_image_data {
            viewport.add_positioned_child(
                ConstrainedBox::new(
                    Image::new(image_preview_source(bytes.clone()), CacheOption::BySize).finish(),
                )
                .with_width(width)
                .with_height(height)
                .finish(),
                unbounded_at(view.x, view.y),
            );
        }
        // The kit dims everything outside the rounded crop square; a border thick enough to
        // reach the viewport corners, whose inner edge follows that square, paints the same mask.
        viewport.add_positioned_child(
            ConstrainedBox::new(
                Container::new(Flex::row().finish())
                    .with_border(
                        Border::all(CROP_MASK_WIDTH)
                            .with_border_fill(Fill::Solid(ColorU::new(0, 0, 0, 140))),
                    )
                    .with_corner_radius(radius(CROP_SQUARE_RADIUS + CROP_MASK_WIDTH))
                    .finish(),
            )
            .with_width(CROP_VIEW_SIZE + 2. * CROP_MASK_WIDTH)
            .with_height(CROP_VIEW_SIZE + 2. * CROP_MASK_WIDTH)
            .finish(),
            unbounded_at(-CROP_MASK_WIDTH, -CROP_MASK_WIDTH),
        );
        viewport.add_child(
            ConstrainedBox::new(
                Container::new(Flex::row().finish())
                    .with_border(
                        Border::all(1.)
                            .with_border_fill(Fill::Solid(ColorU::new(255, 255, 255, 179))),
                    )
                    .with_corner_radius(radius(CROP_SQUARE_RADIUS))
                    .finish(),
            )
            .with_width(CROP_VIEW_SIZE)
            .with_height(CROP_VIEW_SIZE)
            .finish(),
        );
        if self.crop_drag.is_some() {
            let line = Fill::Solid(ColorU::new(255, 255, 255, 89));
            for third in [1., 2.] {
                let offset = (CROP_VIEW_SIZE * third / 3.).round();
                viewport.add_positioned_child(
                    block(line.clone(), 1., CROP_VIEW_SIZE),
                    unbounded_at(offset, 0.),
                );
                viewport.add_positioned_child(
                    block(line.clone(), CROP_VIEW_SIZE, 1.),
                    unbounded_at(0., offset),
                );
            }
        }
        let viewport = Clipped::new(
            ConstrainedBox::new(viewport.finish())
                .with_width(CROP_VIEW_SIZE)
                .with_height(CROP_VIEW_SIZE)
                .finish(),
        )
        .finish();
        let dragging = self.crop_drag.is_some();
        let zoom = view.zoom;
        let viewport = EventHandler::new(
            Hoverable::new(mouse_state("crop-viewport", 0), |_| viewport)
                .with_cursor(if dragging {
                    Cursor::ClosedHand
                } else {
                    Cursor::OpenHand
                })
                .finish(),
        )
        .on_left_mouse_down(|ctx, _, position| {
            ctx.dispatch_typed_action(CreateDialogAction::CropDragStart(
                position.x(),
                position.y(),
            ));
            DispatchEventResult::StopPropagation
        })
        .on_mouse_dragged(|ctx, _, position| {
            ctx.dispatch_typed_action(CreateDialogAction::CropDragTo(position.x(), position.y()));
            DispatchEventResult::StopPropagation
        })
        .on_left_mouse_up(|ctx, _, _| {
            ctx.dispatch_typed_action(CreateDialogAction::CropDragEnd);
            DispatchEventResult::StopPropagation
        })
        .on_scroll_wheel(move |ctx, _, delta, _| {
            let factor = if delta.y() > 0. {
                CROP_WHEEL_FACTOR
            } else {
                1. / CROP_WHEEL_FACTOR
            };
            ctx.dispatch_typed_action(CreateDialogAction::CropZoomBy(zoom * factor - zoom));
            DispatchEventResult::StopPropagation
        })
        .finish();

        let sub = theme.sub_text_color(theme.surface_1());
        let zoom_button = |icon: Icon, key: i64, step: f32| {
            Hoverable::new(mouse_state("crop-zoom-step", key), move |_| {
                sized_icon(icon, sub, 14.)
            })
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(CreateDialogAction::CropZoomBy(step))
            })
            .finish()
        };
        let slider = appearance
            .ui_builder()
            .slider(self.crop_zoom_slider.clone())
            .with_range(1.0..4.0)
            .with_default_value(view.zoom)
            .with_style(UiComponentStyles {
                width: Some(160.),
                ..Default::default()
            })
            .on_drag(|ctx, _, zoom| ctx.dispatch_typed_action(CreateDialogAction::CropZoomTo(zoom)))
            .on_change(|ctx, _, zoom| {
                ctx.dispatch_typed_action(CreateDialogAction::CropZoomTo(zoom))
            })
            .build()
            .finish();
        let zoom_row = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(10.)
            .with_child(zoom_button(Icon::Minus, 0, -CROP_ZOOM_STEP))
            .with_child(slider)
            .with_child(zoom_button(Icon::Plus, 1, CROP_ZOOM_STEP))
            .finish();

        let buttons = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::End)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(8.)
            .with_child(dialog_button(
                appearance,
                None,
                "Voltar",
                &[],
                ButtonVariant::Secondary,
                true,
                ("crop-back", 0),
                |ctx| ctx.dispatch_typed_action(CreateDialogAction::CancelCrop),
            ))
            .with_child(dialog_button(
                appearance,
                None,
                "Aplicar recorte",
                &[],
                ButtonVariant::Primary,
                self.source_image_data.is_some(),
                ("crop-apply", 0),
                |ctx| ctx.dispatch_typed_action(CreateDialogAction::ApplyCrop),
            ))
            .finish();

        Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(14.)
            .with_child(viewport)
            .with_child(
                ConstrainedBox::new(zoom_row)
                    .with_height(CROP_SLIDER_HEIGHT)
                    .finish(),
            )
            .with_child(
                Text::new_inline(
                    "Arraste para posicionar · role para dar zoom",
                    appearance.ui_font_family(),
                    11.,
                )
                .with_line_height_ratio(BODY_LINE_HEIGHT / 11.)
                .with_color(theme.hint_text_color(theme.surface_1()).into())
                .finish(),
            )
            .with_child(buttons)
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

    fn on_focus(&mut self, focus_ctx: &FocusContext, ctx: &mut ViewContext<Self>) {
        if focus_ctx.is_self_focused() {
            ctx.focus(&self.name_editor);
        }
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let confirmation = matches!(
            &self.kind,
            CreateDialogKind::RemoveOrganization(_) | CreateDialogKind::RemoveProject(_)
        );
        let title = match &self.kind {
            _ if self.crop_view.is_some() => "Ajustar imagem".to_owned(),
            CreateDialogKind::CreateOrganization => "Nova organização".to_owned(),
            CreateDialogKind::EditOrganization(_) => "Editar organização".to_owned(),
            CreateDialogKind::RemoveOrganization(_) => "Remover organização".to_owned(),
            CreateDialogKind::CreateProject(_) => match &self.organization_name {
                Some(organization) => format!("Novo projeto em {organization}"),
                None => "Novo projeto".to_owned(),
            },
            CreateDialogKind::EditProject(_) => "Editar projeto".to_owned(),
            CreateDialogKind::RemoveProject(_) => "Remover projeto".to_owned(),
        };
        let cropping = self.crop_view.is_some();
        let close_button = dialog_close(appearance, ("dialog-close", 0), move |ctx| {
            ctx.dispatch_typed_action(if cropping {
                CreateDialogAction::CancelCrop
            } else {
                CreateDialogAction::Cancel
            })
        });
        let mut header = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);
        header.add_child(
            Text::new_inline(title, appearance.ui_font_family(), 18.)
                .with_color(theme.main_text_color(theme.surface_1()).into())
                .with_style(Properties::default().weight(Weight::Semibold))
                .finish(),
        );
        header.add_child(close_button);
        let header = Container::new(
            ConstrainedBox::new(header.finish())
                .with_min_height(HEADER_ROW_HEIGHT)
                .finish(),
        )
        .with_padding_left(24.)
        .with_padding_right(24.)
        .with_padding_top(24.)
        .with_padding_bottom(12.)
        .finish();
        if let Some(view) = &self.crop_view {
            let body = Container::new(self.crop_step(view, appearance))
                .with_padding_left(24.)
                .with_padding_right(24.)
                .with_padding_bottom(24.)
                .finish();
            return dialog_frame(theme, vec![header, body]);
        }
        let mut body = Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(14.);
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
                    .with_line_height_ratio(BODY_LINE_HEIGHT / 12.)
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
            );
        } else {
            let description = match &self.kind {
                CreateDialogKind::CreateOrganization | CreateDialogKind::EditOrganization(_) => {
                    "Agrupe projetos por empresa, cliente ou contexto."
                }
                CreateDialogKind::CreateProject(_) | CreateDialogKind::EditProject(_) => {
                    "Cada projeto guarda as próprias abas."
                }
                CreateDialogKind::RemoveOrganization(_) | CreateDialogKind::RemoveProject(_) => "",
            };
            body.add_child(
                Text::new_inline(description, appearance.ui_font_family(), 12.)
                    .with_line_height_ratio(BODY_LINE_HEIGHT / 12.)
                    .with_color(theme.sub_text_color(theme.surface_1()).into())
                    .finish(),
            );
            body.add_child(field(
                "Nome",
                text_input(&self.name_editor, 36., app),
                appearance,
            ));
        }
        if matches!(
            &self.kind,
            CreateDialogKind::CreateOrganization | CreateDialogKind::EditOrganization(_)
        ) {
            body.add_child(field(
                "Imagem",
                self.image_field(&self.name_editor.as_ref(app).buffer_text(app), appearance),
                appearance,
            ));
            let mut colors = Flex::row().with_spacing(6.);
            for (index, color) in ["blue", "green", "magenta", "yellow", "cyan", "red"]
                .into_iter()
                .enumerate()
            {
                let selected = self.accent_color == color;
                let color_name = color.to_owned();
                colors.add_child(color_dot(
                    super::org_rail::organization_accent(theme, color),
                    selected,
                    theme.accent(),
                    ("dialog-color", index as i64),
                    move |ctx| {
                        ctx.dispatch_typed_action(CreateDialogAction::SelectAccent(
                            color_name.clone(),
                        ))
                    },
                ));
            }
            body.add_child(field(
                if self.image_data.is_some() {
                    "Cor de destaque"
                } else {
                    "Cor"
                },
                colors.finish(),
                appearance,
            ));
            if self.image_selection_error {
                body.add_child(Self::button(
                    "Remover imagem",
                    CreateDialogAction::DiscardImage,
                    appearance,
                ));
            }
        }
        if let Some(path_editor) = &self.path_editor {
            let mut path_field = Stack::new();
            path_field.add_child(text_input(path_editor, PATH_INPUT_HEIGHT, app));
            // The kit types the directory; the folder button keeps the native picker reachable.
            path_field.add_positioned_child(
                Hoverable::new(mouse_state("dialog-pick-path", 0), |mouse| {
                    sized_icon(
                        Icon::Folder,
                        if mouse.is_mouse_over_element() {
                            theme.foreground()
                        } else {
                            theme.foreground().with_opacity(40)
                        },
                        14.,
                    )
                })
                .with_cursor(Cursor::PointingHand)
                .on_click(|ctx, _, _| {
                    ctx.dispatch_typed_action(CreateDialogAction::PickProjectPath)
                })
                .finish(),
                OffsetPositioning::offset_from_parent(
                    vec2f(-11., 0.),
                    ParentOffsetBounds::ParentByPosition,
                    ParentAnchor::MiddleRight,
                    ChildAnchor::MiddleRight,
                ),
            );
            body.add_child(field("Diretório", path_field.finish(), appearance));
        }
        if let Some(error) = &self.error {
            body.add_child(
                Text::new_inline(error.clone(), appearance.ui_font_family(), 12.)
                    .with_color(theme.ui_error_color())
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
        let name_filled = confirmation
            || !self
                .name_editor
                .as_ref(app)
                .buffer_text(app)
                .trim()
                .is_empty();
        let mut footer = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::End)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(8.);
        footer.add_child(dialog_button(
            appearance,
            None,
            "Cancelar",
            &[],
            ButtonVariant::Secondary,
            true,
            ("dialog-cancel", 0),
            |ctx| ctx.dispatch_typed_action(CreateDialogAction::Cancel),
        ));
        footer.add_child(dialog_button(
            appearance,
            None,
            match &self.kind {
                CreateDialogKind::CreateOrganization | CreateDialogKind::CreateProject(_) => {
                    "Criar"
                }
                CreateDialogKind::EditOrganization(_) | CreateDialogKind::EditProject(_) => {
                    "Salvar"
                }
                CreateDialogKind::RemoveOrganization(_) | CreateDialogKind::RemoveProject(_) => {
                    "Remover"
                }
            },
            &["⏎"],
            if confirmation {
                ButtonVariant::Error
            } else {
                ButtonVariant::Primary
            },
            name_filled,
            ("dialog-submit", 0),
            |ctx| ctx.dispatch_typed_action(CreateDialogAction::Submit),
        ));
        let body = Container::new(Shrinkable::new(1., body.finish()).finish())
            .with_padding_left(24.)
            .with_padding_right(24.)
            .with_padding_bottom(24.)
            .finish();
        let footer = Container::new(footer.finish())
            .with_padding_left(24.)
            .with_padding_right(24.)
            .with_padding_top(12.)
            .with_padding_bottom(12.)
            .with_border(Border::top(1.).with_border_fill(neutral(theme, 20)))
            .finish();
        dialog_frame(theme, vec![header, body, footer])
    }
}

/// Kit `Dialog` frame sized to its content: surface-1, 1px neutral-4 border, 8px radius.
fn dialog_frame(
    theme: &warp_core::ui::theme::WarpTheme,
    sections: Vec<Box<dyn Element>>,
) -> Box<dyn Element> {
    let mut dialog = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    for section in sections {
        dialog.add_child(section);
    }
    PixelAligned::new(
        Container::new(dialog.finish())
            .with_background(theme.surface_1())
            .with_border(Border::all(1.).with_border_fill(neutral(theme, 20)))
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(8.),
            ))
            .finish(),
    )
    .finish()
}

/// Kit `TextInput`: 10px padding, 1px border at 20% foreground, accent border while focused.
fn text_input(editor: &ViewHandle<EditorView>, height: f32, app: &AppContext) -> Box<dyn Element> {
    let theme = Appearance::as_ref(app).theme();
    TextInput::new(
        editor.clone(),
        UiComponentStyles::default()
            .set_border_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(4.),
            ))
            .set_border_width(1.)
            .set_border_color(
                if editor.is_focused(app) {
                    theme.accent()
                } else {
                    theme.foreground().with_opacity(20)
                }
                .into(),
            )
            .set_background(theme.background().into())
            .set_padding(Coords {
                top: 10.,
                bottom: 10.,
                left: 10.,
                right: 10.,
            })
            .set_height(height),
    )
    .build()
    .finish()
}

/// Default directory the kit proposes for a project name: `~/` plus an ASCII slug.
fn project_path_placeholder(name: &str) -> String {
    let mut slug = String::new();
    for c in name.trim().to_lowercase().chars() {
        let c = match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            c => c,
        };
        if c.is_ascii_alphanumeric() {
            slug.push(c);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-');
    format!("~/{}", if slug.is_empty() { "projeto" } else { slug })
}

fn expand_home(path: &str) -> PathBuf {
    let home_relative = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\"));
    match (home_relative, dirs::home_dir()) {
        (Some(rest), Some(home)) => home.join(rest),
        (None, Some(home)) if path == "~" => home,
        _ => PathBuf::from(path),
    }
}

fn field(label: &str, content: Box<dyn Element>, appearance: &Appearance) -> Box<dyn Element> {
    let mut field = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(6.);
    field.add_child(
        Text::new_inline(label.to_owned(), appearance.ui_font_family(), 12.)
            .with_line_height_ratio(BODY_LINE_HEIGHT / 12.)
            .with_color(
                appearance
                    .theme()
                    .main_text_color(appearance.theme().surface_1())
                    .into(),
            )
            .finish(),
    );
    field.add_child(content);
    field.finish()
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
            CreateDialogAction::DiscardImage => self.discard_image(ctx),
            CreateDialogAction::CropDragStart(x, y) => {
                if let Some(view) = self.crop_view {
                    self.crop_drag = Some(CropDrag {
                        pointer_x: *x,
                        pointer_y: *y,
                        image_x: view.x,
                        image_y: view.y,
                    });
                    ctx.notify();
                }
            }
            CreateDialogAction::CropDragTo(x, y) => {
                if let Some(drag) = self.crop_drag {
                    self.update_crop_view(
                        |view| {
                            view.moved_to(
                                drag.image_x + x - drag.pointer_x,
                                drag.image_y + y - drag.pointer_y,
                            )
                        },
                        ctx,
                    );
                }
            }
            CreateDialogAction::CropDragEnd => {
                if self.crop_drag.take().is_some() {
                    ctx.notify();
                }
            }
            CreateDialogAction::CropZoomBy(delta) => {
                self.crop_zoom_slider.reset_offset();
                self.update_crop_view(|view| view.zoomed(view.zoom + delta), ctx);
            }
            CreateDialogAction::CropZoomTo(zoom) => {
                self.update_crop_view(|view| view.zoomed(*zoom), ctx);
            }
            CreateDialogAction::ApplyCrop => self.apply_crop(ctx),
            CreateDialogAction::CancelCrop => self.cancel_crop(ctx),
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
        let (zoom, pan_x, pan_y) = self.crop_parameters;
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
                        dialog.set_error("Não foi possível atualizar o recorte.".to_owned(), ctx)
                    }
                }
            },
        ));
    }
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
