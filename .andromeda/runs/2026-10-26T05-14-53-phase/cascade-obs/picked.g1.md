### obs-scope-summary#6.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 50–50
- **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
### obs-scope-summary#6.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 51–51
- **Instrumentability:** instrumentable
### obs-scope-summary#6.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 52–52
- **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
### obs-scope-summary#7.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 58–58
- **Source:** arch excerpt, Stack (ndjson + atomic-write-file, sysinfo 0.39.6) and Workspace
### obs-scope-summary#7.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 59–59
- **Instrumentability:** instrumentable
### obs-scope-summary#7.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 60–60
- **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
### obs-scope-summary#8.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 66–66
- **Source:** arch excerpt, Workspace; Stack (serde_path_to_error drift reports)
### obs-scope-summary#9.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 76–76
- **Source:** arch excerpt, Stack (rmcp) and Workspace
### obs-scope-summary#9.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 78–78
- **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
### obs-scope-summary#11.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 97–97
- **Source:** arch excerpt, Standard Contracts (Hook contract; MCP tools)
### obs-scope-summary#15.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 129–129
- **Source:** arch excerpt, Stack (ndjson + snapshots), Standard Contracts (event line, snapshot envelope, heartbeat); security Data Classifications
### obs-scope-summary#15.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 130–130
- **Instrumentability:** instrumentable (through the `viola-state` call sites)
### obs-scope-summary#19.1 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 170–170
- **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
### obs-scope-summary#23.1 · values · strategy · ## 1. Obs Scope Summary · lines 221–237
- **OTel SDK init:** v1 initialises no OTel SDK exporter. The `tracing` facade plus the tracing-subscriber JSON formatter is the binding pipeline (tests excerpt, Log format; Founder Direction 1: "tests owns them"). The constraints ruling out an exporter:
  - no outbound network calls (security Error reporting);
  - a new listener, including a local OTLP receiver or metrics endpoint, needs a Security Decisions Log entry (security Anti-Patterns);
  - stdout is reserved on `hook`, `mcp` and `run` (arch Obs-Relevant Conventions);
  - `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).

  An OTel bridge layer is optional. It may only feed an in-process or file sink, and it must stay Tokio-free on the sync crates. Phase 2 evaluates whether it adds anything over the tests-fixed format.

  Init order per process:
  1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  2. Parse clap.
  3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument).
  4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  5. Install the subscriber with an explicit writer (never stdout).
  6. Emit `process-start`.

  For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
### obs-scope-summary#24.1 · values · logs · ## 1. Obs Scope Summary · lines 243–252
- **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  - one file per process role under `diagnostics/`;
  - no stdout sink on any role;
  - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).

  The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.

  The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.

  Timestamps must be RFC 3339 UTC with milliseconds and `Z` (tests excerpt, `timestamp`; arch Timestamps). Phase 2 must verify whether the tracing-subscriber default timer meets millisecond precision, or whether a chrono-based `FormatTime` (`to_rfc3339_opts(SecondsFormat::Millis, true)`) is needed.
### obs-scope-summary#40.1 · values · triggers · ## 1. Obs Scope Summary · lines 412–412
- **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
### telemetry-strategy#1.2 · values · strategy · ## 2. Telemetry Strategy · lines 11–11
| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
### telemetry-strategy#3.8 · values · triggers · ## 2. Telemetry Strategy · lines 35–35
| logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
### telemetry-strategy#3.15 · values · triggers · ## 2. Telemetry Strategy · lines 42–42
| creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
### telemetry-strategy#3.16 · values · triggers · ## 2. Telemetry Strategy · lines 43–43
| creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |
### span-trace-coverage#2.4 · values · spans · ## 4. Span / Trace Coverage · lines 21–21
| ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies |
### error-capture-reporting#4.1 · values · errors · ## 7. Error Capture & Reporting · lines 29–29
- vt100 panics are caught on `run`'s feed thread (`src/run/gate.rs`) by a `catch_unwind` around each feed and resize, not inside `run.readiness_gate`. Each poisoning episode gives exactly one `parse-rejected{parser:"vt100-feed", count:1}` at `WARN` with no `corr`, its `detail` set by the cause: `panicked` for a caught vt100 panic, `oversize` for a copy the bounded feed queue dropped (a size message carries the drop count at its offer, so a drop after a queued resize poisons the fresh screen again). The model stays poisoned until the host size changes, and `run` continues. `viola_panic_hook` runs before the catch returns, so a panic poisoning also writes the hook's `event:"panic"` role line and its detail line. Witnessed at run level by `tests/chaos_feed_panic.rs` (chunk 2026-10-04-confirmed-send-with-cl-1-records): exactly one `parse-rejected{vt100-feed, panicked}` and one `event:"panic"` line, both asserted present, then `viola send` refused `send-refused{detail:"input-not-ready"}` (exit 13) while a typed key still reaches the child. G2's single seam exemption is unchanged: that test's home lives outside `target/e2e-home` (`TestHome::outside_scan()`), so G2, G4 and the secret scan do not read it (the founder's ruling, live, 2026-10-04, relayed by the overseer).
### pii-scrubbing-compliance#1.12 · values · pii · ## 8. PII Scrubbing & Compliance › ### PII Scrubbing · lines 21–21
| Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
### pii-scrubbing-compliance#2.1 · values · pii · ## 8. PII Scrubbing & Compliance › ### PII Scrubbing · lines 28–44
5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
   - **High classes** (prompt and send text, `last_assistant_message`, tool `input`, dialog answers, drift-report echoes):
     - Every E2E and integration input is synthetic and embeds a fixed canary string.
     - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.
     - The canary value and the test body are tests-owned.
   - **Detail-file upload** (§9) is admissible only because every input is synthetic. It covers two artifacts: the `test` job's `diag-<os>` and the per-OS `perf` job's `diag-perf-<os>` (`target/e2e-home/**/diagnostics/*.ndjson`), whose inputs are the per-row synthetic hook payloads (string fields, the tests-owned canary in content fields) and the `fake-agent` seam's fixed ASCII panic payload (in `detail-hook.ndjson` only; the home-level `panic` line stays payload-free). The same job's `perf-<os>` (hyperfine timings and repo-relative command lines only) and `diag-perf-<os>` upload only when `always() && steps.secret-scan.outcome == 'success'` — the same channel, scan gate and synthetic-input basis as `diag-<os>`, not a new crossing.
   - The same rule covers the `harness-<os>` artifact. `agent-run logs` merges `events.ndjson` records (`prompt-submitted.data.text`, `last_assistant_message`) and detail lines, so it carries the canary by design. The scan applies the Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff`, and the canary rule exempts those files as it exempts `detail-*.ndjson`. That one file is `run --mutants`' diff, repository source text by construction (it hits the scan's own Critical-class patterns whenever it touches them); the `harness-<os>` upload excludes it (`!target/agent-run/chunk.diff`), so it is never scanned and never uploaded. A `chunk.diff` anywhere else under the capture is still scanned.
   - Any non-synthetic input source must first drop `detail-*.ndjson` and `target/agent-run/` from the upload paths.
   - The local `viola-harness pre-push` document is never uploaded and carries no absolute path, only codes and counts. The gate runs natively on the Linux host in the working tree itself (no clone), so neither the repository root nor the passwd home (`<home>`, `<home>/.cargo/bin`) reaches it. `pre_push_document_carries_no_absolute_path` asserts it, red at the Linux tests and green (chunks 2026-09-26-local-linux-pre-push-gate and 2026-10-03-mutation-scoring-completion).
   - **Unscanned uploads, admissible by content** (operator ratification 2026-09-24):
     - (the CI mutants legs' `mutants-verdict-<os>.json` left with those jobs on 2026-09-28.) `mutants.out/` is never uploaded (its `outcomes.json` held absolute argv paths; its `log/` held test output). On a Windows host it sits in the harness's host mutation scratch `<repo parent>/viola-mutants-scratch/` (`HOST_SCRATCH = cfg!(windows)`, inert elsewhere), outside the repository, whose path is never printed (documents carry only the byte count `scratch_bytes`). On the Linux dev host the same-named directory is only the operator's NOCOW `TMPDIR` for cargo-mutants' copy trees — an environment fact, not harness code — and no document prints it either; `mutants.out/` there stays at the repository root (gitignored). `target/run-archive/<n>/`, where each `run` copies the JUnit files it wrote and the `outcomes.json` it read, is never uploaded either: gitignored and outside `target/agent-run/` and every other upload path; the run document names it only by the repo-relative `archived`.
     - the nightly `fuzz` job's `fuzz/artifacts/` (on `failure()`) holds crash inputs the fuzzer mutated from the committed synthetic `fuzz/corpus/<target>/`. A non-synthetic corpus seed must first drop that upload.
     - the ci.yml `supply-chain` artifact (`target/supply-chain/deny.json`, `deny-fuzz.json` for the `fuzz/Cargo.lock` audit, and `zizmor.json`; `if: always()`, 7 days) holds cargo-deny and zizmor diagnostics only: crate names and versions, advisory ids, registry URLs and workflow-relative locations. It carries no absolute path, argv, log content or user input (as measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes: 0 absolute paths in all three files; the one raw pattern hit was the `s:/` inside `https://`). The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
   - **Scan failure:**
     - The pre-upload ban wins: no `diag-<os>` or `harness-<os>` artifact is uploaded in the `test` job, and no `perf-<os>` or `diag-perf-<os>` in the `perf` job; that job's hit report uploads as `secret-scan-perf-<os>`.
     - Instead, the scan step writes a hit report to `target/secret-scan/` (`file`, line, byte offset and pattern class, never the matched bytes), and the separate §9 step 5 uploads that directory as `secret-scan-${{ matrix.os }}`.
     - So the failing job still leaves agent-readable evidence (§11 CI).
