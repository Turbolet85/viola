@answer n=1 entity=error-capture-reporting#1.1 v=no
why: the session is test-only and moves no src file, so the raw panic backtrace capture in src/panic_frames.rs is unchanged
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: the diagnostics refusal in src/obs.rs (check_diagnostics, open_role_file, write_detail) is product code the session did not change
@answer n=3 entity=error-capture-reporting#2.1 v=no
why: the catch sites on the main thread and the run worker threads are product code the session left unchanged, and the wrapped-child test adds proof only
@answer n=4 entity=error-capture-reporting#3.1 v=no
why: the pre-init window and its instance and home-tree checks are src code the session did not change
@answer n=5 entity=error-capture-reporting#4.1 v=no
why: the vt100 feed catch in src/run/gate.rs and its witness tests/chaos_feed_panic.rs are not among the reached files, and G2's exemption is unchanged
@answer n=6 entity=error-capture-reporting#4.2 v=no
why: the passthrough behaviour in run is unchanged and its witness tests/chaos_feed_panic.rs is not a reached file
@answer n=7 entity=error-capture-reporting#5.1 v=no
why: the frame instance check in crates/viola-mcp/src/tools.rs is unchanged product code that no reached test names
@answer n=8 entity=error-capture-reporting#5.2 v=no
why: check_endpoint_dir and the socket-dir refusal are unchanged product code the session did not reach
@answer n=9 entity=error-capture-reporting#5.3 v=no
why: the UnreachableCause mapping in crates/viola-mcp/src/tools.rs and result.rs is unchanged product code
@answer n=10 entity=error-capture-reporting#5.4 v=no
why: the MCP stdin MAX_FRAME cut in src/main.rs is unchanged, and the cited test name sits in no reached file
@answer n=11 entity=error-capture-reporting#5.5 v=no
why: the cited test mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line is in no reached file and the line it asserts is unchanged
@answer n=12 entity=error-capture-reporting#6.1 v=no
why: sanitised_fault and its two readers in src/cmd/client.rs and crates/viola-mcp/src/result.rs are unchanged product code
@answer n=13 entity=error-capture-reporting#6.2 v=no
why: the is_code_word screening in viola_core and its two readers is unchanged product code
@answer n=14 entity=error-capture-reporting#6.3 v=no
why: the thiserror derive count is a grep over src and crates, which the session did not change
@answer n=15 entity=error-capture-reporting#6.4 v=no
why: the fixed message of Refused::NotADirectory is unchanged product code
@answer n=16 entity=error-capture-reporting#6.5 v=no
why: the exit-2 UNUSABLE lines in src/cmd/answer.rs and src/cmd/send.rs are unchanged, and NOT_ERROR_BODIES in tests/contract_error_bodies.rs is not a reached file
@answer n=17 entity=error-capture-reporting#6.6 v=no
why: unreachable_document, refused_document and the six closed detail words are unchanged src code the session did not reach
