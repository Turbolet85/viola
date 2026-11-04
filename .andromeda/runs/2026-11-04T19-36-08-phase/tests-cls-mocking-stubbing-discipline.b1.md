@answer n=1 entity=mocking-stubbing-discipline#1.1 v=yes
why: the chunk's stale-wrapper witness and wait/kill tests run a real wrapper, and the clock rule says which clock reaches a spawned wrapper (a paused tokio clock never does).
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the chunk adds no outbound HTTP and the entity states only that none is used.
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the chunk's staleness witness and 5 s bound run on the real clock through a spawned wrapper and add no MockClock or mock_instant double.
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the openpty seam and the Module to PTY boundary run over real fake-agent spawns, and the Pty trait is the only mockall seam this chunk's tests would touch.
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes
why: Wrapper::kill, the killed-wrapper child reading, and the stale-wrapper witness rest on real kills and a real SIGSTOP holder, which this entity names as the process-fault pattern.
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes
why: the server-identity negatives run on Linux and Windows only under the macOS carve-out, and the test endpoint directory is made through create_private_dir, which this entity requires.
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: the Unix socket directory 0700 and socket 0600 modes and the refusals in tests/security_negatives_socket_dir.rs are real-file modes this entity sets as the fault pattern.
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the forced-window placed hook in viola-state's fs.rs is a private function parameter, and the chunk's scope names no placement case that would call it.
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: any Unix test the chunk adds must hold the peer-euid decision over injected euids with no env, flag or feature seam, and this entity states the injection pattern that does so.
