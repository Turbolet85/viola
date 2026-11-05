@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the time-mocking rules (Clock seam, mock_instant, budget expiry, the since stamp) rule over clock reads the chunk does not touch, and its not_needed list names time mocking
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: HTTP mocking is N/A, and the chunk adds no outbound HTTP
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the time-mocking library row rules over the clock seam, which the chunk does not change
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the chunk makes the fake agent's per-command process group and SIGHUP-ignoring exit unconditional on Unix and runs its root and e2e children on that fake agent, so the real-stub-executable, no-double and CARGO_BIN_EXE rule binds
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes
why: the chunk removes the Windows arm of the Frozen guard (tests/support/frozen.rs) and its scope names the chaos and exit-cause .cmd cases, so the real-wrapper, real-kill and no-double rule over those helpers binds
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes
why: the chunk re-points tests/security_negatives_server_identity.rs from Linux and Windows to Linux only, and the Linux impostor and create_private_dir endpoint rule governs the server negatives that stay
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: the chunk re-points tests/security_negatives_socket_dir.rs from Linux and macOS to Linux only, and the real-file loosened-directory and socket_dir_verdict rule governs the Linux cases that stay
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the placed hook of place_private_new_with is a parameter of a unit the chunk does not edit, and no chunk test uses it
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: the chunk's viola-channel listener changes and its no-seam rule (needs item 20) bind the peer-euid decision and its argument-injected Admission design, which must gain no env, flag or feature seam
