# tests extract

## Relevance
Partial. The redaction floor, the detail-sink routing and the folded mutation red on `src/obs.rs` are tests surfaces. The CI gates themselves (G1, canary/secret-scan gate, print bans) and the channel/MCP/UI error surfaces belong to other chunks.

## Constraints
- test-plan §1 requires the Comprehensive tier. Its "security-vector-coverage (error sanitization and secret logging)" trigger requires:
  - CLI `--json` errors hold no absolute path, serde path/value, anyhow chain or tool `input`.
  - A grep of all logs and `diagnostics/` finds no token, `Cookie`, `?t=` or stripped `CLAUDE*` value.
  - `diagnostics/` files are 0600.
  - §2 maps this trigger to the integration and E2E layers.
  - At HEAD this applies only to the root `viola` bin and `viola-core`. The MCP `isError`, Problem Details and `error.data` legs wait for their crates.
- test-plan §3 Log format requires:
  - Home-level role files `<home>/diagnostics/{run-,hook-,mcp,ui-,cli-}*.ndjson` stay codes-only.
  - Content-bearing detail (chains, drift reports, panic payload/backtrace) goes only to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`.
  - Files are 0600, dirs 0700, and each line is one `write`.
  - No process-log line may carry the GUI token, `Cookie`, `?t=` or any stripped `CLAUDE*` value.
- test-plan §3 Bootstrap `log-format-bind-with-obs` fixes the harness-grepped fields: `timestamp`, `level`, `target`, `message`, `event`, `process`, `instance`, `corr`. The `event` enum is closed. Redaction work must not rename or remove any of these fields or add an `event` value without a §12 Decisions Log entry. Null `corr`/`instance` is written as key absence.
- test-plan §6 "Error sanitization and secret scan" → Canary requires that the fixed canary constant in `tests/support`, embedded in every synthetic prompt / send text / tool `input` / answer / statusline payload, never shows up in a home-level `diagnostics/*.ndjson` line. It may appear only in `events.ndjson` and `instances/<name>/diagnostics/detail-*.ndjson`. A hit is reported as file, line, byte offset and pattern class, never the matched bytes. Whether the canary constant and the scan helper already exist is research's question.
- test-plan §6 "Schema conformance" requires:
  - every role-file line validates against `schemas/diag-line.v1.json`
  - every detail line validates against `schemas/diag-detail.v1.json`

  So newly routed `chain` / `drift_report` detail records must conform. Failures print file, line and schema keyword, never the line's content.
- test-plan §3 `run` step 4 and §10 Mutation gate require a `counted` verdict with `missed == 0 && timeout == 0` from a fresh `mutants.out/outcomes.json`. The exit code alone is never trusted. §11 Test Strategy says OS-branch code is verified only on its own runner. So the folded `src/obs.rs:193:19` `read_diagnostics_level` NotFound-guard miss must be killed on the CI ubuntu `mutants` job, not just on the Windows dev host.
- test-plan §5 "Module ↔ DB" concurrent-append check: separate processes appending at once to the shared `detail-hook` / `detail-cli` / `detail-mcp` files, with at least one detail line over 4 KiB, must leave every line parseable as one JSON object. This matters once anyhow chains and drift reports route there. Which of these detail files exist at HEAD is research's question.
- test-plan §5 CLI `tests/cli_controls_not_disableable.rs` and the §1 Vector 6 trigger ("no env / flag / config disables a control") cover every `VIOLA_*` env var, global flag and `config.json` key as literal `#[case]` rows. They include a completeness case against `--help` and the config schema. Any setting this chunk adds that touches redaction must appear there and must be unable to widen or disable it. Whether the file exists yet is research's question.

## Patterns to follow
- Unit tests, per test-plan §4 Conventions:
  - Put them inline as `#[cfg(test)] mod tests`.
  - Name them `<subject>_<condition>_<expected>`.
  - Use rstest `#[case::<label>]` tables. These fit the fixed-message `Display` rows, one per `<Crate>Error` variant, and the serde-source → fixed-message mapping.
- Use real files, not mocks, per test-plan §8 ("Filesystem: not mocked") and §5 Setup/teardown: a not-yet-existing home under `tempdir_in("<root>/target/e2e-home")`. The `read_diagnostics_level` NotFound-vs-Unreadable cases therefore need real on-disk inputs that reach the open-error branch on Linux as well as Windows. Doubles are allowed only for seam traits the product already defines (§8 Process mocking, §11 Mocking: no failpoint crate).
- Report leaks by class only. The §7 Fixture hygiene checker reports `absolute-path` / `username` / `schema`, and the §6 secret scan reports file/line/offset/pattern class. Any new redaction or leak assertion should likewise report the violation class, never the offending bytes, and use planted inputs to prove each class goes red.
- `logs` wraps detail lines as `{"src":"diag","file","instance","record"}`, per test-plan §3 `logs`. Panic counts read role files only (`.file|startswith("detail-")|not`), because a panic appears in both.
- Commands to pass, per test-plan §3 `run` / §9:
  - `scripts/agent-run.sh run --unit`, `--integration`, `--coverage` (per OS)
  - `run --mutants` (ubuntu CI job, base `github.event.before` on push)

## Anti-patterns to avoid
- test-plan §11 Unit: never import the product's own NEVER-log list, strip list or fixed error strings as the test oracle. Write the expected names and messages as literals in the test.
- test-plan §11 Integration: never use `std::env::set_var` to plant `CLAUDE*` / token canaries. Use `Command::env` per child. Never `.env_clear()` without re-adding `LLVM_PROFILE_FILE`.
- test-plan §11 Test Strategy / Quality: never treat a local run on one OS as proof for another, and never trust the cargo-mutants exit code alone. The folded mutant was caught on Windows and missed on Linux.

## Contract bindings
- tests §3 Log format ↔ obs-plan D-08 detail sink and obs schemas `diag-line.v1.json` / `diag-detail.v1.json`. obs owns the schemas; tests own the G4 conformance check body (§6 Schema conformance, §12 fix pass 1).
- tests §6 canary / secret scan ↔ obs-plan §8 (High-class verification) and §9 step order (scan before any upload of `target/e2e-home/**`, `target/agent-run/`, `junit.xml`). The CI gate wiring belongs to route entry "Observability gates", not this chunk.
- tests §1 error-sanitization trigger ↔ security-plan §error-sanitization-wire / §logging-redaction-wire (NEVER-log list) and security Vector 6 (no disabling control), via `cli_controls_not_disableable.rs`.

## Acceptance criteria contributions
- (tests) The CI ubuntu `mutants` job's `run --mutants` for the chunk diff reports `"verdict":"counted"` with `missed == 0 && timeout == 0`. The `src/obs.rs` `read_diagnostics_level` `e.kind() == io::ErrorKind::NotFound` guard mutant is caught on that runner (per test-plan §3 `run` step 4 / §10 Mutation gate).
- (tests) An integration test feeds canary-bearing input through an anyhow-chain failure and a serde drift failure. It asserts the canary and chain text appear only in `instances/<name>/diagnostics/detail-<process>.ndjson` (0600 on Unix), and never in any home-level `diagnostics/*.ndjson` line, stderr, or the `--json` error. The `--json` error also contains no absolute path, serde path or `Caused by` (per test-plan §6 Error sanitization and secret scan).
- (tests) Every role-file line produced by the new tests validates against `schemas/diag-line.v1.json`, and every new `chain` / `drift_report` detail line validates against `schemas/diag-detail.v1.json` (per test-plan §6 Schema conformance).
- (tests) `run --coverage` passes on all 3 OSes at lines ≥ 85 / functions ≥ 95 / regions ≥ 80 (per test-plan §10 Coverage thresholds).

## Relevant amendment history
- **2026-09-24-fake-agent-and-test-data-fixtures: mutation verdict for Rust-free diffs.** A `.rs` delta deletes a stale `outcomes.json` and is `counted`; a counted run with no fresh `outcomes.json` is red. Why: cargo-mutants 27.1.0 exits 0 on Rust-free diffs and a stale file was read (CI run 35973118026). This chunk's diff is Rust, so it takes the `counted` path.
- **2026-09-24-three-os-ci-headless-harness-skeleton: mutation gate prebuilds the root bins.** `run --mutants` prebuilds `viola` with `fake-agent` and passes `--copy-target=true`. Why: cargo-mutants scoped the baseline to the touched packages, and the root bins went missing (exit 4).
- **2026-09-24-three-os-ci-headless-harness-skeleton: integration filterset, mutation diff, base and trigger.**
  - The mutation diff is the working tree plus untracked files from `merge-base(base, HEAD)`.
  - The CI `mutants` job runs on push and pull_request, with base = PR base sha or else `github.event.before`.

  Why: the committed-only diff was empty, and `origin/main` never resolves on this branch. This is the job whose run (35990393334) holds the folded miss.
- **2026-09-24-three-os-ci-headless-harness-skeleton: nextest mutants profile.** `[profile.mutants]` uses `fail-fast = { max-fail = 1, terminate = "immediate" }` with a 15 s×2 slow-timeout. Why: a caught mutant was graded Timeout under `fail-fast = true`. Relevant because new obs tests must not hang siblings under mutation.
