mod context_menu;
pub(super) mod create_dialog;
mod home;
pub(super) mod image_cropper;
mod org_rail;
mod project_sidebar;

use warp_core::ui::Icon;
use warpui::ViewHandle;
use warpui::elements::{
    ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult, Element, EventHandler,
    Flex, MainAxisSize, ParentElement, Shrinkable, Text,
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
use crate::local_workspace::model::WorkspaceModel;

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
    search_query: &str,
    unassigned_tabs: &[(usize, String)],
    available_workspace_tabs: &[String],
) -> Box<dyn Element> {
    let mut workspace = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    workspace.add_child(org_rail::render(model, appearance));
    workspace.add_child(project_sidebar::render(
        model,
        appearance,
        search_input,
        search_query,
    ));
    let main_content = if let Some(project_id) = model.selected_project() {
        if let Some(project) = model.project(project_id) {
            let theme = appearance.theme();
            let mut tab_title = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(8.)
                .with_cross_axis_alignment(CrossAxisAlignment::Center);
            for (index, tab_id) in project.tabs.iter().enumerate() {
                let Some(workspace_tab_id) = model.workspace_tab_id(*tab_id) else {
                    continue;
                };
                if !available_workspace_tabs
                    .iter()
                    .any(|id| id == workspace_tab_id)
                {
                    continue;
                }
                let active = project.active_tab == Some(*tab_id);
                let select_action =
                    crate::workspace::action::WorkspaceAction::SelectLocalWorkspaceTab(*tab_id);
                let close_action =
                    crate::workspace::action::WorkspaceAction::CloseLocalWorkspaceTab(*tab_id);
                let move_left_action =
                    crate::workspace::action::WorkspaceAction::MoveLocalWorkspaceTab {
                        tab_id: *tab_id,
                        delta: -1,
                    };
                let move_right_action =
                    crate::workspace::action::WorkspaceAction::MoveLocalWorkspaceTab {
                        tab_id: *tab_id,
                        delta: 1,
                    };
                let mut tab = Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(6.);
                tab.add_child(
                    Icon::Terminal
                        .to_warpui_icon(if active {
                            theme.font_color(theme.surface_1())
                        } else {
                            theme.sub_text_color(theme.surface_1())
                        })
                        .finish(),
                );
                tab.add_child(
                    Text::new_inline(
                        format!("Terminal {}", index + 1),
                        appearance.ui_font_family(),
                        12.,
                    )
                    .with_color(if active {
                        theme.font_color(theme.surface_1()).into()
                    } else {
                        theme.sub_text_color(theme.surface_1()).into()
                    })
                    .finish(),
                );
                tab.add_child(
                    EventHandler::new(
                        Text::new_inline("‹", appearance.ui_font_family(), 13.)
                            .with_color(theme.sub_text_color(theme.surface_1()).into())
                            .finish(),
                    )
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(move_left_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
                );
                tab.add_child(
                    EventHandler::new(
                        Text::new_inline("›", appearance.ui_font_family(), 13.)
                            .with_color(theme.sub_text_color(theme.surface_1()).into())
                            .finish(),
                    )
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(move_right_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
                );
                tab.add_child(
                    EventHandler::new(
                        Text::new_inline("×", appearance.ui_font_family(), 13.)
                            .with_color(theme.sub_text_color(theme.surface_1()).into())
                            .finish(),
                    )
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(close_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
                );
                tab_title.add_child(
                    EventHandler::new(
                        Container::new(tab.finish())
                            .with_background(if active {
                                theme.surface_2()
                            } else {
                                theme.surface_1()
                            })
                            .with_padding_left(10.)
                            .with_padding_right(10.)
                            .with_border(warpui::elements::Border::all(1.).with_border_fill(
                                if active {
                                    theme.outline()
                                } else {
                                    theme.surface_1()
                                },
                            ))
                            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                                warpui::elements::Radius::Pixels(4.),
                            ))
                            .finish(),
                    )
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(select_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
                );
            }
            let new_terminal_action =
                crate::workspace::action::WorkspaceAction::AddLocalWorkspaceTerminal;
            tab_title.add_child(
                EventHandler::new(
                    Container::new(
                        Text::new_inline("+", appearance.ui_font_family(), 16.)
                            .with_color(theme.font_color(theme.surface_1()).into())
                            .finish(),
                    )
                    .with_uniform_padding(6.)
                    .finish(),
                )
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(new_terminal_action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish(),
            );
            let assign_tab_action =
                crate::workspace::action::WorkspaceAction::AssignActiveTabToLocalWorkspaceProject;
            tab_title.add_child(
                EventHandler::new(
                    Container::new(
                        Text::new_inline("Vincular aba ativa", appearance.ui_font_family(), 11.)
                            .with_color(theme.sub_text_color(theme.surface_1()).into())
                            .finish(),
                    )
                    .with_uniform_padding(6.)
                    .finish(),
                )
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(assign_tab_action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish(),
            );
            let search = Text::new_inline(
                format!(
                    "{} · {}",
                    project.name,
                    model
                        .organization(project.organization_id)
                        .map(|org| org.name.as_str())
                        .unwrap_or("")
                ),
                appearance.ui_font_family(),
                12.,
            )
            .with_color(theme.sub_text_color(theme.surface_1()).into())
            .finish();
            tab_title.add_child(
                Shrinkable::new(1., Container::new(search).with_uniform_padding(6.).finish())
                    .finish(),
            );
            let project_bar = ConstrainedBox::new(
                Container::new(tab_title.finish())
                    .with_background(theme.surface_1())
                    .with_padding_left(8.)
                    .with_padding_right(8.)
                    .with_border(
                        warpui::elements::Border::bottom(1.).with_border_fill(theme.outline()),
                    )
                    .finish(),
            )
            .with_height(34.)
            .finish();
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
            Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_child(project_bar)
                .with_child(Shrinkable::new(1., project_content).finish())
                .finish()
        } else {
            home::render(model, appearance, unassigned_tabs)
        }
    } else {
        home::render(model, appearance, unassigned_tabs)
    };
    workspace.add_child(Shrinkable::new(1., main_content).finish());
    workspace.finish()
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
