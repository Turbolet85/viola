@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the session touched no clock seam, send, run, links or cli_links file this row cites
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the HTTP mocking row says no outbound HTTP and the session changed nothing on that
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the time mocking row names the Clock seam and mock_instant, which the session did not touch
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=no
why: the process mocking row cites the fake agent holder and handlers.rs, none of which the session changed
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes mode=stale
why: the Frozen guard's home moved from tests/cli_instance_state.rs to tests/support/frozen.rs, so this row's named file is stale
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=no
why: the fake server row cites hook_fail_open and security_negatives_server_identity, neither of which the session reached
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=no
why: the filesystem faults row cites security_negatives_socket_dir and the section 7 helpers, which the session did not change
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the forced-window hook lives in crates/viola-state/src/fs.rs, which the session did not reach
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=no
why: the peer euid injection lives in crates/viola-channel/src/server.rs, which the session did not reach
