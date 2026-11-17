@row_add n=1 row=decision/test-signals-only-recorded-processes area=security-anti-patterns-never-do-these fact=F1 detector=none
name: Test-side signals only to recorded processes
rule: Test-side code signals only a process it recorded: a witness kills a survivor only when its pid and its start time both still match the tree it recorded, never by pid alone or by name.
guard: cross_killed_test, harness_tied
basis: fct-y7jldnao
because: The chunk adds witnesses that SIGKILL survivors at a deadline only where pid and start time still match the recorded tree, a test-side rule no row of the section yet states.
@row_add n=2 row=decision/test-children-tied-to-their-test area=security-anti-patterns-never-do-these fact=F2 detector=none
name: Long-lived test children tied to their test
rule: Every long-lived viola child a root test spawns, and an in-test harness session's supervisor, is tied to its test by a parent-death SIGKILL, so a killed or panicked test leaves no wrapper or fake agent running.
guard: cross_killed_test_census_finds_no_untied_viola_spawn, killed_test_leaves_no_harness_session
except: the test harness CLI boot's supervisor is left untied
basis: fct-hkrr5z5i
because: The chunk ties every root viola spawn and the in-test harness session's supervisor to the test by parent-death SIGKILL, leaving the harness CLI boot untied, a convention no row of the section yet states.
