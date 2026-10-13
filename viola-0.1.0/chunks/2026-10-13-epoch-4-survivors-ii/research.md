# Codebase Research — 2026-10-13-epoch-4-survivors-ii

## Scope
- **Depth:** moderate · **Reads:** 12 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md`. Structural read: the section and entry index
  (`grep -n '^## \|^- 20'`), then the 2026-09-27 filter entry and the 2026-09-29 local-grading entry read whole.
  1 addition applied: the case filter `test(/(^|::)tests::name$/)`, a red counted only with the case's own panic line.
  `.claude/rules/testing.md` was read in full, its 42 Session Additions included. Applied: 2026-09-24 (an effect a
  test can observe; test waits bounded below the mutants kill; operator-token mutants); 2026-09-25 (the
  remove-the-guard run); 2026-09-27 (forcing a timing window); 2026-09-28 (no reshaping for mutants); 2026-10-10 (a
  kill only at a process the test started); 2026-10-13 (a `-` mutant that panics before its assertion). No live leg.
- **Platform issues consulted:** none. No runner-only bullet, and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the take-up direction: no mutation run, witness run or workflow dispatch; a
  kill shown by a hand-applied mutant; each survivor worded by what the next boundary audit reads; no live `claude`
  start without the founder's cap; sized against one window at P5.

## Files inspected
- `src/cmd/verify/typed.rs` (full, 914 lines at HEAD `64d7e5f`). The 16 sites, read at their columns:
  - `framing_turns` `:257-293`, its waits at `:270-272` and `:286-289`;
  - `SharedPty` `:385-421`;
  - `Run` `:526-687`: `end_by_ctrl_c` `:646-655`, `wait_pump` `:658-666`, `finish` `:670-686`;
  - the test module `:689-914`: 17 cases, none builds a `Run`, a `SharedPty` or a framing wait.
- `crates/viola-pty/src/lib.rs` `:122-132` (the `Pty` trait, seven methods, `#[cfg_attr(test, mockall::automock)]` at
  `:121`) and `:181-235` (`impl Pty for PortablePty`: `close` sets `master = None`; `child_pid` is the child's
  `process_id()`).
- `crates/viola-pty/src/pump.rs` `:15-224`, which covers `pump_with_paste`, `pump_open`, `drain` and `kill_and_reap`.
- `src/cmd/verify.rs:37` (`PROBE_DEADLINE = 120 s`) and the `measure` call at `:187`.
- `crates/viola-core/src/lib.rs:23-35` (`Clock: Send + Sync`, `now(&self) -> Instant`; `SystemClock`).
- `crates/viola-agent-claude/src/ledger.rs:467-474` (`parse_capture_file_name`: `<Event>.<k>.json`, `k > 0`).
- `Cargo.toml` `[dev-dependencies]` `:145-157` (no `mockall`) and `[features]` `:9-10` (`fake-agent` only).
- `.config/nextest.toml` `:17-60`: `profile.mutants` 5 s × 2 by default; the `verify_window_` override is 15 s × 3.
- `src/run/send/test_support.rs:21-62`: `FixedClock`, `JumpClock` and `SwitchClock` are `pub(super)` to `run::send`.
- The earlier chunk's `plan.md` `:1-90`, `scope.md` and `evidence/survivors.ndjson` (its 18 `typed.rs` rows).
- `.andromeda/test-plan.md:1226` (§10 Mutation gate paragraph, the earlier disposal's record).

## Graph impact (rust plane, trace `tree-query-2026-10-13-epoch-4-survivors-ii.json`, three queries)
- **`framing_turns`**: 1 caller, `trusted_run` @ `src/cmd/verify/typed.rs:115`. It calls `count_of` @ `:259`,
  `wait_for` @ `:270`, `:287` and `box_wait` @ `:276`.
- **`end_by_ctrl_c`**: 1 caller, `trusted_run` @ `:116`.
- **`wait_pump`**: 3 call sites, `end_by_ctrl_c` @ `:648`, `:651` and `finish` @ `:671`.
- **`finish`**: 2 callers, `end_by_kill` @ `:642` and `end_by_ctrl_c` @ `:654`.
- **`end_by_kill`**: 4 callers, `measure` @ `:77`, `trusted_run` @ `:111`, `dialog_run` @ `:159`, `plan_run` @ `:188`.
- **`SharedPty`**: 3 references, all in `typed.rs` (`:387`, `:393`, `:570`). Its `resize` / `close` / `child_pid`
  have no resolved caller. They are reached only through `&mut dyn Pty` inside `viola_pty::pump_with_paste`, a
  dynamic dispatch the graph cannot resolve, so this is not leaf evidence. The real reach is in §Measured facts 2.
- **`measure`**: 1 caller, `src/cmd/verify.rs:187`. `typed.rs` exports only `measure`, `Inputs` and `RunDirs` (`pub(super)`).
- No symbol named here has a caller outside `src/cmd/verify/`. The restatement's blast radius is one file
  (graph lines are 0-indexed; every editor line above is +1).

## Measured facts (each re-derived at HEAD `64d7e5f`)
1. **No `Pty` double seam today.** `SharedPty(Arc<Mutex<PortablePty>>)` @ `typed.rs:385` and `Run.pty:
   Arc<Mutex<PortablePty>>` @ `:528`. viola-pty's `MockPty` exists only in viola-pty's own test build
   (`lib.rs:121`, `cfg_attr(test, …)`), and the root package declares no `mockall` (`Cargo.toml:145-157`). So a
   double in `typed.rs`'s tests is hand-written.
2. **Which `SharedPty` forwarders the product path reaches** (`pump.rs` read whole):
   - `resize`: the pump calls it only when `host_size()` returns a size other than the last
     (`pump.rs:186-192`). Verify's pump is given `&mut || None` (`typed.rs:574`), so verify never resizes.
   - `child_pid`: no call in `pump.rs` (`grep -n child_pid crates/viola-pty/src/pump.rs`: 0 hits).
   - `close`: the pump DOES call it, at `pump.rs:171` (child exited) and `:219` (`kill_and_reap`). Its mutant
     `→ ()` leaves the master open until the `Run` drops its `Arc`, which nothing in the root package's tests observes.
   The earlier chunk's measured facts 4 and 5 are verified as written. The one addition: `close` is reached.
3. **The ending's arithmetic.** It reads the injected `Clock`: `self.clock.now() + CTRL_C_AGAIN` @ `:648`,
   `+ PROBE_DEADLINE` @ `:651`, `:671`. The pump's own loop reads real time (`pump.rs:167`, `TICK` 20 ms). So no
   test can steer a whole `Run` through one clock. The deciding parts must be reachable without a pump.
4. **The equalities each kill needs**, at the restated sites (the plan's mechanism, stated as X yields Y):
   - `count(UserPromptSubmit) > prompts && count(Stop) > stops`, given one added `UserPromptSubmit.1.json` and no
     `Stop`, with prompts = stops = 0. The original yields `false`. `>`→`==` and `>`→`>=` on the second operand yield
     `true`.
   - `count(SessionStart) > starts || count(UserPromptSubmit) > prompts`, given an empty captures dir, starts =
     prompts = 0. The original yields `false`. `>`→`==` and `>`→`>=` on the first operand yield `true`.
   - `clock.now() + PROBE_DEADLINE` under a fixed clock at t yields t + 120 s. `-` yields t − 120 s (pick t far enough
     from the clock's origin that `Instant - Duration` cannot overflow).
   - The Ctrl-C ending, with a fixed clock at t and two scripted pump readings `[false, true]` (the child ignores the
     first Ctrl-C and ends on the second), records two Ctrl-C sends, waits at `[t + 500 ms, t + 120 s]` and no kill.
     Deleting `!` at `:648:12` yields one send. Deleting `!` at `:651:12` yields one kill. `+`→`-` at `:648:45` or
     `:651:45` yields `t − 500 ms` or `t − 120 s` among the waits.
   - The pump wait, never finished, with the clock already past `until`, yields `false`. `→ true` yields `true`.
   - `finish`'s wait bound under a fixed clock at t is t + 120 s. `+`→`-` yields t − 120 s.
   - `SharedPty<P>` over a recording double forwards `resize(s)` (the double sees `s`), `close` (the double sees one
     close) and `child_pid` (the double's `Some(4242)` comes back). `→ Ok(())`, `→ ()`, `→ None` / `Some(0)` /
     `Some(1)` each break one of those.
5. **Today's wrappers would re-generate a survivor.** A `Run::wait_pump` kept as a one-call wrapper returning `bool`
   still yields cargo-mutants' `→ true` at its new coordinate. No unit test builds a `Run`, so it would stay missed.
   The same holds for any `+` left inside `Run::finish`. A restatement leaves no operator or bool-returning body in
   `Run`'s ending that only a `Run` reaches (testing.md 2026-09-28: the logic moves into a function that keeps its
   mutants and kills them, never out of the generated set).
6. **No host gate on the 16 sites.** `typed.rs` holds no `cfg(windows)` / `cfg!(…)` item around them. The only
   `cfg(unix)` block is test code, `:894-899`. Every site compiles and runs on Linux, so no "not measured here"
   record is expected (`grep -n 'cfg' src/cmd/verify/typed.rs`: `:689` `#[cfg(test)]` and `:894` `#[cfg(unix)]`).
7. **Capture file names** are `<Event>.<k>.json` with `k ≥ 1` (`ledger.rs:467-474`; the `captured` case at
   `typed.rs:905-913` already plants `UserPromptSubmit.2.json`). A unit case plants empty `{}` files in a tempdir. No
   child and no `claude` are needed.
8. **Clocks are per module.** `wheel.rs:806`, `gate.rs:234`, `dialog.rs:446` and `run/send/test_support.rs:24` each
   define their own `FixedClock`. The latter is `pub(super)`, unreachable from `cmd::verify`, so `typed.rs`'s tests
   define their own.

## Patterns detected
- **The decision passed in by the caller** (`crates/viola-state/src/fs.rs`, the earlier chunk's step 4): the loop moves
  verbatim into a private function that takes the decision as a closure. The old entry point becomes one call. The
  survivors' operators live in the new function and are killed there.
- **The decision as a named function** (`src/cmd/send.rs`, the earlier chunk's step 9): a branch the tests cannot
  reach becomes a pure function of its inputs, with an rstest table.
- **The captures-dir case** (`typed.rs:905-913`): a tempdir, planted capture names, `count_of` / `captured` read back.

## Conventions to follow
- **Names** `<subject>_<condition>_<expected>`; inline `mod tests`; rstest `#[case::label]` for tables
  (test-plan §4; testing-src.md pointers).
- **Time** only through an injected `Clock` or an instant passed in (testing.md §Determinism); a test wait a mutant can
  reach is bounded below the `mutants` profile's 10 s kill (`.config/nextest.toml:39-41`; testing.md 2026-09-24).
- **No new env seam, feature or dependency** (security-plan §Input Validation test-seam rows; architecture §Cross-cutting
  Patterns → Config management): the double and the clocks live in `typed.rs`'s `#[cfg(test)] mod tests`.
- **A scripted `.rs` write runs rustfmt in the same call** (the handoff's standing rules); the rustfmt PostToolUse hook
  reflows after every Edit, so a hand-applied mutant re-reads its site first (testing.md 2026-09-25 extension).

## New files to create
- none

## Files to modify
- `src/cmd/verify/typed.rs` — named framing predicates and one deadline function; `SharedPty` generic over its `Pty`; the Ctrl-C ending and the pump wait as plain functions of a clock and closures; their cases

## Sweeps
- `clock.now() + PROBE_DEADLINE` over `src/cmd/verify/typed.rs` (`grep -n 'clock.now() + PROBE_DEADLINE'`): 6 hits:
  4 `clock.now()` (`:147`, `:180`, `:269`, `:286`) and 2 `self.clock.now()` (`:651`, `:671`) · the plan decides how
  many move to the named deadline.
- `child_pid` over `crates/viola-pty/src/pump.rs` (`grep -n child_pid`): 0 hits · 0 changed (§Measured facts 2).
- `wait_pump` over `src/` (`grep -rn wait_pump src/`): 4 hits, all `typed.rs` (`:648`, `:651`, `:658` def, `:671`) · all
  changed by the restatement.
- `SharedPty` over `src/` (`grep -rn SharedPty src/`): 4 hits, all `typed.rs` · all within the restatement.

## Open questions
- none
