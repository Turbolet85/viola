
## 2026-10-02-epoch-2b-cleanup — test-data cleanup: owner record last
**Section:** §3 → Test data bootstrap (Cleanup)
**Change:** A root `TestHome` removes its home on drop through `tests/support/home.rs` `remove_owned`, which deletes `owner.json` last; the gone-owner sweep removes through the same function. Was "`TempDir` drop removes per-test homes". The keep path is stated as behaviour: a failing root test keeps its home only under `AGENT_RUN_KEEP_FAILED=1`, and CI's `AGENT_RUN_KEEP_HOMES=1` keeps every rstest home (was "calls `TempDir::keep()`"). Added: ownerless remnants still appear on runs failing at the D: dev volume's deadlines (0–3 per run, none on a C: copy), mechanism recorded, not established (M2, open).
**Why:** std's `remove_dir_all` stops at the first undeletable entry in listing order, so the record must go last for a part-way removal to stay reclaimable. The planned git-fixture read-only rule was not added: its premise was measured false (std deletes read-only files).
**Ref:** .andromeda/runs/2026-10-03T07-46-03-wrap/
