## 2026-10-04-confirmed-send-with-cl-1-records — fuzz targets, the chaos test home, the send layout
**Section:** §Occupied Resources (Filesystem: a new test-only entry; Repository: `fuzz/`, `target/e2e-home/…`) · §Infrastructure Patterns → `project-directory-structure` · `crate-dependency-direction`
**Change:**
- `fuzz/`: targets 4 → 5 (`paste_text`); its corpus holds 8 seeds kept byte-exact by `.gitattributes` `fuzz/corpus/paste_text/** binary`.
- Filesystem gains the test-only `<temp dir>/viola-chaos-*/home` (`TestHome::outside_scan()`, `tests/chaos_feed_panic.rs` only), outside `target/e2e-home` and so outside G2, G4 and the secret scan; the test asserts its own panic and `parse-rejected` lines. `target/e2e-home` was "every harness and test home"; now every one but that home.
- The directory contract: `src/run/send.rs` (the `send` method, one in flight, the relabel), `gate.rs` (bounded feed + `Gate`), `src/human.rs` callers `run` · `verify` · `send` (the readback mirror), viola-core's `NotDelivered` + `validate_paste_text`, viola-pty's `PasteHandle`, `fuzz_targets/{…,paste_text}.rs`.
- `viola-core` dependencies were nutype alone; now nutype and serde (`derive`), dev serde_json.
**Why:** the chaos home is the founder's ruling (live, 2026-10-04, relayed by the overseer): the second named test-data carve-out beside `seed_conpty`, all three scans named. The rest records what the chunk landed.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
