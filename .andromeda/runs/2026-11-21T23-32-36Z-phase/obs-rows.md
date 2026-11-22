# obs-plan · the rows that bind 2026-11-21-key-after-a-resize — picked 180 of 1034 (97 by every draw) · rule any · pin 3781

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14.
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
contract/send-log-line/from · Send log line · from — The from field is self-reported and logged as unauthenticated, never as an identity.
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/heartbeat-file/tick · Heartbeat file · tick — The heartbeat file is touched every 1 s as the liveness tick.
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/instance · Process log line · instance — A ViolaName or null.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
contract/process-log/target · Process log line · target — A required field.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors
decision/spans-not-serialized-in-v1 · Spans declared but not serialized in v1 — Spans are declared on the must-trace paths and every crate seam but are not written to output in v1. · why: they give the code its structure and are ready for the deferred OpenTelemetry bridge

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s. · provisional
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/channel-response-always-logged · channel-response always logged — Every dispatched channel request logs its channel-response line, an early return or an error included. · why: The line is emitted from a Drop guard, so an early return or error still logs it.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/portable-pty-log-target-off · portable-pty log target OFF — The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
decision/wheel-typed-mark-uninstrumented · Wheel typed mark and wakes count uninstrumented — The stdin typed mark and the held scheduled-wakes count are in-memory wrapper state that open no span and write no line of their own, and carry no key and no prompt text.
contract/confirm-window-close/clear · run.confirm_window close · /clear — On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
contract/confirm-window-close/compact · run.confirm_window close · /compact — On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
contract/confirm-window-close/context · run.confirm_window close · /context — On a verified CLI, closes on a new local-command-stdout tag in the session transcript within the fallback window, with no prompt submitted; on an unverified CLI, or before the session transcript exists, the send reads unconfirmable.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/confirmed-send-spans/span-chain · confirmed send spans · span chain — send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
contract/send-log-fields/send-confirmed · confirmed send log fields · send-confirmed — send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
contract/send-refused-detail/input-not-ready · wrapper send-refused detail · input-not-ready — send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
contract/send-refused-detail/no-prompt-submitted · wrapper send-refused detail · no-prompt-submitted — Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
contract/send-span-attributes/run-readiness-gate · confirmed send span attributes · run.readiness_gate — outcome (ready or input-not-ready) and vt100_panicked (bool).
contract/send-span-attributes/text-bytes · confirmed send span attributes · text_bytes — text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
contract/send-span-attributes/window-ms · confirmed send span attributes · run.confirm_window window_ms — window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
contract/span-kinds/internal · Span kinds · INTERNAL — The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
decision/manual-readiness-gate-span · Manual span on the readiness gate — The readiness gate of viola run carries a manual span, run.readiness_gate. · why: vt100 has no tracing.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
contract/claude-child-process-log/handle-wait-exit · run process log, claude-child lines · process-start and process-exit of claude-child — process-start is logged once the child is spawned and process-exit once the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/span-kinds/client · Span kinds · CLIENT — The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side) and pty.spawn.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/pty-seam-log/paste · PTY seam log events · paste — A paste is logged with text_bytes only.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/third-party-crate-logs-off · Third-party crate logs off — The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets. · why: Their failures are captured at the viola seam as viola events.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/pty-seam-log/kill-fallback · PTY seam log events · kill-fallback — An exit through the kill fallback is logged with exit_source:"kill-fallback".
contract/pty-seam-log/spawn · PTY seam log events · spawn — A spawn gives process-start{subject:"claude-child"}.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/send-refused-detail/empty-text · send-refused detail · empty-text — Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/vt100-panics-contained · vt100 panics contained — vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/env-vars-not-config-channel · Env vars are not a configuration channel — Backtrace capture is not gated by RUST_BACKTRACE or any env var. · why: env vars are not a configuration channel
decision/vt100-feed-panic-contained · vt100 feed panic contained — A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate
decision/os-gated-test-census · OS-gated test census — Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named. · guard: contract_os_gated_tests
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/nonzero-count-blocks-chunk · Nonzero error count blocks the chunk — Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk. · why: the tests quarantine rule
decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/rerun-no-substitute · Re-run is no substitute for diagnostics — A re-run is not a substitute for the failing job's diagnostics. · why: CI runs under retries = 0.
decision/scrub-before-bytes-leave-process · Scrubbing before any byte leaves the process — Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files. · guard: the secret-scan test

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.
contract/send-line/client-side-refusal · the send line · client-side send-refused — The caller-side send-refused{side:"client"} has corr null.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.

## trace-context-propagation · key of ## 3. Observability Harness Contract
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
contract/send-line/conn-rpc-id · the send line · conn and rpc_id — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.

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
