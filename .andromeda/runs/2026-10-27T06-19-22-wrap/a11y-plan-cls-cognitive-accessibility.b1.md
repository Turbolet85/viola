# Classified — a11y-plan · cognitive-accessibility.b1

## Answers
cognitive-accessibility#1.1 · yes · stale · the MCP wait clamp (`viola_mcp::wait_timeout`) sits in viola-mcp, whose lib.rs the change reached along with the new stdin line cap, and the report reads only the human-bound timeouts, not the clamp on the changed file
cognitive-accessibility#2.1 · no · the session added no refusal code, exit code, error variant or hint line, and the MCP stdin cut is a session end with no CLI refusal
cognitive-accessibility#3.1 · no · its strict-modes refusal is the check in viola-state's strict module, which no reached file holds, and the report adds no refusal code, exit code or hint line
cognitive-accessibility#4.1 · no · the frame verbs' strict-modes refusal is `check_instance` in viola-state's strict module and `client::refuse_untrusted`, neither a reached file, and the report adds no refusal code, exit code or hint line
cognitive-accessibility#5.1 · no · the start refusal is `check_home_tree` in viola-state's strict module and `src/cmd/run.rs`, neither a reached file, and the one reached fs.rs change touched only replace_private_shared, not the pinned copy's placement or the check
cognitive-accessibility#6.1 · no · the verify refusal is `check_home_tree` and `check_stamps` in viola-state's strict module and verify's own code, none a reached file, and the report adds no refusal code, exit code or hint line
cognitive-accessibility#7.1 · no · the unstattable-home rule is the strict check in viola-state's strict module, not a reached file, and the report adds no refusal, exit code or hint line
cognitive-accessibility#8.1 · no · the stopped-instance refusal is the liveness pre-check in `link_call` (`src/cmd/link.rs`), not a reached file, and the report adds no hint line
cognitive-accessibility#9.1 · no · the -32602 fault reaches the verb through `link_call` and the wrapper's channel, neither a reached file, and the report adds no exit code or hint line
cognitive-accessibility#10.1 · no · the cases it names sit in `tests/cli_*`, `tests/security_negatives_server_identity.rs` and `src/human`, none a reached file, and the report adds no refusal, exit code or hint layout
cognitive-accessibility#11.1 · no · the cases it names sit in `tests/cli_links.rs`, which the change does not reach, and the report adds no link refusal, exit code or hint
cognitive-accessibility#12.1 · no · the cases it names sit in `tests/security_negatives_strict_modes.rs`, `tests/run_cli.rs` and `tests/cli_verify.rs`, none a reached file, and the report adds no refusal, exit code or hint line
cognitive-accessibility#13.1 · no · the pinned literals it names sit in CLI test files and `src/human`, none a reached file, and the report says no fixed CLI string, help text or pinned literal changed
cognitive-accessibility#14.1 · no · the three link result lines and their root case sit in `src/human/verbs.rs` and `tests/cli_links.rs`, neither a reached file, and the report says no fixed CLI string changed
cognitive-accessibility#15.1 · no · the link time and name rule is the unit `link_since_is_the_utc_time_alone` in `src/human/verbs.rs`, not a reached file, and the report says no fixed CLI string changed
cognitive-accessibility#16.1 · no · the `viola list` caption order and its `CAPTION` literal in `tests/cli_list.rs` are not reached, as the report says for cognitive-consistency
