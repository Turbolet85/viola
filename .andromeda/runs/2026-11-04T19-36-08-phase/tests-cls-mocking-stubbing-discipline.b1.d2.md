@answer n=1 entity=mocking-stubbing-discipline#1.1 v=yes
why: the 1 s heartbeat and 5 s stale threshold, and the wrapper-side wait deadline, are the values the v1-40 stale-row witness and the 10 s mutants bound rest on (task items 10 and 11)
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: HTTP mocking is N/A; the chunk adds no outbound HTTP and no HTTP double
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=yes
why: the choice between an injected Clock or mock_instant and a real 5 s wait, and the rule that a paused tokio clock never reaches a spawned wrapper, govern how the stale-wrapper witness and the 5 s-inside-10 s test are timed (task items 10 and 11)
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the mockall Pty seam and the viola-fake-agent stub binary resolved through CARGO_BIN_EXE are the seam the openpty backend implements and the only child the Unix suite runs (task items 1 and 18)
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes
why: kill_wrapper for a lost wrapper and the Unix SIGSTOP under the Frozen guard are the kill and stale-holder realities the Unix Wrapper::kill, kill-and-revive and killed-wrapper child reading stand on (task items 3 and 9)
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes
why: the named-pipe and Unix-socket server negatives run on Linux and Windows only, with macOS comparing no pid or start time, and the endpoint directory is made through create_private_dir (task items 2 and 8)
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: the real-mode refusals of a loosened socket directory (0755 or a symlink to 0700) in tests/security_negatives_socket_dir.rs on Linux and macOS are the Unix socket directory 0700 and socket 0600 modes the chunk names (task items 2 and 13)
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the placed hook of place_private_new_with in viola-state fs.rs is a private placement race parameter; the chunk's scope names no forced-window placement and adds no seam there
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: peer_euid_admitted and the Admission argument take injected euids with no env, flag or feature seam, which is the peer-euid invariant the chunk's Unix tests must hold (task item 15)
