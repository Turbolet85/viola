# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · its hook-parse to event-kind to append chain is not reached, and the chunk's cross-crate calls sit at the IPC boundary (#1.4)
integration-test-strategy#1.2 · no · the concurrent-append witness covers parallel hook, mcp, cli and colliding-run appenders, and the chunk adds none of them
integration-test-strategy#1.3 · no · the agents-read fake-agent modes and the .cmd shim fixture are not reached, the chunk changes no fake agent or fixture, and its admission tests drive no claude child
integration-test-strategy#1.4 · yes · the squatter is bound first through interprocess's server API, never try_overwrite, and the pipe DACL is read back and compared in canonical SDDL form, never the literal set
integration-test-strategy#1.5 · no · the ConPTY spawn, resize and key-loss rules and the ConPTY sideload are not reached, the chunk touches no pump, resize or sideload code, and the sideload is on its Not-needed list
integration-test-strategy#1.6 · no · the hook-fed event line, trace and budget.json assertions are not reached, since the hook path is unchanged and case_4 is a root unit case, not a hook-fed run
integration-test-strategy#2.0 · yes · a fresh tempfile home per test under target/e2e-home, wrappers held in a Drop guard, nothing shared between tests, and the wait_ready readiness stall report the chunk names
integration-test-strategy#3.1 · yes · a refused send's send-refused record holds no cursor and control-character wins over a paused budget, the refusal behaviour case_4 re-asserts in the root send unit list
integration-test-strategy#3.2 · no · the events round-trip, torn-line heal, symlink tailing and E5 replay are untouched, and the strict-modes exit 1 for run is a different cause from the squatted-name exit 1 the chunk re-verifies
integration-test-strategy#4.1 · yes · root tests/cli_*.rs pin the --json document per exit code, and the squatted-name exit-1 case's document and hint line must stand unchanged
