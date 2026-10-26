# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · validate_paste_text is in viola-core and the wrapper send in src/run/send.rs is not reached (src/run/send/test_support.rs is a test-only helper), and the Untouched list holds paste-text validation unchanged
input-validation#1.2 · no · the Untouched list holds dialog free-text validation unchanged beside validate_paste_text in viola-core, so the answer handler's check still stands
input-validation#1.3 · no · the channel framing lives in viola-channel, which the session did not reach, and the report holds the wire contract and channel -32603 unchanged while tests/security_negatives_channel.rs adds only Windows pipe-admission tests
input-validation#1.4 · yes · stale · src/cmd/hook/statusline.rs, which holds this entity's statusline stdin cap and capture arm, is reached whole by the session, and the report shows only logged fields unchanged, not the cap
input-validation#1.5 · no · the MCP tool handlers live in viola-mcp, which the session did not reach, and the report holds the MCP transport and the rmcp pins unchanged
input-validation#1.6 · yes · stale · src/main.rs, which holds this entity's clap value parsers, is reached whole by the session, and the report shows the interim gaps standing but not the clap rules
input-validation#1.7 · yes · stale · the threshold read in src/run/budget.rs, the hold read in src/run/dialog.rs and the diagnostics-level read in src/obs.rs are all reached whole by the session, and the report shows only the logged parse-rejected lines unchanged, not the reads
input-validation#1.8 · no · the registry name read lives in src/run/env.rs, which the session did not reach, and the report holds the R8 strip unchanged with no dependency added or bumped, so the windows-sys feature named here stands
input-validation#1.9 · yes · stale · src/cmd/run.rs, which holds this entity's pump-delay seam, is reached whole by the session, and the report shows only logged fields unchanged, not the seam's cap
input-validation#1.10 · no · the panic seam lives in src/cmd/hook/seam.rs and its call in hook() in src/cmd/hook.rs, and the session reached neither
input-validation#1.11 · no · the colour reads live in src/human/style.rs and src/cmd/list.rs, and the session reached neither
input-validation#1.12 · yes · stale · the wrapper's budget gate, which this entity names as a product path to the budget.json checked read and which sits in src/run/budget.rs, is reached whole, and the report does not show that gate's read line unchanged
input-validation#1.13 · no · the notify tailing lives in viola-state, which the session did not reach
input-validation#1.14 · no · the claude agents --json read and verify's PTY output live in viola-agent-claude and src/cmd/verify, none reached, and the report holds the claude agents --json read unchanged
input-validation#1.15 · yes · stale · src/run/gate.rs, which holds this entity's tee and feed thread, is reached whole by the session, and the report shows only the logged parse-rejected lines unchanged, not the tee bounds
input-validation#1.16 · yes · stale · the viola run start in src/cmd/run.rs, which sets the child's cwd and writes the first snapshot's cwd, is reached whole, and the report shows no run output line or refusal moved, not the cwd write
input-validation#1.17 · no · the session-id argv lives in viola-agent-claude, src/cmd/revive.rs and src/cmd/verify, none reached, and the report holds revive's session-id argv unchanged
input-validation#1.18 · yes · stale · src/cmd/run.rs, which holds statusline_source and statusline_command, is reached whole by the session, and the report shows only logged fields unchanged, not the source order
input-validation#1.19 · yes · stale · src/cmd/hook/statusline.rs, which holds the shell-out arm, is reached whole by the session, and the report shows only logged fields unchanged, not the arm's order or stdout cap
input-validation#1.20 · yes · stale · src/main.rs (the restrict_dll_search call as the second statement of main) and src/cmd/run.rs (the conpty_sideload step) are reached whole, and the report holds only the ConPTY pins unchanged, not the DLL-search order
input-validation#1.21 · no · the entity is N/A, with no upload, webhook or database surface, and the session added none
