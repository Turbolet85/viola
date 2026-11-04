@answer n=1 entity=mocking-stubbing-discipline#1.1 v=yes
why: the v1-40 witness's staleness rests on the 1 s heartbeat and 5 s threshold, and a spawned wrapper's wait and clock run on real time, never a paused or mocked clock
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the chunk adds no outbound HTTP and no HTTP fake, so the row's "none" stands
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the MockClock and mock_instant pattern is a unit-tier double the chunk does not add; its witness and 5 s wait run real wrappers on real time, as #1.1 already carries
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the openpty seam and the Module to PTY boundary run over real spawns of the fake agent, with mockall only on the Pty seam trait and no other double
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes
why: Wrapper::kill, the kill-and-revive case and the child-ends-with-its-wrapper reading on Unix are real kills and real SIGSTOP holders, with no double standing in
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes
why: the server-identity impostor runs on Linux and Windows only under the macOS carve-out, and the Unix socket endpoint directory is made by create_private_dir for the Unix socket cases
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: the Unix socket directory 0700 and socket 0600 modes and the loosened-directory refusals in tests/security_negatives_socket_dir.rs on Linux and macOS are made on the real file, never stubbed
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: place_private_new_with's placed hook is a fs.rs test parameter for a placement window, and the chunk names no placement race test and reaches no fs.rs
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: task item 15 binds the peer-euid invariant over any Unix test the chunk adds, so the decision is held only over injected euids with no env, flag or feature seam
