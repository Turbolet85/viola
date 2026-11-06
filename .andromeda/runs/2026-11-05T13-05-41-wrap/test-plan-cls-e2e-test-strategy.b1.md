@answer n=1 entity=e2e-test-strategy#1.1 v=yes mode=stale
why: the session made the cross-list dim test run unconditionally and retired the windows-2025 and macos-latest legs, so the windows-2025 attribute set and the cfg(unix) ubuntu-latest and macos-latest run are stale
@answer n=2 entity=e2e-test-strategy#1.2 v=no
why: the hook statusline driver row names no OS and its fixture and env_clear rule are not moved by the statusline argv change or the Windows retirements
@answer n=3 entity=e2e-test-strategy#1.3 v=yes mode=contradict
why: the session retired ConPTY so pty_backend is always openpty, and this row still names ConPTY beside openpty and says the literals are absent on every OS
@answer n=4 entity=e2e-test-strategy#1.4 v=no
why: the channel error-body test and its seven fault rows are not in a reached file, and the peer euid and server pid retirements do not touch the detail words
@answer n=5 entity=e2e-test-strategy#1.5 v=no
why: the MCP stdio driver row names no OS and its rmcp client, MAX_FRAME cap and kill deadline are untouched by the session
@answer n=6 entity=e2e-test-strategy#1.6 v=no
why: the library-only row names only nextest and doc tests with no OS or count, and the dev-tool versions were re-read unchanged
@answer n=7 entity=e2e-test-strategy#2.1 v=no
why: the Path 1 start steps name no Windows arm, no sideload step and no pin_companions call, and the tamper steps still hold on Linux
@answer n=8 entity=e2e-test-strategy#2.2 v=yes mode=stale
why: the session removed the Windows arm of the settings.json rewrite and of the home-tree strict-modes check, so the On Windows no settings.json clause is stale
@answer n=9 entity=e2e-test-strategy#2.3 v=no
why: the one-flow binary and its wall reading are on the Linux dev host and no reached file or retired item names this flow
@answer n=10 entity=e2e-test-strategy#2.4 v=no
why: the cleanup step names only Drop guards and TempDir removal, which the session did not change
@answer n=11 entity=e2e-test-strategy#3.1 v=yes mode=stale
why: the session retired the Windows and macOS legs, so the claim that the cli_send coverage runs on all three CI OSes is stale
@answer n=12 entity=e2e-test-strategy#3.2 v=no
why: the boot and send steps name no OS and their agent-run boot flags are not among the session's moves
@answer n=13 entity=e2e-test-strategy#3.3 v=yes mode=stale
why: the cli_send tail cases claim all three CI OSes, and the owed Windows reading points at the retired Windows-only live measurements entry
@answer n=14 entity=e2e-test-strategy#3.4 v=no
why: the cleanup step names only the harness cleanup command, which the session did not change
@answer n=15 entity=e2e-test-strategy#4.1 v=yes mode=stale
why: the cli_wait_last coverage claim of all three CI OSes is stale after the session retired the Windows and macOS legs
@answer n=16 entity=e2e-test-strategy#4.2 v=no
why: the wait and last steps name no OS, and their timeouts and the gated path3 script are unchanged by the session
@answer n=17 entity=e2e-test-strategy#4.3 v=no
why: the verification signals name exit codes, record kinds and the endpoint-lost and instance-dead causes, none of which the session moved
@answer n=18 entity=e2e-test-strategy#4.4 v=no
why: the cleanup step checks killed wrappers by sysinfo pid and start time, which no retired item touches
@answer n=19 entity=e2e-test-strategy#4.5 v=yes mode=stale
why: run_child_ends_when_its_wrapper_is_terminated is now kill -KILL only on Unix, so its on every OS claim and its Windows OpenProcess and TerminateProcess arm are stale
@answer n=20 entity=e2e-test-strategy#5.1 v=yes mode=stale
why: the cli_answer coverage claim of all three CI OSes no longer holds after the Windows and macOS legs were removed
@answer n=21 entity=e2e-test-strategy#5.2 v=no
why: the question permission and plan steps name fake scripts and answer invocations that name no OS and sit in no reached file
@answer n=22 entity=e2e-test-strategy#5.3 v=no
why: the expected hook stdout and refusal bodies name no OS, and the session did not move the answer or hook contracts
@answer n=23 entity=e2e-test-strategy#5.4 v=no
why: the cleanup step names only the harness cleanup command, which the session did not change
@answer n=24 entity=e2e-test-strategy#6.1 v=yes mode=stale
why: tests/tui_wheel.rs claims its surfaces on all three CI OSes with a windows-2025 focus case, and the session retired both
@answer n=25 entity=e2e-test-strategy#6.2 v=no
why: the take-the-wheel steps name key writes, pause, release and fake scripts, and none is an OS arm the session retired
