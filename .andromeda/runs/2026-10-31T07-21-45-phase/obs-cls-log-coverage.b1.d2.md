# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · env_stripped_count, env_stripped_known and env_kept carry the names-and-count-only strip record that the witnesses test against a planted CLAUDE* value
log-coverage#1.2 · yes · process-exit detail and exit_code carry the internal-error fault path and its closed detail codes
log-coverage#1.3 · yes · channel-request v, from and from_trust carry the unsupported-version cause and the self-reported-from rule of the channel error answers
log-coverage#1.4 · yes · channel-response error_code and its detail fields are where the -32602 causes must be told apart, the line the Drop guard and the v_rejected metric read
log-coverage#1.5 · yes · send-refused refusal and detail are the codes viola mcp send isError carries, and text_bytes records only the length of the typed text
log-coverage#1.6 · no · dialog events sit outside the three surfaces and the reached units, and the dialog hold reaches only the config-json parse-rejected line
log-coverage#1.7 · yes · hook-invoked and hook-decision detail, deadline_hit and stdin_bytes are what the statusline arm writes on oversize and at the STATUSLINE_DEADLINE bound
log-coverage#1.8 · yes · the v1-47 re-read of content reaching only owner-readable detail files covers the panic payload and backtrace, which go only to detail-<process>.ndjson
log-coverage#1.9 · yes · release-from-driver is a named -32602 cause, and its from is logged only as a valid ViolaName with self-reported trust
log-coverage#1.10 · no · state-recovered is crash-recovery telemetry that the Not-needed list excludes, and no reached unit or surface writes it
log-coverage#1.11 · yes · the mcp-stdin, hook-stdin, claude-agents-json, config-json and channel-frame parsers are the readers the caps and BOUNDARIES rows hit, while the vt100, paste and state-entry parsers are not
log-coverage#2.1 · yes · internal-error and batch-script-child, a code the Refusal callers log, bind, while the revive preflight documents and the exit-cause catalogue sit with their own chunk
log-coverage#2.2 · yes · server-verify-failed on the frame verbs carries no pid, path or endpoint, under the closed kebab-case detail rule
log-coverage#2.3 · yes · wrapper-fault is the exit-20 code that viola mcp isError results carry
log-coverage#2.4 · yes · oversize-stdin, malformed-json and deadline are the statusline arm's fail-open codes under STATUSLINE_DEADLINE
log-coverage#2.5 · yes · oversize for mcp-stdin, hook-stdin, claude-agents-json and channel-frame, and unknown-keys for config-json, are the parse paths the chunk's caps and readers hit
log-coverage#2.6 · yes · the mcp-stdin oversize record, once per session with no byte of the cut line, is the MAX_FRAME test's record, and its obs_event! route is S2's and not carried here
log-coverage#2.7 · yes · the RefusalReason details are the send refusal codes that viola mcp send carries in its isError text
log-coverage#2.8 · yes · the refused-check bodies (instance-unreachable documents, exit 21 with no line) bind on the CLI verbs and viola mcp frame tools, while the check mechanisms sit with the hardening chunk
log-coverage#3.1 · yes · the version-probe and agents-probe spawns, with claude agents --json parse failure giving unknown plus one parse-rejected, bind the child pipe drain
