# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the raw panic backtrace lives in src/panic_frames.rs and the panic hook in src/obs.rs, neither reached, and no panic path moved
error-capture-reporting#1.2 · no · the check_diagnostics gate in src/obs.rs is not reached, and the strict-modes check stays first at every caller
error-capture-reporting#2.1 · yes · add-to · the cli catch site under --json prints the internal-error document on stdout with stderr empty (F6), revive --json start refusals and anyhow errors print documents (F7, F9) while a pump_child error prints none (F8); the human line stays byte for byte
error-capture-reporting#3.1 · yes · add-to · under revive --json the start arm's pre-log trusted and checked_exe errors print the internal-error document and the catch site still logs its line (F7); the no-line statement for refusals holds
error-capture-reporting#4.1 · no · the vt100 feed catch in src/run/gate.rs and its parse-rejected detail set are not reached, and no parser detail moved
error-capture-reporting#4.2 · no · the passthrough witness lives in tests/chaos_feed_panic.rs, which is not reached, and the human pairs stay byte for byte
error-capture-reporting#5.1 · yes · contradict · the MCP isError body for a refused frame check now carries the cause as detail (F10) instead of detail null; the text stays instance-unreachable with no frame or line written
error-capture-reporting#5.2 · no · the oversize cut keeps its exit 0 and 1 and its one parse-rejected line; no exit moved and the mcp role writes no process-exit, so src/main.rs exit_code still holds
error-capture-reporting#5.3 · yes · stale · the cited test mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line sits in crates/viola-e2e/tests/mcp_tools.rs, a reached file the report does not show unmoved
error-capture-reporting#6.1 · no · viola_channel::sanitised_fault is untouched (nothing in viola-channel), the -32603 body is not reached, and the peer-fault readings in reply_of and of_reply are unchanged
error-capture-reporting#6.2 · no · is_code_word and the closed refusal list read by reason_of are unchanged, since no RefusalReason was added and no exit moved
error-capture-reporting#6.3 · no · the report adds no error enum, and AgentError's BatchScriptChild and ProgramNotFound messages stay fixed
error-capture-reporting#6.4 · no · the report adds documents only for exit 21, exit 1 and the cli catch site and moves no exit, so the exit-2 fixed lines in src/cmd/answer.rs and src/cmd/send.rs still write no document
