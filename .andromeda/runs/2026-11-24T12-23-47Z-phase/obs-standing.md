# obs · standing conventions

- S1 · each new log line or event is emitted through the obs_event macro with a closed event value, corr where correlated, and carries no content, path or secret · 11-obs-anti-patterns-never-do-these
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
  decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
  decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
  decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
  decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
  decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
  decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
  decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
  decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
  decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
  decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
  decision/skipped-symlink-never-logged · Skipped symlink never named in logs — The entry name and link target of a skipped symlink are never logged.
  decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
  decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
- S2 · each new log field is additive to the diagnostics line schema, named under the additive naming rule and admitted to the default-deny allow-list · 6-log-coverage · log-coverage
- S3 · each new span declares a static name, skips all arguments and lists its allowed fields explicitly · 4-span-trace-coverage · span-trace-coverage
  decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
  decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
  decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
- S4 · each new integration or end-to-end test keeps its home under the scanned test root until the gates run, and its emitted lines are schema-checked and counted by the panic gate · 9-ci-integration · ci-integration
- S5 · each new fixture or test input is synthetic, never recorded from a real session or the operator's real home · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
  decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
  decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data
  decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
  decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
