@account n=1 entity=quality-gates-coverage-targets#2.0 v=proposed
why: it places outcomes.json under the Windows host scratch, cites HOST_SCRATCH and scratch.rs as live, has the audit dispatch windows-mutants.yml and the two real-cargo-mutants tests compiled out on macOS, all retired by the report
@account n=2 entity=quality-gates-coverage-targets#3.1 v=proposed
why: it calls the socket-directory check and the peer euid decision a real check on Unix only, while the report removes the off-Unix arm of check_socket_dir and folds the euid decision
