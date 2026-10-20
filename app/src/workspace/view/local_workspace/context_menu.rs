use super::ws_style::{mouse_state, radius, sized_icon};
use crate::appearance::Appearance;
use crate::local_workspace::model::ContextMenuTarget;
use crate::workspace::action::WorkspaceAction;
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, Dismiss, DispatchEventResult,
    DropShadow, Element, EventHandler, Expanded, Flex, Hoverable, MainAxisSize,
    ParentElement, Rect, Text,
};
use warpui::platform::Cursor;

struct MenuItem {
    label: &'static str,
    icon: Icon,
    action: WorkspaceAction,
    danger: bool,
    separator_before: bool,
}

pub(super) fn render(
    target: ContextMenuTarget,
    selected_index: usize,
    keyboard_active: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let items = items(target);
    let mut menu = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    let mut action_index = 0;
    for item in items {
        if item.separator_before {
            menu.add_child(
                Container::new(
                    ConstrainedBox::new(Rect::new().with_background(theme.outline()).finish())
                        .with_height(1.)
                        .finish(),
                )
                .with_margin_top(4.)
                .with_margin_bottom(4.)
                .finish(),
            );
        }
        let keyboard_selected = keyboard_active && action_index == selected_index;
        let state = mouse_state("menu-item", action_index as i64);
        action_index += 1;
        let hovered = state.lock().unwrap().is_mouse_over_element();
        let item_color: Fill = if item.danger {
            Fill::Solid(theme.terminal_colors().normal.red.into())
        } else {
            theme.main_text_color(theme.surface_2())
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(8.);
        row.add_child(sized_icon(item.icon, item_color, 14.));
        row.add_child(
            Expanded::new(
                1.,
                Text::new_inline(item.label, appearance.ui_font_family(), 12.)
                    .with_color(item_color.into())
                    .soft_wrap(false)
                    .finish(),
            )
            .finish(),
        );
        let mut body = Container::new(row.finish())
            .with_padding_left(14.)
            .with_padding_right(14.)
            .with_padding_top(5.)
            .with_padding_bottom(5.);
        if hovered || keyboard_selected {
            body = body.with_background(theme.accent());
        }
        let action = item.action;
        menu.add_child(
            Hoverable::new(state, |_| body.finish())
                .with_cursor(Cursor::PointingHand)
                .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
                .finish(),
        );
    }
    let menu = EventHandler::new(
        ConstrainedBox::new(
            Container::new(menu.finish())
                .with_background(theme.surface_2())
                .with_border(Border::all(1.).with_border_fill(theme.outline()))
                .with_corner_radius(radius(5.))
                .with_padding_top(9.)
                .with_padding_bottom(9.)
                .with_drop_shadow(DropShadow {
                    color: ColorU::new(0, 0, 0, 102),
                    offset: vec2f(0., 8.),
                    blur_radius: 24.,
                    spread_radius: 0.,
                })
                .finish(),
        )
        .with_width(186.)
        .finish(),
    )
    .on_keydown(|ctx, _, keystroke| {
        if keystroke.key.eq_ignore_ascii_case("escape") {
            ctx.dispatch_typed_action(WorkspaceAction::DismissLocalWorkspaceContextMenu);
            DispatchEventResult::StopPropagation
        } else {
            DispatchEventResult::PropagateToParent
        }
    })
    .with_always_handle()
    .finish();
    Dismiss::new(menu)
        .on_dismiss(|ctx, _app| {
            ctx.dispatch_typed_action(WorkspaceAction::DismissLocalWorkspaceContextMenu);
        })
        .prevent_interaction_with_other_elements()
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
            },
            MenuItem {
                label: "Remover organização",
                icon: Icon::Trash,
                action: WorkspaceAction::RemoveLocalWorkspaceOrganization(id),
                danger: true,
                separator_before: true,
            },
        ],
        ContextMenuTarget::Project(id) => vec![
            MenuItem {
                label: "Nova aba",
                icon: Icon::Plus,
                action: WorkspaceAction::AddLocalWorkspaceTerminalToProject(id),
                danger: false,
                separator_before: false,
            },
            MenuItem {
                label: "Controle de versão",
                icon: Icon::GitBranch,
                action: WorkspaceAction::ToggleLocalWorkspaceSourceControlForProject(id),
                danger: false,
                separator_before: false,
            },
            MenuItem {
                label: "Remover projeto",
                icon: Icon::Trash,
                action: WorkspaceAction::RemoveLocalWorkspaceProject(id),
                danger: true,
                separator_before: true,
            },
        ],
    }
}
