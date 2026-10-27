# Classified — security-plan · error-handling.b1

## Answers
error-handling#1.1 · no · the hook arms, the stdin cap and the server check are untouched, strict.rs's product check is unchanged, and fs.rs gains place_private_new and Placed beside create_private_new, which the session does not change, so the capture's exclusive claim still holds
error-handling#1.2 · no · the server check and its hook-decision and SessionEnd paths are untouched, as the report's Untouched list names server verification
error-handling#1.3 · no · the replayed or absent snapshot at connect and its channel-unreachable fallback are untouched, and no reached file holds read_snapshot_or_replay
error-handling#1.4 · no · the hook statusline arm and src/cmd/run.rs are unchanged, so nothing runs and nothing prints over a replayed snapshot as before
error-handling#1.5 · no · the viola-mcp server check and its instance-unreachable isError mapping are untouched, and mcp_tools.rs changes only a test-only boot of path 7's MCP leg
error-handling#1.6 · no · viola-mcp's check order is untouched and strict.rs's check_instance product code is unchanged, so the check still runs before the liveness pre-check
error-handling#1.7 · no · the wrapper's link and unlink record and remove failures and their -32603 mapping are untouched, with no new external error or error variant
error-handling#1.8 · no · client::refuse_untrusted and the CLI frame verbs are untouched, and strict.rs's check_instance product code is unchanged
error-handling#1.9 · no · run.rs is unchanged and check_home_tree's product code is unchanged, so the home-tree check still runs before the wrapper log opens
error-handling#1.10 · no · pin.rs adds no error variant or detail code and run.rs's mapping is unchanged, so PinError::StrictModes's fixed message and its strict-modes-failed outcome still hold on the changed pin.rs (stamps.rs is not reached)
error-handling#1.11 · no · strict.rs's check_diagnostics product code is unchanged, and the placement change narrows only the pinned copy, never the diagnostics folder or role file
