@answer n=1 entity=established-decisions#1.1 v=yes mode=contradict
why: the session cut deny-sync.toml to one Linux triple and runs the sync-crate cargo check on one OS, so the Windows, macOS and Linux enforcement it states is no longer true
@answer n=2 entity=established-decisions#2.1 v=yes mode=contradict
why: the session removed the ConPTY sideload, the System32 DLL restriction, the Windows ReadConsoleW host_stdin reader, the TerminateProcess fallback and the Windows 0.9.0 rationale, and F1 (RUSTSEC-2026-0441 on the =0.8.1 pin) is carried beside this pin
@answer n=3 entity=established-decisions#3.1 v=yes mode=stale
why: its owner "Windows-only live measurements" was retired at the 2026-11-04T23-11-43-wrap, so the owner citation must be re-pointed to an open residual or retired with its Windows claim
@answer n=4 entity=established-decisions#4.1 v=no
why: the session's viola-agent-claude changes are the shim resolution and the relied set, and the screen gate's signatures, quiet period, maximum wait and verdict are not among them
@answer n=5 entity=established-decisions#5.1 v=yes mode=contradict
why: the session retired the three-OS CI matrix, so the claim that the send proof runs on the three CI OSes and the Windows-only LF owner are no longer true
@answer n=6 entity=established-decisions#6.1 v=yes mode=stale
why: the separate-lock rationale cites a Windows append-plus-lock failure (rust-lang/rust#54118), a Windows claim the session retired
@answer n=7 entity=established-decisions#7.1 v=yes mode=contradict
why: replace_private no longer retries a refused persist on Windows, so REPLACE_ATTEMPTS, retry_replace, replace_private_retrying and replace_private_with are gone and write_snapshot calls replace_private directly
@answer n=8 entity=established-decisions#8.1 v=no
why: place_private_new is not among the retired replace items, and its no-retry, no-error-variant and pinned-copy claims hold on the changed fs.rs
@answer n=9 entity=established-decisions#8.2 v=yes mode=contradict
why: the Windows arm and its test place_private_new_placed_copy_runs_at_once go with the removed Windows code, so the Windows run claim no longer holds
@answer n=10 entity=established-decisions#8.3 v=no
why: the write-handle-closed-before-placement ordering and its Linux measurement are unaffected by the removed Windows replace arms
@answer n=11 entity=established-decisions#9.1 v=yes mode=contradict
why: the Windows named pipe and the SQOS Identification client open are retired and the one-API-over-named-pipes-and-Unix-sockets rationale is gone, so the endpoint is a Unix socket only
@answer n=12 entity=established-decisions#10.1 v=no
why: Client::connect still carries no check, and the removed Windows SQOS connect arm was never a check
@answer n=13 entity=established-decisions#11.1 v=yes mode=contradict
why: the SERVER_PID_REPORTED flag is folded away and the Windows GetNamedPipeServerProcessId peer-pid path is retired with the named pipe
@answer n=14 entity=established-decisions#11.2 v=yes mode=contradict
why: PEER_EUID_REPORTED is folded so peer_euid_admitted reads observed == Some(own) alone, and the Windows DACL and pid controls the entry names are retired
@answer n=15 entity=established-decisions#12.1 v=no
why: the accept filter still calls peer_admitted on the peer euid and Server::serve still injects it, so the drop-before-worker claim holds
@answer n=16 entity=established-decisions#13.1 v=yes mode=contradict
why: the macOS no-peer-pid carve-out is retired and server_verdict loses its macOS Matches arm, so the recorded macOS gap no longer exists
@answer n=17 entity=established-decisions#14.1 v=yes mode=contradict
why: the off-Unix Ok(()) arm of check_socket_dir is gone and the Unix check body now runs unconditionally
@answer n=18 entity=established-decisions#15.1 v=no
why: the replay path for an unreadable or newer snapshot is outside the session's changed files and its claim is unchanged
@answer n=19 entity=established-decisions#16.1 v=no
why: the statusline serve path with no statusline_command still runs nothing and prints nothing, and the shell_argv change does not touch the replayed-snapshot case
@answer n=20 entity=established-decisions#17.1 v=no
why: the MCP frame tools still run strict::check_instance before live_endpoint, and the strict.rs change removes only the Windows clauses
@answer n=21 entity=established-decisions#18.1 v=no
why: the CappedLines stdin bound at MAX_FRAME is an MCP reader the session does not change
@answer n=22 entity=established-decisions#19.1 v=no
why: check_instance still runs on the home and the instance directory before live_endpoint on Unix, and the removed Windows clauses do not alter that call
@answer n=23 entity=established-decisions#19.2 v=no
why: the UnreachableCause mapping still reads a refused peer_admitted and a failed server_verified as stated, and the session changed only the server_verdict parameter list
@answer n=24 entity=established-decisions#20.1 v=no
why: the mcp-stdin oversize line and its session-end exit path are unaffected by the session
@answer n=25 entity=established-decisions#20.2 v=no
why: the exit 1 before initialize and the Role::Mcp exit mapping are unaffected by the session
