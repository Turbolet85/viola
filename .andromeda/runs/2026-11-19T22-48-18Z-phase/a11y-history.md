@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the tui wheel boundary: the closed non-editing list (focus, mouse, terminal replies), every other byte editing, case (3) probing the wheel by an `answer` to no pending dialog (exit 13 driver, 10 human); the founder's F-W2 / F-W3 rulings that the error only ever favours the human. This is the precedent behind decision/stdin-classifier-non-editing-list and decision/non-editing-sequences: any means that keeps a fired wake from taking the wheel must not widen what leaves the wheel alone beyond that closed list, and the conservative side when it cannot tell stays `human`.

@history_item n=2 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed non-editing list extended by exact grammar (one-field or one-prefix neighbours stay typing) and the first live takeover reading (`wtype` key wrote `wheel{human, human-input}`, the next send exited 10), supplemental and never gating. Bears on decision/stdin-classifier-non-editing-list and decision/live-readings-never-gate: a wake told apart by the pty's byte source must keep this exact-grammar rule, and any live wake reading stays supplemental.

@history_item n=3 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading: a key the founder typed wrote `wheel{holder:"human",cause:"human-input"}`, the next send exited 10 `human-typing`, `viola release` returned the wheel. The live baseline of decision/critical-path-human-takes-wheel that this chunk must keep: a human-typed prompt still takes the wheel while a fired wake does not.

@history_item n=4 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's 8.5 s bound falls on the driver; a key during the gate wait takes the wheel and the send is refused `human-typing` (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on the `wheel` record, never a timer). The guard of decision/human-key-takes-the-wheel; the overseer's "the human always wins" reading this chunk's wake-vs-typed decision must not weaken.

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the dialog deadline bounds the driver, never the human; on expiry the dialog renders for the human and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). Origin of contract/cli-exit-codes/13 and the dialog-held-for-the-driver path this chunk restores after a fired wake.

@history_item n=6 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` 60–3600 s (default 60), one `dialog-expired` line at the hold's end that wakes `wait`; any human key hands a held dialog back at once. Origin of decision/dialog-hold-bounds-the-driver, contract/config-json/dialog-hold-s and contract/event-line/dialog-expired: the next question dialog after a fired wake must be held for the driver under this rule, and a human key still hands it back.

@history_item n=7 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver state provisional → standing, the ceiling measured live and the dialog read on screen after expiry. The row this chunk relies on for "the next question dialog is held for the driver" is standing, not provisional.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — several sub-entries this chunk rests on: P4's cli `human-typing` refusal with its typed exit code checked through `std::process::Command` (guard of decision/send-refused-while-human-typing); the refusal layout, typed exit codes and hint-free `--json` held by literal pins (decision/refusal-layout-literal-pins); the outer-PTY driver on portable-pty `=0.9.0`, inherited and never re-pinned (decision/portable-pty-outer-pty-driver, decision/tui-boundary-driver-pin); the dialog hold's bounds PROVISIONAL, owned by 0.2.0's first entry.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the tui boundary on openpty alone, F-W2's closed list with no Windows arm, each boundary clause its own nextest case on `ubuntu-latest`, the zero-viola-bytes case byte for byte against an unwrapped fake-agent run; the a11y cases' CI home the one ubuntu-latest leg; the Linux-only platform line. Sets where any new wheel / wake case lands (decision/linux-only, decision/zero-viola-bytes-on-the-terminal, decision/wheel-ignores-terminal-events).

@history_item n=10 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary; the keystroke ban and no-conformance-claim ban cover a start a passed revive made, and a passed revive writes no line of its own while the child holds the terminal. A wake-origin change in `src/run/` binds the revived start too (decision/human-keystrokes-never-blocked, contract/tui-passthrough/own-bytes).

@history_item n=11 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the tui passthrough boundary after a contained vt100 feed panic: send refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child, no viola literal on the outer terminal. Origin of contract/tui-passthrough/after-feed-panic and vt100-feed-panic; a wake means reading the vt100 feed must keep this contained-panic boundary.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the tui boundary and the wrapper's drop of a refused peer (no outer-terminal byte, no wheel moved), and hook / MCP on a refused socket directory, not-assertable (fail open, exit 0, empty streams, one `hook-decision` WARN record). Origins of contract/tui-passthrough/refused-peer and contract/hook-call/socket-dir-refused: a UserPromptSubmit payload change for wake origin rides the same hook path and must keep its fail-open, no-human-byte form.

@history_item n=13 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the plugin against an impostor wrapper writes no human-facing byte: a hook event or dialog call meeting such a server sends no frame and fails open (exit 0, empty streams, under 1.0 s). Origin of contract/hook-call/server-verify-failed; the UserPromptSubmit hook this chunk may read for origin keeps it.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the plugin against a strict-modes-refused instance, not-assertable: exit 0 at once, no `config.json` read, no stdin read, no frame, no line, no SessionEnd append. Origin of contract/hook-call/strict-modes-failed; any new hook-payload parse for wake origin must sit after this check and add no read before it.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows (10 s; 40 s for `/compact` on a verified CLI), unconfirmed `not-delivered` / `no-prompt-submitted`, a bound on the driver only. Origin of contract/send-refusal/no-prompt-submitted: "the driver's next send is confirmed" after a fired wake is read against this window.

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 000 ms) as a driver-only bound, the wheel untouched. One of the bounds listed in decision/time-bounds-fall-on-the-driver; a wake that wakes a parked `wait` must keep the bound on the driver.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human drop the budget pause; ~ contract/cli-exit-codes/11 retired and never reused. The rows' lists of bounds are kept whole by named edits: a new bound this chunk adds (e.g. a wake-detection window) must be added to both rows the same way, and exit 11 may not be reused for a wake refusal.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver drop the status-line deadline. Same precedent: both time-bound rows are edited in step whenever a bound comes or goes.

@history_item n=19 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver add the start's Claude config lock wait (falling on viola's own start); ~ decision/run-diagnostics-off-terminal adds the one `trust-record` diagnostics line; ~ decision/wrapped-tui-reads-as-unwrapped gains its untrusted-folder except. The latest edits to four picked rows: a wake-origin verdict reported in diagnostics follows the trust-record form (never a refusal, exit code or hint on the terminal), and a wrapped wake must still read as unwrapped.

@history_item n=20 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the screen parsed only for compiled signatures, the prompt row only for its opening char, never the text after it. A means that tells a wake from a typed prompt by reading the screen cannot read the prompt's text.

@history_item n=21 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gains its test-only live rig except (closed words only, raw frames kept local and gitignored). The rig is where a wake would be rehearsed; its readings stay inside this except.

@history_item n=22 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — ~ decision/no-screen-content-parsing except widened to the box row's cell attributes after a turn end, as closed words, never text. The `SF` reading of the placeholder hint's cell attribute the CARRY cry-iuatjqkx names; its third reading in this chunk stays inside this except.

@history_item n=23 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — ~ decision/no-screen-content-parsing except now "after a turn end or send" the box row's cell attributes (PROVISIONAL, delegate overseer). The latest edit to the row; the hint-cell third reading (a fresh box, keys before any send, no send after) may need the except widened again ("before any send"), a boundary widening that is the founder's word.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown`'s detail a code word or nothing, so it prints no hint (the except of decision/refusal-ends-with-hint), and a peer's refusal screened to a code word. Any new refusal or reason code from the wake path must sit beside `unable` with a hint, or take `unknown`'s carve-out.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the `cli` catch site's `internal-error` document under `--json`, `run`, revive's start, `mcp` and `hook` staying silent there. Origin of contract/json-fault/internal-error; the run / hook seams this chunk touches stay silent on a fault.

@history_item n=26 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) in crate tests are covered by the epoch-boundary mutation audit. Origin of decision/mutation-audit-covers-crate-invariants; a P4 wheel invariant this chunk adds in crate tests falls under that audit.

@history_item n=27 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour decision reads `NO_COLOR`, `TERM`, `COLORTERM` and stdout's terminal state, none under `--json` or a passthrough role. Bears on decision/no-sgr-in-plain-modes / no-sgr-without-tty: nothing on the wake path under `viola run` writes SGR.

@history_item n=28 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision · contract · absence), each verified against the code; relied rows kept or cut. Every picked row's handle dates from here, and later amendments name them by `~` / `+` / `−`, the form this chunk's own a11y amendments take.
