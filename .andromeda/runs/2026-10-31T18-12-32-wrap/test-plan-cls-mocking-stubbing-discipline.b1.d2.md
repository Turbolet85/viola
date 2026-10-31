# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · src/cmd/run.rs (budget gate Utc::now call sites) and tests/cli_links.rs (is_clock_time) are reached whole and the report does not show their clock claims hold
mocking-stubbing-discipline#2.1 · no · the HTTP row names no reached file and no outbound HTTP was added
mocking-stubbing-discipline#2.2 · yes · stale · Cargo.toml is reached and "in no manifest yet" for mock_instant and tokio test-util is not shown to hold after its change
mocking-stubbing-discipline#2.3 · yes · stale · crates/viola-mcp/tests/handlers.rs (its Held child and Drop guard) and crates/viola-state/src/liveness.rs (the liveness trait) are reached whole and their process-mock claims are not shown to hold
mocking-stubbing-discipline#2.4 · yes · stale · tests/security_negatives_server_identity.rs and crates/viola-mcp/tests/handlers.rs, which hold the impostor cases, are reached whole and the impostor claim is not shown to hold
mocking-stubbing-discipline#2.5 · no · the report's Untouched line says no double was added and the filesystem stays real, so the real-home and real-modes claim holds
mocking-stubbing-discipline#2.6 · no · crates/viola-state/src/fs.rs, which holds place_private_new_with, is not in the reached files and the report adds no test seam
