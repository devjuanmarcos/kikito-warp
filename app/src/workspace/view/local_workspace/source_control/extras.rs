//! Panel pieces for the Git flows beyond the kit's mockup (VS Code parity): stashes, merges,
//! confirmations, commit actions and repository initialization.

use warp_core::ui::Icon;
use warpui::elements::{
    ConstrainedBox, Container, CrossAxisAlignment, Element, Expanded, Flex, Hoverable,
    MainAxisSize, ParentElement, Shrinkable, Text,
};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;

use super::changes::{count_badge, section_header, small_secondary_button};
use super::{GitConfirmation, GitPanelSection, SourceControl};
use crate::appearance::Appearance;
use crate::local_git::{CommitFile, LocalCommit, LocalStash, PendingOperation};
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::local_workspace::ws_style::{
    TooltipSide, fg_overlay, mouse_state, radius, sized_icon, tooltip, with_tooltip,
};

const ROW_HEIGHT: f32 = 24.;

/// 20px icon button with a tooltip, like the kit's `GPIconBtn`.
pub(super) fn tool_button(
    icon: Icon,
    tip: &'static str,
    action: WorkspaceAction,
    key: (&'static str, i64),
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let state = mouse_state(key.0, key.1);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let button = Hoverable::new(state, move |mouse| {
        let over = mouse.is_mouse_over_element();
        let color = if over {
            theme.main_text_color(theme.surface_1())
        } else {
            theme.sub_text_color(theme.surface_1())
        };
        let mut button = Container::new(
            ConstrainedBox::new(sized_icon(icon, color, 14.))
                .with_width(14.)
                .with_height(14.)
                .finish(),
        )
        .with_uniform_padding(3.)
        .with_corner_radius(radius(4.));
        if over {
            button = button.with_background(fg_overlay(theme, 10));
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
    .finish();
    if hovered {
        with_tooltip(button, tooltip(appearance, tip), TooltipSide::BelowRight)
    } else {
        button
    }
}

/// Inline confirmation for a destructive action: the consequence, then confirm and cancel.
pub(super) fn confirmation_row(
    confirmation: &GitConfirmation,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let (message, confirm_label, confirm_icon) = match confirmation {
        GitConfirmation::DiscardAll => (
            "Descartar todas as alterações? Os arquivos rastreados voltam ao último commit e os não rastreados são apagados do disco."
                .to_owned(),
            "Descartar tudo",
            Icon::Retry,
        ),
        GitConfirmation::DeleteBranch(branch) => (
            format!("Excluir a branch local {branch}? Commits que só existem nela podem se perder."),
            "Excluir",
            Icon::Trash,
        ),
        GitConfirmation::DropStash(index) => (
            format!("Descartar o stash {index}? As alterações guardadas nele serão perdidas."),
            "Descartar",
            Icon::Trash,
        ),
    };
    let mut actions = Flex::row().with_spacing(6.);
    if !operation_pending {
        actions.add_child(small_secondary_button(
            confirm_icon,
            confirm_label,
            WorkspaceAction::ConfirmLocalGitAction,
            ("git-confirm", 0),
            appearance,
        ));
    }
    actions.add_child(small_secondary_button(
        Icon::X,
        "Cancelar",
        WorkspaceAction::CancelLocalGitConfirmation,
        ("git-confirm-cancel", 0),
        appearance,
    ));
    Container::new(
        Flex::column()
            .with_spacing(6.)
            .with_child(
                Text::new(message, appearance.ui_font_family(), 11.)
                    .with_color(theme.ui_error_color())
                    .finish(),
            )
            .with_child(actions.finish())
            .finish(),
    )
    .with_background(fg_overlay(theme, 5))
    .with_padding_left(14.)
    .with_padding_right(14.)
    .with_padding_top(8.)
    .with_padding_bottom(8.)
    .finish()
}

/// Banner of a merge, cherry-pick or revert Git stopped in the middle of.
pub(super) fn merge_banner(pending: PendingOperation, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let (message, abort_label) = match pending {
        PendingOperation::Merge => (
            "Merge em andamento. Resolva os conflitos (aceite um lado ou edite o arquivo e prepare) e faça o commit, ou aborte o merge.",
            "Abortar merge",
        ),
        PendingOperation::CherryPick => (
            "Cherry-pick em andamento. Resolva os conflitos e faça o commit, ou aborte o cherry-pick.",
            "Abortar cherry-pick",
        ),
        PendingOperation::Revert => (
            "Reversão em andamento. Resolva os conflitos e faça o commit, ou aborte a reversão.",
            "Abortar reversão",
        ),
    };
    Container::new(
        Flex::column()
            .with_spacing(6.)
            .with_child(
                Text::new(message, appearance.ui_font_family(), 11.)
                    .with_color(theme.ui_yellow_color())
                    .finish(),
            )
            .with_child(small_secondary_button(
                Icon::X,
                abort_label,
                WorkspaceAction::AbortLocalGitMerge,
                ("git-merge-abort", 0),
                appearance,
            ))
            .finish(),
    )
    .with_background(fg_overlay(theme, 5))
    .with_padding_left(14.)
    .with_padding_right(14.)
    .with_padding_top(8.)
    .with_padding_bottom(8.)
    .finish()
}

/// Shown instead of the changes when the project folder has no repository.
pub(super) fn init_prompt(appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    Container::new(
        Flex::column()
            .with_spacing(8.)
            .with_child(
                Text::new(
                    "Esta pasta não contém um repositório Git.",
                    appearance.ui_font_family(),
                    12.,
                )
                .with_color(theme.sub_text_color(theme.surface_1()).into())
                .finish(),
            )
            .with_child(small_secondary_button(
                Icon::Plus,
                "Inicializar repositório",
                WorkspaceAction::InitLocalGitRepository,
                ("git-init", 0),
                appearance,
            ))
            .finish(),
    )
    .with_padding_left(14.)
    .with_padding_right(14.)
    .with_padding_top(10.)
    .with_padding_bottom(10.)
    .finish()
}

/// VS Code's "Stashes" view: stash the changes, then apply, pop or drop each entry.
pub(super) fn stash_section(
    source_control: &SourceControl<'_>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let has_changes = source_control
        .status
        .is_some_and(|status| status.changed_path_count() > 0);
    let mut right = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(1.);
    if has_changes && !source_control.operation_pending {
        right.add_child(tool_button(
            Icon::Download01,
            "Guardar alterações no stash",
            WorkspaceAction::StashLocalGit {
                include_untracked: false,
            },
            ("git-stash", 0),
            appearance,
        ));
        right.add_child(tool_button(
            Icon::Plus,
            "Guardar incluindo não rastreados",
            WorkspaceAction::StashLocalGit {
                include_untracked: true,
            },
            ("git-stash-untracked", 0),
            appearance,
        ));
    }
    if !source_control.stashes.is_empty() {
        right.add_child(
            Container::new(count_badge(source_control.stashes.len(), appearance))
                .with_margin_left(4.)
                .finish(),
        );
    }
    let collapsed = source_control.panel.stashes_collapsed;
    let mut section = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(section_header(
            "STASHES",
            GitPanelSection::Stashes,
            collapsed,
            right.finish(),
            appearance,
        ));
    if collapsed {
        return section.finish();
    }
    if let Some(confirmation @ GitConfirmation::DropStash(_)) = source_control.confirmation {
        section.add_child(confirmation_row(
            confirmation,
            source_control.operation_pending,
            appearance,
        ));
    }
    if source_control.stashes.is_empty() {
        section.add_child(
            Container::new(
                Text::new_inline("Nenhum stash guardado.", appearance.ui_font_family(), 12.)
                    .with_color(theme.hint_text_color(background).into())
                    .finish(),
            )
            .with_padding_left(14.)
            .with_padding_top(4.)
            .with_padding_bottom(8.)
            .finish(),
        );
        return section.finish();
    }
    for stash in source_control.stashes {
        section.add_child(stash_row(
            stash,
            source_control.operation_pending,
            appearance,
        ));
    }
    Container::new(section.finish())
        .with_padding_bottom(6.)
        .finish()
}

fn stash_row(
    stash: &LocalStash,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let state = mouse_state("git-stash-row", stash.index as i64);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.)
        .with_child(sized_icon(
            Icon::Bookmark,
            theme.sub_text_color(background),
            12.,
        ))
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(stash.message.clone(), appearance.ui_font_family(), 12.)
                    .with_color(theme.main_text_color(background).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );
    if hovered && !operation_pending {
        let index = stash.index;
        row.add_child(tool_button(
            Icon::Check,
            "Aplicar",
            WorkspaceAction::ApplyLocalGitStash { index, pop: false },
            ("git-stash-apply", index as i64),
            appearance,
        ));
        row.add_child(tool_button(
            Icon::Upload,
            "Aplicar e remover (pop)",
            WorkspaceAction::ApplyLocalGitStash { index, pop: true },
            ("git-stash-pop", index as i64),
            appearance,
        ));
        row.add_child(tool_button(
            Icon::Trash,
            "Descartar stash",
            WorkspaceAction::RequestLocalGitConfirmation(GitConfirmation::DropStash(index)),
            ("git-stash-drop", index as i64),
            appearance,
        ));
    } else if let Ok(time) = chrono::DateTime::parse_from_rfc3339(&stash.created_at) {
        row.add_child(
            Text::new_inline(
                super::super::home::relative_time(time.timestamp_millis()),
                appearance.ui_font_family(),
                11.,
            )
            .with_color(theme.hint_text_color(background).into())
            .finish(),
        );
    }
    Hoverable::new(state, move |mouse| {
        let mut container = Container::new(row.finish())
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

/// Actions of an expanded graph commit, like the commit context menu in VS Code's graph.
pub(super) fn commit_actions(commit: &LocalCommit, appearance: &Appearance) -> Box<dyn Element> {
    let hash = commit.hash.clone();
    let short = commit.hash.chars().take(7).collect::<String>();
    let key = stable_commit_key(&commit.hash);
    Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(2.)
        .with_child(tool_button(
            Icon::Copy,
            "Copiar hash",
            WorkspaceAction::CopyLocalGitText(hash.clone()),
            ("git-commit-copy-hash", key),
            appearance,
        ))
        .with_child(tool_button(
            Icon::Pencil,
            "Copiar mensagem",
            WorkspaceAction::CopyLocalGitText(commit.subject.clone()),
            ("git-commit-copy-message", key),
            appearance,
        ))
        .with_child(tool_button(
            Icon::GitBranch,
            "Criar branch a partir deste commit",
            WorkspaceAction::OpenLocalGitCreateBranch(short),
            ("git-commit-branch", key),
            appearance,
        ))
        .with_child(tool_button(
            Icon::Download01,
            "Aplicar na branch atual (cherry-pick)",
            WorkspaceAction::CherryPickLocalGitCommit(hash.clone()),
            ("git-commit-cherry-pick", key),
            appearance,
        ))
        .with_child(tool_button(
            Icon::Retry,
            "Reverter commit",
            WorkspaceAction::RevertLocalGitCommit(hash),
            ("git-commit-revert", key),
            appearance,
        ))
        .finish()
}

/// Files a commit changed, with their status letter.
pub(super) fn commit_files(files: &[CommitFile], appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let mut list = Flex::column().with_main_axis_size(MainAxisSize::Min);
    for file in files {
        let (dir, name) = match file.path.rsplit_once('/') {
            Some((dir, name)) => (dir.to_owned(), name.to_owned()),
            None => (String::new(), file.path.clone()),
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.)
            .with_child(
                Text::new_inline(name, appearance.ui_font_family(), 11.)
                    .with_color(theme.main_text_color(background).into())
                    .soft_wrap(false)
                    .finish(),
            );
        if !dir.is_empty() {
            row.add_child(
                Shrinkable::new(
                    1.,
                    Text::new_inline(dir, appearance.ui_font_family(), 10.)
                        .with_color(theme.hint_text_color(background).into())
                        .soft_wrap(false)
                        .with_clip(ClipConfig::ellipsis())
                        .finish(),
                )
                .finish(),
            );
        }
        row.add_child(Expanded::new(1., Flex::row().finish()).finish());
        row.add_child(
            Text::new_inline(
                file.status.to_string(),
                appearance.monospace_font_family(),
                11.,
            )
            .with_color(theme.sub_text_color(background).into())
            .finish(),
        );
        list.add_child(ConstrainedBox::new(row.finish()).with_height(20.).finish());
    }
    list.finish()
}

fn stable_commit_key(hash: &str) -> i64 {
    i64::from_str_radix(&hash.chars().take(12).collect::<String>(), 16).unwrap_or_default()
}
