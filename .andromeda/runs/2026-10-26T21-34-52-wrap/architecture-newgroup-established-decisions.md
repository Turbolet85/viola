# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade-g1.md 3 · contradiction · lines 79–79 · «every reader stays on `read_snapshot` (§Standard Contracts, Snapshot envelope), and the readers' move onto `read_snapshot_or_replay` is owed» · the standing line says no reader has moved onto the replay read and the move is still owed to this very entry, while the proposal states what the moved readers (`live_endpoint`, hook `connect`, `hook statusline`'s `serve`) do with a replayed snapshot
- fanout-architecture--cascade-g1.md 6 · distinct · no standing group states the hook event and dialog paths' `check_instance` before obs init (line 116 holds only the absent-`VIOLA_NAME` exit, line 110 only the shared connect)
- fanout-architecture--cascade-g1.md 9 · distinct · no standing group states that revive's preflight takes one snapshot read for its collision check and recorded-directory reading (lines 203–204 name the readings and the twice-run collision check, not the shared read)
- fanout-architecture--cascade-g1.md 15 · distinct · no standing group names `pin_key` / `pin_bytes` or the start pinning the bytes its home-tree check hashed (line 225 states only copy-if-absent and re-hash-before-reuse)
- fanout-architecture--cascade-g1.md 16 · distinct · no standing group states the companions' owner/DACL `check_present` leg or a `strict-modes-failed` fallback (line 225 states the held `FILE_SHARE_READ` open and full re-hash; line 205's `run.conpty_sideload` outcome list does not name the new value)
- cascade-compare-architecture.md 2 · twin · fanout-architecture--cascade-g1.md 6 · both state the hook event and dialog paths' `check_instance` before the diagnostics level and obs init, failing open with nothing read or written, the `--capture` arms ahead of it
- cascade-compare-architecture.md 3 · distinct · the [MCP] paragraph (lines 85–92) states no instance strict-modes check before `live_endpoint` for the frame tools
- cascade-compare-architecture.md 4 · distinct · no standing group states verify's `check_home_tree` before its `cli` log or `update_stamps`' `check_stamps` (line 194 names only "the one writer")
