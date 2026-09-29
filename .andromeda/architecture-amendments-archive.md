# architecture — archived amendment originals

Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-27-wrapper-channel wrap — 17 re-worded · 0 pruned

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

## 2026-09-26-ci-chunk-base-and-union-verdict — derived whole-chunk mutation base, compiling-leg union, syn/proc-macro2 in viola-e2e
**Section:** Stack and Technologies (Code quality row) · Occupied Resources → Environment variables (`AGENT_RUN_CHUNK_BASE`) · Occupied Resources → Repository (`target/agent-run/`) · Infrastructure Patterns → Crate dependency direction · Infrastructure Patterns → CI/CD approach (concurrency rationale; `test` job `harness-<os>` upload; `mutants` legs; `mutants-verdict` union)
**Change:**
- `AGENT_RUN_CHUNK_BASE` is an explicit override of the derived base; CI does not set it.
- `mutants` legs: step "Mutation leg (whole chunk)", only `LEG` through `env:`, base derived by the harness (last master flip before the oldest pre-CI commit, `HEAD^` on the wrap push), the run document names `base`.
- `mutants-verdict`: the union judges each mutant only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`), every leg when none does.
- `harness-<os>` uploads `target/agent-run/` minus `target/agent-run/chunk.diff`; the Repository row records that `chunk.diff` is the one file `secret-scan` skips.
- The no-concurrency reason keeps the `always()` gate/upload chain and retires the "drop a push's `--in-diff` mutation diff" half (every run now covers the whole chunk).
- syn 2.0.119 + proc-macro2 1.0.107 (`span-locations`) registered in the Code quality row and as `viola-e2e`'s dependency-direction entry.
**Why:** chunk 2026-09-26-ci-chunk-base-and-union-verdict report Dependencies, Symbols/APIs, Harness / gate surface, Expected amendments (architecture ×4).
**Sweep:** the test-plan entry's 12 patterns; architecture rows :364 (edited, true: "an explicit override of the mutation gate's diff base"), :519, :522 amended; no other architecture hit. Cross-master: security-plan :331 restated the retired concurrency reason — amended in this pass. Leaves re-derived: `docs/stack.md:33` (Code quality row), `docs/workflow.md:10`, `docs/commands.md:31/:41`; CLAUDE.md `GENERATED:setup:*` blocks recomputed against the amended sections — no block states the base, the union or the scan scope (0 hits of the 12 patterns in CLAUDE.md), no change. Fanned 8 proposals (6 D-arch-resources, 2 D-arch-decisions; 4 `dependent-of`), all applied with text re-derived from the report.

## 2026-09-26-local-linux-pre-push-gate — local pre-push gate, `FAKE_AGENT_PUMP_DELAY_MS` test seam, `target/pre-push/`
**Section:** §Stack CI/CD row · §Established Decisions [Naming] · §Conventions Environment variables · §Cross-cutting Config management · §Occupied Resources → Environment variables, Repository · §Infrastructure Patterns → Project directory structure, CI/CD approach
**Change:**
- CI/CD row + CI/CD approach: before the operator push, `viola-harness pre-push` runs the ubuntu test suites and the `ubuntu-latest` leg in WSL2 `Ubuntu` (provisioned by `scripts/wsl-provision.sh` from ci.yml's pins), the `windows-2025` leg on the host and CI's own union with equal bases; the operator-pass order (stop rust-analyzer → `pre-push` on the uncommitted tree → pre-CI commit → guarded push → CI reads; red stops the pass); CI stays the verdict of record.
- [Naming], Conventions, Config management, env registry: one ratified exception to the `VIOLA_` / `AGENT_RUN_` rule — the test seam `FAKE_AGENT_PUMP_DELAY_MS`, read only under `cfg(feature = "fake-agent")` (`hold_pump_start`, `src/cmd/run.rs`), u64 ms capped at 5 000, absent from release builds, configures nothing.
- Repository: `target/pre-push/` (the sync's temporary index and binary patch; outside `target/agent-run/` because `secret-scan` scans that tree).
- Project tree: `viola-e2e` names the internal subcommands incl. `pre-push` (`harness::pre_push`); `scripts/wsl-provision.sh`.
**Why:** chunk 2026-09-26-local-linux-pre-push-gate report Symbols/APIs, Harness / gate surface, Files, Expected amendments 7-9, 16-17. The seam is a boundary widening touching a locked decision ([Naming]); ratified by the operator's wrap directive (overseer, founder-delegated): "record the test-only `FAKE_AGENT_PUMP_DELAY_MS` seam as the architecture + security-plan amendments — a carve-out behind cfg(feature=\"fake-agent\"), capped at 5 s, absent from release builds". Rejected at validate (registry over-reach): the WSL host-side state (`~/viola-pre-push`, `~/.rustup`, `~/.cargo`) and the test temp trail `viola-resize-<pid>.ndjson`.
**Sweep:** the test-plan entry's 23 patterns (+4 hand-controlled). Architecture rows :36, :106, :171, :368, :396, :483-484, :492-494, :544, :553 amended; :364 (`AGENT_RUN_CHUNK_BASE` registry) and :529 (the `mutants` job) state the base CI derives, which never meets an uncommitted promotion — true, no change; every other row (resize capability, install steps, registry, tree) true. Leaves: `docs/stack.md` CI/CD row re-mirrored verbatim, `docs/conventions.md` (:13 seam exception), `docs/commands.md`, `docs/workflow.md`, `docs/gotchas.md` re-derived; CLAUDE.md `GENERATED:setup:*` recomputed — no block states a changed fact, no change. Full row list: `.andromeda/runs/2026-09-27T00-51-08-wrap/sweep-dispositions.md`.

## 2026-09-27-instance-state-and-start-order — tempfile `persist` snapshot writer, liveness by process check, start order as landed, `viola-state`
**Section:** §Stack (State-file primitives, Content hash) · §Established Decisions [Snapshot writer], [Session Liveness], [Deployment / Distribution] · §Conventions CLI exit codes (`1`) · §Standard Contracts Instance snapshot, Session liveness · §Occupied Resources Workspace crates, Filesystem (`bin/`), Repository (`target/e2e-home`) · §Infrastructure Patterns Build system (lint, rust plane, licence), Crate dependency direction (`viola-state`, `viola-agent-claude`), CI/CD approach · §Inherited Defaults Database
**Change:**
- [Snapshot writer] + State-file primitives + Database: tempfile 3.27.0 `persist` through ONE shared helper (`viola_state::fs::replace_private`, mode set on the temp file before the write; `replace_private_shared` for identical concurrent writes) replaces atomic-write-file 0.3.1 (BSD-3-Clause, outside `deny.toml`); `.lock` siblings opened for write (Windows refuses the lock on an append-only handle).
- [Session Liveness] + Session liveness + exit `1`: pid + start time decide `gone` whatever the beat; same process → `live` (≤ 5 s) or `stale`; a `live`/`stale` name refuses (`already-live`), a `gone` one is taken over at once; start order as landed (resolution → collision → pin + plugin → [version gate, bind: later] → snapshot + heartbeat → events → spawn → `child_pid` rewrite).
- Instance snapshot: `endpoint?` (from "Wrapper channel"), `child_pid?` (post-spawn rewrite), `started_at` = the wrapper's OS start time.
- [Deployment] + `bin/`: re-hash before reuse, mismatch refuses (`pinned-hash-mismatch`); plugin rewritten every start, `hooks`/`mcpServers` empty until their verbs (operator ruling).
- Content hash: sha2 a `viola-state` product dependency; `[profile.dev.package.sha2] opt-level = 3`.
- `viola-state` landed (sync, 6 crates), in the rust plane and the licence list; its dependency line as landed; agent-claude as-landed symbols + `PLUGIN_DIR_FLAG`, `plugin_files`.
- Lint: the root bin carries no print allow — the start refusals use `writeln!` on the locked stderr through one `refuse` helper (re-derived from the code: `grep allow(clippy::print_std src/` → only the fake agent's crate-level allow; the old "local allow" text was already false).
- `target/e2e-home`: owner record + gone-owner sweep; not kept under `run --mutants`. CI/CD: pre-push Linux-leg `TMPDIR` scratch; `run --mutants` keeps no home.
**Why:** chunk report Changes (Symbols, Dependencies, Crates, Harness, Spec claims 1/2), Expected amendments 1–5; [Snapshot writer] is a locked-decision reversal settled by the operator's P4 ruling and the overseer's wrap directive item (3). Registry proposal A17 (Claude Code integration names) rejected — the names stay true; its fact moved to [Deployment].
**Sweep:** 20 patterns (`runs/2026-09-27T06-12-23-wrap/cascade-patterns.toml`); architecture rows :23, :50, :92, :98, :372, :411 are this pass's text; :146 and :244 true — no change. Leaves re-derived: `docs/stack.md` (:18, :19), `docs/services/viola-state.md` (:6, :11, :20, :38), `docs/services/viola.md` (:20), `rules/events.md` (:29, :31); CLAUDE.md `GENERATED:setup:*` recomputed — no change. Full rows: `runs/2026-09-27T06-12-23-wrap/cascade-sweep.md`.

## Registry migration (U35) — 2026-09-29

<!-- U35 · architecture.md · ## Infrastructure Patterns · sha256 e82620ed08c5b82064e9428abec0019b004890059010727d74c391e856c22efb -->

## Infrastructure Patterns

**Build system**
- Cargo workspace with `resolver = "3"` (the edition 2024 default, which prefers dependency versions compatible with `rust-version = "1.96"`). Shared `version.workspace = true`, `license.workspace = true` (`[workspace.package]` `license = "MIT OR Apache-2.0"`) and `[workspace.dependencies]` pin every third-party version in one place (portable-pty as `=0.8.1`; rmcp minor pinned as `~3.4`).
- Lint: `cargo fmt --all --check` and `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`.
  - `[workspace.lints.clippy]` denies `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits it (`[lints] workspace = true`, asserted by `tests/contract_lints.rs`). `viola-e2e` opts out and denies only `dbg_macro`, because the harness prints its JSON document. The fake agent carries a crate-level print allow. The root bin carries no print allow: its only human writer is `src/human.rs`. On stderr, `write_refusal` writes the two fixed `unable:`/`hint:` lines as ONE `write_all` and `write_internal_error` writes exactly `error: internal error\n`; on stdout, `write_result` writes one line per `write_all`. The `refuse` / `internal_error` / `result` wrappers lock the stream, call the writer and drop the result (a closed pipe is swallowed, never a panic), a form `print_stdout`/`print_stderr` do not flag. Its callers are the `viola run` start refusals (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) and `viola verify` (its refusals, step lines, `stamped` summary and the `cli` role's internal error); `hook` and obs never call it.
  - `clippy.toml` `disallowed-macros` bans `tracing::{info,warn,error,debug,trace}`, so `obs_event!` is the only sanctioned emitter. Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`, because clippy 1.98.1 cannot exempt a macro's inner expansion (obs-plan §3 logger-stack-install).
  - `scripts/lint-probes.sh` proves every ban fires and every control passes, as `deny-probes.sh` does for cargo-deny.
- Typecheck: `cargo check --workspace --all-targets`.
- Dependency policy: `cargo deny check` over `deny.toml` — advisories, licences (`allow` exactly `MIT`, `Apache-2.0`, `Zlib`, `Unicode-3.0`; `0BSD` only through per-crate `[[licenses.exceptions]]` for `doctest-file` and `recvmsg`, the two 0BSD crates interprocess 2.4.4 pulls in, never through `allow`: operator ratification 2026-09-25, security-plan Decisions Log), sources (crates.io only) and bans: C-building crates (`cc`, `libsqlite3-sys`, `openssl-sys`), telemetry crates (`opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender`) and feature bans (`veil/toggle`, `rmcp` `auth` / `transport-streamable-http-server`, `axum/http2`, `tracing-subscriber/env-filter`), evaluated for the Windows, macOS and Linux target triples so `cfg`-gated dependencies are covered. The ban on `tokio` anywhere in the normal dependency graph, direct or transitive, of `viola-core`, `viola-pty`, `viola-channel` without its feature, `viola-state` and `viola-agent-claude` lives in `deny-sync.toml` (the same three triples, `exclude-dev = true`) and runs once per crate listed in `scripts/sync-crates.txt`, that crate as the sole root: `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans`. Never a direct-parent allowlist (`wrappers`), because Tokio could otherwise arrive unnoticed through a third-party feature such as interprocess's or notify's; never `--exclude` over the workspace, which false-fails under feature unification — a shared crate's optional `tokio` feature stays on after the crate enabling it is excluded (as measured at chunk 2026-09-24-supply-chain-and-workflow-gates, research.md §Measured facts, cargo-deny 0.19.4 and 0.20.2). Limit, same measurement: the crate owning the optional `tokio` feature (`viola-channel`) false-fails as its own root once `viola-mcp` enables that feature (`cargo metadata` reports the unified set), so it is covered through the sync roots that depend on it. Every ban is proven live by `scripts/deny-probes.sh`: a throwaway project per ban must fail with that ban's own diagnostic, and a clean control must pass both configs. `fuzz/` has its own `Cargo.lock` outside this graph, and its libfuzzer-sys builds C++ via `cc`: the only C build in the repository. The operator ratified this on 2026-09-24 as a test-only exemption. The crate is never linked into `viola`, and `fuzz` never joins the root `[workspace]`. Its lockfile is audited separately: the CI `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, and the weekly nightly `advisories` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`. Both run from the repo root: cargo-deny resolves the root `deny.toml` by walking up from the manifest, and under `fuzz/` rustup would demand the fuzz nightly (measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes).
- Boundary review:
  - `cargo modules orphans --deny` is a CI gate. `scripts/orphans-check.sh` runs it in the per-OS `lint` job once per lib/bin target from `cargo metadata --no-deps`, with `--features` for a target's required features (`viola-fake-agent`), and its `--probe` mode proves it fires on a planted orphan first.
  - The `cargo modules dependencies` graph (`--acyclic`) stays an on-demand review, never a gate: cargo-modules 0.27.0 reports every type and its own inherent method as a cycle, whatever the filters (measured at the same chunk, 3 of 4 targets).
  - The rmcp release-graph check (`cargo tree -e features -p viola --edges normal`, rmcp exactly `server` + `transport-io`) has no subject until `viola-mcp` exists; it lands with the "MCP server for drivers" chunk.
- Code-graph planes (`scripts/code-graph.py`, one DuckDB per language plane under `.andromeda/cache/{plane}/`):
  - **rust:** the root workspace through rust-analyzer SCIP. It indexes every member (`viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-e2e`). `fuzz/`, a separate workspace, is outside it.
  - **ts:** `e2e-web/` only, from its first chunk, through a tracked `e2e-web/tsconfig.json` (noEmit, strict) and scip-typescript. No tsconfig, package manifest or bundler config ever covers `crates/viola-ui/` (the page has no JS build step); a CI-runnable guard lists any such file.
- Install: `cargo install --path .` from the repo root.
- Publishing: every workspace member, including the root `viola` bin, sets `publish = false`. Nothing goes to crates.io in v1.
- Licence: the project is `MIT OR Apache-2.0` (founder ruling 2026-09-25). `license` is set once in root `[workspace.package]` and inherited with `license.workspace = true` by `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state` and `viola-e2e`; `fuzz/Cargo.toml`, its own workspace, carries the literal. The texts are `LICENSE-MIT` (holder Turbolet85) and `LICENSE-APACHE` at the root, and `README.md` names both.

**Deployment model**
- Local-only. There is no Docker, Compose, Kubernetes or serverless.
- The single `viola` binary is installed on PATH. `viola run <name> -- claude <args>` pins a copy of itself in `~/.viola/bin/<version>-<hash>/` (on Windows x64 also the embedded ConPTY companions in its `conpty/` subdirectory, with `conpty.dll` pre-loaded from there before the spawn), writes the embedded plugin (pointing at that copy) to `~/.viola/plugin/<version>-<hash>/` and launches the child with `--plugin-dir`. `viola ui` is started by hand and serves 127.0.0.1.

**Crate dependency direction** (enforced by manifests)
- `viola-core` depends on no viola crate. Third-party: nutype (`ViolaName`, `Percent`).
- `viola-pty` depends on no viola crate and knows no agent. Third-party (as landed): portable-pty, tracing (the `pty.spawn` span), windows-sys (Windows only), libc (Unix only; termios raw mode and the terminal size). No thiserror ([Error Handling]).
- `viola-channel` → `viola-core`, interprocess, serde, serde_json, thiserror, tracing, veil, windows-sys (Windows only: the SQOS client open adopted by `Stream::try_from`, and the listener DACL through `Win32_Security_Authorization`), libc (Unix only: the uid for the socket directory). As landed it is sync and tokio-free, with a sync client and server; the Tokio client arrives behind the `tokio` feature with `viola-mcp`.
- `viola-state` → `viola-core`, chrono, serde, serde_json, sha2 (the pinned-copy key and re-hash), sysinfo (the pid + start-time liveness check, shared by CLI `list`, `mcp` and `ui`), tempfile (the one `persist` helper), thiserror, tracing, and notify with tailing.
- `viola-agent-claude` → `viola-core`, `viola-state`, serde_path_to_error (drift reports on external payloads), vt100 (the screen model and signatures behind the readiness gate; `run`'s pump feeds it bytes). This is the only crate that knows Claude payload shapes, including `claude agents --json`. As landed it depends on `viola-core`, serde, serde_json, serde_path_to_error `=0.1.20` (the `hook` module: `HookEvent`, `normalise`, drift reports) and thiserror (`IDENTITY_FLOOR`, `plan_strip`, `resolve_program`, `Refusal`, `PLUGIN_DIR_FLAG`, `plugin_files` over the `include_str!`-embedded `plugin/` templates, and the pure `ledger` module: the six rows, `parse_version`, `capture_plugin_files`, `check`, `merge_stamp`, `verified`, `scrub`/`is_clean`; `CASE_INSENSITIVE` and `is_script` are `pub` for `verify`), with dev-dependencies proptest and rstest. The ledger chunk added no `viola-state` dependency: the stamps file I/O stays in `viola-state` and the root bin, so the crate stays pure. vt100, and `viola-state` if a later consumer needs it, arrive with the chunks that consume them.
- `viola-mcp` → `viola-core`, `viola-channel[tokio]`, `viola-state`, `viola-agent-claude`, rmcp, schemars, tokio.
- `viola-ui` → `viola-core`, `viola-state`, `viola-agent-claude`, axum, tower-http, tokio.
- serde, serde_json, chrono and thiserror are shared through `[workspace.dependencies]` by every workspace crate except `viola-pty`, which takes none of them. The sync crates (`viola-core`, `viola-state`, `viola-agent-claude`) list chrono directly, because they do not reach it through rmcp.
- `viola-e2e` (test-only; no product crate depends on it); its dependencies are not listed here.
- The `viola` root bin → all members, clap, anyhow, windows-sys (Windows only: the registry reader for the R8 persistent-environment names in `src/run/env.rs`, plus `Win32_System_Diagnostics_Debug` and `Win32_System_LibraryLoader` for `src/panic_frames.rs`'s `RtlCaptureStackBackTrace` / `GetModuleHandleExW` / `GetModuleFileNameW`) and libc `=0.2.189` (Unix only: `backtrace` + `dladdr` for the raw, never-symbolised panic frames). It contains the subcommand dispatch, the `run` pump, the wheel and the budget governor, all of which consume only normalised events.

**Project directory structure**
```
viola/
├── Cargo.toml                  # [package] viola (bin) + [workspace] members = ["crates/*"], exclude = ["fuzz"]
├── Cargo.lock
├── rust-toolchain.toml         # channel = "1.98.1" (exact pin), components = ["rustfmt", "clippy"]
├── deny.toml                   # cargo-deny: advisories, licences, sources, bans (C, telemetry, features)
├── deny-sync.toml              # the tokio ban, run per scripts/sync-crates.txt crate as sole root
├── clippy.toml                 # disallowed-macros: tracing::{info,warn,error,debug,trace}
├── .gitignore
├── LICENSE-MIT                 # MIT text (holder Turbolet85); the project is MIT OR Apache-2.0
├── LICENSE-APACHE              # the standard Apache License 2.0 text
├── README.md                   # description + `## License` naming both licence files
├── plugin/                     # embedded via include_str!, written out by `viola run`
│   ├── .claude-plugin/plugin.json
│   ├── hooks/hooks.json        # exec-form commands, placeholder for the pinned bin copy
│   └── .mcp.json
├── src/                        # the `viola` bin: anyhow edge only
│   ├── main.rs                 # clap 4.6.7 dispatch (Windows: the System32 DLL-search restriction is its second statement)
│   ├── conpty.rs               # Windows x64: the embedded ConPTY companions and their four pins (the vendor script parses this text)
│   ├── human.rs                # human-facing text: the refusal and internal-error stderr writers and the stdout result writer, called by `run` and `verify`
│   ├── cmd/                    # one module per subcommand: run, send, wait, last, list,
│   │                           #   answer, hook, mcp, ui, verify, pause, release, link, unlink, plugin
│   ├── run/                    # PTY pump, wheel, budget governor, readiness gate wiring
│   └── bin/viola-fake-agent.rs # test-only stand-in `claude` (feature `fake-agent`)
├── tests/                      # root integration tests (sync)
│   ├── cmd/*.toml              # trycmd cases: human-mode expected output (snapbox redactions)
│   ├── snapshots/              # insta snapshots (check mode only)
│   └── support/                # the sync root fixture chain + fixture-hygiene checker
├── schemas/                    # JSON schemas: fake-script.v1.json and claude-fixture.v1.json (test-side), diag-line/diag-detail.v1.json (obs line contracts)
├── crates/
│   ├── viola-core/             # normalised events, RefusalReason, ViolaName, Percent, `v` constants
│   │                           #   (+ proptest-regressions/, committed seeds)
│   ├── viola-pty/              # pty seam over portable-pty =0.8.1 (+ windows-sys kill fallback; HostTerminal raw mode: windows-sys Console / libc termios;
│   │                           #   `sideload` (Windows): the System32 DLL-search restriction + the absolute-path conpty.dll pre-load)
│   ├── viola-channel/          # JSON-RPC 2.0 ndjson over interprocess local sockets
│   ├── viola-state/            # ndjson logs, atomic snapshots, File::lock, torn-line healing, tailing
│   ├── viola-agent-claude/     # hook parsing, dialog mapping, R8 strip, shim resolution,
│   │                           #   capability ledger, screen signatures, statusline parsing
│   │                           #   (+ proptest-regressions/, committed seeds)
│   ├── viola-mcp/              # rmcp 3.4.1 stdio server, thin adapter over viola-channel
│   ├── viola-ui/               # axum 0.8.9 GET routes + SSE, Host allowlist
│   │   └── assets/             # embedded page, no JS build step and no tsconfig: index.html, app.css
│   │                           #   (the single stylesheet), vendored Lit 3.3.3 ESM
│   └── viola-e2e/              # test-only: viola-harness (agent-run boot/run/status/cleanup/logs, plus the
│                               #   internal subcommands incl. `gate` and `pre-push`: harness::pre_push)
├── scripts/
│   ├── agent-run.{sh,ps1}      # identical shims over viola-harness
│   ├── conpty-vendor.sh        # re-vendor vendor/conpty/ from the pinned nupkg (+ --verify: sha256 + byte compare + signer; --probe)
│   ├── sync-crates.txt         # the single sync-crate list (CI job 3 + the sole-root tokio ban)
│   ├── deny-probes.sh          # negative probe per cargo-deny ban + a clean control
│   ├── lint-probes.sh          # each clippy ban fires, controls pass, fail-closed raw-event! grep
│   ├── release-check.sh        # target job 6: the release build carries `viola` only, no test-only feature (+ --probe)
│   ├── orphans-check.sh        # cargo modules orphans --deny per lib/bin target (+ --probe)
│   ├── g2-zero-panics.sh       # obs G2: 0 `event:"panic"` role lines under target/e2e-home, exempting only a
│   │                           #   `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>` (fail-closed; + --probe)
│   ├── install-ripgrep.sh      # pinned, sha256-verified ripgrep 15.2.0 → target/tools/ripgrep
│   ├── install-node.sh         # <os-key> <dest>: the official Node build at ci.yml's NODE_PIN_* (parsed from the
│   │                           #   file text), sha256-verified, flattened into <dest> (+ --probe)
│   ├── npm-audit.sh            # e2e-web lockfile: npm audit (every level) + registry.npmjs.org-only sources
│   │                           #   → target/npm-audit/ (+ --advisories-only, --probe)
│   ├── wsl-exec.sh             # operator aid only: [--cd DIR] CMD … through `wsl.exe -d Ubuntu --exec env -i` with the
│   │                           #   distro's HOME and PATH (argv unconverted; --probe); no gate/harness/plan runs a command through it
│   └── wsl-provision.sh        # in-distro WSL provisioning for `pre-push`: sha256-pinned rustup-init 1.29.1,
│                               #   rust-toolchain.toml, `cargo install --locked` of ci.yml's test-job pins, the pinned
│                               #   Node and the locked Playwright's Chromium (+ --check, --probe; --install-deps: uid 0,
│                               #   operator-only)
├── vendor/conpty/<version>/x64/ # the committed Microsoft conpty.dll + OpenConsole.exe (binary per .gitattributes)
├── .config/nextest.toml        # nextest profiles `ci` and `mutants`, `fixed-port` group
├── fuzz/                       # separate cargo-fuzz workspace (own Cargo.lock; excluded from the root)
│   ├── rust-toolchain.toml     # channel = "nightly-2026-09-20" (fuzz only)
│   ├── fuzz_targets/{viola_name,channel_frame,hook_stdin}.rs
│   └── corpus/<target>/        # committed synthetic seeds
├── fixtures/
│   ├── claude/<cli-version>/   # hook-payload fixtures recorded by `viola verify`
│   └── fake-scripts/           # committed fake-agent turn scripts (synthetic)
├── e2e-web/                    # test-side Node only (Playwright; axe and the a11y lint land with the a11y chunks);
│   │                           #   the ts code-graph plane
│   ├── package.json            # pins @playwright/test 1.63.0 (exact; @axe-core/playwright lands with the a11y chunks)
│   ├── package-lock.json       # committed; audited by scripts/npm-audit.sh
│   ├── playwright.config.ts    # headless chromium, retries 0, forbidOnly, reporters pw.json + pw-junit.xml
│   ├── tsconfig.json           # noEmit, strict, e2e-web/** only
│   ├── stub/pipe.html          # the file:// reachability stub (one <h1>, no script or style)
│   ├── eslint.config.js        # eslint-plugin-lit-a11y over the crates/viola-ui Lit sources
│   ├── .htmlvalidate.json      # html-validate over the embedded assets/index.html
│   ├── tests/*.spec.ts         # one spec per bay layout type; today the pipe stub's pipe-reachability.spec.ts
│   ├── fixtures/a11y.ts        # the shared makeAxeBuilder fixture
│   ├── schemas/a11y-row.v1.json  # tests-owned a11y violation-row schema (not obs schemas/)
│   ├── a11y/sc-coverage.json   # per-SC coverage map
│   └── test-results/           # gitignored outputs (a11y/, lint/)
├── a11y/
│   └── sr-pass/                # manual screen-reader passes: TEMPLATE.json, <date>-<at>.json
├── .github/
│   └── workflows/
│       ├── ci.yml              # push + PR: 3-OS test (+ the browser suite)/perf (hyperfine rows + gate --require
│       │                       #   perf)/lint (+ module orphans), msrv, fuzz-replay, 3-OS release
│       │                       #   (release-check), supply-chain (+ fuzz
│       │                       #   lockfile audit, npm lockfile audit); the workflow env holds the NODE_PIN_* lines
│       └── nightly.yml         # weekly schedule + workflow_dispatch: cargo deny check advisories (root + fuzz/Cargo.lock),
│                               #   npm-advisories (npm-audit.sh --advisories-only) + fuzz time-box
├── refs/                       # brief and prior-art survey (arch input)
└── .andromeda/                 # pipeline runs and cache
```

**CI/CD approach**
- GitHub Actions, two workflows. `ci.yml`, triggered on push and pull request, with matrix `os: [windows-2025, macos-latest, ubuntu-latest]` on native runners, is the only push/PR pipeline. `nightly.yml`, triggered by a weekly `schedule` and `workflow_dispatch` (both fire from the repository's default branch), runs three ubuntu jobs with no cache. `advisories` runs `cargo deny check advisories` over the root lockfile and `cargo deny --manifest-path fuzz/Cargo.toml check advisories` over the fuzz lockfile, because the advisory DB moves without code changes; `npm-advisories` does the same for `e2e-web/package-lock.json` (`Node (pinned)`, then `scripts/npm-audit.sh --advisories-only`). `fuzz` runs `cargo +<channel> fuzz run --fuzz-dir fuzz <t> fuzz/corpus/<t> -- -max_total_time=120` per target (the channel comes from `fuzz/rust-toolchain.toml`) and uploads `fuzz/artifacts/` on `failure()`. Neither workflow has a `concurrency:` block. zizmor's pedantic `concurrency-limits` was declined because a concurrency group cancels pending runs, which would drop a cancelled run's `always()` gate and upload chain. (CI runs no mutation job — mutation testing moved to the epoch-boundary code audit on 2026-09-28 — so a cancelled run cannot lose mutation coverage; that half of the original reason is retired.)
- Least privilege, in every workflow: `permissions: {}` at the workflow top and `contents: read` per job; every `uses:` pinned by full commit SHA with a version comment; event-payload values reach a step only through `env:`. zizmor asserts it.
- Setup steps: `actions/checkout` v7.0.1 (`persist-credentials: false`), `rustup toolchain install` (reads `rust-toolchain.toml`: the exact pin plus rustfmt and clippy; no toolchain action), `Swatinem/rust-cache` v2.9.2 (`ci.yml` only), and `taiki-e/install-action` v2.87.19 for the version-pinned cargo tools (cargo-deny 0.20.2 included); zizmor 1.30.1 is installed by `cargo install --locked zizmor@1.30.1` in the job that runs it. ripgrep 15.2.0 (G1/G3) comes from `scripts/install-ripgrep.sh` in the `lint` job, because taiki-e/install-action has no ripgrep manifest: the official release asset, checked against its published sha256 and exported on `PATH`. hyperfine 1.20.0 is installed by `cargo install --locked hyperfine@1.20.0` as its own step in the `perf` job (the zizmor precedent; not on the taiki-e line, so the WSL provisioning replays nothing new). `jq` is runner-provided and never installed: the `test` and `perf` jobs presence-check it (`jq --version`) before G2, whose `scripts/g2-zero-panics.sh` refuses with `tool-missing: jq` without it, and `scripts/release-check.sh` (`release`), `scripts/orphans-check.sh` (`lint`) and `scripts/npm-audit.sh` (`supply-chain`, which also runs `jq --version` first) refuse with `tool-missing: jq` without it. cargo-modules 0.27.0 is installed by `cargo install --locked cargo-modules@0.27.0` in the `lint` job, because the pinned taiki-e/install-action has no cargo-modules manifest. The `test` job adds `rustup component add llvm-tools-preview` and `cargo-llvm-cov@0.9.1` on the taiki-e line. `msrv` runs `rustup toolchain install 1.96 --profile minimal`. The fuzz jobs run `rustup toolchain install` in `working-directory: fuzz` (reading `fuzz/rust-toolchain.toml`) and `cargo install --locked cargo-fuzz@0.13.2`. Node v24.21.0 comes from `scripts/install-node.sh` in the `test` and `supply-chain` jobs (and nightly `npm-advisories`), the install-ripgrep precedent with no setup-node action: the official nodejs.org archive for the OS, checked against its `NODE_PIN_SHA256_*` line, extracted to `$RUNNER_TEMP/node` and exported on `PATH`. The `test` job then runs `npm ci --prefix e2e-web` and the Playwright Chromium install per OS (`--with-deps` on ubuntu). No npm or browser cache.
- Jobs wired today in `ci.yml` (7; 15 check-runs per push, as measured at ci#36483042659 on `17b93c7`): `test` (per OS; on `windows-2025` only, before the coverage run, the `ConPTY vendor verification` step runs `bash scripts/conpty-vendor.sh --verify` then `--probe` (verdicts `conpty-vendor: verified <version>` and `conpty-vendor probe: 4/4 refused, control clean`); then one harness `run --coverage` through the OS's shim, i.e. the instrumented nextest run as suite `coverage` with the per-OS floors 85/95/80, followed by doctest, then the boot → status → logs → cleanup lifecycle, then the browser suite (`Node (pinned)`, `npm ci`, the Chromium install, and `agent-run run --browser` through the OS's shim: suite `playwright`); `AGENT_RUN_KEEP_HOMES=1`; then, in obs-plan §9 order: `jq --version`, G2 (`scripts/g2-zero-panics.sh --probe && scripts/g2-zero-panics.sh`), G4 `schema-check` (`id: schema-conformance`), the `if: failure()` harness capture, and `secret-scan` (`id: secret-scan`, `if: always()`). The uploads are gated on that scan: `diag-<os>` and `junit-<os>` on `always() && steps.secret-scan.outcome == 'success'`, `harness-<os>` (`target/agent-run/` minus `target/agent-run/chunk.diff`, the file the scan skips) on `failure() && … == 'success'`, and `secret-scan-<os>` on `always() && … == 'failure'`, all with actions/upload-artifact v7.0.1 and 7 days (`junit-<os>` carries `target/nextest/ci/junit.xml` and `target/agent-run/artifacts/junit-playwright.xml`); last, an `if: always()` `Gate verdict` step runs `gate --require coverage,doctest,playwright`). `perf` (per OS, `fail-fast: false`, `contents: read`, `AGENT_RUN_KEEP_HOMES=1`; test-plan §9 Perf row) runs checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `agent-run run --perf`, then G2 (probe then check), `schema-check` and `secret-scan` (`id: secret-scan`), all `if: always()`; it uploads `perf-<os>` (`target/agent-run/artifacts/perf-*.json`) and `diag-perf-<os>` (`target/e2e-home/**/diagnostics/*.ndjson`) only on `always() && steps.secret-scan.outcome == 'success'`, and `secret-scan-perf-<os>` on a scan failure; last, an `if: always()` `Gate verdict` step runs `gate --require perf`. `test`, the pre-push and the WSL provisioning carry no perf step. CI runs no mutation job: mutation testing runs only through `agent-run run --mutants` [`--file`], kept for the epoch-boundary `/andromeda-code-audit` (test-plan §10 Mutation gate). `msrv` (ubuntu, rust-cache `key: msrv`) runs `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`. `fuzz-replay` (ubuntu, no cache) runs `agent-run run --fuzz-replay`, then `gate --require fuzz-replay`. `lint` (per OS: target job 3, then target jobs 1 and 2, the ripgrep install, `rg --pcre2-version`, G1 and G3 verbatim from obs-plan §9, `scripts/lint-probes.sh` on Linux, then `Install cargo-modules` and `Module orphans (per lib/bin target)`: `scripts/orphans-check.sh --probe && scripts/orphans-check.sh`). `release` (per OS, `fail-fast: false`, rust-cache: target job 6) runs `scripts/release-check.sh --probe`, then `scripts/release-check.sh`; like `lint` it is a plain exit-code job, with no upload and no `viola-harness gate` step. `supply-chain` (ubuntu) runs:
  - target job 4 as `cargo deny --format json check`;
  - the `Fuzz lockfile audit (advisories, sources)` into `target/supply-chain/deny-fuzz.json`;
  - `Node (pinned)`, then the `npm lockfile audit (advisories, sources)`: `scripts/npm-audit.sh` over `e2e-web/package-lock.json` (0 advisories at `--audit-level=low`, every `packages[].resolved` from `https://registry.npmjs.org/`), its JSON in `target/npm-audit/`, which is not uploaded;
  - the sole-root `deny-sync.toml` tokio ban per listed sync crate;
  - `scripts/deny-probes.sh` and `zizmor --format=json .github/workflows/`.

  Its JSON reports are uploaded from `target/supply-chain/` as artifact `supply-chain` with `if: always()`. Every gate step is fail-closed `shell: bash`.
- Target jobs per OS, each wired by the chunk that owns it (Supply-chain gates, Quality gates, Workspace tree):
  1. `cargo fmt --all --check`
  2. `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` (workspace lint bans and `clippy.toml` in force)
  3. `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (target set `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`; each joins the list with its crate; an empty list or an absent package fails) — proves the sync crates, including the `hook` path, compile on each OS without `viola-channel`'s `tokio` feature; the ban itself is the sole-root `deny-sync.toml` step of job 4
  4. `cargo deny check` (once, on ubuntu)
  5. the workspace test suite against the fake agent, replaying `fixtures/claude/*`
  6. `cargo build --release --locked --bin viola` through `scripts/release-check.sh` (the `release` job): it judges that build's own `compiler-artifact` executables, fails on any other than `viola` (`viola-harness`, `viola-fake-agent`) or on none, fails first on any `compiler-artifact` record whose `features` hold the test-only `test-support` or `fake-agent` (`release-check: FAILED — test-only feature {feature} in {target}`; `--probe` reads `5/5 refused, control clean`), and never lists `target/release/`, where a shared or cached dir keeps stale test-only exes (measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes: a bare `cargo build --release` and `--bin viola` yield `viola` only, `--workspace` adds `viola-harness`)
- The real `claude` CLI runs only locally, never in CI. `viola verify` runs in CI only against the fake agent's `-p/--print` mode (`tests/cli_verify.rs`); its real-CLI probe and `--record` run only locally.
- A local pre-push gate precedes every operator push. The operator pass runs, in order: `viola-harness pre-push` on the uncommitted tree → the pre-CI commit → the guarded push → the CI reads; a red `pre-push` stops the pass. `pre-push` runs on the Windows dev host only (elsewhere it refuses `pre-push-windows-only`, exit 2): it syncs the working tree into a history-carrying clone in WSL2 `Ubuntu` (the clone's tree id must equal the Windows tree id), checks the Linux tools there against their pins (the C linker, the channel, ci.yml's cargo tools and `node --version` = `v<NODE_PIN_VERSION>`, refusing `tool-missing` / `tool-pin-mismatch` with detail `node`), runs the ubuntu `test` job's `run --coverage`, `run --browser` (a red stops before the gate) and `gate --require coverage,doctest,playwright` there, terminates the WSL VM once the ubuntu verdict is on the host (`vm-release`, so its memory returns to the host), then runs `run --coverage` + `gate --require coverage,doctest` on the host (`windows-tests`): stages `tools → sync → cache → linux-tests → vm-release → windows-tests`, `ok:true` when `windows-tests` is green, and no mutation stage (test-plan §3 Internal harness subcommands). Every WSL call is `env -i HOME=… PATH=…` with no further assignment; the `cache` section reports the clone's `target/` as `bytes`, `cap`, `cleaned` and `bytes_after`. The host stages run with `CARGO_BUILD_JOBS=16`, set by the harness. `run --mutants` itself sets `AGENT_RUN_KEEP_HOMES=0` and `AGENT_RUN_KEEP_FAILED=0` on the `cargo mutants` command wherever `run --mutants` runs (a mutation run keeps no test home). It is a filter before the push; CI's run on the pushed sha stays the verdict of record.
- There is no deploy stage in v1. The v1.x release path adds a dist 0.33.0-generated release workflow with cargo-auditable 0.7.6.
