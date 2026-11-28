# obs · standing conventions

- S1 · each new diagnostics line is written through the single obs_event macro with a closed event value, explicit corr and instance, and default-deny fields · 11-obs-anti-patterns-never-do-these
  contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
  decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
- S2 · each new diagnostics line carries only the fixed line fields and is additive to the log schema, never renaming a frozen field · log-format-json-schema
  contract/log-levels/trace · log levels · trace — TRACE is never emitted; the level exists only in the tests' level enum.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
  contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
  contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
  contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log-line/target · the process log line · target — The target is always present.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/target · Process log line · target — A required field.
  contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
  decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
  decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
  decision/no-ci-attributes-in-log-lines · No CI resource attributes in log lines — deployment.environment, ci.run.id and git.commit.sha are never injected into viola log lines; CI identity lives in artifact names and GitHub run metadata. · why: env vars are not a configuration channel
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
  decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
- S3 · each new integration or E2E test that runs a process has its emitted lines schema-checked and its home under the one integration-test root, kept until the gates run · 9-ci-integration · ci-integration
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S4 · each new log line names no path, pid, typed content or secret in a home-level file; content-bearing records go to the instance detail file only · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
  decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
  decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
  decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
  decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
  decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
  decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
  decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
  decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
  decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
  decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.
  decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
  decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
  decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
- S5 · each new obs code path is mutation-tested like product code, and the zero-panics and schema gates still read every role file · ci-integration
  decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
  decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate
  decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.
- S6 · each new test waits on log lines or state by condition, never by sleep · 10-slo-invariants-telemetry-budgets
  decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
