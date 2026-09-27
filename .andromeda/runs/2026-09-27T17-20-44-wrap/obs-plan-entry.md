
## 2026-09-27-epoch-2-cleanup — pre-push host-scratch counts path-free, host scratch and run archive never uploaded
**Section:** §8 PII Scrubbing, Integration points item 6 (the local pre-push document bullet; the unscanned-uploads `mutants.out/` bullet)
**Change:**
- The pre-push document's `cache` gains `windows_scratch_bytes` / `windows_scratch_bytes_after`: byte counts, never the scratch path (`pre_push_document_carries_no_absolute_path` covers them).
- `mutants.out/` stays never uploaded; on a Windows host it sits in the host mutation scratch outside the repository, whose path no document carries (only `scratch_bytes`). `target/run-archive/<n>/` (each run's JUnit and the `outcomes.json` it read, absolute argv paths included) is never uploaded: gitignored and outside every upload path.
**Why:** the Epoch 2 cleanup chunk added path-bearing content in a new on-disk place; the never-upload ledger names it so a later upload edit cannot pick it up unguarded.
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/
