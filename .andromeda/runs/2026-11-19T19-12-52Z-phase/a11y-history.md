@history_item n=1 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows to §8 Timeout extensions: a confirmed `viola send` waits up to `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI up to its own 40 s window. If not confirmed in time it fails `not-delivered` / `no-prompt-submitted`, and the bound falls on the driver only. This is the closest precedent for this chunk: a post-condition ledger row for a local command. Any `/context` confirm window joins the bounds that rows contract/send-refusal/no-prompt-submitted, decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver list, so a new or reused window must be said to fall on the driver and never on a key.

@history_item n=2 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — a verified send waits in the readiness gate (at most 8.5 s), then is refused `input-not-ready` with nothing typed. A human key during that wait takes the wheel, and the send is refused `human-typing`. The case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is sequenced on records, never on a timer. The `/context` paste goes through this same gate (rows contract/send-refusal/input-not-ready, decision/human-key-takes-the-wheel, decision/boundary-tests-sequenced-on-records), so the human must still win during any `/context` send.

@history_item n=3 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/viola-verify/step-line: the line went from pass|fail to pass|absent|fail, with absent printed only on the browser-tools-prompt row. It is PROVISIONAL (delegate overseer). This is the latest precedent for a new ledger row with a `viola verify` probe. The `/context` probe adds a step line, and the absent word must stay scoped to that one row unless the plan amends it.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair: no step line, no summary, `unable: verify was stopped before it finished` and then `hint: run viola verify again to stamp this CLI`, exit 1. The stop path adds no time bound. A new interactive verify probe (the `/context` paste) must keep this behaviour on Ctrl-C or TERM (rows contract/interrupted-verify/exit, contract/interrupted-verify/stderr, contract/viola-verify/interrupted-pair).

@history_item n=5 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — ~ decision/no-screen-content-parsing except: the test-only live rig reads, as closed words and never as text, the trust selector's place, the box (empty, holds-text or absent), modals and, after a turn end, the box row's cell attributes. Carry cry-kutofxml is folded into this chunk: the next reading of the placeholder hint covers every drawn cell past the cursor, attributes only. That reading has to stay inside this exception or amend it.

@history_item n=6 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained its first test-only exception: the live rig reads closed words only, and its raw frames stay local and gitignored. This is where the exception comes from that this chunk's live stamp and hint-cell reading rely on.

@history_item n=7 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing rule: the screen is parsed only for compiled signatures (footer literals, the box's rule and prompt rows, modal literals), and the prompt row only for its opening char. It is PROVISIONAL (delegate overseer card). `/context` must be confirmed from the transcript's stdout tag, not from screen content. Any screen signature added for it would widen this row.

@history_item n=8 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (two row entries) — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver: both lists gained the start's Claude config lock wait, which falls on viola's own start. This is the latest wording of the two time-bound rows that any new `/context` bound would extend.

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human dropped the budget pause. ~ contract/cli-exit-codes/11 retired exit 11, never to be reused. A new send outcome must not take exit 11, and the time-bound lists must carry only live bounds.

@history_item n=10 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the two time-bound rows dropped the status-line deadline, and − decision/statusline-output-passed-byte-for-byte removed that row. This is precedent that the time-bound lists follow each bound's addition or removal row by row.

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 s) is a bound on the driver only. It is part of the time-bound lists (rows decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human) that a `/context` send's confirm wait sits beside.

@history_item n=12 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the non-editing set became the closed list (focus reports, mouse reports, terminal replies), and every other byte counts as editing. Wheel probing was made step-wise. This is the basis of rows decision/non-editing-sequences, decision/wheel-ignores-terminal-events and contract/tui-passthrough/focus-mouse-resize-sequences: a pasted `/context` from the driver must not be read as human input, and a human key must still take the wheel.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — extended the closed non-editing list with seven more `CSI` replies by exact grammar. One-field neighbours stay typing (row decision/stdin-classifier-non-editing-list). It also recorded the first live takeover reading as supplemental and never gating (row decision/live-readings-never-gate). That rule applies to this chunk's live stamp and readings: they never gate CI.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the second live takeover reading: the founder's own key wrote `wheel{human}`, the next send exited 10 `human-typing`, and release returned the wheel. It is supplemental and never gating (rows decision/live-readings-never-gate, contract/cli-exit-codes/10, critical-path-human-takes-wheel).

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home) — the three boundary clauses are each a nextest case on the one `ubuntu-latest` leg. Zero-viola-bytes is compared byte for byte with an unwrapped fake-agent run, and the a11y cases run inside the `coverage` suite gated by `gate --require coverage,doctest`. The fake-agent rehearsal of `/context` and any boundary case land in this setup (rows decision/a11y-ci-github-actions, decision/zero-viola-bytes-on-the-terminal, decision/tui-boundary-assertions-only).

@history_item n=16 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli/tui cases run as nextest cases in the `coverage` suite, and results land in `suites[].failures[]`. The zero-flakiness parallel bans `#[ignore]`. Carry cry-4olcx4il (the intermittent `rig_rehearsal_reads_the_box_empty_after_x_and_del` red) must be fixed by finding the root cause, never by a retry or `#[ignore]` (rows decision/zero-flakiness, contract/structured-violation-json/suites-failures).

@history_item n=17 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline tests report under the CI `coverage` suite, and failures still surface only in `suites[].failures[]`. This is where row contract/structured-violation-json/suites-failures comes from.

@history_item n=18 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow, and the scheduled nightly carries no a11y step (row decision/a11y-ci-github-actions). New a11y-relevant cases belong in `ci.yml`.

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cli checks on std::process::Command and literal pins; §8 verification pinned by literal; portable-pty =0.9.0) — cli output discipline is checked by `std::process::Command` with each fixed string pinned by a literal. Typed exit codes and a hint-free `--json` are literal-asserted. The outer-PTY driver is portable-pty `=0.9.0`. Any new verify step-line string or send detail (`unconfirmable`) must be pinned the same way (rows decision/no-sgr-without-tty, decision/linear-static-output, decision/portable-pty-outer-pty-driver).

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; one document and its exit-2 exception; a peer's refusal screened to a code word) — each `--json` failure is one clean document with a literal exit, empty stderr and no ESC. The `answer`/`send` exit-2 paths write a fixed stderr line and no document. A refusal or detail must be a code word (1–64 bytes of `[a-z0-9-]`, starting with a lower-case letter). A new send detail such as `unconfirmable` must be a code word, and send's exit-2 exception stands (rows decision/json-failure-documents-are-clean, decision/json-one-document-no-hint, contract/cli-refusal/json, contract/code-word/format, contract/cli-exit-codes/2).

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table and one hint per code) — `no_two_codes_share_a_hint` and `no_hint_names_release` in `tests/cross_exit_causes.rs` hold the catalogue. Any new code or hint from the `/context` send path must keep its own hint (row decision/one-hint-per-code).

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' text) — `verify` (and run/revive start) refuse a failing home tree before its log opens: exit 1, no step line, no summary, no line written. A verify run that carries the new probe stays behind this check (row decision/strict-modes-refusal-before-any-write).

@history_item n=23 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — after a feed panic, `send` is refused `input-not-ready` (exit 13) while a human key still reaches the child, and no viola literal reaches the terminal. A `/context` send after a panic must take this same refusal (rows contract/tui-passthrough/vt100-feed-panic, contract/tui-passthrough/after-feed-panic).

@history_item n=24 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary and writes no line of its own while the child holds the terminal. This is where rows contract/tui-passthrough/own-bytes, decision/single-vendor-content-boundary and decision/human-keystrokes-never-blocked get their revive arm, which a `/context` send into a revived start also crosses.

@history_item n=25 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code. Every picked row (handle, rule, guard, why/except) comes from this conversion, and later edits name rows by handle (`~`/`+`/`−`). This chunk's amendments must use that form.
