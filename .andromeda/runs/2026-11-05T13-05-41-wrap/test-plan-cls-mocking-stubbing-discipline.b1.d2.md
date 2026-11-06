@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the clock seam cites src/run/send.rs, src/cmd/run.rs, src/run/links.rs and tests/cli_links.rs, none in the reached set, and no session change touches mock_instant or the Clock seam
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the row says there is no outbound HTTP, which the session does not change
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the session names neither mock_instant nor tokio test-util, so the time-mocking row holds
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes mode=stale
why: viola-pty lib.rs is reached by the openpty-only change, and the mockall seam on the Pty trait is not shown to hold on the changed file
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=no
why: tests/support/frozen.rs keeps kill -STOP ungated so the Unix SIGSTOP guard holds on the changed file, and the session's cross_exit_causes change is the refusal counts only
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes mode=contradict
why: windows-sys CreateNamedPipeW, the Windows test-owned control pipes and the Windows server-verification negatives are retired with windows-sys and tests/security_negatives_channel.rs
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes mode=contradict
why: windows_read_verdict, the windows-2025 read-grant refusal and the conpty seeded-home rule are retired, along with the icacls arms of tests/support/home.rs
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: place_private_new_with and its placed hook are not among the viola-state fs functions the session retires, and fs.rs is not in the reached set
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes mode=contradict
why: peer_euid_admitted drops its reported parameter and becomes observed == Some(own), so the three-argument signature this row states no longer holds
