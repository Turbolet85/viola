@history_item n=1 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (Established Decisions, Conventions, the two later rows) — added `decision/mcp-call-deadline`: a compiled 55 s deadline above the 50 s wait cap, with expiry `call-deadline`. It grew `contract/unreachable-cause/cause-set` to seven causes, `call-deadline` written by MCP alone, and added `call-deadline` to `decision/mcp-unreachable-result-for-every-cause` as a cause found after a frame is sent. It also rewrote `decision/send-time-bound`: «on the MCP surface the MCP call deadline bounds the driver's wait, never the send itself». It names the false clause directly: the old total-under-client-timeout claim «is false while the paste write has no time bound … the paste's gap is the route's CARRY». This chunk discharges that CARRY. The paste bound has to fit the arithmetic against `MCP_CALL_DEADLINE` that these rows set up.

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's 40 s window) — added `COMPACT_CONFIRM_WINDOW` = 40 s (`decision/compact-confirm-window`) beside the 10 s window, `SendSlot::window()` being the one bound. It stated that a compact send's blocking total is 48.5 s plus the paste, under an MCP ceiling read at 50 012 ms. This is the longest send total that the paste bound has to be added to while staying below the 55 s deadline, and «plus the paste» is the unbounded term.

@history_item n=3 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — raised `GATE_MAX_WAIT` to 8.5 s (`decision/gate-timings-compiled`, `decision/send-waits-out-paste-hint`) and set send's longest block at 18.5 s (8.5 + 10). It added the second wheel/turn read after the gate (`contract/refusal-order/send`). Trap it recorded: a case that reads a verified literal-less screen through a clock that never advances waits forever. A paste-bound test using a test clock meets the same hazard.

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract) — set the MCP `wait` cap at 50 s and recorded the client's tool-call timeout as above 50 s, «above the 18.5 s send total». It also defined `-32603` for an internal handler fault and the `isError` shapes (`contract/mcp-tool-errors/-32603`, `contract/mcp-tool-results/*`). The MCP `send` tool's result at the paste bound has to fit these shapes.

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — introduced `send-issued`, `send-confirmed` and `send-refused`, and the rule that «a paste failing after `send-issued` is `input-not-ready` with its cursor» (`contract/send/paste-failure-after-issue`). It added viola-pty's `PasteHandle` and `validate_paste_text`, and the gate tee's `sync_channel(256)` + `try_send`, which drops and poisons rather than blocking (`decision/passthrough-never-waits-on-model`). This is the standing outcome for a failed paste: a cut paste either reuses it or the chunk has to amend it.

@history_item n=6 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Established Decisions) — added `decision/send-flight-lock`: the post-gate reads, the link record, `send-issued` and the paste run under one wrapper flight lock that the hook claim, turn start/end and wheel moves also take, and a human key never takes it. A paste blocked in the PTY write holds this lock, so the bound limits how long hook events and wheel moves can wait behind it.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([PTY] openpty alone; [Human Takeover / Wheel] Windows input retired) — `pty_backend()` is always `openpty`, the pump keeps the input writer until the child exits, and `host_stdin()` is `std::io::stdin()`. The retirement kept «the bridge send as one bracketed write + CR, human bytes held during it» (`decision/atomic-bracketed-send`). That holding is what human keys meet while a paste is blocked or cut, and it is bounded by `decision/human-always-wins-wheel` («never delayed past the current atomic paste»).

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned at =0.9.0) — moved the seam to portable-pty `=0.9.0` (`decision/portable-pty-behind-pty-seam`), the seam unchanged. A write timeout has to be built behind viola's own `pty` seam on this pinned version, with no dependency moved.

@history_item n=9 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — ratified `PtyError` as the one hand-written error, with fixed `Display`/`Error` messages, no thiserror, and the portable-pty error kept only as a boxed source (`decision/pty-error-hand-written`, `decision/pty-knows-no-agent`). A new paste-timeout error in viola-pty follows this form.

@history_item n=10 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** — gave `decision/human-key-never-held-behind-resize` its Linux reason: no key was lost, so viola adds no wait. This is the latest precedent that the human's keys are never held for a PTY-side condition. It applies to what keys do while a paste is blocked and after it is cut.

@history_item n=11 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the wheel as landed) — the stdin classifier only observes and moves the wheel before a read holding an editing key returns (`decision/wheel-classifier-only-observes`, `decision/wheel-byte-source-claim`). The in-memory move is synchronous and the record and snapshot go on a worker thread. This determines whether a key typed during a blocked paste takes the wheel and whether the cut send reads `human-typing`.

@history_item n=12 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — the running-turn state lives in `WheelSlot` and is started by a `prompt-submitted` of any origin. `turn-running` covers a second send in flight, and only `release` of a human-held wheel clears it. This matters for what the next send meets after a cut paste: a still-reserved in-flight slot reads `turn-running`, and a human Enter on the left-over text starts a turn.

@history_item n=13 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added the except of `decision/human-prompt-confirms-wheel-take`: an unsent human-filed prompt with no key since the last human prompt is relabelled harness when a wake was counted. Text left by a cut paste and submitted later has to be read against this rule.

@history_item n=14 marker=2026-10-10-session-links
**2026-10-10-session-links** (the implicit link as landed) — the link is recorded after the second wheel/turn reads and directly before `send-issued`, so «an issued send that then reads `not-delivered` has linked» (`contract/event-line/link`). A send cut at the paste bound has already linked.

@history_item n=15 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (an empty typed text is refused `empty-text`) — precedent for growing the closed `not-delivered` detail set (`contract/refusal-reason/not-delivered`, `decision/refusal-details-closed-set`): the word was added only in viola-core, its rung placed in the send order, and no exit code, method or `v` moved.

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, exit 21's sixth cause) — precedent for adding a closed cause word to `UnreachableCause`: no exit code added, a `diag-line` schema enum entry, and the founder's word because the word crosses outward to a driver. A new closed word at the paste bound is the same kind of crossing.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern and its contract binary) — added `decision/sanitised-error-surfaces`: codes and fixed messages only, enforced by `tests/contract_error_bodies.rs` (`CLI_DOCUMENTS`, `MCP_RESULTS`, the writer census). The driver at the bound gets «a closed word, never upstream text», which has to land as a row the census reads.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the code-word rule and the exit-cause table) — every exit cause and code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`. `tests/cross_exit_causes.rs` holds each cause to its code and hint (`no_two_codes_share_a_hint`, `every_code_is_a_code_word`). A new word or hint for the cut paste joins these tables.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions) — added `decision/retired-exit-codes-never-reused` (exit 11 unassigned, `contract/cli-exit-codes/11`) and the current `send` refusal order without the budget rung (`contract/refusal-order/send`, `contract/send-refusal/order`). A new outcome takes no retired code, and its place in the order comes after `send-issued`.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers; [MCP] the viola mcp stdin line cap) — set `decision/bounded-readers`, held by the census `tests/contract_bounded_readers.rs`. It also set the MCP stdin cap precedent: a bound refuses whole and ends cleanly with one closed `parse-rejected` line, never truncating. This is the nearest standing pattern for bounding the one unbounded PTY write.

@history_item n=21 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — ratified the test seam `FAKE_AGENT_PUMP_DELAY_MS` as a boundary widening (`decision/fake-agent-test-seams`, `decision/test-seams-not-configuration`): fake-agent feature only, capped, absent from release builds. A fake agent that stops reading its PTY falls under the same rule.

@history_item n=22 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam) — `FAKE_AGENT_HOOK_PANIC` became the second seam, ratified by the founder live. Each new env seam has been its own founder-ratified widening (`decision/viola-name-prefix` except, `decision/env-var-viola-prefix`).

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — the founder chose files beside the exe and mode words over «a third fake-agent env seam». This constrains how the fake agent's stop-reading behaviour is switched on: an argv option or mode word, not a new env variable.

@history_item n=24 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the fake agent's argv option list is the count's rule, thirteen options, after adding `--long-turn-screen`. A new stop-reading option updates that registered count.

@history_item n=25 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — `src/run/send.rs` is the module root over `src/run/send/` (`slot.rs` for the in-flight slot and claim, `classify.rs`, `params.rs`), and `src/human/` holds `mirror.rs` and `hints.rs` (`send_hint`). These are where the wrapper's paste path and the human line for a new word live.

@history_item n=26 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame; the paste hint measured) — set `decision/no-probe-for-unrelied-shapes`. It also measured the CLI's 8.0 s paste hint after a long paste. Text a cut paste leaves in the box, and a re-send over it, meet these paste shapes and the hint, and whether a ledger probe is owed turns on whether a send relies on them.

@history_item n=27 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `contract/send-results/input-not-ready` and `contract/human-hint-lines/input-not-ready` now name «a standing modal». These are the current wording of the `input-not-ready` outcome and its hint line, which a cut paste would reuse or differ from.

@history_item n=28 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — reworded `decision/every-send-confirmed`, `decision/send-confirmed-after-the-fact` and `decision/unconfirmed-delivery-not-delivered` with the stdout-echo and unverified-CLI excepts. These excepts are the «never presumed delivered» family that the cut paste's outcome has to stay inside.

@history_item n=29 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — raised the shared root wait bound `WITHIN` to 12 s (`decision/root-waits-share-one-bound`). A test that waits on a blocked paste uses this bound, under the 20 s lint the scope names.
