@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (the tui wheel boundary) widened the non-editing set from "focus, mouse and resize" to F-W2's closed list (focus reports, mouse reports, terminal replies), every other byte editing; boundary case (3) landed step-wise with the resize behind its size receipt, then the focus reports, the wheel probed by an `answer`. This is the standing proof that a resize moves no wheel (rows `contract/tui-passthrough/focus-mouse-resize-sequences`, `decision/wheel-ignores-terminal-events`, `decision/non-editing-sequences`); any settle wait or re-type after a resize must keep it, and a new resize case must be sequenced on the size receipt, never a timer.

@history_item n=2 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's `GATE_MAX_WAIT` (8.5 s) falls on the driver only; a key pressed during the wait takes the wheel and the waiting send is refused `human-typing`, held by `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records, never a timer. The direct precedent for the «settle wait after a resize» option: any new wait on `send`'s path must fall on the driver, let a human key win, and be listed in the time-bound rows (`decision/human-key-takes-the-wheel`, `decision/time-bounds-fall-on-the-driver`, `contract/send-refusal/input-not-ready`).

@history_item n=3 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — extended the closed non-editing list with seven more `CSI` replies by exact grammar, among them the window-size reports `4;n;n t`, `6;n;n t`, `8;n;n t` and the in-band resize `48;n;n;n;n t`, one-field neighbours staying typing; recorded the first live takeover reading, supplemental and never gating. Bears on what a resize puts on stdin (a size reply is non-editing, a near neighbour is typing: `decision/stdin-classifier-non-editing-list`) and on how a live measurement of a lost key is ranked (`decision/live-readings-never-gate`).

@history_item n=4 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading: a key the founder typed on a live `claude` 2.1.296 session wrote `wheel{human, human-input}`, the next `send` exited 10. Precedent for a live key reading on the real CLI under the founder's word: supplemental, never gating; any rig key typed into a real CLI for this chunk's measurement is the operator's word, asked at P3.

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough; the keystroke clause and the focus / mouse / resize clause bind it but have no revive-specific case. A resize fix in the shared start / pump reaches revived starts too (`contract/tui-passthrough/entry`, `decision/human-keystrokes-never-blocked`).

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the first time bound written as falling on the driver, never the human ("viola imposes no limit on a human" re-cast). The founding precedent of the time-bound rows any resize settle wait would join.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 s) added as a driver-only bound, no key blocked and the wheel untouched; one of the members of the time-bound lists a new resize wait would extend.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the 5 s `claude agents --json` read entered §8 as a bound on viola's own child, never on a keystroke; the «agents read» member of the time-bound rows this chunk may extend.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows (`CONFIRM_WINDOW_FALLBACK` 10 s, `/compact` 40 s) recorded as driver-only bounds, unconfirmed → `not-delivered` / `no-prompt-submitted`. A «read-back and one re-type» fix sits on this confirm path (`contract/send-refusal/no-prompt-submitted`); a re-type must keep within these windows and touch no human key.

@history_item n=10 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold made configurable (`dialog_hold_s` 60–3600), any human key handing a held dialog back at once; no human-facing time limit added. The rows `decision/dialog-hold-bounds-the-driver` and `contract/config-json/dialog-hold-s` stand from it; the scope's measurement types into the empty box, never a dialog.

@history_item n=11 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` provisional → standing, after the ceiling was measured live. Precedent: a provisional bound leaves its mark only on a live reading.

@history_item n=12 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the boundary after a contained vt100 feed panic (`chaos_feed_panic_refuses_send_and_keeps_passthrough`): `send` refused `input-not-ready` while a typed human key still reaches the child, no viola literal on the outer terminal. Rows `contract/tui-passthrough/vt100-feed-panic` and `contract/tui-passthrough/after-feed-panic` stand from it; a resize re-sizes the vt100 parser the readiness read rests on, and any change there must keep this case green.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (revive --json documents every refusal) a `pump_child` error prints no document, the child having held the terminal, and viola writes nothing past it (`contract/viola-revive-json/pump-child-error`, `contract/tui-passthrough/pump-child-error`); (the exit-cause table and one hint per code) `no_two_codes_share_a_hint` and `no_hint_names_release` built (`decision/one-hint-per-code`, `decision/no-hint-names-release`). Bears on the pump's write path a resize fix would touch, and on any new refusal word the «refusal» option would add.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `--json` one-document rule gained its exit-2 exception for `answer` / `send`; `unknown`'s detail became a code word or nothing; a word outside the closed refusal list stays visible, exit 14; the census of error writers and `thiserror_messages_interpolate_nothing` built. The rows `contract/cli-refusal/json`, `decision/json-one-document-no-hint`, `decision/unlisted-code-words-stay-visible`, `decision/error-bodies-have-writer-and-witness`, `decision/thiserror-messages-interpolate-nothing` stand from it; a new `send` refusal detail for a key lost after a resize must enter the closed list, the census and its literal pins.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the tui boundary and the wrapper's drop of a refused peer) a refused peer is dropped before its worker thread and any frame, writing nothing to the outer terminal and moving no wheel; the source of `contract/tui-passthrough/refused-peer`, one of the boundary clauses a pump / wheel change must keep.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the tui boundary on openpty alone) the TUI is hosted in openpty only, F-W3's ConPTY arm retired; the focus / mouse / resize case asserts no `wheel` record and bytes unchanged, the zero-viola-bytes case compares byte for byte with an unwrapped fake-agent run, each boundary clause its own nextest case on `ubuntu-latest`; (the a11y cases' CI home) the cli / tui cases run in the single `coverage` suite on `ubuntu-latest`; (the Linux-only platform line) `decision/linux-only`. The resize path to the child's PTY is openpty's alone, and any new resize case lands in that suite.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the outer-PTY driver) the tui boundary driver moved to portable-pty `=0.9.0`, inherited and never re-pinned by a11y (`decision/tui-boundary-driver-pin`, `decision/portable-pty-outer-pty-driver`); (cli checks and §8 verification) the cli checks named `std::process::Command` with literal pins, every fixed CLI string and each refusal's `unable … hint:` layout pinned by literal (`decision/refusal-layout-literal-pins`, `decision/cli-a11y-by-literal-tests`). The outer-PTY resize a new case would send goes through this driver; a new refusal string takes a literal pin.

@history_item n=18 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli output-discipline and TUI boundary cases run as nextest cases inside the `coverage` suite, gated by `gate --require coverage,doctest`, with no a11y driver or stage of their own, `#[ignore]` banned; the source of `decision/a11y-cases-inside-coverage-suite` and the zero-flakiness parallel (`decision/zero-flakiness`) that a timing-sensitive resize case must meet without retries.

@history_item n=19 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) meet mutation only at the epoch-boundary audit, not per chunk; consistent with the scope's «no mutation run» (`decision/cli-invariants-pass-in-ci`).

@history_item n=20 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the screen is parsed for the input box's rule and prompt rows, the prompt row only for its opening char. What tells a lost key from a kept one after a resize must stay within these compiled signatures or the rig's closed words, never text.

@history_item n=21 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its except: the test-only standing live rig reads the trust selector's place, the input box as empty / holds-text / absent and modal dialogs as closed words, raw frames kept local. The rig (`crates/viola-e2e`) is where a measuring arm for a key after a resize would be added, under this exception.

@history_item n=22 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — `~ decision/no-screen-content-parsing` except: the rig also reads the box row's cell attributes after a turn end, as closed words, never text. Each new rig reading widened the except row by name; a resize arm reading anything new must do the same.

@history_item n=23 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ decision/no-screen-content-parsing` except: box-row attributes read after a turn end or send; this chunk's founder answer «2 клавиши: x, DEL» is the rig's x / DEL rehearsal behind carry `cry-234cbfoa` (`rig_rehearsal_reads_the_box_empty_after_x_and_del`), the closest precedent for reading a typed key as kept or lost by the box's closed word.

@history_item n=24 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `~ decision/no-screen-content-parsing` except gained box-row cell attributes before any send (the HB arm, a fresh box); the row's current wording, which a resize measuring arm on the empty box must fit or widen.

@history_item n=25 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `~ decision/no-screen-content-parsing` rule: the readiness predicate reads the compiled shapes over the compiled literals (the box's rows below a modal label). A redraw after a resize re-lays these rows; the readiness gate's reading of them is what a settle wait or read-back would rest on.

@history_item n=26 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/tui-passthrough/human-keystrokes` and `~ contract/tui-passthrough/viola-terminal-bytes`: both promises hold once the wrapped child holds the terminal, a start's automatic verify window the one exception; `~ decision/human-keystrokes-never-blocked` gained that except. The current wording of the rows a resize fix must keep: no new exception to «never blocked or delayed past the current atomic paste» may be added by a settle wait (PROVISIONAL, delegate overseer).

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11` retired and never reused (an old `budget-paused` reads `unknown`, 14); `~ decision/time-bounds-fall-on-the-driver` dropped the budget pause. A new refusal for this chunk cannot take exit 11; the time-bound list is edited by name when a bound comes or goes.

@history_item n=28 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human`, `~ decision/time-bounds-fall-on-the-driver` dropped the status-line deadline; `− decision/statusline-output-passed-byte-for-byte`. The time-bound rows' list is exact: a resize settle wait would be named in both rows.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` gained the start's Claude config lock wait, falling on viola's own start; `~ decision/run-diagnostics-off-terminal` gained the `trust-record` line in `diagnostics/`, never a refusal, exit or hint. Precedent for a new bound on the start path and for where a resize's diagnostics go (`contract/tui-passthrough/run-diagnostics`).

@history_item n=30 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` gained the MCP call deadline; `+ contract/instance-unreachable-detail/call-deadline`. The last chunk's pattern for a new bound: both time-bound rows named in the same wrap; any resize settle wait follows it.
