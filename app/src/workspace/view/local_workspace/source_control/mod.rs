mod changes;
mod commit;
mod diff;
mod extras;
mod graph;
mod refs;

use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{CommitFile, LocalBranch, LocalCommit, LocalStash, RepositoryStatus};
use warpui::ViewHandle;
use warpui::elements::{ClippedScrollStateHandle, Element};

#[derive(Clone, Default)]
pub(crate) struct DiffPreview {
    pub path: Option<String>,
    pub lines: Option<Vec<crate::local_git::DiffLine>>,
    pub hunks: Vec<String>,
    pub staged: bool,
    pub untracked: bool,
    pub binary: bool,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitPanelSection {
    Repositories,
    Changes,
    Stashes,
    Graph,
}

/// Destructive Git actions that wait for a confirmation row in the panel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GitConfirmation {
    DiscardAll,
    DeleteBranch(String),
    DropStash(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitRemoteAction {
    Fetch,
    Pull,
    Push,
}

/// Presentation state of the panel, kept apart from the repository data.
#[derive(Clone, Default)]
pub(crate) struct GitPanelState {
    pub repositories_collapsed: bool,
    pub changes_collapsed: bool,
    pub stashes_collapsed: bool,
    pub graph_collapsed: bool,
    /// "Só a atual": only commits reachable from HEAD.
    pub graph_current_only: bool,
    pub commit_menu_open: bool,
    pub branches_open: bool,
    pub expanded_commit: Option<String>,
    pub remote_busy: Option<GitRemoteAction>,
    /// Confirmation shown under the commit button after an operation succeeds.
    pub note: Option<String>,
    /// Unix time in milliseconds of the last successful fetch.
    pub last_fetch_ms: Option<i64>,
}

/// One repository found in the project folder, as the "Repositórios" list shows it.
pub(crate) struct RepositoryEntry {
    /// Absolute root, used to select the repository.
    pub root: String,
    /// Root relative to the project folder ("." for the folder itself).
    pub label: String,
    pub branch: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub changes: usize,
    pub active: bool,
}

/// Everything the "Controle de versão" panel shows.
pub(crate) struct SourceControl<'a> {
    pub project_name: &'a str,
    /// Repositories in the project folder; the list shows when there is more than one.
    pub repositories: &'a [RepositoryEntry],
    pub status: Option<&'a RepositoryStatus>,
    pub history: &'a [LocalCommit],
    pub branches: &'a [LocalBranch],
    pub create_branch_from: Option<&'a str>,
    pub rename_branch: Option<&'a str>,
    pub branch_name_input: ViewHandle<EditorView>,
    pub error: Option<&'a str>,
    pub commit_input: ViewHandle<EditorView>,
    pub commit_message_empty: bool,
    pub commit_focused: bool,
    pub diff_preview: Option<&'a DiffPreview>,
    pub amend_confirmation: bool,
    pub undo_confirmation: bool,
    pub revert_confirmation: bool,
    pub discard_confirmation: Option<(&'a str, bool)>,
    pub operation_pending: bool,
    pub panel: &'a GitPanelState,
    pub stashes: &'a [LocalStash],
    /// Remote-tracking branches without a local branch of the same name.
    pub remote_branches: &'a [String],
    /// A merge stopped on conflicts and waits to be concluded or aborted.
    pub merging: bool,
    /// Files of the expanded commit, once read.
    pub commit_files: Option<&'a [CommitFile]>,
    pub confirmation: Option<&'a GitConfirmation>,
    /// The project folder has no repository and can be initialized.
    pub can_init: bool,
    pub scroll_state: ClippedScrollStateHandle,
    /// Panel width, see [`panel_width`].
    pub width: f32,
}

/// The kit sizes the panel to 34% of the main area, clamped to 300–380px.
pub(crate) fn panel_width(main_area_width: f32) -> f32 {
    (main_area_width * 0.34).clamp(300., 380.)
}

pub(super) fn render(
    source_control: SourceControl<'_>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    changes::render(source_control, appearance)
}
