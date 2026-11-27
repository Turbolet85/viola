# Cascade dispositions — 2026-11-27-epoch-5-cleanup (wrap 2026-11-27T21-41-23Z-wrap)

- Step 2 sweep: skipped — both landings' dry runs read `patterns: none: … changed prose alone — no file`; step 3's
  leaves read each changed row's delta (`leaf-delta-test-plan.diff`, `leaf-delta-obs-plan.diff`).
- Step 3 leaves: `cascade.py leaves` (pass, `--add` obs-summary, gotchas, testing.md): 5 groups, 9 leaves, unmapped 2
  (`host-linux.md`, `testing-src.md`). Two findings applied, read back:
  - `.claude/docs/obs-summary.md:31` · stale — «spine provisional» → the 1.0 s deadline, no longer provisional (obs-plan
    `decision/spine-hook-budget`, tx 5179).
  - `.claude/docs/gotchas.md:141` · omission — the verify class's named verify-driven tests join its kill (test-plan
    `decision/verify-window-80s-kill`, tx 5178).
  - `.claude/rules/testing.md:57` (the pre-pass sweep's curation row for `verify_window_`): its group's comparer
    found nothing stale — the line names the prefix rule for a new test, still true.
- Binds: tests↔obs — the spine deadline is stated without «provisionally» in test-plan (7 rows) and architecture
  (4 rows) by `rec.py find --regex 'spine deadline|SPINE_DEADLINE'`; nothing to fix. a11y↔log-schema: no change.
- Stale statements of the amended claims in all seven masters: none found beyond the landed rows.
