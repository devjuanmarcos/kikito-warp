use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_workspace::model::{ProjectPathStatus, WorkspaceModel};
use crate::workspace::action::WorkspaceAction;
use warp_core::ui::Icon;
use warpui::ViewHandle;
use warpui::elements::Element;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult, EventHandler, Flex,
    MainAxisAlignment, MainAxisSize, ParentElement, Shrinkable, Text,
};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;

const SIDEBAR_WIDTH: f32 = 248.;

pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    search_input: ViewHandle<EditorView>,
    search_query: &str,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let search_query = search_query.trim().to_lowercase();
    let mut sidebar = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    sidebar.add_child(
        Container::new(
            TextInput::new(search_input, UiComponentStyles::default().set_height(34.))
                .build()
                .finish(),
        )
        .with_padding_left(12.)
        .with_padding_right(12.)
        .with_padding_top(10.)
        .with_padding_bottom(4.)
        .finish(),
    );
    if let Some(error) = model.status_error() {
        sidebar.add_child(empty_state(error, appearance));
    }
    if let Some(organization_id) = model.selected_organization() {
        if let Some(organization) = model.organization(organization_id) {
            sidebar.add_child(
                Container::new(
                    Text::new_inline(organization.name.clone(), appearance.ui_font_family(), 12.)
                        .with_color(theme.font_color(theme.surface_1()).into())
                        .finish(),
                )
                .with_background(theme.surface_1())
                .with_padding_left(16.)
                .with_padding_right(12.)
                .with_padding_top(10.)
                .with_padding_bottom(10.)
                .finish(),
            );
            sidebar.add_child(section_title(
                "PROJETOS",
                organization.projects.len(),
                appearance,
            ));
            for project in model
                .projects_for_organization(organization_id)
                .filter(|project| {
                    search_query.is_empty()
                        || project.name.to_lowercase().contains(&search_query)
                        || project.path.to_lowercase().contains(&search_query)
                })
            {
                let project_id = project.id;
                let active = model.selected_project() == Some(project.id);
                let action = WorkspaceAction::SelectLocalWorkspaceProject(project.id);
                let mut title = Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(8.);
                title.add_child(
                    Icon::Folder
                        .to_warpui_icon(theme.sub_text_color(theme.surface_1()))
                        .finish(),
                );
                title.add_child(
                    Text::new_inline(project.name.clone(), appearance.ui_font_family(), 12.)
                        .with_color(theme.font_color(theme.surface_1()).into())
                        .finish(),
                );
                let menu_action = WorkspaceAction::OpenLocalWorkspaceProjectMenu(project.id);
                title.add_child(
                    EventHandler::new(
                        Text::new_inline("⋯", appearance.ui_font_family(), 14.).finish(),
                    )
                    .on_left_mouse_up(move |ctx, _, _| {
                        ctx.dispatch_typed_action(menu_action.clone());
                        DispatchEventResult::StopPropagation
                    })
                    .finish(),
                );
                let mut details = Flex::column().with_spacing(3.);
                details.add_child(title.finish());
                details.add_child(
                    Text::new_inline(project.path.clone(), appearance.ui_font_family(), 10.)
                        .with_color(theme.sub_text_color(theme.surface_1()).into())
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
                let row = Container::new(details.finish())
                    .with_background(if active {
                        theme.surface_2()
                    } else {
                        theme.surface_1()
                    })
                    .with_padding_left(18.)
                    .with_padding_right(8.)
                    .with_padding_top(7.)
                    .with_padding_bottom(7.)
                    .with_corner_radius(warpui::elements::CornerRadius::with_all(
                        warpui::elements::Radius::Pixels(4.),
                    ))
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
            let visible_projects = model
                .projects_for_organization(organization_id)
                .filter(|project| {
                    search_query.is_empty()
                        || project.name.to_lowercase().contains(&search_query)
                        || project.path.to_lowercase().contains(&search_query)
                })
                .count();
            if visible_projects == 0 {
                sidebar.add_child(empty_state(
                    empty_project_message(organization.projects.len(), &search_query),
                    appearance,
                ));
            }
        }
    } else {
        sidebar.add_child(section_title("PROJETOS", model.project_count(), appearance));
        for project in model.projects().filter(|project| {
            search_query.is_empty()
                || project.name.to_lowercase().contains(&search_query)
                || project.path.to_lowercase().contains(&search_query)
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
                project.name.to_lowercase().contains(&search_query)
                    || project.path.to_lowercase().contains(&search_query)
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
        .with_padding_top(8.)
        .with_padding_bottom(6.)
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
