use super::ws_style::{mouse_state, radius, sized_icon};
use crate::appearance::Appearance;
use crate::local_workspace::model::ContextMenuTarget;
use crate::menu::{
    MENU_ITEM_HORIZONTAL_PADDING, MENU_ITEM_VERTICAL_PADDING, MENU_VERTICAL_PADDING,
    SEPARATOR_VERTICAL_MARGIN,
};
use crate::workspace::action::WorkspaceAction;
use pathfinder_color::ColorU;
use warp_core::ui::Icon;
use warp_core::ui::color::blend::Blend;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, Dismiss, DispatchEventResult,
    DropShadow, Element, EventHandler, Expanded, Flex, Hoverable, MainAxisSize, ParentElement,
    Rect, Text,
};
use warpui::platform::Cursor;

pub(super) struct MenuItem {
    pub label: &'static str,
    pub icon: Icon,
    pub action: WorkspaceAction,
    pub danger: bool,
    pub separator_before: bool,
    pub checked: bool,
}

pub(super) fn render(
    target: ContextMenuTarget,
    selected_index: usize,
    keyboard_active: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    menu(
        items(target),
        Some(selected_index).filter(|_| keyboard_active),
        CONTEXT_MENU_WIDTH,
        WorkspaceAction::DismissLocalWorkspaceContextMenu,
        appearance,
    )
}

const CONTEXT_MENU_WIDTH: f32 = 186.;

/// Same look as the app's `Menu` (the new-session menu): accent hover, danger items in the
/// theme's red, dismissed by Escape or an outside click through `dismiss`.
pub(super) fn menu(
    items: Vec<MenuItem>,
    keyboard_selection: Option<usize>,
    width: f32,
    dismiss: WorkspaceAction,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let menu_background = theme.surface_2();
    let font_size = appearance.ui_font_size();
    let mut menu = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    for (action_index, item) in items.into_iter().enumerate() {
        if item.separator_before {
            menu.add_child(
                Container::new(
                    ConstrainedBox::new(Rect::new().with_background(theme.outline()).finish())
                        .with_height(1.)
                        .finish(),
                )
                .with_margin_top(SEPARATOR_VERTICAL_MARGIN)
                .with_margin_bottom(SEPARATOR_VERTICAL_MARGIN)
                .finish(),
            );
        }
        let keyboard_selected = keyboard_selection == Some(action_index);
        let state = mouse_state("menu-item", action_index as i64);
        let hovered = state.lock().unwrap().is_mouse_over_element();
        let highlight = (hovered || keyboard_selected).then(|| theme.accent_button_color());
        let text_background = match highlight {
            Some(overlay) => menu_background.blend(&overlay),
            None => menu_background,
        };
        let item_color: Fill = if item.danger {
            Fill::Solid(theme.terminal_colors().normal.red.into())
        } else {
            theme.main_text_color(text_background)
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);
        row.add_child(
            Container::new(sized_icon(item.icon, item_color, font_size))
                .with_margin_right(font_size / 2.)
                .finish(),
        );
        row.add_child(
            Expanded::new(
                1.,
                Text::new_inline(item.label, appearance.ui_font_family(), font_size)
                    .with_color(item_color.into())
                    .soft_wrap(false)
                    .finish(),
            )
            .finish(),
        );
        if item.checked {
            row.add_child(sized_icon(Icon::Check, item_color, font_size));
        }
        let mut body = Container::new(row.finish())
            .with_padding_left(MENU_ITEM_HORIZONTAL_PADDING)
            .with_padding_right(MENU_ITEM_HORIZONTAL_PADDING)
            .with_padding_top(MENU_ITEM_VERTICAL_PADDING)
            .with_padding_bottom(MENU_ITEM_VERTICAL_PADDING);
        if let Some(highlight) = highlight {
            body = body.with_background(highlight);
        }
        let action = item.action;
        menu.add_child(
            Hoverable::new(state, |_| body.finish())
                .with_cursor(Cursor::PointingHand)
                .on_click(move |ctx, _, position| {
                    ctx.dispatch_typed_action(action.clone().at_position(position))
                })
                .finish(),
        );
    }
    let menu = EventHandler::new(
        ConstrainedBox::new(
            Container::new(menu.finish())
                .with_background(menu_background)
                .with_border(Border::all(1.).with_border_fill(theme.outline()))
                .with_corner_radius(radius(5.))
                .with_padding_top(MENU_VERTICAL_PADDING)
                .with_padding_bottom(MENU_VERTICAL_PADDING)
                .with_drop_shadow(DropShadow::new_with_standard_offset_and_spread(
                    ColorU::new(0, 0, 0, 48),
                ))
                .finish(),
        )
        .with_width(width)
        .finish(),
    )
    .on_keydown({
        let dismiss = dismiss.clone();
        move |ctx, _, keystroke| {
            if keystroke.key.eq_ignore_ascii_case("escape") {
                ctx.dispatch_typed_action(dismiss.clone());
                DispatchEventResult::StopPropagation
            } else {
                DispatchEventResult::PropagateToParent
            }
        }
    })
    .with_always_handle()
    .finish();
    Dismiss::new(menu)
        .on_dismiss(move |ctx, _app| {
            ctx.dispatch_typed_action(dismiss.clone());
        })
        .prevent_interaction_with_other_elements()
        .dismiss_on_right_click()
        .finish()
}

fn items(target: ContextMenuTarget) -> Vec<MenuItem> {
    match target {
        ContextMenuTarget::Organization(id) => vec![
            MenuItem {
                label: "Editar organização",
                icon: Icon::Pencil,
                action: WorkspaceAction::EditLocalWorkspaceOrganization(id),
                danger: false,
                separator_before: false,
                checked: false,
            },
            MenuItem {
                label: "Remover organização",
                icon: Icon::Trash,
                action: WorkspaceAction::RemoveLocalWorkspaceOrganization(id),
                danger: true,
                separator_before: true,
                checked: false,
            },
        ],
        ContextMenuTarget::Project(id) => vec![
            MenuItem {
                label: "Nova aba",
                icon: Icon::Plus,
                action: WorkspaceAction::OpenLocalWorkspaceNewTabMenu {
                    project_id: id,
                    position: None,
                },
                danger: false,
                separator_before: false,
                checked: false,
            },
            MenuItem {
                label: "Renomear projeto",
                icon: Icon::Pencil,
                action: WorkspaceAction::EditLocalWorkspaceProject(id),
                danger: false,
                separator_before: false,
                checked: false,
            },
            MenuItem {
                label: "Controle de versão",
                icon: Icon::GitBranch,
                action: WorkspaceAction::ToggleLocalWorkspaceSourceControlForProject(id),
                danger: false,
                separator_before: false,
                checked: false,
            },
            MenuItem {
                label: "Remover projeto",
                icon: Icon::Trash,
                action: WorkspaceAction::RemoveLocalWorkspaceProject(id),
                danger: true,
                separator_before: true,
                checked: false,
            },
        ],
    }
}
