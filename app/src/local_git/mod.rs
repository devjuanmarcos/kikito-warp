mod diff;
mod errors;
mod operations;
mod repository;
mod status;

use std::path::{Path, PathBuf};

pub(crate) use diff::{DiffLine, DiffLineKind};
pub(crate) use diff::{parse_diff_hunks, parse_unified_diff};
pub(crate) use errors::GitError;
pub(crate) use operations::DiffKind as LocalGitDiffKind;
use operations::{DiffKind, GitOperation, execute};
use status::read;
pub(crate) use status::{ChangedFile, HeadState, RepositoryStatus};

#[derive(Debug, Clone)]
struct LocalGitService {
    root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalBranch {
    pub name: String,
    pub commit: String,
    pub upstream: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalCommit {
    pub hash: String,
    pub parents: Vec<String>,
    pub author: String,
    pub authored_at: String,
    pub subject: String,
}

impl LocalGitService {
    async fn open(path: &Path) -> Result<Self, GitError> {
        Ok(Self {
            root: repository::discover_repository(path).await?,
        })
    }

    fn root(&self) -> &Path {
        &self.root
    }

    async fn status(&self) -> Result<RepositoryStatus, GitError> {
        read(&self.root).await
    }

    async fn local_branches(&self) -> Result<Vec<LocalBranch>, GitError> {
        let output = execute(&self.root, GitOperation::LocalBranches).await?;
        parse_branches(&output)
    }

    async fn checkout_branch(&self, branch: &str) -> Result<(), GitError> {
        operations::validate_reference(branch)?;
        execute(
            &self.root,
            GitOperation::CheckoutBranch {
                branch: branch.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    async fn history(&self, limit: usize) -> Result<Vec<LocalCommit>, GitError> {
        let output = execute(&self.root, GitOperation::History { limit }).await?;
        parse_history(&output)
    }

    async fn merge_base(&self, left: &str, right: &str) -> Result<String, GitError> {
        repository::merge_base(&self.root, left, right).await
    }

    async fn stage(&self, path: &Path) -> Result<(), GitError> {
        operations::validate_path(path)?;
        execute(
            &self.root,
            GitOperation::Stage {
                path: path.to_path_buf(),
            },
        )
        .await?;
        Ok(())
    }

    async fn stage_all(&self) -> Result<(), GitError> {
        execute(&self.root, GitOperation::StageAll).await?;
        Ok(())
    }

    async fn unstage(&self, path: &Path) -> Result<(), GitError> {
        operations::validate_path(path)?;
        execute(
            &self.root,
            GitOperation::Unstage {
                path: path.to_path_buf(),
            },
        )
        .await?;
        Ok(())
    }

    async fn unstage_all(&self) -> Result<(), GitError> {
        self.unstage(Path::new(".")).await
    }

    async fn discard_worktree(&self, path: &Path, untracked: bool) -> Result<(), GitError> {
        operations::validate_path(path)?;
        execute(
            &self.root,
            GitOperation::DiscardWorktree {
                path: path.to_path_buf(),
                untracked,
            },
        )
        .await?;
        Ok(())
    }

    async fn commit(&self, message: &str, amend: bool) -> Result<(), GitError> {
        if message.trim().is_empty() {
            return Err(GitError::EmptyCommitMessage);
        }
        execute(
            &self.root,
            GitOperation::Commit {
                message: message.to_owned(),
                amend,
            },
        )
        .await?;
        Ok(())
    }

    async fn commit_all(&self, message: &str) -> Result<(), GitError> {
        if message.trim().is_empty() {
            return Err(GitError::EmptyCommitMessage);
        }
        self.stage_all().await?;
        self.commit(message, false).await
    }

    async fn undo_last_commit(&self) -> Result<(), GitError> {
        execute(&self.root, GitOperation::UndoLastCommit).await?;
        Ok(())
    }

    async fn revert_last_commit(&self) -> Result<(), GitError> {
        execute(&self.root, GitOperation::RevertLastCommit).await?;
        Ok(())
    }

    async fn diff(&self, path: &Path, kind: DiffKind) -> Result<String, GitError> {
        operations::validate_path(path)?;
        let operation = GitOperation::Diff {
            path: path.to_path_buf(),
            kind,
        };
        let arguments = operation.arguments();
        let output = operations::execute_bytes(&self.root, &operation, &arguments).await?;
        Ok(String::from_utf8_lossy(&output).into_owned())
    }

    async fn apply_hunk(
        &self,
        path: &Path,
        kind: DiffKind,
        patch: &str,
        reverse: bool,
    ) -> Result<(), GitError> {
        operations::validate_path(path)?;
        let current_diff = self.diff(path, kind).await?;
        if !diff::parse_diff_hunks(&current_diff)
            .iter()
            .any(|current| current == patch)
        {
            return Err(GitError::InvalidPatch);
        }
        operations::execute_with_input(
            &self.root,
            &GitOperation::ApplyCachedPatch { reverse },
            patch.as_bytes(),
        )
        .await
    }

    async fn fetch(&self, remote: &str) -> Result<(), GitError> {
        operations::validate_remote(remote)?;
        execute(
            &self.root,
            GitOperation::Fetch {
                remote: remote.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    async fn pull(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        operations::validate_remote(remote)?;
        operations::validate_reference(branch)?;
        execute(
            &self.root,
            GitOperation::Pull {
                remote: remote.to_owned(),
                branch: branch.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    async fn push(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        operations::validate_remote(remote)?;
        operations::validate_reference(branch)?;
        execute(
            &self.root,
            GitOperation::Push {
                remote: remote.to_owned(),
                branch: branch.to_owned(),
            },
        )
        .await?;
        Ok(())
    }
}

pub(crate) async fn read_status(path: &Path) -> Result<RepositoryStatus, GitError> {
    LocalGitService::open(path).await?.status().await
}

pub(crate) async fn read_local_branches(path: &Path) -> Result<Vec<LocalBranch>, GitError> {
    LocalGitService::open(path).await?.local_branches().await
}

pub(crate) async fn checkout_local_branch(repository: &Path, branch: &str) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .checkout_branch(branch)
        .await
}

pub(crate) async fn stage_path(repository: &Path, path: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository).await?.stage(path).await
}

pub(crate) async fn stage_all(repository: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository).await?.stage_all().await
}

pub(crate) async fn unstage_path(repository: &Path, path: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository).await?.unstage(path).await
}

pub(crate) async fn unstage_all(repository: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository).await?.unstage_all().await
}

pub(crate) async fn discard_worktree(
    repository: &Path,
    path: &Path,
    untracked: bool,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .discard_worktree(path, untracked)
        .await
}

pub(crate) async fn commit(repository: &Path, message: &str, amend: bool) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .commit(message, amend)
        .await
}

pub(crate) async fn commit_all(repository: &Path, message: &str) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .commit_all(message)
        .await
}

pub(crate) async fn undo_last_commit(repository: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .undo_last_commit()
        .await
}

pub(crate) async fn revert_last_commit(repository: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .revert_last_commit()
        .await
}

pub(crate) async fn diff(
    repository: &Path,
    path: &Path,
    kind: LocalGitDiffKind,
) -> Result<String, GitError> {
    LocalGitService::open(repository)
        .await?
        .diff(path, kind)
        .await
}

pub(crate) async fn apply_hunk(
    repository: &Path,
    path: &Path,
    kind: LocalGitDiffKind,
    patch: &str,
    reverse: bool,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .apply_hunk(path, kind, patch, reverse)
        .await
}

pub(crate) async fn merge_base(path: &Path, left: &str, right: &str) -> Result<String, GitError> {
    LocalGitService::open(path)
        .await?
        .merge_base(left, right)
        .await
}

fn parse_branches(output: &str) -> Result<Vec<LocalBranch>, GitError> {
    output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.trim_end_matches(['\r', '\n']);
            let mut fields = line.split('\0');
            let name = fields.next().ok_or(GitError::InvalidOutput)?;
            let commit = fields.next().ok_or(GitError::InvalidOutput)?;
            let upstream = fields.next().ok_or(GitError::InvalidOutput)?;
            let is_current = fields.next().ok_or(GitError::InvalidOutput)? == "*";
            if name.is_empty() || commit.is_empty() {
                return Err(GitError::InvalidOutput);
            }
            Ok(LocalBranch {
                name: name.to_owned(),
                commit: commit.to_owned(),
                upstream: (!upstream.is_empty()).then(|| upstream.to_owned()),
                is_current,
            })
        })
        .collect()
}

fn parse_history(output: &str) -> Result<Vec<LocalCommit>, GitError> {
    output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let fields: Vec<&str> = line.split('\0').collect();
            if fields.len() != 5 || fields[0].is_empty() {
                return Err(GitError::InvalidOutput);
            }
            Ok(LocalCommit {
                hash: fields[0].to_owned(),
                parents: fields[1].split_whitespace().map(str::to_owned).collect(),
                author: fields[2].to_owned(),
                authored_at: fields[3].to_owned(),
                subject: fields[4].to_owned(),
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "operations_tests.rs"]
mod tests;
