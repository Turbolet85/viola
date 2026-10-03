
## 2026-10-02-epoch-2b-cleanup — CI keeps every home: deleter renamed
**Section:** §9 CI Integration (homes kept until the gate steps run)
**Change:** "neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads". Was "an rstest `TempDir` drop": a root test home now drops through `remove_owned`. The behaviour is unchanged: CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home.
**Why:** a cross-master citation of test-plan §3's test-data cleanup, which changed its removal mechanism this chunk.
**Ref:** .andromeda/runs/2026-10-03T07-46-03-wrap/
