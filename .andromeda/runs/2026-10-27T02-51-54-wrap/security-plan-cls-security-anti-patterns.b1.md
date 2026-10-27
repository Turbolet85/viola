# Classified — security-plan · security-anti-patterns.b1

## Answers
security-anti-patterns#1.1 · no · the channel and client frame paths are untouched, and strict.rs changed only its cfg(windows) test module, its product code and verdict unchanged
security-anti-patterns#2.1 · yes · stale · create_private_dir lives in fs.rs, which the change reaches whole with place_private_new, and the report does not show its protected-DACL claim still holds on the changed file
security-anti-patterns#3.1 · no · its keepers live in tests/support/home.rs and the harness, neither reached, and the change to the reached test file mcp_tools.rs is Path 7's MCP boot, which this entity does not name
security-anti-patterns#4.1 · no · the 24-row ledger and the verify-only stamp write live in no reached file, src/cmd/verify.rs is unchanged, and no ledger row is added
security-anti-patterns#4.2 · no · strict.rs's product code, where check_stamps lives, is unchanged (only its cfg(windows) test module moved), and src/cmd/verify.rs is unchanged
security-anti-patterns#5.1 · no · the report shows the seam set unchanged: place_private_new_with's placed hook reads no env and has no feature, and the fake agent's spawn_os_error adds no seam or env read
