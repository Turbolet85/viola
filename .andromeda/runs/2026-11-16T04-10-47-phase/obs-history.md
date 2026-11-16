@history_item n=1 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (turn-running's two causes) — the wrapper `send-refused` keeps `detail` ∈ `input-not-ready|no-prompt-submitted|turn-running`; `turn-running` covers a running turn (in-memory state set by a `prompt-submitted` of any origin, ended by `turn-ended` / `session-start` / `session-end`, cleared by a wheel-returning `release`, writing no event, span or line of its own) or another send in flight; no `cursor` (corr the end offset), no `send-issued`, client exit 13. The precedent for closing the readiness-gate turn race (v2-19) with the existing detail and no new field.

@history_item n=2 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (vt100 panic capture) — the readiness gate's vt100 feed runs on a contained feed thread (`src/run/gate.rs`) under per-call `catch_unwind`; a poisoned model gives one `parse-rejected{vt100-feed}` and the send reads `input-not-ready`. The gate whose window the turn race sits in; its outcomes are the only screen-derived telemetry (rows no-telemetry-from-screen, deterministic-emission except).

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (feed overflow, send.client exits; the G2 chaos home) — `vt100-feed` gains `oversize`; `send.client` exits gain 12 and 1 `internal-error`; the wrapper `send-refused` `wheel` field required; `tests/chaos_feed_panic.rs` witnesses `send` refused `input-not-ready` exit 13 after a feed panic. The confirmed-send path's landed shape this chunk extends; any new race test must keep the outside-scan carve-out rules.

@history_item n=4 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `run.wheel_transition.cause` is `human-input|manual-pause|release`, a point-in-time span opened once per wheel change on the wheel's worker thread (row contract/wheel-span-attributes/run-wheel-transition). The trim mismatch today misreads the driver's prompt as `human-input` and moves the wheel; the trim-tolerant match must stop that move without a new cause word.

@history_item n=5 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `run.confirm_window` closes on the confirmation its send waits for (ordinary send: matching `prompt-submitted{origin:"driver"}`); an unconfirmable send opens no window and logs `send-issued` + `send-confirmed{confirmed:false}`; `viola.send.confirm_ms` counts only confirmed lines. The match being widened is exactly this "matching" close; `src/run/send/classify.rs` reads the exact typed text and is untouched by the trim rule.

@history_item n=6 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival, including the readiness gate's wait up to `GATE_MAX_WAIT` (8.5 s on a delivered send). A send typed only after a turn ends (one fix for the race) lengthens this sample; the field's definition already covers it (row contract/send-log-fields/send-confirmed).

@history_item n=7 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — `text_bytes` is the typed text's length (trailing LF not typed), on `pty.paste_write` and `send-issued`. Precedent for a typing rule that differs from the sent text; the scope keeps typed_text unchanged, so `text_bytes` must not change with the trim-tolerant match.

@history_item n=8 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `empty-text` detail and the `text_bytes` wording) — refusal details gain `empty-text`, one line per deciding side (client before any frame, wrapper with corr/conn/rpc_id, no `send-issued`, never the text); `send-refused.detail` is an open string in `diag-line.v1.json`. Bears on the scope's open question whether a whitespace-only (but non-empty after typed_text) text needs its own decision: a new detail word adds no schema field but needs the catalog rows (refusal-reason-words, send-log-line/detail) amended.

@history_item n=9 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` = typed text with each CR LF and lone CR as one LF, trailing CR/LF removed (row contract/send-span-attributes/text-bytes), measured live on 2.1.287. The typed-text rule this chunk must keep verbatim ("nothing else is taken off").

@history_item n=10 marker=2026-10-10-session-links
**2026-10-10-session-links** — the implicit link is recorded after the second wheel and running-turn reads, directly before `send-issued`; a failed record is `-32603` with nothing issued (rows implicit-link-on-send-and-answer, events-log/link-before-send-issued). Any reordering of the gate/turn rung to close the race must keep the link step's place.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` joins `/clear` on the session-start leg; `window_ms` from `SendSlot::window()` (40 000 compact, else 10 000); expiry `no-prompt-submitted` for all three; verify MM gained a ledger row. `SendSlot` is the surface the trim match edits; also the precedent for standing undocumented CLI behaviour on a ledger row with a verify probe.

@history_item n=12 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify passes `--framing`; seventeen rows stamped through the framing replay. If the trim tolerance stands on a capability-ledger row, the fake agent's verify flag list and boot step 4 move with it.

@history_item n=13 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI verify runs only against the fake agent; real-`claude` verify only via local `run --local-live`, refused under CI (rows ci-verify-fake-agent-only, real-cli-verify-never-in-ci). Constrains how a trim probe can be proven: the fake agent must submit a trimmed prompt.

@history_item n=14 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verification refusal's lines in the scenarios) — the confirmed-send scenario's client-side refusals before any frame write no channel line; the send path's ordering of client checks before `channel-request`. Context for the send event order row this chunk rests on.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words; sanitised_fault / is_code_word; exit-2 paths of send) — `channel-response.detail` only on `-32602`, a refusal's own detail stays on `send-refused`; a peer refusal word is screened by `is_code_word`; `viola send`'s exit-2 path writes fixed `UNUSABLE_TEXT`. A new or reused send refusal must land on `send-refused`, never on the channel line.

@history_item n=16 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — `diag-line.v1.json` requires `corr` on `send-*` when `side` is `wrapper`; mis-shaped `conn` logged as `srv_conn`. Any new wrapper send line from the race fix must carry corr/conn/rpc_id (rows send-line-correlation, send-line/conn-rpc-id).

@history_item n=17 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `obs_event!` attaches event/process/instance; `corr` is a caller-supplied typed field; schema default-deny via `unevaluatedProperties: false` (rows corr-set-explicitly-by-caller, default-deny-log-fields). Any added field on send lines fails G4 unless in the schema.

@history_item n=18 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — raw-tracing ban enforced by `clippy.toml` disallowed-macros plus a fail-closed raw-`event!` grep in `scripts/lint-probes.sh` (row source-allow-list-logging guard). New send-path logging goes through `obs_event!` only.

@history_item n=19 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — the fake agent (`src/bin/viola-fake-agent.rs`) is exempt from the print ban by a crate-level allow (row no-print-macros except). The chunk edits the fake agent to submit a trimmed prompt or start a turn in the gate's window.

@history_item n=20 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — `state-recovered{torn-line-healed}` written by the appender that heals, one per heal, none under a held lock (rows appender-heals-torn-line, state-recovered-detail/torn-line-healed). `append_hook_event` feeds `SendSlot::claim`; the relabel write path sits beside this append.

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `parse-rejected` gains `mcp-stdin`) — per-parser counts add the MCP stdin cut; paste validation refuses as not-delivered instead (row per-parser-parse-failure-counts). The "validated as received, never stripped" half keeps that refusal, not a parse-rejected line.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (MCP's `instance-unreachable` reject writes no line) — MCP rejects are `isError` codes only, some with no line (rows mcp-results-codes-only, mcp-rejections-codes-only). MCP `send` outcomes are touched only as they read the wrapper's reply.

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows in every area, each verified against the code; the picked rows' handles and their twin areas originate here, and later entries amend them by handle.

@history_item n=24 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — `~ decision/secrets-never-logged`, `~ decision/no-secrets-in-log-lines`, `~ decision/content-only-in-instance-detail-file`, `~ decision/claude-env-and-cookie-never-logged` (rule and name), `− decision/cookie-header-never-logged`: the Cookie clause retired. Picked never-log rows read in their current, cookie-free form.

@history_item n=25 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/per-parser-parse-failure-counts`, `~ decision/parse-failures-surface-as-unknown` and `~ decision/parse-failures-unknown-counter` (hyprctl except), `~ contract/diag-line-fields/parse-rejected` (hyprctl-json). Picked rows carry these exceptions; the trim rule in `viola-agent-claude` is a normalised predicate, not a parser.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `− contract/send-refused/budget-paused`, `~ contract/diag-line-fields/send-events` (budget fields dropped), `~ contract/send-refused-detail/refusal-reason-words` (five-hour/seven-day gone), `~ decision/exit-code-log-field` (11 retired), the send path has no budget rung. Also `~ decision/diag-schema-additive-only`, `~ decision/log-fields-additive-only`, `~ decision/log-schema-additive-only`, `~ decision/obs-aligns-to-tests-log-format` narrowed to "a field the product still emits". The send rungs and refusal words this chunk works within, and the schema rule any send-line change must keep.

@history_item n=27 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/sensitive-content-detail-file-only` (send text, tool input, last_assistant_message only), `~ contract/diag-line-fields/hook-events` (nine hook names). Picked rows in their current form: send text stays detail-file only.

@history_item n=28 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/events-stream-never-rotated` (never rewritten; cursors are byte offsets, a reused name keeps earlier lines), `~ contract/run-start-events/order` and `~ decision/product-order-from-events-log` (folder-trust keys write wheel{human-input} before session-start on 2.1.325). The send cursor corr and the wheel's human-input path the trim fix must not disturb; measured on the same CLI version the trim is re-measured on.

@history_item n=29 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `~ contract/diag-line-fields/hook-events`: `stop-failure` joins the hook_event enum (ten), StopFailure ending the turn with a closed cause. A further turn-end source for the running-turn rung the readiness-gate race reads.
