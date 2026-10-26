# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their gate script are in no reached file, and Untouched holds the ConPTY companion re-hash out of reach
dependency-security#2.1 · no · Untouched says no dependency was added or bumped (tokio =1.53.3, the sync-crate tokio bans, deny.toml and channel-tokio-free.sh unchanged), and the Tokio client file gains only TokioClient::server_pid, so the claim still holds on the changed file
dependency-security#3.1 · no · fuzz/, its lockfile and the root Cargo.toml are in no reached file, and no dependency changed, so the separate workspace and its exemption stand
dependency-security#4.1 · no · the four ConPTY pins live in src/conpty.rs, which is not a reached file, and Untouched holds the companion re-hash out of reach
dependency-security#5.1 · no · windows-mutants.yml is not a reached file, and the only new file the task names is a test, so the root package's per-file split stands
dependency-security#6.1 · no · the G2 exemption lives in scripts/g2-zero-panics.sh and keys on the panic line of src/cmd/hook/seam.rs, neither a reached file, so the hook's new server check moves neither
dependency-security#7.1 · no · the action SHAs live in workflow files the change does not reach, and no CI or toolchain pin moved
