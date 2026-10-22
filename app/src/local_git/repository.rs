use std::path::{Path, PathBuf};

use super::{
    errors::{CommandFailure, GitError},
    operations::{GitOperation, execute},
};

pub(super) async fn discover_repository(path: &Path) -> Result<PathBuf, GitError> {
    let root = match execute(path, GitOperation::DiscoverRepository).await {
        Ok(root) => root,
        Err(GitError::CommandFailed {
            failure: CommandFailure::NotRepository,
            ..
        }) => return Err(GitError::NotRepository),
        Err(error) => return Err(error),
    };
    let root = root.trim_end_matches(['\r', '\n']);
    if root.is_empty() {
        return Err(GitError::NotRepository);
    }
    Ok(PathBuf::from(root))
}

pub(super) async fn merge_base(
    repository: &Path,
    left: &str,
    right: &str,
) -> Result<String, GitError> {
    super::operations::validate_reference(left)?;
    super::operations::validate_reference(right)?;
    for reference in [left, right] {
        match execute(
            repository,
            GitOperation::ResolveReference {
                reference: reference.to_owned(),
            },
        )
        .await
        {
            Ok(_) => {}
            Err(error) => return Err(map_resolution_error(reference, error)),
        }
    }
    execute(
        repository,
        GitOperation::MergeBase {
            left: left.to_owned(),
            right: right.to_owned(),
        },
    )
    .await
}

pub(super) fn map_resolution_error(reference: &str, error: GitError) -> GitError {
    match error {
        GitError::CommandFailed {
            failure: CommandFailure::MissingReference,
            ..
        } => GitError::MissingReference(reference.to_owned()),
        error => error,
    }
}

/// Folders skipped while looking for nested repositories: dependency and build output trees are
/// large and never hold the user's own repositories.
const SKIPPED_FOLDERS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "out",
    ".next",
    ".venv",
    "venv",
    "__pycache__",
    "vendor",
];

/// Repositories whose root is strictly inside `folder`, at most `max_depth` levels below it. A
/// found repository isn't searched further, like VS Code's sub-folder detection.
pub(super) fn find_nested_repositories(folder: &Path, max_depth: usize) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![(folder.to_path_buf(), 0)];
    while let Some((directory, depth)) = pending.pop() {
        if depth >= max_depth {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        let mut children: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
            .filter(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                !SKIPPED_FOLDERS.contains(&name.as_ref())
            })
            .map(|entry| entry.path())
            .collect();
        children.sort();
        for child in children.into_iter().rev() {
            if child.join(".git").exists() {
                found.push(child);
            } else {
                pending.push((child, depth + 1));
            }
        }
    }
    found.sort();
    found
}
