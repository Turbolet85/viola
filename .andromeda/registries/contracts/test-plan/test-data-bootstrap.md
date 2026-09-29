### Test data bootstrap

- **Strategy:** self-bootstrapping (no developer-seeded data)
- **Mechanism:**
  - a tempfile 3.27.0 home per test or session
  - rstest 0.27.0 `#[fixture]` chain `home → fake_agent_path → stamped_home → booted_wrapper`
  - fake-agent replay of `fixtures/claude/<cli-version>/`
  - `viola verify --home <home>` against the fake agent for stamps
  - generated statusline stdin piped into `viola hook statusline` for `budget.json`
  - proptest 1.11.0 strategies with committed `proptest-regressions/` seeds for randomized inputs
  - Windows x64, root tests only: `tests/support/home.rs` `seed_conpty(home)` places the two ConPTY companions under `<home>/bin/<key>/conpty/` from the per-run `target/conpty-seed/<key>/` copy before the test's `viola run` (the §5 carve-out); the harness `boot`, `run_viola_unseeded` starts and `tests/conpty_sideload.rs` never seed
- **Per-test isolation:**
  - Every test gets a fresh home, so no endpoint, log or `budget.json` is shared: the FNV-1a endpoint hash includes the absolute home path.
  - The few tests touching the default port 47319 run in the nextest test group `fixed-port` (`max-threads = 1`).
- **Cleanup:** `TempDir` drop removes per-test homes. A test the runner kills (nextest `terminate = "immediate"`) never drops its home, so every root `TestHome` records `owner.json` `{pid, started_at}` beside it, and a new `TestHome`, when no `AGENT_RUN_KEEP_*` is set, first removes only the dirs whose recorded owner process is gone (pid dead, or alive with another start time) — never by age or name, never a dir without a record; a kept home drops its record. Harness sessions are cleared by `cleanup`. A failing test calls `TempDir::keep()` only when `AGENT_RUN_KEEP_FAILED=1`, so that `logs` can inspect the home. viola-e2e's booted lifecycle tests (`crates/viola-e2e/tests/harness_lifecycle.rs`) hold a `Booted` guard instead: on drop, even after a failed assertion, it runs `cleanup(.., keep_homes = true)` (every recorded process stopped, the home kept; the tests' own cleanup calls assert `home_removed:"kept"`) and then removes the home unless `AGENT_RUN_KEEP_FAILED=1` and the thread is panicking — `AGENT_RUN_KEEP_HOMES` is not read there, so a passing test's home is always removed. In CI, `AGENT_RUN_KEEP_HOMES=1` (set by `ci.yml`, overridden to `0` under `run --mutants`) makes every rstest home call `TempDir::keep()`, passing or failing, and makes harness `cleanup` keep its home (§3 `cleanup` step 6). The homes then stay under `target/e2e-home/` until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
