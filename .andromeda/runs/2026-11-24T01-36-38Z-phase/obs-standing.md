# obs · standing conventions

- S1 · every new log line goes through the single obs_event macro, with a closed event value and the required fields, corr passed explicitly, and no raw tracing or print macros · logging-stack
  contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
  decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
  decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
  decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
  decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
  decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
  decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
  decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
  decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
  decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
  decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
  decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
  decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
  decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
  decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
  decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
  decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
  decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
  decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
- S2 · every new span is a manual span with a static area.operation name, skips all of its arguments and declares an explicit field allow-list, with no err or ret options · 4-span-trace-coverage · span-trace-coverage
  decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
  decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
  decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
  decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
  decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
  decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
  decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
  decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
  decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
  decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
  decision/static-span-names · Static span names — Span names are static strings; instance names, dialog ids and paths never appear in a span name.
- S3 · every new process-exit or refusal detail is a closed code word, distinct per cause, with no path or pid in it · log-coverage
  contract/exit-1-detail/already-live · Exit 1 detail codes · already-live — already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/socket-dir-refused · Exit 1 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/squatted-name · Exit 1 detail codes · squatted-name — squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/stale-heartbeat · Exit 1 detail codes · stale-heartbeat — stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
  contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/run-start-failure-detail/already-live · run start process-exit · already-live — process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
  contract/run-start-failure-detail/pinned-hash-mismatch · run start process-exit · pinned-hash-mismatch — process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
  contract/run-start-failure-detail/squatted-name · run start process-exit · squatted-name — process-exit{subject:"self", exit_code:1, detail:"squatted-name"}, with no path or pid.
  contract/run-start-failure-detail/stale-heartbeat · run start process-exit · stale-heartbeat — process-exit{subject:"self", exit_code:1, detail:"stale-heartbeat"} for a stale holder, with no path or pid.
  decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
- S4 · every new integration or E2E test keeps its home under the scanned test-home root until the gates run, and its emitted lines are schema-checked · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S5 · every new fixture holds synthetic, scrubbed input only and is never read as telemetry · pii-scrubbing-compliance
  decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
- S6 · every new logged field is added under the default-deny allow-list, additive only, with no paths, content or secrets · 8-pii-scrubbing-compliance
  decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
  decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
  decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
  decision/no-allow-list-widening · Allow-list cannot be widened — No config key, flag or env var can widen the logged-field allow-list or disable redaction.
  decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
  decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
  decision/no-paths-in-run-spans · No endpoint, path or env values on start spans — Start spans never carry the endpoint name, the pinned binary path or environment values.
  decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
  decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
  decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
  decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
  decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
