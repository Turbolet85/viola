# architecture — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — toolchain floor and exact pin
**Section:** §Stack and Technologies (Language / runtime) · §Infrastructure Patterns (Build system; Project directory structure `rust-toolchain.toml` comment) · §Inherited Defaults (Framework)
**Change:** Rust 1.98.1 is pinned exactly by `rust-toolchain.toml` (rustfmt + clippy). The workspace `rust-version` is 1.96 (was 1.89). Was "1.89 is the highest floor"; now "1.96 is the declared floor", with sysinfo 1.95, `File::lock` 1.89, rmcp 1.88 and axum 1.80 listed below it.
**Why:** the chunk shipped `rust-version = "1.96"` and `channel = "1.98.1"`.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — CI setup and wired jobs
**Section:** §Stack and Technologies (CI/CD row) · §Infrastructure Patterns (CI/CD approach)
**Change:**
- CI installs the toolchain with `rustup toolchain install` from `rust-toolchain.toml` (no toolchain action; was `dtolnay/rust-toolchain@stable`) and uses SHA-pinned actions/checkout 7.0.1, Swatinem/rust-cache 2.9.2, taiki-e/install-action 2.87.19 and actions/upload-artifact 7.0.1.
- The workflow sets `permissions: {}` at the top and `contents: read` per job.
- Two jobs are wired today: `test` (3-OS harness unit/integration plus the lifecycle leg, with an `agent-run-<os>` upload) and `mutants` (ubuntu, push + PR, base via `env:`).
- The six target jobs are kept and marked as owned by later chunks.
**Why:** the chunk shipped `.github/workflows/ci.yml` in this shape.
**Kept:** the nightly fuzz toolchain and the MSRV 1.96 job install a different toolchain and are unchanged; the mutable-ref ban still holds.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — test-only crate, bins, env vars, paths
**Section:** §Established Decisions [Module Boundaries] · §Occupied Resources (Binary; Workspace crates; Environment variables; Filesystem; Repository) · §Conventions (Naming — environment variables) · §Infrastructure Patterns (Project directory structure)
**Change:**
- Registered the test-only crate `viola-e2e` (harness library + `viola-harness`, no-op `fake-agent` feature) and the test-only bins `viola-fake-agent` (root `[[bin]]`, feature `fake-agent`) and `viola-harness`.
- Registered the harness-only env vars `AGENT_RUN_CHUNK_BASE` and `AGENT_RUN_KEEP_HOMES`, and the `AGENT_RUN_` prefix for harness-only variables.
- Registered the home-level `diagnostics/` (`run-<name>.ndjson`, 0700/0600) and the repository paths `target/agent-run/`, `target/e2e-home/` and `target/harness/`.
- Added `src/bin/viola-fake-agent.rs`, `tests/`, `crates/viola-e2e/`, `scripts/agent-run.{sh,ps1}` and `.config/nextest.toml` to the tree.
- Module Boundaries states that each product crate is created by its first consumer.
**Why:** the chunk shipped these crates, bins, env vars and paths, none of which the registry named.
**Kept:** the child env the harness sets (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) is not registered: the arch registry tracks only variables the product reads or sets.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — run's process log location, logging and serialization rows
**Section:** §Cross-cutting Patterns (Diagnostic output channels) · §Stack and Technologies (Serialization; new Logging row)
**Change:**
- `run`'s codes-only process log is the home-level `diagnostics/run-<name>.ndjson` (was: its diagnostics go to the instance); content-bearing detail stays in the instance's `diagnostics/`, per obs D-08.
- New Stack row: tracing 0.1.44 + tracing-subscriber 0.3.23 (root only).
- serde_json carries `preserve_order` (indexmap), so printed documents keep their declared key order.
**Why:** `viola run` appends the home-level role file; the chunk added tracing, tracing-subscriber and serde_json `preserve_order`; the spec's key-order claim was disproved.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-fake-agent-and-test-data-fixtures — test homes, test env vars and test-side paths registered
**Section:** §Occupied Resources (Environment variables · Filesystem · Repository) · §Infrastructure Patterns directory tree
**Change:**
- Env vars: `AGENT_RUN_KEEP_FAILED` registered (read only by the root test chain). `AGENT_RUN_KEEP_HOMES` is now read by `viola-harness` and the root test chain (was: only `viola-harness`). None is read by `viola`.
- e2e-home: `viola-session-*/home` (harness) and `viola-test-*/home` (root rstest) both named (was: `viola-session-*` held every test home).
- Repository gains `fixtures/fake-scripts/`, `schemas/fake-script.v1.json` and `crates/viola-core/proptest-regressions/`.
- Filesystem gains the test-home-only `fake/<name>.control` / `fake/<name>.receipt.ndjson`.
- The tree gains `tests/support/`, `schemas/`, `fixtures/fake-scripts/` and viola-core `proptest-regressions/`.
**Why:** root tests use `viola-test-*` homes and the root test chain also reads `AGENT_RUN_KEEP_HOMES`, disproving both earlier claims; the chunk added the new files and Wrapper::boot paths.
**Kept:** `tests/support/` is not a Repository entry (registry over-reach; the tree carries it); no §Stack Testing row, because [Deferred] hands test libraries to test-plan, which pins them.
**Ref:** .andromeda/runs/2026-09-24T08-45-42-wrap/

## 2026-09-24-supply-chain-and-workflow-gates — sole-root tokio ban, four-family deny policy, nightly.yml
**Section:** §Stack Code-quality row · §Established Decisions [Concurrency] · §Occupied Resources → Repository · §Infrastructure Patterns → Build system · directory tree · CI/CD approach
**Change:**
- Build system: the tokio ban is `deny-sync.toml`, run once per crate in `scripts/sync-crates.txt` as the sole root (was: checked over a graph with the async crates excluded). The `viola-channel` own-root limit is recorded. `deny.toml` carries four families: advisories, licences, sources, and bans (C, telemetry, feature).
- [Concurrency] enforcement text names the sole-root ban and the sync-crate list.
- Code-quality row: cargo-deny families corrected; zizmor 1.30.1 added (workflow lint).
- Occupied Resources: `target/deny-probes/` and `target/supply-chain/` registered.
- Tree: `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh` and `workflows/nightly.yml` added; the `deny.toml` comment is corrected.
- CI/CD: two workflows (`ci.yml` push/PR, `nightly.yml` weekly + dispatch; was a single workflow). `ci.yml` jobs are `test`, `mutants`, `lint`, `supply-chain`. Target job 3 reads `scripts/sync-crates.txt`. zizmor is installed by `cargo install --locked`.
**Why:** `--exclude` false-fails under feature unification, which falsified the excluded-graph mechanism. The operator placed the weekly run in `nightly.yml`. The chunk's config files go to the tree, not the registry (playbook "Registry over-reach").
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-diagnostics-plane — diagnostics roots, config key, diag schemas, v exception
**Section:** §Stack and Technologies (ORM / migrations row) · §Established Decisions [Hook Contract] · §Occupied Resources → Filesystem (`config.json`, `diagnostics/`, `instances/<ViolaName>/…` rows) · §Occupied Resources → Repository (`schemas/` rows) · §Infrastructure Patterns → Project directory structure (`schemas/` comment) · §Cross-cutting Patterns → Config management (Settings) · §Cross-cutting Patterns → Diagnostic output channels
**Change:**
- **`v` rule:** was "every record carries `v`"; now it names its exception: process-log lines carry no `v`, and their version lives in the schema filename.
- **Logging destinations:** `hook` logs to the home-level `diagnostics/hook-<name>.ndjson`, with content-bearing detail only in the instance's `detail-hook.ndjson`. Every role's codes-only log goes to its home-level role file (`run-`/`hook-`/`mcp`/`ui-`/`cli-`). `mcp` falls back to stderr JSON only when its file cannot be opened.
- **Registry and settings:**
  - The `config.json` row registers `diagnostics_level` (info | debug), the `MAX_FRAME`-capped read that requires `v` = 1, and the `parse-rejected` report.
  - The home `diagnostics/` row lists all five role files; only `run` has a producer today.
  - The instance `diagnostics/` row names the owner-only `detail-<role>.ndjson` files.
  - `schemas/diag-line.v1.json` and `diag-detail.v1.json` are registered in the Repository rows and the tree.
  - The Settings bullet names `diagnostics_level` as the only source of the level; `RUST_LOG` has no effect.
**Why:** the chunk shipped these facts and disproved the earlier destination claims. The `v` exception is also the operator's direction.
**Kept:** obs-plan §1 keeps its pending wording, as a verbatim copy of obs-scope.
**Ref:** .andromeda/runs/2026-09-24T10-40-06-wrap/

## 2026-09-24-log-redaction-and-never-log-floor — anyhow's scope includes the catch-site reporter
**Section:** §Stack and Technologies (Error types row, :27) · §Established Decisions [Error Handling] (:95)
**Change:** anyhow 1.0.104 stays in the `viola` bin only, now named as `main`, dispatch (`cmd::dispatch` returns the error with the resolved home and instance) and the catch-site reporter `viola::obs::report_internal_error` (was "only at dispatch" / "`main` and dispatch"). Context chains are "built at dispatch and recorded only in the owner-only instance detail file" (`chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`), never a role line, stdout or stderr. The rationale now reads "only useful where a person reads them: the owner's post-mortem".
**Why:** the chunk routed the dispatch error's `chain()` through `src/obs.rs` into the detail file, which obs-plan §7 scrubbing layer 3 already required; the old wording left the reporter outside the decision. Still inside the root bin crate; no dependency added.
**Kept:** obs-plan §1 still quotes the retired "context chains only at dispatch", as the verbatim obs-scope copy kept by rule.
**Ref:** .andromeda/runs/2026-09-24T11-37-52-wrap/

## 2026-09-24-observability-gates — lint bans, gate tools, new target/ paths, scan-gated CI uploads
**Section:** §Stack and Technologies (Code quality row) · §Occupied Resources → Repository · §Infrastructure Patterns → Build system (Lint), Project directory structure, CI/CD approach (Setup steps, Jobs wired today, target job 2)
**Change:**
- The Code quality row names:
  - the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros;
  - ripgrep 15.2.0 for G1/G3;
  - runner `jq` for G2;
  - jsonschema 0.57.0 for the harness `schema-check`.
- Registered paths: `target/secret-scan/hits.json`, `target/tools/ripgrep/bin/`, `target/lint-probes/`.
- The tree gains `clippy.toml`, `scripts/lint-probes.sh` and `scripts/install-ripgrep.sh`.
- The Lint bullet and target job 2 carry `--features fake-agent` and the bans (was featureless `--all-targets -- -D warnings`); the raw `event!` grep and `tests/contract_lints.rs` are named.
- Setup steps name `scripts/install-ripgrep.sh` (install-action has no ripgrep manifest) and runner `jq`.
- `test` job: follows the obs-plan §9 gate order with scan-gated uploads, replacing the unscanned `agent-run-<os>` upload.
- `lint` job: wires target jobs 1–2 plus the ripgrep install, G1 and G3, and the lint probes on Linux.
**Why:** the chunk shipped these gates and tools. New gitignored `target/` resources are registered in the same form as `target/deny-probes/`.
**Ref:** .andromeda/runs/2026-09-24T13-07-17-wrap/

## 2026-09-24-quality-gates — fuzz workspace, coverage and fuzz tooling, MSRV job, two-leg mutation, download-artifact
**Section:** Stack and Technologies (Language / runtime · CI/CD · Code quality rows) · Established Decisions [Module Boundaries] · Occupied Resources (Workspace crates · Repository) · Infrastructure Patterns (Build system Dependency policy · Project directory structure · CI/CD approach: workflows, Setup steps, Jobs wired today)
**Change:**
- `fuzz/` (`viola-fuzz`) is a separate cargo workspace excluded from the root (`exclude = ["fuzz"]`), registered with its toolchain file, targets, corpus and gitignored outputs. It is not a member.
- `artifacts/` also holds `llvm-cov-summary.json` and `mutants-verdict-<leg>.json`; `target/lcov.info` is registered.
- The runtime row records the CI `msrv` check (rustup 1.96 + `RUSTUP_TOOLCHAIN`) and the fuzz-only `nightly-2026-09-20`.
- The Code quality row adds cargo-llvm-cov 0.9.1 and cargo-fuzz 0.13.2 plus the fuzz lockfile exemption.
- The CI/CD row and Setup steps add actions/download-artifact 8.0.1 (5 pinned actions; was four) and the new installs.
- The Dependency policy bullet records `fuzz/Cargo.lock` outside `cargo deny`: the operator-ratified test-only exemption, with its audit owed to "Workspace tree and code-graph planes".
- CI/CD: `nightly.yml` runs advisories + fuzz (was advisories only), with no `concurrency:` block (declined, why). The 7 `ci.yml` jobs are `test` (coverage + gate), the two-leg `mutants` matrix, `mutants-verdict` (union), `msrv`, `fuzz-replay`, `lint` and `supply-chain`.
- The tree shows `fuzz/`, the root `exclude` and the workflow comments.
**Why:** the chunk shipped the fuzz workspace, coverage, MSRV job and two-leg mutation. The operator ratified the fuzz-lock exemption at wrap, with its audit carried to "Workspace tree and code-graph planes".
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-24-workspace-tree-and-code-graph-planes — tree lists every test/obs/a11y artifact, release-check, orphans gate, fuzz lock audit, code-graph planes
**Section:** Stack and Technologies (Code quality row) · Occupied Resources → Repository · Infrastructure Patterns (Build system: Dependency policy, Boundary review, a new Code-graph planes bullet · Project directory structure · CI/CD approach: nightly sentence, Setup steps, jobs wired, target job 6)
**Change:**
- Code quality: cargo-modules 0.27.0 via `cargo install --locked` in `lint`; the orphans gate runs in CI, the dependency graph is reviewed on demand. `jq` is also used by `scripts/release-check.sh` and `scripts/orphans-check.sh`.
- Repository: `target/supply-chain/` names `deny-fuzz.json`, admissible by content (obs-plan §8 item 6); new `target/orphans-probes/run-<utc>-<pid>/`, `target/release-check/`, `target/perf/` (hyperfine exports are the separate `perf/*.json`), `target/nextest/ci/junit.xml`, `e2e-web/test-results/` with `a11y/` and `lint/`.
- Build system: the fuzz lockfile audit is wired (ci.yml `supply-chain` advisories + sources into `deny-fuzz.json`; weekly nightly advisories), run from the repo root, where cargo-deny finds the root `deny.toml` by walking up. Boundary review: `cargo modules orphans --deny` per lib/bin target via `scripts/orphans-check.sh` (+ `--probe`) is a CI gate; the `--acyclic` graph is on-demand. The rmcp `cargo tree -e features` check lands with "MCP server for drivers". Code-graph planes: rust = the root workspace via rust-analyzer SCIP (every member; `fuzz/` outside); ts = `e2e-web/` only via a tracked `e2e-web/tsconfig.json` (noEmit, strict); nothing under `crates/viola-ui/` is ever covered by a tsconfig, package manifest or bundler config.
- Tree: `tests/cmd/*.toml` (trycmd), `tests/snapshots/`, `crates/viola-ui/assets/` (`index.html`, `app.css`, vendored Lit 3.3.3 ESM), `scripts/release-check.sh`, `scripts/orphans-check.sh`, `e2e-web/` with its 10 entries, `a11y/sr-pass/`, updated ci.yml / nightly.yml comments.
- CI/CD: nightly `advisories` covers both lockfiles (was root only); Setup steps add the jq consumers and the cargo-modules install route (no taiki-e manifest); 8 jobs wired, 15 check-runs per push (was 7 / 12): `lint` gains the orphans steps, `supply-chain` the fuzz lockfile audit, new `release` job (plain exit-code, no gate step); target job 6 is `cargo build --release --locked --bin viola` through `scripts/release-check.sh`, judged on the build's own artifact records, never a `target/release/` listing.
**Why:** the chunk shipped these files and gates and disproved three spec claims; the tree inventory went from 25 missing rows of 45 to 0. The operator decided the layout at P4 and ratified the obs §8 upload admissibility at wrap.
**Ref:** .andromeda/runs/2026-09-24T16-23-20-wrap/

## 2026-09-24-epoch-1-cleanup — the mutation union is no longer the only red path
**Section:** §Infrastructure Patterns → CI/CD approach (jobs wired today, the `mutants-verdict` sentence)
**Change:** after "a mutant is red only when no leg caught it and some leg missed it or timed out", the body adds that before that union a leg is red at its own run when its unviable mutants outnumber its caught ones (test-plan §3 `run` step 4, §10 Mutation gate). Was: the union was the only red path.
**Why:** the sentence cites test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker).
**Ref:** .andromeda/runs/2026-09-25T11-29-18-wrap/

## 2026-09-25-security-prerequisites — SQOS client open, sha2 content hash, licence MIT OR Apache-2.0, 0BSD exceptions
**Section:** §Stack and Technologies (Wrapper IPC row; new Content hash row); §Established Decisions [Message Broker / IPC]; §Occupied Resources → IPC endpoints (test-only pipe); §Infrastructure Patterns → Build system (shared `[workspace.package]` inheritance, Dependency policy licences, new Licence bullet), Project directory structure (root `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`), Crate dependency direction (`viola-channel` + windows-sys on Windows); §Inherited Defaults → Publishability.
**Change:**
- The Wrapper IPC row and the IPC decision record the Windows client open: windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by `Stream::try_from`, never the default connect. A non-overlapped handle hangs.
- A Content hash row names sha2 `=0.11.0` (`default-features = false`) for the 16-hex `<hash>` keys, a root dev-dependency until `viola-state`.
- `viola-channel`'s planned third-party deps gain windows-sys (Windows only).
- The licence `MIT OR Apache-2.0` is recorded in `[workspace.package]` + `license.workspace = true`, as a literal in `fuzz/Cargo.toml`, by the root texts, and in Publishability.
- Dependency policy records the licence `allow` list and the per-crate `0BSD` exceptions (`doctest-file`, `recvmsg`), never through `allow`.
- The tree gains the three root files.
- The test-only pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` is registered, outside the `viola-<h12>` namespace.
**Why:** the chunk landed the SQOS open, the hash crate and the licence files. The licence is a founder ruling (2026-09-25), routed to every arch project-metadata site; the 0BSD exceptions are an operator-ratified boundary widening (security-plan sidecar, same marker).
**Kept:** the PTY layer row's windows-sys role is not widened: its `TerminateProcess` fallback in `viola-pty` stays true; the SQOS use belongs to the channel, in the IPC row and the dependency direction.
**Ref:** .andromeda/runs/2026-09-25T13-11-43-wrap/

## 2026-09-25-pty-wrapper-on-windows — PTY wrapper as built: R8 prefix rule + identity floor, viola-side program resolution, HostTerminal, PtyError exception
**Section:** Stack and Technologies (PTY layer, Error types) · Established Decisions [PTY], [CLI Version Compatibility] ledger row, [Error Handling], [CI/CD] · Conventions (CLI exit codes, Rust error types) · Occupied Resources (Workspace crates, Environment variables, Filesystem `config.json`, Repository orphans-probes + ripgrep probe dirs) · Infrastructure Patterns (Build system Lint, code-graph rust plane, Licence; Crate dependency direction viola-pty / viola-agent-claude / shared deps / root bin; project tree `viola-pty`) · Cross-cutting Config management (Settings) · Inherited Defaults (Errors)
**Change:**
- R8 is now a `CLAUDE*` prefix rule with a persistent-environment exemption (Windows registry `Environment` value names, Unix `config.json` `claude_env_keep`) and an 11-name identity floor; the ledger row is the identity floor, no longer a strip list; `config.json` + Settings register `claude_env_keep`.
- [PTY] records the as-built host-terminal raw mode (`HostTerminal`), explicit child cwd, viola-side program resolution and the kept ConPTY input writer; [CI/CD] no longer says the child is "spawned as given"; exit 1 covers the `batch-script-child` refusal.
- `viola-pty` deps as landed: portable-pty, windows-sys (Windows), libc (Unix), no thiserror; `viola-agent-claude` depends on thiserror only; root bin + windows-sys (registry reader). Both crates are registered as landed, with a code-free `fake-agent` feature, in the plane member list and the licence list.
- `PtyError` is the one ratified exception to "thiserror per crate" (hand-written fixed `Display`/`Error`).
- Lint records the root bin's one local `print_stderr` allow (the refusal fn); Repository registers `target/tools/ripgrep/probe-*` and the 2-case orphans probe set.
**Why:** the chunk landed the wrapper this way. The operator ratified the R8 rule as a boundary widening and the PtyError exception at wrap.
**Kept:** "environment variables are not a configuration channel" is not qualified: the registry read is R8's input, names only, not configuration. "While the child runs, `run` writes nothing except the child's own output" stays true.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/

## 2026-09-26-ci-chunk-base-and-union-verdict — derived whole-chunk mutation base, compiling-leg union, syn/proc-macro2 in viola-e2e
**Section:** Stack and Technologies (Code quality row) · Occupied Resources → Environment variables (`AGENT_RUN_CHUNK_BASE`) · Occupied Resources → Repository (`target/agent-run/`) · Infrastructure Patterns → Crate dependency direction · Infrastructure Patterns → CI/CD approach (concurrency rationale; `test` job `harness-<os>` upload; `mutants` legs; `mutants-verdict` union)
**Change:**
- `AGENT_RUN_CHUNK_BASE` is an explicit override of the derived base; CI does not set it.
- `mutants` legs: step "Mutation leg (whole chunk)", only `LEG` through `env:`, base derived by the harness (last master flip before the oldest pre-CI commit, `HEAD^` on the wrap push), the run document names `base`.
- `mutants-verdict`: the union judges each mutant only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`), every leg when none does.
- `harness-<os>` uploads `target/agent-run/` minus `target/agent-run/chunk.diff`; the Repository row records that `chunk.diff` is the one file `secret-scan` skips.
- The no-concurrency reason keeps the `always()` gate/upload chain and retires the "drop a push's `--in-diff` mutation diff" half (every run now covers the whole chunk).
- syn 2.0.119 + proc-macro2 1.0.107 (`span-locations`) registered in the Code quality row and as `viola-e2e`'s dependency-direction entry.
**Why:** the chunk shipped the derived whole-chunk base, the compiling-leg union and the syn/proc-macro2 dependency. security-plan restated the retired concurrency reason and was amended in the same pass.
**Ref:** .andromeda/runs/2026-09-26T20-59-23-wrap/

## 2026-09-26-local-linux-pre-push-gate — local pre-push gate, `FAKE_AGENT_PUMP_DELAY_MS` test seam, `target/pre-push/`
**Section:** §Stack CI/CD row · §Established Decisions [Naming] · §Conventions Environment variables · §Cross-cutting Config management · §Occupied Resources → Environment variables, Repository · §Infrastructure Patterns → Project directory structure, CI/CD approach
**Change:**
- CI/CD row + CI/CD approach: before the operator push, `viola-harness pre-push` runs the ubuntu test suites and the `ubuntu-latest` leg in WSL2 `Ubuntu` (provisioned by `scripts/wsl-provision.sh` from ci.yml's pins), the `windows-2025` leg on the host and CI's own union with equal bases; the operator-pass order (stop rust-analyzer → `pre-push` on the uncommitted tree → pre-CI commit → guarded push → CI reads; red stops the pass); CI stays the verdict of record.
- [Naming], Conventions, Config management, env registry: one ratified exception to the `VIOLA_` / `AGENT_RUN_` rule — the test seam `FAKE_AGENT_PUMP_DELAY_MS`, read only under `cfg(feature = "fake-agent")` (`hold_pump_start`, `src/cmd/run.rs`), u64 ms capped at 5 000, absent from release builds, configures nothing.
- Repository: `target/pre-push/` (the sync's temporary index and binary patch; outside `target/agent-run/` because `secret-scan` scans that tree).
- Project tree: `viola-e2e` names the internal subcommands incl. `pre-push` (`harness::pre_push`); `scripts/wsl-provision.sh`.
**Why:** the chunk shipped the local pre-push gate. The seam is a boundary widening touching a locked decision ([Naming]), ratified by the operator's founder-delegated wrap direction as a carve-out behind the `fake-agent` feature, capped at 5 s, absent from release builds.
**Kept:** the WSL host-side state (`~/viola-pre-push`, `~/.rustup`, `~/.cargo`) and the test temp trail `viola-resize-<pid>.ndjson` are not registered (registry over-reach). The `AGENT_RUN_CHUNK_BASE` registry and the `mutants` job state the base CI derives, which never meets an uncommitted promotion, and stay true.
**Ref:** .andromeda/runs/2026-09-27T00-51-08-wrap/

## 2026-09-27-instance-state-and-start-order — tempfile `persist` snapshot writer, liveness by process check, start order as landed, `viola-state`
**Section:** §Stack (State-file primitives, Content hash) · §Established Decisions [Snapshot writer], [Session Liveness], [Deployment / Distribution] · §Conventions CLI exit codes (`1`) · §Standard Contracts Instance snapshot, Session liveness · §Occupied Resources Workspace crates, Filesystem (`bin/`), Repository (`target/e2e-home`) · §Infrastructure Patterns Build system (lint, rust plane, licence), Crate dependency direction (`viola-state`, `viola-agent-claude`), CI/CD approach · §Inherited Defaults Database
**Change:**
- [Snapshot writer] + State-file primitives + Database: tempfile 3.27.0 `persist` through ONE shared helper (`viola_state::fs::replace_private`, mode set on the temp file before the write; `replace_private_shared` for identical concurrent writes); was atomic-write-file 0.3.1 (BSD-3-Clause, outside `deny.toml`). `.lock` siblings opened for write (Windows refuses the lock on an append-only handle).
- [Session Liveness] + Session liveness + exit `1`: pid + start time decide `gone` whatever the beat; same process → `live` (≤ 5 s) or `stale`; a `live`/`stale` name refuses (`already-live`), a `gone` one is taken over at once; start order as landed (resolution → collision → pin + plugin → [version gate, bind: later] → snapshot + heartbeat → events → spawn → `child_pid` rewrite).
- Instance snapshot: `endpoint?` (from "Wrapper channel"), `child_pid?` (post-spawn rewrite), `started_at` = the wrapper's OS start time.
- [Deployment] + `bin/`: re-hash before reuse, mismatch refuses (`pinned-hash-mismatch`); plugin rewritten every start, `hooks`/`mcpServers` empty until their verbs (operator ruling).
- Content hash: sha2 a `viola-state` product dependency; `[profile.dev.package.sha2] opt-level = 3`.
- `viola-state` landed (sync, 6 crates), in the rust plane and the licence list; its dependency line as landed; agent-claude as-landed symbols + `PLUGIN_DIR_FLAG`, `plugin_files`.
- Lint: the root bin carries no print allow (was: one local allow, already false); the start refusals use `writeln!` on the locked stderr through one `refuse` helper.
- `target/e2e-home`: owner record + gone-owner sweep; not kept under `run --mutants`. CI/CD: pre-push Linux-leg `TMPDIR` scratch; `run --mutants` keeps no home.
**Why:** the chunk landed `viola-state` and the start order. [Snapshot writer] is a locked-decision reversal settled by the operator's ruling and the overseer's wrap direction.
**Kept:** the Claude Code integration names are not re-registered: they stay true, and their fact moved to [Deployment].
**Ref:** .andromeda/runs/2026-09-27T06-12-23-wrap/

## 2026-09-27-wrapper-channel — viola-channel landed: Unix endpoint path and arbiter, frame rules, bind step, dependencies
**Section:** §Stack (Wrapper IPC, Logging) · §Established Decisions [Session Liveness], [Error Handling] · §Conventions (CLI exit code 1) · §Standard Contracts (Instance snapshot, Wrapper channel frames) · §Occupied Resources (IPC endpoints, Workspace crates, Repository `fuzz/`) · §Infrastructure Patterns (code-graph rust plane, Licence, Crate dependency direction, directory tree, local pre-push gate)
**Change:**
- IPC endpoints, Unix: was `$TMPDIR/viola-<h12>.sock`; now `<socket dir>/viola-<h12>.sock` in the per-user 0700 dir (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, fallback `/tmp/viola-<uid>/`; security-plan arch amendment 2 folded), chmod 0600 after the bind, the `<socket dir>/viola-<h12>.lock` sibling as the start arbiter. Test-only `viola-test-chan-<pid>-<label>` registered beside `viola-test-sqos-*`.
- Frames: one line ≤ `MAX_FRAME` (`\n` counted), longer → `-32600` + close; only `hook.event` id-less, any other id-less frame not dispatched (`parse-rejected{channel-frame, malformed}`); `conn` stripped before dispatch, never identity.
- Start order: the endpoint bind has its step (taken → exit 1 `squatted-name`, fixed stderr pair); `endpoint` is present from the first snapshot; only the version gate still has no step.
- Stack: tracing gains `attributes`; veil `=0.3.0` (no `toggle`); the server DACL converted by windows-sys `Win32_Security_Authorization`; libc for the Unix socket dir.
- `viola-channel` joins Landed so far, the sync crates, the rust plane and the licence list; its deps + serde, serde_json, thiserror, tracing, veil, libc (sync, tokio-free as landed). `viola-pty` gains tracing, so [Error Handling]'s exact set is portable-pty, tracing, windows-sys, libc.
- Fuzz targets `viola_name` and `channel_frame`. Pre-push: `vm-release` (WSL VM terminated after the ubuntu verdict) and `windows-tests` on the host; host stages at `CARGO_BUILD_JOBS=16`.
**Why:** the wrapper channel chunk landed the crate and its bind. Detector proposals citing source or manifest lines were rejected and their report-carried facts re-raised by the orchestrator; the collateral facts they carried (dir fallback on a relative value, lock mode, unlink order, dependency versions) were not written.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
