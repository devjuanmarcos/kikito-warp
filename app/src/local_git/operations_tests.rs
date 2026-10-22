use std::path::Path;

use super::operations::{GitOperation, OperationKind};

#[test]
fn status_and_ref_reads_are_local_only() {
    let operations: [GitOperation; 7] = [
        GitOperation::Status,
        GitOperation::DiscoverRepository,
        GitOperation::LocalBranches,
        GitOperation::ResolveReference {
            reference: "main".to_owned(),
        },
        GitOperation::ValidateBranchName {
            name: "feature/example".to_owned(),
        },
        GitOperation::History { limit: 20 },
        GitOperation::MergeBase {
            left: "HEAD".to_string(),
            right: "main".to_string(),
        },
    ];
    for operation in operations {
        assert_eq!(operation.kind(), OperationKind::LocalRead);
        assert!(
            !operation
                .arguments()
                .iter()
                .any(|arg| { matches!(arg.as_str(), "fetch" | "pull" | "push") })
        );
    }
}

#[test]
fn reverting_last_commit_creates_a_local_revert_commit() {
    let operation = GitOperation::RevertLastCommit;

    assert_eq!(operation.kind(), OperationKind::LocalWrite);
    assert_eq!(operation.arguments(), ["revert", "--no-edit", "HEAD"]);
}

#[test]
fn checkout_branch_uses_local_branch_switch_without_network_options() {
    let operation = GitOperation::CheckoutBranch {
        branch: "feature/example".to_owned(),
    };

    assert_eq!(operation.kind(), OperationKind::LocalWrite);
    assert_eq!(operation.arguments(), ["switch", "--", "feature/example"]);
}

#[test]
fn create_branch_from_ref_switches_to_the_new_local_branch() {
    let operation = GitOperation::CreateBranchFromRef {
        name: "feature/example".to_owned(),
        source: "main".to_owned(),
    };

    assert_eq!(operation.kind(), OperationKind::LocalWrite);
    assert_eq!(
        operation.arguments(),
        ["switch", "--create", "feature/example", "main"]
    );
}

#[test]
fn rename_branch_uses_local_branch_rename_without_network_options() {
    let operation = GitOperation::RenameBranch {
        old: "feature/old".to_owned(),
        new: "feature/new".to_owned(),
    };

    assert_eq!(operation.kind(), OperationKind::LocalWrite);
    assert_eq!(
        operation.arguments(),
        ["branch", "--move", "feature/old", "feature/new"]
    );
}

#[test]
fn explicit_remote_variants_are_the_only_network_capable_operations() {
    let operations: [GitOperation; 14] = [
        GitOperation::Status,
        GitOperation::DiscoverRepository,
        GitOperation::LocalBranches,
        GitOperation::ResolveReference {
            reference: "main".to_owned(),
        },
        GitOperation::History { limit: 10 },
        GitOperation::MergeBase {
            left: "HEAD".to_string(),
            right: "main".to_string(),
        },
        GitOperation::UndoLastCommit,
        GitOperation::ApplyCachedPatch { reverse: false },
        GitOperation::ValidateBranchName {
            name: "feature/example".to_owned(),
        },
        GitOperation::CreateBranchFromRef {
            name: "feature/example".to_owned(),
            source: "main".to_owned(),
        },
        GitOperation::RenameBranch {
            old: "feature/old".to_owned(),
            new: "feature/new".to_owned(),
        },
        GitOperation::Fetch {
            remote: "origin".to_string(),
        },
        GitOperation::Pull {
            remote: "origin".to_string(),
            branch: "main".to_string(),
        },
        GitOperation::Push {
            remote: "origin".to_string(),
            branch: "main".to_string(),
        },
    ];

    let network_count = operations
        .iter()
        .filter(|operation| operation.kind() == OperationKind::Remote)
        .count();
    assert_eq!(network_count, 3);
}

#[tokio::test]
async fn repository_discovery_finds_git_worktree_root() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    let nested = temporary.path().join("src").join("module");
    std::fs::create_dir_all(&nested).unwrap();

    let root = super::repository::discover_repository(&nested)
        .await
        .unwrap();
    assert_eq!(root, temporary.path());
}

#[test]
fn status_parser_distinguishes_staged_unstaged_untracked_and_renamed_files() {
    let output = concat!(
        "# branch.oid abc123\0",
        "# branch.head main\0",
        "# branch.ab +3 -2\0",
        "1 M. N... 100644 100644 100644 a a staged.txt\0",
        "1 .M N... 100644 100644 100644 b b modified.txt\0",
        "? new file.txt\0",
        "2 R. N... 100644 100644 100644 c c R100 renamed.txt\0",
        "old name.txt\0",
    );
    let status = super::status::parse_porcelain_v2(output).unwrap();
    assert_eq!(status.staged.len(), 2);
    assert_eq!(status.unstaged.len(), 1);
    assert_eq!(status.untracked.len(), 1);
    assert_eq!(status.staged[1].path, "renamed.txt");
    assert_eq!(
        status.staged[1].original_path.as_deref(),
        Some("old name.txt")
    );
    assert_eq!(
        status.upstream,
        Some(super::status::UpstreamCounts {
            ahead: 3,
            behind: 2
        })
    );
    assert_eq!(
        status.head,
        super::status::HeadState::Branch("main".to_owned())
    );
    assert_eq!(status.head_oid.as_deref(), Some("abc123"));
}

#[test]
fn status_parser_preserves_ignored_paths_and_unmerged_entries() {
    let output = concat!(
        "! ignored file.txt\0",
        "u UU N... 100644 100644 100644 100644 a b c conflicted file.txt\0",
    );
    let status = super::status::parse_porcelain_v2(output).unwrap();

    assert_eq!(status.ignored.len(), 1);
    assert_eq!(status.ignored[0].path, "ignored file.txt");
    assert_eq!(status.conflicted.len(), 1);
    assert_eq!(status.conflicted[0].path, "conflicted file.txt");
    assert_eq!(status.staged.len(), 1);
    assert_eq!(status.unstaged.len(), 1);
}

#[test]
fn status_parser_recognizes_an_unborn_branch() {
    let status =
        super::status::parse_porcelain_v2("# branch.oid (initial)\0# branch.head main\0").unwrap();

    assert_eq!(status.head, super::status::HeadState::Unborn);
    assert_eq!(status.head_oid, None);
}

#[test]
fn status_changed_path_count_includes_staged_only_and_deduplicates_conflicts() {
    let output = concat!(
        "1 M. N... 100644 100644 100644 a a staged-only.txt\0",
        "1 .M N... 100644 100644 100644 b b modified.txt\0",
        "? untracked.txt\0",
        "u UU N... 100644 100644 100644 100644 c d e conflicted.txt\0",
        "2 R. N... 100644 100644 100644 c c R100 renamed.txt\0",
        "old-name.txt\0",
    );
    let status = super::status::parse_porcelain_v2(output).unwrap();

    assert_eq!(status.staged.len(), 3);
    assert_eq!(status.unstaged.len(), 2);
    assert_eq!(status.untracked.len(), 1);
    assert_eq!(status.conflicted.len(), 1);
    assert_eq!(status.changed_path_count(), 6);
    assert_eq!(status.staged_path_count(), 2);
}

#[test]
fn status_parser_accepts_non_utf8_file_names_lossily() {
    let mut output = b"# branch.oid abc123\0# branch.head main\0? ".to_vec();
    output.push(0xff);
    output.extend_from_slice(b".png\0");

    let status = super::status::parse_porcelain_v2_bytes(&output).unwrap();

    assert_eq!(status.untracked.len(), 1);
    assert_eq!(status.untracked[0].path, "\u{fffd}.png");
    assert_eq!(
        status.head,
        super::status::HeadState::Branch("main".to_owned())
    );
}

#[test]
fn operation_paths_remain_single_arguments() {
    let path = Path::new("a path; with spaces/file.txt");
    let operation = GitOperation::Stage {
        path: path.to_path_buf(),
    };
    assert!(
        operation
            .arguments()
            .iter()
            .any(|arg| arg == &format!(":(literal){}", path.display()))
    );
}

#[test]
fn pathspecs_are_literal_except_for_explicit_workspace_root() {
    let wildcard_path = GitOperation::Stage {
        path: Path::new("[ab]*.txt").to_path_buf(),
    };
    assert!(
        wildcard_path
            .arguments()
            .contains(&":(literal)[ab]*.txt".to_owned())
    );

    let wildcard_diff = GitOperation::Diff {
        path: Path::new("[ab]*.txt").to_path_buf(),
        kind: super::operations::DiffKind::Unstaged,
    };
    assert!(
        wildcard_diff
            .arguments()
            .contains(&":(literal)[ab]*.txt".to_owned())
    );

    let all_paths = GitOperation::Unstage {
        path: Path::new(".").to_path_buf(),
    };
    assert!(all_paths.arguments().ends_with(&[".".to_owned()]));
}

#[tokio::test]
async fn repository_discovery_reports_non_repository_without_masking_spawn_errors() {
    let temporary = tempfile::tempdir().unwrap();
    assert_eq!(
        super::repository::discover_repository(temporary.path()).await,
        Err(super::errors::GitError::NotRepository)
    );
}

#[test]
fn status_parser_distinguishes_no_upstream_and_detached_head() {
    let no_upstream =
        super::status::parse_porcelain_v2("# branch.oid abc123\0# branch.head main\0").unwrap();
    assert_eq!(no_upstream.upstream, None);
    assert_eq!(
        no_upstream.head,
        super::status::HeadState::Branch("main".to_owned())
    );

    let detached =
        super::status::parse_porcelain_v2("# branch.oid abc123\0# branch.head (detached)\0")
            .unwrap();
    assert_eq!(detached.head, super::status::HeadState::Detached);
    assert_eq!(detached.upstream, None);
}

#[test]
fn status_parser_rejects_malformed_untracked_and_ignored_records() {
    for malformed in ["?", "? ", "!", "! "] {
        assert_eq!(
            super::status::parse_porcelain_v2(malformed),
            Err(super::errors::GitError::InvalidOutput)
        );
    }
}

#[test]
fn command_errors_expose_safe_actionable_categories_without_git_output() {
    use super::errors::CommandFailure;

    assert_eq!(
        super::operations::classify_failure(b"error: Your local changes would be overwritten"),
        CommandFailure::DirtyWorktree
    );
    assert_eq!(
        super::operations::classify_failure(b"fatal: not a git repository"),
        CommandFailure::NotRepository
    );
    assert_eq!(
        super::operations::classify_failure(b"fatal: Needed a single revision"),
        CommandFailure::MissingReference
    );
    let error = super::errors::GitError::CommandFailed {
        operation: "integrar altera\u{e7}\u{f5}es do remoto",
        code: Some(1),
        failure: CommandFailure::DirtyWorktree,
    }
    .to_string();
    assert!(error.contains("alterações locais"));
    assert!(!error.contains("http"));
}

#[test]
fn missing_reference_mapping_preserves_git_and_permission_errors() {
    use super::errors::{CommandFailure, GitError};

    assert_eq!(
        super::repository::map_resolution_error("main", GitError::GitUnavailable),
        GitError::GitUnavailable
    );
    let denied = GitError::CommandFailed {
        operation: "verificar a referência local",
        code: Some(128),
        failure: CommandFailure::PermissionDenied,
    };
    assert_eq!(
        super::repository::map_resolution_error("main", denied.clone()),
        denied
    );
    assert_eq!(
        super::repository::map_resolution_error(
            "missing",
            GitError::CommandFailed {
                operation: "verificar a referência local",
                code: Some(1),
                failure: CommandFailure::MissingReference,
            }
        ),
        GitError::MissingReference("missing".to_owned())
    );
    let unrelated = GitError::CommandFailed {
        operation: "verificar referência",
        code: Some(128),
        failure: CommandFailure::Other,
    };
    assert_eq!(
        super::repository::map_resolution_error("main", unrelated.clone()),
        unrelated
    );
    assert!(!unrelated.to_string().contains("Busque"));
    let missing = GitError::MissingReference("missing".to_owned());
    assert!(missing.to_string().contains("Busque"));
}

#[test]
fn local_branch_and_history_parsers_preserve_metadata() {
    let branches = "main\0abc123\0origin/main\0*\nfeature/topic\0def456\0\0 \n";
    let parsed_branches = super::parse_branches(branches).unwrap();
    assert_eq!(parsed_branches[0].name, "main");
    assert_eq!(parsed_branches[0].upstream.as_deref(), Some("origin/main"));
    assert!(parsed_branches[0].is_current);
    assert!(!parsed_branches[1].is_current);

    let history = concat!(
        "abc123\0parent1 parent2\0Ada\0",
        "2026-10-18T10:00:00-03:00\0merge commit\0HEAD -> main, origin/main, tag: v1\n",
        "def456\0\0Lin\0",
        "2026-10-17T09:00:00-03:00\0first commit\0\n",
    );
    let commits = super::parse_history(history).unwrap();
    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].parents, ["parent1", "parent2"]);
    assert_eq!(commits[0].subject, "merge commit");
    assert_eq!(commits[0].refs, ["HEAD -> main", "origin/main", "tag: v1"]);
    assert!(commits[1].refs.is_empty());
}

#[tokio::test]
async fn local_reads_do_not_contact_or_modify_remote_refs() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    git(temporary.path(), &["config", "commit.gpgsign", "false"]);
    std::fs::write(temporary.path().join("README.md"), "local\n").unwrap();
    git(temporary.path(), &["add", "README.md"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    git(
        temporary.path(),
        &["remote", "add", "origin", "http://127.0.0.1:1/no-network"],
    );
    std::fs::write(temporary.path().join("untracked file.txt"), "untracked\n").unwrap();
    std::fs::write(temporary.path().join(".gitignore"), "ignored file.txt\n").unwrap();
    std::fs::write(temporary.path().join("ignored file.txt"), "ignored\n").unwrap();
    let before = git_output(temporary.path(), &["show-ref", "--head"]);
    let tracked_before = std::fs::read(temporary.path().join("README.md")).unwrap();
    let index_before = std::fs::read(temporary.path().join(".git/index")).unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    let expected_branch = git_output(temporary.path(), &["branch", "--show-current"]);
    let status = service.status().await.unwrap();
    assert_eq!(
        status.head,
        super::status::HeadState::Branch(expected_branch.trim().to_owned())
    );
    assert!(
        status
            .untracked
            .iter()
            .any(|file| file.path == "untracked file.txt")
    );
    assert!(
        status
            .ignored
            .iter()
            .any(|file| file.path == "ignored file.txt")
    );
    assert!(
        !super::read_local_branches(temporary.path())
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(service.history(20).await.unwrap()[0].subject, "initial");
    let missing = service
        .merge_base("HEAD", "origin/missing")
        .await
        .unwrap_err()
        .to_string();
    assert!(missing.contains("Busque as referências manualmente"));
    let after = git_output(temporary.path(), &["show-ref", "--head"]);

    assert_eq!(before, after);
    assert_eq!(
        tracked_before,
        std::fs::read(temporary.path().join("README.md")).unwrap()
    );
    assert_eq!(
        index_before,
        std::fs::read(temporary.path().join(".git/index")).unwrap()
    );
}

#[tokio::test]
async fn checkout_refuses_to_overwrite_local_worktree_changes() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "base\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "base"]);
    git(temporary.path(), &["branch", "-M", "main"]);
    git(temporary.path(), &["switch", "-q", "-c", "feature"]);
    std::fs::write(temporary.path().join("tracked.txt"), "feature\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "feature"]);
    git(temporary.path(), &["switch", "-q", "main"]);
    std::fs::write(temporary.path().join("tracked.txt"), "local changes\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    assert!(service.checkout_branch("feature").await.is_err());

    assert_eq!(
        git_output(temporary.path(), &["branch", "--show-current"]).trim(),
        "main"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"local changes\n"
    );
}

#[tokio::test]
async fn create_branch_from_reference_uses_the_selected_commit() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "base\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "base"]);
    git(temporary.path(), &["branch", "-M", "main"]);
    let source = git_output(temporary.path(), &["rev-parse", "main"]);

    super::create_local_branch_from_ref(temporary.path(), "main", "feature/from-main")
        .await
        .unwrap();

    assert_eq!(
        git_output(temporary.path(), &["branch", "--show-current"]).trim(),
        "feature/from-main"
    );
    assert_eq!(git_output(temporary.path(), &["rev-parse", "HEAD"]), source);
}

#[tokio::test]
async fn renaming_current_local_branch_preserves_head_and_updates_branch_name() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "base\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "base"]);
    git(temporary.path(), &["branch", "-M", "main"]);
    let head_before = git_output(temporary.path(), &["rev-parse", "HEAD"]);

    super::rename_local_branch(temporary.path(), "main", "stable")
        .await
        .unwrap();

    assert_eq!(
        git_output(temporary.path(), &["branch", "--show-current"]).trim(),
        "stable"
    );
    assert_eq!(
        git_output(temporary.path(), &["rev-parse", "HEAD"]),
        head_before
    );
    assert!(git_output(temporary.path(), &["branch", "--list", "main"]).is_empty());
}

#[tokio::test]
async fn branch_creation_rejects_invalid_branch_names_without_mutating_refs() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "base\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "base"]);
    git(temporary.path(), &["branch", "-M", "main"]);

    let error = super::create_local_branch_from_ref(temporary.path(), "main", "feature..broken")
        .await
        .unwrap_err();

    assert!(matches!(error, super::GitError::InvalidBranchName));
    assert!(git_output(temporary.path(), &["branch", "--list", "feature..broken"]).is_empty());
    assert_eq!(
        git_output(temporary.path(), &["branch", "--show-current"]).trim(),
        "main"
    );
}

#[tokio::test]
async fn staging_and_unstaging_a_file_changes_only_the_index_state() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "before\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "after\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.stage(Path::new("tracked.txt")).await.unwrap();
    let staged = service.status().await.unwrap();
    assert_eq!(staged.staged.len(), 1);
    assert!(staged.unstaged.is_empty());

    service.unstage(Path::new("tracked.txt")).await.unwrap();
    let unstaged = service.status().await.unwrap();
    assert!(unstaged.staged.is_empty());
    assert_eq!(unstaged.unstaged.len(), 1);
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"after\n"
    );
}

#[tokio::test]
async fn unstage_all_preserves_worktree_content() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "before\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "after\n").unwrap();
    std::fs::write(temporary.path().join("new.txt"), "new\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.stage_all().await.unwrap();
    assert_eq!(service.status().await.unwrap().staged.len(), 2);
    service.unstage_all().await.unwrap();
    let status = service.status().await.unwrap();

    assert!(status.staged.is_empty());
    assert_eq!(status.unstaged.len(), 1);
    assert_eq!(status.untracked.len(), 1);
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"after\n"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("new.txt")).unwrap(),
        b"new\n"
    );
}

#[tokio::test]
async fn discard_restores_only_selected_tracked_worktree_file() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("one.txt"), "before one\n").unwrap();
    std::fs::write(temporary.path().join("two.txt"), "before two\n").unwrap();
    git(temporary.path(), &["add", "."]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("one.txt"), "after one\n").unwrap();
    std::fs::write(temporary.path().join("two.txt"), "after two\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service
        .discard_worktree(Path::new("one.txt"), false)
        .await
        .unwrap();

    let status = service.status().await.unwrap();
    assert_eq!(status.unstaged.len(), 1);
    assert_eq!(status.unstaged[0].path, "two.txt");
    assert_eq!(
        std::fs::read(temporary.path().join("one.txt")).unwrap(),
        b"before one\n"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("two.txt")).unwrap(),
        b"after two\n"
    );
}

#[tokio::test]
async fn discard_removes_only_selected_untracked_file() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    std::fs::write(temporary.path().join("one.txt"), "one\n").unwrap();
    std::fs::write(temporary.path().join("two.txt"), "two\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service
        .discard_worktree(Path::new("one.txt"), true)
        .await
        .unwrap();

    let status = service.status().await.unwrap();
    assert_eq!(status.untracked.len(), 1);
    assert_eq!(status.untracked[0].path, "two.txt");
    assert!(!temporary.path().join("one.txt").exists());
    assert_eq!(
        std::fs::read(temporary.path().join("two.txt")).unwrap(),
        b"two\n"
    );
}

#[tokio::test]
async fn undo_last_commit_moves_head_back_and_preserves_content() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "initial\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    let previous_head = git_output(temporary.path(), &["rev-parse", "HEAD"]);
    std::fs::write(temporary.path().join("tracked.txt"), "second\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "second"]);

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.undo_last_commit().await.unwrap();

    assert_eq!(
        git_output(temporary.path(), &["rev-parse", "HEAD"]),
        previous_head
    );
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"second\n"
    );
    let status = service.status().await.unwrap();
    assert_eq!(status.staged.len(), 1);
    assert_eq!(status.staged[0].path, "tracked.txt");
    assert!(status.unstaged.is_empty());
}

#[tokio::test]
async fn reverting_last_commit_preserves_history_and_restores_previous_content() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "initial\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "second\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "second"]);
    let reverted_commit = git_output(temporary.path(), &["rev-parse", "HEAD"]);

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.revert_last_commit().await.unwrap();

    assert_ne!(
        git_output(temporary.path(), &["rev-parse", "HEAD"]),
        reverted_commit
    );
    assert_eq!(
        git_output(temporary.path(), &["rev-parse", "HEAD^"]),
        reverted_commit
    );
    assert_eq!(
        git_output(temporary.path(), &["log", "-1", "--format=%s"]).trim(),
        "Revert \"second\""
    );
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"initial\n"
    );
    assert!(service.status().await.unwrap().staged.is_empty());
    assert!(service.status().await.unwrap().unstaged.is_empty());
}

#[tokio::test]
async fn amend_replaces_head_commit_and_keeps_hook_managed_commit_content() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "initial\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "second\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "second"]);
    let previous_head = git_output(temporary.path(), &["rev-parse", "HEAD"]);
    let parent = git_output(temporary.path(), &["rev-parse", "HEAD^"]);

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.commit("amended", true).await.unwrap();

    let new_head = git_output(temporary.path(), &["rev-parse", "HEAD"]);
    assert_ne!(new_head, previous_head);
    assert_eq!(
        git_output(temporary.path(), &["rev-parse", "HEAD^"]),
        parent
    );
    assert_eq!(
        git_output(temporary.path(), &["log", "-1", "--format=%s"]).trim(),
        "amended"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"second\n"
    );
    assert!(service.status().await.unwrap().staged.is_empty());
}

#[tokio::test]
async fn staging_one_hunk_keeps_other_hunks_unstaged_and_worktree_unchanged() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    let original = (1..=18)
        .map(|line| format!("line {line}\n"))
        .collect::<String>();
    std::fs::write(temporary.path().join("tracked.txt"), &original).unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    let changed = original
        .replace("line 1\n", "first hunk\n")
        .replace("line 18\n", "last hunk\n");
    std::fs::write(temporary.path().join("tracked.txt"), &changed).unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    let diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Unstaged,
        )
        .await
        .unwrap();
    let hunks = super::diff::parse_diff_hunks(&diff);
    assert_eq!(hunks.len(), 2);
    service
        .apply_hunk(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Unstaged,
            &hunks[0],
            false,
        )
        .await
        .unwrap();

    let status = service.status().await.unwrap();
    assert_eq!(status.staged.len(), 1);
    assert_eq!(status.unstaged.len(), 1);
    let staged_diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Staged,
        )
        .await
        .unwrap();
    let unstaged_diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Unstaged,
        )
        .await
        .unwrap();
    assert!(staged_diff.contains("first hunk"));
    assert!(!staged_diff.contains("last hunk"));
    assert!(unstaged_diff.contains("last hunk"));
    assert!(!unstaged_diff.contains("first hunk"));
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        changed.as_bytes()
    );
}

#[tokio::test]
async fn unstaging_one_hunk_keeps_other_hunks_staged_and_worktree_unchanged() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(temporary.path(), &["config", "core.autocrlf", "false"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    let original = (1..=18)
        .map(|line| format!("line {line}\n"))
        .collect::<String>();
    std::fs::write(temporary.path().join("tracked.txt"), &original).unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    let changed = original
        .replace("line 1\n", "first hunk\n")
        .replace("line 18\n", "last hunk\n");
    std::fs::write(temporary.path().join("tracked.txt"), &changed).unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    let diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Staged,
        )
        .await
        .unwrap();
    let hunks = super::diff::parse_diff_hunks(&diff);
    assert_eq!(hunks.len(), 2);
    service
        .apply_hunk(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Staged,
            &hunks[0],
            true,
        )
        .await
        .unwrap();

    let status = service.status().await.unwrap();
    assert_eq!(status.staged.len(), 1);
    assert_eq!(status.unstaged.len(), 1);
    let staged_diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Staged,
        )
        .await
        .unwrap();
    let unstaged_diff = service
        .diff(
            Path::new("tracked.txt"),
            super::operations::DiffKind::Unstaged,
        )
        .await
        .unwrap();
    assert!(staged_diff.contains("last hunk"));
    assert!(!staged_diff.contains("first hunk"));
    assert!(unstaged_diff.contains("first hunk"));
    assert!(!unstaged_diff.contains("last hunk"));
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        changed.as_bytes()
    );
}

#[tokio::test]
async fn staging_untracked_hunk_keeps_file_bytes_unchanged() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    let contents = (1..=8)
        .map(|line| format!("line {line}\n"))
        .collect::<String>();
    std::fs::write(temporary.path().join("new.txt"), &contents).unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    let diff = service
        .diff(Path::new("new.txt"), super::operations::DiffKind::Untracked)
        .await
        .unwrap();
    let hunks = super::diff::parse_diff_hunks(&diff);
    assert_eq!(hunks.len(), 1);
    service
        .apply_hunk(
            Path::new("new.txt"),
            super::operations::DiffKind::Untracked,
            &hunks[0],
            false,
        )
        .await
        .unwrap();

    let status = service.status().await.unwrap();
    assert_eq!(status.staged.len(), 1);
    assert!(status.untracked.is_empty());
    assert_eq!(
        std::fs::read(temporary.path().join("new.txt")).unwrap(),
        contents.as_bytes()
    );
}

#[tokio::test]
async fn commit_records_staged_content_and_preserves_later_worktree_edits() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "before\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    let initial_head = git_output(temporary.path(), &["rev-parse", "HEAD"]);
    std::fs::write(temporary.path().join("tracked.txt"), "staged\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    std::fs::write(temporary.path().join("tracked.txt"), "worktree\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.commit("salvar alteração", false).await.unwrap();

    assert_ne!(
        git_output(temporary.path(), &["rev-parse", "HEAD"]),
        initial_head
    );
    assert_eq!(
        git_output(temporary.path(), &["log", "-1", "--format=%s"]).trim(),
        "salvar alteração"
    );
    let status = service.status().await.unwrap();
    assert!(status.staged.is_empty());
    assert_eq!(status.unstaged.len(), 1);
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"worktree\n"
    );
}

#[tokio::test]
async fn commit_rejects_an_empty_message_without_changing_repository_state() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    std::fs::write(temporary.path().join("new.txt"), "new\n").unwrap();
    git(temporary.path(), &["add", "new.txt"]);
    let index_before = std::fs::read(temporary.path().join(".git/index")).unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    assert_eq!(
        service.commit("  ", false).await,
        Err(super::errors::GitError::EmptyCommitMessage)
    );
    assert_eq!(
        index_before,
        std::fs::read(temporary.path().join(".git/index")).unwrap()
    );
}

#[tokio::test]
async fn commit_all_stages_tracked_and_untracked_changes_without_editing_files() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "before\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "after\n").unwrap();
    std::fs::write(temporary.path().join("new.txt"), "new file\n").unwrap();

    let service = super::LocalGitService::open(temporary.path())
        .await
        .unwrap();
    service.commit_all("salvar tudo").await.unwrap();

    assert_eq!(
        git_output(temporary.path(), &["show", "HEAD:tracked.txt"]),
        "after\n"
    );
    assert_eq!(
        git_output(temporary.path(), &["show", "HEAD:new.txt"]),
        "new file\n"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"after\n"
    );
    assert_eq!(
        std::fs::read(temporary.path().join("new.txt")).unwrap(),
        b"new file\n"
    );
    assert_eq!(git_output(temporary.path(), &["status", "--porcelain"]), "");
}

#[tokio::test]
async fn staged_and_unstaged_diffs_compare_against_the_correct_git_snapshots() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    git(temporary.path(), &["config", "user.name", "Kikito Tests"]);
    git(
        temporary.path(),
        &["config", "user.email", "tests@example.invalid"],
    );
    std::fs::write(temporary.path().join("tracked.txt"), "base\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    git(temporary.path(), &["commit", "-qm", "initial"]);
    std::fs::write(temporary.path().join("tracked.txt"), "index version\n").unwrap();
    git(temporary.path(), &["add", "tracked.txt"]);
    std::fs::write(temporary.path().join("tracked.txt"), "worktree version\n").unwrap();

    let staged = super::diff(
        temporary.path(),
        Path::new("tracked.txt"),
        super::LocalGitDiffKind::Staged,
    )
    .await
    .unwrap();
    let unstaged = super::diff(
        temporary.path(),
        Path::new("tracked.txt"),
        super::LocalGitDiffKind::Unstaged,
    )
    .await
    .unwrap();

    assert!(staged.contains("-base"));
    assert!(staged.contains("+index version"));
    assert!(!staged.contains("worktree version"));
    assert!(unstaged.contains("-index version"));
    assert!(unstaged.contains("+worktree version"));
    let staged_rows = super::parse_unified_diff(&staged);
    let unstaged_rows = super::parse_unified_diff(&unstaged);
    assert!(staged_rows.iter().any(|row| {
        row.old_text.as_deref() == Some("base") && row.new_text.as_deref() == Some("index version")
    }));
    assert!(unstaged_rows.iter().any(|row| {
        row.old_text.as_deref() == Some("index version")
            && row.new_text.as_deref() == Some("worktree version")
    }));
    assert_eq!(
        std::fs::read(temporary.path().join("tracked.txt")).unwrap(),
        b"worktree version\n"
    );
}

#[tokio::test]
async fn untracked_diff_shows_new_file_without_staging_it() {
    let temporary = tempfile::tempdir().unwrap();
    git(temporary.path(), &["init", "-q"]);
    std::fs::write(temporary.path().join("new.txt"), "new content\n").unwrap();

    let diff = super::diff(
        temporary.path(),
        Path::new("new.txt"),
        super::LocalGitDiffKind::Untracked,
    )
    .await
    .unwrap();

    assert!(diff.contains("+new content"));
    assert!(git_output(temporary.path(), &["status", "--porcelain"]).contains("?? new.txt"));
}

fn git(cwd: &Path, args: &[&str]) {
    git_output(cwd, args);
}

fn git_output(cwd: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn missing_remote_is_reported_as_unavailable_remote() {
    use super::errors::CommandFailure;

    assert_eq!(
        super::operations::classify_failure(
            b"fatal: 'origin' does not appear to be a git repository
fatal: Could not read from remote repository."
        ),
        CommandFailure::RemoteUnavailable
    );
}

#[test]
fn nested_repositories_are_found_up_to_two_levels_and_not_inside_each_other() {
    let root = tempfile::tempdir().unwrap();
    let make_repo = |relative: &str| {
        let path = root.path().join(relative);
        std::fs::create_dir_all(path.join(".git")).unwrap();
        path
    };
    let api = make_repo("api");
    let web = make_repo("apps/web");
    make_repo("api/vendored");
    make_repo("apps/deep/too-far");
    make_repo("node_modules/package");
    std::fs::create_dir_all(root.path().join("docs")).unwrap();

    let found = super::repository::find_nested_repositories(root.path(), 2);

    assert_eq!(found, vec![api, web]);
}

#[test]
fn same_path_ignores_separators_case_and_trailing_slashes() {
    assert!(super::same_path(
        Path::new(r"C:\Projetos\Api\"),
        Path::new("c:/projetos/api")
    ));
    assert!(!super::same_path(
        Path::new("c:/projetos/api"),
        Path::new("c:/projetos/web")
    ));
}

#[test]
fn phase_two_operations_build_local_git_commands() {
    assert_eq!(
        GitOperation::DeleteBranch {
            branch: "feature/a".to_owned(),
            force: false
        }
        .arguments(),
        ["branch", "-d", "feature/a"]
    );
    assert_eq!(
        GitOperation::CheckoutRemoteBranch {
            remote_branch: "origin/feature/a".to_owned()
        }
        .arguments(),
        ["switch", "--track", "origin/feature/a"]
    );
    assert_eq!(
        GitOperation::StashApply {
            index: 2,
            pop: true
        }
        .arguments(),
        ["stash", "pop", "stash@{2}"]
    );
    assert_eq!(
        GitOperation::StashDrop { index: 0 }.arguments(),
        ["stash", "drop", "stash@{0}"]
    );
    assert_eq!(
        GitOperation::ResolveConflict {
            path: Path::new("src/a.rs").to_path_buf(),
            theirs: true
        }
        .arguments(),
        ["checkout", "--theirs", "--", ":(literal)src/a.rs"]
    );
    assert_eq!(GitOperation::StashList.kind(), OperationKind::LocalRead);
    assert_eq!(
        GitOperation::AbortPending(super::PendingOperation::CherryPick).arguments(),
        ["cherry-pick", "--abort"]
    );
    assert_eq!(
        super::operations::classify_failure(b"The previous cherry-pick is now empty"),
        super::errors::CommandFailure::NothingToApply
    );
}

#[test]
fn stash_remote_branch_and_commit_file_parsers() {
    assert_eq!(
        super::parse_stashes("stash@{0}\0WIP on main: abc\x002026-10-21T10:00:00-03:00\nstash@{1}\0On main: teste\x00\n")
            .unwrap(),
        vec![
            super::LocalStash {
                index: 0,
                message: "WIP on main: abc".to_owned(),
                created_at: "2026-10-21T10:00:00-03:00".to_owned(),
            },
            super::LocalStash {
                index: 1,
                message: "On main: teste".to_owned(),
                created_at: String::new(),
            },
        ]
    );
    assert_eq!(
        super::parse_remote_branches(
            "origin/HEAD\0refs/remotes/origin/main\norigin/main\0\norigin\0\n"
        ),
        vec!["origin/main".to_owned()]
    );
    assert_eq!(
        super::parse_commit_files("M\tsrc/a.rs\nR100\told.rs\tnew.rs\n"),
        vec![
            super::CommitFile {
                status: 'M',
                path: "src/a.rs".to_owned()
            },
            super::CommitFile {
                status: 'R',
                path: "new.rs".to_owned()
            },
        ]
    );
}
