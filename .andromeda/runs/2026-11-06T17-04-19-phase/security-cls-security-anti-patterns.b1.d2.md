@answer n=1 entity=security-anti-patterns#1.1 v=yes
why: the live sessions and the channel handle measurement drive channel frames, which must pass the existing peer and server-verification checks before any send (task item 8 and need 4).
@answer n=2 entity=security-anti-patterns#2.1 v=yes
why: the e2e harness edits under crates/viola-e2e/src/harness and the target/e2e-home keepers and removal rules are named by task need 17.
@answer n=3 entity=security-anti-patterns#3.1 v=no
why: the chunk answers no dialog with a decision and changes no stamp write (task not_needed 13); the dialog hold is read, not answered.
@answer n=4 entity=security-anti-patterns#3.2 v=no
why: the chunk runs no viola verify recording and touches no verify home-tree or stamps ordering (task not_needed 6).
@answer n=5 entity=security-anti-patterns#4.1 v=yes
why: the test seams FAKE_AGENT_HOOK_PANIC and PTY_SIDELOAD_TEST_CASE, the no-new-seam rule and the config-switches-no-control rule are named by task needs 7, 10 and 12.
@answer n=6 entity=security-anti-patterns#4.2 v=yes
why: task need 13 names the always-admit peer_admitted survivor at the three client seams and bars adding any seam to reach it.
