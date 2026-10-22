mod diff;
mod errors;
mod operations;
mod repository;
mod status;

use std::path::{Path, PathBuf};

pub(crate) use diff::{DiffLine, DiffLineKind};
pub(crate) use diff::{parse_diff_hunks, parse_unified_diff};
use errors::CommandFailure;
pub(crate) use errors::GitError;
pub(crate) use operations::DiffKind as LocalGitDiffKind;
pub(crate) use operations::PendingOperation;
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

/// An entry of `git stash list`, newest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalStash {
    pub index: usize,
    pub message: String,
    pub created_at: String,
}

/// A file a commit touched, as `git show --name-status` reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommitFile {
    pub status: char,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalCommit {
    pub hash: String,
    pub parents: Vec<String>,
    pub author: String,
    pub authored_at: String,
    pub subject: String,
    /// Decorations as `git log --format=%D` prints them, e.g. `HEAD -> main`, `origin/main`,
    /// `tag: v1.4.0`.
    pub refs: Vec<String>,
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

    async fn create_branch_from_ref(&self, source: &str, name: &str) -> Result<(), GitError> {
        operations::validate_reference(source)?;
        operations::validate_reference(name)?;
        match execute(
            &self.root,
            GitOperation::ValidateBranchName {
                name: name.to_owned(),
            },
        )
        .await
        {
            Ok(_) => {}
            Err(GitError::CommandFailed { .. }) => return Err(GitError::InvalidBranchName),
            Err(error) => return Err(error),
        }
        execute(
            &self.root,
            GitOperation::ResolveReference {
                reference: source.to_owned(),
            },
        )
        .await
        .map_err(|error| repository::map_resolution_error(source, error))?;
        execute(
            &self.root,
            GitOperation::CreateBranchFromRef {
                name: name.to_owned(),
                source: source.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    async fn rename_branch(&self, old: &str, new: &str) -> Result<(), GitError> {
        operations::validate_reference(old)?;
        operations::validate_reference(new)?;
        if !self
            .local_branches()
            .await?
            .iter()
            .any(|branch| branch.name == old)
        {
            return Err(GitError::MissingReference(old.to_owned()));
        }
        match execute(
            &self.root,
            GitOperation::ValidateBranchName {
                name: new.to_owned(),
            },
        )
        .await
        {
            Ok(_) => {}
            Err(GitError::CommandFailed { .. }) => return Err(GitError::InvalidBranchName),
            Err(error) => return Err(error),
        }
        execute(
            &self.root,
            GitOperation::RenameBranch {
                old: old.to_owned(),
                new: new.to_owned(),
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

    async fn run(&self, operation: GitOperation) -> Result<(), GitError> {
        execute(&self.root, operation).await?;
        Ok(())
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

/// Repositories for a project folder: the one containing the folder (if any) first, then the
/// ones nested up to two levels below it.
pub(crate) async fn discover_repositories(folder: &Path) -> Vec<PathBuf> {
    let mut repositories = Vec::new();
    if let Ok(root) = repository::discover_repository(folder).await {
        repositories.push(root);
    }
    let folder = folder.to_path_buf();
    let nested = repository::find_nested_repositories(&folder, NESTED_REPOSITORY_DEPTH);
    for root in nested {
        if !repositories.iter().any(|known| same_path(known, &root)) {
            repositories.push(root);
        }
    }
    repositories
}

const NESTED_REPOSITORY_DEPTH: usize = 2;

/// Compares paths the way Git and the file system report them on Windows (separators and case
/// may differ).
pub(crate) fn same_path(left: &Path, right: &Path) -> bool {
    let normalize = |path: &Path| {
        path.to_string_lossy()
            .replace('\\', "/")
            .trim_end_matches('/')
            .to_lowercase()
    };
    normalize(left) == normalize(right)
}

pub(crate) async fn read_status(path: &Path) -> Result<RepositoryStatus, GitError> {
    LocalGitService::open(path).await?.status().await
}

pub(crate) async fn read_local_branches(path: &Path) -> Result<Vec<LocalBranch>, GitError> {
    LocalGitService::open(path).await?.local_branches().await
}

/// Newest-first commits of every ref, at most `limit`.
pub(crate) async fn read_history(path: &Path, limit: usize) -> Result<Vec<LocalCommit>, GitError> {
    LocalGitService::open(path).await?.history(limit).await
}

pub(crate) async fn fetch_remote(repository: &Path, remote: &str) -> Result<(), GitError> {
    LocalGitService::open(repository).await?.fetch(remote).await
}

pub(crate) async fn pull_branch(
    repository: &Path,
    remote: &str,
    branch: &str,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .pull(remote, branch)
        .await
}

pub(crate) async fn push_branch(
    repository: &Path,
    remote: &str,
    branch: &str,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .push(remote, branch)
        .await
}

pub(crate) async fn checkout_local_branch(repository: &Path, branch: &str) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .checkout_branch(branch)
        .await
}

pub(crate) async fn create_local_branch_from_ref(
    repository: &Path,
    source: &str,
    name: &str,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .create_branch_from_ref(source, name)
        .await
}

pub(crate) async fn rename_local_branch(
    repository: &Path,
    old: &str,
    new: &str,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .rename_branch(old, new)
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
            if fields.len() != 6 || fields[0].is_empty() {
                return Err(GitError::InvalidOutput);
            }
            Ok(LocalCommit {
                hash: fields[0].to_owned(),
                parents: fields[1].split_whitespace().map(str::to_owned).collect(),
                author: fields[2].to_owned(),
                authored_at: fields[3].to_owned(),
                subject: fields[4].to_owned(),
                refs: fields[5]
                    .split(", ")
                    .map(str::trim)
                    .filter(|decoration| !decoration.is_empty())
                    .map(str::to_owned)
                    .collect(),
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "operations_tests.rs"]
mod tests;

/// Discards every working-tree change; `remove_untracked` also deletes untracked files.
pub(crate) async fn discard_all(repository: &Path, remove_untracked: bool) -> Result<(), GitError> {
    let service = LocalGitService::open(repository).await?;
    service.run(GitOperation::RestoreAllWorktree).await?;
    if remove_untracked {
        service.run(GitOperation::CleanUntracked).await?;
    }
    Ok(())
}

pub(crate) async fn delete_local_branch(
    repository: &Path,
    branch: &str,
    force: bool,
) -> Result<(), GitError> {
    operations::validate_reference(branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::DeleteBranch {
            branch: branch.to_owned(),
            force,
        })
        .await
}

/// Remote-tracking branches, without the remotes' `HEAD` aliases.
pub(crate) async fn read_remote_branches(repository: &Path) -> Result<Vec<String>, GitError> {
    let service = LocalGitService::open(repository).await?;
    let output = execute(service.root(), GitOperation::RemoteBranches).await?;
    Ok(parse_remote_branches(&output))
}

pub(crate) async fn checkout_remote_branch(
    repository: &Path,
    remote_branch: &str,
) -> Result<(), GitError> {
    operations::validate_reference(remote_branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::CheckoutRemoteBranch {
            remote_branch: remote_branch.to_owned(),
        })
        .await
}

pub(crate) async fn merge_branch(repository: &Path, branch: &str) -> Result<(), GitError> {
    operations::validate_reference(branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::MergeBranch {
            branch: branch.to_owned(),
        })
        .await
}

pub(crate) async fn abort_pending(
    repository: &Path,
    pending: PendingOperation,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::AbortPending(pending))
        .await
}

/// The operation Git stopped in the middle of, waiting to be concluded or aborted.
pub(crate) async fn pending_operation(repository: &Path) -> Option<PendingOperation> {
    for pending in [
        PendingOperation::Merge,
        PendingOperation::CherryPick,
        PendingOperation::Revert,
        PendingOperation::Rebase,
    ] {
        let found = execute(
            repository,
            GitOperation::ResolveReference {
                reference: pending.head_ref().to_owned(),
            },
        )
        .await
        .is_ok();
        if found {
            return Some(pending);
        }
    }
    None
}

pub(crate) async fn stash_push(repository: &Path, include_untracked: bool) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::StashPush { include_untracked })
        .await
}

pub(crate) async fn read_stashes(repository: &Path) -> Result<Vec<LocalStash>, GitError> {
    let service = LocalGitService::open(repository).await?;
    let output = execute(service.root(), GitOperation::StashList).await?;
    parse_stashes(&output)
}

pub(crate) async fn stash_apply(
    repository: &Path,
    index: usize,
    pop: bool,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::StashApply { index, pop })
        .await
}

pub(crate) async fn stash_drop(repository: &Path, index: usize) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::StashDrop { index })
        .await
}

/// Applies `commit` on the current branch. A commit whose changes are already there leaves
/// nothing to apply, so that cherry-pick is aborted instead of left pending.
pub(crate) async fn cherry_pick(repository: &Path, commit: &str) -> Result<(), GitError> {
    operations::validate_reference(commit)?;
    let service = LocalGitService::open(repository).await?;
    let result = service
        .run(GitOperation::CherryPick {
            commit: commit.to_owned(),
        })
        .await;
    if let Err(GitError::CommandFailed {
        failure: CommandFailure::NothingToApply,
        ..
    }) = &result
    {
        let _ = service
            .run(GitOperation::AbortPending(PendingOperation::CherryPick))
            .await;
    }
    result
}

pub(crate) async fn revert_commit(repository: &Path, commit: &str) -> Result<(), GitError> {
    operations::validate_reference(commit)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::RevertCommit {
            commit: commit.to_owned(),
        })
        .await
}

pub(crate) async fn read_commit_files(
    repository: &Path,
    commit: &str,
) -> Result<Vec<CommitFile>, GitError> {
    operations::validate_reference(commit)?;
    let service = LocalGitService::open(repository).await?;
    let output = execute(
        service.root(),
        GitOperation::CommitFiles {
            commit: commit.to_owned(),
        },
    )
    .await?;
    Ok(parse_commit_files(&output))
}

/// Keeps one side of a conflicted file and stages it, marking the conflict resolved.
pub(crate) async fn resolve_conflict(
    repository: &Path,
    path: &Path,
    theirs: bool,
) -> Result<(), GitError> {
    operations::validate_path(path)?;
    let service = LocalGitService::open(repository).await?;
    service
        .run(GitOperation::ResolveConflict {
            path: path.to_path_buf(),
            theirs,
        })
        .await?;
    service.stage(path).await
}

pub(crate) async fn init_repository(folder: &Path) -> Result<(), GitError> {
    execute(folder, GitOperation::Init).await?;
    Ok(())
}

fn parse_remote_branches(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let mut fields = line.trim_end_matches('\r').split('\0');
            let name = fields.next()?.trim();
            let symref = fields.next().unwrap_or_default();
            (!name.is_empty() && symref.is_empty() && name.contains('/')).then(|| name.to_owned())
        })
        .collect()
}

fn parse_stashes(output: &str) -> Result<Vec<LocalStash>, GitError> {
    output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut fields = line.trim_end_matches('\r').split('\0');
            let reference = fields.next().ok_or(GitError::InvalidOutput)?;
            let message = fields.next().ok_or(GitError::InvalidOutput)?;
            let created_at = fields.next().unwrap_or_default();
            let index = reference
                .strip_prefix("stash@{")
                .and_then(|rest| rest.strip_suffix('}'))
                .and_then(|index| index.parse().ok())
                .ok_or(GitError::InvalidOutput)?;
            Ok(LocalStash {
                index,
                message: message.to_owned(),
                created_at: created_at.to_owned(),
            })
        })
        .collect()
}

fn parse_commit_files(output: &str) -> Vec<CommitFile> {
    output
        .lines()
        .filter_map(|line| {
            let mut fields = line.trim_end_matches('\r').split('\t');
            let status = fields.next()?.chars().next()?;
            // Renames and copies list the old path first; show the new one.
            let path = fields.next_back()?;
            Some(CommitFile {
                status,
                path: path.to_owned(),
            })
        })
        .collect()
}

/// Concludes the pending operation with the message Git prepared (merge, cherry-pick and revert
/// need the conflicts resolved and staged; rebase moves to the next commit).
pub(crate) async fn continue_pending(
    repository: &Path,
    pending: PendingOperation,
) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::ContinuePending(pending))
        .await
}

pub(crate) async fn fetch_prune(repository: &Path, remote: &str) -> Result<(), GitError> {
    operations::validate_remote(remote)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::FetchPrune {
            remote: remote.to_owned(),
        })
        .await
}

pub(crate) async fn fetch_all(repository: &Path) -> Result<(), GitError> {
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::FetchAll)
        .await
}

pub(crate) async fn pull_rebase(
    repository: &Path,
    remote: &str,
    branch: &str,
) -> Result<(), GitError> {
    operations::validate_remote(remote)?;
    operations::validate_reference(branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::PullRebase {
            remote: remote.to_owned(),
            branch: branch.to_owned(),
        })
        .await
}

pub(crate) async fn push_force_with_lease(
    repository: &Path,
    remote: &str,
    branch: &str,
) -> Result<(), GitError> {
    operations::validate_remote(remote)?;
    operations::validate_reference(branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::PushForceWithLease {
            remote: remote.to_owned(),
            branch: branch.to_owned(),
        })
        .await
}

pub(crate) async fn push_tags(repository: &Path, remote: &str) -> Result<(), GitError> {
    operations::validate_remote(remote)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::PushTags {
            remote: remote.to_owned(),
        })
        .await
}

pub(crate) async fn create_tag(
    repository: &Path,
    name: &str,
    commit: &str,
) -> Result<(), GitError> {
    operations::validate_reference(name)?;
    operations::validate_reference(commit)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::CreateTag {
            name: name.to_owned(),
            commit: commit.to_owned(),
        })
        .await
}

pub(crate) async fn delete_tag(repository: &Path, name: &str) -> Result<(), GitError> {
    operations::validate_reference(name)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::DeleteTag {
            name: name.to_owned(),
        })
        .await
}

/// Deletes `remote_branch` (`origin/feature`) on its remote.
pub(crate) async fn delete_remote_branch(
    repository: &Path,
    remote_branch: &str,
) -> Result<(), GitError> {
    let (remote, branch) = remote_branch
        .split_once('/')
        .ok_or(GitError::InvalidReference)?;
    operations::validate_remote(remote)?;
    operations::validate_reference(branch)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::DeleteRemoteBranch {
            remote: remote.to_owned(),
            branch: branch.to_owned(),
        })
        .await
}

pub(crate) async fn rebase_onto(repository: &Path, onto: &str) -> Result<(), GitError> {
    operations::validate_reference(onto)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::Rebase {
            onto: onto.to_owned(),
        })
        .await
}

pub(crate) async fn checkout_detached(repository: &Path, commit: &str) -> Result<(), GitError> {
    operations::validate_reference(commit)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::CheckoutDetached {
            commit: commit.to_owned(),
        })
        .await
}

pub(crate) async fn read_remotes(repository: &Path) -> Result<Vec<String>, GitError> {
    let service = LocalGitService::open(repository).await?;
    let output = execute(service.root(), GitOperation::Remotes).await?;
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect())
}

pub(crate) async fn add_remote(repository: &Path, name: &str, url: &str) -> Result<(), GitError> {
    operations::validate_remote(name)?;
    if url.trim().is_empty() || url.starts_with('-') {
        return Err(GitError::InvalidReference);
    }
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::AddRemote {
            name: name.to_owned(),
            url: url.trim().to_owned(),
        })
        .await
}

pub(crate) async fn remove_remote(repository: &Path, name: &str) -> Result<(), GitError> {
    operations::validate_remote(name)?;
    LocalGitService::open(repository)
        .await?
        .run(GitOperation::RemoveRemote {
            name: name.to_owned(),
        })
        .await
}

/// Web address of `commit` on GitHub when the `origin` remote points there.
pub(crate) async fn github_commit_url(repository: &Path, commit: &str) -> Option<String> {
    let service = LocalGitService::open(repository).await.ok()?;
    let url = execute(
        service.root(),
        GitOperation::RemoteUrl {
            name: "origin".to_owned(),
        },
    )
    .await
    .ok()?;
    github_web_url(url.trim()).map(|base| format!("{base}/commit/{commit}"))
}

/// `https://github.com/owner/repo` for an `origin` URL on GitHub (HTTPS or SSH form).
pub(crate) fn github_web_url(remote_url: &str) -> Option<String> {
    let path = remote_url
        .strip_prefix("git@github.com:")
        .or_else(|| remote_url.strip_prefix("ssh://git@github.com/"))
        .or_else(|| remote_url.strip_prefix("https://github.com/"))
        .or_else(|| remote_url.strip_prefix("http://github.com/"))?;
    let path = path.trim_end_matches('/').trim_end_matches(".git");
    (path.split('/').count() == 2).then(|| format!("https://github.com/{path}"))
}
