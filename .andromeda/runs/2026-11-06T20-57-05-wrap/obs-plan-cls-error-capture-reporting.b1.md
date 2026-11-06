@answer n=1 entity=error-capture-reporting#1.1 v=no
why: its backtrace rule lives in src/panic_frames.rs and src/obs.rs, neither reached, and its Windows-runner measurement is a historical citation the session did not move
@answer n=2 entity=error-capture-reporting#1.2 v=yes mode=stale
why: check_diagnostics lives in crates/viola-state/src/strict.rs, a reached file, and the report does not show its symlink and read-rule claim holds on the changed file
@answer n=3 entity=error-capture-reporting#2.1 v=yes mode=stale
why: its catch sites read internal_error_document in src/cmd/mod.rs and the pump thread body in crates/viola-pty/src/pump.rs, both reached, and the panic=unwind profile sits in Cargo.toml, which the portable-pty pin move reached
@answer n=4 entity=error-capture-reporting#3.1 v=yes mode=stale
why: client::refuse_untrusted in src/cmd/client.rs and the hook event path in src/cmd/hook.rs are reached whole, and the report does not show their pre-init refusals hold
@answer n=5 entity=error-capture-reporting#4.1 v=yes mode=stale
why: its witness tests/chaos_feed_panic.rs and the send-refused exit path in src/cmd/send.rs are reached, and the report does not show the claim holds on them
@answer n=6 entity=error-capture-reporting#4.2 v=yes mode=stale
why: its witness tests/chaos_feed_panic.rs is reached whole, and the report does not show the passthrough claim holds there
@answer n=7 entity=error-capture-reporting#5.1 v=no
why: its frame-tool instance check lives in crates/viola-mcp/src/tools.rs, which the change does not reach
@answer n=8 entity=error-capture-reporting#5.2 v=no
why: check_endpoint_dir lives in the unreached viola-mcp tools.rs, and the socket-dir wording was confirmed unchanged with no PROVISIONAL mark on this row
@answer n=9 entity=error-capture-reporting#5.3 v=no
why: the refusal-to-cause mapping lives in the unreached viola-mcp tools.rs and result.rs, and no reached file hosts it
@answer n=10 entity=error-capture-reporting#5.4 v=no
why: the MAX_FRAME cut rule lives in the unreached src/main.rs and viola-mcp, and it cites no reached witness
@answer n=11 entity=error-capture-reporting#5.5 v=yes mode=stale
why: its witness test mcp_tools_a_line_past_max_frame_before_initialize lives in crates/viola-e2e/tests/mcp_tools.rs, a reached file, and the report does not show the claim holds there
@answer n=12 entity=error-capture-reporting#6.1 v=yes mode=stale
why: reply_of in src/cmd/client.rs is a reader of the peer fault hosted in a reached file, and the report does not show its screening holds on it
@answer n=13 entity=error-capture-reporting#6.2 v=yes mode=stale
why: reply_of in src/cmd/client.rs screens replies by is_code_word, and the report does not show that path holds on the reached file
@answer n=14 entity=error-capture-reporting#6.3 v=yes mode=stale
why: the thiserror census was re-worded to product sources so the grep now reads 10 rows with the 10th test-only in crates/viola-e2e/src/harness/mod.rs, while the count stays 9
@answer n=15 entity=error-capture-reporting#6.4 v=no
why: the census report shows the count stays 9 over the same product rows, so NotADirectory still adds no derive of its own
@answer n=16 entity=error-capture-reporting#6.5 v=yes mode=stale
why: src/cmd/send.rs hosts the exit-2 UNUSABLE_TEXT stderr path and the change reached that file, and the report does not show the path holds
@answer n=17 entity=error-capture-reporting#6.6 v=yes mode=stale
why: unreachable_document in src/cmd/client.rs and refused_document in src/cmd/list.rs both host the document writers this entity names, and both files are reached
