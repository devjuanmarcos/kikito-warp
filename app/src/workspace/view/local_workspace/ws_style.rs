use std::cell::RefCell;
use std::collections::HashMap;

use pathfinder_geometry::vector::vec2f;
use warp_core::ui::Icon;
use warp_core::ui::color::blend::Blend;
use warp_core::ui::theme::{Fill, WarpTheme};
use warpui::elements::{
    Border, ChildAnchor, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, DragAxis,
    Draggable, DraggableState, Element, Flex, Hoverable, MainAxisAlignment, MainAxisSize,
    MouseState, MouseStateHandle, OffsetPositioning, ParentAnchor, ParentElement,
    ParentOffsetBounds, Radius, Stack, Text,
};
use warpui::platform::Cursor;

use crate::appearance::Appearance;
use crate::local_workspace::model::DragItem;
use crate::workspace::action::WorkspaceAction;

thread_local! {
    static MOUSE_STATES: RefCell<HashMap<(&'static str, i64), MouseStateHandle>> =
        RefCell::new(HashMap::new());
}

/// Returns the same `MouseStateHandle` for a given key across renders, so hover and press
/// survive the per-frame rebuild of the element tree.
pub(super) fn mouse_state(kind: &'static str, id: i64) -> MouseStateHandle {
    MOUSE_STATES.with(|states| states.borrow_mut().entry((kind, id)).or_default().clone())
}

thread_local! {
    static DRAGGABLE_STATES: RefCell<HashMap<DragItem, DraggableState>> =
        RefCell::new(HashMap::new());
}

/// Returns the same `DraggableState` for an item across renders, so a drag survives the
/// per-frame rebuild of the element tree.
pub(super) fn draggable_state(item: DragItem) -> DraggableState {
    DRAGGABLE_STATES.with(|states| states.borrow_mut().entry(item).or_default().clone())
}

/// Lets `element` be dragged along `axis` to reorder `item` within its list. With a
/// `placeholder_radius`, the emptied slot shows a faint rounded placeholder during the drag, like
/// the tabs sidebar.
pub(super) fn reorder_draggable(
    element: Box<dyn Element>,
    item: DragItem,
    axis: DragAxis,
    placeholder_radius: Option<f32>,
    theme: &WarpTheme,
) -> Box<dyn Element> {
    let state = draggable_state(item);
    let dragging = state.is_dragging();
    let draggable = Draggable::new(state, element)
        .with_drag_axis(axis)
        .on_drag(move |ctx, _, position, _| {
            ctx.dispatch_typed_action(WorkspaceAction::DragLocalWorkspaceItem { item, position })
        })
        .on_drop(move |ctx, _, _, _| {
            ctx.dispatch_typed_action(WorkspaceAction::DropLocalWorkspaceItem(item))
        })
        .finish();
    match placeholder_radius {
        Some(placeholder_radius) if dragging => Container::new(draggable)
            .with_background(fg_overlay(theme, 5))
            .with_corner_radius(radius(placeholder_radius))
            .finish(),
        _ => draggable,
    }
}

pub(super) fn fg_overlay(theme: &WarpTheme, percent: u8) -> Fill {
    theme.foreground().with_opacity(percent)
}

/// Opaque tone between the theme background and foreground (the design system's `neutral-N`).
pub(super) fn neutral(theme: &WarpTheme, percent: u8) -> Fill {
    Fill::Solid(theme.background().into_solid()).blend(&theme.foreground().with_opacity(percent))
}

pub(super) fn radius(pixels: f32) -> CornerRadius {
    CornerRadius::with_all(Radius::Pixels(pixels))
}

pub(super) fn sized_icon(icon: Icon, color: Fill, size: f32) -> Box<dyn Element> {
    ConstrainedBox::new(icon.to_warpui_icon(color).finish())
        .with_width(size)
        .with_height(size)
        .finish()
}

#[derive(Clone, Copy)]
pub(super) enum TooltipSide {
    AboveRight,
    BelowRight,
    /// Beside the element; `gap` is measured from its right edge.
    Right {
        gap: f32,
    },
}

/// Inverted pill used for hover hints: tooltip background, theme-background text, 10px UI font.
pub(super) fn tooltip(appearance: &Appearance, label: &str) -> Box<dyn Element> {
    let theme = appearance.theme();
    let content = Flex::row()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Text::new_inline(label.to_owned(), appearance.ui_font_family(), 10.)
                .with_color(theme.background().into())
                .soft_wrap(false)
                .finish(),
        )
        .finish();
    Container::new(
        ConstrainedBox::new(
            Container::new(content)
                .with_padding_left(7.)
                .with_padding_right(7.)
                .finish(),
        )
        .with_height(20.)
        .finish(),
    )
    .with_background(neutral(theme, 60))
    .with_border(Border::all(1.).with_border_fill(neutral(theme, 10)))
    .with_corner_radius(radius(4.))
    .finish()
}

/// Anchors `tooltip` next to `element` without taking part in the element's layout.
pub(super) fn with_tooltip(
    element: Box<dyn Element>,
    tooltip: Box<dyn Element>,
    side: TooltipSide,
) -> Box<dyn Element> {
    let positioning = match side {
        TooltipSide::AboveRight => OffsetPositioning::offset_from_parent(
            vec2f(0., -4.),
            ParentOffsetBounds::WindowByPosition,
            ParentAnchor::TopRight,
            ChildAnchor::BottomRight,
        ),
        TooltipSide::BelowRight => OffsetPositioning::offset_from_parent(
            vec2f(0., 4.),
            ParentOffsetBounds::WindowByPosition,
            ParentAnchor::BottomRight,
            ChildAnchor::TopRight,
        ),
        TooltipSide::Right { gap } => OffsetPositioning::offset_from_parent(
            vec2f(gap, 0.),
            ParentOffsetBounds::WindowByPosition,
            ParentAnchor::MiddleRight,
            ChildAnchor::MiddleLeft,
        ),
    };
    let mut stack = Stack::new().with_child(element);
    stack.add_positioned_overlay_child(tooltip, positioning);
    stack.finish()
}

fn pressed(mouse: &MouseState) -> bool {
    mouse.is_mouse_over_element() && mouse.is_clicked()
}

/// Background of a naked/ghost button: transparent, `fg-overlay-2` on hover, `fg-overlay-3` on press.
fn ghost_background(theme: &WarpTheme, mouse: &MouseState, active: Option<Fill>) -> Option<Fill> {
    if pressed(mouse) {
        Some(fg_overlay(theme, 15))
    } else if mouse.is_mouse_over_element() {
        Some(fg_overlay(theme, 10))
    } else {
        active
    }
}

/// 24px "naked small" button: transparent square with a centered 14px icon.
pub(super) fn naked_icon_button(
    appearance: &Appearance,
    icon: Icon,
    color: Fill,
    active_background: Option<Fill>,
    action: WorkspaceAction,
    key: (&'static str, i64),
    tooltip_label: Option<(&str, TooltipSide)>,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let state = mouse_state(key.0, key.1);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let button = Hoverable::new(state, |mouse| {
        let glyph = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(sized_icon(icon, color, 14.))
            .finish();
        let mut button = Container::new(
            ConstrainedBox::new(glyph)
                .with_width(24.)
                .with_height(24.)
                .finish(),
        )
        .with_corner_radius(radius(4.));
        if let Some(background) = ghost_background(theme, mouse, active_background) {
            button = button.with_background(background);
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, position| {
        ctx.dispatch_typed_action(action.clone().at_position(position))
    })
    .finish();
    match tooltip_label {
        Some((label, side)) if hovered => with_tooltip(button, tooltip(appearance, label), side),
        _ => button,
    }
}

/// Secondary (outlined) button body. `height` is 24 for the small size and 32 for the default.
pub(super) fn secondary_button(
    theme: &WarpTheme,
    content: Box<dyn Element>,
    height: f32,
    action: WorkspaceAction,
    key: (&'static str, i64),
) -> Box<dyn Element> {
    Hoverable::new(mouse_state(key.0, key.1), |mouse| {
        let mut button = Container::new(
            ConstrainedBox::new(content)
                .with_height(height - 2.)
                .finish(),
        )
        .with_border(warpui::elements::Border::all(1.).with_border_fill(neutral(theme, 20)))
        .with_corner_radius(radius(4.));
        if let Some(background) = ghost_background(theme, mouse, None) {
            button = button.with_background(background);
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, position| {
        ctx.dispatch_typed_action(action.clone().at_position(position))
    })
    .finish()
}
