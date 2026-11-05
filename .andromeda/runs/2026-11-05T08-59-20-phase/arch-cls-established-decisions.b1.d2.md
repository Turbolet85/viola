@answer n=1 entity=established-decisions#1.1 v=yes
why: its CI enforcement runs the sole-root tokio ban and the sync-crate check on the Windows and macOS triples and jobs the removal drops from ci.yml and deny.toml, so the Linux CI must carry it
@answer n=2 entity=established-decisions#2.1 v=yes
why: the Windows ConPTY sideload, DLL-search and TerminateProcess arms leave with src/conpty.rs and crates/viola-pty/src/sideload.rs, while the portable-pty pin and the child-exit-on-handle rule it states stay for Linux
@answer n=3 entity=established-decisions#3.1 v=yes
why: its Windows byte-oracle half is the Windows arm of tests/chaos_feed_panic.rs that the removal drops, and its owner Windows-only live measurements is retired
@answer n=4 entity=established-decisions#4.1 v=no
why: the vt100 readiness gate is OS-neutral, sits in no reached file, and has no Windows arm or per-OS witness
@answer n=5 entity=established-decisions#5.1 v=yes
why: its proof on the three CI OSes is a witness that the removal of the Windows and macOS CI jobs changes
@answer n=6 entity=established-decisions#6.1 v=no
why: the ndjson lock and append rules are OS-neutral, Windows is named only as rationale, and the chunk adds no log line
@answer n=7 entity=established-decisions#7.1 v=yes
why: its ConPTY companion replace, the companions' held-open retry and the Windows error-5 retry rule over companion code that the removal of bin/<version>-<hash>/conpty/ drops
@answer n=8 entity=established-decisions#8.1 v=no
why: the no-clobber pinned-copy placement is OS-neutral and names no Windows arm or reached unit
@answer n=9 entity=established-decisions#8.2 v=yes
why: its Windows test place_private_new_placed_copy_runs_at_once is a cfg(windows) test the removal drops, while the Unix exec test it also states stays
@answer n=10 entity=established-decisions#8.3 v=no
why: the write-handle-closed rule is a Linux ETXTBSY measurement over code no reached file holds, with no Windows arm
@answer n=11 entity=established-decisions#9.1 v=yes
why: the Windows named-pipe client connect and its SQOS Identification open leave with the Windows arm, while the Linux socket rules it states stay
@answer n=12 entity=established-decisions#10.1 v=no
why: the rule that no check sits inside viola_channel::Client::connect is OS-neutral and the chunk does not move it
@answer n=13 entity=established-decisions#11.1 v=yes
why: the macOS no-peer-pid carve-out and the Windows GetNamedPipeServerProcessId arm leave, and SERVER_PID_REPORTED with them, while the Linux SO_PEERCRED pid stays
@answer n=14 entity=established-decisions#11.2 v=yes
why: PEER_EUID_REPORTED is a cfg!(unix) gate that the removal makes unconditional, and the peer-euid admission it defines is the Linux trust boundary
@answer n=15 entity=established-decisions#12.1 v=yes
why: the accept loop's peer-euid filter is a Unix gate that the change makes unconditional, and no env, flag or feature seam may force its refusal
@answer n=16 entity=established-decisions#13.1 v=yes
why: its macOS carve-out, where no peer pid is reported and server_verdict skips the comparison, is a macOS arm the removal drops
@answer n=17 entity=established-decisions#14.1 v=yes
why: the off-Unix Ok(()) arm of check_socket_dir is a cfg(not unix) arm the removal drops, while its Unix read rule stays
@answer n=18 entity=established-decisions#15.1 v=no
why: the replayed snapshot read as an absent one is OS-neutral and sits in no reached file
@answer n=19 entity=established-decisions#16.1 v=yes
why: the statusline shell-out that the replayed-snapshot rule governs gets an unconditional Unix gate, and its rule that a replayed snapshot runs nothing must hold on that path
@answer n=20 entity=established-decisions#17.1 v=no
why: the MCP tool surface and its CLI-only release rule are OS-neutral, with no Windows arm and no per-OS witness
@answer n=21 entity=established-decisions#18.1 v=no
why: the MCP stdin line cap is OS-neutral and sits in no reached file
@answer n=22 entity=established-decisions#19.1 v=no
why: the frame's check_instance call order and its strict-modes-failed refusal are OS-neutral and sit in no reached file
@answer n=23 entity=established-decisions#19.2 v=yes
why: the ServerVerify mapping of a refused peer_admitted(server_euid) is a peer decision whose Unix gate the change makes unconditional
@answer n=24 entity=established-decisions#20.1 v=yes
why: it states the exit codes 0 and 1 of the viola mcp stdin oversize path, which the chunk must keep unchanged on Linux
@answer n=25 entity=established-decisions#20.2 v=yes
why: it states exit 1 for viola mcp past MAX_FRAME before initialize, an exit code the chunk must keep unchanged on Linux
