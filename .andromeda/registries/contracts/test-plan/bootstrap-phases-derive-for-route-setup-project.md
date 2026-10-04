### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness. route may reorder or
combine them, and setup-project may add stack-specific intermediate steps.

- **test-runner-install:**
  - Install cargo-nextest 0.9.146 via taiki-e/install-action (SHA-pinned) in CI, or `cargo install --locked` locally.
  - Add `.config/nextest.toml` with:
    - `[profile.ci]`: `junit.path = "junit.xml"`, `retries = 0`, `slow-timeout = { period = "30s", terminate-after = 4 }`, `fail-fast = false`
    - `[profile.mutants]`: `fail-fast = { max-fail = 1, terminate = "wait" }`, `slow-timeout = { period = "5s", terminate-after = 2 }`, plus `[[profile.mutants.overrides]] filter = 'package(viola-e2e)'` with `slow-timeout = { period = "15s", terminate-after = 2 }`.
      - The first failure stops scheduling and the tests already running finish, so their temp dirs and session guards drop. The slow-timeout kill bounds any hang among them below cargo-mutants' 20 s floor, so a caught mutant ends at most at the kill line (10 s; viola-e2e 30 s) and is never graded Timeout. As measured at chunk 2026-10-04-windows-boundary-mutation-workflow:
        - 0 Timeout grades over 711 Linux viola-e2e mutants and 508 Windows mutants;
        - in a two-sided witness, 170 leftover temp dirs under `terminate = "immediate"` against 0 under `wait`;
        - after a full viola-e2e run, 38 `.tmp*` dirs and 0 nested copies, against 25 275 and 62 before. Of the 38, 17 come from tests nextest's slow-timeout or a mutant-made SIGKILL killed (by design), and 21 are half-removed throwaway git repos, a `terminate`-independent class.
      - Each caught mutant now waits for its running tests, so a full Linux viola-e2e run took 78 m against 23 m under `immediate`, with identical counts.
      - `immediate` had replaced a plain `fail-fast = true` on 2026-09-24, when a caught mutant that hung sibling tests outlived cargo-mutants' timeout and graded Timeout (auto timeout 108 s). The slow-timeout kill above now bounds that hang.
      - cargo-mutants 27.1.0 auto-sets its timeout to about `max(20 s, 5 × baseline test time)`. As measured at chunk 2026-09-24-observability-gates, that was 20 s on a 1 s root-package baseline and 110 s on a 21 s baseline with `viola-e2e` in the diff.
      - So every in-test wait a root-package mutant can reach, and the 10 s kill, stay below 20 s. At or above it, a hang grades Timeout instead of caught, as run `35995290314` showed.
      - The `viola-e2e` override keeps a 30 s kill because the harness's own tests wait out its 20 s boot deadline by design.
    - `[test-groups] fixed-port = { max-threads = 1 }` plus `[[profile.default.overrides]] filter = 'test(/default_port/)'`, `test-group = 'fixed-port'` (catalog configuration). Overrides on the default profile are inherited by `ci` and `mutants`. Every test that touches port 47319 has `default_port` in its name. Without the override the group is empty and those tests race.
  - Dev-deps:
    - rstest 0.27, tempfile 3.27, assert_cmd 2.2.2, predicates 3.1.4, trycmd 1.2.1, insta 1.48.0, jsonschema 0.57.0, proptest 1.11.0, mockall 0.15.0, mock_instant 0.6.1
    - windows-sys 0.61.2 with `Win32_Security_Authorization` (cfg windows)
    - in `viola-e2e`: rmcp `=3.4.1` with `client` and `transport-child-process`, reqwest 0.13.5 (no compression features), eventsource-client 0.18.0, tokio 1.53.1 with `test-util`
  - Node side: `e2e-web/package.json` pinning `@playwright/test@1.63.0` exactly (the committed `package-lock.json` resolves `playwright` and `playwright-core` 1.63.0, all from registry.npmjs.org; `@axe-core/playwright@4.13.0` joins with the a11y chunks of Epoch 8), `e2e-web/tsconfig.json` (`noEmit`, `strict`), plus `e2e-web/playwright.config.ts` (catalog configuration):
      - `use: { headless: true }` and a single `chromium` project
      - `retries: 0` and `forbidOnly: true` (a stray `test.only` would silently skip the rest)
      - the test-scoped session fixture (§3 `run` step 3) declares its own fixture `timeout`. It is computed from boot's deadlines (20 s per `--instance` plus 10 s with `--ui`), plus a fixed margin for boot steps 1 and 4 and for teardown `cleanup` (10 s supervisor deadline). Boot time therefore never counts against the test body's timeout. A fixture timeout is a failure, never retried.
      - no `webServer`, and `globalSetup` limited to boot step 1 (§3 `run` step 3)
      - `reporter: [['json',{outputFile:'pw.json'}],['junit',{outputFile:'pw-junit.xml'}]]`, the two files `run` and `gate` read
- **5-command-discipline-wire:** wire `scripts/agent-run.{sh,ps1}` as shims over `viola-harness` (boot / run / status / cleanup / logs), as specified in 5-command implementation above. Binding contract: both shell variants must expose identical semantics.
- **status-endpoint-implement:** the product side (`/health`, `/ready`, `/api/info`, `/api/sessions`, `viola list --json`) is arch-owned. This phase implements the `agent-run status` aggregation per the Status endpoint shape above, and the `api_sessions_equal_list` comparison.
- **log-format-bind-with-obs:** implement §3 Log format above as the single source of truth.
  - The test plan owns the harness-grepped fields (`timestamp`, `level`, `target`, `message`, `event`, `process`, `instance`, `corr`) and the `logs` wrapper shape.
  - obs-plan §3 is a downstream reader. It may add fields, but renaming or removing one needs a Decisions Log entry in this plan first.
  - Do not wait for an obs-plan schema. The phase is done when `agent-run logs` output passes the `jq -e` / jaq assertions in §3 `logs`.
  - Binding contract: the tests harness greps logs for assertions, so a format break is a harness break.
- **pid-file-commitment-wire:** wire the `session.json` write at boot readiness, the pid + start-time verified kill, and the supervisor `stop.request` / `supervisor-exit.json` handshake at cleanup, per PID file above.
- **test-data-bootstrap-wire:**
  - `viola-fake-agent` bin (feature `fake-agent`), its receipt format and its scripted modes (§7)
  - the rstest fixture chain in `tests/support/`
  - the fixture scrub-and-schema walk
  - committed `proptest-regressions/`
  - the `cleanup` home removal
- **coverage-tooling-install:** install cargo-llvm-cov 0.9.1 via taiki-e/install-action. It emits `target/lcov.info` and `cargo llvm-cov report --json --summary-only`. Any test that uses `env_clear()` must re-add `LLVM_PROFILE_FILE`.
- **ci-tool-install:** every other CI-invoked binary, version-pinned:
  - cargo-mutants 27.1.0 and cargo-deny 0.20.2 via taiki-e/install-action v2.87.19 (SHA-pinned), alongside cargo-nextest and cargo-llvm-cov
  - `cargo install --locked` for hyperfine 1.20.0, cargo-modules 0.27.0 and zizmor 1.30.1
  - CI's log assertion G2 is `scripts/g2-zero-panics.sh`, a fail-closed script on the runner-provided `jq` (`tool-missing: jq` without it), run as `--probe` then the check in the `test` and `perf` jobs; each presence-checks jq (`jq --version`), which is never installed. It counts `event:"panic"` role lines under `target/e2e-home` and exempts only a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`. `scripts/release-check.sh` (the `release` job), `scripts/orphans-check.sh` (`lint`) and `scripts/npm-audit.sh` (`supply-chain`, nightly `npm-advisories`) use the same runner-provided `jq` and refuse with `tool-missing: jq` without it (runner images read at chunk 2026-09-24-workspace-tree-and-code-graph-planes: jq 1.8.1 on windows-2025, 1.8.2 on macos-15 arm64, 1.7 on ubuntu-24.04). jaq 3.1.1 remains a valid local assertion form (§3 `logs`).
  - ripgrep 15.2.0 (PCRE2), the G1/G3 gate tool, comes from `scripts/install-ripgrep.sh` in the `lint` job. The script downloads the official release asset, checks it against the sha256 its release publishes, installs it idempotently into `target/tools/ripgrep/bin`, and prints `tool-missing: <tool>` when a tool it needs is absent. taiki-e/install-action `7623a79…` has no ripgrep manifest.
  - cargo-fuzz 0.13.2 via `cargo install --locked`, in the ci.yml `fuzz-replay` and `nightly.yml` `fuzz` jobs only, with the nightly toolchain installed by `rustup toolchain install` from `fuzz/rust-toolchain.toml` (no toolchain action)
  - Node v24.21.0 via `scripts/install-node.sh` (the official nodejs.org build, sha256-checked against ci.yml's workflow `NODE_PIN_*` lines it parses from the file text; no setup-node) in the `test` and `supply-chain` jobs and nightly `npm-advisories`; then, in each OS's `test` job, `npm ci --prefix e2e-web` and `npx --no --prefix e2e-web playwright install chromium` (`--with-deps` on ubuntu). The native Linux `pre-push` checks the host's pinned Node (`~/.local/viola-node`, `NODE_PIN_VERSION`) and installs nothing.

  `agent-run run --perf` and `--fuzz-replay` check that their tool is on PATH first. If it is missing, they exit 1 with `reason:"tool-missing"` and the tool name, never a silent pass.
- **quality-gate-config-emit:** emit `.github/workflows/ci.yml` enforcing:
  - the coverage thresholds in §10 (lines 85 / functions 95 / regions 80, per OS)
  - `retries = 0`
  - hyperfine `max` gates
  - cargo deny (root graph and `fuzz/Cargo.lock`), zizmor and cargo-modules orphans (`scripts/orphans-check.sh`, per lib/bin target) exit-code gates, and the `release` job's `scripts/release-check.sh`
  - all actions SHA-pinned, with `permissions: {}` at the top level
  - `viola-harness gate --require <that job's suites>` as the last step of every job that runs a harness suite (§3 Internal harness subcommands); `lint`, `supply-chain` and `release` are plain exit-code jobs with no gate step

route uses this list to plan phase ordering (typically:
test-runner-install → 5-command-discipline-wire → status-endpoint-implement
→ log-format-bind-with-obs → pid-file-commitment-wire →
test-data-bootstrap-wire → coverage-tooling-install → ci-tool-install →
quality-gate-config-emit). setup-project uses this list to materialize
each phase's bootstrap script + dependency list + verification
command. Ownership:
- This plan is the binding source for the harness-grepped log fields and the `agent-run status` shape.
- The nested product shapes inside `status` come from the arch GUI HTTP contract.
- obs-plan §3 reads both and may extend them but not redefine them.

---
