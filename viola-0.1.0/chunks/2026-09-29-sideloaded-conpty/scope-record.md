# Scope record — 2026-09-29-sideloaded-conpty

- `tests/support/outer_pty.rs` · in-intent · serves tests/conpty_sideload.rs · self
- `schemas/diag-line.v1.json` · companion · serves src/run/mod.rs · self
- `tests/run_cli.rs` · companion · serves src/run/mod.rs · self
- `tests/support/home.rs` · in-intent · serves tests/run_cli.rs · self
- `tests/support/piped.rs` · in-intent · serves step 8 · self
- `tests/support/mod.rs` · companion · serves tests/support/piped.rs · self
- `tests/cli_fake_agent.rs` · in-intent · serves tests/support/piped.rs · self
- `tests/cli_instance_state.rs` · in-intent · serves tests/support/piped.rs · self
- `tests/cli_program_resolution.rs` · in-intent · serves tests/support/piped.rs · self
- `tests/cli_version_gate.rs` · in-intent · serves tests/support/piped.rs · self
- `tests/contract_diag_schema.rs` · in-intent · serves tests/support/piped.rs · self
- `tests/tui_env_strip.rs` · in-intent · serves tests/support/home.rs · self
- `tests/tui_passthrough.rs` · in-intent · serves tests/support/home.rs · self
