@history_item n=1 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the dialog hold bounds owned by 0.2.0's first entry) — made `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` each PROVISIONAL with 0.2.0's first entry as owner, under the founder's conditional that a red near-ceiling hold leaves the ceiling PROVISIONAL. The 0.1.0 done-check's live round never read the hold, because an unverified CLI replies `null` to a dialog at once. That is this chunk's matrix v2-02 and fork (4): the reading needs a ledger with `cli_verified:true`, and it settles or moves the mark on contract/config-json/dialog-hold-s and decision/dialog-hold-bounds-the-driver.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — replaced the 60 s `DIALOG_DEADLINE` with the configurable `dialog_hold_s` (60–3600 s, default 60, PROVISIONAL bounds), set under the `hooks.json` dialog `timeout` of 3615 s and the hook's bound of 3605 s. At the hold's end the wrapper writes `null` plus one `dialog-expired` line, and any human key still hands the held dialog back at once. This is the origin of decision/dialog-hold-bounds-the-driver, contract/config-json/dialog-hold-s and contract/event-line/dialog-expired. A near-3600 s reading has to fit under those 3605/3615 s outer bounds, and the hold must stay a driver-only bound.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — superseded the delegate's provisional confirm on the 2026-10-19 dialog-hold entry with the founder's own word. Only the history mark went; the bounds kept their own PROVISIONAL status, so this chunk still owes the settling reading (fork 4). This precedent shows how a founder's word discharges a mark in this file.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — first put the dialog deadline on the driver and never on the human: at expiry the dialog renders for the human in the `claude` TUI, and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). This is the root of contract/cli-exit-codes/13 and of the fail-open rule that the hold-near-ceiling reading must show still holds.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility, 6 rows; 6. Visual; 8. Cognitive, 1 row) — retired exit 11 (`~ contract/cli-exit-codes/11`, an older wrapper's `budget-paused` now reads `unknown`, exit 14). It dropped the budget pause from `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human`, removed `− decision/budget-gate`, `− decision/budget-gate-only-on-send` and `− contract/config-json/five-hour` / `seven-day`, and took `budget-paused` off the board's state words. This is half of the "trimmed product" the live round runs on. The wrapped start's documented order has no budget step. The live home's stale `~/.viola/budget.json` is the operator's desk clean-up and out of scope.

@history_item n=6 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility ×3; 3. A11y Assertion Harness Contract) — dropped the status-line deadline from `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver`. It also removed `− decision/statusline-output-passed-byte-for-byte` (the arm, the shell-out and the TUI statusline passthrough case) and `− contract/viola-hook-statusline/exit` (the word is now an unknown hook event). This is the other half of the trimmed product. CARRY 3's wrapped-vs-unwrapped status-line reading is the live half of this removal, and CARRY 2's box-only fixture became recordable because the status-line rows are retired.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — added the statusline-bearing start's own zero-viola-bytes reading (`passthrough_with_a_statusline_source_adds_no_viola_bytes`, which wrote the settings override and passed `--settings`) and the 5 s `STATUSLINE_DEADLINE`. Both are now removed. This is the history behind CARRY 3: a wrapped start now passes no `--settings`, so the live status-line reading must show the user's own setting read as unwrapped, with nothing written to `~/.claude/settings.json`.

@history_item n=8 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — added the budget pause as a driver-only bound and a closed-gate keyboard reading (`path6_a_reading_over_the_threshold_refuses_the_send`) to the keyboard-test-harness Tooling. That reading is retired with the gate, so the live wrapped start must not lean on any budget-specific boundary case.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause on Linux, supplemental and never gating: a `wtype` key into a session on a compositor the test started wrote `wheel{human, human-input}`, and the next `send` exited 10. It also widened the closed non-editing list to its exact grammars (decision/stdin-classifier-non-editing-list, decision/non-editing-sequences). This is the precedent for decision/live-readings-never-gate: v2-03's live reading of human keys passed unchanged is supplemental evidence and never a gate. This chunk runs headless on the rig's own pty with no compositor.

@history_item n=10 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — recorded the second live takeover reading, on the product binary over a live `claude` 2.1.296 session: the founder's own key wrote `wheel{holder:"human",cause:"human-input"}`, the driver `send` exited 10 `human-typing` with no `send-issued`, and `viola release` gave the wheel back. This is the nearest precedent for a live wrapped-start reading on a real CLI, and 2.1.296 is the shim version named in this chunk's fork (3).

@history_item n=11 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — set the non-editing set to the closed list of focus and mouse reports plus terminal replies, with every other byte counted as editing. It also defined the wheel case (resize, then focus reports with a mouse report as a read barrier, the wheel probed by an `answer` to no pending dialog). This explains decision/wheel-ignores-terminal-events and decision/non-editing-sequences: the live wrapped start's "human keys passed unchanged" claim reads against these clauses.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — put the readiness gate's 8.5 s bound on the driver only and added the outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records and never on a timer. This is the guard of decision/human-key-takes-the-wheel and the origin of decision/boundary-tests-sequenced-on-records. Any new rig case follows the same records-not-timers rule.

@history_item n=13 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — made a passed `viola revive` enter the TUI passthrough boundary, adding the clause that it writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). The revive exit-1 refusals take the one `unable:` line + `hint:` form. This is the source of contract/tui-passthrough/entry and contract/tui-passthrough/own-bytes. The wrapped start's zero-viola-bytes reading covers `run`, and a revive start shares that boundary.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a lost wrapper and a stale holder each read their own code; `viola revive --json` documents every refusal of its start) — split a start against a holder into `stale-heartbeat` (stale) and `already-live` (live, or a fresh replayed beat), with stale heartbeat as exit 1's cause alone. It also gave every refusal of the start revive shares with `run` one `{"v":1,"error":…,"detail":null}` document, and a `pump_child` error prints none. This explains contract/start-holder-refusal/*, contract/revive-json-refusal/* and contract/start-json-refusal/document, which back v2-03's "a live or stale name refused".

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home; the Linux-only platform line; revive's start refusals without batch-script-child; the manual screen-reader pass, Orca on Linux alone; the three-runner greens re-worded as dated readings) — moved the TUI to openpty only. The zero-viola-bytes case now compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, each boundary clause is its own nextest case on `ubuntu-latest`, and the CI gate is the single `coverage` suite there. It set Linux-only (decision/linux-only), cut revive's `--json` codes to ten, made the tui manual pass Orca + foot on Hyprland and founder-owned (decision/wrapped-tui-reads-as-unwrapped, decision/manual-keyboard-pass), and re-worded dated greens as readings. v2-03 is the Linux wrapped start, and its zero-bytes oracle is this unwrapped comparison.

@history_item n=16 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0; the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal; the cli tooling in the a11y testing contracts) — moved the tui boundary driver to portable-pty `=0.9.0`, which a11y inherits and never re-pins (decision/tui-boundary-driver-pin, decision/portable-pty-outer-pty-driver). It replaced trycmd/assert_cmd with `std::process::Command` and literal pins for every fixed CLI string, refusal layout and typed exit code (decision/fixed-cli-strings-pinned*, decision/refusal-layout-literal-pins, decision/cli-a11y-by-literal-tests). Any rig or fixture work that touches a fixed string has to update its literal on purpose.

@history_item n=17 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (11. A11y Anti-Patterns, 1 row) — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening character. This was ratified as a boundary widening and is PROVISIONAL on the delegate's card confirm. CARRY 2's box-only `Screen.*` fixture (rule and prompt rows, no footer literal) is exactly this signature set: its `--record` scrub and replay must not read past it.

@history_item n=18 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the TUI passthrough entity after a contained vt100 feed panic (`chaos_feed_panic_refuses_send_and_keeps_passthrough`): `send` is refused `input-not-ready` while a human key still reaches the child, and no viola literal reaches the outer terminal. This is a boundary clause of contract/tui-passthrough/vendor-content (viola parses through a vt100 feed) that a live screen fed into the rig must leave intact.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — clamped the MCP `wait` tool's `timeout_ms` to `WAIT_CAP` 50 000 ms as a driver-only bound (no key blocked, wheel untouched). This is one of the bounds in decision/time-bounds-fall-on-the-driver. The MCP server is a not-assertable plugin channel (decision/no-assertions-for-plugin-and-ipc), so CARRY 1's `Mcp.tools-list-1.json` re-record through `viola-mcp-tap` changes a fixture and carries no a11y assertion.

@history_item n=20 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows (`CONFIRM_WINDOW_FALLBACK` 10 s, `COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL) as driver-only bounds to the time-bounds list. The hour-long hold keeps a window idle past the CLI's ≈55-minute compact, which is a nearby driver-side bound the live hold's journal must account for.

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the 5 s `claude agents --json` read (fail open, the bound on viola's child) that `list` and `revive` make. This is the "agents read" entry of decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver, and the revive preflight a wrapped live start may cross.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — gave a stopped `viola verify` no step line and no summary, only the one `unable:` / `hint:` pair with exit 1. This is one of the fixed-message exceptions in contract/cli-refusal/unable-line beside contract/viola-verify/step-line. It bears on rigs that run `viola verify` / `verify --record` with a kill deadline.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities; the strict-modes refusals' text) — `run`, a revive start and `verify` over a home tree that fails the strict-modes check (the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) refuse before any log with exit 1 and start no child. Live sessions use fresh test homes on tmpfs (`target/e2e-home/viola-live-*`), so their modes must pass this check before the pinned copy runs through its freshly written plugin.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start; the tui boundary and the wrapper's drop of a refused peer) — `run` and a revive start refuse a socket directory that fails its check before any snapshot, bind or child (one `unable:` line + `hint:`, the fixed-message form in contract/cli-refusal/unable-line). It also added the TUI boundary clause that a refused peer is dropped with nothing written to the outer terminal. Both are part of the documented start order the live wrapped start runs through.

@history_item n=25 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing; `--json`'s one document and its exit-2 exception) — made `unknown` (exit 14) carry a code word or nothing and print no hint. That is the exception in decision/refusal-ends-with-hint, and the exit an older wrapper's `budget-paused` now maps to. It also set the `--json` one-document rule behind contract/start-json-refusal/document.

@history_item n=26 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — made the cli / tui cases nextest cases in the `test` job's `coverage` suite under `gate --require coverage,doctest`, with failure on `suite-failed`, `suite-skipped`, `suite-missing` or `artifact-missing`, and `#[ignore]` banned. This is the origin of decision/a11y-gate-breach-fails-build, decision/gate-requires-coverage-doctest, decision/ci-test-gate-no-skips and decision/zero-flakiness. Live operator steps cannot sit in the gated suite as skipped or ignored cases: they need their own `leg = 'operator'` home.

@history_item n=27 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation coverage of the CLI output-discipline invariants to the epoch-boundary audit after the per-chunk mutation gate was removed. This is consistent with the scope's "no mutation run" boundary, where a guard's remove-the-guard control goes through `gate.py control` instead.

@history_item n=28 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour decision (reading `NO_COLOR`, `TERM`, `COLORTERM` and the stdout terminal state, with no SGR under `--json` or a passthrough role) and its outer-PTY checks (`cross_list_rows_*`). This is the source of decision/no-sgr-off-terminal and decision/no-sgr-without-tty: rigs reading viola output beside a live child must see no viola SGR under `viola run` or a passed revive.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — retired the frontend stubs to N/A (no web or graphical frontend, the terminal fleet console's a11y scope at «Fleet a11y verification»). This is the basis of the absences and N/A overviews in the rows file: the live round adds no rendered page and no browser tooling.

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows (decision · contract · absence), each verified against the code. Every handle in the rows file dates from this conversion, and later amendments edit those rows by handle.
