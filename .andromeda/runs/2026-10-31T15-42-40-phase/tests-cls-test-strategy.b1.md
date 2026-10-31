# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the exit-code table (0/1/2/10–14/20/21) and the root bin's exit-code mapping unit cases, which the scope names as bounds; the server check's macOS-compiled-out cases bind the OS bounds
test-strategy#1.2 · yes · the matrix rows in security_negatives_server_identity.rs, security_negatives_strict_modes.rs, run_cli.rs, cli_verify.rs and cli_revive.rs are cases of these integration suites, each with a real tempfile home and assert_cmd against CARGO_BIN_EXE_viola
test-strategy#1.3 · yes · Path 1's refusals as table cases, Path 3's endpoint-lost killed wrapper read at CLI and MCP, and the harness-booted fake-agent run they use
test-strategy#1.4 · no · the property suite and cargo-fuzz targets are not reached: the chunk changes no parser
test-strategy#1.5 · yes · the 5 s heartbeat staleness that the stale-heartbeat case uses is named by the scope, and classify fed the beat's age is how that verdict is checked
test-strategy#1.6 · yes · the SIGSTOP heartbeat freeze, the no-EOF child and the endpoint vanishing during wait are the chaos faults the stale-heartbeat, script-child and endpoint-lost cases use
test-strategy#1.7 · yes · the remove-the-guard rule binds the new refusal cases (duplicate run, squatted endpoint, tampered pinned exe, refused home tree): each neutralised guard needs a bounded witness through Running and WITHIN, which tests/support/fake.rs re-reads; the on-demand mutation gate does not bind
test-strategy#1.8 · no · the fake-agent fixture contract and its insta decision bodies are not reached: no hook payload, fixture or recorded set moves and nothing is re-recorded
test-strategy#2.1 · yes · the scope names the cross_ prefix and layer filterset of the new root file tests/cross_exit_causes.rs, and the shared tests/support helpers it draws on (events.rs error-body check, cli.rs guard); rstest is root-only, not viola-e2e
test-strategy#3.1 · yes · the server-verification cause in security_negatives_server_identity.rs (off macOS), the vanished-server read in viola-channel client.rs, the raw link/unlink frame cases in link.rs and unlink.rs, and the hook fail-open impostor rows
test-strategy#3.2 · no · paste validation (validate_paste_text, Path 2) is not reached: the scope excludes the paste flow and its matcher, and the viola-core reach is for is_code_word, not the paste proptest
test-strategy#3.3 · yes · the hook boundary's assert_cmd hook cases hold the hook contract the scope names (exit 0, no body, no stderr, hook-decision its only record), and refuse_untrusted, which the hook refusal runs through, is edited
test-strategy#3.4 · yes · MCP surface: today's instance-unreachable payload and refusal text, isError payloads holding codes only, the MCP frame tools' strict-modes refusal, and the cross-surface parity read through the rmcp client in crates/viola-e2e/tests/mcp_tools.rs
test-strategy#3.5 · yes · CLI --json documents and exit codes of the reached verbs, and the root bin's exit-code mapping cases, since the chunk's codes are CLI surface values
test-strategy#3.6 · yes · strict-modes refusals of the six frame verbs and run/verify's refused home tree, each refused before any log with no line written, in security_negatives_strict_modes.rs and cli_verify.rs
test-strategy#3.7 · yes · the script-child case (.cmd/.bat refused with exit 1, driven on the Windows runner) and internal-error's claude absent from a test-scoped PATH, both child-spawn boundaries
test-strategy#3.8 · no · supply-chain CI gates are not reached: the chunk adds no CI job, edits no workflow, and no dependency change is named in scope
test-strategy#3.9 · no · the property suite and cargo-fuzz targets are not reached: the chunk changes no parser
test-strategy#3.10 · yes · the chaos suite holds the SIGSTOP stale-heartbeat, endpoint-vanishing-during-wait and script-child cases the scope names
test-strategy#3.11 · no · no performance budget is named or moved: the 5 s staleness is a liveness bound, not a perf budget, and the hook deadlines are untouched
test-strategy#3.12 · yes · cross-surface parity: the MCP structured refusal and detail equal the CLI --json refusal and detail once causes are told apart
test-strategy#3.13 · yes · the Windows-only script-child case runs on the Windows runner and the server-check cases are Linux and Windows only; the scope's OS-only checks and the rule on where that OS's code is verified bind through the matrix
test-strategy#3.14 · no · multi-version replay and per-CLI-version fixtures are not reached: no ledger row or CLI version changes
test-strategy#3.15 · no · the sandbox contract (fake agent against recorded viola verify fixtures) is not reached; the error-body census in contract_error_bodies.rs is carried by test-strategy#3.16
test-strategy#3.16 · yes · codes-only error bodies on CLI --json, MCP and channel error data, the three surface witnesses, and the census in tests/contract_error_bodies.rs, which every new code and writer must stay named in
