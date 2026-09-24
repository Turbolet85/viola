# CI windows mutation leg stall — run 36046091888 (sha bf87d71)

## What was measured

- **Windows leg** (job `mutants (windows-2025)`, job log fetched with `gh api repos/Turbolet85/viola/actions/jobs/107789894889/logs`,
  309 lines).
  - The step `bash scripts/agent-run.sh run --mutants --leg "$LEG"` starts at 19:09:10Z with `AGENT_RUN_CHUNK_BASE: 9df9e45…`.
  - `Compiling viola` finishes at 19:09:31Z. Its last line is ` INFO Auto-set test timeout to 210s` at 19:11:51Z, which means a baseline
    test phase of about 42 s.
  - Next comes `##[error]The operation was canceled.` at 21:56:42Z, after 2 h 45 m of silence.
  - The verdict upload then warns `No files were found … mutants-verdict-windows-2025.json`.
- **Ubuntu leg** (same run): ` INFO Auto-set test timeout to 140s` at 19:10:26Z.
  - Then every cargo-mutants line lands at once at 19:25:37Z, when the process exits: `Found 143 mutants to test` ·
    `143 mutants tested in 17m: 139 caught, 4 unviable`.
  - 0 missed, 0 timeout.
- **Local windows host** (this session, same tree, base 9df9e45): `143 mutants tested in 12m: 138 caught, 5 unviable`, 0 timeout.
  - The slowest mutant's test phase is 10.5 s (`src/bin/viola-fake-agent.rs:482:8: delete ! in main`), measured from `outcomes.json`
    `phase_results` durations.
  - No mutant came near its 120 s timeout.

## Which mutant stalled: NOT MEASURABLE from run 36046091888

The run recorded no per-mutant datum. Two reasons:

1. **The harness swallowed the progress.** `run_forwarding` runs every tool with `Command::output()`, which captures stdout and echoes
   it to stderr only after the process exits. That is visible on the ubuntu leg: every line has the exit timestamp. A leg that never
   exits therefore prints nothing past cargo-mutants' own stderr INFO line.
2. **No artifact survived.** The leg verdict is written only after cargo-mutants finishes. `mutants.out/` is deliberately not uploaded
   (security-plan §Bootstrap `secret-scanning-ci-gate`, 2026-09-24-quality-gates decision: its `outcomes.json` / logs carry absolute
   argv paths and test output).

So the stalled mutant's identity cannot be recovered from this run. The honest answer is "unobtainable from the recorded run". Any named
candidate would be inference.

## What cannot be the explanation (measured)

- Not slowness alone. Ubuntu took 17 min, and chunk 7 (2026-09-24-quality-gates, run 36019646063) took 41 min on windows against 16 on
  ubuntu, a ratio of about 2.6. The same ratio predicts roughly 45 min here, not more than 165.
- Not a mutant that times out everywhere. The same 143 mutants graded 0 timeout on ubuntu CI and on the local windows host.

## Answer to "should the leg stream progress and upload mutants.out with if: always()?"

- **Stream progress: yes, done in this chunk** (`crates/viola-e2e/src/harness/run/mutants.rs`, in scope):
  - cargo-mutants' stdout goes live to the harness's stderr (`.stdout(Stdio::from(std::io::stderr()))`), so the CI step log shows each
    line as it happens;
  - `--caught --unviable` prints EVERY outcome line (name + build/test seconds), not only missed/timeout. A stall is therefore localized
    to the mutant after the last printed line, in `cargo mutants --list` order;
  - `--build-timeout-multiplier=5` bounds each mutant's build (cargo-mutants sets no build timeout by default), so a build-phase hang
    ends as a named Timeout instead of an unbounded wait. The test phase was already bounded by the auto-set timeout.
  - The streamed lines carry repo-relative mutant names and durations only, never test output (no `--all-logs`).
- **Upload `mutants.out` with `if: always()`: no.** That would reverse the ratified security decision above (raw argv paths, test output
  in an artifact).
  - The admissible form would be a reduced partial verdict (names + outcomes from the partial `outcomes.json`) in an `if: always()` step.
  - That needs a `ci.yml` step and a new verdict value or artifact name in test-plan §3's closed enums. Both are outside this chunk's
    modify-set and are spec changes. It is recorded as an operator decision for the wrap (proposal), not built here.

## Local re-run with the fix (gate block run 3, windows host, base 9df9e45)

- `entries 18 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 3` (the 3 are the operator legs).
- Gate 15: `Found 143 mutants to test` · `143 mutants tested in 13m: 137 caught, 6 unviable`, 0 missed, 0 timeout.
- **Streaming witnessed live:** a poll of the gate-15 log DURING the run read 4 outcome lines already present, before cargo-mutants
  exited. The finished log holds 143 `caught`/`unviable` lines with timings, e.g.
  `caught   src/bin/viola-fake-agent.rs:421:5: replace script_steps -> Option<Vec<Step>> with None in 14s build + 1s test`.

## Tests for the change

- `harness::run::mutants::tests::run_mutants_prints_every_outcome_and_bounds_each_build` asserts the three flags on the `cargo mutants`
  invocation, with literal oracles.
- Remove-the-guard run: the flags line removed → the test FAILs at `mutants.rs:583`; restored → passes (1 passed). Scratch logs
  `prog-red.log` / `prog-restored.log`.
