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

## 2026-10-10-statusline-pass-through — the statusline pass-through as landed: the source rule, the arm's order, the override
**Section:** §Standard Contracts (Hook contract; Instance snapshot) · §Occupied Resources (Claude Code integration names: flags passed to the child, per-session settings override; Filesystem: the instance files, `budget.json`, `statusline-source.json`, the user's settings file) · §Established Decisions ([Session Liveness]; [Budget Governor]) · §Cross-cutting Patterns (Config management; Diagnostic output channels)
**Change:**
- Hook contract: `hook statusline` is a hidden arm taken on the word before the event is mapped (`HookEvent` keeps nine). The command's source was "the user's effective Claude Code settings"; now one named source viola only reads: `<home>/statusline-source.json` whenever it exists, else in the default home the user's `.claude/settings.json`, else none, so a `--home` session shows no user status line until the file is planted. The arm's order: canonicalise, `check_instance`, the log, stdin through `take(MAX_FRAME + 1)`, the `budget.json` write only with a `rate_limits` object, the snapshot read, the command. The shell was "the same shell Claude Code would use (a ledger row)"; now one argument of `/bin/sh -c`, environment and directory unchanged, stderr discarded, bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL); none on Windows.
- Settings override: `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, rewritten whole at every start (revived starts too) through `replace_private` at 0600, after the first snapshot and before the spawn; Unix only, and only for a pinned path of ASCII letters, digits and `_ - . / :`. The child gets `--settings <instance dir>/settings.json` after `--plugin-dir` and before the caller's arguments, `--resume` included.
- `budget.json`: `{"v":1,"five_hour","seven_day","read_at"}`, a window `"unknown"` or `{used_percentage, resets_at}`, replaced at 0600 under its lock.
- Instance snapshot: `statusline_command` has its bullet (omitted when absent, `v` stays 1). [Budget Governor]: the wrap is Unix only. Config management: the source follows the home; no `config.json` key, no environment variable.
- The instance files have two checking readers (was one): `revive` and `hook statusline`. `hook`'s stdout on the statusline arm is the user's output.
**Why:** the chunk built the pass-through, and its measurement showed that a check made after the log opens cannot refuse a widened home. The Windows override and shell-out are cut on the founder's word (live, 2026-10-10, relayed verbatim by the operator) and owed to the working-route entry "Windows-only live measurements".
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — four crossings ratified by the founder: the override, the shell-out, the source files, the revive session id
**Section:** §Standard Contracts (Hook contract) · §Occupied Resources (Claude Code integration names: flags passed to the child, per-session settings override)
**Change:**
- The settings override, a code-bearing file written at every start and handed to the CLI with `--settings`, reads as a ratified boundary widening.
- The user's command run through `/bin/sh -c`, the only shell-out in viola, reads as a ratified boundary widening.
- The two files that decide that command (`<home>/statusline-source.json`, and for the default home the user's `.claude/settings.json`) read as a ratified boundary widening, with the file in the viola home as the option chosen.
- The logged session id as the value of `--resume` on a revived child's command line reads as ratified (was "his word on it is owed, and no ratification is recorded here", per "2026-10-10-viola-revive — the `revive` verb registered: subcommand, exit-1 refusals, child flags, log producer"; the rest of that entry stands).
- The `--settings` flag is the agent crate's constant `statusline::SETTINGS_FLAG`; `src/cmd/verify/typed.rs` holds the same literal at two sites from before it, owed to the working-route entry "Interrupted verify cleanup".
**Why:** each of the first three was shown to the founder as itself with its options priced and answered by him live on 2026-10-10 (relayed verbatim by the operator); the session id was shown to him as a crossing with two options and ratified live the same day (relayed verbatim by the overseer). By his ruling of 2026-10-10 a widening he answered before the wrap still halts there as one card; the operator confirmed that card at this wrap, 2026-10-10, as a check that each cited word matches. Standing rule set for later chunks: the playbook's "Boundary widening the founder already answered".
**Kept:** no ledger row gates any of the three statusline shapes; where each row is owed is in the Capability ledger entry of this wrap.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — the capability ledger: three relied-on dependences with no row, and where each row is owed
**Section:** §Cross-cutting Patterns (Capability ledger as the single gate) · §Established Decisions ([CLI Version Compatibility]: the Statusline rows bullet, the closing paragraph)
**Change:**
- Capability ledger: "Two relied-on shapes have no row yet" (per "2026-10-10-viola-revive — root wait count 26 sites in 19 files, the kill path's endpoint wait, ten fake-agent options, a second relied-on shape with no row") is now three relied-on dependences: `send`'s LF inside a paste, `viola revive`'s `claude --resume <id>`, and the statusline pass-through's three shapes (the payload's `rate_limits` object, the `statusLine` the child takes through `--settings`, `/bin/sh` as the shell the CLI runs a statusline command string through). The three were read by hand in three live sessions on 2.1.287 on the Linux dev host with a project-scope status line; CI reads them under the fake agent only. The ledger stays at seventeen rows.
- The rows are owed: `rate_limits` on the working-route entry "Budget governor", the settings override and the Unix shell on "Paste newline ledger row", the Windows legs of both on "Windows-only live measurements".
- [CLI Version Compatibility]: the Statusline bullet of the row list says none of the three has a row yet; the closing paragraph no longer says the statusline rows land with Epochs 4 and 5.
**Why:** the founder chose a hand reading of three live starts with no row, no probe and no fixture for this chunk (live, 2026-10-10, relayed verbatim by the operator); the operator's direction at this wrap, 2026-10-10, named the entry each row is owed to.
**Kept:** the pattern's rule stands: a new dependence on an undocumented `claude` behaviour is a ledger row with a probe. These three are named exceptions with an owner each, not a change to the rule.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — counts, crate edges and one order finding: the fake agent, the seeds, the snapshot readers, revive's log before its check
**Section:** §Occupied Resources (Binary and test-only binaries; Repository: `fuzz/`, proptest regressions; Filesystem: the instance files) · §Standard Contracts (Snapshot envelope) · §Conventions (Data model conventions, Percentages) · §Infrastructure Patterns → Crate dependency direction · §Infrastructure Patterns → Project directory structure
**Change:**
- `viola-fake-agent` has twelve argv options (was ten, per the 2026-10-10-viola-revive entry on the root wait count and the ten fake-agent options; the count's rule is architecture's own list): `--settings <file>` and `--statusline-stdin <file>` serve the statusline tests, and it takes one mode word, `statusline-echo`. It files a `statusline` receipt.
- `fuzz/corpus/hook_stdin` has 12 seeds (was 10; rule `ls fuzz/corpus/hook_stdin | wc -l`), and that target also feeds the statusline payload reader. `crates/viola-agent-claude/proptest-regressions/` holds `statusline.txt` beside `hook.txt`.
- `read_snapshot` has five product callers (was four): `src/cmd/hook/statusline.rs` joins. The rule: `read_snapshot(` under `src/`, outside test modules.
- Percentages: an external budget reading's `used_percentage` is a `Reading<f64>` in `viola_core::BudgetWindow`, kept from 0 to 100 and otherwise `"unknown"`; the `Percent` newtype stays viola's own.
- Crate dependency direction: `viola-agent-claude` names `chrono` (the workspace pin, features `clock` and `std`) for its pure `statusline` module; it gained no `viola-state` edge.
- Project directory structure: `src/cmd/hook/statusline.rs`, `viola-state`'s `budget.rs`, `viola-core`'s budget reading types and the agent crate's `statusline` module are named.
- The instance files: `revive`'s start arm opens its wrapper log before its instance check, and opening a log sets an existing home to 0700, so on `revive` the check cannot refuse a home for a group- or other-writable mode. Written as read in source and not measured on `revive`.
**Why:** each count moved with the chunk's code. The order on `revive` was found while measuring the same mechanism on the statusline arm; the fix is owed to the working-route entry "Budget governor" (the operator's direction at this wrap, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-budget-governor — the budget gate as landed
**Section:** §Established Decisions [Budget Governor], [Session Liveness] (the start order) · §Standard Contracts, Channel methods (`send`, `release`, the answered-after bullet)
**Change:** [Budget Governor] gains the gate as landed (`src/run/budget.rs`, one `BudgetGate` per wrapper, no thread, no timer). A window is crossed only when its `used_percentage` is known and at or above its threshold and its `resets_at` is known, parses as RFC 3339 and is later than now; an `"unknown"`, unparseable or passed `resets_at` never pauses. Both crossed names `five-hour`. Refusal order on `send`: `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait; the gate is read once per `send`. `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read it (read in the diff, no case). An override covers both windows until its end and does not outlive its wrapper: every start evaluates from `budget.json` alone. `release --budget` with no pause records nothing. `budget-gate` and the snapshot's `budget_paused` / `budget_override_until` are written only when (paused, window, override end) changes. Thresholds are read once at start. The gate is fail-open. A snapshot can read `budget_paused: true` after the reset until the next evaluation point. The start order gains the gate's evaluation between the endpoint bind and the first snapshot; record two carries its verdict (was the constant `{"paused": false}`). `release`'s result was `{wheel, budget_paused}`; now `{wheel, budget_paused, override_until?}`: `budget:true` lifts the gate, a plain `release` answers the gate's recorded state (was the constant `false`), and a `budget:true` release is answered only after its `budget-gate` line and the snapshot land (`-32603` otherwise). `send` names its gate read and the `-32603` on a change that cannot be recorded.
**Why:** the chunk built the gate. The decision's wording left open the rung, an unknown reset, the override's reach and life, and a lift with no pause; the operator answered each at planning. The start order claims a place for every step and had none for the gate.
**Kept:** three evaluation points and no poll, as the decision already said. A fourth point on `release --budget` and restoring an override at start were both rejected at planning.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-budget-governor — the instance check before any log, on both arms
**Section:** §Occupied Resources (the instance files, `diagnostics/`, the CLI surface) · §Conventions (CLI exit codes, exit 1) · §Standard Contracts (Hook contract, the statusline arm's order; Instance snapshot, the replay fallback) · §Established Decisions [Session Liveness] · §Infrastructure Patterns → Project directory structure
**Change:** `viola revive`'s start arm runs `check_instance` before its wrapper log opens; was "opens its wrapper log before the check (read in source, not measured on `revive`)", the order owed to the working-route entry "Budget governor". A refusal is the state-files pair and exit 1 with no log opened, no `process-start`, no `process-exit`, no child, the home's mode left as found; was one `process-exit` in `run-<name>.ndjson` with detail `strict-modes-failed`. The preflight has three readings (holder, session, recorded directory); was four, the strict-modes one first. The replay fallback is the preflight's second reading; was "its third". `revive`'s exit 1 is "a refused strict-modes check or a preflight refusal". `viola hook statusline` runs its check before anything under the home is read or opened, and a refused instance returns exit 0 at once with no `config.json` read, no role file, no `hook-invoked` or `hook-decision`, no stdin read; was "exit 0 with nothing printed, no command run and no `budget.json` written (detail `strict-modes-failed`)". Neither arm writes `strict-modes-failed` on any line. `check_instance` has two product call sites (the rule: `check_instance(` under `src/`). Both earlier orders are recorded as measured at the chunk's evidence.
**Why:** opening a log sets an existing home to 0700, so a check made after it passed a home at 0770, and a refusal that then opened its log lasted one invocation. Both were measured and fixed. That a refused check writes no line is the operator's answer at planning; the founder has not ruled on it, and no sentence says he has.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-budget-governor — the threshold keys, the reader of budget.json, the owed ledger row
**Section:** §Occupied Resources (`config.json`, `budget.json`, the root-watch bullet) · §Cross-cutting Patterns (Capability ledger) · §Established Decisions (the capability-ledger bullet) · §Infrastructure Patterns → Project directory structure, Crate dependency direction
**Change:** `config.json` names `five_hour` and `seven_day`: top-level, whole numbers 0 to 100, defaults 90 and 85, read once at the wrapper's start; a value that is no whole percentage takes that key's default, one `parse-rejected{parser:"config-json", detail:"malformed"}` per bad key, and no value switches the gate off. `budget.json` gains its one reader, `viola_state::budget::read_budget`: `check_path` on the home and the file, a shared lock, `take(MAX_FRAME + 1)`, a tolerant parse, `v` = 1; every failure is no reading with no line; it never writes. The wrapper refusing a driver on `budget.json`, a hook-written file off the trusted-file list, is recorded as a boundary widening NOT ratified. The `rate_limits` ledger row and its probe are owed on "Paste newline ledger row"; was on "Budget governor". The gate's refusal rests on that shape with no row, outside the ruled limit "a shape no `send` relies on needs no probe". Root waits `Instant::now() + WITHIN` under `tests/`: 28 sites in 20 files; was 26 in 19. The directory map lists `src/run/budget.rs`, `read_budget` and `BudgetWindowKind`; `viola-core`'s serde use names `BudgetWindowKind`, and no edge changed.
**Why:** the chunk landed the keys, the reader and the refusal. The `budget.json` widening and the unrowed reliance landed on the operator's word, which ratifies neither: the founder's word is owed on both, and the operator directed at the wrap that no card be confirmed. The row's new owner is the operator's direction at the wrap.
**Kept:** the wait count's rule stays the pattern, not a census of every wait on the bound.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-session-links — the implicit link as landed, the explicit verbs held
**Section:** §Established Decisions ([Links]) · §Standard Contracts (Channel methods `send`, `answer`, `wait`, `link` / `unlink`; Event `data` per kind, the wrapper-appended kinds)
**Change:** [Links] gains an as-landed passage: only the implicit link is built. The driven wrapper's `LinkSet` records it on a `send` after the second wheel and running-turn reads and directly before `send-issued`, and on an `answer` once every refusal has passed and the kind matched, before the answer is taken. A link that cannot be recorded is `-32603`, with nothing issued, or the answer not taken and the dialog held. Five shapes stand on the operator's word and are not ruled by the founder: a refused `send` or `answer` records no link; a `from` equal to the wrapper's own name links nothing; the `link` line's `source` is `wrapper`; the record precedes `send-issued`, so an issued send that then reads `not-delivered` has linked; a `link` line with no string `ts` reads `since: "unknown"`. The `send` and `answer` rows carry the same order and the same `-32603`. The wrapper-appended kinds gain `link` (`source: wrapper`, once per new pair, log-only), and the `wait` row's list of wrapper lines that never wake gains it. The `link` / `unlink` row says neither is built and the wrapper answers both `-32601`. No product path appends `unlink`: `LinkSet::remove` has no caller.
**Why:** The chunk built Part A of its plan. The explicit verbs are held for the founder's own word on a boundary widening (two more served methods, the verbs' frame after the liveness-only pre-check, their snapshot read without the strict-modes check); a direction given before the widening is shown to him does not ratify it. The five shapes follow the rule for held items landed on the operator's word: written as built, his word owed, an answer of his that differs comes as a revision.
**Kept:** The decision's design sentences on `viola link`, `viola unlink` and the `{}` result stand as the design. The operator's answers for the unbuilt part (the `link` result `{since, already_linked}`, an `unlink` of an absent pair, the `-32602` shapes) are not written into the body until that part is built.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-10-session-links — the link set in the replay, the snapshot and the start
**Section:** §Standard Contracts (Snapshot envelope; Instance snapshot, `links`) · §Established Decisions ([Session Liveness], [Database / State Store], [Budget Governor]) · §Infrastructure Patterns → Project directory structure
**Change:** Snapshot envelope: was "`links` replays empty until the link kinds land"; now the replay recovers `links` in its one pass through the public rule `viola_state::replay::apply_link_line`: a `link` line whose `data.driver` and `data.driven` are strings adds `{driver, driven, since}` with `since` the line's `ts` unless the set holds the pair, an `unlink` line removes the pair, any other line changes nothing, and a `link` line with no string `ts` gets `since: "unknown"`. Instance snapshot: the wrapper holds the set in `LinkSet` (`src/run/links.rs`, one per wrapper process, one lock), restores it at every start, a revived one included, through `events::read_from` and the rule before `server.serve`, writes it as the first snapshot's `links`, and rewrites `links` through `Snapshots::update` at each change; a failed append leaves the set as it was, and a line that landed is in the set even when the snapshot write then fails. Start chain: the start reads `events.ndjson` twice before the endpoint serves (`WaitFeed::rebuild`, then `LinkSet::restore`), appends no line for links, and its start records are unchanged. The one reader's list gains that restore; `link` and `unlink` are known kinds, returned and no longer counted as `unknown_kinds`. [Budget Governor]: was "no case drives one of them under a closed gate"; now `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) drives `answer`, `wait`, `last` and `pause` under one closed gate, and no case drives `hook.dialog` under one. Directory tree: `src/run/links.rs` is listed, and `viola-state` lists the link rule.
**Why:** The two kinds, the rule and the wrapper's set landed, and the test the previous wrap pinned on this entry landed with them. `since: "unknown"` is a shape no input worded; it is kept on the operator's word.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-11-the-board-viola-list — viola list as landed: a caller-side board, wrapped rows only
**Section:** §Standard Contracts (Channel methods, Session liveness, Snapshot envelope); §Established Decisions ([API Style], [Database / State Store]); §Occupied Resources (Binary, subcommands and exit codes; Filesystem: `instances/`, `budget.json`, `config.json`, `diagnostics/`); §Conventions (exit `1`, exit `21`); §Infrastructure Patterns → Project directory structure
**Change:**
- `list` as landed reads `<home>/instances/` (`viola_state::board::scan`, then `board::read`) and `budget.json` (through `read_budget_stamped`), contacts no wrapper, and reads, spawns and parses no `claude agents --json`: every item is `wrapped: true`, `status: "unknown"`, no unwrapped row. Was: `list` reads `claude agents --json` itself; that read, the unwrapped rows and idle/busy STATUS are now owed to "Unwrapped sessions on the board", the join-field ledger row and probe to "Paste newline ledger row".
- The replay fallback has two product readers (was one): `viola revive`'s preflight and `board::read`, which gives a `live` item for a replayed snapshot only with a beat at most `STALE_AFTER` (5 s) old, never `stale`.
- `events::read_from` gains `board::log_pass`; `viola list` is the first surface that shows the three skip counts (was: no surface shows them, owed to the board entry), summed over every scanned instance, gone ones included.
- Three readers check the instance files (was two): `list`'s scan runs `check_path` on the home and `instances/` and `check_instance` per instance in name order, before any open and before its log; a failure refuses the whole board, exit 21, no log, no line, the home's mode as found.
- `budget.json` has one checked read, `read_budget_stamped` (`{reading, read_at}`), with two product paths: the gate's `read_budget`, which wraps it, and `list`. `config.json`'s thresholds gain `list` as a second reader through `read_thresholds`.
- `diagnostics/`: `list` is a `cli` producer under `VIOLA_NAME` only (self pair, `state-entry`, `state-recovered`, `config-json`); none on a refused scan.
- Exits: `list [--json]` → 0 / 21 / 1 / 2; exit 21 also names `list`'s refusal; exit 1 covers an unlistable `instances/` and a failed log start, a per-instance read error giving no row.
- The tree: `viola-state` gains `board.rs` and `read_budget_stamped`; `src/human.rs` gains `escape_row`, the two state pairs and the board writers; `cmd/list.rs`.
- The link/unlink hold moved to "Explicit session links".
**Why:** the chunk landed the cut the operator directed; the review card the founder approved names every held part's owner. The whole-board refusal and its no-line shape are the operator's answer at planning, not ruled by the founder; the exit-1 paths are implement's decisions, accepted by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-t06-38-35-wrap — the founder's Epoch 4 batch: budget.json ratified, rate_limits ruled, the board's refusal his, the link hold in Epoch 6
**Section:** §Established Decisions ([Links]) · §Conventions (exit codes 1 and 21) · §Occupied Resources (the `instances/<ViolaName>/…` and `budget.json` bullets) · §Cross-cutting Patterns (Capability ledger as the single gate)
**Change:**
- `budget.json`: the wrapper refusing a driver on it is a boundary widening the founder ratified, as built (read behind the strict-modes check on the home and the file, failing open); was "NOT ratified … the founder's word is owed".
- Capability ledger: the `budget-paused` refusal resting on `rate_limits` with no row: the build stays as it is and the row and its probe join "Paste newline ledger row" by his ruling; was "the founder has not ruled on it".
- `viola list`: the whole-board exit-21 refusal with no log opened and no line written is his (exit 21 and the `instances/` bullet); of its exit-1 decisions only an `instances/` that cannot be listed is his, and an entry whose type cannot be read, a failed log start and the no-row read error stay implement's, accepted by the operator, not ruled.
- The instance-check arms: the statusline arm's no-trace shape is his; `revive`'s start arm writing no line stays the operator's answer, not ruled (exit 1 and the `instances/` bullet).
- [Links]: the explicit `viola link` / `viola unlink` stay held and the widening stays not ratified; the hold's entry "Explicit session links" now stands in Epoch 6 after "Windows endpoint admission" and "Server verification before any frame"; was the last Epoch 5 entry.
Each superseded mark is kept in the body as the operator's word, superseded 2026-10-13.
**Why:** the founder's own answers of 2026-10-13 to the epoch-boundary batch («Подтвердить всё», «Строку к Paste newline ledger row», «Ратифицировать», «Не делать до Эпохи 6»), relayed verbatim by the Viola overseer through the operator's directive. The operator scoped them at this wrap: the `revive` arm and the list's other exit-1 paths were not in the question shown to him, so they go to his next batch.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-epoch-4-cleanup — send and human split by phase
**Section:** §Infrastructure Patterns → Project directory structure; §Infrastructure Patterns → Build system; [Delivery Confirmation]; [CLI Version Compatibility]
**Change:**
- The tree: `src/human.rs` is the `human` module root — the shared writers (`write_refusal` / `refuse`, `write_internal_error` / `internal_error`, `write_result` / `result`) and `pub(crate) use` re-exports, so every caller keeps its path; its phases sit under `src/human/`: `mirror.rs` (`send`'s readback mirror, `write_wrapper_fault`, `write_unable`), `escape.rs` (`escape_message`, `escape_row`), `board.rs` (the two state pairs and the board writers), `wait.rs` (the `wait` / `last` lines), `verbs.rs` (the `answer`, `pause` / `release` and budget lift / open lines), `hints.rs` (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint`). Was: one file holding all of it.
- The tree: `run/send.rs` is the `send` module root — the wrapper's `send` on `Ctx` with its refusal order, re-exporting `SendSlot`, `append_hook_event`, `parse_from`; its phases under `run/send/`: `slot.rs` (the one-in-flight `SendSlot`, `append_hook_event` with the driver relabel, the `session-start` post-condition claim), `classify.rs` (the local-command decision), `params.rs`, `budget.rs` (`Ctx::budget_paused`), `relabel.rs` (only `#[cfg(test)]` cases), `test_support.rs` (`#[cfg(test)]` fixtures). Was: one file.
- Build system: the root bin's only human writer is the `human` module (`src/human.rs` and `src/human/`); was `src/human.rs`.
- [Delivery Confirmation] cites the claim / relabel / settle at `src/run/send/slot.rs` and the local-command decision at `classify` in `src/run/send/classify.rs`; [CLI Version Compatibility] cites `send_hint` at `src/human/hints.rs`. Was `src/run/send.rs` and `src/human.rs`.
**Why:** the audit's M1 and M2 (the two files at 2088 and 1064 code lines), split along their phases as a pure move: no output, refusal, span or event moved, and no file of either split over 800 code lines.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-epoch-4-cleanup — the mutation scratch re-created owner-only
**Section:** §Occupied Resources (the `<repo parent>/viola-mutants-scratch/` row)
**Change:** the wiped scratch is re-created through `viola_state::fs::create_private_dir`: on Windows it is the topmost folder created outside the profile and takes the protected user + SYSTEM DACL, so every temp copy and test home laid inside inherits owner-only, as one under `%TEMP%` does (a plain-created scratch on the runner's workspace drive inherits BUILTIN\Users write, refused `Writable` by the instance check). Witness: `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` on CI's `test (windows-2025)` leg, as measured at ci#38375200217. The root package's Windows mutation baseline reading green is not yet measured (the next `windows-mutants.yml` dispatch). Was: "removed and recreated" with no lay-down mode.
**Why:** run 38215104120's red baseline on the root package — its cause was the scratch's DACL, not the instance check, which stands unchanged.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-epoch-4-survivors — the replace loop's retry decision passed in, the pty watch report per process, the answers tempdir
**Section:** [Snapshot writer]; Occupied Resources → Filesystem (the viola-pty watch report); Occupied Resources → Repository
**Change:**
- [Snapshot writer]: the replace loop sits in the private `replace_private_retrying`, which takes the decision to retry a failed rename (from its raw OS error) from its caller; `replace_private_with` keeps its signature and passes `retry_replace(_, HOST_IS_WINDOWS)`, so the product's behaviour stands as stated (mode before any byte, `sync_all`, `persist`, at most `REPLACE_ATTEMPTS` = 100 attempts, only raw OS error 5 on a Windows host retried) and the loop's attempt bound is exercised by a unit case on every host. Was: the loop inside `replace_private_with`, its decision the const alone.
- Filesystem: the real-PTY report files are `<temp dir>/viola-pty-watch/<test name>.<pid>.report` and `<test name>.<pid>.test.report`, `<pid>` the test process's id, so two test processes running the same test never share one; keep-on-failure and remove-on-pass unchanged. Was: `<test name>.report` / `<test name>.test.report`.
- Repository: new test-only row `target/viola-answers-*/` (under the test process's current dir), the tempfile dir of `answer_reads_no_relative_answers_dir_that_resolves`, reached only by `../<current dir's name>/target/<tmp>/answers`, removed on drop, gitignored.
**Why:** the Epoch 4 boundary audit's survivors: five const-gated `replace_private_with` mutants reachable on Linux only once the caller supplies the decision; the `resize` mutant read missed by the audit yet red when its tests ran alone, so the shared report file is closed off (its two-job cause recorded, not established); a relative path to a temp dir on another Windows drive does not exist.
**Ref:** .andromeda/runs/2026-10-13T16-59-54-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's Ctrl-C / TERM stop, the owed literals moved, the Windows matrix line
**Section:** §Occupied Resources → Filesystem (`ledger/probes/<pid>/`, verify's interactive probe dirs, `diagnostics/` verify's spawn pairs) · §Occupied Resources → Claude Code integration names · [CLI Version Compatibility] (Run B's ending) · §Conventions → CLI exit codes · §Infrastructure Patterns → Crate dependency direction · CI/CD approach · Project directory structure
**Change:**
- Verify's dirs were "removed on every exit path"; now they are removed when verify passes, fails, or is stopped by Ctrl-C or TERM: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`, through `cmd::verify::stop`, installed by `viola verify` alone; its handler only sets one flag every wait reads. A stopped run kills and reaps its child with no key, and its drop guards remove only what that run created, never by a pattern or a listing. SIGKILL, a crash, a failed install (ignored, fail-open) and a Windows close, logoff or shutdown keep the OS default and leave the dirs by hand.
- Run B was ended only by Ctrl-C, Ctrl-C again, then a kill; a stopped run is now ended by a kill with no key, starts no later run and writes no stamp.
- Exit 1 covers an interrupted `verify`: no stamp, stdout empty, stderr exactly `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, and `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` under a `VIOLA_NAME`. A child killed at the stop closes its spawn pair with `duration_ms` and no `child_exit_status`.
- Was: Runs C and D's two `--settings` literals were owed to "Interrupted verify cleanup". Now verify reads `SETTINGS_FLAG` and its default program reads `viola_agent_claude::PROGRAM`.
- The root bin's `windows-sys` gains `Win32_System_Console` `SetConsoleCtrlHandler`, and its `libc` gains `sigaction`, both for `src/cmd/verify/stop.rs`; no dependency or feature was added.
- `windows-mutants.yml` was nine items with the root package four; now ten and five (`viola-cmd-verify-stop` · `src/cmd/verify/stop.rs`).
- The tree lists `verify/stop.rs`.
**Why:** The scope is the founder's narrow entry (relayed by the overseer, 2026-10-10). The event set, the exit 1 and the `detail` are the operator's P4 answers; the pair is a planning form with the founder's wording owed. The matrix line was forced by `tests/contract_windows_mutation_scope.rs` and taken on the operator's word at implement; nothing was dispatched. A later chunk adding a `cfg(windows)` product file owes that workflow its line.
**Kept:** «all gone when verify ends» (`:100`) and run 38036448183's «all nine jobs» stand, both still true; real Windows console delivery stays owed to "Windows-only live measurements".
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-13-paste-newline-ledger-row — eighteen ledger rows, `paste-newline`, Run B's fifth paste, the owed rows moved to "Owed ledger rows"
**Section:** §Established Decisions → [CLI Version Compatibility] · [Delivery Confirmation] · §Cross-cutting Patterns (Capability ledger) · §Standard Contracts (`list`) · §Occupied Resources (Repository · Binary, subcommands and exit codes) · §Infrastructure Patterns → CI/CD approach · Crate dependency direction
**Change:**
- Was seventeen rows; now eighteen, the members of `LedgerRow::ALL`, the last `paste-newline` (an LF typed inside a bracketed paste stays inside the one prompt submitted). It is checked by `pasted(&probes.trusted, PROBE_NEWLINE_PASTE)`. Stamped on live 2.1.287 on the Linux dev host only; Windows owed to "Windows-only live measurements".
- `viola verify` prints `[NN/MM]` step lines, `MM` = `LedgerRow::ALL.len()`, 18.
- A stamp of the fourteen or the seventeen older ids, all `pass`, reads unverified. Every home stamped before this chunk reads `cli_verified:false` until its next `viola verify`.
- The hypothetical PermissionRequest `updatedInput` body row is now the 19th.
- Run B: was four compiled pastes; now five. `PROBE_NEWLINE_PASTE` (113 bytes, one inner LF, no CR) goes before `/clear` under the same per-paste guards. `FRAMING_TURNS` `paste-1` · `paste-2` · `paste-3` · `clear-1`, and `box_wait` holds after each added text turn.
- [Delivery Confirmation] and Capability ledger: the inner-LF shape is rowed. Was three relied-on dependences without a row; now two, `viola revive`'s `--resume` and the statusline's three shapes.
- The rows still owed (and the `claude agents --json` join-field row in `list`) moved from "Paste newline ledger row" to "Owed ledger rows". `cli_verified` is eighteen-row.
- Repository: `2.1.287` holds 5 framing variants (29 files), stamped at eighteen rows (`18 pass  0 fail`); `--framing` replays `paste-3` through `framing_stem`, with no option added.
- CI/CD: verify covers eighteen rows. Crate dependency direction: `ledger` holds eighteen rows and `PROBE_NEWLINE_PASTE`.
**Why:** The newline paste is a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 1, relayed verbatim by the operator (the chunk's `inputs#I2`). The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The owed rows' move is his own answer to fork 3 (`inputs#I2`), minted on the operator's word (`inputs#I4`). Counts carry their rule (`LedgerRow::ALL`).
**Kept:** the `stamped 2.1.287  17 pass  0 fail` of chunk 2026-10-08's re-verify round stands as a dated record.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/

## 2026-10-13-owed-ledger-rows — three statusline ledger rows and the per-OS relied set
**Section:** §Established Decisions → [CLI Version Compatibility]; §Standard Contracts → Ledger stamps envelope, hook statusline; §Occupied Resources → Claude Code integration names (settings override); §Cross-cutting Patterns → Capability ledger; registries `ci-cd-approach`, `crate-dependency-direction`
**Change:**
- Was eighteen rows; now twenty-one, the members of `LedgerRow::ALL`, ending `paste-newline` · `statusline-override` · `statusline-shell` · `statusline-rate-limits`.
- The relied set is `LedgerRow::relied()`: `ALL` filtered by `relied_on(row, statusline::WRAPPED_HERE)`. All twenty-one are relied on Unix, the first eighteen on Windows.
- A version is verified only when every relied row reads `pass` (was every landed row). Unix reads an all-`pass` stamp of the eighteen pre-chunk ids as unverified, Windows as verified.
- `MM` in the `[NN/MM]` step lines is `LedgerRow::relied().len()`: 21 on Unix, 18 on Windows.
- `Probes` gains `statusline`. Its three checks:
  - override: at least one capture;
  - shell: at least one capture, every verdict `bin-sh`;
  - rate-limits: a capture whose `rate_limits.five_hour` and `.seven_day` each hold an integer `used_percentage` 0–100 and an integer `resets_at` (`whole_rate_limits`).
- `--record` also writes `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json` (`ledger::statusline_variants`).
- The statusline override, `/bin/sh` and `rate_limits` are now rows. They were "relied on with no row", and `budget-paused` now rests on a stamped row. They are stamped on live 2.1.287 on the Linux dev host (`21 pass  0 fail`). Their Windows legs are owed to "Windows-only live measurements".
- A single relied dependence still has no row: `viola revive`'s `--resume`.
- `cli_verified` is read over the relied set. The hypothetical PermissionRequest body row is now the 22nd.
- CI/CD: verify covers the relied rows. Crate dependency direction: `ledger` holds the twenty-one rows, `relied()`, `ShellVerdict`, `StatusLineCapture` and `statusline_variants`; `statusline` adds `WRAPPED_HERE`, `BIN_SH`, `SHELL_FLAG` and `probe_document`.
**Why:** The three shapes are relied on only where the status line is wrapped. Shipping them on Windows would leave every Windows home unverified, and passing them there would claim an unmeasured shape. The founder chose the cut (P4 fork 1, `inputs#I1`, relayed verbatim by the operator). Counts carry their rule (`LedgerRow::ALL`, `relied()`).
**Kept:** the paste-newline chunk's dated `stamped 2.1.287  18 pass  0 fail` readings stand as history.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm and the moved rows
**Section:** §Established Decisions → [CLI Version Compatibility], [Hook Contract], [Plugin Scope]; §Standard Contracts → list, hook statusline; §Occupied Resources → Hidden flags, Claude Code integration names, viola home, Repository; §Cross-cutting Patterns → Diagnostic output channels, Capability ledger; registry `project-directory-structure`
**Change:**
- Run B, on Unix and only for shell-plain paths, gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (`statusline::probe_document`, 0600). Its `statusLine` command is `<pinned viola> hook statusline --capture <dir> --shell "$0"`.
- Windows passes no `--settings`. Runs A, C and D keep their argv, and no run is added.
- On Unix, Run B's status line is the probe capture, not the user's.
- New hidden arm: `hook statusline --capture <DIR> [--shell <WORD>]`. It files `StatusLine.<k>.json` and `StatusLine.<k>.shell` (`bin-sh` | `other`) under the same claim and 0600 write. The word is written nowhere.
- The arm reads no `VIOLA_*`, does no obs init, opens no channel, writes no `budget.json`, prints nothing and exits 0.
- The probe dir gains `typed/statusline/` and `typed/statusline.json`.
- Repository:
  - `2.1.287` gains the two `StatusLine` variants (31 files) and is stamped at the relied set.
  - New `schemas/claude-statusline-fixture.v1.json`.
  - A `workspace.repo` naming this repository's own remote is kept in a fixture.
- The fake agent's `--framing` replays the variants through a `--settings` statusLine command. The `"$0"` word is handed over as `/bin/sh`, with no shell. The first-Stop replay comes after the turn screen.
- Moves:
  - `--resume` and `/compact` were owed on "Owed ledger rows"; now owed on "Resume and compact ledger rows".
  - The `claude agents --json` join row and probe were owed on "Owed ledger rows"; now owed on "Unwrapped sessions on the board".
**Why:**
- The probe `--settings` and the capture arm are a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
- The kept `workspace.repo` is the founder's own word (`inputs#I3`, relayed verbatim by the operator, given after it was shown to him). Its landing in each master is the overseer's application of it.
- The moves are his answers to forks 1 and 2 (`inputs#I1`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — the resume row, Run E and /compact listed
**Section:** §Stack and Technologies (Screen model); §Established Decisions → [CLI Version Compatibility], [Plugin Scope], [Delivery Confirmation]; §Standard Contracts → Ledger stamps envelope; §Cross-cutting Patterns → Capability ledger; registries `ci-cd-approach`, `crate-dependency-direction`, `project-directory-structure`
**Change:**
- The ledger holds twenty-two rows. The 22nd, `resume-session` (`--resume <id> reopens the session under that id`), is relied on every OS.
- The relied set is 22 on Unix and 19 on Windows (the first eighteen and `resume-session`), so `MM` is 22 / 19.
- Older stamps:
  - Was: an all-`pass` stamp of the eighteen ids verified on Windows.
  - Now: one of the 14, 17, 18 or 21 older ids reads unverified on every OS, and eighteen plus `resume-session` verifies on Windows only.
  - Every home stamped before reads `cli_verified:false` until it is re-stamped.
- verify drives five interactive runs (was four). Run E runs last:
  - it takes Run B's first SessionStart id through `ledger::resume_target`, of the session-id shape only; with none it spawns nothing and the row fails;
  - it runs in a 0700, guarded `<cwd>/.viola-verify-<pid>-resume/`;
  - its argv is the run args, then `--plugin-dir` with the `resume/` capture plugin, then `resume_args(id, false)`;
  - input is empty and there is no `--settings`;
  - it waits for one SessionStart, settles, and is killed.
- A verify is six `claude` starts. `Probes` gains `resumed`, and `--record` writes `SessionStart.resume-1.json` (`resume_variants`).
- `LOCAL_COMMANDS` holds `/clear` (a new session), `/remote-control` and `/compact` (none). A `/compact` send is `unconfirmable` at once.
- The Capability ledger's "one relied-on dependence has no row" (`--resume`) is retired.
- `/compact`'s post-condition row is owed to "Compact post-condition ledger row" at the end of Epoch 5.
- As measured live on 2.1.287 on the Linux dev host:
  - `stamped 2.1.287  22 pass  0 fail`;
  - `--resume <id>` found Run B's session from Run E's own `-resume` project dir;
  - Run E added no transcript.
**Why:**
- Run E is a boundary widening of the verify child set. It is ratified by the founder's own answer to P4 fork 2, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer, `inputs#I6`), superseded by his own word at the Epoch 5 boundary.
- Listing `/compact` and owing its row is his answer to fork 1 (`inputs#I1`).
- The row's placement is his own word at this wrap, relayed verbatim by the overseer (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — Run E's resources and a killed PTY child's exit status
**Section:** §Occupied Resources → Binary, subcommands and exit codes (test-only binaries), Claude Code integration names, Filesystem (`diagnostics/`, `ledger/probes/<pid>/`, interactive probe dirs), Repository
**Change:**
- **Integration names:** verify has five children.
  - Run E's argv: the run args, `--plugin-dir` with the `resume/` plugin, then `--resume <id>` from `resume_args`, its second caller beside `viola revive`.
  - Run E's cwd: `-resume/`.
- **`diagnostics/`: seven spawn pairs** (was six), Run E's `verify-pty-probe` last.
- **A child killed at a Ctrl-C or TERM stop:**
  - Was: it closes with `duration_ms` and no `child_exit_status`.
  - Now: a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code, as measured by the resume-run stop case.
  - A print-mode or `--version` probe child killed at the stop is not measured.
- **Probe dirs:** the probe dir gains `resume/`, and the interactive dirs gain Run E's `-resume/`.
- **Repository:**
  - The stem `SessionStart.resume-<n>.json` (`resume_variants`) is hygiene-walked.
  - `2.1.287` gains `SessionStart.resume-1.json` (10 key names) and reads 32 files (rule `ls fixtures/claude/2.1.287 | wc -l`).
  - It is stamped at 22 / 19 by `contract_ledger_probes`.
- **The fake agent** (test-only):
  - `--resume <id>` builds the launch SessionStart on the set's `SessionStart.resume-1.json` when the set holds it (`launch_variant`, `RESUME_VARIANT`), else on `SessionStart.default`.
  - It also serves Run E under the fake agent.
  - It still has twelve options.
**Why:**
- The `child_exit_status` wording was measured false. The behaviour predates this chunk's code. It is amended to the measured scope on the overseer's founder-delegated word (`inputs#I6`).
- The resources follow Run E, ratified by the founder's own answer to P4 fork 2 (`inputs#I1`, relayed verbatim by the operator; card confirm PROVISIONAL, delegate viola-overseer).
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-16T14-58-10-wrap — 0.1.0 is a pure CLI; the frontend moves to 0.2.0
**Section:** §Stack and Technologies (HTTP server, Push / real-time, Mobile and Browser e2e rows); §Established Decisions ([Concurrency], [Real-time Strategy], [GUI Control Scope], [Mobile], [Deferred], and the wheel, link and budget entries); §Conventions (interfaces, `v`, GUI routes, GUI HTTP errors, error enums, CLI flags); §Standard Contracts (the GUI block, the `list` payload); §Occupied Resources (protocols, Network, HTTP routes, `ui/<port>.url`, role logs, `config.json`, `budget.json`, stamps); §Cross-cutting Patterns; §Project Intent; §Inherited Defaults; §Infrastructure Patterns → Deployment model, Build system, Crate dependency direction, Project directory structure
**Change:**
- Was a view-only loopback GUI: `viola ui` (axum + tower-http on `127.0.0.1:47319`), SSE `/api/events`, cookie-gated `/api/*`, RFC 9457 errors, `ui/<port>.url`, the `viola-ui` crate, Tokio in `mcp` and `ui`. Now the frontend is deferred to 0.2.0 and its technology is not chosen. [Real-time Strategy], [GUI Control Scope] and the GUI contracts block hold that stub line. [GUI Control Scope] keeps what the page shows as the 0.2.0 frontend's starting intent.
- Four interfaces from one binary. Tokio only in `viola-mcp`. No `ui` subcommand, `--port`, GUI port setting, `UiError`, `ui-<port>` role file, notify tail or listening TCP port.
- The item, `budget` and `skipped` shapes leave the GUI envelope and stand as the `list` payload contract (CLI `list --json`, MCP `list`). §Conventions, the board read and the MCP rows cite it.
- The product type's third part is the frontend, deferred to 0.2.0. A phone view is a later version, behind authentication.
- Build system: the 2026-09-30 React + TypeScript ruling is replaced by "its technology is not chosen".
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI, moves the whole frontend to 0.2.0 and cuts the masters' web sections to a stub. The operator's answer at this wrap (overseer, technical, founder-delegated) keeps the masters true to the tree, so the browser test leg stays, marked as owned by the route entry "Remove the browser leg".
**Kept:**
- The browser-e2e Stack row and its Occupied Resources rows, marked.
- `deny.toml`'s `axum` `http2` ban and the `crates/viola-ui/` tsconfig guard.
- The absences that stay true: no WebSocket and no polling endpoint.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-remove-the-browser-leg — the Browser e2e row and the leg's occupied resources leave
**Section:** §Stack and Technologies (CI/CD, Browser e2e, Code quality rows); §Occupied Resources (CI workflow data lines, Test-side install sites, `target/nextest/ci/junit.xml`, `target/npm-audit/audit.json`, the `e2e-web/` outputs, `target/run-archive/<n>/`)
**Change:**
- The Browser e2e (test-side only) row is gone: Node v24.21.0 with its `NODE_PIN_*` pin and `scripts/install-node.sh`, `@playwright/test` 1.63.0 and its Chromium, `e2e-web/` with `npm ci` and `scripts/npm-audit.sh`, `run --browser` and the pre-push browser stage. The stack names no test-side Node, npm or browser tool.
- CI/CD row: the pre-push runs every child through `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, and its `tools` stage checks `cc`, the channel and ci.yml's `test`-job pins (was also `NODE_PIN_VERSION`). Code quality row: `jq`'s consumers are G2, `scripts/release-check.sh` and `scripts/orphans-check.sh`.
- Occupied Resources: the `NODE_PIN_*` workflow data lines, the Node and Playwright install sites (`$RUNNER_TEMP/node`, `~/.local/viola-node/`, `~/.cache/ms-playwright/`), `target/npm-audit/` and the `e2e-web/` outputs leave the registry. `junit-<os>` carries `target/nextest/ci/junit.xml` alone; the run archive holds `junit-{nextest-unit,nextest-integration,coverage}.xml` and `outcomes.json`.
**Why:** This chunk removed the browser test leg (founder ruling 2026-10-16, item 3), as the row's own stand-until clause asked of this entry's wrap. No resource was added.
**Kept:** The npm-shim → `claude.exe` resolution text, the product's own reading of the CLI's npm shim.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-remove-the-browser-leg — Infrastructure Patterns: the browser pipe, the ts plane and the axum ban leave
**Section:** §Infrastructure Patterns → CI/CD approach (nightly jobs, Setup steps, Jobs: `test` and `supply-chain`, the pre-push bullet); → Build system (Dependency policy, Code-graph planes **ts**); → Project directory structure (`scripts/`, `.config/nextest.toml`, `e2e-web/`, the workflow comments)
**Change:**
- CI/CD approach: `nightly.yml` runs two ubuntu jobs, `advisories` and `fuzz` (was three, `npm-advisories`). Setup steps install no Node, run no `npm ci` and no Chromium install; `jq`'s refusing scripts are G2, `release-check.sh` and `orphans-check.sh`. The `test` job has no browser suite, `junit-<os>` carries `target/nextest/ci/junit.xml` alone and `Gate verdict` runs `gate --require coverage,doctest`; `supply-chain` has no Node or npm lockfile step. The pre-push runs every child under `PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, checks the C linker, the channel and ci.yml's cargo tools, then `run --coverage` and `gate --require coverage,doctest` (was also `node --version`, `run --browser` and `playwright`).
- Build system: the feature bans are `veil/toggle`, `rmcp` `auth` / `transport-streamable-http-server` and `tracing-subscriber/env-filter` (`axum/http2` and its probe left). The **ts** plane has no input in 0.1.0: its one input, the tracked `e2e-web/tsconfig.json`, left with the browser leg, so `scripts/code-graph.py` detects no ts plane; the `crates/viola-ui/` tsconfig guard is gone.
- Project directory structure: `e2e-web/`, `scripts/install-node.sh` and `scripts/npm-audit.sh` leave the tree; `.config/nextest.toml` holds the `ci` and `mutants` profiles (the `fixed-port` group left); the `ci.yml` and `nightly.yml` comments name no browser suite, npm audit, `NODE_PIN_*` env or `npm-advisories`.
**Why:** This chunk removed the browser test leg (founder ruling 2026-10-16, item 3). The axum ban and its probe went on the operator's P4 answer as the overseer (technical, founder-delegated; `inputs#I1`): axum is in no lockfile and the ban's reason left with the GUI.
**Kept:** The 1.96 floor's text: build-system's ban list was its only axum mention.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — Standard Contracts: the agents read, the join and the unwrapped rack land
**Section:** §Established Decisions [API Style]; §Standard Contracts (`list` payload, `list` is not a channel method, Session liveness); §Occupied Resources (Filesystem → `diagnostics/`, the `list` lines); §Infrastructure Patterns → Project directory structure (`list.rs`, `run/agents.rs`, `viola-state/`), → Crate dependency direction (`viola-agent-claude`)
**Change:**
- CLI `list` reads the disk (`board::read`), then makes one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from its current directory) and joins it by `viola_state::board::join`: a row matching a wrapped item's `child_pid` or `agent_session_id` sets its `status` `idle` / `busy`; every unjoined row is an unwrapped item (`wrapped: false`, its name the row's `sessionId`, `live`, `dialog_pending: false`, no `wheel` / `budget_paused` / `cli_version` / `cli_verified`), sorted by name after the wrapped items. A failed, oversize or malformed read leaves the board as `board::read` gave it. Was: `list` reads, spawns and parses no `claude agents --json`, every status `unknown`, the read owed to this entry.
- Session liveness: the read is spawned by `list`, `revive`'s preflight and verify's Run B through `run::agents` (`read_logged`: R8 strip, `take(MAX_FRAME + 1)` by `version_gate::run_capped`, 5 s kill, one `agents-probe` pair under an open log), parsed by the pure `viola_agent_claude::agents::parse` into rows or `AgentsFailure` (`oversize` · `malformed`); an over-cap read joins the failure outcome; `mcp`'s `list` is not built. Both join fields are exposed (`pid`, `sessionId`) and the join is row `agents-join` (was: none does yet; which field, an open item). The `list` payload's `wrapped: false` name is the `sessionId`.
- diagnostics: `list`, only under a parsed `VIOLA_NAME`, writes one `agents-probe` pair per read and one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer, holding no row text, id, `cwd` or path.
- The keys register `run/agents.rs`, the agents read in `list.rs`, `viola-state`'s `JoinKeys` · `Observation` · `join` · `Item::unwrapped` and `liveness::start_times`, and the pure `agents` module of `viola-agent-claude` (`KEYS`, `AgentStatus`, `AgentsRow` / `joins`, `AgentsList`, `AgentsFailure`, `parse`).
**Why:** The chunk built the board's agents read (`matrix#v1-37`). `list`'s own spawn was stated before it; `revive`'s and verify's are ratified in the sibling entries.
**Kept:** No Stack row moves: no crate, dependency, IPC method, port or env var was added. [Plugin Scope]'s transient-session list stays: Run B's agents read is a bounded child, not a hook-bearing session.
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — CLI Version Compatibility: the agents-join row and Run B's agents read
**Section:** §Established Decisions [CLI Version Compatibility] (the open items, the row list, the relied set, `Probes`, Run B, step lines, `--record`, the older-stamp clause, the 24th row); §Standard Contracts (Ledger stamps envelope); §Cross-cutting Patterns (Capability ledger); §Occupied Resources (`diagnostics/`: verify's pairs); §Infrastructure Patterns → CI/CD approach, → Crate dependency direction
**Change:**
- `LedgerRow::ALL` holds twenty-three rows, `agents-join` last (`claude agents --json names the live session by pid and session id`), relied on every OS: 23 on Unix, 20 on Windows (the first eighteen, `resume-session`, `agents-join`); step lines end `[23/23]` / `[20/20] agents-join`. Was twenty-two, 22 / 19, ending `resume-session`.
- `Probes` gains `agents` (`AgentsCapture`): the row passes only when the capture parses and holds exactly one row with Run B's child pid, the `session_id` of Run B's first SessionStart capture and status idle or busy (`agents_joined`); no capture fails.
- Once Run B's first turn has settled, before the framing pastes, verify makes ONE `claude agents --json` read (`typed::agents_read`: Run B's program, the probe dir, R8 strip, `VERSION_DEADLINE`), the capture in memory; verify writes eight spawn pairs (was seven), the eighth `agents-probe`; no live start is added; `--record` keeps Run B's own row as `Agents.recorded-1.json` (`ledger::agents_variant`). The open item is answered: both fields exposed, as measured on 2.1.287 on the Linux dev host (`stamped 2.1.287  23 pass  0 fail`), its Windows reading owed on "Windows-only live measurements".
- A stamp of the twenty-two pre-chunk ids reads unverified on every OS, and one of the eighteen and `resume-session` on Windows; the next row is the 24th; the agents join landed, plugin precedence stays owed. `list` and `revive` gate their read on no stamp.
**Why:** Run B's read widens verify's child set: ratified by the founder himself, his own answer to P4 fork 1 (`inputs#I1`, relayed verbatim by the operator); the one-read narrowing on the operator's yes (`inputs#I2`); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. Every home stamped before this chunk reads `cli_verified:false` until its next `viola verify`.
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — Session Liveness: revive refuses a session id live elsewhere
**Section:** §Established Decisions [Session Liveness]; §Conventions (exit code `1`); §Standard Contracts (Snapshot envelope); §Occupied Resources (Filesystem → `diagnostics/`, `revive`'s lines); §Infrastructure Patterns → Project directory structure (`revive.rs`)
**Change:**
- `viola revive`'s preflight takes four readings (was three): the holder, the logged session, `session-live`, the recorded directory, stopping at the first refusal. `session-live` is one `claude agents --json` read: a row whose `sessionId` is the logged id about to be resumed refuses `detail:"session-live"` with `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`, one `process-exit{subject:"self", exit_code:1, detail:"session-live"}` and no child; `--fork` skips it; a failed read passes (fail open). Its `agents-probe` pair lands in `run-<name>.ndjson`. The logged session is the second reading of four (was of three).
- As measured on 2.1.287 on the Linux dev host (the chunk's P4 / P12 leg): a second interactive `claude --resume <id>` of an id a live `claude` holds runs under the same id, two writers on one transcript, so the reading stays a refusal; after `/clear` the agents row of the session's pid names the new id, so a session-id join survives `/clear` because the wrapper rewrites the snapshot's `agent_session_id` on each `session-start`.
**Why:** `revive`'s agents read and refusal are the founder's own answer to P4 fork 3 and the pair his own answer to fork 4 (`inputs#I1`, relayed verbatim by the operator); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. P4 read two writers, so no "Session-live as a warning" entry is minted (the operator's P5 card, `inputs#I2`).
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — Occupied Resources: the fake agent's agents mode, the Agents fixture class and the sixth fuzz target
**Section:** §Occupied Resources (Binary → test-only binaries: the fake agent; Repository → `fixtures/claude/<cli-version>/`, the schemas bullets, `fuzz/`); §Infrastructure Patterns → Project directory structure (`schemas/`, `fixtures/claude/`, `fuzz_targets/`)
**Change:**
- The fake agent takes two mode words (was one, `statusline-echo`): `agents` answers `agents --json` from files beside its own executable — `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows, printed unchanged) and a live `sessions/<pid>.json` registry its interactive runs write (a row written just before its launch SessionStart fires, its `cwd` the launch fixture's recorded `cwd`, `~` without one; a dead or reused pid left out through one `viola_state::liveness::start_times()` read). No option and no env are added.
- `fixtures/claude/`: Run B's agents read is written as `Agents.recorded-1.json` (Run B's own row only), through the same scrub and refusal, validated by the new `schemas/claude-agents-fixture.v1.json`, hygiene-walked and replayed by the fake agent's `agents` mode. `2.1.287` holds 33 files by `ls fixtures/claude/2.1.287 | wc -l` (was 32), stamped at the relied set (23 / 20, `ROW_IDS` the twenty-three ids, `23 pass  0 fail` on Unix).
- `fuzz/`: six targets by `ls fuzz/fuzz_targets | wc -l` (was five), the sixth `claude_agents_json` with 6 synthetic seeds, kept byte-exact by `.gitattributes` `fuzz/corpus/claude_agents_json/** binary`.
**Why:** Files beside the exe are the founder's own answer to P4 fork 2 (`inputs#I1`, relayed verbatim by the operator): no third fake-agent env seam. The registry row replays recorded values only, because the host's real cwd leaked a CI checkout path into `--record`.
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-end-to-end-path-binary-and-harness-legs — arch: the harness session guard reads the keep variables; bin/agents-mode
**Section:** §Occupied Resources → Environment variables (`AGENT_RUN_KEEP_HOMES`, `AGENT_RUN_KEEP_FAILED`); §Occupied Resources → Filesystem (`target/agent-run/<session>/`)
**Change:**
- `AGENT_RUN_KEEP_HOMES` and `AGENT_RUN_KEEP_FAILED` gain a reader: viola-e2e's `harness::session` guard, whose `Drop` (after its `cleanup`) keeps the home under `AGENT_RUN_KEEP_HOMES=1`, or `AGENT_RUN_KEEP_FAILED=1` while the thread is panicking. No variable is new.
- `target/agent-run/<session>/` holds `bin/agents-mode` beside the session's `bin/claude[.exe]` copy: `boot --agents-mode`'s value (0600), a stale one removed before the stamp and the new one written after it, before `supervise.json`; `session.json` and `supervise.json` carry `agents_mode` (serde default).
**Why:** The registry lists each variable's readers and each harness-written file, so the new reader and file are named where they stand.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/

## 2026-10-17-cli-machine-contract — architecture: revive takes --json on both arms; revive --list is a cli producer
**Section:** §Established Decisions [CLI Conventions]; §Standard Contracts CLI `--json` output; §Conventions CLI exit codes (exit 1, `role_of`); §Occupied Resources subcommands row, Filesystem `diagnostics/` and `instances/` rows, Repository (`schemas/config.v1.json`); §Infrastructure Patterns → Project directory structure (`schemas/` row)
**Change:**
- `viola revive` takes `--json` on both arms (was: "takes no `--json`", owed to "CLI machine contract"). A refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (exit 1, stderr empty; `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; `client::refused_document`, codes only), the same log lines; `<name> --list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order, or the `no-session` document with exit 1. Without `--json` the pairs and rows are byte-identical.
- A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document; that document is owed to "Exit-cause code catalogue".
- `role_of` files `revive` followed (after any `--home` pair) by `--list` before a `--` under `cli`; every other `revive` stays beside `run`. `revive --list` joins the `cli` producers: under a parsed `VIOLA_NAME`, after its strict-modes check, `process-start` / `process-exit{subject:"self"}` (`exit_code` 0, or 1 `detail:"no-session"`) in the caller's `cli-<VIOLA_NAME>.ndjson`; no name, no log; a refused check on either arm opens none (was: `--list` opens no process log).
- `schemas/config.v1.json` registered: the tolerant `config.json` schema (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day` — the five `CONFIG_KEYS`), test-side.
**Why:** the chunk landed revive's machine form and its `cli` log. The post-preflight gap is the overseer's ledger call (`v1-39` un-claimed, its documents a CARRY on "Exit-cause code catalogue"); the `--list` log is the `cli` role's existing rule after the check (the overseer, technical).
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-machine-contract — architecture: the CLI verbs take the VIOLA_DIR step and canonicalise the home (not ratified)
**Section:** §Occupied Resources Environment variables (`VIOLA_DIR`); §Cross-cutting Patterns Config management (Viola home, the statusline source rule)
**Change:**
- `VIOLA_DIR` is read by `hook` and every CLI verb, `run` and `revive` included (was: "`hook` is its only reader: the CLI verbs do not read it yet"); the `mcp` verb is not built.
- `resolve_home(flag, name, dir, user_home)` (`src/cmd/mod.rs`) resolves `--home` as given, else the `instance_of` home when `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check, else `<user home>/.viola`, a failing pair falling through, never refusing; `resolved_home` reads the two variables and `std::env::home_dir()` once at every verb arm (was: the CLI verbs take `--home`, else the default; the step owed to "CLI machine contract").
- `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`; Windows `\\?\C:\…` written back as `C:\…`, other verbatim forms kept) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing; `run`'s start canonicalises after its wrapper log created the home, before the collision check, the pin and the bind.
- The statusline source's default-home test is `canonical_home(default_home(user_home))`; a home named by `--home` or reached through `VIOLA_DIR` reads no user settings; the `home_dir()` read is `resolved_home`'s.
**Why:** a boundary widening (a CLI verb's home is chosen by an environment variable) landed as built and NOT ratified: the Viola overseer's answers (P4 `inputs#I2`, this wrap's Q1 `inputs#I3`) are a delegate's, so the founder's own word is owed at his next batch.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — the colour reads NO_COLOR / TERM / COLORTERM; style.rs and the VT enable registered
**Section:** §Occupied Resources → Environment variables; §Established Decisions → [Naming]; §Conventions → Environment variables; §Cross-cutting Patterns → Config management; §Infrastructure Patterns → Crate dependency direction; §Infrastructure Patterns → Project directory structure
**Change:**
- Environment variables: a new group, read by every `viola` build's human output (`src/human/style.rs` `read`, `viola list` only), never set: `NO_COLOR` (present, an empty value included, switches colour off), `TERM` (`dumb` off; `256color` picks 256), `COLORTERM` (`truecolor` / `24bit` picks truecolor); compared only, never echoed; they decide only whether and how deep SGR bytes are written, configure nothing, reach no log line, error or refusal, disable no control and widen no redaction; `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read.
- [Naming] and Conventions: was "two ratified exceptions", the `fake-agent` test seams; now the seams plus the terminal's own colour names, read never set in every build, unprefixed.
- Config management: was "the only other variables any `viola` build reads are the two test seams"; now the seams and, in every build, the three colour reads by viola's own colour layer; clap stays without `color`, so its anstream → anstyle-query route stays closed and viola's reader is the only one.
- Crate dependency direction: the root bin's `Win32_System_Console` use gains `GetConsoleMode` / `SetConsoleMode` for the stdout VT enable (`0x0004` alone, never viola-pty's `0x000c`; only once every no-colour rung passed; the saved mode restored on drop; a failed get or set writes no colour); no dependency or feature added.
- Project directory structure: `src/human/style.rs` (the colour layer); the `board.rs` gloss names the width cut and the spans; `tests/support/sgr.rs` (the SGR reader).
**Why:** a boundary widening ratified by the founder's own answer «Читать их» to P4 fork Q1 (relayed verbatim by the operator); the answer card confirmed by a delegate, the Viola overseer — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch. Precedent: any further environment read, or a colour fact reaching a control, needs its own ratification.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — the MCP tools' contract as landed
**Section:** §Established Decisions [MCP], [Delivery Confirmation]; §Stack and Technologies (MCP server row); §Conventions Error handling schema (MCP bullet), CLI exit codes; §Occupied Resources Claude Code integration names, Binary, subcommands and exit codes
**Change:**
- [MCP]: was "the default `timeout_ms` for MCP `wait` is an open item"; now `wait` sends `timeout_ms` = `min(given, 50 000)` (`WAIT_CAP`, `wait_timeout`), 50 000 ms by default, `after` as given; the bound sits below claude 2.1.287's stdio tool-call default, which exceeds 50 s (measured live; the exact default unmeasured). `from` only when the server has a name. Each handler re-validates its raw arguments: a schema violation is `invalid-params`, an internal handler fault JSON-RPC `-32603` `internal error`, `data: null`.
- [Delivery Confirmation]: the MCP client's tool-call timeout exceeds 50 s on 2.1.287, above the 18.5 s send total.
- Conventions, MCP bullet: was "only faults on the MCP connection itself are JSON-RPC errors"; now `instance-unreachable` carries `detail: null`, `wrapper-fault` the fault's `{code,message,data}`, `invalid-params` a closed field word (`target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame; every `isError` text `refused: <code>`; JSON-RPC errors are the connection's faults, an unknown tool (`-32602`) and an internal handler fault (`-32603`); `tools/list` carries `ttlMs` 0 and `cacheScope` `private`, without which 2.1.287 (protocol `2026-07-28`) drops every tool.
- CLI exit codes: `role_of` excludes `mcp` from `cli`; `Role::Mcp` exits like `Other` and never prints `error: internal error`.
- Integration names: tools reach a session as `mcp__plugin_viola_viola__<tool>`; `--allowedTools mcp__plugin_viola_viola` allowed them with no dialog; they load deferred, through `ToolSearch`.
- Subcommands: `mcp` hidden from `--help`, no argument but `--home`, exit 1 printing nothing when its home or `diagnostics/` fails `check_path`.
- MCP server row: release graph rmcp exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); `viola-e2e` takes rmcp `client` alone, never `transport-child-process`.
**Why:** the chunk built `viola mcp`; the `ttlMs` / `cacheScope` requirement and the over-50 s client default were measured at its two live starts (the second on the founder's own raised cap).
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — MCP list builds the board and reads agents
**Section:** §Standard Contracts Session liveness, Channel methods (`list`); §Established Decisions [MCP] (its `list` clause); §Infrastructure Patterns → Project directory structure (`run/agents.rs`, `cmd/list.rs`, `cmd/mcp.rs`)
**Change:**
- Session liveness: was "`mcp`'s `list` is not built"; now the `mcp` process's `list` tool makes the same one `claude agents --json` read on each call — provider `list::board_document` (scan → read with the agents read and join → document) on a blocking thread, its `agents-probe` pair in `mcp.ndjson`, the board's refusal returned as `instance-unreachable`; the spawner list names it beside CLI `list`, `revive`'s preflight and verify's Run B.
- Channel methods `list`: MCP `list` builds the same board through `board_document` and returns the document's `ok`; CLI `list` keeps its own order (scan, its log opened, read, document).
- [MCP]: `list` builds the CLI's board and makes its one agents read.
- Directory structure: `run/agents.rs` also serves the `mcp` `list` tool; `cmd/list.rs` holds `board_document`; `cmd/mcp.rs` the hidden verb.
**Why:** a boundary widening (a new process spawning `claude`), ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents», live in the overseer's dialog, relayed verbatim by the overseer. The wrap's answer-card confirm that these sections lie inside it is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — the Tokio client and viola-channel's tree-form ban
**Section:** §Occupied Resources Workspace crates; §Established Decisions [Module Boundaries], [Concurrency / Backend Framework]; §Stack and Technologies (Wrapper IPC row); §Infrastructure Patterns → Build system, Crate dependency direction, CI/CD approach, Project directory structure
**Change:**
- Workspace crates: every member landed, `viola-mcp` included; the sync crates of `scripts/sync-crates.txt` are four (`viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`) — was five with `viola-channel`; `viola-channel` declares `tokio = ["dep:tokio", "interprocess/tokio"]` and `fake-agent = ["tokio"]`.
- [Module Boundaries]: was "only `viola-mcp` lists `tokio` … a feature only `viola-mcp` enables"; now only `viola-mcp` enables tokio (`rt`) among product crates, the channel lists it optional behind its feature, enabled by `viola-mcp` and by its own test-only `fake-agent`; `viola-e2e` `rt` + `process`.
- [Concurrency] and Wrapper IPC: the Tokio flavour sits in `viola-channel`'s feature-gated `TokioClient`, used by `viola-mcp` alone through the shared SQOS `client::open_handle`; `viola-channel` is off the sole-root list, its no-feature graph proven by `scripts/channel-tokio-free.sh`.
- Build system: was "covered through the sync roots that depend on it" (premise false: no listed crate depends on `viola-channel`); now the tree form (`cargo tree -p viola-channel -e normal --target all -i tokio`, absence read from the printed `tokio v` line, its `--features tokio` control), in CI supply-chain, proven by `deny-probes.sh`'s `channel-tokio-edge` probe and control (13/13); the rmcp release-graph check landed as `scripts/rmcp-features.sh` in the lint job; rmcp pinned `>=3.4.1, <3.5` (was `~3.4`); the rust plane indexes `viola-mcp`.
- Crate dependency direction: `viola-channel`'s `TokioClient` and its shared helpers; `viola-state` hosts `liveness::live_endpoint`; `viola-mcp`'s edges as landed (+ serde, serde_json, thiserror, tracing; tokio `rt`; `fake-agent = []`).
- CI/CD approach: job 3's set is four; supply-chain runs the tree form after the loop; lint runs `rmcp features` after `Lint probes` on every OS.
- Directory structure: the two scripts, `tokio_client.rs`, `liveness::live_endpoint`.
**Why:** the chunk's CARRY 1 premise was corrected by measurement. The test-only `fake-agent` enabler is written as built on the Viola overseer's answer at this wrap, a delegate's — PROVISIONAL; the release graph stays tokio-free outside `viola-mcp`.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — the plugin registers viola mcp, its log and its home
**Section:** §Established Decisions [Deployment / Distribution]; §Occupied Resources Filesystem (`diagnostics/`), Environment variables (`VIOLA_DIR`)
**Change:**
- [Deployment / Distribution]: was "until the `mcp` verb exists, `.mcp.json` ships `{"mcpServers": {}}`"; now `.mcp.json` registers `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}`, the pinned path substituted at every start; whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured.
- Filesystem: `mcp.ndjson` has a producer — once its home and `diagnostics/` pass `check_path` (a refusal opens no role file): `process-start{subject:"self"}`, the Tokio client's `channel-*` lines, one client-side `send-refused` per refused text, the `list` tool's `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines (`config-json` included), and `process-exit` on stdin EOF — none when Claude Code ends it by a signal (measured live on 2.1.287).
- Environment variables: was "the `mcp` verb is not built"; now `viola mcp` resolves its home through the same `resolved_home` and reads `VIOLA_NAME` for its own name.
**Why:** the chunk built the verb. Its home step through `resolved_home` (the `VIOLA_DIR` step and the canonicalisation) is written as built and is NOT ratified, on the Viola overseer's answer at this wrap, a delegate's; it joins the carried CLI-verbs item for the founder's own word.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — ServeError, a second open divergence
**Section:** §Established Decisions [Error Handling]; §Conventions Rust error types; §Inherited Defaults (Errors)
**Change:** was one open divergence (`viola-agent-claude`'s `Refusal`); now two: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`). The one-enum-per-crate rule stands; the fold or ratification of each is open.
**Why:** the chunk landed two enums in one crate. Recorded as an open divergence, not an exception, on the Viola overseer's answer at this wrap (a delegate's, PROVISIONAL), with no route work.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-optional-plugin-install — plugin install as built; the precedence and the exit-1 start not measured
**Section:** §Established Decisions [Plugin Scope], [CLI Version Compatibility], [Deployment / Distribution]
**Change:**
- [Plugin Scope]: was "which `viola` MCP server a wrapped session then uses, is a capability-ledger row"; now it is no ledger row (no `send` relies on it, the ruled limit) and is not measured, owed to the route entry "Compact post-condition ledger row", its Windows leg to "Windows-only live measurements". As built, `viola plugin install` writes only `<home>/marketplace/` and spawns no `claude`; its `.mcp.json` registers `viola` as `{"command": <the canonical current_exe, forward slashes>, "args": ["--home", <the canonical home, forward slashes>, "mcp"]}`, each document built by `serde_json` (`viola_agent_claude::marketplace::marketplace_files`).
- [CLI Version Compatibility]: the ledger-row bullet "Plugin precedence" is removed; was "plugin precedence stays owed"; now "plugin precedence is no row ([Plugin Scope])".
- [Deployment / Distribution]: was "unmeasured (the founder's live-start cap)"; now unmeasured, owed to "Compact post-condition ledger row".
**Why:** no 24th `LedgerRow` and one live round are the founder's own answer to P4 fork Q2, relayed verbatim by the Viola overseer. The round took none of its readings: inside the work tree the local scope landed at its root, and with `proj/` its own git repo both wrapped starts were held by the folder-trust dialog, start 3 unspent. The owner placement of the unread readings is the Viola overseer's answer at this wrap (`inputs#I8`), a delegate's — PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-17-optional-plugin-install — plugin install's exits, document, names and files
**Section:** §Conventions (CLI exit codes 1, 21); §Standard Contracts CLI `--json` output; §Occupied Resources (Binary, subcommands and exit codes; Claude Code integration names; Filesystem `marketplace/`, `diagnostics/`); §Infrastructure Patterns → Project directory structure
**Change:**
- Subcommands: `plugin install` takes `[--json]` and nothing else (`viola plugin` alone a usage error; clap's own `viola plugin help` served); exits 0 / 21 (its strict-modes check refusing the home or an existing `marketplace/` tree dir, nothing created, no log opened) / 1 (the `cli` catch site) / 2.
- Exit 21 gains its refusal (the same document; human mode the home-state pair); exit 1's catch-site verbs gain `plugin install`.
- `--json`: `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (exit 0, no path); its refusal the `instance-unreachable` document through `client::refused_document`.
- Integration names: the local marketplace `viola` (`marketplace::NAME`), one plugin `viola` (source `./viola`), added as `viola@viola` (`PLUGIN_REF`), its `.mcp.json` registering server `viola`, no hooks.
- Filesystem: was "`marketplace/`: written only by `viola plugin install`"; now with its dirs `.claude-plugin/`, `viola/`, `viola/.claude-plugin/` (0700, `create_private_dir`) and three files (0600, `replace_private`, rewritten whole), after its strict-modes check, any other entry left as found, never read back. `diagnostics/`: `plugin install` writes the self pair into the caller's `cli-<VIOLA_NAME>.ndjson` only under a parsed name and after its check passed.
- Directory structure: `cmd/plugin.rs`; `human/verbs.rs`'s install lines (`write_plugin_installed`, `command_word`, a Windows-gated source in `windows-mutants.yml`'s `viola-main` job); `human.rs`'s callers gain `plugin install`; `viola-agent-claude`'s pure `marketplace` module.
**Why:** the chunk built the verb; its argument and exit shape follow `viola list`'s strict-check-before-log order.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-18-compact-post-condition-ledger-row — /compact confirmed by its same-session SessionStart, its own 40 s window
**Section:** §Established Decisions → [Delivery Confirmation]
**Change:** `/compact` now has a measured post-condition: a SessionStart with source `compact` and the session's own `session_id` (`PostCondition::CompactedSession`; `LOCAL_COMMANDS`' `/compact` entry carries it; row `local-command-compact`). On a verified version a `/compact` send waits its own window, `COMPACT_CONFIRM_WINDOW` = 40 s (PROVISIONAL, validated per version by the row against `compact_latency_ms`), beside the unchanged 10 s `CONFIRM_WINDOW_FALLBACK`; `SendSlot::window()` is the one bound `confirm`'s deadline and the `run.confirm_window` span's `window_ms` read. The tap claims it on a `session-start` of cause `compact` whose string `agent_session_id` EQUALS the id remembered before that line; another cause, another id, no string id or nothing remembered claims nothing, and it expires at 40 s as `not-delivered` / `no-prompt-submitted` with its cursor. The blocking total gains 48.5 s plus the paste for a compact send, under the MCP tool-call ceiling read at 50 012 ms. Was: `/compact` "with none", unconfirmable at once on every version (`/remote-control` and `/compact` today), its row owed to the route entry "Compact post-condition ledger row"; now only `/remote-control` has none, and `/compact` is unconfirmable only on an unverified CLI. The proof sentence names the recorded `compact-1` and Run B's live reading (no live driver `send` of it yet).
**Why:** the chunk built the post-condition, its claim arm and its window (report Symbols / APIs); CARRY 1 of chunk 2026-10-14-resume-and-compact-ledger-rows discharged.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-compact-post-condition-ledger-row — the twenty-fourth ledger row, local-command-compact, and Run B's sixth paste
**Section:** §Established Decisions → [CLI Version Compatibility] (the ledger, the relied set, `check`, Run B's pastes, `box_wait`, the step lines, `--record`, the landed rows, the older-stamp list, the next row); §Cross-cutting Patterns (Capability ledger); §Standard Contracts (Ledger stamps envelope); §Occupied Resources (fixtures, `ledger/stamps.json`, the fake agent); §Infrastructure Patterns → CI/CD approach, → Crate dependency direction
**Change:** the ledger holds twenty-four rows, `local-command-compact` (`/compact keeps the session id and submits no prompt`) the 24th and last, relied on every OS: relied 24 on Unix, 21 on Windows (was 23 / 20); step lines `[NN/24]` / `[NN/21]`, the last `local-command-compact`; its check (`local_command_compacted`, `compact_start`) as built; Run B gets six compiled pastes, `PROBE_COMPACT_COMMAND` (`/compact`) after the newline paste and before `/clear`, only into the settled input box, its answer waited up to `PROBE_DEADLINE`, then `box_wait` before `/clear`; `FRAMING_TURNS` `paste-1` · `paste-2` · `paste-3` · `compact-1` · `clear-1`; `--record` writes `SessionStart.compact-<n>.json` (stems `paste`, `compact`, `clear`); the 2.1.287 set holds 34 files (was 33) with six framing variants, stamped `24 pass  0 fail`; `typed_probe` gains `compact_latency_ms` (additive, no `v` bump); the fake agent's `--framing` replays `compact-1` under the last fired SessionStart's id; a stamp of the twenty-three pre-chunk ids reads unverified on every OS, and every home stamped before this chunk reads `cli_verified:false` until its next verify; the next row would be the 25th. Was: "`/compact` … needs no row under the ruled limit"; now it is stamped live on 2.1.287 (Linux dev host, `compact_latency_ms` 21413), its Windows reading owed on "Windows-only live measurements".
**Why:** the chunk built the row and its probe (report Symbols / APIs, Counts); the sixth Run B paste is the founder's own answer to P4 fork 1 («Шестая вставка в Run B», `inputs#I3`, relayed verbatim by the operator); this wrap's answer-card confirm of the proposals inside it is a delegate's (delegate the Viola overseer), PROVISIONAL until his own word in the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-compact-post-condition-ledger-row — CARRY 2's readings to their own entry; Run B's three transcript files
**Section:** §Established Decisions → [Deployment / Distribution]; → [Plugin Scope]
**Change:** whether a registered `viola mcp` that exits 1 blocks the CLI's start, and which `viola` MCP server a session holding both plugins starts and uses, are owed to the route entry "Plugin precedence live readings" (was "Compact post-condition ledger row"). Run B leaves three transcript files per real-CLI verify — its session, the `<id>-backup-<n>.jsonl` the CLI writes at its pasted `/compact`'s compaction, and the session its `/clear` opens (inferred) — and C and D one each, E none, as measured on 2.1.287 at this chunk (60 → 65). Was: "Run B two per real-CLI verify".
**Why:** the founder's own answer to P4 fork 3 («Отдельный новый вход», `inputs#I3`) moved CARRY 2 to a new entry, written by this wrap (the P5 card row 1, the operator's own word `inputs#I4`); the transcript count was measured at the live round; the residual is inside the sixth paste: the sixth Run B paste is the founder's own answer to P4 fork 1 («Шестая вставка в Run B», `inputs#I3`, relayed verbatim by the operator); this wrap's answer-card confirm of the proposals inside it is a delegate's (delegate the Viola overseer), PROVISIONAL until his own word in the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-plugin-precedence-live-readings — the two plugin readings measured: precedence and the exit-1 server
**Section:** §Established Decisions → [Plugin Scope]; §Established Decisions → [Deployment / Distribution]
**Change:** [Plugin Scope]: was "which `viola` MCP server a wrapped session then starts and uses … is not measured: owed to the route entry"; now a wrapped session that also has the installed plugin enabled starts one `viola` MCP server, the `--plugin-dir` plugin's, which answers its tools; the installed plugin's `viola mcp` never starts there; the session runs one SessionStart hook process; still no capability-ledger row (the ruled limit); its Windows leg owed to "Windows-only live measurements". [Deployment / Distribution]: was "Whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured"; now an exit-1 `viola mcp` (its home refused by `home_passes`) does not block the CLI's start: the session reaches `idle` and the CLI logs that server's connection as failed; Windows leg owed. Both as measured on claude 2.1.287 on the Linux dev host at this chunk's `evidence/live-round.md`.
**Why:** the route entry "Plugin precedence live readings" owned both owed readings and took them live inside the founder's cap of 3 starts (his own answer F2); the owner title leaves the masters with them.
**Ref:** .andromeda/runs/2026-10-19T07-00-05-wrap/

## 2026-10-18-plugin-precedence-live-readings — the VIOLA_DIR home step ratified
**Section:** §Occupied Resources (`VIOLA_DIR`); §Cross-cutting Patterns → Config management (Viola home)
**Change:** was "The CLI verbs' reading of it, and `mcp`'s through `resolved_home`, are NOT ratified" and "The `VIOLA_DIR` step for the CLI verbs and the canonicalisation are NOT ratified: the founder's own word is owed"; now both sentences are gone — the CLI verbs' `VIOLA_DIR` step, the canonicalisation and `mcp`'s use of `resolved_home` stand as built, ratified.
**Why:** the founder ratified the whole home step, his own word «Ратифицировать» relayed verbatim by the operator, answering a question that described every CLI verb (`run`, `revive` and `mcp` included) choosing its home by `VIOLA_DIR` when `VIOLA_NAME` + `VIOLA_DIR` pass the shape check, else the default, then canonicalised (the operator's correction at the P5 review); confirmed in this wrap's halt by the operator. It supersedes the delegate answers the marks were written on.
**Ref:** .andromeda/runs/2026-10-19T07-00-05-wrap/

## 2026-10-19-dialog-hold-for-the-driver — the configurable dialog hold and the dialog-expired record
**Section:** §Established Decisions [Hook Contract] (with two detail groups below it), [Human Takeover / Wheel], [Message Broker / IPC], [Deployment / Distribution], [Validation]; §Stack and Technologies (Domain newtypes); §Conventions (Normalised event kinds); §Standard Contracts (`pending_dialog`, `wait`, Event `data` per kind, wrapper-appended records); §Occupied Resources (`config.json`, `schemas/config.v1.json`, Registered hook events); §Cross-cutting Patterns (Config management, Settings); the subject line (new: `dec-hook-contract`).
**Change:**
- The hold is `config.json`'s `dialog_hold_s`, whole seconds from 60 to 3600, default 60 (`DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL), a `DialogHold` read once at start. Was `viola_core::DIALOG_DEADLINE` = 60 s, retired.
- The hook's reply bound is `DIALOG_HOLD_MAX + 5 s` (3605 s), the ceiling (was `DIALOG_DEADLINE + 5 s`). The dialog hooks' `hooks.json` `timeout` is 3615 (was 75), so the rendered plugin stays byte-identical across starts.
- At the hold's end the reply stays `null`, and the wrapper appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`) that wakes `wait`. It is the one wrapper-appended kind that wakes; "only hook lines signal" now excepts it. A hand-back or an answer appends none.
- The normalised kinds gain `dialog-expired`. The wake set is six: `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired`.
- `pending_dialog` is written before the dialog's event line and written back to none on a failed append. It was written after the line.
- `DialogHold` joins `ViolaName` and `Percent` as a nutype.
- "dialog deadline" reads "dialog hold" in [Human Takeover / Wheel].
- The live reading of a 671 s hold on claude 2.1.287, Linux, `as measured at` the chunk's `evidence/live-round.md`. A pending group owes the near-ceiling reading to "Overseer switch to viola" and the Windows leg to "Windows-only live measurements".
**Why:** The founder answered the P4 forks himself, relayed verbatim by the operator (`inputs#I2`). Q1 ratified the key as a boundary widening, Q2 kept `null` at expiry, and Q3 capped the live round. The wrap's answer-card confirm of these proposals was the Viola overseer's (delegate viola-overseer), PROVISIONAL, superseded by his own word at the Epoch 5 boundary. The order fix answers a macOS CI red whose cause predates the chunk, on the operator's word (`inputs#I4`). The hook's bound and the timeout follow the ceiling, not each start's hold, because the wrapper replies at its own hold's end.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/

## 2026-10-19-overseer-switch-to-viola — `question` gains `option_details`, `turn-ended` gains `background_tasks`
**Section:** §Standard Contracts (Event `data` per kind: `turn-ended`, `question`; the `turn-ended` `hook.event` example frame).
**Change:**
- `turn-ended` is `{last_assistant_message, background_tasks}`. `background_tasks` is the Stop payload's `background_tasks` array length, a count only (no element read or carried), or `null` when the key is absent, `null` or not an array. Was `{last_assistant_message}`.
- `question` is `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`. `option_details` is parallel to `options`, each field the option's string or `null`, never a refusal; the option text is upstream content carried into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line. Was `{dialog_id, questions:[{question, options:[<label>], multi_select}]}` (the row also omitted the `header` the data already carried).
- The `turn-ended` example frame's `data` carries `"background_tasks":null`.
**Why:** The chunk built both gaps research found for the overseer's driving. The option-text crossing is a boundary widening the founder answered himself at P4 fork Q4 («Строить оба», `inputs#I14`, relayed by the overseer), the count-only form of `background_tasks` by the same answer; the wrap's answer-card confirm is his own word, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-overseer-switch-to-viola — the 3600 s ceiling's first live reading moves to "Version done-check"
**Section:** §Established Decisions (the dialog-hold `pending` group, `dec-hook-contract`).
**Change:** A hold near the 3600 s ceiling is first read live at "Version done-check"; the long hold's Windows leg stays at "Windows-only live measurements"; the ceiling stays PROVISIONAL. Was "first read live at "Overseer switch to viola"".
**Why:** No hold near the ceiling arose naturally in the switch (the one hold measured was 27 345 ms), and a contrived long hold was ruled out by its plan. The re-pin is the P5 review card's row 2, approved by the operator's word (`inputs#I15`), not the founder's.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-overseer-switch-to-viola — verify's print-mode probe leaves an empty CLI project dir
**Section:** §Established Decisions ([Plugin Scope] rationale, `viola verify`'s transient unwrapped children).
**Change:** The print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript), the same accepted residual as Runs B–D's transcripts, as measured on 2.1.296 and 2.1.302 on the Linux dev host. Was unstated (the rationale named Runs B–E's residual only).
**Why:** The omission was measured at the switch's verify residual count (4 new `~/.claude/projects/` entries per verify, one of them this dir). The founder accepted it into the same class by his own word, live, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-newer-claude-cli-rows — the second input-box literal and the three-screen input-box-signature row
**Section:** Established Decisions ([Screen Model]; [CLI Version Compatibility]).
**Change:**
- [Screen Model] `SIGNATURES.input_box` is `["for agents", "(shift+tab to cycle)"]`. It was `["for agents"]`. The second literal is the resting footer a CLI keeps once it hides the first. On 2.1.302 the footer hint `for agents` is gone 15 s after the start, and the row reads `⏵⏵ auto mode on (shift+tab to cycle)`. Its mode word is the user's permission mode, so the literal is the mode-independent `(shift+tab to cycle)`, which no other CLI text read holds. The readiness rule (a row with an input-box literal and no modal row) is unchanged.
- [Screen Model] The `input-box-signature` row reads the ready screen, the turn screen and the screen Run B settles on after the long paste's turn, each holding an input-box literal and no modal; a long turn whose screen never settles fails it. It was "the ready and turn screens". Its words are unchanged.
- [CLI Version Compatibility] Run B's added pastes go only into rows that hold *an* input-box literal (was "the input-box literal"). The re-stamp clause names the stamped versions 2.1.287 and 2.1.302, each `24 pass  0 fail` on the Linux dev host. It was "2.1.287 alone today".
**Why:** 2.1.302 hides `for agents` once its attacher-caps timer expires, so a two-screen row passed while the literal vanished later, and `run`'s pre-send gate would have refused `send` `input-not-ready` on a verified 2.1.302. The literal was measured live by one scratch footer reading. That crossing is the founder's own answer to Fork 2, relayed by the Viola overseer (security-plan's entry). 2.1.302's full stamp was then recorded live (24/0).
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-19-newer-claude-cli-rows — the 2.1.302 fixture set and the long-turn screen switch
**Section:** Occupied Resources (Binary, subcommands and exit codes — the test-only binaries; Repository — `fixtures/claude/<cli-version>/`).
**Change:**
- The fake agent has thirteen argv options (the count's rule is the list), nine of them serving verify's interactive runs and their tests. It was twelve and eight. The new one is the test-only `--long-turn-screen <phase>`: with `--framing` and `--turn-stop`, after the long paste's turn alone and after its Stop, `Screen.<phase>` of the set is drawn in place of `Screen.turn`. It sits beside `--tag-turn-screen`.
- The committed sets are `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`; `2.1.302` is new. `2.1.302` holds 4 spine payloads + 3 screens + 12 dialog variants + 6 framing variants + 2 status line variants + 1 resume variant + 1 agents variant: 29 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`. It was recorded whole by one `viola verify --record` round. Its `Screen.ready` and `Screen.turn` keep one row holding both input-box literals. It is stamped at the relied set the same way as `2.1.287`; `2.1.288` and `2.1.283` stay drift-only.
**Why:** The fixtures for the measured literal: CI's fake agent replays a recorded 2.1.302 set, and `contract_ledger_probes` stamps it (its `STAMPED` list). The screen switch reproduces the stop that held Run B after the long paste's turn without a live start.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch: the delegate marks his, the bodies cleared
**Section:** §Established Decisions ([Hook Contract]; verify's Run B and Run E under the CLI version gate; [Error Handling]; [Module Boundaries]) · §Conventions (exit code 1) · §Occupied Resources (the `instances/<ViolaName>/events.ndjson` bullet)
**Change:**
- The statusline capture arm, Run B's probe `--settings`, its agents read, its newline and `/compact` pastes and Run E stand as the founder's own answers; was "the wrap's card confirm … is provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary" on five clauses and "the wrap's card confirm delegate viola-overseer, provisional" on one; now the clauses are gone.
- `ServeError` stays the second open divergence, its fold or ratification open; was "recorded on the Viola overseer's answer at its wrap, a delegate's"; now the parenthetical names the chunk alone.
- `viola-channel`'s test-only `fake-agent = ["tokio"]` stands as built; was "written as built on the Viola overseer's answer at its wrap, a delegate's"; now the chunk alone.
- Exit 1: `viola revive`'s strict-modes refusal writing no line, the interrupted verify's `unable:` / `hint:` pair, and `viola list`'s exit 1 for an `instances/` entry whose type cannot be read and for a failed log start, with no row on a per-instance read error, stand as built; was "the operator's answer at its planning, the founder not having ruled", "a planning form on the operator's word, the founder's wording owed" and "implement's decisions, accepted by the operator … not ruled by him"; now each parenthetical names its chunk alone.
- `revive`'s start arm writing no line after a refused check: was "the operator's answer at that chunk's planning, and the founder has not ruled on it"; now the sentence is gone.
**Why:** the founder's own answers to his Epoch 5 batch, 2026-10-20 (ratify every delegate mark of the epoch; accept the refused checks' no-line shapes as built), relayed by the Viola overseer through the operator's directive, and his own word in the same dialog widening it to every Epoch 5 delegate mark, the unreadable-type entry and the interrupted pair included, the axum ban and the GUI scan forms excepted. Each provisional record stands in its own entry, superseded 2026-10-20. The bodies take writing rule 7's form, not the directive's (a `superseded` clause in the body): the overseer's technical answer at this wrap.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-t06-58-31-wrap — the Epoch 5 entries' provisional records, superseded 2026-10-20
**Section:** none in the body — this file's history: the entries named below
**Change:** each named entry's provisional mark (a delegate's card confirm, a delegate's technical or halt answer, a wording or a shape owed to the founder) is superseded 2026-10-20 by his own word; every other claim of those entries stands, and none is pruned:
- 2026-10-13-interrupted-verify-cleanup — verify's Ctrl-C / TERM stop, the owed literals moved, the Windows matrix line (the pair's wording, his)
- 2026-10-13-paste-newline-ledger-row — eighteen ledger rows, `paste-newline`, Run B's fifth paste, the owed rows moved to "Owed ledger rows"
- 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm and the moved rows
- 2026-10-14-resume-and-compact-ledger-rows — the resume row, Run E and /compact listed
- 2026-10-14-resume-and-compact-ledger-rows — Run E's resources and a killed PTY child's exit status (the exit-status wording and the card, his)
- 2026-10-16T14-58-10-wrap — 0.1.0 is a pure CLI; the frontend moves to 0.2.0 (the masters kept true to the tree, his)
- 2026-10-16-unwrapped-sessions-on-the-board — CLI Version Compatibility: the agents-join row and Run B's agents read
- 2026-10-16-unwrapped-sessions-on-the-board — Session Liveness: revive refuses a session id live elsewhere
- 2026-10-17-cli-output-discipline — the colour reads NO_COLOR / TERM / COLORTERM; style.rs and the VT enable registered
- 2026-10-17-mcp-server-for-drivers — MCP list builds the board and reads agents
- 2026-10-17-mcp-server-for-drivers — the Tokio client and viola-channel's tree-form ban
- 2026-10-17-mcp-server-for-drivers — ServeError, a second open divergence
- 2026-10-17-optional-plugin-install — plugin install as built; the precedence and the exit-1 start not measured (the unread readings' owner placement, his)
- 2026-10-18-compact-post-condition-ledger-row — the twenty-fourth ledger row, local-command-compact, and Run B's sixth paste
- 2026-10-19-dialog-hold-for-the-driver — the configurable dialog hold and the dialog-expired record
**Why:** the founder's own word on his Epoch 5 batch, 2026-10-20, relayed by the Viola overseer: every Epoch 5 delegate mark is his. The axum ban of 2026-10-16-remove-the-browser-leg stays a separate founder item; the `VIOLA_DIR` marks were lifted at 2026-10-18-plugin-precedence-live-readings.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-epoch-5-cleanup — the keepers' headroom floor and the census's second form
**Section:** §Occupied Resources (Filesystem: `/tmp/viola-e2e-home-<uid>/`; Repository: `target/e2e-home`, `target/profraw-census/`) · §Infrastructure Patterns → `project-directory-structure` (the `scripts/profraw-census.sh` comment)
**Change:**
- The two keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`) refuse a linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`). The message is fixed: `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (two counts, no path). There is one compiled constant per keeper, equal in both; no variable, flag or config key sets it. Its basis is the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. Was: three checks (absolute target, made 0700 when gone, a real owner-only directory). Now: four.
- `target/profraw-census/`: the script's second form, `--case <binary stem> <test name> <runs> <workers> [<name>]`, runs one integration case of the same instrumented build. Each profile is named with its binary signature (`r<i>-%p-%<nproc>m.profraw`) and offered alone to `llvm-profdata merge`; a refused one is kept under `refused/`. Its last line is `profraw-census: runs N · passed N · profiles P · refused R`; exit 0 only when every run passed and R is 0, 1 otherwise, 2 on usage, with no instrumented binary, or with no `llvm-profdata`. Was: the first form only (the start test). The tree comment names both forms.
**Why:** the stamped-home gates struck EDQUOT on the dev host's quota'd tmpfs, so a home is now refused before the quota is reached; the quota itself is unreadable from the session's shell. The census's second form is the witness that named the truncated profile's killer, a hangup reaching the fake agent's hooks.
**Kept:** a plain-directory base (every CI runner) reads no headroom. The boot refusal stays the closed `build-failed`, and `run --local-live`'s stays `verify-exit-none`; no reason was added.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the recorded live MCP exchange: the tap, the `Mcp.` class and its schema
**Section:** §Occupied Resources (Binary, subcommands and exit codes: the test-only binaries; Workspace crates: `viola-e2e`; Repository: the `fixtures/claude/<cli-version>/` bullet and its `2.1.302` set, `schemas/claude-mcp-fixture.v1.json`) · §Infrastructure Patterns → `project-directory-structure` (the viola-e2e row, `schemas/`, `fixtures/claude/<cli-version>/`)
**Change:**
- The test-only binaries gain `viola-mcp-tap`, a `[[bin]]` of `viola-e2e`: `record <capture> <viola exe> <args…>` (a direct spawn, no shell and no PATH lookup; each line through `take(MAX_FRAME)` appended once, tagged with its direction, to a capture created 0600; it exits with the child) and `fixture <capture> <out>` (the client frames up to and including the first `tools/list` request and each request's result keyed by method, through `ledger::scrub` and then `ledger::screen_fault`, refusing with the file name and one closed code `home-path` · `absolute-path` · `username` · `email`, never the content). Was: `viola-e2e`'s one bin `viola-harness`.
- The `Mcp.<variant>.json` class is recorded by the tap, not by `--record`, validated by `schemas/claude-mcp-fixture.v1.json` (`v` const 1; `client` JSON-RPC 2.0 frames that contain a `tools/list` request with an `id`; `results` keyed by method, `tools/list` required with a `tools` array of named tools, any other key an object), hygiene-walked apart from the hook payloads (`mcp_fixtures_pass_hygiene`) and replayed by `tests/mcp_recorded_exchange.rs`.
- The `2.1.302` set: was 29 files "recorded whole by one `viola verify --record` round"; now 30 files, its `Mcp.tools-list-1.json` the tap's (the count's rule `ls fixtures/claude/2.1.302 | wc -l`).
**Why:** item (7) records what the live CLI's MCP client sends, so a replay can hold `viola mcp`'s served result to it. The CLI's MCP client crossing into the test-only tap, its scrubbed frames committed, is a boundary widening the founder answered himself at its P4 fork and again when the cap rose to two starts, each relayed verbatim; at this wrap's answer card he confirmed, relayed verbatim, that these sections lie wholly inside his answers.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the split modules, `relied_here` and the `viola-mcp` serde edge retired
**Section:** §Established Decisions (the capability-ledger / `viola verify` decision: its interactive runs, `agents_read`, `trusted_args`) · §Infrastructure Patterns → `project-directory-structure` (`human/board.rs`, `verify/typed.rs`, the viola-agent-claude row) · §Infrastructure Patterns → `crate-dependency-direction` (`viola-agent-claude`'s ledger list, `viola-mcp`, the shared pins)
**Change:**
- `src/cmd/verify/typed.rs` is the module root (`RunDirs`, `Inputs`, `RunStep`, `measure`, `POLL`) over `src/cmd/verify/typed/`: `runs.rs` (Runs A–E, `trusted_args`, `TrustedDir`), `turns.rs` (`poll_until`, `wait_for`, `turn`, `agents_read`, `framing_turns`, `probe_until`, `wait_capture`), `pty.rs` (`KeyPipe`, `SharedPty`, `Feed`, `Sink`, `settled`, `box_wait`), `run.rs` (`Run`, `ctrl_c_ending`, `ended_by`) and the `#[cfg(test)]` `test_support.rs`; the decision cites `typed::turns::agents_read` and `typed::runs::trusted_args`. Was: one file, `typed::agents_read`, `typed::trusted_args`.
- `src/human/board.rs` is the module root re-exporting `BayBudget`, `home_state_pair` / `instance_state_pair` and `write_board` over `src/human/board/`: `pairs.rs`, `bay.rs`, `cells.rs`, `rows.rs` and the `#[cfg(test)]` `test_support.rs`. Was: one file.
- The viola-agent-claude row names `src/ledger/test_cases.rs`, the ledger's unit cases, declared `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod tests`.
- The ledger's per-OS relied set is `relied()`, filtering on `relied_on(WRAPPED_HERE)`. Was: `relied()`, `relied_here`.
- `viola-mcp` takes no serde (was `serde` among its edges); the shared pins are taken by the crates that take them, `viola-pty` none and `viola-mcp` no serde. Was: "every workspace crate except `viola-pty`".
**Why:** the size split moved code with every caller's path kept; `relied_here` had one caller and was folded into `relied()`; the serde edge was unused (`cargo machete crates/viola-mcp` clean).
**Kept:** viola-e2e's new `viola-agent-claude` path edge takes no row: the key states viola-e2e's dependencies are not listed.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the measured 2.1.302 MCP opening and its recorded replay
**Section:** §Standard Contracts (the MCP result shape: the `tools/list` result)
**Change:** claude 2.1.302 on protocol `2026-07-28` opens with `server/discover` (its JSON-RPC id a string) and then `tools/list`, each carrying `_meta` (`io.modelcontextprotocol/protocolVersion`, `…/clientInfo`, `…/clientCapabilities`), and sends no `initialize` or `notifications/initialized`; viola's two results each carry `ttlMs` 0 and `cacheScope` `private`, as measured live (start 2). The recorded exchange `Mcp.tools-list-1.json` is replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs`, which takes the `tools/list` response by its request's id and holds the served result to the recorded result's key and type shape, its tool names in order. Was: only the 2.1.287 reading (the two keys required).
**Why:** the plan assumed an `initialize`-flavoured handshake; start 2's capture disproved it for 2.1.302, so the body states the measured opening.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the test-only child marker and the orphans probe's two cases
**Section:** §Occupied Resources (Environment variables) · §Infrastructure Patterns → `build-system` (the orphans gate's `--probe`)
**Change:**
- `MCP_SERVE_TEST_CHILD`: a test-only marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn, read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`; read by no `viola` build, outside the `VIOLA_*` namespace. Was: unregistered.
- `scripts/orphans-check.sh --probe` fires on each of its two planted orphans. Was: "fires on a planted orphan".
**Why:** the child entry kills `serve`'s survivor inside its own crate; `VIOLA_*` is product plumbing, so the marker carries no prefix. The probe has held two planted cases since before this chunk; its log reads `2/2 fired`.
**Kept:** `PTY_SIDELOAD_TEST_CASE`, the precedent the marker follows, stays unregistered here: it is not this chunk's.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-25-u55-door — architecture: split into its body and 6 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 6 category files; the map .andromeda/architecture.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-25 for the door on viola's masters (F2), given after three review rounds, the scoped third pass ACCEPT with 0 DISPUTE, at 1.46 M (1.00× its cost guide); relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/a2a855dc-9f94-4016-87d5-2f52c0246ab5/scratchpad/architecture.card.md

## 2026-10-25-windows-endpoint-admission — the root bin's Windows test dev-dependency on windows-sys
**Section:** the master's subjects record (architecture.md); §Infrastructure Patterns → Crate dependency direction (crate-dependency-direction.md: a new `values` detail block after the root bin bullet)
**Change:**
- The master's subjects record now ends `crosscut-capability-ledger · infra-crate-dependency-direction` (was ending at `crosscut-capability-ledger`).
- Crate dependency direction gains a `values · infra-crate-dependency-direction` detail block: the `viola` root bin's test-only Windows dev-dependency is windows-sys (the workspace pin `=0.61.2`) under `[target.'cfg(windows)'.dev-dependencies]`, with both features `Wdk_Storage_FileSystem` and `Win32_System_IO` (`NtQueryInformationFile` and its `IO_STATUS_BLOCK` argument need both), for the pipe-type read-back of `tests/security_negatives_channel.rs`. Both features already reach the Windows graph by unification (mio, interprocess, socket2), so `Cargo.lock` does not move, and under `resolver = "3"` no release build carries them. Before this block, the key file named windows-sys for the root bin only as a normal Windows-only dependency, and no dev-dependency features.
**Why:** the chunk landed a new root dev-dependency table with two windows-sys features where the plan's step 5 named one. Measured: windows-sys 0.61.2 gates `NtQueryInformationFile` on `Win32_System_IO` as well as on its module's `Wdk_Storage_FileSystem`. With no dependency added or bumped, the lock unchanged and `cargo deny check bans licenses sources` green. Naming both features is the operator's wrap directive (the overseer, founder-delegated). Trap for later chunks: a windows-sys feature can compile through unification from another crate and still be undeclared, so a dev-dependency declares every feature its test's types need.
**Kept:** the root bin bullet's text is unchanged. The stack's windows-sys pin and the `viola-test-chan-<pid>-<label>` test-pipe namespace under §Occupied Resources also stand. The dev-dependency's features are recorded once, here.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the pinned Tokio version moved to 1.53.3
**Section:** §Stack and Technologies (stack-and-technologies.md: the Backend framework (concurrency model) row) · §Inherited Defaults (architecture.md: the Framework line)
**Change:**
- The Backend framework row's Choice cell reads "Tokio 1.53.3 built only inside `mcp`" (was 1.53.1); the rest of the row unchanged.
- The Inherited Defaults Framework line reads "Tokio 1.53.3 only in `mcp`" (was 1.53.1); Rust 1.98.1, MSRV 1.96, edition 2024, rmcp 3.4.1 and clap 4.6.7 unchanged.
**Why:** the `[workspace.dependencies]` exact pin moved `tokio` `=1.53.1` → `=1.53.3` (`default-features = false` kept) and the lock resolves 1.53.3, a version RUSTSEC-2026-0386 lists as patched, so `cargo deny check` exits 0 without it and with no ignore widened. The containment both sites state is unmoved: Tokio stays built only in `viola-mcp`, `viola-channel`'s optional `tokio` feature and the test-only `viola-e2e`, with the four sole-root `deny-sync.toml` bans exiting 0 and `channel-tokio-free: ok`.
**Kept:** the stack table carries the version only, at category grain. It gains no Tokio advisory-floor row, because that advisory's record belongs to security-plan's Dependency Security group. It gains no row for the four `AsyncFd` constructors Tokio 1.53.3 deprecates either: viola calls none of them, and clippy `-D warnings` is the gate that holds that fact.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-server-verification-before-any-frame — exit 21 and the MCP unreachable result for a server that is not the recorded wrapper
**Section:** §Conventions (conventions.md: CLI exit codes; MCP results)
**Change:**
- CLI exit codes: exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper (the connected peer's pid or OS start time is not the snapshot's, or the check cannot be made): no frame is written; under `--json` the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, else the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording owned by design-system, surface cli-send). The client's own refusals (exit 13) come first; the cause has no exit code, `--json` code or `RefusalReason` of its own.
- MCP results: a frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper, or whose check cannot be made, returns the `instance-unreachable` `isError` result, text `refused: instance-unreachable`, with no frame written and no log line; the start time is read on `tokio::task::spawn_blocking`.
**Why:** the chunk built the check on the six CLI verbs (each through `cmd::client::connect`) and on the MCP `frame` (through `server_verified`); its Outcome reads both against an impostor server on Linux and Windows, and no config key, `--json` code, exit code, `RefusalReason` or MCP error word was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — both clients read the serving process's pid; the macOS recorded gap
**Section:** §Established Decisions [Message Broker / IPC] (established-decisions.md) · §Stack and Technologies (stack-and-technologies.md: Wrapper IPC, Process liveness)
**Change:**
- [Message Broker / IPC]: `viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the client end the SQOS Identification open adopts, none on macOS, where `SERVER_PID_REPORTED` is false); they write and log nothing. Every frame-writing client compares that pid and the process's OS start time with the snapshot's before its first frame, so on Linux and Windows a squatted or rogue server is never sent a frame.
- [Message Broker / IPC]: on macOS `viola_state::liveness::server_verdict` returns `Matches` with no comparison, the recorded gap; the macOS client euid and socket-directory checks and the 0755-directory exit-cause case are owed to the working-route entry "Unix endpoint and home hardening".
- Stack, Wrapper IPC: both clients read the serving process's pid through `StreamCommon::peer_creds().pid()`, the identity each frame-writing client checks before its first frame. Process liveness: sysinfo's process start time is also the observed side of that check. No pin moved.
**Why:** the chunk added the peer-pid read to both clients and the check to every frame-writing client; no dependency was added or bumped. interprocess 2.4.4's Windows peer pid was read in its source and witnessed by the two `server_pid` tests on windows-2025. The macOS cases' owner is the P5 review card's re-pin, the word of the Viola overseer as the founder's delegate.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — fork 1: a verified client meets an unreadable or newer snapshot as an absent one
**Section:** §Established Decisions [Message Broker / IPC] (established-decisions.md)
**Change:**
- A verified client meets an unreadable or newer snapshot as an absent one: every reader stays on `read_snapshot` (§Standard Contracts, Snapshot envelope), and the readers' move onto `read_snapshot_or_replay` is owed to the working-route entry "Home and code-bearing file integrity".
**Why:** fork 1, option 1, is the founder's own ruling, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it replaces the overseer's PROVISIONAL delegate answer the chunk report records. The classified read is taken only after `check_instance`, the check that route entry adds before these same readers' reads, so the move goes there with it.
**Kept:** option 1 changes no outcome: every reader reads such a snapshot as absent, as before the chunk.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook's one connect and its fail-open on a failed server check
**Section:** §Established Decisions [Hook Transport], [Hook Contract] (established-decisions.md) · §Standard Contracts, Channel methods (standard-contracts.md: `hook.event`)
**Change:**
- [Hook Transport]: the event and dialog paths share one connect: `read_snapshot`, then `Client::connect_by`, then `verify_server`. A serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame: the hook records `server-verify-failed` and exits 0 with no body, the event lost as for an unreachable endpoint.
- [Hook Transport], SessionEnd: was the channel, else a direct `try_lock` append, else nothing; now the direct append is taken when the endpoint is not reached or its server check fails.
- [Hook Contract]: a dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
- `hook.event`: sent id-less by `hook` only once its server check has passed; a failed check sends nothing and the hook fails open.
**Why:** the chunk built the shared connect in `src/cmd/hook.rs` for `deliver` and `ask`. Its Outcome: the hook's `stop`, `pre-tool-use` and `permission-request` against an impostor exit 0 with empty streams under 1.0 s and no frame, and a SessionEnd whose check fails takes the direct append. The `hook.event` and `hook.dialog` gaps' server-identity half is retired on Linux and Windows; their strict-modes half stands until "Home and code-bearing file integrity".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the snapshot's pid and started_at as the server identity every frame-writing client checks
**Section:** §Established Decisions [Session Liveness] (established-decisions.md) · §Occupied Resources IPC endpoints (occupied-resources.md) · §Standard Contracts, Channel methods and Session liveness (standard-contracts.md: a new detail group), Instance snapshot (architecture.md)
**Change:**
- Instance snapshot: was "`pid` and `started_at` identify the wrapper process for the liveness check"; now also, on Linux and Windows, for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame (macOS reports no peer pid, the recorded gap).
- Session liveness: `viola_state::liveness::live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call, so the liveness-only pre-check comes first and the server check after it. The closed `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares in whole seconds, `started_at` read through its RFC 3339 form; with `peer_pid_reported` false it returns `Matches` with no comparison. The joined `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` reads the start time through `process_start_time` (a zombie reads none).
- [Session Liveness]: the snapshot's `pid` and `started_at` are also the server identity, returned by `live_endpoint` and compared by `verify_server`.
- Channel methods: before its first frame the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, makes that comparison from its one snapshot read; a mismatch or an unmakeable check writes no frame.
- IPC endpoints: every process connecting to the recorded `endpoint` but `run`'s collision check makes the check; a failure is CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open.
**Why:** the chunk retired the server-identity half of the first and the fourth to eighth dated gaps on Linux and Windows (`hook.event`; CLI `send`; `wait` / `last`; `answer` + `hook.dialog`; `pause` / `release`; `viola mcp`'s four frame tools); their strict-modes half stands until "Home and code-bearing file integrity". The collision check writes no frame and verifies no server.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the diagnostics lines a failed server check writes
**Section:** §Occupied Resources Filesystem (occupied-resources.md: `diagnostics/`)
**Change:**
- `cli-<name>.ndjson`: exit 21 writes `detail:"instance-dead"` + `during`, or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line), at WARN, with no pid, start time, path or endpoint.
- `mcp.ndjson`: no `channel-request` / `channel-response` lines for a frame tool whose server check fails.
- `hook-<name>.ndjson`: a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN; on the dialog path after `hook-invoked{corr:null}`, with `corr:null, decision_emitted:false`.
**Why:** `server-verify-failed` already stood in `schemas/diag-line.v1.json`'s `process-exit` and `hook-decision` `detail` enums; this build is the first that writes it, and no pid, start time, path or endpoint reaches any line (asserted). The MCP writes no line for the mismatch, as on its unreachable path.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — read_snapshot's product callers re-measured at three
**Section:** §Standard Contracts, Snapshot envelope (standard-contracts.md)
**Change:**
- `read_snapshot` has three product callers (was five, per "2026-10-10-statusline-pass-through — counts, crate edges and one order finding: the fake agent, the seeds, the snapshot readers, revive's log before its check"): `src/cmd/hook.rs` once, the read its event and dialog paths share; `run`'s collision check; `src/cmd/hook/statusline.rs`. The rule: `read_snapshot(` under `src/`, outside test modules. `src/cmd/client.rs` leaves the list: the CLI frame verbs and `viola mcp` read the snapshot inside `viola_state::liveness::live_endpoint`.
**Why:** measured at the chunk's tree. The base read four, `src/cmd/client.rs` gone since chunk 2026-10-17-mcp-server-for-drivers moved the CLI read into `live_endpoint`, and this chunk joined hook.rs's two reads in one. The plan's "four" was counted before that join: a count carried from a plan is re-measured at the wrap, never copied.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — viola-state's server identity check and the verified CLI connect in the key files
**Section:** §Infrastructure Patterns → Crate dependency direction (crate-dependency-direction.md: `viola-state`) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/cmd/client.rs`, `viola-state`)
**Change:**
- Crate dependency direction: `viola-state`'s sysinfo edge also serves the start-time read of `liveness::verify_server`, made before the first frame by CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook` and `viola-mcp`'s frame tools; `live_endpoint` returns `LiveEndpoint` (the endpoint, pid and start time) from one snapshot read (was the liveness-only endpoint read, "moved unchanged from `src/cmd/client.rs`"). No dependency added.
- Directory structure, `src/cmd/client.rs`: was the channel client `send` / `wait` / `last` share; now `connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens (the live endpoint, then the server identity check, then the client), the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`, and the channel client those verbs share.
- Directory structure, `viola-state`: was "the liveness-only endpoint read"; now the live-endpoint read (`live_endpoint` → `LiveEndpoint`) and the server identity check (`ServerVerdict`, `server_verdict`, `verify_server`).
**Why:** the chunk closed the frame verbs' liveness-only pre-check as their whole guard and built `LiveEndpoint`, `ServerVerdict`, `server_verdict`, `verify_server` and `cmd::client::connect`; no dependency was added or bumped.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-explicit-session-links — the link verbs' exit codes
**Section:** §Conventions (conventions.md: CLI exit codes `1`, `20`, `21` and the server-mismatch exit-21 group; architecture.md: CLI exit codes) · §Occupied Resources (occupied-resources.md: a new `res-binaries` group, the `link` / `unlink` subcommands)
**Change:**
- Exit `1`: the `cli`-role verbs gain `link` and `unlink`; for them exit 1 also covers a refusal reply and an `ok` of another shape than the verb's (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object).
- Exit `20`: a `-32602` from `link` / `unlink` is included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`); the self-pair is refused by the wrapper only, never client-side.
- Exit `21`: `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens) is the same exit and document as `list` and `plugin install` (human mode the instance-state `unable:` / `hint:` pair; no log opened, no line written, the mode left as found), not `revive`'s exit 1. architecture.md: was "a refused strict-modes check of `viola list` or `viola plugin install`"; now also `viola link` or `viola unlink`.
- Server mismatch: was six verbs, `send` to `release` (per "2026-10-26-server-verification-before-any-frame — exit 21 and the MCP unreachable result for a server that is not the recorded wrapper"); now eight with `link` and `unlink`, whose instance check (exit 21) comes first with the client's own refusals (exit 13), before any connect.
- Subcommands: `link` and `unlink` each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the verb's instance the driven one; exit 0 with one result line, or `{"v":1,"ok":<ok>}` under `--json` (`link` `{since, already_linked}`, `unlink` `{}`); 21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper; 20 for a wrapper error, a `-32602` included; 1 for a refusal reply or an `ok` of another shape.
**Why:** the chunk built both verbs; their shared call maps the reply to 0 / 20 / 21 / 1 and refuses the instance check with exit 21. A refused check exiting 21 with no log and no line is the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (`inputs#I3`). The wrapper-only self-pair refusal is the operator's answer at chunk 2026-10-10-session-links, a lean of the P5 card the founder approved by his own word, relayed verbatim by the operator (`inputs#I4`).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — [Links]: the explicit verbs and methods built in the instance-check form, the hold retired
**Section:** §Established Decisions [Links] (established-decisions.md: `dec-links`; architecture.md)
**Change:**
- Was "As landed (chunk 2026-10-10-session-links) only the implicit link is built" (per "2026-10-10-session-links — the implicit link as landed, the explicit verbs held"); now both are built: the driven instance's wrapper serves the `link` and `unlink` methods (`LinkSet::link` / `LinkSet::unlink`, sharing `driver_of`), sent by `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, and records the implicit link itself, its order and `-32603` unchanged.
- An explicit `viola link` of an existing pair adds no line; was "returns `{}`"; now it answers its held `since` with `already_linked: true`.
- The hold: was not built (`-32601`, no subcommand, no product `unlink` line, `LinkSet::remove` with no caller) and held on "Explicit session links" by the founder's 2026-10-13 answer (per "2026-10-13-t06-38-35-wrap — the founder's Epoch 4 batch: budget.json ratified, rate_limits ruled, the board's refusal his, the link hold in Epoch 6"); now built with no dated gap, the `unlink` method `LinkSet::remove`'s product caller, so a product path appends `unlink` lines (`source: wrapper`, `{driver, driven}`).
- New: the verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700 and a check made after it passes a home at 0770; a refusal is exit 21. Then the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request.
- architecture.md: was "only the implicit link is built, the explicit verbs held"; now the implicit link and the explicit verbs are built, the verbs checking the driven instance's files before any log and its wrapper serving `link` / `unlink`.
**Why:** the chunk landed both verbs and both served methods and retired the hold, the `-32601` answer and `LinkSet::remove`'s dead-code allow. The boundary widening they carry is ratified, with no dated gap, by the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (`inputs#I3`); this wrap's answer-card confirm is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`). The order is held by a test that reads the mode after the refusal.
**Kept:** the implicit link's five shapes on the operator's word stand, not ruled by the founder.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link methods' log lines and the since stamp
**Section:** §Established Decisions [Links] (established-decisions.md: two new `dec-links` bullets)
**Change:**
- The wrapper opens no span and writes no process-log line of its own for `link` / `unlink`: its `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name. The verbs run inside the `cli.link` / `cli.unlink` dispatch span.
- `LinkSet::line` stamps the `link` line's `ts`, the pair's `since`, with `Utc::now()` directly, outside the `Clock` seam.
**Why:** as built and measured at the chunk: its tests read the channel pair holding no names, and its root cases assert `since` by its shape only, no seam injecting it.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the server check before the first frame covers link and unlink
**Section:** §Occupied Resources, IPC endpoints (occupied-resources.md) · §Standard Contracts, Channel methods (standard-contracts.md: the server-check group)
**Change:**
- IPC endpoints: the CLI frame verbs that check the serving process before their first frame were `send` / `wait` / `last` / `answer` / `pause` / `release`; now also `link` / `unlink`.
- Channel methods: the clients that compare the peer pid and its OS start time with the snapshot's `pid` and `started_at` before the first frame gain `link` and `unlink` (was `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.event`, `hook.dialog` and the MCP frame tools, per "2026-10-26-server-verification-before-any-frame — the snapshot's pid and started_at as the server identity every frame-writing client checks"). macOS stays the recorded gap.
**Why:** each verb sends its one request through `cmd::client::connect`, so the CLI frame verbs are eight (six before); against an impostor server both read no frame and exit 21 in both modes on Linux and Windows. The server check is part of the form the founder named at this chunk's P4 fork, his own answer relayed verbatim by the operator (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — link and unlink write the driven instance's cli log
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: `diagnostics/`)
**Change:**
- The `cli` verbs with producers gain `link` and `unlink`: like `send` to `release` they write into `cli-<name>.ndjson`, `<name>` the driven instance, `process-start`, the client `channel-request` / `channel-response` lines and `process-exit{subject:"self"}`, with the exit-21 `instance-dead` / `server-verify-failed` and exit-20 `wrapper-fault` details.
- A refused `link` / `unlink` instance check opens no role file and writes no line.
**Why:** the verbs open the `cli-<driven>` role file only after the instance check has passed, and a refused check writes no line in any log.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — link and unlink check the instance files before any log; check_instance at three sites
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: `instances/<ViolaName>/`; architecture.md: `instances/<ViolaName>/`)
**Change:**
- Was "Three readers check these files before they use them"; now four (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).
- `viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before `client::start`, before any role file or log opens and before any snapshot value is read; a refusal is exit 21 (`--json` the `instance-unreachable` document, stderr empty; human the instance-state `unable:` / `hint:` pair), no line written, the mode left as found. Was "Both checks come first"; now "Each of these checks comes first".
- `check_instance` product call sites: was two; now three (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share; the rule `check_instance(` under `src/`).
- architecture.md: the processes that check these files ahead of any log gain `viola link` and `viola unlink`.
**Why:** measured at the chunk's tree, `check_instance(` under `src/` reads 2 → 3. The plan's "3 → 4" counted `list`'s check, which runs inside `viola_state::board::scan`, outside `src/`, against the master's own rule: a call-site count is taken by the rule its master states.
**Kept:** the count's rule stays `check_instance(` under `src/`; `list` is a checking reader with no call site under it.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the served link and unlink methods and the link set
**Section:** §Standard Contracts, Instance snapshot (standard-contracts.md: `contract-instance-snapshot`, four new bullets) · Channel methods (standard-contracts.md: a new `contract-channel-methods` group; architecture.md: the `link` / `unlink` row)
**Change:**
- Channel methods: was "`link` and `unlink` `{driver, driven}` … → `{}`. Neither is built …: the wrapper answers both `-32601`" (per "2026-10-10-session-links — the implicit link as landed, the explicit verbs held"); now `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
- `link` and `unlink` read no `from`: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write.
- `links`: the set changes on an implicit link and on a served call: `LinkSet::link` adds the pair through `record`, `LinkSet::unlink` removes it through `remove`. A `link` of a held pair appends nothing and answers that pair's `since` with `already_linked: true`; a new pair answers its new `link` line's `ts` as `since`, with `already_linked: false`. `unlink` answers `{}` whether or not the set held the pair; an absent pair appends nothing and leaves the set as it was. A `-32602` writes no line; a failed record or remove is `-32603`.
**Why:** the chunk serves both methods with these params, refusals and payloads. Serving them is the boundary widening the founder ratified at this chunk's P4 fork, his own answer relayed verbatim by the operator (`inputs#I3`).
**Kept:** the set still follows the log: its restore at every start, its replay rule and its failed-append rule stand as written.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — a product path appends the unlink event
**Section:** §Standard Contracts, Event data (architecture.md: the wrapper-appended paragraph) · Channel methods (standard-contracts.md: the `wait` group's wake sentence)
**Change:**
- Was "Since chunk 2026-10-10-session-links the driven instance's wrapper also appends `link` … on an implicit link … No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served"; now the driven wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new pair, on an implicit link or a served `link` call, and `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes (`LinkSet::remove`). Both are log-only and never wake `wait`, and a replay learns the link set from them.
- `wait`: the wrapper records that never wake gain `unlink` (the `send-*`, `link`, `unlink` and start records).
**Why:** `LinkSet::remove` has its first product caller, so the `unlink` line, a kind already in the closed set, is written by a product path for the first time; it is not in `WAIT_WAKE`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' --json documents
**Section:** §Standard Contracts (standard-contracts.md: a new `contract-cli-json-output` group; architecture.md: the subjects list gains `contract-cli-json-output`)
**Change:**
- New group: `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21), stderr empty and no line in any log.
**Why:** the chunk's verbs print these documents, and the detail file held no group for CLI `--json` output.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — links.rs in the directory structure
**Section:** §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/run/links.rs`)
**Change:**
- Was "`remove`, which has no product caller yet"; now `remove` (its product caller the served `unlink` method), and the served `link` / `unlink` channel methods, sharing `driver_of`.
**Why:** `LinkSet::link` / `LinkSet::unlink` share `driver_of`, and `LinkSet::remove` has its first product caller; the key file restated the retired no-caller claim, a site the chunk's sweep did not list.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the strict-modes refusals' exits: 21 for the six frame verbs, 1 for a refused home tree
**Section:** §Conventions (conventions.md: three new `conv-exit-codes` groups, the server-mismatch exit-21 group; architecture.md: CLI exit codes) · §Occupied Resources (occupied-resources.md: a new `res-binaries` group) · §Standard Contracts (standard-contracts.md: a new `contract-cli-json-output` group) · §Infrastructure Patterns → Build system (build-system.md: the `human` writer's callers)
**Change:**
- Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`, the verb's first step, after its text read for `send`): human mode `link`'s instance-state `unable:` / `hint:` pair, `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; nothing under the home opened, no line written. architecture.md: was "`viola list`, `viola plugin install`, `viola link` or `viola unlink`"; now also a CLI frame verb.
- Server mismatch: was "the client's own refusals (exit 13), and for `link` / `unlink` the instance check of the driven instance (exit 21), come first" (per "2026-10-26-explicit-session-links — the link verbs' exit codes"); now the strict-modes instance check (the target instance, the driven one for `link` / `unlink`) and the client's own refusals (exit 13) come first on all eight verbs, before any connect.
- Exit `1` also covers `run`'s start and `revive`'s start arm refusing the home tree before the wrapper log opens (`checked_exe`, then `refuse_tree`): the home-state pair, or the instance-state pair when the refused path is the instance's; no log, no line, no child; under `revive --json` the `strict-modes-failed` document. And `verify` refusing the home tree (`check_home_tree` with no name and no key) before its `cli` log, or `update_stamps`' `StampsError::StrictModes`: the home-state pair, no step line, no summary, no line. architecture.md's exit `1` names them.
- Binaries and CLI `--json`: the six verbs' exit 21 and document, `verify`'s and the starts' exit 1, as above.
- Build system: the `human` writer's callers gain the start's refused home tree and the CLI frame verbs' instance-state refusal (`client::refuse_untrusted`, shared with `link` / `unlink`).
**Why:** the chunk built `refuse_untrusted` at each verb's entry, `checked_exe` / `refuse_tree` and `verify`'s tree check. A refused check writing no line, as `link`'s, and the six verbs printing `link`'s instance pair are the founder's own answers to this chunk's P4 forks Q2 and Q3, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the instance check first at every CLI frame verb and MCP frame tool
**Section:** §Conventions (conventions.md: a new `conv-mcp-results` group) · §Established Decisions [MCP], [Links], [Session Liveness] (established-decisions.md: a new `dec-mcp` group, the explicit-verbs bullet, the server-identity bullet) · §Standard Contracts, Channel methods and Session liveness (standard-contracts.md: a new `contract-channel-methods` group, `contract-session-liveness`) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/cmd/client.rs`)
**Change:**
- MCP: each frame tool (`send`, `wait`, `last`, `answer`) runs `viola_state::strict::check_instance` on the home and the target's instance directory before `live_endpoint`; a refusal is the `instance-unreachable` `isError` result, `detail: null`, text `refused: instance-unreachable`, no frame, no line.
- [Links]: the explicit verbs' check runs through `client::refuse_untrusted` (`src/cmd/client.rs`, shared with the six frame verbs), its place and exit 21 unchanged (per "2026-10-26-explicit-session-links — [Links]: the explicit verbs and methods built in the instance-check form, the hold retired").
- [Session Liveness] and Session liveness: was "every frame-writing client takes the liveness-only pre-check first" (per "2026-10-26-server-verification-before-any-frame — the snapshot's pid and started_at as the server identity every frame-writing client checks"); now each runs the instance's strict-modes check first (the CLI's `client::refuse_untrusted`, `viola mcp`'s frame tools, the hook's `trusted`), then the liveness-only pre-check, then the server check.
- Channel methods: before the server comparison each client runs that check (`refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`; `trusted` for `hook.event` and `hook.dialog`; `strict::check_instance` in the MCP frame tools); a refused check writes no frame and no line.
- Directory structure, `client.rs`: gains `refuse_untrusted` (`check_instance` on the absolute home and `instances/<name>`; refused: exit 21 with the instance pair, or the `instance-unreachable` document under `--json`; nothing opened under the home, no line), run first by the six frame verbs and `link` / `unlink`.
**Why:** the chunk wired the check into `tools.rs`'s `frame` and into `refuse_untrusted` at each verb, retiring the strict-modes half of the dated gaps of CLI `send`, `wait` / `last`, `answer`, `pause` / `release` and `viola mcp`'s four frame tools. No line on a refused check is the founder's own answer to this chunk's P4 fork Q2, live by dialog, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the hook's instance check before anything under the home on its event, dialog and statusline paths
**Section:** §Cross-cutting Patterns, Config management (cross-cutting-patterns.md) · §Established Decisions [Hook Transport], [Hook Contract] (established-decisions.md: SessionEnd; a new `dec-hook-contract` bullet) · §Occupied Resources, Environment variables (occupied-resources.md: a new `res-env-vars` group; architecture.md: `VIOLA_NAME`) · §Standard Contracts, Channel methods and Hook statusline (standard-contracts.md: `hook.event`; the arm's order) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/cmd/hook.rs`, `hook/statusline.rs`)
**Change:**
- `viola hook` takes the `instance_of` session only when its `VIOLA_DIR` instance directory exists (`canonical`) and passes `check_instance` (`trusted`), both now in `src/cmd/hook.rs` (was `hook/statusline.rs`), shared by the event, dialog and statusline arms, before `diagnostics_level` is read or obs starts. A `VIOLA_DIR` naming no existing instance directory, or a refused instance, exits 0 at once: no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. The two `--capture` arms, which read no `VIOLA_*`, stay ahead of it.
- `VIOLA_NAME`: was "its absence makes every hook event path a silent exit 0"; now its absence, a `VIOLA_DIR` naming no existing instance directory or a refused instance makes every hook event and dialog path a silent exit 0, with nothing read or written.
- [Hook Transport] SessionEnd: the direct `try_lock` append is taken only once the instance check has passed (was whenever the endpoint was not reached or its server check failed, per "2026-10-26-server-verification-before-any-frame — the hook's one connect and its fail-open on a failed server check").
- [Hook Contract]: beside the absent-`VIOLA_NAME` exit, the event and dialog paths run the check on the canonical instance directory before the diagnostics level and obs init.
- `hook.event`: sent only once `trusted` and the server check have passed; a refused or absent instance sends nothing and the hook fails open.
- Hook statusline: the arm's `canonical` / `trusted` are hook.rs's, shared with the event and dialog paths.
**Why:** the chunk moved `canonical` / `trusted` into hook.rs ahead of `read_diagnostics_level` and obs init, retiring the dated gap of `hook.event`'s snapshot read without strict-modes and `hook.dialog`'s share of `answer`'s. A `VIOLA_DIR` naming no directory writing nothing is one of the three behaviour changes the wrap directive has stated in the masters, the overseer's, founder-delegated (`inputs#I5`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the ConPTY companions' owner/DACL check and the strict-modes-failed fallback
**Section:** §Established Decisions [PTY], [Session Liveness], [Deployment / Distribution] (established-decisions.md: the sideload bullet, the start-order outcome list, a new `dec-deployment-distribution` group) · §Occupied Resources (architecture.md: `bin/<version>-<hash>/conpty/`)
**Change:**
- [Deployment / Distribution]: on Windows x64, before the held open, `pin_companions` runs `viola_state::strict::check_present` on `bin/<version>-<hash>/conpty/` and on each companion that exists; a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit code unchanged.
- [PTY]: the fail-open causes gain a `conpty/` directory or companion failing its owner/DACL check.
- `run.conpty_sideload`'s `outcome`: was `loaded` · `hash-mismatch` · `unreadable` · `load-failed` · `not-built`; now `strict-modes-failed` after `hash-mismatch`, still never a refusal.
- `conpty/` (architecture.md): was "at every start each is held with `FILE_SHARE_READ` only and fully re-hashed"; now the directory and each companion that exists first pass the strict-modes check (owner and DACL), then the held re-hash; a refused check or a mismatch writes nothing, leaves the file as found, never loads it, never refuses the start, and falls back with `strict-modes-failed` or `hash-mismatch`.
**Why:** the chunk built the owner/DACL leg, retiring the dated gap of the companions loaded under the SHA-256 re-hash alone; its Outcome reads the fallback on `windows-2025` with the exit unchanged, and `schemas/diag-line.v1.json`'s `sideload_fallback` enum gained the word.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers moved onto the replay: six product readers, read_snapshot none
**Section:** §Established Decisions [Message Broker / IPC] (established-decisions.md: `dec-message-broker-ipc`) · §Standard Contracts, Snapshot envelope (standard-contracts.md; architecture.md: a new bullet) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `viola-state`'s `replay.rs`, `src/cmd/hook.rs`)
**Change:**
- [Message Broker / IPC]: was "every reader stays on `read_snapshot` …, and the readers' move onto `read_snapshot_or_replay` is owed to the working-route entry 'Home and code-bearing file integrity'"; now a verified client meets an unreadable or newer snapshot as an absent one through `replay::read_snapshot_or_replay`: a replayed snapshot holds no `endpoint`, so no endpoint and no frame.
- Snapshot envelope: the fallback's product readers 2 → 6, each after the instance's strict-modes check: beside `viola revive`'s preflight and `viola list`'s `board::read`, `viola_state::liveness::live_endpoint` (the CLI frame verbs, `link` / `unlink` and `viola mcp`'s frame tools), `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check.
- `read_snapshot` keeps its results and has no product caller (was three: `src/cmd/hook.rs`, `run`'s collision check, `src/cmd/hook/statusline.rs`); the rule `read_snapshot(` under `src/`, outside test modules.
- architecture.md: a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `live_endpoint` and the hook's connect read the instance as unreachable, `hook statusline` runs nothing and the collision check judges liveness from the heartbeat alone.
- Directory structure: `replay.rs`'s readers gain `liveness::live_endpoint`, hook `connect`, `serve` and the collision check; `hook.rs` holds the event and dialog paths' `connect`, a replayed snapshot unreachable.
**Why:** moving the readers now, so hook, `mcp` and CLI processes read `events.ndjson` on a new basis, is the founder's own answer to this chunk's P4 fork Q1, his boundary-widening ratification, live by dialog, relayed by the overseer (`inputs#I3`); this wrap's confirm of the proposals inside it is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`). Measured at the chunk's tree, `read_snapshot(` product callers under `src/` 3 → 0.
**Supersedes:**
2026-10-26-server-verification-before-any-frame — fork 1: a verified client meets an unreadable or newer snapshot as an absent one
2026-10-26-server-verification-before-any-frame — read_snapshot's product callers re-measured at three
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — what each replay reader decides; one read per revive, one state-recovered line
**Section:** §Established Decisions [Message Broker / IPC], [Hook Transport], [Session Liveness] (established-decisions.md) · §Occupied Resources, IPC endpoints and Filesystem (occupied-resources.md: `res-ipc-endpoints`; a new `res-filesystem` group) · §Standard Contracts, Channel methods, Hook statusline, Session liveness (standard-contracts.md)
**Change:**
- IPC endpoints: was "It rebuilds the path by the same rule only when the snapshot is missing or unreadable"; now each connecting process reads the snapshot through the replay and only a snapshot read whole yields an `endpoint`: `live_endpoint` returns none on a replay, an absence or an error; hook `connect` is unreachable on a replay or an absence (fail open; SessionEnd appends directly). None rebuilds the path.
- Session liveness: `live_endpoint` returns `LiveEndpoint` from its one `read_snapshot_or_replay` call only on a real snapshot (was its one `read_snapshot` call); the channel clients' one read is the replay, and only a real snapshot goes on to the server comparison.
- [Hook Transport]: was `read_snapshot`, then `Client::connect_by`, then `verify_server` (per "2026-10-26-server-verification-before-any-frame — the hook's one connect and its fail-open on a failed server check"); now, once the instance check has passed, `read_snapshot_or_replay` first, a replayed or absent snapshot an endpoint not reached (`channel-unreachable`); `hook.event` is not sent then.
- Hook statusline: `serve` reads through the replay; a replayed snapshot holds no `statusline_command`, so nothing runs or prints.
- [Session Liveness]: `run`'s collision check takes its caller's one read; over a replayed snapshot it decides from the heartbeat alone (a beat at most `STALE_AFTER` old refuses `already-live`, an older one free); a failed read or no snapshot is free. `viola revive`'s preflight reads once and hands that read to its collision check and its recorded-directory reading.
- Filesystem: a replayed snapshot writes one `state-recovered` line into the reading process's role file (`cli-<name>`, `mcp`, `hook-<name>` or `run-<name>`) from each replay reader; one line per revive.
- [Message Broker / IPC]: the `state.snapshot_recover` span sits under `run.start` before `run.collision_check`, and under `hook.handle` on the hook path.
**Why:** the readers' move is the founder's own answer to this chunk's P4 fork Q1, his boundary-widening ratification, relayed by the overseer (`inputs#I3`); this wrap's confirm is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`). The one read per revive is the chunk's: a replay in each reading would write two lines in one revive. The span's place is one of the three behaviour changes the wrap directive has stated, the overseer's, founder-delegated (`inputs#I5`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the home-tree check before the start's and verify's log; the pin key taken once
**Section:** §Established Decisions [Session Liveness], CLI version compatibility, [Deployment / Distribution] (established-decisions.md: new bullets in `dec-session-liveness`, `dec-cli-version-compatibility`, `dec-deployment-distribution`) · §Occupied Resources, Filesystem (occupied-resources.md: a new `res-filesystem` group; architecture.md: a new Home tree row)
**Change:**
- `viola_state::strict::check_home_tree(home, name, key)` checks the home (the read rule); with a name, `check_instance`'s set; then `ledger/`, `bin/` and `plugin/`; with a key, `bin/<key>/`, the pinned `viola` (`viola.exe` on Windows), `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (the write rule); only entries that exist, refusing as `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`.
- Every start (`run`'s and a revived one) calls it before its wrapper log opens, through `checked_exe` (the instance's name and the `pin_key` of the exe bytes read once): a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal; no log, no line, no child.
- `viola verify` calls it with no name and no key before its `cli` log: the home pair, exit 1, no step line, no summary, no line.
- [Deployment / Distribution]: `viola_state::pin::pin_key(bytes)` names the pinned copy's key, `<version>-<16 hex>`, in one place; `pin_bytes(home, bytes)` pins bytes already read and `pin_exe(home, exe)` delegates to it; `run` pins the very bytes its check hashed, and `verify` calls `pin_exe`.
- architecture.md Home tree: the start's check over those paths before any log, and `verify`'s over the home, `ledger/`, `bin/` and `plugin/` before its log.
**Why:** the chunk built `check_home_tree`, `checked_exe` / `refuse_tree` and `verify`'s check; its Outcome reads `run` refusing a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy, and `verify` a widened `ledger/`, each before any log. A refused check writing no line is the founder's own answer to this chunk's P4 fork Q2, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the stamps writer's strict-modes check; StampsError, a third open divergence
**Section:** §Established Decisions CLI version compatibility, [Error Handling] (established-decisions.md; architecture.md: [Error Handling]) · §Conventions, Rust error types (architecture.md) · §Occupied Resources, Filesystem (occupied-resources.md: `ledger/stamps.json`) · §Standard Contracts, Ledger stamps envelope (standard-contracts.md: a new `contract-ledger-stamps` group)
**Change:**
- `viola_state::stamps::update_stamps` runs `strict::check_stamps(home)` before it creates `ledger/` and again under its lock before the capped read; a refusal (`StampsError::StrictModes`) reads and writes nothing, and `verify` maps it to the home pair with exit 1.
- Its error is `StampsError` (`StrictModes` · `State`), each variant with a fixed message.
- [Error Handling], detail and broad lines, and Rust error types: the open divergences 2 → 3, `viola-state`'s `StampsError` beside `StateError` joining `Refusal` and `ServeError`; the rule stands, its fold or ratification open.
**Why:** the chunk's `update_stamps` refuses before it reads, retiring the dated gap of `update_stamps` reading `ledger/stamps.json` without strict-modes. The check also precedes the create, because under the lock alone `create_private_dir` would narrow a widened `ledger/` to 0700 first.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — diagnostics/: the opens' check, the refusals that open no role file, the detail file named
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: the `diagnostics/` producers bullet, a new `res-filesystem` group; architecture.md: `diagnostics/`)
**Change:**
- `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on an existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symbolic link refused, a folder to the read rule, a file to the read rule on Unix and the write rule on Windows, an absent path passing; a refusal is a failed open (the writer ends at `std::io::sink`, a detail line dropped).
- Producers: `run`'s start and `revive`'s start arm run the home-tree check through `checked_exe` / `refuse_tree` before `open_wrapper_log`, a refusal opening no process log; a refused instance check of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` opens no role file and writes no line (was `link` / `unlink`, per "2026-10-26-explicit-session-links — link and unlink write the driven instance's cli log"); `verify` writes its lines only once `check_home_tree` has passed; `mcp.ndjson` gets no line for a refused instance check; `hook-<name>.ndjson` none on any path for a refused or absent instance, `trusted` coming before `read_diagnostics_level` and obs init.
- Content-bearing detail: was "stays in `instances/<ViolaName>/diagnostics/`"; now in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson`, architecture.md naming their format `schemas/diag-detail.v1.json`.
**Why:** the chunk added `check_diagnostics` to both opens. Naming the detail file on the master's `diagnostics/` row is item (a) of the wrap directive, the overseer's, founder-delegated (`inputs#I5`). A refused check writing no line is the founder's own answer to this chunk's P4 fork Q2, relayed by the overseer (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the read rule and check_instance's widened set; the Windows instance-tree cases built
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: `viola revive`'s check bullet, the Windows proof bullet, a new `res-filesystem` group; architecture.md: `instances/<ViolaName>/`) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `viola-state`'s `strict`)
**Change:**
- `viola_state::strict` holds two rules: the read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, of any allow ACE, an inherit-only one included, granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects (`unix_read_verdict`, `windows_read_verdict`, `check_read`). `Refused` holds `Readable` for a read-rule refusal and `Symlink` for a symbolic link `check_diagnostics` meets, each a fixed message.
- `check_instance`'s set: was the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`; now the home, the instance directory and its `diagnostics/` (new) to the read rule, `instances/` and `snapshot.json` to the write rule, `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds.
- Windows: was "No case widens the DACL of an instance tree on Windows yet", owed to "Home and code-bearing file integrity"; now `cfg(windows)` cases prove it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused, naming its inherited Users (`S-1-5-32-545`) write entry.
- Directory structure: `strict` lists the write rule and the read rule (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`), `check_instance`, `check_home_tree` → `TreeRefused`, `check_diagnostics`, `check_present` and `check_stamps` (was the instance check alone).
**Why:** the chunk built the read rule and widened the set; the `cfg(windows)` cases are green on `windows-2025`, disproving the standing "no case" line.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the checked entry points and check_instance's call sites; viola list's two home checks
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: the `instances/<ViolaName>/` bullets, two new `res-filesystem` groups; architecture.md: `instances/<ViolaName>/`)
**Change:**
- Entry points: was four readers, `viola list`, `viola revive`, `hook statusline` and `viola link` / `viola unlink` (per "2026-10-26-explicit-session-links — link and unlink check the instance files before any log; check_instance at three sites"); now `viola list`; `viola revive`; `viola run`'s start (`check_home_tree` with its name); `viola hook`'s statusline, event and dialog paths (sharing `trusted`); `viola mcp`'s frame tools; and the CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (sharing `client::refuse_untrusted`), each ahead of any log, a refused check writing no line, the mode or ACL left as found.
- Call sites: three by the rule `check_instance(` under `src/`, their files moved: `src/cmd/hook.rs` `trusted` (was `src/cmd/hook/statusline.rs`), `src/cmd/revive.rs`, and `src/cmd/client.rs` `refuse_untrusted` (was `src/cmd/link.rs`), shared by `link` / `unlink` and the six frame verbs; outside the rule `viola-mcp`'s `tools.rs` `frame` calls it and `run`'s start reaches its set through `check_home_tree`.
- `viola list`: `board::scan`'s own `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused (`ScanRefused::Instance`) once it holds one; their reconciliation is owed to a CARRY on the working-route entry "Unix endpoint and home hardening".
**Why:** measured at the chunk's tree, `check_instance(` under `src/` 3 → 3, its files moved. The rewritten call-site line, against its own standing line, is applied with its companion on this wrap's halt answer Q2, the overseer's, founder-delegated (`inputs#I6`). The list CARRY's owner is the halt's Q3 answer, the overseer's, founder-delegated (`inputs#I6`), on item (c) of the wrap directive (`inputs#I5`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — three Windows readings for tests: a path under a regular file, the hard-linked seed, the protected DACL's LA
**Section:** §Occupied Resources, Filesystem and Repository (occupied-resources.md: a new `res-filesystem` group, two new `res-repository` groups, the new `res-binaries` group's last sentence)
**Change:**
- A path under a regular file stats `NotFound` on Windows, so the strict-modes check skips it as absent and the `cli` catch-site line stands there; on Unix the same home is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair); a test that plants a regular file above a checked path needs a per-OS oracle.
- An inheritable `icacls` grant (`(OI)(CI)`) on a seeded test home's `bin/`, or on the home, reaches through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until removed; a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded, and `seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir`.
- The protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on `windows-2025`, read back through `icacls /save`: canonical SDDL spells the runner's user, the local Administrator (RID 500), `LA`, so an oracle matching its SID string fails.
**Why:** each measured on `windows-2025` in the operator pass's two red CI runs and fixed by the chunk's two fix commits. The per-OS reading of an unstattable home is one of the three behaviour changes the wrap directive has stated, the overseer's, founder-delegated (`inputs#I5`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — [Snapshot writer]: the pinned copy's no-clobber placement beside the replace helpers
**Section:** §Established Decisions [Snapshot writer] (established-decisions.md: the decision's line, a new `dec-snapshot-writer` group; architecture.md: its broad line) · §Stack, State-file primitives (stack-and-technologies.md)
**Change:**
- Head: was "tempfile 3.27.0 `persist` through ONE shared helper" (per "2026-09-27-instance-state-and-start-order — tempfile `persist` snapshot writer, liveness by process check, start order as landed, `viola-state`"); now "tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`", in the detail and the broad line.
- `replace_private`'s mode: was 0600, or 0700 for the pinned `viola`; now 0600. `replace_private_shared`'s files: was the pinned copy, the plugin files and the ConPTY companions; now the plugin files and the companions. The pinned copy's absent-copy write is never a replace; no call site carries its own temp + rename code.
- New: `viola_state::fs::place_private_new(path, bytes, mode) -> Result<Placed, StateError>` (public `Placed { Written, Present }`), the pinned copy's absent-copy write (`viola_state::pin::place_absent`): a temp file beside the target (`NamedTempFile::new_in`), its mode set before any byte (the pinned `viola` at `DIR_MODE`), the bytes written, `sync_all`, the write handle closed (`into_temp_path`), then `TempPath::persist_noclobber`. `AlreadyExists` is `Ok(Placed::Present)`, the target untouched and the temp path removed; any other failure is an error leaving the target as it was; no retry and no error variant of its own.
- New: its private body `place_private_new_with(path, bytes, mode, placed)` runs `placed` right after a `Written` placement, a forced-window test parameter, not a seam (no env read, no feature): `place_private_new_placed_copy_execs_at_once` execs the placed copy from it on Unix, `place_private_new_placed_copy_runs_at_once` runs it on Windows.
- State-file primitives: adds tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`place_private_new`) for the pinned copy's absent-copy write, placed once and never replaced.
**Why:** `NamedTempFile::persist` renames while the write handle is still open, and a concurrent first start's agent exec of a copy still open for write fails `ETXTBSY` (raw errno 26): 4 of 24 two-instance boots failed so through the open-handle rename and 0 of 24 through `place_private_new`, as measured on the Linux dev host. tempfile 3.27.0 already held both calls.
**Kept:** the Inherited Defaults Database line («tempfile 3.27.0 `persist` through one shared helper») stands: it speaks of snapshots and logs, which still go through `replace_private`. `replace_private_shared`'s body and the companions' held-open retry under `REPLACE_ATTEMPTS` are unchanged.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — [Deployment / Distribution]: one source tree can build two files and key two pinned copies
**Section:** §Established Decisions [Deployment / Distribution] (established-decisions.md: the `pin_key` group, one item added; architecture.md: the decision's broad line)
**Change:**
- New: six cold release builds of one tree at one target path gave two files, 8 179 264 B and 8 179 272 B, three each in no fixed order, unequal in 3 of 34 sections (`.note.gnu.build-id`, `.symtab`, `.strtab`) with every loaded section equal and 6 `strings` lines apart (three `anon.<hash>.<n>.llvm.<number>` local symbols), so one tree can key two `bin/<key>/` and two `plugin/<key>/` folders. Its cause is not explained; acting on it is owed to the working-route entry "Version done-check".
- Broad line: the rationale that the content hash keys the copy because the version alone collides across rebuilds now adds that one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to "Version done-check".
**Why:** measurement B, on the Linux dev host, disproved the route CARRY's «the build was repeatable for a given source text». That it is recorded as measured, with no cause claimed, as a founder item carried on "Version done-check", is the operator's wrap directive, the overseer's word, founder-delegated (`inputs#I6` item 4).
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the pinned copy on disk: placed once with no clobber, a lost placement re-hashing the copy that stands
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: a new `res-filesystem` group; architecture.md: `bin/<version>-<hash>/viola(.exe)`)
**Change:**
- `bin/<version>-<hash>/viola(.exe)`: was "the pinned copy of the binary that `viola run` creates if it is absent"; now `viola run` places it once when it is absent, with no clobber, and never replaces it (0700 on Unix); the re-hash on every start now also covers a copy a concurrent first start placed first, a mismatch refusing the start (exit 1, `pinned-hash-mismatch`) and leaving the file as found.
- New record: when the copy is absent, `viola_state::pin::pin_bytes` places it through `pin::place_absent` (`viola_state::fs::place_private_new` at `DIR_MODE`, [Snapshot writer]). `Written` is ok. `Present` (a concurrent first start placed the copy first) re-hashes the copy that stands: equal bytes are reused, different bytes are `PinError::HashMismatch`, the file left as found and the start refused `pinned-hash-mismatch` (exit 1). `run.pin_copy` reads `ok` or `pinned-hash-mismatch`, a lost placement with equal bytes reading `ok`.
**Why:** `pin_bytes`' absent-copy arm called `replace_private_shared` before the chunk and now calls `place_absent`; no log line, field or code was added, and `run.pin_copy` keeps its outcomes.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the Windows plain-home refusal read by its inherited untrusted write grant; runner ACLs differ by host
**Section:** §Occupied Resources, Filesystem and Repository (occupied-resources.md: the Windows proof bullet of `instances/<ViolaName>/`, a `res-repository` item added)
**Change:**
- Plain home: was refused "naming its inherited Users (`S-1-5-32-545`) write entry (as measured at ci#40403389629, `windows-2025`)", per "2026-10-26-home-and-code-bearing-file-integrity — the read rule and check_instance's widened set; the Windows instance-tree cases built"; now refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it, while a home `instance_tree()` makes reads none (as measured at ci#40439188053, `windows-2025`).
- New: on `windows-2025` runners of one image (20261021.278.1) the temp dir's inherited ACL differs by host: Users write on one (ci#40403389629, westus3), Authenticated Users (`S-1-5-11`, mask `0x001301BF`) write with Users read only on another (ci#40425866096, northcentralus). An oracle that names one SID for a runner-inherited grant is runner-dependent, so the plain-home case reads the grant by its right (inherited, a `WRITE_RIGHTS` bit, a SID `trusted_sid` does not trust; `strict::win::tests::inherited_untrusted_writes`), with a control that cannot pass vacuously.
**Why:** ci#40425866096 disproved the Users-SID reading: the product verdict `Err(Writable)` was right and the oracle that demanded `S-1-5-32-545` failed. The test-only oracle fix is the chunk's red 2, folded here on delegate overseer1's word, PROVISIONAL, owed the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — bounded readers, a cross-cutting pattern held by a census
**Section:** §Cross-cutting Patterns (architecture.md: a broad bullet «Bounded readers» after «Untrusted upstream text»; the subjects line); cross-cutting-patterns.md (a new `procedure · crosscut-bounded-readers` group)
**Change:**
- The body gains the broad bullet: every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.
- The detail gains the group «Bounded readers at every boundary»: no product reader reads an external byte source to its end unbounded. The E2E contract binary (a `contract_` binary, run under `run --e2e`) holds it: its `BOUNDARIES` table, one `Boundary {` block per bounded read, names each read and its witness test, checked by `boundaries_each_name_a_bounded_read_that_stands`; its `EXCEPTIONS` table names the own-executable reads, which are no external input. The census `product_readers_are_each_bounded_or_named` walks the product sources and judges each reader call bounded over its statement up to the call, matching an `EXCEPTIONS` needle against the call's whole source line; its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one` reads a planted unbounded read as a hit and passes a bounded one; `serde_json_unbounded_depth_is_banned_in_deny_toml` reads the `deny.toml` feature ban. A new reader lands bounded in its statement or as a named exception, or the census goes red.
- The subjects line gains `crosscut-bounded-readers`.
**Why:** the chunk's acceptance, met: the census reads every product reader bounded in-statement or an `EXCEPTIONS` row, and its self-check reads a planted unbounded read as a hit. The needle is matched against the whole source line because the statement ends at the call's `(` and the executable argument the needle names lies past it (an implement deviation, its census hazard). The body bullet was raised so the new detail group stands under a broad line, the body's Cross-cutting Patterns holding none for the rule.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [PTY] OPEN: the Windows reading of the wide character after a feed panic
**Section:** §Established Decisions [PTY] (established-decisions.md: a new `pending · dec-pty` group)
**Change:**
- A new OPEN, PROVISIONAL: on a Windows host, how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal after a contained vt100 feed panic under a 1-column ConPTY is not read; `tests/chaos_feed_panic.rs` asserts those bytes on Unix and, on Windows, only that no viola literal appears on the outer terminal. The reading and a Windows byte oracle from it are owed to "Windows-only live measurements".
**Why:** ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so the Windows byte reading was unrunnable here; the chaos case is green on every CI OS, Windows reading the literal absence only. The owner is the P5 card's Beyond-the-plan CARRY on «Windows-only live measurements», approved by delegate overseer1 (founder-delegated, relayed by the operator), PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [Screen Model] the poisoning episode's parse-rejected detail
**Section:** §Established Decisions [Screen Model] (established-decisions.md: `dec-screen-model`)
**Change:**
- Was «one `parse-rejected{parser:"vt100-feed", detail:"oversize"}` per poisoning episode» (for a caught panic and a dropped message alike); now one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, its `detail` `panicked` for a caught panic and `oversize` for a message the full queue drops. The rest of the entry is unchanged.
**Why:** a spec claim disproved by measurement: the code writes `panicked` for a caught panic and `oversize` for a dropped copy, each asserted by a unit case of `src/run/gate.rs`; the security-plan and obs-plan details already said so.
**Kept:** the body's [Screen Model] entry, which names no `detail`, stands unchanged.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [MCP] the viola mcp stdin line cap
**Section:** §Established Decisions [MCP] (architecture.md: the body entry; established-decisions.md: three new `dec-mcp` groups)
**Change:**
- The body [MCP] entry gains: `viola mcp`'s stdin is read under a `MAX_FRAME` line cap, each line's newline counted; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it; the bound sits in the reader viola hands rmcp, because rmcp 3.4.1's stdio transport reads each line unbounded.
- Detail: the reader is the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` bytes per line, handed to rmcp as the reader of its `(reader, writer)` stdio transport (the path `transport-io` already enables; rmcp's features stay `server` + `transport-io`). A line of exactly the cap passes whole; at the end of stdin a held unterminated line within the cap is released; a longer line releases no byte, sets the reader's cut flag and then reads as end of stream. The wrapper channel's close-on-oversize rule is its precedent.
- Detail: after a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns `Ok(())` once the session initialized (exit 0, as at a closed stdin) or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.
- Detail: rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` bounds only the writer side (as read in rmcp 3.4.1's crates.io source); `CappedLines` signals a cut as end of stream, never as an I/O error, so rmcp writes nothing of its own.
**Why:** rmcp's unbounded `read_until` on `viola mcp`'s stdin was named this chunk's own subject (overseer1's ruling under the founder's night word). A truncated line would dispatch a padded valid request and a skipped one would leave the peer's request unanswered with no signal, so the line is refused whole. A cut before `initialize` keeps what a closed stdin before `initialize` gives (an implement deviation). No exit code, tool, method or error variant was added.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the mcp verb's session end and its mcp.ndjson line
**Section:** §Occupied Resources (occupied-resources.md: `res-binaries`, the subcommands line; `res-filesystem`, the `diagnostics/` `mcp.ndjson` list)
**Change:**
- The hidden `mcp` serves the five tools until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`) (was: until stdin closes). The subcommands line's text from «`verify` takes» onward moved, unchanged, to a continuation line.
- `mcp.ndjson` gains one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line); `process-exit` is written on stdin EOF or at that cut (was: on stdin EOF).
**Why:** the cut's exits are `mcp`'s existing closed-stdin outcomes, no exit code added; `schemas/diag-line.v1.json`'s `parse-rejected.parser` enum gains `mcp-stdin`, and `serve` writes the line after the session ends.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the serde_json unbounded_depth ban and the deny probe count
**Section:** §Stack and Technologies (stack-and-technologies.md: the Serialization row); §Infrastructure Patterns → Build system (registry key `architecture/build-system.md`: the Dependency policy bullet)
**Change:**
- The Serialization row's purpose adds: serde_json's `unbounded_depth` feature is banned in `deny.toml` (Build system), so every parse of external bytes keeps serde_json's default recursion limit.
- The Dependency policy's feature bans gain `serde_json/unbounded_depth`, its probe row `serde_json-unbounded-depth`.
- The probe verdict: was `deny-probes: 13/13 banned, control clean`, per «2026-10-17-mcp-server-for-drivers — the Tokio client and viola-channel's tree-form ban»; now `deny-probes: 14/14 banned, control clean`, the count one per `probes=(` row of `scripts/deny-probes.sh` plus the planted `channel-tokio-edge`.
**Why:** `deny.toml` gains a `[[bans.features]]` table denying serde_json's `unbounded_depth`, and `scripts/deny-probes.sh` gains its probe row, measured `14/14` with a clean control; the count now carries the rule that produced it, and every master site of it moved in this pass.
**Kept:** the Serialization row's versions: no dependency was added or bumped.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
