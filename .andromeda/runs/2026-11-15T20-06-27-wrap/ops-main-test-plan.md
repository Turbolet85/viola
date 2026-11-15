@row_set n=1 row=decision/mutants-kill-below-cargo-mutants-timeout cell=rule seen=1 detector=D-tests-count-literals
value: Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout.
basis: fct-xwczwhqo
because: The mutants profile's default kill moved from 10 s to 15 s, still below the 20 s floor, a stated departure from the 2026-10-20 line that a slow case is fixed by removing its slow work and never by raising the default, taken on the operator's P4 answer relayed by the overseer as delegate (inputs#I2), since the stamped-home class is now the default population.
