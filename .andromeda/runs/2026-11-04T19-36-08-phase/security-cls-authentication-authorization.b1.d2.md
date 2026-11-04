@answer n=1 entity=authentication-authorization#1.1 v=no
why: password hashing is N/A (no accounts or passwords), so the chunk's code holds no credential
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the all-or-nothing capability model is a description the chunk's code does not act on; its enforcement rows bind on their own
@answer n=3 entity=authentication-authorization#1.3 v=yes
why: the Windows protected user plus SYSTEM listener DACL and inheritable(false) are Windows controls task item 17 says the shared pty seam change must leave as they are
@answer n=4 entity=authentication-authorization#1.4 v=yes
why: the Unix primary control, the 0700 per-user socket directory and endpoint layout, is what the Unix suite exercises on both OSes (task item 3)
@answer n=5 entity=authentication-authorization#1.5 v=yes
why: check_socket_dir is the directory verdict run before bind and before any client connect on Linux and macOS (task item 3)
@answer n=6 entity=authentication-authorization#1.6 v=yes
why: a refused socket directory must stay as found, so the new Unix socket-dir tests must not expect a chmod (task item 3)
@answer n=7 entity=authentication-authorization#1.7 v=yes
why: check_endpoint_dir is the one helper the bind and the client seams share, and the Unix client seams run on both OSes (task items 3 and 5)
@answer n=8 entity=authentication-authorization#1.8 v=yes
why: socket_dir_ready runs before the child and Server::bind on the Unix start path that the suite now runs on both OSes (task item 3)
@answer n=9 entity=authentication-authorization#1.9 v=yes
why: a killed wrapper's leftover socket file must stay, so Wrapper::kill, the kill-and-revive path and the test harness cleanup must not remove it (task items 3 and 18)
@answer n=10 entity=authentication-authorization#1.10 v=yes
why: the server-side euid check on every accept is the Unix peer check task item 4 names
@answer n=11 entity=authentication-authorization#1.11 v=yes
why: the drop happens on the accept thread before any frame, which the Unix peer-check test must observe as end-of-file (task item 4)
@answer n=12 entity=authentication-authorization#1.12 v=yes
why: the Unix euid identity check, the Windows SQOS open and the macOS no-pid carve-out bind the client seams the shared change reaches (task items 5, 6 and 17)
@answer n=13 entity=authentication-authorization#1.13 v=yes
why: the client seam order (directory check, connect, euid decision, server verify) is what Unix client verification runs on both OSes (task item 5)
@answer n=14 entity=authentication-authorization#1.14 v=yes
why: the Unix suite runs the driver release tests on both OSes (task item 3), so the release-from-driver refusal and its channel-response line are re-asserted by the chunk
@answer n=15 entity=authentication-authorization#1.15 v=yes
why: the Unix home strict-modes (0700 dirs, 0600 files via replace_private, the read rule) is task item 7, applied before the first read in every process
@answer n=16 entity=authentication-authorization#1.16 v=yes
why: scan's home read rule runs before the board opens, and the chunk's viola list witness under the outer PTY runs this scan on the Unix suite (task item 13)
@answer n=17 entity=authentication-authorization#1.17 v=yes
why: check_home_tree on run start checks the home, ledger, bin and plugin trees before the log and child open on Linux and macOS (task item 7)
@answer n=18 entity=authentication-authorization#2.1 v=yes
why: the Windows remote-client rejection witness is a Windows control task item 17 keeps, and CI's test (windows-2025) job the chunk re-measures runs it
@answer n=19 entity=authentication-authorization#2.2 v=yes
why: the Windows anonymous-open denial witness is a Windows DACL control task item 17 keeps, run by the CI test (windows-2025) job the chunk re-measures
@answer n=20 entity=authentication-authorization#3.1 v=yes
why: the observed server pid is compared on Linux via SO_PEERCRED and absent on macOS, the carve-out task items 5 and 6 name
@answer n=21 entity=authentication-authorization#3.2 v=yes
why: peer euid admission is the Unix client check task item 5 names, refusing an unreadable euid as server-verify-failed
@answer n=22 entity=authentication-authorization#3.3 v=yes
why: the Matches, Mismatch and Unverifiable verdict decides whether a frame is written, with no pid comparison on macOS (task items 5 and 6)
@answer n=23 entity=authentication-authorization#3.4 v=yes
why: the no-endpoint reading yields the instance-dead or unwrapped-name details that task item 9 keeps closed on every OS
@answer n=24 entity=authentication-authorization#3.5 v=yes
why: link and unlink run the client connect seam on Unix, with its directory check, euid decision and server check (task item 5)
@answer n=25 entity=authentication-authorization#3.6 v=yes
why: the CLI refusal, exit 21 with server-verify-failed or socket-dir-refused and no frame written, is what task items 5 and 11 name
