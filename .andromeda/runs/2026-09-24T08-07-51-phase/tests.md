# tests extract

## Relevance
Relevant. This chunk is the test-plan §3 Bootstrap phases `test-data-bootstrap-wire` item plus one fold into the §3 `run` step 4 mutation-gate verdict. Test tier is Comprehensive (per test-plan §10).

## Constraints
- **Fake agent contract.** test-plan §7 "Fake agent (binding contract behaviour)" requires all of the following:
  - The JSON turn script (`--script`) is resolved against `<workspace root>` the same way as `--fixtures`.
  - Each gated step waits for the next line appended to `--control`, read by byte offset. This is how tests get deterministic ordering with no sleeps.
  - The control path is always `<home>/fake/<name>.control`.
  - Scenario scripts are checked in as `fixtures/fake-scripts/<scenario>.json` and hold synthetic text only.
  - `--receipt` records received prompts (text, raw hex, `bare_esc`), keystrokes, env names, Unix fds and hook invocations (exit code, stderr length, stdout bytes).
  - It runs the listed modes. `--report-version` changes only the `--version` answer, and fixture replay still follows `--cli-version` / `--fixtures`.
  - It exits on `\x03`.
  - It reads hook commands from the `plugin/` and `settings.json` files `run` wrote, never from PATH.
  - Placement is a root-package `[[bin]]` with `required-features = ["fake-agent"]` (per test-plan §2 Test directory conventions and §12 "Fake agent placement").
- **Fixture chain.** Per test-plan §3 `run` step 2, the chain `home → fake_agent_path → stamped_home → booted_wrapper` exists **twice against one contract**: a sync root copy in `tests/support/` and a copy in `viola_e2e::fixtures`. The plan's reasons: root `tests/support/` cannot be imported by another package, and the root package must not dev-depend on the tokio-based `viola-e2e`.
  - Scope item 4 says the chain lives "in `viola_e2e::fixtures`, used from root `tests/support/`". That conflicts with this rule.
  - Resolving it is research's or planning's call. If the plan is changed instead, it needs an amendment.
  - Fake-agent lookup also differs by location: root tests use `CARGO_BIN_EXE_viola-fake-agent`, while `viola-e2e` uses the assert_cmd target-dir lookup and never `CARGO_BIN_EXE_*` (per test-plan §8 Process mocking row and §3 `boot` step 5).
- **Home lifecycle.** Per test-plan §3 `boot` step 2, §3 Test data bootstrap (Cleanup) and §7 Test data lifecycle:
  - The home path must not exist yet, so that viola creates it with 0700 on Unix or the protected DACL on Windows.
  - It sits inside a `tempfile` dir under `<workspace root>/target/e2e-home/`.
  - `TempDir::keep()` runs on a failing test when `AGENT_RUN_KEEP_FAILED=1`, and on every test when `AGENT_RUN_KEEP_HOMES=1`.
  - Env is set per child with `Command::env`, never `std::env::set_var`.
- **Stamps and fixture replay.** Per test-plan §7 Seed strategies and §12 "Stamps conflict", `stamped_home` stamps only through `viola verify --home <home>` against the fake agent. No test or harness code writes `ledger/stamps.json`.
  - The interim `stamped_home` (before the verify verb exists) must stay within that rule.
  - Whether the interim can be anything other than a documented no-stamp seam is research's question.
- **Fixture hygiene.** Per test-plan §7 Fixture hygiene and §6 Contract suite:
  - One rstest `#[files("fixtures/claude/*/*.json")]` walk rejects absolute paths (`^[A-Za-z]:[\\/]`, `/home/`, `/Users/`, `\\Users\\`) and non-placeholder usernames.
  - The same walk validates each file with jsonschema 0.57.0 against a tolerant schema: unknown fields allowed, closed enums required.
  - test-plan §3 preamble bans a vacuous pass ("never a vacuous pass"), which applies to an empty glob.
- **Property seeds.** Per test-plan §7 Seed strategies (Randomized data row) and §4 Fixture pattern, proptest 1.11.0 strategies run at `cases: 512` and their seeds are committed under `proptest-regressions/`. At HEAD the only strategy in scope is `ViolaName` plus its invalid neighbours.
- **Mutation verdict.** test-plan §3 `run` step 4 Verdict/Base requires:
  - `outcomes.json` must show `missed == 0 && timeout == 0`, and the exit code alone is never trusted.
  - A false `tested:0` pass is never allowed.
  - test-plan §10 Mutation gate reports a tests-only diff as `"mutants":{"tested":0}`.
  - The plan has no arm for a non-empty diff with no `.rs` files, where no `outcomes.json` is produced. The operator's explicit "no-rust-delta" verdict (item 7) is therefore a new value. Per test-plan §3 Closed enums, a new value needs a §12 Decisions Log entry and a §3 `run` step 4 amendment.

## Patterns to follow
- **Byte-offset control file** (per test-plan §7 and §3 `run` step 3): a Rust test and a Playwright spec both release gated steps by appending to `<home>/fake/<name>.control`. The fake agent reads new bytes from its last offset. Readiness waits use a bounded file-state probe, not sleeps (§3 `boot` Readiness signal).
- **Receipts as the verdict source** (per test-plan §11 E2E): verdicts come from the fake-agent receipt and `events.ndjson`, never from parsing the PTY or vt100 screen. A receipt line should be one ndjson record carrying `v` (the scope's inferred rule; consistent with §3 "every harness document starts `{"v":1,…}`").
- **rstest conventions** (per test-plan §4 Conventions and Fixture pattern): `#[fixture]` constructors, `#[case::readable_label]` tables, `#[files(...)]` for per-fixture tests, and test names of the form `<subject>_<condition>_<expected>`.
- **Location and naming** (per test-plan §2 Test directory conventions): root sync helpers go in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs}`. Integration binaries keep a `<surface>_<topic>.rs` prefix so layer filtersets still select them (§3 `run` step 1: `kind(test)` until an E2E-prefixed binary exists).
- **Unit-testing the harness verdict** (per test-plan §3 `run` step 4 Verdict): the rule that counts decide, never the exit code alone, extends to the new arm. A diff with no `.rs` files passes only with the diff checked and named in the output. A diff with `.rs` files and no `outcomes.json` stays `outcomes-missing`. Both need unit tests.

## Anti-patterns to avoid
- Never use `std::env::set_var`. Never call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`. Never use `sleep(N)` for synchronisation (per test-plan §11 Integration and §11 E2E).
- Never hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Never use non-synthetic or unscrubbed fixture text. Never use a generator without committed `proptest-regressions/` seeds (per test-plan §11 Test Data and §11 Universal).
- Never widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates/viola-e2e|tests/support|fuzz/`) to pass a gate, and never allow a vacuous mutation pass (per test-plan §11 CI and §11 Quality "NEVER trust the cargo-mutants exit code alone").

## Contract bindings
- **tests ↔ obs:** homes must live under `target/e2e-home/`, because obs-plan §9 G2, G4, the secret scan and the `diag-<os>` upload read `target/e2e-home/**`. `AGENT_RUN_KEEP_HOMES=1` keeps homes until those gates have run (per test-plan §3 `boot` step 2 and §7 Test data lifecycle).
- **tests ↔ obs:** the fake agent's exemption from the print-ban lint is carried to "Observability gates" (scope Boundaries). The receipt is a harness-owned test format, not a product diag-line, so it must not be written into `<home>/diagnostics/` files that G4 validates (per test-plan §3 Log format).
- **tests ↔ security:** the home is created by viola itself (0700 on Unix, protected user + SYSTEM DACL on Windows for a `--home` outside `%USERPROFILE%`). Only `viola verify` writes `stamps.json` (per test-plan §3 `boot` step 2 and §12 Stamps conflict).
- **tests ↔ arch:** the `fake-agent` feature and root `[[bin]]` stay out of `cargo build --release --bin viola` (per test-plan §9 Release build row and §12 Fake agent placement).
- **tests ↔ mutation scope:** per test-plan §3 `run` step 4 Test scope, root-package tests must kill mutants in `src/bin/viola-fake-agent.rs` if cargo-mutants mutates that file. Whether `.cargo/mutants.toml` already excludes it is research's question.

## Acceptance criteria contributions
- `scripts/agent-run.sh run --unit` and `run --integration` pass on all three OSes, including the new fake-agent control/receipt tests, the scrub-and-schema walk and the `ViolaName` proptest. `proptest-regressions/` is committed (per test-plan §3 `run` step 1 and §7 Seed strategies).
- The scrub-and-schema walk is non-vacuous: it asserts a nonzero match count, or covers `fixtures/fake-scripts/*.json`. Every rejection arm (each absolute-path pattern, a non-placeholder username, a schema violation) is proven red on a planted bad input (per test-plan §7 Fixture hygiene and §3 preamble "never a vacuous pass").
- Scripted-step ordering is driven only by `--control` appends and asserted from `--receipt` lines, with no sleep anywhere in the test (per test-plan §7 Fake agent and §11 E2E).
- `run --mutants` has two unit tests:
  - A non-empty diff with no `.rs` files yields an explicit no-rust-delta verdict that names the diff.
  - A diff with `.rs` files and no `outcomes.json` stays `failures:["outcomes-missing"]`, `ok:false`.

  The chunk's own mutation gate must show `missed == 0 && timeout == 0`, and a §12 entry must record the new closed value (per test-plan §3 `run` step 4, §3 Closed enums and §10 Mutation gate).

## Relevant amendment history
- **Harness builds in its own target dir** (§3 `boot` steps 1/3/5, `run` steps 1–3). The harness builds with `CARGO_TARGET_DIR=target/harness` and `--features viola/fake-agent`, and the fake agent is copied from `<bin dir>` = `target/harness/debug`.
  - Why: `os error 5` when relinking the running `viola-harness.exe` on Windows.
  - Relevance: this is where `fake_agent_path` and the harness look up the fake agent.
- **Integration filterset, mutation diff, base and trigger** (§3 `run` steps 1 and 4, §9).
  - The integration layer is `kind(test)` until an E2E-prefixed binary exists. The mutation diff is the working tree plus untracked files against `merge-base`. The `mutants` job runs on push and PR, with base = PR base sha or else `github.event.before`.
  - Why: nextest rejects an unmatched `binary()` regex, and the committed-only diff was 0 lines.
  - Relevance: this is the diff source behind item 7's docs-only push failure.
- **Mutation gate prebuilds the root bins** (§3 `run` step 4 Command). The root `viola` and `viola-fake-agent` bins are prebuilt, and `--copy-target=true` is passed.
  - Why: cargo-mutants scopes the baseline to the packages the diff touches, so a diff touching only `viola-e2e` failed with `fake agent: NotFound`.
  - Relevance: this is the direct neighbour of item 7's Rust-free-diff arm in the same `run.rs::mutants` path.
- **Nextest mutants profile** (§3 test-runner-install). `[profile.mutants]` terminates immediately on the first failure, with a 15s×2 slow-timeout.
  - Why: a caught mutant was graded Timeout under `fail-fast = true`.
  - Relevance: new fake-agent tests that block on the control file must stay inside this timeout when run under mutation.
- **Interim supervisor, readiness, status and cleanup** (§3). Interim `null` fields are allowed, the grammar grows per chunk, and an unbuilt selector is a usage error rather than a vacuous pass.
  - Relevance: this is the precedent for an interim `stamped_home` seam that is documented and never falsely green.
