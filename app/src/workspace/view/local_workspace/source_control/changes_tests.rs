use crate::local_git::{ChangedFile, HeadState, RepositoryStatus};

use super::{ChangeAction, can_discard, change_groups};
use crate::local_git::LocalGitDiffKind;

#[test]
fn change_groups_keep_conflicts_separate_from_stageable_files() {
    let status = RepositoryStatus {
        head: HeadState::Branch("main".to_owned()),
        staged: vec![file("staged.txt", 'M', '.'), file("conflict.txt", 'U', 'U')],
        unstaged: vec![
            file("modified.txt", '.', 'M'),
            file("conflict.txt", 'U', 'U'),
        ],
        untracked: vec![file("new.txt", '.', '?')],
        conflicted: vec![file("conflict.txt", 'U', 'U')],
        ..Default::default()
    };

    let groups = change_groups(&status);

    assert_eq!(
        groups[0]
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["staged.txt"]
    );
    assert_eq!(
        groups[1]
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["modified.txt"]
    );
    assert_eq!(
        groups[2]
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["new.txt"]
    );
    assert_eq!(
        groups[3]
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["conflict.txt"]
    );
    assert_eq!(groups[0].action, Some(ChangeAction::Unstage));
    assert_eq!(groups[1].action, Some(ChangeAction::Stage));
    assert_eq!(groups[2].action, Some(ChangeAction::Stage));
    assert_eq!(groups[3].action, None);
}

#[test]
fn discard_is_available_only_for_unstaged_and_untracked_files() {
    assert!(can_discard(
        Some(ChangeAction::Stage),
        LocalGitDiffKind::Unstaged
    ));
    assert!(can_discard(
        Some(ChangeAction::Stage),
        LocalGitDiffKind::Untracked
    ));
    assert!(!can_discard(
        Some(ChangeAction::Unstage),
        LocalGitDiffKind::Staged
    ));
    assert!(!can_discard(None, LocalGitDiffKind::Unstaged));
}

fn file(path: &str, index_status: char, worktree_status: char) -> ChangedFile {
    ChangedFile {
        path: path.to_owned(),
        original_path: None,
        index_status,
        worktree_status,
    }
}
