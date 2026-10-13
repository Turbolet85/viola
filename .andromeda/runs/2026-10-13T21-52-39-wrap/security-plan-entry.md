
## 2026-10-13-paste-newline-ledger-row — Run B's fifth compiled paste, eighteen stamp rows, the shell's row owed on "Owed ledger rows"
**Section:** §Threat Model Summary (Child process spawning, `viola verify`'s interactive children) · §Security Anti-Patterns → Universal (the stamp) · §Input Validation (Statusline shell-out row)
**Change:**
- Run B: was four bracketed pastes; now five compiled literals. `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, one inner LF, no CR) goes before `/clear`. Was three added pastes; now four, each only into rows holding the input-box literal and no modal literal, none after the stop. After the long, tag-like and newline turns Run B waits for those rows. No start, spawn pair or transcript is added.
- The stamp: was seventeen rows, `/17`; now eighteen (`LedgerRow::ALL`, counter `/18`), the newline row `paste-newline` after the three framing rows. The dialog-less home reads `14 pass  4 fail`. A stamp holding exactly the fourteen, or exactly the seventeen ids of an older stamp, all `pass`, records `cli_verified:false`.
- Statusline shell-out: the shell's row was owed to "Paste newline ledger row"; now to "Owed ledger rows".
**Why:** The fifth paste widens the ratified verify child set. Ratified by the founder's own answer to P4 fork 1, relayed verbatim by the operator (the chunk's `inputs#I2`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The earlier three added pastes stay his 2026-10-06 answer. The owed row's move is his own answer to fork 3 (`inputs#I2`).
**Kept:** §Data Protection's Run B transcript count (two per real-CLI verify) stands: the newline paste opens no session.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/
