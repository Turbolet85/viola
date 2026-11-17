@row_set n=1 row=decision/no-advisory-ignored cell=except seen=2 detector=none
value: Ignored with a reason: an advisory only in an rmcp feature banned or off in the release graph, until rmcp leaves <3.5; an unmaintained-upstream advisory with no safe upgrade, re-read each epoch.
basis: fct-s6xve3jr
because: The ignore list gains RUSTSEC-2026-0543 on the unmaintained-upstream ground, so the exception now holds two grounds, not only the rmcp one.
@row_set n=2 row=decision/no-ignored-advisories cell=except seen=2 detector=none
value: Ignored with a reason: an advisory only in an rmcp feature banned or off in the release graph, until rmcp leaves <3.5; an unmaintained-upstream advisory with no safe upgrade, re-read each epoch.
basis: fct-s6xve3jr
because: The ignore list gains RUSTSEC-2026-0543 on the unmaintained-upstream ground, so the exception now holds two grounds, not only the rmcp one.
@row_set n=3 row=decision/supervisor-ordinary-child cell=rule seen=2 detector=none
value: The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session ties it to the test thread by parent-death SIGKILL and its stamping viola verify by SIGTERM.
basis: fct-pr4bii4w
because: The in-test boot's tied stamping verify now takes SIGTERM at its test thread's end while the supervisor keeps SIGKILL, so one parent-death kill for both is no longer true.
@row_set n=4 row=decision/rstest-case-tables cell=except seen=1 fact=F3 detector=none
value: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases, and the own cases of a shared helper taken by path, which run in every taker, are labelled tables in one test.
basis: fct-fnnj7js2
because: A shared helper taken by path carries its own cases into every taker, viola-e2e among them, so its cases join the exception to rstest case tables.
@row_add n=1 row=decision/no-untied-harness-boot area=integration-test-strategy fact=F1 detector=none
name: No viola-e2e test boots a harness session untied
rule: No viola-e2e test boots a harness session untied: a census over viola-e2e's test sources finds every in-test boot tied, and is read red on a planted untied boot first.
guard: viola_e2e_census_finds_no_untied_boot
basis: fct-iefl2sdr
because: The chunk ties every in-test boot and adds a census proven on a planted positive, a rule the root-only untied-spawn row does not cover.
@row_add n=2 row=decision/tied-verify-kill-witness area=test-strategy fact=F2 detector=none
name: Kill witness for a tied stamping verify
rule: A kill witness for a tied stamping verify waits until verify holds a child inside a viola-verify run dir, asserts an owned dir stood at the kill, and reads the stamp's absence besides processes and dirs.
why: An untied verify orphaned by the kill finishes inside the wait bound and removes its own dirs, so processes and dirs alone cannot tell it from a tied one.
guard: killed_test_leaves_no_stamping_verify
basis: fct-d3od7oe6
because: Measurement showed a process-and-dir witness cannot separate a tied verify from an untied one, so the witness design is a decision future kill witnesses must keep.
