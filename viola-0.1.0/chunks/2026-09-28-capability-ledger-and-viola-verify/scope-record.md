# Scope record — 2026-09-28-capability-ledger-and-viola-verify

- `tests/support/verify.rs` · in-intent · serves step 9 · self
- `tests/support/mod.rs` · companion · serves tests/support/verify.rs · self
- `tests/run_cli.rs` · companion · serves src/run/version_gate.rs · self
- `tests/contract_diag_schema.rs` · companion · serves src/run/version_gate.rs · self
- `src/panic_frames.rs` · in-intent · serves src/main.rs · word: "yes, do not symbolise in the hook panic path. Keep the frame addresses plus each module path and base, enough for an offline resolver against the pinned copy; the wrap amends obs-plan section 7 with the reason (measured 351 of 403 ms)." — the Viola overseer
- `Cargo.lock` · mechanical · serves Cargo.toml · self
- `crates/viola-e2e/src/harness/run/mutants.rs` · in-intent · serves tests/cli_verify.rs · word: "Option 2, folded into this chunk: the red is this chunk's (the load it added pushed the tests over). Add per-phase timing, measure on the macOS runner, remove the slow work, and record the out-of-list files in the scope record as in-intent." — the Viola overseer
