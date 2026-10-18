# Kikito Terminal Workspace and Git Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a local organization/project workspace with persistent project tabs and a real, manually operated Git experience matching the approved design and VS Code native Git workflows.

**Architecture:** Add a local workspace domain in SQLite and connect it to existing terminal/session persistence without repurposing the existing `projects` or cloud `workspaces` tables. Build the new navigation in the desktop workspace UI and place local Git reads/commands behind a repository-scoped service that uses the user's installed Git and never performs implicit network operations.

**Tech Stack:** Rust, WarpUI, Diesel, SQLite migrations, existing terminal/session models, existing local Git helpers and watcher, installed Git executable, GUI integration framework, Rust unit tests.

---

## Approved references and invariants

Read `docs/superpowers/specs/2026-10-18-kikito-workspace-git-design.md` before implementation. The ZIP `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip` is the pixel reference; its React/localStorage and fake Git implementation are prototypes only. The target uses real app state and Git operations.

Keep these invariants throughout implementation:

- All organization/project metadata is local. CRUD does not move or delete user directories.
- Existing terminal/session persistence remains authoritative for process/session state. New persistence associates tabs with projects; it does not duplicate terminal state.
- Existing `projects(path)` remains available to its current callers. Existing cloud `workspaces` are not local organizations.
- Opening a Source Control panel, refreshing status, browsing refs, comparing local refs, and viewing history never invokes a remote operation.
- Fetch, pull, push, remote publish/delete, and sync require an explicit user action and show their destination.
- Never discard/stash/force checkout implicitly to make another Git action succeed.
- All new product UI strings are pt-BR. Do not add explanatory code comments; keep existing comments unless their behavior changes.
- Follow the repository `AGENTS.md` validation order. The user explicitly requested rigorous testing, so targeted tests, GUI verification, and network audit are in scope. Do not run `./script/presubmit` unless requested.

## Target file map

The exact module boundaries must be confirmed against current patterns before code changes; keep these responsibilities separate and adjust paths only to follow a stronger established pattern.

- `crates/persistence/migrations/2026-10-18-000000_create_local_workspace/up.sql` and `down.sql`: organizations, local projects, per-project tab membership, ordering and selected workspace state with foreign keys/indexes.
- `crates/persistence/src/schema.rs`, `model.rs`, and focused `*_tests.rs`: Diesel mappings and persistence models.
- `app/src/local_workspace/` (new module): domain model, commands/events, persistence adapter, path normalization and validation.
- `app/src/lib.rs`: initialize and retain the local workspace model using existing persistence startup patterns.
- `app/src/workspace/view.rs` and focused modules under `app/src/workspace/view/`: workspace composition and navigation state.
- `app/src/workspace/view/local_workspace/` (new focused modules): organization rail, project sidebar, home, project tab bar, create/edit dialogs, image crop flow, menus and visual state.
- `app/src/workspace/view/left_panel.rs`: integrate Source Control entry only where the existing panel/toolbelt contract requires it.
- `app/src/local_git/` (new module): repository-scoped read model, command service, typed operations/results, Git error mapping and watcher lifecycle.
- `app/src/util/git.rs`, `app/src/code_review/diff_state/local.rs`, and `app/src/code_review/git_repo_model/local.rs`: selectively extract/reuse local behavior and remove implicit fetch from any path used by the new local UI. Keep legacy code-review semantics covered.
- `app/src/workspace/view/local_workspace/source_control/` (new focused modules): changes tree, diff/staging controls, refs tree, graph, branch/commit menus, stash, worktrees, conflict editor, Timeline/blame and manual remote actions.
- `app/src/workspace/view_tests.rs`, adjacent `*_tests.rs`, `crates/persistence/src/*_tests.rs`, and `crates/integration/`: tests close to the corresponding existing patterns.

## Task 1: Confirm persistence/session contracts and define local workspace model

**Files:**
- Inspect: `app/src/workspace/`, `app/src/projects.rs`, `app/src/persistence/`, `crates/persistence/src/model.rs`, `crates/persistence/src/schema.rs`, tab/session migrations.
- Create: `app/src/local_workspace/model.rs`, `app/src/local_workspace/model_tests.rs`, `app/src/local_workspace/mod.rs`.
- Modify: `app/src/lib.rs` only after model/persistence contracts are covered.

- [ ] Trace how a persisted tab maps to a session, window, pane group, current directory and restoration callback. Record the existing identifiers and lifecycle in the module design before introducing any relation.
- [ ] Define local entities for organization, local project, selected organization/project, project order and tab membership. Keep branch/repository status derived from Git, not persisted as truth.
- [ ] Define invariants in tests: project path is normalized and unique within the intended product scope; one tab belongs to at most one project; deleting organization metadata cannot delete project directories or terminal sessions; selecting a project does not mutate another project's active tab.
- [ ] Run the smallest new model test target and confirm the tests fail for absent behavior before implementing model operations.
- [ ] Implement pure validation/normalization and ordering operations with deterministic tests for Windows drive paths, UNC paths, separators, case handling, Unicode, duplicate paths and missing directories.

## Task 2: Add reversible SQLite persistence

**Files:**
- Create: `crates/persistence/migrations/2026-10-18-000000_create_local_workspace/up.sql`, `down.sql`.
- Modify: `crates/persistence/src/schema.rs`, `crates/persistence/src/model.rs`, persistence connection/migration registration if generated by local conventions.
- Create: `crates/persistence/src/local_workspace_tests.rs` and include it through the repository's required test-module convention.
- Modify: `app/src/persistence/` for transactions/commands and add focused tests there.

- [ ] Add tables for organizations, local workspace projects, project-tab membership and persisted selection/order. Use foreign keys with explicit delete behavior and indexes for path/ordering queries.
- [ ] Add `down.sql` that reverses only the new schema. Do not alter or drop the legacy `projects`, `tabs`, `windows`, `tab_groups`, cloud `workspaces`, or user directories.
- [ ] Add Diesel table declarations and typed insert/query models; run Diesel schema/type compilation as the smallest targeted feedback.
- [ ] Implement transactional create, rename, reorder, select and metadata-delete operations. A metadata delete cascades only through the new local workspace relations; session rows remain intact.
- [ ] Add migration tests starting from a database containing legacy project and session rows. Assert migration preserves all legacy rows and that rollback removes only new tables/data.
- [ ] Add persistence tests for duplicate normalized paths, orphan prevention, transaction rollback and restoration of organization/project/tab selection.

## Task 3: Build organization and project navigation

**Files:**
- Create: `app/src/workspace/view/local_workspace/org_rail.rs`, `project_sidebar.rs`, `home.rs`, `create_dialog.rs`, `image_cropper.rs`, `context_menu.rs`, `mod.rs` (split further if an existing view exceeds a clear responsibility).
- Modify: `app/src/workspace/view.rs`, `app/src/workspace/view/left_panel.rs`, relevant action/keybinding registries and app initialization.
- Create: focused render-to-structure/unit tests beside new view modules and GUI integration scenarios under `crates/integration/`.

- [ ] Capture baseline app screenshots and record viewport, scale, theme and font settings. Extract reference dimensions and exact colors/type/spacing from ZIP `tokens/`, `guidelines/` and `ui_kits/workspace/`; map them to existing WarpUI tokens/components.
- [ ] Implement layout geometry first: 64px organization rail, 248px project sidebar, 34px project tab bar and remaining terminal canvas according to the kit; verify constraints at the reference viewport before adding interactions.
- [ ] Implement org selection, add/edit/remove dialogs and accessible context menus. Removing an organization requires confirmation and states clearly that project folders and sessions remain on disk.
- [ ] Implement project create/rename/remove, path picking, duplicate detection, search, recent state and branch/tab summary. A project without Git remains usable as a terminal project.
- [ ] Implement image upload/crop with file-type/size/read validation, bounded zoom, crop preview and persisted local image bytes/path using the existing asset convention.
- [ ] Implement menu keyboard behavior: right-click and menu button, target-specific actions, disabled-state explanations, focus order, Escape dismissal and in-window positioning.
- [ ] Implement Home/recent cards and empty/loading/error states from the design kit. Do not implement the prototype's tweaks panel as a product settings feature.
- [ ] Add visual assertions or screenshots for initial, empty, selected, hover, context-menu, create/edit, invalid-image and narrow-window states; compare against reference with a documented tolerance for font rasterization.

## Task 4: Bind project tabs to existing terminal sessions

**Files:**
- Modify: `app/src/workspace/view.rs`, `app/src/workspace/tab_group.rs`, `app/src/workspace/registry.rs`, existing session restore/persistence modules as identified in Task 1.
- Create: focused project-tab model tests and GUI integration flows in `crates/integration/`.

- [ ] Add an end-to-end test that creates two projects, associates separate existing terminal tabs, switches projects, and verifies selected tab, session reuse, and cwd isolation. Model tests now cover per-project active-tab selection and UUID ownership, but not the WorkspaceView session/cwd flow.
- [x] Add a restart/restore persistence test that stores organizations, projects, tab membership, active tab and UUID identity, then reopens SQLite and reconstructs the local workspace model.
- [x] Implement project-scoped tab creation from the project's root path. The current terminal UI has no per-tab cwd picker, so project creation uses the validated root; a missing root falls back to the existing startup-directory behavior.
- [x] Implement project switching without closing or recreating sessions; switching resolves the stored UUID to the existing terminal tab and restores its active selection.
- [x] Implement tab close through the existing terminal close lifecycle. Removing project metadata leaves terminal sessions available from Home without closing them.
- [x] Integrate project tab creation, close, reorder, selection and active-tab updates with existing terminal sessions and persist membership/active selection in one transaction.
- [ ] Run focused persistence and GUI integration tests for two-project isolation, empty project, last tab, app restart and closed/missing cwd. `cargo check -p warp --lib` passes; the app test harness was interrupted when free RAM fell to about 1.6 GB, and Windows GUI integration is unavailable in this environment.

## Task 5: Establish the local Git service and remove implicit network reads

**Files:**
- Create: `app/src/local_git/repository.rs`, `status.rs`, `operations.rs`, `errors.rs`, `mod.rs`, with test modules.
- Modify: `app/src/util/git.rs`, `crates/warp_util/src/git.rs`, `app/src/code_review/diff_state/local.rs`, `app/src/code_review/git_repo_model/local.rs` only for shared local behavior.
- Modify: `app/src/workspace/view/local_workspace/source_control/` to bind selected project to repository state.

- [ ] Create temporary-repository test helpers that initialize branches, commits, remotes and configured identities without network access; ensure each test cleans its temporary directory.
- [ ] Inventory every Git command reachable from status, branch listing, graph and compare. Classify each as local-read, local-write or network and assert panel/refresh/compare paths contain no fetch/pull/push.
- [ ] Add typed command execution that passes argument vectors directly to the Git process, uses the selected repo cwd, captures stdout/stderr/exit code, handles cancellation, and returns actionable localized UI errors without exposing secrets.
- [ ] Implement repository discovery, detached HEAD, status groups, renames, ignored/untracked distinction, staged/unstaged diffs and upstream counts from local refs only.
- [ ] Connect the existing repository watcher or a scoped replacement to project selection; ensure dropping project/UI handles stops watcher and async work.
- [ ] Audit `get_or_fetch_merge_base` and all callers. Add a local-only comparison path that reports missing ref/merge base and offers a manual Fetch action; do not silently fetch.
- [ ] Add tests asserting branch/status/history calls succeed with network blocked and do not modify refs or working files; add a command-boundary test that only explicit remote-operation variants may invoke network-capable commands.

## Task 6: Implement Source Control changes, diffs and commits

**Files:**
- Create: `app/src/workspace/view/local_workspace/source_control/changes.rs`, `diff.rs`, `commit.rs`, `mod.rs` and tests.
- Reuse selectively: `app/src/code_review/diff_state/local.rs` parsing and diff models; avoid dependencies on remote code-review models.
- GUI verification: integration scenarios under `crates/integration/`.

- [ ] Render actual Changes groups for staged, unstaged, untracked, renamed/deleted and conflicted files, including status glyphs/counts and empty/loading/error states.
- [ ] Implement stage/unstage for file and directory and verify with `git status --porcelain=v2` that index and worktree states match expectations.
- [ ] Implement discard with a confirmation naming affected paths and refusing to proceed if the selected operation would touch unselected files.
- [ ] Implement side-by-side diff for working tree/index/HEAD and committed revisions; handle rename, deletion, binary, large, invalid UTF-8 and CRLF files.
- [ ] Implement hunk/line staging by applying validated patch fragments to the index only. Add tests proving unselected lines remain unstaged and worktree content is byte-for-byte preserved.
- [ ] Implement commit staged, commit all, amend and undo/revert commands with explicit scope and confirmation where history or working files change. Preserve Git hooks, signing and identity config from installed Git.
- [ ] Add temporary-repo tests for each status transition and command failure; verify UI refresh happens after command completion and not before the Git result is known.

## Task 7: Implement refs tree, graph and branch/commit context menus

**Files:**
- Create: `app/src/workspace/view/local_workspace/source_control/refs.rs`, `graph.rs`, `branch_menu.rs`, `commit_menu.rs`, `comparison.rs` and tests.
- Reuse only local branch/commit formatting and graph primitives where their dependencies are local-safe.

- [ ] Load local/remote refs and tags from local Git metadata; group current, local, remote and tag refs; display stale refs as local metadata and never fetch during rendering.
- [ ] Render graph lanes and commit rows with stable identity, branch/tag decorations, selection, changed-file list and loading for older history.
- [ ] Add branch context menu actions: open changes, checkout, create branch from ref, rename, merge, rebase, compare with current/selected ref, publish and delete local/remote. Gate actions by ref type, current branch, remote availability and operation state.
- [ ] For checkout, let Git reject unsafe worktree changes; show the actual reason and offer inspect/stash/cancel, never auto-stash. For delete, prevent deleting checked-out branch and show the target name.
- [ ] Add commit context menu actions: open changes, compare with HEAD/ref/merge base, create branch from commit, cherry-pick, checkout detached, revert when safe, copy full/short SHA and inspect files.
- [ ] Add comparison view that names both refs and supports two-dot/merge-base semantics consistently; comparing must not switch branch or alter worktree.
- [ ] Add temporary-repository tests covering each menu command's enabled predicate and resulting refs/index/worktree; include cherry-pick conflict and abort.

## Task 8: Implement stash, worktrees and conflict workflows

**Files:**
- Create: `app/src/workspace/view/local_workspace/source_control/stash.rs`, `worktrees.rs`, `conflicts.rs`, tests, and any required local Git operation types.
- Modify: local workspace project opening/creation and existing diff/merge editor integration where available.

- [ ] Implement stash create/list/inspect/apply/pop/drop, with explicit controls for staged and untracked content. Test preservation of staged/index/worktree state, conflicts and failed pop.
- [ ] Implement worktree list/add/open/remove with branch/ref selection. Show worktree path and checked-out branch, prevent duplicate branch checkout, and require confirmation describing untracked/ignored-file risk before removal.
- [ ] Ensure opening a worktree creates/selects a local project entry without duplicating or moving the directory; closing its app entry does not remove the Git worktree.
- [ ] Build a three-way conflict presentation using existing editor/diff primitives: current, incoming and base panes, per-hunk accept actions, manual edit, mark resolved, continue/abort operation.
- [ ] Detect in-progress merge, rebase and cherry-pick state from Git; only expose valid continuation/abort actions, and keep conflict markers/paths until Git confirms completion.
- [ ] Test clean/conflicted merge, rebase, cherry-pick, stash apply, worktree add/remove, interrupted operation restart and invalid resolution.

## Task 9: Add file Timeline/blame and manual remote operations

**Files:**
- Create: `app/src/workspace/view/local_workspace/source_control/timeline.rs`, `blame.rs`, `remotes.rs` and tests.
- Modify: local Git service command types and UI action dispatch.

- [ ] Load file history and blame on demand for the selected repository/path; navigate from line attribution to commit details and diff without switching branch.
- [ ] Add Fetch, Pull, Push, publish branch, sync and delete remote branch as explicit menu/button actions. Present repository, remote, ref and direction before execution when multiple targets exist.
- [ ] Use installed Git's credential helper/config and do not prompt for or store Warp credentials. Map authentication/cancel/non-fast-forward/network failures to local messages with retry only after another user action.
- [ ] Add tests using local bare remotes for fetch/push/pull success and rejection paths; use a command spy/network-blocking harness to prove no command fires before user action.
- [ ] Verify remotes never run on app launch, workspace switch, status watcher updates, graph scroll, or Source Control panel open.

## Task 10: Pixel fidelity, accessibility and end-to-end verification

**Files:**
- Modify: visual/UI files created above only after screenshot review.
- Add/update: GUI integration scenarios in `crates/integration/`, view tests and test fixtures.
- Documentation: update the feature design/implementation notes with measured deviations and supported keyboard actions.

- [ ] Capture matched screenshots at the ZIP prototype's viewport and at compact width for Home, selected project, terminal tabs, Source Control, branch menu, commit menu, create/edit dialogs, stash/worktree/conflict states and empty/error states.
- [ ] Compare geometry, typography, color, icon, border, hover, selection and spacing against the ZIP. Fix measured mismatches; document only unavoidable native font/rasterization differences.
- [ ] Verify keyboard-only selection, open/close menu, navigation, dialog submit/cancel, focus restoration and shortcuts on Windows. Check accessible labels and disabled-reason text.
- [ ] Run focused Rust unit and persistence tests with `cargo nextest run -p persistence` and app/package-specific targets identified from Cargo manifests; run focused GUI integration scenarios with the repository integration runner.
- [ ] Run targeted Clippy for changed crates, relevant type/build checks, and then the applicable formatter once after all code changes, following `AGENTS.md`. Do not run the full presubmit unless explicitly requested.
- [ ] Run the complete Git temporary-repository matrix and a network audit with remote endpoints unavailable. Confirm local workflows remain functional and every network action is user initiated.
- [ ] Review `git diff` for accidental changes to Warp account/remote behavior, data deletion, secrets, generated files and code comments before delivery.

## Dependency order and completion gates

Tasks 1–2 establish the safe local data model. Tasks 3–4 build the project workspace on that model. Task 5 establishes the local-only Git boundary. Task 6 depends on Task 5; Task 7 depends on Tasks 5–6; Task 8 depends on the Git operation layer and existing diff editor; Task 9 depends on refs/history. Task 10 runs after all behavior is integrated and is the release gate.

Each task is complete only when its targeted tests pass, the GUI state is reviewed where applicable, and no implicit network operation or destructive side effect is introduced. A later code, migration, test, generated-code or configuration change reruns affected validation and finishes with the applicable formatter.
