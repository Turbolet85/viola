## 2026-10-31-sanitised-error-surfaces — v1-47's secret-scan reading, provisional
**Section:** §9 CI Integration (ci-integration.md: a new values group after step 1's)
**Change:**
- `v1-47`'s secret-scan reading is stated: after each job's end-to-end run, not after every test, with `AGENT_RUN_KEEP_HOMES` `1` on the job (`.github/workflows/ci.yml`), the scan `if: always()` and the `viola-chaos-*` home out (step 1).
- The reading stands OPEN and PROVISIONAL, its owner the open decision on `v1-47`'s after-every-test reading.
**Why:** `v1-47`'s "after every end-to-end test" was concretized to after each job's end-to-end run, measured on the job, and accepted by the overseer's word as the founder's delegate, 2026-10-31, PROVISIONAL, owed the founder's own. The `viola-chaos-*` home stays out by the founder's ruling of 2026-10-04.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the mcp role writes no self process-exit
**Section:** §2 Telemetry Strategy (telemetry-strategy.md: the Structured Logs and Errors rows) · §7 Error Capture & Reporting (obs-plan.md: the `mcp` per-role branch; error-capture-reporting.md: the catch-site list, the stdin line)
**Change:**
- §7, `mcp`'s main-thread panic: was `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` then exit 1; now no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only) and exit 1, nothing on stdout.
- The catch-site list gains: the `mcp` role writes no self `process-exit` line, `internal_error_exit_line` (`src/obs.rs`) writing for `Run` and `Cli` only.
- A `viola mcp` stdin line past `MAX_FRAME` once `initialize` has completed: was `process-exit` 0; now process exit 0 and no self `process-exit` line.
- The Structured Logs row's process lifecycle and the Errors row's `process-exit` `detail` codes each carve out "the `mcp` role writes no self `process-exit`, Section 7".
**Why:** measured: `internal_error_exit_line` writes for `Run` and `Cli` only, and a grep of `ObsEvent::ProcessExit` shows no self `process-exit` written on the `mcp` role. The exit-0 half rests on that grep: the live case asserts one `parse-rejected` and does not read a self line there. The catch-site item, escalated as a contradiction of the stdin line's `process-exit` 0, was resolved apply by the overseer as the founder's delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — where -32603 is proved
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the catch-site list)
**Change:**
- The list gains: no failpoint or counted panic drives a `-32603` in a scanned home; the unit layer holds that fault (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
**Why:** the chunk added no failpoint or counted panic, so no scanned home meets that fault; the two unit cases are its proof.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — viola mcp's exit at an oversize line before initialize
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the `mcp` Exporter) · §7 Error Capture & Reporting (error-capture-reporting.md: the stdin line and the item after it)
**Change:**
- §7's stdin line, the cut before `initialize`: was `ServeError::Initialize` (exit 11); now process exit 1 (`ServeError::Initialize`; `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1), 11 the code of the unit child alone (`serve_child_entry`).
- §7 gains: the same line before `initialize` leaves both streams empty and its one record reads `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, witnessed by `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`.
- The Exporter gains: such a line before `initialize` exits the process 1 with both streams empty and that same one record; 11 is the unit child's own code, not `viola mcp`'s exit.
**Why:** exit 11 as `viola mcp`'s exit was disproved by measurement: the live case reads exit 1, both streams empty and one line, and 11 is read only at the unit child. The new §7 item, escalated as a contradiction of the stdin line's exit 11, was resolved apply by the overseer as the founder's delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — a peer's reply screened by sanitised_fault and is_code_word
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the `mcp` notes) · §4 Span / Trace Coverage (span-trace-coverage.md: a new item on the clients) · §7 Error Capture & Reporting (error-capture-reporting.md: a new values group)
**Change:**
- §7 gains `viola_channel::sanitised_fault`, the one reader of a peer's fault, for `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`). No integer `code` reads nothing; each of the five known codes reads its fixed `message`. `-32602` with an integer `data.supported` and a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+` reads the unsupported-version body with those two values; `data.reason` exactly `release-from-driver` reads that body; any other `-32602` reads `invalid params`, `data` null; every other known code reads `data` null. A code outside the five reads `message` `wrapper fault`, `data` null.
- §7 gains the screen by `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, a non-string `detail` is no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>` by design, its exit `unknown`'s 14 by `reason_of`.
- §1's `mcp` notes gain the same screen on `of_reply`, `wrapper-fault`'s `detail` the closed object `sanitised_fault` reads.
- §4 gains: the screen adds no span and no line.
**Why:** `sanitised_fault` replaces the two private `fault_detail` copies, so a peer's `message`, `data`, `refusal` or `detail` reaches no document or text; a code word outside the closed list stays visible by design.
**Kept:** §8's PII table gains no row for a peer's reply: the proposed row, classing a peer's `refusal` and `detail` as untrusted under the Medium rule beside the table's Low row for refusal details, was escalated as a contradiction and resolved reject by the overseer as the founder's delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the nine thiserror derives, CoreError retired
**Section:** §7 Error Capture & Reporting (obs-plan.md: Platform pick; error-capture-reporting.md: the new values group)
**Change:**
- Platform pick's thiserror list: was `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`; now `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError`.
- The detail gains the count: the product's `thiserror::Error` derives read 9, those nine, as measured at this chunk by a grep of `thiserror::Error` over `src` and `crates`; `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed.
**Why:** `CoreError` never existed in the code (a grep reads 0 hits). `viola_agent_claude::Refusal` folded into `AgentError`, moving the derives from 10 to 9, and the census found `PinError`, `StampsReadError` and `Refused` unnamed.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the exit-2 paths of answer and send
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the new values group)
**Change:**
- The detail gains: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.
**Why:** the claim that an oversize `viola answer` exits non-zero with one `--json` error document was disproved by measurement: those paths write the fixed line and no document.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — channel-response's detail closed to three words
**Section:** §6 Log Coverage (log-coverage.md: the `channel-response` row) · §1 Obs Scope Summary (obs-scope-summary.md: the send path's fields, the wheel path's spans, the channel scrub rule) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the protocol-codes row) · §4 Span / Trace Coverage (span-trace-coverage.md: the channel row and a new item, the send, answer and release scenarios, the `link` / `unlink` trace)
**Change:**
- §6: `detail`, unqualified, now reads `unsupported-version` / `invalid-params` / `release-from-driver`, closed by `schemas/diag-line.v1.json`: the three `-32602` faults, on the server line and the client line alike, absent on every other `channel-response` line.
- §1: the send path's `detail` list adds `invalid-params` and `unsupported-version` on a `channel-response` answering `-32602`; on the wheel path `release-from-driver` is also that `detail` value and does not replace its event; the scrub rule's "only" list admits the closed word.
- §8: the row names the three words, naming no param or value, asserted by `answer_invalid_params_names_its_cause_and_no_param`, `reply_from_logs_the_cause_of_each_invalid_params` and `link_verbs_log_the_channel_pair_and_no_names`.
- §4, the writer: `ResponseLine`'s `Drop` from `ProtocolError::cause` (`None` for the other four faults), on both lines; the client's `fault_of` reads the reply's `data` (`supported`, `reason` or neither).
- §4, send: was `channel-response{corr:<id>, conn, result_class, refusal, detail, duration_ms}`; now `detail` on a `-32602` only, absent on a refusal's line, whose own `detail` stays on `send-refused`.
- §4, answer: was `detail:"unknown-dialog"` on the line; now no `detail` there, `unknown-dialog` the refusal's own.
- §4, release: the string-`from` reply carries `detail:"release-from-driver"`, a plain `-32602` `detail:"invalid-params"`, each on both sides at WARN.
- §4, `link` / `unlink`: a `-32602` also carries `detail:"invalid-params"`; the pair holds no name and no param.
**Why:** the schema closes the field from any string to this enum, with the level unchanged, the wire form unchanged, `v` still 1 and the seven fault bodies unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — v_rejected counts version rejections alone
**Section:** §5 Metric Coverage (metric-coverage.md: the higher-`v` row; obs-plan.md: per-surface metrics) · §1 Obs Scope Summary (obs-scope-summary.md: the higher-`v` signal)
**Change:**
- metric-coverage.md: was `viola.compat.v_rejected` ← `channel-response{error_code:-32602}`, over-counting until the line tells the causes apart; now ← `channel-response{error_code:-32602, detail:"unsupported-version"}`, version rejections alone: invalid params on `send` and a bad `link` / `unlink` name or pair read `invalid-params`, and `release` carrying `from` reads `release-from-driver`.
- obs-plan.md: was the over-count line; now the metric counts version rejections alone, the other `-32602` causes each carrying another `detail` word.
- §1: was -32602, not a higher-`v` count alone; now `detail:"unsupported-version"`, the code also carrying `detail:"invalid-params"` (`send`'s invalid params; a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`) and `detail:"release-from-driver"` (`release` carrying `from`), so the count reads `detail` and not the code.
**Why:** the line now tells the three `-32602` causes apart, the end the caveat named, and `fault_of` reads `data` instead of labelling any `-32602` unsupported-version. The metric-coverage.md and §1 lines, each escalated as a contradiction of its own standing line, were resolved apply by the overseer as the founder's delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error-body census, check and live witnesses
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the verification list)
**Change:**
- Census: `tests/contract_error_bodies.rs` (`andromeda:walks-tree`) holds the error bodies of CLI `--json`, MCP results and the seven channel faults in tables: `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each body to a writer and a witness that stand, `product_error_writers_are_each_named` every product error writer to a name, and `thiserror_messages_interpolate_nothing` `thiserror` messages free of interpolation.
- Check: `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes. No check runs after every test: the three live witnesses, the enumeration and the census read it instead. It stands beside the secret scan and leaves the scan as it is.
- Live witnesses: `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`), `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`), `mcp_tools_error_results_hold_no_path_or_upstream_text` (`crates/viola-e2e/tests/mcp_tools.rs`).
**Why:** the chunk built these and no §8 item held them. test-plan §6's after-every-test check was measured absent before the chunk and was not built; the witnesses, the enumeration and the census replace it.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
