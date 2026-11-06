@apply_row n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=10 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g1.md ref_n=14 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g1.md ref_n=17 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture.md ref_n=1 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-architecture.md to_n=1
@apply_row n=7 ref_file=fanout-architecture.md ref_n=2 v=applied edit=E6
@apply_row n=8 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=folded_into to_file=fanout-architecture.md to_n=2
@apply_row n=9 ref_file=fanout-architecture.md ref_n=3 v=refused
why: a key file outside this writer's targets, main's edit
@apply_row n=10 ref_file=fanout-architecture.md ref_n=5 v=refused
why: a key file outside this writer's targets, main's edit
@apply_row n=11 ref_file=fanout-architecture.md ref_n=6 v=applied edit=E7
@apply_row n=12 ref_file=fanout-architecture--cascade-g1.md ref_n=19 v=folded_into to_file=fanout-architecture.md to_n=6
@apply_row n=13 ref_file=fanout-architecture.md ref_n=7 v=refused
why: a key file outside this writer's targets, main's edit
@apply_row n=14 ref_file=fanout-architecture.md ref_n=4 v=folded_into to_file=fanout-architecture.md to_n=7
@apply_row n=15 ref_file=fanout-architecture.md ref_n=8 v=applied edit=E8,E9
@apply_row n=16 ref_file=fanout-architecture.md ref_n=9 v=refused
why: neither fct-i4wacyrp nor fct-6e4hf2ai names the ceiling's owner (each states only `dialog_hold_s` 60-3600), so each is left as it reads, as the ruling directs
@apply_row n=17 ref_file=fanout-architecture--cascade-g1.md ref_n=1 v=applied edit=E10

@fact id=fct-zhr3wx2i expect_rev=1
old: Rationale: exec form skips Git Bash path rewriting and shell startup.
new: Rationale: exec form skips shell startup.

@fact id=fct-4nt6qtyp expect_rev=2
old: ceiling PROVISIONAL, owner: "Version done-check")
new: ceiling PROVISIONAL, owner: 0.2.0's first entry)

@fact id=fct-sha7o35z expect_rev=1
old: One ratified exception, `viola-pty`'s hand-written `PtyError`, and two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds, the rule standing;
new: Ratified exceptions: `viola-pty`'s hand-written `PtyError`, and `viola-mcp`'s `ServeError`, `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`;

@fact id=fct-c3wssf3l expect_rev=1
old: its cause not explained and acting on it owed to the working-route entry "Version done-check".
new: its cause not explained, known and left without action in 0.1.0, the re-hash the guard.

@fact id=fct-omwrdnlc expect_rev=2
old: **[PTY] portable-pty `=0.8.1` pinned
new: **[PTY] portable-pty `=0.9.0` pinned
old: Rationale: 0.8.1 is spike-proven; moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check".
new: Rationale: exact, as the workspace pins every third-party version; 0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family, and `viola-pty` builds and passes its tests on it unchanged.

@fact id=fct-zegjs5rw expect_rev=2
old: portable-pty `=0.8.1` behind
new: portable-pty `=0.9.0` behind

@fact id=fct-nvft5tqc expect_rev=1
old: leading-slash argument.** viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths, and agents need machine-parseable refusals.
new: leading-slash argument.** Rationale: agents need machine-parseable refusals.

@fact id=fct-nhsg4k7g expect_rev=1
old: `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the two open divergences named there, with `PinError`, `StampsReadError` and `Refused` the three the census adds, nine `thiserror` derives over the product sources in all, by
new: `viola-mcp`'s `ServeError` beside `McpError`, `viola-state`'s `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused` are the ratified exceptions named there, nine `thiserror` derives over the product sources in all, `crates/viola-e2e` outside the count, by

@fact id=fct-vf75c7sp expect_rev=1
old: (except `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are open divergences, with `PinError`, `StampsReadError` and `Refused` the three the census adds, [Error Handling])
new: (ratified exceptions: `viola-pty`'s hand-written `PtyError`, and `viola-mcp`'s `ServeError` beside `McpError`, `viola-state`'s `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`, [Error Handling])

@fact id=fct-kxiazeyf expect_rev=2
old: portable-pty `=0.8.1` (pinned)
new: portable-pty `=0.9.0` (pinned)

@pattern id=arch1-01 ignore_case=true
fixed: skips Git Bash path rewriting
@pattern id=arch1-02
fixed: owner: "Version done-check"
@pattern id=arch1-03 ignore_case=true
fixed: open divergences
@pattern id=arch1-04
fixed: owed to the working-route entry
@pattern id=arch1-05
fixed: 0.8.1 is spike-proven
@pattern id=arch1-06
fixed: portable-pty `=0.8.1`
@pattern id=arch1-07 ignore_case=true
fixed: Git Bash rewritten-path prefix
@pattern id=arch1-08
fixed: the three the census adds
