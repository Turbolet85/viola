# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the stale-heartbeat case's 5 s staleness and the carve-out for a clock-driven deadline in a unit case, plus the `wait` timeout_ms deadline's injected Clock, bind the new cases' time seam
mocking-stubbing-discipline#2.1 · no · the chunk adds no outbound HTTP and no HTTP fake server, so no unit of it reaches this row
mocking-stubbing-discipline#2.2 · yes · the stale-heartbeat unit case and its 5 s staleness drive heartbeat time through the injected Clock, the chunk's own clock-driven case
mocking-stubbing-discipline#2.3 · yes · the server-verification row in tests/security_negatives_server_identity.rs uses a held live process that an impostor's snapshot records, and the stale-heartbeat SIGSTOP case holds a real process
mocking-stubbing-discipline#2.4 · yes · the squatter bound through the server API and the server-identity impostor listener are the chunk's squatter and impostor cases
mocking-stubbing-discipline#2.5 · yes · the strict-modes, refused home tree and socket-directory cases must be refused on a widened home tree made on the real file, not a stub
mocking-stubbing-discipline#2.6 · no · place_private_new_with's placed hook sits in crates/viola-state/src/fs.rs, which the chunk does not reach, and no chunk test forces a placement window
