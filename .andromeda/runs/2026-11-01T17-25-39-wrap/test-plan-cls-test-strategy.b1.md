# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · no · the new socket-dir span cases run on macOS, but the row's exception names only the server-check cases, and refused_inside stays macOS-gated (only the Seen recorder moved out)
test-strategy#1.2 · no · the tempfile home and the real interprocess endpoint stand; the session changes only the endpoint directory's mode, not the home
test-strategy#1.3 · no · the seven critical paths and four expansion scenarios are unchanged; the Path 1 start order is internal and the harness-booted, PTY and rmcp claims hold
test-strategy#1.4 · no · hook_stdin is the stdin parser's fuzz target, untouched; the session adds socket-dir and euid checks to the hook's connect path, and the cargo-fuzz targets are not among the changed files
test-strategy#1.5 · no · the five hyperfine hook rows and their max gate stand; no perf value moves and the row states no reading
test-strategy#1.6 · no · the listed fault sources (set_len truncation, snapshot corruption, SIGSTOP freeze, no-EOF child, mockall seam) are unchanged; the force-killed-wrapper harness case is not a chaos-suite fault
test-strategy#1.7 · no · the mutation gate's rule and verdict arithmetic stand; the client-seam survivor is a named gap, not a change to any verdict, and the remove-the-guard rule's process-start scope does not reach client seams
test-strategy#1.8 · no · the fake-agent fixtures and their jsonschema checks are unchanged; schemas/diag-line.v1.json is not among the schemas this row names
test-strategy#2.1 · no · the directory pattern, the home.rs and start.rs helper lists and the one-reader rule stand; first_life_on sits in home.rs by design, and the new binary fits the security_negatives_<topic> pattern
test-strategy#3.1 · yes · add-to · the session adds tests/security_negatives_socket_dir.rs, a Unix socket-directory security negative for the IPC endpoint, which this row's list of root security negatives does not name
test-strategy#3.2 · no · paste injection is untouched; the viola-core sixth cause adds a unit case, but the paste proptest and plan locations stand
test-strategy#3.3 · no · the hook stdin trigger's rows are untouched; the new hook fail-open causes are socket-directory and euid refusals, not stdin parsing
test-strategy#3.4 · no · the MCP trigger's E2E scenario and Security sweep locations stand; the new MCP socket-dir case is an integration case in crates/viola-mcp/tests/handlers.rs, not a change to this row's layers
test-strategy#3.5 · no · the CLI and env trigger's unit and integration plan locations stand; the new not-disableable case adds a test, but this row names no test file
test-strategy#3.6 · yes · add-to · the session adds tests/security_negatives_socket_dir.rs (socket-directory negatives, filesystem mode), and the home-read refusal now covers viola list as well as run, beside this row's strict-modes negatives
test-strategy#3.7 · no · the child spawn's §5 and E2 locations stand; a refused socket directory spawns no child, a pre-spawn guard this row does not describe
test-strategy#3.8 · no · no CI step, sync-crate or dependency version changes; Cargo.toml gains only a [[test]] table, which the supply-chain gates do not read
test-strategy#3.9 · no · no proptest or parser surface changes; the property-test row's §4 and §9 fuzz locations stand
test-strategy#3.10 · no · the chaos suite's §6 location stands; the session adds no chaos scenario, its force-kill harness case lives in viola-e2e
test-strategy#3.11 · no · the performance-budget row names §10 only; the perf re-read moves no value and the hook gate is unchanged
test-strategy#3.12 · no · the E2E layer and the §6 scenario locations stand; the new MCP parity witness sits in the cross-surface section this row points to, and the row lists no causes
test-strategy#3.13 · no · the multi-OS CI matrix is unchanged; no CI step changed, and the new binary's cfg(unix) gate lives in the test file, not the matrix
test-strategy#3.14 · no · multi-version compatibility rests on viola-agent-claude and the fixtures, both untouched
test-strategy#3.15 · no · the sandbox contract trigger rests on fake-agent fixtures and §7, both untouched; the contract_error_bodies addition is error-body, not sandbox
test-strategy#3.16 · yes · add-to · tests/cli_error_bodies.rs and tests/contract_error_bodies.rs gain the socket-directory error documents, so this row's enumeration of error bodies and writers must extend
