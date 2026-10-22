use std::collections::HashMap;

use warp_core::ui::Icon;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult, DragAxis, Element,
    EventHandler, Expanded, Flex, Hoverable, MainAxisSize, ParentElement, Shrinkable, Text,
};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;

use super::super::vertical_tabs::{
    VERTICAL_TABS_ADD_TAB_POSITION_ID, VERTICAL_TABS_SETTINGS_BUTTON_POSITION_ID,
};
use super::ws_style::{
    TooltipSide, fg_overlay, mouse_state, naked_icon_button, radius, reorder_draggable, sized_icon,
    with_tooltip,
};
use crate::appearance::Appearance;
use crate::local_workspace::model::{DragItem, ProjectId, WorkspaceModel};
use crate::workspace::action::NewSessionMenuAnchor;
use crate::workspace::action::WorkspaceAction;
use warpui::elements::SavePosition;
use warpui::geometry::vector::Vector2F;

const BAR_HEIGHT: f32 = 34.;
pub(crate) const TABS_LAYOUT_BUTTON_POSITION_ID: &str = "local_workspace:tabs_layout_button";
const CLOSE_SLOT: f32 = 16.;
const TAB_HEIGHT: f32 = 26.;
const TAB_MIN_WIDTH: f32 = 72.;
const TAB_MAX_WIDTH: f32 = 220.;
const SEARCH_WIDTH: f32 = 240.;
/// Space the native window controls occupy at the top-right of the client area.
pub(super) const WINDOW_CONTROLS_INSET: f32 = 138.;
/// Width of the notifications and settings buttons left of the window controls.
pub(super) const WINDOW_BUTTONS_WIDTH: f32 = 52.;

#[allow(clippy::too_many_arguments)]
pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    project_id: ProjectId,
    available_workspace_tabs: &[String],
    tab_titles: &HashMap<String, String>,
    source_control_open: bool,
    tabs_in_sidebar: bool,
    mut rename_editor: Option<(i32, Box<dyn Element>)>,
    git_changes: usize,
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
    let header_tabs: &[i32] = if tabs_in_sidebar { &[] } else { &project.tabs };
    for tab_id in header_tabs {
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
        let label = match rename_editor.take_if(|(renaming, _)| *renaming == tab_id) {
            Some((_, editor)) => warpui::elements::Clipped::new(editor).finish(),
            None => Text::new_inline(title, appearance.ui_font_family(), 12.)
                .with_color((if active { main_text } else { sub_text }).into())
                .soft_wrap(false)
                .with_clip(ClipConfig::ellipsis())
                .finish(),
        };
        content.add_child(Expanded::new(1., label).finish());
        // The close button sits in a fixed slot inside the row so it stays centered on the title
        // and the tab doesn't change width when it appears.
        let close: Box<dyn Element> = if hovered || active {
            let close_state = mouse_state("project-tab-close", tab_id as i64);
            Hoverable::new(close_state, move |mouse| {
                let mut button = Container::new(sized_icon(Icon::X, sub_text, 12.))
                    .with_uniform_padding(2.)
                    .with_corner_radius(radius(3.));
                if mouse.is_mouse_over_element() {
                    button = button.with_background(fg_overlay(theme, 10));
                }
                button.finish()
            })
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::CloseLocalWorkspaceTab(tab_id))
            })
            .finish()
        } else {
            Flex::row().finish()
        };
        content.add_child(
            ConstrainedBox::new(close)
                .with_width(CLOSE_SLOT)
                .with_height(CLOSE_SLOT)
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
            .with_padding_right(4.)
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

        let tab = Hoverable::new(tab_state, |_| body)
            .with_cursor(Cursor::PointingHand)
            .with_defer_events_to_children()
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::SelectLocalWorkspaceTab(tab_id))
            })
            .finish();
        let tab = EventHandler::new(tab)
            .on_right_mouse_down(move |ctx, _, position, _| {
                ctx.dispatch_typed_action(WorkspaceAction::OpenLocalWorkspaceTabMenu {
                    tab_id,
                    position,
                });
                DispatchEventResult::StopPropagation
            })
            .finish();
        let tab = reorder_draggable(
            tab,
            DragItem::Tab(tab_id),
            DragAxis::HorizontalOnly,
            Some(4.),
            theme,
        );
        let tab = SavePosition::new(tab, &super::project_tab_position_id(tab_id)).finish();
        tabs.add_child(Shrinkable::new(1., tab).finish());
    }
    // In sidebar mode the vertical tabs panel hosts these buttons and their position ids.
    if !tabs_in_sidebar {
        // `+` opens the session menu (shells, tab configs) the vertical tabs panel used to host; its
        // position id lets that menu anchor below this button.
        tabs.add_child(
            SavePosition::new(
                header_button(
                    appearance,
                    Icon::Plus,
                    ("tab-plus", 0),
                    "Nova aba",
                    |ctx, position| {
                        ctx.dispatch_typed_action(WorkspaceAction::ToggleNewSessionMenu {
                            anchor: NewSessionMenuAnchor::AddTabButton(position),
                        })
                    },
                ),
                VERTICAL_TABS_ADD_TAB_POSITION_ID,
            )
            .finish(),
        );
        tabs.add_child(
            SavePosition::new(
                header_button(
                    appearance,
                    Icon::Sliders,
                    ("tab-view-options", 0),
                    "Opções de visualização",
                    |ctx, _| {
                        ctx.dispatch_typed_action(WorkspaceAction::ToggleVerticalTabsSettingsPopup)
                    },
                ),
                VERTICAL_TABS_SETTINGS_BUTTON_POSITION_ID,
            )
            .finish(),
        );
    }

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
    bar.add_child(search);
    let git_button = naked_icon_button(
        appearance,
        Icon::GitBranch,
        theme.foreground(),
        source_control_open.then(|| fg_overlay(theme, 10)),
        WorkspaceAction::ToggleLocalWorkspaceSourceControl,
        ("tab-git", 0),
        Some(("Controle de versão", TooltipSide::BelowRight)),
    );
    // Like VS Code's activity bar badge: how many files have pending changes.
    if git_changes > 0 {
        let label = if git_changes > 99 {
            "99+".to_owned()
        } else {
            git_changes.to_string()
        };
        let mut stack = warpui::elements::Stack::new().with_child(git_button);
        stack.add_positioned_child(
            Container::new(
                Text::new_inline(label, appearance.ui_font_family(), 9.)
                    .with_color(theme.foreground().into())
                    .finish(),
            )
            .with_background(theme.accent())
            .with_corner_radius(radius(6.))
            .with_padding_left(3.)
            .with_padding_right(3.)
            .finish(),
            warpui::elements::OffsetPositioning::offset_from_parent(
                pathfinder_geometry::vector::vec2f(4., -3.),
                warpui::elements::ParentOffsetBounds::WindowByPosition,
                warpui::elements::ParentAnchor::TopRight,
                warpui::elements::ChildAnchor::TopRight,
            ),
        );
        bar.add_child(stack.finish());
    } else {
        bar.add_child(git_button);
    }
    bar.add_child(
        SavePosition::new(
            header_button(
                appearance,
                Icon::LayoutAlt01,
                ("tab-layout", 0),
                "Layout das abas",
                |ctx, _| {
                    ctx.dispatch_typed_action(WorkspaceAction::OpenLocalWorkspaceTabsLayoutMenu)
                },
            ),
            TABS_LAYOUT_BUTTON_POSITION_ID,
        )
        .finish(),
    );

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

/// Header of the settings screen: a button that closes it (the settings pane shows its own title).
pub(super) fn render_settings_bar(appearance: &Appearance) -> Box<dyn Element> {
    let mut bar = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    bar.add_child(Expanded::new(1., Flex::row().finish()).finish());
    bar.add_child(header_button(
        appearance,
        Icon::X,
        ("settings-close", 0),
        "Fechar configurações",
        |ctx, _| ctx.dispatch_typed_action(WorkspaceAction::CloseActiveTab),
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
            .with_padding_right(16. + WINDOW_CONTROLS_INSET + WINDOW_BUTTONS_WIDTH)
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

/// Naked 24px header button that hands the click position to `on_click`, with a tooltip below.
pub(super) fn header_button(
    appearance: &Appearance,
    icon: Icon,
    key: (&'static str, i64),
    label: &'static str,
    on_click: impl Fn(&mut warpui::EventContext, Vector2F) + 'static,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let state = mouse_state(key.0, key.1);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let button = Hoverable::new(state, move |mouse| {
        let mut button = Container::new(sized_icon(icon, theme.foreground(), 14.))
            .with_uniform_padding(5.)
            .with_corner_radius(radius(4.));
        if mouse.is_clicked() {
            button = button.with_background(fg_overlay(theme, 15));
        } else if mouse.is_mouse_over_element() {
            button = button.with_background(fg_overlay(theme, 10));
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, position| on_click(ctx, position))
    .finish();
    if hovered {
        with_tooltip(
            button,
            super::ws_style::tooltip(appearance, label),
            TooltipSide::BelowRight,
        )
    } else {
        button
    }
}
