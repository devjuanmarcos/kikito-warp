use super::{errors::GitError, operations::GitOperation};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChangedFile {
    pub path: String,
    pub original_path: Option<String>,
    pub index_status: char,
    pub worktree_status: char,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RepositoryStatus {
    pub head: HeadState,
    pub head_oid: Option<String>,
    pub upstream: Option<UpstreamCounts>,
    pub staged: Vec<ChangedFile>,
    pub unstaged: Vec<ChangedFile>,
    pub untracked: Vec<ChangedFile>,
    pub ignored: Vec<ChangedFile>,
    pub conflicted: Vec<ChangedFile>,
}

impl RepositoryStatus {
    pub(crate) fn staged_path_count(&self) -> usize {
        let conflicted_paths = self
            .conflicted
            .iter()
            .map(|file| file.path.as_str())
            .collect::<HashSet<_>>();
        self.staged
            .iter()
            .map(|file| file.path.as_str())
            .filter(|path| !conflicted_paths.contains(path))
            .collect::<HashSet<_>>()
            .len()
    }

    pub(crate) fn changed_path_count(&self) -> usize {
        self.staged
            .iter()
            .chain(&self.unstaged)
            .chain(&self.untracked)
            .chain(&self.conflicted)
            .flat_map(|file| [Some(file.path.as_str()), file.original_path.as_deref()])
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum HeadState {
    Branch(String),
    Detached,
    #[default]
    Unborn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UpstreamCounts {
    pub ahead: usize,
    pub behind: usize,
}

pub(super) async fn read(repository: &Path) -> Result<RepositoryStatus, GitError> {
    let operation = GitOperation::Status;
    let arguments = operation.arguments();
    let output = super::operations::execute_bytes(repository, &operation, &arguments).await?;
    parse_porcelain_v2_bytes(&output)
}

pub(super) fn parse_porcelain_v2_bytes(output: &[u8]) -> Result<RepositoryStatus, GitError> {
    parse_porcelain_v2(&String::from_utf8_lossy(output))
}

pub(super) fn parse_porcelain_v2(output: &str) -> Result<RepositoryStatus, GitError> {
    let mut status = RepositoryStatus::default();
    let mut entries = output.split('\0');
    while let Some(entry) = entries.next() {
        if entry.is_empty() {
            continue;
        }
        if let Some(header) = entry.strip_prefix("# ") {
            if let Some(oid) = header.strip_prefix("branch.oid ") {
                status.head_oid = (oid != "(initial)").then(|| oid.to_owned());
            } else if let Some(branch) = header.strip_prefix("branch.head ") {
                status.head = if branch == "(detached)" {
                    HeadState::Detached
                } else {
                    HeadState::Branch(branch.to_owned())
                };
            } else if let Some(ahead_behind) = header.strip_prefix("branch.ab ") {
                let mut counts = ahead_behind.split_whitespace();
                let ahead = counts
                    .next()
                    .and_then(parse_signed_count)
                    .ok_or(GitError::InvalidOutput)?;
                let behind = counts
                    .next()
                    .and_then(parse_signed_count)
                    .ok_or(GitError::InvalidOutput)?;
                if counts.next().is_some() {
                    return Err(GitError::InvalidOutput);
                }
                status.upstream = Some(UpstreamCounts { ahead, behind });
            }
            continue;
        }
        match entry.as_bytes().first().copied() {
            Some(b'1') | Some(b'2') => {
                let is_rename = entry.starts_with("2 ");
                let fields: Vec<&str> = if is_rename {
                    entry.splitn(10, ' ').collect()
                } else {
                    entry.splitn(9, ' ').collect()
                };
                let path = fields
                    .last()
                    .copied()
                    .filter(|path| !path.is_empty())
                    .ok_or(GitError::InvalidOutput)?;
                let status_pair = fields
                    .get(1)
                    .and_then(|value| value.chars().next())
                    .ok_or(GitError::InvalidOutput)?;
                let worktree = fields
                    .get(1)
                    .and_then(|value| value.chars().nth(1))
                    .ok_or(GitError::InvalidOutput)?;
                let original_path = if is_rename {
                    Some(
                        entries
                            .next()
                            .filter(|path| !path.is_empty())
                            .ok_or(GitError::InvalidOutput)?
                            .to_owned(),
                    )
                } else {
                    None
                };
                let file = ChangedFile {
                    path: path.to_owned(),
                    original_path,
                    index_status: status_pair,
                    worktree_status: worktree,
                };
                if status_pair != '.' {
                    status.staged.push(file.clone());
                }
                if worktree != '.' {
                    status.unstaged.push(file.clone());
                }
                if status_pair == 'U' || worktree == 'U' {
                    status.conflicted.push(file);
                }
            }
            Some(b'u') => {
                let fields: Vec<&str> = entry.splitn(11, ' ').collect();
                let path = fields
                    .last()
                    .copied()
                    .filter(|path| !path.is_empty())
                    .ok_or(GitError::InvalidOutput)?;
                let file = ChangedFile {
                    path: path.to_owned(),
                    original_path: None,
                    index_status: 'U',
                    worktree_status: 'U',
                };
                status.conflicted.push(file.clone());
                status.staged.push(file.clone());
                status.unstaged.push(file);
            }
            Some(b'?') => status.untracked.push(ChangedFile {
                path: entry
                    .strip_prefix("? ")
                    .filter(|path| !path.is_empty())
                    .ok_or(GitError::InvalidOutput)?
                    .to_owned(),
                original_path: None,
                index_status: '.',
                worktree_status: '?',
            }),
            Some(b'!') => status.ignored.push(ChangedFile {
                path: entry
                    .strip_prefix("! ")
                    .filter(|path| !path.is_empty())
                    .ok_or(GitError::InvalidOutput)?
                    .to_owned(),
                original_path: None,
                index_status: '.',
                worktree_status: '!',
            }),
            _ => return Err(GitError::InvalidOutput),
        }
    }
    if status.head_oid.is_none() && matches!(status.head, HeadState::Branch(_)) {
        status.head = HeadState::Unborn;
    }
    Ok(status)
}

fn parse_signed_count(value: &str) -> Option<usize> {
    let (sign, count) = value.split_at_checked(1)?;
    if !matches!(sign, "+" | "-") {
        return None;
    }
    count.parse().ok()
}
