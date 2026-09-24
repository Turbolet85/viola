# tests extract

## Relevance
Partial. The chunk is obs-owned (obs-plan §3 `obs-ci-gate-wire`). Tests owns five parts of it: the check bodies behind G4 and the secret scan, where test homes are placed and kept, CI job and gate discipline, the mutation-timeout fold, and the rule that proofs are agent-runnable.

## Constraints
- The G4 schema-conformance check body is owned by tests, and obs owns the schemas. Per test-plan §6 "Schema conformance" and §12 (overseer fix pass, D-21 items), the check must do four things. It validates every home-level `diagnostics/*.ndjson` line under `target/e2e-home/**` against `schemas/diag-line.v1.json`. It validates every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json`. It skips non-JSON lines and counts them. It prints only the file, line number and failing keyword, never the line's content. The same body also validates `e2e-web/test-results/a11y/*.ndjson`, but against `e2e-web/schemas/a11y-row.v1.json` (§3 Log format, harness-side a11y rows). G2 and G4 must never read those files.
- Secret scan and canary rules come from test-plan §6 "Error sanitization and secret scan". All inputs are synthetic and embed one fixed canary constant from `tests/support`. The canary must never appear in home-level `<home>/diagnostics/*.ndjson`. It is allowed only in `events.ndjson` and `detail-*.ndjson`. In CI the scan covers `target/e2e-home/**`, `target/agent-run/` and `target/nextest/ci/junit.xml`, and it runs before any upload. A hit is reported as file, line, byte offset and pattern class, never the matched bytes. Whether a canary constant and the scan body already exist in `tests/support` is for research to confirm.
- Every harness and rstest home must be created under `<workspace>/target/e2e-home/`. The rules come from test-plan §3 `boot` step 2, §3 `run` step 2 (the root chain writes `target/e2e-home/viola-test-*`), §3 Test data bootstrap → Cleanup, §3 `cleanup` step 6, and the §7 CI line (~:1355):
  - The path is passed as a not-yet-existing path, so viola creates the home itself.
  - With `AGENT_RUN_KEEP_HOMES=1`, every rstest home calls `TempDir::keep()`, passing or failing.
  - Harness `cleanup` reports `home_removed:"kept"`, which is not a failure.

  G2's non-empty check depends on this. Whether HEAD's crate-level `tests/` suites and `crates/viola-e2e` homes already follow the rule is for research to establish.
- Mutation verdict rules come from test-plan §10 Mutation gate, §3 `run` step 4 (Verdict) and §11 Quality:
  - A `timeout` outcome fails the gate exactly like `missed`.
  - Retries and timeout bumps are banned (§10 Zero-flakiness budget).

  Relevant values for the timeout fold:
  - The rstest `booted_wrapper` interim readiness wait is bounded at 20 s (§3 `run` step 2, "Cleanup TempDir drop" bullet).
  - Harness `boot` readiness is 20 s per instance (§3 `boot` Timeout).
  - Both equal the cargo-mutants auto timeout of 20 s seen in run 35995290314.
  - §3 Readiness says a `process-exit` for `self` arriving first means `run-exited`, and that both pids must be alive.

  The plan therefore requires wrapper death to be detected as a fast failure rather than a wait for the deadline. Whether `tests/support/home.rs` detects a wrapper that exits with no lines at all (the `main -> Default` mutant) is for research to establish. Lowering the 20 s constants would diverge from §3's letter, so it needs a divergence or amendment note.
- CI rules come from test-plan §9 (paragraph after the table) and §11 CI:
  - Every `uses:` is SHA-pinned.
  - `permissions: {}` sits at the top and `contents: read` per job.
  - Each tool has exactly one version source.
  - Rust log assertions in CI use `jaq`. CI neither installs nor pins `jq`.

  The ripgrep/PCRE2 install step obs adds must follow the pin and single-source rules.
- Per test-plan §3 `gate` and §9 (Coverage report row), `viola-harness gate --require <that job's suites>` stays the **last** step of every job. The obs gate steps (G1–G4, the scan, the uploads) must be inserted before it, without changing any job's `--require` list.
- Local gate set: `scripts/agent-run.{sh,ps1}` must expose byte-identical semantics (test-plan §3 Harness implementation, `5-command-discipline-wire`). A missing tool is `exit 1` with `reason:"tool-missing"`, never a silent pass (§3 `ci-tool-install`). This constrains the honest local replacement for `rg`-based G3.

## Patterns to follow
- The hook panic path is witnessed through the fail-open matrix in root `hook_fail_open.rs`, per test-plan §6 (~:1217, :1229) and §1 (hook entity):
  - The forced-panic case uses a trigger compiled only under feature `fake-agent`.
  - It asserts exit 0, empty stderr, no body, and one `level:"ERROR", event:"panic"` line (§3 Log format, Agent parsing).
- Panic counts read only role files, via `.file|startswith("detail-")|not`, because a panic line is written to both its role file and its detail file (test-plan §3 `logs`). G2 must follow the same rule.
- Tests follow the naming in test-plan §2: `<subject>_<condition>_<expected>` in snake_case. Root sync suites go under `tests/{cli,hook,…}_<topic>.rs`, which keeps the nextest layer filtersets working for any new gate test. The "panic hook is first" witness and the lint both-ways proofs are machine verdicts, meaning an exit code or JSON (§2 Agent-runnable invariants).
- Per test-plan §3 `run` step 4 Classification, a chunk diff with no `.rs` path passes as `verdict:"no-rust-delta"`. A diff with any `.rs` change (for example the fake-agent `#![allow]` or output-module `#[allow]`s) is `counted`.

## Anti-patterns to avoid
- Do not fix the mutant timeouts by raising a timeout, adding a retry, or using `#[ignore]`. Fix the wait instead (test-plan §10 Zero-flakiness budget; §11 Quality and CI: nextest `retries` must never go above 0).
- Do not widen the coverage `--ignore-filename-regex` or lower the §10 thresholds (lines 85 / functions 95 / regions 80) to absorb new gate code (test-plan §11 CI).
- Do not use a test-owned `sleep` or elapsed-time verdict for synchronisation in a readiness or gate fix. Use file-state or pid probes on a bounded interval (test-plan §11 E2E and Universal, §3 Readiness signal).

## Contract bindings
- tests §3 Log format ↔ obs `schemas/diag-line.v1.json`: the harness greps the fields `timestamp`, `level`, `target`, `message`, `event`, `process`, `instance` and `corr`. A null `corr` or `instance` is written as an absent key, and the schema rejects a literal `null`. obs may add fields but must not rename or remove any (§3 `log-format-bind-with-obs`).
- The tests §6 G4 check body and secret-scan body are called by the obs §9 CI steps `id: schema-conformance` and `id: secret-scan`, in obs's step order.
- tests §3 `cleanup` step 6 and Test data bootstrap (`AGENT_RUN_KEEP_HOMES=1`) ↔ the obs §9 homes lifecycle: homes stay until G2, G4, the scan and the scan-gated uploads have run.
- The print bans (`print_stdout` / `print_stderr` on `--all-targets`, per test-plan §9 Lint row) collide with tests-owned code in three places:
  - `viola-harness` must print one JSON document on stdout (§3 Exit codes; §11 Universal). This is why `viola-e2e` keeps its own `[lints.clippy]` table.
  - The root fake-agent `[[bin]]` (§12 Fake agent placement) needs the crate-level `#![allow]` from the scope's CARRY.
  - Root `tests/` and `tests/support` inherit root-package lints under `--all-targets`. Whether they contain `println!`/`eprintln!` is for research to check.
- Security NEVER-log floor ↔ tests §3 Log format constraint: no line may contain the GUI token, a `Cookie` header, `?t=`, or a stripped `CLAUDE*` value.

## Acceptance criteria contributions
- An injected fault must make each gate fail, and the clean tree must make it pass, all within the pinned rules:
  - A non-conforming diagnostics line fails the G4 check body, and the report names file, line and keyword with no line content (per test-plan §6 Schema conformance).
  - A canary planted in a home-level `diagnostics/*.ndjson` fails the scan, and the report names file, line, offset and class with no matched bytes (per test-plan §6 Error sanitization and secret scan).
- After a green run with `AGENT_RUN_KEEP_HOMES=1`, `target/e2e-home/` holds at least one home with a role file under `diagnostics/`, harness `cleanup` reports `home_removed:"kept"`, and every job's last step is still `viola-harness gate --require <suites>` (per test-plan §3 `cleanup` step 6 / Test data bootstrap and §3 `gate`).
- The `mutants` job on the chunk's CI run shows `missed == 0 && timeout == 0` in `outcomes.json`. The fix changes a wait so that it fails fast, and changes no timeout value or retry. The written argument is backed by the ubuntu test log (per test-plan §10 Mutation gate and Zero-flakiness budget).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` passes with the new bans on all 3 OSes. The per-OS `coverage,doctest` gate stays green at lines 85 / functions 95 / regions 80 (per test-plan §9 Lint row and §10 Coverage thresholds).

## Relevant amendment history
- **2026-09-24, three-os-ci-headless-harness-skeleton: nextest mutants profile.** `[profile.mutants]` became `fail-fast = { max-fail = 1, terminate = "immediate" }` with a 15 s×2 slow-timeout. The reason: a caught mutant was graded Timeout under a plain `fail-fast = true`. This is the earlier fix for the same class of symptom the chunk now folds (mutants graded TIMEOUT), and it was fixed through test termination, not through a longer timeout.
- **2026-09-24, three-os-ci-headless-harness-skeleton: mutation gate prebuilds root bins.** `run --mutants` prebuilds `viola`/`viola-fake-agent` and passes `--copy-target=true`, because package-scoped baselines failed with `fake agent: NotFound`. Relevant because the timed-out mutants (`main`, `dispatch`) are killed only by tests that spawn those bins.
- **2026-09-24, three-os-ci-headless-harness-skeleton: interim readiness, status and cleanup.** Interim readiness became the `process-start` lines for `self` and `claude-child` plus both pids alive, and `processes_gone` was added. This is the readiness surface whose deadline behaviour decides whether the `main -> Default` mutant is caught or times out.
- **2026-09-24, fake-agent-and-test-data-fixtures: fake-agent contract as built.** Only the sync root chain exists, with homes under `target/e2e-home/viola-test-*`. The root `stamped_home` is an interim no-stamp seam, and `booted_wrapper` waits for interim readiness bounded at 20 s. This bears directly on item 6 (home placement) and item 7 (the 20 s wait against the 20 s mutant timeout).
- **2026-09-24, fake-agent-and-test-data-fixtures: mutation verdict for Rust-free diffs.** This added the `no-rust-delta` / `counted` verdicts, and a counted run with no fresh `outcomes.json` is red. It decides how this chunk's diff, which mixes `.rs` and toml/yaml changes, is graded.
- **2026-09-24, supply-chain-and-workflow-gates: Lint row and Supply-chain stage.** The Lint row now reads `scripts/sync-crates.txt`, and a separate `supply-chain` job was added, both in `ci.yml`. The obs lint and gate steps must fit alongside these jobs without disturbing them.
