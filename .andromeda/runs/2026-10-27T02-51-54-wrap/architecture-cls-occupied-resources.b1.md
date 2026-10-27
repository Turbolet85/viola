# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the IPC endpoints and the Unix socket dir are untouched per the Untouched list, and no reached file holds their rule
occupied-resources#2.1 · no · the server identity check is untouched per the Untouched list, and no reached file holds the peer check
occupied-resources#3.1 · yes · add-to · the fake agent's hook receipt gains spawn_os_error on a failed absolute spawn (F7) while its binary list and receipt path stand
occupied-resources#4.1 · no · the subcommand surface and its CLI exit codes are untouched per the Untouched list, with no src/cmd file changed
occupied-resources#5.1 · no · the link and unlink arguments and their exit codes are untouched per the Untouched list, with no src/cmd file changed
occupied-resources#6.1 · no · the exit codes and refusal pairs are untouched per the Untouched list, and the home-tree check was re-read as unchanged in Touches
occupied-resources#7.1 · no · the hidden capture arms and their bounds are untouched, since the Hook Contract is unchanged and no src/cmd file changed
occupied-resources#8.1 · no · the flags passed to the child and the per-session settings override are untouched per the Untouched list
occupied-resources#9.1 · no · verify's probe plugin, its child flags and its runs are untouched, and the pinned path it names is unchanged per Touches
occupied-resources#10.1 · no · the per-session settings override is untouched per the Untouched list, and the pinned path it names is unchanged per Touches
occupied-resources#11.1 · no · no environment variable is read or set, per the Untouched list, so VIOLA_DIR's readers are untouched
occupied-resources#12.1 · no · the hook's instance order and fail-open path are untouched, since the Hook Contract is unchanged and no environment variable changed
occupied-resources#13.1 · no · the R8 strip and identity floor are untouched, since no environment variable changed and the strip lives in the agent crate, not a reached file
occupied-resources#14.1 · no · the test-harness environment variables are untouched, since no environment variable is read or set differently
occupied-resources#15.1 · no · the test seams are untouched, since no test seam was added and the fake agent reads no environment
occupied-resources#16.1 · no · no config key changed, and the config readers are not in a reached file
occupied-resources#17.1 · no · the role-file set, its producers and the line format are untouched, since no log line, schema or refusal code changed
occupied-resources#18.1 · no · the replay readers and their state-recovered lines are untouched, since no log line changed and replay is not a reached file
occupied-resources#19.1 · yes · stale · strict.rs is reached whole and the report does not re-read check_diagnostics' per-path verdicts
occupied-resources#20.1 · yes · contradict · the report disproves the plain-home refusal naming Users (S-1-5-32-545), since the refused entry is the inherited untrusted write grant whichever SID holds it (F6)
occupied-resources#21.1 · yes · stale · strict.rs is reached whole and the report does not re-read the read rule's unix and windows verdicts or its READ_RIGHTS mask
occupied-resources#22.1 · no · the home-tree check was re-read as unchanged, with run still pinning the exe bytes it checked and the start order unchanged
occupied-resources#23.1 · no · the call sites and their start order are untouched, since no src/cmd file changed and run's start order is unchanged
occupied-resources#24.1 · yes · stale · strict.rs is reached whole and the report does not re-read the NotFound skip or the per-OS regular-file oracle
occupied-resources#25.1 · yes · stale · strict.rs is reached whole and the report does not re-read check_path's write rule against check_instance's read rule
