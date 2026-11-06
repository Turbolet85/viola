@answer n=1 entity=unit-test-strategy#1.1 v=no
why: the viola-core code-word and UnreachableCause cases sit in crates/viola-core/src/lib.rs, a file the session did not reach and no task item names
@answer n=2 entity=unit-test-strategy#1.2 v=no
why: the viola-channel codec, fault screen, endpoint vectors and peer-credential cases sit in crates/viola-channel, which the session did not reach
@answer n=3 entity=unit-test-strategy#1.3 v=yes mode=stale
why: its viola list replay witness cites tests/cli_list.rs, a reached file whose lines can have moved with the Windows-remnant edits
@answer n=4 entity=unit-test-strategy#1.4 v=yes mode=contradict
why: the statusline-rate-limits arm and the claim that all 24 rows are relied on every build treat the row as a CLI capture carrying both windows, which the live round disproved (seven_day only, verify read 23 pass 1 fail on claude 2.1.287)
@answer n=5 entity=unit-test-strategy#1.5 v=no
why: the viola-mcp stdin cap and serve cases sit in crates/viola-mcp, which the session did not reach and no task item names
@answer n=6 entity=unit-test-strategy#2.1 v=yes mode=stale
why: its refusal-order cases name src/cmd/send.rs (deliver_empty_text_is_refused_before_any_frame and refusal_of_reads_a_refused_character_before_an_empty_text), a reached file whose Git Bash rewritten-path warning of send was retired
@answer n=7 entity=unit-test-strategy#2.2 v=yes mode=stale
why: ensure_e2e_home and its e2e_home_* cases sit in crates/viola-e2e/src/harness/mod.rs, a reached file the session edited whole and whose claim is not shown to hold
