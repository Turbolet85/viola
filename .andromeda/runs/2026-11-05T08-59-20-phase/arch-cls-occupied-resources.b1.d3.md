@answer n=1 entity=occupied-resources#1.1 v=yes
why: the Linux socket-dir rules (per-user 0700 dir, socket-dir-refused, 0600 socket, no abstract names) stay, and the macOS $TMPDIR arm this entity names is removed (needs 2).
@answer n=2 entity=occupied-resources#2.1 v=no
why: a force-killed wrapper's stale socket file is left in place; the chunk adds no socket removal and no OS arm touches that lifecycle.
@answer n=3 entity=occupied-resources#3.1 v=yes
why: the pre-frame peer pid and start-time check and the Unix euid check on connect stay on Linux; the macOS pid carve-out loses its code and the euid gate becomes unconditional (needs 8, 9).
@answer n=4 entity=occupied-resources#4.1 v=no
why: the fake-agent argv options, print mode and output contracts are test-only behaviour the chunk neither gates nor removes; the scripts/agent-run pointer does not carry this rule.
@answer n=5 entity=occupied-resources#5.1 v=yes
why: the exit codes of run, revive, list, plugin install and verify must come out of the removal unchanged on Linux (needs 7; the scope's exit-code clause).
@answer n=6 entity=occupied-resources#6.1 v=yes
why: link and unlink exit 0, 1, 20 and 21, including the Unix-only euid refusal, must stay unchanged on Linux (needs 7, 9).
@answer n=7 entity=occupied-resources#7.1 v=yes
why: send, wait, last, answer, pause and release exit 21 on a refused instance, socket directory or euid mismatch, and those exit codes must hold on Linux (needs 7, 9).
@answer n=8 entity=occupied-resources#8.1 v=yes
why: hook <event> --capture and hook statusline --capture always exit 0 with empty stderr, the hook contract the scope keeps unchanged on Linux (needs 7).
@answer n=9 entity=occupied-resources#9.1 v=yes
why: the child's --settings flag is passed only when an override was written, and none is written on Windows; the Unix gate becomes unconditional (needs 11).
@answer n=10 entity=occupied-resources#10.1 v=yes
why: verify's Run B passes --settings on Unix with no flag on Windows; the Windows arm goes and the Unix gate becomes unconditional (needs 11).
@answer n=11 entity=occupied-resources#11.1 v=yes
why: the per-session settings override is Unix-only, its Unix gate becomes unconditional, and its Windows override is owed to the retired Windows-only live measurements entry (needs 11, 21).
@answer n=12 entity=occupied-resources#12.1 v=no
why: VIOLA_DIR's setting, reads and grandparent home derivation carry no OS arm, name no unit the scope edits, and state no exit code or hook contract.
@answer n=13 entity=occupied-resources#13.1 v=yes
why: hook's trusted and check_instance run before any read, and an absent instance exits 0 with nothing written, the hook contract the scope keeps; the strict-modes Windows halves leave (needs 7, 10).
@answer n=14 entity=occupied-resources#14.1 v=yes
why: the R8 strip's Windows registry reader leaves, the Unix claude_env_keep pass-list stays, and the identity floor and names-only rule must hold (needs 12).
@answer n=15 entity=occupied-resources#15.1 v=yes
why: it names the AGENT_RUN_ readers the scope changes: run --mutants (AGENT_RUN_CHUNK_BASE, KEEP_* set to 0) on the removed windows-mutants.yml path, and the test-side keepers under target/e2e-home (needs 15, 17).
@answer n=16 entity=occupied-resources#16.1 v=yes
why: the forced-panic path of hook exits 0 with empty stdout and stderr, the hook contract the scope keeps unchanged on Linux (needs 7).
@answer n=17 entity=occupied-resources#17.1 v=yes
why: config.json's claude_env_keep is the Unix R8 pass-list that stays, with its bad-value rejection (needs 12).
@answer n=18 entity=occupied-resources#18.1 v=yes
why: the pinned copy's viola(.exe) naming is a Windows-naming closed word the removal leaves, pin_companions shares this unit, and pinned-hash-mismatch exit 1 stays (needs 3, 7, 22).
@answer n=19 entity=occupied-resources#19.1 v=no standing=S6
why: the codes-only, one-line-per-write diagnostics rule fixed by the obs-owned diag schemas is the log-line convention; the chunk adds no line instance this table must record.
@answer n=20 entity=occupied-resources#20.1 v=yes
why: the channel-response detail closed words and their WARN Drop are wire closed words the scope keeps unchanged on Linux.
@answer n=21 entity=occupied-resources#21.1 v=yes
why: verify's refusal codes include batch-script-child, a closed word naming Windows (.cmd/.bat children) that the removal leaves naming code that no longer exists (needs 22).
@answer n=22 entity=occupied-resources#22.1 v=no standing=S6
why: the role-file routing and codes-only shape of one state-recovered log line is the log-line convention; the chunk adds no such line instance.
@answer n=23 entity=occupied-resources#23.1 v=yes
why: check_diagnostics on diagnostics/ and its role and detail files, where the Windows write rule leaves and the Unix read rule keeps its gate (needs 10).
@answer n=24 entity=occupied-resources#24.1 v=yes
why: the strict-modes entry points over instance files and diagnostics/, including events.ndjson's write rule on Unix and read rule on Windows; the Windows halves leave and the Unix halves lose their gate (needs 10).
@answer n=25 entity=occupied-resources#25.1 v=yes
why: viola_state::strict's read rule has a Windows DACL half that leaves and a Unix mode half that loses its gate (needs 10).
