# Codebase Research — 2026-09-28-hook-perf-gate

## Scope
- **Depth:** deep (mature codebase; the chunk extends the hook verb, the panic path, the harness `run` arms, `gate`
  and `ci.yml`) · **Reads:** 14 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 13 176 B) — 5 Session Additions
  applied (the `--in-diff` witness rule, the Windows `#[cfg(unix)]` leg rule, never pipe `boot`, judge a leg by its
  verdict, full-path nextest filters); `.claude/rules/testing.md` (read in full) — 15 Session Additions applied (the
  process-global `OnceLock` isolation, the observable-effect rule, bounded waits below the kill line, force a timing
  window open, per-crate security tests)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's only run, ci#36380661854 on `85ae5aa`, is
  green) and no CI-reading entry outside the operator leg

## Files inspected
- `src/cmd/hook.rs` (full, 379 lines) — `hook()` resolves the instance (`instance_of`, :49), then `viola_obs_init`
  (:81) sets the panic sink with the instance, then `handle` (:106). `SPINE_DEADLINE` = 750 ms (:33) is the
  `connect_by` deadline (:174). No seam yet. Unit tests at :220–:379 drive `handle` in-process.
- `src/main.rs` (:1–:240) — `main` installs `viola_panic_hook` first (:44), runs clap + dispatch inside
  `catch_unwind` (:46), and `exit_code` maps `(Role::Hook, _)` to `ExitCode::SUCCESS` (:104–:106), `Panicked`
  included. `viola_panic_hook` (:115) returns silently when `PANIC_SINK` is unset (:116); else `write_panic_lines`
  (:137) writes the codes-only role line (`panic_line`, :191: `event:"panic"`, `panic_location`, `thread`, no payload)
  with one `write_all` (:146), then — only when the sink has an instance (:147) — the detail line
  (`panic_detail_line`, :160: `panic_payload` + a `backtrace` array from `Backtrace::force_capture()`, :168) via
  `obs::write_detail`.
- `src/obs.rs` (:215–:319) — `internal_error_exit_line` writes only for `run` (:220–:231), so a caught hook panic
  adds no role line after the panic line. `write_detail` (:308) opens append and makes one `write_all` (:316–:318).
- `src/cmd/run.rs` (:40–:110) — the precedent seam `hold_pump_start` (:48): `#[cfg(feature = "fake-agent")]`, one
  read site, a closed parse. The wrapper's `hook.event` handler (:100–:105) only appends the event line: a timed
  SessionEnd does not stop the wrapper, so all perf rows can share one live session.
- `tests/hook_fail_open.rs` (full, 310 lines) — 11 rstest cases (:133–:144; `pre-tool-use` is the `unknown_event`
  case at :140, so that hook is an immediate no-op at HEAD), each asserting exit 0, empty stdout and stderr and
  `took < SPINE_BOUND` = 1 s (:23, :173–:176). The module doc (:5) says the forced-panic case waits for this chunk.
  `hook_processes_append_whole_lines_side_by_side` (:243) spawns 8 `viola hook session-start` processes: 16 role
  lines and 8 `parse-rejected` detail lines, none over 4 KiB.
- `crates/viola-e2e/src/harness/run.rs` (:1–:330; 751 lines) — `Selection` (:36) with `from_flags` (:51) keeping
  named-only arms out of the default and `--all`; `test_suites` (:236) and `tool_arms` (:278) are the arm wiring;
  `Refusal` (:99) carries `reason` + `detail`; the `Runner` seam (:33) lets harness tests stand in for tools.
- `crates/viola-e2e/src/harness/run/fuzz.rs` (:1–:120) — the sibling arm: a tool probe through the runner, then
  `Refusal::new("tool-missing", Some("cargo-fuzz"))` (:49), one `Suite` with per-target failures.
- `crates/viola-e2e/src/harness/gate.rs` (:1–:219) — `perf` is already a closed suite (:22); `summary_checks` (:93)
  requires the suite entry with `failed == 0` and `skipped == 0`; `perf()` (:193) reads every
  `artifacts/perf-*.json` and breaches on `results[0].max >= SPINE_DEADLINE_S` (1.0, :37) or an unreadable file, but
  only on an EMPTY set for absence (:202): a missing one of the required rows passes today.
- `crates/viola-e2e/src/harness/boot.rs` (:21–:100) — `BootOptions { ws, bin_dir, session, instances, cli_version,
  build }` (:56): `bin_dir` makes a `target/perf/release` boot possible, `build: false` skips the harness build
  (:105). No `--unstamped` option exists (no stamping at HEAD).
- `crates/viola-e2e/src/bin/viola-harness.rs` (:70–:170) — `RunArgs` (:77) and `run_cmd` (:147) build the one
  exhaustive `Selection` literal (:149).
- `.github/workflows/ci.yml` (:1–:195) — the `test` job sets `AGENT_RUN_KEEP_HOMES: "1"` (:28) and gates on
  `coverage,doctest,playwright` (:189). **G2** (:124–:129) fails on ANY `event:"panic"` line in any non-`detail-*`
  `diagnostics/*.ndjson` under `target/e2e-home`. taiki-e's tool line (:41) is the one `wsl-provision.sh` replays.
- `scripts/wsl-provision.sh` (:40–:170) — `pins` (:44–:47) greps exactly the `tool:` line holding
  `cargo-nextest@…cargo-llvm-cov@`; a separate `cargo install --locked hyperfine@…` step is not replayed.
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (index) — `linux_tests` (:343) runs `run --coverage`,
  `run --browser`, then `gate --require coverage,doctest,playwright`.
- `schemas/diag-detail.v1.json` (grep) — `panic` is in the inlined event enum (:14); `panic_payload` (:20) and the
  `backtrace` string array (:21) are admitted; a panic line needs `panic_location` and `thread` (:26–:27).
- `Cargo.toml` (grep) — root `[features] fake-agent = []` (:10); each root integration test is a `[[test]]` entry with
  `required-features = ["fake-agent"]` (e.g. `hook_fail_open`, :81–:84), and the harness nextest/coverage runs pass
  `--features viola/fake-agent` (run/nextest.rs:61–:62, run/coverage.rs:28–:29), so `CARGO_BIN_EXE_viola` in a root
  test is a `fake-agent` build.

## Graph impact (from the code-graph query; plane `rust`, db_state `fresh`, trace `tree-query-2026-09-28-hook-perf-gate.json`)
- **`hook`** — 1 caller, `cmd/dispatch()` @ `src/cmd/mod.rs:45`: the seam call inside `hook()` changes no signature.
- **`handle`** — 2 callers, `hook()` @ `src/cmd/hook.rs:88`, the unit helper `handled()` @ `src/cmd/hook.rs:288`:
  untouched by a seam placed in `hook()` before `handle`.
- **`write_panic_lines`** — 3 callers, `viola_panic_hook()` @ `src/main.rs:127` and two unit tests @ `src/main.rs:341`,
  `:375`: the panic path is reused unchanged.
- **`test_suites` / `tool_arms`** — 1 caller each, `run_with()` @ `crates/viola-e2e/src/harness/run.rs:130`, `:132`.
- **`from_flags`** — callers `run_cmd()` @ `crates/viola-e2e/src/bin/viola-harness.rs:160`, `flags()` @ `run.rs:388`,
  tests @ `run.rs:637`, `:639`, `coverage_only()` @ `run/coverage.rs:118`.
- **`perf`** (gate) — 1 caller, `gate()` @ `crates/viola-e2e/src/harness/gate.rs:79`.
- `Selection { … }` companion sweep: `rg "Selection \{" crates src tests` — 13 hits · 1 changed (the exhaustive
  literal at `viola-harness.rs:149`) · 12 no-change (each spreads `..` a default or named value, checked per hit
  within 10 lines: pre_push.rs:187, :248; run/browser.rs:313; run/coverage.rs:119, :143; run/fuzz.rs:221;
  run/mutants.rs:588; run.rs:382, :604, :628, :632, :640).

## Patterns detected
- **Feature-gated test seam** (`src/cmd/run.rs:44–:55`): one `#[cfg(feature = "fake-agent")]` fn, read at one site,
  a closed parse, nothing configured. `FAKE_AGENT_HOOK_PANIC` copies it.
- **Tool arm with probe and refusal** (`run/fuzz.rs:42–:87`): probe through `Runner`, `tool-missing` + the tool's
  name, one `Suite`, failures named.
- **Named-only arm** (`run.rs:47–:59`): `--browser` / `--fuzz-replay` join neither the default nor `--all`; `--perf`
  follows (test-plan §3: `--perf` runs the §10 gates).
- **Per-job gate tail** (`ci.yml:186–:189`, `:290–:293`, `:314–:317`): an `if: always()` `gate --require <suites>`.
- **Fail-open matrix** (`tests/hook_fail_open.rs:133–:222`): rstest cases, `Command::env` per child, the 1 s bound.

## Conventions to follow
- **Seam semantics**: a closed trigger value (test-plan §5's Vector 6 sets disabling-shaped values `0`, `false`,
  `off` and empty, so a presence trigger would fire on them). Lean: fire only on the exact value `1`.
- **The seam fires after `viola_obs_init`** — the equality the design needs: a panic raised after `hook.rs:81` finds
  `PANIC_SINK` set with `instance = Some(name)` (obs.rs:131), so it yields exactly one role line in
  `diagnostics/hook-<name>.ndjson` and one detail line in `instances/<name>/diagnostics/detail-hook.ndjson`; a panic
  before it yields nothing (main.rs:116). Either way the exit is 0 (main.rs:106).
- **No `std::env::set_var`**; `Command::env` per child (testing.md §Determinism).
- **Keep `run.rs` from growing**: at 751 lines it takes the new arm as a sibling module `run/perf.rs`, as `fuzz.rs`
  and `browser.rs` are.

## New files to create
- `crates/viola-e2e/src/harness/run/perf.rs` — the `--perf` arm: the `target/perf` release build, one booted perf
  session under `target/e2e-home/`, synthetic hook payloads the harness writes (no recorded fixtures exist yet:
  `fixtures/` holds only `fake-scripts/`), hyperfine per row, the zero-panic check, cleanup, the `perf` suite
- `tests/cli_controls_not_disableable.rs` — the seam's rstest row over the hook-path controls (P4 fork 3)
- `src/cmd/hook/seam.rs` — the forced panic, the only statement of a `fake-agent`-gated fn; the module is declared
  unconditionally, since `scripts/orphans-check.sh` analyses the `viola` bin without features (its target rows carry
  only `required-features`, :13–:32) and a cfg-gated `mod` file reads as orphaned (its header, :5–:7, measured)
- `scripts/g2-zero-panics.sh` — G2 as one script both jobs call, with the exact-path seam exemption and a `--probe`
  that proves a panic line in any other file (and a look-alike path) still fails (P4 fork 2)

## Files to modify
- `src/cmd/hook.rs` — the `FAKE_AGENT_HOOK_PANIC` seam after `viola_obs_init`, with its unit test
- `tests/hook_fail_open.rs` — the forced-panic fail-open case and the over-4 KiB concurrent detail line
- `crates/viola-e2e/src/harness/run.rs` — `mod perf`, `Selection::perf`, the arm wiring
- `crates/viola-e2e/src/bin/viola-harness.rs` — `RunArgs --perf` and the exhaustive `Selection` literal
- `crates/viola-e2e/src/harness/gate.rs` — require every named perf row, not just a non-empty set
- `.github/workflows/ci.yml` — the new per-OS `perf` job (P4 fork 1), and the `test` job's G2 step calling the script
- `Cargo.toml` — the `[[test]]` entry for `cli_controls_not_disableable` with `required-features`
<!-- P4 fork 1 took the own-job branch: pre_push/linux.rs, scripts/wsl-provision.sh and the WSL allowlist stay
untouched, and CARRY 3 moves on. -->

## Open questions
- **Job shape (CARRY 1):** own per-OS `perf` job (test-plan §9) vs a step inside `test` (obs-plan §9 / §10) → blocks:
  plan-decision. Own job: its own G2 / G4 / secret scan and uploads under distinct names; `test` and pre-push stay as
  they are, and CARRY 3 moves on. Inside `test`: G2, G4, the scan and `diag-<os>` cover the perf home as they stand,
  but the gate list grows and pre-push's mirror of it would need hyperfine in WSL, which triggers CARRY 3.
- **The deliberate panic against G2** (ci.yml:124–:129 under `AGENT_RUN_KEEP_HOMES: "1"`, :28): every forced-panic
  test leaves an `event:"panic"` role line in a kept home under `target/e2e-home`, so G2 goes red as written, and
  obs-plan has no exemption → blocks: plan-decision.
- **The `cli_controls_not_disableable.rs` row:** test-plan §5's table re-runs four negatives (`send` ESC → 13,
  `answer` unstamped → 12, human wheel → 10, a 0770 `--home` → 21) and a completeness case; none of the verbs or
  controls they need exists at HEAD (`src/cmd/` = `hook.rs`, `run.rs`, `mod.rs`) → blocks: plan-decision.
- Note (implementation-scope): the forced-panic case keeps the 1 s bound, but `force_capture()` symbolisation cost on
  a Windows debug build is unmeasured; implement measures it before asserting.
