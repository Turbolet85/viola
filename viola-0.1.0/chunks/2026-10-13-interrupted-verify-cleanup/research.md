# Codebase Research — 2026-10-13-interrupted-verify-cleanup

## Scope
- **Depth:** deep (one verb's whole run path) · **Reads:** 14 · **Globs/Greps:** 16 · **Graph queries:** 1 (this run dir's
  `tree-query-2026-10-13-interrupted-verify-cleanup.json`, plane `rust`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 16 Session Additions; applied: the
  kill filter `test(/(^|::)tests::name(::|$)/)` (2026-09-27, corrected 2026-10-13), a whole root file by
  `binary(<stem>)`, the fake agent's receipt and `Exits on \x03 … session leader` facts, `schema-check` reads kept homes
  only (2026-10-11). `.claude/rules/testing.md` — read in full, 42 Session Additions; applied: the 10 s `mutants` kill
  for a root test without a `verify_window_` prefix (2026-09-24, extended 2026-10-06), a kill only at a process the test
  started (2026-10-10), `cfg(unix)`-only bodies kept to a minimal OS reader (2026-09-24), cfg-only imports (2026-10-03),
  the Windows-target check-only clippy (2026-10-03, extended 2026-10-10), the remove-the-guard run (2026-09-25).
- **Platform issues consulted:** none — no runner-only bullet: the Setup 5a read of `d9f01b1` was `in progress` both
  times (`ci#38446140379`, 15/15 checks, wall 706 s at the P3 re-read), with no red to fold.
- **External inputs:** `inputs#I1` — the operator's directions for this chunk (no mutation run, witness run or workflow
  dispatch; no live `claude` start without the founder's cap; sized against one window at P5; every record time from
  `date -u` in the same call).

## Files inspected
- `src/cmd/verify.rs` (1–540) — `verify` (`:52`) opens the obs init and calls `measure` (`:66`); `measure` (`:102`)
  resolves the default program as the literal `OsString::from("claude")` (`:105`), runs the version probe and the print
  probe through `run_logged` → `run_bounded` (`:149`, `:171`), creates `ProbeDir` (`:165`), calls `typed::measure`
  (`:187`), drops the probe dir (`:209`), then prints the step lines in `check_rows` (`:354`, one `human::result` per
  row), writes the stamp (`update_stamps`, `:214`), prints `stamped …` (`:226`) and runs `record` (`:233`). `ProbeDir`
  (`:244`) is `<home>/ledger/probes/<pid>/`, removes a same-pid path first (`:263`) and removes its dir in `Drop`
  (`:304`).
- `src/cmd/verify/typed.rs` (1–760, 1118–1134) — `measure` (`:63`): Run A in a `tempfile::Builder::new()
  .prefix("viola-verify-").tempdir()` (`:68`) ended by `end_by_kill` (`:77`), then `trusted_run` (`:87`, `TrustedDir`
  `""`, ends by `end_by_ctrl_c` `:116` or `end_by_kill` `:111`), `dialog_run` (`:125`, `-dialogs`, `--settings` literal
  `:132`), `plan_run` (`:168`, `-plan`, `--settings` literal `:173`). Waits: `wait_for` (`:195`), `wait_capture`
  (`:328`), `ended_by` (`:559`), `Run::settle` (`:639`), `Run::box_wait` (`:655`) — each a `POLL` (5 ms) loop on the
  injected `Clock` with no stop input. `TrustedDir` (`:351`) removes a same-pid path first (`:356`) and its dir in `Drop`
  (`:363`). `Run::kill` (`:675`) kills through the PTY seam with no key; `finish` (`:705`) waits the pump up to
  `PROBE_DEADLINE` and logs the run's `process-exit`.
- `src/run/version_gate.rs` (20–125, 285–346) — `run_bounded` (`:38`): a `std::process::Command` child (stdin null, R8
  strip), a `POLL` (10 ms) `try_wait` loop to `deadline`, then `kill` + `wait`; called by verify's `run_logged` and by
  `run`'s `version_gate` (`:115`); its two unit cases spawn the test binary (`--list`) and a missing program.
- `src/main.rs` (47–60) — the panic hook is statement 1, `restrict_dll_search()` statement 2 (`:52`), dispatch inside
  `catch_unwind` (`:54`–`:60`): any handler `verify` installs is registered after both.
- `src/bin/viola-fake-agent.rs` (26–126, 540–680) — the option list (`:92`–`:126`); `submit` (`:555`) fires
  UserPromptSubmit, and only under `--turn-stop` ends the turn (`:574`), then writes the `prompt` receipt (`:579`);
  `start_receipts` (`:644`) writes `start`, then `cwd` (`:660`), at every start.
- `tests/cli_verify.rs` (1–190, 370–450, 728–806) — `probes_left` (`:77`), `assert_plain` (`:84`), the six-pair log
  case (`:398`, a killed run's exit has `duration_ms` and no `child_exit_status`), `verify_leaves_neither_run_dir_behind`
  (`:749`: `TMPDIR`/`TMP`/`TEMP` set per child, `cwds` from the receipt, `tmp` empty after).
- `tests/support/verify.rs` (280–456) — `verify_with` (`:425`) always passes `--screens --turn-stop`; every runner waits
  the child (`wait_bounded`, `:311`); no helper returns a running verify.
- `tests/support/cli.rs` (full) — `Running::over(child)` (`:26`), `finish` (`:36`, `try_wait` loop to `WITHIN`), killed
  on drop.
- `Cargo.toml` (12–24, 140–232) — root `[target.'cfg(windows)'.dependencies] windows-sys.workspace = true` (`:160`),
  `[target.'cfg(unix)'.dependencies] libc.workspace = true` (`:164`); the workspace `windows-sys` line (`:232`) lists
  `Win32_System_Console` among its features.
- `crates/viola-agent-claude/src/lib.rs` (`:117`, `:686`) — `pub const PROGRAM: &str = "claude"`, pinned by a literal
  case; `crates/viola-agent-claude/src/statusline.rs` (`:14`, `:373`) — `pub const SETTINGS_FLAG: &str = "--settings"`,
  pinned by a literal case.
- `schemas/diag-line.v1.json` — `process-exit.detail` is a closed enum of 11 members (`already-live` · `squatted-name` ·
  `pinned-hash-mismatch` · `batch-script-child` · `internal-error` · `instance-dead` · `strict-modes-failed` ·
  `server-verify-failed` · `wrapper-fault` · `cwd-missing` · `no-session`, read by a python load of the file);
  `exit_source` is `handle-wait | kill-fallback`.
- `.andromeda/playbook.md` (the `verdict: escalate` patterns) — read for P4's fork round.

## Graph impact (from the code-graph query)
- **`run_bounded`** — 2 product callers: `run_logged` @ `src/cmd/verify.rs:86`, `version_gate` @
  `src/run/version_gate.rs:115`; 2 unit cases (`:301`, `:321`). A changed signature reaches `run`'s version gate.
- **`typed::measure`** — 1 caller: `measure` @ `src/cmd/verify.rs:187`.
- **`end_by_kill`** — `measure` (`typed.rs:77`), `trusted_run` (`:111`), `dialog_run` (`:159`), `plan_run` (`:188`);
  **`end_by_ctrl_c`** — `trusted_run` (`:116`) only.
- **`settle`** — 7 sites in `typed.rs` (`:76`, `:105`, `:155`, `:185`, `:246`, `:289`, `:671`); **`wait_for`** — 4
  (`:148`, `:181`, `:269`, `:285`); **`wait_capture`** — 2 (`:240`, `:242`); **`box_wait`** — `framing_turns` (`:275`)
  and `Run::box_wait` (`:659`) plus 3 unit cases; **`ended_by`** — `end_by_ctrl_c` (`:697`), `finish` (`:707`) plus 1
  unit case.
- **`verify`** (the verb) — `cmd::dispatch` @ `src/cmd/mod.rs:154`; the root tests reach it through the `verify` helper:
  `tests/cli_verify.rs` (17 cases), `tests/cli_version_gate.rs:37`, `tests/contract_fake_agent_drift.rs:248`,
  `tests/contract_ledger_probes.rs:76`, `tests/support/home.rs:350` (`stamped_home`).
- Every line above is the graph's 0-indexed `line` + 1.

## Patterns detected
- **Drop-guard ownership** (`verify.rs:304`, `typed.rs:363`, the `tempfile::TempDir` at `typed.rs:68`): each class of
  dir is owned by the value that made it; removal happens when that value drops, so an unwound return removes all three
  classes and a process killed by a signal's default action removes none.
- **Polled waits on an injected clock** (`typed.rs:195`, `:328`, `:559`, `:639`, `:655`; `version_gate.rs:59`): every
  wait verify makes is a short `sleep(POLL)` loop that re-reads a predicate, so a stop flag read in the same loop ends
  the wait within one `POLL`.
- **Ending a run with no key** (`typed.rs:684` `end_by_kill`): the kill goes through the PTY seam and writes no byte to
  the child.
- **Spawn pairs at the call site** (`verify.rs:76` `run_logged`; `typed.rs:597`/`:715`): each child's
  `process-start` / `process-exit` is written by verify, never by `run_bounded`.
- **OS console code behind `viola-pty`** (`crates/viola-pty/src/lib.rs:296`, `:429`, `:469`, `:780`): the only
  `Win32::System::Console` users today.

## Conventions to follow
- **Refusal lines** through `human::refuse` (`src/human.rs:38`): one `unable: <text>` line and one `hint:` line, fixed
  text, no path or pid.
- **Self exit line** through `run::log_self_exit(code, detail)` (`src/run/mod.rs:76`), as `verify` does at
  `verify.rs:68`; a failing row logs exit 1 with no `detail` (`tests/cli_verify.rs:390`–`:391`).
- **`unsafe` in the root package** exists today (`src/panic_frames.rs:41`–`:89`, `src/run/env.rs:98`), each FFI call
  in its own block.
- **Oracles are literals**: `tests/cli_verify.rs` asserts the step lines, refusals and exits as literals (`:26`, `:65`,
  `:156`).

## Measured facts
1. **No handler today** (re-derived: `command grep -rn -i -E
   "sigterm|sigint|ctrlc|signal_hook|SetConsoleCtrlHandler|signal::|libc::signal|sigaction"` over `src` and `crates` →
   0 hits). Rust's std installs no SIGINT / SIGTERM disposition, so a stop by either takes the default action (process
   termination), which runs no `Drop`: the CARRY's mechanism ("a killed `viola verify` leaves the probe dir of the run it
   was in") holds for SIGINT and SIGTERM as well as SIGKILL. VERIFIED.
2. **The three classes, one guard each** (CARRY): `<home>/ledger/probes/<pid>/` = `ProbeDir` (`verify.rs:244`, `Drop`
   `:304`); Run A's `viola-verify-*` = a `tempfile::TempDir` under `std::env::temp_dir()` (`typed.rs:68`, dropped
   `:78`); `<cwd>/.viola-verify-<pid>{,-dialogs,-plan}/` = `TrustedDir` (`typed.rs:351`, `Drop` `:363`), one per run.
   Every class has a guard; no new guard is needed. VERIFIED.
3. **What reaches the stop on each OS.** Unix: portable-pty starts each PTY child as its own session leader (measured:
   tests-history `2026-10-04-running-turn-refusal`, the fake agent "is its PTY's session leader"), so a terminal's Ctrl-C
   reaches `viola verify` and its `run_bounded` child (same process group: `run_bounded` sets no group), never a PTY
   child; a `kill <pid>` reaches `viola verify` alone. Windows: a console Ctrl-C / Ctrl-Break reaches every process
   attached to that console, `viola verify` and its `run_bounded` child; the ConPTY children are attached to their
   pseudo console (`viola-pty`). So in every case verify itself must end the child it is running.
4. **The equality the stop needs**: with a stop flag set while verify is inside any of its waits, the wait returns
   within one `POLL` (5 ms in `typed.rs`, 10 ms in `run_bounded`); the run then ends its child by `Run::kill` (no key)
   and `finish` (pump joined within `PROBE_DEADLINE`, in practice at once after a kill: the child's PTY reaches EOF);
   `measure` returns before `check_rows`, so no step line, no `stamped` line and no `update_stamps` call; the guards drop
   in reverse order on that return. Producing frames: `typed.rs:195`/`:328`/`:559`/`:639`/`:655` (the loops),
   `:675`/`:684` (the kill), `verify.rs:209`–`:214` (drop, then stamp).
5. **A deterministic stop point exists with no new fake-agent option**: with `--screens` and the workspace as trusted
   root but without `--turn-stop`, Run B pastes the probe prompt, the fake agent fires UserPromptSubmit and writes its
   `prompt` receipt (`viola-fake-agent.rs:568`, `:579`) and fires no Stop (`:574`), so verify waits in
   `wait_capture(Stop)` (`typed.rs:242`) for up to `PROBE_DEADLINE` (120 s). A test that waits for the receipt's third
   `cwd` line and its first `prompt` line has an observed event, then signals the pid it started. No hook is in flight
   then (the UserPromptSubmit hook ran before the `prompt` receipt was written), so the kill hangs up no hook mid-exit.
6. **Windows reach**: the root package's `windows-sys` inherits the workspace feature list, which holds
   `Win32_System_Console` (`Cargo.toml:160`, `:232`); `libc` is the root's Unix dependency (`:164`). No manifest line and
   no `Cargo.lock` package is added by a handler on either OS. Architecture's crate-direction text names the root's
   `windows-sys` roles without the console (arch-history `2026-09-28-capability-ledger-and-viola-verify`): a wrap
   amendment, not a code change.
7. **The two literals**: `PROGRAM` and `SETTINGS_FLAG` hold exactly `claude` and `--settings`, each pinned by a literal
   case in its crate, and both are `pub` (`viola_agent_claude::PROGRAM` already read by `src/cmd/revive.rs:10`,
   `SETTINGS_FLAG` by `src/run/mod.rs:18`). Reading them changes no byte of any argv. VERIFIED.
8. **Same-pid pre-clean**: `ProbeDir::create` (`verify.rs:263`) and `TrustedDir::create` (`typed.rs:356`) remove a path
   of this run's own name (`<pid>`) before creating it. That is a leftover of an earlier process that had the same pid,
   removed by name. It predates the founder's narrow ruling (chunk 2026-10-05-real-cli-verify-probes) and this chunk
   leaves it unchanged; recorded so the P5 card shows it.
9. **Log shape**: `process-exit.detail` is closed at 11 values (fact above). An interrupted-run `detail` would be the
   twelfth: a `schemas/diag-line.v1.json` change plus an obs Decisions Log entry at the wrap (obs-history
   `2026-09-29-sideloaded-conpty`, the D-35/D-36 template). `exit_source: "kill-fallback"` is a `run`-PTY field; the
   verify-pty-probe exits already log a killed run as `duration_ms` without `child_exit_status`
   (`tests/cli_verify.rs:441`–`:443`).

## New files to create
- `src/cmd/verify/stop.rs` — the stop flag and its per-OS handler installation for `viola verify` (Unix SIGINT and
  SIGTERM, Windows console control events), with its inline unit cases.

## Files to modify
- `src/cmd/verify.rs` — installs the stop before the first child, reads `PROGRAM`, ends an interrupted run before the step lines and the stamp with its exit and line, the stop observed by the bounded children.
- `src/cmd/verify/typed.rs` — every wait observes the stop, a stopped run ends by kill, `measure` reports the stop, the two `--settings` sites read `SETTINGS_FLAG`.
- `src/run/version_gate.rs` — the shared bounded child's wait observes a stop input, the gate passing none.
- `schemas/diag-line.v1.json` — the twelfth `process-exit.detail` member, if P4 takes it.
- `tests/cli_verify.rs` — the stop cases against the fake agent.
- `tests/support/verify.rs` — a verify started and not waited, without the turn's Stop.

## Sweeps
- `"--settings"` over `src/` and `crates/` (`command grep -rn '"--settings"' src crates --include=*.rs`): 14 hits · 2
  changed (`src/cmd/verify/typed.rs:132`, `:173`) · 12 no-change (the fake agent's own option parser and its unit cases
  `src/bin/viola-fake-agent.rs:124`, `:967`, `:1045`, `:1348`–`:1360`, `:1380`; oracle literals in
  `src/run/mod.rs`'s `mod tests` `:264`, `:275`; the constant and its pin `statusline.rs:14`, `:373`).
- `"claude"` over `src/` (`command grep -rn '"claude"' src --include=*.rs`): 4 hits · 1 changed (`src/cmd/verify.rs:105`)
  · 3 no-change (the fake agent's own `claude` copy name `viola-fake-agent.rs:1222`; `src/main.rs:318`, `:329` rstest
  argv literals in `mod tests`).
- Signal and console-control names over `src` and `crates` (Measured fact 1's command): 0 hits.

## Open questions
- The interrupted run's exit status, human lines and log `detail` (architecture §Conventions holds no row; design and
  layout hold no form) → blocks: plan-decision (P4 fork).
- Which Windows console events count as TERM, and whether CTRL_CLOSE is held for cleanup → blocks: plan-decision (P4
  fork).
- How the Windows arm is proven on `windows-2025` (a direct call of the handler routine, or an event delivered to a
  `viola` in its own process group, unmeasured on this host) → blocks: plan-decision (P4 fork).
