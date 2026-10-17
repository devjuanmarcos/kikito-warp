//! The "Editor and Code Review" settings page, shown under the Code umbrella.

use warp_core::settings::{Setting as _, ToggleableSetting as _};
use warp_errors::report_if_error;
use warpui::elements::Element;
#[cfg(feature = "local_fs")]
use warpui::elements::{ChildView, Empty};
use warpui::keymap::ContextPredicate;
use warpui::ui_components::components::UiComponent;
use warpui::ui_components::switch::SwitchStateHandle;
use warpui::{
    Action, AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

#[cfg(feature = "local_fs")]
use super::features::external_editor::ExternalEditorView;
use super::settings_page::{
    MatchData, PageTitle, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
    render_body_item, render_dropdown_item,
};
use super::{
    LocalOnlyIconState, SettingsAction, SettingsSection, ToggleSettingActionPair, ToggleState,
    flags,
};
use crate::appearance::Appearance;
use crate::settings::{AppEditorSettings, CodeEditorLineNumberMode, CodeSettings};
use crate::terminal::general_settings::GeneralSettings;
use crate::view_components::{Dropdown, DropdownItem};
use crate::workspace::tab_settings::TabSettings;
use crate::{TelemetryEvent, send_telemetry_from_ctx};

const PAGE_TITLE: &str = "Editor e revisão de código";

pub struct EditorAndCodeReviewPageView {
    page: PageType<Self>,
    #[cfg(feature = "local_fs")]
    external_editor_view: Option<ViewHandle<ExternalEditorView>>,
    code_editor_line_number_mode_dropdown: ViewHandle<Dropdown<EditorAndCodeReviewPageAction>>,
}

impl EditorAndCodeReviewPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        #[cfg(feature = "local_fs")]
        let external_editor_view = Some(ctx.add_typed_action_view(ExternalEditorView::new));

        let code_editor_line_number_mode_dropdown = ctx.add_typed_action_view(Dropdown::new);
        Self::update_code_editor_line_number_mode_dropdown(
            code_editor_line_number_mode_dropdown.clone(),
            ctx,
        );

        ctx.subscribe_to_model(&AppEditorSettings::handle(ctx), |me, _, _, ctx| {
            Self::update_code_editor_line_number_mode_dropdown(
                me.code_editor_line_number_mode_dropdown.clone(),
                ctx,
            );
            ctx.notify();
        });

        Self {
            page: Self::build_page(ctx),
            #[cfg(feature = "local_fs")]
            external_editor_view,
            code_editor_line_number_mode_dropdown,
        }
    }

    fn build_page(ctx: &mut ViewContext<Self>) -> PageType<Self> {
        #[cfg(feature = "local_fs")]
        let mut widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
            vec![Box::new(ExternalEditorCodeWidget)];
        #[cfg(not(feature = "local_fs"))]
        let mut widgets: Vec<Box<dyn SettingsWidget<View = Self>>> = vec![];

        if AppEditorSettings::as_ref(ctx)
            .code_editor_line_number_mode
            .is_supported_on_current_platform()
        {
            widgets.push(Box::new(CodeEditorLineNumberModeWidget::default()));
        }

        widgets.extend([
            Box::new(AutoOpenCodeReviewPaneCodeWidget::default())
                as Box<dyn SettingsWidget<View = Self>>,
            Box::new(CodeReviewPanelToggleWidget::default()),
            Box::new(CodeReviewDiffStatsToggleWidget::default()),
            Box::new(ProjectExplorerToggleWidget::default()),
            Box::new(GlobalSearchToggleWidget::default()),
            Box::new(ShowHiddenFilesToggleWidget::default()),
            Box::new(FormatOnSaveToggleWidget::default()),
            Box::new(AutoSaveToggleWidget::default()),
        ]);

        PageType::new_uncategorized(widgets, Some(PageTitle::new(PAGE_TITLE)))
    }

    fn update_code_editor_line_number_mode_dropdown(
        dropdown: ViewHandle<Dropdown<EditorAndCodeReviewPageAction>>,
        ctx: &mut ViewContext<Self>,
    ) {
        dropdown.update(ctx, |dropdown, ctx| {
            let values = [
                CodeEditorLineNumberMode::Absolute,
                CodeEditorLineNumberMode::Relative,
            ];

            let current_value = *AppEditorSettings::as_ref(ctx)
                .code_editor_line_number_mode
                .value();

            let selected_index = values
                .iter()
                .position(|val| *val == current_value)
                .unwrap_or(0);

            dropdown.set_items(
                values
                    .into_iter()
                    .map(|val| {
                        DropdownItem::new(
                            val.dropdown_item_label(),
                            EditorAndCodeReviewPageAction::SetCodeEditorLineNumberMode(val),
                        )
                    })
                    .collect(),
                ctx,
            );
            dropdown.set_selected_by_index(selected_index, ctx);
        });
    }
}

impl Entity for EditorAndCodeReviewPageView {
    type Event = ();
}

impl View for EditorAndCodeReviewPageView {
    fn ui_name() -> &'static str {
        "EditorAndCodeReviewPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum EditorAndCodeReviewPageAction {
    ToggleCodeReviewPanel,
    ToggleShowCodeReviewDiffStats,
    ToggleAutoOpenCodeReviewPane,
    ToggleProjectExplorer,
    ToggleGlobalSearch,
    ToggleShowHiddenFiles,
    ToggleFormatOnSave,
    ToggleAutoSave,
    SetCodeEditorLineNumberMode(CodeEditorLineNumberMode),
}

impl TypedActionView for EditorAndCodeReviewPageView {
    type Action = EditorAndCodeReviewPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            EditorAndCodeReviewPageAction::ToggleCodeReviewPanel => {
                TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_code_review_button.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleShowCodeReviewDiffStats => {
                TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(
                        settings
                            .show_code_review_diff_stats
                            .toggle_and_save_value(ctx)
                    );
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleProjectExplorer => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_project_explorer.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleGlobalSearch => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_global_search.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleShowHiddenFiles => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_hidden_files.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleFormatOnSave => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.format_on_save.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleAutoSave => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.auto_save.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::ToggleAutoOpenCodeReviewPane => {
                GeneralSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(
                        settings
                            .auto_open_code_review_pane_on_first_agent_change
                            .toggle_and_save_value(ctx)
                    );
                });
                send_telemetry_from_ctx!(
                    TelemetryEvent::FeaturesPageAction {
                        action: "ToggleAutoOpenCodeReviewPane".to_string(),
                        value: format!(
                            "{}",
                            *GeneralSettings::as_ref(ctx)
                                .auto_open_code_review_pane_on_first_agent_change
                        )
                    },
                    ctx
                );
                ctx.notify();
            }
            EditorAndCodeReviewPageAction::SetCodeEditorLineNumberMode(mode) => {
                AppEditorSettings::handle(ctx).update(ctx, |editor_settings, ctx| {
                    report_if_error!(
                        editor_settings
                            .code_editor_line_number_mode
                            .set_value(*mode, ctx)
                    );
                    ctx.notify();
                });
                send_telemetry_from_ctx!(
                    TelemetryEvent::FeaturesPageAction {
                        action: "SetCodeEditorLineNumberMode".to_string(),
                        value: format!("{mode:?}"),
                    },
                    ctx
                );
            }
        }
    }
}

impl SettingsPageMeta for EditorAndCodeReviewPageView {
    fn section() -> SettingsSection {
        SettingsSection::EditorAndCodeReview
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<EditorAndCodeReviewPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<EditorAndCodeReviewPageView>) -> Self {
        SettingsPageViewHandle::EditorAndCodeReview(view_handle)
    }
}

pub fn init_actions_from_parent_view<T: Action + Clone>(
    app: &mut AppContext,
    context: &ContextPredicate,
    builder: fn(SettingsAction) -> T,
) {
    ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
        vec![
            ToggleSettingActionPair::new(
                "Abrir automaticamente o painel de revisão de código",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleAutoOpenCodeReviewPane,
                )),
                context,
                flags::AUTO_OPEN_CODE_REVIEW_PANE_FLAG,
            ),
            ToggleSettingActionPair::new(
                "Botão de revisão de código",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleCodeReviewPanel,
                )),
                context,
                flags::SHOW_CODE_REVIEW_BUTTON_FLAG,
            ),
            ToggleSettingActionPair::new(
                "Mostrar estatísticas de diff no botão de revisão de código",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleShowCodeReviewDiffStats,
                )),
                context,
                flags::SHOW_CODE_REVIEW_DIFF_STATS_FLAG,
            ),
            ToggleSettingActionPair::new(
                "Explorador de projetos",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleProjectExplorer,
                )),
                context,
                flags::SHOW_PROJECT_EXPLORER,
            ),
            ToggleSettingActionPair::new(
                "Busca global de arquivos",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleGlobalSearch,
                )),
                context,
                flags::SHOW_GLOBAL_SEARCH,
            ),
            ToggleSettingActionPair::new(
                "Mostrar arquivos ocultos no explorador de projetos",
                builder(SettingsAction::EditorAndCodeReview(
                    EditorAndCodeReviewPageAction::ToggleShowHiddenFiles,
                )),
                context,
                flags::SHOW_HIDDEN_FILES,
            ),
        ],
        app,
    );
}

#[cfg(feature = "local_fs")]
struct ExternalEditorCodeWidget;

#[cfg(feature = "local_fs")]
impl SettingsWidget for ExternalEditorCodeWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "editor de código arquivos abertos markdown conversas IA layout painel aba code editor open files markdown AI conversations layout pane tab"
    }

    fn render(
        &self,
        view: &Self::View,
        _appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        if let Some(editor_view) = &view.external_editor_view {
            ChildView::new(editor_view).finish()
        } else {
            Empty::new().finish()
        }
    }
}

#[derive(Default)]
struct AutoOpenCodeReviewPaneCodeWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for AutoOpenCodeReviewPaneCodeWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "abrir painel de revisão de código automaticamente primeira alteração aceita conversa modo agente oz auto open code review pane panel agent mode change first time accepted diff view conversation"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let general_settings = GeneralSettings::as_ref(app);
        render_body_item::<EditorAndCodeReviewPageAction>(
            "Abrir automaticamente o painel de revisão de código".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*general_settings.auto_open_code_review_pane_on_first_agent_change)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(
                        EditorAndCodeReviewPageAction::ToggleAutoOpenCodeReviewPane,
                    );
                })
                .finish(),
            Some("Quando ativada, o painel de revisão de código será aberto na primeira alteração aceita de uma conversa.".into()),
        )
    }
}

#[derive(Default)]
struct CodeReviewPanelToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for CodeReviewPanelToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "painel de revisão de código lado direito diferenças git code review panel right side diff git"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let tab_settings = TabSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Mostrar botão de revisão de código".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*tab_settings.show_code_review_button)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleCodeReviewPanel);
                })
                .finish(),
            Some(
                "Exibe um botão no canto superior direito da janela para alternar o painel de revisão de código."
                    .into(),
            ),
        )
    }
}

#[derive(Default)]
struct CodeReviewDiffStatsToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for CodeReviewDiffStatsToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "estatísticas de diferenças linhas adicionadas removidas revisão de código code review diff stats lines added removed counts"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let tab_settings = TabSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Mostrar estatísticas de diff no botão de revisão de código".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*tab_settings.show_code_review_diff_stats)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(
                        EditorAndCodeReviewPageAction::ToggleShowCodeReviewDiffStats,
                    );
                })
                .finish(),
            Some("Exibe no botão de revisão de código a quantidade de linhas adicionadas e removidas.".into()),
        )
    }
}

#[derive(Default)]
struct ProjectExplorerToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for ProjectExplorerToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "explorador de projetos árvore de arquivos painel esquerdo ferramentas project explorer file tree left panel tools"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Explorador de projetos".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_project_explorer)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleProjectExplorer);
                })
                .finish(),
            Some(
                "Adiciona um explorador de projetos em árvore ao painel de ferramentas à esquerda."
                    .into(),
            ),
        )
    }
}

#[derive(Default)]
struct GlobalSearchToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for GlobalSearchToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "busca global pesquisar arquivos painel esquerdo ferramentas global search file search left panel tools"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Busca global de arquivos".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_global_search)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleGlobalSearch);
                })
                .finish(),
            Some("Adiciona a busca global de arquivos ao painel de ferramentas à esquerda.".into()),
        )
    }
}

#[derive(Default)]
struct ShowHiddenFilesToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for ShowHiddenFilesToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "mostrar arquivos ocultos dotfiles explorador de projetos árvore de arquivos show hidden files dotfiles project explorer file tree"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Mostrar arquivos ocultos no explorador de projetos".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_hidden_files)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleShowHiddenFiles);
                })
                .finish(),
            Some(
                "Exibe no explorador de projetos dotfiles e outros arquivos ocultos (iniciados por ponto).".into(),
            ),
        )
    }
}

#[derive(Default)]
struct FormatOnSaveToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for FormatOnSaveToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "formatar ao salvar servidor de linguagem formatação editor format on save lsp language server formatting reformat editor"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Formatar ao salvar (requer um servidor de linguagem ativo)".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.format_on_save)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleFormatOnSave);
                })
                .finish(),
            Some(
                "Aplica-se quando há um servidor de linguagem ativo para o arquivo. Formata o arquivo ao salvar; os outros recursos LSP, como dicas, ir para definição, referências e diagnósticos, não são afetados."
                    .into(),
            ),
        )
    }
}

#[derive(Default)]
struct AutoSaveToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for AutoSaveToggleWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "auto save autosave automatically save editor files on type focus"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<EditorAndCodeReviewPageAction>(
            "Salvamento automático".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.auto_save)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(EditorAndCodeReviewPageAction::ToggleAutoSave);
                })
                .finish(),
            Some(
                "Salva automaticamente as alterações no editor de texto do Kikito Terminal enquanto você digita e quando o editor perde o foco."
                    .into(),
            ),
        )
    }
}

#[derive(Default)]
struct CodeEditorLineNumberModeWidget {}

impl SettingsWidget for CodeEditorLineNumberModeWidget {
    type View = EditorAndCodeReviewPageView;

    fn search_terms(&self) -> &str {
        "números de linha linha relativa modo vim margem editor de código line number numbers relative line vim gutter code editor"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        render_dropdown_item(
            appearance,
            "Números de linha do editor de código:",
            None,
            None,
            LocalOnlyIconState::Hidden,
            None,
            &view.code_editor_line_number_mode_dropdown,
        )
    }
}
