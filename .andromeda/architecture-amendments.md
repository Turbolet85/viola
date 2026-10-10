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

## 2026-09-27-epoch-2-cleanup — host mutation scratch, run archive, test-support feature, wsl-exec.sh, release-check feature refusal
**Section:** §Occupied Resources (Workspace crates, Environment variables, Filesystem, Repository) · §Infrastructure Patterns (Project directory structure: `scripts/`; CI/CD approach: target job 6, the local pre-push gate bullet)
**Change:**
- Workspace crates: viola-channel's test-only `test-support` feature (`pub mod test_support`), enabled only by the root `[dev-dependencies]`.
- Env vars: `AGENT_RUN_KEEP_FAILED` was "read only by the root test chain"; now also read by viola-e2e's `harness_lifecycle` `Booted` guard.
- Filesystem: test-only `<temp dir>/viola-pty-watch/<test name>.report`.
- Repository: `target/run-archive/<n>/` (newest 10, never uploaded) and the host mutation scratch `<repo parent>/viola-mutants-scratch/` (guard, wipe, `TMP`/`TEMP` + `--output`, path never printed).
- Tree: `scripts/wsl-exec.sh` (operator aid; no gate/harness/plan runs a command through it); `release-check.sh` also refuses a test-only feature.
- CI/CD: job 6 fails first on a `test-support` / `fake-agent` artifact (probe `5/5`); the pre-push bullet names the Windows leg's host scratch; the operator pass no longer starts with stopping rust-analyzer (as measured: three pre-pushes and two scoped runs with it running).
**Why:** the Epoch 2 cleanup chunk as built. `wsl-exec.sh` is a boundary widening ratified live by the overseer at this wrap under the founder's 2026-09-27 ruling (security-plan Decisions Log). Rejected: registering the split submodules in the tree (registry over-reach) and naming cargo-machete in §Stack (the code audit's instrument, not a project gate).
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/

## 2026-09-27-browser-verdict-reachability — the browser pipe: Node pin, npm audit, install sites, root-watch reports
**Section:** §Stack and Technologies (CI/CD row; new Browser e2e row; Code quality `jq`) · §Occupied Resources (Environment variables; Filesystem; Repository) · §Infrastructure Patterns → Project directory structure · §Infrastructure Patterns → CI/CD approach
**Change:**
- A test-side Browser e2e row: Node v24.21.0 (the official build, sha256-pinned by ci.yml's `NODE_PIN_*`, `scripts/install-node.sh`; never runner-image Node or a setup action) + `@playwright/test` 1.63.0 (3 locked packages) and its Chromium; `npm ci` and the Playwright CLI spawned directly; axe lands with the a11y chunks.
- WSL provisioning adds the pinned Node and Chromium; Chromium's system libraries are an operator-only root install (`--install-deps`).
- Registered: the `NODE_PIN_*` workflow data lines (never read from the environment by viola or viola-harness); `<temp dir>/viola-root-watch/`; the install sites `$RUNNER_TEMP/node`, `~/.local/viola-node/`, `~/.cache/viola-provision/e2e-web/`, `~/.cache/ms-playwright/`; `target/npm-audit/audit.json`; `e2e-web/node_modules/`, `pw.json`, `pw-junit.xml`, `test-results/` (written now, was "land with the Web UI chunks"); `junit-<os>` carries 2 paths; run-archive holds the playwright JUnit.
- Tree: `scripts/install-node.sh`, `scripts/npm-audit.sh`, `e2e-web/package-lock.json`, `stub/pipe.html`; `package.json` pins `@playwright/test` only (was also `@axe-core/playwright`); workflow comments.
- CI/CD: `nightly.yml` runs three jobs (was two; + `npm-advisories`); Node via `install-node.sh`; the `test` job runs the browser suite and gates `coverage,doctest,playwright` (was `coverage,doctest`); `supply-chain` adds the npm lockfile audit; pre-push's Linux leg checks `node` and runs `run --browser` + the playwright gate.
**Why:** founder ruling W125 (the pipe proven before any feature needs it); P4 operator forks 1–3; the root launch ratified live by the overseer, operator-only.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-27-hooks-to-normalised-events — seven exec-form hooks, `hook.event` served, 750 ms hook deadline, paste pair, replace retry
**Section:** [Snapshot writer] · [Hook Transport] · [CLI Version Compatibility] (paste-wrapper and tag-escaping rows) · [Deployment / Distribution] · §Standard Contracts (frame example; Channel methods) · §Occupied Resources (Claude Code integration names; Workspace crates; Filesystem `diagnostics/`; Repository) · Crate dependency direction · the directory tree
**Change:**
- `hooks.json` registers seven exec-form entries (`"command": "@@VIOLA_BIN@@"`, `"args": ["hook", "<kebab event>"]`): SessionStart / UserPromptSubmit / Stop `"timeout": 5`, SessionEnd no timeout key, Notification / PostToolUse / PostToolUseFailure `"async": true`; the dialog tier joins with the dialog chunk. Was: `{"hooks": {}}` until the `hook` verb exists; only `.mcp.json` stays empty.
- The spine deadline, "an open item", is now provisionally the hook-local, crate-private `SPINE_DEADLINE` = 750 ms (one value for every event, the `connect_by` deadline); naming the product constant stays with "Readiness gate and timing constants". SessionEnd keeps its ~1 s budget: the channel under the same 750 ms, else `try_append_event` (`Source::Hook`, writes nothing while the lock is held), else nothing.
- Paste row: the CLI's own pair is unescaped, `<pasted_content id="X">\n…\n</pasted_content id="X">` (same id), unwrapped only as that exact pair. Tag escaping: a typed close arrives as `<\/`; order = classify `harness` on the raw prefix → unwrap → un-escape `<\` before an ASCII letter or `/`.
- `hook.event` is a served method: `{ts, event:{kind, data}}`, re-validated (5 hook kinds, object `data`, `prompt-submitted` `text`/`origin`), appended with `source:"hook"`; `-32603` on an append failure. The frame example carries `data`.
- [Snapshot writer]: on Windows a `persist` refused with raw OS error 5 retries every 10 ms, at most `REPLACE_ATTEMPTS` = 100, re-persisting the same temp file.
- `diagnostics/`: `run` and `hook` have producers (was only `run`). Repository: `crates/viola-agent-claude/proptest-regressions/`, fuzz target `hook_stdin` (10 seeds), `target/mutants/` (the mutation run's own target dir). `viola-fuzz` also depends on `viola-agent-claude`. `viola-agent-claude` as landed: `viola-core`, serde, serde_json, serde_path_to_error `=0.1.20`, thiserror (was thiserror only).
**Why:** the hooks chunk as built; the 750 ms value stays provisional because it is crate-private and unmeasured by a perf gate; the retry is the chunk's CARRY 5 (a reader held `snapshot.json` open during the SessionStart hook's read).
**Ref:** .andromeda/runs/2026-09-27T23-42-19-wrap/

## 2026-09-28-hook-perf-gate — the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`
**Section:** §Established Decisions [Naming]; §Conventions Environment variables; §Occupied Resources → Environment variables (Test seams); §Cross-cutting Patterns Config management
**Change:**
- Was one ratified exception (`FAKE_AGENT_PUMP_DELAY_MS`); now two test seams, each read only under `cfg(feature = "fake-agent")` and absent from release builds.
- `FAKE_AGENT_HOOK_PANIC`: `fn panic_if_asked` in `src/cmd/hook/seam.rs` (module declared without a cfg), called in `hook()` right after `viola_obs_init`, before the stdin lock; panics only on exactly `1` with the fixed 4 608 B payload `"forced-hook-panic ".repeat(256)` → one codes-only `panic` role line + one `detail-hook.ndjson` line over 4 KiB, exit 0, empty streams. Named in product source only in that file; configures nothing, disables no control, widens no redaction.
**Why:** the fail-open contract proven on a real panic in the real binary. A boundary widening, ratified by the founder live on 2026-09-28 (the seam at 06:21; the seam with G2's exact-path exemption at 09:52:07; relay the Viola overseer).
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-hook-perf-gate — perf arm registry, G2 script, the `perf` CI job, hyperfine in §Stack
**Section:** §Stack and Technologies Code quality; §Occupied Resources Repository (`target/agent-run/`, `target/perf/`, `target/g2-probe/`); §Infrastructure Patterns directory tree (`scripts/`, `ci.yml`); §CI/CD Setup steps and Jobs wired today
**Change:**
- `target/perf/` builds with `--features viola/fake-agent` (was `fake-agent`); its hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json`, four rows, all required by `gate --require perf` (was the separate `perf/*.json`); the perf session's `<session>/` holds its synthetic `payload-<hook>.json`.
- New `target/g2-probe/` (the `--probe` scope) and `scripts/g2-zero-panics.sh` (G2, fail-closed, exempting only `src/cmd/hook/seam.rs:<digits>`, + `--probe`).
- CI: 9 jobs / 18 check-runs (was 8 / 15, as measured at ci#36390764600); the per-OS `perf` job (hyperfine via `cargo install --locked`, `run --perf`, G2 probe then check, G4, its own scan, scan-gated `perf-<os>` / `diag-perf-<os>`, `gate --require perf`); the `test` job's G2 step runs the script; `test`, pre-push and WSL carry no perf step.
- §Stack lists hyperfine 1.20.0 (as measured on the Windows dev host).
**Why:** the perf gate lands in its own per-OS job (operator P4 fork 1), so CARRY 3 does not fire; the export path is the one `gate.rs` already read.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-cli-output-tokens — clap without `color`, the env sentence true again, `src/human.rs` the one refusal writer
**Section:** §Stack and Technologies CLI parser row; §Infrastructure Patterns Build system (lint bullet) and Project directory structure; §Cross-cutting Patterns Config management (environment variables)
**Change:**
- clap 4.6.7 is `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color` (was "clap 4.6.7 (derive)"); 8 packages left `Cargo.lock`, none added.
- The exhaustive env sentence now says why it holds: clap's `color` route (anstream → anstyle-query, reading `CLICOLOR`, `CLICOLOR_FORCE`, `NO_COLOR`, `TERM`, `COLORTERM`, `CI`) was closed by this chunk, and a dependency's env read breaks the rule like a direct one.
- The root bin's only human-stderr writer is `src/human.rs` (`write_refusal`, one `write_all`; `refuse`, locked stderr, result dropped), called only by `run`'s five start refusals, squatted endpoint included (was "one `refuse` helper with `writeln!`", four kinds); `src/human.rs` joins the directory tree.
**Why:** the sentence was false at c04e332 through clap's `color` (as measured at research F1: `CLICOLOR_FORCE=1 viola --help` into a pipe, 25 ESC bytes); the chunk shipped the feature change and the writer (report Changes; ci#36404931982 green).
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — the six landed ledger rows, `viola verify`'s probe, the stamps envelope and their registry rows
**Section:** Established Decisions [CLI Version Compatibility]; §Standard Contracts (new Ledger stamps envelope); §Occupied Resources Binary, Claude Code integration names, Filesystem (`ledger/stamps.json`, `ledger/probes/`, `diagnostics/`), Repository; §Infrastructure Patterns CI/CD approach and Project directory structure (`schemas/`)
**Change:**
- The ledger (`viola_agent_claude::ledger`, closed) holds six landed rows: `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload`; a version is verified only when every landed row reads `pass`. The other listed rows land with named owners (screen signatures → readiness gate; local commands → confirmed send; S3/S7/S8 + dialog concurrency → dialog answers; long-paste, tag escaping, harness prefixes, R8 floor → first live test; statusline, `agents --json` join, plugin precedence → Epochs 4/5).
- `viola verify [--record <DIR>] [-- <program> [args…]]` drives ONE print-mode probe (`-p … --model haiku --plugin-dir <probe>/plugin --no-session-persistence`, 120 s) through the `viola-verify-probe` plugin; a failing row still stamps and exits 1; `--record` writes scrubbed `<Event>.default.json` fixtures only at 0 fail. `--version` is read with no user args, drained through `MAX_FRAME`, killed at 5 s; first line exactly `X.Y.Z (Claude Code)`.
- Stamps envelope `{"v":1,"written_at","writer":"verify","data":{"versions":{…}}}`: per-version `rows` + `measured`; merge keeps unknown versions and fields; wrong shape replaced whole.
- Registered: verify's usage; the hidden `hook --capture` flag; the fake agent's `-p/--print`; the probe plugin and its child flags; `ledger/probes/<pid>/` (0700, drop guard) with 0600 transient captures; `stamps.json` modes and its one writer `update_stamps` / lock-free `read_stamps`; `cli-<name>.ndjson` produced by `verify`; `schemas/claude-fixture.v1.json` and the recorded fixture shape (first set 2.1.283).
- "The real `claude` CLI and `viola verify` run only locally" is now: the real CLI runs only locally; `verify` runs in CI against the fake agent's print mode.
**Why:** the chunk landed the ledger, the verb, the stamps writer and the first recorded set (CI ci#36460408121 green on `6486276`; live recording `stamped 2.1.283  6 pass  0 fail`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — the version gate placed and split by crate, `src/human.rs` shared with `verify`, raw panic frames' dependencies
**Section:** Established Decisions [Session Liveness], [Agent Coverage]; §Conventions CLI exit codes; §Infrastructure Patterns Build system (lint bullet), Crate dependency direction (viola-agent-claude, root bin), Project directory structure (`human.rs`)
**Change:**
- `run` start order as landed: program resolution → strip plan → collision → pinned copy and plugin → version gate (`run::version_gate`: a `version-probe` start/exit pair, the stamps read, `cli_version`/`cli_verified` into the first snapshot and the `claude-child` `process-start`, nothing printed) → bind → snapshot → heartbeat → start events → spawn (was "only the version gate has none").
- The version gate is split: Claude parsing, rows, stamp merge/verdict and scrub in the pure `viola-agent-claude` (no `viola-state` dependency); the spawn in root `run::version_gate`; stamps I/O in `viola_state::stamps` (was "the CLI version gate live[s] only in `viola-agent-claude`").
- Exit `1` also covers a `verify` failing row, verify's four `unable:`/`hint:` refusals plus run's pinned-copy refusal, and the `cli` role's exact `error: internal error`.
- `src/human.rs` adds `write_internal_error`/`internal_error` and the stdout `write_result`/`result`; its callers are `run`'s start refusals and `viola verify` (was "only caller is the `viola run` start refusals").
- The root bin also takes windows-sys `Win32_System_Diagnostics_Debug` + `Win32_System_LibraryLoader` and libc `=0.2.189` (Unix) for `src/panic_frames.rs`.
**Why:** the report's Symbols and Dependencies (both crates already in §Stack; `cargo deny check` green).
**Kept:** the 2026-09-28-cli-output-tokens entry's clap facts stand; only its "called only by `run`" clause is retired here, so it is not superseded whole.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — the hidden `hook --capture` arm and verify's probe session, founder-ratified exceptions
**Section:** Established Decisions [Hook Contract], [Plugin Scope]; §Occupied Resources Environment variables (`VIOLA_NAME`)
**Change:**
- [Hook Contract]: `hook` exits 0 at once when `VIOLA_NAME` is absent, except the hidden `hook <event> --capture <DIR>` arm (called only by `verify`'s probe plugin): no `VIOLA_*` read, no obs init, no channel; an absolute existing `<DIR>`, stdin through `take(MAX_FRAME + 1)`, a raw 0600 write to the first free `<DIR>/<PascalEvent>.<k>.json` (`k` across events); every failure writes nothing; exit 0 with empty streams always.
- [Plugin Scope]: unwrapped sessions carry no viola hooks, except `verify`'s transient `claude -p` probe, which loads only `viola-verify-probe` from `ledger/probes/<pid>/plugin/`, removed when `verify` ends.
- `VIOLA_NAME`'s absence makes every hook EVENT path a silent exit 0 (was "every hook").
**Why:** a boundary widening, ratified live by the founder on 2026-09-28 at 20:24:32 after the arm was shown (relay: the Viola overseer); recorded in security-plan's Decisions Log. No other caller may register the arm.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — `StampError` named as an interim divergence from one enum per crate
**Section:** Established Decisions [Error Handling]; §Conventions Rust error types; Inherited Defaults (Errors)
**Change:** the one-enum-per-crate rule stands with its one exception (`PtyError`); `viola-agent-claude`'s `StampError` (`Malformed`, the stamps envelope) beside `AgentError` is named as an interim divergence, not an exception, which the "Verify-stamped test homes and harness" route entry folds into `AgentError`.
**Why:** the chunk landed a second enum where the plan read both ways; the overseer ruled at this wrap that the locked rule stands and the fold rides the route (a CARRY on that entry).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-mutation-testing-to-the-epoch-boundary — mutation leaves CI and the pre-push; syn, proc-macro2 and the download-artifact pin out
**Section:** Stack and Technologies (CI/CD, Code quality rows) · Occupied Resources (Environment variables `AGENT_RUN_CHUNK_BASE`; Filesystem test-side install sites; Repository `target/agent-run/`, `viola-mutants-scratch/`) · Infrastructure Patterns (Project directory structure `ci.yml` comment; Crate dependency direction; CI/CD approach: workflows note, Setup steps, Jobs wired today, the pre-push paragraph)
**Change:**
- CI: 7 jobs / 15 check-runs per push (was 9 / 18), as measured at ci#36483042659 on `17b93c7`; the `mutants` matrix and the `mutants-verdict` union job are gone — CI runs no mutation job, mutation runs only through `agent-run run --mutants` [`--file`], kept for `/andromeda-code-audit`. The SHA-pinned set is checkout, rust-cache, install-action, upload-artifact (`actions/download-artifact` 8.0.1 gone).
- Pre-push: stages `tools → sync → cache → linux-tests → vm-release → windows-tests`, `ok:true` when `windows-tests` is green; no mutation stage, no union, no equal-`base` rule; every WSL call is `env -i HOME=… PATH=…` with no further assignment (the `TMPDIR=<distro home>/viola-pre-push-scratch` sentence retired); `cache` reports `bytes`, `cap`, `cleaned`, `bytes_after`.
- Stack: syn 2.0.119 / proc-macro2 1.0.107 (the union's `#[cfg]` reader `harness::cfg_legs`) removed from `viola-e2e` and the Code quality row; the crate-direction bullet no longer lists them.
- Occupied Resources: `artifacts/mutants-verdict-<leg>.json` retired; `chunk.diff` is `run --mutants`' diff; the host mutation scratch serves `run --mutants` on a Windows host (no pre-push or CI leg); `AGENT_RUN_CHUNK_BASE` overrides `run --mutants`' base, and CI never sets it; the distro PATH component comes from `Linux::cmd` (was `Linux::cmd_env`).
**Why:** the founder's ruling of 2026-09-28 17:59 moves mutation testing to the epoch-boundary code audit; the overseer's P4 answer removes the leg and union machinery that no caller keeps.
**Ref:** .andromeda/runs/2026-09-28T21-04-49-wrap/

## 2026-09-29-h2-conpty-resize-probe — H2 recorded under [PTY]: a key written right after a ConPTY resize can fail to reach a test child
**Section:** Established Decisions [PTY]
**Change:** [PTY]'s as-built paragraph gains the measured H2 fact: on the windows-2025 runner (image `windows-2025-vs2026` 20260922.246.2), in an isolated loop of the viola-pty red test under `cargo llvm-cov nextest`, a key written through the seam right after `resize` returned never reached the Rust test child in 13 of 200 iterations (ci#36527891850), every loss with the child's key-free size watcher seeing the new size, viola's writer having written and flushed the key, and no cursor-position request from ConPTY (`dsr-cpr 0`; also 0 on the dev host, conhost 10.0.26100.8875, 20/20). The measured span is the ConPTY input-pipe write to that child's console read; where inside it the key is lost is not established (an overseer source read of microsoft/terminal, relayed, found no input-buffer flush on the resize path — not verified here). Whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses such a key is unmeasured and owned by the real-CLI verify entry. viola never holds, queues or re-orders a human key behind a resize ([Human Takeover / Wheel]). A test that writes a key after a resize waits for the child to observe the new size first (0 of 200, ci#36529038462).
**Why:** the H2 probe's document branch, selected by its pre-stated rule: every loss was class K (the resize landed, the key racing it was lost) with no CPR, and no loss localised to a cause viola controls; the only viola-side remedy in reach would hold a key. The overseer's correction at the wrap keeps the cause unestablished and the product impact unmeasured rather than written as a ConPTY defect. The founder's product question on H2 stays open. Measured scope: an isolated loop on one runner image under llvm-cov, not the full suite (where the rate was about 1 in 16 runs).
**Ref:** .andromeda/runs/2026-09-29T06-18-44-wrap/

## 2026-09-29-h2-conpty-resize-probe — the viola-pty-watch dir holds the test's own report beside the child's
**Section:** Occupied Resources → Filesystem (`<temp dir>/viola-pty-watch/`)
**Change:** the test-only row now registers two files per real-PTY test: `<test name>.report` (the child's lines — `start`, the key-free size watcher's `size`, `byte`, `restored`) and the sibling `<test name>.test.report` (the test's steps `resize-returned` · `key-written` · `key-flushed` · `dsr-cpr {n}`, the count of `ESC [ 6 n` the rig's output drain saw). Both kept on a panic or kill, both removed on a pass; `viola` never reads or writes either (was: the child's `.report` only).
**Why:** the H2 recorder needed the test side of the sequence to localise a lost key (resize never applied · resize applied and key lost · key before the size); codes and counts only, so no user content reaches either file.
**Ref:** .andromeda/runs/2026-09-29T06-18-44-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — StampError folded into AgentError; Refusal named the open divergence
**Section:** Established Decisions → [Error Handling]; Conventions → Rust error types; Standard Contracts → Ledger stamps envelope; Inherited Defaults → Errors
**Change:** was "one interim divergence: `viola-agent-claude` carries `StampError` (`Malformed`) beside `AgentError`, awaiting its fold"; now `AgentError` sits at the crate root holding `Malformed` (the hook payload) and `StampsMalformed` ("the capability stamps are malformed"); `ledger::verified` returns `Result<bool, AgentError>`, and a non-`v:1` or wrong-shaped stamps envelope is `AgentError::StampsMalformed` to a reader (`run`'s gate still records `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}`). One open divergence, not an exception: the crate's pre-existing thiserror enum `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal) beside `AgentError`; the one-enum rule stands, and its fold or ratification is open.
**Why:** the one-enum rule (an overseer ruling at the capability-ledger wrap) folded the second enum; the plan's "one thiserror enum" acceptance was measured false against the pre-existing `Refusal`, which the arch registry already listed but the rule's exception text never named — carried on the operator's word, with its owner pinned on the route.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — verify's spawn pairs, the harness CI read, verify-stamped homes and the ninth root wait
**Section:** Occupied Resources → Environment variables (test-harness only); Occupied Resources → Filesystem (`diagnostics/` `cli-<name>.ndjson`; `viola-root-watch` Watch report); Occupied Resources → Repository (`target/e2e-home/…`)
**Change:**
- `cli-<name>.ndjson`: verify's self pair now holds two spawn pairs, `version-probe` (the `--version` read) then `verify-probe` (the print-mode probe), each `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` from verify's call-site wrapper; `run_bounded` stays unlogged, so `run`'s gate keeps one `version-probe` pair.
- `CI` registered as a test-harness-only read: `viola-harness` (bin and `run::run`) reads it for presence and passes it to `run_with` as `ci`; `run --local-live` under it is `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite. `viola` never reads it.
- Repository: harness session homes are stamped at `boot` step 4 by `viola verify` against the fake agent (unless `--unstamped`), root rstest homes by `stamped_home` at the recorded 2.1.283, and `target/e2e-home/viola-live-<pid>/home` is the one real-`claude` verify home of the local-only `run --local-live`.
- The Watch report's bound is used by 9 root waits on a child (was 8): the ninth, `wait_endpoint_gone`, runs in `Wrapper::stop` / `stop_keep` after the exit until the recorded endpoint is unconnectable.
**Why:** each is a resource or contract shape this chunk landed; stamps come only from `viola verify`, and the real CLI never runs in CI.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — the H2 real-claude measurement is owned by "First live test and self-drive"
**Section:** Established Decisions → [PTY] (the H2 note)
**Change:** was "the real-CLI verify entry owns that measurement"; now the working-route entry "First live test and self-drive" owns whether the real `claude` loses a key typed right after a resize, and the harness's local `run --local-live` (a real-`claude` `viola verify`) does not claim it. The founder's product question on H2 stays open.
**Why:** "real-CLI verify entry" read as this chunk's `--local-live`, which measures the capability-ledger rows only; the handoff and the prior wrap pinned the H2 measurement to "First live test and self-drive".
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-sideloaded-conpty — the Windows x64 child is hosted in Microsoft's sideloaded ConPTY
**Section:** Stack and Technologies → PTY layer, Content hash; Established Decisions → [PTY]; Infrastructure Patterns → Project directory structure
**Change:**
- [PTY] as-built: every `viola` process restricts its DLL search to System32 as the second statement of `main` (`viola_pty::sideload::restrict_dll_search`); `run` pre-loads the pinned `bin/<version>-<hash>/conpty/conpty.dll` by absolute path, so portable-pty `=0.8.1`'s bare-name load returns it and the child is hosted by the pinned `OpenConsole.exe`; the seam is unchanged and knows no pinned path. `pty_backend()` reads `conpty-sideload` · `conpty` · `openpty`; any sideload failure fails open to the inbox ConPTY. Harness, `OuterPty` and viola-pty tests stay inbox.
- H2 with/without in one windows-2025 run (ci#36563868040): sideloaded 0 of 200, inbox 14 of 200, beside the inbox 13 of 200; no rate.
- The sideloaded preamble `ESC[1t ESC[c ESC[?1004h ESC[?9001h`; its DA1 query holds the child's start until answered (3.54 s vs 0.54 s on the dev host, as measured at the chunk's `evidence/da1-stall.md`); viola stays silent; the headless stall is an open finding owned by the route entry that first runs viola headless.
- Stack: was "Hosts the unmodified `claude` in ConPTY (Windows)"; now the sideloaded ConPTY on Windows x64 (Microsoft.Windows.Console.ConPTY 1.24.260710001, MIT, vendored and embedded), the inbox ConPTY the fallback; windows-sys also covers `SetDefaultDllDirectories` and `LoadLibraryExW`; sha2 also re-hashes the companions.
- Tree: `src/conpty.rs`, `main.rs`'s restriction, viola-pty `sideload`, `scripts/conpty-vendor.sh`, `vendor/conpty/<version>/x64/`.
**Why:** the founder's acceptance was the H2 measurement with and without; the restriction also closes a planting hole that existed before this chunk (a bare-name `conpty.dll` load from the CWD or `PATH`). The vendored delivery is a boundary widening ratified by the founder live (see security-plan's `2026-09-29` Log entry).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-sideloaded-conpty — the ConPTY sideload step in the start order
**Section:** Established Decisions → [Session Liveness] (both start-order sites); Infrastructure Patterns → CI/CD approach
**Change:**
- The prose order now reads collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open), version gate, bind.
- The as-landed chain was "program resolution → strip plan → collision → pinned copy and plugin → version gate"; now "program resolution → collision → pinned copy and plugin → ConPTY sideload (`run.conpty_sideload`: `pin_companions` + the absolute-path pre-load; `outcome` `loaded` · `hash-mismatch` · `unreadable` · `load-failed` · `not-built`, never a refusal; held handles live until `spawn_child` returns) → strip plan → version gate". The strip plan's place was already stale before this chunk; it follows the pinned copy.
- CI/CD: the `test` job's `windows-2025`-only `ConPTY vendor verification` step (`conpty-vendor.sh --verify` then `--probe`) runs before the coverage run.
**Why:** the chunk added one start step between the pinned copy and the strip plan (report Symbols/APIs; the span-order test); the step never changes the exit.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-sideloaded-conpty — the embedded ConPTY companions on disk and in the repository
**Section:** Established Decisions → [Deployment / Distribution], [Snapshot writer]; Occupied Resources → Filesystem, Repository; Infrastructure Patterns → Deployment model
**Change:**
- [Deployment / Distribution]: on Windows x64 the binary embeds `OpenConsole.exe` + `conpty.dll` (`src/conpty.rs`, the four pins' one textual home) and writes them write-if-absent to `bin/<version>-<hash>/conpty/`; each start holds them `FILE_SHARE_READ`-only and re-hashes them in full; unlike the exe a mismatch never refuses the start, it is left as found and the child runs on the inbox ConPTY. The subdirectory keeps them off the child's PATH. First-start cost, as measured at the chunk on the dev host under a parallel suite: `pin_companions` median 1 193 ms; CI stayed green; the root tests seed their homes instead; no bound raised.
- [Snapshot writer]: the companions join `replace_private_shared`'s users; the held open retries a Win32 error 32 under `REPLACE_ATTEMPTS` (two concurrent first starts, 5-11 ms measured).
- Filesystem: `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` registered. Repository: `vendor/conpty/<version>/x64/` (MIT nupkg bytes, `.gitattributes` binary, `conpty-vendor.sh` the one writer and verifier) and the test-side `target/conpty-seed/<key>/`.
- Deployment model: `bin/<version>-<hash>/` also holds `conpty/` on Windows x64, pre-loaded before the spawn.
**Why:** new resources this chunk landed, registered where their category lives; the fail-open treatment keeps the human's start unblocked.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-fake-agent-drift-contract — Harness prefixes: the cross-session tag, escaped and plain
**Section:** Established Decisions → [Human Takeover / Wheel]; [CLI Version Compatibility] → the Harness prompt prefixes row and the Tag escaping row
**Change:** The harness-injected prompt set was the two literals `<agent-message from=` / `<task-notification>`; now it is `viola-agent-claude`'s four compiled `HARNESS_PREFIXES`, matched on the prompt's raw start with no trim: `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`. Tag escaping gains its one exception: an escaped tag is typed text and never classifies `harness`, except the cross-session message, whose escaped form is the CLI's own injection — so a human who types that tag at a prompt's start is filed `harness`. The Harness prompt prefixes row names the set and records the two cross-session forms as a relayed measurement, not yet measured in this repository.
**Why:** another Claude session's message was filed `human` and flipped the wheel. The widening of the classifier's `harness` class, with its typed-tag side effect, was ratified live by the founder (relay: the Viola overseer). The first live test measures a real cross-session UserPromptSubmit `prompt` and owns the ledger row.
**Kept:** no trim before the match (the raw start is the narrower class); the escaped-means-typed rule for every other tag.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/

## 2026-10-01-t12-19-55-wrap — GUI reads gated by the per-launch cookie; the page's toolkit is React + TypeScript
**Section:** Established Decisions → [GUI Control Scope]; Conventions → GUI HTTP errors; Occupied Resources → Filesystem; Occupied Resources → Repository (the `e2e-web/test-results/lint/` row); §Infrastructure Patterns → Build system; §Infrastructure Patterns → Project directory structure
**Change:**
- [GUI Control Scope] was "no token or CSRF"; now v1 reads are gated: a per-launch token exchanged once at `/?t=` for the `viola_<port>` cookie (`HttpOnly; SameSite=Strict; Path=/`, 303 to `/`) gates `/api/info`, `/api/sessions`, `/api/links` and SSE `/api/events` (401 without it); `/`, `/assets/*`, `/health`, `/ready` stay ungated. View-only and the v1.x brake contract are unchanged.
- GUI HTTP errors add `urn:viola:problem:unauthorized` (401) and `urn:viola:problem:cross-origin-forbidden` (403, reserved for the v1.x brake, not served in v1).
- Filesystem adds `ui/<port>.url`: the launch URL, written through `replace_private` (0600, dir 0700 on Unix), overwritten at the next launch on that port, removed on graceful shutdown, never read back as a credential, never logged.
- The lint report row names eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y).
- Build system keeps its guard (no tsconfig, package manifest or bundler config under `crates/viola-ui/`) and now names the founder ruling: the route's frontend-toolchain entry replaces the guard and brings the frontend sources into the ts plane.
- The directory tree's `assets/` line no longer names vendored Lit 3.3.3 ESM; the bundle layout is OPEN. `e2e-web/eslint.config.js` runs eslint-plugin-jsx-a11y over the frontend sources.
**Why:** security-plan's arch amendments 1 and 5 and amendment 3's two URNs were ratified and never folded, which left arch contradicting security on GUI reads; the overseer (founder-delegated) directed the fold at this wrap. The toolkit move is the founder's ruling of 2026-09-30, relayed by the overseer. Standing rule: retiring the no-bundler guard is a boundary widening the founder rules live at the frontend-toolchain entry, not before.
**Kept:** amendment 3's `control-character` detail, unfolded until the route's "Confirmed send" entry, which names it; the guard text itself.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-02-epoch-2b-cleanup — test homes removed with their owner record last
**Section:** §Occupied Resources → Repository (`target/e2e-home/` entry)
**Change:** The `viola-test-*` entry now adds: both removals, the test home's own drop and the gone-owner sweep, go through `remove_owned` (`tests/support/home.rs`), which deletes `owner.json` last, so a removal that stops part-way keeps the record for a later sweep. Was: the entry named only the sweep's owner rule; the drop was a `TempDir` drop.
**Why:** std's `remove_dir_all` stops at the first entry it cannot delete, in listing order, so a scratch entry sorted after `owner.json` let a plain removal take the record first and leave an ownerless dir no sweep may take. The owner-record rule itself is unchanged (pid + start time only).
**Kept:** the ownerless remnants that D:'s deadline-failing runs leave are not closed by this; their mechanism is not established and M2 owns them.
**Ref:** .andromeda/runs/2026-10-03T07-46-03-wrap/

## 2026-10-03-mutation-scoring-completion — pre-push native on the Linux host
**Section:** §Stack and Technologies (CI/CD row, Browser e2e row) · §Infrastructure Patterns → CI/CD approach · §Infrastructure Patterns → Project directory structure · §Occupied Resources (CI workflow data lines, test-side install sites, `target/pre-push/`)
**Change:**
- Was: `pre-push` runs on the Windows dev host, drives a WSL2 `Ubuntu` clone (sync, tree-id check, 40 GiB cache), terminates the VM (`vm-release`), then runs `windows-tests` on the host with `CARGO_BUILD_JOBS=16`, refusing `pre-push-windows-only` elsewhere; provisioned by `scripts/wsl-provision.sh` (with an operator-only root `--install-deps`).
- Now: `pre-push` runs on a Linux host only (`pre-push-linux-only`, exit 2, elsewhere), natively in the working tree (no clone, no sync). Every child is `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` from the repository root, `<home>` the passwd entry's field 6 read by two PATH-only probes (`tool-missing` `passwd-home` on failure). Stages `tools → linux-tests`; it installs nothing.
- `scripts/wsl-exec.sh` and `scripts/wsl-provision.sh` leave the directory tree; `target/pre-push/` and `~/.cache/viola-provision/` leave Occupied Resources; the install sites live in the Linux host user's passwd home; `wsl-provision.sh` leaves the `NODE_PIN_*` parsers.
- The CI/CD hyperfine clause and the jobs clause no longer cite the WSL provisioning; `run --mutants` lists the whole-member `--package <member>` form.
**Why:** the dev host is Linux since 2026-10-03; the WSL clone was that host's filesystem bridge (overseer, founder-delegated, at plan review). No widening: only HOME and PATH cross, HOME from the passwd entry, never the harness's `$HOME` (shown to the founder at plan review).
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/

## 2026-10-03-mutation-scoring-completion — the mutation scratch dir on the Linux host
**Section:** §Occupied Resources (`<repo parent>/viola-mutants-scratch/`)
**Change:** The harness arm stays Windows-only (`HOST_SCRATCH = cfg!(windows)`), and its wipe now runs before every counted, scoped or package run (was "counted or scoped"). New: on the Linux dev host the same-named dir is the operator's `TMPDIR` for every mutation run, an environment fact rather than harness code, and it must be NOCOW (`chattr +C`). The reasons: cargo-mutants 27.1.0 copies the tree (`target/` included, ~14 GB) into the temp dir; `/tmp` there is a 32 GB `usrquota` tmpfs (a full viola-e2e run died at 460/709); and a COW btrfs reflink copy drops the exec bit of the prebuilt `viola-fake-agent`. The body carries this as measured at the chunk's `evidence/m3.md`. No document prints the path.
**Why:** overseer direction (founder-delegated): every mutation run on this host takes `TMPDIR` on btrfs. The NOCOW requirement is the measured remedy for the reflink mode loss (`reflink 0.1.3` creates the clone with `create_new` and copies no mode).
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/

## 2026-10-04-windows-boundary-mutation-workflow — the dispatch-only Windows mutation workflow
**Section:** §Infrastructure Patterns → CI/CD approach · §Infrastructure Patterns → Project directory structure · §Occupied Resources (`AGENT_RUN_CHUNK_BASE`, `<repo parent>/viola-mutants-scratch/`)
**Change:**
- CI/CD approach: "two workflows" → three. `windows-mutants.yml`:
  - `workflow_dispatch` only, no `inputs:`, dispatched only at the epoch-boundary audit; never a gate, a required check or a `ci.yml` dependency, and it adds no check to a push run;
  - one job `mutants (<package>)` on `windows-2025`: `contents: read`, `timeout-minutes: 120`, `fail-fast: false`, a six-package `matrix.include` of each package's Windows-gated `files`;
  - checkout, `rustup toolchain install`, install-action with `cargo-nextest@0.9.146,cargo-mutants@27.1.0`, then one pwsh step running `scripts/agent-run.ps1 run --mutants --package <member> --file …` from step `env:`;
  - no cache, upload, secret, `concurrency:` or `needs:`; its jobs read red while scoped files carry compiled-out `#[cfg(unix)]` twins.
- `tests/contract_windows_mutation_scope.rs` keeps the file lists and pins equal to the sources and to ci.yml.
- "Neither workflow has a `concurrency:` block" → "No workflow …".
- The concurrency parenthetical was "CI runs no mutation job"; it now reads: no push or pull-request run carries one, and the workflow uploads nothing.
- The jobs paragraph's "CI runs no mutation job" → "`ci.yml` runs no mutation job", with the audit's Windows leg named.
- Directory tree: `windows-mutants.yml` added under `.github/workflows/`.
- `AGENT_RUN_CHUNK_BASE` row: was "CI runs no mutation job and never sets it"; now no CI workflow sets it, and the workflow's `--package` arm reads no base.
- `viola-mutants-scratch` row: the workflow runs the Windows `HOST_SCRATCH` arm on the runner, so the scratch sits beside the runner's checkout; documents carry only `scratch_bytes`.
**Why:** founder ruling C2 (2026-10-04, relayed by the overseer) ships a dispatch-only, non-blocking Windows mutation leg while keeping the 2026-09-28 ruling: no chunk, pre-push or blocking CI mutation gate. Not a boundary widening: it adds no permission, secret, input class, upload or action pin.
**Kept:** `ci.yml` and `nightly.yml` byte-unchanged; the push check count stays 15.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/

## 2026-10-04-readiness-gate-and-timing-constants — the readiness gate as landed and its provisional timing values
**Section:** §Stack and Technologies (Screen model row) · §Established Decisions [Screen Model], [Delivery Confirmation], [Hook Transport], [CLI Version Compatibility]
**Change:**
- Screen model row: vt100 runs on `run`'s feed thread, handed a copy of the pump's output after the human's write (was "on `run`'s pump thread").
- [Screen Model]: the model is the pure `viola_agent_claude::screen::Screen`; `run` feeds it on its own feed thread (`src/run/gate.rs`) through a tee that writes the human's bytes first; each feed and resize runs under `catch_unwind`; a caught panic poisons the model until the host size changes and a poisoned model reads `input-not-ready`. Was "the quiet period and the gate's maximum wait are open items, held as ledger rows"; now `QUIET_PERIOD` = 300 ms and `GATE_MAX_WAIT` = 5 s are compiled built-ins, PROVISIONAL and not measured; the signature format (`Signatures { input_box, modals }`, two compiled literal lists, row-contains matching) is PROVISIONAL; the signature, quiet-period and maximum-wait ledger rows are HELD, owed to the first live test or the founder's ruling; the verdict's first consumer is confirmed `send`. "Signatures live in the capability ledger" now reads "once those held rows land".
- [Delivery Confirmation]: the built-in fallback window was "value: open item"; now `CONFIRM_WINDOW_FALLBACK` = 10 s in `viola_agent_claude::screen`, PROVISIONAL and not measured.
- [Hook Transport]: the spine deadline is the product constant `viola_core::SPINE_DEADLINE` = 1 s, the bound the perf gate reads; the hook's 750 ms connect deadline is the crate-private `CONNECT_DEADLINE` (provisional), asserted below it (was the hook-local `SPINE_DEADLINE` = 750 ms with the naming open); SessionEnd's connect deadline reads `CONNECT_DEADLINE`.
- [CLI Version Compatibility]: the screen signatures, with the quiet-period and maximum-wait values, are HELD past the readiness gate (was "land with the readiness gate").
**Why:** the chunk landed the gate's mechanism and named its timing constants; the overseer's directions keep the held widening (the PTY typed-input probe, the live 2.1.287 recording, the signature and timing rows) held and record the three values and the signature format as provisional.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/

## 2026-10-04-readiness-gate-and-timing-constants — vt100 in viola-agent-claude, the new modules and the fourth fuzz target
**Section:** §Infrastructure Patterns → crate-dependency-direction, project-directory-structure · §Occupied Resources (`fuzz/`)
**Change:**
- crate-dependency-direction: `viola-agent-claude` depends, as landed, on vt100 `=0.16.2` (the pure `screen` module); its vt100 line reads "`run`'s feed thread feeds it a copy of the pump output" (was "`run`'s pump feeds it bytes"); the closing sentence no longer lists vt100 as arriving later.
- project-directory-structure: `src/run/` names `gate.rs` (the pump-output tee + vt100 feed thread); `viola-core/` lists `SPINE_DEADLINE`, `Clock` / `SystemClock`; `viola-agent-claude/` names the vt100 screen model (`screen`) in place of "screen signatures"; `fuzz_targets/` adds `vt100_feed`.
- `fuzz/` row: targets `viola_name`, `channel_frame`, `hook_stdin` and `vt100_feed`; `vt100_feed` has 6 synthetic seeds.
**Why:** vt100 0.16.2 entered the graph through `viola-agent-claude` only, and the chunk added `src/run/gate.rs`, the `screen` module, the two viola-core items and the `vt100_feed` target.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — confirmed send: kinds, refusals, exits, the relabel
**Section:** §Conventions (normalised event kinds; `RefusalReason` details; the `send` refusal order; CLI exits `1` and `2`) · §Standard Contracts (`hook.event`; Event `data` per kind; the wrapper-appended kinds) · [Delivery Confirmation] · [Human Takeover / Wheel] · [Screen Model] · Design Philosophy
**Change:**
- Event kinds gain `send-issued` `{cursor, from?}` · `send-confirmed` `{cursor}` · `send-refused` `{refusal, detail, cursor?}` (`cursor` absent before `send-issued`), wrapper-appended, log-only.
- `not-delivered` details gain `control-character`. The `send` order was `human-typing` → `budget-paused` → `turn-running` → gate → confirmation; now `control-character` first (`validate_paste_text`, CLI and wrapper), `turn-running` also for a second send in flight, `input-not-ready` also with no child, and a paste failing after `send-issued` is `input-not-ready` with its cursor.
- `prompt-submitted.origin` `driver` is set only by the wrapper: a prompt whose text equals the in-flight send's exactly (any hook origin) is appended `driver`, then settles the send with that line's `ts`.
- [Delivery Confirmation]: no local-command row is compiled, so a local command, `/clear` included, ends `not-delivered` until those rows land.
- [Screen Model]: was "on unverified builds the gate falls back to delivery confirmation only"; now the gate is partial while no signature row is compiled (every build): poisoned or not quiet by the maximum wait refuses, quiet is ready with no row read. The tee → feed queue is `sync_channel(256)` + `try_send`; a dropped copy poisons like a panic (one `oversize` line per episode, a size message carrying the drop count). Design Philosophy names the partial gate.
- Exit 1 gains `viola send`'s `Err` path (the `Send` arm prints `error: internal error`) and its open panic path (no stderr line: `src/main.rs` files only `verify` as `cli`); exit 2 gains over-cap or non-UTF-8 `send` text.
**Why:** the first driving verb landed. The partial gate is the overseer's F1 reading; one-in-flight = `turn-running` was the P4 lean, reviewed at P5. Not a widening here: the boundary crossing is the security-plan's F3 entry.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — fuzz targets, the chaos test home, the send layout
**Section:** §Occupied Resources (Filesystem: a new test-only entry; Repository: `fuzz/`, `target/e2e-home/…`) · §Infrastructure Patterns → `project-directory-structure` · `crate-dependency-direction`
**Change:**
- `fuzz/`: targets 4 → 5 (`paste_text`); its corpus holds 8 seeds kept byte-exact by `.gitattributes` `fuzz/corpus/paste_text/** binary`.
- Filesystem gains the test-only `<temp dir>/viola-chaos-*/home` (`TestHome::outside_scan()`, `tests/chaos_feed_panic.rs` only), outside `target/e2e-home` and so outside G2, G4 and the secret scan; the test asserts its own panic and `parse-rejected` lines. `target/e2e-home` was "every harness and test home"; now every one but that home.
- The directory contract: `src/run/send.rs` (the `send` method, one in flight, the relabel), `gate.rs` (bounded feed + `Gate`), `src/human.rs` callers `run` · `verify` · `send` (the readback mirror), viola-core's `NotDelivered` + `validate_paste_text`, viola-pty's `PasteHandle`, `fuzz_targets/{…,paste_text}.rs`.
- `viola-core` dependencies were nutype alone; now nutype and serde (`derive`), dev serde_json.
**Why:** the chaos home is the founder's ruling (live, 2026-10-04, relayed by the overseer): the second named test-data carve-out beside `seed_conpty`, all three scans named. The rest records what the chunk landed.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-wait-and-last — wait and last as built; the cli role's one internal-error printer
**Section:** Conventions → CLI exit codes (exit `1`) · Standard Contracts → Channel methods (`from`, `wait`, `last`) · Established Decisions [Database / State Store] · [Message Broker / IPC] · Occupied Resources → `diagnostics/` · Infrastructure Patterns → build-system · project-directory-structure
**Change:**
- Exit `1`: was "a failed or panicked `verify` prints `error: internal error`; `send` prints it from its dispatch arm and a `send` panic prints nothing (only `verify` is `cli`)"; now `role_of` files every first word but `run`, `hook` and a leading `-` flag under `cli` (`send`, `wait`, `last`, `verify`) and the `main` catch site prints the line once for a dispatch `Err` or a caught panic; wait/last exit 1 covers a refusal reply, an unknown reply shape and any channel error but a failed connect or a closed/reset connection.
- `wait` was `{after?, timeout_ms?}`; now `{after?, timeout_ms?, from?}`, `after`/`timeout_ms` absent or `u64` (else `-32602`), start offset `after` or the log end at the call, line-starts rule, `EventKind::WAIT_WAKE`, `checked_add` deadline, and the `WaitFeed` wake: one Mutex + Condvar, the hook line's append inside the lock, only hook lines signal, 20 ms clock re-reads that never re-scan. `last` was `{}`; now `{from?}`, null/null before a turn, the newest turn updated under the append's lock and rebuilt before `server.serve`.
- The envelope types no `from`: each method answers a mistyped one `-32602`, never `-32600`.
- As landed, the one events reader skips (never heals) a torn last line, healing owed to `:85`; an `after`-less `wait` starts at the log end, the pending-dialog return owed to `:78`.
- `cli-<name>.ndjson` gains send/wait/last as producers; the tree gains `cmd/client.rs` and `run/wait.rs`, `human.rs`'s callers gain send/wait/last and the catch site.
**Why:** the chunk built `wait` / `last` and moved the `cli` line to one printer (CARRY 2); the in-lock feed update is the operator pass's fold of a macOS red where `last` read the turn before one already on disk.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — the dialog tier served: hook.dialog, answer, the deadline, nine hooks
**Section:** Standard Contracts → Channel methods (`from`, `wait`, `answer`, `hook.dialog`) · the dialog-event paragraph · Conventions → refusal order · CLI exit codes (exit `1`) · Hook → kind map · Established Decisions [Message Broker / IPC] · [Hook Contract] · [Deployment / Distribution] · Occupied Resources → registered hook events · `diagnostics/` · `target/perf/` · `target/agent-run/` · Infrastructure Patterns → build-system · project-directory-structure
**Change:**
- `hook.dialog` was `{kind, data}` and unserved; now served, `{kind, data, hook_event, tool?, input?, continuation?}` → `{dialog_id, response}`. A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` (question and plan, per tool) is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise. The exactly-once rule and the kind map name that exception.
- `wait`: a pending dialog comes first (an `after`-less `wait`, or one at or before its line, returns it at once); [Message Broker / IPC] was "no `hook.dialog` is served yet, the pending-dialog return owed to `:78`", now landed through `WaitFeed::appending_dialog` / `dialog_settled`.
- `from` is typed by `answer` too. `answer`'s refusal order now opens with `not-delivered`/`control-character`, checked by the CLI before any frame and again by the wrapper. The `permission` answer has no suggestion slot: an offered `permission_suggestions` entry cannot be picked, a deliberate v1 limit.
- [Hook Contract]: the dialog deadline was an open item; now `viola_core::DIALOG_DEADLINE` = 60 s PROVISIONAL, the dialog hooks' `hooks.json` `timeout` 75 s above it (test-pinned), the hook's reply read bounded at `DIALOG_DEADLINE + 5 s`, one stdout write.
- `hooks.json` was seven entries with the dialog tier "registered by the dialog chunk"; now nine, PreToolUse (`matcher` `AskUserQuestion|ExitPlanMode`) and PermissionRequest at `timeout` 75.
- `answer` joins the `cli-<name>.ndjson` producers, `role_of`'s `cli` list, `human.rs`'s callers (`write_answered`, `answer_hint`); the tree gains `run/dialog.rs` (the `DialogSlot`). Perf exports were four rows; now five with `pre-tool-use`, the perf session unstamped.
**Why:** the chunk landed dialog answers by `dialog_id`. Serving the two methods widens the channel boundary: the operator ratified at this wrap the founder's live ruling F1 (P4, relayed by the overseer, the residual shown); `answer`'s permission-suggestion limit is the founder's live ruling F3; the deadline value is the overseer's founder-delegated call.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — decisions on the six-row stamp, the strict stamps read, the 2.1.287 fixtures
**Section:** Established Decisions [CLI Version Compatibility] · Cross-cutting Patterns → capability ledger as the single gate · Standard Contracts → Ledger stamps envelope · Occupied Resources → `ledger/stamps.json` · `fixtures/claude/<cli-version>/` · `target/e2e-home/` · Stack → State-file primitives · Infrastructure Patterns → crate-dependency-direction · project-directory-structure
**Change:**
- [CLI Version Compatibility] was "S3/S7/S8 and dialog concurrency with dialog answers"; now those rows and their own `viola verify` re-probe are owed to `:82`, and until then a non-`null` dialog decision flows whenever the six-row stamp passes (`cli_verified`), a dated gap whose residual is that an S3/S7/S8 body-shape change in a new CLI goes uncaught. The capability-ledger gate names that one dated exception.
- `run`'s version gate reads `ledger/stamps.json` through `read_stamps_strict` (the `viola_state::strict` check first); a strict-modes refusal is `cli_verified:false` with `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}`; other readers keep `read_stamps`.
- Fixtures: was "the first recorded set is `2.1.283`"; now sets `2.1.283` and `2.1.287`, the latter the four spine fixtures recorded against the pinned 2.1.287 binary plus four relayed dialog fixtures from the viola-lab prototype's live captures (`RELAYED.md`, hygiene-walked), superseded by `:82`'s re-probe. `stamped_home` stamps at 2.1.287.
- viola-state takes libc (Unix: the strict owner + mode check) and windows-sys (Windows: the owner + DACL check, and the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL `fs::create_private_dir` gives the topmost folder it creates outside the profile, `GetUserProfileDirectoryW`, the pin gaining `Win32_UI_Shell`; a DACL that cannot be set fails the creation). viola-agent-claude gains the pure `dialog` module and the insta `=1.48.0` dev-dependency, its snapshots in `src/snapshots/`.
**Why:** print mode raises no dialog hook, so no `viola verify` probe could record the dialog tier: the founder's live rulings R1 (relayed fixtures, rows to `:82`) and R2 (decisions on the six-row stamp, a boundary widening the operator ratified at this wrap as the founder's, relayed by the overseer, `:82` the closer); R3 pulled the creation half forward from `:111`.
**Kept:** §Stack lists no test library, so insta is not added there.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-the-wheel — the wheel as landed: the closed non-editing list and the Windows platform fact
**Section:** Established Decisions → [Human Takeover / Wheel]
**Change:**
- Was "focus, mouse and resize sequences do not count"; now only a closed non-editing list never counts: focus reports (`CSI I` / `CSI O`), mouse reports (X10 `CSI M` + 3, SGR, urxvt) and terminal replies (DA1, DA2, CPR, DECRPM, kitty flags, OSC replies ending BEL or ST, DCS replies ending ST); a resize is no stdin byte. Every other byte takes the wheel, a sequence past 64 bytes of parameters or payload and a C0 inside a sequence included; a read ending on a lone ESC is the Esc key; sequences carry across reads. The stdin observer only observes and moves the wheel before a read holding an editing key returns.
- Windows: the inbox ConPTY outer terminal swallows focus reports, and under the sideloaded ConPTY's win32-input-mode (`ESC[?9001h`) an injected mouse report arrives as win32 key-down records and takes the wheel — measured for injected bytes; a real Windows terminal's mouse report is not yet measured.
- A human-filed `prompt-submitted` moves the wheel before its line is appended. The wheel lives in `src/run/wheel.rs` (holder + cause, one lock; the in-memory move synchronous, the `wheel` record, snapshot, span and dialog hand-back on its worker thread, flushed ≤ 2 s at exit before raw mode ends); every snapshot write goes through `src/run/snapshot.rs`. A `release` on a driver-held wheel changes nothing.
- Was "the `viola release` that returns the wheel also clears the running-turn state"; now the clearing stays the intent, and as built no running-turn state exists beyond the in-flight `send` slot, so `release` returns the wheel alone.
**Why:** the founder's live rulings F-W2 (the closed list, the CPR / Shift+F3 collision shown) and F-W3 ("pin the platform fact", the measured win32-input-mode mechanism shown, the error only ever favouring the human), 2026-10-04, relayed by the overseer; the move-before-append order is a CI-measured race. A real-terminal Windows mouse report is measured live at route `:82`.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-04-the-wheel — pause and release served; release-from-driver landed
**Section:** §Standard Contracts → Channel methods · Established Decisions → [MCP] · §Conventions → CLI exit codes (`1`) · §Occupied Resources → Filesystem (`diagnostics/`) · §Infrastructure Patterns → Project directory structure
**Change:**
- Channel methods: was `pause` `{}` and `release` `{budget?:bool}`; now `pause` `{from?}` → `{wheel:"human"}` and `release` `{budget?:bool, from?}` → `{wheel, budget_paused}`. A string `from` on `release` is `-32602` "invalid params" with `data: {"reason":"release-from-driver"}` plus one obs line; another `from` type or a non-bool `budget` (`null` included) is `-32602` `data: null`; `budget:true` leaves the wheel. Both reply only after their `wheel` record lands (`-32603` if not). The `from` readers add `pause`.
- [MCP]: was "whether a wrapper should refuse a `release` whose `params` carry `from` is left to the security specialist"; now the wrapper refuses a string `from` (`release-from-driver`), so CLI `viola release` inside a wrapped session is refused — self-reported, a deterrent, not enforcement.
- The `cli` role's verbs add `pause` and `release` (exit 1 on an internal error); `cli-<name>.ndjson` producers add `pause` / `release`.
- Directory structure: `src/run/` adds `wheel.rs` and `snapshot.rs`; `human.rs` adds the `pause` / `release` lines and callers; viola-core adds `HumanTyping`, `WheelCause`; viola-pty adds `host_stdin()`.
**Why:** the chunk served the two wheel methods (were `-32601`), the security plan's `release-from-driver` guard landing with them.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-04-the-wheel — viola's own Windows console stdin reader
**Section:** Established Decisions → [PTY] · §Stack → PTY layer
**Change:** the host stdin handed to the pump is `viola_pty::host_stdin()`: on Windows with a console stdin, viola's own `ReadConsoleW` reader (no Ctrl-Z wakeup control; UTF-16 → UTF-8, a split surrogate carried, every `0x1A` kept, a 0-unit read read again); elsewhere, and for a redirected Windows stdin, `std::io::stdin()`. Callers: `run`'s pump and the fake agent. windows-sys's roles add the console input read.
**Why:** std's console stdin drops a read's trailing `0x1A` and ends input on a lone `^Z` — source-read at the pinned toolchain and measured on `windows-2025`, where the first red was the fake agent's own std read, not viola's.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-04-running-turn-refusal — the running-turn state as built
**Section:** Established Decisions → [Human Takeover / Wheel]
**Change:**
- Was "as built, no running-turn state exists beyond the in-flight `send` slot … so `release` returns the wheel alone", per "2026-10-04-the-wheel — the wheel as landed: the closed non-editing list and the Windows platform fact"; now the `viola release` that returns the wheel clears the running-turn state, as measured at this chunk.
- The state lives in memory in `WheelSlot` beside holder and cause ("holder, cause and the running turn under one lock") and records no event, snapshot field, span or obs line; a restarted wrapper starts with none.
- `run::send::append_hook_event` marks it before the hook line is appended, keyed on kind alone: a `prompt-submitted` of any origin (the claimed driver prompt included) starts it, `turn-ended` / `session-start` / `session-end` end it, `activity` leaves it; the turn is marked ahead of a human prompt's wheel move.
- The `send` rung decides at arrival (`turn-running`: a turn running or another `send` in flight). Only a `release` returning a human-held wheel clears it, in the same lock hold; a driver-held `release`, `release --budget`, a refused `release`, `pause` and human keys leave it — recovery is `pause` then `release`. An unsent human prompt takes the wheel first, so the next `send` reads `human-typing`, never `turn-running`.
- Residuals: a turn starting during the readiness gate's wait (up to `GATE_MAX_WAIT`, 5 s provisional) is not refused; a SessionEnd the hook appends directly leaves the turn marked.
**Why:** the founder's live ruling minting the entry (relayed by the overseer at the wheel's wrap) asked for the running-turn refusal; the hypothesis that a driver `send` is typed into a turn it did not start was measured true before the fix and refused after it on three CI OSes. The rung-at-arrival residual was accepted at P5 by the overseer and recorded on the operator's directive at this wrap.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/

## 2026-10-05-t00-09-13-wrap — the typed-input probe ratified; its rows owed to "Real-CLI verify probes"
**Section:** Established Decisions → [Screen Model]; [Delivery Confirmation]; [CLI Version Compatibility] (its Harness prompt prefixes row and its owning-chunks paragraph)
**Change:**
- [Screen Model]: the per-version signature, quiet-period and maximum-wait ledger rows were "HELD, owed to the First live test and self-drive entry or the founder's ruling"; now they ride the typed-input `viola verify` probe the founder ratified, which runs the installed `claude` locally on the dev host only, owed to the "Real-CLI verify probes" route entry. No signature set exists yet; QUIET_PERIOD 300 ms, GATE_MAX_WAIT 5 s and the two-literal-list format stay PROVISIONAL.
- [Delivery Confirmation]: the local-command rows were "held with the typed probe, owed to the First live test entry or the founder's ruling"; now they ride the same ratified probe, owed to "Real-CLI verify probes". Until then every local command, /clear included, still ends not-delivered / no-prompt-submitted.
- Harness prompt prefixes row: the real cross-session UserPromptSubmit `prompt` is measured by "Real-CLI verify probes" (was "the first live test").
- [CLI Version Compatibility]: the screen signatures with the timing values, and the local-command rows (was "with confirmed send", which landed without them), land with the ratified probe at `working-route.md:84` ("Real-CLI verify probes"). The long-paste wrapper, tag escaping, harness prefixes and the R8 identity floor go to that entry too (was "the first live test"). S3/S7/S8 and dialog concurrency keep `:84`.
**Why:** the founder ruled live (R-S2, 2026-10-05, relayed by the overseer) that the held widening is ratified: viola verify gains a PTY-driven typed-input probe against the live installed claude, run locally only, with no Claude credential on a CI runner. The founder also split "First live test and self-drive" (R-S1), and the overseer's answers at this wrap moved these rows to the new entry minted ahead of it. Route coordinates were renumbered by manifest.
**Kept:** H2's real-claude key loss after a resize stays owned by "First live test and self-drive" (now `:86`), as do the DA1 stall, the real Windows terminal's mouse report and the live "dialog never renders" measurement. The six-row stamp's dated gap (founder ruling R2) still closes when the S3/S7/S8 rows land.
**Ref:** .andromeda/runs/2026-10-05T00-09-13-wrap/

## 2026-10-05-real-cli-verify-probes — screen signatures compiled, ten ledger rows, verify's interactive runs
**Section:** §Design Philosophy · §Stack (Screen model) · [Screen Model] · [Delivery Confirmation] · [Human Takeover / Wheel] · [CLI Version Compatibility] (rows list, body) · Cross-cutting → Capability ledger
**Change:**
- [Screen Model]: was "PROVISIONAL and not measured … no signature set exists yet … partial on every build"; now `SIGNATURES` is compiled (`input_box: ["for agents"]`, `modals: ["Yes, I trust this folder", "Yes, allow external imports"]`). `QUIET_PERIOD` 300 ms / `GATE_MAX_WAIT` 5 s are unchanged and validated per version by `quiet-period`. `run` passes `Some(&SIGNATURES)` exactly when `cli_verified`, so the full gate runs on a verified version and the partial gate otherwise. `Screen::rows()` is verify-only. Measured on 2.1.288 (M13 / M19): trust is inherited from a trusted parent, while the external-imports approval is keyed on the git root.
- [Delivery Confirmation]: was "an open item … PROVISIONAL"; now `CONFIRM_WINDOW_FALLBACK` = 10 s is compiled and validated by `confirm-window`, and `run` reads no window number. Local-command rows owed to "Local-command and paste-framing rows".
- [CLI Version Compatibility]: six rows → ten (`modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window`); `check(row, &Probes)`. verify adds two interactive PTY runs (Run A untrusted in an OS-temp dir, killed; Run B trusted in `<cwd>/.viola-verify-<pid>/`, one `PROBE_PROMPT` paste, Ctrl-C ×2 then kill; no key into a CLI dialog) and signature-only `--record` screens. Owed rows re-pointed from `:84`: local-command, paste-wrapper, tag-escaping, harness-prefix and R8 rows to "Local-command and paste-framing rows"; S3/S7/S8 + dialog concurrency + re-probe to "Dialog rows and re-probe". The R2 dated gap now rides the ten-row stamp. CI also drives the fake agent's interactive modes.
- §Design Philosophy, §Stack, [Human Takeover], Cross-cutting: the same facts restated (partial gate unverified-only, `GATE_MAX_WAIT` not provisional, the dialog exception's owner).
**Why:** the chunk built W1 + W5 of the founder's three-way split (live, 2026-10-05 05:58Z, relayed by the overseer); W2–W4 and W6 moved to the two new entries ahead of "First live test and self-drive". The probe is the founder's R-S2 widening plus his "two runs, never accept" and two-dirs rulings, ratified at this wrap.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-real-cli-verify-probes — registry rows for verify's interactive runs and screen fixtures
**Section:** [Plugin Scope] · §Standard Contracts → Ledger stamps envelope · §Occupied Resources (Binary · Claude Code integration names · `diagnostics/` · `ledger/stamps.json` · probe dirs · Repository) · §Infrastructure Patterns → CI/CD approach · Project directory structure · Crate dependency direction
**Change:**
- [Plugin Scope]: was "the one exception is verify's print-mode probe". Now verify's transient children are the exceptions: the print probe, Run A, and Run B, which runs the user's global hooks and status line and leaves the CLI's transcript (the accepted residual).
- Envelope and `ledger/stamps.json`: `measured` also holds `typed_probe {ready_settle_ms, turn_settle_ms, prompt_latency_ms, max_turn_gap_ms}`; `run` reads no number from it.
- `diagnostics/`: verify logs four spawn pairs (was two): `version-probe`, `verify-probe`, `verify-pty-probe` ×2.
- Filesystem: the two interactive probe dirs, 0700 and removed on every exit path: Run A's OS-temp `viola-verify-*` (`tempfile`), and Run B's `<cwd>/.viola-verify-<pid>/` (gitignored `/.viola-verify-*/`).
- Repository: `schemas/claude-screen.v1.json`. `fixtures/claude/<v>/` gains `Screen.<phase>.json` (signature rows only) and the `2.1.288` set. 2.1.287 / 2.1.288 are stamped at ten rows and 2.1.283 is drift-only. The relayed-fixture supersession is now owned by "Dialog rows and re-probe". The `--record` refusal was the fixed "a recorded payload still holds a path or a username" (also at the error-text row); now it names the file and the closed code, never the content.
- Binary: the fake agent gains `--trusted-root` / `--screens` / `--turn-stop` (argv, no env).
- Integration names: verify's two interactive children.
- Contracts: CI drives the fake agent's interactive modes too; the tree lists `verify/typed.rs`, `claude-screen.v1.json` and the screens; the root bin lists `tempfile =3.27.0` (Run A's dir); viola-agent-claude's landed API is updated (ten rows, `SIGNATURES`, `rows()`, the typed-probe helpers, compiled constants).
**Why:** the chunk landed these resources. The named refusal is the founder's live ruling at this wrap (through the overseer's AskUserQuestion, the closed code set shown). The dirs and the no-key rule are his 2026-10-05 live rulings, relayed by the overseer and ratified at this wrap.
**Kept:** the stamps-envelope example keeps `2.1.283` as an illustrative key.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-dialog-rows-and-re-probe — fourteen ledger rows, Runs C and D, the hook answers
**Section:** §Stack (Screen model) · [Hook Contract] · [CLI Version Compatibility] (S7 row, body) · [Plugin Scope] · Cross-cutting → Capability ledger
**Change:**
- [CLI Version Compatibility]: ten rows → fourteen (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`); `Probes { print, typed, dialogs }`; a run that raised no dialog fails its rows. verify drives four interactive PTY runs (was two): Run C in `<cwd>/.viola-verify-<pid>-dialogs/` (`--model haiku --settings <DIALOG_SETTINGS>`, three dialog prompts, its one `allow` running `touch viola-probe-permission` in its own dir) and Run D in `-plan/` (`--permission-mode plan`, `plansDirectory` a 0700 `plans/` inside its dir, none under `~/.claude/plans`, measured on 2.1.288 and 2.1.287); their dialogs answered only by the probe hook, settled after the last Stop, ended by a kill. `--record` also writes `<Event>.<stem>-<n>.json` variants. The R2 dated gap (decisions on the ten-row stamp, per "2026-10-05-real-cli-verify-probes — screen signatures compiled, ten ledger rows, verify's interactive runs") is closed; the `permission` kind's end to end is owed to "Permission end to end". CI adds `--dialogs`.
- S7 row: was "approves only through PreToolUse"; now as `allow` + `updatedInput` = the tool's own input, unchanged (a bare `allow` left the plan dialog up on live 2.1.288).
- [Hook Contract]: the capture arm claims `<PascalEvent>.<k>.json` exclusively (`create_private_new`, the next `k` on `AlreadyExists`, was "the first free `k`") and gains the hidden `--answers <DIR>`: a dialog event's ordinal selects a `take(64)` closed `ProbeAnswer` id and `probe_body` prints one `decision_body` to stdout (was "always empty stdout").
- [Plugin Scope]: five transient children (was three); Runs B, C, D leave transcripts (+2 per verify).
- §Stack, Cross-cutting: four runs; the dialog bodies gated by their own rows (the dated exception removed).
**Why:** the chunk landed the dialog rows and their re-probe. The `--answers` body, the `touch` and the +2 transcripts are the founder's live M7 = A ruling; the echoed-input approve and `plansDirectory` his STOP 7 ruling (both 2026-10-05, relayed by the overseer, ratified by the operator at this wrap as his). The free-`k` naming was measured racy.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/

## 2026-10-05-dialog-rows-and-re-probe — registry rows for Runs C and D, the dialog variants and the answers flag
**Section:** §Standard Contracts → Ledger stamps envelope · §Occupied Resources (Binary · Claude Code integration names · `diagnostics/` · `ledger/stamps.json` · `ledger/probes/<pid>/` · probe dirs · Repository) · §Infrastructure Patterns → CI/CD approach · Project directory structure · Crate dependency direction
**Change:**
- Envelope and `ledger/stamps.json`: `measured` also holds `dialog_probe {parallel_both_before_first_post}` (bool or null, additive, no `v` bump); `run` reads no field of it.
- `diagnostics/`: six spawn pairs (was four), `verify-pty-probe` ×4.
- `ledger/probes/<pid>/`: adds `questions/` and `plan/` roots, each with `plugin/`, `captures/`, `answers/`.
- Probe dirs: four (was two), adding `-dialogs/` and `-plan/` (with `plans/`).
- Binary: hidden `--answers <DIR>` beside `--capture` (was "empty stdout"); the fake agent's argv options three → five (`--dialogs`, `--stop-receipt-hold-ms`, capped at 1 000 ms).
- Integration names: the dialog-kind probe plugin (PreToolUse matcher `AskUserQuestion|ExitPlanMode`, PermissionRequest, PostToolUse, `--answers`); four interactive children with their flags.
- Repository: the `<Event>.<stem>-<n>.json` variant class (12 per version), 2.1.287 / 2.1.288 stamped at fourteen rows (was ten); the relayed set superseded and kept, `RELAYED.md` dated "Superseded".
- Contracts: CI `--dialogs`, four typed runs, fourteen rows; the tree's `typed.rs` and fixtures comments; the ledger module lists fourteen rows and `probe_body` / `ProbeAnswer` / `dialog_variants`.
**Why:** the chunk landed these resources; the answers flag and Runs C/D are the founder's live rulings of 2026-10-05 (M7 = A, STOP 7), relayed by the overseer and ratified by the operator at this wrap as his.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/

## 2026-10-05-permission-end-to-end — the permission kind end to end; the question first raised by PermissionRequest stays null
**Section:** §Established Decisions → [CLI Version Compatibility]; §Standard Contracts (the `hook.dialog` receipt paragraph)
**Change:**
- [CLI Version Compatibility]: was "the `permission` kind's end to end and a `question` first raised by PermissionRequest are owed to the 'Permission end to end' route entry"; now the `permission` kind runs end to end (`tests/cli_answer.rs` over the gated `fixtures/fake-scripts/path4-permission.json`), and a `question` first raised by PermissionRequest stays `null` to the human: on 2.1.288 and 2.1.287 a PermissionRequest `allow` WITH `updatedInput` would satisfy it (read statically from both installed bundles, never run), but that body has no ledger row; adding one needs a 15th row, its `viola verify` probe and a re-stamp of both versions (at least 8 live sessions).
- §Standard Contracts: the list of dialogs logged once and answered `null` at once gains a `question` first raised by PermissionRequest with no matching armed dialog; a late `answer` to it is refused `unknown-dialog`.
**Why:** the chunk landed the permission end-to-end case and measured the question's body statically with zero live sessions; the `null` stays because the capability ledger is the single gate for a CLI behaviour and the 15th row's cost exceeds the live-session cap left (the overseer's word at the P5 review, 2026-10-05).
**Kept:** viola's bare PermissionRequest `allow` on ExitPlanMode stays "ignored": true for the body viola sends, although an `allow` WITH `updatedInput` would satisfy ExitPlanMode on both CLIs (static read).
**Ref:** .andromeda/runs/2026-10-05T15-56-38-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — seventeen ledger rows, the unwrap of the CLI's paste frame, Run B at four pastes
**Section:** §Established Decisions [CLI Version Compatibility] (the harness-prefix, long-paste, local-command and tag-escaping bullets; the verify paragraph) · [Delivery Confirmation] · [Plugin Scope] · §Standard Contracts `prompt-submitted` · §Cross-cutting Patterns (capability ledger)
**Change:**
- The ledger holds seventeen rows (was fourteen): `long-paste-wrapper` · `tag-escaping` · `local-command-clear` after `dialog-concurrency`; `Probes` gains `trusted`; a stamp of the fourteen older ids reads unverified.
- Long-paste wrapper: was "keeps the ends byte for byte"; now the unwrap removes, with a matched same-id pair, the frame the CLI writes (two newlines directly before the open tag, one directly after the close tag) and nothing wider. [Delivery Confirmation] and the `prompt-submitted` contract say the same.
- Tag escaping: was "a backslash after `<` in tag-like text the user typed"; now only a typed `pasted_content` tag is stated escaped (2.1.287, mid-text); a typed `<task-notification>` mid-text arrives as typed; the start-of-prompt position is unmeasured.
- Local commands: the list is compiled (`LOCAL_COMMANDS`); `/clear` fires SessionEnd `clear` then SessionStart `clear` with a new `session_id` and no UserPromptSubmit. [Delivery Confirmation]: was "No local-command row is compiled yet"; now `send` does not consume the compiled list yet, owed to "Local-command send outcomes".
- Run B: was one bracketed paste; now four compiled pastes, each added one only into rows holding the input-box literal and no modal; after the long and the tag-like turn the rows come from a wait bounded by `PROBE_DEADLINE` from that turn's Stop, not by `GATE_MAX_WAIT`. Measured: 2.1.287's footer holds no input-box literal for 8.0 s after a long paste. Run B leaves two transcripts per real-CLI verify.
- The harness prefixes and the R8 identity floor go to "First live test and self-drive".
**Why:** the chunk landed the three rows by typed probe and measured that the CLI frames its pair, so the kept ends made a wrapped `send` unclaimable. The Run B widening is ratified by the founder's own live answers, each given after the change was shown and relayed by the overseer: 2026-10-06T19:38Z for the three pastes and the one more transcript (a card labelled a widening), and 2026-10-06T20:49Z for the wait up to the 120 s probe deadline with the guard kept (the red-round card, which was not labelled a widening); the operator confirmed the record at this wrap. The unwrap change is not a widening (the overseer, founder-delegated, 2026-10-06).
**Kept:** the readiness gate of `viola run` and its 5 s maximum ([Screen Model]) are unchanged, although the same literal is absent for 8.0 s after a long paste; that reading is carried on "Local-command send outcomes", unmeasured end to end.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — registry rows for the framing variants, the fake agent's two options and the committed sets
**Section:** §Occupied Resources (Test-only binaries, `viola-fake-agent`; Repository, `fixtures/claude/<cli-version>/`) · [CLI Version Compatibility] verify paragraph (`--record`, the CI modes) · §Infrastructure Patterns → Crate dependency direction · CI/CD approach · Project directory structure
**Change:**
- The fake agent's argv options serving verify's interactive runs are seven (was five): `--framing` (a compiled paste text fires its recorded `paste-1` / `paste-2` UserPromptSubmit variant unchanged; the probed local command fires the recorded `clear-1` SessionEnd then SessionStart and no UserPromptSubmit) and the test-only hold `--paste-hint-ms <ms>`, capped at 8 000 ms. Still no env.
- `--record` also writes Run B's framing captures as `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `clear`), through the same scrub, refusal and hygiene walk.
- Committed sets: was `2.1.287` and `2.1.288` stamped at fourteen rows, `2.1.283` drift-only; now `2.1.287` (with 4 framing variants) stamped at seventeen rows, `2.1.288` (no framing variant) and `2.1.283` drift-only.
- CI runs verify against the fake agent's `--screens --turn-stop --trusted-root --dialogs --framing` modes, seventeen rows (the verify paragraph and the CI/CD key).
- Key files: the `ledger` module at seventeen rows with its compiled paste texts and local-command list; `fixtures/claude/` holds dialog and framing variants; `verify/typed.rs` holds Run B's wait.
**Why:** the registry rows follow what the chunk landed: two argv options of the test-only binary, four recorded variants under 2.1.287 only, and a stamped list of one. 2.1.288 was not re-recorded (the live cap named 2.1.287 alone), so it can no longer stamp every row.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-06-local-command-send-outcomes — send consumes the local-command list
**Section:** §Established Decisions [Delivery Confirmation] · [CLI Version Compatibility] (the closing "owed" clause) · §Standard Contracts → Channel methods (`hook.event`) · Event `data` per kind (`send-confirmed`)
**Change:** was "`send` does not consume them yet … every local command still ends `not-delivered` / `no-prompt-submitted`, never `ok`"; now `send` consumes `LOCAL_COMMANDS`.
- The text as sent is classified by exact equality with a list entry when the slot is reserved: no trim, no case folding, never a leading-slash test. Every refusal rung applies to a listed command.
- A listed command with post-condition "none", and any listed command on an unverified CLI version (`/remote-control` on every version, `/clear` with no passing stamp): `send-issued`, the paste, then `send-confirmed {cursor, confirmed:false}` and `ok` `unconfirmable` at once, with no window. The verified bit is the version gate's `cli_verified` through `SendSlot::new`; the send path reads no `ledger/stamps.json`.
- `/clear` on a verified version waits the window for its post-condition only; no `prompt-submitted` claims it. The tap claims it on a `session-start` with `cause` `clear` and a string `agent_session_id` differing from the remembered id, appends the line unchanged, then settles with that line's `ts` as `submitted_at`.
- The remembered id is the last `session-start`'s `agent_session_id`, whatever its cause, in memory under the in-flight lock: no string id clears it, nothing remembered makes any string id new, nothing persists it.
- A window that closes first is `not-delivered` / `no-prompt-submitted` with the cursor; no new detail.
- `send-confirmed` data is `{cursor, confirmed?}` (was `{cursor}`): additive, no `v` bump, no product reader.
- The `hook.event` relabel sentence gains its one exception, the send waiting for a post-condition.
- [CLI Version Compatibility]: `send`'s use of the list "landed" (was "owed to the "Local-command send outcomes" route entry").
**Why:** the chunk landed the two clauses the matrix capability `v1-29` still owed. Limit kept in the body: the `/clear` confirmation is proven on the recorded 2.1.287 `clear-1` variants replayed by the fake agent; the live proof is owed to "First live test and self-drive".
**Kept:** one detail for a missed post-condition (`no-prompt-submitted`), no new `NotDelivered` value; the entry's opening sentences stay and are scoped to a send that is not a listed command.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
## 2026-10-06-local-command-send-outcomes — the paste-hint reading, the fake agent's eighth option, the mirror's fourth line
**Section:** §Established Decisions [CLI Version Compatibility] (after "unchanged by this") · §Occupied Resources → Binary, subcommands and exit codes (`viola-fake-agent`) · §Infrastructure Patterns → Project directory structure (the `human.rs` and `send.rs` lines)
**Change:**
- [CLI Version Compatibility] gains the reading as measured under the fake agent only: on a verified CLI a `send` issued while the fake agent holds the paste hint (a quiet cleared screen with no compiled literal) ends `not-delivered` / `input-not-ready` with nothing typed, and the same sequence without the hold is delivered. No `send` was run in the real CLI's hint window. What `send` should do while the hint stands is recorded as the founder's open decision.
- The fake agent has eight argv options (was seven): `--tag-turn-screen <phase>` draws `Screen.<phase>` of the set after the turn of the compiled tag-like paste, and after no other turn, in place of `Screen.turn`, with `--framing` and `--turn-stop`; a missing file draws nothing.
- The `human.rs` line lists four mirror lines (was three): `[  ] unconfirmable` with its one fixed note joins `[  ] open`, `[RB] read back`, `[/ ] unable`.
- The `send.rs` line names the remembered session id, the local-command decision over `LOCAL_COMMANDS` and the `session-start` post-condition claim.
**Why:** the chunk measured the readiness-gate reading under the fake agent with the gate unchanged (the P4 card's answer, option B — delegate the overseer, 2026-10-06), added the option for the control of the guard before verify's local-command paste, and added the unconfirmable mirror writer. The open decision is the overseer's disposition at this wrap, relayed by the operator, 2026-10-06: nothing about `send` under the hint is decided here.
**Kept:** `GATE_MAX_WAIT`, `QUIET_PERIOD` and the readiness verdict are unchanged; the 5 s maximum plays no part in this reading, since a quiet screen with no literal is refused at its first quiet instant.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/

## 2026-10-07-t05-47-07-wrap — the live-test split: the Windows-only items and the live rows get their own owners
**Section:** Established Decisions → [PTY] (H2's key loss after a resize; the DA1 start stall); the Harness prompt prefixes row; [CLI Version Compatibility] (the paste-hint reading and its owning-chunks paragraph)
**Change:**
- [PTY]: whether the real `claude` loses a key typed right after a resize is owned by the route entry "Windows-only live measurements", blocked on an interactive Windows host (was "First live test and self-drive"). The stall of a `viola run` with no terminal on its stdio is owned by the same entry (was "the route entry that first runs viola headless").
- Harness prompt prefixes row: the real cross-session UserPromptSubmit `prompt` is measured by "Live rows and paste shapes on the dev host" (was "First live test and self-drive").
- [CLI Version Compatibility]: the `send` run in the real CLI's hint window is owed to "Live rows and paste shapes on the dev host", which also times the hint a second time on the live CLI (was owed to "First live test and self-drive"). What `send` does while the hint stands stays the founder's open decision; it is now made on that entry's numbers, and the gate stays as it is until then. The harness prefixes and the R8 identity floor are owed to the same entry (was "First live test and self-drive").
**Why:** the founder ruled live (R-L1 to R-L4, 2026-10-07, relayed by the overseer) that "First live test and self-drive" is split three ways, that the first live test runs on the Linux dev host, and that the paste-hint choice waits for a second live timing. The overseer's answers at this wrap moved the Windows-only live items to a new Epoch 7 entry blocked on an interactive Windows host, and the rows, paste shapes and hint window to a new entry ahead of the live test. No interactive Windows host exists since the dev host became Linux on 2026-10-03.
**Kept:** `/clear`'s proof against a live CLI stays owed to "First live test and self-drive". The gate, `GATE_MAX_WAIT` and `QUIET_PERIOD` are unchanged. The self-healing-state route coordinate moved by manifest only.
**Ref:** .andromeda/runs/2026-10-07T05-47-07-wrap/
## 2026-10-07-test-homes-off-the-contended-volume — the test homes' base, its keepers and the dev host's tmpfs backing
**Section:** §Occupied Resources → Repository (the `target/e2e-home/…` entry) · §Occupied Resources → Filesystem (a new test-only bullet)
**Change:**
- Repository: the base `target/e2e-home` is prepared before a home is made by two test-side keepers under one contract, `prepare_home_base` (`tests/support/home.rs`, from `TestHome::new`) and `Workspace::ensure_e2e_home` (the harness, from `boot`'s start and from `run --local-live` after its build). A plain base is created as a directory, as before. On Unix a linked base needs an absolute target, made 0700 when gone (one non-recursive create, never a chmod after, never a parent) and accepted only when one `lstat` reads a real directory with no group or other bit; else the start is refused with a fixed message naming no path (`build-failed` at `boot`, `verify-exit-none` at `run --local-live`, no reason added). On Windows the keepers are the plain create.
- Repository: on the Linux dev host `target/e2e-home` is a link to `/tmp/viola-e2e-home-<uid>`, an environment fact named by no variable, flag or config key; the path string is unchanged for the three home classes. Was: every home physically under the repository's `target/`, removals inside the working directory; now the owner sweep, a test home's drop and harness `cleanup` create and delete outside it through the link, each only a dir the test side itself made. No CI runner has a link. `cargo clean` removes the link and the next start makes a plain directory with no signal.
- Filesystem: `/tmp/viola-e2e-home-<uid>/` registered as test-only, outside the repository, Linux dev host only: tmpfs, 0700, made by the builder and re-made after a reboot by the keepers; a kept home there does not survive a reboot and ages out after ten untouched days; `viola` never reads or writes it by that name.
**Why:** a start's pinned copy stalled behind other builders' writes on the shared volume; measured in a natural window at this chunk, five 52 MB writes at once read a 10.999 s median there and 0.285 s at most through the link, the verify-driven binary green in the same window. The creation and removal outside the working directory is a boundary widening, ratified by the founder, 2026-10-07T07:25Z, live, after it was shown to him in those terms, relayed by the overseer. Standing rule for later chunks: a stalled-start red is a finding about the backing, reported and never re-run for green.
**Kept:** the path statement (every home but the chaos one under `target/e2e-home`) holds by path; `replace_private`'s `sync_all()` and CI's homes on the runner's disk are unchanged. `target/e2e-home.disk/` (the pre-link entries) is not registered: it is a leftover the operator owns.
**Ref:** .andromeda/runs/2026-10-07T08-21-13-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the long-paste frame as measured beside typed text and between two pairs
**Section:** §Established Decisions → [CLI Version Compatibility] (the long-paste wrapper) · [Delivery Confirmation] (the normalisation parenthetical) · §Standard Contracts → `prompt-submitted` (the frame parenthetical) · §Cross-cutting Patterns → Capability ledger (the ruled limit)
**Change:**
- The frame: two newlines before the open tag; after the close tag one newline at the prompt's end, two when typed text follows, three in all between two adjacent pairs. Was "two before and one after".
- `hook::unwrap_pastes` removes the two before, the one after, and a second after the close when text follows that is not the next pair's own two-newline frame. A third newline before, a second after at the prompt's end and a lone one before stay. Was "a second after … stay[s]" without the condition.
- The id is 4 hex characters, one for every pair of a session (the two pairs of one prompt included), different between sessions. Was "differs per paste".
- A pasted text's own last newline never reaches the hook: a wrapped text ending in a newline gets none added before the close tag (the static reading, now measured), and an unwrapped text loses it too.
- The frame beside typed text and between two pairs is compiled on one measurement (2.1.287) and held by `hook.rs` unit cases only; no `viola verify` run types those shapes, so the `long-paste-wrapper` row re-validates only the lone-paste frame. No probe is owed for them: a shape no `send` relies on needs no probe, and §Cross-cutting Patterns → Capability ledger carries that limit.
- "Unmeasured" now names only a long or a repeated text pasted under the paste hint. The wrap threshold and the feature flag stay read statically.
**Why:** three live shapes were measured on `claude` 2.1.287 and the paste-then-typed one falsified the base unwrap, which this chunk fixed inside `unwrap_pastes`. The probe gap was escalated at this wrap; the overseer directed it recorded and brought as a route card, since a new Run B paste is a widening and an accepted limit is a ruling on the ledger rule, both the founder's; he ruled the limit at that card (live, 2026-10-07T11:37Z, relayed by the overseer).
**Kept:** the exact-match claim and the unwrap's "nothing wider" rule.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the cross-session form and the typed task-notification as measured
**Section:** §Established Decisions → [CLI Version Compatibility] (Harness prompt prefixes · Tag escaping) · [Human Takeover / Wheel] (the `HARNESS_PREFIXES` parenthetical and the side-effect sentence)
**Change:**
- On 2.1.287 on the Linux dev host a real cross-session message reaches UserPromptSubmit unescaped, `<cross-session-message from="…" from-name="…" from-mode="…">`, a newline, the text, a newline and `</cross-session-message>`, and the plain prefix files it `harness`. Was "the two cross-session forms rest on a relayed measurement, not yet measured in this repository" and "the escaped form is the one the CLI injects".
- The escaped form rests on the relayed measurement alone; it was not seen and its prefix stays compiled. The four compiled prefixes are unchanged.
- A `<task-notification>` typed at the very start of a prompt arrives as typed, so a human who types it first is filed `harness`. Was "the start-of-prompt position … is unmeasured". The wheel entry states it as measured, not as a new ratification.
- On PATH `claude` (the builder's own CLI, counts only) the `<agent-message from=` and `<task-notification>` forms reach the hook starting at the tag, with no preface.
- The readings came from a scratch probe, not a `viola verify` probe: the harness-prefix ledger row stays owed.
**Why:** one fixed synthetic peer message into a scratch 2.1.287 session, and one typed paste, measured both; the forward reference to this chunk is spent.
**Kept:** the founder's 2026-09-29 ratification of the escaped prefix and its side effect on a human who types that tag.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — a send whose text ends in a newline is delivered and not claimed
**Section:** §Established Decisions → [Delivery Confirmation] · [Human Takeover / Wheel] (the "never move it" parenthetical) · §Standard Contracts → `prompt-submitted` (the "`text` is the pasted text" clause)
**Change:**
- [Delivery Confirmation] carries one measured exception to "every `send` is confirmed", unfixed: the CLI drops a pasted text's last newline before UserPromptSubmit, wrapped or not, so for a sent text whose last byte is a newline `prompt-submitted`'s `text` is one byte short and the exact match fails. The text is delivered and runs a turn, its prompt is filed `human` and moves the wheel (`cause` `human-input`), and the send ends `not-delivered` / `no-prompt-submitted` when the window closes.
- `prompt-submitted`: `text` for a text that ended in a newline is one byte short of it and does not match the sent text.
- [Human Takeover / Wheel]: an in-flight send whose text ends in a newline is not relabelled, so its own prompt moves the wheel to the human.
- No remedy is built; it is carried on the working route.
**Why:** measured end to end on 2.1.287 on a verified and an unverified home. It was escalated at this wrap as a qualification of a locked decision; the overseer directed that all three sections say what the product does today, measured and unfixed, with the remedy left to its route card.
**Kept:** the exact-match claim itself: no trim was added.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the paste hint measured on the live CLI, and the founder's decision
**Section:** §Established Decisions → [CLI Version Compatibility] (the `viola verify` paragraph: the paste-hint passage)
**Change:**
- What `send` does with the hint screen is measured on the real CLI as well as under the fake agent: on a verified CLI a `send` issued while the hint stands ends `not-delivered` / `input-not-ready` with nothing typed, 0.63 s after it was issued. Was "measured end to end under the fake agent only" and "no `send` was run in the real CLI's hint window".
- The hint is a timer of 8.0 s from the last long paste (nine timings, 8.000 s to 8.023 s; a second long paste restarts it, a short one does not), 3.8 s to 7.0 s of it after the turn's Stop.
- A no-cursor `viola wait` issued in the window woke on nothing and ran to its deadline; `wait --after` the earlier cursor returns at once, inside the window. The `input-not-ready` hint line's advice does not lead out of it.
- On an unverified CLI the partial gate typed a short text under the hint and the CLI submitted and confirmed it. A long or a repeated text under the hint is unmeasured.
- What `send` should do is decided: on a verified CLI the gate waits for the input box on a quiet screen with no literal, and the bound rises to 8.5 s. Was "the founder's decision and is open".
- It is not built: the gate, `GATE_MAX_WAIT`, the hint line and the fake agent's 8 000 ms hold cap stay as they are until the route entry that builds it lands.
**Why:** two hint runs on `claude` 2.1.287 gave the numbers; the founder chose on a priced card (his own live answer of 2026-10-07T10:29Z, relayed by the overseer), having been told the price: one by-path re-verify of 5 live starts, `send`'s longest block 18.5 s, the fake agent's hold cap to raise, no human keystroke delayed.
**Kept:** Run B's own measured sentence (8.0 s from the paste, 6.5 s after that turn's Stop) and its wait rule.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the identity floor's Linux reading, two hook-process names, and the two rows still owed
**Section:** §Occupied Resources → Environment variables (the R8 line; the provided-by-Claude-Code line) · §Established Decisions → [CLI Version Compatibility] (the `viola verify` paragraph: the owed rows)
**Change:**
- Two Linux dev-host readings stand beside the floor's Windows origin, names only. The tool environment PATH `claude` 2.1.289 hands a session: 10 `CLAUDE*` names, all on the floor (`CLAUDE_CODE_BRIDGE_SESSION_ID` absent). The environment 2.1.287 hands a hook, in a child started with every inherited `CLAUDE*` name removed: 12 names, eight of the eleven (absent `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_EFFORT`) and four outside them (`CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`).
- The floor stays the eleven: the four are not identity names. The prefix rule already removes those four whenever they are inherited outside the persistent set.
- `CLAUDE_ENV_FILE` and `CLAUDE_PROJECT_DIR` are registered beside `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA` as names a 2.1.287 hook process was handed; viola reads none of the four.
- The harness prefixes and the R8 identity floor were measured by a scratch probe and landed no row: both rows stay owed, with no route entry minted for them. Was "owed to 'Live rows and paste shapes on the dev host'".
**Why:** the scratch hook's names-only list was the founder's own live answer of 2026-10-07T09:43Z (relayed by the overseer), as was "measure here, rows next". The floor's ruling is the founder's own (live, 2026-10-07T11:37Z, relayed by the overseer), given on a route card of this wrap.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/

## 2026-10-07-send-waits-out-the-paste-hint — the gate waits for the input box, bound 8.5 s
**Section:** §Established Decisions [Screen Model] · [Delivery Confirmation] · [Human Takeover / Wheel] · [CLI Version Compatibility] · §Conventions (the `send` refusal order) · §Occupied Resources (the fake agent's options)
**Change:**
- [Screen Model]: `GATE_MAX_WAIT` is 8.5 s (was 5 s); `QUIET_PERIOD` stays 300 ms. On a verified CLI a modal row refuses at once and a quiet screen with no input-box row waits for one until `GATE_MAX_WAIT` has passed since the send entered the gate; refused at the bound, never at the first quiet instant (was "either check fails" refuses at once). The wait lives in `Screen::verdict`; `Gate::wait_ready` is unchanged. Measured settles on 2.1.287: ready 1 103 ms, turn 617 ms.
- [CLI Version Compatibility]: the hint remedy is built (was "decided, not built"). A `send` under the paste hint on a verified CLI is delivered once the input box returns; a hint past the bound is refused `input-not-ready` with nothing typed (was refused 0.63 s in, kept as the dated state before the chunk). Measured under the fake agent only; a `send` under the real CLI's hint is unmeasured since the build. "The run gate is unchanged by this" is retired: verify's `box_wait` and the run gate are two waits. The `input-not-ready` hint line is unchanged.
- [Human Takeover / Wheel] and §Conventions: `send` reads the wheel and then the running turn twice, at arrival and again after the gate returns ready; `human-typing` or `turn-running` with no `send-issued` and nothing typed. The 2026-10-04 residual (a turn starting during the wait is not refused) is retired. The wait is not interrupted when the wheel moves.
- [Delivery Confirmation]: `send` blocks at most 18.5 s (8.5 s and 10 s; was 15 s).
- §Occupied Resources: `--paste-hint-ms` is capped at 10 000 ms (was 8 000 ms); eight options.
**Why:** The wait and the 8.5 s bound are the founder's live ruling of 2026-10-07T10:29Z, relayed by the overseer, so that the gate covers the CLI's 8.0 s paste hint; he ruled it knowing the price (one by-path re-verify, the longer block, the raised cap). The second read of the wheel and the turn is the overseer's technical answer of 2026-10-07T12:10Z, not the founder's: a wait of seconds would otherwise let a send paste over a human who started typing during it. It is listed for the founder as a fact. Trap for later chunks: any case that reads a verified literal-less screen through a clock that never advances now waits forever.
**Kept:** the `input-not-ready` hint line's wording, open for the founder; `Gate` knowing nothing of the wheel; no compiled literal for the hint's own text.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
## 2026-10-07-a-send-ending-in-a-newline-is-confirmed — send types a text without its trailing LF; the newline exception retired; the census script registered
**Section:** §Established Decisions → [Delivery Confirmation] · [Human Takeover / Wheel] · [CLI Version Compatibility] (the long-paste wrapper row) · §Standard Contracts → Channel methods (`hook.event`) · Event `data` per kind (`prompt-submitted`) · §Occupied Resources → Repository · §Infrastructure Patterns → Project directory structure (key file)
**Change:**
- [Delivery Confirmation]: `validate_paste_text` runs first on the text as received; then the typed text is taken once (`viola_agent_claude::hook::typed_text`: every trailing LF and nothing else, not a CR, a TAB, a space or an inner newline) and is the only text `send` uses: the local-command classification, the in-flight text the match compares, `text_bytes` and the paste. The match is still exact, now on the typed text (was "the sent text"). The sentence "One measured exception … stands, unfixed … No remedy is built" is retired: a send ending in LF ends `ok` `{submitted_at, cursor}`, relabelled `driver`, with no `wheel` record. A listed command followed by newlines is that command (`/clear` and a newline is `/clear`), proved at the unit layer only. A text of only newlines is typed as an empty text.
- [Human Takeover / Wheel]: the "one measured exception, unfixed" sentence is retired; a prompt that is not the in-flight send's typed text claims nothing and is appended as the hook filed it.
- `prompt-submitted`: `text` is the prompt as typed (was "as it was sent"); for a send that ended in LF it is the text as typed, not as sent. `hook.event`: the relabel compares with the send's typed text.
- The long-paste wrapper row and the `prompt-submitted` row keep the CLI measurement (2.1.287 drops a pasted text's last newline); no `send` relies on it.
- Repository: `target/profraw-census/<utc>-<pid>/` or `/<name>/`, written by `scripts/profraw-census.sh <runs> <workers> [<name>]` (exit 0 only when every run passed, profiles equal runs, `third` and `short` are 0; 1 on any other count; 2 on usage or no instrumented binary). The `scripts/` tree lists it. No CI job, `pre-push` stage or harness command runs it.
**Why:** the founder's ruling, live, 2026-10-07T15:21Z, relayed by the overseer, every option and the `/clear`-plus-newline consequence shown to him: strip every trailing LF. It was first the overseer's delegate answer of 2026-10-07. Proved under the fake agent on the three CI OSes; a live CLI send ending in newlines, a text of only newlines and a trailing CR stay unmeasured and are carried on the route.
**Kept:** the claim itself (`SendSlot::claim`), exact; a tolerant claim in the wrapper was the boundary widening not chosen; no ledger row, probe paste or stamp was added, since no `send` relies on the dropped newline.
**Ref:** .andromeda/runs/2026-10-07T19-12-23-wrap/
## 2026-10-08-first-live-test-and-self-drive — the closed non-editing list gains seven terminal reply shapes
**Section:** §Established Decisions [Human Takeover / Wheel]
**Change:** the list's terminal replies now also hold seven shapes after `CSI`, each by exact grammar (fields compared as bytes, no intermediate byte, the field count exact): `0 n`, `? 997;1 n` and `? 997;2 n`, `4;n;n t`, `6;n;n t`, `8;n;n t`, `48;n;n;n;n t`, `> 4;n m`. Was: DA1, DA2, CPR, DECRPM, kitty flags, OSC and DCS replies only. The entry states the before and after as measured on foot 1.28.0 (seven replies read as typing and a live CLI taking the wheel 237 ms after its start; then none of 23 probe queries), that the nearest human keys (`n`, `t`, `m`, their Alt forms and kitty key events) and a sequence one field or one prefix away still take the wheel, and that a reply no terminal has been measured to write is not added.
**Why:** a boundary widening of F-W2's list, ratified by the founder himself on 2026-10-08 after the exact grammars and a narrower option of two were shown, relayed verbatim by the overseer. Standing rule: the list grows only by measured shapes, each with its exact grammar and its negative controls.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-08-first-live-test-and-self-drive — the live readings on 2.1.287 are measured, and the hint line's wording has landed
**Section:** §Established Decisions [Delivery Confirmation] (three sentences) · [CLI Version Compatibility] (three sentences)
**Change:** [Delivery Confirmation]: a send ending in newlines is confirmed live (`text_bytes` 31 of 33) with no `wheel` record; a text of only newlines is issued with `text_bytes` 0, draws no `prompt-submitted` and ends `not-delivered` / `no-prompt-submitted`, exit 13, the wheel unmoved; a text ending in one CR is submitted by the CLI without the CR, so the prompt is filed `human`, the wheel moves and the send ends `not-delivered` / `no-prompt-submitted`, exit 13 (no fix; carried on the working route). Was: all three unmeasured on a live CLI. The `/clear` confirmation and `/clear` with a newline are measured live (was: owed, and unit layer only). [CLI Version Compatibility]: a `send` under the real paste hint is measured (sent 4 248 ms after the long paste, confirmed, `duration_ms` 4 081 against 8 500); the `input-not-ready` hint line reads `<name> was not ready for input; send again, and if it repeats a human must look at the session` (was: unchanged, its wording open for the founder), and the sentence judging the old `viola wait` advice reads as history.
**Why:** the chunk took the live readings earlier chunks left to it and landed the founder's hint wording. Trap for later chunks: the trailing-CR outcome is a known defect, not a designed refusal.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-08-first-live-test-and-self-drive — first live on the Linux dev host; the `VIOLA_DIR` step as landed
**Section:** §Design Philosophy (Windows first) · §Project Intent (Scale path) · §Cross-cutting Patterns (Config management, Viola home) · §Occupied Resources (Environment variables, `VIOLA_DIR`)
**Change:** Windows is "the first target" (was: "the live-supported target"); the first live run was on the Linux dev host, and Windows live behaviour stays proven on the CI runner under the fake agent until an interactive Windows host exists; the Scale path line says the same (was: "Windows live, macOS and Linux CI-tested"). Config management keeps the resolution order `--home`, grandparent of `VIOLA_DIR`, default, and adds its as-landed state: the CLI verbs take `--home`, else the default, only `hook` takes the `VIOLA_DIR` step, and that step for the CLI verbs is owed to the working-route entry "CLI machine contract". The `VIOLA_DIR` registry line adds that `hook` is its only reader as landed and that the `mcp` verb is not built.
**Why:** the founder's ruling R-L1 (2026-10-07, relayed by the overseer) moved the first live test to the Linux dev host, and this chunk ran it; the home-resolution gap was read from the code at this chunk and is a sequencing deferral with a route owner, not a dropped design.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-epoch-3-cleanup — the typed text drops trailing CR and LF
**Section:** §Established Decisions › [Delivery Confirmation], [Human Takeover / Wheel]; §Standard Contracts › Channel methods (`hook.event`), Event `data` per kind (`prompt-submitted`)
**Change:** `typed_text` is the sent text without its trailing CR and LF characters, every one of them in any order, and nothing else: not a TAB, a space, or a CR or LF that is not at the very end (was: every trailing LF and nothing else, "not a CR"). A send ending in LF, CR, CRLF or several of them is confirmed like any other and moves no wheel. `/clear` and a CR or a CRLF is `/clear`, as `/clear` and a newline was. `typed_text` has two product callers, the wrapper's `send` and the client's `refusal_of`. The 2026-10-08 live readings (a newline-only text issued with `text_bytes` 0; a text ending in one CR typed with it and filed `human`) stand as records of the earlier build. This chunk's readings on live 2.1.287 replace the sentence "No fix landed at that chunk; the trailing-CR case is carried on the working route": endings of CR, CR LF and CR CR are typed without them and confirmed; a CR or a CR LF inside a text is submitted by the CLI as one LF, so that send ends `not-delivered` / `no-prompt-submitted` and takes the wheel, and it is the inner case the route now carries.
**Why:** the founder ruled on 2026-10-09 (live in the overseer's dialog, the options shown, relayed by the operator) that a trailing CR and CRLF are stripped as the trailing LF is. The CRLF and two-CR endings were measured live before any test pinned them.
**Kept:** an inner CR or LF is not stripped and the match stays exact. The ruling covers the ending only, and no statement of the later inner-CR ruling is in the body: it reached the wrap by relay and is carried on the route.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
## 2026-10-09-epoch-3-cleanup — an empty typed text is refused `empty-text`
**Section:** §Established Decisions › [Delivery Confirmation]; §Conventions › Error handling schema
**Change:** the closed `not-delivered` detail set gains `empty-text`, its sixth value (was five, ending at `control-character`). A text whose typed text is empty, the empty text or a text of only CR and LF characters, is refused `not-delivered` / `empty-text`, exit 13, and is never issued: by the client before any frame and by the wrapper directly after the typed text is taken and before the first wheel read, with nothing reserved, issued or typed, no cursor, and the wheel and the running-turn state not read. The `send` refusal order reads `control-character`, then `empty-text`, then `human-typing` and the rest as before; a text that also holds a refused character is still `control-character`. Was: "A text of only newlines is typed as an empty text", and such a text was issued and waited out the window.
**Why:** the founder's ruling of 2026-10-09 (the name, the condition and the hint; relayed by the operator); the rung is the overseer's answer.
**Kept:** `validate_paste_text` is unchanged and `answer` has no such refusal. No exit code, channel method or `v` moves.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
## 2026-10-09-epoch-3-cleanup — the self-healing route entry cited by title
**Section:** §Established Decisions › [Database / State Store]; §Infrastructure Patterns › Project directory structure (key file)
**Change:** both sites name the working-route entry "Self-healing state" by its title. Was: `working-route.md:109` in the body and "route :93" in the key file's tree comment.
**Why:** the operator's direction that every stale bare route number found is cited by title. `:93` was stale; `:109` was current and this wrap's own insertion above the entry moves it. A bare route number goes stale at every insertion ahead of its entry, and the citation sweep reads no bare number.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/

## 2026-10-09-inner-cr-and-crlf-in-a-sent-text — the typed text types an inner CR or CR LF as one LF, the founder's ruling
**Section:** §Established Decisions [Delivery Confirmation] · [Human Takeover / Wheel] · §Conventions → Error handling schema · §Standard Contracts (`hook.event`; `prompt-submitted`) · §Cross-cutting Patterns (Capability ledger as the single gate for CLI-specific behaviour)
**Change:** the typed text (`viola_agent_claude::hook::typed_text`) is the sent text with every CR LF pair as one LF, every other CR as one LF, and every trailing CR and LF removed; it holds no CR and nothing else changes (was: every trailing CR and LF "and nothing else", so an inner CR was typed). Every gloss of the typed text in the five sections carries that wording. A send whose text holds a CR or a CR LF inside is one paste of the typed text, confirmed, filed `driver`, with no `wheel` record (was: "not covered by the strip", ending `not-delivered` / `no-prompt-submitted`, "no fix landed, carried on the working route"). The readings `after-inner-crlf` and `after-inner-cr` stay as records of the build before the rule. Recorded as measured on live `claude` 2.1.287 on the Linux dev host: five inner shapes confirmed (`rule-inner-crlf`, `rule-inner-cr`, `rule-inner-cr-cr`, `rule-inner-lf-cr`, `rule-crlf-lines`) after the control `rule-inner-lf`, an LF typed inside a bracketed paste submitted unchanged. Not measured: any other CLI version, a long or wrapped multi-line text, an LF inside a paste on Windows, a typed CR CR or LF CR. `prompt-submitted.text` is the text as typed for a send that held an inner CR too. The capability-ledger pattern names one shape `send` relies on with no row yet, the LF typed inside a paste: its row is owed to `v1-34` on the working-route entry "Paste newline ledger row"; the seventeen-row count and the ruled limit are unchanged.
**Why:** the founder ruled, live in the overseer's dialog on 2026-10-09T16:51Z with the options shown, relayed by the operator, that `send` types a CR or a CR LF inside a text as the LF the CLI submits; the chunk landed it and measured it. The owed row's route entry is the founder's word of 2026-10-09T20:58Z, live in the overseer's dialog with four placements shown, relayed by the operator. No `send` types a CR now, so nothing relies on what the CLI does with one.
**Kept:** the match stays an exact equality with no tolerance on the submitted side; `validate_paste_text` runs first on the text as received; the refusal order and the `not-delivered` detail list are unchanged. Nothing is ruled about the reach of the ledger requirement: the pattern sentence states a fact and its owner.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-09-epoch-3-cleanup-ii — the Watch bound's waits counted by pattern, the ordinal retired
**Section:** Occupied Resources → Filesystem (the `viola-root-watch` Watch report row)
**Change:** `WITHIN` = 7 s is "shared by the root waits on a child": `Instant::now() + WITHIN` reads 22 sites in 16 files under `tests/`, as measured at this chunk's report on the Linux dev host's tree, with the shared `tests/support/events.rs` and `tests/support/cli.rs` holding one each. The pattern is the count's rule and not a census of every wait on the bound. `wait_endpoint_gone` stays named as one of the waits. Was "used by the 9 root waits on a child; the ninth is `wait_endpoint_gone`".
**Why:** the 9 was a named list (the eight sites moved from a 10 s bound at chunk 2026-09-27-browser-verdict-reachability plus `wait_endpoint_gone`) that no wrap re-took while later chunks added waits; by the pattern it read 30 at this chunk's base, and the chunk's lift of the per-file scaffolding into `tests/support/` moved it to 22. The operator directed that the count be amended only after its rule was read. Trap for later chunks: a wait written through `EXIT_WITHIN` or a qualified path is outside the pattern, so the number is true only with its pattern beside it.
**Kept:** the bound, the report's path and lifecycle, and `wait_endpoint_gone`'s description are unchanged.
**Ref:** .andromeda/runs/2026-10-10T01-25-38-wrap/

## 2026-10-10-windows-mutation-grade — nine labelled Windows mutation jobs, the harness's host exclusion, syn and proc-macro2 in viola-e2e
**Section:** §Infrastructure Patterns → CI/CD approach · §Infrastructure Patterns → Project directory structure · §Stack and Technologies (Code quality row)
**Change:**
- CI/CD approach: `windows-mutants.yml` is one job definition named `mutants (<label>)` over a nine-item `matrix.include`, counted as its `- package:` items, each item a `package`, a `label` and its `files` (was "One job, `mutants (<package>)`" over "a six-package `matrix.include`"). The root package `viola` is four of the nine, split by file: `src/cmd/run.rs`; `src/main.rs` with `src/conpty.rs`; `src/run/env.rs`; `src/panic_frames.rs`. `tests/contract_windows_mutation_scope.rs` also keeps the labels distinct, one per item.
- CI/CD approach: "its jobs read red while scoped files carry `#[cfg(unix)]` twins the Windows build compiles out" is retired. The harness document judges a job; the harness leaves a missed mutant the Windows build never compiled out of the count and names it in `mutants.host_excluded`, while cargo-mutants' own lines still say MISSED. Run 38036448183 read all nine jobs green with 30 left out.
- CI/CD approach: after the sentence that states founder ruling C2, one sentence names the one dispatch made outside the epoch-boundary audit: 2026-10-10, for this chunk alone, on the founder's own word, run 38036448183.
- Project directory structure: the `windows-mutants.yml` tree comment reads per labelled matrix item, the root package split by file; the `viola-e2e/` comment names `harness::run::mutants::host`, the `#[cfg]` reader behind `run --mutants`' host exclusion.
- Stack: syn `=2.0.119` (`default-features = false`; `full`, `parsing`, `printing`, `visit`) and proc-macro2 `=1.0.107` (`span-locations`) stand beside jsonschema as direct dependencies of the test-only `viola-e2e` alone.
**Why:** the chunk split the `viola` job per file so that every job ends inside its 120-minute ceiling, and moved the classification of host-excluded twins from the audit's hand into the harness, with the retired syn reader revived for it. The dispatch was allowed by the founder for this chunk only, relayed verbatim by the operator on 2026-10-10; it is an exception, not a change of the ruling.
**Kept:** every sentence that states ruling C2 (dispatched only during the epoch-boundary audit, never a gate, never a dependency of `ci.yml`) stands word for word. Crate dependency direction is not edited: its `viola-e2e` line says that crate's dependencies are not listed there. `timeout-minutes: 120`, the trigger, the permissions, the pins and the step bodies are unchanged.
**Ref:** .andromeda/runs/2026-10-10T08-56-51-wrap/

## 2026-10-10-self-healing-state — the next append heals a torn last line; the reader counts three names; the replay stands as library code
**Section:** §Established Decisions [Database / State Store] · §Standard Contracts → Snapshot envelope · §Infrastructure Patterns → Project directory structure
**Change:**
- [Database / State Store], the decision sentence: "The next append heals a torn last line; readers count it and never rewrite the log" (was "Readers heal a torn last line").
- [Database / State Store], as landed: every appender reads the log's last byte under its lock and, when the log is not empty and that byte is not LF, writes one LF and its line in a single write; no earlier byte moves; `append_event_at` hands and returns L + 1 after a heal, L otherwise; one `state-recovered` line per heal, none without one. The reader `events::read_from` counts per read `unknown_kinds`, `unknown_fields`, `torn_lines`: an unknown kind is not returned; a known-kind line with a key outside the six top-level ones or outside its kind's `data` list is returned and counted once; an over-long line, a non-object line and the unterminated last line are torn. The counts are not logged; showing them is owed to "The board: viola list". Was: the reader skips the unterminated last line uncounted and counts over-long and non-object lines, with healing and `state-recovered` owed to "Self-healing state".
- Snapshot envelope, as landed: `snapshot::read_snapshot_classified` tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first (above 1 unsupported, 1 must parse whole, anything else unreadable); `replay::read_snapshot_or_replay` replays for the last two and logs one `state-recovered` line; the replay writes no file; a field no line gave is absent; `links` replays empty until "Session links". No reader takes the replay: `read_snapshot`'s four product callers still read such a snapshot as absent. The first reader is owed to "viola revive".
- Project directory structure: the `viola-state/` comment names the healing append, the three counts, the classified read, `replay.rs` (no caller yet) and the crate's `tests/` (`state_events.rs`, `state_replay.rs`).
**Why:** the chunk landed the heal in the one shared write path, because a reader holds no lock and writes nothing. The decision sentence's actor was reworded on the operator's own word at this wrap's P2 halt, 2026-10-10: the invariant stands and only the actor differs.
**Kept:** "Readers always tolerate a torn last line" (§Cross-cutting) and the contract sentence that replay recovers `links` stand unchanged.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/

## 2026-10-10-self-healing-state — the root waits read 23 sites in 17 files; tracing-subscriber's dev-dependents named
**Section:** §Occupied Resources → Filesystem · §Stack and Technologies (Logging) · §Infrastructure Patterns → Crate dependency direction
**Change:**
- Filesystem, the root watch: `Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at this chunk's report (was 22 sites in 16 files). The pattern stays the count's rule; `tests/chaos_torn_append.rs` adds one site and one file.
- Stack, Logging row: the root bin is tracing-subscriber's only product dependent, and `viola-channel` and `viola-state` take it as a dev-dependency for their unit tests' line capture (was "root bin only").
- Crate dependency direction, the `viola-state` row: its dev-dependency beside rstest is tracing-subscriber at the workspace pin, for the `#[cfg(test)]` line capture; no product edge.
**Why:** the chunk added a root chaos test with one wait, and gave `viola-state` the test capture `viola-channel` already had. No product dependency moved and no crate entered the graph.
**Kept:** the same count in test-plan §3 → 5-command implementation moved in the same pass.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/

## 2026-10-10-viola-revive — the `revive` verb registered: subcommand, exit-1 refusals, child flags, log producer
**Section:** §Stack and Technologies (CLI parser row) · §Conventions (CLI exit codes, the `1` bullet; CLI) · §Occupied Resources (Binary, subcommands and exit codes; Claude Code integration names; Filesystem, `diagnostics/`) · §Established Decisions [CLI Conventions] · §Infrastructure Patterns → Project directory structure
**Change:**
- The subcommand lists name `revive` after `run` (the Stack row and the Subcommands bullet). `revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]` or `<name> --list` (`--list` conflicts with the other three); no `--json`; exits 0, 1 for a preflight refusal, 2 for a clap usage error, a malformed `--id` included. It sends no channel frame.
- Exit 1 gains revive's preflight: four readings in a fixed order that stop at the first refusal, `strict-modes-failed`, `already-live` (`run`'s collision check and its two pairs, unchanged), `no-session` (two pairs) and `cwd-missing`; each one `unable:` / `hint:` pair holding no recorded directory, pid or logged id, and one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson`. `--list` has two of them as the stderr pair and exit 1 with no log line.
- `role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli` (was `run`, `hook` and a flag).
- "Flag passed to the child" became "Flags": `--plugin-dir` first on every start; a revived start launches `claude` by name on its own `PATH` and adds `--resume <id>`, `--fork-session` with `--fork`, then the words after `--`; the id is one argv element, the newest logged id of the session-id shape or an `--id` the log holds.
- `diagnostics/`: `run-<name>.ndjson` has a second producer, revive's start arm, which logs as process `run`; `--list` opens no process log.
- [CLI Conventions]: `revive` has no machine form yet; its `--json` is owed to the working-route entry "CLI machine contract".
- The directory tree lists `revive` among `src/cmd/`'s modules, with a line for `revive.rs`, and among `human.rs`'s callers.
**Why:** the chunk landed the verb. The logged id on the child's command line is a crossing the founder was not shown as one: the body states what is built and that his word is owed, and no ratification is recorded, on the operator's answer at this wrap's Phase 2 halt. A later wrap records his word as his.
**Kept:** the wording "one endpoint per `viola run`" in the IPC rows and the `VIOLA_*` lines set "by `viola run`" stand: the report states no env or settings fact for revive, and a revived wrapper goes through the same start.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-viola-revive — the snapshot's `cwd` and the child's second cwd source; the instance check before a read
**Section:** §Established Decisions [PTY] · §Standard Contracts (Instance snapshot; Snapshot envelope, the fields no event carries) · §Occupied Resources → Filesystem (`instances/<ViolaName>/`)
**Change:**
- Instance snapshot: `data` gains the optional `cwd?`, the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; additive, `v` stays 1; a host path that lives only in the 0600 snapshot, on no log line, error body or stdout; its one reader is `viola revive`. The envelope's list of fields no event carries gains `cwd`, so a replay gives none and a revive over a replayed snapshot is refused `cwd-missing`.
- [PTY]: the child's cwd has a second source. `run` uses viola's current directory; a revived start uses the snapshot's recorded `cwd`, read only after the strict-modes check on the instance's files and required to be an existing directory, else `cwd-missing` (was one source, viola's current directory). Program resolution and the version gate never read it.
- Filesystem: `viola revive`, both arms, runs `strict::check_instance` over the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists and in that order, before it uses them; a refusal is exit 1 `strict-modes-failed`. Proven on Unix (five widened modes), with no Windows case widening an instance tree's DACL, owed to the route entry "Home and code-bearing file integrity".
**Why:** the chunk landed the field and the check. The recorded directory as a spawn directory is a boundary widening the founder ratified live on 2026-10-10, after it was shown to him as one, the overseer and the operator as relay.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-viola-revive — the replay's first product reader, the session chain, revive as the second way a wrapper starts
**Section:** §Design Philosophy (No daemon) · §Established Decisions ([Database / State Store]; [Session Liveness]) · §Standard Contracts (Snapshot envelope) · §Infrastructure Patterns → Project directory structure (`viola-state/`)
**Change:**
- Snapshot envelope: the fallback has one product reader, `viola revive`'s third preflight reading (was "library code … no reader takes it yet" and a closing sentence that owed the first reader to the route entry "viola revive", per "2026-10-10-self-healing-state — the next append heals a torn last line; the reader counts three names; the replay stands as library code"). `read_snapshot`'s four product callers are named (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice) and still read an unreadable or newer snapshot as absent.
- `replay::session_chain` is registered: one pass over `events.ndjson` through `events::read_from`, one link per `session-start` line whose `agent_session_id` is a string, with the pass's skip counts; it writes and logs nothing. [Database / State Store] names it as `read_from`'s third use (was "the one reader so far", two uses). The directory tree's `viola-state/` comment names the chain, its reader and `strict::check_instance` (was "no caller yet").
- [Session Liveness]: `viola revive` refuses a `live` or `stale` name through the same collision check and its two pairs, as the second of four preflight readings; a passing revive goes through the same start as `run`, so the check runs twice. It resumes the `gone` instance in place: the log is appended to with its earlier bytes unchanged, no launch is replayed, and its first record is `wheel{holder:"driver", cause:"start"}`.
- Design Philosophy: every wrapper owns its endpoint, whether `viola run` or a passed `viola revive` started it; `revive --list` and a refused revive own none (was "every `viola run`").
**Why:** the chunk landed the reader. The wheel at a revived start is the founder's answer, live, 2026-10-10, relayed by the operator: it stays at `driver`.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-viola-revive — root wait count 26 sites in 19 files, the kill path's endpoint wait, ten fake-agent options, a second relied-on shape with no row
**Section:** §Occupied Resources (Filesystem, the test-only `viola-root-watch` bullet; Binary, subcommands and exit codes, Test-only binaries) · §Cross-cutting Patterns → Capability ledger as the single gate for CLI-specific behaviour
**Change:**
- Root wait count: `Instant::now() + WITHIN` reads 26 sites in 19 files under `tests/` (was 23 in 17, per "2026-10-10-self-healing-state — the root waits read 23 sites in 17 files; tracing-subscriber's dev-dependents named"); the pattern stays the count's rule; the three new sites are in `tests/support/home.rs`, `tests/chaos_revive.rs` and `tests/cli_revive.rs`. The same count moved in the test-plan key file `5-command-implementation.md` in this pass.
- The endpoint wait after a kill: `Wrapper::kill` waits on `holder_gone` (on Unix a connect refused or the path absent, on Windows the pipe not found), because a killed wrapper leaves its socket file on Unix, where the stop path's rule is the file being absent. `wait_endpoint_gone` stays the stop path's.
- Test-only binaries: the fake agent has ten argv options (was eight); the list is the count's rule. The two new ones serve revive's tests: `--resume <id>` (the launch SessionStart is the recorded default with `source` `resume` and that `session_id`) and `--fork-session` (beside `--resume`, the compiled id `0f0e0d0c-0b0a-4908-8706-050403020100`; alone it changes nothing). Without `--resume` every payload is the recorded bytes.
- Capability ledger: two relied-on shapes have no row (was one). The second is `claude --resume <id>` reopening the same session with a SessionStart of source `resume`, measured on 2.1.287 on the Linux dev host in three live starts and read under the fake agent on CI. Its row is owed to `v1-34` on the route entry "Paste newline ledger row", beside the LF row. The seventeen rows stand.
**Why:** the counts moved with the chunk's tests and options. Revive landed with no row on the founder's word, live, 2026-10-10, relayed by the operator; the owning entry is the operator's answer at this wrap. The plan's endpoint wait for a killed wrapper was disproved by measurement: on Unix it could never end.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
