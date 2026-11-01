# Classified — tests · e2e-test-strategy.b3

## Answers
e2e-test-strategy#17.1 · yes · the endpoint-vanishing case (`a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`, exit 21 with detail `endpoint-lost`, killed through `support::start::kill_wrapper`) is named left true by the task; the torn-append, no-EOF, stale-heartbeat, vt100-panic, forward-compat, snapshot-replay and Windows `.cmd` bullets are faults or paths the chunk does not change
e2e-test-strategy#18.1 · no · the proptest and cargo-fuzz parser properties (paste, ndjson framing, hook stdin, statusline, agents JSON, vt100 feed) are not moved; the chunk adds no parser surface and changes no paste behaviour
e2e-test-strategy#19.1 · yes · `tests/contract_error_bodies.rs`'s tables and census of writers bind the new closed codes, hints and directory and euid refusals, each needing a named writer and a witness; the fixture-drift, content-hash and bounded-reader bullets are not moved by the chunk
