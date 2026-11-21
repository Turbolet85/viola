@row_set n=1 row=decision/claude-internals-not-a-contract cell=except seen=4 detector=none
value: A send's echo counts a transcript stdout tag, no byte kept; verify writes and removes its temp trust record; a verified start sets its trust flag; verify's sender targets the CLI's inbox socket name.
basis: fct-zjeenbpz, fct-vli7nbiq
because: Run B's cross-session turn now addresses its child by the CLI's own inbox socket name, composed from the runtime dir and the child pid, so the CLI's internal socket naming is read as a contract and the row's exceptions must name it.
@row_set n=2 row=contract/verify-process-exit/child-exit-status-and-duration cell=promise seen=2 detector=D-obs-instrumentation
value: Each verify spawn pair's process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
basis: fct-zjeenbpz, fct-wvhiwmit
because: verify gained the verify-sender spawn pair, so the promise's count of nine pairs no longer holds and the row states it by rule instead.
@row_set n=3 row=contract/diag-line-fields/process-start cell=promise seen=4 detector=D-obs-instrumentation
value: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, verify-sender, hyprctl-probe, auto-verify), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
basis: fct-wvhiwmit
because: The diag-line schema's subject enum gained verify-sender, so the row's closed subject list must carry it.
@row_set n=4 row=contract/process-log/child-and-probe-lifecycle cell=promise seen=1 detector=D-obs-instrumentation
value: process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes, and for verify's print-mode sender.
basis: fct-zjeenbpz, fct-wvhiwmit
because: verify's Run B cross-session turn spawns the print-mode sender verify-sender, logged by its own process-start and process-exit pair, which the lifecycle row omits.
@row_set n=5 row=decision/claude-fixtures-scrub-required cell=except seen=1 fact=F5 detector=D-obs-pii
value: The recorded cross-session arrival keeps, inside the CLI's own prefix, the recording host's runtime-dir socket path holding the uid, the sender's pid and a numeric from-name.
basis: fct-wn36g3ew
because: The fixture scrub and the hygiene walk leave the runtime-dir socket path, pid and from-name inside the recorded cross-session arrival, an exception to the claude fixtures being scrubbed of host values.
@row_set n=6 row=decision/fixtures-not-telemetry cell=except seen=1 detector=D-obs-pii
value: The recorded cross-session arrival keeps, inside the CLI's own prefix, the recording host's runtime-dir socket path holding the uid, the sender's pid and a numeric from-name.
basis: fct-wn36g3ew
because: The recorded set now holds a fixture the scrub does not normalise, so the row's scrub-required rule carries that exception.
@row_add n=1 row=contract/verify-spawn-subject/verify-sender area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
boundary: verify child spawn subjects
item: verify-sender
promise: The print-mode claude sender of Run B's cross-session turn is logged as one process-start / process-exit pair with subject verify-sender; its prompt and the composed socket address appear in no line.
basis: fct-zjeenbpz, fct-udkluxmt
because: verify gained a new child spawn, the cross-session sender, instrumented by its own process pair, and the spawn-subject contract lists one row per subject.
@row_add n=2 row=contract/diag-subject/verify-sender area=log-coverage fact=F2 detector=D-obs-instrumentation
boundary: diag-line subject enum
item: verify-sender
promise: Subject of the process-start and process-exit pair of the print-mode sender that viola verify's Run B cross-session turn spawns.
basis: fct-wvhiwmit
because: The diag-line schema's closed subject enum gained verify-sender, and the enum is held one row per word.
@row_add n=3 row=decision/verify-runtime-dir-value-unlogged area=4-span-trace-coverage fact=F4 detector=D-obs-pii
name: verify's runtime-dir value never logged
rule: viola verify reads XDG_RUNTIME_DIR once into memory, and its value reaches only the cross-session sender's prompt argument, never a log line, span, stamp, step line, error or fixture verify writes itself.
why: Ratified as the one environment value verify reads to compose the sender's address.
basis: fct-zjeenbpz, fct-udkluxmt
because: verify now reads an environment value it did not read before, and the plan must bind where that value may and may not go.
@row_none n=1 fact=F3 type=4
relied: The names-only mode of the hidden hook capture arm files only closed tool words and no payload byte; its being uninstrumented is already stated whole by the standing capture-arm rows, which cover every mode of the arm.
