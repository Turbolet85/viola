# Classified — architecture · established-decisions.b3

## Answers
established-decisions#23.1 · no · the ledger's rows and stamps are unchanged (report: no ledger row added or re-stamped) and its code in viola-agent-claude is not in the reached files
established-decisions#24.1 · no · verify's home-tree check and the stamps writer are not in the reached files, and the one changed helper is called only by the pin writers (report)
established-decisions#25.1 · no · the start order, heartbeat and revive paths are untouched (report: no session-liveness code path changed) and none sit in a reached file
established-decisions#25.2 · no · the server-identity and liveness checks are untouched (report), and the reached viola-mcp change is the stdin reader and its cut only
established-decisions#25.3 · no · the collision check's snapshot read goes through a replay reader the report shows unchanged
established-decisions#25.4 · no · revive's single snapshot read and its state-recovered line are unchanged, and the one reached schema change adds only the mcp-stdin parser value
established-decisions#25.5 · no · the start's home-tree check sits in strict.rs, which is not reached, and the report shows no session-liveness code path changed
established-decisions#26.1 · no · ServeError is unchanged in the reached lib.rs and the three open divergences, viola-mcp's ServeError beside McpError included, stand as written (report)
established-decisions#27.1 · no · no crate was added or removed, viola-mcp gains only a crate-private module, and Tokio containment holds (report)
established-decisions#28.1 · yes · stale · the session changed root tests that the seeding claim covers (tests/chaos_feed_panic.rs, new tests/contract_bounded_readers.rs), and the report does not show that the seeding still holds on them
established-decisions#29.1 · no · pin_key, pin_bytes and pin_exe are unchanged, and the one changed helper leaves the pin writers' outcome the same (report: callers unchanged)
established-decisions#29.2 · no · the check before the held open and the refusal that writes nothing both hold, since the changed helper's outcome is the same and pin_companions is an unchanged caller (report)
established-decisions#29.3 · no · the two-builds measurement concerns the build process, and the report shows no dependency or Cargo.lock change
