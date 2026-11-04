@answer n=1 entity=occupied-resources#1.1 v=yes
why: the Linux and macOS socket path, its per-user 0700 directory and the check before bind are task item n=8
@answer n=2 entity=occupied-resources#2.1 v=yes
why: a force-killed wrapper's leftover socket that the next bind takes over is what the Unix kill-and-revive support (task item n=13) runs under
@answer n=3 entity=occupied-resources#3.1 v=yes
why: the peer pid and start-time check, the Unix peer-euid check and the macOS no-peer-pid gap are task items n=8 to n=10
@answer n=4 entity=occupied-resources#4.1 v=yes
why: the viola-fake-agent argv that the fake-agent suite drives on all three OSes is the fake agent of task item n=2
@answer n=5 entity=occupied-resources#5.1 v=yes
why: the list, revive and plugin install exit codes and the revive refusal documents are CLI contracts named by task items n=6 and n=18
@answer n=6 entity=occupied-resources#6.1 v=yes
why: the link and unlink --json documents with their closed detail words and the euid refusal are CLI contracts named by task items n=6 and n=10
@answer n=7 entity=occupied-resources#7.1 v=yes
why: the exit 21 instance-unreachable documents with detail socket-dir-refused and server-verify-failed are the closed words of task items n=6 and n=8
@answer n=8 entity=occupied-resources#8.1 v=yes
why: the hook capture arms are the hook contract of task item n=18 (exit 0, empty stderr), and verify's print probe drives them in the fake-agent suite
@answer n=9 entity=occupied-resources#9.1 v=yes
why: the child's Unix-only --settings flag is OS-differing task item n=14, and the child argv is spawned directly per task item n=3
@answer n=10 entity=occupied-resources#10.1 v=yes
why: verify's probe plugin and its Runs A to E are driven by the fake agent in the suite, and Run B's --settings is Unix only (task item n=14)
@answer n=11 entity=occupied-resources#11.1 v=yes
why: the per-session settings.json override written only on Unix is named by task item n=14
@answer n=12 entity=occupied-resources#12.1 v=yes
why: the VIOLA_DIR grandparent rule keeps a session started with --home and its hooks and nested CLI in its own test home, the isolation of task item n=3
@answer n=13 entity=occupied-resources#13.1 v=yes
why: the hook's VIOLA_DIR check and its exit 0 with no write are the hook contract of task item n=18
@answer n=14 entity=occupied-resources#14.1 v=yes
why: the Unix claude_env_keep pass-list and the identity floor are named by task item n=14
@answer n=15 entity=occupied-resources#15.1 v=yes
why: AGENT_RUN_KEEP_HOMES keeps the target/e2e-home homes that the obs gates read in CI, as task item n=4 names, so the entity binds through that variable
@answer n=16 entity=occupied-resources#16.1 v=yes
why: FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC are named by task item n=2
@answer n=17 entity=occupied-resources#17.1 v=yes
why: config.json carries claude_env_keep, the Unix R8 pass-list named by task item n=14; its budget thresholds are the not-needed part
@answer n=18 entity=occupied-resources#18.1 v=no
why: the pinned-copy layout under bin/ is not_needed (task item n=13, owned by Version done-check) and no new code or test must act on pin placement
@answer n=19 entity=occupied-resources#19.1 v=yes
why: the diagnostics role files, codes only and one line per write, are named by task item n=5 and the one-schema scope item
@answer n=20 entity=occupied-resources#20.1 v=yes
why: channel-response's closed detail enum is a process-log closed word that the one-schema scope item covers
@answer n=21 entity=occupied-resources#21.1 v=yes
why: verify's process-exit refusal codes run under the fake agent in the suite and are closed words of the one diagnostics schema
@answer n=22 entity=occupied-resources#22.1 v=yes
why: the state-recovered process-log line on replay is a closed-word line that the one-schema scope item covers
@answer n=23 entity=occupied-resources#23.1 v=yes
why: the diagnostics strict-modes check reads the Unix read rule and the Windows write rule per OS, a difference task item n=14 names
@answer n=24 entity=occupied-resources#24.1 v=yes
why: the instance files, the events.ndjson write rule on Unix and the Unix-only settings.json are named by task items n=11 and n=14
@answer n=25 entity=occupied-resources#25.1 v=yes
why: the strict read and write rules, the Unix group and other bit refusal and socket-dir-refused are named by task items n=8 and n=14
