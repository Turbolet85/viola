# Report — 2026-10-13-epoch-4-survivors-ii

**Chunk:** the Epoch 4 audit's 16 typed.rs survivors in viola verify's Run B driver disposed
**Date:** 2026-10-13
**Commits:** `94f0be9 chore(2026-10-13-epoch-4-survivors-ii): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-13T17:14:05Z; `git log --format='%h %s' 64d7e5f7..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `src/cmd/verify/typed.rs` (the one source file, basis: `git diff --name-only 64d7e5f7` with `.andromeda/`,
  `viola-0.1.0/` and `.claude/` excluded; `gate.py scope`: `clean — changed 1 · listed 1`). The chunk records are
  `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors-ii/evidence/{survivors.ndjson, survivors.md, operator.md}`.
- **Symbols / APIs:** all private to `src/cmd/verify/typed.rs`. `typed.rs` still exports only `measure`, `Inputs`
  and `RunDirs` (`pub(super)`). Nothing changed outside `src/cmd/verify/`. Coordinates come from §New text, by line.
  - `fn added_turn_ended(captures: &Path, prompts: usize, stops: usize) -> bool` (293-297 @294). It is
    `framing_turns`' first wait condition, unchanged:
    `count_of(captures, UserPromptSubmit) > prompts && count_of(captures, Stop) > stops`.
  - `fn local_command_answered(captures: &Path, starts: usize, prompts: usize) -> bool` (299-304 @301). It is the
    second wait condition, unchanged:
    `count_of(captures, SessionStart) > starts || count_of(captures, UserPromptSubmit) > prompts`.
  - `fn probe_until(clock: &dyn Clock) -> Instant` (306-309 @307), which returns `clock.now() + PROBE_DEADLINE`. It
    replaces all six `clock.now() + PROBE_DEADLINE` sites: `dialog_run`, `plan_run`, `framing_turns` ×2,
    `Run::end_by_ctrl_c`'s second wait (now inside `ctrl_c_ending`) and `Run::finish`. `turn`'s
    `pasted + PROBE_DEADLINE` and `Run::settle` / `Run::box_wait`'s `from + PROBE_DEADLINE` are unchanged.
  - `struct SharedPty<P>(Arc<Mutex<P>>)`, now generic over its `Pty`. `lock` is on `impl<P> SharedPty<P>`, and the
    seven forwarders are on `impl<P: Pty> Pty for SharedPty<P>` with their bodies unchanged. `Run::spawn` builds
    `SharedPty(shared)` with `P = PortablePty`. `Run.pty` stays `Arc<Mutex<PortablePty>>`. viola-pty's `Pty` trait is
    unchanged.
  - `fn ctrl_c_ending(clock: &dyn Clock, ctrl_c: impl FnMut(), ended: impl FnMut(Instant) -> bool, kill: impl
    FnMut())` (541-556 @543). Its order is `end_by_ctrl_c`'s, verbatim: Ctrl-C, then a second Ctrl-C unless
    `ended(now + CTRL_C_AGAIN)`, then `kill` unless `ended(probe_until(clock))`.
  - `fn ended_by(clock: &dyn Clock, until: Instant, finished: impl Fn() -> bool) -> bool` (558-567 @559). This is the
    old `Run::wait_pump` loop with `finished()` in place of `self.pump.is_finished()`.
  - `Run::wait_pump` is removed. `Run::end_by_ctrl_c(self, keys)` is now `ctrl_c_ending(…)` followed by
    `self.finish()`. `Run::finish`'s wait is `ended_by(clock, probe_until(clock), || self.pump.is_finished())`. Its
    status read and `obs_event!(ProcessExit)` are unchanged. Remaining callers: `end_by_ctrl_c` is called only by
    `trusted_run`; `finish` by `end_by_kill` and `end_by_ctrl_c` (research.md §Graph impact, re-read at the diff).
  - Test module, test-only (`#[cfg(test)] mod tests`, 954-1133):
    - helpers `plant` (955-959), `FixedClock` (`impl Clock`, 1003-1007), `Recording` (`impl Pty`, 1015-1050) and
      `shared()` (1052-1054);
    - eight case functions: `added_turn_ended_needs_a_new_prompt_and_a_new_stop` (an rstest table, 5 rows),
      `local_command_answered_needs_a_new_start_or_a_new_prompt` (4 rows), `probe_until_is_the_probe_deadline_after_now`,
      `shared_pty_resize_reaches_its_pty_with_the_size`, `shared_pty_close_reaches_its_pty_once`,
      `shared_pty_child_pid_is_its_ptys`, `ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it` (4 rows) and
      `ended_by_reads_true_once_finished_and_false_once_the_deadline_passes` (4 rows). That is 21 nextest cases.
- **Crates / modules:** none added or removed. Only the root package `viola` changed, in `cmd::verify::typed`.
- **Dependencies:** none. `rstest` was already a root dev-dependency (`Cargo.toml` `[dev-dependencies]`), and no
  `mockall` was added. The preservation guard over every other file read green, as did `cargo deny check`.
- **Schema / config:** none. `schemas/diag-line.v1.json` is unchanged (the preservation guard).
- **Spec-master edits:** none this chunk. The phase made none, and implement writes none.
- **Counts / qualifiers moved:** test-plan §10 Mutation gate (`.andromeda/test-plan.md:1226`) says "the 16 in
  `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) are owed to the route entry
  "Epoch 4 survivors II"". This chunk disposed all 16, so none is owed any longer:
  - each restated into a named function in `typed.rs`;
  - each killed by a case in `typed.rs`'s `mod tests`, shown red under its mutation applied by hand at the restated
    site and green restored;
  - with no mutation run made.

  The Epoch 5 boundary audit's own run is the reader that confirms these kills, as it is for the earlier 29. Basis:
  site sweep pattern `e4s2` (`Epoch 4 survivors II`): standing 1 (`test-plan.md:1226` @c4257) and leaf 1
  (`.claude/docs/tests-summary.md:42` @c1964). Pattern `restated` (the restated symbols): standing 1
  (`test-plan.md:1226` ×2 @c4168,4187, the same clause). Patterns `owed16` (`16 typed.rs` / `typed.rs survivors`) and
  `killfilter` (the kill filter form `(^|::)tests::` / `test(=tests::`): the tool refused them, its control unfired
  over the seven masters' pre-pass text at 94f0be90. So no master states either form (`cascade.py sweep --pre-pass`,
  trail `cascade-2026-10-13-epoch-4-survivors-ii-pre-pass.json`).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. No harness script, CI step or verdict shape changed. The leaf that states the kill
  filter, `.claude/rules/verification-harness.md` `## Session Additions` 2026-09-27, gives the form
  `test(/(^|::)tests::name$/)`. That form selects no rstest table case, whose name ends `::case_<n>_<label>`. This
  chunk measured it (below). The leaf's fix is a curation extension of that entry on the operator's word
  (`inputs#I2`), not a master change.
- **Cross-project / external claims:**
  - CI run `ci#38442894573` (push) measured sha `94f0be906ceb`, the pre-CI commit: `verdict: green · checks 15/15 ·
    wall 559 s` across `test`/`lint`/`perf`/`release` on windows-2025, macos-latest and ubuntu-latest, plus `msrv`,
    `fuzz-replay` and `supply-chain` (`evidence/operator.md`). This wrap's commit adds only bookkeeping to that tree.
  - `inputs.py verify` (wrap P1):
    - `I1 · message: the operator, in the /andromeda-phase invocation, 2026-10-13T17:26Z · copy message · n/a — a
      message has no live source`. It is cited at scope.md, research.md and plan.md. Its fact: the take-up direction
      (no mutation run, witness run or workflow dispatch; a kill shown by a hand-applied mutant; no live `claude`;
      sized against one window). Its `why` names no P5 review card.
    - `I2 · message: the operator, in the /andromeda-wrap-session invocation, 2026-10-13T18:08Z · copy message` was
      snapped at this wrap and is cited here. Its fact: "Amend the leaf that holds the kill filter form so a table
      case (::case_n_label) matches, as your deviation found. The 85 % and 90 % context alarms are expected: finish
      the wrap; stop only at 95 %."
    - Summary: `inputs: 1 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a
      1 · uncited 0 · unparsed 0`, read before I2 was snapped. No DRIFT.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** the plan's kill filter (`plan.md` Implementation Steps step 4,
  `test(/(^|::)tests::<name>$/)`, copied from `.claude/rules/verification-harness.md` 2026-09-27 "write
  `test(/(^|::)tests::name$/)`"). For an rstest table function it selects no case: nextest names each row
  `cmd::verify::typed::tests::<name>::case_<n>_<label>`, so the `$` after `<name>` never matches. Evidence: the first
  green run of the new cases listed `…::added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_2_a_prompt_and_no_stop`
  and its siblings (`target/run-archive/1158`). Every hand-applied run instead used
  `test(/(^|::)tests::(<name>)(::|$)/)`, which selected the table's rows. A master states the form nowhere (pattern
  `killfilter`, control unfired), so the claim lives only in the leaf and in this chunk's plan. Route: P3 curation
  (`inputs#I2`).
- **Expected amendments (from plan):**
  - test-plan §10 Mutation gate. The 16 `typed.rs` survivors owed to "Epoch 4 survivors II" are disposed by
    restatement and cases in `typed.rs`, none left owed: the framing waits are named predicates, the deadline is one
    named function, `SharedPty` is generic over its `Pty`, and the Ctrl-C ending and the pump wait are plain
    functions. **Carried:** the Counts / qualifiers moved bullet above, and Symbols / APIs.
    - Search: site sweep `e4s2` standing 1, at `test-plan.md:1226`, the only master hit. Registries: 0 hits (the sweep
      reads `.andromeda/registries/**` with the masters).
    - Owner: test-plan.
- **Coverage of new surfaces:**
  - `added_turn_ended` / `local_command_answered` / `probe_until` (private decision functions) → validation n/a ·
    instrumentation n/a (no new log line; `obs_event!` sites unchanged) · PII n/a · tests unit · a11y n/a · tokens n/a
  - `SharedPty<P>` (private generic wrapper) → validation n/a · instrumentation n/a · PII n/a · tests unit · a11y n/a ·
    tokens n/a
  - `ctrl_c_ending` / `ended_by` (private, on the `viola verify` Run B ending path) → validation n/a ·
    instrumentation n/a (the `process-exit` line stays in `Run::finish`) · PII n/a · tests unit, plus integ
    (`cli_verify`, over the fake agent) · a11y n/a · tokens n/a

## Deviations from intent
- **The kill filter's anchor.** The plan's Implementation Steps step 4 says to run each case through
  `test(/(^|::)tests::<name>$/)`. Four of the eight new case functions are rstest tables, whose cases that `$`
  never selects. Every hand-applied run used `(::|$)` in its place, through one driver that selected each mutant's
  case or table. Justification: the plan's form would have read `nextest-exit-4` (nothing selected), not red, for
  every table, so no kill would have been shown. The plan was left as written.
- **The m03 reading's breadth.** The `probe_until` mutant (m03) ran the `ctrl_c_ending` table beside
  `probe_until_is_the_probe_deadline_after_now`, which gave 5 reds where step 2d names one case. Justification: step
  4e cites `probe_until`'s reading for `:651:45` and `:671:53` too. Running the ending's table under the same mutant
  shows the restated `+` read through `ctrl_c_ending`'s wait instants as well.
- scope record: none — `gate.py scope` clean, 0 recorded (`clean — changed 1 · listed 1 · recorded 0`).

## Decisions & corrections
- The operator's take-up direction (`inputs#I1`): no mutation run, witness run or workflow dispatch, a kill shown by
  a hand-applied mutant, and no live `claude` start. It was honoured: 14 hand applications and 0 mutation runs; no
  test or step started a PTY child or `claude` (the outcome's census).
- The operator's implement direction, given in the `/andromeda-implement` arguments this session:
  - every time written into a record comes from `date -u` in the same call;
  - pipeline-tool output goes to a file that is read whole;
  - each long gate run is journaled in the run dir before the wait;
  - the operator pass runs with the `ci.py` conclusion read.

  How it was applied: the evolve records and `survivors.md`'s stamp read the clock inside the writing process, and
  the journal holds 5 lines.
- The operator's wrap direction (`inputs#I2`): amend the leaf that holds the kill filter form so an rstest table case
  (`::case_n_label`) matches. Its home is `.claude/rules/verification-harness.md` `## Session Additions` 2026-09-27
  (a preserve-verbatim curation home, so P3 makes the change as an in-place extension). Evidence: this chunk's
  first green run named the cases `…::<name>::case_<n>_<label>`, and the `(::|$)` form selected and read red every
  table row under its mutant (`evidence/survivors.md`).
- A sweep hazard: an rstest table's case name is `<fn>::case_<n>_<label>`. A nextest name filter, or any grep of
  test names, anchored on `<fn>$` silently selects none of the table's rows. The pattern that misleads is
  `tests::<name>$`.
- A positive: a hand-applied mutant read 2 s from apply to red, because the root package rebuilt incrementally. A red
  that fast was checked against its log (m01-red), which showed the case's own assertion
  (`assertion left == right failed · left: true right: false`), not a compile error. Every one of the 14 was read the
  same way (`extract.py` over each red log).
- The `Instant - Duration` panic that testing.md 2026-10-13 records for a `-` mutant did not occur. m03 and m12 read
  red on their assertions because the host's monotonic clock was past 120 s.
- Guard or hook refusals: none this chunk. Recurrence of a standing learning: none.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - (tests) 16 survivors disposed by restatement plus named case, each red under its hand-applied mutation and green
    restored, both readings in `evidence/survivors.md`, none called equivalent, none owed again. **Met**: 16 ledger
    rows, `form: restatement`, `by` naming the case; 14 red/green pairs.
  - (tests) The ledger entry reads `last line 1`. **Met**: gate entry
    `python3 -c 'import json; old=…'` green, last line 1.
  - (tests) No mutation run, witness run or workflow dispatch, and no live `claude` or other PTY child. **Met**: none
    was fired, and no case builds a `Run`. The double is `Recording`, and the clocks are `FixedClock`.
  - (arch / security / obs / layouts) Run B unchanged. **Met**: the removed-lines entry reads `last line 0`, and
    `run --integration --filter 'binary(cli_verify)'` reads `"ok":true`.
  - (arch / security) Every new seam is test-only. **Met**: `Recording` and `FixedClock` are inside
    `#[cfg(test)] mod tests`; no manifest, lockfile, feature or env var changed (preservation guard and
    `cargo deny check` green); the `Pty` trait is unchanged.
  - (obs) G1, the lint probes, G2 and G4 are green, and `schemas/diag-line.v1.json` is unchanged. **Met.**
  - (tests) `run --unit` reads `"ok":true`; `pre-push` reads `"ok":true` at stage `linux-tests`; the CI read is green
    across the three OSes with the run named. **Met**: `ci#38442894573`.
- Gates (implement P2 firing 1, then the operator pass), in block order:
  - `cargo fmt --all --check`: green
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green
  - `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings`: green
  - `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .`: green (exit 1, no output)
  - `bash scripts/lint-probes.sh`: green
  - `bash scripts/agent-run.sh run --unit`: green (`"ok":true`)
  - `bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify)'`: green (`"ok":true`, 35.27 s)
  - the survivor-ledger probe: green (last line 1)
  - the removed-lines guard on `typed.rs`: green (last line 0)
  - the preservation guard: green (exit 0)
  - `cargo deny check`: green
  - the smoke `cleanup` / `boot` / `status`, `g2-zero-panics.sh` and `schema-check` (G4), then `cleanup`: green
    (`processes_gone` and `endpoint_gone` true)
  - `bash scripts/agent-run.sh pre-push`: green in firing 1 (67.61 s) and again in firing 2 at the operator pass
    (66.61 s): coverage 2452 passed / 0 failed, playwright 1 passed, gate breaches `[]`
  - `gate.py hygiene` (`leg = 'operator'`): `hygiene: clean`, read in `evidence/operator.md`
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`): green,
    `history moved: … 64d7e5f7→94f0be90`, read in `evidence/operator.md`
  - `ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`): `verdict: green · checks 15/15`,
    `ci#38442894573` on `94f0be906ceb`, read in `evidence/operator.md`
  - No `defer` and no deferral.
  - Smoke: recorded from P2 (the plan's `role = 'smoke'` entries).
- Watches: none.
- Outcome basis:
  - this window's implement run (`.andromeda/runs/2026-10-13T17-47-22-implement/`, whose
    `outcome-2026-10-13-epoch-4-survivors-ii.md` is this window's own P4 report, the outcome word green + smoke);
  - the operator pass: the commit list `94f0be9`, and the final HEAD's CI run recorded in `evidence/operator.md`.

  The gate verdicts above rest on the pass's final state: pre-push firing 2 on the final tree, then the CI on
  `94f0be906ceb`. No operator directive came between implement and this report other than `inputs#I2`, which is
  about curation.
- Process hygiene: implement P4's census, measured against the host process list at 2026-10-13T17:55:13Z. The smoke
  wrapper `p-e4s2-smoke` and every test binary of the gate runs and the 28 hand-mutant runs read terminated: no
  process whose exe is under this repository. The `viola` processes listed are the overseer bridge's
  (`…/viola-lab/prototype/target/debug/viola`), left running and not this chunk's.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 64d7e5f7 (the parent of the oldest pre-CI commit 94f0be90) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### src/cmd/verify/typed.rs — added 251 line(s) in 13 range(s)
added: 147 · 180 · 269-270 · 285-286 · 293-310 · 401 · 403-404 · 409 · 541-568 · 691-699 · 706-707 · 727-730 · 954-1133
- 293-297 @294 «fn added_turn_ended(captures: &Path, prompts: usize, stops: usize) -> bool {»
  - 295-296 «count_of(captures, HookEvent::UserPromptSubmit) > prompts»
- 299-304 @301 «fn local_command_answered(captures: &Path, starts: usize, prompts: usize) -> bool {»
  - 302-303 «count_of(captures, HookEvent::SessionStart) > starts»
- 306-309 @307 «fn probe_until(clock: &dyn Clock) -> Instant {»
- 541-556 @543 «fn ctrl_c_ending(»
  - 550-552 «if !ended(clock.now() + CTRL_C_AGAIN) {»
  - 553-555 «if !ended(probe_until(clock)) {»
- 558-567 @559 «fn ended_by(clock: &dyn Clock, until: Instant, finished: impl Fn() -> bool) -> bool {»
  - 560-565 «while !finished() {»
  - 955-959 «fn plant(dir: &Path, event: &str, n: usize) {»
  - 961-978 @967 «fn added_turn_ended_needs_a_new_prompt_and_a_new_stop(»
  - 980-999 @985 «fn local_command_answered_needs_a_new_start_or_a_new_prompt(»
  - 1003-1007 «impl Clock for FixedClock {»
  - 1009-1013 @1010 «fn probe_until_is_the_probe_deadline_after_now() {»
  - 1015-1019 @1016 «struct Recording {»
  - 1021-1050 «impl Pty for Recording {»
  - 1052-1054 «fn shared() -> SharedPty<Recording> {»
  - 1056-1071 @1057 «fn shared_pty_resize_reaches_its_pty_with_the_size() {»
  - 1073-1078 @1074 «fn shared_pty_close_reaches_its_pty_once() {»
  - 1080-1083 @1081 «fn shared_pty_child_pid_is_its_ptys() {»
  - 1085-1113 @1090 «fn ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it(»
  - 1115-1133 @1120 «fn ended_by_reads_true_once_finished_and_false_once_the_deadline_passes(»
