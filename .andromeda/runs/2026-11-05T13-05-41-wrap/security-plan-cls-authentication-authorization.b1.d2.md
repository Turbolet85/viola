@answer n=1 entity=authentication-authorization#1.1 v=no
why: no accounts or passwords exist and no session change names or reaches the password claim.
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the nine methods and the OS-user boundary are unchanged and the Unix IPC rows it cites still stand.
@answer n=3 entity=authentication-authorization#1.3 v=yes mode=contradict
why: the IPC access control (Windows) row is retired with its protected DACL, SDDL and first-pipe-instance controls (task item 6).
@answer n=4 entity=authentication-authorization#1.4 v=yes mode=stale
why: the macOS $TMPDIR/viola/ socket-dir form is retired and the macOS ListenerOptionsExt::mode reason is gone (task items 3 and 24).
@answer n=5 entity=authentication-authorization#1.5 v=yes mode=contradict
why: the off-Unix Ok(()) arm of check_socket_dir in the reached strict.rs is gone (task item 8).
@answer n=6 entity=authentication-authorization#1.6 v=yes mode=stale
why: check_socket_dir in the reached strict.rs was edited (task item 8) and the report does not show its never-chmod claim holds on the changed file.
@answer n=7 entity=authentication-authorization#1.7 v=yes mode=stale
why: check_endpoint_dir sits in the reached strict.rs and its caller src/cmd/run.rs is reached, and the report does not show the shared-helper claim holds on them.
@answer n=8 entity=authentication-authorization#1.8 v=yes mode=stale
why: socket_dir_ready, refuse_socket_dir and Bound sit in the reached src/cmd/run.rs, and the report does not show that unchanged.
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the cited cleanup.rs is not reached and the per-user socket file rule is Unix-only and unchanged by the session.
@answer n=10 entity=authentication-authorization#1.10 v=yes mode=stale
why: the accept-time euid drop lives in the reached viola-channel server.rs and the report does not show the claim holds on the changed file.
@answer n=11 entity=authentication-authorization#1.11 v=yes mode=stale
why: the accept_loop drop and its peer_admitted decision sit in the reached server.rs and the report does not show them unchanged.
@answer n=12 entity=authentication-authorization#1.12 v=yes mode=contradict
why: the Windows SQOS client open, the Windows identity check and the macOS no-pid carve-out are retired, and SERVER_PID_REPORTED and PEER_EUID_REPORTED are removed (task items 6 and 7).
@answer n=13 entity=authentication-authorization#1.13 v=no
why: the seam order sits in src/cmd/client.rs, src/cmd/hook.rs and viola-mcp tools.rs, none reached, and it holds unchanged on Linux.
@answer n=14 entity=authentication-authorization#1.14 v=yes mode=stale
why: the bin's channel dispatch that refuses release-from-driver sits in the reached bin entry src/main.rs and the report does not show that refusal holds on it.
@answer n=15 entity=authentication-authorization#1.15 v=yes mode=contradict
why: the Windows owner/DACL strict-modes arm, the Windows read rule and the %USERPROFILE% inheritance acceptance are retired (task items 8 and 25), and fs.rs replace_private is one attempt (task item 9).
@answer n=16 entity=authentication-authorization#1.16 v=yes mode=stale
why: scan's every-OS wording is an on-every-OS clause that is reworded to the Linux statement (task item 1).
@answer n=17 entity=authentication-authorization#1.17 v=yes mode=stale
why: the viola(.exe) path form of the home-tree check is retired (task item 10) and the reached src/cmd/run.rs and strict.rs hold that check.
@answer n=18 entity=authentication-authorization#2.1 v=yes mode=contradict
why: its witness tests/security_negatives_channel.rs is deleted with the Windows pipe (task item 29).
@answer n=19 entity=authentication-authorization#2.2 v=yes mode=contradict
why: its witness tests/security_negatives_channel.rs is deleted and the protected DACL it asserts is retired (task items 6 and 29).
@answer n=20 entity=authentication-authorization#3.1 v=yes mode=contradict
why: the Windows GetNamedPipeServerProcessId arm and the macOS none case are gone and SERVER_PID_REPORTED is removed (task items 5 and 7).
@answer n=21 entity=authentication-authorization#3.2 v=yes mode=contradict
why: PEER_EUID_REPORTED is removed and peer_euid_admitted now reads observed == Some(own) (task item 7).
@answer n=22 entity=authentication-authorization#3.3 v=yes mode=contradict
why: server_verdict and verify_server lose the peer_pid_reported input and the no-pid macOS Matches branch is retired (task item 7).
@answer n=23 entity=authentication-authorization#3.4 v=yes mode=stale
why: without_endpoint sits in the reached liveness.rs, where server_verdict and verify_server lost peer_pid_reported, and the report does not show this claim holds on it.
@answer n=24 entity=authentication-authorization#3.5 v=yes mode=contradict
why: the macOS server-check carve-out that link and unlink name is retired (task item 7).
@answer n=25 entity=authentication-authorization#3.6 v=no
why: the CLI refusal documents and their exit 21 details (server-verify-failed, socket-dir-refused) are untouched and src/cmd/client.rs and hints.rs are not reached.
