@answer n=1 entity=authentication-authorization#1.1 v=no
why: password hashing is N/A (no accounts or passwords), so the chunk adds no hashing code or test rule
@answer n=2 entity=authentication-authorization#1.2 v=no
why: a summary of the all-or-nothing capability model that states no code or test rule of its own, its enforcing rows (1.4, 1.10, 1.11) bind directly
@answer n=3 entity=authentication-authorization#1.3 v=yes
why: the protected user plus SYSTEM listener DACL, with accept_remote(false), FIRST_PIPE_INSTANCE and inheritable(false), is a Windows control the shared pty-seam or wrapper change must leave as it is
@answer n=4 entity=authentication-authorization#1.4 v=yes
why: the Unix primary control (per-user 0700 socket directory, chmod 0600 after bind instead of ListenerOptionsExt::mode, try_overwrite off, no abstract namespace) is item 3's requirement on both Unix OSes
@answer n=5 entity=authentication-authorization#1.5 v=yes
why: the directory verdict (check_socket_dir with its symlink and non-directory refusals) is the socket-directory check item 3 and item 5 rely on for both Unix OSes
@answer n=6 entity=authentication-authorization#1.6 v=yes
why: a refused socket directory must never be chmodded, a call the chunk's macOS socket path must avoid while it verifies the 0700 directory item 3 names
@answer n=7 entity=authentication-authorization#1.7 v=yes
why: check_endpoint_dir is the one helper shared by the bind and the client seams, which item 5 says check the socket directory before connecting
@answer n=8 entity=authentication-authorization#1.8 v=yes
why: socket_dir_ready runs before any snapshot, child or bind, and item 3 brings run's child resolution and socket bind onto both Unix OSes
@answer n=9 entity=authentication-authorization#1.9 v=yes
why: the Unix suite now kills wrappers and binds sockets on both OSes, so the rule that nothing removes a leftover socket binds the test-side cleanup the chunk runs there
@answer n=10 entity=authentication-authorization#1.10 v=yes
why: the Unix server-side euid check that drops a peer before any frame is read is item 4's requirement
@answer n=11 entity=authentication-authorization#1.11 v=yes
why: the accept-loop drop before any worker thread or frame, with the peer reading end-of-file and no line written, is how item 4's refusal must act
@answer n=12 entity=authentication-authorization#1.12 v=yes
why: the Unix identity check (peer euid, Linux peer pid with start time, macOS euid-only carve-out) and its pre-frame timing are items 5 and 6
@answer n=13 entity=authentication-authorization#1.13 v=yes
why: the client seam order (directory check, connect, peer euid decision, server verification) is the order item 5 requires on both Unix OSes
@answer n=14 entity=authentication-authorization#1.14 v=yes
why: the release-from-driver refusal, its channel-response log detail and its sanitised wire body are closed words and sanitised errors that item 9 keeps the same on every OS
@answer n=15 entity=authentication-authorization#1.15 v=yes
why: the Unix home strict-modes rules (0700 and 0600 through replace_private, group or other bits refused on home, instance directory and diagnostics/) are item 7's requirement, and the Windows owner and DACL half is item 17's control
@answer n=16 entity=authentication-authorization#1.16 v=yes
why: the board scan holds the home to check_read before any file opens, and item 13's viola list witness under the outer PTY runs through this check on Unix
@answer n=17 entity=authentication-authorization#1.17 v=yes
why: check_home_tree at run start, including the pinned viola at 0700 before the wrapper log opens, is what the chunk's Unix run path passes through under item 7
@answer n=18 entity=authentication-authorization#2.1 v=yes
why: the Windows remote-client rejection witness runs in the CI test (windows-2025) job the chunk re-measures beside Unix, and the accept_remote(false) it asserts must survive the shared pty-seam change
@answer n=19 entity=authentication-authorization#2.2 v=yes
why: the anonymous-open denial witnesses the protected user plus SYSTEM DACL that item 17 names as a control the shared change must leave as it is
@answer n=20 entity=authentication-authorization#3.1 v=yes
why: the observed peer pid (SO_PEERCRED on Linux, none on macOS) is the value item 5 compares on Linux and item 6's carve-out leaves unread on macOS
@answer n=21 entity=authentication-authorization#3.2 v=yes
why: peer_euid_admitted, refusing an unreadable euid on Unix, is the own-euid admission item 5 requires on both Unix OSes
@answer n=22 entity=authentication-authorization#3.3 v=yes
why: the ServerVerdict with its macOS Matches-without-comparison is the carve-out item 6 says no parity test may read as a comparison
@answer n=23 entity=authentication-authorization#3.4 v=yes
why: the instance-dead and unwrapped-name readings from without_endpoint are instance-unreachable details that item 9 keeps closed and identical on every OS
@answer n=24 entity=authentication-authorization#3.5 v=yes
why: link and unlink run the strict-modes check and the shared client connect with its socket-dir and peer euid checks, which items 5 and 7 require on Unix
@answer n=25 entity=authentication-authorization#3.6 v=yes
why: the CLI refusals with socket-dir-refused and server-verify-failed, writing no frame and exiting 21, are the refusal item 5 requires on both Unix OSes with the closed details item 9 names
