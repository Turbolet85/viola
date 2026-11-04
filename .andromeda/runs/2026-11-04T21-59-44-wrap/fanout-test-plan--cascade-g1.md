@proposal n=1 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-fnukifof basis_doc=doc:2026-11-04-linux-and-macos-parity/report basis_fact=fct-wo5npqx2
section: ## 1. Test Scope Summary
change: append to the cli surface's **Notes**: "As landed (chunk 2026-11-04-linux-and-macos-parity): the dim `stale` row is witnessed under the outer PTY by `tests/cross_list_rows.rs` `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (Unix, `TERM=xterm-256color`, the wrapper frozen through `Frozen` and its beat back-dated 60 s): its escape sequences are exactly `[DIM, BOLD, RESET, DIM, RESET]`, `stale` shown."
sidecar: test-scope-summary cli surface Notes — the outer-PTY witness of the dim stale row landed (cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal, Unix).
rationale: the report's Symbols / APIs lands this test as v1-40's integration-tier witness, and its Outcome reads it passing on ubuntu-latest and macos-latest (ci#41918694243); the Notes name the outer-PTY driver for this rendering but no witness.
@proposal n=2 detector=D-cascade severity=warning file=test-strategy anchor=fct-hckuzazj basis_doc=doc:2026-11-04-linux-and-macos-parity/report basis_fact=fct-wo5npqx2
section: ## 2. Test Strategy
change: in **Sync (tokio-free) E2E suites**, the helper set reads `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs,frozen.rs}`, add "`frozen.rs` holds `Frozen`, the one freeze guard (`attach(pid)` and `Drop`; on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, `kill -CONT` on drop; on Windows a debugger attach, detached on drop)", and the no-copy rule reads "the reader, the wait, the guard, the freeze guard or the result struct".
sidecar: test-strategy directory pattern — tests/support/frozen.rs joins the shared helper set as the one home of the Frozen freeze guard.
rationale: the report's Symbols / APIs moves `Frozen` whole out of `tests/cli_instance_state.rs` into `tests/support/frozen.rs`, its sole home now, and `tests/support/mod.rs:9` declares `pub mod frozen;` (Crates / modules); the Retired bullet retires the two local twins.
@proposal n=3 detector=D-new-detail severity=warning file=test-strategy anchor=fct-hckuzazj fact=F6 basis_doc=doc:2026-11-04-linux-and-macos-parity/report basis_fact=fct-url4by23
section: ## 2. Test Strategy
change: add to the **Sync (tokio-free) E2E suites** item: "`tests/contract_os_gated_tests.rs` is a root `contract_` binary, auto-discovered with no `[[test]]` table; the E2E layer's regex selects it, so it runs under `run --e2e` in suite `nextest-e2e`."
sidecar: test-strategy directory pattern — the root contract_ binary contract_os_gated_tests, auto-discovered, in the E2E layer (run --e2e, nextest-e2e).
rationale: the report's Crates / modules names the new root test binary contract_os_gated_tests (auto-discovered, its contract_ prefix selected by the E2E regex, run --e2e), and Harness / gate surface places it in suite nextest-e2e.
@proposal n=4 detector=D-cascade severity=warning file=test-strategy anchor=fct-gldxhydz basis_doc=doc:2026-11-04-linux-and-macos-parity/report basis_fact=fct-oi4iu3a7
section: ## 2. Test Strategy
change: the row reads `| multi-os-compat | CI matrix + contract | §9 Matrix builds; §6 Contract suite (`tests/contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run) |`
sidecar: test-strategy trigger-layer map — multi-os-compat also held by the OS-gated census tests/contract_os_gated_tests.rs.
rationale: the report's Expected amendments name test-plan §2 for the census, and its Symbols / APIs gives the census over tests/ and crates/*/tests/ with faults both ways (unnamed / no longer stands); the row names only the CI matrix.
@remark
F1-F5 (e2e-test-strategy) and F7-F9 (ci-integration, incl. the kill line's 27 binaries) are not this group's; the Frozen file sites at integration-test-strategy fct-lzdm6e7w and mocking-stubbing-discipline fct-63urz52n belong to other groups; scripts/junit-os-sets.py carries no committed test by the report (an operator instrument, D-tests-coverage n/a).
