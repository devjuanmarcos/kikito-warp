mod context_menu;
pub(super) mod create_dialog;
mod dialog_ui;
mod home;
pub(super) mod image_cropper;
mod org_rail;
mod project_sidebar;
mod project_tabs;
mod source_control;
mod ws_style;
pub(crate) use source_control::{
    DiffPreview, GitConfirmation, GitPanelSection, GitPanelState, GitRemoteAction,
    RepositoryEntry, SourceControl,
};

/// Window border (both sides), organization rail and project sidebar, in pixels.
const WINDOW_BORDERS_WIDTH: f32 = 2.;
const RAIL_TOTAL_WIDTH: f32 = 65.;
const SIDEBAR_TOTAL_WIDTH: f32 = 249.;

/// Width of the source control panel for a window `window_width` pixels wide.
pub(crate) fn source_control_width(model: &WorkspaceModel, window_width: f32) -> f32 {
    let sidebar = if should_render_project_sidebar(model) {
        SIDEBAR_TOTAL_WIDTH
    } else {
        0.
    };
    source_control::panel_width(window_width - WINDOW_BORDERS_WIDTH - RAIL_TOTAL_WIDTH - sidebar)
}

use crate::workspace::action::WorkspaceAction;
use warpui::ViewHandle;
use warpui::elements::{
    Align, ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment, Element, Empty,
    Flex, MainAxisSize, ParentElement, Shrinkable, Stack, Text,
};

pub(crate) use home::{HomeAsk, ask_placeholder, ask_results};

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
use crate::local_git::RepositoryStatus;
use crate::local_workspace::model::WorkspaceModel;
use warp_core::ui::Icon;

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

    // Tabs outside every project are hidden stand-ins (e.g. the one that keeps the window open
    // after a project's last tab closes), so activating one never leaves the current view.
    match (model.selected_project(), target_project) {
        (Some(selected), Some((_, target))) if selected == target => {
            TabActivationRoute::KeepSelection
        }
        (_, Some((organization_id, project_id))) => TabActivationRoute::Project {
            organization_id,
            project_id,
        },
        (_, None) => TabActivationRoute::KeepSelection,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    content: Box<dyn Element>,
    search_input: ViewHandle<EditorView>,
    search_focused: bool,
    new_tab_keys: &[String],
    search_query: &str,
    project_sidebar_scroll: ClippedScrollStateHandle,
    available_workspace_tabs: &[String],
    workspace_tab_titles: &std::collections::HashMap<String, String>,
    home_scroll: ClippedScrollStateHandle,
    home_ask: home::HomeAsk<'_>,
    git_status: Option<&RepositoryStatus>,
    source_control: Option<source_control::SourceControl<'_>>,
    tabs_in_sidebar: bool,
    rename_editor: Option<(i32, Box<dyn Element>)>,
    settings_open: bool,
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
            search_focused,
            search_query,
            project_sidebar_scroll,
            git_status,
        ));
    }
    let main_content = if settings_open {
        Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_child(project_tabs::render_settings_bar(appearance))
            .with_child(Shrinkable::new(1., content).finish())
            .finish()
    } else if let Some(project_id) = model.selected_project() {
        if model.project(project_id).is_some() {
            let theme = appearance.theme();
            let project_bar = project_tabs::render(
                model,
                appearance,
                project_id,
                available_workspace_tabs,
                workspace_tab_titles,
                source_control.is_some(),
                tabs_in_sidebar,
                rename_editor,
                git_status.map_or(0, |status| status.changed_path_count()),
            )
            .unwrap_or_else(|| Flex::row().finish());
            let has_open_project_tab =
                project_has_open_terminal(model, project_id, available_workspace_tabs);
            let project_content: Box<dyn Element> = if !has_open_project_tab {
                let keys: Vec<&str> = new_tab_keys.iter().map(String::as_str).collect();
                let mut empty = Flex::column()
                    .with_main_axis_size(MainAxisSize::Min)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(12.);
                empty.add_child(
                    Text::new_inline(
                        "Projeto quietinho. Abro um terminal?",
                        appearance.ui_font_family(),
                        14.,
                    )
                    .with_color(theme.sub_text_color(theme.background()).into())
                    .finish(),
                );
                empty.add_child(dialog_ui::dialog_button(
                    appearance,
                    Some(Icon::Plus),
                    "Nova aba",
                    &keys,
                    dialog_ui::ButtonVariant::Secondary,
                    true,
                    ("empty-project-new-tab", 0),
                    move |ctx, position| {
                        ctx.dispatch_typed_action(WorkspaceAction::OpenLocalWorkspaceNewTabMenu {
                            project_id,
                            position: Some(position),
                        })
                    },
                ));
                Container::new(Align::new(empty.finish()).finish())
                    .with_background(theme.background())
                    .finish()
            } else {
                content
            };
            // The kit opens the source control panel beside the project content, under the
            // project bar.
            let body = match source_control {
                Some(source_control) => Flex::row()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .with_child(Shrinkable::new(1., project_content).finish())
                    .with_child(source_control::render(source_control, appearance))
                    .finish(),
                None => project_content,
            };
            Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_child(project_bar)
                .with_child(Shrinkable::new(1., body).finish())
                .finish()
        } else {
            home::render(
                model,
                appearance,
                workspace_tab_titles,
                home_scroll,
                &home_ask,
            )
        }
    } else if should_render_empty_organization_state(model) {
        empty_organization_state(model, appearance)
    } else {
        home::render(
            model,
            appearance,
            workspace_tab_titles,
            home_scroll,
            &home_ask,
        )
    };
    workspace.add_child(Shrinkable::new(1., main_content).finish());
    workspace.finish()
}

fn empty_organization_state(model: &WorkspaceModel, appearance: &Appearance) -> Box<dyn Element> {
    let Some(organization) = model
        .selected_organization()
        .and_then(|organization_id| model.organization(organization_id))
    else {
        return Empty::new().finish();
    };
    let organization_id = organization.id;
    let theme = appearance.theme();
    let mut empty = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
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
    empty.add_child(dialog_ui::dialog_button(
        appearance,
        Some(Icon::Plus),
        "Novo projeto",
        &[],
        dialog_ui::ButtonVariant::Secondary,
        true,
        ("empty-organization-new-project", 0),
        move |ctx, _| {
            ctx.dispatch_typed_action(WorkspaceAction::CreateLocalWorkspaceProject(
                organization_id,
            ))
        },
    ));
    Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(project_tabs::render_title_bar(
            appearance,
            &organization.name,
        ))
        .with_child(
            Shrinkable::new(
                1.,
                Container::new(Align::new(empty.finish()).finish())
                    .with_background(theme.background())
                    .finish(),
            )
            .finish(),
        )
        .finish()
}

/// Branch checked out in the Git work tree at `path`, read from `HEAD` without spawning Git so
/// every sidebar row can show one. `None` when the folder is not a work tree.
pub(crate) async fn read_checked_out_branch(path: &str) -> Option<String> {
    let dot_git = std::path::Path::new(path).join(".git");
    let git_dir = match async_fs::read_to_string(&dot_git).await {
        Ok(pointer) => {
            let target = pointer.trim().strip_prefix("gitdir:")?.trim().to_owned();
            std::path::Path::new(path).join(target)
        }
        Err(_) => dot_git,
    };
    let head = async_fs::read_to_string(git_dir.join("HEAD")).await.ok()?;
    branch_from_head(&head)
}

fn branch_from_head(head: &str) -> Option<String> {
    let head = head.trim();
    if head.is_empty() {
        return None;
    }
    Some(match head.strip_prefix("ref: ") {
        Some(reference) => reference
            .strip_prefix("refs/heads/")
            .unwrap_or(reference)
            .to_owned(),
        None => head.chars().take(7).collect(),
    })
}

/// Splits a displayed keystroke ("⇧⌘T" on macOS, "Ctrl Shift T" elsewhere) into keycap labels.
pub(crate) fn keycaps_from_display(displayed: &str) -> Vec<String> {
    if displayed.contains(' ') {
        return displayed.split_whitespace().map(str::to_owned).collect();
    }
    let mut caps = Vec::new();
    let mut key = String::new();
    for c in displayed.chars() {
        if matches!(c, '⌃' | '⌥' | '⇧' | '⌘') && key.is_empty() {
            caps.push(c.to_string());
        } else {
            key.push(c);
        }
    }
    if !key.is_empty() {
        caps.push(key);
    }
    caps
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;

/// Context menu of a project tab.
pub(crate) fn tab_menu(
    model: &WorkspaceModel,
    tab_id: i32,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let (can_move_left, can_move_right) = model
        .tab_owner(tab_id)
        .and_then(|project_id| model.project(project_id))
        .and_then(|project| {
            let index = project.tabs.iter().position(|id| *id == tab_id)?;
            Some((index > 0, index + 1 < project.tabs.len()))
        })
        .unwrap_or_default();
    let mut items = vec![context_menu::MenuItem {
        label: "Renomear aba",
        icon: Icon::Pencil,
        action: WorkspaceAction::RenameLocalWorkspaceTab(tab_id),
        danger: false,
        separator_before: false,
        checked: false,
    }];
    if can_move_left {
        items.push(context_menu::MenuItem {
            label: "Mover para a esquerda",
            icon: Icon::ArrowLeft,
            action: WorkspaceAction::MoveLocalWorkspaceTab { tab_id, delta: -1 },
            danger: false,
            separator_before: false,
            checked: false,
        });
    }
    if can_move_right {
        items.push(context_menu::MenuItem {
            label: "Mover para a direita",
            icon: Icon::ArrowRight,
            action: WorkspaceAction::MoveLocalWorkspaceTab { tab_id, delta: 1 },
            danger: false,
            separator_before: false,
            checked: false,
        });
    }
    items.push(context_menu::MenuItem {
        label: "Fechar aba",
        icon: Icon::X,
        action: WorkspaceAction::CloseLocalWorkspaceTab(tab_id),
        danger: true,
        separator_before: true,
        checked: false,
    });
    context_menu::menu(
        items,
        None,
        LOCAL_MENU_WIDTH,
        WorkspaceAction::DismissLocalWorkspaceMenus,
        appearance,
    )
}

/// Menu that picks where project tabs are listed.
pub(crate) fn tabs_layout_menu(tabs_in_sidebar: bool, appearance: &Appearance) -> Box<dyn Element> {
    context_menu::menu(
        vec![
            context_menu::MenuItem {
                label: "Abas no cabeçalho",
                icon: Icon::LayoutAlt01,
                action: WorkspaceAction::SetLocalWorkspaceTabsInSidebar(false),
                danger: false,
                separator_before: false,
                checked: !tabs_in_sidebar,
            },
            context_menu::MenuItem {
                label: "Abas na barra lateral",
                icon: Icon::LeftSidebarOpen,
                action: WorkspaceAction::SetLocalWorkspaceTabsInSidebar(true),
                danger: false,
                separator_before: false,
                checked: tabs_in_sidebar,
            },
        ],
        None,
        LOCAL_MENU_WIDTH,
        WorkspaceAction::DismissLocalWorkspaceMenus,
        appearance,
    )
}

const LOCAL_MENU_WIDTH: f32 = 220.;

/// Notifications and settings buttons shown left of the window controls on every screen.
pub(crate) fn window_buttons(appearance: &Appearance, has_unread: bool) -> Box<dyn Element> {
    let mut notifications = Stack::new().with_child(project_tabs::header_button(
        appearance,
        Icon::Inbox,
        ("window-notifications", 0),
        "Notificações",
        |ctx, _| {
            ctx.dispatch_typed_action(WorkspaceAction::ToggleNotificationMailbox {
                select_first: false,
            })
        },
    ));
    if has_unread {
        notifications.add_positioned_child(
            ConstrainedBox::new(
                warpui::elements::Rect::new()
                    .with_background(appearance.theme().accent())
                    .with_corner_radius(warpui::elements::CornerRadius::with_all(
                        warpui::elements::Radius::Percentage(50.),
                    ))
                    .finish(),
            )
            .with_width(6.)
            .with_height(6.)
            .finish(),
            warpui::elements::OffsetPositioning::offset_from_parent(
                pathfinder_geometry::vector::vec2f(-3., 3.),
                warpui::elements::ParentOffsetBounds::WindowByPosition,
                warpui::elements::ParentAnchor::TopRight,
                warpui::elements::ChildAnchor::TopRight,
            ),
        );
    }
    let buttons = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.)
        .with_child(notifications.finish())
        .with_child(project_tabs::header_button(
            appearance,
            Icon::Gear,
            ("window-settings", 0),
            "Configurações",
            |ctx, _| ctx.dispatch_typed_action(WorkspaceAction::ShowSettings),
        ))
        .finish();
    ConstrainedBox::new(buttons)
        .with_width(project_tabs::WINDOW_BUTTONS_WIDTH)
        .finish()
}

pub(crate) use project_tabs::TABS_LAYOUT_BUTTON_POSITION_ID;

/// Horizontal offset of `window_buttons` from the window's right edge.
pub(crate) const WINDOW_BUTTONS_RIGHT_INSET: f32 = project_tabs::WINDOW_CONTROLS_INSET + 8.;

pub(super) fn project_row_position_id(
    project_id: crate::local_workspace::model::ProjectId,
) -> String {
    format!("local_workspace:project_row:{project_id}")
}

pub(super) fn organization_tile_position_id(
    organization_id: crate::local_workspace::model::OrganizationId,
) -> String {
    format!("local_workspace:organization_tile:{organization_id}")
}

/// The open organization or project context menu and where it sits. It renders at the window's
/// root so its outside-click dismissal sees clicks anywhere in the window.
pub(crate) fn context_menu_overlay(
    model: &WorkspaceModel,
    appearance: &Appearance,
) -> Option<(Box<dyn Element>, warpui::elements::OffsetPositioning)> {
    use crate::local_workspace::model::ContextMenuTarget;
    use warpui::elements::{
        ChildAnchor, OffsetPositioning, PositionedElementAnchor, PositionedElementOffsetBounds,
    };
    let target = model.context_menu()?;
    let menu = context_menu::render(
        target,
        model.context_menu_index(),
        model.context_menu_keyboard_active(),
        appearance,
    );
    let positioning = match target {
        ContextMenuTarget::Organization(id) => {
            OffsetPositioning::offset_from_save_position_element(
                organization_tile_position_id(id),
                pathfinder_geometry::vector::vec2f(
                    org_rail::RAIL_WIDTH / 2.,
                    org_rail::ORGANIZATION_TILE_SIZE / 2.,
                ),
                PositionedElementOffsetBounds::WindowByPosition,
                PositionedElementAnchor::TopLeft,
                ChildAnchor::TopLeft,
            )
        }
        ContextMenuTarget::Project(id) => OffsetPositioning::offset_from_save_position_element(
            project_row_position_id(id),
            pathfinder_geometry::vector::vec2f(-38., 3.),
            PositionedElementOffsetBounds::WindowByPosition,
            PositionedElementAnchor::TopRight,
            ChildAnchor::TopRight,
        ),
    };
    Some((menu, positioning))
}

/// Context menu of a changed file in the Git panel.
pub(crate) fn git_file_menu(
    repository: &std::path::Path,
    path: &str,
    untracked: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let absolute = repository.join(path).to_string_lossy().into_owned();
    context_menu::menu(
        vec![
            context_menu::MenuItem {
                label: "Copiar caminho",
                icon: Icon::Copy,
                action: WorkspaceAction::CopyLocalGitText(absolute),
                danger: false,
                separator_before: false,
                checked: false,
            },
            context_menu::MenuItem {
                label: "Copiar caminho relativo",
                icon: Icon::Copy,
                action: WorkspaceAction::CopyLocalGitText(path.to_owned()),
                danger: false,
                separator_before: false,
                checked: false,
            },
            context_menu::MenuItem {
                label: "Adicionar ao .gitignore",
                icon: Icon::Plus,
                action: WorkspaceAction::IgnoreLocalGitPath(path.to_owned()),
                danger: false,
                separator_before: false,
                checked: false,
            },
            context_menu::MenuItem {
                label: "Descartar alterações",
                icon: Icon::Retry,
                action: WorkspaceAction::RequestDiscardLocalGitPath {
                    path: path.to_owned(),
                    untracked,
                },
                danger: true,
                separator_before: true,
                checked: false,
            },
        ],
        None,
        LOCAL_MENU_WIDTH,
        WorkspaceAction::DismissLocalGitFileMenu,
        appearance,
    )
}
