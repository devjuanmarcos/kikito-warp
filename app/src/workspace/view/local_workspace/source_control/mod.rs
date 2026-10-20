mod changes;
mod commit;
mod diff;
mod refs;

use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{LocalBranch, RepositoryStatus};
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

pub(super) fn render(
    status: Option<&RepositoryStatus>,
    branches: &[LocalBranch],
    branch_menu: Option<&str>,
    create_branch_from: Option<&str>,
    rename_branch: Option<&str>,
    branch_name_input: ViewHandle<EditorView>,
    error: Option<&str>,
    commit_input: ViewHandle<EditorView>,
    diff_preview: Option<&DiffPreview>,
    amend_confirmation: bool,
    undo_confirmation: bool,
    revert_confirmation: bool,
    discard_confirmation: Option<(&str, bool)>,
    operation_pending: bool,
    scroll_state: ClippedScrollStateHandle,
    appearance: &Appearance,
) -> Box<dyn Element> {
    changes::render(
        status,
        branches,
        branch_menu,
        create_branch_from,
        rename_branch,
        branch_name_input,
        error,
        commit_input,
        diff_preview,
        amend_confirmation,
        undo_confirmation,
        revert_confirmation,
        discard_confirmation,
        operation_pending,
        scroll_state,
        appearance,
    )
}
