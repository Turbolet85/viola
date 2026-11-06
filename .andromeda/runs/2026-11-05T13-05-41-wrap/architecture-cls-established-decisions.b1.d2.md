@answer n=1 entity=established-decisions#1.1 v=yes mode=contradict
why: the session moved deny-sync.toml and the sync-crate cargo check from three OS triples to ubuntu-latest alone, so the three-OS CI claim no longer holds
@answer n=2 entity=established-decisions#2.1 v=yes mode=contradict
why: the Windows ConPTY sideload, the ReadConsoleW host_stdin and the TerminateProcess fallback are removed, the =0.8.1 pin stands, and RUSTSEC-2026-0441 (F1) awaits the CARRY on Version done-check
@answer n=3 entity=established-decisions#3.1 v=yes mode=stale
why: its owner "Windows-only live measurements" is the working-route entry retired at 2026-11-04T23-11-43-wrap, so the owner citation must be re-pointed or retired
@answer n=4 entity=established-decisions#4.1 v=no
why: the vt100 gate, its signatures and its quiet-period and wait rows are not among the session's changes, and the ledger.rs change touches only the shim row
@answer n=5 entity=established-decisions#5.1 v=yes mode=stale
why: its "three CI OSes" claim and its owed "Windows-only live measurements" citation move with CI now running on ubuntu-latest only
@answer n=6 entity=established-decisions#6.1 v=yes mode=contradict
why: the lock-file separation rationale rests on a Windows claim the session retired, so it no longer holds
@answer n=7 entity=established-decisions#7.1 v=yes mode=contradict
why: replace_private is now one attempt, with retry_replace, REPLACE_ATTEMPTS, replace_private_retrying and replace_private_with removed and the Windows companions example gone
@answer n=8 entity=established-decisions#8.1 v=no
why: place_private_new and Placed are not among the names the session removed or moved, and its no-retry, no-clobber contract holds on fs.rs
@answer n=9 entity=established-decisions#8.2 v=yes mode=contradict
why: the Windows run variant of place_private_new_placed_copy_runs_at_once goes with the removed Windows code, so "runs it on Windows" no longer holds
@answer n=10 entity=established-decisions#8.3 v=no
why: the write-handle-closed-before-placement rule and its Linux ETXTBSY measurement are unchanged by the replace_private edits
@answer n=11 entity=established-decisions#9.1 v=yes mode=contradict
why: the Windows named pipe, the SQOS Identification client open and the one-API-over-named-pipes rationale are retired, leaving a Unix socket only
@answer n=12 entity=established-decisions#10.1 v=no
why: client.rs still carries no check inside Client::connect (only the Windows SQOS open leaves it), so the claim still holds on the changed file
@answer n=13 entity=established-decisions#11.1 v=yes mode=contradict
why: the Windows GetNamedPipeServerProcessId leg and the SERVER_PID_REPORTED macOS flag are retired with the folded pid report
@answer n=14 entity=established-decisions#11.2 v=yes mode=contradict
why: PEER_EUID_REPORTED is folded away and peer_euid_admitted is now observed == Some(own), so the reported-flag form no longer holds
@answer n=15 entity=established-decisions#12.1 v=no
why: the accept_loop admission chain through peer_admitted holds on the changed files, and the folded flag inside peer_euid_admitted is one this entity does not name
@answer n=16 entity=established-decisions#13.1 v=yes mode=contradict
why: the macOS no-peer-pid recorded gap and its Matches arm in server_verdict are retired
@answer n=17 entity=established-decisions#14.1 v=yes mode=contradict
why: the off-Unix Ok(()) arm of check_socket_dir is gone and the Unix check body is unconditional
@answer n=18 entity=established-decisions#15.1 v=no
why: the replay path read_snapshot_or_replay is not among the session's changes
@answer n=19 entity=established-decisions#16.1 v=no
why: statusline serve reads through replay and runs nothing when the replayed snapshot holds no statusline_command, which the shell_argv change does not alter
@answer n=20 entity=established-decisions#17.1 v=no
why: the MCP tools, rmcp and release surface are untouched per the task
@answer n=21 entity=established-decisions#18.1 v=no
why: the MCP CappedLines stdin bound and MAX_FRAME are untouched
@answer n=22 entity=established-decisions#19.1 v=no
why: check_instance still runs before live_endpoint with its strict-modes-failed refusal, and the removed Windows strict clauses do not alter that
@answer n=23 entity=established-decisions#19.2 v=no
why: the unreachable mapping still reads each refusal to its cause, and the folded verify_server parameter does not change the ServerVerify result
@answer n=24 entity=established-decisions#20.1 v=no
why: the MCP oversize cut and its parse-rejected line are untouched
@answer n=25 entity=established-decisions#20.2 v=no
why: the exit_code mapping for Role::Mcp and the unit child's own code 11 are untouched
