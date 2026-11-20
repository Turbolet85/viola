@history_item n=1 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (where a vt100 panic is caught and what it writes) — set the readiness gate's feed thread (`src/run/gate.rs`) as a contained worker: a `catch_unwind` around each feed and resize, one `parse-rejected{parser:"vt100-feed", detail:"panicked"}` at WARN for each poisoning episode, the model poisoned until the host size changes, and `send-refused{detail:"input-not-ready"}` once send reads the verdict. Any new discriminator in the screen predicate runs on that same feed and model. A poisoned model must keep refusing `input-not-ready` (the conservative side), and the new reading must add no line of its own.

@history_item n=2 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (feed overflow, send.client exits) — `oversize` also covers `vt100-feed`: a copy that the bounded feed queue drops poisons the model, one line per episode, with the detail set by its cause (`panicked` or `oversize`). The wrapper `send-refused` `wheel` field stays required. A predicate change must leave both poisoning causes and their single line unchanged. The client exit 13 for `input-not-ready` stays the refusal's outcome.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the G2 question answered: the chaos home outside the scans) — `tests/chaos_feed_panic.rs` witnesses one `parse-rejected{vt100-feed, panicked}` and one panic line, then `viola send` refused `input-not-ready` (exit 13), from its `TestHome::outside_scan()` home outside G2, G4 and the secret scan. This is the precedent for a refusal-side gate test. A new case that forces a panic must use that carve-out and must not add a second G2 exemption.

@history_item n=4 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic test's passthrough half) — the same chaos test also asserts that none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal, and that the child's bytes do. A gate change must keep run silent on the terminal (`decision/run-silent-terminal`) while it refuses.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival, so it includes the readiness gate's wait, up to architecture's `GATE_MAX_WAIT` (named by its constant, never copied). If the discriminator changes how long the gate waits before it gives a verdict, this field moves with it, and `contract/send-log-fields/send-confirmed` stays worded on the constant.

@history_item n=6 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — the wrapper `send-refused` detail stays in the closed `input-not-ready|no-prompt-submitted|turn-running` set. A new cause behind an existing detail is stated in the scenario with no new field or line. This is the precedent if the gate keeps `input-not-ready` for a standing modal: any narrowing or new cause is worded under the existing detail, not as a new code.

@history_item n=7 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. Log Coverage: 3 rows) — changed `contract/send-refused-detail/refusal-reason-words` to the current closed list (manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text) and `contract/diag-line-fields/send-events` (the budget fields dropped). This is the register that a new refusal word would extend, only through a Decisions Log entry.

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (the additive-only rules narrowed: `decision/diag-schema-additive-only`, `decision/log-fields-additive-only`, `decision/log-schema-additive-only`, `decision/obs-aligns-to-tests-log-format`) — each now reads "may add, never rename or remove a field the product still emits", on the operator's Q2. Any field or detail word this chunk touches must keep to that rule.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. Span / Trace Coverage: 1 row) — added `decision/browser-tools-prompt-reading-uninstrumented`: reading a modal literal (the browser-tools prompt) off the screen adds no log line, span, event or detail code. This is the direct precedent: a change to how the gate reads compiled modal literals stays uninstrumented in the same way.

@history_item n=10 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. Span / Trace Coverage: 3 rows) — changed `contract/verify-stdout/step-line`, `contract/verify-stdout/summary-line` and `contract/verify-exit-codes/0` for the `absent` word of the browser-tools-prompt row. If the predicate fix also moves verify's modal reading (Run B framing pastes share the predicate), these verify stdout promises must still hold.

@history_item n=11 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — changed `decision/browser-tools-prompt-reading-uninstrumented` so that verify's one fixed stdout line on a met prompt is the only record that the prompt stood at start. A modal reading's only output surfaces are the stamp and that stdout line, never a log line.

@history_item n=12 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (10. SLO Invariants: 1 row) — changed the except of `decision/deterministic-emission`: the readiness gate reads the vt100 screen before a paste and can refuse a send as `input-not-ready`, and a local command's echo keys on its transcript tag count. The gate is the one named screen-derived exception. A discriminator must stay inside that exception (gate outcomes only, `decision/no-telemetry-from-screen`) and must not derive telemetry from screen content. The founder ratified it on 2026-11-17 (2026-11-17T05-21-35Z-wrap).

@history_item n=13 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Obs Scope Summary: event order and deterministic event emission) — changed `contract/send-path-log-events/event-order` and the except of `decision/deterministic-event-emission` for echo-confirmed local commands. The send's line order is the frame that a delivered send (no longer refused at the gate) must follow.

@history_item n=14 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (4. Span / Trace Coverage: 3 rows) — changed `contract/send-log-fields/hook-invoked` and `contract/send-refused-detail/no-prompt-submitted`, and added `decision/transcript-echo-read-uninstrumented`. The transcript is read only for the tag count, inside `run.confirm_window`. This is a trap: telling a quoting row from a standing modal must not lean on the transcript file, which is outside that bounded exception.

@history_item n=15 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (11. Obs Anti-Patterns: 1 row) — added the one except to `decision/transcripts-not-a-contract`, the stdout tag count only, with no byte kept. A gate discriminator that reads the session transcript would need a new ruled exception, which this chunk's scope does not have.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. Obs Scope Summary: `decision/claude-internals-not-a-contract` and `decision/refusal-telemetry` excepts) — set the current wording of both picked rows. Claude internals are read only at the boundary plus the named exceptions. The gate must read viola's own screen model, not claude state, which is consistent with the scope's "no relied ledger row changes".

@history_item n=17 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. Obs Scope Summary: `decision/claude-boundary-only-instrumentation`) — the closed boundary list is spawn and exit, paste writes, the vt100 readiness-gate outcome and the start's trust-record line. The gate's outcome is the one instrumented screen boundary, and this chunk must add no further boundary item.

@history_item n=18 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — set the confirm window's closes and the unconfirmable send (one `send-issued` plus `send-confirmed{confirmed:false}`, no window, chain ending at `pty.paste_write`). These are the picked `contract/send-line/unconfirmable` and the span-chain rows. The "both directions" tests of a delivered send read these lines after the gate passes.

@history_item n=19 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — changed `contract/confirm-window-close/ordinary-send` (a trailing-whitespace tolerance) and `contract/send-refused-detail/turn-running` (a turn known before the paste, one flight lock). The founder ratified both on 2026-11-16 (2026-11-16T06-05-33-wrap). A send delivered past a quoting row is confirmed by this close rule.

@history_item n=20 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — set the compact window (`COMPACT_CONFIRM_WINDOW` 40 s, PROVISIONAL) beside the 10 s window for every other confirmed send, and `window_ms`. This is the picked `decision/compact-confirm-window`, which stands unchanged next to the gate.

@history_item n=21 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added `contract/confirm-window-close/context` to the closed list of `run.confirm_window` closes. This is the latest extension of the send confirmation register next to this chunk.

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `empty-text` detail and the `text_bytes` wording) — added `empty-text` with one line per deciding side (client before any frame, wrapper with corr, conn and rpc_id), never the text. These are the picked `contract/send-refused-client/*` rows: the validation refusals run ahead of the gate and are unchanged by it.

@history_item n=23 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — set the current `text_bytes` rule (CR LF and lone CR as one LF, trailing CR and LF removed) on `pty.paste_write` and `send-issued`. This is the picked `contract/send-span-attributes/text-bytes`. Test fixtures that quote a modal literal in sent text must still log only `text_bytes`, never the text.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words) — `channel-response` carries `detail` only on a `-32602`. A send refusal's detail (for example `input-not-ready`) stays on `send-refused` alone, never on the channel line. A refusal in this chunk's tests must follow that rule.

@history_item n=25 marker=2026-10-10-session-links
**2026-10-10-session-links** — the implicit link is recorded after the second wheel and running-turn reads, directly before `send-issued`, and is skipped on a refused call (picked `decision/implicit-link-on-send-and-answer`, `contract/events-log/link-before-send-issued`). A send now delivered instead of refused at the gate begins recording its link, which is a behaviour change the tests should expect.

@history_item n=26 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (`decision/parse-failures-surface-as-unknown` and `decision/parse-failures-unknown-counter` excepts) — set the current wording of the picked parse-failure rows. If the gate "cannot tell", that is a refusal on the conservative side, not a parse failure. The discriminator must not invent an `unknown` value or a new parse-rejected parser without a Decisions Log entry.

@history_item n=27 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's exact-path seam exemption) — G2 leaves out only a panic at exactly `src/cmd/hook/seam.rs:<digits>`. These are the picked `decision/zero-panics-gate` and `decision/g2-zero-panics-gate`. Any forced vt100 panic in this chunk's tests must use the chaos home carve-out (n=3), never a new G2 exemption.

@history_item n=28 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added `decision/wheel-typed-mark-uninstrumented`: in-memory wrapper state, with instrumentation not applicable by plan. This is the latest precedent, from the chunk just before, for recording a new in-memory gate or screen-model state as uninstrumented with a row of its own.
