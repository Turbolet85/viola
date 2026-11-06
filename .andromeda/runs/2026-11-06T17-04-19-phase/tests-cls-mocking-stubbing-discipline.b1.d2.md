@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the clock seam, mock_instant and paused tokio clock rules govern timers and the wait deadline, which the chunk neither changes nor re-reads; DialogHold bounds are plain unit cases, not clock injection
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: no outbound HTTP exists and the chunk adds none
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the time-mocking library table names mock_instant and tokio test-util, which the chunk neither adds nor pins, and no manifest change of those crates is in scope
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the mockall automock on the viola-pty Pty seam and the real fake-agent stub executables bind, since the portable-pty move lands under that seam and item 2 requires the six-method seam and its mockall double to hold, with item 3's real openpty spawns of the fake agent
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=no
why: the process-fault rule (real kill, Frozen guard, kill_wrapper, lost-wrapper rows) is over tests the chunk neither names nor re-reads; the fake agent's process-group and SIGHUP behaviour is not a kill or fault double
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=no standing=S3
why: the test-server endpoint directory made through viola_state::fs::create_private_dir is the fresh-home convention S3 names, which every new test owes; the chunk adds no squatter, impostor or server test
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: item 22 is a chunk need naming the socket-directory verdict tests, and this entity's real-file Form rule and pure socket_dir_verdict uid injection govern those tests
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the placed hook of place_private_new_with in viola-state fs.rs is over a placement test the chunk does not name, and it adds no seam the chunk would have to keep out
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: item 18 holds that no seam may be added and names the peer_admitted mutant at the client seams, and this entity states that peer euid decisions take arguments with no env, flag or feature seam, the rule the chunk must keep
