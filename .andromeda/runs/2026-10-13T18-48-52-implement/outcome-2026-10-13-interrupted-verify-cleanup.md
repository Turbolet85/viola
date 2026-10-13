# Implement outcome — 2026-10-13-interrupted-verify-cleanup

**Outcome: surfaced.** All non-leg entries but one are green. Entry 9, the preservation guard, reads red on one file,
`.github/workflows/windows-mutants.yml`. The repository's contract test requires that file's matrix line for the new
Windows-gated `src/cmd/verify/stop.rs`, and the operator chose to add it (below). Smoke ✓, recorded from P2.

The operator pass ran on the operator's word and is closed (`evidence/operator-pass.md`):
- `pre-push` green on the final tree;
- hygiene clean;
- pre-CI commit `c22161e791ff`, pushed through entry 19;
- the CI read: **`verdict: green · checks 15/15 · attempt 2 · ci#38453057445`**.

Attempt 1 read red. `test (macos-latest)` was never acquired by a hosted runner (0 steps). The operator's word allowed a
re-run of that one job, and macOS then ran the stop table green.

Next → `/andromeda-wrap-session`, which resolves entry 9.

## Gates (final whole-block run, `gate-2026-10-13-interrupted-verify-cleanup.json`, run 4)
- 1–8, 10–17 green:
  - fmt, clippy, the Windows-target clippy, G1, lint probes;
  - unit (`run --unit`, `"ok":true`);
  - integration (`cli_verify` + `cli_version_gate` + `contract_diag_schema`, 72 passed, the two stop cases among
    them);
  - the literal probe (last line `0`) and `cargo deny check`;
  - the smoke (cleanup → boot → status, G2, G4, cleanup with `processes_gone` and `endpoint_gone` true);
  - `pre-push` (`"ok":true`, stage `linux-tests`).
- 9 red, `exit 0 ✗ (exit 1)`: surfaced, see below.
- 18–20 not run: `leg operator`.
- No deferral; every changed-surface gate ran. No `watch:` subject.

## Files changed
- New: `src/cmd/verify/stop.rs`.
- Modified (research's list):
  - `src/cmd/verify.rs`
  - `src/cmd/verify/typed.rs`
  - `src/run/version_gate.rs`
  - `schemas/diag-line.v1.json`
  - `tests/cli_verify.rs`
  - `tests/support/verify.rs`
- Out of list, recorded in `scope-record.md`: `.github/workflows/windows-mutants.yml` (companion, the operator's
  word).
- `gate.py scope`: `clean — changed 8 · listed 7 · recorded 1`.

## Deviations from the plan, with justifications
1. **Where the pair is printed.** Step 5 says `measure` prints it. `measure` instead returns `Ended::Interrupted`, and
   `verify` prints the pair through `human::refuse`, then logs `process-exit{exit_code:1, detail:"interrupted"}`. The
   lines, their order before the self-exit line and the exit are the same; the `return Ok(1)` sites stay single
   values.
2. **`run_bounded`'s stop.** It takes the stop as a `fn() -> bool` and reads it before the child's exit on every
   poll. `Bounded` gains `stopped: bool`, which is how the "stopped" outcome is told apart from the deadline's kill.
   `version_gate` passes `never_stopped`.
3. **One stop read covers the interactive runs.** Run A is now `untrusted_run`, and the four runs run in one loop
   with one stop read before each. So no run starts after the stop, and the stop table's log-shape assertion kills
   that read. After Run D, `verify`'s post-drop read is the one that applies.
4. **No paste after the stop.** `Run::paste_text` refuses a paste once the stop is set. This covers a stop that lands
   between a wait's success and the next paste: no byte reaches a PTY child after the stop (the security criterion).
5. **`ctrl_c_ending` reads the stop before its first Ctrl-C too.** A stop before Run B's ending sends no Ctrl-C and
   goes straight to the kill. A stop during the ending sends no second Ctrl-C and kills. The existing cases are
   unchanged with the stop unset.
6. **The helper returns the pid.** `tests/support/verify.rs` `verify_started` returns
   `StartedVerify { running, pid, receipt }`, because the case's `libc::kill` needs the pid of the `viola` it started.
   `tests/support/cli.rs` is untouched.
7. **Unit cases beyond step 7's list.** All are in-intent, each pins a stop path:
   - `run_bounded_with_the_stop_set_reads_stopped` (every OS), beside the Unix `/bin/sleep 30` case;
   - `never_stopped_is_never_set`;
   - `poll_until_ends_at_a_stop_set_while_it_waits`;
   - `wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition`;
   - `ctrl_c_ending_at_the_stop_sends_no_more_and_kills`.
8. **The `WITHIN` count note.** The receipt watch goes through the existing `fake::wait_for` helper, so no new
   `Instant::now() + WITHIN` site was written. Measured by its rule, a fixed-string grep of `Instant::now() + WITHIN`
   under `tests/`: 28 sites in 20 files, 0 added against `d9f01b1b8642`.
9. **Remove-the-guard controls** (testing.md 2026-09-25), in `evidence/guards.md`. Three guards were each
   neutralised, and the stop table read red on both cases with its own panic line; each file was restored, its
   sha256 read back equal:
   - `stop::install`: exit `None`;
   - the per-run stop read: Runs C and D's extra spawn pairs;
   - `poll_until`'s stop read: `viola never exited`.

## Surfaced (author nothing; the wrap resolves)
**The plan's preservation guard is jointly unsatisfiable with the repository's contract test.**
- **The guard.** Entry 9's pathspec holds every file outside the chunk's list unchanged, workflows included (its
  note: "no other … workflow").
- **The contract.** `tests/contract_windows_mutation_scope.rs`, run by entry 17's `pre-push`, requires every product
  file with a `cfg(windows)` gate to be listed in `.github/workflows/windows-mutants.yml`. `stop.rs` carries the
  Windows console routine the plan's step 1 places there, so the first `pre-push` read red: `unlisted:
  src/cmd/verify/stop.rs`.
- **The operator's word (2026-10-13, given here):** "Add the workflow line — one matrix line, nothing dispatched;
  surface entry 9 as a plan defect at the wrap". The line added: `package: viola`, `label: viola-cmd-verify-stop`,
  `files: src/cmd/verify/stop.rs`. No workflow was dispatched.
- **Entry 9 read two-sided** on the final tree, both by hand outside the tool:
  - the guard's command as written: exit 1;
  - the same command with only `:!.github/workflows/windows-mutants.yml` added: exit 0.

  So the acceptance criterion it serves (`Cargo.toml`, `Cargo.lock` and the fake agent unchanged; no dependency,
  feature, seam or variable) holds.
- **For the wrap:** the plan's guard pathspec and research's Files to modify both missed the companion. Likely
  amendment targets: test-plan §10 or the plan template's guard form for Windows-gated new files. The
  architecture's crate-direction text is already expected.

## Process hygiene
Measured with `ps -eo pid,ppid,etimes,comm,args` and `/proc/<pid>/exe` against the repository root after the last
gate run.

| process | started by | final state |
|---|---|---|
| gate entries' cargo / nextest / test binaries / `viola` and fake-agent children (4 block runs, 1 targeted run) | this run | terminated — no process whose exe lies under this repository remains |
| harness session `p-ivc-smoke` (supervisor, wrapper, fake agent) | entry 12 (boot), twice | terminated — entry 16 cleanup `processes_gone:true`, `endpoint_gone:true` both runs |
| remove-the-guard controls' `agent-run.sh run --integration` (3) | this run | terminated — exited; files restored |
| `/bin/sleep 30` of `run_bounded_with_the_stop_set_kills_and_reaps_a_running_child` | this run (unit case) | terminated — killed and reaped by `run_bounded`; no `sleep 30` in the process list |
| `viola` (viola-lab prototype) and `claude` processes | the operator / other sessions | left running: not this run's (another tree's exe) — the operator's |

## Decisions & corrections
- The operator's ruling on the plan defect: the companion `windows-mutants.yml` matrix line, nothing dispatched,
  entry 9 surfaced at the wrap (2026-10-13, given here; `scope-record.md`).
- The operator's word on the CI re-run (2026-10-13, given here): "the founder's no-dispatch direction covers the
  mutation workflow, not a re-run of the ordinary CI job a runner never picked up". `gh run rerun 38453057445
  --failed` re-ran the one cancelled macOS job.
- A sweep hazard, measured. A `journal.md` line appended into the committed run dir after the pre-CI commit dirtied
  the tree, and entry 19's guard refused the push (`git diff --quiet` exit 1, history unmoved). The line was set aside,
  the file restored to HEAD, the push re-fired green, and the line put back. Journal an operator-pass step only after
  the push, or outside the tree.
- A guard refusal: the Bash guard's `cd` arm refused a `cd` into the session scratchpad once. It was re-issued with
  absolute paths.
- The operator's invocation directions for this run: no live `claude` start, no mutation, witness run or workflow
  dispatch; signals only at processes the test started; record times from `date -u` in the same call;
  pipeline-tool output to a file, read whole; each long gate run journalled before the wait (`journal.md`). All
  were held. Every evolve record's `ts` came from `date -u` inside its appending call.
