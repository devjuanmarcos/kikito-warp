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
