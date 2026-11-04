@answer n=1 entity=error-handling#1.26 v=no
why: the rule lives in src (refused_with, refuse_socket_dir), which the session did not reach and no refusal word changed
@answer n=2 entity=error-handling#1.27 v=no
why: prints_internal_error and json_asked live in src/main.rs, which the session did not reach
@answer n=3 entity=error-handling#1.28 v=no
why: collision_check and live_endpoint live in product src, not reached, and the stale-holder logic is unchanged (only the Frozen guard moved in tests)
@answer n=4 entity=error-handling#1.29 v=no
why: the StampsError and PinError fixed Display messages live in viola_state src, not reached, and no error type changed
@answer n=5 entity=error-handling#1.30 v=no
why: Refused and check_socket_dir live in product src, not reached, and the error set is unchanged
@answer n=6 entity=error-handling#1.31 v=no
why: the thiserror derives live in product sources, not reached, and no error type changed so the count of nine stands
@answer n=7 entity=error-handling#1.32 v=no
why: the open divergences are product error enums in src, not reached, and nothing moved them
@answer n=8 entity=error-handling#1.33 v=no
why: resolve_program and AgentError::BatchScriptChild live in src, not reached, and the .cmd/.bat refusal is unchanged (verify_a_batch_script_refuses only gained a reason line)
@answer n=9 entity=error-handling#1.34 v=no
why: tests/contract_error_bodies.rs is not among the files the session reached and no product error writer changed
@answer n=10 entity=error-handling#1.35 v=yes mode=stale
why: tests/cross_exit_causes.rs is a reached file whose two gates gained reason lines, and the report does not show its claim still holds on the changed file
@answer n=11 entity=error-handling#1.36 v=no
why: tests/support/events.rs is not reached (only tests/support/mod.rs was) and the error-body check it holds is not in the change
@answer n=12 entity=error-handling#1.37 v=no
why: no check was added after E2E tests and the census of writers in contract_error_bodies.rs is not reached; the new census names OS-gated tests, not error writers
@answer n=13 entity=error-handling#1.38 v=no
why: strict::check_diagnostics lives in src, not reached, and the strict-modes change only added reason lines to tests
