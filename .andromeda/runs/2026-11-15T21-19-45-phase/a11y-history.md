@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the tui wheel boundary: a closed non-editing set, and the wheel probed by exit 13 for the driver and exit 10 for the human. Clearing turn-running on an API-error end must not blur `not-delivered` (13) and `human-typing` (10, contract/cli-exit-codes/10): the wheel's holder and its turn state stay separate.

@history_item n=2 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's 8.5 s bound falls on the driver only, and a human key during the gate wait wins (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records, never a timer). This is the guard of decision/human-key-takes-the-wheel and decision/boundary-tests-sequenced-on-records. The "next send accepted" after an API-error end still passes this gate, and a new test must be sequenced on log records too.

@history_item n=3 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live takeover reading, supplemental and never gating. It is the precedent behind decision/live-readings-never-gate, which applies to any live `claude` reading this chunk asks for.

@history_item n=4 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the second live takeover reading on claude 2.1.296: `wheel{human}`, then send exit 10, then `release` back to the driver. Live readings stay never-gating. This is the same host and CLI line where the API-error defect was measured, and it is how a live reading is recorded without gating.

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a start made by a passed `viola revive` enters the same TUI passthrough boundary (decision/single-vendor-content-boundary, decision/human-keystrokes-never-blocked). The API-error turn-end path covers a revived start too.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the STATUS words `idle` / `busy` are printed words with no SGR. This is the basis of decision/state-carried-by-printed-word. After the turn end the board must read `idle` by its word, with no new colour-only state.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap, `WAIT_CAP` of 50 s, falls on the driver only. A `wait` on the turn end must now return on the API-error end, not run out to its cap or the caller's timeout (contract/viola-wait/timed-out: true only for a caller-passed timeout).

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows (10 s fallback, 40 s compact) are driver-only bounds, listed in decision/time-bounds-fall-on-the-driver. The next send after the API-error end runs under them unchanged, and no new bound may fall on the human.

@history_item n=9 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the hold's end appends one `dialog-expired` line that wakes `wait`. This is a precedent for a new event line that ends a wait, and the API-error `turn-ended` path should follow the same form. The hold bounds the driver, never the human.

@history_item n=10 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the plugin against an impostor wrapper) a hook event or dialog call that meets a server other than the recorded wrapper sends no frame and fails open: exit 0, empty streams, under 1.0 s. That is contract/hook-call/server-verify-failed. A newly registered StopFailure-class hook call must go through the same shared `connect` and fail open the same way.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the plugin against a strict-modes-refused instance) a refused hook call exits 0 at once: no `config.json` read, no stdin read, no frame, no line. That is contract/hook-call/strict-modes-failed. The new hook arm's payload read must sit behind this check, and it stays not-assertable (decision/no-assertions-for-plugin-and-ipc).

@history_item n=12 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — after a contained feed panic the driver's `send` is refused `not-delivered` (exit 13) while a human key still reaches the child, and no viola literal reaches the outer terminal. This is a precedent for an exit-13 driver refusal that leaves the human untouched. The API-error defect is an unwanted exit-13 `turn-running` refusal.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the closest precedent for this chunk's closed cause set. A peer's value is screened by `viola_core::is_code_word` (contract/code-word/format), and anything outside it reads a fixed fallback (`unknown`, or `wrapper fault` for a fault). `unknown`'s detail is a code word or nothing. The `--json` failure documents are clean (decision/json-failure-documents-are-clean, contract/cli-json-failure/document). The census of error writers and `thiserror_messages_interpolate_nothing` stand (decision/error-bodies-have-writer-and-witness, decision/thiserror-messages-interpolate-nothing). Mapping the CLI `error` string to a closed cause, with a fixed fallback member and no error text in any line, extends this register.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the closed cause words in exit 21's `detail` (and MCP's `isError` body), with `endpoint-lost` and `instance-dead` told apart. It also added the exit-cause table `tests/cross_exit_causes.rs` (fake-agent) with `no_two_codes_share_a_hint`. This is the model of a closed viola-owned cause set with every member named, and a new cause word must not collide with these or reach the human as free text (decision/error-word-detail-hint).

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (hook and MCP on a refused socket directory) a hook call fails open with its one record, a `hook-decision` at WARN with `detail` `socket-dir-refused`. That is contract/hook-call/socket-dir-refused. A new hook registration inherits this fail-open behaviour and record form.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the tui boundary is on openpty alone, and each of the three boundary clauses is its own nextest case on `ubuntu-latest` inside the `coverage` suite. That is where this chunk's a11y-relevant cases run (decision/boundary-tests-sequenced-on-records).

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the cli checks moved to `std::process::Command` with literal pins (the guard of decision/c0-c1-escaped, decision/linear-static-output, decision/send-refused-while-human-typing). The typed exit codes 10–14, 20 and 21 are pinned by literal asserts, and the outer-PTY driver is portable-pty `=0.9.0` (decision/portable-pty-outer-pty-driver). Any new test of the accepted next send or the refusal codes follows this form.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, and every picked row was verified against the code at that point. It is the baseline the handles in the rows file are read against.

@history_item n=19 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures, and the prompt row only for its opening char. If this chunk ends the turn from the API-error text, it must come from the hook payload or the transcript, never from screen text. The text itself never reaches a line.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11` (retired, never reused; an older wrapper's `budget-paused` reads `unknown` 14), plus `~ decision/time-bounds-fall-on-the-driver`, `~ decision/time-bounds-never-on-human` and `~ decision/state-carried-by-printed-word` (`budget-paused` dropped). This is a recent precedent for retiring a code without reuse and for keeping a closed set's fallback stable across mixed binary versions.

@history_item n=21 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` (the statusline deadline dropped). `− contract/viola-hook-statusline/exit`: the word is now an unknown hook event, its exit standing in architecture's hook-to-kind map. Registering a new hook event in `plugin/hooks/hooks.json` moves a word from unknown to known in that same map.
