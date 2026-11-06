@answer n=1 entity=unit-test-strategy#1.1 v=no
why: viola-core's rules live in crates/viola-core/src/lib.rs, which the change does not reach, and no moved crate feeds them.
@answer n=2 entity=unit-test-strategy#1.2 v=no
why: viola-channel's rules live in crates/viola-channel/src, which the change does not reach, and its peer-credential and frame claims name none of the moved crates.
@answer n=3 entity=unit-test-strategy#1.3 v=no
why: viola-state's rules live in crates/viola-state/src, which the change does not reach, and its cited tests/cli_list.rs replay read is not one the Windows-remnant retirement alters.
@answer n=4 entity=unit-test-strategy#1.4 v=yes mode=contradict
why: the statusline-rate-limits row is disproved as a CLI behaviour (five_hour unknown, seven_day only on claude 2.1.287), so its CLI-capture arm and its relied-set-on-every-build claim no longer hold.
@answer n=5 entity=unit-test-strategy#1.5 v=no
why: viola-mcp's rules live in crates/viola-mcp/src, which the change does not reach, and its serve child cases name no moved crate or Windows reading.
@answer n=6 entity=unit-test-strategy#2.1 v=yes mode=stale
why: src/cmd/send.rs is reached whole (the Git Bash rewritten-path warning emitter retired there), and the client refusal and dispatch-span send tests this entity cites live in that file.
@answer n=7 entity=unit-test-strategy#2.2 v=yes mode=stale
why: crates/viola-e2e/src/harness/mod.rs is reached whole (Windows remnants retired there), and this entity's ensure_e2e_home headroom floor and refusal rule live in that file.
