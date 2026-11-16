@row_set n=1 row=decision/claude-internals-not-a-contract cell=except seen=1 detector=D-obs-instrumentation
value: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
basis: fct-ywi7ddhz
because: The send now reads the transcript's stdout tag count as a ratified echo confirmer, so the row's flat ban on reading transcripts needs that one named exception.
@row_set n=2 row=decision/transcripts-not-a-contract cell=except seen=1 detector=D-obs-instrumentation
value: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
basis: fct-ywi7ddhz
because: The transcript's stdout tag count is now relied on to confirm a local command send, so the row must name that one exception to transcripts never being read as a contract.
@row_set n=3 row=contract/send-path-log-events/event-order cell=promise seen=1 detector=D-obs-instrumentation
value: A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
basis: fct-mce4hr7d
because: An echo-confirmed local command is confirmed by its transcript tag with only the existing send-issued and send-confirmed lines, so it awaits no hook-invoked line and the order must say so.
@row_set n=4 row=contract/send-log-fields/hook-invoked cell=promise seen=1 detector=D-obs-instrumentation
value: hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
basis: fct-mce4hr7d
because: A local command confirmed by its transcript echo closes on the tag count, not a hook, so the row must list it beside the unconfirmable send as awaiting no hook line.
@row_set n=5 row=contract/send-refused-detail/no-prompt-submitted cell=promise seen=1 detector=none
value: Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
basis: fct-rsw37fwj
because: An echo-confirmed local command with no tag in its window now reads no-prompt-submitted at the window, so the row's list of sends sharing that detail must include it.
@row_set n=6 row=decision/refusal-telemetry cell=except seen=1 detector=D-obs-instrumentation
value: The send's transcript echo read refuses a symlinked, non-regular or foreign transcript with no line of its own; the outcome shows only on the send's own lines.
basis: fct-ywi7ddhz
because: The new echo read opens with O_NOFOLLOW, a regular-file and owner check and keeps a count alone with no line, so its symlink refusal is a named exception to refusal telemetry.
@row_set n=7 row=decision/deterministic-emission cell=except seen=1 detector=D-obs-instrumentation
value: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
basis: fct-ywi7ddhz
because: send-confirmed for a local command now keys on the transcript tag count rather than a hook event or an events.ndjson offset, so the row's exception must name it.
@row_set n=8 row=decision/deterministic-event-emission cell=except seen=1 detector=D-obs-instrumentation
value: A local command's echo confirmation keys on the stdout tag count in its session transcript.
basis: fct-ywi7ddhz
because: send-confirmed for a local command now keys on the transcript tag count rather than a hook event or an events.ndjson offset, so the row needs that named exception.
@row_add n=1 row=decision/transcript-echo-read-uninstrumented area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Transcript echo read uninstrumented
rule: The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
basis: fct-mce4hr7d
because: The chunk adds the transcript echo read inside the existing run.confirm_window span with no new line, a coverage choice no standing row records.
@row_add n=2 row=decision/transcript-path-and-bytes-never-kept area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Transcript path and bytes never logged or stored
rule: A session's transcript path and transcript bytes are never logged or stored: the path travels only as the hook.event transcript param, redacted in the hook's types and kept out of event data, and the transcript read keeps a count alone.
guard: send_local_command_echo_is_read_back_on_a_verified_cli
basis: fct-mce4hr7d
because: The chunk carries the transcript path across the channel and reads the transcript, and no standing row bans that path and those bytes from every log and store.
