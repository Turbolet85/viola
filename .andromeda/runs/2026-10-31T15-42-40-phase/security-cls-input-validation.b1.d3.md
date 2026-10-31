# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the paste-text rule (validate_paste_text, the typed text, the empty-text refusal, the send-refused line) is unchanged; the chunk reaches send only through refuse_untrusted, which #1.7 carries
input-validation#1.2 · no · the dialog free-text rule (answer's exit 13 before any frame, the wrapper's answer check) is unchanged; the chunk reaches answer only through refuse_untrusted, which #1.7 carries
input-validation#1.3 · yes · the "A reply, read by a client" paragraph: reply_of (src/cmd/client.rs) and of_reply (crates/viola-mcp/src/result.rs) take refusal or detail only as a code word, so each new code must pass viola_core::is_code_word, and the channel-response detail rule binds the vanished-server reading in viola-channel/src/client.rs; the wrapper's frame limits do not
input-validation#1.4 · yes · the hook event and dialog instance check (canonical, then trusted, before any config.json read, stdin read, line, frame or SessionEnd append, exit 0 at once) is the hook carve-out the scope names; no catalogue code reaches hook
input-validation#1.5 · yes · the MCP isError body shape (structuredContent {"error","detail"} with closed detail words, text `refused: <code>`) that the new MCP codes enter, which the census in tests/contract_error_bodies.rs must record beside invalid-params
input-validation#1.6 · no · the MCP stdin line cap and its initialize-cut exit 1 are not a cause the scope names; crates/viola-mcp/src/stdin.rs is not reached and the chunk's exit-1 list has no line-cut entry
input-validation#1.7 · yes · the frame verbs run refuse_untrusted first and refuse with exit 21 or `{"v":1,"error":"instance-unreachable","detail":null}` under --json, with nothing opened and no line; revive's closed --json codes and no-session before any spawn; list, run, verify and mcp home checks before any read or log
input-validation#1.8 · yes · a refused `viola list` reads no config.json, a rule the edited whole-board strict-modes refusal must keep
input-validation#1.9 · no · the Windows persistent-environment name read decides only the R8 keep set; the chunk's run.rs edits (start refusals, checked_exe, collision check) do not touch it, and the Not needed list excludes the R8 strip
input-validation#1.10 · no · standing S7 · test-only features fake-agent and test-support stay out of the release build
input-validation#1.11 · no · standing S7 · test-only features fake-agent and test-support stay out of the release build
input-validation#1.12 · no · the colour decision writes SGR only around escaped list rows, never a refusal, hint or log; style.rs is not reached and the list edit is a refusal document, which takes no colour
input-validation#1.13 · yes · the strict-modes check before every snapshot, events, heartbeat or stamps read; the live_endpoint liveness pre-check and the replayed-snapshot rule (no endpoint, so no frame); run's collision check judging liveness from the heartbeat alone, all of which the edited pre-check and collision check must keep
input-validation#1.14 · no · tailed paths decide no cause (the Not needed list names them) and notify tailing is not reached
input-validation#1.15 · yes · an unwrapped row name is never a ViolaName or target and human list escapes it; the new unwrapped-name hint prints only the typed name, and revive's preflight fails open on a failed agents read
input-validation#1.16 · no · the vt100 feed and PTY output bytes decide no cause or code (Not needed), and crates/viola-pty and gate.rs are not reached
input-validation#1.17 · yes · batch-script-child, the claude-not-on-PATH internal-error and revive's cwd-missing are start causes the table covers, with no path or pid in the document or the hint; the recorded cwd is a spawn directory only, read after the strict-modes check
input-validation#1.18 · yes · revive's no-session comes before any spawn, and its --json output carries session-shaped ids only, with no recorded directory, pid or logged id in any refusal line or hint
input-validation#1.19 · yes · run's statusline source read (statusline_command) sits on the start path, which keeps the strict-modes check ahead of it; the start-refusal edits must not move that read ahead of the check
