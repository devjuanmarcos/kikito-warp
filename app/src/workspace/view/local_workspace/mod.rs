mod context_menu;
pub(super) mod create_dialog;
mod home;
pub(super) mod image_cropper;
mod org_rail;
mod project_sidebar;

use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_workspace::model::WorkspaceModel;
use warp_core::ui::Icon;
use warpui::ViewHandle;
use warpui::elements::{
    ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, MainAxisSize, ParentElement,
    Shrinkable, Text,
};

pub(crate) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    content: Box<dyn Element>,
    search_input: ViewHandle<EditorView>,
    search_query: &str,
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
                let active = project.active_tab == Some(*tab_id);
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
                tab_title.add_child(
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
                );
            }
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
            Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_child(project_bar)
                .with_child(Shrinkable::new(1., content).finish())
                .finish()
        } else {
            home::render(model, appearance)
        }
    } else {
        home::render(model, appearance)
    };
    workspace.add_child(Shrinkable::new(1., main_content).finish());
    workspace.finish()
}
