@row_set n=1 row=decision/r8-strip-logged-as-names-only cell=rule seen=2 fact=F5 detector=D-obs-pii
value: The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run and revive start read, and no value is logged.
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read by the run and revive start too, through the same config file lookup, so the row's «verify only» reading no longer holds.
@row_set n=2 row=decision/env-vars-not-configuration cell=rule seen=2 detector=D-obs-pii
value: Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.
basis: fct-xro6zomc
because: The start now reads CLAUDE_CONFIG_DIR's value beside verify to find the config file the trust record lives in, so the row naming verify as its one reader is false.
@row_set n=3 row=decision/claude-internals-not-a-contract cell=except seen=3 detector=none
value: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
basis: fct-xro6zomc
because: The run and revive start now writes the claude config's trust record for its session folder, flag only, so the exception naming verify's write alone is incomplete.
@row_set n=4 row=decision/claude-boundary-only-instrumentation cell=rule seen=1 detector=D-obs-instrumentation
value: The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config.
basis: fct-osk2sbyj
because: The start's trust step on the claude config is a new boundary operation instrumented by one trust-record line, so the closed boundary list must name it.
@row_set n=5 row=decision/refusal-telemetry cell=except seen=3 detector=none
value: The transcript echo read and the verify and start trust-record writes refuse a symlinked, non-regular or foreign file with no refusal line; the send's lines, stamps or trust-record word show it.
basis: fct-osk2sbyj
because: The start's trust step refuses through the same guarded writer and its outcome rides the trust-record line's word, so the exception naming only the send and verify is incomplete.
@row_add n=1 row=contract/start-trust-log/trust-record area=6-log-coverage fact=F1 detector=D-obs-instrumentation
boundary: the start's trust-record log line
item: trust-record
promise: Each wrapped start of viola run and viola revive on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.
basis: fct-xro6zomc
because: The chunk adds the trust-record event as the start trust step's one telemetry, a promise at the log line no standing row makes.
@row_add n=2 row=contract/diag-line-fields/trust-record area=log-coverage fact=F2 detector=none
boundary: diagnostic line additive fields
item: trust-record
promise: A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.
basis: fct-wmizsmr6
because: Both schemas now carry trust-record in their event enums with its message and detail rules, and the per-event field rows hold no row for it.
@row_add n=3 row=decision/start-trust-step-unspanned area=4-span-trace-coverage fact=F3 detector=D-obs-instrumentation
name: Start trust step opens no span
rule: The run and revive start's trust step (the walk, the flag-only write and the read-back) opens no span; its only telemetry is the one trust-record line.
basis: fct-osk2sbyj
because: The report records the new start trust step as uninstrumented by a span by decision, a choice future instrumentation work must respect.
@row_add n=4 row=decision/start-trust-step-placement area=span-trace-coverage fact=F4 detector=none
name: Start trust step after the version gate
rule: The run and revive start's trust step runs after the version gate and before the channel bind, only when the CLI is verified; a revive keys its trust record by the snapshot's recorded cwd.
basis: fct-xro6zomc
because: The chunk places a new step in the start sequence, gated on a verified CLI, and no row of the start's coverage says where it stands.
