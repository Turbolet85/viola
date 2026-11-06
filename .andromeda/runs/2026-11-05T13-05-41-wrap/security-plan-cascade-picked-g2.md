@account n=1 entity=authentication-authorization#1.3 v=proposed
why: the Windows named-pipe listener with its SDDL DACL and accept_remote(false) is retired and server/win.rs deleted, so the whole row goes
@account n=2 entity=authentication-authorization#1.4 v=proposed
why: the macOS $TMPDIR/viola/ socket-dir form is retired, and the row still names it along with an on-every-Unix-OS chmod and a macOS reason
@account n=3 entity=authentication-authorization#1.5 v=proposed
why: the off-Unix Ok(()) arm of check_socket_dir is removed, so its Unix body now runs unconditionally
@account n=4 entity=authentication-authorization#1.6 v=not_touched
why: the report keeps the Unix body of check_socket_dir (NotADirectory, Symlink, Unreadable unchanged) and adds no chmod, so never-chmods still holds on the reached strict.rs
@account n=5 entity=authentication-authorization#1.7 v=not_touched
why: the reached run.rs, client.rs, hook.rs and tools.rs lose only the SERVER_PID_REPORTED argument and Windows arms, and check_endpoint_dir stays the shared helper
@account n=6 entity=authentication-authorization#1.8 v=not_touched
why: run.rs loses only the conpty_sideload step, pin_companions, sideload_fallback and the BatchScriptChild refusal, and socket_dir_ready and its order stand
@account n=7 entity=authentication-authorization#1.10 v=not_touched
why: PEER_EUID_REPORTED folds to observed == Some(own), an unreadable euid is still refused, and accept_loop stays the one server
@account n=8 entity=authentication-authorization#1.11 v=not_touched
why: the report states peer_admitted and serve_admitting keep their roles in the reached server.rs, with no seam added
@account n=9 entity=authentication-authorization#1.12 v=proposed
why: the Windows SQOS open, the Windows identity check and the macOS no-peer-pid carve-out are retired, and SERVER_PID_REPORTED and peer_pid_reported are gone
@account n=10 entity=authentication-authorization#1.14 v=not_touched
why: the report names no change to the release-from-driver guard, ProtocolError::ReleaseFromDriver or the from param, and the exit set stands
@account n=11 entity=authentication-authorization#1.15 v=proposed
why: the Windows owner/DACL strict-modes arm, the ConPTY companions check, viola.exe and windows-sys are retired, and the row still states them and three-OS proofs as current
@account n=12 entity=authentication-authorization#1.16 v=proposed
why: board.rs lost its OS arms and the read rule stays, so the on-every-OS wording is stale on a Linux-only product
@account n=13 entity=authentication-authorization#1.17 v=proposed
why: the pinned viola(.exe) path form is stale now that the Windows exe is gone, and check_home_tree otherwise stands
