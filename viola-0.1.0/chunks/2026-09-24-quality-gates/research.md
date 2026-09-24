# Codebase Research — 2026-09-24-quality-gates

## Scope
- **Depth:** moderate. The harness crate is mature; the CI workflows, the harness dispatch, `run.rs`, `secret_scan.rs` and the shims were read in full or by section. There are eight host measurements (M1–M8).
- **Reads:** 14 · **Globs/Greps:** 12 · **graph queries:** 1 (45 rows, `db_state` fresh)
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`, read in full: 1 Session Addition applied (`--in-diff` mutates only lines the chunk diff touches).
  - `.claude/rules/testing.md`, read in full: 7 Session Additions. Applied: #1 (every function needs an observable effect), #5 (process-global `OnceLock`), #6 (a mutant-reachable wait stays under 20 s) and #7 (keep `#[cfg(unix)]` to a minimal reader).
  - `.claude/rules/host-win32.md`, always loaded: 5 Session Additions, including the separate-`CARGO_TARGET_DIR` rule for a running `.exe`.
- **Platform issues consulted:**
  - Query: cargo-mutants `#[cfg]` platform code reported missed.
  - Fetched source: https://mutants.rs/limitations.html, verbatim: *"cargo-mutants does not yet understand conditional compilation, such as `#[cfg(target_os = "linux")]`. It will report functions for other platforms as missed, when it should know to skip them."*
  - This explains the folded CI red (M1). The runner is not at fault.

## Measurements (host: Windows 11, x86_64-pc-windows-msvc)
- **M1: which `file_mode` mutants were missed.** Command: `cargo mutants --list --file crates/viola-e2e/src/harness/secret_scan.rs --workspace` (cargo-mutants 27.1.0).
  - `224:5` is the `#[cfg(unix)]` reader (`secret_scan.rs:222-226`), with `None`, `Some(0)` and `Some(1)`.
  - `230:5` is the `#[cfg(not(unix))]` stub body `None` (`secret_scan.rs:228-231`), with `Some(0)` and `Some(1)` only, because the body already is `None`.
  - The CI run `36005608858` MISSED pair (the overseer's reading: `230:5 -> Some(0)`, `-> Some(1)`) is therefore the **non-unix stub**. Linux never compiles that body, so no Linux test can observe it.
  - The 3 survivors of the prior chunk's local Windows gate were `224:5`, the unix reader, which Windows does not compile. **Each OS misses the other OS's cfg-gated body.**
  - The existing unix tests already observe the mode value: `secret_scan_flags_a_diagnostics_file_that_is_not_owner_only` (`secret_scan.rs:463`, 0644 → `["mode"]`) kills `224:5 → None`, and `secret_scan_passes_a_clean_tree` (`:287`, planted 0600 → exit 0) kills `224:5 → Some(0)/Some(1)`. The CI caught count (70) is consistent with that.
- **M2: the cfg-split census at HEAD.** Command: `grep -rn -A1 -E '#\[cfg\((windows|not\(unix\)|unix|not\(windows\))\)\]' --include=*.rs src crates tests | grep -E 'fn '` gives 2 hits, both `file_mode` (`secret_scan.rs:223`, `:229`). No other cfg-gated function body exists at HEAD, so the blind spot's only live instance is this pair. Epoch 2 (the Windows slice: ConPTY, SQOS pipes, DACL) will add `#[cfg(windows)]` bodies, which the ubuntu-only `mutants` job cannot kill (the fetched limitation above).
- **M3: MSRV build.**
  - `rustup toolchain install 1.96 --profile minimal` resolves to `rustc 1.96.1 (31fca3adb 2026-06-26)`.
  - Build: `CARGO_TARGET_DIR=target/msrv-probe cargo +1.96 check --workspace --all-targets --features viola/fake-agent` exits 0, with 0 `warning`/`error` lines (`grep -cE '^(warning|error)'` on the log = 0).
- **M4: MSRV unit leg.** `RUSTUP_TOOLCHAIN=1.96 CARGO_TARGET_DIR=target/msrv-probe cargo nextest run --workspace --features viola/fake-agent --profile ci -E 'kind(lib) | kind(bin)'` ran 158 tests: 158 passed, 0 skipped, exit 0. `RUSTUP_TOOLCHAIN=1.96 rustc --version` prints `1.96.1`, so the env override outranks `rust-toolchain.toml`'s `1.98.1` pin. That is the mechanism the MSRV job needs to select 1.96 without editing the pin.
- **M5: coverage, Windows leg.** Command: `cargo llvm-cov nextest --workspace --features fake-agent --profile ci --ignore-filename-regex '(viola-fake-agent|crates/viola-e2e|tests/support|fuzz/)' --json --summary-only` (cargo-llvm-cov 0.9.1, the CI pin, installed on the host for this probe; cargo-nextest 0.9.133 on the host vs the 0.9.146 CI pin). Result: 248 tests run, 248 passed, exit 0.
  - **The ignore regex does not match on Windows.** The report lists `crates\viola-e2e\...` files, because llvm-cov gives Windows paths with `\`, and `python -c "re.search(r'(…crates/viola-e2e…)', r'D:\…\crates\viola-e2e\src\x.rs')"` returns `False`. `tests/support` is subject to the same defect, and `fuzz/` will be once it exists. Only `viola-fake-agent` (no separator) matches.
  - Totals as reported (e2e wrongly included): lines 97.85 %, functions 97.22 %, regions 97.66 %.
  - **Product-only totals** (the 7 files outside `viola-e2e`: `crates/viola-core/src/{lib,obs}.rs`, `src/{main,obs}.rs`, `src/cmd/{mod,run}.rs`, `src/run/mod.rs`): lines 98.21 % (879/895), **functions 96.55 % (112/116)**, regions 98.34 % (1540/1566). This is above 85 / 95 / 80. Functions has the thinnest margin: 4 uncovered functions, one each in `src/cmd/mod.rs`, `src/cmd/run.rs`, `src/main.rs` and `src/obs.rs`. Losing 2 more drops the Windows leg under 95.
  - The Linux and macOS legs are unmeasured here. Their witness is the CI `test` job's `--coverage` step on this chunk's push.
- **M6: JUnit location under llvm-cov.** `stat` before and after M5: `target/nextest/ci/junit.xml` was rewritten (mtime 15:24 → 15:53), and `target/llvm-cov-target/nextest/ci/junit.xml` does not exist. So test-plan §3 `gate` Inputs' "for coverage it is `target/llvm-cov-target/nextest/ci/junit.xml`" is falsified at nextest 0.9.133 + llvm-cov 0.9.1. nextest's store stays workspace-root-relative under llvm-cov too, the same fact test-plan §3 step 1 already records for `CARGO_TARGET_DIR`. Consequence: the secret-scan root (`secret_scan.rs:43`) and the `junit-<os>` upload path (`ci.yml:124`) remain correct under `--coverage`, with no scan-scope change.
- **M7: zizmor pedantic.** `zizmor --persona pedantic --format plain .github/workflows/` reports 2 findings, both `help[concurrency-limits]` "workflow is missing concurrency setting" (one per workflow): 0 informational, 2 low, 0 medium, 0 high. The default persona reports none.
- **M8: `mutants.out/` contents** (the dev host's last local run):
  - `outcomes.json` carries each phase's `argv` with absolute toolchain paths (`D:\dev\rust\rustup\toolchains\…\cargo.exe`, obs-plan §8 Medium class);
  - `log/*.log` carries test output;
  - `diff/`, `*.txt`, `mutants.json`, `lock.json` and `debug.log` are also present.

  The upload (`ci.yml:158-165`, `if: always()`, no scan) ships all of it.

## Files inspected
- `crates/viola-e2e/src/harness/secret_scan.rs` (full). The scan, the `Roots` (the JUnit root at `:43`), `mode_hit`/`owner_only`/`file_mode` (`:213-231`) and the 15 tests.
- `crates/viola-e2e/src/harness/run.rs` (1–520):
  - `Selection` (`:19-36`: unit/integration/mutants only);
  - `nextest` (`:119-160`: deletes the JUnit source and copies it to `artifacts/junit-<suite>.xml`);
  - `exit_must_agree`, `parse_junit`, `parse_doctest`, `merge_summary` (`:245-262`) and the mutants gate (`:400-460`).
- `crates/viola-e2e/src/harness/mod.rs` (full): `Workspace::{artifacts, cargo_target, e2e_home}`, `Outcome::{new, usage}`, `write_json`, `read_json`.
- `crates/viola-e2e/src/bin/viola-harness.rs` (full): the `Cmd` enum (no `Gate`, no `--coverage` / `--fuzz-replay`) and `COMMANDS` (`:73-82`, 8 entries).
- `crates/viola-e2e/tests/cli.rs` (55–79): `unbuilt_selectors_and_unknown_commands_are_usage` pins `run --coverage` as usage at `:61`.
- `scripts/agent-run.sh` (40–80) and `agent-run.ps1` (grep): `gate` is already forwarded by both (sh `supervise|ui-restart|gate|schema-check|secret-scan`; ps1 `'gate'` at `:70`).
- `.github/workflows/ci.yml` (full):
  - `test` (3 OS): `run --unit` then `run --integration` through the shims, the harness lifecycle, and G2 / G4 / capture / scan / uploads;
  - `mutants` (ubuntu, `fetch-depth: 0`, `AGENT_RUN_CHUNK_BASE` via `env:`, unscanned `mutants.out/` upload);
  - `lint` (3 OS);
  - `supply-chain`.

  No `gate` step, no MSRV, coverage or fuzz job, no `concurrency:`.
- `.github/workflows/nightly.yml` (full): one `advisories` job, with no cache and no toolchain action.
- `Cargo.toml` (full): `members = ["crates/*"]`, `rust-version = "1.96"`, proptest `=1.11.0` pinned, `[profile.*] panic = "unwind"`.
- `.config/nextest.toml` (full): `[profile.ci] retries = 0`. The `mutants` profile sets no `retries` (default 0). The default profile has no `retries` key.
- `crates/viola-core/src/lib.rs` (1–110): `MAX_FRAME`, `ViolaName` (nutype, `is_valid_name`), 4 proptest properties at `cases: 512` with `FileFailurePersistence::SourceParallel("proptest-regressions")`.
- `crates/viola-e2e/Cargo.toml` (grep): the no-op `fake-agent` feature and its own `[lints.clippy] dbg_macro`.
- `.andromeda/test-plan.md` §3 `run` / `gate` / Internal subcommands / Closed enums (530–664), §6 Property suite (1311–1320), §9 (1417–1471) and §10 (1475–1528).

## Graph impact
- **`file_mode`**: 1 production caller, `harness/secret_scan/scan()` @ `crates/viola-e2e/src/harness/secret_scan.rs:64`. A same-signature restructure has zero boundary callers.
- **`Selection::from_flags`**: 1 production caller, `main()` @ `crates/viola-e2e/src/bin/viola-harness.rs:134`, plus the `run.rs` unit tests (`selection_defaults_to_everything`, `selection_picks_only_the_named_suites`). Adding selector fields threads through exactly these.
- **`nextest`** (harness `run`): callers `run()` @ `run.rs:71`, `:74`. The coverage suite is a sibling call at the same site.
- **`mutants_suite`**: caller `mutants()` @ `run.rs:448`. `gate`'s mutants breach reads the merged `run-summary.json` suite and `outcomes.json`, and does not re-derive the verdict.
- **`Roots::of` / `scan`**: `secret_scan()` @ `secret_scan.rs:50`, plus 13 in-file tests. The JUnit root is unchanged (M6).
- Crate edges are not queried: every change is inside `viola-e2e` (test-only, a leaf by arch §Module Boundaries), plus workflows and scripts.

## Patterns detected
- **Internal subcommand shape** (`viola-harness.rs:69-70`, `:168-169`; `secret_scan.rs:49-100`): a library fn returns `Outcome{doc, code}`, `main` `emit`s one JSON document, the arg-parse failure is `Outcome::usage(cmd, "arguments")`, and `COMMANDS` names the verb for the usage `cmd`. `gate` follows it.
- **Suite from a tool run** (`run.rs:119-160`): delete the tool's report first, run with stdout forwarded to stderr, read the report, and treat a missing report as `artifact-missing`. `exit_must_agree` keeps a red exit red. `--coverage` and `--fuzz-replay` follow it.
- **Merged summary** (`run.rs:245-262`): `artifacts/run-summary.json` is merged by `suite` across invocations. That is `gate`'s primary input.
- **Harness tests drive tools against a temp Cargo project** (`run.rs` tests such as `run_mutants_passes_when_the_change_is_tested`; testing.md Session Addition #1): never a nested build of this workspace.
- **Proven-live probes** (`scripts/deny-probes.sh`, `scripts/lint-probes.sh`): a negative probe plus a clean control, with a fixed last line.

## Conventions to follow
- **The harness prints by design**: `viola-e2e` has its own `[lints.clippy]` with only `dbg_macro` (`crates/viola-e2e/Cargo.toml:31-32`). Product crates stay print-free.
- **Mutant-reachable waits** stay exit-aware and under 20 s (testing.md SA #6; `.config/nextest.toml` `[profile.mutants]` 5 s×2, and `viola-e2e` 15 s×2).
- **`#[cfg(unix)]` bodies stay minimal readers**, with the decision logic in plain functions tested on every OS (testing.md SA #7; `owner_only` / `mode_hit` already follow this).
- **CI steps**: `shell: bash`, fail-closed; `uses:` SHA-pinned with a version comment; event values only through `env:`; uploads carrying test output only behind `steps.secret-scan.outcome == 'success'` (`ci.yml:103-134`).
- **Toolchains come from `rustup toolchain install`, never a toolchain action** (`ci.yml:25-26` and every job). The MSRV job can stay inside this convention (M3/M4).

## New files to create
- `crates/viola-e2e/src/harness/gate.rs`: the `gate --require <suites> [--artifacts <dir>]` body.
- `crates/viola-e2e/tests/` coverage for `gate` and the new `run` selectors, as unit tests in-file per the crate's pattern, plus CLI cases in `tests/cli.rs`.
- The zero-retries check: a contract test or probe over `.config/nextest.toml`, with a negative probe and a clean control. Its home is P4's choice.
- `fuzz/` (its own workspace: `fuzz/Cargo.toml` with `[workspace]`, `publish = false`), `fuzz/fuzz_targets/<target>.rs` and `fuzz/corpus/<target>/`. Whether a target lands now is a P4 fork (Open questions).

## Files to modify
- `crates/viola-e2e/src/harness/run.rs`:
  - `Selection` gains `coverage` and `fuzz_replay`;
  - `run()` swaps the unit/integration nextest calls for one `cargo llvm-cov nextest …` suite `coverage` when `--coverage` is set (JUnit from `target/nextest/ci/junit.xml` per M6, copied to `junit-coverage.xml`; `cargo llvm-cov report --json --summary-only` → `artifacts/llvm-cov-summary.json`), with `doctest` following;
  - a `fuzz-replay` suite (`tool-missing`, `corpus-empty`);
  - tests.
- `crates/viola-e2e/src/harness/mod.rs`: `pub mod gate;`.
- `crates/viola-e2e/src/bin/viola-harness.rs`: `Run { coverage, fuzz_replay }`, `Cmd::Gate { require, artifacts }`, and `"gate"` in `COMMANDS`.
- `crates/viola-e2e/tests/cli.rs:61`: `run --coverage` stops being an unbuilt selector. The case moves to a still-unbuilt one (`run --perf`), keeping the test's intent.
- `crates/viola-e2e/src/harness/secret_scan.rs:222-231`: `file_mode` becomes ONE function whose `#[cfg(unix)]` / `#[cfg(not(unix))]` arms are blocks inside a shared body. Its whole-body replacement mutants are then compiled on Linux and killed there by the two existing tests (M1). The restructure touches the lines, so `--in-diff` regenerates them on this chunk's push (verification-harness SA).
- `.github/workflows/ci.yml`:
  - `test`: cargo-llvm-cov 0.9.1 joins the taiki-e line, plus an explicit `rustup component add llvm-tools-preview`; `run --coverage` replaces `run --unit` + `run --integration`; `gate --require coverage,doctest` goes last. The G2 / G4 / scan / upload order is unchanged.
  - `mutants`: `gate --require mutants`, and the `mutants.out/` upload decision.
  - A new `msrv` job: `rustup toolchain install 1.96`, `RUSTUP_TOOLCHAIN=1.96` for `cargo check --workspace` and `run --unit`, then `gate --require nextest-unit` under the pinned toolchain, with its own rust-cache key.
  - A new fuzz-replay job, conditional on the P4 fork.
- `.github/workflows/nightly.yml`: the time-boxed fuzz job, conditional on the P4 fork. No cache.
- `Cargo.toml`: `[workspace] exclude = ["fuzz"]`, if `fuzz/` lands.
- `.gitignore`: `fuzz/target/`, `fuzz/artifacts/` (a reproducer is committed into `corpus/` with its fix, never from `artifacts/`), if `fuzz/` lands.
- Sweep: `nextest/ci/junit`: 2 hits (`ci.yml:124`, `secret_scan.rs:433`), 0 changed, 2 no-change (the path holds under llvm-cov, M6). `run --coverage`: 1 pin (`tests/cli.rs:61`), changed.

## Open questions
- **The mutation gate cannot kill cfg-gated bodies of another OS** (M1, M2, the fetched limitation). The only live instance at HEAD is fixed by the `file_mode` restructure. Epoch 2's `#[cfg(windows)]` code will be red by construction under the ubuntu-only `mutants` job. Options:
  - a `windows-2025` mutants leg whose verdict is the per-mutant union ("caught on any leg");
  - keep ubuntu-only and record a forward CARRY onto the first Windows chunk.

  → blocks: plan-decision.
- **Fuzz replay with no parser at HEAD.** None of test-plan §6's seven property/fuzz parsers exists yet: `validate_paste_text`, `Last-Event-ID`, ndjson framing, the hook stdin parser, `agents --json`, vt100 and `prompt-submitted` normalisation. `grep -rnE 'fn validate_paste_text|Last-Event-ID|fn parse_hook|agents --json|vt100'` over `crates src` returns 0 product hits. Owners on the route: vt100 → `working-route.md:45`, `agents --json` → `:66`, `Last-Event-ID` → `:100`, the channel framing entries → `:28`/`:34`. Options:
  - seed the pipeline now with a genuine target on the one input validator that exists (`ViolaName::try_new`, which gates every path join), amending test-plan to add it;
  - build the verb only and move the CI job, nightly time-box and targets to the first parser chunk.

  → blocks: plan-decision.
- **(none further).** The toolchain source, the `concurrency:` block and the `mutants.out/` upload each have a decisive lean from the artifacts above (M3/M4, M7 with the obs/tests bindings, M8). They are stated in the plan, not asked.
