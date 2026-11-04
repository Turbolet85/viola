@answer n=1 entity=error-handling#1.26 v=no
why: the socket-directory refusal in refused_with and refuse_socket_dir lives in src/, which this session did not reach
@answer n=2 entity=error-handling#1.27 v=no
why: prints_internal_error and json_asked live in src/main.rs, which this session did not reach
@answer n=3 entity=error-handling#1.28 v=no
why: collision_check lives in product src/, not reached, and its stale and live holder readings are unchanged
@answer n=4 entity=error-handling#1.29 v=no
why: the StampsError and PinError messages live in product crates this session did not reach, and the strict-modes check is unchanged
@answer n=5 entity=error-handling#1.30 v=no
why: Refused::NotADirectory and check_socket_dir live in product src/, which this session did not reach
@answer n=6 entity=error-handling#1.31 v=no
why: the nine thiserror derives are product sources, and their census file tests/contract_error_bodies.rs is not among the reached files
@answer n=7 entity=error-handling#1.32 v=no
why: the open divergences concern product error enums, and this session changed no error type
@answer n=8 entity=error-handling#1.33 v=no
why: resolve_program and its AgentError refusals live in src/run/mod.rs and viola_agent_claude, which this session did not reach, and the .cmd/.bat refusal is unchanged
@answer n=9 entity=error-handling#1.34 v=no
why: tests/contract_error_bodies.rs is not among the reached files, and no error writer or message changed
@answer n=10 entity=error-handling#1.35 v=yes mode=stale
why: the session reached tests/cross_exit_causes.rs (task file 6), and the report does not show its exit-21 and exit-1 cause table and required-features line hold on the changed file
@answer n=11 entity=error-handling#1.36 v=no
why: the error-body check lives in tests/support/events.rs, which this session did not reach, and only the sibling tests/support/mod.rs changed
@answer n=12 entity=error-handling#1.37 v=no
why: the absence statement reads tests/support/events.rs and the writer census, neither reached, and no after-every-E2E check was added
@answer n=13 entity=error-handling#1.38 v=no
why: strict::check_diagnostics is product code under src/, not reached, and the strict-modes controls are unchanged with only gate-reason lines added to tests
