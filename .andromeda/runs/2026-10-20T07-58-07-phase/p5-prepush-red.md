# P5 — `pre-push`'s baseline run read red on the untouched tree: the truncated `.profraw` recurred

Fired by P5 (validation check 4 (9)) as gate entry 23, `bash scripts/agent-run.sh pre-push`, through `gate.py run
--entry 23` with this run dir; HEAD `d35644a58909aec12c4898e5a066bcfca5a8ebd2`, the tree untouched (only the phase's
own chunk folder, run dir and friction lines uncommitted). `df -h /tmp` before: 32G, 18G used, 14G avail.

## The reading
- Entry line: `23 probe red · exit 0 ✗ (exit 1) · exit 1 · 87.34s · 389167 B → 23.log`; `contains '"ok":true'` ✗,
  `contains '"stage":"linux-tests"'` ✓ (the trail `gate-2026-10-20-epoch-5-cleanup.json`).
- Suite `coverage`: `Summary [  68.502s] 2974 tests run: 2974 passed (9 slow), 0 skipped` — every test passed.
- The merge: `warning: ./target/llvm-cov-target/viola-3929550-11199965415080257827_14.profraw: invalid instrumentation
  profile data (file header is corrupt)`, then `error: no profile can be merged` (twice: the lcov and the summary
  merges).
- The file: 90 112 B, mtime 2026-10-20 08:48:36.507Z; its 7 969 siblings of the same binary signature
  `11199965415080257827` are all 146 344 B (`ls -l target/llvm-cov-target/viola-*-11199965415080257827_*.profraw`,
  7 970 files). Kept outside the tree: the session scratchpad's `profraw-p5/`, sha256 `9de3dda876dfb232…`.
- The JUnit (`target/run-archive/1669/junit-coverage.xml`, read by `$TMPDIR/{scratchpad}/junit_window.py`): 32 cases
  hold the instant; the first to end is `viola::chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21` (0.644 s,
  ending 99 ms after the file's mtime) — the same kill-shaped case the switch chunk's reading named closest (ending
  36 ms after, `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/profraw-red.md`). Two readings, one
  candidate: a hypothesis still, not a reading of who killed the writer.

## Standing
- The red is the WATCH's own class («a `viola` process's truncated `.profraw` refused by `llvm-cov` at `pre-push`'s
  coverage merge, every test passing»), recurring: the WATCH's retire condition «a recurrence» is met, and the red is
  item (8)'s subject, which this chunk builds — the cause, read two-sided (plan step 10). It is no other chunk's.
- Which process wrote the file (the counter method of the switch chunk's evidence) was not read at P5: implement's
  step 10 starts from this file.
