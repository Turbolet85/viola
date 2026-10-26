# obs-plan · PII Scrubbing & Compliance
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its PII Scrubbing & Compliance detail.

## 8. PII Scrubbing & Compliance › ### PII Scrubbing

<!-- detail: values · pii -->

| Data type | Classification | Obs handling | Scrubbing mechanism |
|-----------|---------------|--------------|---------------------|
| `Cookie` header | Critical | never-log | secret-scan test (the case-insensitive `cookie:` header form) over all `diagnostics/` |
| `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` values | Critical | never-log anywhere, including detail files | R8 strip logs `env_stripped_count` + `env_stripped_known` (stripped names) + `env_kept` (kept names), names only, values never read (D-34); env maps never passed to a span or event; secret-scan test with one canary per identity-floor name plus an unknown name |
| v1.x signing credentials | Critical | never-log | Not present in v1 runtime; CI secrets never echoed into artifacts (Section 9) |
| Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |
| Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report | `#[redact]` on `HookDialogParams.data`, `AnswerParams.response`; codes-only dialog events (`dialog_kind`) |
| serde_path_to_error drift reports | High | instance-scoped `detail-<process>.ndjson` only | Replaced by fixed `Display` at the seam; full report is a `drift_report` field in the detail file only |
| `statusline_command` | High (integrity) | never in home-level logs; detail file only if part of an error chain | `statusline.shell_out` span records `shell_exit_status` + `duration_ms` only |
| Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
| Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
| `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
| `claude agents --json` rows / unwrapped session ids | Low, untrusted | OK structured; C0/C1 escaped in human terminal output | JSON serialization escapes C0 controls in files; rows never logged verbatim, only `parse-rejected` counts |
| Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
| Recorded fixtures | Medium | scrub-required (security/tests-owned) | Not a telemetry source; obs adds no fixture content |

<!-- /detail -->

<!-- detail: values · pii -->

5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
   - **High classes** (prompt and send text, `last_assistant_message`, tool `input`, dialog answers, drift-report echoes):
     - Every E2E and integration input is synthetic and embeds a fixed canary string.
     - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.
     - The canary value and the test body are tests-owned.
   - **Detail-file upload** (§9) is admissible only because every input is synthetic. It covers two artifacts: the `test` job's `diag-<os>` and the per-OS `perf` job's `diag-perf-<os>` (`target/e2e-home/**/diagnostics/*.ndjson`), whose inputs are the per-row synthetic hook payloads (string fields, the tests-owned canary in content fields) and the `fake-agent` seam's fixed ASCII panic payload (in `detail-hook.ndjson` only; the home-level `panic` line stays payload-free). The same job's `perf-<os>` (hyperfine timings and repo-relative command lines only) and `diag-perf-<os>` upload only when `always() && steps.secret-scan.outcome == 'success'` — the same channel, scan gate and synthetic-input basis as `diag-<os>`, not a new crossing.
   - The same rule covers the `harness-<os>` artifact. `agent-run logs` merges `events.ndjson` records (`prompt-submitted.data.text`, `last_assistant_message`) and detail lines, so it carries the canary by design. The scan applies the Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff`, and the canary rule exempts those files as it exempts `detail-*.ndjson`. That one file is `run --mutants`' diff, repository source text by construction (it hits the scan's own Critical-class patterns whenever it touches them); the `harness-<os>` upload excludes it (`!target/agent-run/chunk.diff`), so it is never scanned and never uploaded. A `chunk.diff` anywhere else under the capture is still scanned.
   - Any non-synthetic input source must first drop `detail-*.ndjson` and `target/agent-run/` from the upload paths.
   - The local `viola-harness pre-push` document is never uploaded and carries no absolute path, only codes and counts. The gate runs natively on the Linux host in the working tree itself (no clone), so neither the repository root nor the passwd home (`<home>`, `<home>/.cargo/bin`) reaches it. `pre_push_document_carries_no_absolute_path` asserts it, red at the Linux tests and green (chunks 2026-09-26-local-linux-pre-push-gate and 2026-10-03-mutation-scoring-completion).
   - A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
   - **Unscanned uploads, admissible by content** (operator ratification 2026-09-24):
     - (the CI mutants legs' `mutants-verdict-<os>.json` left with those jobs on 2026-09-28.) `mutants.out/` is never uploaded (its `outcomes.json` held absolute argv paths; its `log/` held test output). On a Windows host it sits in the harness's host mutation scratch `<repo parent>/viola-mutants-scratch/` (`HOST_SCRATCH = cfg!(windows)`, inert elsewhere), outside the repository, whose path is never printed (documents carry only the byte count `scratch_bytes`). On the Linux dev host the same-named directory is only the operator's NOCOW `TMPDIR` for cargo-mutants' copy trees — an environment fact, not harness code — and no document prints it either; `mutants.out/` there stays at the repository root (gitignored). `target/run-archive/<n>/`, where each `run` copies the JUnit files it wrote and the `outcomes.json` it read, is never uploaded either: gitignored and outside `target/agent-run/` and every other upload path; the run document names it only by the repo-relative `archived`.
     - the nightly `fuzz` job's `fuzz/artifacts/` (on `failure()`) holds crash inputs the fuzzer mutated from the committed synthetic `fuzz/corpus/<target>/`. A non-synthetic corpus seed must first drop that upload.
     - the ci.yml `supply-chain` artifact (`target/supply-chain/deny.json`, `deny-fuzz.json` for the `fuzz/Cargo.lock` audit, and `zizmor.json`; `if: always()`, 7 days) holds cargo-deny and zizmor diagnostics only: crate names and versions, advisory ids, registry URLs and workflow-relative locations. It carries no absolute path, argv, log content or user input (as measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes: 0 absolute paths in all three files; the one raw pattern hit was the `s:/` inside `https://`). The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
   - **Scan failure:**
     - The pre-upload ban wins: no `diag-<os>` or `harness-<os>` artifact is uploaded in the `test` job, and no `perf-<os>` or `diag-perf-<os>` in the `perf` job; that job's hit report uploads as `secret-scan-perf-<os>`.
     - Instead, the scan step writes a hit report to `target/secret-scan/` (`file`, line, byte offset and pattern class, never the matched bytes), and the separate §9 step 5 uploads that directory as `secret-scan-${{ matrix.os }}`.
     - So the failing job still leaves agent-readable evidence (§11 CI).

<!-- /detail -->
