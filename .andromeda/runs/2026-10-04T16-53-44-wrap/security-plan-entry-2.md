
## 2026-10-04-dialog-answers-by-dialog-id — run's strict stamps read; decisions on the six-row stamp until :82
**Section:** Authentication & Authorization → `~/.viola/` access control (the second interim gap) · Input Validation → CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Universal
**Change:**
- The second interim gap was "`viola run`'s version gate and `viola verify`'s `update_stamps` read `ledger/stamps.json` without strict-modes"; now `verify`'s `update_stamps` alone. `run`'s gate reads through `viola_state::stamps::read_stamps_strict` (`strict::check_stamps` first): a strict-modes refusal, an unreadable or a malformed file is one `parse-rejected{parser:"ledger-stamps"}` WARN (detail `strict-modes-failed` on a refusal) and `cli_verified:false`.
- Universal: the interim ban "NEVER answer non-`null` while `run`'s gate still reads stamps without strict-modes … closes before Dialog answers by dialog_id answers any dialog" is now standing: NEVER answer non-`null` unless `run`'s gate read the stamps through the strict check.
- Universal stamp gate: a dated gap until `working-route.md:82` — a stamp holds only the six spine rows (`LedgerRow::ALL`, counter `/06`), no S3 / S7 / S8 / dialog-concurrency row, so a non-`null` decision flows on that six-row stamp whenever `cli_verified` is true; residual: an S3 / S7 / S8 body-shape change in a new CLI is not caught until `:82` lands the rows and their re-probe.
**Why:** the strict read is this chunk's PREREQ before any decision flows (a tightening). The six-row flow is a boundary widening: the operator ratified at this wrap the founder's live ruling R2 (2026-10-04, relayed by the overseer, the residual shown), `:82` the closer.
**Kept:** the R3 creation half (a home outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation) landed exactly as `~/.viola/` access control already words it, so that row's body is unchanged.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
