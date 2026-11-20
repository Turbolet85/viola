@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — set `run.wheel_transition.cause` to `human-input|manual-pause|release` (a point-in-time span, opened once per change on the wheel's worker thread), traced `pause` like `release`, and fixed `release-from-driver`'s trigger. This is the wheel vocabulary a wake that keeps the wheel must leave unchanged (rows contract/wheel-span-attributes/run-wheel-transition, contract/wheel-spans/span-chain, contract/process-log-event/release-from-driver). Any new cause or "no transition" reading needs a closed-enum Decisions Log entry.

@history_item n=2 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running` covers two causes under one closed detail. The running turn is in-memory wrapper state, marked by a `prompt-submitted` of ANY origin, and writes no event, span or line of its own. A fired wake's prompt-submitted therefore still marks a running turn, even after the wake is told apart from a human prompt. The driver's next send can refuse `turn-running` rather than `human-typing`, so the plan must say which of the two it promises.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — kept the wrapper `send-refused`'s `wheel` field required, and it ships with the wheel. This is the field that shows `human` on a send a fired wake turns away (row contract/send-refused/human-typing, contract/wheel-log-line/human-typing). The send.client exit set includes 10 for human-typing.

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `run.confirm_window` closes on the confirmation its send waits for. For an ordinary send that is the matching `prompt-submitted{origin:"driver"}`, and no `prompt-submitted` of another origin claims it. The `origin` this chunk decides for a wake's prompt therefore also feeds the confirm path. A wake told as `driver` could wrongly close a driver send's window, so the told-apart origin must not be `driver`.

@history_item n=5 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the ordinary-send close now matches the prompt text: equal to the typed text, or equal once a trailing ASCII whitespace run is removed. `turn-running` now covers only a turn the wrapper knows of before the paste. It runs under one flight lock, and a UserPromptSubmit arriving after the paste stays undetected (fct-izz7vw4n). That is a known gap in how the wrapper reads a prompt it did not type, and it sits right next to a wake that fires during or after a driver send.

@history_item n=6 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified two things. The trailing-only confirm tolerance stands, and a leading trim waits on a live reading. StopFailure stands registered with NO ledger row and no verify probe. That is the precedent the scope's open question ("whether a ledger row with a verify probe is part of it") must weigh against the trust-record, /compact and /context rows, which did add one.

@history_item n=7 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — the `hook_event` enum gained `stop-failure` (ten closed names), resolved as "no ledger row". This is the precedent for a hook-payload-derived reading that lands with no relied ledger row (row contract/diag-line-fields/hook-events). Any new hook_event or field this chunk reads from UserPromptSubmit changes that closed list through the same route.

@history_item n=8 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — added `parse-rejected{parser:"hook-stdin", detail:"unknown-value"}` for a hook stdin field whose value is outside its closed set, and the second exception to "parse failures surface as unknown" (rows contract/parse-rejected/hook-stdin-unknown-value, contract/parse-rejected-detail/unknown-value, decision/parse-failures-unknown-counter, decision/parse-failures-surface-as-unknown). If the means reads the optional UserPromptSubmit `source` (`schedule_wakeup` · `loop_wakeup`), an outside word follows this precedent: a codes-only line, never `unknown`.

@history_item n=9 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added the named exceptions for reading a session transcript's stdout-tag count. That read is an exception to claude-internals-not-a-contract, refusal-telemetry, transcripts-not-a-contract, deterministic-event-emission and deterministic-emission, and the read itself is uninstrumented (decision/transcript-echo-read-uninstrumented). It also listed the echo-confirmed send as awaiting no hook-invoked. A candidate means that reads the transcript or anything claude-internal to spot a wake needs a further named exception to these same rows.

@history_item n=10 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery exceptions (claude-internals-not-a-contract, refusal-telemetry, send-path event order, deterministic-event-emission, deterministic-emission, transcripts-not-a-contract). The same wrap carried the scheduled-wake entry's founder word that keeps this chunk standing with its means left to its phase. Each widening of these rows has gone to the founder.

@history_item n=11 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify gained Run F (six `verify-pty-probe` runs, nine spawn pairs). claude-internals-not-a-contract and refusal-telemetry gained the trust-record exceptions, and the trust-record write became uninstrumented. A wake ledger row with a verify probe would move these counts again (rows contract/verify-spawn-subject/verify-pty-probe, contract/verify-stdout/step-line, summary-line).

@history_item n=12 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — narrowed claude-internals-not-a-contract and refusal-telemetry to their present excepts, added the start's one `trust-record` line to claude-boundary-only-instrumentation, and added contract/start-trust-log rows. These are the current wordings of the rows a wake-detection means would touch, PROVISIONAL on the delegate's confirm.

@history_item n=13 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — measured that folder-trust keys passing through the wrapper write `wheel{human-input}` before `session-start`, and that a later human key adds no wheel line because the wheel is already human. The start order is read from that start's own lines, since events.ndjson is never rewritten (rows contract/run-start-events/order, decision/product-order-from-events-log, decision/events-stream-never-rotated). Any wake test must read the wheel from its own start's lines, and the trust-keys start already leaves the wheel `human`.

@history_item n=14 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — moved dialog expiry to the wrapper (`dialog-expired` product line, `dialog-answered{deadline_hit:true}`), made the hook's `deadline_hit` only its 3605 s backstop, and introduced `dialog_hold_s`. This is the held-for-the-driver dialog path that a fired wake currently routes to the human, and that the chunk must keep (rows contract/events-log/dialog-expired, contract/hook-decision/deadline-hit, contract/config-json/dialog-hold-s, decision/expired-dialog-renders-for-human).

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `dialog-answered`'s `from` / `from_trust` appear only on an answer that carried a `from` (measured live). This bounds what the dialog lines show of who answered a held dialog (row contract/dialog-log-fields/dialog-answered).

@history_item n=16 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained the `absent` verdict, and exit 0 now counts an absent row. It also added decision/browser-tools-prompt-reading-uninstrumented, a screen reading with no telemetry by design. This is a precedent both for a relied row that can read "absent" when a live event did not occur (a wake may not fire in a verify window) and for keeping a new reading uninstrumented.

@history_item n=17 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — the browser-tools reading's only record is one fixed verify stdout line, with no log line, span or event. This is the narrowest record a new uninstrumented reading has been given.

@history_item n=18 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added contract/confirm-window-close/context, the 25th relied row's close. This is the latest change to `run.confirm_window`'s closed list of closes, which a wake-told prompt must not satisfy.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added `/compact`'s session-start close and the 40 s `window_ms` (rows decision/compact-confirm-window, contract/send-span-attributes/window-ms). It is a precedent for a confirm-path change that landed with a ledger row and a verify paste, and added no new event, detail or field.

@history_item n=20 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — made the hidden `hook --capture` arm uninstrumented by design and made verify's outcome its stdout step and summary lines. A wake probe in verify would run through this arm with no hook lines (rows decision/hidden-capture-arms-uninstrumented, decision/hook-capture-arms-unlogged).

@history_item n=21 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the capture arm, with `--answers`, prints one product-built decision body and stays uninstrumented, and verify's interactive runs grew (Runs C and D). This is how a probe captures a UserPromptSubmit payload without logging it.

@history_item n=22 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the hook has one capture arm, its stdout carries only the decision body, and `hook_event` dropped `statusline` (rows decision/hook-instance-check-before-obs-init, contract/hook-stdout/hook-event, contract/stdout-by-role/hook, decision/hook-capture-arms-unlogged). These are the current shapes of the hook surface that reads UserPromptSubmit.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — removed the `budget-paused` refusal, exit 11 (never reused) and the `budget-gate` start record. The start order became `wheel{cause:"start"}` then `session-start`, and a `release` with any budget key is refused `invalid-params`. It also narrowed the additive-only schema rows to "a field the product still emits" (decision/diag-schema-additive-only, decision/log-fields-additive-only, decision/log-schema-additive-only, decision/obs-aligns-to-tests-log-format). That narrowing bounds what this chunk may change in the line schema.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook instance check before obs init) — every hook arm but `--capture` runs its instance check before log init, and a refused or absent instance stays silent: no stdin read, no line, no frame. The hook's instance check also sits inside the timed spine path. Any UserPromptSubmit reading the means adds runs only after this check and inside the same budget (rows decision/hook-refused-instance-silent, decision/hook-silent-without-trusted-instance, decision/spine-hook-perf-gate).

@history_item n=25 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — built the hook perf rows judged by `gate --require perf` (max under the spine deadline). If the means adds work on the user-prompt-submit hook path, this gate times it (rows decision/hook-latency-budget, decision/hook-perf-budgets-timed, decision/perf-gate-max-sample).

@history_item n=26 marker=2026-10-10-session-links
**2026-10-10-session-links** — the implicit link record stands directly before `send-issued`, a failed record answers -32603 with nothing issued, and it has no line or span. This is the send path's step order between the wheel and running-turn reads and `send-issued` (row contract/events-log/link-before-send-issued).

@history_item n=27 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival and includes the readiness gate's wait (row contract/send-log-fields/send-confirmed). The confirmed driver send this chunk promises after a wake is measured on this basis.

@history_item n=28 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` is the typed text's length after CR/CRLF folding and trailing CR/LF strip (row contract/send-span-attributes/text-bytes). That is the typed text the prompt-submitted claim compares against.

@history_item n=29 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed thread is a contained worker, and a caught panic poisons the model and writes one `parse-rejected{vt100-feed}`. If a means reads the PTY or screen side (the overseer's hypothesis: no human byte before a wake's submit), it lives beside this worker and under no-telemetry-from-screen and no-hot-path-instrumentation (rows decision/no-hot-path-instrumentation, decision/pty-byte-stream-never-logged, decision/typed-keys-never-logged).

@history_item n=30 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words) — `release-from-driver` became a closed `channel-response` detail beside `invalid-params` and `unsupported-version`. This is the wheel path's release reply as logged on both sides (row contract/process-log-event/release-from-driver).

@history_item n=31 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `corr` required in the line schema on `dialog-raised`, `dialog-answered`, `release-from-driver`, wrapper `send-*` and dialog `hook-decision`. It also set when a dialog hook's corr may be null. These are the corr rules any new or changed wheel, send or dialog line must keep (rows contract/corr-by-event/*, contract/process-log-corr/*).
