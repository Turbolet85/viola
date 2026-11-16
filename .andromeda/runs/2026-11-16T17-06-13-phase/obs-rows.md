# obs-plan · the rows that bind 2026-11-16-granular-api-error-cause — picked 134 of 1022 (75 by every draw) · rule any · pin 2030

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/assistant-message-never-logged · Assistant message never logged on readback — On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
decision/channel-lines-carry-no-bodies · Channel log lines carry no bodies — IPC channel log lines never carry `params` or result bodies.
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream.
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
decision/hook-stderr-forbidden · No stderr from hooks — viola hook writes nothing to stderr.
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body.
decision/mcp-rejections-codes-only · MCP rejections carry codes only — MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/viola-core-emits-no-telemetry · viola-core emits no telemetry — The viola-core crate supplies log-field values but emits nothing and is not instrumented. · why: A pure type crate with no I/O.
contract/viola-mcp-tools/wait · viola-mcp stdio server tools · wait — The viola-mcp stdio server offers a wait tool.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/hook-log-write-failure-swallowed · Hook log write failure swallowed — A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
contract/agent-run-status/state-last-error · agent-run status · state and last_error — The aggregate health snapshot gives `state` (`ready`, `degraded` or `down`) and `last_error` (null, or `list-exit-<code>` built from the list exit code).
contract/harness-logs/merge · harness logs command · merged sources — Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`).
contract/harness-logs/torn-line · harness logs command · torn lines — A torn line is emitted with its `src` and `offset` and `"torn":true`, never dropped.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/hook-latency-budget · Hook latency budget — Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append. · guard: run --perf hook rows under SPINE_DEADLINE
decision/hook-log-failure-swallowed · Hook log write failure swallowed — A failed log write in a hook is swallowed and the hook still exits 0.
decision/hook-never-writes-stderr · Hook never writes stderr — A hook process never writes to stderr.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/hook-fail-open/exit · Hook fail-open · exit — A hook that fails open exits 0 and leaves stderr empty.
contract/hook-stdout/hook-event · Hook stdout · viola hook <event> — stdout carries only the decision body.
contract/wait-log-line/required-fields · Wait log line · required fields — A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
contract/wait-path-log-events/channel-pair · Wait path log events · channel pair — A wait logs channel-request and channel-response, the response carrying the outcome.
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/home-level-mcp-log-no-user-content · Home-level MCP log holds no user content — An mcp.ndjson at home level never carries user content.
decision/hook-silent-without-trusted-instance · Hook silent without a trusted instance — A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/wait-log-line/outcome · Wait log line · outcome — The wait outcome is the woken kind, timed-out or instance-unreachable.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
decision/metrics-derived-by-jq · Metrics derived by jq, no instruments — Metrics have no in-process instruments and no exporter; they are derived by jq over the ndjson logs.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/hook-silent-without-instance · Hook silent without an instance — A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors
decision/metrics-derived-from-ndjson · Metrics derived from logs, no instruments — There are no in-process metric instruments and no exporter; metrics are derived by jq over the ndjson logs, and perf gates run on hyperfine JSON exports.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/hook-output/stderr · hook output streams · stderr — A hook writes nothing to stderr.
contract/hook-output/stdout · hook output streams · stdout — A hook writes only its decision body to stdout, and no telemetry reaches stdout.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/channel-response-always-logged · channel-response always logged — Every dispatched channel request logs its channel-response line, an early return or an error included. · why: The line is emitted from a Drop guard, so an early return or error still logs it.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/reader-never-heals-torn-lines · Reader never heals torn lines — A reader of an append log neither heals a torn line nor logs one; only the appender heals.
decision/uninstrumented-components · Components left uninstrumented — viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented. · why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
decision/unwrapped-hook-no-op · Unwrapped hook is a no-op — A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty. · guard: edge test E1, asserted by absence
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/hook-always-exits-zero · viola hook keeps exit 0 — viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
contract/wait-log-fields/channel-request · wait and last log fields · channel-request — channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
contract/wait-log-fields/channel-response-outcome · wait and last log fields · channel-response outcome — For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
decision/hook-refused-instance-silent · Refused instance makes the hook silent — On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
decision/no-params-in-channel-lines · No params or result bodies in channel lines — channel-request and channel-response lines never carry params or result bodies.
contract/channel-lines/fields · channel-request and channel-response lines · fields — channel-request carries method, corr, conn and sender; channel-response carries method, corr, conn, result_class and error_code, plus detail on a -32602.
contract/mcp-spans/mcp-tool-call · viola mcp spans · mcp.tool_call — viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/ipc-invocation-log/fields · IPC invocation log lines · fields — Channel-request and channel-response lines carry method, conn (srv_conn when the client sent none) and the corr of a frame with an id; a request line adds sender, a response line result class and an error's error code but never sender.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-line-fields/channel-response · diagnostic line additive fields · channel-response — method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/mcp-rejects-codes-only · MCP rejects are codes only — MCP rejects are isError codes only, and a logged reject's line holds codes only.
decision/scrubbing-at-source · Scrubbing at the source — Error scrubbing sits at the source, not in a reporter's before_send hook. · why: No reporter exists.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/peer-replies-screened-as-code-words · Peer replies screened as code words — A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14. · why: by design

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/no-regex-scrubbers · No regex scrubbers — Sensitive data is never scrubbed by regular expressions. · why: viola's sensitive data is known by type

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/hook-obs-overhead-budget · Hook observability overhead budget — Obs overhead on hook, logger init being one open plus appends, is held inside the same max < 1.0 s perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home. · guard: gate --require perf
decision/spine-hook-budget · Spine hook budget — The spine hooks and pre-tool-use on an unverified CLI complete under the tests-owned spine deadline, provisionally 1.0 s. · guard: gate --require perf · provisional
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/spine-hook-perf-gate · Spine hook perf gate — Spine hooks (session-start, user-prompt-submit, stop) and pre-tool-use on an unverified CLI keep their hyperfine max under the spine deadline, timed over every check a frame-writing hook makes before its first frame. · guard: gate --require perf

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/derived-metrics-by-closed-enums · Derived metrics grouped by closed enums — Derived metrics are grouped by the closed enums only, never by unbounded keys such as path, corr, conn, pid or instance.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh

## log-format-json-schema · key of ## 3. Observability Harness Contract
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-internal-error-fallback · No stderr fallback for subscriber internal errors — The diagnostics subscriber never reports its own internal errors by printing to stderr. · why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
contract/agent-run-logs/torn-line · agent-run logs output · torn line — A torn line appears as {"torn":true,"offset":n}, and readers skip it rather than fail.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/hook-refusal-exits-zero · Refused hook fails open — A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
contract/diagnostics-events/config-parse-rejected · diagnostics events · parse-rejected config-json — Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.

## product-mode-normal-vs-inverted-self-observing · key of ## 3. Observability Harness Contract
decision/normal-observability-mode · Normal observability mode — viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector. · why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/agent-run-logs/events-record · agent-run logs output · events record — Each events.ndjson line is merged as {"src":"events","instance","offset","record"}.

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
