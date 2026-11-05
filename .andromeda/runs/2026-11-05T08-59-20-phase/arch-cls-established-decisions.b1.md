@answer n=1 entity=established-decisions#1.1 v=yes
why: the KEYSTONE's CI enforcement (sole-root cargo deny tokio ban over the Windows, macOS and Linux triples, cargo check of the sync crates on three OSes) must stay green on the shrunk Linux CI (task item 5)
@answer n=2 entity=established-decisions#2.1 v=yes
why: the Windows arms of the PTY decision (ConPTY sideload, System32 DLL restriction, host_stdin ReadConsoleW, Windows child-exit and cwd rationale) leave with the removed code while the portable-pty pin and handle-wait rule stay for Linux (task item 1)
@answer n=3 entity=established-decisions#3.1 v=yes
why: the OPEN Windows reading of the wide-character oracle in tests/chaos_feed_panic.rs names a Windows arm the chunk removes and an owner entry that retires (task item 21)
@answer n=4 entity=established-decisions#4.1 v=no
why: the screen-model readiness gate names no Windows or macOS arm and no exit code, and the chunk does not change the gate
@answer n=5 entity=established-decisions#5.1 v=yes
why: it states the send confirmation rules are proved on the three CI OSes, which the Windows and macOS job removal changes, and it names exit 13 (task item 7)
@answer n=6 entity=established-decisions#6.1 v=no
why: the ndjson state store lock and append rules name no OS arm the chunk removes, and the Windows lock mention is rationale only (task item 14 not needed)
@answer n=7 entity=established-decisions#7.1 v=yes
why: the Windows retry loop (HOST_IS_WINDOWS, error 5 and error 32 retries) and the ConPTY companion placement are Windows arms and companions the chunk removes (task item 3)
@answer n=8 entity=established-decisions#8.1 v=no
why: place_private_new names no Windows or macOS arm and the chunk does not touch the no-clobber placement
@answer n=9 entity=established-decisions#8.2 v=yes
why: it names the Windows test place_private_new_placed_copy_runs_at_once, a cfg(windows) test the removal takes with its arm, while the Unix test and the hook stay
@answer n=10 entity=established-decisions#8.3 v=no
why: the write-handle close before placement is a Linux ETXTBSY measurement with no OS arm the chunk removes
@answer n=11 entity=established-decisions#9.1 v=yes
why: the Windows named pipe and the SQOS Identification open leave with the removed code while the Linux socket rules stay (task item 2)
@answer n=12 entity=established-decisions#10.1 v=no
why: it says no check sits inside Client::connect and names no OS arm; the Linux socket-directory callers are untouched
@answer n=13 entity=established-decisions#11.1 v=yes
why: server_pid is Linux SO_PEERCRED and Windows GetNamedPipeServerProcessId with the macOS none-case SERVER_PID_REPORTED, whose macOS carve-out loses its code (task items 8 and 22)
@answer n=14 entity=established-decisions#11.2 v=yes
why: PEER_EUID_REPORTED is cfg!(unix), a gate the chunk makes unconditional while the peer-euid decision must not gain an env, flag or feature seam (task item 9)
@answer n=15 entity=established-decisions#12.1 v=yes
why: the accept-loop admission filter is the Unix peer-euid gate the chunk makes unconditional, with no env, flag or feature seam (task item 9)
@answer n=16 entity=established-decisions#13.1 v=yes
why: the macOS recorded gap where server_verdict compares no pid is the carve-out whose code the chunk removes (task item 8)
@answer n=17 entity=established-decisions#14.1 v=yes
why: check_socket_dir returns Ok(()) off Unix, a non-Unix arm the chunk removes while the Unix socket-directory check stays (task items 2 and 9)
@answer n=18 entity=established-decisions#15.1 v=no
why: the replay of an unreadable or newer snapshot names no OS arm and the chunk does not change the replay path
@answer n=19 entity=established-decisions#16.1 v=yes
why: the chunk makes the statusline /bin/sh shell-out unconditional, and this entity requires that a replayed snapshot runs nothing, a call the new code must avoid (task item 11)
@answer n=20 entity=established-decisions#17.1 v=no
why: the MCP decision, its tools and result shapes are not needed by this chunk (task item 6) and name no OS arm
@answer n=21 entity=established-decisions#18.1 v=no
why: the MCP stdin line cap names no OS arm and the chunk does not touch the stdin reader
@answer n=22 entity=established-decisions#19.1 v=no
why: the MCP frame tools call check_instance before live_endpoint and map its refusal, a call and mapping the removal leaves unchanged; the strict-modes rules belong to their own decision
@answer n=23 entity=established-decisions#19.2 v=yes
why: the MCP refusal mapping names peer_admitted over server_euid and a failed server_verified, the peer-admission decision whose Unix gates the chunk makes unconditional (task item 9)
@answer n=24 entity=established-decisions#20.1 v=yes
why: it names exit 0 and exit 1 for the viola mcp oversize cut, which the exit-code contract must keep unchanged (task item 7)
@answer n=25 entity=established-decisions#20.2 v=yes
why: it names exit 1 for viola mcp on an oversize line before initialize, a typed exit code the removal must keep unchanged (task item 7)
