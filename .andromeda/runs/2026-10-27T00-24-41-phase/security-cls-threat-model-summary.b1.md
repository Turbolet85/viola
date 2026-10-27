# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the chunk's units never write or read the prompt-submitted or turn-ended lines of events.ndjson, and run.rs's pump is a re-check, not an edited unit
threat-model-summary#1.2 · no · the permission, plan and dialog payloads and hook.dialog sit outside the reached units, and dialog and frame validation are Not-needed
threat-model-summary#1.3 · yes · names the pinned bin/<version>-<hash>/viola(.exe), the plugin hooks.json and .mcp.json exec paths, snapshot pinned_bin and ledger/stamps.json, each a Need the absent-copy write, the per-start rewrite or verify's pin_exe must hold
threat-model-summary#1.4 · no · the R8 strip and the inherited CLAUDE* credentials are Not-needed, with no environment handling change
threat-model-summary#1.5 · yes · the 0600 snapshot's mode runs through the replace_private change, and pid, cwd and OS username must stay off the pin fault's error, the new pin log lines and the measurement records (Needs)
threat-model-summary#2.1 · no · IPC listener hardening and client server verification are Not-needed, and the chunk opens no endpoint
threat-model-summary#3.1 · no · the send paste path is Not-needed, with no send path change
threat-model-summary#4.1 · no · the stdin classifier and the wheel are unedited units, and the start's pin wait reads no keystroke
threat-model-summary#5.1 · yes · the hook's fail-open (exit 0, no body) is the Need the race fault rests on: a failed pinned spawn, never a hook-side refusal
threat-model-summary#6.1 · no · the viola mcp tools and their strict-modes checks are untouched, and the .mcp.json exec path is carried by #1.3
threat-model-summary#7.1 · yes · check_home_tree before any read or open for run, revive's start and verify, which the pin runs after, and --home or VIOLA_DIR choose the tree it writes under
threat-model-summary#8.1 · yes · the pinned binary and plugin folders' integrity: check_home_tree over bin/<version>-<hash>/ and plugin/, pin_companions' check before the held open, and the never-deleted folders
threat-model-summary#9.1 · no · the chunk changes no child argv, PATH or spawn: list, mcp, revive's launch, verify's five runs and run's version gate are unedited
threat-model-summary#10.1 · yes · the two clean builds run under the rust-toolchain.toml channel (1.98.1) and the committed Cargo.lock the entity states, while its SHA-pinned actions and cargo deny stay CI's and untouched
threat-model-summary#11.1 · yes · the per-OS release build through scripts/release-check.sh (--locked, test-only binaries refused) is the route the two clean builds re-read, while the other CI jobs stay untouched
