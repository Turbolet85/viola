# obs-plan · picked accounting

## Picked
obs-scope-summary#27.3 · not touched · its sha-256 mismatch exit-1 cause still holds: the report adds no `detail` code and leaves `src/cmd/run.rs` unchanged, and the lost placement's mismatch is the same `PinError::HashMismatch`, carried at #38.1's hash-mismatch telemetry
obs-scope-summary#38.1 · proposed · its pinned-exe hash-mismatch refusal telemetry gains the lost-placement path (`place_absent`'s `HashMismatch`), with no new line
span-trace-coverage#4.2 · proposed · its `run.pin_copy` outcome carries F1: a lost placement re-hashes the standing copy, `ok` when equal and `pinned-hash-mismatch` when it differs
