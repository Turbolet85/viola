# obs-plan · the rows that bind 2026-11-14-status-line-interception-removed — picked 90 of 1049 (44 by every draw) · rule any · pin 1206

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/statusline-command-confined · Statusline command confined to diagnostics — The statusline_command string never appears outside diagnostics/.
decision/statusline-command-duration-exit-only · Statusline command observed by duration and exit — The user's statusline_command shell-out is observed by its duration and exit status only. · why: A user-written shell command.
decision/statusline-command-string-diagnostics-only · Statusline command string kept in diagnostics — The user's statusline_command string is never logged outside the diagnostics logs.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/hook-stdout/capture-arms · Hook stdout · --capture arms — The two hidden --capture arms write no log sink and print nothing, except --answers' probe decision body.
contract/hook-stdout/hook-event · Hook stdout · viola hook <event> — stdout carries only the decision body.
contract/hook-stdout/statusline · Hook stdout · viola hook statusline — stdout carries only the user's status-line command's output.
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/replayed-snapshot/live-fields · A snapshot.json rebuilt by replay · endpoint, pid, started_at, statusline_command — A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, hook statusline prints nothing, and run's collision check reads the heartbeat alone.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/hook-instance-check-before-obs-init · Hook instance check before logging starts — The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging. · except: The two --capture arms run ahead of the check.
decision/hook-latency-budget · Hook latency budget — Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append. · guard: run --perf hook rows under SPINE_DEADLINE · except: viola hook statusline is outside the budget.
decision/statusline-command-never-in-home-logs · Statusline command never in home-level logs — The statusline_command string never appears outside diagnostics/, reaching a detail file only as part of an error chain; its shell-out records only shell_exit_status and duration_ms. · why: The statusline command is classed High for integrity.
decision/statusline-command-only-in-diagnostics · Status-line command string kept to diagnostics — The statusline_command string is never logged outside diagnostics/.
decision/statusline-shell-out-instrumentation · Status-line shell-out instrumented by duration and exit — The `viola hook statusline` shell-out of the user's command is instrumented by its duration and exit status only. · why: A user-written shell command.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/hook-output/statusline-stdout · hook output streams · viola hook statusline stdout — Carries the user's status-line command output byte for byte; nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden --capture arm.
contract/hook-output/stdout · hook output streams · stdout — A hook writes only its decision body to stdout, and no telemetry reaches stdout.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/child-settings-never-logged · Child settings argument never logged — The claude child's --settings argument appears in no log line or span.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/no-endpoint-over-replayed-snapshot · No endpoint over a replayed snapshot — Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent, and the statusline runs no command. · guard: edge test E5
decision/statusline-command-never-logged · Statusline command never logged — The statusline command and its source path appear in no log line or span.
decision/statusline-deadline · Statusline shell-out deadline — The statusline shell-out of the user's command is bounded by STATUSLINE_DEADLINE, 5 s. · provisional
decision/statusline-stdout-passthrough · Statusline stdout passes through unlogged — The user's statusline stdout passes through unchanged and is never logged.
decision/unwrapped-hook-no-op · Unwrapped hook is a no-op — A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty. · guard: edge test E1, asserted by absence
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
contract/state-recovered-detail/snapshot-replayed · state-recovered detail · snapshot-replayed — A state-recovered line at WARN carries this detail, one line per snapshot read call.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/hidden-capture-arms-uninstrumented · Hidden hook capture arms uninstrumented — The hidden hook capture arms used by verify's probes read no VIOLA_* variable, do no obs init, emit no hook lines, keep stderr empty and always exit 0. · why: uninstrumented by design, as ratified by the founder
decision/hook-capture-arms-unlogged · Hidden capture arms write no lines — The hidden capture arms of viola hook (hook <event> --capture and hook statusline --capture) initialise no observability and write no lines.
decision/no-statusline-settings-in-telemetry · No settings or statusline command in telemetry — The child's --settings argument, the statusline source path and the statusline command appear in no log line or span.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/rejected-statusline-stdin-runs-nothing · Rejected statusline stdin runs nothing — Oversize or malformed statusline stdin prints nothing and runs no shell.
decision/statusline-settings-steps-unlogged · Statusline read and settings write unlogged — The start's statusline source read and settings override write carry no span and no log line. · why: Ratified as by design.
decision/statusline-silent-without-instance · Statusline arm silent without a valid instance — The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses: it returns exit 0 at once with no role file opened.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
contract/budget-span-attributes/statusline-shell-out · budget span attributes · statusline.shell_out — statusline.shell_out carries shell_exit_status and duration_ms.
contract/cli-exit-21/instance-dead · cli exit 21 process-exit · instance-dead — detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
contract/hook-decision-detail/socket-dir-refused · hook-decision line · socket-dir-refused — A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
contract/run-span-attributes/run-collision-check · viola run start span attributes · run.collision_check — outcome is free or already-live; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
contract/run-start-failure-detail/already-live · run start process-exit · already-live — process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
contract/span-kinds/client · Span kinds · CLIENT — The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side), statusline.shell_out and pty.spawn.
contract/span-kinds/server · Span kinds · SERVER — The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call, hook.handle and hook.statusline.
contract/state-recovered/snapshot-replayed · state-recovered line · snapshot-replayed — state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
contract/statusline-decision-detail/deadline · statusline hook-decision detail · deadline — The shell-out reaching its bound ends in hook-decision{detail:"deadline", deadline_hit:true}.
contract/statusline-decision-detail/malformed-json · statusline hook-decision detail · malformed-json — The statusline hook-decision carries detail malformed-json on that fail-open path.
contract/statusline-decision-detail/oversize-stdin · statusline hook-decision detail · oversize-stdin — The statusline hook-decision carries detail oversize-stdin on that fail-open path.
contract/statusline-log-fields/hook-decision · statusline hook log fields · hook-decision — hook-decision{hook_event:"statusline", duration_ms}, deadline_hit only when true.
contract/statusline-log-fields/hook-invoked · statusline hook log fields · hook-invoked — hook-invoked{hook_event:"statusline", corr:null}.
contract/statusline-log-fields/parse-rejected · statusline hook log fields · parse-rejected — parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
contract/statusline-log-fields/statusline-shell · statusline hook log fields · statusline-shell — process-start and process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}, only when the snapshot records a statusline_command.
contract/statusline-spans/span-chain · statusline spans · span chain — hook.statusline (SERVER) › statusline.shell_out (CLIENT).

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child and shell spawns carry a subject from a closed set.
contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, statusline-shell, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each of the eight verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
decision/statusline-command-deadline · Statusline command deadline — The user's statusline command is bounded at 5 s, reported as deadline. · provisional
contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, shell_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-subject/statusline-shell · diag-line subject enum · statusline-shell — Subject of the process-start and process-exit pair of a statusline shell spawn.
contract/hook-decision-detail/fail-open-codes · hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
contract/hook-decision-detail/statusline-codes · hook-decision detail · statusline codes — viola hook statusline writes only oversize-stdin, malformed-json and deadline; a refused instance check writes no line.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/statusline-command-out-of-home-logs · statusline_command out of home-level logs — statusline_command stays out of home-level logs.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body or, for hook statusline, the user's status-line output.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/direct-chrono-dependency · Direct chrono dependency for timestamps — The root binary takes chrono directly, without default features, to produce the millisecond UTC timestamp. · why: tracing-subscriber's chrono feature does not re-export chrono
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.

## log-format-json-schema · key of ## 3. Observability Harness Contract
contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## logging-stack · key of ## 3. Observability Harness Contract
contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The two hook --capture arms run ahead of the check.
decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
decision/tracing-subscriber-minimal-features · Minimal tracing-subscriber features — tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off. · why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.

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
