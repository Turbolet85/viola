# Classified — tests · e2e-test-strategy.b3

## Answers
e2e-test-strategy#17.1 · yes · the endpoint-vanishes-during-`wait` exit 21 case, the SIGSTOP stale-heartbeat case refusing a new `run` with exit 1, and the `.cmd`/`.bat` batch-script-child exit 1 case bind; the torn-append, the vt100 panic `send` exit 13 (Path 2, not re-opened) and the forward-compat bullets do not
e2e-test-strategy#18.1 · no · the proptest and fuzz parsers (hook normalise, statusline reader, agents JSON, vt100 feed, paste text, prompt normalisation, ndjson framing) are parsers the chunk does not edit, the scope bars parser changes, and the viola-channel edit is the vanished-server read, not the framing unit
e2e-test-strategy#19.1 · yes · `contract_error_bodies.rs`'s tables and guards bind (every new code's error body and writer must be named there), and `contract_bounded_readers.rs`'s `checked_exe` EXCEPTIONS row binds because `run.rs` edits `checked_exe`; the hook-stream byte compare, annotations, claude fixture schema and sha2 vectors do not
