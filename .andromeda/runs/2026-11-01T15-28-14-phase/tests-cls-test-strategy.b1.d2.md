# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the exit-code table (0/1/2/10–14/20/21) and the unit cases of the peer euid check with injected euids, plus the macOS server_pid match-with-no-comparison case
test-strategy#1.2 · yes · the wrapper-bind and client-connect directory cases and the peer checks sit in the Wrapper channel's crate-level suite on a real endpoint
test-strategy#1.3 · yes · Path 1's squatted-endpoint exit 1, Path 3's killed-wrapper exit 21 and the harness cleanup measurement run as harness-booted viola run E2E paths
test-strategy#1.4 · no · the chunk adds no parser surface (Not needed: parser properties and cargo-fuzz targets)
test-strategy#1.5 · yes · the hook time bound over the added checks under CONNECT_DEADLINE, the hook_fails_open_silently_within_the_spine_bound case and the hyperfine max rows
test-strategy#1.6 · yes · the chaos endpoint-vanishing case for a killed wrapper, a process fault left true; torn, no-EOF, heartbeat and vt100 faults stay out (Not needed)
test-strategy#1.7 · yes · the per-chunk remove-the-guard rule: each new directory and euid control needs a bounded witness through tests/support/cli.rs Running and WITHIN; the mutation gate itself stays out
test-strategy#1.8 · no · the chunk records no new CLI behaviour and changes no hook stdin, recorded fixture or fake-agent byte comparison (Not needed)
test-strategy#2.1 · no · standing S1 · Integration binaries named by the <surface>_<topic>.rs prefix and test functions in <subject>_<condition>_<expected> snake_case
test-strategy#3.1 · yes · the IPC vector: the directory and peer euid checks, the server-identity impostors, TestWrapper in handlers.rs, the macOS-off server-identity negatives and the hook impostor rows
test-strategy#3.2 · no · the chunk changes no typing, confirmation or validate_paste_text behaviour (Not needed: Path 2)
test-strategy#3.3 · yes · the hook boundary's fail-open refusal for a refused directory and an euid mismatch (exit 0, no body, no stderr) and its security-sweep rows
test-strategy#3.4 · yes · the MCP frame tools: the client-side euid check, the instance-unreachable refusal and the handlers.rs impostor case
test-strategy#3.5 · yes · the rule that no env, flag or config disables one of the new controls, and the CLI and env refusals
test-strategy#3.6 · yes · Unix strict modes at the viola-state unit layer and the filesystem negatives, the home-tree check and the board's one home rule
test-strategy#3.7 · yes · the loosened or replaced socket-directory cases re-point XDG_RUNTIME_DIR, TMPDIR and the /tmp path for their own child spawn
test-strategy#3.8 · no · the chunk names no manifest, dependency or CI gate change
test-strategy#3.9 · no · the chunk adds no parser surface (Not needed: parser properties and cargo-fuzz targets)
test-strategy#3.10 · yes · the chaos suite's endpoint-vanishing case for a killed wrapper, left true beside the .lock start arbiter
test-strategy#3.11 · yes · the hook deadline budget over the added checks: the hyperfine max rows and hook_fails_open_silently_within_the_spine_bound
test-strategy#3.12 · yes · cross-surface parity for each new exit-21 cause, the MCP isError body against the CLI --json document
test-strategy#3.13 · yes · the macos-latest matrix row carrying the macOS carve-out, the macOS exit-cause case and the macOS server-verification case
test-strategy#3.14 · no · the chunk touches no agent-claude code and no versioned fixture
test-strategy#3.15 · no · the chunk records no new CLI behaviour in the contract suite's fixtures or drift checks (Not needed)
test-strategy#3.16 · yes · error sanitization of the new refusals: a code and hint with no absolute path, read by contract_error_bodies.rs and the surface witnesses
