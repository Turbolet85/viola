# Report — 2026-09-24-log-redaction-and-never-log-floor

**Chunk:** skip-all spans, veil-redacted payload types, fixed error displays, anyhow chains + drift reports to instance detail files, NEVER-log floor at HEAD sinks, CI mutants red folded
**Date:** 2026-09-24T11:40:00Z
**Commits:** none since `last_wrap` 2026-09-24T10:52:16Z (`git log --oneline 809456e..HEAD` → empty; this wrap commits the chunk)

## Changes (structured — detectors read this)
- **Files:** `src/main.rs` · `src/cmd/mod.rs` · `src/cmd/run.rs` · `src/obs.rs` · `tests/run_cli.rs` (`git diff --stat -- src tests`: 5 files, +273 / −12). No new source files.
- **Symbols / APIs:**
  - **New `cmd::Failure { error: anyhow::Error, sink: Option<obs::DetailSink> }`** (`pub(crate)`, `src/cmd/mod.rs`). This is the dispatch error plus, once home and instance resolved, where its chain may go.
  - **`cmd::dispatch` signature changed** from `anyhow::Result<ExitCode>` to `Result<ExitCode, Failure>`. Its sole caller is `main` @ `src/main.rs:44` (research.md §Graph impact: `calls WHERE callee_name='dispatch'` → 1 row). A home-resolution failure carries `sink: None`, and a `run` failure carries `sink: Some(DetailSink{home, instance: RunArgs.name, process: Run})`.
  - **New `obs::DetailSink { home, instance, process }`** and **`obs::report_internal_error(&anyhow::Error, Option<&DetailSink>)`** (`pub(crate)`, `src/obs.rs`). The report emits the role line (below). When a sink is present, it writes ONE detail line through the existing `detail_line` + `write_detail`.
  - **New private `obs::chain_detail_line`** builds the detail line:
    - `event:"process-exit"`, `message:"process-exit"`, `level:"ERROR"`, `target:"viola::obs"`, plus `timestamp` / `process` / `instance`;
    - one content field, `chain: [string]`: every `anyhow::Error::chain()` cause's `Display`, outermost first;
    - no `subject` / `exit_code` / `detail` keys.
  - **Renamed `obs::panic_exit_line` → `obs::internal_error_exit_line`** (same body: the `run`-only `process-exit{subject:"self", exit_code:1, detail:"internal-error", duration_ms}` role line, emitted only when `ProcessCtx` is set). Callers after the change: `main`'s panic arm (`src/main.rs`) and `report_internal_error`. Remaining name references: `grep -rn panic_exit_line src crates tests` → 0 hits. Prior chunk artifacts (the diagnostics-plane plan/report) keep the old name as history.
  - **`cmd::run::RunArgs.name`** visibility `private → pub(super)`, so `dispatch` reads it.
  - **Catch-site behaviour** (`src/main.rs:42-55`). The `Err` arm now calls `obs::report_internal_error(&failure.error, failure.sink.as_ref())`, then returns exit `1`. It writes NOTHING to stdout or stderr: `run` stays silent per obs-plan §7 per-role behaviour, and no `cli` verb exists. Before this chunk the arm dropped the error (`Err(_) => ExitCode::from(1)`).
  - When `viola_obs_init` itself failed (the role file did not open), no subscriber exists. The detail line is then the only record.
  - When the home did not resolve, nothing is written.
  - The panic arm is unchanged except for the rename.
  - No new env vars, ports, pipes or files beyond the existing `instances/<name>/diagnostics/detail-run.ndjson`.
- **Crates / modules:** none added / removed. `src/obs.rs` gains `anyhow` use (the root bin already depends on it). No `viola-core` change.
- **Dependencies:** none added or bumped (`git diff --name-only HEAD` carries no `Cargo.toml` / `Cargo.lock`).
- **Schema / config:** none changed. The chain detail line validates against the existing `schemas/diag-detail.v1.json` (`chain` was already admitted, `unevaluatedProperties: false` holds), and `schemas/diag-line.v1.json` is untouched. No config key, no redaction shape; veil is not added.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** none — verified. The local suite sizes moved: unit 135 → 136 on Windows, where the new `#[cfg(unix)]` test does not compile, so 137 on Unix; integration 82 → 84 (gate logs 4.log / 5.log, `Summary` lines). No master states either count (`grep -rn "135\b\|84 tests" .andromeda/*.md .claude/docs/*.md` → 0 hits).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. The harness, the CI workflows and the verdict shapes are untouched. The plan adds three operator-leg CI reads (all check-runs on the pushed sha; the ubuntu and macOS `test` job logs for the new test id), which are plan entries, not harness changes.
- **Cross-project / external claims:** CI run `35990393334` (workflow `ci`, push) on sha `809456e329cf10bcf9fc783dc7885b50de9cada0`, the previous chunk. Job `mutants` (id `107602836363`) concluded `failure`, with `93 mutants tested in 9m: 1 missed, 64 caught, 28 unviable` and `MISSED src/obs.rs:193:19: replace match guard e.kind() == io::ErrorKind::NotFound with true in read_diagnostics_level`. The other 7 check runs (test ×3, lint ×3, supply-chain) concluded `success` (read via `gh api …/commits/809456e…/check-runs` at phase Setup 5a). This chunk folds that red; its witness is the post-push read of THIS chunk's sha (owed, below).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. Two nearby claims were checked and stand:
  - architecture §Conventions → Error handling schema (`.andromeda/architecture.md:153`) names `CoreError` among the per-crate enums. That is the target naming convention: `viola-core` has no fallible API at HEAD and no error enum (`grep -rn -E "thiserror|enum [A-Za-z]*Error" src crates tests` → only the test-only `HarnessError`).
  - The phase security EXTRACT's "`CoreError` is the one at HEAD" was an extract claim, not a master's, and was overridden at P4.
- **Expected amendments (from plan):** none listed. The plan's `Expected amendments (wrap)` reads "none anticipated", and its v1-47 note was written at phase P5.
- **Coverage of new surfaces:**
  - `catch-site dispatch error → detail file` → validation n/a (internal error value, no external input) · instrumentation log✓ (`process-exit{internal-error}` role line + `process-exit` detail line) · PII redacted✓ (the chain content goes only to the owner-only instance detail file, never a role line or stdout/stderr) · tests unit/integ (`obs::tests::internal_error_detail_line_carries_the_chain`, `obs::tests::internal_error_exit_line_writes_run_internal_error`, `run_cli::run_internal_error_routes_the_chain_to_the_detail_file`) · a11y n/a (no terminal output) · tokens n/a
  - `NEVER-log floor at HEAD sinks (CLAUDE* canaries)` → validation n/a · instrumentation n/a · PII redacted✓ (three planted `CLAUDE_*` canary values absent from every home file and both streams on a clean run, the catch-site error and `diagnostics_level: debug`) · tests integ (`run_cli::run_never_writes_a_claude_canary_anywhere`) · a11y n/a · tokens n/a
  - `read_diagnostics_level open-error arm on Unix` → tests unit (`obs::tests::read_diagnostics_level_file_as_home_is_unreadable`, `#[cfg(unix)]`); the rest n/a

## Deviations from intent
- **Deferred, per the P3 scope closure:** of the working entry's subjects, veil-redacted payload types, skip-all span fields, fixed `<Crate>Error` displays and serde drift-report routing were not built.
  - Why: research measured zero sites at HEAD. The counts were 0 `#[instrument`, 0 product error enums, 0 payload types, 0 `serde_path_to_error` producers, and no `veil` dependency (research.md §Subjects at HEAD, with its greps).
  - Why not build them anyway: `.claude/rules/testing.md` Session Addition 1 says to leave a function with no consumer out until its consumer exists.
  - Plan route pins: "Wrapper channel" (veil + `#[derive(Redact)]` on the first payload types, the first skip-all spans, the fixed-`Display` `ChannelError`) and "Hooks to normalised events" (the `drift_report` routing + the `hook` catch site exit 0 / empty stderr).
- **Deferred to "CLI output tokens":** the `cli` catch-site `error: internal error` stderr line. Why: no `cli`-process verb exists, and `run` must stay silent (obs-plan.md:1145, rules/observability.md:37).
- **Plan step 6 split across two tests** (implement P1):
  - What the plan asked: the new test was to check the home-level line "captured for the same failure".
  - Why it moved: that role line is emitted only when `ProcessCtx` is set, and only one test per binary may set that `OnceLock` (testing.md Session Addition 4).
  - What was done: the check (one role line, `detail:"internal-error"`, no `chain`, no canary) lives in the existing ctx-setting test, renamed `internal_error_exit_line_writes_run_internal_error`. The new `internal_error_detail_line_carries_the_chain` covers the chain line, its schema validity and the detail-file write.

## Decisions & corrections
- **Operator decision (phase P4, overseer founder-delegated):** the Linux kill of `src/obs.rs:193:19` is witnessed by the ubuntu and macOS `test` job logs carrying the new test's PASS line, not only the job conclusion, with the argument written into the plan's step 1. No Docker mutants run, and no edit made only to steer the diff-scoped gate.
- **Operator decision (new-session):** fold the red CI mutants verdict on `809456e` into this chunk, same precedent as dc01bd9's red folded into chunk 2. The fix commits through the loop.
- **Measured (P3):**
  - **Linux 6.6 (WSL2 busybox):** `open()` of a directory succeeds and the failure comes at read (EISDIR). `open()` of `<file>/config.json` fails at open (ENOTDIR).
  - **Windows 11 (python):** a directory → errno 13, and a file-parent → errno 2.
  - **Rust mapping:** ENOTDIR → `io::ErrorKind::NotADirectory` ≠ `NotFound` (std doc, stable 1.83).
  - **Consequence:** the old directory-based `Unreadable` test reached `:194` only on Windows.
- **Measured (P4):** the harness mutation gate runs `cargo mutants --in-diff <chunk.diff>` (`crates/viola-e2e/src/harness/run.rs:426-440`). A chunk whose diff does not touch a line never regenerates that line's mutant, so a green `mutants` job cannot witness a kill of a mutant that lives outside the diff.
- **Sweep hazard:** a bare `grep "instrument"` over `src crates tests` is the right census for `#[instrument` (0 hits at HEAD). A `grep -c` for a zero-is-healthy count exits 1 on the healthy state, so it needs `|| true` in any chain (host rule).

## Outcome
- Kill `:193` on Linux (ubuntu/macOS `test` job logs carry `PASS … obs::tests::read_diagnostics_level_file_as_home_is_unreadable`): **owed** — operator-leg reads after the push. The equality is argued in plan step 1, and the test compiles only under `cfg(unix)` (confirmed absent from the Windows unit selection: `run --unit` ran 136 tests with no such id).
- Every check run on the pushed sha is `success`: **owed** — operator-leg read after the push. The local chunk-diff `run --mutants` is **met**: `"verdict":"counted"`, 5 tested, 5 caught, `"survived":0`.
- Forced catch-site error → one schema-valid `process-exit` + `chain` detail line, no `chain` in any home-level file: **met** (`run_internal_error_routes_the_chain_to_the_detail_file`).
- That error exits 1 with empty stdout and stderr: **met** (same test).
- A canary-bearing error puts the canary only in the detail line's `chain`: **met** (`internal_error_detail_line_carries_the_chain` for the detail side; `internal_error_exit_line_writes_run_internal_error` for the role line carrying neither `chain` nor the canary).
- `CLAUDE_*` canaries in no home file and neither stream (clean, error, debug): **met** (`run_never_writes_a_claude_canary_anywhere`).
- `run --unit` / `run --integration` ok, and the spawn-failure regression guard green: **met**.
- **Gates** (implement P2, one pass, gate tool `gate v1.1`):
  - `cargo build --workspace --features fake-agent` green
  - `cargo fmt --all --check` green
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green
  - `bash scripts/agent-run.sh run --unit` green (136 passed)
  - `bash scripts/agent-run.sh run --integration` green (84 passed, 1 leaky: the pre-existing `cli_fake_agent::fake_agent_exit_no_eof_exits_while_stdout_is_held`, already in the diagnostics-plane report)
  - the `internal_error_detail_line_carries_the_chain` filter green (1 selected)
  - the `run_internal_error_routes…|run_never_writes…` filter green (2 selected)
  - the `run_is_silent…|run_with_a_missing_program…` filter green (2 selected)
  - `cargo deny check` green
  - the `deny-sync.toml` loop green
  - the `env::set_var` grep green (`exit 1`, no output)
  - `agent-run.sh cleanup --session gate-smoke` green
  - `agent-run.sh boot --session gate-smoke --instance builder` green
  - `agent-run.sh status --session gate-smoke` green
  - `agent-run.sh logs --session gate-smoke --process run` green
  - `agent-run.sh cleanup --session gate-smoke` green (`processes_gone:true`)
  - `AGENT_RUN_CHUNK_BASE=809456e… agent-run.sh run --mutants` green (counted, 5/5 caught)
  - `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` — leg operator, not fired by implement; this wrap's P7 push is the push
  - the check-runs read and the two job-log reads — leg operator, owed after the push; the wrap re-verifies nothing for them (no recorded artifact exists yet)
- **Smoke** (implement P3, boot path changed): `boot --session p3-smoke` ready, `status` ready, merged `logs` 0 `"chain"` lines, `cleanup` `processes_gone:true` and `home_removed:true`.
- **Outcome basis:** implement's P4 report as given in this session's conversation, plus the gate trail `.andromeda/runs/2026-09-24T11-31-53-implement/`.
- **Process hygiene** (implement P4 census, re-measured at wrap Setup by `Get-Process`):

  | Process | Started by | Final state |
  |---|---|---|
  | gate-smoke and p3-smoke wrappers/children (p3: wrapper 25544, child 59316) | this chunk's runs | terminated (absent from the process list) |
  | cargo-mutants / nextest | this chunk's runs | terminated |
  | `viola.exe` 5188 and 12172 (`viola-lab/prototype`) | the operator, not this project | left running (operator's) |
