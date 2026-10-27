# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the session bounds only viola mcp's stdin, and events.ndjson retention and the wait/last payloads are unchanged
threat-model-summary#1.2 · no · the permission, plan and question payloads and the hook.dialog path are unchanged
threat-model-summary#1.3 · no · the config, snapshot, stamps and plugin files keep their contents, since replace_private_shared's callers and write path are unchanged
threat-model-summary#1.4 · no · no credential path or env var changed
threat-model-summary#1.5 · no · the snapshot and budget.json fields listed are unchanged
threat-model-summary#2.1 · no · the IPC methods, endpoint, peer and start-time checks are untouched, and viola mcp's stdin bound is a client reader rather than channel input
threat-model-summary#3.1 · no · the paste rule, validate_paste_text and the typed-text rule are unchanged, and MCP send.text still enters through the same tool
threat-model-summary#4.1 · no · the stdin classifier, the wheel and viola run's stdin path are unchanged, and the new bound sits on viola mcp's stdin only
threat-model-summary#5.1 · no · the hook stdin cap and fail-open behaviour are unchanged
threat-model-summary#6.1 · yes · add-to · F8: the MCP stdin line is capped at MAX_FRAME, so an over-long driver line is refused whole and ends the session, never dispatched in part
threat-model-summary#7.1 · no · no CLI flag, env var or home-resolution rule changed, and the new bound sits on the hidden mcp transport
threat-model-summary#8.1 · no · the readers, OS-permission rules and strict-modes rows are unchanged, and replace_private_shared's only change is its compare read's bound
threat-model-summary#9.1 · no · the pinned-copy placement and re-hash rows are untouched, and only replace_private_shared's compare read is bounded in fs.rs
threat-model-summary#10.1 · no · the child spawn and PATH rules are unchanged, and the agents read's cap is unchanged with the census added as its witness
threat-model-summary#11.1 · no · deny.toml still holds the advisory, licences, sources and feature bans this boundary names, the new serde_json ban being one more feature ban
threat-model-summary#12.1 · no · the CI workflow is not reached, and the new probe sits in scripts/deny-probes.sh, the script behind the entity's ban-probes step
