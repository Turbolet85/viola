@row_set n=1 row=contract/verify-spawn-subject/verify-pty-probe cell=promise seen=1 detector=none
value: Each of verify's six interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
basis: fct-skwar3iy
because: verify's interactive runs went from five to six (A to F, Run F the trust-record run), so the run count the promise holds moved.
@row_set n=2 row=contract/diag-subject/verify-pty-probe cell=promise seen=1 detector=none
value: Subject of the process-start and process-exit pair of each of the six interactive PTY runs of viola verify, Run A to Run F.
basis: fct-skwar3iy
because: verify gained Run F, logged with the existing verify-pty-probe pair, so the count and the run span the subject covers moved.
@row_set n=3 row=contract/verify-process-exit/child-exit-status-and-duration cell=promise seen=1 detector=none
value: Each of the nine verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
basis: fct-skwar3iy
because: verify's spawn pairs went from eight to nine, so the count the promise holds moved.
@row_set n=4 row=decision/claude-internals-not-a-contract cell=except seen=2 detector=none
value: A send's echo confirmation counts its local command's stdout tag in the session transcript, keeping no byte; viola verify writes and removes its own temp folder's trust record in the claude config.
basis: fct-phopoxyf
because: verify now writes and removes its own temp folder's trust record in the claude config file its child reads, a ruled bounded exception the row must name.
@row_set n=5 row=decision/refusal-telemetry cell=except seen=2 detector=D-obs-instrumentation
value: The send's transcript echo read and verify's trust-record write refuse a symlinked, non-regular or foreign file with no line of their own; the outcome shows only on the send's lines or the stamps.
basis: fct-oyqnxj2g
because: the trust-record writer refuses a link, a non-regular or a foreign config file with no log line, its outcome riding the stamp only, a second exception to refusal telemetry.
@row_set n=6 row=decision/r8-strip-logged-as-names-only cell=rule seen=1 detector=D-obs-pii
value: The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value viola verify reads, and no value is logged.
basis: fct-v6anf47h
because: viola verify now reads CLAUDE_CONFIG_DIR's value when the strip keeps that name, so the row's «values never read» no longer holds whole.
@row_set n=7 row=decision/env-vars-not-configuration cell=rule seen=1 fact=F2 detector=none
value: Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value viola verify reads, only when the strip keeps that name, locates the claude config file its child reads, is never logged and configures nothing of viola.
basis: fct-v6anf47h
because: verify reads one environment value to find its child's config file, and the row that says env vars configure nothing must name that bounded read.
@row_add n=1 row=decision/trust-record-write-uninstrumented area=4-span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Trust-record write and removal uninstrumented
rule: viola verify's trust-record write and removal emit no log line and no span; the outcome rides only the stamps, as the trust-record row's word and the trust probe's write and remove words, and no path, key or value of the claude config reaches any line, stamp or summary.
basis: fct-oyqnxj2g
because: the chunk adds a config-file writer whose outcome is deliberately carried by the stamps alone, a rule future instrumentation and redaction work must respect.
