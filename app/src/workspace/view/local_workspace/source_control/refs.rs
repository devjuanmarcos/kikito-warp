use warp_core::ui::Icon;
use warpui::elements::{
    ConstrainedBox, Container, CrossAxisAlignment, Element, Expanded, Flex, Hoverable,
    MainAxisSize, ParentElement, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::text_layout::ClipConfig;

use super::GitConfirmation;
use super::extras::tool_button;
use crate::appearance::Appearance;
use crate::local_git::LocalBranch;
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::local_workspace::ws_style::{fg_overlay, mouse_state, sized_icon};

const ROW_HEIGHT: f32 = 24.;

/// Local and remote branches, with VS Code's branch actions on hover: checkout, merge into the
/// current branch, branch from here, rename and delete.
#[allow(clippy::too_many_arguments)]
pub(super) fn render(
    branches: &[LocalBranch],
    remote_branches: &[String],
    create_from: Option<&str>,
    rename_branch: Option<&str>,
    branch_name_input: warpui::ViewHandle<crate::editor::EditorView>,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let mut section = Flex::column().with_main_axis_size(MainAxisSize::Min);
    section.add_child(group_label("Branches locais", appearance));
    for (index, branch) in branches.iter().enumerate() {
        section.add_child(local_branch_row(
            index,
            branch,
            operation_pending,
            appearance,
        ));
        if create_from == Some(branch.name.as_str()) {
            section.add_child(branch_form(
                "Criar e mudar",
                WorkspaceAction::CreateLocalGitBranchFrom(branch.name.clone()),
                branch_name_input.clone(),
                appearance,
            ));
        }
        if rename_branch == Some(branch.name.as_str()) {
            section.add_child(branch_form(
                "Renomear",
                WorkspaceAction::RenameLocalGitBranch(branch.name.clone()),
                branch_name_input.clone(),
                appearance,
            ));
        }
    }
    // A branch created from a commit isn't in the list; its form goes after the branches.
    if let Some(source) =
        create_from.filter(|source| !branches.iter().any(|branch| branch.name == *source))
    {
        section.add_child(branch_form(
            "Criar e mudar",
            WorkspaceAction::CreateLocalGitBranchFrom(source.to_owned()),
            branch_name_input.clone(),
            appearance,
        ));
    }
    if !remote_branches.is_empty() {
        section.add_child(group_label("Branches remotas", appearance));
        for (index, remote) in remote_branches.iter().enumerate() {
            section.add_child(remote_branch_row(
                index,
                remote,
                operation_pending,
                appearance,
            ));
        }
    }
    Container::new(section.finish())
        .with_padding_bottom(6.)
        .finish()
}

fn group_label(label: &'static str, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    ConstrainedBox::new(
        Container::new(
            Text::new_inline(label, appearance.ui_font_family(), 12.)
                .with_color(theme.main_text_color(theme.surface_1()).into())
                .finish(),
        )
        .with_padding_left(14.)
        .with_padding_top(4.)
        .finish(),
    )
    .with_height(22.)
    .finish()
}

fn local_branch_row(
    index: usize,
    branch: &LocalBranch,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let state = mouse_state("git-branch-row", index as i64);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.)
        .with_child(sized_icon(
            if branch.is_current {
                Icon::Check
            } else {
                Icon::GitBranch
            },
            theme.sub_text_color(background),
            12.,
        ))
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(branch.name.clone(), appearance.monospace_font_family(), 11.)
                    .with_style(if branch.is_current {
                        Properties::default().weight(Weight::Semibold)
                    } else {
                        Properties::default()
                    })
                    .with_color(theme.main_text_color(background).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );
    if hovered && !operation_pending {
        let name = branch.name.clone();
        let key = index as i64;
        if !branch.is_current {
            row.add_child(tool_button(
                Icon::Check,
                "Fazer checkout",
                WorkspaceAction::CheckoutLocalGitBranch(name.clone()),
                ("git-branch-checkout", key),
                appearance,
            ));
            row.add_child(tool_button(
                Icon::ArrowLeft,
                "Fazer merge na branch atual",
                WorkspaceAction::MergeLocalGitBranch(name.clone()),
                ("git-branch-merge", key),
                appearance,
            ));
        }
        row.add_child(tool_button(
            Icon::Plus,
            "Criar branch a partir desta",
            WorkspaceAction::OpenLocalGitCreateBranch(name.clone()),
            ("git-branch-create", key),
            appearance,
        ));
        row.add_child(tool_button(
            Icon::Pencil,
            "Renomear",
            WorkspaceAction::OpenLocalGitRenameBranch(name.clone()),
            ("git-branch-rename", key),
            appearance,
        ));
        if !branch.is_current {
            row.add_child(tool_button(
                Icon::Trash,
                "Excluir branch",
                WorkspaceAction::RequestLocalGitConfirmation(GitConfirmation::DeleteBranch(name)),
                ("git-branch-delete", key),
                appearance,
            ));
        }
    } else if let Some(upstream) = &branch.upstream {
        row.add_child(
            Text::new_inline(upstream.clone(), appearance.ui_font_family(), 10.)
                .with_color(theme.hint_text_color(background).into())
                .soft_wrap(false)
                .finish(),
        );
    }
    row_frame(state, row.finish(), appearance)
}

fn remote_branch_row(
    index: usize,
    remote: &str,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let state = mouse_state("git-remote-branch-row", index as i64);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.)
        .with_child(sized_icon(
            Icon::Cloud,
            theme.sub_text_color(background),
            12.,
        ))
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(remote.to_owned(), appearance.monospace_font_family(), 11.)
                    .with_color(theme.sub_text_color(background).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );
    if hovered && !operation_pending {
        row.add_child(tool_button(
            Icon::Check,
            "Fazer checkout (cria a branch local)",
            WorkspaceAction::CheckoutRemoteLocalGitBranch(remote.to_owned()),
            ("git-remote-checkout", index as i64),
            appearance,
        ));
    }
    row_frame(state, row.finish(), appearance)
}

fn row_frame(
    state: warpui::elements::MouseStateHandle,
    row: Box<dyn Element>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    Hoverable::new(state, move |mouse| {
        let mut container = Container::new(row)
            .with_padding_left(14.)
            .with_padding_right(8.);
        if mouse.is_mouse_over_element() {
            container = container.with_background(fg_overlay(theme, 5));
        }
        ConstrainedBox::new(container.finish())
            .with_height(ROW_HEIGHT)
            .finish()
    })
    .with_defer_events_to_children()
    .finish()
}

/// Branch name field with its confirm and cancel buttons.
fn branch_form(
    confirm_label: &'static str,
    confirm: WorkspaceAction,
    input: warpui::ViewHandle<crate::editor::EditorView>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    use warpui::ui_components::components::{UiComponent, UiComponentStyles};
    use warpui::ui_components::text_input::TextInput;

    let actions = Flex::row()
        .with_spacing(6.)
        .with_child(super::changes::small_secondary_button(
            Icon::Check,
            confirm_label,
            confirm,
            ("git-branch-form-confirm", 0),
            appearance,
        ))
        .with_child(super::changes::small_secondary_button(
            Icon::X,
            "Cancelar",
            WorkspaceAction::CancelLocalGitCreateBranch,
            ("git-branch-form-cancel", 0),
            appearance,
        ))
        .finish();
    Container::new(
        Flex::column()
            .with_spacing(6.)
            .with_child(
                TextInput::new(input, UiComponentStyles::default().set_height(28.))
                    .build()
                    .finish(),
            )
            .with_child(actions)
            .finish(),
    )
    .with_background(fg_overlay(appearance.theme(), 5))
    .with_padding_left(14.)
    .with_padding_right(14.)
    .with_padding_top(6.)
    .with_padding_bottom(8.)
    .finish()
}
