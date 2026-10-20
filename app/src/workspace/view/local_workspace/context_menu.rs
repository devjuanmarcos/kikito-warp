use crate::appearance::Appearance;
use crate::local_workspace::model::ContextMenuTarget;
use crate::workspace::action::WorkspaceAction;
use warp_core::ui::Icon;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, Dismiss, DispatchEventResult, Element,
    EventHandler, Flex, MainAxisSize, ParentElement, Rect, Shrinkable, Text,
};

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
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let items = items(target);
    let mut menu = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(1.);
    let mut action_index = 0;
    for item in items {
        if item.separator_before {
            menu.add_child(
                Container::new(
                    ConstrainedBox::new(
                        Rect::new()
                            .with_background(theme.outline())
                            .finish(),
                    )
                    .with_height(1.)
                    .finish(),
                )
                .with_margin_left(10.)
                .with_margin_right(10.)
                .with_margin_top(4.)
                .with_margin_bottom(4.)
                .finish(),
            );
        }
        let selected = action_index == selected_index;
        action_index += 1;
        let label_color = if item.danger {
            theme.ui_error_color().into()
        } else {
            theme.font_color(theme.surface_2()).into()
        };
        let icon_color = if item.danger {
            theme.ui_error_color().into()
        } else {
            theme.sub_text_color(theme.surface_2())
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(8.);
        row.add_child(
            ConstrainedBox::new(item.icon.to_warpui_icon(icon_color).finish())
                .with_width(14.)
                .with_height(14.)
                .finish(),
        );
        row.add_child(Shrinkable::new(
            1.,
            Text::new_inline(item.label, appearance.ui_font_family(), 12.)
                .with_color(label_color)
                .finish(),
        ).finish());
        let action = item.action;
        menu.add_child(
            EventHandler::new(
                Container::new(row.finish())
                    .with_background(if selected {
                        theme.surface_3()
                    } else {
                        theme.surface_2()
                    })
                    .with_uniform_padding(5.)
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );
    }
    let menu = EventHandler::new(
        ConstrainedBox::new(
            Container::new(menu.finish())
                .with_background(theme.surface_2())
                .with_border(Border::all(1.).with_border_fill(theme.outline()))
                .with_corner_radius(warpui::elements::CornerRadius::with_all(
                    warpui::elements::Radius::Pixels(5.),
                ))
                .with_uniform_padding(4.)
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
