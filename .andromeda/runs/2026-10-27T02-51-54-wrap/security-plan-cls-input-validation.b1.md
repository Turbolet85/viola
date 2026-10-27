# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · paste text is listed untouched, and its rule lives in viola-core, viola-mcp, src/cmd/send.rs, src/run/send.rs and viola-pty, none reached
input-validation#1.2 · no · dialog free text is listed untouched, and its validation lives in viola-core and the answer handler, none reached
input-validation#1.3 · no · channel frames are listed untouched, and their framing lives in viola-channel, which the change does not reach
input-validation#1.4 · yes · stale · the change adds place_private_new to reached fs.rs, which also holds the create_private_new and replace_private this capture claim names, and no report line shows them unchanged
input-validation#1.5 · no · MCP tool inputs are listed untouched, and the handlers live in viola-mcp, while the reached mcp_tools.rs only changes which pair the MCP leg boots
input-validation#1.6 · no · strict.rs is reached but its product code is unchanged (only a cfg(windows) test module gained a control), so the check_instance and check_home_tree calls it names still hold
input-validation#1.7 · no · config.json is listed untouched, and its reads live in viola-state and root bin files, none reached
input-validation#1.8 · no · the registry name read lives in src/run/env.rs, which the change does not reach
input-validation#1.9 · no · the FAKE_AGENT_PUMP_DELAY_MS seam lives in src/cmd/run.rs, not reached, and the new placed hook is a private test parameter with no env read, so this row still holds
input-validation#1.10 · no · the FAKE_AGENT_HOOK_PANIC seam lives in src/cmd/hook/seam.rs, not reached, and the fake agent change adds only a receipt field, so this row still holds
input-validation#1.11 · no · the colour-fact reads live in src/human/style.rs and src/cmd/list.rs, neither reached, and the session adds no env read
input-validation#1.12 · no · its state readers live in viola-state files the change does not reach, and the strict-modes checks it cites are unchanged in product code
input-validation#1.13 · no · the tailing rule lives in viola-state tailing code outside the reached files, and no events.ndjson path changes
input-validation#1.14 · no · child output reads live in viola-agent-claude and the verify and list code, none reached, and src/cmd/verify.rs is unchanged
input-validation#1.15 · no · PTY output and the vt100 feed live in src/run/gate.rs and viola-agent-claude, neither reached
input-validation#1.16 · no · child program resolution lives in viola-agent-claude, src/run/mod.rs, viola-pty and src/cmd/revive.rs, none reached
input-validation#1.17 · no · the session-id argv rule lives in viola-agent-claude, src/cmd/revive.rs, viola-state's replay code and verify's Run E, none reached
input-validation#1.18 · no · the statusline source read lives in src/cmd/run.rs and viola-agent-claude, and the report keeps src/cmd/run.rs unchanged
input-validation#1.19 · no · the shell-out lives in viola-agent-claude, src/cmd/hook/statusline.rs and src/cmd/hook.rs, and the report keeps the statusline arm unchanged
