# Code audit stopped: resume note

Run: `.andromeda/runs/2026-10-12T06-54-29-code-audit/` · ledger mode (trend) · Epoch 4 boundary `dffad3e` · baseline `e304994`.
Stopped on the founder's word at 2026-10-12T09:07:27Z, during Phase 2 (mutation tier). Nothing was appended to
`.andromeda/code-metrics.ndjson` and nothing was committed. Phases 3 to 5 (record, diff, render) have not run.

## Done
- Phase 1 (cheap tiers) is complete. Its results are in `c-sizes.json`, `c-dup.json`, `c-cx.json`, `c-graph.json`,
  `c-dead.json`, `c-churn.json` and `c-hotspots.json`, with the control at `e304994` in `ctl/`.
- Four mutation units are finished, each complete by the tool's own markers, with `c-mutation-{unit}.json` written.
  The command form is `--copy-target=false`, at jobs 2.

| unit | tested / listed | score caught/(caught+missed) | survivors | not measured on this host | unviable | wall |
|---|---|---|---|---|---|---|
| viola-core | 54/54 | 100 | 0 | 0 | — | 28 s |
| viola-pty | 115/115 | 98.51 | 1 | 25 | — | 190 s |
| viola-agent-claude | 524/524 | 99.57 | 2 | 0 | 58 | 405 s |
| viola-state | 360/360 | 96.25 | 10 | 69 | 24 | 538 s |

## Stopped: viola (root package)
- **Progress:** stopped at **678/1299**. That count is the outcomes in the copy, excluding the baseline. The baseline was `Success`.
- **Tallies so far:** caught 595, missed 15 (not yet split by the host-excluded cover recipe), unviable 68, timeout 0.
  `end_time` is unset, so the invocation is incomplete and no `c-mutation-viola.json` was written.
- **Timing:** started 07:17:31Z, interrupted at 09:07:27Z (about 6,600 s of a 31,737 s cap). That is about 9.7 s per mutant at jobs 2.
- **The copy:** `mutants-viola-partial/mutants.out/`, copied byte for byte from `mutants-viola/mutants.out/` before the stop.
  - The first copy attempt tore against a mutant landing mid-copy. The second read identical to the live dir under `diff -rq`.
  - The `outcomes.json` sha256 is `7439ee703ae08ec18b0e658728a8e3a4d7432bec9e9cb6d3fe55c575fa411c07`.
  - After the stop, the live dir differs from the copy only in `debug.log` and the two logs of the in-flight mutants at
    `src/cmd/verify.rs:491:5`, which the interrupt cut short. Every result file (`outcomes.json`, `caught.txt`,
    `missed.txt`, `unviable.txt`, `timeout.txt`, `mutants.json`) still matches.
- **Live dir:** the tool's own `mutants-viola/` is left as it stood.
- **Torn first copy:** its removal was denied by the permission layer. It was not removed another way; it was moved
  aside to `torn-copy-viola-mutants.out/` in this run dir. It is not a resume input.

## Not started: viola-e2e
- The first invocation (`mutants-viola-e2e/`) failed its unmutated baseline. 3 `harness::supervise` tests got ENOENT
  for `target/mutants/debug/viola-fake-agent` under `--copy-target=false`. `c-mutation-viola-e2e.json` records it as
  `baseline-test-failure`.
- The operator's remedy was a re-run from a `git archive dffad3e` extract with `--copy-target=true`, through `e2e_x.py`.
  It never started: the waiter was stopped while waiting for the driver's done line. No `x-src/`, no
  `mutants-viola-e2e-x/` and no `state-viola-e2e-x.json` exist.

## Processes
Three processes were stopped at 09:07Z, and a re-read found none left:
- `e2e_x.py`, by SIGTERM;
- `mut-driver.py`, by SIGTERM, before it could write `state-viola.json` or "driver done";
- the cargo-mutants process group for viola, by SIGINT.

## The resume (UNTESTED)
None of the steps below has been run or tried on this host.
1. Re-enter `/andromeda-code-audit`. The skill keeps no state, so it resumes by artifact presence:
   - Phase 1's `c-*.json` files;
   - the four finished units' `c-mutation-{unit}.json`.
2. Re-run viola with `--iterate` from the copy. That is the recorded command form plus `--iterate`, with `--output`
   pointing at a dir holding the copy's `mutants.out`. Points to check before relying on it:
   - The tool's documentation says `--iterate` skips mutants an earlier run caught or found unviable, and re-tests
     missed and timed-out ones. That has not been read back on this host at 27.1.0.
   - A re-invocation rotates the existing `mutants.out` aside. Run it against a further copy so
     `mutants-viola-partial/` stays intact.
   - `mutsum.py` reads one `outcomes.json`, so the two invocations' outcomes need a merge before the unit is scored.
   - `mut-driver.py` refuses a unit whose output dir exists without a state file. It stops on `mutants-viola/`, so a
     resume needs a changed driver or a separate invocation.
   - Remaining work is about 621 mutants, roughly 6,000 s at jobs 2 at the rate measured here.
3. Then viola-e2e, from the extract, per `e2e_x.py`. Its guard is clear: no state or output for `-x` exists.
4. Then coverage, the Windows read-back (run 38215104120), the record, the diff and the render.
