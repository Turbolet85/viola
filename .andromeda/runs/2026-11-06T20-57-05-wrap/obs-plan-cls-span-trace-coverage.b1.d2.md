@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the CLIENT span names are ours, the session renames none, and the portable-pty bump leaves pty.spawn in place
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names are unchanged, and the session retires only Windows/macOS remnants and a stderr warning
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the INTERNAL span names are not renamed by the 0.9.0 pin move, and pty_backend stays openpty
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the session moves portable-pty, nix and the serial family, not clap 4.6.7 or any cli span or process-log line
@answer n=5 entity=span-trace-coverage#2.2 v=yes mode=stale
why: the session moves the portable-pty pin from =0.8.1 to =0.9.0, which this row still cites as 0.8.1
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the hook events, hook-decision details and capture arms are unchanged, and the session's remnant removal names no hook log line
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: interprocess 2.4.4 is not among the Cargo.lock moves (portable-pty, nix, serial family, pin-utils), and the channel spans and fields are unchanged
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the viola-channel accept path is not in the change, and the peer_admitted drop before any span stands
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: rmcp 3.4.1 is not among the Cargo.lock moves, and viola-mcp tools.rs is not in the change
@answer n=10 entity=span-trace-coverage#3.1 v=no
why: the connect sequence and the Linux-only peer pid and start-time comparison hold on the changed client.rs, and the session only retires Windows/macOS remnants this row already excludes
@answer n=11 entity=span-trace-coverage#3.2 v=no
why: the server_euid read and the peer_admitted refusal hold on the changed client.rs and hook.rs, and the session adds or moves no euid rule
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the named refusal tests and the cmd::client::refused_inside helper are unrenamed on the changed send, client and hook files, and the new contract test is a different one in tests/contract_fake_agent_drift.rs
@answer n=13 entity=span-trace-coverage#3.4 v=no
why: the two named socket-directory cases and the Seen recorder are unrenamed on the changed client.rs and hook.rs, and no socket-directory check moves
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: client::refuse_untrusted and its exit 21 instance pair hold on the changed client.rs, and the session retires Windows remnants, not the instance refusal
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: the check on the recorded endpoint's parent and its socket-dir-refused process-exit hold, the session moves no socket-directory rule, and this row carries no PROVISIONAL mark
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the plain-ASCII socket-directory refusal pair and its hint line are unchanged, and the session retires the Git Bash warning, a different stderr line
@answer n=17 entity=span-trace-coverage#3.8 v=no
why: the hook's snapshot read, check_endpoint_dir, connect_by and verify_server order holds on the changed hook.rs, and the session names no hook change
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook-decision socket-dir-refused line and its dialog-path order hold on the changed hook.rs, and the session moves no such rule
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: the -32602 closed-word detail lives in viola-channel, which the change does not reach, and its wording is unchanged
@answer n=20 entity=span-trace-coverage#3.11 v=no
why: check_instance, check_endpoint_dir and server_verified in viola-mcp are not in the change, and their order and codes-only lines stand
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: reply_of and of_reply screen replies through viola_core and viola_channel, which the session does not change
@answer n=22 entity=span-trace-coverage#4.1 v=no
why: the run start span chain, the snapshot steps and the version-probe spawn hold, and the session moves no start step even though version_gate.rs is reached
@answer n=23 entity=span-trace-coverage#4.2 v=no
why: pty.spawn's pty_backend stays openpty on the 0.9.0 re-read, and the run.version_gate, collision and pin attributes are unchanged
@answer n=24 entity=span-trace-coverage#4.3 v=no
why: the process-start claude-child field names and the home-tree check names are unchanged, and pty_backend keeps its string type
@answer n=25 entity=span-trace-coverage#4.4 v=no
why: the handle-wait exit source is unchanged on the 0.9.0 re-read of the PTY seam, so no claude-child process-exit value moves
