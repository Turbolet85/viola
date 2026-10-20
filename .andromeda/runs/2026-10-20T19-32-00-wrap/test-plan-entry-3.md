
## 2026-10-20-epoch-5-cleanup-ii — `prepare` killed on every host, the Epoch 5 survivors disposed
**Section:** §10 Quality Gates & Coverage Targets (Mutation gate paragraph)
**Change:**
- `prepare` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`), so both arms are reached on every host, and its `Ok(None)` and deleted-`!` mutants read killed on the Linux dev host by its scratch case, as measured at the chunk's survivors ledger. Was: "reachable only on a Windows host, and are recorded not measured here … On the Linux dev host they still read missed". The Epoch 4 note reads the 2 `prepare` rows "stood as recorded (killed on every host since)".
- The Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement, each red under its hand-applied mutant and green reverted byte-exact, with no mutation run made and none equivalent or recorded.
**Why:** item (4) closes the audit's survivors so the next audit reads them closed.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/
