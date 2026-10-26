# Task — design-system · 2026-10-25-windows-endpoint-admission

## The session
Added five Windows-only integration tests in tests/security_negatives_channel.rs: the viola pipe listener rejects remote clients (a pipe-type read-back with accept-remote and reject-remote controls), and an anonymous-token open of the listener is denied with ERROR_ACCESS_DENIED while the same open of a default-descriptor pipe succeeds. All five were asserted green on CI's test (windows-2025) leg (ci#40247500584).
Added a test-only helper test_support::with_scoped_default in src/main.rs, plus a witness test. It keeps a second dispatcher registered so a scoped tracing capture is never the only one. Nine root-bin scoped captures now go through it instead of tracing::subscriber::with_default.
Added a root Windows dev-dependency table: windows-sys (pinned =0.61.2) with the features Wdk_Storage_FileSystem and Win32_System_IO. Cargo.lock and the release graph are unchanged.
Every edit is test code or a dev-dependency table. No product file changed, and no human output, refusal text, hint line, exit code, SGR byte, stream assignment or --json document moved. The report's acceptance row for design · layouts · a11y reads met.
Retired nothing. Measured as false the test-plan's claim that remote pipe clients and the Windows pipe DACL cannot be driven on hosted runners.

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs

## Touches
- Surface: cli toolkit, the windows-sys 0.61.2 pin behind Windows VT enabling (SetConsoleMode). Read again, unchanged: the version did not move. The new feature table is a dev-dependency on a test target only and reaches no release build, so the toolkit row stays true.
- Surface: cli toolkit, the claim that the styling module adds no dependency. Read again, unchanged: the only dependency change is a test-only dev-dependency table, and the SGR module did not change.
- Colour decision order, item 2 (viola run prints nothing while the child runs). src/cmd/run.rs changed only inside its test module (two scoped captures re-pointed), so run's silent passthrough and its start-refusal lines did not move.
- Colour decision order, item 3, and Platform-Specific Notes (viola hook writes nothing to stderr and always exits 0). src/cmd/hook/statusline.rs changed only at one test capture line, so the statusline arm's output contract did not move.
- viola send readback mirror (the [RB] / [  ] / [/ ] states, unable plus reason, hint lines). src/run/send/test_support.rs changed only at one test capture (capture_lines, case_4), so no refusal word, hint or readback state moved.
- Domain status rows and the cocked strip (DIALOG, dialog-expired, the dialog hold). src/run/dialog.rs changed only at two test captures, so no dialog status word or treatment moved.
- Fuel state and budget-paused phraseology, and the BAY line's 5H / 7D figures. src/run/budget.rs changed only at one test capture, so no budget word or reading form moved.
- Navigation Pattern, the viola --help rows and the hidden mcp verb. src/main.rs gained only a cfg(test) helper and its witness, with no clap or dispatch change, so the served-verb rows and the hidden mcp verb stand.
- The rule that stack traces never print and full detail goes only to diagnostics/. src/obs.rs and src/run/gate.rs changed only at test captures. The binary's one global subscriber install (src/run/gate.rs set_global_default) is untouched, so log routing did not move.
- Domain status guard: an access refusal never shows the token, the URL or the file path. The new Windows negatives assert pipe admission at the OS level and add no refusal text, so the guard stands.
- Surface: cli Platform list (Windows 10/11 terminals first). The Windows leg now asserts the listener's admission controls, but no terminal-facing behaviour changed.

## New facts
- (none)

## Untouched
- Color Palette (core colours, surface scale, text hierarchy, semantic colours, border progression): 0.2.0 frontend design inputs. The session renders no page and changes no colour.
- Typography, Spacing, Border Radius, Motion and Iconography: 0.2.0 frontend design inputs, plus the CLI's ASCII-only glyph rules. No glyph or token changed.
- Self-Validation Protocol, Token Test: it governs a 0.2.0 frontend's tokens, and none was added.
- Brand Identity's domain anchors and the readback box's three drawn states: no readback, post-condition or phraseology word changed.
- viola list, the k9s strip table: its columns, row order, UNWRAPPED separator, escaping, truncation and exit-21 board refusal did not change. Its sources, src/cmd/list.rs and src/human/style.rs, were not edited.
- CLI colour tokens (amber DIALOG, dim stale row, bold NAME, depth choice from COLORTERM / TERM): the SGR module was not edited.
- viola wait / viola last result lines and the message-mode escaper: neither verb's source was edited.
- viola verify's step counter and summary line, and viola plugin install's four output lines: neither source was edited, and no ledger row count moved.
- Exit-code phraseology (0, 1, 2, 10 to 14, 20, 21) and its --json documents: no exit code or document changed.
- Anti-Patterns, the universal and cli bans: no output was added that could break them.
- Design Decisions Log: the session makes no design decision.
