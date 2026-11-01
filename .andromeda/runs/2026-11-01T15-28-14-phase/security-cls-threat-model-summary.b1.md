# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · events.ndjson in the instance directory, whose no-group-or-other read rule (check_instance) this chunk applies
threat-model-summary#1.2 · yes · the same instance-directory file, holding tool-call input and dialog text, is protected by the instance read rule this chunk applies
threat-model-summary#1.3 · yes · config, settings, snapshot, stamps and plugin and bin artefacts under the home are governed by the home write rule (check_path) and the strict-modes checks this chunk applies
threat-model-summary#1.4 · no · transient CLAUDE* environment and subscription credentials, never stored; the chunk changes no child environment (R8 strip excluded)
threat-model-summary#1.5 · yes · snapshot pid, child_pid and cwd must reach no error body, so the new refusal's external text holds no pid, path or uid
threat-model-summary#2.1 · yes · the socket endpoint, squatting, the /tmp fallback and the macOS pid gap are the chunk's core: socket directory, peer euid and the carve-out
threat-model-summary#3.1 · no · CLI send text written into the PTY; validate_paste_text and the send free-text set are excluded and the chunk changes no typed text
threat-model-summary#4.1 · no · human terminal stdin and the typing classifier; excluded, no PTY or terminal path moves
threat-model-summary#5.1 · yes · every hook path runs check_instance before any home read and fails open to exit 0, which is how this chunk's refusals reach hook
threat-model-summary#6.1 · yes · MCP send, wait, last and answer run check_instance before the frame and the server check, and the new refusals take the isError instance-unreachable shape
threat-model-summary#7.1 · no · MCP stdin line cap at MAX_FRAME; excluded, the chunk changes no MCP transport
threat-model-summary#8.1 · yes · --home and VIOLA_DIR choose the tree that the home strict-modes check and the exit 21 frame-verb refusal govern; no VIOLA_* variable may reach the new checks
threat-model-summary#9.1 · yes · check_home_tree and the replay readers, including viola list's board::read, sit under the home check this chunk applies to board::scan
threat-model-summary#10.1 · no · pinned-binary placement and ETXTBSY; the chunk touches no placement of the pinned binary
threat-model-summary#11.1 · no · child spawns of claude agents and PATH resolution; the chunk changes no child spawn (excluded)
threat-model-summary#12.1 · no · standing S7 · dependency-security: a dependency a change adds keeps the workspace pin, the licence list and the bans
threat-model-summary#13.1 · yes · the per-OS fake-agent test matrix on ubuntu, macOS and Windows carries the chunk's macOS exit-cause case and the Linux and macOS parity suite
