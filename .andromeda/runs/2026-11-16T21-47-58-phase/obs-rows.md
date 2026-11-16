# obs-plan · the rows that bind 2026-11-16-scheduled-wake-count — picked 159 of 1024 (74 by every draw) · rule any · pin 2090

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream.
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
decision/hook-stderr-forbidden · No stderr from hooks — viola hook writes nothing to stderr.
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body.
decision/mcp-rejections-codes-only · MCP rejections carry codes only — MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/viola-core-emits-no-telemetry · viola-core emits no telemetry — The viola-core crate supplies log-field values but emits nothing and is not instrumented. · why: A pure type crate with no I/O.
contract/viola-mcp-tools/wait · viola-mcp stdio server tools · wait — The viola-mcp stdio server offers a wait tool.
decision/hook-log-write-failure-swallowed · Hook log write failure swallowed — A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
decision/redaction-not-disableable · Redaction cannot be disabled — No verbosity control, env, flag or config may disable or bypass redaction.
contract/viola-mcp-tools/last · viola-mcp stdio server tools · last — The viola-mcp stdio server offers a last tool.
contract/viola-mcp-tools/list · viola-mcp stdio server tools · list — The viola-mcp stdio server offers a list tool.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
decision/hook-latency-budget · Hook latency budget — Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append. · guard: run --perf hook rows under SPINE_DEADLINE
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
decision/hook-never-writes-stderr · Hook never writes stderr — A hook process never writes to stderr.
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/hook-fail-open/exit · Hook fail-open · exit — A hook that fails open exits 0 and leaves stderr empty.
contract/hook-stdout/hook-event · Hook stdout · viola hook <event> — stdout carries only the decision body.
contract/wait-log-line/outcome · Wait log line · outcome — The wait outcome is the woken kind, timed-out or instance-unreachable.
contract/wait-log-line/required-fields · Wait log line · required fields — A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/hook-log-failure-swallowed · Hook log write failure swallowed — A failed log write in a hook is swallowed and the hook still exits 0.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/redaction-cannot-be-disabled · Redaction cannot be disabled — No environment variable, flag or config setting may disable log redaction.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/wait-log-line/exit-code-21 · Wait log line · exit_code 21 — A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).
contract/wait-path-log-events/channel-pair · Wait path log events · channel pair — A wait logs channel-request and channel-response, the response carrying the outcome.
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors
decision/hook-silent-without-instance · Hook silent without an instance — A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
contract/strict-modes-refusal-exit/21 · strict-modes refusal exit codes · 21 — The six frame verbs and link / unlink exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/hook-output/stderr · hook output streams · stderr — A hook writes nothing to stderr.
contract/hook-output/stdout · hook output streams · stdout — A hook writes only its decision body to stdout, and no telemetry reaches stdout.
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/uninstrumented-components · Components left uninstrumented — viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented. · why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/unwrapped-hook-no-op · Unwrapped hook is a no-op — A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty. · guard: edge test E1, asserted by absence
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
contract/cli-wait-last-connect/strict-modes-failed · CLI wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/mcp-wait-last-connect/server-verify-failed · MCP wait / last connect refusal · server-verify-failed — A serving process that is not the recorded wrapper or runs under another euid returns instance-unreachable with detail server-verify-failed and writes no line.
contract/mcp-wait-last-connect/socket-dir-refused · MCP wait / last connect refusal · socket-dir-refused — A refused socket directory returns instance-unreachable with detail socket-dir-refused and writes no line.
contract/mcp-wait-last-connect/strict-modes-failed · MCP wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
contract/state-recovered-detail/snapshot-replayed · state-recovered detail · snapshot-replayed — A state-recovered line at WARN carries this detail, one line per snapshot read call.
contract/state-recovered-detail/snapshot-unsupported-v · state-recovered detail · snapshot-unsupported-v — A state-recovered line at WARN carries this detail, one line per snapshot read call.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
decision/no-raw-snapshot-reads · Product readers read the snapshot through replay — Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
contract/mcp-refusal-text/instance-unreachable · viola mcp wait and last refusal · strict-modes-failed body — MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/wait-log-fields/channel-request · wait and last log fields · channel-request — channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
contract/wait-log-fields/channel-response-outcome · wait and last log fields · channel-response outcome — For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
contract/wait-span-attributes/run-wait-dispatch · wait span attributes · run.wait_dispatch — woken_kind, one of turn-ended, question, permission, plan, session-end, dialog-expired, or timed_out.
contract/wait-span-attributes/wait-block · wait span attributes · wait.block — wait.block carries after and timeout_ms.
contract/wheel-span-attributes/run-wheel-transition · wheel span attributes · run.wheel_transition — wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
contract/wheel-spans/span-chain · wheel spans · span chain — run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
decision/dialog-hook-invoked-after-response · Dialog hook-invoked written after the response — A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once. · why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
decision/expired-dialog-renders-for-human · Expired dialog returns to the human — At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
decision/hook-always-exits-zero · viola hook keeps exit 0 — viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
decision/hook-refused-instance-silent · Refused instance makes the hook silent — On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
decision/product-order-from-events-log · Start product order read from the events log — The start product order, wheel{cause:"start"} before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/dialog-log-fields/hook-lines · dialog log fields · hook-invoked and hook-decision — hook-invoked{corr:<dialog_id>, hook_event, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
contract/mcp-instance-unreachable/server-verify-failed · viola mcp frame tool refusal · server-verify-failed — A refused euid, a mismatch or a check that cannot be made returns the codes-only isError instance-unreachable with detail server-verify-failed, no frame and no log line.
contract/mcp-instance-unreachable/socket-dir-refused · viola mcp frame tool refusal · socket-dir-refused — A refused socket directory returns instance-unreachable with detail socket-dir-refused, no frame and no line.
contract/mcp-instance-unreachable/strict-modes-failed · viola mcp frame tool refusal · strict-modes-failed — A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
contract/send-log-fields/channel-request · confirmed send log fields · channel-request — channel-request{corr:<id>, conn, method, sender}.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/state-recovered/snapshot-replayed · state-recovered line · snapshot-replayed — state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of ten closed hook names), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
contract/hook-decision-detail/fail-open-codes · hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
contract/hook-decision-detail/server-verify-failed · hook-decision detail · server-verify-failed — viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/unknown-value · parse-rejected detail · unknown-value — hook-stdin: a hook stdin field held a value outside its closed set, first the StopFailure error word; the line carries the code and count only.
contract/parse-rejected/hook-stdin-unknown-value · parse-rejected diagnostic line · parser hook-stdin, detail unknown-value — A StopFailure error word outside the CLI's 13 writes one WARN parse-rejected line with parser hook-stdin, detail unknown-value and count 1, carrying the code and count only, and the event is still delivered as an api-error turn end; a known word writes no line.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/mcp-rejects-codes-only · MCP rejects are codes only — MCP rejects are isError codes only, and a logged reject's line holds codes only.
contract/panic-outcome-by-role/hook · panic outcome by role · hook — After a panic, hook exits 0 with empty stdout.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/payload-types-derive-redact · Type-level redaction of payload types — Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/veil-redact-second-layer · veil Redact as the second redaction layer — veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/perf-budget-asserted-on-max · Perf budgets asserted on the maximum sample — Performance budgets are asserted on the max of hyperfine samples, and a failing perf gate still leaves its samples.

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/hyperfine-max-regression-fails · Hyperfine max regression is a failure — A hyperfine max regression beyond the telemetry latency budgets counts as an SLO invariant failure.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/spine-hook-budget · Spine hook budget — The spine hooks and pre-tool-use on an unverified CLI complete under the tests-owned spine deadline, provisionally 1.0 s. · guard: gate --require perf · provisional
decision/hook-obs-overhead-budget · Hook observability overhead budget — Obs overhead on hook, logger init being one open plus appends, is held inside the same max < 1.0 s perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home. · guard: gate --require perf
decision/hook-perf-budgets-timed · Hook performance budgets timed with hyperfine — The session-start, user-prompt-submit, stop, session-end and pre-tool-use hooks are timed with hyperfine on an unstamped session in the perf job, and the perf gate requires all five; the async tier is untimed. · guard: gate --require perf
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/perf-gate-max-sample · Performance budgets gate the max sample — Every performance budget gates hyperfine's max sample; p99 is not computed. · why: max is an upper bound on every quantile · guard: gate --require perf

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/spine-hook-perf-gate · Spine hook perf gate — Spine hooks (session-start, user-prompt-submit, stop) and pre-tool-use on an unverified CLI keep their hyperfine max under the spine deadline, timed over every check a frame-writing hook makes before its first frame. · guard: gate --require perf
contract/hook-decision/channel-unreachable · Hook decision line · channel-unreachable — A hook whose snapshot read is replayed or absent fails open as channel-unreachable.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.
decision/allow-list-not-widenable · Allow-list and redaction not configurable — config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
decision/budgets-asserted-by-exit-code · Budgets asserted by exit code — Every budget has an exit-code-setting assertion (hyperfine max via jq).
decision/hyperfine-max-is-enforced · Hyperfine max is the enforced number — The max from hyperfine is the enforced latency number.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/veil-redact-payload-types · veil redacts payload types — Payload types derive veil's Redact, and cargo deny bans veil's toggle feature. · guard: cargo deny veil toggle ban · except: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field.
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them

## logging-stack · key of ## 3. Observability Harness Contract
decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-internal-error-fallback · No stderr fallback for subscriber internal errors — The diagnostics subscriber never reports its own internal errors by printing to stderr. · why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/hook-refusal-exits-zero · Refused hook fails open — A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
decision/hook-single-log-open · One config read and one log open per hook — A hook invocation makes one config.json read and exactly one log open, then only appends. · guard: SPINE_BOUND

## product-mode-normal-vs-inverted-self-observing · key of ## 3. Observability Harness Contract
decision/normal-observability-mode · Normal observability mode — viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector. · why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.

# obs-plan · its absences, names and overviews, whole

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
absence/audit-log-compliance-trail — An audit-log compliance trail
absence/otlp-receiver — An OTLP or pulse-class telemetry receiver
absence/pii-scrubbing-regime — A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
absence/deployment-environments — Deployment environments or hosting
absence/interactive-terminal-interface — An interactive terminal interface: prompt, TUI or pager

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
overview/2-telemetry-strategy — Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
absence/web-or-graphical-frontend — A web or graphical frontend

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
absence/in-process-metrics-pipeline — An in-process metrics pipeline, with no listener and no exporter

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
absence/database — A database
absence/grpc — gRPC of any kind

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
absence/otel-span-processing — An OTel SpanProcessor
absence/sentry-error-reporting — Sentry error reporting

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
absence/otel-trace-export — OTel trace export: no exporter

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
absence/on-call-paging — On-call or paging

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
absence/network-egress — Network egress of any kind
absence/telemetry-exporter-stage — A telemetry exporter stage

## service-identity · key of ## 3. Observability Harness Contract
absence/hosted-deployment — A hosted deployment or deployment environment; viola is local-only

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
absence/pulse-class-receiver — A pulse-class log receiver
