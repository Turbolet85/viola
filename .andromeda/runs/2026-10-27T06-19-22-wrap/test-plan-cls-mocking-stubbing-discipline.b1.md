# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the viola-mcp change adds a byte cap and no adapter timer, and its new serve and mcp_tools kill deadlines are real-time bounds on spawned children, not adapter timers
mocking-stubbing-discipline#2.1 · no · the session adds no outbound HTTP client, fake server or dependency, so the N/A row still holds
mocking-stubbing-discipline#2.2 · no · the session adds, bumps and locks no dependency (Cargo.lock untouched), so mock_instant and tokio test-util stay in no manifest and the Clock pattern is unchanged
mocking-stubbing-discipline#2.3 · no · the new serve and mcp_tools children are real processes and the root-test changes are assertions and a source census that spawn no double, so CARGO_BIN_EXE resolution is unchanged
mocking-stubbing-discipline#2.4 · no · the session adds stdio framing and a line cap only, with no socket, listener, squatter, impostor or named pipe, so the fake-server rows and their tests are unchanged
mocking-stubbing-discipline#2.5 · no · the new fs.rs unit case runs real targets (equal, longer, shorter, different, absent) and a real read-only dir on Unix, so no filesystem fault is faked and the real-tempfile claim holds
mocking-stubbing-discipline#2.6 · no · the fs.rs change is the bounded fallback compare, its private holds_exactly and one unit case, adding no env read, feature, double or seam, and the report names no change to place_private_new_with or its placed hook
