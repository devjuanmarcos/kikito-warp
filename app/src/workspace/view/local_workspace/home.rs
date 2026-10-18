use crate::appearance::Appearance;
use crate::local_workspace::model::{ProjectPathStatus, WorkspaceModel};
use crate::workspace::action::WorkspaceAction;
use warpui::elements::Element;
use warpui::elements::{
    Container, CrossAxisAlignment, DispatchEventResult, EventHandler, Flex, MainAxisSize,
    ParentElement, Shrinkable, Text,
};

pub(super) fn render(model: &WorkspaceModel, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut content = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(8.);
    content.add_child(
        Text::new_inline("Início".to_owned(), appearance.ui_font_family(), 18.)
            .with_color(theme.font_color(theme.background()).into())
            .finish(),
    );
    content.add_child(
        Text::new_inline(
            "Seus projetos recentes".to_owned(),
            appearance.ui_font_family(),
            12.,
        )
        .with_color(theme.sub_text_color(theme.background()).into())
        .finish(),
    );
    if let Some(error) = model.status_error() {
        content.add_child(
            Text::new_inline(error.to_owned(), appearance.ui_font_family(), 12.)
                .with_color(theme.ui_error_color().into())
                .finish(),
        );
    }
    for project in model.recent_projects() {
        let action = WorkspaceAction::SelectLocalWorkspaceProject(project.id);
        let mut card = Flex::column().with_spacing(4.);
        card.add_child(
            Text::new_inline(project.name.clone(), appearance.ui_font_family(), 13.)
                .with_color(theme.font_color(theme.background()).into())
                .finish(),
        );
        card.add_child(
            Text::new_inline(project.path.clone(), appearance.ui_font_family(), 11.)
                .with_color(theme.sub_text_color(theme.background()).into())
                .finish(),
        );
        let path_status = model.project_path_status(project.id);
        let (path_label, path_color) = match path_status {
            ProjectPathStatus::Checking => (
                format!("Verificando pasta · {} terminais", project.tabs.len()),
                theme.sub_text_color(theme.background()).into(),
            ),
            ProjectPathStatus::Available => (
                format!("{} terminais", project.tabs.len()),
                theme.sub_text_color(theme.background()).into(),
            ),
            ProjectPathStatus::Missing => (
                format!("Pasta não encontrada · {} terminais", project.tabs.len()),
                theme.ui_error_color().into(),
            ),
            ProjectPathStatus::NotDirectory => (
                format!(
                    "O caminho não é uma pasta · {} terminais",
                    project.tabs.len()
                ),
                theme.ui_error_color().into(),
            ),
            ProjectPathStatus::Inaccessible => (
                format!("Sem acesso à pasta · {} terminais", project.tabs.len()),
                theme.ui_error_color().into(),
            ),
        };
        card.add_child(
            Text::new_inline(path_label, appearance.ui_font_family(), 10.)
                .with_color(path_color)
                .finish(),
        );
        content.add_child(
            EventHandler::new(
                Container::new(card.finish())
                    .with_background(theme.surface_1())
                    .with_uniform_padding(12.)
                    .with_corner_radius(warpui::elements::CornerRadius::with_all(
                        warpui::elements::Radius::Pixels(8.),
                    ))
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );
    }
    if model.project_count() == 0 {
        content.add_child(
            Text::new_inline(
                "Crie uma organização para começar a organizar seus projetos.".to_owned(),
                appearance.ui_font_family(),
                12.,
            )
            .with_color(theme.sub_text_color(theme.background()).into())
            .finish(),
        );
    }
    Container::new(Shrinkable::new(1., content.finish()).finish())
        .with_uniform_padding(24.)
        .finish()
}
