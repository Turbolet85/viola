@answer n=1 entity=unit-test-strategy#1.1 v=no
why: its rules live in crates/viola-core/src/lib.rs, which the session did not reach, and it names no reached test file
@answer n=2 entity=unit-test-strategy#1.2 v=no
why: its rules live in viola-channel src files, which the session did not reach, and it names no reached test file
@answer n=3 entity=unit-test-strategy#1.3 v=yes mode=stale
why: it cites tests/cli_list.rs, a reached file, for e5_list_reads_a_live_wrapper_through_the_replay, and no report shows that claim still holds on the changed file
@answer n=4 entity=unit-test-strategy#1.4 v=yes mode=stale
why: it cites tests/cli_verify.rs, a reached file, as holding the refusal messages and outputs, and no report shows that claim still holds on the changed file
@answer n=5 entity=unit-test-strategy#1.5 v=no
why: its rules live in viola-mcp src files, which the session did not reach, and it names no reached test file
@answer n=6 entity=unit-test-strategy#2.1 v=no
why: its rules live in root bin src files and in tests/contract_bounded_readers.rs, none of which the session reached, and it names no Frozen guard, OS gate or census row the session moved
