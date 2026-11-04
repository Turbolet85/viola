@answer n=1 entity=security-anti-patterns#1.1 v=yes
why: the pid, euid and start-time verification before any channel frame, and the macOS euid-only carve-out, bind the Unix client and server seams (task items 5 and 6)
@answer n=2 entity=security-anti-patterns#2.1 v=no
why: the run --mutants scratch wipe is Windows-only and the chunk does not touch run --mutants (task item 5, not_needed)
@answer n=3 entity=security-anti-patterns#3.1 v=yes
why: the removal and linked-base keepers under target/e2e-home are task item 18, which the Unix fake-agent tests now hit
@answer n=4 entity=security-anti-patterns#4.1 v=yes
why: task item 11 names the hook's no-body output on a failing security check, the same behaviour this states for an unstamped home's hook.dialog
@answer n=5 entity=security-anti-patterns#4.2 v=yes
why: task item 7 puts the Unix strict-modes check before the first read in every process, the ordering verify and update_stamps state
@answer n=6 entity=security-anti-patterns#5.1 v=yes
why: the scope names the seams FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC (item 14) and NO_COLOR, TERM and COLORTERM (item 13), so the switch-off ban over them binds
@answer n=7 entity=security-anti-patterns#5.2 v=yes
why: task item 4 names the ban on any env, flag or feature seam forcing a refusing peer-euid decision and on an unrecorded Unix-only test seam
