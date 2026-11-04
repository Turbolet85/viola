@answer n=1 entity=occupied-resources#1.1 v=yes
why: the Linux and macOS socket path and its per-user 0700 directory are the IPC endpoint that task item 8 names
@answer n=2 entity=occupied-resources#2.1 v=yes
why: a force-killed wrapper's socket file stays, so the kill-and-revive tests of task item 13 must allow a revived bind to take it over
@answer n=3 entity=occupied-resources#3.1 v=yes
why: the pre-frame peer pid, start-time and Unix euid checks and the macOS no-peer-pid gap are the endpoint and carve-out of task items 8 and 9
@answer n=4 entity=occupied-resources#4.1 v=yes
why: viola-fake-agent and its argv options are the CI stand-in the suite drives on all three OSes, task item 2
@answer n=5 entity=occupied-resources#5.1 v=yes
why: the list and revive subcommands, their exit codes and their --json refusal documents are the CLI contract that task item 6 names
@answer n=6 entity=occupied-resources#6.1 v=yes
why: the link and unlink exit codes and --json documents, including the Unix euid refusal, are the CLI contract of task items 6 and 8
@answer n=7 entity=occupied-resources#7.1 v=yes
why: the exit 21 of send, wait, last, answer, pause and release on a refused socket directory or a server of another euid is the Unix endpoint check of task items 8 and 10
@answer n=8 entity=occupied-resources#8.1 v=no
why: the hidden hook capture arms serve verify's probe, which the chunk neither names nor changes; hook's exit contract (task item 18) is carried by the product arms
@answer n=9 entity=occupied-resources#9.1 v=yes
why: the child's --settings flag on Unix and the direct child spawn with --resume and --fork-session are the run path that task items 3 and 14 name
@answer n=10 entity=occupied-resources#10.1 v=no
why: verify's probe plugin and Runs A to E are verify's own surface, which the scope does not name and the chunk does not change
@answer n=11 entity=occupied-resources#11.1 v=yes
why: the per-session settings.json override written only on Unix is named by task item 14
@answer n=12 entity=occupied-resources#12.1 v=yes
why: VIOLA_DIR set in the child carries the test's --home into hooks and CLI calls, the per-test home isolation of task item 3
@answer n=13 entity=occupied-resources#13.1 v=yes
why: task item 18 names viola hook's always-exit-0 contract, and a VIOLA_DIR naming a missing instance is one of the hook paths the hook tests read on each OS
@answer n=14 entity=occupied-resources#14.1 v=yes
why: the R8 CLAUDE* strip, the identity floor and the Unix claude_env_keep pass-list are named by task item 14
@answer n=15 entity=occupied-resources#15.1 v=yes
why: AGENT_RUN_KEEP_HOMES keeps the target/e2e-home homes the obs gates read (task item 4), and the CI variable refuses a live claude (task item 6)
@answer n=16 entity=occupied-resources#16.1 v=yes
why: task item 2 names the two seams FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC, both read only under the fake-agent feature
@answer n=17 entity=occupied-resources#17.1 v=yes
why: config.json's claude_env_keep is the Unix R8 pass-list that task item 14 names
@answer n=18 entity=occupied-resources#18.1 v=no
why: the pinned bin copy placement is owned by Version done-check (task item 13, not_needed), and the chunk adds no pin path
@answer n=19 entity=occupied-resources#19.1 v=yes
why: the diagnostics role files, codes only, one line per write, fixed by diag-line.v1.json are named by task items 4 and 5
@answer n=20 entity=occupied-resources#20.1 v=yes
why: the channel-response detail closed enum in diag-line.v1.json is a closed word that the one-schema rule of task items 4 and 5 carries on every OS
@answer n=21 entity=occupied-resources#21.1 v=no
why: verify's refusal codes in process-exit detail are verify's own closed words, which the scope does not name and the chunk does not change
@answer n=22 entity=occupied-resources#22.1 v=no
why: the state-recovered line written by replay readers is not an OS-differing behaviour and the scope names no replay path
@answer n=23 entity=occupied-resources#23.1 v=yes
why: the diagnostics folder and file strict-modes rule (Unix read rule, Windows write rule) is an OS-differing strict-modes rule that task item 14 names
@answer n=24 entity=occupied-resources#24.1 v=yes
why: the instance files and their entry-point checks, with events.ndjson's write rule on Unix, are the strict-modes rules that task item 14 names
@answer n=25 entity=occupied-resources#25.1 v=yes
why: the strict module's read rule (mode bits on Unix, the ACE rule on Windows) is the OS-differing strict-modes rule that task item 14 names
