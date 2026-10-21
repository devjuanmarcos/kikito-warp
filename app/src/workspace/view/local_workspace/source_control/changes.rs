use std::collections::HashSet;

use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{ChangedFile, LocalBranch, LocalGitDiffKind, RepositoryStatus};
use crate::workspace::action::WorkspaceAction;
use warpui::ViewHandle;
use warpui::elements::new_scrollable::{NewScrollable, ScrollableAppearance, SingleAxisConfig};
use warpui::elements::{
    ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult,
    Element, EventHandler, Fill, Flex, MainAxisSize, ParentElement, ScrollbarWidth, Shrinkable,
    Text,
};

struct ChangeGroup {
    title: &'static str,
    files: Vec<ChangedFile>,
    action: Option<ChangeAction>,
    diff_kind: LocalGitDiffKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChangeAction {
    Stage,
    Unstage,
}

fn change_groups(status: &RepositoryStatus) -> [ChangeGroup; 4] {
    let conflicted = status
        .conflicted
        .iter()
        .map(|file| file.path.as_str())
        .collect::<HashSet<_>>();
    [
        ChangeGroup {
            title: "Alterações preparadas",
            files: status
                .staged
                .iter()
                .filter(|file| !conflicted.contains(file.path.as_str()))
                .cloned()
                .collect(),
            action: Some(ChangeAction::Unstage),
            diff_kind: LocalGitDiffKind::Staged,
        },
        ChangeGroup {
            title: "Alterações",
            files: status
                .unstaged
                .iter()
                .filter(|file| !conflicted.contains(file.path.as_str()))
                .cloned()
                .collect(),
            action: Some(ChangeAction::Stage),
            diff_kind: LocalGitDiffKind::Unstaged,
        },
        ChangeGroup {
            title: "Arquivos não rastreados",
            files: status.untracked.clone(),
            action: Some(ChangeAction::Stage),
            diff_kind: LocalGitDiffKind::Untracked,
        },
        ChangeGroup {
            title: "Conflitos",
            files: status.conflicted.clone(),
            action: None,
            diff_kind: LocalGitDiffKind::Unstaged,
        },
    ]
}

pub(super) fn render(
    status: Option<&RepositoryStatus>,
    branches: &[LocalBranch],
    branch_menu: Option<&str>,
    create_branch_from: Option<&str>,
    rename_branch: Option<&str>,
    branch_name_input: ViewHandle<EditorView>,
    error: Option<&str>,
    commit_input: ViewHandle<EditorView>,
    diff_preview: Option<&super::DiffPreview>,
    amend_confirmation: bool,
    undo_confirmation: bool,
    revert_confirmation: bool,
    discard_confirmation: Option<(&str, bool)>,
    operation_pending: bool,
    scroll_state: ClippedScrollStateHandle,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    // Lives inside a vertical scrollable, whose unbounded height a max-sized column can't fill.
    let mut panel = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    panel.add_child(
        Container::new(
            Text::new_inline("CONTROLE DE CÓDIGO-FONTE", appearance.ui_font_family(), 10.)
                .with_color(theme.sub_text_color(theme.surface_1()).into())
                .finish(),
        )
        .with_padding_left(14.)
        .with_padding_right(10.)
        .with_padding_top(14.)
        .with_padding_bottom(8.)
        .finish(),
    );
    if operation_pending {
        panel.add_child(message(
            "Aplicando operação local do Git…",
            appearance,
            false,
        ));
    }
    if let Some((path, untracked)) = discard_confirmation {
        panel.add_child(discard_confirmation_row(
            path,
            untracked,
            operation_pending,
            appearance,
        ));
    }
    if let Some(error) = error {
        panel.add_child(message(error, appearance, true));
    } else if let Some(status) = status {
        let head = match &status.head {
            crate::local_git::HeadState::Branch(name) => name.as_str(),
            crate::local_git::HeadState::Detached => "HEAD separado",
            crate::local_git::HeadState::Unborn => "sem commits",
        };
        panel.add_child(message(head, appearance, false));
        panel.add_child(super::refs::render(
            branches,
            branch_menu,
            create_branch_from,
            rename_branch,
            branch_name_input,
            appearance,
        ));
        let groups = change_groups(status);
        if groups.iter().all(|group| group.files.is_empty()) {
            panel.add_child(message("Nenhuma alteração detectada", appearance, false));
        }
        for group in groups {
            if group.files.is_empty() {
                continue;
            }
            panel.add_child(group_header(&group, operation_pending, appearance));
            for file in group.files {
                panel.add_child(file_row(
                    file,
                    group.action,
                    group.diff_kind,
                    operation_pending,
                    appearance,
                ));
            }
        }
    } else {
        panel.add_child(message("Detectando repositório Git…", appearance, false));
    }
    if let Some(preview) = diff_preview {
        panel.add_child(super::diff::render(preview, operation_pending, appearance));
    }
    let scrollable = NewScrollable::vertical(
        SingleAxisConfig::Clipped {
            handle: scroll_state,
            child: panel.finish(),
        },
        theme.sub_text_color(theme.surface_1()).into(),
        theme.font_color(theme.surface_1()).into(),
        Fill::None,
    )
    .with_vertical_scrollbar(ScrollableAppearance::new(ScrollbarWidth::Auto, true))
    .finish();
    let mut content = Flex::column().with_main_axis_size(MainAxisSize::Max);
    content.add_child(Shrinkable::new(1., scrollable).finish());
    content.add_child(super::commit::render(
        status,
        commit_input,
        amend_confirmation,
        undo_confirmation,
        revert_confirmation,
        operation_pending,
        appearance,
    ));
    ConstrainedBox::new(
        Container::new(content.finish())
            .with_background(theme.surface_1())
            .with_border(warpui::elements::Border::left(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_width(320.)
    .finish()
}

fn group_header(
    group: &ChangeGroup,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut header = Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);
    header.add_child(
        Shrinkable::new(
            1.,
            Text::new_inline(
                format!("{} · {}", group.title, group.files.len()),
                appearance.ui_font_family(),
                11.,
            )
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish(),
        )
        .finish(),
    );
    if let Some(action) = group.action.filter(|_| !operation_pending) {
        let (glyph, action) = match action {
            ChangeAction::Stage => ("+", WorkspaceAction::StageAllLocalGit),
            ChangeAction::Unstage => ("−", WorkspaceAction::UnstageAllLocalGit),
        };
        header.add_child(
            EventHandler::new(
                Text::new_inline(glyph, appearance.ui_font_family(), 14.)
                    .with_color(theme.font_color(theme.surface_1()).into())
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );
    }
    Container::new(header.finish())
        .with_background(theme.surface_2())
        .with_padding_left(12.)
        .with_padding_right(8.)
        .with_padding_top(7.)
        .with_padding_bottom(7.)
        .finish()
}

fn message(text: &str, appearance: &Appearance, is_error: bool) -> Box<dyn Element> {
    let theme = appearance.theme();
    Container::new(
        Text::new_inline(text.to_owned(), appearance.ui_font_family(), 11.)
            .with_color(if is_error {
                theme.ui_error_color().into()
            } else {
                theme.sub_text_color(theme.surface_1()).into()
            })
            .finish(),
    )
    .with_padding_left(12.)
    .with_padding_right(12.)
    .with_padding_top(8.)
    .with_padding_bottom(8.)
    .finish()
}

fn file_row(
    file: ChangedFile,
    action: Option<ChangeAction>,
    diff_kind: LocalGitDiffKind,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let row_path = file.path.clone();
    let mut row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.);
    let status = format!("{}{}", file.index_status, file.worktree_status);
    row.add_child(
        Text::new_inline(status, appearance.ui_font_family(), 10.)
            .with_color(theme.sub_text_color(theme.surface_1()).into())
            .finish(),
    );
    row.add_child(
        Shrinkable::new(
            1.,
            Text::new_inline(file.path.clone(), appearance.ui_font_family(), 11.)
                .with_color(theme.font_color(theme.surface_1()).into())
                .finish(),
        )
        .finish(),
    );
    if let Some(action) = action.filter(|_| !operation_pending) {
        let git_path = file.path.clone();
        let action = match action {
            ChangeAction::Stage => WorkspaceAction::StageLocalGitPath(git_path),
            ChangeAction::Unstage => WorkspaceAction::UnstageLocalGitPath(git_path),
        };
        let glyph = match &action {
            WorkspaceAction::StageLocalGitPath(_) => "+",
            WorkspaceAction::UnstageLocalGitPath(_) => "−",
            _ => unreachable!(),
        };
        row.add_child(
            EventHandler::new(
                Text::new_inline(glyph, appearance.ui_font_family(), 14.)
                    .with_color(theme.font_color(theme.surface_1()).into())
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );
    }
    if !operation_pending && can_discard(action, diff_kind) {
        let path = file.path.clone();
        let untracked = diff_kind == LocalGitDiffKind::Untracked;
        row.add_child(
            EventHandler::new(
                Text::new_inline("×", appearance.ui_font_family(), 14.)
                    .with_color(theme.ui_error_color().into())
                    .finish(),
            )
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::RequestDiscardLocalGitPath {
                    path: path.clone(),
                    untracked,
                });
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );
    }
    EventHandler::new(
        Container::new(row.finish())
            .with_padding_left(12.)
            .with_padding_right(12.)
            .with_padding_top(5.)
            .with_padding_bottom(5.)
            .finish(),
    )
    .on_left_mouse_up(move |ctx, _, _| {
        ctx.dispatch_typed_action(WorkspaceAction::OpenLocalGitDiff {
            path: row_path.clone(),
            staged: diff_kind == LocalGitDiffKind::Staged,
            untracked: diff_kind == LocalGitDiffKind::Untracked,
        });
        DispatchEventResult::StopPropagation
    })
    .finish()
}

fn can_discard(action: Option<ChangeAction>, diff_kind: LocalGitDiffKind) -> bool {
    action == Some(ChangeAction::Stage)
        && matches!(
            diff_kind,
            LocalGitDiffKind::Unstaged | LocalGitDiffKind::Untracked
        )
}

fn discard_confirmation_row(
    path: &str,
    untracked: bool,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let detail = if untracked {
        "O arquivo será removido do disco."
    } else {
        "As alterações não preparadas serão descartadas."
    };
    let mut content = Flex::column().with_spacing(6.);
    content.add_child(
        Text::new_inline(
            format!("Descartar {path}? {detail}"),
            appearance.ui_font_family(),
            10.,
        )
        .with_color(theme.ui_error_color().into())
        .finish(),
    );
    let mut actions = Flex::row().with_spacing(6.);
    actions.add_child(confirmation_button(
        "Descartar",
        WorkspaceAction::ConfirmDiscardLocalGitPath,
        !operation_pending,
        appearance,
    ));
    actions.add_child(confirmation_button(
        "Cancelar",
        WorkspaceAction::CancelDiscardLocalGitPath,
        true,
        appearance,
    ));
    content.add_child(actions.finish());
    Container::new(content.finish())
        .with_background(theme.surface_2())
        .with_uniform_padding(8.)
        .finish()
}

fn confirmation_button(
    label: &'static str,
    action: WorkspaceAction,
    enabled: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let button = Container::new(
        Text::new_inline(label, appearance.ui_font_family(), 10.)
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish(),
    )
    .with_uniform_padding(5.)
    .finish();
    if enabled {
        EventHandler::new(button)
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    } else {
        button
    }
}

#[cfg(test)]
#[path = "changes_tests.rs"]
mod tests;
