@row_set n=1 row=decision/root-wait-bound cell=rule seen=1 detector=D-tests-count-literals
value: Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first.
basis: fct-w5d65k72
because: The shared root wait bound moved from 7 s to 12 s and the mutants default kill from 10 s to 15 s, so the row's two count literals are stale.
@row_set n=2 row=decision/mutants-kill-below-cargo-mutants-timeout cell=rule seen=1 detector=D-tests-count-literals
value: Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout.
basis: fct-xwczwhqo
because: The mutants profile's default slow-timeout kill moved from 10 s to 15 s, still below the 20 s floor, so the rule's kill literal must follow.
@row_set n=3 row=decision/send-window-paste-hint-30s-kill cell=why seen=1 detector=D-tests-count-literals
value: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint.
basis: fct-xthvqirk
because: The why names the 10 s default kill, which is now 15 s while the class's own 30 s kill is unchanged, so the stale default-kill literal leaves the reason.
@row_set n=4 row=decision/within-7s-wait-bound cell=rule seen=1 detector=D-tests-count-literals
value: WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
basis: fct-w5d65k72
because: The shared root wait bound WITHIN moved from 7 s to 12 s, so the rule's count literal is stale.
@row_add n=1 row=decision/record-home-names-no-home area=test-data-fixtures fact=F1 detector=none
name: Record homes never named home
rule: A home a test gives to viola verify --record comes from the shared record-home helper and fixture, and no component of its path is named home.
guard: record_home_names_no_path_component_home
basis: fct-iq6zeo4z
because: The chunk adds the record-home rule for test homes given to verify --record, and no test-plan row states it.
@row_add n=2 row=decision/test-sockets-under-short-tmp-root area=test-data-fixtures fact=F2 detector=none
name: Test channel sockets under a short /tmp root
rule: Every test-made channel socket binds under a short test-owned socket root in /tmp and never under TMPDIR, in every crate that binds one.
why: A long scratch TMPDIR would overflow sun_path.
guard: impostor_endpoint_fits_sun_path_at_the_widest_pid_and_label
basis: fct-w5d65k72
because: Every viola-test-chan- socket moved to a short /tmp root after a long scratch TMPDIR overflowed sun_path, and no test-plan row states where test sockets bind.
