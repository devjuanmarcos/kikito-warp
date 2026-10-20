use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{HeadState, RepositoryStatus};
use crate::local_workspace::model::{ProjectPathStatus, WorkspaceModel};
use crate::workspace::action::WorkspaceAction;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::Icon;
use super::ws_style::{
    TooltipSide, fg_overlay, mouse_state, naked_icon_button, radius, secondary_button, sized_icon,
};
use warpui::platform::Cursor;
use warpui::ViewHandle;
use warpui::fonts::{Properties, Weight};
use warpui::text_layout::ClipConfig;
use warpui::elements::Element;
use warpui::elements::new_scrollable::{NewScrollable, ScrollableAppearance, SingleAxisConfig};
use warpui::elements::{
    Border, ChildAnchor, ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, EventHandler, Expanded, Fill, Flex, Hoverable, MainAxisAlignment, MainAxisSize,
    OffsetPositioning, ParentAnchor, ParentElement, ParentOffsetBounds, ScrollbarWidth, Shrinkable,
    Stack, Text,
};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;

const SIDEBAR_WIDTH: f32 = 248.;

pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    search_input: ViewHandle<EditorView>,
    search_query: &str,
    scroll_state: ClippedScrollStateHandle,
    git_status_summary: Option<&str>,
    git_status_error: Option<&str>,
    git_status: Option<&RepositoryStatus>,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let search_query = search_query.trim().to_lowercase();
    let mut sidebar = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    if let Some(error) = model.status_error() {
        sidebar.add_child(empty_state(error, appearance));
    }
    if let Some(organization_id) = model.selected_organization() {
        if let Some(organization) = model.organization(organization_id) {
            let mut header = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(8.);
            header.add_child(
                ConstrainedBox::new(
                    Icon::CircleFilled
                        .to_warpui_icon(super::org_rail::organization_accent(
                            theme,
                            &organization.accent_color,
                        ))
                        .finish(),
                )
                .with_width(8.)
                .with_height(8.)
                .finish(),
            );
            header.add_child(
                Expanded::new(
                    1.,
                    Text::new_inline(
                        organization.name.clone(),
                        appearance.ui_font_family(),
                        12.,
                    )
                    .with_style(Properties::default().weight(Weight::Semibold))
                    .with_color(theme.main_text_color(theme.surface_1()).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
                )
                .finish(),
            );
            let create_project_action =
                WorkspaceAction::CreateLocalWorkspaceProject(organization_id);
            header.add_child(naked_icon_button(
                appearance,
                Icon::Plus,
                theme.foreground(),
                None,
                create_project_action,
                ("header-new-project", 0),
                Some(("Novo projeto", TooltipSide::BelowRight)),
            ));
            sidebar.add_child(
                ConstrainedBox::new(
                    Container::new(header.finish())
                .with_background(theme.surface_1())
                .with_padding_top(1.)
                .with_padding_left(16.)
                        .with_padding_right(8.)
                        .with_border(
                            Border::bottom(1.).with_border_fill(theme.outline()),
                        )
                        .finish(),
                )
                .with_height(34.)
                .finish(),
            );
            let search_icon = Icon::Search
                .to_warpui_icon(theme.sub_text_color(theme.surface_1()))
                .finish();
            let mut search_row = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(8.);
            search_row.add_child(
                ConstrainedBox::new(search_icon)
                    .with_width(14.)
                    .with_height(14.)
                    .finish(),
            );
            search_row.add_child(Shrinkable::new(
                1.,
                TextInput::new(search_input, UiComponentStyles::default().set_height(28.))
                    .build()
                    .finish(),
            ).finish());
            sidebar.add_child(
                Container::new(search_row.finish())
                    .with_background(theme.surface_2())
                    .with_padding_left(8.)
                    .with_padding_right(8.)
                    .with_margin_top(10.)
                    .with_margin_left(8.)
                    .with_margin_right(8.)
                    .with_margin_bottom(6.)
                    .with_corner_radius(warpui::elements::CornerRadius::with_all(
                        warpui::elements::Radius::Pixels(4.),
                    ))
                    .finish(),
            );
            sidebar.add_child(section_title(
                "PROJETOS",
                organization.projects.len(),
                appearance,
            ));
            let mut project_list =
                Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
            for project in model
                .projects_for_organization(organization_id)
                .filter(|project| {
                    project_matches_search(&project.name, &project.path, &search_query)
                })
            {
                let project_id = project.id;
                let active = model.selected_project() == Some(project.id);
                let row_state = mouse_state("project-row", project_id as i64);
                let hovered = row_state.lock().unwrap().is_mouse_over_element();
                let action = WorkspaceAction::SelectLocalWorkspaceProject(project.id);
                let row_background = theme.surface_1();
                let name_color = if active {
                    theme.main_text_color(row_background)
                } else {
                    theme.sub_text_color(row_background)
                };
                let hint_color = theme.hint_text_color(row_background);
                let mut title = Flex::row()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(8.);
                title.add_child(sized_icon(Icon::Folder, name_color, 14.));
                let mut name = Text::new_inline(
                    project.name.clone(),
                    appearance.ui_font_family(),
                    12.,
                )
                .with_color(name_color.into())
                .soft_wrap(false)
                .with_clip(ClipConfig::ellipsis());
                if active {
                    name = name.with_style(Properties::default().weight(Weight::Semibold));
                }
                title.add_child(Expanded::new(1., name.finish()).finish());
                if hovered {
                    let mut actions = Flex::row().with_spacing(2.);
                    actions.add_child(naked_icon_button(
                        appearance,
                        Icon::Plus,
                        theme.foreground(),
                        None,
                        WorkspaceAction::AddLocalWorkspaceTerminalToProject(project_id),
                        ("row-plus", project_id as i64),
                        Some(("Nova aba", TooltipSide::AboveRight)),
                    ));
                    actions.add_child(naked_icon_button(
                        appearance,
                        Icon::DotsHorizontal,
                        theme.foreground(),
                        None,
                        WorkspaceAction::OpenLocalWorkspaceProjectMenu(project_id),
                        ("row-more", project_id as i64),
                        None,
                    ));
                    title.add_child(
                        Container::new(actions.finish())
                            .with_margin_top(-5.)
                            .with_margin_bottom(-5.)
                            .with_margin_right(-2.)
                            .finish(),
                    );
                } else {
                    title.add_child(
                        Text::new_inline(
                            project.tabs.len().to_string(),
                            appearance.ui_font_family(),
                            10.,
                        )
                        .with_color(hint_color.into())
                        .finish(),
                    );
                }
                let mut details = Flex::column().with_spacing(2.);
                details.add_child(title.finish());
                let branch = if active {
                    git_status.and_then(|status| match &status.head {
                        HeadState::Branch(name) => Some(name.as_str()),
                        HeadState::Detached => Some("HEAD separado"),
                        HeadState::Unborn => Some("sem branch"),
                    })
                } else {
                    None
                };
                let mut detail_line = Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(4.);
                if branch.is_some() {
                    detail_line.add_child(sized_icon(Icon::GitBranch, hint_color, 10.));
                }
                detail_line.add_child(
                    Shrinkable::new(
                        1.,
                        Text::new_inline(
                            branch.unwrap_or(&project.path).to_owned(),
                            appearance.monospace_font_family(),
                            10.,
                        )
                        .with_color(hint_color.into())
                        .soft_wrap(false)
                        .with_clip(ClipConfig::ellipsis())
                        .finish(),
                    )
                    .finish(),
                );
                details.add_child(
                    Container::new(detail_line.finish())
                        .with_padding_left(22.)
                        .finish(),
                );
                let path_status = model.project_path_status(project.id);
                if let Some(message) = path_status_message(path_status) {
                    details.add_child(
                        Text::new_inline(message, appearance.ui_font_family(), 10.)
                            .with_color(if path_status == ProjectPathStatus::Checking {
                                theme.sub_text_color(theme.surface_1()).into()
                            } else {
                                theme.ui_error_color().into()
                            })
                            .finish(),
                    );
                }
                if active {
                    if let Some(summary) = git_status_summary {
                        details.add_child(
                            Text::new_inline(summary.to_owned(), appearance.ui_font_family(), 10.)
                                .with_color(theme.sub_text_color(theme.surface_1()).into())
                                .finish(),
                        );
                    }
                    if let Some(error) = git_status_error {
                        details.add_child(
                            Text::new_inline(error.to_owned(), appearance.ui_font_family(), 10.)
                                .with_color(theme.ui_error_color().into())
                                .finish(),
                        );
                    }
                }
                let row = Container::new(details.finish())
                    .with_background(if active {
                        fg_overlay(theme, 10)
                    } else if hovered {
                        fg_overlay(theme, 5)
                    } else {
                        fg_overlay(theme, 0)
                    })
                    .with_border(if active {
                        Border::all(1.).with_border_fill(fg_overlay(theme, 15))
                    } else {
                        Border::all(1.)
                    })
                    .with_padding_left(10.)
                    .with_padding_right(8.)
                    .with_padding_top(7.)
                    .with_padding_bottom(7.)
                    .with_margin_left(8.)
                    .with_margin_right(8.)
                    .with_corner_radius(radius(4.))
                    .finish();
                let row = Hoverable::new(row_state, |_| row)
                    .with_cursor(Cursor::PointingHand)
                    .with_defer_events_to_children()
                    .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
                    .on_right_click(move |ctx, _, _| {
                        ctx.dispatch_typed_action(WorkspaceAction::OpenLocalWorkspaceProjectMenu(
                            project_id,
                        ))
                    })
                    .finish();
                let mut project_item = Stack::new().with_child(row);
                if model.context_menu()
                    == Some(crate::local_workspace::model::ContextMenuTarget::Project(
                        project.id,
                    ))
                {
                    project_item.add_positioned_overlay_child(
                        super::context_menu::render(
                            crate::local_workspace::model::ContextMenuTarget::Project(project.id),
                            model.context_menu_index(),
                            appearance,
                        ),
                        OffsetPositioning::offset_from_parent(
                            vec2f(-8., 0.),
                            ParentOffsetBounds::ParentByPosition,
                            ParentAnchor::TopRight,
                            ChildAnchor::TopRight,
                        ),
                    );
                }
                project_list.add_child(project_item.finish());
            }
            let visible_projects = model
                .projects_for_organization(organization_id)
                .filter(|project| {
                    project_matches_search(&project.name, &project.path, &search_query)
                })
                .count();
            if visible_projects == 0 {
                project_list.add_child(empty_state(
                    empty_project_message(organization.projects.len(), &search_query),
                    appearance,
                ));
            }
            let project_scroll = NewScrollable::vertical(
                SingleAxisConfig::Clipped {
                    handle: scroll_state,
                    child: project_list.finish(),
                },
                theme.sub_text_color(theme.surface_1()).into(),
                theme.font_color(theme.surface_1()).into(),
                Fill::None,
            )
            .with_vertical_scrollbar(ScrollableAppearance::new(ScrollbarWidth::Auto, true))
            .finish();
            sidebar.add_child(Shrinkable::new(1., project_scroll).finish());
            let create_project_action =
                WorkspaceAction::CreateLocalWorkspaceProject(organization_id);
            let mut create_label = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::Center)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(2.);
            create_label.add_child(sized_icon(Icon::Plus, theme.foreground(), 14.));
            create_label.add_child(
                Text::new_inline("Novo projeto", appearance.ui_font_family(), 12.)
                    .with_style(Properties::default().weight(Weight::Semibold))
                    .with_color(theme.foreground().into())
                    .finish(),
            );
            let create_project_button = ConstrainedBox::new(secondary_button(
                theme,
                Container::new(create_label.finish())
                    .with_padding_bottom(2.)
                    .finish(),
                24.,
                create_project_action,
                ("footer-new-project", 0),
            ))
            .with_width(SIDEBAR_WIDTH - 17.)
            .finish();
            sidebar.add_child(
                Container::new(create_project_button)
                    .with_padding_left(8.)
                    .with_padding_right(8.)
                    .with_padding_top(8.)
                    .with_padding_bottom(8.)
                    .with_border(Border::top(1.).with_border_fill(theme.outline()))
                    .finish(),
            );
        }
    } else {
        sidebar.add_child(section_title("PROJETOS", model.project_count(), appearance));
        for project in model.projects().filter(|project| {
            project_matches_search(&project.name, &project.path, &search_query)
        }) {
            let project_id = project.id;
            let action = WorkspaceAction::SelectLocalWorkspaceProject(project.id);
            let mut row = Flex::column().with_spacing(3.);
            let mut title = Flex::row()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween);
            title.add_child(
                Text::new_inline(project.name.clone(), appearance.ui_font_family(), 12.)
                    .with_color(theme.font_color(theme.surface_1()).into())
                    .finish(),
            );
            let menu_action = WorkspaceAction::OpenLocalWorkspaceProjectMenu(project.id);
            title.add_child(
                EventHandler::new(Text::new_inline("⋯", appearance.ui_font_family(), 14.).finish())
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(menu_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
            );
            row.add_child(title.finish());
            row.add_child(
                Text::new_inline(project.path.clone(), appearance.ui_font_family(), 10.)
                    .with_color(theme.sub_text_color(theme.surface_1()).into())
                    .finish(),
            );
            let path_status = model.project_path_status(project.id);
            if let Some(message) = path_status_message(path_status) {
                row.add_child(
                    Text::new_inline(message, appearance.ui_font_family(), 10.)
                        .with_color(if path_status == ProjectPathStatus::Checking {
                            theme.sub_text_color(theme.surface_1()).into()
                        } else {
                            theme.ui_error_color().into()
                        })
                        .finish(),
                );
            }
            let row = Container::new(row.finish())
                .with_background(theme.surface_1())
                .with_padding_left(18.)
                .with_padding_right(8.)
                .with_padding_top(7.)
                .with_padding_bottom(7.)
                .finish();
            let mut project_item = Flex::column()
                .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                .with_child(
                    EventHandler::new(row)
                        .on_left_mouse_up(move |ctx, _, _| {
                            ctx.dispatch_typed_action(action.clone());
                            DispatchEventResult::StopPropagation
                        })
                        .on_right_mouse_down(move |ctx, _, _, _| {
                            ctx.dispatch_typed_action(
                                WorkspaceAction::OpenLocalWorkspaceProjectMenu(project_id),
                            );
                            DispatchEventResult::StopPropagation
                        })
                        .finish(),
                );
            if model.context_menu()
                == Some(crate::local_workspace::model::ContextMenuTarget::Project(
                    project.id,
                ))
            {
                project_item.add_child(super::context_menu::render(
                    crate::local_workspace::model::ContextMenuTarget::Project(project.id),
                    model.context_menu_index(),
                    appearance,
                ));
            }
            sidebar.add_child(project_item.finish());
        }
        if !search_query.is_empty()
            && !model.projects().any(|project| {
                project_matches_search(&project.name, &project.path, &search_query)
            })
        {
            sidebar.add_child(empty_state("Nenhum projeto encontrado.", appearance));
        }
    }
    ConstrainedBox::new(
        Container::new(Shrinkable::new(1., sidebar.finish()).finish())
            .with_background(theme.surface_1())
            .with_border(Border::right(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_width(SIDEBAR_WIDTH)
    .finish()
}

fn section_title(label: &str, count: usize, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Center);
    row.add_child(
        Text::new_inline(label.to_owned(), appearance.ui_font_family(), 10.)
            .with_color(theme.sub_text_color(theme.surface_1()).into())
            .finish(),
    );
    row.add_child(
        Text::new_inline(count.to_string(), appearance.ui_font_family(), 10.)
            .with_color(theme.sub_text_color(theme.surface_1()).into())
            .finish(),
    );
    Container::new(row.finish())
        .with_padding_left(16.)
        .with_padding_right(16.)
        .with_padding_top(6.)
        .with_padding_bottom(4.)
        .finish()
}

fn empty_state(message: &str, appearance: &Appearance) -> Box<dyn Element> {
    Container::new(
        Text::new_inline(message.to_owned(), appearance.ui_font_family(), 12.)
            .with_color(
                appearance
                    .theme()
                    .sub_text_color(appearance.theme().surface_1())
                    .into(),
            )
            .finish(),
    )
    .with_padding_left(16.)
    .with_padding_right(16.)
    .with_padding_top(12.)
    .finish()
}

fn empty_project_message(project_count: usize, search_query: &str) -> &'static str {
    if project_count > 0 && !search_query.is_empty() {
        "Nenhum projeto corresponde à busca."
    } else {
        "Ainda não há projetos nesta organização."
    }
}

fn project_matches_search(name: &str, path: &str, search_query: &str) -> bool {
    let search_query = search_query.to_lowercase();
    search_query.is_empty()
        || name.to_lowercase().contains(&search_query)
        || path.to_lowercase().contains(&search_query)
}

fn path_status_message(status: ProjectPathStatus) -> Option<&'static str> {
    match status {
        ProjectPathStatus::Checking => Some("Verificando pasta…"),
        ProjectPathStatus::Available => None,
        ProjectPathStatus::Missing => Some("Pasta não encontrada"),
        ProjectPathStatus::NotDirectory => Some("O caminho não é uma pasta"),
        ProjectPathStatus::Inaccessible => Some("Sem acesso à pasta"),
    }
}

#[cfg(test)]
#[path = "project_sidebar_tests.rs"]
mod tests;
