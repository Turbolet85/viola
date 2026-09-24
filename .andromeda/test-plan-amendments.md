# test-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — harness builds in its own target dir
**Section:** §3 `boot` steps 1, 3, 5 · §3 `run` steps 1–3 (layer commands, globalSetup) · §3 `run` step 4 Test scope
**Change:**
- The harness's cargo work runs with `CARGO_TARGET_DIR=<root>/target/harness` and `--features viola/fake-agent`, and the CLI `<bin dir>` is `target/harness/debug`.
- The fake agent is copied from `<bin dir>`.
- JUnit is still read from `<root>/target/nextest/ci/junit.xml`.
- `viola-e2e` declares a no-op `fake-agent` feature for cargo-mutants' package-scoped runs.

**Why:** measured on Windows 2026-09-24. `os error 5` relinking the running `target/debug/viola-harness.exe`; `UnitDependencyInfoChanged` in cargo's fingerprint log (report Spec claims disproved 2; Deviations 1, 4). Sweep `target/<profile>|--features fake-agent|cargo build --workspace` over test-plan:
- amended: lines 515, 517, 519, 534, 537, 546, 557;
- no change:
  - 541 ("never a literal `target/<profile>`" still holds);
  - 555 (cargo-mutants passes the literal feature);
  - 558 (llvm-cov builds in its own target dir);
  - 1386 (the CI lint runs clippy directly, not through the harness);
  - 1452 (perf builds in `target/perf`).

## 2026-09-24-three-os-ci-headless-harness-skeleton — integration filterset, mutation diff, base and trigger
**Section:** §3 `run` step 1 (integration layer) · §3 `run` step 4 (Base, Diff) · §9 Pipeline structure (Mutation row)
**Change:**
- The integration layer is `kind(test)` until an E2E-prefixed binary exists.
- The mutation diff is the working tree plus untracked files from `merge-base(<base>, HEAD)`.
- The `mutants` job runs on push and pull_request, with base = PR base sha or else `github.event.before`, passed through `env:`.
- The `origin/main` fallback is recorded as never resolving on this project's single build branch.

**Why:** measured:
- nextest 0.9.133 rejects an unmatched `binary()` regex;
- the committed-only diff was 0 lines, against 10 309 working-tree lines;
- `git ls-remote --heads origin` shows only `build/viola-0.1.0`.

(Report Spec claims disproved 1, 3, 4; operator decision at phase P4.) Sweep `binary\(/\^\(path|<base>\.\.\.HEAD|merge-base HEAD origin/main|pull_request\.base\.sha` over test-plan:
- amended: lines 536, 553, 554, 1391;
- no change: 537 (the E2E layer keeps its `binary()` selector, which becomes valid once E2E binaries exist).

## 2026-09-24-three-os-ci-headless-harness-skeleton — interim supervisor, readiness, status and cleanup
**Section:** §3 preamble (Exit codes) · §3 `boot` step 5 + Readiness signal · §3 `run` step 2 (harness_session, harness cleanup meaning) · §3 `cleanup` step 1 + Verification · §3 `supervise` · §3 Status endpoint shape
**Change:**
- The supervisor is an ordinary child process and holds a stdin pipe per wrapper until `viola-pty` exists.
- Interim readiness: the role file holds `process-start` lines for `self` and `claude-child`, and both pids are alive.
- The status shape adds `instances[]`, and `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null` in the interim.
- The cleanup report adds `processes_gone`, and `endpoint_gone`, `port_free` and `url_file_removed` are `null` in the interim.
- The harness cleanup assertion now reads "no field false".
- A usage error carries `reason:"usage"` plus a `detail` code, and an unbuilt selector is a usage error.
- The key order `{"v","cmd","ok",…}` is held by serde_json `preserve_order`.

**Why:** report Harness / gate surface, Reverted facts (detach flags removed), Deviations 3 and 6, expected amendment 5, and the operator decision (the grammar grows per chunk). A §12 Decisions Log entry records the new closed values. Sweep `detached|every field \`true\`|endpoint_gone` over test-plan: lines 519, 541, 544, 589, 596, 611 and 612 amended; 0 remaining hits.

## 2026-09-24-three-os-ci-headless-harness-skeleton — nextest mutants profile and toolchain source
**Section:** §3 Bootstrap test-runner-install · §9 CI Integration (tool paragraph; Matrix builds → Language version)
**Change:**
- `[profile.mutants]` is `fail-fast = { max-fail = 1, terminate = "immediate" }` with a 15s×2 slow-timeout.
- rustfmt and clippy come from the `rust-toolchain.toml` components, installed by `rustup toolchain install`.
- The language version is the exact 1.98.1 pin.

**Why:** a caught mutant was graded Timeout under `fail-fast = true` (auto timeout 108 s, measured 2026-09-24); the chunk shipped the rustup step (report Deviation 5, Harness / gate surface; expected amendment 6). Sweep `dtolnay` over test-plan: lines 1398 and 1403 amended; 756 and 1393 no change (the nightly fuzz toolchain and the MSRV 1.96 job are separate toolchains).

## 2026-09-24-three-os-ci-headless-harness-skeleton — mutation gate prebuilds the root bins (post-commit CI fix)
**Section:** §3 `run` step 4 (Command)
**Change:** `run --mutants` first runs `cargo build --package viola --features fake-agent`, which fails with `reason:"build-failed"`, and then passes `--copy-target=true` to cargo-mutants, so the scratch tree carries the root `viola` / `viola-fake-agent` bins that the harness tests spawn.
**Why:** cargo-mutants 27.1 scopes the baseline to the packages the diff touches. A diff touching only `crates/viola-e2e` failed its baseline with `fake agent: NotFound`, exit 4, measured on the dev host 2026-09-24. `test_workspace` / `test_package` in `.cargo/mutants.toml` and `--test-workspace=true` did not widen that scope (measured). The operator chose prebuild + copy-target over moving tests or `--in-place` (founder-delegated, 2026-09-24). Cost: one `target/` copy per run, 2.9 GB on the dev host. Sweep `cargo mutants --workspace` over test-plan: the line-555 command was amended; 0 other hits. Leaf `.claude/docs/commands.md` re-derived.
