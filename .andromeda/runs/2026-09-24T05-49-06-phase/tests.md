# tests extract

## Relevance
Relevant. This chunk is the plan's Foundation test infrastructure. It covers test-plan §Bootstrap phases test-runner-install, 5-command-discipline-wire, pid-file-commitment-wire, ci-tool-install, and the mutation/CI parts of quality-gate-config-emit.

## Constraints
- **Harness contract (per test-plan §3 Test Harness Contract, preamble and §3 Internal harness subcommands → Closed enums).**
  - `scripts/agent-run.{sh,ps1}` must be byte-identical thin shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command>`, with all logic in Rust.
  - Every command must print exactly one `{"v":1,"cmd":…,"ok":…}` document and exit 0, 1 or 2.
  - Failure `reason` values and the nine `suite` values are closed enums. Adding a value needs a §12 Decisions Log entry.
  - This chunk can build the skeleton only as far as the §3 `boot` / `status` reasons allow: `build-failed`, `run-exited`, `readiness-timeout`, `unknown-session` and so on. Research and P4 must decide how a "not yet built" step reports without inventing an off-enum reason.
- **Session record and cleanup (per test-plan §3 PID file, §3 `cleanup`).**
  - Session record: `target/agent-run/<session>/session.json`.
  - Homes: created under `target/e2e-home/viola-session-*`, and passed as a path that does not exist yet so viola creates it (§3 `boot` step 2).
  - Any kill target must be checked by pid + start time via sysinfo 0.39.6 before signalling.
  - `cleanup` must be idempotent: with no session it returns `{"ok":true,"cleaned":[]}` and exit 0.
  - Under `AGENT_RUN_KEEP_HOMES=1`, `cleanup` keeps the home and reports `home_removed:"kept"`.
  - Whether the scaffold's `viola` binary already creates `--home` with 0700 and the Windows protected DACL is a research question.
- **`run` and the mutation step (per test-plan §3 `run` step 4, §10 Mutation gate).**
  - Base: `AGENT_RUN_CHUNK_BASE`, else `git merge-base HEAD origin/main`. An unreachable base must exit 1 with `reason:"base-missing"` and must never write an empty `chunk.diff`.
  - Verdict comes from `mutants.out/outcomes.json` and requires `missed == 0 && timeout == 0`. Exit codes 1/4/5/6/70 map to `reason:"mutants-exit-<code>"`. Only exit 0, 2 or 3 can pass.
  - No `--test-workspace` flag: a mutant in `viola-e2e` or the root must be killed by that package's own tests.
  - A tests-only diff reports `"mutants":{"tested":0}`.
- **Unbuilt `run` selectors (per test-plan §3 `run`, §11 Universal).**
  - Suites not built in this chunk (`--coverage`, `--perf`, `--fuzz-replay`, `--browser`) must be refused with a JSON reason: `tool-missing`, or `browser-linux-only` with exit 2 off Linux. They must never pass silently.
  - The `run` output shape is `suites[{suite,passed,failed,skipped,survived,artifact,failures[]}]`.
  - `skipped` counts only JUnit `<skipped/>` entries. It never counts filterset exclusions or cfg-excluded tests.
- **nextest config (per test-plan §3 Bootstrap phases → test-runner-install).** `.config/nextest.toml` must have:
  - `[profile.ci]` with `junit.path = "junit.xml"`, `retries = 0`, `slow-timeout = { period = "30s", terminate-after = 4 }` and `fail-fast = false`
  - `[profile.mutants]` with `fail-fast = true`
  - `[test-groups] fixed-port = { max-threads = 1 }`, plus a default-profile override `filter = 'test(/default_port/)'`
- **CI workflow (per test-plan §9 CI Integration, §9 Matrix builds).**
  - Matrix: `windows-2025`, `macos-latest`, `ubuntu-latest`, with `fail-fast: false`.
  - `permissions: {}` at the top and `contents: read` per job. Every `uses:` pinned by full SHA.
  - Tools come from taiki-e/install-action v2.87.19 as `cargo-nextest@0.9.146,cargo-mutants@27.1.0`. rustfmt and clippy come from dtolnay/rust-toolchain (SHA-pinned).
  - `rust-toolchain.toml` pins one exact stable version ≥ 1.96. The host is updated to the same pin with `rustup` (§9 Language version; §12 fix pass entry I2/I3).
  - Windows legs run `scripts/agent-run.ps1`.
  - The ubuntu PR mutation job uses `fetch-depth: 0` and `AGENT_RUN_CHUNK_BASE=${{ github.event.pull_request.base.sha }}`.
  - Artifacts: `target/agent-run/artifacts/` uploaded as `agent-run-${{ matrix.os }}` with upload-artifact v7.0.1.
- **Per-role JSON line (per test-plan §3 Log format).**
  - Process logs go to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`, one JSON line per `write`. Files are 0600 and dirs 0700.
  - Required fields: `timestamp` (RFC 3339 UTC, ms, `Z`), `level`, `target`, `message`, `event` (closed kebab enum, e.g. `process-start`), `process`.
  - A null `instance` or `corr` must be written as an absent key, never a literal `null`.
  - A panic must produce one `level:"ERROR", event:"panic"` line and never a multi-line trace.

## Patterns to follow
- **Shared library, thin shims (per test-plan §3 `run` step 2).** Put boot and cleanup in `viola_e2e::harness::{boot, cleanup}` so the later rstest `harness_session` fixture calls the same code. Nextest tests never shell out to `agent-run` / `viola-harness`, so there is never a nested `cargo run`.
- **Bounded readiness polling (per test-plan §3 `boot` Readiness signal).** Readiness polls at 100 ms against a deadline (20 s per instance, 10 s for the UI). This counts as a file-state probe, not sleep-based synchronisation.
- **`logs` wrapper shape (per test-plan §3 `logs`).**
  - `{"src":"events","instance","offset","record"}`
  - `{"src":"diag","file","record"}`
  - Torn lines become `{"src":…,"torn":true,"offset":n}` and are never dropped.
  - Output must be `jq -e` / jaq-assertable.
- **Test file layout and naming (per test-plan §2 Test directory + naming conventions).**
  - The fake agent lives at `src/bin/viola-fake-agent.rs` as a root `[[bin]]` with `required-features = ["fake-agent"]`.
  - The harness lives in `crates/viola-e2e` with `publish = false`.
  - Test names follow `<subject>_<condition>_<expected>`.
  - Unit tests go in an inline `#[cfg(test)] mod tests`.
- **Child environment (per test-plan §7 Test data lifecycle, §3 `boot` step 3).** Set env and PATH per child with `Command::env`. Any `env_clear()` must re-add `LLVM_PROFILE_FILE` (§3 Bootstrap coverage-tooling-install).

## Anti-patterns to avoid
- **Retries and masked verdicts (per test-plan §11 CI, §11 Quality).**
  - No nextest `retries` above 0.
  - No mutable-tag Actions such as `@v2` or `@main`.
  - No cache other than Swatinem/rust-cache in `ci.yml`.
  - Never trust the cargo-mutants exit code alone.
- **Unsafe waits and env mutation (per test-plan §11 Universal, §11 Integration).**
  - No `sleep` used for synchronisation.
  - No `std::env::set_var`, which is unsafe in edition 2024.
  - No harness command may print a human-only message without the JSON document and exit code.
- **Real CLI in CI (per test-plan §11 Test Strategy).** Never run the real `claude` in CI. `--local-live` must refuse with exit 2, `reason:"live-in-ci"`, when `CI` is set.

## Contract bindings
- **tests ↔ obs, log format (per test-plan §3 Bootstrap phases → log-format-bind-with-obs).** The tests plan owns the harness-grepped fields and the `logs` wrapper shape. obs-plan §3 may add fields but not rename them. The panic-hook-first ordering comes from obs-plan §3.
- **tests ↔ obs, CI homes (per test-plan §3 `cleanup` step 6).** CI homes are kept under `target/e2e-home/**` via `AGENT_RUN_KEEP_HOMES=1` for obs-plan §9 gates G2/G4 and the secret scan. Those gates are later chunks, but the home placement must be right now.
- **tests ↔ security (per test-plan §9 CI Integration).** SHA pins, `permissions: {}` and the 1.96 toolchain floor are shared with security §Dependency Security. cargo-deny and zizmor are deferred to the Supply-chain chunk.
- **tests ↔ arch (per test-plan §12 initial entry, Test crate deviation).** The test-only crate `crates/viola-e2e` is a deviation from arch's "No separate test crate is declared". It still needs the arch tree amendment, which is scoped to the Workspace-tree chunk.

## Acceptance criteria contributions
- `scripts/agent-run.sh cleanup` and `scripts/agent-run.ps1 cleanup` with no session each print `{"v":1,"cmd":"cleanup","ok":true,"cleaned":[]}` and exit 0. An unknown flag exits 2 with one JSON document. (per test-plan §3 `cleanup` Idempotency, §3 Exit codes)
- `agent-run run --mutants` with an unreachable `AGENT_RUN_CHUNK_BASE` exits 1 with `reason:"base-missing"` and writes no `chunk.diff`. With a valid base, the verdict reads `outcomes.json` `missed == 0 && timeout == 0`. (per test-plan §3 `run` step 4, §10 Mutation gate)
- CI legs on `windows-2025`, `macos-latest` and `ubuntu-latest` all run the build and the harness `run` green. zizmor-style inspection shows no unpinned `uses:`, and `.config/nextest.toml` has `retries = 0` in `ci`. (per test-plan §9 Matrix builds, §3 Bootstrap phases → test-runner-install)
- `agent-run logs` emits at least one `{"src":"diag",…}` line whose `record` passes `jq -e '.timestamp and .level and .target and .message and .event and .process'`, with no literal `null` for `instance` / `corr`. (per test-plan §3 Log format, §3 `logs`)

## Relevant amendment history
(none). `D:/dev/projects/viola/.andromeda/test-plan-amendments.md` does not exist.

Two in-plan §12 Decisions Log entries touch this area:
- Fix pass 1, I2/I3: MSRV 1.96, with one exact stable version pinned in `rust-toolchain.toml`.
- Fix pass 2, B1: CI homes kept via `AGENT_RUN_KEEP_HOMES=1` under `target/e2e-home/` for obs gates.

Research should also check one inconsistency: the initial §12 entry still says libtest "Rust 1.95.0".
