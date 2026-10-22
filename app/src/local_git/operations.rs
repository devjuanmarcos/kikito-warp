use std::path::{Path, PathBuf};

use super::errors::{CommandFailure, GitError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OperationKind {
    LocalRead,
    LocalWrite,
    Remote,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum DiffKind {
    Staged,
    Unstaged,
    Untracked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GitOperation {
    Status,
    DiscoverRepository,
    LocalBranches,
    CheckoutBranch { branch: String },
    ValidateBranchName { name: String },
    CreateBranchFromRef { name: String, source: String },
    RenameBranch { old: String, new: String },
    ResolveReference { reference: String },
    History { limit: usize },
    MergeBase { left: String, right: String },
    Stage { path: PathBuf },
    StageAll,
    Unstage { path: PathBuf },
    DiscardWorktree { path: PathBuf, untracked: bool },
    Commit { message: String, amend: bool },
    UndoLastCommit,
    RevertLastCommit,
    ApplyCachedPatch { reverse: bool },
    Diff { path: PathBuf, kind: DiffKind },
    Fetch { remote: String },
    Pull { remote: String, branch: String },
    Push { remote: String, branch: String },
    RestoreAllWorktree,
    CleanUntracked,
    DeleteBranch { branch: String, force: bool },
    RemoteBranches,
    CheckoutRemoteBranch { remote_branch: String },
    MergeBranch { branch: String },
    MergeAbort,
    StashPush { include_untracked: bool },
    StashList,
    StashApply { index: usize, pop: bool },
    StashDrop { index: usize },
    CherryPick { commit: String },
    RevertCommit { commit: String },
    CommitFiles { commit: String },
    ResolveConflict { path: PathBuf, theirs: bool },
    Init,
}

impl GitOperation {
    pub(super) fn kind(&self) -> OperationKind {
        match self {
            Self::Status
            | Self::DiscoverRepository
            | Self::LocalBranches
            | Self::ValidateBranchName { .. }
            | Self::ResolveReference { .. }
            | Self::History { .. }
            | Self::MergeBase { .. }
            | Self::RemoteBranches
            | Self::StashList
            | Self::CommitFiles { .. } => OperationKind::LocalRead,
            Self::Diff { .. } => OperationKind::LocalRead,
            Self::Stage { .. }
            | Self::CheckoutBranch { .. }
            | Self::CreateBranchFromRef { .. }
            | Self::RenameBranch { .. }
            | Self::StageAll
            | Self::Unstage { .. }
            | Self::DiscardWorktree { .. }
            | Self::Commit { .. }
            | Self::UndoLastCommit
            | Self::RevertLastCommit
            | Self::ApplyCachedPatch { .. }
            | Self::RestoreAllWorktree
            | Self::CleanUntracked
            | Self::DeleteBranch { .. }
            | Self::CheckoutRemoteBranch { .. }
            | Self::MergeBranch { .. }
            | Self::MergeAbort
            | Self::StashPush { .. }
            | Self::StashApply { .. }
            | Self::StashDrop { .. }
            | Self::CherryPick { .. }
            | Self::RevertCommit { .. }
            | Self::ResolveConflict { .. }
            | Self::Init => OperationKind::LocalWrite,
            Self::Fetch { .. } | Self::Pull { .. } | Self::Push { .. } => OperationKind::Remote,
        }
    }

    pub(super) fn arguments(&self) -> Vec<String> {
        match self {
            Self::Status => vec![
                "status".into(),
                "--porcelain=v2".into(),
                "-z".into(),
                "--branch".into(),
                "--untracked-files=all".into(),
                "--ignored=matching".into(),
            ],
            Self::DiscoverRepository => vec!["rev-parse".into(), "--show-toplevel".into()],
            Self::LocalBranches => vec![
                "for-each-ref".into(),
                "--format=%(refname:short)%00%(objectname)%00%(upstream:short)%00%(HEAD)".into(),
                "refs/heads".into(),
            ],
            Self::CheckoutBranch { branch } => {
                vec!["switch".into(), "--".into(), branch.clone()]
            }
            Self::ValidateBranchName { name } => {
                vec!["check-ref-format".into(), format!("refs/heads/{name}")]
            }
            Self::CreateBranchFromRef { name, source } => vec![
                "switch".into(),
                "--create".into(),
                name.clone(),
                source.clone(),
            ],
            Self::RenameBranch { old, new } => {
                vec!["branch".into(), "--move".into(), old.clone(), new.clone()]
            }
            Self::ResolveReference { reference } => vec![
                "rev-parse".into(),
                "--verify".into(),
                format!("{reference}^{{commit}}"),
            ],
            Self::History { limit } => vec![
                "log".into(),
                // The stash's internal commits aren't history, like in VS Code's graph.
                "--exclude=refs/stash".into(),
                "--all".into(),
                "--date-order".into(),
                "--format=%H%x00%P%x00%an%x00%aI%x00%s%x00%D".into(),
                "-n".into(),
                (*limit).clamp(1, 5000).to_string(),
            ],
            Self::MergeBase { left, right } => {
                vec!["merge-base".into(), left.clone(), right.clone()]
            }
            Self::Stage { path } => vec!["add".into(), "--".into(), literal_pathspec(path)],
            Self::StageAll => vec!["add".into(), "--all".into(), "--".into(), ".".into()],
            Self::Unstage { path } => vec![
                "restore".into(),
                "--staged".into(),
                "--".into(),
                literal_pathspec(path),
            ],
            Self::DiscardWorktree { path, untracked } => {
                if *untracked {
                    vec![
                        "clean".into(),
                        "-f".into(),
                        "--".into(),
                        literal_pathspec(path),
                    ]
                } else {
                    vec![
                        "restore".into(),
                        "--worktree".into(),
                        "--".into(),
                        literal_pathspec(path),
                    ]
                }
            }
            Self::Commit { message, amend } => {
                let mut arguments = vec!["commit".into()];
                if *amend {
                    arguments.push("--amend".into());
                }
                arguments.extend(["-m".into(), message.clone()]);
                arguments
            }
            Self::UndoLastCommit => vec!["reset".into(), "--soft".into(), "HEAD~1".into()],
            Self::RevertLastCommit => vec!["revert".into(), "--no-edit".into(), "HEAD".into()],
            Self::ApplyCachedPatch { reverse } => {
                let mut arguments = vec!["apply".into(), "--cached".into()];
                if *reverse {
                    arguments.push("--reverse".into());
                }
                arguments
            }
            Self::Diff { path, kind } => {
                let mut arguments = vec!["diff".into()];
                if *kind == DiffKind::Untracked {
                    arguments.push("--no-index".into());
                }
                arguments.extend([
                    "--no-ext-diff".into(),
                    "--no-textconv".into(),
                    "--no-color".into(),
                    "--unified=3".into(),
                ]);
                if *kind == DiffKind::Staged {
                    arguments.push("--cached".into());
                }
                arguments.push("--".into());
                if *kind == DiffKind::Untracked {
                    arguments.push("/dev/null".into());
                }
                arguments.push(if *kind == DiffKind::Untracked {
                    path.to_string_lossy().into_owned()
                } else {
                    literal_pathspec(path)
                });
                arguments
            }
            Self::Fetch { remote } => vec!["fetch".into(), remote.clone()],
            Self::Pull { remote, branch } => {
                vec!["pull".into(), remote.clone(), branch.clone()]
            }
            Self::Push { remote, branch } => {
                // `-u` makes publishing a branch also set its upstream, so status can report sync.
                vec!["push".into(), "-u".into(), remote.clone(), branch.clone()]
            }
            Self::RestoreAllWorktree => vec![
                "restore".into(),
                "--worktree".into(),
                "--".into(),
                ".".into(),
            ],
            Self::CleanUntracked => vec!["clean".into(), "-fd".into(), "--".into(), ".".into()],
            Self::DeleteBranch { branch, force } => vec![
                "branch".into(),
                if *force { "-D" } else { "-d" }.into(),
                branch.clone(),
            ],
            Self::RemoteBranches => vec![
                "for-each-ref".into(),
                "--format=%(refname:short)%00%(symref)".into(),
                "refs/remotes".into(),
            ],
            Self::CheckoutRemoteBranch { remote_branch } => {
                vec!["switch".into(), "--track".into(), remote_branch.clone()]
            }
            Self::MergeBranch { branch } => vec!["merge".into(), "--no-edit".into(), branch.clone()],
            Self::MergeAbort => vec!["merge".into(), "--abort".into()],
            Self::StashPush { include_untracked } => {
                let mut arguments = vec!["stash".into(), "push".into()];
                if *include_untracked {
                    arguments.push("--include-untracked".into());
                }
                arguments
            }
            Self::StashList => vec![
                "stash".into(),
                "list".into(),
                "--format=%gd%x00%s%x00%cI".into(),
            ],
            Self::StashApply { index, pop } => vec![
                "stash".into(),
                if *pop { "pop" } else { "apply" }.into(),
                format!("stash@{{{index}}}"),
            ],
            Self::StashDrop { index } => {
                vec!["stash".into(), "drop".into(), format!("stash@{{{index}}}")]
            }
            Self::CherryPick { commit } => vec!["cherry-pick".into(), commit.clone()],
            Self::RevertCommit { commit } => {
                vec!["revert".into(), "--no-edit".into(), commit.clone()]
            }
            Self::CommitFiles { commit } => vec![
                "show".into(),
                "--no-color".into(),
                "--name-status".into(),
                "--format=".into(),
                commit.clone(),
            ],
            Self::ResolveConflict { path, theirs } => vec![
                "checkout".into(),
                if *theirs { "--theirs" } else { "--ours" }.into(),
                "--".into(),
                literal_pathspec(path),
            ],
            Self::Init => vec!["init".into()],
        }
    }

    pub(super) fn label(&self) -> &'static str {
        match self {
            Self::Status => "ler o estado do repositório",
            Self::DiscoverRepository => "identificar o repositório local",
            Self::LocalBranches => "listar as branches locais",
            Self::CheckoutBranch { .. } => "mudar para a branch local selecionada",
            Self::ValidateBranchName { .. } => "validar o nome da nova branch",
            Self::CreateBranchFromRef { .. } => "criar e mudar para a nova branch local",
            Self::RenameBranch { .. } => "renomear a branch local selecionada",
            Self::ResolveReference { .. } => "verificar a referência local",
            Self::History { .. } => "consultar o histórico local",
            Self::MergeBase { .. } => "comparar referências locais",
            Self::Stage { .. } => "preparar o arquivo",
            Self::StageAll => "preparar todas as alterações",
            Self::Unstage { .. } => "retirar o arquivo da preparação",
            Self::DiscardWorktree { .. } => "descartar as alterações selecionadas",
            Self::Commit { .. } => "criar o commit local",
            Self::UndoLastCommit => "desfazer o último commit local",
            Self::RevertLastCommit => "criar um commit que reverta o último commit",
            Self::ApplyCachedPatch { .. } => "aplicar o bloco selecionado ao índice",
            Self::Diff { .. } => "comparar o arquivo com a versão local",
            Self::Fetch { .. } => "buscar alterações do remoto",
            Self::Pull { .. } => "integrar alterações do remoto",
            Self::Push { .. } => "enviar alterações ao remoto",
            Self::RestoreAllWorktree => "descartar as alterações dos arquivos rastreados",
            Self::CleanUntracked => "remover os arquivos não rastreados",
            Self::DeleteBranch { .. } => "excluir a branch local",
            Self::RemoteBranches => "listar as branches remotas",
            Self::CheckoutRemoteBranch { .. } => "criar a branch local a partir da remota",
            Self::MergeBranch { .. } => "fazer o merge da branch",
            Self::MergeAbort => "abortar o merge",
            Self::StashPush { .. } => "guardar as alterações no stash",
            Self::StashList => "listar os stashes",
            Self::StashApply { pop: false, .. } => "aplicar o stash",
            Self::StashApply { pop: true, .. } => "aplicar e remover o stash",
            Self::StashDrop { .. } => "descartar o stash",
            Self::CherryPick { .. } => "aplicar o commit (cherry-pick)",
            Self::RevertCommit { .. } => "reverter o commit",
            Self::CommitFiles { .. } => "listar os arquivos do commit",
            Self::ResolveConflict { .. } => "resolver o conflito do arquivo",
            Self::Init => "inicializar o repositório",
        }
    }
}

fn literal_pathspec(path: &Path) -> String {
    if path == Path::new(".") {
        ".".into()
    } else {
        format!(":(literal){}", path.to_string_lossy())
    }
}

pub(super) async fn execute(
    repository: &Path,
    operation: GitOperation,
) -> Result<String, GitError> {
    let arguments = operation.arguments();
    let stdout = execute_bytes(repository, &operation, &arguments).await?;
    String::from_utf8(stdout).map_err(|_| GitError::InvalidOutput)
}

pub(super) async fn execute_bytes(
    repository: &Path,
    operation: &GitOperation,
    arguments: &[String],
) -> Result<Vec<u8>, GitError> {
    let mut git_arguments = vec!["-c".to_owned(), "diff.autoRefreshIndex=false".to_owned()];
    git_arguments.extend(arguments.iter().cloned());
    let argument_refs = git_arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let env = [("GIT_OPTIONAL_LOCKS", "0")];
    let output = warp_util::git::run_git_command_output(repository, &argument_refs, &env)
        .await
        .map_err(|_| GitError::GitUnavailable)?;
    // `git diff` exits with 1 when it finds differences.
    let diff_found =
        matches!(operation, GitOperation::Diff { .. }) && output.status.code() == Some(1);
    if !output.status.success() && !diff_found {
        return Err(GitError::CommandFailed {
            operation: operation.label(),
            code: output.status.code(),
            failure: classify_failure(&output.stderr),
        });
    }
    Ok(output.stdout)
}

pub(super) async fn execute_with_input(
    repository: &Path,
    operation: &GitOperation,
    input: &[u8],
) -> Result<(), GitError> {
    let arguments = operation.arguments();
    let mut git_arguments = vec!["-c".to_owned(), "diff.autoRefreshIndex=false".to_owned()];
    git_arguments.extend(arguments);
    let argument_refs = git_arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let env = [("GIT_OPTIONAL_LOCKS", "0")];
    let output =
        warp_util::git::run_git_command_with_input(repository, &argument_refs, &env, input)
            .await
            .map_err(|_| GitError::GitUnavailable)?;
    if !output.status.success() {
        return Err(GitError::CommandFailed {
            operation: operation.label(),
            code: output.status.code(),
            failure: classify_failure(&output.stderr),
        });
    }
    Ok(())
}

pub(super) fn classify_failure(stderr: &[u8]) -> CommandFailure {
    let stderr = String::from_utf8_lossy(stderr).to_ascii_lowercase();
    if stderr.contains("not a git repository") {
        CommandFailure::NotRepository
    } else if stderr.contains("needed a single revision")
        || stderr.contains("unknown revision or path not in the working tree")
        || stderr.contains("not a valid object name")
        || stderr.contains("bad revision")
    {
        CommandFailure::MissingReference
    } else if stderr.contains("conflict") || stderr.contains("unmerged") {
        CommandFailure::Conflict
    } else if stderr.contains("local changes") || stderr.contains("would be overwritten") {
        CommandFailure::DirtyWorktree
    } else if stderr.contains("permission denied") || stderr.contains("access is denied") {
        CommandFailure::PermissionDenied
    } else if stderr.contains("not something we can merge")
        || stderr.contains("no merge base")
        || stderr.contains("no common ancestor")
    {
        CommandFailure::NoCommonHistory
    } else if stderr.contains("does not appear to be a git repository")
        || stderr.contains("could not read from remote repository")
        || stderr.contains("no configured push destination")
        || stderr.contains("could not resolve host")
    {
        CommandFailure::RemoteUnavailable
    } else {
        CommandFailure::Other
    }
}

pub(super) fn validate_reference(reference: &str) -> Result<(), GitError> {
    if reference.is_empty() || reference.starts_with('-') || reference.contains('\0') {
        return Err(GitError::InvalidReference);
    }
    Ok(())
}

pub(super) fn validate_remote(remote: &str) -> Result<(), GitError> {
    if remote.is_empty() || remote.starts_with('-') || remote.contains('\0') {
        return Err(GitError::InvalidReference);
    }
    Ok(())
}

pub(super) fn validate_path(path: &Path) -> Result<(), GitError> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(GitError::InvalidReference);
    }
    Ok(())
}
