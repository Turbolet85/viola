@row_set n=1 row=decision/wrapper-drop-guard cell=rule seen=1 fact=F1 detector=none
value: Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper or fake agent; a Drop guard that kills, then waits, still covers the normal end.
basis: fct-trse7ov6, fct-y7jldnao, fct-hkrr5z5i
because: The chunk ties every long-lived viola child of a root test to its test thread, so the Drop guard is no longer the whole lifecycle of a test wrapper and the row must say the tie.
@row_set n=2 row=decision/one-verify-stamp-per-build cell=rule seen=1 fact=F4 detector=none
value: Stamped test homes copy one viola verify stamp per pair of built binaries, kept under target/e2e-home keyed by their hashes and taken under an exclusive lock; a stamp counts only with its completion and stamps files, and a hollow one is neither trusted nor removed: the next generation dir is stamped
basis: fct-trse7ov6, fct-y7jldnao, fct-hm4n5rph, fct-uoztxj4k
because: A stamp now counts only by its completion file and stamps file and a hollow one gets a next-generation dir with nothing removed, which the row's rule omits.
@row_set n=3 row=decision/supervisor-ordinary-child cell=rule seen=1 fact=F5 detector=D-tests-obs-harness
value: The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session ties it and its stamping viola verify to the test thread by a parent-death kill.
basis: fct-uvauzl42, fct-y7jldnao
because: An in-test harness session's supervisor now dies with its test thread, so the row's claim that the supervisor stops only through its stop request holds for the CLI boot alone.
@row_add n=1 row=decision/no-untied-viola-spawn area=integration-test-strategy fact=F2 detector=none
name: No root test spawns viola untied
rule: No root test spawns viola untied: a census over every test source, recursively, counts any spawn whose path equals or ends in the viola binary constant outside the tied builder, and is proven on a planted positive.
why: A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant.
guard: cross_killed_test_census_finds_no_untied_viola_spawn
basis: fct-haww247t, fct-surved5a, fct-jllwual7
because: The chunk adds a census that holds every root viola spawn tied, a convention future test files must respect and no row states.
@row_add n=2 row=decision/tied-child-spawn-thread area=integration-test-strategy fact=F3 detector=none
name: Tied children spawned on a thread that outlives them
rule: A tied viola child is spawned only on a thread that outlives it.
why: The parent-death signal follows the spawning thread.
basis: fct-jfgqbjlt
because: The parent-death tie binds where a tied child may be spawned, a constraint every future tied spawn must respect and no row states.
@row_none n=1 fact=F6 type=4
relied: the killed-test witness kills a survivor only when its pid and start time still match, as the standing pid-and-start-time kill decision already requires.
