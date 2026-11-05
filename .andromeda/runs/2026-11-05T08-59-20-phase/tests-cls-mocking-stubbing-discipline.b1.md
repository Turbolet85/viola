@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: time mocking through the Clock seam is marked not needed by the task (item 14) and the chunk reads or drives no clock
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: HTTP mocking is N/A in this repository and the chunk adds no outbound HTTP
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the time-mocking row is the Clock and start_paused surface the task marks not needed (item 14), which the chunk does not touch
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes
why: the chunk edits viola-pty (the windows-sys TerminateProcess fallback and its cfg(windows) unit cases, task item 9) and the viola-fake-agent Unix arms (item 12), and this row fixes that external processes are real stub executables and never mockall doubles
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=no
why: the row's fault helpers (Frozen guard, kill_wrapper, kill_by_pid) are named by no scope item, and the fake agent's unconditional SIGHUP exit stays a real signal with no stand-in added
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes
why: tests/security_negatives_server_identity.rs is re-gated from Linux and Windows to Linux (task item 11), and its Linux impostor and create_private_dir endpoint rules keep binding the server-verification negatives it keeps
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes
why: tests/security_negatives_socket_dir.rs is re-gated from Linux and macOS to Linux (task item 11), and its real 0755 and symlink-to-0700 forms and the pure socket_dir_verdict rule bind it
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: the placed hook of place_private_new_with in viola-state fs.rs is named by no scope item, and the chunk adds no forced-window test or seam
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=yes
why: task item 20 bans any env, flag or feature seam forcing a refusing peer-euid decision, and the chunk re-gates the macOS peer-pid carve-out (item 11), so the dependency-injection rule over peer_euid_admitted and serve_admitting binds
