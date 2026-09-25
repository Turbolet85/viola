# architecture — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — toolchain floor and exact pin
**Section:** §Stack and Technologies (Language / runtime) · §Infrastructure Patterns (Build system; Project directory structure `rust-toolchain.toml` comment) · §Inherited Defaults (Framework)
**Change:** Rust 1.98.1 is pinned exactly by `rust-toolchain.toml` (rustfmt + clippy). The workspace `rust-version` is 1.96, not 1.89. The "1.89 is the highest floor" claim is replaced by "1.96 is the declared floor", with sysinfo 1.95, `File::lock` 1.89, rmcp 1.88 and axum 1.80 listed below it.
**Why:** intent F-19 / matrix v1-19; the chunk shipped `rust-version = "1.96"` and `channel = "1.98.1"`. Sweep `1\.89|rust-version = "1\.89"|MSRV 1\.89|channel = "stable"|host 1\.95|Rust stable` over all 7 masters: 4 arch sites amended; obs-plan.md:1565 no change (§12 Decisions Log history); 0 elsewhere.

## 2026-09-24-three-os-ci-headless-harness-skeleton — CI setup and wired jobs
**Section:** §Stack and Technologies (CI/CD row) · §Infrastructure Patterns (CI/CD approach)
**Change:**
- CI now installs the toolchain with `rustup toolchain install` from `rust-toolchain.toml` (no toolchain action) and uses SHA-pinned actions/checkout 7.0.1, Swatinem/rust-cache 2.9.2, taiki-e/install-action 2.87.19 and actions/upload-artifact 7.0.1.
- The workflow sets `permissions: {}` at the top and `contents: read` per job.
- Two jobs are wired today: `test` (3-OS harness unit/integration plus the lifecycle leg, with an `agent-run-<os>` upload) and `mutants` (ubuntu, push + PR, base via `env:`).
- The six target jobs are kept and marked as owned by later chunks.

**Why:** the chunk shipped `.github/workflows/ci.yml` (report: Harness / gate surface). Sweep `dtolnay|rust-toolchain@stable|@stable` over all 7 masters:
- 2 arch sites amended; 2 security sites amended (security-plan sidecar);
- test-plan.md:1398 and :1403 amended (test-plan sidecar); test-plan.md:756 (nightly fuzz toolchain) and :1393 (MSRV 1.96 job) no change, because they install a different toolchain;
- security-plan.md:548 no change (the mutable-ref ban still holds).

## 2026-09-24-three-os-ci-headless-harness-skeleton — test-only crate, bins, env vars, paths
**Section:** §Established Decisions [Module Boundaries] · §Occupied Resources (Binary; Workspace crates; Environment variables; Filesystem; Repository) · §Conventions (Naming — environment variables) · §Infrastructure Patterns (Project directory structure)
**Change:**
- Registered the test-only crate `viola-e2e` (harness library + `viola-harness`, no-op `fake-agent` feature) and the test-only bins `viola-fake-agent` (root `[[bin]]`, feature `fake-agent`) and `viola-harness`.
- Registered the harness-only env vars `AGENT_RUN_CHUNK_BASE` and `AGENT_RUN_KEEP_HOMES`, and the `AGENT_RUN_` prefix for harness-only variables.
- Registered the home-level `diagnostics/` (`run-<name>.ndjson`, 0700/0600) and the repository paths `target/agent-run/`, `target/e2e-home/` and `target/harness/`.
- Added `src/bin/viola-fake-agent.rs`, `tests/`, `crates/viola-e2e/`, `scripts/agent-run.{sh,ps1}` and `.config/nextest.toml` to the tree.
- Module Boundaries states that each product crate is created by its first consumer.

**Why:** report Changes (Crates, Symbols, Env vars; expected amendment 3; `grep -c` of each name in architecture was 0). The child env the harness sets (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) was not registered: registry over-reach, because the arch registry tracks variables the product reads or sets.

## 2026-09-24-three-os-ci-headless-harness-skeleton — run's process log location, logging and serialization rows
**Section:** §Cross-cutting Patterns (Diagnostic output channels) · §Stack and Technologies (Serialization; new Logging row)
**Change:**
- `run`'s codes-only process log is the home-level `diagnostics/run-<name>.ndjson`; content-bearing detail stays in the instance's `diagnostics/`, per obs D-08.
- New Stack row: tracing 0.1.44 + tracing-subscriber 0.3.23 (root only).
- serde_json carries `preserve_order` (indexmap), so printed documents keep their declared key order.

**Why:** report Symbols (`viola run` appends the home-level role file), Dependencies (tracing, tracing-subscriber, serde_json `preserve_order`) and Spec claims disproved 5 (key order). Sweep `Its diagnostics go to the instance|diagnostics go to`: 1 arch site amended; 0 in other masters.

## 2026-09-24-fake-agent-and-test-data-fixtures — test homes, test env vars and test-side paths registered
**Section:** §Occupied Resources (Environment variables · Filesystem · Repository) · §Infrastructure Patterns directory tree
**Change:**
- Env vars: `AGENT_RUN_KEEP_FAILED` registered (read only by the root test chain). `AGENT_RUN_KEEP_HOMES` is now read by `viola-harness` and the root test chain. None is read by `viola`.
- e2e-home: `viola-session-*/home` (harness) and `viola-test-*/home` (root rstest) both named.
- Repository gains `fixtures/fake-scripts/`, `schemas/fake-script.v1.json` and `crates/viola-core/proptest-regressions/`.
- Filesystem gains the test-home-only `fake/<name>.control` / `fake/<name>.receipt.ndjson`.
- The tree gains `tests/support/`, `schemas/`, `fixtures/fake-scripts/` and viola-core `proptest-regressions/`.

**Why:** report Spec claims disproved 1 (architecture.md:374 claimed `viola-session-*` held every test home; root tests use `viola-test-*`) and 2 (:359 said only `viola-harness` reads `AGENT_RUN_KEEP_HOMES`); report Files / Env vars / Symbols (Wrapper::boot paths); plan expected amendment.
- Rejected: `tests/support/` as a Repository entry (registry over-reach; the tree carries it), and a §Stack Testing row (the invariant holds, because [Deferred] :106 hands test libraries to test-plan, which pins them).
- Sweep `every harness and test home`, `read by \`viola-harness\` and never`, `e2e-home`, `AGENT_RUN_KEEP` over all seven masters: 2 arch sites amended (:359, :374). The CI-jobs line :448 (`AGENT_RUN_KEEP_HOMES=1` set by ci.yml) needs no change. The test-plan and obs-plan `e2e-home` sites name the directory, not a prefix claim, so they need no change. test-plan :596 (harness cleanup removes its own `viola-session-*` parent) is accurate as written, and `viola-session-row` hits in a11y/test-plan are UI element names.

## 2026-09-24-supply-chain-and-workflow-gates — sole-root tokio ban, four-family deny policy, nightly.yml
**Section:** §Stack Code-quality row · §Established Decisions [Concurrency] · §Occupied Resources → Repository · §Infrastructure Patterns → Build system · directory tree · CI/CD approach
**Change:**
- Build system: the tokio ban moved to `deny-sync.toml`, run once per crate in `scripts/sync-crates.txt` as the sole root. It is no longer checked over a graph with the async crates excluded. `--exclude` false-fails under feature unification, as measured at research.md §Measured facts; the `viola-channel` own-root limit is recorded. `deny.toml` now carries four families: advisories, licences, sources, and bans (C, telemetry, feature).
- [Concurrency] enforcement text names the sole-root ban and the sync-crate list.
- Code-quality row: cargo-deny families corrected; zizmor 1.30.1 added (workflow lint).
- Occupied Resources: `target/deny-probes/` and `target/supply-chain/` registered.
- Tree: `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh` and `workflows/nightly.yml` added; the `deny.toml` comment is corrected.
- CI/CD: two workflows (`ci.yml` push/PR, `nightly.yml` weekly + dispatch). `ci.yml` jobs are now `test`, `mutants`, `lint`, `supply-chain`. Target job 3 reads `scripts/sync-crates.txt`. zizmor is installed by `cargo install --locked`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates shipped these. The report's "Spec claims disproved" #1 falsified the excluded-graph mechanism. The P4 operator decision placed the weekly run in `nightly.yml`. Of the fan-out's 9 D-arch proposals, 8 were applied as re-derived. The Occupied-Resources proposal was applied only in part: the two `target/` dirs were registered, and its config files were routed to the tree (playbook "Registry over-reach").

Sweep over all seven masters, patterns `wrappers list`, `excluded, not with`, `one workflow .ci`, `single workflow`, `separate workflow trigger`, `licences, C-crate bans, tokio`, `licences, C-dependency`, `-p viola-core -p viola-pty`, `check bans -c`, `tokio wrappers`, plus the claim reads `tokio.{0,40}(ban|wrappers)`, `(ban|deny).{0,60}tokio`, `all three targets`:
- Before apply, 17 amend-sites were found: arch :36, :44, :392, :418, :456, :463; security :134, :161, :308, :314, :327; test :486, :1399, :1411, :1434, :1617; obs :1217; a11y :1108. The remaining hits were left unchanged: arch :35; test :98, :164, :363, :396; obs :406, :777, :1239, :1378, :1443; a11y :175, :280, :499, :624, :793, :807, :813, :1063, :1153. They are still true or unrelated "all three OSes" wording.
- After apply: 0 hits in the masters and 0 in the preserve-verbatim homes, playbook and drift-base. The control fired on leaves commands.md:47, stack.md:32 and services/viola-mcp.md:17, which are re-derived in cascade step 3.

## 2026-09-24-diagnostics-plane — diagnostics roots, config key, diag schemas, v exception
**Section:** §Stack and Technologies (ORM / migrations row) · §Established Decisions [Hook Contract] · §Occupied Resources → Filesystem (`config.json`, `diagnostics/`, `instances/<ViolaName>/…` rows) · §Occupied Resources → Repository (`schemas/` rows) · §Infrastructure Patterns → Project directory structure (`schemas/` comment) · §Cross-cutting Patterns → Config management (Settings) · §Cross-cutting Patterns → Diagnostic output channels
**Change:**
- **`v` rule:** "every record carries `v`" now names its exception. Process-log lines carry no `v`, and their version lives in the schema filename.
- **Logging destinations:** `hook` logs to the home-level `diagnostics/hook-<name>.ndjson`, with content-bearing detail only in the instance's `detail-hook.ndjson`. Every role's codes-only log goes to its home-level role file (`run-`/`hook-`/`mcp`/`ui-`/`cli-`). `mcp` falls back to stderr JSON only when its file cannot be opened.
- **Registry and settings:**
  - The `config.json` row registers `diagnostics_level` (info | debug), the `MAX_FRAME`-capped read that requires `v` = 1, and the `parse-rejected` report.
  - The home `diagnostics/` row lists all five role files; only `run` has a producer today.
  - The instance `diagnostics/` row names the owner-only `detail-<role>.ndjson` files.
  - `schemas/diag-line.v1.json` and `diag-detail.v1.json` are registered in the Repository rows and the tree.
  - The Settings bullet names `diagnostics_level` as the only source of the level; `RUST_LOG` has no effect.
**Why:** chunk 2026-09-24-diagnostics-plane shipped these facts: report Changes → Schema/config and Counts, and Spec claims disproved #2. All 9 D-arch fan-out proposals were applied, re-derived. The `v` exception is also the operator's P5 direction.

This pass's sweep covered all seven masters, CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`, playbook and drift-base. Patterns: `Every record carries .v.|every (viola )?format carries`, `reserved for .hook.|diagnostics go only to the instance|diagnostics go to stderr|run-<name>.ndjson. today|Its diagnostics go`, `test-side JSON schemas`, `budget thresholds, GUI port\)`, plus the mechanism reads `` `mcp`.{0,80}stderr ``, `when .VIOLA_DIR. is set`, `diagnostics go (only )?to`, `in-session .mcp. diagnostics`.
- **Amended:**
  - arch :21, :67, :365, :369, :370, :378 (+1 row), :435, :483, :492;
  - obs :554 cited the retired arch wording as a verbatim quote and was rewritten to cite §Diagnostic output channels.
- **No change:**
  - obs :202, :45, :286 sit in obs §1, a verbatim copy of obs-scope whose pending wording is kept by rule (obs :485).
  - obs :485, :635, :1298, :1749 already state D-09.
- **Routed:** CLAUDE.md :119 (`USER:session-learnings`) goes to P3 curation, per the operator's directive.
- **Leaf re-derived:** `.claude/docs/stack.md:17`.
- **Control:** `unevaluatedProperties` fired 2 hits in obs.

## 2026-09-24-log-redaction-and-never-log-floor — anyhow's scope includes the catch-site reporter
**Section:** §Stack and Technologies (Error types row, :27) · §Established Decisions [Error Handling] (:95)
**Change:** anyhow 1.0.104 stays in the `viola` bin only, now named as `main`, dispatch (`cmd::dispatch` returns the error with the resolved home and instance) and the catch-site reporter `viola::obs::report_internal_error`. Context chains are "built at dispatch and recorded only in the owner-only instance detail file" (`chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`), never a role line, stdout or stderr. The rationale now reads "only useful where a person reads them: the owner's post-mortem".
**Why:** the chunk routed the dispatch error's `chain()` through `src/obs.rs` into the detail file (report Changes → Symbols / APIs, Crates / modules), which obs-plan §7 scrubbing layer 3 already required. The old "only at dispatch" / "`main` and dispatch" wording left the reporter outside the decision. Still inside the root bin crate; no dependency added.
**Sweep (cascade step 2):**
- **Patterns:** `only at dispatch|main\` and dispatch|main and dispatch|only useful where errors reach|anyhow[^|]{0,80}dispatch|context chains`, over the seven masters, CLAUDE.md, `.claude/rules`, `.claude/docs`, playbook and drift-base.
- **Amended:** arch :27, :95; obs :1117 (anyhow "at the root-bin dispatch edge only") — see obs-plan-amendments.
- **No change:** obs :54 quotes arch Stack's retired "context chains only at dispatch" inside obs §1, the verbatim obs-scope copy kept by rule (obs :485); arch :413 (root bin → anyhow; no dispatch-only claim); security :447 (external errors carry no anyhow chains — still true); CLAUDE.md :28 ("the only crate with anyhow" — still true) and :37 (external errors — still true); `rules/observability.md:19` (chain only in detail files — already true); `docs/services/viola.md:11` (dependency list), :28 (chain to detail file — already true).
- **Leaves re-derived:** `.claude/docs/stack.md:23`, `.claude/docs/conventions.md:34`, `.claude/docs/services/viola.md:6`.
- **Result:** 1 retired-claim hit remains, by rule: obs :54 in §1 (post-sweep grep over the same set). **Control:** the pattern fired 4 hits on `.raw-fanout-arch.md`.

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
- The Lint bullet and target job 2 carry `--features fake-agent` and the bans; the raw `event!` grep and `tests/contract_lints.rs` are named.
- Setup steps name `scripts/install-ripgrep.sh` (install-action has no ripgrep manifest) and runner `jq`.
- `test` job: follows the obs-plan §9 gate order with scan-gated uploads, replacing the unscanned `agent-run-<os>` upload.
- `lint` job: wires target jobs 1–2 plus the ripgrep install, G1 and G3, and the lint probes on Linux.
**Why:** chunk 2026-09-24-observability-gates (report Changes: Schema / config, Dev-tool versions, Harness / gate surface, Counts moved; Spec claims disproved 1). New gitignored `target/` resources registered in the same form as `target/deny-probes/`.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `all-targets -- -D warnings` (featureless clippy): 0 hits after the apply;
  - `agent-run-<os>`: arch 0 hits;
  - the CLAUDE.md lint line already carried `--features fake-agent`.
- Leaves: `.claude/docs/stack.md` Code quality row (verbatim mirror) re-derived; `.claude/docs/commands.md` re-derived. CLAUDE.md `GENERATED:setup:*` was recomputed against the amended Stack, Occupied Resources and Infrastructure: no stale line (pointer table, warnings and workflow unchanged).

## 2026-09-24-quality-gates — fuzz workspace, coverage and fuzz tooling, MSRV job, two-leg mutation, download-artifact
**Section:** Stack and Technologies (Language / runtime · CI/CD · Code quality rows) · Established Decisions [Module Boundaries] · Occupied Resources (Workspace crates · Repository) · Infrastructure Patterns (Build system Dependency policy · Project directory structure · CI/CD approach: workflows, Setup steps, Jobs wired today)
**Change:**
- `fuzz/` (`viola-fuzz`) is a separate cargo workspace excluded from the root (`exclude = ["fuzz"]`), registered with its toolchain file, targets, corpus and gitignored outputs. It is not a member.
- `artifacts/` also holds `llvm-cov-summary.json` and `mutants-verdict-<leg>.json`; `target/lcov.info` is registered.
- The runtime row records the CI `msrv` check (rustup 1.96 + `RUSTUP_TOOLCHAIN`) and the fuzz-only `nightly-2026-09-20`.
- The Code quality row adds cargo-llvm-cov 0.9.1 and cargo-fuzz 0.13.2 plus the fuzz lockfile exemption.
- The CI/CD row and Setup steps add actions/download-artifact 8.0.1 (5 pinned actions) and the new installs.
- The Dependency policy bullet records `fuzz/Cargo.lock` outside `cargo deny`: the operator-ratified test-only exemption, with its audit owed to "Workspace tree and code-graph planes".
- CI/CD: `nightly.yml` runs advisories + fuzz, with no `concurrency:` block (declined, why). The 7 `ci.yml` jobs are `test` (coverage + gate), the two-leg `mutants` matrix, `mutants-verdict` (union), `msrv`, `fuzz-replay`, `lint` and `supply-chain`.
- The tree shows `fuzz/`, the root `exclude` and the workflow comments.
**Why:** chunk 2026-09-24-quality-gates (report Changes: Crates/modules, Dependencies, Schema/config, Harness/gate surface, Counts). Operator rulings at wrap P2: the fuzz-lock exemption plus an audit CARRY.
**Sweep:** `dtolnay` 0 · `runs only`/`only for advisories` 0 · "four" actions 0 in masters. The `nightly.yml` hits in masters that stay true are the Threat Model CI/CD line (amended in security-plan), test-plan `:486`/`:1429` and a11y `:1108`. Leaves re-derived: `.claude/docs/stack.md` (Language / runtime and CI/CD rows), `.claude/docs/commands.md`, `.claude/docs/workflow.md`. CLAUDE.md `GENERATED:setup:*` recomputed with no change (the stack line already says "MSRV floor 1.96", and no block names CI jobs, actions or fuzz).

## 2026-09-24-workspace-tree-and-code-graph-planes — tree lists every test/obs/a11y artifact, release-check, orphans gate, fuzz lock audit, code-graph planes
**Section:** Stack and Technologies (Code quality row) · Occupied Resources → Repository · Infrastructure Patterns (Build system: Dependency policy, Boundary review, a new Code-graph planes bullet · Project directory structure · CI/CD approach: nightly sentence, Setup steps, jobs wired, target job 6)
**Change:**
- Code quality row: cargo-modules 0.27.0 is installed with `cargo install --locked` in `lint`; the orphans gate runs in CI and the dependency graph is reviewed on demand. `jq` is also used by `scripts/release-check.sh` and `scripts/orphans-check.sh`.
- Repository:
  - `target/supply-chain/` names `deny-fuzz.json` and is admissible by content (obs-plan §8 item 6);
  - new entries: `target/orphans-probes/run-<utc>-<pid>/`, `target/release-check/`, `target/perf/` (its hyperfine exports are the separate `perf/*.json`), `target/nextest/ci/junit.xml`, `e2e-web/test-results/` with `e2e-web/test-results/a11y/` and `e2e-web/test-results/lint/`.
- Build system:
  - the fuzz lockfile's audit is wired (ci.yml `supply-chain` advisories + sources into `deny-fuzz.json`; weekly nightly advisories), run from the repo root, where cargo-deny resolves the root `deny.toml` by walking up;
  - boundary review: `cargo modules orphans --deny` per lib/bin target via `scripts/orphans-check.sh` (+ `--probe`) is a CI gate, and the `--acyclic` graph is an on-demand review;
  - the rmcp `cargo tree -e features` check lands with the "MCP server for drivers" chunk;
  - new code-graph planes bullet: rust = the root workspace through rust-analyzer SCIP (every member; `fuzz/` outside); ts = `e2e-web/` only via a tracked `e2e-web/tsconfig.json` (noEmit, strict); nothing under `crates/viola-ui/` is ever covered by a tsconfig, package manifest or bundler config.
- Tree: `tests/cmd/*.toml` (trycmd), `tests/snapshots/`, `crates/viola-ui/assets/` (`index.html`, `app.css`, vendored Lit 3.3.3 ESM), `scripts/release-check.sh`, `scripts/orphans-check.sh`, `e2e-web/` with its 10 entries, `a11y/sr-pass/`, and the updated ci.yml / nightly.yml comments.
- CI/CD:
  - nightly `advisories` covers both lockfiles;
  - Setup steps add the jq consumers and the cargo-modules install route (no taiki-e manifest);
  - 8 jobs wired and 15 check-runs per push: `lint` gains the orphans steps, `supply-chain` the fuzz lockfile audit, and the new `release` job (plain exit-code, no gate step);
  - target job 6 is `cargo build --release --locked --bin viola` through `scripts/release-check.sh`, judged on the build's own artifact records, never a `target/release/` listing.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Files, Symbols/APIs, Schema/config, Code-graph planes, Counts moved, Dev-tool versions, Harness/gate surface; Spec claims disproved 1-3). v1-23's inventory gate read 45 rows / 25 missing before this amendment and 45 / 0 after it. Operator P4 decisions 1-3. The obs §8 upload admissibility was ratified at this wrap's P2.
**Sweep** (cascade step 2, `sweep.py` over the seven masters + CLAUDE.md + `.claude/rules/*` + `.claude/docs/**`; 16 pattern families including `owed` word-bounded, `root Cargo.lock only`, `outside cargo deny`, `module graph review`/`graph on demand`, `acyclic`, `cargo-modules cycle`, `rmcp client in the release graph`, `cargo tree -e features`, `Jobs wired (7)`, `12 check-runs`, bare `cargo build --release`, `jq (G2)` sole consumer, `last step of every job`, nightly advisories without fuzz, unscanned uploads, `No separate test crate`): architecture hits after the apply are all this pass's own new text (`acyclic` :412, `cargo tree` :413, bare `cargo build --release` :528 inside the measured sentence), 0 stale. The known-positive control fired (`acyclic` 6 hits corpus-wide). Leaves re-derived: CLAUDE.md `GENERATED:setup:overview` (Key directories gains the ts plane note and `a11y/sr-pass/`), `.claude/docs/stack.md` (Code quality row recomputed from the arch row: it had lagged cargo-llvm-cov/cargo-fuzz; Testing jq line), `.claude/docs/commands.md` (install, jq, Lint lines 51-55, release build, scip-typescript line).

## 2026-09-24-epoch-1-cleanup — the mutation union is no longer the only red path
**Section:** §Infrastructure Patterns → CI/CD approach (jobs wired today, the `mutants-verdict` sentence)
**Change:** after "a mutant is red only when no leg caught it and some leg missed it or timed out", the body adds that before that union a leg is red at its own run when its unviable mutants outnumber its caught ones (test-plan §3 `run` step 4, §10 Mutation gate).
**Why:** a cross-master citation of test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker). Flagged out of detector scope by the arch doc-agent and folded by the cascade step-2 sweep; the report's Harness/gate surface carries the rule.
**Sweep:** the same pass pattern (`sweep.txt`) found 1 architecture hit, `:515`, amended. There is no other architecture hit: `grep -c` of the union sentence = 1 before the edit.

## 2026-09-25-security-prerequisites — SQOS client open, sha2 content hash, licence MIT OR Apache-2.0, 0BSD exceptions
**Section:** §Stack and Technologies (Wrapper IPC row; new Content hash row); §Established Decisions [Message Broker / IPC]; §Occupied Resources → IPC endpoints (test-only pipe); §Infrastructure Patterns → Build system (shared `[workspace.package]` inheritance, Dependency policy licences, new Licence bullet), Project directory structure (root `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`), Crate dependency direction (`viola-channel` + windows-sys on Windows); §Inherited Defaults → Publishability.
**Change:**
- The Wrapper IPC row and the IPC decision record the Windows client open. It is windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by `Stream::try_from`, never the default connect. A non-overlapped handle hangs, as measured at the chunk.
- A Content hash row names sha2 `=0.11.0` (`default-features = false`) for the 16-hex `<hash>` keys, a root dev-dependency until `viola-state`.
- `viola-channel`'s planned third-party deps gain windows-sys (Windows only).
- The licence `MIT OR Apache-2.0` is recorded:
  - in `[workspace.package]` + `license.workspace = true`;
  - as a literal in `fuzz/Cargo.toml`;
  - by the root texts, and in Publishability.
- Dependency policy records the licence `allow` list and the per-crate `0BSD` exceptions (`doctest-file`, `recvmsg`), never through `allow`.
- The tree gains the three root files.
- The test-only pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` is registered, outside the `viola-<h12>` namespace.
**Why:**
- The chunk's report: Dependencies, Schema / config, Files, Symbols / APIs, Cross-project claims, Expected amendments.
- Founder ruling 2026-09-25 (licence), relayed by the overseer, whose wrap P2 directive item (4) routes the licence to every arch project-metadata site.
- Operator ratification of the 0BSD exceptions (see security-plan sidecar, same marker; boundary widening).
- Orchestrator-raised (Validate check 5): the `[workspace.package]` inheritance line (:403), a metadata site the detector did not propose.
- Rejected: widening the PTY layer row's windows-sys role (:17). That row's claim, the `TerminateProcess` fallback in `viola-pty`, stays true; the SQOS use belongs to the channel and lands in the IPC row and the dependency direction instead.
**Sweep:**
- Cascade step 2 read every `windows-sys` line across the masters (29).
  - architecture :17, :46 and the tree's `viola-pty` line (kill fallback, own ConPTY): true, no change;
  - :19, :51 and :430 amended.
- `viola-channel\` → \`viola-core\`, interprocess\.` = 0 after the apply (1 before).
- `publish = false` metadata sites: :350 and :351 (per-crate registry notes, no metadata restatement) no change; :403, :418 (+ new Licence bullet) and :589 amended.
- Leaves re-derived:
  - `.claude/docs/stack.md` (Wrapper IPC row, Content hash row, licence line);
  - `.claude/docs/conventions.md:55` (`license.workspace = true`);
  - `.claude/docs/services/viola-channel.md` (the resolved spike).
- CLAUDE.md `GENERATED` blocks recomputed from the amended sections: no line changes (none states the IPC open, the hash crate or the licence).

## 2026-09-25-pty-wrapper-on-windows — PTY wrapper as built: R8 prefix rule + identity floor, viola-side program resolution, HostTerminal, PtyError exception
**Section:** Stack and Technologies (PTY layer, Error types) · Established Decisions [PTY], [CLI Version Compatibility] ledger row, [Error Handling], [CI/CD] · Conventions (CLI exit codes, Rust error types) · Occupied Resources (Workspace crates, Environment variables, Filesystem `config.json`, Repository orphans-probes + ripgrep probe dirs) · Infrastructure Patterns (Build system Lint, code-graph rust plane, Licence; Crate dependency direction viola-pty / viola-agent-claude / shared deps / root bin; project tree `viola-pty`) · Cross-cutting Config management (Settings) · Inherited Defaults (Errors)
**Change:**
- R8 is now a `CLAUDE*` prefix rule with a persistent-environment exemption (Windows registry `Environment` value names, Unix `config.json` `claude_env_keep`) and an 11-name identity floor; the ledger row is the identity floor, not a strip list; `config.json` + Settings register `claude_env_keep`.
- [PTY] records the as-built host-terminal raw mode (`HostTerminal`), explicit child cwd, viola-side program resolution and the kept ConPTY input writer; [CI/CD] no longer says the child is "spawned as given"; exit 1 covers the `batch-script-child` refusal.
- `viola-pty` deps as landed: portable-pty, windows-sys (Windows), libc (Unix), no thiserror; `viola-agent-claude` as landed depends on thiserror only; root bin + windows-sys (registry reader). Both crates are registered as landed, with a code-free `fake-agent` feature, in the plane member list and the licence list.
- `PtyError` recorded as the one ratified exception to "thiserror per crate" (hand-written fixed `Display`/`Error`).
- Lint records the root bin's one local `print_stderr` allow (the refusal fn); Repository registers `target/tools/ripgrep/probe-*` and the 2-case orphans probe set.
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Changes (Dependencies, Symbols, Schema / config, Harness / gate surface) and Deviations 1-5; operator ruling 1 (R8) ratified as a boundary widening at wrap P2 (E1); the PtyError exception ratified at wrap P2 (E2). Rejected: A3 (qualifying "environment variables are not a configuration channel" — per E1 the registry read is R8's input, names only, not configuration).
**Sweep:** patterns `14-name|14 names|14 known|14 S6|14 variables|strip list|compile-time list|spawned as given|program as given|stdin pipe|until … seam exists|thiserror … every/one-enum|shared by every workspace crate|writes nothing to the terminal except` over the 7 masters: architecture :553 ("while the child runs, `run` writes nothing … except the child's own output") true, no change; every other architecture hit amended. Leaves re-derived: `.claude/docs/stack.md` (PTY layer, Error types rows), `.claude/docs/conventions.md:15`, `.claude/docs/gotchas.md` (npm shim, parent identity), `.claude/docs/services/{viola-pty,viola-agent-claude,viola}.md`. CLAUDE.md `GENERATED` blocks recomputed: no line changes (none states the strip list, the crates' deps or the error-crate rule).
