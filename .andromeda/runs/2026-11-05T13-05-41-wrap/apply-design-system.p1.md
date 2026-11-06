@apply_row n=1 ref_file=fanout-design-system--cascade.md ref_n=3 v=applied edit=E1
@apply_row n=2 ref_file=fanout-design-system.md ref_n=7 v=folded_into to_file=fanout-design-system--cascade.md to_n=3
@apply_row n=3 ref_file=fanout-design-system--cascade.md ref_n=4 v=applied edit=E2
@apply_row n=4 ref_file=fanout-design-system.md ref_n=8 v=folded_into to_file=fanout-design-system--cascade.md to_n=4
@apply_row n=5 ref_file=fanout-design-system.md ref_n=1 v=applied edit=E3
@apply_row n=6 ref_file=fanout-design-system.md ref_n=2 v=refused
why: a hand removal · fct-2wyyc34q
@apply_row n=7 ref_file=fanout-design-system.md ref_n=3 v=refused
why: a hand removal · fct-jijjnoyy
@apply_row n=8 ref_file=fanout-design-system.md ref_n=4 v=applied edit=E4
@apply_row n=9 ref_file=fanout-design-system.md ref_n=5 v=refused
why: a hand removal · fct-jphpatxv
@apply_row n=10 ref_file=fanout-design-system.md ref_n=6 v=applied edit=E5
@apply_row n=11 ref_file=fanout-design-system--cascade.md ref_n=16 v=folded_into to_file=fanout-design-system.md to_n=6
@apply_row n=12 ref_file=fanout-design-system.md ref_n=9 v=applied edit=E6
@apply_row n=13 ref_file=fanout-design-system--cascade.md ref_n=1 v=applied edit=E7
@fact id=fct-prvstcem expect_rev=1
old: It ships with Windows 10 1709+, and Windows is the first target.
new: It ships with Windows 10 1709+.

@fact id=fct-ithdbbjj expect_rev=1
old: **Both faces are Windows-only.** D3 makes macOS and Linux supported, so
new: **Both faces are Windows-only** and viola targets Linux only, so
old: the Linux fallback is a first-class rendering, not
new: the Linux fallback is the target's own rendering, not

@fact id=fct-7z5o76y5 expect_rev=1
old: - Windows 10/11 (Windows Terminal, conhost), macOS and Linux terminals.
new: - Linux terminals (the core on any current distribution, CI on `ubuntu-latest`).

@fact id=fct-vzhczpnh expect_rev=1
old: `256color`, or on Windows once VT enabling has succeeded;
new: `256color`;

@fact id=fct-2t54hjhs expect_rev=1
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`

@fact id=fct-nqpspgzt expect_rev=1
old: Each role token's stack names a face per OS (Windows, macOS, Linux) and ends at
new: Each role token's stack ends at

@fact id=fct-64jvg77y expect_rev=1
old: | Role token | Windows (live target) | macOS (CI-tested) | Linux (CI-tested) | Last resort |
new: | Role token | Windows | macOS | Linux (the target) | Last resort |
@pattern id=ds1-01 ignore_case=true
fixed: Windows is the first target
@pattern id=ds1-02 ignore_case=true
fixed: makes macOS and Linux supported
@pattern id=ds1-03
fixed: (Windows Terminal, conhost), macOS and Linux terminals
@pattern id=ds1-04 ignore_case=true
fixed: on Windows once VT enabling has succeeded
@pattern id=ds1-05
fixed: `stale-heartbeat` · `batch-script-child`
@pattern id=ds1-06
fixed: names a face per OS (Windows, macOS, Linux)
@pattern id=ds1-07
fixed: Windows (live target)
