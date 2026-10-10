
## 2026-10-10-self-healing-state — the snapshot read and the replay as landed; Scenario E5 owed; the crate-level suite named; the root waits at 23 in 17
**Section:** §1 Test Scope Summary (Coverage triggers, chaos-test) · §4 Unit Test Strategy (What unit tests cover, viola-state) · §5 Integration Test Strategy (Cross-module patterns, On-disk) · §6 E2E Test Strategy (Scenario E5) · §3 → 5-command implementation
**Change:**
- §4 viola-state: the classified read tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first; an unsupported `v` or a parse failure goes to replay through the read-or-replay function, one `state-recovered` line each and none for a present or absent snapshot (`snapshot_cause`, `replay_recovers`); library code, no product reader yet. Replay: a field no line gave is absent, `links` replays empty until "Session links", no file is written.
- §1 chaos-test trigger, snapshot bullet: the same limits (`links` empty until "Session links", no file written, the reader owed first to "viola revive").
- §6 Scenario E5: owed, not built. It needs `link` ("Session links"), a budget pause ("Budget governor"), `list --json` ("The board: viola list") and a reader of the replay (first "viola revive"). Two signals are set against what landed, for the building entry to restate: every start appends `wheel{driver, start}`, so `wheel:"human"` does not hold after step 3; the replay writes no file, so no snapshot is rebuilt from it. The scenario's own lines are unchanged.
- §5 On-disk: `crates/viola-state/tests/` is two files, `state_events.rs` and `state_replay.rs`, with their cases named; the root `path2` and two `path4` cases read a product-written log back with zero on all three counts.
- §3 → 5-command implementation: `Instant::now() + WITHIN` reads 23 sites in 17 files (was 22 in 16).
**Why:** the chunk landed the replay below any surface and measured the second start's first record. The operator approved E5 as owed at the P5 review.
**Kept:** E5's steps and verification lines stand as written under the new as-landed bullet.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/
