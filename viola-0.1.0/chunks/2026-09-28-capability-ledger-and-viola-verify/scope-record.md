# Scope record — 2026-09-28-capability-ledger-and-viola-verify

- `tests/support/verify.rs` · in-intent · serves step 9 · self
- `tests/support/mod.rs` · companion · serves tests/support/verify.rs · self
- `tests/run_cli.rs` · companion · serves src/run/version_gate.rs · self
- `tests/contract_diag_schema.rs` · companion · serves src/run/version_gate.rs · self
- `.config/nextest.toml` · in-intent · serves crates/viola-e2e/src/harness/run/mutants.rs · word: "A timing-only push to measure is fine; say what the phases show before you change the work." — the Viola overseer
- `crates/viola-e2e/src/harness/run/mutants.rs` · in-intent · serves tests/cli_verify.rs · word: "Option 2, folded into this chunk: the red is this chunk's (the load it added pushed the tests over). Add per-phase timing, measure on the macOS runner, remove the slow work, and record the out-of-list files in the scope record as in-intent." — the Viola overseer
