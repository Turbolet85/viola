# Classified — obs · span-trace-coverage.b2

## Answers

span-trace-coverage#7.2 · no · wait.block's `after` / `timeout_ms` and run.wait_dispatch's `woken_kind` are span attributes the chunk neither adds nor changes.
span-trace-coverage#7.3 · yes · the wait/last exit-21 `process-exit` detail set (`server-verify-failed` during connect at WARN) and the instance-check order before frames are where the client euid mismatch and the socket-directory refusal sit, with their new closed codes and the `--json` shape.
span-trace-coverage#7.4 · yes · wait.block must close on the client's new pre-frame connect refusal as it does on a server-verification refusal: no channel.request, and the process-exit is its outcome line.
span-trace-coverage#8.1 · yes · the server check before channel.request(hook.dialog) in hook.handle and before channel.request(answer) in answer.client is where the chunk's Unix client euid check must sit.
span-trace-coverage#8.2 · no · the attributes of run.dialog_register, run.dialog_await and hook.decision_emit are unchanged; the chunk adds no span or attribute on them.
span-trace-coverage#8.3 · yes · the fail-open pair `hook-invoked{corr:null}` + `hook-decision{corr:null, decision_emitted:false, detail}` for a failed server check is where the hook-level refusal lands, with its new fail-open detail.
span-trace-coverage#8.4 · yes · the fail-open-before-dialog_id pair `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` (Section 6 fail-open codes) is where the new hook connect refusal's detail must be admitted.
span-trace-coverage#9.1 · yes · pause.client and release.client run the server check before channel.request, and the chunk's client euid check on those Unix clients must stand before the frame.
span-trace-coverage#9.2 · no · run.wheel_transition's wheel_from / wheel_to / cause attributes are not touched; the chunk moves no wheel.
span-trace-coverage#9.3 · yes · the pause/release server-verification refusal (no channel-request line, process-exit exit 21 detail `server-verify-failed` during connect at WARN) is the shape the chunk's new client refusal sits beside.
span-trace-coverage#9.4 · no · run.wheel_transition's point-in-time cleanup and the `wheel` audit record in events.ndjson are unchanged by the chunk.
span-trace-coverage#10.1 · no · the budget spans (hook.statusline, state.budget_write, statusline.shell_out, run.budget_eval) are not reached; the chunk's hook refusal covers the dialog hook connect only.
span-trace-coverage#10.2 · no · budget span attributes (five_hour_pct, seven_day_pct, shell_exit_status, trigger, paused, window) are not touched; no budget behaviour moves.
span-trace-coverage#10.3 · no · the statusline arm's log fields and its detail codes (oversize-stdin, malformed-json, deadline) are the statusline hook's, which the chunk does not reach.
span-trace-coverage#10.4 · no · statusline.shell_out cleanup and STATUSLINE_DEADLINE are unchanged; the budget governor is not moved.
span-trace-coverage#11.1 · no · the R8 strip fields (env_stripped_*, env_kept) on process-start claude-child; the child-spawn environment does not change.
span-trace-coverage#12.1 · yes · a call the wrapper's new peer euid check refuses records no link, since the implicit link record comes only after refusals; the new refusal must sit before the send/answer link record.
span-trace-coverage#13.1 · yes · the driven wrapper's link/unlink dispatch sits behind the new wrapper-side peer euid refusal, so a refused link connection has no channel-request pair; the -32602 and -32603 rules stand for accepted calls.
span-trace-coverage#14.1 · yes · live_endpoint's per-call `state-recovered` line is read by the CLI frame verbs and the mcp frame tools, where the chunk's connect-time refusals sit; their ordering against that one-line-per-call read must be stated.
span-trace-coverage#15.1 · no · the torn-line heal in the events.ndjson appender; the chunk touches no append path.
span-trace-coverage#16.1 · no · the capability ledger read (read_stamps_strict, state.ledger_read) is not reached by the chunk.
span-trace-coverage#17.1 · no · verify's check_home_tree, plugin install's tree check and their process-exit details are other functions than board::scan's home check, the only home check the chunk edits.
