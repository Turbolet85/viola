# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · yes · stale · the session reaches crates/viola-core/src/lib.rs whole, where validate_paste_text lives, and the report does not show its character set holds on the changed file
input-validation#1.2 · yes · stale · the session reaches crates/viola-core/src/lib.rs whole, where the shared validate_paste_text lives, and the report does not show it holds there
input-validation#1.3 · yes · stale · the session reaches viola-channel's lib.rs, client.rs and server.rs whole and drops a refused peer before any frame in accept_loop, and the frame cap, depth and version rules in those files are not shown to hold
input-validation#1.4 · yes · stale · src/cmd/hook.rs is reached whole and this row's instance-check enumeration is one of the hook's checks the session extends with the directory and euid checks at connect
input-validation#1.5 · yes · stale · crates/viola-mcp/src/tools.rs is reached whole and the MCP frame tools gain the directory and euid refusals, while the invalid-params rule lives in that file and the report does not show it holds
input-validation#1.6 · no · the MCP stdin cap lives in viola-mcp stdin.rs and src/main.rs's exit mapping, neither reached, and the session reaches no stdin or serve path
input-validation#1.7 · yes · stale · scan now holds the home to check_read on every OS where this row says check_path (F9), the frame verbs' server check now runs on macOS too, and revive --json's exit-1 codes move from ten to eleven
input-validation#1.8 · yes · stale · crates/viola-core/src/lib.rs is reached whole and holds Percent and DialogHold, the types this row names, and the report does not show them holding on the changed file
input-validation#1.9 · no · the registry name read lives in src/run/env.rs, which the session does not reach
input-validation#1.10 · no · F4 puts the start check before the snapshot, child and bind, and hold_pump_start in src/cmd/run.rs still holds the pump after the child starts
input-validation#1.11 · no · the seam fires in hook() before the stdin lock, and the session's directory and euid refusals sit at connect after the stdin read and obs init, leaving the exit-0 and empty-stream fail-open unchanged
input-validation#1.12 · no · the colour rule lives in src/human/style.rs, which the session does not reach, and the new refusal wording in hints.rs is not colour
input-validation#1.13 · yes · stale · board.rs is reached and scan now holds the home to check_read on every OS, the home check that list's board read here follows, so this row's home strict-modes claim moved for list
input-validation#1.14 · no · the tail's symlink and name rule lives in viola-state's tailing, which the session does not reach, and the list home change is scan's, not the tail's
input-validation#1.15 · no · the agents read's failure only leaves board::read's result as given, and F9 keeps the scan order, whole-board refusal and no-open unchanged
input-validation#1.16 · no · the tee and screen rules live in src/run/gate.rs and viola-agent-claude, neither reached, and the session's run change sits before the child starts, not on the pump output
input-validation#1.17 · no · resolve_program and the cwd-missing and batch-script refusals live in src/run/mod.rs and src/cmd/revive.rs, neither reached, and the session's start check runs before any child, so those refusals still spawn none
input-validation#1.18 · no · the session-id shape and the no-session refusal live in viola-agent-claude, viola-state's replay and src/cmd/revive.rs, none reached, and the new start refusal is a separate exit that spawns nothing either
