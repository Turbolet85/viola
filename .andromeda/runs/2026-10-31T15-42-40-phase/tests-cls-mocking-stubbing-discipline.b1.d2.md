# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the stale-heartbeat case's 5 s staleness and the `wait` timeout_ms deadline read through the injected Clock, which the chunk's table case drives
mocking-stubbing-discipline#2.1 · no · the chunk adds no outbound HTTP and no HTTP fake server; no case of the table or the freight reaches one
mocking-stubbing-discipline#2.2 · yes · the stale-heartbeat case advances an injected MockClock past the 5 s staleness, so the clock-driven unit case binds to this pattern
mocking-stubbing-discipline#2.3 · yes · the held live-process cases (already-live, server-verification impostor, endpoint-lost, stale heartbeat) need a real held process whose pid the snapshot records, not a double
mocking-stubbing-discipline#2.4 · yes · the squatted-endpoint case and the server-verification impostor are bound through the interprocess server API, which this entity names
mocking-stubbing-discipline#2.5 · yes · the refused home-tree cases of `run` and `verify` and the strict-modes refusals are made on a real widened tree and refused by the product's own check
mocking-stubbing-discipline#2.6 · no · the chunk adds no test of `place_private_new_with` in viola-state and no seam; its tests build real homes and never drive that placement hook
