# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · yes · the chunk's module-to-module tests (channel peer check with injected euids, the transport-free Tools handle in viola-mcp, TokioClient cases) are direct crate-API calls with rstest fixtures
integration-test-strategy#1.2 · yes · the strict-modes filesystem negatives and the .lock start arbiter, left true by the chunk, run on real files in a tempfile home; the concurrent-append item does not reach this chunk
integration-test-strategy#1.3 · no · the external boundary is the fake claude agent on a test PATH; the chunk's refusals run before any child or agents read and it changes no claude-agents behaviour
integration-test-strategy#1.4 · yes · the squatter bound first through the server API, and the Unix socket dir 0700 and file 0600 set by chmod after the bind and read back with metadata, are Needs of this chunk
integration-test-strategy#1.5 · no · ConPTY, openpty, resize and key behaviour are untouched; no chunk need or scope names PTY behaviour
integration-test-strategy#1.6 · yes · hook fail-open tests against a live wrapper with VIOLA_NAME and VIOLA_DIR bind the new hook refusal (exit 0, no body, no stderr) for a refused directory and an euid mismatch
integration-test-strategy#2.1 · no · the forced-window placement witness covers the ETXTBSY guard of fs.rs place_private_new, which the chunk does not touch
integration-test-strategy#3.0 · no · standing S2 · each test seeds its own fresh tempfile home under target/e2e-home, made through viola_state::fs::create_private_dir, with no state shared between tests
integration-test-strategy#4.1 · yes · the MCP impostor and widened-instance cases, the MCP instance-unreachable mapping (server-verify-failed, strict-modes-failed), the hook and CLI server-identity impostor rows and the macOS pid gap all bind the chunk
integration-test-strategy#4.2 · yes · the strict-modes refusal table (run, revive, verify, mcp exit 1; hook 0 with no output; MCP instance-unreachable; CLI 21 for other verbs), the send and link check_instance rows and the home-tree checks bind the new refusals
integration-test-strategy#5.1 · yes · the CLI --json document per exit code, the viola list board cases in tests/cli_list.rs, the link verb refusals in tests/cli_links.rs and the no-control-disables table in tests/cli_controls_not_disableable.rs bind the new refusals
