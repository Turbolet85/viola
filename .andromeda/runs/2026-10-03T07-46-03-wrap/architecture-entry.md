
## 2026-10-02-epoch-2b-cleanup — test homes removed with their owner record last
**Section:** §Occupied Resources → Repository (`target/e2e-home/` entry)
**Change:** The `viola-test-*` entry now adds: both removals, the test home's own drop and the gone-owner sweep, go through `remove_owned` (`tests/support/home.rs`), which deletes `owner.json` last, so a removal that stops part-way keeps the record for a later sweep. Was: the entry named only the sweep's owner rule; the drop was a `TempDir` drop.
**Why:** std's `remove_dir_all` stops at the first entry it cannot delete, in listing order, so a scratch entry sorted after `owner.json` let a plain removal take the record first and leave an ownerless dir no sweep may take. The owner-record rule itself is unchanged (pid + start time only).
**Kept:** the ownerless remnants that D:'s deadline-failing runs leave are not closed by this; their mechanism is not established and M2 owns them.
**Ref:** .andromeda/runs/2026-10-03T07-46-03-wrap/
