# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · the change reached src/cmd/run.rs, which holds one of the budget gate's direct Utc::now() reads, and the viola-core and viola-mcp crates that hold its Clock and start_paused claims, reached whole and not shown to hold
mocking-stubbing-discipline#2.1 · no · the report leaves the HTTP entry untouched, adding no double, fake server or failpoint, and no reached file names this row
mocking-stubbing-discipline#2.2 · yes · stale · the change reached Cargo.toml, where the two new [[test]] tables sit, so the claim that mock_instant and tokio test-util are in no manifest yet can have moved and is not shown to hold
mocking-stubbing-discipline#2.3 · yes · stale · the change reached crates/viola-mcp/tests/handlers.rs by adding the free-text refusal test, and that file holds the Held child and its Drop guard
mocking-stubbing-discipline#2.4 · yes · stale · the change reached crates/viola-channel/src/server.rs, where viola_channel::Server lives, and crates/viola-mcp/tests/handlers.rs, which holds the MCP impostor case
mocking-stubbing-discipline#2.5 · no · the test-home helpers, seed_conpty, stamped_home and tests/support/home.rs are outside the change and unchanged, and no reached file names this row
mocking-stubbing-discipline#2.6 · no · the session changed no crate-state path, so the placed hook in crates/viola-state/src/fs.rs is outside the change, and the report adds no double, fake server or failpoint
