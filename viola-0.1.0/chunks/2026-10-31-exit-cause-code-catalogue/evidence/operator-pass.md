# Operator pass — 2026-10-31-exit-cause-code-catalogue

Run by the session on the operator's word (`inputs#I4`, the /andromeda-implement invocation of 2026-10-31: "Run the operator pass with the ci.py conclusion read (leg=operator) as usual; its first push carries the two spine-door commits."). Base: HEAD `404f7496dbd7` before the pre-CI commit. `origin/build/viola-0.1.0` read `16ef6b4813e7`, so the first push also carries `df4ca577` and `404f7496`.

## Entry 20 re-run — `bash scripts/agent-run.sh pre-push` (the final uncommitted tree)
Fired through `gate.py run --entry 20`, started 2026-10-31T17:37:32Z:
```
 20 probe       green · exit 0 · 95.36s · 445566 B → 20.3.log · bash scripts/agent-run.sh pre-push
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
Its log: `Summary [  72.546s] 3403 tests run: 3403 passed (5 slow), 0 skipped`, and `{"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests","linux":{"run":{"ok":true,"suites":[{"suite":"coverage","passed":3403,"failed":0,"skipped":0,"failures":[]},{"suite":"doctest","passed":0,"failed":0,"skipped":0,"failures":[]}]},"gate":{"ok":true,"breaches":[]}}}`.

Host record: `QUIET for viola - 0 s stalled on IO`.

WATCH, `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, in this firing: PASS `4.491 s` (`case_1_sigint`), PASS `4.484 s` (`case_2_sigterm`).

## Entry 21 — fired as written, read 2026-10-31T17:39:40Z
The first firing read `hygiene: refused 2 files — P1 2`: the phase run's `p5-controls.md:42` and `p5-resume.md:22`, each the temp path of the `target/e2e-home` link. Each was rewritten to its `$TMPDIR/…` spelling, nothing else changed, and the verb re-fired.

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`
```
gate v1.23 · 2601bda6
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 658 (runs 650 · evidence 2 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
This record is written after that read, as the plan template says.
