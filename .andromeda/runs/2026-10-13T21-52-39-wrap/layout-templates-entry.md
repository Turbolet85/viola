
## 2026-10-13-paste-newline-ledger-row — `viola verify` output at eighteen rows
**Section:** §Output structure — `viola verify`
**Change:** The example block reads `/18`, ending `[17/18] local-command-clear …` and `[18/18] paste-newline an LF inside a paste stays inside the one prompt submitted  pass`, then `stamped 2.1.287  18 pass  0 fail`. The failing example reads `16 pass  2 fail`. The ledger-order list gains `paste-newline`. `MM` was 17; now it is `LedgerRow::ALL.len()`, 18 today.
**Why:** The capability ledger gained its eighteenth row, and `viola verify` prints one step line per row in ledger order. The count now carries its rule.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/
