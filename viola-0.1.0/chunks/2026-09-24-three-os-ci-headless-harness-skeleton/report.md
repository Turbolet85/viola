# Report — 2026-09-24-three-os-ci-headless-harness-skeleton

**Chunk:** 3-OS CI + five-command headless harness skeleton, minimal fake agent, mutation gate
**Date:** 2026-09-24T07:10:00Z
**Commits:** none since `last_wrap` (null): HEAD `13b7ee3` "chore(setup-project): configure Claude Code for viola"; this wrap's commit carries the chunk

## Changes (structured — detectors read this)
- **Files:** 25 new, 0 modified (`git status --short` at wrap Setup; `.gitignore` and `scripts/agent-run.{sh,ps1}` untouched):
  - `rust-toolchain.toml`
  - `Cargo.toml`, `Cargo.lock`
  - `.config/nextest.toml`
  - `.github/workflows/ci.yml`
  - `crates/viola-core/{Cargo.toml, src/lib.rs}`
  - `src/main.rs`, `src/cmd/{mod.rs, run.rs}`, `src/run/mod.rs`, `src/bin/viola-fake-agent.rs`
  - `tests/run_cli.rs`
  - `crates/viola-e2e/{Cargo.toml, src/lib.rs, src/bin/viola-harness.rs, src/harness/{mod,boot,run,status,cleanup,logs,supervise}.rs, tests/harness_lifecycle.rs, tests/cli.rs}`
- **Symbols / APIs:**
  - **`viola` bin**
    - Global `--home <dir>`; home resolution is `--home`, then `<user home>/.viola` (the `VIOLA_DIR` step is not built).
    - Verb `viola run <name> -- <program> [args]`:
      - the name goes through `ViolaName::try_new` via a clap value parser (invalid → exit 2);
      - it creates the home and `diagnostics/` (Unix 0700) and appends `diagnostics/run-<name>.ndjson` (Unix 0600);
      - it logs `process-start` twice (`subject:"self"` with `service_name`/`version`/`os`/`pid`, and `subject:"claude-child"` with `child_pid`);
      - it spawns the child directly (no shell, stdio inherited, no env added) and waits on the handle;
      - it logs `process-exit` for the child (`child_exit_status`, `exit_source:"handle-wait"`), then `process-exit` for self (`exit_code` 0);
      - a spawn failure writes `process-exit` self at ERROR (`exit_code` 1, `detail:"internal-error"`) and exits 1.
    - Panic hook `viola_panic_hook` is the first statement of `main`; `catch_unwind` at the main catch site gives exit 1. The hook writes one JSON line via a `OnceLock` role file, with no stderr, no payload, a workspace-relative `panic_location` and `thread`.
    - No other verbs.
  - **`viola-fake-agent`** (feature `fake-agent`): `--version` prints `<ver> (Claude Code)` (`--cli-version`, default `2.1.0`); any other invocation reads stdin until `\x03` or EOF, then exits 0.
  - **`viola-harness`** (in `viola-e2e`):
    - `boot [--session][--instance <name>[:<args>]]…[--cli-version]` (default instances `overseer`,`builder`; no `--ui`/`--unstamped`/`--agents-mode`/`--statusline-echo`)
    - `run [--unit|--integration|--mutants|--all][--filter]` (`--e2e`/`--browser`/`--coverage`/`--perf`/`--fuzz-replay`/`--local-live` are usage errors)
    - `status [--session]`
    - `cleanup [--session|--all]`
    - `logs [--session][--instance][--process]` (no `--kind`/`--after`; diag source only)
    - internal `supervise --session`
    - Every document starts `{"v":1,"cmd":…,"ok":…}`; exits 0/1/2.
  - **`viola-harness` records and reasons:**
    - Harness files:
      - `target/agent-run/<session>/{session.json, supervise.json, stop.request, supervisor-exit.json, bin/claude[.exe]}`
      - `target/agent-run/{chunk.diff, artifacts/{junit-<suite>.xml, run-summary.json}}`
      - `target/e2e-home/viola-session-*/home`
      - `target/harness/` (the harness's own cargo target dir)
    - Failure reasons used: `build-failed`, `run-exited`, `readiness-timeout`, `unknown-session`, `usage`, `base-missing`, `mutants-exit-<code>`. The `usage` detail codes are `invalid-session-id`, `session-exists`, `invalid-instance`, `arguments` and `no-supervise-spec`.
  - **Env vars read (harness only, never the product):** `AGENT_RUN_CHUNK_BASE`, `AGENT_RUN_KEEP_HOMES`. Set on children: `PATH` (session bin first), `CARGO_TARGET_DIR` (harness builds), `NEXTEST_PROFILE=mutants`.
  - **CI:** workflow `ci.yml` with jobs `test` (3-OS matrix) and `mutants` (ubuntu).
- **Crates / modules:** added:
  - `viola-core` (`ViolaName`, `SERVICE_NAME`, `VERSION`; no error enum);
  - `viola-e2e` (test-only harness lib + bin `viola-harness`; declares a no-op `fake-agent` feature);
  - root `viola` bin modules `cmd`, `cmd::run`, `run`.
- **Dependencies** (all exact `=` pins in `[workspace.dependencies]`, crates.io only; `Cargo.toml`@wrap):
  - anyhow 1.0.104 (root only)
  - chrono 0.4.45 (`clock`,`std`)
  - clap 4.6.7 (`derive`)
  - nutype 0.8.0
  - serde 1.0.229 (`derive`)
  - serde_json 1.0.151 (`preserve_order`, which pulls indexmap)
  - sysinfo 0.39.6 (default-features off, `system`)
  - tempfile 3.27.0
  - thiserror 2.0.20
  - tracing 0.1.44 (default off, `std`)
  - tracing-subscriber 0.3.23 (default off, `fmt,json,registry,std`; root only)
  - No tokio, no C-building crate, and no assert_cmd, rstest or proptest yet.
- **Schema / config:**
  - Root `Cargo.toml`:
    - `[workspace]` members `crates/*`, `resolver = "3"`;
    - `[workspace.package]` has edition 2024, `rust-version = "1.96"`, version 0.1.0, `publish = false`;
    - `[workspace.lints.rust] unused_must_use = "deny"`;
    - `[profile.dev|release] panic = "unwind"`.
  - `rust-toolchain.toml` sets `channel = "1.98.1"` with rustfmt and clippy.
  - `.config/nextest.toml`:
    - `[profile.ci]`: retries 0, fail-fast false, slow-timeout 30s×4, junit `junit.xml`;
    - `[profile.mutants]`: `fail-fast = { max-fail = 1, terminate = "immediate" }`, slow-timeout 15s×2;
    - `fixed-port` group plus the default-profile override.
  - Role-file line fields: `timestamp` (RFC 3339 UTC ms Z), `level`, `target` (`viola::…`), `message == event`, `event ∈ {process-start, process-exit, panic}`, `process:"run"`, `instance`, and the additive fields listed above. `corr` is never written. The level filter is `Targets` with `viola` at INFO; third-party targets are off.
- **Spec-master edits:** none (implement and phase write no master).
- **Counts / qualifiers moved:**
  - The workspace `rust-version` is now 1.96. Arch states 1.89 at 3 sites and obs mentions 1.89 at 2 (`grep -cE '1\.89'` per master: architecture=3, obs-plan=2, others 0).
  - The workspace crate count is now 3 (`viola`, `viola-core`, `viola-e2e`); arch §Occupied Resources lists 8 target crates, and `viola-e2e` is absent from it (`grep -c viola-e2e architecture.md` = 0).
- **Dev-tool versions:** rust toolchain (compiler, dev host Windows 11) changed from 1.95.0 (default `stable-x86_64-pc-windows-gnu`) to 1.98.1 (`rustup toolchain install` from `rust-toolchain.toml`, 2026-09-24; installed msvc). Unchanged re-reads:
  - cargo-nextest 0.9.133 on the dev host (the CI pin is 0.9.146, installed by taiki-e);
  - cargo-mutants 27.1.0 (the CI pin);
  - jq present;
  - jaq absent;
  - rg absent from the gate shell PATH (exit 127 at the P5 baseline).
- **Harness / gate surface:**
  - `viola-harness` behind the unchanged setup-seeded shims.
  - The status shape per test-plan §3 has `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` as `null`, plus an added `instances:[{name,wrapper_pid,alive}]`.
  - The cleanup report has `processes_gone`, with `endpoint_gone`, `port_free` and `url_file_removed` as `null`.
  - Interim boot readiness means:
    - the role file has `process-start` self and `process-start` claude-child lines;
    - both pids are alive, checked by pid + start time.
  - The supervisor is an ordinary child process (no detach flags): it pipes stdin to each `viola run` and stops wrappers by writing `\x03` into that pipe, then closing it.
  - The harness builds into `target/harness/` (`CARGO_TARGET_DIR`), and boot's bin dir is `target/harness/debug`; the reason is under Deviations.
  - The unit layer is `kind(lib) | kind(bin)`. The integration layer is `kind(test)`, without test-plan's `!binary(/^(path|…)_/)` exclusion.
  - `run --mutants` diff = working tree + untracked files from `merge-base(base, HEAD)`.
  - CI:
    - SHA-pinned `actions/checkout` v7.0.1, `Swatinem/rust-cache` v2.9.2, `taiki-e/install-action` v2.87.19 and `actions/upload-artifact` v7.0.1;
    - `permissions: {}` at the top and `contents: read` per job;
    - `rustup toolchain install`, no toolchain action;
    - `AGENT_RUN_KEEP_HOMES=1` on the `test` job;
    - `agent-run-${{ matrix.os }}` upload of `target/agent-run/artifacts/`;
    - the `mutants` job on push and pull_request with `AGENT_RUN_CHUNK_BASE` via `env:` = PR base sha || `github.event.before`;
    - no fmt/clippy/deny/zizmor/MSRV/coverage/release/gate steps.
- **Cross-project / external claims:**
  - Action SHAs were resolved 2026-09-24 via `gh api repos/{r}/commits/{tag}` (chunk research.md).
  - Rust 1.98.1 is current stable (`rustup check`).
  - The runner labels come from the `actions/runner-images` README (`macos-latest` = macOS 26 arm64, `windows-2025` = Server 2025).
  - No CI run exists yet for this chunk: the plan's operator entries (commit + push, then the check-runs read) are this wrap's P7.
- **Reverted / negative API facts:**
  - `CoreError` and the `VIOLA_DIR` child env were planned, not written, and have no consumer yet.
  - The supervisor detach flags (`process_group(0)`, `CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW`) were written, then removed: they had no observable effect a test could pin, so they were an unkillable mutant.
  - `--features fake-agent` in the harness's own cargo calls became `--features viola/fake-agent` (see Deviations).
- **Insufficient fixes (written, kept, not the remedy):** none. The first exe-lock fix (`viola/fake-agent` scoping) is kept and correct; it avoids feature-driven relinks, but workspace-wide dependency unification still relinked `viola-harness`. The remedy was the dedicated `target/harness` dir.
- **Spec claims disproved by measurement:**
  1. test-plan §3 `run` step 1, line 536: the integration filterset `kind(test) & !binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)` is rejected by cargo-nextest 0.9.133 while no binary matches (`error: operator didn't match any binary names … failed to parse filterset`, implement gate 5 log). The exclusion can only exist once an E2E binary does.
  2. test-plan §3 preamble line 506 + `boot` step 1 line 515 + `run` step 1: the shim runs `viola-harness` from `target/debug` via `cargo run`, and boot/run then build the workspace into the same target. On Windows the running `viola-harness.exe` cannot be relinked (`failed to remove file …viola-harness.exe … Access is denied (os error 5)`, implement gate 4 log). Measured cause: `cargo test --no-run --workspace` marks `viola-harness` dirty via `UnitDependencyInfoChanged` (fingerprint log, `.andromeda/runs/2026-09-24T06-07-48-implement/fp.log`). The harness now builds into `target/harness`, and test-plan §3's `<bin dir>` = `target/<profile>` (3 sites, `grep -c 'target/<profile>' test-plan.md`) is `target/harness/debug` for the CLI.
  3. test-plan §3 `run` step 4, line 553: `git diff <base>...HEAD` sees only committed work, and /implement never commits, so the gate would mutate an empty diff. Measured: the chunk diff is 10 309 lines as working tree + untracked (`wc -l target/agent-run/chunk.diff`) versus 0 committed.
  4. test-plan §9 Mutation row (line 1390) and §3 line 553: a PR-only job with `origin/main` merge-base never fires on this project's single build branch (`git ls-remote --heads origin` shows only `build/viola-0.1.0`).
  5. test-plan §3 "Every command prints one JSON document starting `{"v":1,"cmd":…,"ok":…}`": key order held only after serde_json `preserve_order`. The default sorts keys (`"cmd"` first, `"ok"` after `"instances"`), which failed the plan's `contains "cmd":"boot","ok":true` atom. The fix is in code; the spec is unchanged and correct.
- **Expected amendments (from plan):**
  1. architecture §Stack and Technologies / §Inherited Defaults, rust-version 1.89 → 1.96 and an exact toolchain pin: **carried** (Schema/config + Counts; sites: `grep -cE '1\.89'` architecture=3, obs-plan=2; `channel = "stable"` architecture=1).
  2. architecture §Infrastructure Patterns "CI/CD approach", SHA-pinned actions + rustup step instead of `dtolnay/rust-toolchain@stable`: **carried** (Harness/gate surface; `grep -cE 'rust-toolchain@stable|dtolnay/rust-toolchain'` architecture=2, security-plan=2, test-plan=4).
  3. architecture §Occupied Resources, crate `viola-e2e`, bins `viola-harness`/`viola-fake-agent`, feature `fake-agent`, env `AGENT_RUN_*`, paths `target/agent-run/`, `target/e2e-home/`, plus `target/harness/` (new this chunk): **carried** (Crates + Symbols; `grep -c` in architecture: viola-e2e=0, viola-harness=0, viola-fake-agent=0, AGENT_RUN_KEEP_HOMES=0, target/agent-run=0, target/e2e-home=0).
  4. test-plan §3 `run` step 4 + §9 Mutation row, the push trigger / `github.event.before` base / working-tree diff: **carried** (Spec claims disproved 3–4; `merge-base HEAD origin/main` test-plan=1, `pull_request.base.sha` test-plan=2, `<base>...HEAD` test-plan=1).
  5. test-plan §3 `boot`/`status`/`cleanup`, interim readiness, `null` product fields, `processes_gone`: **carried** (Harness/gate surface).
  6. test-plan §9 toolchain from `rust-toolchain.toml` via rustup: **carried** (Harness/gate surface; test-plan `dtolnay/rust-toolchain` hits = 4).
- **Coverage of new surfaces:**
  - `viola run` (verb; external input = name + argv) → validation ✓ (`ViolaName::try_new` via clap) · instrumentation log ✓ (process-start/-exit, panic) · PII n/a (no env or argv logged; tested with a canary) · tests unit+integ ✓ · a11y n/a · tokens n/a
  - `viola --home` path → validation ✗ (not canonicalised or strict-modes-checked; owned by the home-integrity chunk) · instrumentation n/a · PII n/a · tests integ ✓ · a11y n/a · tokens n/a
  - panic hook → validation n/a · instrumentation log ✓ · PII redacted ✓ (payload excluded; absolute dep paths trimmed) · tests unit ✓ · a11y n/a · tokens n/a
  - `viola-fake-agent` (test-only) → validation n/a · instrumentation n/a · PII n/a · tests integ ✓ · a11y n/a · tokens n/a
  - `viola-harness` five commands + supervise (test-only) → validation ✓ (session id charset, `ViolaName` instances) · instrumentation n/a (harness documents) · PII n/a · tests unit+integ ✓ · a11y n/a · tokens n/a
  - `ci.yml` → validation n/a · instrumentation n/a · PII n/a (no diagnostics or homes uploaded) · tests unrunnable-here (runs on push) · a11y n/a · tokens n/a

## Deviations from intent
1. **The harness builds in `target/harness`, not `target/`.** The Windows running-exe lock forced this (Spec claims disproved 2). The CLI's `boot` bin dir is `target/harness/debug`. JUnit is still read from `<root>/target/nextest/ci/junit.xml`, because the nextest store is workspace-root-relative whatever the target dir (measured).
2. **The integration filterset drops the E2E exclusion** until the first E2E binary exists (Spec claims disproved 1).
3. **Three plan items were not built:** `CoreError` (plan step 3), `VIOLA_DIR` on the child (step 5) and the supervisor detach flags (step 8). Each has no consumer, or no observable effect, so each would be an unkillable mutant under the chunk's own zero-missed gate. The supervisor outlives `boot` as an ordinary child (measured on Windows: lifecycle and smoke green through the shims).
4. **`viola-e2e` declares a no-op `fake-agent` feature,** because cargo-mutants passes `--features fake-agent` to package-scoped runs. For the same reason, the harness's own cargo calls use `viola/fake-agent`.
5. **nextest `[profile.mutants]` gained terminate-immediate fail-fast and a 30 s slow-timeout.** Without them, a caught mutant that hangs sibling tests was graded Timeout (`expired -> false`, first run).
6. **Test-only additions:**
   - `crates/viola-e2e/tests/cli.rs` (not in the plan's list);
   - throwaway temp Cargo projects (named `viola`, in the system temp dir) that test run/boot's cargo orchestration without a nested workspace build;
   - a stray-process test replacing "dead supervisor leaves orphans". The premise was false: killing the supervisor closes the wrapper's stdin pipe, so the tree exits cleanly.
7. **The fake agent shares the root package's `[lints]`.** Plan step 6's own `[lints.clippy]` table is impossible for a `[[bin]]` of the root package; no print-ban lint exists yet.
8. **Product lines use raw `tracing::info!`/`error!`,** not `obs_event!`; that macro and the `disallowed-macros` ban belong to the Diagnostics-plane and Observability-gates chunks.
9. **The windows protected DACL for `--home` is deferred** to the home-integrity chunk, per the plan's Constraints.

## Decisions & corrections
- **Operator (P4, founder-delegated):** the walking-skeleton `viola run` is the first slice of the real verb in the same crate and on the same spawn seam, with no abstraction over emptiness. The mutation gate fires on push + PR. The run/boot grammar grows per chunk (unbuilt selectors are usage errors).
- **Windows host:**
  - A running `target/debug/*.exe` cannot be relinked by a nested `cargo build/test --workspace`. Even with identical features, workspace-wide dependency unification re-fingerprints a package's bin (`CARGO_LOG=cargo::core::compiler::fingerprint=info` shows `UnitDependencyInfoChanged`). A tool that runs cargo on its own workspace needs a separate `CARGO_TARGET_DIR`.
  - `Monitor`/`TaskStop` on a `tail -F` pipeline leaves `tail.exe`/`grep.exe` running on Windows. The open handles blocked cargo-mutants' `mutants.out → mutants.out.old` rename (Access denied, twice). Watch cargo-mutants output with a handle-free poll (`cat` per interval), never `tail -F`.
  - `rg` is not on the gate shell's PATH (Git Bash `bash -c`); use `grep -rE` in gate commands.
- **Sweep hazard:** a pipeline count probe `grep … | grep -Evc …` under `pipefail` reads green (`exit 1`, last line `0`) when the file is MISSING: the first grep's exit 2 is masked by the second's exit 1. Count with a single process that errors on a missing input.
- **nextest:** `fail-fast = true` waits for running tests after the first failure, so a mutant that hangs sibling tests is graded Timeout by cargo-mutants (auto timeout 108 s here). Use `fail-fast = { max-fail = 1, terminate = "immediate" }` for the mutation profile.
- **nextest:** `binary()` / `binary_id()` regex operators that match no binary are a filterset PARSE error, not an empty set.
- **serde_json:** the default `Map` sorts keys; a contract that fixes the key order of a printed document needs `preserve_order`.

## Outcome
- **Acceptance criteria, re-asserted against the diff:**
  - Root manifest is bin + workspace, resolver 3, edition 2024, `publish = false`, `panic = "unwind"`, lock committed; only `viola-core` + `viola-e2e` crates: **met** (Cargo.toml; `crates/` holds 2 dirs).
  - anyhow only in root; tokio nowhere; fake agent feature-gated: **met** (`grep -rn 'anyhow\|tokio' crates/*/Cargo.toml` → 0).
  - Toolchain 1.98.1 pinned, `rust-version = "1.96"`, host builds with it: **met**.
  - nextest profiles: **met.** `retries = 0` and `fail-fast = false` in ci; the mutants profile additionally has terminate-immediate and a slow-timeout (deviation 5). No `#[ignore]` (`grep -rc '#\[ignore\]'` = 0).
  - `run --unit` / `--integration` single documents with the suite fields and closed suite names: **met.**
  - Lifecycle leg boot → status ready → logs process-start without `corr` → cleanup `processes_gone` → cleanup `cleaned:[]`, no SGR: **met.**
  - `harness_lifecycle.rs` ready → alive → diag line → teardown → idempotent → degraded: **met** (5 tests: also force-kill and readiness-timeout/abort).
  - Mutation gate (`verification-matrix.json#v1-06`): **met locally.** Base-missing exits 1 with no `chunk.diff`; the 13b7ee3 run shows survived 0 (286 mutants: 244 caught, 42 unviable, 0 missed, 0 timeout) over a diff including the panic hook and role logger; the guard tests kill their guards within that run. The CI clause is pending the P7 operator push + check-runs read.
  - Role-file line fields: **met** (`tests/run_cli.rs::run_with_fake_agent_writes_start_and_exit_lines`).
  - Panic line single, payload-free, exit decided at catch site: **met** for the line (`src/main.rs` tests). The exit-1-at-catch-site path is exercised only by construction; no product panic trigger exists.
  - G3 no-abort probe exits 1 with no output: **met.**
  - Pin audit 0 unpinned; `permissions: {}` + `contents: read`; base via `env:`: **met.**
  - Unix modes 0700/0600, no `CLAUDE*` or argv in lines: **met** for the canary/argv assertion (all OSes). The mode assertion is a `#[cfg(unix)]` test that runs on the CI Linux/macOS legs only.
  - CI checks all `success` on the operator push: **pending P7.**
- **Gates** (implement run `.andromeda/runs/2026-09-24T06-07-48-implement`, final tree):

  | Gate (`run`) | Verdict |
  |---|---|
  | `cargo build --workspace --features fake-agent` | green |
  | `cargo fmt --all --check` | green |
  | `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` | green |
  | `bash scripts/agent-run.sh run --unit` | green, 82 passed (run-summary.json) |
  | `bash scripts/agent-run.sh run --integration` | green, 21 passed |
  | `bash scripts/agent-run.sh cleanup --session gate-smoke` (pre-clean) | green |
  | `… boot --session gate-smoke --instance builder` | green |
  | `… status --session gate-smoke` | green, `state:"ready"` |
  | `… logs … \| jq -e …` | green |
  | `… cleanup --session gate-smoke` | green, `processes_gone:true` |
  | `… cleanup --session gate-smoke` (again) | green, `cleaned:[]` |
  | `AGENT_RUN_CHUNK_BASE=0000… … run --mutants` | green, exit 1 `base-missing` |
  | `test ! -e target/agent-run/chunk.diff` | green |
  | `AGENT_RUN_CHUNK_BASE=13b7ee3… … run --mutants` | green, survived 0, 1149.5 s |
  | pin-audit python probe | green, last line 0 |
  | G3 `grep -rEn …` | green, exit 1 no output |
  | `git diff --quiet && … git push origin build/viola-0.1.0` | leg operator, P7 |
  | `gh api …/check-runs …` | leg operator, P7 |
- **Smoke:** the default two-instance boot through `scripts/agent-run.ps1` (not a gate) was ready. status was ready, logs showed overseer + builder process-start, cleanup reported `processes_gone`, and a second cleanup was empty.
- **Outcome basis:** implement's P4 report as given in this session's conversation.
- **Process hygiene:**
  - Everything this chunk started has terminated: the harness supervisors, `viola run`, the fake agents, cargo-mutants, nextest and the scratch test binaries. Re-measured at wrap Setup: no process with an executable under `D:\dev\projects\viola\` or a `cargo-mutants-viola*` scratch.
  - Three monitor pipeline processes (tail 14096, grep 50388, grep 54492) were stopped by pid during implement.
  - Two `viola.exe` processes belong to `viola-lab/prototype`, not this chunk.
