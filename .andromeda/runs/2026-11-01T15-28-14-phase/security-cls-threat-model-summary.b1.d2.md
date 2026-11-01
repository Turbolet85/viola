# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · describes what events.ndjson and the replies hold; the chunk's checks, refusals and tests state no rule over that content
threat-model-summary#1.2 · no · describes tool-call arguments, plans and dialog text in events.ndjson and hook.dialog; no rule the chunk's code or tests must honour
threat-model-summary#1.3 · yes · the integrity of config.json, settings.json, snapshot.json (endpoint, statusline_command), stamps, hooks.json, .mcp.json and bin is what the Unix home strict-modes check (check_path, check_instance, board::scan) must keep guarding before any read
threat-model-summary#1.4 · no · a transient credential in the wrapper's own environment; the chunk changes no child environment (the R8 strip is Not needed) and stores no credential
threat-model-summary#1.5 · yes · pid and cwd stay out of log lines, error bodies and stdout, so the new refusal text and any new log line must hold no pid, uid or path
threat-model-summary#2.1 · yes · the endpoint trust boundary: the socket-directory, peer euid and server-verify checks, the macOS pid gap, and the instance-unreachable causes the new refusals extend
threat-model-summary#3.1 · no · the send paste write and its empty-text refusal; the chunk changes no typed text (validate_paste_text is Not needed) and its pre-frame check alters no text
threat-model-summary#4.1 · no · the stdin wheel and typing classifier, which are Not needed; the run refusal precedes any stdin read and 4.1 states no rule over it
threat-model-summary#5.1 · yes · every hook path runs the strict-modes check before any read and fails open with exit 0 and no body, which the chunk's directory and peer refusals must reach only as a fail-open
threat-model-summary#6.1 · yes · each MCP send, wait, last and answer runs check_instance before its frame, with the chunk's peer and directory checks after it, and a refusal sends no frame and writes no line
threat-model-summary#7.1 · no · the MCP stdin line cap at MAX_FRAME; the chunk changes no tool, transport or stdin line reader (Not needed)
threat-model-summary#8.1 · yes · the frame verbs' check_instance refusal on exit 21 and run, revive and verify's check_home_tree refusal, with --home and VIOLA_DIR unable to switch off the new checks
threat-model-summary#9.1 · yes · viola list's board::read and the other readers run the strict-modes check before their replay, which the chunk's board::scan home rule must keep as one rule
threat-model-summary#10.1 · no · the pinned binary's first placement by concurrent starts (place_private_new); the chunk changes no placement and rewrites no code-bearing artefact
threat-model-summary#11.1 · no · the claude agents spawn after board::read and its agents-probe logging; the chunk edits only board::scan's home check and leaves the spawn and its logging unchanged
threat-model-summary#12.1 · no · standing S7 · A dependency a change adds keeps the workspace pin, the licence list and the bans, tokio in the sync crates included
threat-model-summary#13.1 · yes · the per-OS ci.yml test matrix runs the same end-to-end suite on Linux and macOS, which the chunk's macOS exit-cause case and Linux parity legs run through
