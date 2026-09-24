# Report — 2026-09-24-workspace-tree-and-code-graph-planes

**Chunk:** workspace tree + code-graph planes: arch tree lists test/obs/a11y artifacts (v1-23), release build free of test binaries, Rust plane ok, TypeScript plane decided, fuzz lock audit, cargo tree/modules decision, quality-gates CI witness
**Date:** 2026-09-24T16:25:50Z
**Commits:** none since last_wrap 2026-09-24T15:08:00Z. `git log --oneline 3f385dd..HEAD` is empty, so this chunk rides the wrap commit.

## Changes (structured — detectors read this)
- **Files** (`git status --short`: 2 modified + 2 new source/CI paths; no `.rs`, `Cargo.toml` or `Cargo.lock`):
  - modified: `.github/workflows/ci.yml`, `.github/workflows/nightly.yml`;
  - new: `scripts/release-check.sh`, `scripts/orphans-check.sh`;
  - new chunk-folder data: `viola-0.1.0/chunks/2026-09-24-workspace-tree-and-code-graph-planes/artifact-inventory.tsv` (45 rows, header `token	path	owner	arch-home`);
  - bookkeeping riding the commit: route, master, matrix, friction log, the chunk folder and the run dirs.
- **Symbols / APIs:** no Rust symbol changed (the mutants gate read `"files":41,"tested":0,"verdict":"no-rust-delta"`). The new script surfaces:
  - **`scripts/release-check.sh [--probe]`**
    - Default mode runs `cargo build --release --locked --bin viola --message-format=json` and judges the build's own `compiler-artifact` records whose `executable` is non-null. It never lists `target/release/`.
    - Verdict lines: `release-check: viola only` (exit 0); `release-check: FAILED — test-only binary <name>`; `release-check: FAILED — no executable in the build output`; `release-check: FAILED — cargo build exited N` (exit 1).
    - `--probe` feeds synthetic records and prints `release-check probes: 3/3 refused, control clean`. The three refused inputs are viola+viola-harness, viola+viola-fake-agent and an empty stream; the control is a lib record, viola and build-finished.
    - `tool-missing: jq` exits 1; any other argument is usage, exit 2.
    - It honours an inherited `CARGO_TARGET_DIR` and never sets it.
  - **`scripts/orphans-check.sh [--probe]`**
    - Default mode runs `cargo modules orphans -p <pkg> {--lib|--bin <name>} [--features <required-features>] --deny` for every lib/bin target from `cargo metadata --no-deps`. At HEAD that is 5 targets: `viola-core` lib, `viola-e2e` lib, `viola-harness` bin, `viola` bin, and `viola-fake-agent` bin with `--features fake-agent`.
    - Verdicts: `orphans-check: N/N targets clean`; `orphans-check: FAILED — <pkg>/<target> …`; `orphans-check: FAILED — no lib or bin target in the workspace`.
    - `--probe` writes two throwaway crates, each with an empty `[workspace]`, under `target/orphans-probes/run-<utc>-<pid>/`. The planted `src/stray.rs` must produce ``orphaned module `stray` ``, and the control must print `No orphans found.`. The verdict is `orphans-check probes: 1/1 fired, control clean`.
    - `tool-missing: cargo-modules` or `tool-missing: jq` exits 1.
  - **Env vars:** none new. `CARGO_TARGET_DIR` is cargo's own.
  - **Ports / sockets:** none.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none. The root and fuzz lockfiles are unchanged (`git diff --stat -- Cargo.lock fuzz/Cargo.lock` is empty).
- **Schema / config:**
  - `target/supply-chain/deny-fuzz.json` is a new CI report file (cargo-deny JSON diagnostics over `fuzz/Cargo.lock`). It rides the existing `supply-chain` artifact upload.
  - New local gitignored dirs (`git check-ignore -q` exit 0 each):
    - `target/orphans-probes/run-<utc>-<pid>/` (probe crates);
    - `target/release-check/` (the gate entry's `CARGO_TARGET_DIR`);
    - research-only `target/release-probe/`, `target/tool-build/` and `target/modules-probe-research/`.
- **Code-graph planes** (`scripts/code-graph.py` unchanged; detection per its `detect_planes`: rust = root `Cargo.toml` + rust-analyzer SCIP, ts = a tracked-or-unignored `tsconfig.json` + scip-typescript):
  - **rust plane**, measured at phase P3 (trace `tree-query-2026-09-24-workspace-tree-and-code-graph-planes.json`, `db_state: fresh`): viola 341 symbols / 15 files, viola-core 70 / 2, viola-e2e 501 / 16. This equals the tracked `.rs` counts (`git ls-files` per crate). `fuzz/` has 0 symbols: it is a separate workspace outside the plane.
  - This wrap's P4 refresh read `tree-refresh[rust]` again.
  - **ts plane:** dormant at HEAD (0 tracked `tsconfig.json`, 0 tracked `.ts`/`.js`). Its scope is the operator's P4 decision 2.
- **Spec-master edits:** none by /implement (the wrap applies them below).
- **Counts / qualifiers moved:**
  - `ci.yml` jobs go from 7 to 8 (+ `release`, a 3-OS matrix); read from the ci.yml job keys: test, mutants, mutants-verdict, msrv, fuzz-replay, lint, release, supply-chain.
  - Check-runs per push go from 12 to 15 (+3 `release (…)` legs).
  - The `lint` job gains 2 steps; `supply-chain` gains 1; nightly `advisories` gains 1 line.
  - The pinned third-party action set is unchanged at 5 (the gate SHA probe printed `0` unpinned).
- **Dev-tool versions:**
  - `cargo-modules` (the module-graph / orphans checker) was upgraded on the dev host from 0.26.0 to 0.27.0, the CI pin, on 2026-09-24 at phase P3: `cargo install --locked`, with `CARGO_TARGET_DIR=target/tool-build`.
  - In CI, cargo-modules 0.27.0 is new in the `lint` job (all 3 OS), through `cargo install --locked` (taiki-e v2.87.19 has no cargo-modules manifest: `gh api …/manifests/cargo-modules.json?ref=7623a79c…` returns 404).
  - None changed and re-read: cargo-deny 0.20.2, zizmor 1.30.1, jq 1.8.1 (dev host).
  - Runner-image readmes list jq 1.8.1 on windows-2025, 1.8.2 on macos-15 arm64 and 1.7 on ubuntu-24.04.
- **Harness / gate surface:**
  - **`ci.yml` `lint` (3 OS)** appends `Install cargo-modules` (`cargo install --locked cargo-modules@0.27.0`) and `Module orphans (per lib/bin target)` (`bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh`, `shell: bash`). The existing steps and their order are untouched.
  - **New `ci.yml` `release`:** a 3-OS matrix (windows-2025, macos-latest, ubuntu-latest), `fail-fast: false`, `contents: read`. Steps: checkout, `rustup toolchain install`, rust-cache, `release-check.sh --probe`, then `release-check.sh`. This is architecture target job 6. It has no upload and no `viola-harness gate` step (a plain exit-code job, like `lint`).
  - **`ci.yml` `supply-chain`:** a new step after `Dependency policy`, `Fuzz lockfile audit (advisories, sources)`: `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json`. It runs from the repo root; under `fuzz/`, rustup would demand the fuzz nightly.
  - **`nightly.yml` `advisories`** adds `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
  - No new `viola-harness` subcommand, suite value or gate token.
- **Cross-project / external claims:**
  - **CI run 36019646063 on sha `3f385ddf55da967cd0a39eda67eaf5f8d07556a2`** (2026-09-24-quality-gates' push), read at phase P3 through `gh api repos/Turbolet85/viola/commits/{sha}/check-runs`:
    - all 12 check-runs are `completed success`;
    - `mutants-verdict-ubuntu-latest` has 118 mutants (114 caught, 3 unviable, 1 missed, `run.rs:106 fuzz_host_supported -> true`), and all 3 `secret_scan.rs:225:5 file_mode` mutants are `caught`;
    - `mutants-verdict-windows-2025` has `file_mode -> None` missed, `Some(0)`/`Some(1)` caught, `fuzz_host_supported -> true` caught and `-> false` missed;
    - the union job is `success`, and the `msrv` log line 295 reads `rustc 1.96.1 (31fca3adb 2026-06-26)`;
    - the overseer relayed the same and directed that it be recorded as this chunk's witness of that chunk.
  - GitHub runner-images readmes (`gh api repos/actions/runner-images/contents/images/{windows/Windows2025,macos/macos-15-arm64,ubuntu/Ubuntu2404}-Readme.md`): jq present on all three (lines 71, 62, 84).
  - taiki-e/install-action at `7623a79c…`: `manifests/cargo-modules.json` → 404.
- **Reverted / negative API facts:**
  - The first form of the viola-ui build-free guard, `git ls-files … 'crates/viola-ui/**/tsconfig*.json'`, was written at phase P5 and replaced before approval. Its planted control (`crates/viola-ui/tsconfig.json`) printed nothing: git's default pathspec needs a directory level between `**/` and the name. The shipped form is `'crates/viola-ui/*tsconfig*.json' 'crates/viola-ui/*package.json' 'crates/viola-ui/*.config.*'` plus a root `tsconfig.json`, and its control lists both planted files.
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. **cargo-modules `--acyclic` as a CI gate.** test-plan §9 Pipeline structure (Lint row, `test-plan.md:1428`) puts `cargo modules dependencies --package <crate> --acyclic` per crate in the per-OS lint job, and test-plan §9 Build failure conditions (`test-plan.md:1470`, "a cargo-modules cycle/orphan") treats a cycle as a build failure.
     - Measured at cargo-modules 0.27.0 (the CI pin): `--acyclic` exits 1 on 3 of 4 targets (`viola-core` lib, `viola` bin, `viola-e2e` lib). Every diagnostic is a type ↔ its own inherent method (`ViolaName`↔`try_new`, `ObsProcess`↔`as_str`, `MillisUtc`↔`format_time`, `Outcome`↔`new`).
     - It still exits 1 under `--no-fns --no-types --no-traits --no-owns --no-externs --no-sysroot`.
     - It fails by construction, not because of a code fault. The operator decision at phase P4: `--acyclic` returns to architecture's "on demand" boundary review.
  2. **The rmcp `cargo tree -e features` assertion.** Stated in the test-plan §9 Lint row and in test-plan §9 Build failure conditions ("rmcp `client` in the release graph"). It is vacuous at HEAD: `cargo tree -e features -p viola --edges normal | grep -c rmcp` returns `0`. It cannot run non-vacuously until `viola-mcp` lands. Operator decision: CARRY it to the "MCP server for drivers" entry, its true first consumer.
  3. **The phase P3 premise "a bare root `cargo build --release` may build test-only binaries"** (scope item 4): measured false. The bare form, and `--bin viola`, give `viola` only; `--workspace` adds `viola-harness`; the fake agent is never built without its feature. Measured with `--message-format=json` into `target/release-probe`. The premise was corrected in scope at phase P3. No master states it; the architecture's target job 6 text, "`cargo build --release`", is accurate at HEAD but weaker than test-plan's `--bin viola`.
- **Expected amendments (from plan)**, one line each. Each site is from a `grep -n` per master, re-run this wrap. The `owed` hits use a word-bounded read: bare `owed` also matches `allowed`, `followed` and `showed`.
  - **architecture §Project directory structure** — carried (Changes: Files, artifact-inventory). Site: the tree block, `architecture.md:424-478`. The 25 inventory tokens missing from the file are listed under Outcome (inventory gate).
  - **architecture §Occupied Resources → Repository** — carried (Schema / config). Site: `architecture.md:376-391`. `target/perf/`, `target/nextest/ci/`, `target/orphans-probes/`, `target/release-check/`, `e2e-web/test-results/{a11y,lint}/` and `deny-fuzz.json` have 0 hits each.
  - **architecture §Infrastructure Patterns → Build system** — carried (Harness / gate surface; Spec claims disproved 1).
    - Dependency policy "owed to" at `architecture.md:404` (1 hit).
    - "Boundary review: `cargo modules` graph on demand" at `architecture.md:405` (1 hit for `on demand`).
    - A code-graph planes statement: 0 existing `code-graph` sites besides :404's chunk title; `tsconfig` has 0 hits.
  - **architecture §Infrastructure Patterns → CI/CD approach** — carried (Counts moved; Harness surface). Site: `architecture.md:484`, "Jobs wired today in `ci.yml` (7)"; target job 6 at `:491`; the nightly sentence at `:481`; the tree comment `nightly.yml … cargo deny check advisories` at `:475`.
  - **architecture §Stack and Technologies, Code quality row** — carried (Dev-tool versions). Site: `grep -c 'cargo-modules\|cargo modules'` gives 2 hits, the Stack row and `:405`.
  - **test-plan §9 Pipeline structure** — carried (Harness surface; Spec claims disproved 1-2). Sites: the Lint row at `test-plan.md:1428` (cargo tree / acyclic / orphans); the Release build row at `:1438`; the Supply-chain row at `:1429` (weekly advisories); the tool-install paragraph at `:1441` (cargo-modules via `cargo install --locked`, unchanged and accurate).
  - **test-plan §9 / §10 Build failure conditions and §3 Bootstrap** — carried. Sites:
    - `test-plan.md:1470` ("a cargo-modules cycle/orphan", "rmcp `client` in the release graph");
    - `:1471` (the weekly nightly advisories run failing);
    - `:1529` (the §10 "cargo-modules failure");
    - `:785` and `:797` (§3 Bootstrap `ci-tool-install` / `quality-gate-config-emit`);
    - `:98` (the V9 trigger row: "cargo-modules boundary review").
    - `grep -c 'cargo-modules\|cargo modules'` gives 8 hits in total, the 6 above plus `:99` and `:1441`.
  - **test-plan §12 Decisions Log** — carried (a new entry for this chunk's dispositions).
  - **security-plan §Dependency Security, `fuzz/` bullet and CI integration Job 4** — carried (Harness surface). Sites: `security-plan.md:315` ("is owed to"), `:327` ("covers the root `Cargo.lock` only, not `fuzz/Cargo.lock`"), `:328` (weekly advisories).
  - **security-plan §Threat Model Summary (Supply chain)** — carried. Site: `security-plan.md:134` ("`cargo deny` covers the root `Cargo.lock` only … its advisory and source audit is owed").
  - **security-plan Decisions Log 2026-09-24 Conditions** — carried. Site: `security-plan.md:648` ("is owed to … (route CARRY)").
  - **obs-plan §8 Integration points item 6** — carried (Schema / config). Site: `obs-plan.md:1208-1210`, the unscanned-uploads inventory. It names the mutants verdicts and nightly `fuzz/artifacts/` but not the `supply-chain` artifact at all (0 hits for `supply-chain` in that block).
    - Measured this wrap: `deny.json`, `deny-fuzz.json` and `zizmor.json` all hold 0 absolute paths (`grep -E '[A-Za-z]:\\|[A-Za-z]:/|/home/|/Users/|/d/dev'`).
    - The one raw hit is `s:/` inside `https://` in `deny.json`, a false positive.
  - **Route CARRY (wrap route-resolve):** carried to P5 — onto "MCP server for drivers" (`working-route.md:73`).
  - **Wrap witness:** carried. CI run 36019646063 is recorded above (Cross-project).
- **Coverage of new surfaces:**
  - `scripts/release-check.sh` → validation: closed argument set, usage exit 2 ✓ · instrumentation: n/a (a CI gate script, verdict line on stdout) · PII: n/a · tests: self-probe (3 refused + control) and the gate entries ✓ · a11y: n/a · tokens: n/a
  - `scripts/orphans-check.sh` → validation: closed argument set ✓ · instrumentation: n/a · PII: n/a · tests: self-probe (1 fired + control) and the gate entry ✓ · a11y: n/a · tokens: n/a
  - `ci.yml` `release` job → tests: the release-check probe and default mode · instrumentation: n/a · the rest n/a
  - `target/supply-chain/deny-fuzz.json` (a new upload member) → PII: 0 absolute paths measured ✓ (admissible by content) · the rest n/a

## Deviations from intent
- **Inventory token for `tests/cmd/*.toml`.** Plan step 7 says the token is "the leaf spelling the tree prints". The leaf `cmd/` would falsely match the existing `src/cmd/` tree line, so the row uses `trycmd`, the name test-plan §2 gives those cases. The token checks presence only; the wrap's tree entry must carry the word `trycmd`.
- **Release job shells.** Plan step 3 says the release job's steps are "all `shell: bash`". Checkout and rust-cache are `uses:` steps, and the toolchain step carries no shell anywhere in `ci.yml`, so `shell: bash` is only on the two gate steps (existing convention, research §Conventions).
- **Inventory forecast.** The plan forecast about 45 rows with about 20 missing; the measurement is 45 rows with 25 missing (gate `python … artifact-inventory.tsv` printed `45`, `25`). This is a forecast miss, not a scope change.
- **v1-23 claim condition** (overseer, at the P5 `yes`): the `verified` flip happens only after BOTH (1) the wrap re-runs the inventory gate after its own architecture amendment and reads 0, AND (2) the three `release (…)` legs read `success` on the pushed sha. It stays `implemented` until both are read. This is recorded in matrix `v1-23` notes.

## Decisions & corrections
- **Operator decisions (phase P4):**
  1. Wire `cargo modules orphans --deny` per lib/bin target in the per-OS lint job; `--acyclic` returns to on-demand review; the rmcp `cargo tree` assertion becomes a CARRY to the viola-mcp entry.
  2. The ts plane is scoped to `e2e-web/` only, from its first chunk, through a tracked `e2e-web/tsconfig.json` (noEmit, strict). viola-ui assets are never under a tsconfig.
  3. `fuzz/Cargo.lock` joins the weekly nightly advisories.
  - The overseer (founder-delegated) added: the rmcp CARRY is legitimate only because viola-mcp is its true first consumer; the code graph is seeded and used on every plane that has code.
- **Overseer directives:**
  - "start now, do not idle on CI": distill and research ran before the CI result, and the final CI result was read before P4;
  - "record this as the 3f385dd CI witness at this chunk wrap; nothing to fold";
  - the v1-23 flip condition above.
- **Sweep hazards found this chunk:**
  - (a) A bare `owed` grep over the masters matches `allowed`, `followed` and `showed` (security-plan had 11 raw hits, 3 real). Word-bound it: `grep -w owed`.
  - (b) An absolute-path probe `[A-Za-z]:/` matches the `s:/` in `https://`.
  - (c) A git pathspec `dir/**/name` misses `dir/name`: without `:(glob)` magic, `**/` needs a directory level. `dir/*name` (fnmatch, `*` crosses `/`) catches both.
  - (d) A presence grep by a directory LEAF (`cmd/`) false-matches an unrelated tree line (`src/cmd/`).
- **Measured tool behaviours:**
  - cargo-modules 0.27.0 `dependencies --acyclic` is unusable as a gate: every inherent method closes a type↔method cycle, whatever the filters.
  - `cargo deny --manifest-path fuzz/Cargo.toml` resolves the root `deny.toml` by walking up from the manifest dir, and runs under the cwd's toolchain.
  - A shared `target/release/` keeps a stale `viola-harness.exe` from an earlier `--workspace` build.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - **Release build contains no test-only binary (v1-23 a): met locally.**
    - `bash scripts/release-check.sh --probe` → green, `last line release-check probes: 3/3 refused, control clean`.
    - `CARGO_TARGET_DIR=target/release-check bash scripts/release-check.sh` → green, `last line release-check: viola only`.
    - The 3 `release (…)` legs on the pushed sha are **owed**, as operator gate entries after the push. They are the v1-23 witness (2).
  - **Tokio ban holds (v1-23 b): met.** The sole-root loop is green, and `deny-probes: 13/13 banned, control clean`.
  - **Architecture lists every artifact (v1-23 c): inventory written**, 45 rows with 25 missing before the amendment. The wrap's architecture amendment must bring the inventory gate to `0`; that is v1-23 witness (1), re-read after P2.
  - **Fuzz lock audited: met locally.** `advisories ok, sources ok`, and nightly form `advisories ok`. The CI `supply-chain` witness on the pushed sha is owed (operator check-runs entry).
  - **Orphans gated: met locally.** `orphans-check probes: 1/1 fired, control clean`, `orphans-check: 5/5 targets clean`. The CI lint legs are owed on the pushed sha.
  - **Workflow hygiene: met.** SHA probe `0`, zizmor green, `${{` in run bodies `0`, G3 exit 1, nightly a11y grep `0`.
  - **Mutation: met.** `"verdict":"no-rust-delta"`.
  - **No tsconfig or build step under viola-ui: met.** The guard printed no output, exit 0.
- **Gates** (/implement run, gate log dir `implement-2026-09-24T16-17-36`): 27 entries — 20 green, 0 red, 1 recorded, 3 deferred, 3 operator.
  - **Green:** release-check `--probe`; release-check default; orphans-check `--probe`; orphans-check default; `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`; `… check advisories`; `cargo deny check`; the sole-root tokio loop; `bash scripts/deny-probes.sh`; `zizmor .github/workflows/`; the SHA-pin python probe (`0`); the run-body expression probe (`0`); the nightly a11y grep (exit 1, `0`); `grep -c 'bash scripts/release-check.sh'` (`2`); `grep -c 'scripts/orphans-check.sh'` (`1`); `grep -c 'manifest-path fuzz/Cargo.toml'` (`ci.yml:1`, `nightly.yml:1`); `bash scripts/install-ripgrep.sh`; G3; the viola-ui guard; `AGENT_RUN_CHUNK_BASE=3f385dd… run --mutants` (`no-rust-delta`).
  - **Recorded:** the inventory gate printed `45` / `25` (pre-amendment; its post-amendment read is owed at this wrap).
  - **Deferred** (`defer` key; zero `.rs` delta, confirmed by grepping every uncommitted file's name over the `*.rs` sources, 0 hits): `cargo fmt --all --check`, `cargo clippy …`, `bash scripts/agent-run.sh run --unit`.
  - **`leg = 'operator'`, not fired by /implement:** the push (clean-tree guard), the check-runs read (`last line success`), and the release legs read (`success,success,success`).
  - **Smoke:** skipped — no boot-path / UI-surface change.
- **Outcome basis:** implement's P4 report as given in this session's conversation, plus the overseer's P5 directive (the v1-23 witness condition).
- **Process hygiene** (implement P4 census, re-measured with `Get-Process`):
  - every cargo, cargo-deny, cargo-modules, jq, rg and zizmor child that the gate entries started has terminated;
  - left running, not this chunk's: viola-lab prototype `viola.exe` 12172 and 57904 (`viola wait viola-builder`, the operator's driver), and rust-analyzer 20712 / 39408 (the operator's IDE, toolchain 1.95.0).
