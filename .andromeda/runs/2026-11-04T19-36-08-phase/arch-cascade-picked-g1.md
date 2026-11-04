@account n=1 entity=stack-and-technologies#1.1 v=not_in_scope
why: the toolchain pin and the msrv floor; the chunk re-reads the test, perf and lint jobs, never the msrv job, and moves no toolchain
@account n=2 entity=stack-and-technologies#1.2 v=carried
why: the openpty wrapper's code on the run and pty path must stay on std threads, Tokio confined to mcp
@account n=3 entity=stack-and-technologies#1.3 v=not_in_scope
why: the clap parser and its subcommand list; the chunk adds no verb, flag or help output
@account n=4 entity=stack-and-technologies#1.4 v=carried
why: the PTY layer row names portable-pty =0.8.1's openpty backend on Unix and libc's host tty raw mode, the very seam the chunk brings to parity
@account n=5 entity=stack-and-technologies#1.5 v=not_in_scope
why: the vt100 readiness model is a component the suite only drives, and the scope names none of its behaviour
@account n=6 entity=stack-and-technologies#1.6 v=carried
why: the Unix socket endpoint's uid, chmod 0600 and socket-dir check are behaviour both Unix OSes share that the parity suite reads on ubuntu and macos
@account n=7 entity=stack-and-technologies#1.7 v=not_in_scope
why: the no-database store choice; the one-schema rule reads the formats' fields, never the store kind
@account n=8 entity=stack-and-technologies#1.8 v=carried
why: it states how diag-line.v1.json versions the process-log lines, a reached file and the one-schema obligation's own subject
@account n=9 entity=stack-and-technologies#1.9 v=carried
why: the strict-modes check through libc on Unix and windows-sys on Windows is an OS-differing behaviour the scope names and the parity tests read per OS
@account n=10 entity=stack-and-technologies#1.11 v=not_in_scope
why: serde's key order and recursion limit are a component the chunk only uses; the scope names fields and closed words, not their order
@account n=11 entity=stack-and-technologies#1.12 v=carried
why: the logging stack writes the one-line JSON process logs whose schema the chunk re-asserts identical on every OS
@account n=12 entity=stack-and-technologies#1.14 v=not_in_scope
why: chrono's timestamp rendering is a component the chunk only uses, and the scope names no timestamp value
@account n=13 entity=stack-and-technologies#1.15 v=not_in_scope
why: the per-crate error types are a re-check every change owes; the scope names no error type
@account n=14 entity=stack-and-technologies#1.16 v=not_in_scope
why: no message broker; the chunk adds no IPC transport and the endpoint is carried through the Wrapper IPC rows
@account n=15 entity=stack-and-technologies#1.17 v=carried
why: pid plus process start time is how the kill-and-revive case reads the child gone on Unix and how the v1-40 witness's stale wrapper is judged
@account n=16 entity=stack-and-technologies#1.22 v=carried
why: the three-OS matrix is where the chunk re-reads the test, perf and lint jobs green on ubuntu-latest and macos-latest beside windows-2025
@account n=17 entity=stack-and-technologies#1.23 v=carried
why: ripgrep, jq, hyperfine and jsonschema are the tools of the G1 to G4 gates and the perf and lint jobs the chunk re-reads green on both Unix OSes
@account n=18 entity=stack-and-technologies#2.1 v=carried
why: peer_creds gives a pid on Linux and none on macOS and the euid on Unix, the macOS carve-out and peer-euid check the scope holds
@account n=19 entity=stack-and-technologies#3.1 v=carried
why: the server check's observed start time is the comparison macOS keeps out of, which no test may read as a comparison there
