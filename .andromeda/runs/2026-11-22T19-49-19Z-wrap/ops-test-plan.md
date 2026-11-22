@row_set n=1 row=decision/verify-window-80s-kill cell=rule seen=1 detector=none
value: Tests named verify_window_, and the fake agent dialog and framing replay tests over every recorded set, take an 80 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
basis: fct-wyctk5vr
because: The verify_window_ override of both the ci and mutants profiles now also matches the two fake_agent dialog and framing replay tests, so the class the 80 s kill covers is no longer the verify_window_ names alone.
@row_add n=1 row=decision/claimed-start-failed-snapshot-unconfirmed area=unit-test-strategy fact=F2 detector=D-tests-coverage
name: Claimed session-start, failed snapshot write, unconfirmed
rule: A session-start the send confirmation claimed whose snapshot write fails settles the send empty, so the confirmation deadline ends it not-delivered / no-prompt-submitted, never confirmed and never hung.
guard: send_a_claimed_session_start_whose_snapshot_write_fails_ends_unconfirmed
basis: fct-ybqmyx4p
because: The chunk made a claimed session-start whose snapshot write fails settle the slot empty and end through the confirmation deadline, a send outcome no unit-strategy row states.
@row_add n=2 row=decision/harness-abort-wait-below-ci-kill area=5-command-implementation fact=F3 detector=none
name: Harness abort wait below the ci kill
rule: After a failed readiness the harness boot waits at most 15 s for its abort, so a stuck boot ends within 17 s, below the 20 s ci per-test kill of the harness lifecycle binary; the 20 s readiness bound per instance stays.
basis: fct-ybqmyx4p
because: The abort wait after a failed readiness moved from 20 s to 15 s so a stuck boot ends below the harness lifecycle binary's ci kill, a bound no harness row states.
@row_none n=1 fact=F1 type=4
relied: The claude agents --json read used by list, revive and verify is killed at its own 5 s bound, apart from the version probe's; the code and the security-plan row on the agents read hold it, the value kept and not measured against the real CLI.
