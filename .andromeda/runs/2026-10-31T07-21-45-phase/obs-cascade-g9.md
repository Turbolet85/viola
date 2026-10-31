# obs · cascade read · group 9 of 9

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10) The scan the chunk's never-log leg re-reads must hold the `Cookie` header pattern and run over every `diagnostics/` file.
  > | `Cookie` header | Critical | never-log | secret-scan test (the case-insensitive `cookie:` header form) over all `diagnostics/` |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11) The witness's planted stripped `CLAUDE*` value must appear on no surface and in no sink, detail files included, and the strip stays recorded by names and count only.
  > | `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` values | Critical | never-log anywhere, including detail files | R8 strip logs `env_stripped_count` + `env_stripped_known` (stripped names) + `env_kept` (kept names), names only, values never read (D-34); env maps never passed to a span or event; secret-scan test with one canary per identity-floor name plus an unknown name |
  (.andromeda/obs-plan.md:734)
  > NEVER log `CLAUDE*` values or a `Cookie` header, at any level, in any file, detail files included.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) The content leg of `v1-47` must read prompt and assistant payloads as reaching only an instance detail file, and no error body on the three surfaces may hold them.
  > | Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |
  (.andromeda/obs-plan.md:555)
  > prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance `detail-<process>.ndjson` inside a chain or drift report; `statusline_command` stays out of home-level logs.
  (.andromeda/obs-plan.md:568)
  > **Sink separation:** content-bearing records go only to instance-scoped `detail-<process>.ndjson`.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14) The same leg must hold for tool input and dialog answers, and an `answer` or dialog error body stays codes only.
  > | Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report | `#[redact]` on `HookDialogParams.data`, `AnswerParams.response`; codes-only dialog events (`dialog_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15) A drift report must meet every surface as a fixed `Display` and reach only the `drift_report` field of a detail file, which is both the scope's `thiserror` work and the drift leg of `v1-47`.
  > | serde_path_to_error drift reports | High | instance-scoped `detail-<process>.ndjson` only | Replaced by fixed `Display` at the seam; full report is a `drift_report` field in the detail file only |
  (.andromeda/obs-plan.md:567)
  > **Error layer:** fixed thiserror `Display`, and serde-sourced chains mapped to fixed messages at dispatch.
  (.andromeda/obs-plan.md:724)
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:16) The status-line row's cap-driving case must leave `statusline_command` out of every home-level file, with the shell-out recording exit status and duration only.
  > | `statusline_command` | High (integrity) | never in home-level logs; detail file only if part of an error chain | `statusline.shell_out` span records `shell_exit_status` + `duration_ms` only |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) Every error body on the three surfaces must be free of absolute paths and internal type names, and paths may appear only in instance detail files.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) No error body may name `endpoint`, `pinned_bin` or a path-valued config value, and the `config.json` readers' cap-driving cases must hold them to that rule.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
  (.andromeda/obs-plan.md:557)
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged; `claude agents --json` rows are never logged verbatim, only `parse-rejected` counts; low-class metadata is logged as typed fields, never inside `detail` codes.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The code-shaped `detail` of a CLI or MCP error must hold no pid, session id, time or budget value.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
  (.andromeda/obs-plan.md:735)
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20) The child pipe drain's cap-driving case must leave the rows unlogged verbatim, with only the `parse-rejected` count recording the failure.
  > | `claude agents --json` rows / unwrapped session ids | Low, untrusted | OK structured; C0/C1 escaped in human terminal output | JSON serialization escapes C0 controls in files; rows never logged verbatim, only `parse-rejected` counts |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) Any way the `channel-response` line tells a bad `link` or `unlink` name or pair apart from another `-32602` cause must keep `driver` and `driven` off the line, and its test must keep holding.
  > | `link` / `unlink` pair names (`driver`, `driven`, each a `ViolaName`) | Low | OK in the contract payload: the wrapper's `link` / `unlink` event lines in `events.ndjson` (`data: {driver, driven}`, `source: wrapper`); never in a process-log line | the served methods open no span and write no process-log line of their own, and the wrapper's `channel-request` / `channel-response` pair logs `method`, never the `driver` / `driven` params; the verbs' result lines print their own parsed `ViolaName` arguments, never a name read back from the reply or the log; asserted by `link_verbs_log_the_channel_pair_and_no_names` (`tests/cli_links.rs`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) Every protocol code and refusal detail in the enumeration, and any value that tells the `-32602` causes apart, must come from a closed kebab-case enum.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
  (.andromeda/obs-plan.md:643)
  > **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:29) The secret-scan leg of `v1-47` must be read against this statement of its patterns, its coverage and its place before upload.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
  (.andromeda/obs-plan.md:759)
  > NEVER skip scrubbing before any byte leaves the process. In v1 the only destination is local files, and the secret-scan test proves the floor.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32) A witness's content canary may reach only detail files, so a planted value in a home-level file reads as a real leak.
  > The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:33) The new pre-`initialize` over-long line case is this case's sibling, and it owes the same fresh home and the same canary assertion.
  > The `mcp_tools` E2E case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` runs on its own fresh home, writes the canary inside its over-long stdin line, and asserts the canary in no home-level file of that home.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:36) The scan's reading of `target/agent-run/*` decides how a witness's planted values behave in that input, and its Critical-class patterns stay applied there.
  > The scan applies the Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff`, and the canary rule exempts those files as it exempts `detail-*.ndjson`.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:39) The CLI's server-verification refusal is an error body in the enumeration, and it must keep its no-pid, no-path form and its `digit_runs` witness.
  > A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40) The `instance-unreachable` body named here is the exact `--json` and MCP form the enumeration must name and the per-surface witnesses must hold to.
  > A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows. The `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and write no line (`binary(handlers)`).
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:41) The owner-readable half of the detail-file leg of `v1-47` must be read against this strict-modes check on the content sink, and a refused open must drop the line.
  > The content sink is opened only behind its strict-modes check: `src/obs.rs`'s `check_diagnostics` runs `viola_state::strict::check_diagnostics` on the `diagnostics/` folder and on the role or `detail-<process>.ndjson` file, each that exists, in `open_role_file` and `write_detail`, before `ensure_private_dir` narrows anything. A symlink, a folder another user may read (Unix: a group or other bit; Windows: a read grant to another SID) or a file another user may read (Unix: a group or other bit; Windows: the write rule) refuses the open as a failed open: the writer ends at `std::io::sink` and the detail line is dropped, so High-class content is never written through a symlink or into a folder another user may read (unit cases in `src/obs.rs`).
  (.andromeda/obs-plan.md:667)
  > A secret-scan hit, or a `diagnostics/` file that is not 0600 (or not DACL-protected on Windows).
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:48) A scan hit on a witness's planted value must be reported by location and class only, with the matched bytes never in the report.
  > Instead, the scan step writes a hit report to `target/secret-scan/` (`file`, line, byte offset and pattern class, never the matched bytes), and the separate §9 step 5 uploads that directory as `secret-scan-${{ matrix.os }}`.
  (.andromeda/obs-plan.md:572)
  > The scan-failure hit report names file, line, byte offset and pattern class, never the matched bytes.
- (.andromeda/obs-plan-detail/ci-integration.md:10) The home diagnostics glob is the scan's first input and the scan gates its upload, so a witness's home under that glob is part of what the floor's proof reads.
  > | `diagnostics/*.ndjson` + `instances/*/diagnostics/detail-*.ndjson` from `target/e2e-home/**` | every E2E job, `if: always() && steps.secret-scan.outcome == 'success'`. A bare `always()` would also upload after a failed scan |
- (.andromeda/obs-plan-detail/ci-integration.md:11) The harness capture must run before the scan so its files stay among the inputs the leg re-reads.
  > | `agent-run logs` merged ndjson + `agent-run status` JSON, captured to `target/agent-run/{logs.ndjson,status.json}` by an `if: failure()` step that runs **before** the scan, so the scan covers them |
- (.andromeda/obs-plan-detail/ci-integration.md:12) The JUnit file is a scan input holding failing tests' captured output, so a witness's failure output must not echo a planted value it did not find as a leak.
  > | cargo-nextest 0.9.146 JUnit XML (`[profile.ci.junit] path = "junit.xml"` in `.config/nextest.toml` → `target/nextest/ci/junit.xml`) | every test job, `if: always() && steps.secret-scan.outcome == 'success'`.
- (.andromeda/obs-plan-detail/ci-integration.md:23) A change to the `channel-response` line that tells the `-32602` causes apart must be witnessed by this in-memory parse against the schema.
  > | Unit tests | In-memory `Vec<u8>` writer tests parse emitted lines with serde_json against `schemas/diag-line.v1.json` (tests-owned bodies) | nextest JUnit |
- (.andromeda/obs-plan-detail/ci-integration.md:41) G4 must validate real output for both schemas, which is the check that witnesses the `channel-response` line and the detail-file leg of `v1-47`.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
  (.andromeda/obs-plan-detail/ci-integration.md:42)
  > It exits non-zero on any failing line and prints `file`, line number and the failing schema keyword, never the line's content (detail lines carry the canary).
  (.andromeda/obs-plan.md:577)
  > **Detail-file scope:** `schemas/diag-line.v1.json` validates home-level lines only. `detail-<process>.ndjson` lines are validated by a sibling `schemas/diag-detail.v1.json`, with a top-level `unevaluatedProperties: false`.
  (.andromeda/obs-plan.md:580)
  > exactly these content fields: `panic_payload` and `backtrace` (panic, §7), `chain` (§7 layer 3) and `drift_report` (§8 table).
  (.andromeda/obs-plan.md:668)
  > A schema-conformance failure: unknown `event` value, missing required field, or a field outside the catalog.
- (.andromeda/obs-plan-detail/ci-integration.md:59) "A secret scan after every end-to-end test" must be read against this order, its one home outside the scan and the kept homes that give the scan something to read.
  > 1. Integration and E2E tests (every viola home under `target/e2e-home/` but the one `viola-chaos-*` home of `tests/chaos_feed_panic.rs`, outside G2, G4 and the secret scan by the founder's ruling of 2026-10-04), then G2 and G4, **both** `if: always()`.
  (.andromeda/obs-plan-detail/ci-integration.md:59)
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).
  (.andromeda/obs-plan.md:621)
  > 3. Secret scan, `id: secret-scan`, `if: always()`, over `target/e2e-home/**/diagnostics/*.ndjson` and `target/agent-run/*` except `run --mutants`' `target/agent-run/chunk.diff` (skipped by exact path and excluded from the step-4 `harness-<os>` upload), and also `target/nextest/ci/junit.xml`, because nextest stores failing tests' captured output in it.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:9) A witness that drives a handler fault must not do it by a panic that G2 counts, since only the `fake-agent` seam's panic is exempt from the count.
  > **Counting rule:** "exactly one", the `viola.panic` counter and gate G2 (§9) count home-level role files only. G2 alone also leaves out the `fake-agent` seam's deliberate panic (a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`, §9): that line is still written exactly once — the invariant holds for it — and only its count is exempt.
  (.andromeda/obs-plan.md:654)
  > **Definition:** zero tolerance on three counts: 0 `event:"panic"` lines in any CI artifact (as G2 counts them: the `fake-agent` seam's deliberate panic at exactly `src/cmd/hook/seam.rs:<digits>` excepted, §9), 0 secret-scan hits, 0 schema-conformance failures.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17) The enumeration of CLI and MCP bodies must agree with these refused-check outcomes on `list`, `plugin install`, `link`, `unlink` and `mcp`; the `revive` start-refusal documents stay with the exit-cause chunk.
  > it opens no role file and writes no line (revive: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1; statusline: exit 0 and nothing printed; list and plugin install: exit 21, the `--json` error document or the stderr pair; link and unlink: exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document; mcp: exit 1 with nothing on stdout or stderr), so a panic there writes none, by §7's rule; no forced-panic case reads it.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18) The frame verbs' refused check must print only the `instance-unreachable` form under `--json`, the body the CLI witness and the enumeration name.
  > 8. a refused strict-modes check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or under `--json` the `instance-unreachable` document), on `viola verify`'s home tree (exit 1, the home pair) or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair): it opens no role file and writes no line, so a panic there writes none, by §7's rule.
- (.andromeda/obs-plan.md:684) The way the `channel-response` line tells its causes apart must be a closed value, never free text.
  > NEVER let the signal pyramid collapse into free text. Every line carries the closed `event` enum and `corr`.
- (.andromeda/obs-plan.md:700) Correcting `viola.compat.v_rejected` must group its derivation by a closed enum, never by an unbounded key.
  > NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.
- (.andromeda/obs-plan.md:741) The secret-scan leg of `v1-47` must find the scan running after every end-to-end test whatever its outcome, with uploads gated on its success.
  > NEVER lose the failing job's `diagnostics/`. G2, G4 and the secret scan run `if: always()`, and the uploads run `if: always()` / `failure()` combined with `steps.secret-scan.outcome == 'success'`, never bare. On a failed scan the hit report is uploaded instead (§9 Step order). A re-run is not a substitute under `retries = 0`.
  (.andromeda/obs-plan.md:739)
  > NEVER upload `diagnostics/` artifacts before the secret-scan step passes.
- (.andromeda/obs-plan.md:698) The correction of `viola.compat.v_rejected`'s over-count must remain a `jq` derivation over counts of `event`/`detail` values in the ndjson, with no in-process instrument and no exporter added to tell the causes apart.
  > NEVER create in-process metrics instruments (`opentelemetry` meters, `metrics` 0.24.6). There is no reader, exporter or listener to flush them.

## Bindings
- **PII floor ↔ CI scan**: the planted stripped value and the content canary are proved absent only by a scan that covers the home glob, the harness capture and the JUnit file, so a witness must keep planted values out of those inputs unless a real leak put them there, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11) and (.andromeda/obs-plan-detail/ci-integration.md:12).
- **PII drift and content rows ↔ CI G4**: the `chain` and `drift_report` content that may reach a detail file is what G4 validates under `schemas/diag-detail.v1.json`, so the detail-file leg of `v1-47` stands on both, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15) and (.andromeda/obs-plan-detail/ci-integration.md:41).
- **PII refusal bodies ↔ SLO exemptions**: the strict-modes refusal and exemption 8 state one body, the `instance-unreachable` document with no line, and the enumeration must name it the same way in both, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40) and (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18).
- **PII link names ↔ the metric over-count**: telling the `-32602` causes apart must end the `viola.compat.v_rejected` over-count with a closed code and no name on the line, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21), (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) and (.andromeda/obs-plan.md:418).
- **PII error layer ↔ §7 scrubbing layers**: a serde-sourced chain must become the fixed `internal error` on all three surfaces while its full form goes only to the detail file, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15) and (.andromeda/obs-plan.md:510).
- **SLO counting rule ↔ CI scan and G2**: a fault witness's home sits under the gates, so its forced failure must leave 0 counted panics and 0 scan hits, (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:9) and (.andromeda/obs-plan-detail/ci-integration.md:59).
