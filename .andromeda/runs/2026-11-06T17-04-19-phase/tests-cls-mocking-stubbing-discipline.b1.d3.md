@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: time mocking (Clock, mock_instant, paused tokio clock, Utc::now reads) is not this chunk's work per not_needed item 10; no clock seam is touched.
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: HTTP mocking is N/A; the chunk adds no outbound HTTP and no HTTP fake, so the no-HTTP rule is a re-check every change owes.
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the time-mocking library table is the time discipline that not_needed item 10 excludes from this chunk.
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the mockall 0.15.0 double on the viola-pty Pty trait is the seam needs item 2 names, with consumers tested against a mockall double that the new portable-pty pin must keep.
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=no
why: process faults (Frozen SIGSTOP, kill_wrapper, kill_by_pid) are not named by the scope; item 16 names fake-agent process-group and SIGHUP behaviour, not a real wrapper kill.
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=no
why: fake-server fixtures (squatter, impostor, Server::bind listeners, endpoint dirs) are not added or touched by the chunk; the socket-directory verdict tests of needs item 22 are carried by #2.6.
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: needs item 22 names the socket-directory verdict tests, and this row's real-file refusals (0755 or symlink to 0700 in the Form cases) and the pure socket_dir_verdict uid injection are those tests.
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the placed hook of place_private_new_with in viola-state/src/fs.rs is not in the scope, and the chunk edits no placement code.
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: needs item 18 says no seam may be added at the peer-admission decisions, and this row states peer_euid_admitted and serve_admitting take arguments with no env, flag or feature seam, which the chunk's code must keep.
