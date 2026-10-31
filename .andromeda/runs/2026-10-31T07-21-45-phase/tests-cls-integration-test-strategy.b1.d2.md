# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · yes · the MCP witness calls viola_mcp::Tools directly against TestWrapper, a module-to-module call with shared rstest fixtures
integration-test-strategy#1.2 · no · the chunk adds no parallel appender and no shared diagnostics file, and torn-append chaos is outside its scope
integration-test-strategy#1.3 · yes · the child-pipe-drain witness runs viola-fake-agent as claude on a test-scoped PATH in oversize mode, and the .cmd and npm-shim cases use the .cmd shim fixture
integration-test-strategy#1.4 · yes · the channel witnesses use a real endpoint bound by viola-channel in a temp home with jsonschema-validated frames, which the -32600, -32602 and -32603 shapes need; the squatter, socket-mode and DACL readbacks stay with their route entry
integration-test-strategy#1.5 · no · PTY spawn, resize, no-EOF and .cmd refusal checks are viola-pty's own, the chunk drives no PTY spawn itself, and no-EOF child chaos is excluded
integration-test-strategy#1.6 · yes · the status-line and config.json thresholds witnesses drive viola hook statusline with fixture stdin under VIOLA_NAME and VIOLA_DIR and assert its hook trace and budget.json
integration-test-strategy#2.1 · no · the ETXTBSY placed-copy witness covers viola-state's place_private_new guard, which the chunk does not touch
integration-test-strategy#3.0 · no · standing S2 · a fresh home per test under target/e2e-home, its started wrappers held in a Drop guard, nothing shared between tests
integration-test-strategy#4.1 · yes · the -32600 close on an oversize frame, the higher params.v -32602 rejection, the schema-validated error shapes, the MCP instance-unreachable impostor and widened-instance cases, and the unserved-method channel-response line with no method key bind
integration-test-strategy#4.2 · yes · the CLI --json instance-unreachable and strict-modes-failed documents and the MCP instance-unreachable body with detail null bind; the event round-trip and snapshot clauses are viola-state's
integration-test-strategy#5.1 · yes · root tests/cli_*.rs --json documents per exit code, the cli_slash_args --json usage errors, the cli_links channel-request and channel-response pair with method and no names on the reshaped line, and the catch-site fault's --json form bind; the human verify, trycmd and help literals do not
