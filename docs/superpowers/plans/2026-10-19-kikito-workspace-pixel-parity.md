# Kikito Terminal Workspace UI Implementation Plan

> **For agentic workers:** Execute one task at a time. Keep the current workspace model, SQLite state, terminal sessions and local Git service. Each task has a visual and interaction gate before the next task starts.

**Goal:** Rebuild organizations, projects, tabs, Home, dialogs, menus and Source Control UI to match the supplied design at 98% or better while restoring reliable clicking.

**Architecture:** Keep the existing Rust/WarpUI domain and actions. Replace the view composition in focused local workspace modules, reuse shared UI primitives, and validate each surface against the 1280×800 source prototype before moving on.

**Tech Stack:** Rust, WarpUI, SQLite persistence, app integration harness, source React/CSS kit as visual reference, screenshot pixel comparison.

---

## Constraints and source of truth

- The exact design archive is `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip`, SHA-256 `8CCDB225255F422E72B13915DE815911B62092345B73F31374B98BA6A8D09474`.
- Detailed measured components and source file paths are in `docs/design-harvest/kikito-workspace/`.
- Baseline is 1280×800 CSS px, scale 100%, dark Phenomenon theme, default tone Jarvis, no rail names, no nested tabs.
- Do not use fixture project names, branches, Git states or fake timers in the native app.
- Do not change global WarpUI fonts, terminal layout, terminal animations or workspace-wide theme tokens for this feature.
- Preserve the pre-existing working tree changes in Git operations. Inspect `git status --short` and diffs before modifying overlapping files.
- User has asked for work directly on the current main checkout; do not wait for intermediate visual approval.

## Task 0: Capture source and diagnose dead hit targets

**Files:** no product source changes before reproducing. Evidence under `output/design-harvest/kikito-workspace/`.

- [ ] Serve a clean extraction of the ZIP on loopback HTTP and verify React, ReactDOM, Babel, bundle, fonts and SVG requests succeeded.
- [ ] Capture reference Home and project at 1280×800 with the TweaksPanel hidden for capture only. Record console, CSS fonts, viewport, DPR and frame bounds.
- [ ] Launch the exact checked-out app in an isolated data profile and capture the same state without changing the installed user's profile.
- [ ] Click Home, each org tile, add org, project row, search field, row plus, row menu, tab, close tab, add tab and dialog submit; record which element received each event and which WorkspaceAction ran.
- [ ] Trace failures through layout bounds, z-index, `EventHandler::dispatch_event`, Stack dispatch order and action handler. Write a regression test for each proven failure before changing that path.
- [ ] Do not continue to Task 1 until the nonresponsive hit target and the app build identity are explained by evidence.

**Gate:** all source assets load, reference capture is valid, and each failed click has a minimal reproducible test.

## Task 1: Organization rail

**Files:**
- Modify: `app/src/workspace/view/local_workspace/org_rail.rs`
- Modify: `app/src/workspace/view/local_workspace/mod.rs`
- Modify: `app/src/workspace/view.rs` only for the proven focus/action/hit-test defect
- Create: `app/src/workspace/view/local_workspace/org_rail_tests.rs` when the rendering/action harness supports the assertion

- [ ] Rebuild 64 px rail, top Home tile, divider, 36 px organization tiles, left marker, add tile, settings, tooltip, image/initial, active/hover and agent badge from the measured source.
- [ ] Use existing `Icon`, `Button`, `Tooltip`, `NotificationDot` and theme APIs. Map `wsTint` to theme color alpha; do not use app icon yellow for rail chrome.
- [ ] Keep the actual native window chrome as the owner of title/window controls; compare before deciding whether the prototype's 34 px traffic-light row belongs in the in-app rail.
- [ ] Ensure a tile click runs only `SelectLocalWorkspaceOrganization`; right click/menu button opens the menu without changing selection.
- [ ] Add focused GUI event steps for Home, org 1, org N, add, settings and context menu; assert selected model state and no terminal command receives the click.
- [ ] Capture reference/native rail states at the same crop and score. Fix each visible difference before advancing.

**Gate:** all rail clicks and shortcuts pass; active, hover, image, initial, badge and overflow states achieve ≥98% visual parity; critical bounds differ ≤1 px.

## Task 2: Project sidebar

**Files:**
- Modify: `app/src/workspace/view/local_workspace/project_sidebar.rs`
- Modify: `app/src/workspace/view/local_workspace/context_menu.rs`
- Modify: `app/src/workspace/view/local_workspace/mod.rs`
- Create or update: `project_sidebar_tests.rs`, `context_menu_tests.rs`

- [ ] Rebuild 248 px sidebar with 34 px org header, 10/8/6 search region, 10 px project overline, scrolling list, 24 px row action targets and fixed New Project footer.
- [ ] Match 8 px outer row margins, `7px 8px 7px 10px` row padding, radius4, 12 px title, 14 px folder glyph, 10 px branch/path hint and the exact fg overlay values.
- [ ] Match hover replacement: show plus and overflow buttons only while hovered/focused; show live status and count when not hovered. Never let those child buttons trigger parent row selection.
- [ ] Implement path/name search, total count, empty org, no-result and path checking/available/missing/not-directory/inaccessible states without changing the selected project.
- [ ] Test click, right click, plus, menu open/selection, filter field focus/typing/clear, list scroll and long Unicode paths.
- [ ] Capture/compare default, selected, hover, overflow, empty, no results and error states.

**Gate:** all sidebar interactions pass; visual score ≥98% for all listed states before Task 3.

## Task 3: Project tab bar and terminal canvas

**Files:**
- Modify: `app/src/workspace/view/local_workspace/mod.rs`
- Create: `app/src/workspace/view/local_workspace/project_tabs.rs`
- Modify: `app/src/workspace/view.rs`
- Create/update: `project_tabs_tests.rs`, workspace integration steps

- [ ] Move tab rendering into a focused module without changing project-tab identity/model semantics.
- [ ] Match 34 px bar, tab height26, min72/max220, 10 px left padding/24 px close reserve, status glyph, active/hover borders/background, plus, launcher and Git toggle.
- [ ] Add focused hit targets for selecting, closing, reordering and adding tabs. Child close click must stop parent selection and never close the wrong tab.
- [ ] Preserve terminal pane/session objects while switching projects. Project with no open tab shows the zero state and creates exactly one tab/session from the button.
- [ ] Test project with 0/1/many tabs, agent/terminal, active fallback after close, long tab title, cross-project restoration and keyboard navigation.
- [ ] Compare tab/canvas captures at baseline viewport.

**Gate:** tab/canvas visual score ≥98%; every action targets the right tab and preserves session identity.

## Task 4: Home and project/tab launcher

**Files:**
- Modify: `app/src/workspace/view/local_workspace/home.rs`
- Create if required: `app/src/workspace/view/local_workspace/home_ask.rs`
- Create/update: `home_tests.rs`, `mod_tests.rs`

- [ ] Match max-width960, padding48/40/40, section gap32, title23, sub14, logo44 by tone, and copy from the approved tone variants.
- [ ] Implement 44 px launcher, focus accent border, Cmd+K keycap, six results, arrow/Enter/Escape handling and true project/tab navigation.
- [ ] Implement six recent cards with 220 px minimum grid width, card spacing/hover, organization/branch/tab-count/recent-time fields.
- [ ] Implement seven recent tab rows at 40 px height and the exact status indicator/text layout.
- [ ] Make recency derive from persisted local data, not fixtures. Ensure empty Home and zero-result states have valid calls to create/open.
- [ ] Compare empty/seeded/sparse/wide query/blocked agent states.

**Gate:** all Home actions pass and score ≥98% across tones, results, cards and recent rows.

## Task 5: Organization/project create, edit, remove and crop flows

**Files:**
- Modify: `app/src/workspace/view/local_workspace/create_dialog.rs`
- Modify: `app/src/workspace/view/local_workspace/image_cropper.rs`
- Modify: `app/src/workspace/view.rs`
- Update tests in `create_dialog_tests.rs`, `image_cropper_tests.rs` and the workspace integration harness

- [ ] Match 420 px dialog, title18/600, 24 px sides, field gap14, form footer and status/error positions.
- [ ] Keep all text and errors in pt-BR; keyboard labels, Git terms, path, branch and user data remain canonical.
- [ ] Validate name, real project directory, image type/size and generation when async work returns. A canceled picker leaves dialog values intact.
- [ ] Match 56 px organization image field, six 16 px color dots, 240 px crop window, 128 px output, pointer pan, 1×–4× zoom, wheel zoom, grid during pan and crop actions.
- [ ] Add confirmation text for metadata removal; verify test fixture directories remain present and sessions stay open.
- [ ] Cover submit via Enter and button, Cancel, X, Escape, click outside, invalid form, picker cancel, image processing failure and persist failure.
- [ ] Capture every open/invalid/loading/error/confirmation/crop state at reference viewport.

**Gate:** score ≥98% for dialogs/crop and no form loses state or deletes user files.

## Task 6: Context menus, keyboard and overlay behavior

**Files:**
- Modify: `context_menu.rs`, `org_rail.rs`, `project_sidebar.rs`, `workspace/view.rs`
- Add/update integration steps for menu and modal layering

- [ ] Reuse native Menu/Button primitives and reproduce width186, row padding5×14, radius5, hover accent, separator, disabled/danger and shortcut appearance.
- [ ] Position menus within viewport from both mouse context event and explicit menu button.
- [ ] Implement Up/Down/Enter/Space/Escape and focus return; mouse click outside closes without swallowing the next action.
- [ ] Test menu at top/bottom/right edges, inside scroll, outside click, stacked menu+dialog and rapid repeated input.
- [ ] Audit all handlers at release-like Stack dispatch mode so duplicate broadcast cannot trigger multiple operations.

**Gate:** all menu/overlay events pass and visual score ≥98% for organization and project menus.

## Task 7: Git panel and graph visual integration

**Files:**
- Modify: `app/src/workspace/view/local_workspace/source_control/*.rs`
- Modify only proven Git UI action bindings in `app/src/workspace/view.rs` and `app/src/workspace/action.rs`
- Test: `app/src/local_git/*tests.rs` and targeted Source Control integration tests

- [ ] Match panel width clamp300–380, header34, section header26, file rows24, status bar28, commit controls, color, icon sizes and graph lane geometry from `GitPanel.jsx`/`GitGraph.jsx`.
- [ ] Use real `RepositoryStatus`, refs, commits and `app/src/local_git` operations; never import fixtures or timers.
- [ ] Make stage/unstage/discard/commit, section collapse, graph expand, filters and close panel independently clickable.
- [ ] Keep source operations local unless an explicit user command initiates remote work; show actual remote and branch for any remote action.
- [ ] Test a temporary repo for clean/dirty/staged/conflict/no repo/operation error and graph branch topology; add visual states after functionality passes.
- [ ] Capture the supplied VS Code reference panel as an adjacent functional/layout reference; do not copy its repo names or theme wholesale.

**Gate:** all currently in-scope Source Control interactions pass, no fixture appears, and panel/graph visual score ≥98%.

## Task 8: Integrated regression and delivery evidence

**Files:** evidence artifacts and final UI modules only.

- [ ] Run targeted Rust/UI tests for every touched package.
- [ ] Run GUI integration flows for rail/sidebar/tab/dialog/menu and Git view.
- [ ] Capture all reference/native state pairs at 1280×800, then theme-light, long-content, scroll and reduced-motion verification.
- [ ] Generate component scorecard and list each bounding-box error, pixel error, font/icon identity and exception mask.
- [ ] Verify persistence by closing/reopening app in an isolated test profile; assert same organization, project, tab and active tab.
- [ ] Verify no terminal fonts, shell behavior, hovers or animations changed outside new workspace modules.
- [ ] Run targeted formatter after implementation changes and self-review `git diff` including pre-existing dirty Git changes.

**Final gate:** every stage records ≥98% visual parity with no critical subcategory below 95%; all functional flows pass; every click target works; persisted state and terminals are intact.
