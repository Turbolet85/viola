# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · a re-check every change owes: the verbatim wait/last output and the events.ndjson content keep holding, and its retention is Not-needed
threat-model-summary#1.2 · no · dialog content and its hook.dialog carriage keep holding (a re-check), answer free text is Not-needed, and the chunk changes only the snapshot read before that frame
threat-model-summary#1.3 · yes · the Needs name snapshot.json's endpoint, pinned_bin and statusline_command, ledger/stamps.json, plugin/ hooks and .mcp.json, marketplace/.mcp.json, the pinned bin/ exe and the settings.json override
threat-model-summary#1.4 · no · the R8 strip and the transient credential are Not-needed, and the chunk stores and logs no environment value
threat-model-summary#1.5 · yes · the Needs' refusal rule keeps the snapshot's pid, start time and cwd off every line and output, and the 0600 snapshot sits under the owner-only check
threat-model-summary#2.1 · yes · the snapshot read before each frame is the gap the chunk closes: strict-modes before the server-verified hook.event, hook.dialog, CLI frame verbs and viola mcp frames
threat-model-summary#3.1 · no · Not-needed: paste-text validation, empty-text and the typed-text rewrite, since the chunk changes nothing typed into the PTY
threat-model-summary#4.1 · no · Not-needed: the stdin classifier and the wheel's typing rules, since no keystroke path changes
threat-model-summary#5.1 · yes · the nine hook events and hook statusline read the snapshot through VIOLA_DIR, so the check precedes that read, the fail-open (exit 0, no body) binds, and statusline_command never runs from a home or instance directory that fails strict-modes
threat-model-summary#6.1 · yes · viola-mcp's frame tools (tools.rs edited) read live_endpoint behind the check and return instance-unreachable with detail null, and names pass ViolaName::try_new before any instances join
threat-model-summary#7.1 · yes · --home and VIOLA_DIR pick the home the checks guard (canonical_home, the VIOLA_NAME/VIOLA_DIR pair), and no config key, VIOLA_* variable or flag may switch the checks off
threat-model-summary#8.1 · yes · the OS-permission boundary on the home is what the strict-modes and DACL legs, owner-only creation and the bin/, plugin/ and marketplace/ integrity rules implement
threat-model-summary#9.1 · yes · run's start and pin_and_plugin (edited) resolve the child, run its --version and put the pinned bin folder first on PATH; the agents and verify spawns are Not-needed or unchanged
threat-model-summary#10.1 · yes · the windows-sys feature a new ACL call may add, and viola_state's no-tokio and no-C-build graph, run through cargo deny and the sole-root tokio ban; the fuzz exemption and nightly runs are Not-needed
threat-model-summary#11.1 · yes · the chunk's Windows DACL proof runs on CI's windows-2025, and its release-build measurement runs through scripts/release-check.sh in the per-OS release job
