@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — 6. Log Coverage: changed contract/diag-line-fields/hook-events, so the diag-line `hook_event` enum holds ten names with `stop-failure` added. This is the chunk this entry supersedes, the one that minted `turn-ended`'s single `api-error` word. It shows how the StopFailure hook landed in the obs rows: only the hook name was added, with no cause field in any diag line. Growing the `cause` set from one word to nine must keep that form. The word lives in `events.ndjson`, `wait --json` and MCP `wait`, and no process-log line carries it or the error text.

@history_item n=2 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified) — the founder ratified "no ledger row": StopFailure stays registered with no ledger row and no verify probe, and its live reading belongs to «The 2.1.325 recorded fixture set». This is the precedent for the scope's ban on a live start. The nine-way mapping rests on recorded payloads and the fake agent. Any value that can only be read live is named with its probe shape, not added as a verify row.

@history_item n=3 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Obs Scope Summary) — added an except to decision/claude-internals-not-a-contract: the transcript's stdout tag count is read as an echo confirmation, and no transcript byte is kept. It also added an except to decision/refusal-telemetry. The picked row now carries this one exception. Classifying StopFailure's `error` word is a hook-payload read, not a transcript read, so it needs no second exception. The rule that no byte is kept is the same rule this chunk applies to `error_details` and `last_assistant_message`.

@history_item n=4 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (11. Obs Anti-Patterns) — added the same tag-count except to decision/transcripts-not-a-contract. If the chunk's plan for noticing an unmapped `error` value touches any transcript or upstream-text read, this exception is the only one allowed. Anything wider takes a new row amendment.

@history_item n=5 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (10. SLO Invariants) — changed the except of decision/deterministic-emission, a picked row: emission keys on hook events and `events.ndjson` offsets, apart from the vt100 gate and the tag-count echo. The `cause` word must come from the StopFailure hook payload alone, never from screen content, so no further exception is needed.

@history_item n=6 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary) — narrowed decision/diag-schema-additive-only to "must not rename or remove a field the product still emits". The operator chose this with «Сузить правило». It binds the chunk: the nine words are an additive change to a closed value set. `api-error` stays one of them and is never removed or renamed.

@history_item n=7 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary) — the same narrowing for decision/log-fields-additive-only, a picked row: a required field the product still emits is never renamed or removed. If the chunk adds a counter or a closed log word for unmapped values, it is additive and touches no existing field.

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. Log Coverage) — narrowed decision/log-schema-additive-only to "renames nothing and removes only a field no build emits". The same rule as items 6 and 7, for any `diag-line.v1.json` enum value the chunk might add, such as a parse-rejected `detail` or `parser` word for an unmapped `error` value. This entry also removed EventKind::BudgetGate, a precedent for changing a closed `viola-core` event vocabulary through row amendments.

@history_item n=9 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary, 3 rows) — changed decision/per-parser-parse-failure-counts to add the hyprctl replies, and decision/parse-failures-surface-as-unknown to add the except that a hyprctl parse failure reads a closed error word with one parse-rejected line. Both are picked rows. This is the nearest precedent for the scope's open question of how a new CLI `error` value is noticed. One option is a per-parser count or one closed parse-rejected word. The other is to fold the value into `api-error`, which leaves it indistinguishable from `unknown`.

@history_item n=10 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary, 2 rows) — added the same hyprctl except to decision/parse-failures-unknown-counter, a picked row. If the chunk logs an unmapped `error` value as a closed word, the row's except list must name it the same way. If the value just reads `api-error` with no line, the row stands but the plan must say plainly that the value is not noticed.

@history_item n=11 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage) — added `hyprctl-json` to the parse-rejected `parser` enum and created contract/parse-rejected/hyprctl-json "with its emitter". This is the template if the chunk counts unmapped StopFailure `error` values on a parse-rejected line, for example under a hook-stdin `detail`. Each new closed word needs its emitter, a contract row and a schema enum member. contract/diag-line-fields/parse-rejected is a picked row.

@history_item n=12 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `parse-rejected` gains the `mcp-stdin` parser) — a new closed `parser` value requires a Decisions Log record in the amendments file, and the count follows the schema enum. This is the precedent for any closed log word the chunk adds to mark an unmapped `error` value. Without that record, decision/event-values-closed and decision/field-allow-list-default-deny forbid it.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — §6 gained the rule that a closed schema word no build can emit leaves the schema with its emitter. The counterpart for this chunk: each of the nine `cause` words, and any new unmapped-value log word, needs a real emitter in `viola-agent-claude`'s classifier. No word is reserved without one.

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `empty-text` detail) — added a refusal detail and noted that `send-refused.detail` is an open string in `diag-line.v1.json`, "so no schema moved". This precedent separates widening a word set from widening a schema enum. The chunk should check whether `turn-ended`'s `cause` is enforced by `TurnEndCause` and the wrapper's line check alone, or also by a schema.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause) — introduced `viola_core::UnreachableCause` as five closed words: no serde path reaches them, both clients write each word into their documents, and the type writes no log line. This is the closest precedent for `TurnEndCause` growing in `viola-core` and crossing into `--json` and MCP as a closed word. Only the word crosses, never the upstream text.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `instance-unreachable` body names its cause) — MCP `isError` bodies carry the closed cause word from `viola-core`, with codes only and no line. This bears on MCP `wait` carrying the nine words and on the picked row decision/mcp-rejections-codes-only: a word outside the closed set must never reach an MCP result.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's closed causes, five to six) — grew `UnreachableCause` by one word with no serde path. The `viola-core` entity row and every consumer list were amended together. This is the pattern for growing `TurnEndCause` from one word to nine: every consumer that matches on the word is amended in the same pass.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's reply screened by sanitised_fault and is_code_word) — a peer's `refusal` and `detail` are screened as code words (`is_code_word`), and an unknown code word stays visible and exits 14. This is the source of the picked rows decision/clients-screen-peer-replies-to-code-words and decision/peer-replies-screened-as-code-words. Readers of a `turn-ended` `cause` in `wait`'s reply must likewise accept the nine and refuse any other text.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words) — closed an open `detail` string to a schema enum of three words, with `v` still 1 and the wire form unchanged. This is the source of the picked row contract/channel-response/detail and a precedent for widening a closed word set without bumping `v`.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body census, check and live witnesses) — `tests/contract_error_bodies.rs` and `assert_error_body_clean` (classes `absolute-path`, `caused-by`, `serde-path`, `planted`) are the source of the picked rows decision/error-bodies-clean and decision/error-messages-interpolate-nothing. If the chunk adds any error writer or body, it must name it in the census. They are also the existing witnesses that no CLI error text reaches a document.

@history_item n=21 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the thiserror derive census on the product sources) — the census reads 9 `thiserror::Error` derives over the product sources (outside `crates/viola-e2e`). If the classifier or `TurnEndCause` change adds or removes a derive, the count moves and must be re-measured. The picked row decision/error-messages-interpolate-nothing applies to any new message.

@history_item n=22 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (wait channel lines scoped) — `wait`'s `outcome` is the waking kind or `timed-out`, derived once in `viola-channel`, and no result body reaches either line. This is the source of the picked rows contract/wait-log-line/outcome, contract/wait-log-fields/channel-response-outcome and contract/wait-log-line/required-fields. The `cause` word rides in the `wait` result body only, never on the wait log line, whose `outcome` stays `turn-ended`.

@history_item n=23 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `woken_kind` and `outcome` list grew to six kinds, those of `EventKind::WAIT_WAKE`. This is the precedent for the wake set: the chunk mints no new EventKind (per the scope), so `WAIT_WAKE` and the `outcome` enum stay unchanged.

@history_item n=24 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — the wrapper's in-memory running turn is ended by `turn-ended`, `session-start` or `session-end`, and writes no line of its own. The running-turn state is a consumer of `turn-ended`. It must keep ending the turn on every one of the nine causes, not only on `api-error`.

@history_item n=25 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (8. PII Scrubbing) — changed decision/payload-types-derive-redact: every payload type that carries content derives veil Redact, and a type with no content field derives none. veil-redaction is a picked row. The StopFailure payload carries `error_details` and `last_assistant_message`, so its type keeps Redact. A type holding only the closed `cause` word derives none.

@history_item n=26 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11) — the §5 grouping keys became `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. This is the source of the picked row decision/metric-grouping-closed-enums. A count of unmapped `error` values would be derived by jq over one of these closed keys, never by the raw value.

@history_item n=27 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp list tool's lines, invalid-params) — the codes-only set gained `invalid-params`, its `detail` a closed field word. This underlies the picked rows decision/mcp-rejections-codes-only and decision/mcp-results-codes-only: MCP `wait`'s result carries the nine-word `cause` and no CLI text.

@history_item n=28 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`channel-response` logs `method` only for a closed-list name) — measured: a name off the closed list logs no key. This is the source of the picked row contract/diag-line-fields/channel-response. It is a trap for any test that asserts a value outside a closed list on a line. The same applies to tests of a `cause` outside the nine.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook event and dialog paths' instance check before obs init) — every `viola hook` arm except `--capture` checks its instance before obs init. A refused instance exits 0 with no stdin read and no line. This is the source of the picked rows decision/hook-silent-without-trusted-instance and decision/hook-refused-instance-silent. The StopFailure hook path that classifies `error` runs only after this check.

@history_item n=30 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. Obs Scope Summary, 4 rows) — narrowed decision/hook-stdout-decision-body-only, a picked row, to "the decision body" only. The StopFailure hook's stdout stays the decision body. No classified word or error text reaches it.

@history_item n=31 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. Obs Scope Summary, 7 rows) — cleared the except of decision/hook-latency-budget, a picked row. Every hook, StopFailure included, is now inside the max < 1.0 s budget. The classification stays a pure in-memory map with no added I/O.

@history_item n=32 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (11. Obs Anti-Patterns) — changed decision/sensitive-content-detail-file-only and contract/stdout-by-role/hook, both picked rows. `last_assistant_message` reaches only an instance detail file, inside a chain or drift report. The scope's rule that the error text crosses nothing rests on this row.

@history_item n=33 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (3. Observability Harness Contract) — changed decision/content-only-in-instance-detail-file, a picked row: home-level logs carry no user content, token or CLAUDE* value. The CLI's `error_details` and raw `error` value fall under this row and never reach a home-level line.

@history_item n=34 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (2. Telemetry Strategy) — changed decision/events-stream-never-rotated, a picked row: `events.ndjson` is never rotated or rewritten, and a reused name keeps earlier starts' lines. Old `turn-ended` lines carrying `api-error` stay in place. Readers of the event log must accept all nine words alongside earlier lines.

@history_item n=35 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (obs_event! field set, schema default-deny keyword) — schema default-deny is enforced by a top-level `unevaluatedProperties: false`. This underlies the picked rows decision/field-allow-list-default-deny and decision/schema-conformance-failure-fails. Any log field or word the chunk adds, such as a count of unmapped values, must be named in the schema or G4 fails.

@history_item n=36 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows) — turned the obs master into rows, each verified against the code. Since then the obs plan changes only through row amendments such as `~ decision/...` and `+ contract/...`. The scope expects the obs rows that name `api-error` to be reconciled at the wrap through these row amendments.
