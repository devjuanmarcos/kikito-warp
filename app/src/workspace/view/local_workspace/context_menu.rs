use crate::appearance::Appearance;
use crate::local_workspace::model::ContextMenuTarget;
use crate::workspace::action::WorkspaceAction;
use warpui::elements::{
    Container, CrossAxisAlignment, DispatchEventResult, Element, EventHandler, Flex, ParentElement,
    Text,
};

pub(super) fn render(
    target: ContextMenuTarget,
    selected_index: usize,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut menu = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(2.);
    let items = match target {
        ContextMenuTarget::Organization(id) => vec![
            (
                "Novo projeto",
                WorkspaceAction::CreateLocalWorkspaceProject(id),
            ),
            (
                "Editar organização",
                WorkspaceAction::EditLocalWorkspaceOrganization(id),
            ),
            (
                "Remover organização",
                WorkspaceAction::RemoveLocalWorkspaceOrganization(id),
            ),
        ],
        ContextMenuTarget::Project(id) => vec![
            (
                "Editar projeto",
                WorkspaceAction::EditLocalWorkspaceProject(id),
            ),
            (
                "Remover projeto",
                WorkspaceAction::RemoveLocalWorkspaceProject(id),
            ),
        ],
    };
    for (index, (label, action)) in items
        .into_iter()
        .chain(std::iter::once((
            "Fechar menu",
            WorkspaceAction::DismissLocalWorkspaceContextMenu,
        )))
        .enumerate()
    {
        let row = Container::new(
            Text::new_inline(label, appearance.ui_font_family(), 12.)
                .with_color(theme.font_color(theme.surface_2()).into())
                .finish(),
        )
        .with_background(if index == selected_index {
            theme.surface_3()
        } else {
            theme.surface_2()
        })
        .with_border(if index == selected_index {
            warpui::elements::Border::all(1.).with_border_fill(theme.accent())
        } else {
            warpui::elements::Border::all(1.).with_border_fill(theme.surface_2())
        })
        .with_padding_left(12.)
        .with_padding_right(18.)
        .with_padding_top(8.)
        .with_padding_bottom(8.)
        .finish();
        menu.add_child(
            EventHandler::new(row)
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish(),
        );
    }
    EventHandler::new(
        Container::new(menu.finish())
            .with_background(theme.surface_2())
            .with_border(warpui::elements::Border::all(1.).with_border_fill(theme.outline()))
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(6.),
            ))
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
    .finish()
}
