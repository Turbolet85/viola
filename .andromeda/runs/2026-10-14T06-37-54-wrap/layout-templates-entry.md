
## 2026-10-13-owed-ledger-rows — the `viola verify` output at the relied count
**Section:** §Output structure — `viola verify`
**Change:**
- The worked example is a Unix host at twenty-one relied rows: `[01/21]` … `[18/21] paste-newline`, then `[19/21] statusline-override`, `[20/21] statusline-shell`, `[21/21] statusline-rate-limits`, and `stamped 2.1.287  21 pass  0 fail`. The failing example is `19 pass  2 fail`.
- The usage line lists the three ids after `paste-newline`. `MM` is the relied count, `LedgerRow::relied()`'s length: 21 on Unix, 18 on Windows, which stops at `paste-newline`. It was `LedgerRow::ALL.len()`, 18.
- One step line per relied row; exit 0 when every relied row passes.
**Why:** the three statusline rows are relied only where the status line is wrapped (the founder's cut, `inputs#I1`). The count carries its rule (`relied()`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
