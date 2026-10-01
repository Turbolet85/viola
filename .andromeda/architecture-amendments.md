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
