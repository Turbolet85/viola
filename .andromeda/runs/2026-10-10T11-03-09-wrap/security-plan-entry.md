
## 2026-10-10-self-healing-state — own state files on read: three counts, the appender's heal, the replay as library code
**Section:** §Input Validation (row "Own state files on read") · Threat Model Summary (Filesystem state, Trust boundary)
**Change:**
- "Own state files on read", as landed: the reader counts per read `Skipped{unknown_kinds, unknown_fields, torn_lines}`, not logged: an over-long line, a non-object line and an unterminated last line are `torn_lines`; an unknown kind is `unknown_kinds` and is not returned; a known-kind line with a key outside its contract is returned and counted once. Was `Skipped{oversize, malformed}` with the last line neither returned nor healed and healing owed to "Self-healing state".
- The same row: a reader never writes. The next append heals a torn last line under the lock every append holds, after a one-byte read of the log's end, by one LF ahead of its line in the same single write, and logs one `state-recovered` line holding a detail code, a basename and an offset.
- The same row: `snapshot::read_snapshot_classified` and `viola_state::replay` read through the same bounds and write no file; library code with no caller, so no product reader's handling of an absent, unreadable or newer snapshot changed.
- Threat Model Summary: readers skip and count torn lines and the next append heals; the replay for a snapshot that fails to parse or carries a newer `v` stands as library code that no product reader takes yet (was "Readers tolerate torn lines and replay the log when a snapshot fails to parse").
**Why:** the chunk landed the healing and its count. No boundary widened: the line cap, the `MAX_FRAME` reads and the Integrity part are unchanged, and nothing under `src/` moved.
**Kept:** the row's Line cap sentence and its Integrity part, with every dated exception, stand as written.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/
