use std::collections::HashMap;

use warp_core::ui::Icon;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult, Element,
    EventHandler, Expanded, Flex, Hoverable, MainAxisSize, ParentElement, Shrinkable, Stack, Text,
};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;

use super::ws_style::{
    TooltipSide, fg_overlay, mouse_state, naked_icon_button, radius, sized_icon,
};
use crate::appearance::Appearance;
use crate::local_workspace::model::{ProjectId, WorkspaceModel};
use crate::workspace::action::WorkspaceAction;

const BAR_HEIGHT: f32 = 34.;
const TAB_HEIGHT: f32 = 26.;
const TAB_MIN_WIDTH: f32 = 72.;
const TAB_MAX_WIDTH: f32 = 220.;
const SEARCH_WIDTH: f32 = 240.;
/// Space the native window controls occupy at the top-right of the client area.
const WINDOW_CONTROLS_INSET: f32 = 138.;

pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    project_id: ProjectId,
    available_workspace_tabs: &[String],
    tab_titles: &HashMap<String, String>,
    source_control_open: bool,
) -> Option<Box<dyn Element>> {
    let project = model.project(project_id)?;
    let theme = appearance.theme();
    let bar_background = theme.surface_1();
    let main_text = theme.main_text_color(bar_background);
    let sub_text = theme.sub_text_color(bar_background);

    let mut tabs = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.);
    for tab_id in &project.tabs {
        let Some(workspace_tab_id) = model.workspace_tab_id(*tab_id) else {
            continue;
        };
        if !available_workspace_tabs
            .iter()
            .any(|id| id == workspace_tab_id)
        {
            continue;
        }
        let tab_id = *tab_id;
        let active = project.active_tab == Some(tab_id);
        let tab_state = mouse_state("project-tab", tab_id as i64);
        let hovered = tab_state.lock().unwrap().is_mouse_over_element();
        let title = tab_titles
            .get(workspace_tab_id)
            .filter(|title| !title.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| "Terminal".to_owned());

        let mut content = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.);
        content.add_child(sized_icon(Icon::Terminal, sub_text, 12.));
        content.add_child(
            Shrinkable::new(
                1.,
                Text::new_inline(title, appearance.ui_font_family(), 12.)
                    .with_color((if active { main_text } else { sub_text }).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );

        let background = if active {
            Some(fg_overlay(theme, 10))
        } else if hovered {
            Some(fg_overlay(theme, 5))
        } else {
            None
        };
        let border_color = if active {
            Some(fg_overlay(theme, 20))
        } else if hovered {
            Some(fg_overlay(theme, 15))
        } else {
            None
        };
        let mut body = Container::new(content.finish())
            .with_padding_left(10.)
            .with_padding_right(24.)
            .with_corner_radius(radius(4.));
        if let Some(background) = background {
            body = body.with_background(background);
        }
        body = body.with_border(match border_color {
            Some(color) => Border::all(1.).with_border_fill(color),
            None => Border::all(1.),
        });
        let body = ConstrainedBox::new(body.finish())
            .with_height(TAB_HEIGHT)
            .with_min_width(TAB_MIN_WIDTH)
            .with_max_width(TAB_MAX_WIDTH)
            .finish();

        let mut tab = Stack::new().with_child(
            Hoverable::new(tab_state, |_| body)
                .with_cursor(Cursor::PointingHand)
                .with_defer_events_to_children()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(WorkspaceAction::SelectLocalWorkspaceTab(tab_id))
                })
                .finish(),
        );
        if hovered || active {
            let close = EventHandler::new(
                Container::new(sized_icon(Icon::X, sub_text, 12.))
                    .with_uniform_padding(2.)
                    .with_corner_radius(radius(3.))
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::CloseLocalWorkspaceTab(tab_id));
                DispatchEventResult::StopPropagation
            })
            .finish();
            tab.add_positioned_overlay_child(
                close,
                warpui::elements::OffsetPositioning::offset_from_parent(
                    pathfinder_geometry::vector::vec2f(-4., 0.),
                    warpui::elements::ParentOffsetBounds::ParentByPosition,
                    warpui::elements::ParentAnchor::MiddleRight,
                    warpui::elements::ChildAnchor::MiddleRight,
                ),
            );
        }
        tabs.add_child(Shrinkable::new(1., tab.finish()).finish());
    }
    tabs.add_child(naked_icon_button(
        appearance,
        Icon::Plus,
        theme.foreground(),
        None,
        WorkspaceAction::AddLocalWorkspaceTerminal,
        ("tab-plus", 0),
        Some(("Nova aba", TooltipSide::BelowRight)),
    ));

    let organization_name = model
        .organization(project.organization_id)
        .map(|organization| organization.name.as_str())
        .unwrap_or_default();
    let search = search_pill(
        appearance,
        bar_background,
        format!("{} · {organization_name}", project.name),
    );

    let mut bar = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    bar.add_child(Expanded::new(1., tabs.finish()).finish());
    bar.add_child(naked_icon_button(
        appearance,
        Icon::Link,
        theme.foreground(),
        None,
        WorkspaceAction::AssignActiveTabToLocalWorkspaceProject,
        ("tab-link", 0),
        Some(("Vincular aba ativa", TooltipSide::BelowRight)),
    ));
    bar.add_child(search);
    bar.add_child(naked_icon_button(
        appearance,
        Icon::GitBranch,
        theme.foreground(),
        source_control_open.then(|| fg_overlay(theme, 10)),
        WorkspaceAction::ToggleLocalWorkspaceSourceControl,
        ("tab-git", 0),
        Some(("Controle de versão", TooltipSide::BelowRight)),
    ));

    Some(bar_frame(appearance, bar.finish()))
}

/// Header shown above a view without a project (an empty organization): its title on the left
/// and the palette search on the right.
pub(super) fn render_title_bar(appearance: &Appearance, title: &str) -> Box<dyn Element> {
    let theme = appearance.theme();
    let bar_background = theme.surface_1();
    let mut bar = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    bar.add_child(
        Expanded::new(
            1.,
            Container::new(
                Text::new_inline(title.to_owned(), appearance.ui_font_family(), 12.)
                    .with_color(theme.main_text_color(bar_background).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .with_padding_left(8.)
            .finish(),
        )
        .finish(),
    );
    bar.add_child(search_pill(
        appearance,
        bar_background,
        "Buscar projetos e abas...".to_owned(),
    ));
    bar_frame(appearance, bar.finish())
}

fn bar_frame(appearance: &Appearance, bar: Box<dyn Element>) -> Box<dyn Element> {
    let theme = appearance.theme();
    ConstrainedBox::new(
        Container::new(bar)
            .with_background(theme.surface_1())
            .with_padding_top(1.)
            .with_padding_left(8.)
            .with_padding_right(8. + WINDOW_CONTROLS_INSET)
            .with_border(Border::bottom(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_height(BAR_HEIGHT)
    .finish()
}

/// Palette entry point styled as a 24px search field; clicking it opens Home.
fn search_pill(
    appearance: &Appearance,
    bar_background: warp_core::ui::theme::Fill,
    label: String,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut search = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    search.add_child(sized_icon(
        Icon::Search,
        theme.hint_text_color(bar_background),
        14.,
    ));
    search.add_child(
        Shrinkable::new(
            1.,
            Text::new_inline(label, appearance.ui_font_family(), 12.)
                .with_color(theme.hint_text_color(bar_background).into())
                .soft_wrap(false)
                .with_clip(ClipConfig::ellipsis())
                .finish(),
        )
        .finish(),
    );
    EventHandler::new(
        ConstrainedBox::new(
            Container::new(search.finish())
                .with_padding_left(10.)
                .with_padding_right(10.)
                .with_background(fg_overlay(theme, 5))
                .with_corner_radius(radius(4.))
                .finish(),
        )
        .with_height(24.)
        .with_width(SEARCH_WIDTH)
        .finish(),
    )
    .on_left_mouse_up(move |ctx, _, _| {
        ctx.dispatch_typed_action(WorkspaceAction::SelectLocalWorkspaceHome);
        DispatchEventResult::StopPropagation
    })
    .finish()
}
