# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the stale-heartbeat case's 5 s staleness and the `wait` deadline bind through the injected `Clock` seam, and the unit case that blocks on a clock-driven product deadline
mocking-stubbing-discipline#2.1 · no · the row is HTTP mocking N/A, and the chunk adds no outbound HTTP and no HTTP double
mocking-stubbing-discipline#2.2 · yes · the stale-heartbeat unit case's clock-driven 5 s staleness goes through the injected Clock with `MockClock::advance`, and its manifest note that mock_instant and tokio test-util are in no manifest yet applies if the chunk adds them
mocking-stubbing-discipline#2.3 · yes · the server-identity impostor's held live process is a real process, never a double, and the squatter and impostor cases' stub binaries and Drop-guard kill bind in the chunk's root tests
mocking-stubbing-discipline#2.4 · yes · the squatter binds through the server API and the server-verification impostor on the recorded endpoint binds in tests/security_negatives_server_identity.rs, Linux and Windows only
mocking-stubbing-discipline#2.5 · yes · the refused home-tree cases of `run`, `verify` and Path 1 are made on the real widened home tree and refused by the product's own check, never stubbed
mocking-stubbing-discipline#2.6 · no · the chunk reaches no forced placement window, its squatter case binds through the server API, and crates/viola-state is not reached
