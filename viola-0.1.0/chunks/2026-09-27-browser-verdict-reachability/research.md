# Codebase Research — 2026-09-27-browser-verdict-reachability

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 11 · **Graph queries:** 1 (rust plane)
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full, 5 Session Additions. Two bear on this chunk: `run --mutants` mutates only diff lines (2026-09-24), and "never pipe `boot`" (2026-09-25).
  - `.claude/rules/testing.md`: read in full, 15 Session Additions. Those applied:
    - 2026-09-24: a wait below the kill line, extended 2026-09-27 to "strictly below" plus a streamed report.
    - 2026-09-25: the remove-the-guard pair.
    - 2026-09-24: every function needs an observable effect.
    - 2026-09-27: force the window open, never sample.
    - 2026-09-27: remove slow work, never raise a timeout.
  - `.claude/rules/host-win32.md` (always loaded): the MSYS arg conversion hit the WSL probe below; `MSYS2_ARG_CONV_EXCL='*'` fixed it.
- **Platform issues consulted** (fetched 2026-09-27; these are the stated contents, not search summaries):
  - `actions/runner-images` `README.md`: `ubuntu-latest` = Ubuntu 24.04, `macos-latest` = macOS 26 Arm64.
  - `images/ubuntu/Ubuntu2404-Readme.md` (image 20260920.314.1): Node.js 22.23.2, npm 10.9.8, Chromium 153.0.8010.0. Playwright is not listed.
  - `images/windows/Windows2025-Readme.md` (image 20260922.270.2): Node.js 22.23.2, npm 10.9.8.
  - `images/macos/macos-26-arm64-Readme.md` (image 20260907.0351.1): **Node.js 24.20.0**, npm 11.19.0.
  - `playwright.dev/docs/intro` System requirements: Node "latest 22.x, 24.x or 26.x"; Linux "Debian 12 / 13, Ubuntu 22.04 / 24.04 / 26.04 (x86-64 or arm64)"; macOS 14+; Windows Server 2019+ or WSL.
  - `npm view @playwright/test@1.63.0` (this host, 2026-09-27): `version = '1.63.0'`, `engines = { node: '>=20' }`, `dist-tags.latest = '1.63.0'`.

## Files inspected
- `crates/viola-e2e/src/harness/run.rs` (1–345) — `Selection` (35–57), `from_flags` (48), `run_with` (111–142), `archive_sources` (146–159, JUnit suites listed by name), `test_suites` / `tool_arms` (216–280), `document` (284), `merge_summary` (320). No `browser` token in the file (`grep -nE 'browser|Browser'` found 0 hits here).
- `crates/viola-e2e/src/bin/viola-harness.rs` (20–245) — `RunArgs` has no `--browser` flag (76–98). `main` maps any clap error to `Outcome::usage(cmd, "arguments")` (206–217). So today `run --browser` exits 2 `{"reason":"usage","detail":"arguments"}`. That already satisfies test-plan §3's "an unbuilt selector is a usage error", and it is the arm this chunk replaces.
- `crates/viola-e2e/src/harness/run/fuzz.rs` (1–80) — the OS-gated arm precedent: `FUZZ_HOST_SUPPORTED` is a const (not a fn: "a fn body equal to one OS's answer is an unkillable mutant"). The refusal `tool-missing` + detail comes before the suite, the suite is built by counting targets, and `run_with` returns exit 2 `fuzz-linux-only` before anything runs (run.rs:119–122).
- `crates/viola-e2e/src/harness/run/nextest.rs` (full) — `junit_suite` copies a JUnit source to `artifacts/junit-<suite>.xml`. A missing source is `failed:1, failures:["artifact-missing"]`. `exit_must_agree` turns a red tool exit red even with nothing red in the report. `parse_junit` counts `<failure|error>` / `<skipped` / pass per `<testcase`. The stale rule is at nextest.rs:50: the source is deleted before the invocation.
- `crates/viola-e2e/src/harness/gate.rs` (1–120) — `SUITES` already holds `"playwright"` (gate.rs:15–25), and so does `JUNIT` (27–34). `summary_checks` breaches `suite-missing` / `suite-failed` / `suite-skipped` for every suite, and `artifact-missing` for a JUNIT suite without `junit-<suite>.xml`. **The gate needs no product change** for `--require playwright`. A test exists (`gate_fails_failed_and_skipped_suites_playwright_included`, gate.rs:329).
- `.github/workflows/ci.yml` (1–146) — the `test` job is the 3-OS matrix (`[windows-2025, macos-latest, ubuntu-latest]`, ci.yml:18), and its steps are:
  1. `run --coverage` per shim (36–45);
  2. the harness lifecycle (47–71);
  3. `jq --version` (75–77), G2 (78–83), G4 (84–88), the `failure()` capture (89–95), `secret-scan` (96–100);
  4. the scan-gated `diag-`/`harness-`/`junit-<os>` uploads (102–128);
  5. `Gate verdict` `gate --require coverage,doctest` (138–141).

  No `setup-node`, no `npm`, no Playwright token in the file.
- `scripts/wsl-provision.sh` (full) — the pins come from `rust-toolchain.toml` and ci.yml's `tool:` line (`pins()` 38–41). `need()` prints a `wsl.exe -d Ubuntu -u root --exec /usr/bin/apt-get install -y build-essential` line for a missing root package; it is never sudo (18–26). `--probe` proves 2 refusals: the checksum and the pin-mismatch (112–133).
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (1–160, 270–375):
  - `Linux::cmd_env` runs `env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin` (45–60).
  - `ci_pins` reads only the llvm-cov tool line (88–100).
  - `tools()` checks `cc` and then each pin's `--version` word 2 (124–157).
  - `linux_tests` runs `run --coverage`, then `gate --require coverage,doctest` (321–340).
  - `summary()` reduces a doc to codes and counts and drops `artifact` paths (287–308).
- `crates/viola-e2e/src/harness/pre_push.rs` (175–260) — the host stages are `windows-tests` (`run_with` coverage + gate, 247–260) and then `windows-leg`. Each builds `Selection { …, ..Selection::default() }`.
- `crates/viola-e2e/src/harness/secret_scan.rs` (1–40) — the scan reads the home diagnostics, the harness capture (`target/agent-run`) and the nextest JUnit. `junit-playwright.xml` copied into `target/agent-run/artifacts/` is inside that scope.
- `.gitignore` (29–35) — `node_modules/`, `*.tsbuildinfo`, `e2e-web/test-results/`, `e2e-web/playwright-report/`, `e2e-web/pw.json` and `e2e-web/pw-junit.xml` are **already ignored**, and the lockfile "IS committed". No `.gitignore` edit is owed.
- `scripts/code-graph.py` (1–68) — `detect_planes` adds `ts` for any tracked or unignored `tsconfig.json` (`git ls-files … tsconfig.json */tsconfig.json`, 55–68). A missing `scip-typescript` skips the plane with the recipe. On this host `type scip-typescript` answers `/d/dev/node/npm/scip-typescript`, so the ts plane will build here once the tsconfig lands. From then on every query needs its plane argument (two planes).
- The 8 folded waits, read at their loops:
  - `tests/support/fake.rs:12,44–55` `wait_for`: polls the fake-agent receipt ndjson for `pred`; panics `"timed out waiting for {what}"`.
  - `tests/support/home.rs:19,221–237` `wait_ready`: polls `Starts` (wrapper · child · receipt counts), `snapshot_ready`, `beat_fresh` and `pty.try_wait()` (exit-aware).
  - `tests/support/outer_pty.rs:15,88–97` `wait_exit`: polls `try_wait`. `EXIT_WITHIN` is `pub` and also bounds `finish()` (108) and `Drop` (118–125).
  - `tests/run_cli.rs:19,33–51` `wait_raw`: polls the receipt for `"kind":"start"` (exit-aware via `exited()`). At `:262` it polls `run-builder.ndjson` for `"subject":"claude-child"`.
  - `tests/cli_instance_state.rs:220`: polls `ps -o stat=` after `-STOP` (Unix).
  - `tests/contract_diag_schema.rs:244`: polls the raw-terminal receipt, and `:264` polls `child.try_wait()`.

## Graph impact
The rust plane DB was `fresh`, with 24 rows. The trace is `{run_dir}/tree-query-2026-09-27-browser-verdict-reachability.json`. The query was canonical query 1 over `run_with`, `linux_tests`, `test_suites`, `tool_arms` and `junit_suite`.
- **`run_with`** — production callers are `run()` (run.rs:90), `run_cmd` (viola-harness.rs:154), `pre_push::stages` (pre_push.rs:155, 184) and `windows_tests` (pre_push.rs:251). The rest are tests in `run/{coverage,fuzz,mutants}.rs` and `run.rs`. A new `browser` selection must not change what those callers select by default.
- **`test_suites` / `tool_arms`** — called only from `run_with` (run.rs:126, 128). A browser arm threads through one of them.
- **`junit_suite`** — called from `nextest()` (nextest.rs:67) and `coverage()` (coverage.rs:45). A Playwright arm can reuse its copy-and-count shape. `junit_suite` hard-codes `junit_source(ws)`, so it needs a source parameter or a sibling fn.
- **`linux_tests`** — called only from `pre_push::stages`. The browser step joins it, or a new stage follows it before `vm-release`.

## Patterns detected
- **OS-gated arm as a const** (`run/fuzz.rs:11`): `pub const FUZZ_HOST_SUPPORTED: bool = cfg!(target_os = "linux");`. The early exit-2 refusal is at run.rs:119–122. If the browser arm keeps any OS gate, it takes this shape.
- **Tool refusal before the suite** (`run/fuzz.rs:43–55`): `Refusal::new("tool-missing", Some(detail))` and no suite. test-plan §3 makes a missing Chromium or a failed `npm ci` a **suite failure** instead (`suite:"playwright"`, `failed ≥ 1`, `reason:"browser-missing"`), so the arm pushes a red suite *and* sets the reason.
- **Fresh-file-only report** (`run/nextest.rs:50`, `:27–33`): delete the source before the invocation. A missing source afterwards is `artifact-missing`, never an earlier run's file.
- **Runner seam for tools** (`run.rs:32`, `Runner`): the harness's own tests drive tools through a stand-in runner and never nest the real tool inside nextest. The browser arm's unit tests stand in for `npm` / `npx` the same way.
- **Pins parsed from ci.yml** (`scripts/wsl-provision.sh:38–41`, `pre_push/linux.rs:88–100`): one version source per tool, and a mismatch is `tool-pin-mismatch` naming the tool.
- **Root package printed as a provisioning line** (`scripts/wsl-provision.sh:21–23`): a root-only apt dependency is refused with an install line for the operator, never installed with sudo. This is the shape for Playwright's Chromium system libraries in the distro.
- **Recorder** (`crates/viola-pty/src/lib.rs:500–523, 566–568`):
  - `CHILD_WITHIN` is 7 s.
  - `report_path()` is `<temp dir>/viola-pty-watch/<thread name with :: → .>.report`, removed at start.
  - `impl Drop` removes the report only when not panicking.
  - The panic message carries the report so far.

## Conventions to follow
- **Outcome documents** (`viola-harness.rs:113–116`; test-plan §3): one JSON document per command, exit 0 / 1 / 2, `reason` from the closed list. Adding `browser-missing` extends the closed reasons, and adding a new `detail` for a pre-push stop needs its Decisions Log entry (tests-history, the interim-supervisor entry).
- **Key order** (`run.rs:282–302`): `v, cmd, ok, reason, detail, suites, mutants, archived`.
- **Archive** (`run.rs:146–159`): only the named JUnit suites are archived. Adding `playwright` to that filter puts `junit-playwright.xml` in `target/run-archive/<n>/` (never uploaded).
- **CI step shape** (`ci.yml:36–45`): the pwsh shim on Windows and the sh shim elsewhere, each exiting on a non-zero `$LASTEXITCODE`.

## New files to create
- `e2e-web/package.json` — `@playwright/test` pinned exactly at `1.63.0` (npm registry: exists, `engines.node >=20`). No test script (a11y-plan §11 CI).
- `e2e-web/package-lock.json` — committed, produced by `npm install` against the pin.
- `e2e-web/playwright.config.ts` — headless, one `chromium` project, `retries: 0`, `forbidOnly: true`, no `webServer`, `testDir: 'tests'`, `outputDir: 'test-results'`, and the JSON (`pw.json`) + JUnit (`pw-junit.xml`) reporters.
- `e2e-web/tsconfig.json` — `noEmit`, `strict`, `include` covering `e2e-web/**` only.
- `e2e-web/tests/<stub>.spec.ts` — one test whose title is selectable by `--grep`, carrying no `@a11y`/`@sc-*` tag. Its name must not take `bay-*` (reserved for Epoch 8).
- `e2e-web/<stub page>.html` — a static page opened by `file://` (see the premise closure below). Unstyled, with no script and no inline style.
- `crates/viola-e2e/src/harness/run/browser.rs` — the arm. `npm ci --prefix e2e-web`, then `npx --prefix e2e-web playwright test`, each through the `Runner` seam. It deletes `e2e-web/pw.json` / `pw-junit.xml` first, then counts from the report and copies it to `junit-playwright.xml`.
- Root test support for the recorder (a module under `tests/support/`, or an extension of an existing one). Its placement is P4's call.

## Files to modify
- `crates/viola-e2e/src/harness/run.rs` — `Selection` gains `browser` (35–43). The arm threads through `test_suites` or `tool_arms`. `archive_sources` lists `playwright` (149). Whether `from_flags` (48–56) adds `browser` under `--all` is P4's call (see Open questions).
- `crates/viola-e2e/src/bin/viola-harness.rs` — `RunArgs` gains `--browser`. The `Selection { … }` at :147 is the one **full literal** (no `..` base), so it breaks unless updated.
- The companion sweep was `grep -rnE 'Selection \{' crates src tests`: 16 hits.
  - 1 changes: viola-harness.rs:147, the only full literal.
  - 1 is the definition (run.rs:35).
  - The other 14 spread a base `..` line and need no change: `..Selection::default()`, `..flags(…)` or `..named`.
  - [corrected at P5] The first P3 classification also read `mutants.rs:581` as a full literal, because its awk knew only the
    `..Selection::default()` / `..Default` / `..named` / `..sel` spellings. That literal ends `..flags(false, false, true, false)`.
    The re-read counts any `..` line up to the literal's closing brace.
- `crates/viola-e2e/src/harness/run/nextest.rs` — only if `junit_suite` is generalised to take a source path (a signature change). Its callers are `nextest()` :67 and `coverage()` coverage.rs:45 (graph).
- `crates/viola-e2e/src/harness/pre_push/linux.rs` — `linux_tests` (321–340) gains the browser run and `--require …,playwright`, or a new stage does. `tools()` (124–157) gains the Node/npm/Playwright pin checks. `ci_pins` reads only the cargo tool line, so a Node/Playwright pin needs its own reader.
- `crates/viola-e2e/src/harness/pre_push.rs` — only if a new stage value or document key is added. The closed `stage` enum gets a Decisions Log entry (tests-history, pre-push entry).
- `scripts/wsl-provision.sh` — installs and checks the Node pin, `npm ci`, and `playwright install chromium`. The Chromium system libraries are printed as a root install line (no sudo). `--probe` extends to the new refusal.
- `.github/workflows/ci.yml` — in the `test` job, per OS: whatever Node step P4 settles on, `npm ci --prefix e2e-web`, the Chromium install (`--with-deps` on ubuntu only; apt), `run --browser` per shim, and `Gate verdict` `--require coverage,doctest,playwright`. Adding steps to the existing `test` job keeps the 8 jobs / 15 check-runs per push (arch-history, workspace-tree entry). Any upload of `pw.json`/`test-results` must be scan-gated and OS-suffixed.
- The 8 waits: `tests/support/fake.rs:12`, `tests/support/home.rs:19`, `tests/support/outer_pty.rs:15`, `tests/run_cli.rs:19` and `:262`, `tests/cli_instance_state.rs:220`, `tests/contract_diag_schema.rs:244` and `:264`.
  - Re-derived with `grep -rnE 'from_secs\((1[0-9]|[2-9][0-9])\)' tests crates src --include=*.rs`: 9 hits under `tests/`. 8 are the waits. `tests/cli_instance_state.rs:239` is an mtime back-date, not a wait. The list matches the handoff's.
  - Outside `tests/` the same grep hits:
    - `crates/viola-e2e` (boot/cleanup/supervise 10–30 s): viola-e2e runs under its own 15 s×2 = 30 s mutants override, so these are out of scope.
    - `src/bin/viola-fake-agent.rs:23` `HOLD_FOR` 10 s: a grandchild that holds stdout for `--exit-no-eof`. It is a child's hold, not a test wait. The tests that use it must see viola exit well before it, and a 7 s `EXIT_WITHIN` keeps that true. No change; it is noted so the implementer does not "fix" it.

## Measured facts (this host, 2026-09-27)
- WSL `Ubuntu` is release 26.04 (`lsb_release -rs`). Under the pre-push `env -i HOME PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, `command -v node` and `command -v npm` both answer absent, and there is no `~/.cache/ms-playwright`. Provisioning must install Node and Chromium, and Node must land on a PATH component derived from the distro home (a constant, like `.cargo/bin`) or the leg cannot see it.
- The host has Node v24.13.1 and npm 11.8.0 (`node --version`, `npm --version`).
- The CI verdict at Setup 5a has now settled: `49f644472a11 verdict: green · checks 15/15 · wall 235 s · runs ci#36340086338 completed/success` (`ci.py conclusion`, re-read at P3). For CARRY 2's watches, this is the **2nd of 3** consecutive green CI runs with no recurrence of either watched name. The 1st was ci#36333711860 on `f0e6dbc`.

## Open questions
- **Node source for "one set of pins".** The runner images disagree: ubuntu and windows are 22.23.2, macOS is 24.20.0. "Runner-image Node, no setup-node" (test-plan §12 Browser caching; security-plan "no toolchain action") therefore gives no pin that the WSL leg could copy, as security.md requires ("CI's own pins only"). The choices:
  - (a) A pinned Node download step: the official tarball or zip, sha256-checked, per OS, with no action. CI and WSL read one pin line.
  - (b) A SHA-pinned `actions/setup-node` with an exact version. This reverses "no toolchain action" and needs live ratification (security-history, the wrapper-channel entry).
  - (c) Runner-image Node in CI, and a Node pin that WSL invents. That breaks CI-pins-only.

  → blocks: plan-decision (P4 fork).
- **`--all` and the local Windows host.** Adding `browser` to `--all` makes every local `run --all` need Node and Chromium. The CI `test` job calls `--coverage` and `--browser` separately, never `--all`. Whether `--all` includes `browser`, and whether the pre-push host `windows-tests` stage also runs the browser (the entry names only the WSL leg), is a scope choice. → blocks: plan-decision.
- **npm lockfile audit.** Either wire an `npm audit` advisory and sources step into `supply-chain` now (the fuzz-lock template), or record a test-only exemption, which needs the founder's live ratification (security-history, wrapper-channel). → blocks: plan-decision.
