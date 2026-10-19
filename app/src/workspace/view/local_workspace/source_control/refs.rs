use crate::appearance::Appearance;
use crate::local_git::LocalBranch;
use crate::workspace::action::WorkspaceAction;
use warpui::elements::{
    Container, DispatchEventResult, Element, EventHandler, Flex, ParentElement, Text,
};

pub(super) fn render(
    branches: &[LocalBranch],
    open_menu: Option<&str>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut section = Flex::column();
    section.add_child(
        Container::new(
            Text::new_inline("BRANCHES LOCAIS", appearance.ui_font_family(), 10.)
                .with_color(theme.sub_text_color(theme.surface_1()).into())
                .finish(),
        )
        .with_background(theme.surface_2())
        .with_padding_left(12.)
        .with_padding_right(8.)
        .with_padding_top(7.)
        .with_padding_bottom(7.)
        .finish(),
    );
    for branch in branches {
        let label = if branch.is_current {
            format!("●  {}", branch.name)
        } else {
            format!("   {}", branch.name)
        };
        let row = Container::new(
            Text::new_inline(label, appearance.ui_font_family(), 11.)
                .with_color(theme.font_color(theme.surface_1()).into())
                .finish(),
        )
        .with_padding_left(12.)
        .with_padding_right(12.)
        .with_padding_top(5.)
        .with_padding_bottom(5.)
        .finish();
        let branch_name = branch.name.clone();
        section.add_child(
            EventHandler::new(row)
                .on_right_mouse_down(move |ctx, _, _, _| {
                    ctx.dispatch_typed_action(WorkspaceAction::OpenLocalGitBranchMenu(
                        branch_name.clone(),
                    ));
                    DispatchEventResult::StopPropagation
                })
                .finish(),
        );
        if open_menu == Some(branch.name.as_str()) {
            section.add_child(branch_menu(branch, appearance));
        }
    }
    Container::new(section.finish()).finish()
}

fn branch_menu(branch: &LocalBranch, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut menu = Flex::column();
    if branch.is_current {
        menu.add_child(menu_item(
            "Branch atual",
            WorkspaceAction::DismissLocalGitBranchMenu,
            false,
            appearance,
        ));
    } else {
        menu.add_child(menu_item(
            "Fazer checkout",
            WorkspaceAction::CheckoutLocalGitBranch(branch.name.clone()),
            true,
            appearance,
        ));
    }
    menu.add_child(menu_item(
        "Fechar menu",
        WorkspaceAction::DismissLocalGitBranchMenu,
        true,
        appearance,
    ));
    Container::new(menu.finish())
        .with_background(theme.surface_2())
        .with_border(warpui::elements::Border::all(1.).with_border_fill(theme.outline()))
        .with_padding_left(8.)
        .with_padding_right(8.)
        .with_padding_top(4.)
        .with_padding_bottom(4.)
        .finish()
}

fn menu_item(
    label: &'static str,
    action: WorkspaceAction,
    enabled: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let item = Container::new(
        Text::new_inline(label, appearance.ui_font_family(), 10.)
            .with_color(if enabled {
                theme.font_color(theme.surface_2()).into()
            } else {
                theme.sub_text_color(theme.surface_2()).into()
            })
            .finish(),
    )
    .with_uniform_padding(5.)
    .finish();
    if enabled {
        EventHandler::new(item)
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    } else {
        item
    }
}
