@answer n=1 entity=conventions#1.1 v=no
why: the session added and changed no RefusalReason word, detail or ordering, and its tests are test-only
@answer n=2 entity=conventions#2.1 v=no
why: viola_core::is_code_word is product code under src/ or crates/*/src/, which the session did not move
@answer n=3 entity=conventions#3.1 v=no
why: reply_of and of_reply are product code the session did not touch
@answer n=4 entity=conventions#4.1 v=no
why: UnreachableCause's six words are product code the session did not change
@answer n=5 entity=conventions#5.1 v=no
why: the CLI exit-code set is product behaviour, and the session moved no product exit code, only test gating and a test-only binary
@answer n=6 entity=conventions#6.1 v=no
why: json_asked and internal_error_document are product code in src/main.rs and src/cmd/mod.rs, which the session did not move
@answer n=7 entity=conventions#7.1 v=no
why: socket_dir_ready in src/cmd/run.rs is product code the session did not move
@answer n=8 entity=conventions#8.1 v=no
why: refuse_tree and checked_exe in src/cmd/run.rs are product code the session did not move
@answer n=9 entity=conventions#9.1 v=no
why: check_home_tree and update_stamps are product code the session did not move
@answer n=10 entity=conventions#10.1 v=no
why: verify's refusal codes in src/cmd/verify.rs are product code the session did not move
@answer n=11 entity=conventions#11.1 v=no
why: the peer_admitted and pid or start-time server check is product code the session did not change, and the no-peer-pid gap is unchanged
@answer n=12 entity=conventions#12.1 v=no
why: the hint wording in src/human/hints.rs and hint_cause in src/cmd/client.rs is product code the session did not change
@answer n=13 entity=conventions#13.1 v=no
why: the macOS no-peer-pid carve-out is a product gap the session did not change, and the report says the gap itself is unchanged, so the claim still holds
@answer n=14 entity=conventions#14.1 v=no
why: client::refuse_untrusted is product code the session did not change
@answer n=15 entity=conventions#15.1 v=no
why: answer_of and Unreachable::without_endpoint are product code the session did not change
@answer n=16 entity=conventions#16.1 v=no
why: connect's refused socket-directory check in src/cmd/client.rs is product code the session did not change
@answer n=17 entity=conventions#17.1 v=no
why: the MCP result shapes are product behaviour, and the test and fixture files it names (tests/mcp_recorded_exchange.rs, fixtures/claude/2.1.302) were not reached by the session
@answer n=18 entity=conventions#18.1 v=no
why: the frame-tool server-verify refusal is product code in the MCP handler, which the session did not change
@answer n=19 entity=conventions#19.1 v=no
why: MCP's frame path and its socket-directory refusal are product code the session did not change
@answer n=20 entity=conventions#20.1 v=no
why: the MCP strict-modes check via check_instance is product code the session did not change
