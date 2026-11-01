### error-handling#1.26 · values · errors · ## Error Handling · lines 33–33
- `resolve_program` returns `Result<PathBuf, AgentError>` in `viola_agent_claude` and in the root wrapper `src/run/mod.rs`, its refusals `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, each a fixed message; `src/cmd/revive.rs`'s private `Refusal` is another type.
### error-handling#1.27 · values · errors · ## Error Handling · lines 34–34
- `tests/contract_error_bodies.rs`, carrying `andromeda:walks-tree`, enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions); `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.
### error-handling#1.28 · values · errors · ## Error Handling · lines 35–35
- `tests/cross_exit_causes.rs`, a root `cross_` binary with `required-features = ["fake-agent"]`, holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.
### error-handling#1.29 · values · errors · ## Error Handling · lines 36–36
- `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, with the classes `absolute-path` · `caused-by` · `serde-path` · `planted`; it reports place and class, never the bytes it read.
### error-handling#1.30 · values · errors · ## Error Handling · lines 37–37
- Absence: no check runs after every E2E test; the three surface witnesses, the enumeration and the census of writers stand in its place, reading the check in `tests/support/events.rs`.
### error-handling#1.31 · values · errors · ## Error Handling · lines 38–38
- A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open, checked before anything is narrowed: the role writer ends at `std::io::sink` and the detail line is dropped.
