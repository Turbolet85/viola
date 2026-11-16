@history_item n=1 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (6. Log Coverage: 2 rows) — added `+ contract/parse-rejected-detail/unknown-value` and `+ contract/parse-rejected/hook-stdin-unknown-value`: an outside StopFailure error word writes one codes-only WARN `parse-rejected{parser:"hook-stdin", detail:"unknown-value", count:1}` and the event is still delivered as an api-error turn end. This is the last turn-end change on the Stop-family hook stdin and the precedent for how a new hook-stdin field is read. The scope says `session_crons` absent, null or non-array maps to none as `background_tasks` does. P3 must decide whether that path writes a `parse-rejected` line (this precedent) or none (`background_tasks`'s rule).

@history_item n=2 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (1. Obs Scope Summary: `decision/parse-failures-unknown-counter`, `decision/parse-failures-surface-as-unknown`) — the except grew a second carve-out: a StopFailure error word outside the CLI's set reads cause api-error with one parse-rejected line, never `unknown`. Both rows are in the picked set. A malformed `session_crons` falls under the general rule (`unknown` plus a counter) unless the plan matches `background_tasks`'s none reading. A third carve-out would need these two rows amended together.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (6. Log Coverage) — `~ contract/diag-line-fields/hook-events`: hook_event went from nine to ten names (stop-failure). The operator's delegate resolved F1 as «no ledger row»: StopFailure is registered with no capability-ledger row and no verify probe. This is the direct precedent for scope premise fct-6zblfae3, whether the undocumented `session_crons` needs a ledger row and probe.

@history_item n=4 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (StopFailure no-ledger-row ratified) — the founder ratified «no ledger row» for StopFailure, its live reading owned by «The 2.1.325 recorded fixture set». This bears on fct-6zblfae3: a read-only count that fails to none has a ratified precedent for standing with no ledger row. The same wrap carried the founder's ruling B that this chunk folds in.

@history_item n=5 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — wait's `after` / `timeout_ms` and `outcome` ride `wait` only. `outcome` is the waking kind or `timed-out`, derived once in `viola-channel` over the shared wake set. `last` carries no outcome, and no result body reaches either channel line (the rows `contract/wait-log-fields/channel-request`, `contract/wait-log-fields/channel-response-outcome`). The new count rides the wait/last result body, so it must not appear on `channel-request` / `channel-response`. The wake set and `outcome` derivation stay untouched, matching the boundary that wait's wake rule does not change.

@history_item n=6 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `woken_kind` and the `channel-response` `outcome` list six kinds, the six of `EventKind::WAIT_WAKE` (row `contract/wait-span-attributes/run-wait-dispatch`). The count must leave `WAIT_WAKE` and `woken_kind` unchanged, so no wake rule moves. Directive item 2 (`dialog_pending: false` on a wake's question) touches this hold, which P3's placement block reads.

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running` has two causes. The running turn is in-memory wrapper state, marked by a `prompt-submitted` of any origin and ended by `turn-ended` / `session-start` / `session-end`; it writes no event, span or line of its own. Directive item 2 (a wake's prompt logged `origin: human`) runs through this turn state. P3's block on whether the count lets a wake's prompt be told from a typed one reads here: `turn-ended` is the event this chunk extends.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `run.wheel_transition.cause` is `human-input|manual-pause|release`, and pause/release are traced as `pause.client` / `release.client` › `channel.request` (rows `contract/wheel-span-attributes/run-wheel-transition`, `contract/wheel-spans/span-chain`, `contract/wheel-log-line/human-typing`). Directive item 2's measured trap: a CLI-scheduled wake's prompt moves the wheel to `human` and every driver send is refused `human-typing`. This chunk is placed only, not built, against that wheel path, and must not change it.

@history_item n=9 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — hooks became normalised events sent through `hook.event`, with `session-start{source:"hook"}` as start record three, and the hook perf rows went to «Hook perf gate». This established the hook → normalised event path the count rides: Stop payload → `viola-agent-claude` → `viola-core` normalised `turn-ended` → `events.ndjson`.

@history_item n=10 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (2. Telemetry Strategy: `decision/events-stream-never-rotated`) — events.ndjson is the product audit stream, never rotated nor rewritten, and a reused instance name keeps earlier starts' lines at its head. The added `turn-ended` data field lands in a stream whose old lines stay unchanged, so readers meet `turn-ended` lines both with and without the field. This bears on the boundary that every reader skips unknown fields across mixed binary versions.

@history_item n=11 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (4. Span / Trace Coverage: `decision/product-order-from-events-log`; 1. Obs Scope Summary: `contract/run-start-events/order`) — the start product order is verified from that start's own lines of the events log, with `wheel{human-input}` before `session-start` where trust keys pass through. The order is not duplicated into process logs. Any test of the count's `turn-ended` line must read the start's own lines, not the log's head.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary / 3. / 6.: `decision/diag-schema-additive-only`, `decision/log-fields-additive-only`, `decision/log-schema-additive-only`, `decision/obs-aligns-to-tests-log-format`) — the never-remove rules were narrowed to «a field the product still emits» (operator Q2 «Сузить правило»). Those are the additive-only rows this chunk rests on. Adding the count is additive, but any diag-line field or event-enum growth still goes through a Decisions Log entry, and nothing may be renamed.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Span / Trace Coverage, 6. Log Coverage) — `decision/product-order-from-events-log` dropped `budget-gate`. `contract/diag-line-fields/hook-events` and `send-events` dropped `budget_written`, window, used_percentage, resets_at and paused when the gate was removed. This is the most recent schema move on hook and event lines, and it shows the budget fields went from both `events.ndjson` data and diag-line together. The count is the reverse case: an addition to event data only, with no diag-line field implied.

@history_item n=14 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage: `contract/diag-line-fields/hook-events` ten → nine; 1./11.: `decision/hook-stdout-decision-body-only`, `contract/stdout-by-role/hook`, `decision/sensitive-content-detail-file-only`, `decision/hook-latency-budget` except cleared) — these are picked rows this chunk relies on. The Stop hook's stdout stays the decision body only, and its latency budget applies with no exception. A wake's content (prompt, cron, id) is never sensitive content placed in any file.

@history_item n=15 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — the diag-line schema's default-deny is a top-level `unevaluatedProperties: false` over per-event field lists (rows `decision/schema-top-level-unevaluated-properties`, `decision/default-deny-log-fields`). Any count added to a process-log line, rather than to event data alone, fails G4 unless the schema names it. The scope asks for no diag-line field, which keeps the count off process logs (`decision/event-payload-not-in-process-logs`).

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `parse-rejected` gains `mcp-stdin`) — the template for a closed schema-word extension: the parser enum grows by one, and the Decisions Log record lives in the amendments file, which §12 points to. If the plan adds any closed word (a parser, a detail) for `session_crons`, it follows this form (row `decision/diag-schema-additive-only`).

@history_item n=17 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: `decision/per-parser-parse-failure-counts`, `decision/parse-failures-surface-as-unknown`) — the per-parser count list gained the hyprctl replies, and the parse-failure rows gained their first exception. This is the precedent for widening the counting-parser list. Hook stdin is already in it, so reading `session_crons` inside the hook-stdin parse needs no new parser word.

@history_item n=18 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: `decision/parse-failures-unknown-counter`) — the except first named a closed error word over `unknown`, the row later widened by 2026-11-16-granular-api-error-cause. Together they give the row's present text, which governs a malformed `session_crons`.

@history_item n=19 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (3. / 8.: `decision/veil-redact-payload-types`, `decision/payload-types-derive-redact`) — a payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field. The Stop payload's `session_crons` is read as `IgnoredAny` and kept as a count: a number is no content and needs no Redact, and elements are never typed, so no content field enters the payload type.

@history_item n=20 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (3. Observability Harness Contract: `decision/content-only-in-instance-detail-file`) — home-level files are codes-only and carry no user content, token or CLAUDE* value; the Cookie clause is gone. This is the boundary on wake content: no prompt, cron expression or id of a wake may reach any line. Only the count crosses, and it goes to events, not process logs.

@history_item n=21 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Obs Scope Summary: `decision/claude-internals-not-a-contract`) — the except names the one transcript read, a local command's stdout tag count. This is the nearest precedent for reading an undocumented Claude field as a count with no byte kept. `session_crons` comes from the Stop hook payload, which is viola's boundary, not a transcript, so the row's except need not grow. The ledger-row question (fct-6zblfae3) is the open one.

@history_item n=22 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (4. Span / Trace Coverage: `contract/send-log-fields/hook-invoked`) — an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line. This is a picked row on the send path that the directive's wake-prompt trap (a human-origin prompt, then `human-typing`) runs beside. Unchanged by this chunk.

@history_item n=23 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (4. Span / Trace Coverage: `contract/confirm-window-close/ordinary-send`, `contract/send-refused-detail/turn-running`) — the confirm window closes on the `prompt-submitted{origin:"driver"}` whose prompt equals the typed text, with a trailing-whitespace tolerance. `turn-running` covers a turn known before the paste. A wake-submitted prompt logs `origin: human` and so never closes a driver's window. This is relevant to P3's directive-item-2 block on telling a wake's prompt from a typed one.

@history_item n=24 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (4. Span / Trace Coverage: `contract/confirm-window-close/ordinary-send`) — the founder ratified the trailing-only tolerance, so the row stands without its provisional mark. This is the current text of the picked row.

@history_item n=25 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — `state-recovered` is written by `read_snapshot_or_replay` at WARN with no `corr` (`snapshot-replayed`, `snapshot-unsupported-v`), one line per call (rows `contract/state-recovered/snapshot-replayed`, `contract/state-recovered-detail/*`). The snapshot is rebuilt from `events.ndjson`. If the count is carried into snapshot state the way `background_tasks` is (`src/run/send/slot.rs`, `src/cmd/run.rs`), the replay must rebuild it from the `turn-ended` line, and old lines with no field must replay to none.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers two to six) — hook `connect`, `run`'s start, `live_endpoint`, `revive` and `list` all read through the replay. Whatever hop the count takes into `list` / `last` (scope fct-wgwmlnbf, measured at P3) passes through these readers. A replayed snapshot carries no live fields (rows `contract/hook-decision/channel-unreachable`, `decision/no-raw-snapshot-reads`).

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook event and dialog paths' instance check before obs init) — every hook arm but `--capture` runs the instance check first, and a refused or absent instance exits 0 with no stdin read and no line (rows `decision/hook-refused-instance-silent`, `decision/hook-silent-without-instance`). The Stop payload's `session_crons` is read only after that check passes, so no new hook path may read stdin before it.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check inside the timed path) — `run --perf` keeps the five hook rows (stop included) under `SPINE_DEADLINE`, with the obs overhead held inside the same gate (rows `decision/spine-hook-perf-gate`, `decision/hook-obs-overhead-budget`, `decision/hook-perf-budgets-timed`). Counting `session_crons` on the stop hook adds to that timed path and must stay inside `max < 1.0 s`.

@history_item n=29 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's reply screened by `sanitised_fault` and `is_code_word`) — the CLI and MCP clients screen a peer reply's refusal and detail to code words, the screen adding no span or line (row `decision/clients-screen-peer-replies-to-code-words`). A count in the wait/last result body is a typed number, not a refusal or detail, so the screen leaves it alone. The plan should confirm the count is no string field that a client screen or error-body census (`decision/error-bodies-clean`) would need to cover.
