# Report — 2026-09-24-quality-gates

**Chunk:** quality CI gates: MSRV 1.96 job, per-OS llvm-cov floors (85/95/80), seeded proptest + cargo-fuzz corpus replay, zero-retry check, viola-harness gate per job; zizmor concurrency decision, mutants.out scan, CI-missed file_mode mutant kill
**Date:** 2026-09-24T14:50:00Z
**Commits:** none since last_wrap 2026-09-24T13:24:00Z (`git log --oneline 39c3d2b..HEAD` is empty); this chunk rides the wrap commit.

## Changes (structured — detectors read this)
- **Files** (`git status --short`: 9 modified + 3 new source/test/CI paths):
  - modified: `crates/viola-e2e/src/harness/{secret_scan,run,mod}.rs`, `crates/viola-e2e/src/bin/viola-harness.rs`, `crates/viola-e2e/tests/cli.rs`, `.github/workflows/{ci,nightly}.yml`, `Cargo.toml`;
  - new: `crates/viola-e2e/src/harness/gate.rs`, `crates/viola-e2e/tests/zero_retries.rs`, `fuzz/` (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `fuzz_targets/viola_name.rs`, `corpus/viola_name/*` with 18 synthetic seeds).
  - Bookkeeping riding the commit: route/master/matrix/friction log, the chunk folder, and run dirs.
- **Symbols / APIs** (all in the test-only `viola-e2e` crate; no product crate changed):
  - **`secret_scan::file_mode`** is now ONE function whose body holds a `#[cfg(unix)]` block and a `#[cfg(not(unix))]` block. The signature is unchanged; the sole caller is `scan()` @ `secret_scan.rs:64` (graph query, phase run).
  - **`harness::run`:**
    - `Selection` gains `coverage` and `fuzz_replay` and derives `Default`.
    - `Selection::from_flags(named: Selection, all: bool)` replaces `from_flags(unit, integration, mutants, all)`. Its only production caller is `viola-harness` `main()`; the other callers are in-file tests.
    - New `pub` items: `run_with(ws, sel, filter, chunk_base, leg, runner)`, `type Runner`, `run_forwarding` (made `pub`), `COVERAGE_IGNORE`, `COVERAGE_FLOORS`, `fuzz_host_supported`, `fuzz_channel`, `Refusal { reason, detail }`, `leg_verdict`, `leg_verdict_path`.
    - `run()` keeps its signature and delegates to `run_with(…, None, &mut run_forwarding)`.
    - `nextest`, `doctest` and `mutants` now take the runner (a tool-invocation seam, used by tests with a stand-in).
    - New private suites: `coverage` and `fuzz_replay`.
  - **`harness::gate`** (new): `gate(artifacts, require, legs)`, `parse_require`, `parse_legs`, `SUITES` (the 9 closed values), `SPINE_DEADLINE_S = 1.0`.
  - **`viola-harness` CLI:**
    - `run` gains `--coverage`, `--fuzz-replay` and `--leg <name>` (clap `requires = "mutants"`; a name outside the session-id charset is usage `detail:"invalid-leg"`).
    - New subcommand `gate --require <suites> [--artifacts <dir>] [--mutants-legs a,b]`. Usage details: `unknown-suite`, `invalid-leg`, `arguments`.
    - `COMMANDS` now has 9 entries.
  - **New document fields and reasons:**
    - `run` may carry `"detail"` beside `"reason"`;
    - reasons `fuzz-linux-only` (exit 2), `tool-missing` (detail `cargo-fuzz` | `fuzz/rust-toolchain.toml`) and `corpus-empty`;
    - suite failure codes `llvm-cov-summary-missing` and `llvm-cov-exit-N`;
    - `mutants` gains `"leg"`;
    - `gate` breaches `{gate, suite, detail}`, where `gate` ∈ {`suite-missing`, `suite-failed`, `suite-skipped`, `coverage`, `mutants`, `perf`, `artifact-missing`} and `detail` is a fixed code or a count (`absent`, `no-run-summary`, `failed N`, `skipped N`, `survived N`, `lines 84.99 < 85`, `<metric> unreadable`, `junit-<suite>.xml`, `llvm-cov-summary.json`, `mutants-verdict-<leg>.json`, `perf-*.json`, `<file> max X >= 1`, or a mutant name).
  - **Env vars:** none new. The MSRV job sets rustup's own `RUSTUP_TOOLCHAIN=1.96` per step. `LEG` is a CI step env for the leg name, read by the step's shell, not by the harness.
- **Crates / modules:**
  - added `harness::gate` (in `viola-e2e`);
  - added `fuzz/` as a SEPARATE cargo workspace (`viola-fuzz`, `publish = false`, `[package.metadata] cargo-fuzz = true`, its own `[workspace]`);
  - the root `Cargo.toml` `[workspace]` gains `exclude = ["fuzz"]`.
- **Dependencies:**
  - root graph: none. The root `Cargo.lock` is unchanged (`git diff --stat -- Cargo.lock` is empty).
  - `fuzz/` (its own lockfile, outside `cargo deny`'s root graph): `libfuzzer-sys = "=0.4.13"` (builds C++ via `cc`), `arbitrary = "=1.4.2"`, and a path dep on `viola-core`.
- **Schema / config:**
  - `fuzz/rust-toolchain.toml` `channel = "nightly-2026-09-20"` (a second toolchain file, fuzz-only). The root `rust-toolchain.toml` is unchanged at `1.98.1` (gate `grep -c 'channel = "1.98.1"'` → 1).
  - The per-leg verdict file `target/agent-run/artifacts/mutants-verdict-<leg>.json` = `{"v":1,"leg","verdict":"counted"|"no-rust-delta","mutants":[{"name","outcome":"caught"|"missed"|"timeout"|"unviable"}]}`. It holds names and outcomes only: no argv, log path or test output.
  - `target/agent-run/artifacts/llvm-cov-summary.json` and `target/lcov.info` are new harness outputs. `.config/nextest.toml` is unchanged.
- **Spec-master edits:** none this chunk (the wrap applies them below).
- **Counts / qualifiers moved:**
  - the harness `COMMANDS` go from 8 to 9 (`viola-harness.rs`);
  - the pinned third-party action set goes from 4 to 5 (`actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c # v8.0.1`, ci.yml `mutants-verdict`);
  - `ci.yml` jobs go from 4 (`test`, `mutants`, `lint`, `supply-chain`) to 7 (+ `mutants-verdict`, `msrv`, `fuzz-replay`), and `mutants` becomes a 2-leg matrix;
  - `nightly.yml` jobs go from 1 (`advisories`) to 2 (+ `fuzz`);
  - fuzz targets go from 0 to 1 (`viola_name`);
  - the MSRV-toolchain unit count is 191 (gate log `Summary 191 tests run: 191 passed`).
- **Dev-tool versions:**
  - `cargo-llvm-cov` (the coverage runner) was installed on the dev host at the CI pin: 0.8.5 → 0.9.1 (2026-09-24, phase P3);
  - rustup component `llvm-tools-preview` was added to 1.98.1 on the dev host;
  - toolchain `1.96` (rustc 1.96.1) was installed on the dev host (the MSRV compiler);
  - toolchain `nightly-2026-09-20` (rustc 1.100.0-nightly feaadeeac) was installed on the dev host (the fuzz toolchain);
  - `cargo-fuzz` 0.13.2 (the fuzz runner) is CI-only, installed by `cargo install --locked` in the `fuzz-replay` and nightly `fuzz` jobs. It is absent on the dev host.
  - `libfuzzer-sys` / `arbitrary` are lockfile-resolved crates, NOT this line's subject.
- **Harness / gate surface:**
  - **`ci.yml` `test` (3 OS):**
    - `rustup component add llvm-tools-preview`; taiki-e `tool:` gains `cargo-llvm-cov@0.9.1`;
    - "Unit and integration" becomes "Coverage and doctest": ONE `agent-run run --coverage` whose suite is `coverage`, replacing the `nextest-unit` and `nextest-integration` suites in CI. The CLI output-discipline tests still run inside it on all 3 OSes; they now report under suite `coverage`.
    - G2, G4, the capture, the scan and the scan-gated uploads are unchanged. The JUnit path is unchanged (`target/nextest/ci/junit.xml`).
    - The last step is `Gate verdict` (`if: always()`): `gate --require coverage,doctest`.
  - **`mutants` → `mutants (${{ matrix.os }})`:**
    - legs `ubuntu-latest` and `windows-2025`, `fail-fast: false`, running `run --mutants --leg "$LEG"` (env);
    - the upload is ONLY `mutants-verdict-<os>.json`; **the `mutants.out/` upload is removed**.
  - **New `mutants-verdict`** (ubuntu, `needs: mutants`, `if: always()`): download-artifact `pattern: mutants-verdict-*`, `merge-multiple`, then `gate --require mutants --mutants-legs ubuntu-latest,windows-2025`. The union makes a mutant red only when no leg caught it and some leg missed it or timed out it.
  - **New `msrv`** (ubuntu, rust-cache `key: msrv`, nextest + cargo-mutants):
    - `rustup toolchain install 1.96 --profile minimal`;
    - `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`;
    - `gate --require nextest-unit`.
  - **New `fuzz-replay`** (ubuntu, no cache): `rustup toolchain install` (root, then `working-directory: fuzz`), `cargo install --locked cargo-fuzz@0.13.2`, `run --fuzz-replay`, then `gate --require fuzz-replay`.
  - **`nightly.yml` new `fuzz` job** (no cache): per target, `cargo +<channel> fuzz run --fuzz-dir fuzz <t> fuzz/corpus/<t> -- -max_total_time=120`, uploading `fuzz/artifacts/` on `failure()`. It has no a11y step.
  - **No `concurrency:` block** (zizmor pedantic `concurrency-limits` declined; see Decisions).
  - The harness shims are unchanged (both already forwarded `gate`).
- **Cross-project / external claims:**
  - CI run `36005608858` on sha `39c3d2b` (the prior chunk's push), overseer-read: `mutants` 78 tested, 70 caught, 6 unviable, 2 MISSED (`secret_scan.rs:230:5` `file_mode -> Some(0)` / `-> Some(1)`); `test (ubuntu-latest)` has exactly one PASS line for `wrapper_boot_exiting_before_ready_fails_as_exited`. check-runs on `39c3d2bf…`: `mutants` failure, the other 7 success (phase Setup read).
  - cargo-mutants documentation, https://mutants.rs/limitations.html (fetched at phase P3): "cargo-mutants does not yet understand conditional compilation, such as `#[cfg(target_os = "linux")]`. It will report functions for other platforms as missed, when it should know to skip them."
  - `actions/download-artifact` latest release v8.0.1 = commit `3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c` (`gh api repos/actions/download-artifact/releases/latest` + `git/ref/tags/v8.0.1`, phase P4).
  - This chunk's own CI witness is not yet taken; it is owed after the wrap's push (Outcome).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. test-plan §3 Internal harness subcommands `gate` Inputs (`test-plan.md` `llvm-cov-target` 1 hit): "For coverage it is `target/llvm-cov-target/nextest/ci/junit.xml`". **Measured false.** Under `cargo llvm-cov nextest` (llvm-cov 0.9.1, nextest 0.9.133), nextest writes `target/nextest/ci/junit.xml`, and `target/llvm-cov-target/nextest/ci/junit.xml` does not exist (research M6, `stat` before and after). This chunk's `run --coverage` copies from `target/nextest/ci/junit.xml`, and gate `run --coverage` was green with `junit-coverage.xml` present.
  2. test-plan §3 `--coverage` / §10 Stack adjustments: the ignore regex `(viola-fake-agent|crates/viola-e2e|tests/support|fuzz/)` does NOT exclude those trees on Windows, because llvm-cov reports `crates\viola-e2e\…` (research M5: 10 viola-e2e files counted; `re.search` on a backslash path → False). The shipped `COVERAGE_IGNORE` is `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the same four trees. Gate probe: 0 viola-e2e files in `llvm-cov-summary.json`.
  3. Route PREREQ on `working-route.md:23` (now frozen) plus the prior handoff: "Its 3 `…secret_scan.rs` `file_mode` mutants (`#[cfg(unix)]`…) must read caught there". The 2 CI-MISSED mutants were NOT those three. `cargo mutants --list` puts `224:5` (the unix reader, 3 mutants, caught on Linux) apart from `230:5` (the `#[cfg(not(unix))]` stub, 2 mutants), which Linux never compiles (research M1). The overseer accepted the correction at the phase review. The fix is one function body (gate `cargo mutants --list … | grep -c 'replace file_mode'` → 3).
- **Expected amendments (from plan):** hit counts from the per-master grep this wrap ran (`grep -c` in each of the 7 masters):
  - test-plan §3 `run --coverage`: JUnit source plus separator-agnostic regex. **Carried** (claims 1 and 2). Sites: test-plan `llvm-cov-target` 1, `crates/viola-e2e\|tests/support` 16 (the regex lines among them); obs-plan 1 and architecture 2 hits on that pattern are to be read by their detectors.
  - test-plan §3 `gate` Inputs: coverage JUnit, `--mutants-legs` and the union rule. **Carried** (Harness / gate surface; claim 1). Sites: test-plan `llvm-cov-target` 1, `mutants-verdict` 0 (new text).
  - test-plan §3 `run`: `--leg`, `mutants-verdict-<leg>.json`, `--fuzz-replay` reading `fuzz/rust-toolchain.toml`, `fuzz-linux-only`. **Carried** (Symbols / APIs). Sites: test-plan `fuzz-replay` 6, `cargo +nightly fuzz` 2.
  - test-plan §6 Property suite: `viola_name` as the eighth, pre-parser seed target. **Carried** (Crates / modules; Counts). Sites: test-plan `fuzz/` 6.
  - test-plan §9 MSRV / Fuzz replay rows: rustup, not dtolnay; the Mutation row's two legs plus `mutants-verdict`; Test report format without `mutants.out/`. **Carried** (Harness / gate surface). Sites: test-plan `dtolnay` 2, `mutants\.out` 9, `nightly\.yml` 7.
  - test-plan §10 Mutation gate: the union verdict. **Carried** (Harness / gate surface).
  - test-plan §12: three Decisions Log entries (union verdict, eighth target, declined `concurrency:`). **Carried** (Decisions).
  - architecture §Infrastructure Patterns → CI/CD approach: the new jobs, the nightly fuzz job, download-artifact in the pinned set. **Carried** (Harness / gate surface; Counts). Sites: architecture `nightly\.yml` 2, `no toolchain action` 2, `concurrency` 2, `download-artifact` 0 (new text).
  - architecture §Occupied Resources → Repository and the tree: `fuzz/`, `fuzz/rust-toolchain.toml`, `fuzz/corpus/<target>/`, `target/lcov.info`, `artifacts/llvm-cov-summary.json`, `artifacts/mutants-verdict-<leg>.json`. **Carried** (Schema / config; Crates / modules). Sites: architecture `fuzz/` 0, `llvm-cov-summary` 0 (new text), `rust-toolchain\.toml` 5.
  - security-plan §Dependency Security: the pinned-action set plus download-artifact, `fuzz/Cargo.lock` outside `cargo deny`, `concurrency-limits` declined. **Carried** (Counts; Dependencies; Decisions). Sites: security-plan `no toolchain action` 2, `rust-toolchain\.toml` 3, `nightly\.yml` 3, `concurrency` 0.
  - security-plan §Secret Management / §Bootstrap `secret-scanning-ci-gate`: the `mutants.out/` CARRY closed by removal. **Carried** (Harness / gate surface). Sites: security-plan `mutants\.out` 1.
  - obs-plan §9 Platform: `nightly.yml` also runs the fuzz time-box. **Carried**. Sites: obs-plan `nightly\.yml` 1.
  - obs-plan §8 integration point 6: `mutants.out/` no longer uploaded. **Carried**. Sites: obs-plan `mutants\.out` 1.
- **Coverage of new surfaces:**
  - `viola-harness gate` → validation closed-enum `--require` + leg charset✓ · instrumentation n/a (harness, one JSON document) · PII n/a (breach detail = codes, counts, mutant names; never file content) · tests unit (13 in `gate.rs`) + integ (`tests/cli.rs` 2) · a11y n/a · tokens n/a
  - `viola-harness run --coverage` → validation n/a · instrumentation n/a · PII n/a (lcov not uploaded; the summary leaves only in the scan-gated `harness-<os>`) · tests unit (6, stand-in runner) + gate run (green, Windows) · a11y n/a · tokens n/a
  - `viola-harness run --fuzz-replay` → validation `fuzz_channel` parse, corpus non-empty✓ · instrumentation n/a · PII n/a · tests unit (6) + host arm gate (exit 2 `fuzz-linux-only`); the Linux replay is `unrunnable-here` (CI `fuzz-replay` job) · a11y n/a · tokens n/a
  - `viola-harness run --mutants --leg` / `mutants-verdict-<leg>.json` → validation leg charset✓ · instrumentation n/a · PII names and outcomes only✓ · tests unit (6) + local leg gate (118 mutants) · a11y n/a · tokens n/a
  - `fuzz/fuzz_targets/viola_name.rs` → validation exercises `ViolaName::try_new` against an independent oracle · instrumentation n/a · PII synthetic corpus✓ (gate grep 0 path hits) · tests the fuzz target itself (the Linux replay is `unrunnable-here`); `cd fuzz && cargo check --bins` green on the host · a11y n/a · tokens n/a
  - `crates/viola-e2e/tests/zero_retries.rs` → validation n/a · instrumentation n/a · PII n/a · tests integ (2: the repo config clean, 5 planted negative forms flagged) · a11y n/a · tokens n/a
  - CI jobs `msrv`, `fuzz-replay`, `mutants-verdict`, nightly `fuzz` → validation n/a · instrumentation n/a · PII uploads: verdict JSON (names only), `fuzz/artifacts/` (fuzzer inputs, on failure) · tests: CI witness owed · a11y n/a · tokens n/a

## Deviations from intent
- **The tool seam goes wider than planned.** Plan step 4 named a program seam for fuzz only. /implement generalised it to a `Runner` closure threaded through `run_with`, `nextest`, `doctest`, `coverage` and `mutants`, and made `run_forwarding` `pub` so `main` passes the real one. Justification: the zero-missed mutation gate needs every new branch killable. The only alternative was nested `cargo llvm-cov` / `cargo fuzz` inside the nextest run that CI's `--coverage` itself hosts, plus those tools in every job running unit tests (msrv, both mutants legs).
- **`.gitignore` is unchanged** although the plan listed it. It already ignores `target/` (nested `fuzz/target/` included), `fuzz/artifacts/` and `fuzz/coverage/`. P5's `git check-ignore fuzz/target` read 1 only because a trailing-slash pattern cannot match an absent directory.
- **The local single-leg union forecast missed.** The plan forecast exactly 1 red name (`file_mode -> None`, equivalent on Windows). The final read is 2: the second is `fuzz_host_supported -> false`, which is the real value on Windows and the same class. The Linux leg kills both (`fuzz_host_is_linux_only`; the 0644 mode test). The CI union is the verdict.
- **Two more design choices, both within the plan's step intent:** the harness `cli.rs` "unbuilt selector" case moved from `run --coverage` to `run --perf`, and `run` documents may now carry `detail`.

## Decisions & corrections
- **Overseer correction (phase P5, accepted):** "230:5 is the non-unix stub; my read was wrong." The overseer had read the CI misses as a Unix test not observing the mode. cargo-mutants' own `--list` shows `230:5` is the `#[cfg(not(unix))]` body, which Linux never compiles.
- **Operator decisions (phase P4):**
  - add a `windows-2025` mutants leg with a union verdict now, not in Epoch 2 ("problems are fixed at once, never pushed to the tail");
  - seed the fuzz pipeline with the `ViolaName::try_new` target now ("ViolaName is a real consumer, so the fuzz pipeline is infrastructure with a target, not an abstraction over emptiness");
  - record the macOS-only (`target_os = "macos"`) residual with its owner: "Unix endpoint and home hardening" (Epoch 7).
- **Leans taken at P4 and approved at the P5 review:**
  - **MSRV toolchain:** `rustup toolchain install 1.96` + `RUSTUP_TOOLCHAIN=1.96`, not dtolnay (architecture/security "no toolchain action"). Research M4 measured that the env override outranks `rust-toolchain.toml`.
  - **No `concurrency:` block.** A group cancels pending runs even with `cancel-in-progress: false`, which drops that push's `--in-diff` mutation diff and its `always()` gate and upload chain.
  - **The `mutants.out/` upload is removed** rather than scanned. `outcomes.json` holds absolute argv paths, `log/` holds test output, and `secret-scan` is `empty-scope` in a job with no homes.
  - **The fuzz toolchain comes from `fuzz/rust-toolchain.toml`.**
- **Sweep hazards found this chunk:**
  - (a) cargo-mutants swaps an operator TOKEN textually, so `a || b || c` with the second `||` → `&&` becomes `a || (b && c)` by precedence. A test that sets `a` can never kill it (measured: `run.rs:139:36` survived every test that set `unit`). Test the selection with `a` false.
  - (b) `git check-ignore` on a not-yet-existing path returns 1 for a trailing-slash directory pattern, so a P5 gitignore probe on an unborn dir reads "not ignored" falsely.
  - (c) A mutation test with a "lacks failed" atom hits test NAMES containing "failed". Use nextest's summary token `failed,`.
  - (d) The test-plan regex form `crates/viola-e2e` never matches llvm-cov's Windows paths.

## Outcome
- **Acceptance criteria, re-asserted against the diff:**
  - (tests) `run --coverage` reports `coverage` and `doctest`, `ok:true`, on this host: **met**. The CI 3-OS floors and gate are **owed** to the CI witness after the push.
  - (tests) The planted-artifact `gate` breaches, exit 2 on an empty, unknown or `a11y` `--require`: **met** (13 `gate.rs` unit tests + `cli.rs` `gate_and_leg_usage_errors_are_exit_2`, `gate_over_an_empty_artifacts_dir_names_the_missing_suite`, all green).
  - (tests) The union logic and exactly 3 `file_mode` mutants: **met locally** (gate `cargo mutants --list … | grep -c` → 3; union unit tests green). The ubuntu leg catching all 3 is **owed** to the CI witness.
  - (tests) The fuzz-replay arms: **met** (6 unit tests + host arm exit 2). The CI `fuzz-replay` job is **owed**.
  - (tests) `zero_retries.rs`: **met**.
  - (tests) MSRV: **met locally** (check green on rustc 1.96.1; 191/191 unit). The CI `msrv` log showing `rustc 1.96` is **owed**. verification-matrix.json#v1-19 is `implemented`.
  - (arch) `fuzz/` is its own workspace and excluded; `cargo deny check` and the deny-sync runs are green; no `cc` in the root graph: **met**.
  - (arch) `rust-toolchain.toml` still reads `1.98.1`: **met** (last line 1).
  - (security) zizmor default is clean; every `uses:` SHA-pinned; 0 `${{` in `run:` bodies: **met**.
  - (security / obs) No `mutants.out` upload remains (grep → 0); verdict files carry names only: **met**.
  - (obs) G2, G4 and the scan are unchanged after the instrumented run; G1/G3 are green with `fuzz/`: **met locally** (G1/G3 gates green; the CI test job is the witness).
  - (a11y) No a11y step in `nightly.yml` (grep → 0); no `a11y` suite: **met**.
  - (security) Corpus paths: **met** (grep → no output).
- **Gates** (implement run `2026-09-24T14-09-21-implement`, second full block; entry names by `run` text):
  - `cargo build --workspace --features fake-agent` green · `cargo fmt --all --check` green · `cargo clippy … -D warnings` green;
  - `agent-run.sh run --unit` green · `run --integration` green · the `run --unit --filter` (gate/secret_scan/fuzz_replay/coverage/leg/selection) green · the `run --integration --filter` (unbuilt_selectors/gate_/zero_retries/leg_) green;
  - `cargo mutants --list … | grep -c 'replace file_mode'` green (last line 3);
  - `run --coverage` green (artifact fresh: product lines 97.86 / functions 95.40 / regions 98.05) · `gate --require coverage,doctest` green · the `llvm-cov-summary.json` viola-e2e count green (last line 0);
  - `RUSTUP_TOOLCHAIN=1.96 … cargo check` green · `RUSTUP_TOOLCHAIN=1.96 … cargo nextest run … kind(lib)|kind(bin)` green (191 passed);
  - `cd fuzz && rustup toolchain install && cargo check --bins` green · `run --fuzz-replay` green (exit 2, `fuzz-linux-only`);
  - the corpus path grep green · the `rust-toolchain.toml` pin grep green · the nightly a11y grep green · the `mutants.out` grep green · the `${{` run-body probe green;
  - `install-ripgrep.sh` green · G1 green · G3 green · `lint-probes.sh` green · `cargo deny check` green · the deny-sync loop green · `deny-probes.sh` green · `zizmor .github/workflows/` green;
  - `zizmor --persona pedantic` **recorded**: exit 12, 2 low `concurrency-limits` findings. This is the declined finding, kept visible by design.
  - `env::set_var|env_clear` grep green;
  - smoke `cleanup` / `boot` / `status` / `logs` / `cleanup --session gate-smoke`: all green;
  - `AGENT_RUN_CHUNK_BASE=39c3d2bf… run --mutants --leg windows-host` green (118 mutants: 110 caught, 2 missed, 6 unviable; `verdict:"counted"`, leg `windows-host`);
  - `gate --require mutants --mutants-legs windows-host` **recorded**: exit 1, 2 red names (`fuzz_host_supported -> false`, `file_mode -> None`), both equivalent on Windows. Disposition: killed only by the Linux leg by construction; the CI union is the verdict.
  - The 4 `leg = 'operator'` entries were not fired (the tool never fires a leg). Their treatment: the push is this wrap's P7 push, and the three CI reads (check-runs all `success`; the ubuntu verdict file holding 3 caught `file_mode` mutants; the `msrv` log with `rustc 1.96`) are **owed after the push**. Nothing recorded yet.
  - **Smoke:** hand-driven `impl-smoke-qg`, boot ready, status `ready`, cleanup `processes_gone:true`, `killed:[]`.
- **Outcome basis:** implement's P4 report in this conversation. No operator directive came between implement and this report.
- **Process hygiene** (implement's census, re-read against the host list by PowerShell `Get-Process`):

  | process | started by | final state |
  |---|---|---|
  | gate-block tools | this run | terminated |
  | smoke wrapper 52796, child 27980, supervisor | this run | terminated |
  | `viola.exe` 12172 and 56736 (viola-lab prototype, another tree) | the operator | left running: not this project's |
