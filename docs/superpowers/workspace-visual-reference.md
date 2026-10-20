# Workspace visual reference (legacy summary)

> Especificação atualizada e inventário completo: [coleta de design do workspace](../design-harvest/kikito-workspace/00-overview.md). Evidência de captura: [08-playwright-evidence.md](../design-harvest/kikito-workspace/08-playwright-evidence.md). Use a coleta como fonte de verdade.

Reference source: `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip`, extracted from `tokens/`, `guidelines/`, and `ui_kits/workspace/`.

## Capture baseline

The kit targets a **1280 × 800 CSS-pixel viewport**. The reference provides both dark and light themes; the implementation baseline is the app's active dark theme. Target UI scale is 100%. UI text uses Roboto at 12 px; terminal text uses Hack at 13 px. App screenshot capture is pending: the installed profile was not launched or modified during implementation.

## Geometry and type

| Element | Reference size |
| --- | ---: |
| Organization rail | 64 px |
| Project sidebar | 248 px |
| Project tab bar | 34 px |
| Base UI text | 12 px |
| Overline | 10 px |
| Dialog title | 18 px |
| Small / medium / round radius | 4 / 8 / 9999 px |
| Standard button | 32 px high, 12 px horizontal padding |

Common spacing values are 4, 8, 12, 16, 20 and 24 px. Borders are 1 px.

## Theme mapping

The kit derives its colors from the existing Warp theme inputs. For the dark theme, the inputs are background `#050505`, foreground `#FFFFFF`, and accent `#19AAD8`; surfaces are `#121212`, `#1E1E1E`, `#2B2B2B`, and `#373737`. Text uses foreground opacity 90%, 60%, and 40%; outlines use foreground opacity 10%.

The light theme uses background `#FFFFFF`, foreground `#111111`, and accent `#00C2FF`; its first four surfaces are `#F3F3F3`, `#E7E7E7`, `#DBDBDB`, and `#CFCFCF`.

The organization accent choices in the kit are blue, green, magenta, yellow, cyan, and red. The application icon palette (`#FFB800`, `#FDCB03`, and `#080D11`) is separate from workspace chrome.

## Interaction and state coverage

Organization and project menus support right-click and an explicit menu button. When open, the workspace receives focus; Up and Down move the visible selection, Enter or Space activates it, Escape closes it, and a click outside dismisses it. Removing an organization requires confirmation and states that its folders and terminal sessions remain on disk. Removing the selected organization selects a remaining organization or Home and clears any stale project selection.

Recent projects are ordered by persisted last-opened time and show the terminal count. Project path status is checked asynchronously outside render with at most eight metadata operations in flight; missing paths, non-directory paths, and inaccessible paths have distinct labels, including for UNC paths. Persistence failures and workspace restore failures appear in the UI. Search distinguishes an empty organization from a query with no matches. Initial workspace hydration completes synchronously before the view is rendered; path checks show a checking state while their bounded worker runs.

The rail's organization tile and menu button expose hover styling and mouse/context-menu actions. WarpUI's current GUI `EventHandler` routes key events but does not make an element focusable or provide element-level focus/accessible-label semantics; focus is managed at the view level. The rail therefore has no independent Tab/arrow focus order yet. Menu rows are keyboard-navigable once their menu is open.

Project terminal switching and the live assignment of terminal tabs to projects remain in Task 4. This navigation slice hydrates and persists the existing project-tab identity records but does not change terminal selection behavior.

Organization image input accepts PNG, JPG, and SVG files up to 5 MiB. File metadata and reads run asynchronously, and reads are capped at 5 MiB plus one byte even if the file changes after metadata is checked. Decode, SVG rasterization, and crop work run off the UI thread through a serialized worker. Raster decode and SVG output are bounded to 4096 pixels per side, 4 million pixels, and a 40 MiB decode/raster allocation; the persisted preview is a 128 × 128 PNG. Zoom is limited to 1×-4×. Pointer dragging uses the source aspect ratio, moves image content with the pointer, and updates the preview when the drag ends. Picker cancellation and read/validation errors are shown in the dialog. A result is ignored if its dialog has closed or a newer image request replaced it.

## Screenshot comparison

The kit has no original full-workspace screenshot. A clean extraction was served over local HTTP and rendered in isolated Chromium at 1280×800, DPR 1, dark scheme. The valid Home capture and evidence details are in `docs/design-harvest/kikito-workspace/08-playwright-evidence.md`; it is a render of the prototype, not proof of native app parity. The implementation screenshot and comparison are still pending. Earlier startup notes in this file describe a different capture attempt and do not supersede the current evidence report.
