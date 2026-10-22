use std::collections::HashSet;

use pathfinder_color::ColorU;
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill;
use warpui::elements::new_scrollable::{NewScrollable, ScrollableAppearance, SingleAxisConfig};
use warpui::elements::{
    Axis, Border, ConstrainedBox, Container, CrossAxisAlignment, Element, Expanded, Flex,
    Hoverable, MainAxisAlignment, MainAxisSize, ParentElement, ScrollbarWidth, Shrinkable, Text,
    Wrap,
};
use warpui::fonts::{Properties, Weight};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;

use super::graph::{self, GraphColors, GraphRow};
use super::{GitPanelSection, GitRemoteAction, SourceControl};
use crate::appearance::Appearance;
use crate::local_git::{ChangedFile, HeadState, LocalCommit, LocalGitDiffKind, RepositoryStatus};
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::local_workspace::ws_style::{
    TooltipSide, fg_overlay, mouse_state, naked_icon_button, neutral, radius, sized_icon,
};

// The kit's heights exclude their 1px borders (CSS content-box), so each bordered bar is one
// pixel taller here.
const HEADER_HEIGHT: f32 = 35.;
const SECTION_HEADER_HEIGHT: f32 = 27.;
const LIST_HEADER_HEIGHT: f32 = 22.;
const FILE_ROW_HEIGHT: f32 = 24.;
const FOOTER_HEIGHT: f32 = 29.;
const VISIBLE_REFS: usize = 2;

struct ChangeGroup {
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
            files: status.untracked.clone(),
            action: Some(ChangeAction::Stage),
            diff_kind: LocalGitDiffKind::Untracked,
        },
        ChangeGroup {
            files: status.conflicted.clone(),
            action: None,
            diff_kind: LocalGitDiffKind::Unstaged,
        },
    ]
}

fn can_discard(action: Option<ChangeAction>, diff_kind: LocalGitDiffKind) -> bool {
    action == Some(ChangeAction::Stage)
        && matches!(
            diff_kind,
            LocalGitDiffKind::Unstaged | LocalGitDiffKind::Untracked
        )
}

/// Kit status letter of a file in a list: staged files show the index side, the rest the
/// worktree side, untracked files `U` and conflicts `!`.
fn status_letter(file: &ChangedFile, diff_kind: LocalGitDiffKind, conflict: bool) -> char {
    if conflict {
        return '!';
    }
    let letter = match diff_kind {
        LocalGitDiffKind::Staged => file.index_status,
        LocalGitDiffKind::Unstaged => file.worktree_status,
        LocalGitDiffKind::Untracked => 'U',
    };
    if letter == '.' { 'M' } else { letter }
}

fn head_name(status: &RepositoryStatus) -> &str {
    match &status.head {
        HeadState::Branch(name) => name.as_str(),
        HeadState::Detached => "HEAD",
        HeadState::Unborn => "sem commits",
    }
}

pub(super) fn render(
    source_control: SourceControl<'_>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let mut sections = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    if source_control.repositories.len() > 1 {
        sections.add_child(repositories_section(&source_control, appearance));
    }
    sections.add_child(changes_section(&source_control, appearance));
    sections.add_child(graph_section(&source_control, appearance));
    let scrollable = NewScrollable::vertical(
        SingleAxisConfig::Clipped {
            handle: source_control.scroll_state.clone(),
            child: sections.finish(),
        },
        theme.sub_text_color(background).into(),
        theme.font_color(background).into(),
        warpui::elements::Fill::None,
    )
    .with_vertical_scrollbar(ScrollableAppearance::new(ScrollbarWidth::Auto, true))
    .finish();

    let panel = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(header(&source_control, appearance))
        .with_child(Expanded::new(1., scrollable).finish())
        .with_child(footer(&source_control, appearance))
        .finish();
    ConstrainedBox::new(
        Container::new(panel)
            .with_background(background)
            .with_border(Border::left(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_width(source_control.width)
    .finish()
}

fn header(source_control: &SourceControl<'_>, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.)
        .with_child(sized_icon(
            Icon::GitBranch,
            theme.sub_text_color(background),
            14.,
        ))
        .with_child(
            Text::new_inline("Controle de versão", appearance.ui_font_family(), 12.)
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(theme.main_text_color(background).into())
                .finish(),
        )
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(
                    source_control.project_name.to_owned(),
                    appearance.ui_font_family(),
                    11.,
                )
                .with_color(theme.hint_text_color(background).into())
                .soft_wrap(false)
                .with_clip(ClipConfig::ellipsis())
                .finish(),
            )
            .finish(),
        )
        .with_child(naked_icon_button(
            appearance,
            Icon::X,
            theme.foreground(),
            None,
            WorkspaceAction::ToggleLocalWorkspaceSourceControl,
            ("git-close", 0),
            Some(("Fechar painel", TooltipSide::BelowRight)),
        ))
        .finish();
    ConstrainedBox::new(
        Container::new(row)
            .with_padding_top(1.)
            .with_padding_left(12.)
            .with_padding_right(6.)
            .with_border(Border::bottom(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_height(HEADER_HEIGHT)
    .finish()
}

/// Collapsible section header: chevron, small caps title and controls on the right.
fn section_header(
    title: &'static str,
    section: GitPanelSection,
    collapsed: bool,
    right: Box<dyn Element>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let sub = theme.sub_text_color(theme.surface_1());
    let title_row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.)
        .with_child(sized_icon(
            if collapsed {
                Icon::ChevronRight
            } else {
                Icon::ChevronDown
            },
            sub,
            12.,
        ))
        .with_child(tracked_title(title, sub, appearance))
        .finish();
    // Only the title area toggles the section, so clicks on the controls at the right (branch,
    // fetch, pull, push) don't collapse it.
    let toggle = Hoverable::new(mouse_state("git-section", section as i64), |_| {
        ConstrainedBox::new(title_row)
            .with_height(SECTION_HEADER_HEIGHT - 1.)
            .finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| {
        ctx.dispatch_typed_action(WorkspaceAction::ToggleLocalGitPanelSection(section))
    })
    .finish();
    let row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.)
        .with_child(Expanded::new(1., toggle).finish())
        .with_child(right)
        .finish();
    ConstrainedBox::new(
        Container::new(row)
            .with_padding_left(6.)
            .with_padding_right(8.)
            .with_border(Border::top(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_height(SECTION_HEADER_HEIGHT)
    .finish()
}

/// Section title with the kit's `letter-spacing: .04em`, which `Text` can't express: one
/// glyph per child, spaced by that amount.
fn tracked_title(title: &'static str, color: Fill, appearance: &Appearance) -> Box<dyn Element> {
    let mut letters = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(TITLE_TRACKING);
    for letter in title.chars() {
        letters.add_child(
            Text::new_inline(letter.to_string(), appearance.ui_font_family(), 10.)
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(color.into())
                .finish(),
        );
    }
    letters.finish()
}

const TITLE_TRACKING: f32 = 0.4;

/// Kit `GPIconBtn`: 20px, sub color turning main with a fg-overlay-2 background on hover.
fn icon_button(
    icon: Icon,
    badge: Option<String>,
    busy: bool,
    action: WorkspaceAction,
    key: (&'static str, i64),
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    Hoverable::new(mouse_state(key.0, key.1), move |mouse| {
        let hovered = mouse.is_mouse_over_element();
        let color = if hovered || busy {
            theme.main_text_color(theme.surface_1())
        } else {
            theme.sub_text_color(theme.surface_1())
        };
        let mut row = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(2.)
            .with_child(sized_icon(icon, color, 14.));
        if let Some(badge) = &badge {
            row.add_child(
                Text::new_inline(badge.clone(), appearance.ui_font_family(), 10.)
                    .with_color(color.into())
                    .finish(),
            );
        }
        let mut button =
            Container::new(ConstrainedBox::new(row.finish()).with_height(20.).finish())
                .with_padding_left(3.)
                .with_padding_right(3.)
                .with_corner_radius(radius(4.));
        if hovered || busy {
            button = button.with_background(fg_overlay(theme, 10));
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
    .finish()
}

fn changes_section(
    source_control: &SourceControl<'_>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let sub = theme.sub_text_color(background);
    let panel = source_control.panel;
    let busy = |action| panel.remote_busy == Some(action);
    let open_diff = source_control
        .diff_preview
        .and_then(|preview| preview.path.as_deref().map(|path| (path, preview.staged)));
    let mut right = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(1.);
    if let Some(status) = source_control.status {
        let branch = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(3.)
            .with_child(sized_icon(Icon::GitBranch, sub, 11.))
            .with_child(
                Shrinkable::new(
                    1.,
                    Text::new_inline(
                        head_name(status).to_owned(),
                        appearance.monospace_font_family(),
                        11.,
                    )
                    .with_color(sub.into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
                )
                .finish(),
            )
            .finish();
        right.add_child(
            Container::new(
                Hoverable::new(mouse_state("git-branch-label", 0), |_| {
                    ConstrainedBox::new(branch).with_max_width(120.).finish()
                })
                .with_cursor(Cursor::PointingHand)
                .on_click(|ctx, _, _| {
                    ctx.dispatch_typed_action(WorkspaceAction::ToggleLocalGitBranches)
                })
                .finish(),
            )
            .with_margin_right(4.)
            .finish(),
        );
        let ahead = status
            .upstream
            .map(|upstream| upstream.ahead)
            .filter(|ahead| *ahead > 0);
        right.add_child(icon_button(
            Icon::Refresh,
            None,
            busy(GitRemoteAction::Fetch),
            WorkspaceAction::RunLocalGitRemote(GitRemoteAction::Fetch),
            ("git-fetch", 0),
            appearance,
        ));
        right.add_child(icon_button(
            Icon::Download01,
            None,
            busy(GitRemoteAction::Pull),
            WorkspaceAction::RunLocalGitRemote(GitRemoteAction::Pull),
            ("git-pull", 0),
            appearance,
        ));
        right.add_child(icon_button(
            Icon::Upload,
            ahead.map(|ahead| ahead.to_string()),
            busy(GitRemoteAction::Push),
            WorkspaceAction::RunLocalGitRemote(GitRemoteAction::Push),
            ("git-push", 0),
            appearance,
        ));
    }
    let mut section = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(section_header(
            "ALTERAÇÕES",
            GitPanelSection::Changes,
            panel.changes_collapsed,
            right.finish(),
            appearance,
        ));
    if panel.changes_collapsed {
        return section.finish();
    }
    if panel.branches_open {
        section.add_child(super::refs::render(
            source_control.branches,
            source_control.branch_menu,
            source_control.create_branch_from,
            source_control.rename_branch,
            source_control.branch_name_input.clone(),
            appearance,
        ));
    }
    if let Some((path, untracked)) = source_control.discard_confirmation {
        section.add_child(discard_confirmation_row(
            path,
            untracked,
            source_control.operation_pending,
            appearance,
        ));
    }
    let Some(status) = source_control.status else {
        section.add_child(message(
            source_control
                .error
                .unwrap_or("Detectando repositório Git…"),
            appearance,
            source_control.error.is_some(),
        ));
        return section.finish();
    };
    section.add_child(super::commit::render(source_control, status, appearance));
    if let Some(error) = source_control.error {
        section.add_child(message(error, appearance, true));
    }

    let groups = change_groups(status);
    let [staged, unstaged, untracked, conflicted] = groups;
    let pending = source_control.operation_pending;
    if !staged.files.is_empty() {
        section.add_child(list_header(
            "Preparadas",
            staged.files.len(),
            Some((Icon::Minus, WorkspaceAction::UnstageAllLocalGit)).filter(|_| !pending),
            ("git-unstage-all", 0),
            appearance,
        ));
        for file in &staged.files {
            section.add_child(file_row(
                file, &staged, false, pending, open_diff, appearance,
            ));
        }
    }
    let changes_count = unstaged.files.len() + untracked.files.len() + conflicted.files.len();
    let mut changes = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(list_header(
            "Alterações",
            changes_count,
            Some((Icon::Plus, WorkspaceAction::StageAllLocalGit))
                .filter(|_| !pending && changes_count > conflicted.files.len()),
            ("git-stage-all", 0),
            appearance,
        ));
    for file in &conflicted.files {
        changes.add_child(file_row(
            file,
            &conflicted,
            true,
            pending,
            open_diff,
            appearance,
        ));
    }
    for group in [&unstaged, &untracked] {
        for file in &group.files {
            changes.add_child(file_row(file, group, false, pending, open_diff, appearance));
        }
    }
    if staged.files.is_empty() && changes_count == 0 {
        changes.add_child(
            Container::new(
                Text::new_inline(
                    "Nenhuma alteração. Árvore de trabalho limpa.",
                    appearance.ui_font_family(),
                    12.,
                )
                .with_color(theme.hint_text_color(background).into())
                .finish(),
            )
            .with_padding_left(14.)
            .with_padding_right(14.)
            .with_padding_top(4.)
            .with_padding_bottom(4.)
            .finish(),
        );
    }
    section.add_child(
        Container::new(changes.finish())
            .with_padding_bottom(8.)
            .finish(),
    );
    if let Some(preview) = source_control.diff_preview {
        section.add_child(super::diff::render(preview, pending, appearance));
    }
    section.finish()
}

/// "Preparadas" / "Alterações" row: label, bulk action and count badge.
fn list_header(
    label: &'static str,
    count: usize,
    bulk: Option<(Icon, WorkspaceAction)>,
    key: (&'static str, i64),
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(label, appearance.ui_font_family(), 12.)
                    .with_color(theme.main_text_color(background).into())
                    .finish(),
            )
            .finish(),
        );
    if let Some((icon, action)) = bulk {
        row.add_child(icon_button(icon, None, false, action, key, appearance));
    }
    row.add_child(
        Container::new(count_badge(count, appearance))
            .with_margin_left(4.)
            .finish(),
    );
    ConstrainedBox::new(
        Container::new(row.finish())
            .with_padding_left(14.)
            .with_padding_right(8.)
            .finish(),
    )
    .with_height(LIST_HEADER_HEIGHT)
    .finish()
}

fn file_row(
    file: &ChangedFile,
    group: &ChangeGroup,
    conflict: bool,
    operation_pending: bool,
    open_diff: Option<(&str, bool)>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let letter = status_letter(file, group.diff_kind, conflict);
    let letter_color: Fill = match letter {
        'A' | 'U' => Fill::Solid(theme.ui_green_color()),
        'D' | '!' => Fill::Solid(theme.terminal_colors().normal.red.into()),
        'M' | 'R' | 'C' | 'T' => Fill::Solid(theme.ui_yellow_color()),
        _ => theme.sub_text_color(background),
    };
    let (dir, name) = match file.path.rsplit_once('/') {
        Some((dir, name)) => (dir.to_owned(), name.to_owned()),
        None => (String::new(), file.path.clone()),
    };
    let deleted = letter == 'D';
    let path = file.path.clone();
    let diff_kind = group.diff_kind;
    let action = group.action.filter(|_| !operation_pending && !conflict);
    let discardable = !operation_pending && !conflict && can_discard(group.action, diff_kind);
    let row_key = mouse_state("git-file-row", stable_key(&file.path, diff_kind));
    let hovered = row_key.lock().unwrap().is_mouse_over_element();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.)
        .with_child(sized_icon(
            Icon::File,
            theme.sub_text_color(background),
            13.,
        ))
        .with_child(
            Text::new_inline(name, appearance.ui_font_family(), 12.)
                .with_color(if deleted {
                    theme.sub_text_color(background).into()
                } else {
                    theme.main_text_color(background).into()
                })
                .soft_wrap(false)
                .finish(),
        )
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(dir, appearance.ui_font_family(), 11.)
                    .with_color(theme.hint_text_color(background).into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );
    if hovered {
        let mut actions = Flex::row().with_spacing(1.);
        if discardable {
            actions.add_child(icon_button(
                Icon::Retry,
                None,
                false,
                WorkspaceAction::RequestDiscardLocalGitPath {
                    path: path.clone(),
                    untracked: diff_kind == LocalGitDiffKind::Untracked,
                },
                ("git-file-discard", stable_key(&path, diff_kind)),
                appearance,
            ));
        }
        if let Some(action) = action {
            let (icon, action) = match action {
                ChangeAction::Stage => {
                    (Icon::Plus, WorkspaceAction::StageLocalGitPath(path.clone()))
                }
                ChangeAction::Unstage => (
                    Icon::Minus,
                    WorkspaceAction::UnstageLocalGitPath(path.clone()),
                ),
            };
            actions.add_child(icon_button(
                icon,
                None,
                false,
                action,
                ("git-file-stage", stable_key(&path, diff_kind)),
                appearance,
            ));
        }
        row.add_child(actions.finish());
    }
    row.add_child(
        ConstrainedBox::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::Center)
                .with_child(
                    Text::new_inline(letter.to_string(), appearance.monospace_font_family(), 11.)
                        .with_style(Properties::default().weight(Weight::Semibold))
                        .with_color(letter_color.into())
                        .finish(),
                )
                .finish(),
        )
        .with_width(12.)
        .finish(),
    );
    let staged = diff_kind == LocalGitDiffKind::Staged;
    let untracked = diff_kind == LocalGitDiffKind::Untracked;
    let selected = open_diff == Some((file.path.as_str(), staged));
    Hoverable::new(row_key, move |mouse| {
        let mut container = Container::new(row.finish())
            .with_padding_left(26.)
            .with_padding_right(8.);
        if selected {
            container = container.with_background(fg_overlay(theme, 10));
        } else if mouse.is_mouse_over_element() {
            container = container.with_background(fg_overlay(theme, 5));
        }
        ConstrainedBox::new(container.finish())
            .with_height(FILE_ROW_HEIGHT)
            .finish()
    })
    .with_defer_events_to_children()
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| {
        ctx.dispatch_typed_action(WorkspaceAction::OpenLocalGitDiff {
            path: path.clone(),
            staged,
            untracked,
        })
    })
    .finish()
}

/// Mouse-state key for a file row, stable across frames for the same path and list.
fn stable_key(path: &str, diff_kind: LocalGitDiffKind) -> i64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(path, &mut hasher);
    std::hash::Hash::hash(&(diff_kind as u8), &mut hasher);
    std::hash::Hasher::finish(&hasher) as i64
}

fn graph_colors(appearance: &Appearance) -> GraphColors {
    let theme = appearance.theme();
    let normal = &theme.terminal_colors().normal;
    GraphColors {
        lanes: vec![
            normal.yellow.into(),
            normal.magenta.into(),
            normal.cyan.into(),
            normal.blue.into(),
            normal.green.into(),
            normal.red.into(),
            // `--kt-coral`, the brand's traffic-light coral.
            ColorU::new(0xFD, 0x70, 0x4A, 0xFF),
        ],
        head: theme.accent().into_solid(),
        background: theme.background().into_solid(),
    }
}

fn graph_section(source_control: &SourceControl<'_>, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let sub = theme.sub_text_color(background);
    let panel = source_control.panel;
    let filter = Hoverable::new(mouse_state("git-graph-filter", 0), |_| {
        Container::new(
            ConstrainedBox::new(
                Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(3.)
                    .with_child(sized_icon(Icon::GitBranch, sub, 11.))
                    .with_child(
                        Text::new_inline(
                            if panel.graph_current_only {
                                "Só a atual"
                            } else {
                                "Todas"
                            },
                            appearance.ui_font_family(),
                            11.,
                        )
                        .with_color(sub.into())
                        .finish(),
                    )
                    .finish(),
            )
            .with_height(20.)
            .finish(),
        )
        .with_padding_left(5.)
        .with_padding_right(5.)
        .with_background(fg_overlay(theme, 5))
        .with_corner_radius(radius(4.))
        .finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(|ctx, _, _| ctx.dispatch_typed_action(WorkspaceAction::ToggleLocalGitGraphFilter))
    .finish();
    let right = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(1.)
        .with_child(filter)
        .with_child(icon_button(
            Icon::Refresh,
            None,
            panel.remote_busy == Some(GitRemoteAction::Fetch),
            WorkspaceAction::RunLocalGitRemote(GitRemoteAction::Fetch),
            ("git-graph-refresh", 0),
            appearance,
        ))
        .finish();
    let mut section = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(section_header(
            "GRAFO",
            GitPanelSection::Graph,
            panel.graph_collapsed,
            right,
            appearance,
        ));
    if panel.graph_collapsed {
        return section.finish();
    }
    let commits = if panel.graph_current_only {
        graph::reachable_from_head(source_control.history)
    } else {
        source_control.history.to_vec()
    };
    let (rows, lanes) = graph::layout(&commits);
    let width = graph::graph_width(lanes);
    let colors = graph_colors(appearance);
    let local_branches: HashSet<&str> = source_control
        .branches
        .iter()
        .map(|branch| branch.name.as_str())
        .collect();
    let mut list = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    for (row, commit) in rows.iter().zip(&commits) {
        list.add_child(graph_row(
            row,
            commit,
            width,
            &colors,
            &local_branches,
            panel.expanded_commit.as_deref() == Some(commit.hash.as_str()),
            appearance,
        ));
    }
    section.add_child(
        Container::new(list.finish())
            .with_padding_bottom(6.)
            .finish(),
    );
    section.finish()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RefKind {
    Local,
    Remote,
    Tag,
}

struct RefLabel {
    name: String,
    kind: RefKind,
    head: bool,
}

/// Splits `%D` decorations into pills, HEAD first; unknown names are treated as remote refs.
fn ref_labels(commit: &LocalCommit, local_branches: &HashSet<&str>) -> Vec<RefLabel> {
    let mut labels: Vec<RefLabel> = commit
        .refs
        .iter()
        .map(|decoration| {
            if decoration == "HEAD" {
                RefLabel {
                    name: "HEAD".to_owned(),
                    kind: RefKind::Local,
                    head: true,
                }
            } else if let Some(branch) = decoration.strip_prefix("HEAD -> ") {
                RefLabel {
                    name: branch.to_owned(),
                    kind: RefKind::Local,
                    head: true,
                }
            } else if let Some(tag) = decoration.strip_prefix("tag: ") {
                RefLabel {
                    name: tag.to_owned(),
                    kind: RefKind::Tag,
                    head: false,
                }
            } else {
                RefLabel {
                    name: decoration.clone(),
                    kind: if local_branches.contains(decoration.as_str()) {
                        RefKind::Local
                    } else {
                        RefKind::Remote
                    },
                    head: false,
                }
            }
        })
        .collect();
    labels.sort_by_key(|label| !label.head);
    labels
}

fn with_alpha(color: ColorU, alpha: u8) -> Fill {
    Fill::Solid(ColorU::new(color.r, color.g, color.b, alpha))
}

/// Kit `RefPill`: 16px, lane-tinted (accent for HEAD), icon by kind.
fn ref_pill(label: &RefLabel, lane: ColorU, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let (background, border, text) = if label.head {
        // The kit's `text-on-accent` is the theme foreground.
        (theme.accent(), theme.accent(), theme.foreground())
    } else {
        (
            with_alpha(lane, 61),
            with_alpha(lane, 140),
            theme.main_text_color(theme.surface_1()),
        )
    };
    let icon = match label.kind {
        RefKind::Remote => Icon::Cloud,
        RefKind::Tag => Icon::Bookmark,
        RefKind::Local => Icon::GitBranch,
    };
    ConstrainedBox::new(
        Container::new(
            Flex::row()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(3.)
                .with_child(sized_icon(icon, text, 10.))
                .with_child(
                    Shrinkable::new(
                        1.,
                        Text::new_inline(label.name.clone(), appearance.ui_font_family(), 10.)
                            .with_color(text.into())
                            .soft_wrap(false)
                            .with_clip(ClipConfig::ellipsis())
                            .finish(),
                    )
                    .finish(),
                )
                .finish(),
        )
        .with_padding_left(5.)
        .with_padding_right(5.)
        .with_background(background)
        .with_border(Border::all(1.).with_border_fill(border))
        .with_corner_radius(radius(4.))
        .finish(),
    )
    .with_height(16.)
    .with_max_width(150.)
    .finish()
}

fn graph_row(
    row: &GraphRow,
    commit: &LocalCommit,
    width: f32,
    colors: &GraphColors,
    local_branches: &HashSet<&str>,
    expanded: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let head = graph::is_head(commit);
    let lane = colors.lane(row.col);
    let labels = ref_labels(commit, local_branches);
    let mut line = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(graph::svg_cell(
            graph::row_svg(row, commit, width, colors),
            width,
        ))
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(commit.subject.clone(), appearance.ui_font_family(), 12.)
                    .with_style(if head {
                        Properties::default().weight(Weight::Semibold)
                    } else {
                        Properties::default()
                    })
                    .with_color(if commit.parents.len() > 1 && !head {
                        theme.sub_text_color(background).into()
                    } else {
                        theme.main_text_color(background).into()
                    })
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
            )
            .finish(),
        );
    if !labels.is_empty() {
        let mut pills = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(3.);
        for label in labels.iter().take(VISIBLE_REFS) {
            pills.add_child(ref_pill(label, lane, appearance));
        }
        if labels.len() > VISIBLE_REFS {
            pills.add_child(
                Text::new_inline(
                    format!("+{}", labels.len() - VISIBLE_REFS),
                    appearance.ui_font_family(),
                    10.,
                )
                .with_color(theme.hint_text_color(background).into())
                .finish(),
            );
        }
        line.add_child(Container::new(pills.finish()).with_margin_left(6.).finish());
    }
    let hash = commit.hash.clone();
    let line = Hoverable::new(
        mouse_state("git-graph-row", stable_key(&hash, LocalGitDiffKind::Staged)),
        move |mouse| {
            let mut container = Container::new(line.finish()).with_padding_right(8.);
            if expanded {
                container = container.with_background(theme.accent().with_opacity(25));
            } else if mouse.is_mouse_over_element() {
                container = container.with_background(fg_overlay(theme, 5));
            }
            ConstrainedBox::new(container.finish())
                .with_height(graph::ROW_HEIGHT)
                .finish()
        },
    )
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| {
        ctx.dispatch_typed_action(WorkspaceAction::ToggleLocalGitCommitDetails(hash.clone()))
    })
    .finish();
    if !expanded {
        return line;
    }
    let sub = theme.sub_text_color(background);
    let mut meta = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(10.)
        .with_child(
            Text::new_inline(
                commit.hash.chars().take(7).collect::<String>(),
                appearance.monospace_font_family(),
                11.,
            )
            .with_color(theme.main_text_color(background).into())
            .finish(),
        )
        .with_child(
            Text::new_inline(commit.author.clone(), appearance.ui_font_family(), 11.)
                .with_color(sub.into())
                .finish(),
        );
    if let Ok(time) = chrono::DateTime::parse_from_rfc3339(&commit.authored_at) {
        meta.add_child(
            Text::new_inline(
                super::super::home::relative_time(time.timestamp_millis()),
                appearance.ui_font_family(),
                11.,
            )
            .with_color(sub.into())
            .finish(),
        );
    }
    if commit.parents.len() > 1 {
        meta.add_child(
            Text::new_inline(
                format!("merge de {} pais", commit.parents.len()),
                appearance.ui_font_family(),
                11.,
            )
            .with_color(sub.into())
            .finish(),
        );
    }
    let mut details = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_spacing(4.)
        .with_child(
            Text::new(commit.subject.clone(), appearance.ui_font_family(), 12.)
                .with_color(theme.main_text_color(background).into())
                .finish(),
        )
        .with_child(meta.finish());
    if !labels.is_empty() {
        let mut pills = Wrap::new(Axis::Horizontal)
            .with_spacing(4.)
            .with_run_spacing(4.);
        for label in &labels {
            pills.add_child(ref_pill(label, lane, appearance));
        }
        details.add_child(pills.finish());
    }
    Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(line)
        .with_child(
            Container::new(details.finish())
                .with_padding_left(width)
                .with_padding_right(12.)
                .with_padding_top(8.)
                .with_padding_bottom(10.)
                .with_background(fg_overlay(theme, 5))
                .with_border(Border::bottom(1.).with_border_fill(theme.outline()))
                .finish(),
        )
        .finish()
}

fn footer(source_control: &SourceControl<'_>, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let sub = theme.sub_text_color(background);
    // The kit's gap is 10px; the app's mono font draws the branch name 1px wider than the kit's.
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(9.);
    if let Some(status) = source_control.status {
        let branch = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(4.)
            .with_child(sized_icon(Icon::GitBranch, sub, 12.))
            .with_child(
                Shrinkable::new(
                    1.,
                    Text::new_inline(
                        head_name(status).to_owned(),
                        appearance.monospace_font_family(),
                        11.,
                    )
                    .with_color(sub.into())
                    .soft_wrap(false)
                    .with_clip(ClipConfig::ellipsis())
                    .finish(),
                )
                .finish(),
            )
            .finish();
        // Like VS Code's status bar branch: clicking it opens the branch list.
        row.add_child(
            Shrinkable::new(
                1.,
                Hoverable::new(mouse_state("git-footer-branch", 0), |_| branch)
                    .with_cursor(Cursor::PointingHand)
                    .on_click(|ctx, _, _| {
                        ctx.dispatch_typed_action(WorkspaceAction::ToggleLocalGitBranches)
                    })
                    .finish(),
            )
            .finish(),
        );
        let sync = match status.upstream {
            Some(upstream) => {
                row.add_child(
                    Text::new_inline(
                        format!("↑{} ↓{}", upstream.ahead, upstream.behind),
                        appearance.ui_font_family(),
                        11.,
                    )
                    .with_color(sub.into())
                    .finish(),
                );
                if upstream.ahead > 0 {
                    Some(("Sincronizar", GitRemoteAction::Push))
                } else if upstream.behind > 0 {
                    Some(("Sincronizar", GitRemoteAction::Pull))
                } else {
                    None
                }
            }
            None => {
                row.add_child(
                    Text::new_inline("não publicada", appearance.ui_font_family(), 11.)
                        .with_color(theme.ui_yellow_color())
                        .finish(),
                );
                matches!(status.head, HeadState::Branch(_))
                    .then_some(("Publicar", GitRemoteAction::Push))
            }
        };
        row.add_child(Expanded::new(1., Flex::row().finish()).finish());
        if let Some(fetched) = source_control.panel.last_fetch_ms {
            row.add_child(
                Text::new_inline(
                    format!("fetch {}", super::super::home::relative_time(fetched)),
                    appearance.ui_font_family(),
                    11.,
                )
                .with_color(theme.hint_text_color(background).into())
                .finish(),
            );
        }
        if let Some((label, action)) = sync {
            row.add_child(small_secondary_button(
                Icon::Upload,
                label,
                WorkspaceAction::RunLocalGitRemote(action),
                ("git-sync", 0),
                appearance,
            ));
        }
    }
    ConstrainedBox::new(
        Container::new(row.finish())
            .with_padding_left(12.)
            .with_padding_right(12.)
            .with_border(Border::top(1.).with_border_fill(theme.outline()))
            .finish(),
    )
    .with_height(FOOTER_HEIGHT)
    .finish()
}

/// Kit small secondary `Button`: 24px, outlined, icon plus semibold label.
pub(super) fn small_secondary_button(
    icon: Icon,
    label: &'static str,
    action: WorkspaceAction,
    key: (&'static str, i64),
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let content = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(2.)
        .with_child(sized_icon(icon, theme.foreground(), 14.))
        .with_child(
            Text::new_inline(label, appearance.ui_font_family(), 12.)
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(theme.foreground().into())
                .finish(),
        )
        .finish();
    crate::workspace::view::local_workspace::dialog_ui::small_secondary(
        theme,
        key,
        move |ctx| ctx.dispatch_typed_action(action.clone()),
        // The kit pads 8px around a label the app's UI font draws 2px narrower.
        Container::new(content)
            .with_padding_left(9.)
            .with_padding_right(9.)
            .with_padding_bottom(2.)
            .finish(),
    )
}

fn message(text: &str, appearance: &Appearance, is_error: bool) -> Box<dyn Element> {
    let theme = appearance.theme();
    Container::new(
        Text::new(text.to_owned(), appearance.ui_font_family(), 11.)
            .with_color(if is_error {
                theme.ui_error_color()
            } else {
                theme.sub_text_color(theme.surface_1()).into()
            })
            .finish(),
    )
    .with_padding_left(14.)
    .with_padding_right(14.)
    .with_padding_top(6.)
    .with_padding_bottom(6.)
    .finish()
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
    let mut actions = Flex::row().with_spacing(6.);
    if !operation_pending {
        actions.add_child(small_secondary_button(
            Icon::Retry,
            "Descartar",
            WorkspaceAction::ConfirmDiscardLocalGitPath,
            ("git-discard-confirm", 0),
            appearance,
        ));
    }
    actions.add_child(small_secondary_button(
        Icon::X,
        "Cancelar",
        WorkspaceAction::CancelDiscardLocalGitPath,
        ("git-discard-cancel", 0),
        appearance,
    ));
    Container::new(
        Flex::column()
            .with_spacing(6.)
            .with_child(
                Text::new(
                    format!("Descartar {path}? {detail}"),
                    appearance.ui_font_family(),
                    11.,
                )
                .with_color(theme.ui_error_color())
                .finish(),
            )
            .with_child(actions.finish())
            .finish(),
    )
    .with_background(fg_overlay(theme, 5))
    .with_uniform_padding(8.)
    .with_margin_left(10.)
    .with_margin_right(10.)
    .with_margin_bottom(8.)
    .with_corner_radius(radius(4.))
    .finish()
}

#[cfg(test)]
#[path = "changes_tests.rs"]
mod tests;

/// VS Code's "Source Control Repositories": every repository in the project folder with its
/// branch, sync counts and pending changes; clicking one makes it the panel's repository.
fn repositories_section(
    source_control: &SourceControl<'_>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let sub = theme.sub_text_color(background);
    let mut section = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(section_header(
            "REPOSITÓRIOS",
            GitPanelSection::Repositories,
            source_control.panel.repositories_collapsed,
            Flex::row().finish(),
            appearance,
        ));
    if source_control.panel.repositories_collapsed {
        return section.finish();
    }
    for (index, repository) in source_control.repositories.iter().enumerate() {
        let name_color = if repository.active {
            theme.main_text_color(background)
        } else {
            sub
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.)
            .with_child(sized_icon(Icon::Folder, sub, 14.))
            .with_child(
                Shrinkable::new(
                    1.,
                    Text::new_inline(repository.label.clone(), appearance.ui_font_family(), 12.)
                        .with_style(if repository.active {
                            Properties::default().weight(Weight::Semibold)
                        } else {
                            Properties::default()
                        })
                        .with_color(name_color.into())
                        .soft_wrap(false)
                        .with_clip(ClipConfig::ellipsis())
                        .finish(),
                )
                .finish(),
            );
        if let Some(branch) = &repository.branch {
            row.add_child(
                Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_spacing(3.)
                    .with_child(sized_icon(Icon::GitBranch, sub, 11.))
                    .with_child(
                        Text::new_inline(branch.clone(), appearance.monospace_font_family(), 11.)
                            .with_color(sub.into())
                            .soft_wrap(false)
                            .finish(),
                    )
                    .finish(),
            );
        }
        row.add_child(Expanded::new(1., Flex::row().finish()).finish());
        if repository.ahead > 0 || repository.behind > 0 {
            row.add_child(
                Text::new_inline(
                    format!("↑{} ↓{}", repository.ahead, repository.behind),
                    appearance.ui_font_family(),
                    11.,
                )
                .with_color(sub.into())
                .finish(),
            );
        }
        if repository.changes > 0 {
            row.add_child(count_badge(repository.changes, appearance));
        }
        let root = repository.root.clone();
        let active = repository.active;
        let state = mouse_state("git-repository", index as i64);
        section.add_child(
            Hoverable::new(state, move |mouse| {
                let mut container = Container::new(row.finish())
                    .with_padding_left(14.)
                    .with_padding_right(8.);
                if active {
                    container = container.with_background(fg_overlay(theme, 10));
                } else if mouse.is_mouse_over_element() {
                    container = container.with_background(fg_overlay(theme, 5));
                }
                ConstrainedBox::new(container.finish())
                    .with_height(FILE_ROW_HEIGHT)
                    .finish()
            })
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::SelectLocalGitRepository(root.clone()))
            })
            .finish(),
        );
    }
    Container::new(section.finish())
        .with_padding_bottom(6.)
        .finish()
}

/// Kit count badge: a 16px neutral pill.
fn count_badge(count: usize, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let count = count.to_string();
    // A row has no width bound, so the badge's centered text needs an explicit width: the kit's
    // 16px pill, widened by about 6px per extra digit.
    let badge_width = (8. + 6. * count.len() as f32).max(16.);
    ConstrainedBox::new(
        Container::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::Center)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(
                    Text::new_inline(count, appearance.ui_font_family(), 10.)
                        .with_color(theme.main_text_color(theme.surface_1()).into())
                        .finish(),
                )
                .finish(),
        )
        .with_background(neutral(theme, 20))
        .with_corner_radius(radius(8.))
        .finish(),
    )
    .with_width(badge_width)
    .with_height(16.)
    .finish()
}
