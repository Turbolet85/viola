
## 2026-10-13-paste-newline-ledger-row — `verify` step lines at eighteen rows
**Section:** §Surface: cli, Component Patterns 5 (`verify`)
**Change:** Was `[01/17]` and seventeen rows; now `[01/18]` and eighteen rows, the members of `LedgerRow::ALL`. The last eight are the dialog rows, the framing rows and the newline row `paste-newline` (an LF inside a paste stays inside the one prompt submitted). The summary reads `stamped 2.1.287  18 pass  0 fail`.
**Why:** The capability ledger gained its eighteenth row (`paste-newline`), and the step counter derives from `LedgerRow::ALL.len()`. The count now carries its rule.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/
