@answer n=1 entity=authentication-authorization#1.1 v=no
why: no accounts or passwords exist and the session touches no credential claim
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the capability claim names only the OS check and the IPC rows, which stand on Linux, and carries no OS-scoped or count wording
@answer n=3 entity=authentication-authorization#1.3 v=yes mode=contradict
why: the session retires the Windows named-pipe endpoint, its protected SDDL DACL and the whole Windows IPC access control this row states
@answer n=4 entity=authentication-authorization#1.4 v=yes mode=contradict
why: the session retires the macOS TMPDIR socket directory, so the $TMPDIR/viola/ macOS form and the macOS mode 0o600 reason no longer hold, and the every-Unix-OS clause is reworded to Linux
@answer n=5 entity=authentication-authorization#1.5 v=yes mode=contradict
why: the session removes the off-Unix Ok arm of check_socket_dir, so the claim that off Unix it returns Ok no longer holds
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the Unix socket-directory check still never chmods, and the session removes only the Windows and off-Unix arms of strict.rs
@answer n=7 entity=authentication-authorization#1.7 v=no
why: check_endpoint_dir stays the one helper shared by the bind and the three client seams on Linux, and the session changes none of that sharing
@answer n=8 entity=authentication-authorization#1.8 v=no
why: socket_dir_ready keeps its check, create-when-absent and refusal order on Linux, and the session names no part of it
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the leftover socket file in the per-user directory is unchanged and the session touches no cleanup or unconnectable code
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the Unix accept-time euid drop is unchanged on Linux, and the session removes only the Windows listener and not this check
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the drop before any frame by the euid decision still holds on Linux, and the session's euid-rule change leaves Linux admission unchanged
@answer n=12 entity=authentication-authorization#1.12 v=yes mode=contradict
why: the session removes the Windows SQOS client open and the macOS no-peer-pid carve-out this rule states, with SERVER_PID_REPORTED and PEER_EUID_REPORTED
@answer n=13 entity=authentication-authorization#1.13 v=no
why: the seam order of directory check, connect, euid decision and server check still holds on Linux, and this row names no Windows or macOS step
@answer n=14 entity=authentication-authorization#1.14 v=no
why: the release-from-driver rule is independent of OS and the session touches none of its params check or log line
@answer n=15 entity=authentication-authorization#1.15 v=yes mode=contradict
why: the session removes the Windows strict-modes owner and DACL arm, the Windows read rule, the ConPTY companion pin check and its StrictModes refusal this row states
@answer n=16 entity=authentication-authorization#1.16 v=yes mode=stale
why: the scan and read-rule claims read on every OS, and the session makes every OS Linux alone, so the every-OS clause is reworded to the Linux statement
@answer n=17 entity=authentication-authorization#1.17 v=yes mode=stale
why: the pinned viola(.exe) path form is now the Linux viola alone, the Windows .exe form is retired
@answer n=18 entity=authentication-authorization#2.1 v=yes mode=contradict
why: the Windows remote-client witness test in tests/security_negatives_channel.rs is deleted with the Windows listener
@answer n=19 entity=authentication-authorization#2.2 v=yes mode=contradict
why: the Windows DACL-effect witness in tests/security_negatives_channel.rs is deleted with the Windows listener
@answer n=20 entity=authentication-authorization#3.1 v=yes mode=contradict
why: the session removes the Windows GetNamedPipeServerProcessId arm, the macOS none case and SERVER_PID_REPORTED that this row states
@answer n=21 entity=authentication-authorization#3.2 v=yes mode=contradict
why: the session removes PEER_EUID_REPORTED and the Windows no-euid case, so peer_euid_admitted becomes observed equal to own alone
@answer n=22 entity=authentication-authorization#3.3 v=yes mode=contradict
why: the session removes peer_pid_reported from server_verdict and verify_server, so the macOS Matches-with-no-comparison arm this verdict states is gone
@answer n=23 entity=authentication-authorization#3.4 v=no
why: without_endpoint reads the instance directory alone by symlink_metadata, and the session's liveness change touches only server_verdict and verify_server
@answer n=24 entity=authentication-authorization#3.5 v=yes mode=contradict
why: the session removes the macOS no-pid carve-out and the Windows server check that link and unlink state
@answer n=25 entity=authentication-authorization#3.6 v=no
why: the CLI refusal on mismatch and its socket-dir and strict-modes detail words and exit 21 stay on Linux, and the session changes none of them
