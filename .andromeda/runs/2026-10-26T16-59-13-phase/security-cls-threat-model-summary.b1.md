# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · user content and growth of events.ndjson; the chunk reaches the file only through check_instance's mode and DACL check (Needs), and retention is Not needed
threat-model-summary#1.2 · no · dialog and plan content sent over hook.dialog; the chunk changes only the snapshot read before that frame, and the destination check is #2.1's rule
threat-model-summary#1.3 · yes · the pinned bin/<version>-<hash>/viola, plugin/ hooks and .mcp.json, marketplace/ .mcp.json, snapshot.json and ledger/stamps.json are the code-bearing and integrity-sensitive files that pin_exe, the plugin rewrite, the marketplace check and the single writers guard
threat-model-summary#1.4 · no · standing S3 · each new log or diagnostics line keeps the NEVER-log floor, a convention the task lists
threat-model-summary#1.5 · no · standing S2 · each new refusal or error carries codes and fixed messages only, with no pid, start time or absolute path, a convention the task lists
threat-model-summary#2.1 · yes · the frame writers' server check runs on a snapshot read without strict-modes for hook.event, hook.dialog, the CLI frame verbs and viola mcp's frames, the gap the chunk closes
threat-model-summary#3.1 · no · the chunk changes nothing typed into the PTY (paste-text validation and the typed-text rewrite are Not needed), and send.rs gains only checks before its frame
threat-model-summary#4.1 · no · the human's stdin classifier and the wheel's typing rules are Not needed, and the chunk changes no keystroke path
threat-model-summary#5.1 · yes · hook.rs's snapshot read through VIOLA_DIR gets strict-modes before it, hook statusline's instance check precedes any home read, and every hook failure fails open with exit 0 and no body
threat-model-summary#6.1 · yes · tools.rs's frame tools read the snapshot through live_endpoint after the strict-modes check, and the ViolaName input check precedes any instances/<name> join
threat-model-summary#7.1 · yes · --home and VIOLA_DIR choose the home the strict-modes check guards, so home resolution and the 0770 --home negative (exit 21) in cli_controls_not_disableable.rs bind
threat-model-summary#8.1 · yes · the home's OS-permission boundary and its code-bearing bin/, plugin/ and marketplace/ files, which the chunk checks and rewrites, are what the chunk makes whole
threat-model-summary#9.1 · yes · run puts the pinned bin/<version>-<hash>/ folder first on the child's PATH and runs the child for its --version gate, so pin_exe's re-hash and run's start checks guard what children resolve; revive's and list's spawns are not reached
threat-model-summary#10.1 · no · standing S6 · each new dependency or feature passes cargo deny check, the sync-crate tokio ban and the C-build ban, a convention the task lists
threat-model-summary#11.1 · yes · the windows-2025 proof of check_instance's DACL leg and the two-build release measurement through scripts/release-check.sh's rules are CI work this chunk adds or re-measures
