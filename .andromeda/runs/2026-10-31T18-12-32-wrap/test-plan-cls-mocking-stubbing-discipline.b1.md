# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · tests/cli_links.rs (home of is_clock_time) and src/cmd/run.rs (a budget-gate Utc::now read site) are reached whole, and the report does not show their clock claims holding
mocking-stubbing-discipline#2.1 · no · the session adds no HTTP dependency and no outbound HTTP; Cargo.toml's change is one [[test]] entry with required-features
mocking-stubbing-discipline#2.2 · no · Cargo.toml's change adds only a [[test]] entry, so mock_instant and tokio test-util stay out of every manifest, and viola-core gains UnreachableCause, not a Clock change
mocking-stubbing-discipline#2.3 · no · the report says no double was added and the filesystem stays real; the Held child is a real process left as it was, and liveness.rs gains a plain function, not a seam trait
mocking-stubbing-discipline#2.4 · no · the server-identity negatives and handlers.rs's MCP case still use a server that is not the recorded wrapper and still write no frame and no line; the change adds a code, not a server
mocking-stubbing-discipline#2.5 · no · the report says no double was added and the filesystem stays real; a widened tree is still refused by the product's own check, now reading strict-modes-failed
mocking-stubbing-discipline#2.6 · no · fs.rs, where place_private_new_with lives, is not reached, and no seam, feature or double was added
