# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · liveness.rs (heartbeat on the injected Clock) and src/cmd/wait.rs are reached whole; live_endpoint now returns a LiveEndpoint, and no Clock-driven case there is shown to hold
mocking-stubbing-discipline#2.1 · no · the session adds no HTTP client, server or dependency (the pins are unchanged), and its impostor is a local endpoint
mocking-stubbing-discipline#2.2 · yes · stale · liveness.rs is reached whole (live_endpoint now returns a LiveEndpoint, verify_server is new); the MockClock heartbeat case behind the advance example is not shown to hold there, and the manifest pins are unchanged
mocking-stubbing-discipline#2.3 · yes · contradict · the handler tests in crates/viola-mcp/tests/handlers.rs now run the fake agent's holder mode outside root tests/, so the stub is no longer resolved in root tests/ only
mocking-stubbing-discipline#2.4 · no · the new impostor is a real endpoint, not a double (the report's §8 line), so the squatter-listener row holds, and the listener-admission negatives are not reached
