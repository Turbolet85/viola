@row_set n=1 row=decision/compiled-classification-sources cell=rule seen=1 detector=none
value: Screen signatures, harness prefixes and local-command names come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text; a typed text is a local command only by its compiled name, alone or followed by one space and arguments.
basis: fct-ywi7ddhz
because: Local commands are now classified by their compiled name rather than by exact text, a ratified widening, so the row must say what the classification is made from.
@row_set n=2 row=decision/shared-validators-in-core cell=except seen=2 detector=D-security-input
value: The Hyprland seam's window Address validator lives in the seam crate, and the Claude transcript path shape check in the Claude adapter crate, which the hook and the wrapper both call.
basis: fct-ywi7ddhz
because: The new transcript path shape check is a shared validator that lives in the Claude adapter crate rather than viola-core, so the row's named exception must include it.
@row_set n=3 row=contract/wrapper-channel-methods/hook-event cell=promise seen=1 detector=D-security-input
value: A notification from the hook to the wrapper; for a SessionStart it may carry the optional transcript param beside the event, which the wrapper re-checks for a session-start line only and drops otherwise.
basis: fct-ywi7ddhz
because: hook.event gains the optional transcript sibling param, so the method's promise must name it and its wrapper-side re-check.
@row_add n=1 row=decision/transcript-path-shape-check area=input-validation fact=F1 detector=D-security-input
name: Transcript path shape checked on both sides
rule: A SessionStart transcript path is accepted only when absolute, with no parent component, its file name exactly the payload's own session id plus .jsonl and that id of the closed session-id shape; the hook checks it and the wrapper re-checks it.
guard: run_hook_event_transcript_is_re_validated
basis: fct-ywi7ddhz
because: The transcript path is a new external input crossing the channel, and the validation its boundary takes is a standing control.
@row_add n=2 row=decision/transcript-read-bounded area=input-validation fact=F2 detector=D-security-input
name: Transcript read bounded and count-only
rule: The Claude transcript is read without following a symlink, only as a regular file owned by the effective user, from the recorded pre-send offset through MAX_FRAME, yielding a count of local-command-stdout tags alone with no byte kept.
guard: echo_a_symlink_is_refused; echo_a_read_is_capped_at_max_frame
basis: fct-mce4hr7d
because: The transcript file is a new external read, and the controls on it are what the input validation section must hold.
@row_add n=3 row=decision/transcript-path-redacted area=data-protection fact=F3 detector=none
name: Transcript path redacted
rule: The Claude transcript path is a redacted value, carried only as the optional transcript param beside hook.event, and appears in no event data, events log, log line, span, reply or verify record.
guard: session_start_transcript_path_stays_out_of_data
basis: fct-mce4hr7d
because: The transcript path carries the OS username, and where it may and may not appear is a new data-protection rule.
@row_add n=4 row=decision/local-command-echo-confirmation area=threat-model-summary fact=F4 detector=none
name: Local command confirmed by its stdout echo
rule: A local command whose post-condition is the stdout echo (/mcp) is confirmed delivered only on a verified CLI by one or more new stdout tags in the transcript past its pre-send end within the confirm window; without a transcript it is unconfirmable.
why: A ratified widening: the transcript's stdout tag is an echo send may rely on, a count with no content crossing.
except: /cost and /usage confirm nothing.
guard: send_local_command_echo_is_read_back_on_a_verified_cli
basis: fct-vrnudrgx
because: Send now relies on the CLI's transcript tag to confirm a local command, a ratified trust widening the threat model must record.
@row_add n=5 row=decision/verify-echo-probe-residuals area=data-protection fact=F5 detector=none
name: Verify's echo probe residuals accepted
rule: verify's Run B pastes /mcp disable viola-probe-absent seventh as the probe of the local-command-echo relied row; whether it writes to the user's real Claude config stays unmeasured and it adds one input-history line.
why: Accepted residuals of a ratified widening.
basis: fct-vrnudrgx
because: The seventh Run B paste is a new real-CLI probe whose named residuals the wrap directive says land as standing rows.
