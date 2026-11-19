@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify/step-line`: the step line went from `pass|fail` to `pass|absent|fail`, with `absent` only on the browser-tools-prompt row when no run met the prompt (delegate overseer, PROVISIONAL). This chunk adds verify's own fixed words for a run that meets the prompt. They must stay beside this row: stamp and exit unchanged, no new step word, and the row's PROVISIONAL word still open.

@history_item n=2 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (`~ decision/run-diagnostics-off-terminal`) — the start's trust step reports its word in one `trust-record` line in `diagnostics/`, never as a refusal, exit code or hint. This is the precedent for keeping a run's prompt and trust outcomes off the terminal and out of refusals. The new browser-tools-prompt notice is verify's own output, not a `run` diagnostic, so its stream has to be named against this rule.

@history_item n=3 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (`~ decision/time-bounds-never-on-human`, `~ decision/time-bounds-fall-on-the-driver`) — both time-bound rows now list the start's Claude config lock wait, which falls on viola's own start. The live verify starts under the real `~/.claude.json` go through that trust step, so any new wait (the PREREQ read, verify's prompt check) must fall on the driver, the child or viola's own start, never on a key.

@history_item n=4 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its except: the test-only standing live rig reads the trust selector, input box and modal dialogs as closed words, raw frames local and gitignored. The live 24-row verify and the `/context` and `/mcp disable` readings must stay inside this exception, or inside compiled signatures, when they read the vendor screen.

@history_item n=5 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), and the prompt row only for its opening char. Verify can detect the browser-tools prompt only by a compiled modal literal, never by reading other screen text.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair: no step line, no summary, `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, exit 1, pinned by the stop table in `tests/cli_verify.rs` (wording later ruled the founder's). The new prompt line must not disturb this pair, which the rows `contract/interrupted-verify/*` and `contract/viola-verify/interrupted-pair` hold.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `viola verify`, `run` and a revive start over a home tree failing strict modes (`ledger/`, `bin/`, `plugin/`, the pinned copy) refuse before any log with exit 1 and the home-state pair, with no step line or summary. `update_stamps`'s own refusal gives the same pair. A home under a regular file is a strict-modes refusal. The live verify writes `ledger/stamps.json` and pins under `~/.viola/bin/<key>/` in the founder's real home, so a widened tree there is a RED this chunk must name. Rows: `contract/start-refusal/home-strict-modes`, `decision/strict-modes-refusal-before-any-write`, `decision/unstattable-home-is-strict-modes-refusal`.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — retired the Windows NotFound and ACL strict-modes arms (`verify_under_a_regular_file_refuses_the_home` now has one oracle, the home-state pair). It also made `[ubuntu-latest]` the one CI leg for the cli and tui a11y cases, inside `coverage`, and made Orca on Linux the only manual screen-reader pass. Any new verify-line witness runs on that one leg. Rows: `decision/a11y-ci-github-actions`, `decision/cli-invariants-pass-in-ci`, `decision/no-a11y-tool-output-discipline-assertions`.

@history_item n=9 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the cli checks were renamed to `std::process::Command` with literal pins (trycmd and assert_cmd retired). Every fixed CLI string is pinned by literal in root and unit tests, and refusal layout, typed exit codes and hint-free `--json` are held by literal asserts. Verify's new fixed browser-tools-prompt words must take a literal pin in the same way. Rows: `decision/fixed-cli-strings-pinned*`, `decision/refusal-layout-literal-pins`, `decision/cli-a11y-by-literal-tests`.

@history_item n=10 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, on the product binary over a live claude session, with the founder's own key and supplemental, never gating. It is the precedent for switching the overseer's session onto a product binary and for live readings on the real home. This chunk's re-pin and switch is an operator step (`decision/live-readings-never-gate`).

@history_item n=11 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause as supplemental and never gating. This chunk's live 24-row verify and the `/context` and `/mcp disable` readings likewise give no a11y gate (`decision/live-readings-never-gate`).

@history_item n=12 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` moved from provisional to standing after a live measurement on the product. This is the precedent for a live product round settling a row's state. The hold bounds the driver, and any human key hands a held dialog back.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — dropped the budget pause from both time-bound rows and retired exit 11, which now reads `unknown`, exit 14, and is never reused. History of the time-bound rows this chunk rests on: exit 11 must not be revived for a new verify outcome.

@history_item n=14 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — dropped the status-line deadline from `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver`. It is part of how those rows' bound lists reached their current form.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (one hint per code; the exit-cause table) — `no_two_codes_share_a_hint`, `no_hint_names_release` and the `tests/cross_exit_causes.rs` table hold each code to its own hint. If verify's prompt line or a new refusal adds a code or hint, these guards bind it (rows `decision/one-hint-per-code`, `decision/no-hint-names-release`).

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (census of error writers; peer refusal screened to a code word; `unknown`'s detail) — each error body names a writer and a witness, and `thiserror` messages interpolate nothing. A peer's non-code-word refusal reads `unknown`, and an unlisted code word stays visible with exit 14. Any new verify error text or body must join the census. Rows: `decision/error-bodies-have-writer-and-witness`, `decision/thiserror-messages-interpolate-nothing`, `contract/code-word/format`, `decision/unlisted-code-words-stay-visible`, and the `refusal-ends-with-hint` except.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start) — `viola run` and a revive start over a failing socket directory exit 1 before any snapshot, bind or child, printing the fixed-message pair. Verify's live runs make wrapped starts, so this refusal and its form bind them. Rows: `contract/start-refusal/socket-dir`, `decision/socket-dir-refusal-before-any-frame`, `contract/cli-fixed-message/form`, `contract/cli-refusal/unable-line`.

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: SGR only beside its word, no SGR under `NO_COLOR`, `TERM=dumb`, non-TTY, `--json` or a passthrough role, and the colour decision inputs and depth. `viola wait` and `--help` carry no ESC. Verify's new line must keep `fail` and `unable` uncoloured and its words plain in plain modes. Rows: `decision/sgr-beside-its-word`, `decision/no-sgr-off-terminal`, `decision/colour-decision-inputs`, `decision/unable-fail-never-coloured`.

@history_item n=19 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, and the no-SGR list and keystroke ban cover it. This is the origin of `decision/single-vendor-content-boundary` and `decision/human-keystrokes-never-blocked` as they read now.

@history_item n=20 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli and tui cases became nextest cases in the `coverage` suite, their failures only in `suites[].failures[]`, with no a11y rows or artifact. A new verify-line case surfaces the same way. Rows: `contract/structured-violation-json/*`.

@history_item n=21 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline evidence reports inside the CI `coverage` suite. This is the origin of `decision/cli-invariants-pass-in-ci`.

@history_item n=22 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow, and the nightly workflow carries no a11y step. This is the origin of `decision/a11y-ci-github-actions`.

@history_item n=23 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the P4 and P6 invariants in crate tests meet mutation only at the epoch-boundary audit, consistent with this chunk's no-mutation-run boundary. Row: `decision/mutation-audit-covers-crate-invariants`.

@history_item n=24 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's answer kept the a11y tier at `Standard (1)`, which `decision/a11y-tier-standard` rests on.

@history_item n=25 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surfaces left 0.1.0 and the terminal surfaces stand, with the tier's justification resting on cli and tui alone. Background for the tier row and the output-discipline stand-in rows.

@history_item n=26 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census declared no web or graphical frontend and made §5–§7 and the harness keys N/A. `viola list`'s fixed captions and slot order are the critical path. This is the source of the plan's absences and of `decision/critical-path-cli-board-output`.

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code. This is the origin of every picked row's current wording and handle.
