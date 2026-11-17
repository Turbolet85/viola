@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (11. A11y Anti-Patterns) `~ decision/no-screen-content-parsing` gained its except: the test-only standing live rig may read the trust selector's place, the input box as empty / holds-text / absent, and modal dialogs, as closed words only, its raw frames local and gitignored. This chunk's readings (R2's `box-text: holds-text`, the external-imports modal, the wake and verify readings) all sit inside that exception, so each must stay closed words in evidence; reading anything wider off the vendor screen (e.g. the text after the prompt char) would need a new amendment.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char; a ratified boundary widening (card confirm by delegate, PROVISIONAL). The base rule the rig exception extends; the R2 holds-text investigation must not read the prompt row past its opening char outside the rig exception.

@history_item n=3 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's `GATE_MAX_WAIT` (8.5 s) falls on the driver only; a human key during the wait goes to the child, takes the wheel, and the waiting send is refused `human-typing`; held by `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on the `channel-request` line and `wheel` record, never a timer, with a stated limit (cannot tell send's first wheel read from its second). Rows `decision/human-key-takes-the-wheel` and `decision/boundary-tests-sequenced-on-records` rest on it; R3c's hook-latency window (a turn the CLI starts before its `UserPromptSubmit` reaches the wrapper ahead of a send's paste) measures the residual this case and the flight lock do not close.

@history_item n=4 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause on Linux, "supplemental and never gating" (a `wtype` key wrote `wheel{human, human-input}`, focus wrote none, next driver send exited 10), and widened the non-editing terminal-reply list by exact grammar. Precedent for `decision/live-readings-never-gate`: this chunk's live rig readings are recorded the same way, as supplemental evidence, never a CI gate.

@history_item n=5 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading on the product binary over live `claude` 2.1.296, the key typed by the founder himself (`wheel{holder:"human",cause:"human-input"}`, send exit 10 `human-typing`, `viola release` back to driver). Precedent that any key into a live CLI session is the founder's to give, matching this chunk's boundary that keys into CLI-native modals (the external-imports modal) are the founder's, and that each live reading names its CLI version.

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the closed non-editing set (focus reports, mouse reports, terminal replies DA1/DA2/CPR/DECRPM/kitty/OSC/DCS), every other byte editing, and the step-wise focus/mouse/resize case probing the wheel; founder rulings F-W2/F-W3 ("the error only ever favouring the human"). Underlies `contract/tui-passthrough/focus-mouse-resize-sequences` and `decision/wheel-ignores-terminal-events`; a live wake or modal reading must not count terminal replies as human editing.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the tui boundary on openpty alone; the a11y cases' CI home) the TUI is hosted in openpty only, the zero-viola-bytes case compares byte for byte with an unwrapped fake-agent run, each of the three boundary clauses is its own nextest case on `ubuntu-latest`, and the cli / tui cases run inside the `test` job's single `coverage` suite gated by `gate --require coverage,doctest`. Bears on `decision/zero-viola-bytes-on-the-terminal`, `decision/boundary-tests-sequenced-on-records`, `decision/cli-invariants-pass-in-ci` and the nextest kill-line carry: the `ci` profile's 20 s kill override acts on these same coverage-suite cases.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (strict-modes refusals, the Windows ACL and NotFound arms retired) a home under a regular file is a strict-modes refusal with its hint (`verify` the home-state pair, exit 1), `verify_under_a_regular_file_refuses_the_home`'s oracle the home-state pair alone; strict-modes cases on ubuntu-latest. Rests under `decision/unstattable-home-is-strict-modes-refusal` and `contract/start-refusal/home-strict-modes`, which R4's live `viola verify --record` home (from a `record_home()`-like helper) must pass.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the three-runner greens re-worded as dated readings) a dated measurement is re-worded by its owner, never left as current, keyboard-test-harness's readings now "green in CI" with their chunks and runs kept. Precedent for how this chunk's live readings (each naming its `claude` binary, 2.1.325 vs 2.1.326) are written: dated, attributed, not as standing truth.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Linux-only platform line) Linux only, CI on `ubuntu-latest`, diag-line `os` reads `linux`. Frames the rig's host and the log identity rows (`contract/log-identity/process`, `contract/log-identity/service-name`) the live readings' records carry.

@history_item n=11 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the browser a11y leg left; the cli output-discipline and TUI boundary cases run as nextest cases in the `test` job's `coverage` suite, no stage, budget or second driver of their own, results in `suites[].failures[]`, `#[ignore]` banned. Source of `decision/no-a11y-performance-budget` and `decision/cli-invariants-pass-in-ci`; the kill-line fix must stay a nextest profile setting inside that suite, adding no a11y stage or budget, and must not end a case by the kill line.

@history_item n=12 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline evidence reports in CI inside the single instrumented `coverage` suite, gated by `gate --require coverage,doctest`; failures surface only as nextest failures. Origin of the coverage-suite home the nextest `ci` kill override governs.

@history_item n=13 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) meet mutation only at the epoch-boundary audit (`run --mutants`), the per-chunk and CI mutation gates removed. Consistent with this chunk's "No mutation run" boundary.

@history_item n=14 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause from their lists; `~ contract/cli-exit-codes/11` retired. The two picked time-bound rows as they now stand; any bound this chunk's readings or the kill-line fix add or measure (the hook-latency window, a shortened boot readiness deadline in a test) must not land on the human or move the wheel.

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human`, `~ decision/time-bounds-fall-on-the-driver` dropped the status-line deadline; `− decision/statusline-output-passed-byte-for-byte`; `− contract/viola-hook-statusline/exit`. Gives the current text of both picked time-bound rows (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read).

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a confirmed send waits `CONFIRM_WINDOW_FALLBACK` (10 s), a `/compact` on a verified CLI `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), else `not-delivered` / `no-prompt-submitted`; the bound on the driver only. Source of `contract/send-refusal/no-prompt-submitted`; the rig's sends (R3b's two sends, R3c's latency window) read against these windows.

@history_item n=17 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold (`dialog_hold_s` 60–3600, default 60) bounds the driver, never the human; any human key hands the held dialog back at once. One of the time bounds in the picked rows; the external-imports modal reading must keep the human's keys unblocked whatever the start reads.

@history_item n=18 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` state provisional → standing, after the ceiling was measured live and the dialog read on the screen after expiry. Precedent that a live reading can close a provisional a11y row, as this chunk's external-imports modal reading is to close v2-32's provisional narrowing (the founder's word still owed).

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the cli checks re-worded to `std::process::Command` with literal pins (`decision/cli-a11y-by-literal-tests`), P4's `human-typing` refusal checked through `std::process::Command`, and the tui driver on portable-pty `=0.9.0` (`decision/portable-pty-outer-pty-driver`), inherited, never re-pinned by a11y. Constrains any rig or harness test this chunk adds or touches (`harness_lifecycle`, `cli`, `harness_tied` binaries): literal pins over `std::process::Command`, no re-pin of portable-pty.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the TUI passthrough boundary after a contained vt100 feed panic (`chaos_feed_panic_refuses_send_and_keeps_passthrough`): send refused `not-delivered` / `input-not-ready` (exit 13), a typed human key still reaches the child, no viola literal reaches the outer terminal, the child's own bytes do. Source of `contract/tui-passthrough/after-feed-panic` and `contract/tui-passthrough/vt100-feed-panic`.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, writing no line of its own while the child holds the terminal; the no-SGR list and keystroke ban cover a start a passed revive made. Source of `contract/tui-passthrough/own-bytes` and the revive clauses in `decision/single-vendor-content-boundary`, `decision/human-keystrokes-never-blocked` and the no-SGR rows.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM prints no step line and no summary, only the fixed `unable:` / `hint:` pair, exit 1; the stop path adds no time bound. Bears on `contract/viola-verify/step-line` and the CLI verify output critical path R4's live `viola verify --record` exercises, including a rig exit path that stops verify.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `viola run`, a `viola revive` start and `viola verify` over a home tree failing strict modes refuse before any log opens (exit 1, stdout empty, home-state pair with `hint:` last, no child; verify no step line or summary, `update_stamps` the same pair); an unstattable home is a strict-modes refusal. Source of `contract/start-refusal/home-strict-modes`, `decision/strict-modes-refusal-before-any-write` and `decision/unstattable-home-is-strict-modes-refusal`: R4's live record home must pass this check (scratch root modes) or verify refuses before its first step.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's refusal screened to a code word) `reply_of` / `of_reply` screen a peer's reply with `is_code_word` (1–64 bytes `[a-z0-9-]`, a lower-case letter first); a non-code-word refusal reads `unknown`, a non-code-word detail reads nothing. Source of `decision/peer-replies-screened-to-code-words` and `contract/code-word/format`; closed words the readings record (e.g. a wake's `source` value `schedule_wakeup` / `loop_wakeup`, box-text words) fit that format.

@history_item n=25 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `list` and `revive` each make one `claude agents --json` read under a 5 s kill, fail open; the bound falls on viola's own child, never the human. The "agents read" entry in both picked time-bound rows.

@history_item n=26 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` 50 000 ms, a driver-only bound. The "MCP wait cap" entry in `decision/time-bounds-fall-on-the-driver`.

@history_item n=27 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer: the SGR decision reads `NO_COLOR`, `TERM`, `COLORTERM` and stdout's terminal state, none under `--json` or a passthrough role; checked under the outer PTY by `cross_list_rows_*`. Source of the no-SGR rows (`decision/no-sgr-off-terminal` with its `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json` guard, `decision/no-sgr-in-plain-modes`, `decision/no-sgr-without-tty`) that bind the rig's `viola run` starts.

@history_item n=28 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (every area: decision 131 · contract 110 · absence 5), each verified against the code; the founder's fates on dropped and kept records. The origin of every picked row and absence in this chunk's rows file; later `~` / `−` entries amend these rows by handle.
