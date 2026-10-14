
## 2026-10-13-owed-ledger-rows — verify's step counter at the relied count
**Section:** §Surface: cli → Component Patterns, pattern 5 (`verify`)
**Change:**
- Was one step line per ledger row, eighteen today, the summary `stamped 2.1.287  18 pass  0 fail`.
- Now one step line per relied ledger row: `MM` is `LedgerRow::relied()`'s count, 21 on Unix and 18 on Windows, where the status line is not wrapped.
- The rows named after `paste-newline` are `statusline-override` · `statusline-shell` · `statusline-rate-limits`, each with its words.
- The summary's n + m equals `MM`: `stamped 2.1.287  21 pass  0 fail` on Unix.
**Why:** the three statusline rows landed relied only where the status line is wrapped (`statusline::WRAPPED_HERE`), the cut the founder chose (`inputs#I1`). The count carries its rule (`relied()`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
