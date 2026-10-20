mod context_menu;
pub(super) mod create_dialog;
mod home;
pub(super) mod image_cropper;
mod org_rail;
mod project_sidebar;
mod project_tabs;
mod source_control;
mod ws_style;
pub(crate) use source_control::DiffPreview;

use warpui::ViewHandle;
use crate::workspace::action::WorkspaceAction;
use warpui::elements::{
    Align, Border, ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, Element, EventHandler, Flex, MainAxisAlignment, MainAxisSize,
    ParentElement, Shrinkable, Text,
};

pub(super) fn preferred_open_project_tab(
    model: &WorkspaceModel,
    project_id: crate::local_workspace::model::ProjectId,
    available_workspace_tabs: &[String],
) -> Option<i32> {
    let project = model.project(project_id)?;
    let is_open = |tab_id| {
        model
            .workspace_tab_id(tab_id)
            .is_some_and(|identity| available_workspace_tabs.iter().any(|id| id == identity))
    };
    project
        .active_tab
        .filter(|tab_id| is_open(*tab_id))
        .or_else(|| project.tabs.iter().copied().find(|tab_id| is_open(*tab_id)))
}

fn project_has_open_terminal(
    model: &WorkspaceModel,
    project_id: crate::local_workspace::model::ProjectId,
    available_workspace_tabs: &[String],
) -> bool {
    preferred_open_project_tab(model, project_id, available_workspace_tabs).is_some()
}

use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{LocalBranch, RepositoryStatus};
use crate::local_workspace::model::WorkspaceModel;

fn should_render_project_sidebar(model: &WorkspaceModel) -> bool {
    model.selected_organization().is_some()
}

fn should_render_empty_organization_state(model: &WorkspaceModel) -> bool {
    model.selected_organization().is_some() && model.selected_project().is_none()
}

pub(super) fn can_activate_tab_in_selected_project(
    model: &WorkspaceModel,
    selected_project: Option<crate::local_workspace::model::ProjectId>,
    workspace_tab_id: &str,
) -> bool {
    let Some(project_id) = selected_project else {
        return true;
    };
    model
        .tab_identity_id(workspace_tab_id)
        .and_then(|tab_id| model.tab_owner(tab_id))
        == Some(project_id)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TabActivationRoute {
    KeepSelection,
    Home,
    Project {
        organization_id: crate::local_workspace::model::OrganizationId,
        project_id: crate::local_workspace::model::ProjectId,
    },
}

pub(super) fn tab_activation_route(
    model: &WorkspaceModel,
    workspace_tab_id: &str,
) -> TabActivationRoute {
    let target_project = model
        .tab_identity_id(workspace_tab_id)
        .and_then(|tab_id| model.tab_owner(tab_id))
        .and_then(|project_id| {
            model
                .project(project_id)
                .map(|project| (project.organization_id, project_id))
        });

    match (model.selected_project(), target_project) {
        (Some(selected), Some((_, target))) if selected == target => {
            TabActivationRoute::KeepSelection
        }
        (_, Some((organization_id, project_id))) => TabActivationRoute::Project {
            organization_id,
            project_id,
        },
        (Some(_), None) => TabActivationRoute::Home,
        (None, None) if model.selected_organization().is_some() => TabActivationRoute::Home,
        (None, None) => TabActivationRoute::KeepSelection,
    }
}

pub(crate) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    content: Box<dyn Element>,
    search_input: ViewHandle<EditorView>,
    commit_input: ViewHandle<EditorView>,
    search_query: &str,
    project_sidebar_scroll: ClippedScrollStateHandle,
    unassigned_tabs: &[(usize, String)],
    available_workspace_tabs: &[String],
    workspace_tab_titles: &std::collections::HashMap<String, String>,
    home_scroll: ClippedScrollStateHandle,
    git_status_summary: Option<&str>,
    git_status_error: Option<&str>,
    git_status: Option<&RepositoryStatus>,
    git_branches: &[LocalBranch],
    git_branch_menu: Option<&str>,
    git_create_branch_from: Option<&str>,
    git_rename_branch: Option<&str>,
    git_branch_name_input: ViewHandle<EditorView>,
    git_diff_preview: Option<&DiffPreview>,
    git_amend_confirmation: bool,
    git_undo_confirmation: bool,
    git_revert_confirmation: bool,
    git_discard_confirmation: Option<(&str, bool)>,
    git_operation_pending: bool,
    source_control_scroll: ClippedScrollStateHandle,
    source_control_open: bool,
) -> Box<dyn Element> {
    let mut workspace = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    workspace.add_child(org_rail::render(model, appearance));
    if should_render_project_sidebar(model) {
        workspace.add_child(project_sidebar::render(
            model,
            appearance,
            search_input,
            search_query,
            project_sidebar_scroll,
            git_status_summary,
            git_status_error,
            git_status,
        ));
    }
    let main_content = if let Some(project_id) = model.selected_project() {
        if model.project(project_id).is_some() {
            let theme = appearance.theme();
            let project_bar = project_tabs::render(
                model,
                appearance,
                project_id,
                available_workspace_tabs,
                workspace_tab_titles,
                source_control_open,
            )
            .unwrap_or_else(|| Flex::row().finish());
            let has_open_project_tab =
                project_has_open_terminal(model, project_id, available_workspace_tabs);
            let project_content: Box<dyn Element> = if !has_open_project_tab {
                let action = crate::workspace::action::WorkspaceAction::AddLocalWorkspaceTerminal;
                let empty = EventHandler::new(
                    Container::new(
                        Text::new_inline(
                            "Este projeto ainda não tem terminais. Crie um terminal para começar.",
                            appearance.ui_font_family(),
                            13.,
                        )
                        .with_color(theme.sub_text_color(theme.background()).into())
                        .finish(),
                    )
                    .with_uniform_padding(24.)
                    .finish(),
                )
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish();
                empty
            } else {
                content
            };
            let project_view = Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_child(project_bar)
                .with_child(Shrinkable::new(1., project_content).finish())
                .finish();
            if source_control_open {
                Flex::row()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .with_child(Shrinkable::new(1., project_view).finish())
                    .with_child(source_control::render(
                        git_status,
                        git_branches,
                        git_branch_menu,
                        git_create_branch_from,
                        git_rename_branch,
                        git_branch_name_input,
                        git_status_error,
                        commit_input,
                        git_diff_preview,
                        git_amend_confirmation,
                        git_undo_confirmation,
                        git_revert_confirmation,
                        git_discard_confirmation,
                        git_operation_pending,
                        source_control_scroll,
                        appearance,
                    ))
                    .finish()
            } else {
                project_view
            }
        } else {
            home::render(
                model,
                appearance,
                unassigned_tabs,
                workspace_tab_titles,
                home_scroll,
            )
        }
    } else if should_render_empty_organization_state(model) {
        empty_organization_state(model, appearance, workspace_tab_titles, home_scroll)
    } else {
        home::render(
            model,
            appearance,
            unassigned_tabs,
            workspace_tab_titles,
            home_scroll,
        )
    };
    workspace.add_child(Shrinkable::new(1., main_content).finish());
    workspace.finish()
}

fn empty_organization_state(
    model: &WorkspaceModel,
    appearance: &Appearance,
    workspace_tab_titles: &std::collections::HashMap<String, String>,
    home_scroll: ClippedScrollStateHandle,
) -> Box<dyn Element> {
    let Some(organization_id) = model.selected_organization() else {
        return home::render(model, appearance, &[], workspace_tab_titles, home_scroll);
    };
    let Some(organization) = model.organization(organization_id) else {
        return home::render(model, appearance, &[], workspace_tab_titles, home_scroll);
    };
    let theme = appearance.theme();
    let search = Container::new(
        Text::new_inline("Buscar projetos e abas...", appearance.ui_font_family(), 12.)
            .with_color(theme.sub_text_color(theme.surface_1()).into())
            .finish(),
    )
    .with_background(theme.surface_2())
    .with_padding_left(10.)
    .with_padding_right(10.)
    .with_padding_top(5.)
    .with_padding_bottom(5.)
    .with_corner_radius(warpui::elements::CornerRadius::with_all(
        warpui::elements::Radius::Pixels(4.),
    ))
    .finish();
    let mut header = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    header.add_child(
        Shrinkable::new(
            1.,
            Text::new_inline(
                organization.name.clone(),
                appearance.ui_font_family(),
                12.,
            )
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish(),
        )
        .finish(),
    );
    header.add_child(ConstrainedBox::new(search).with_width(240.).finish());
    let header = ConstrainedBox::new(
        Container::new(header.finish())
            .with_background(theme.surface_1())
            .with_padding_left(16.)
            .with_padding_right(8.)
            .with_border(Border::bottom(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_height(34.)
    .finish();
    let create_action = WorkspaceAction::CreateLocalWorkspaceProject(organization_id);
    let create_button = EventHandler::new(
        Container::new(
            Text::new_inline("+  Novo projeto", appearance.ui_font_family(), 12.)
                .with_color(theme.font_color(theme.surface_1()).into())
                .finish(),
        )
        .with_background(theme.surface_2())
        .with_padding_left(12.)
        .with_padding_right(12.)
        .with_padding_top(8.)
        .with_padding_bottom(8.)
        .with_corner_radius(warpui::elements::CornerRadius::with_all(
            warpui::elements::Radius::Pixels(4.),
        ))
        .finish(),
    )
    .on_left_mouse_up(move |ctx, _, _| {
        ctx.dispatch_typed_action(create_action.clone());
        DispatchEventResult::StopPropagation
    })
    .finish();
    let mut empty = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::Center)
        .with_spacing(12.);
    empty.add_child(
        Text::new_inline(
            "Nada aqui ainda. Quer que eu crie um projeto?",
            appearance.ui_font_family(),
            14.,
        )
        .with_color(theme.sub_text_color(theme.background()).into())
        .finish(),
    );
    empty.add_child(create_button);
    Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(header)
        .with_child(Shrinkable::new(1., Align::new(empty.finish()).finish()).finish())
        .finish()
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
