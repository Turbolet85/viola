# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · prompt and reply text never reach the pin path or the chunk's new lines; events.ndjson is untouched
threat-model-summary#1.2 · no · tool-call, plan and dialog text reach no pin path or chunk log line; the dialog and event logging is untouched
threat-model-summary#1.3 · yes · the pinned `bin/<version>-<hash>/viola(.exe)`, the plugin's absolute exec commands, snapshot `pinned_bin` and `ledger/stamps.json` are what the absent-copy write, the per-start rewrite and verify's pin_exe change
threat-model-summary#1.4 · no · the chunk changes no environment handling and stores no credential; the R8 strip is the task's Not needed
threat-model-summary#1.5 · yes · the pin-path fault's error body and the new pin log lines must hold no pid or path, as the task's Needs state for a pin fault
threat-model-summary#2.1 · no · the chunk opens no endpoint and changes no listener, peer or client check, which the task's Not needed names
threat-model-summary#3.1 · no · no send, answer or frame path changes, so the paste-text rules are the task's Not needed
threat-model-summary#4.1 · no · the chunk edits no stdin classifier, wheel or pump; the non-editing list is untouched
threat-model-summary#5.1 · yes · the hook's fail-open rule binds: the race fault is a failed pinned spawn, so no fix may add a hook-side refusal (exit 0, no body)
threat-model-summary#6.1 · no · the chunk adds no MCP tool, frame or `viola mcp` check; the task's Not needed names the MCP tools as untouched
threat-model-summary#7.1 · yes · `run`'s start and `verify` hold the home tree to `check_home_tree` before the pin and before any log (exit 1, no log opened), re-asserted unchanged
threat-model-summary#8.1 · yes · the pinned binary and plugin folders are never deleted, truncated or renamed away, and `check_home_tree` and `pin_companions`' check hold the pinned copy and conpty companions before the open
threat-model-summary#9.1 · no · the chunk adds no spawn and changes no program resolution, child PATH or version gate; the hook reaches the pinned copy by its absolute exec command, never a PATH lookup
threat-model-summary#10.1 · yes · the two clean release builds run under the committed `Cargo.lock` and the `rust-toolchain.toml` channel (1.98.1), which the measurement re-asserts
threat-model-summary#11.1 · yes · the release route through `scripts/release-check.sh` (`--release --locked --bin viola`, refusing any test-only binary) is the build the measurement re-reads for profile and target dir
