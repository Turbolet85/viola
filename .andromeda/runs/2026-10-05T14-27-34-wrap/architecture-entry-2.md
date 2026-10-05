
## 2026-10-05-dialog-rows-and-re-probe — registry rows for Runs C and D, the dialog variants and the answers flag
**Section:** §Standard Contracts → Ledger stamps envelope · §Occupied Resources (Binary · Claude Code integration names · `diagnostics/` · `ledger/stamps.json` · `ledger/probes/<pid>/` · probe dirs · Repository) · §Infrastructure Patterns → CI/CD approach · Project directory structure · Crate dependency direction
**Change:**
- Envelope and `ledger/stamps.json`: `measured` also holds `dialog_probe {parallel_both_before_first_post}` (bool or null, additive, no `v` bump); `run` reads no field of it.
- `diagnostics/`: six spawn pairs (was four), `verify-pty-probe` ×4.
- `ledger/probes/<pid>/`: adds `questions/` and `plan/` roots, each with `plugin/`, `captures/`, `answers/`.
- Probe dirs: four (was two), adding `-dialogs/` and `-plan/` (with `plans/`).
- Binary: hidden `--answers <DIR>` beside `--capture` (was "empty stdout"); the fake agent's argv options three → five (`--dialogs`, `--stop-receipt-hold-ms`, capped at 1 000 ms).
- Integration names: the dialog-kind probe plugin (PreToolUse matcher `AskUserQuestion|ExitPlanMode`, PermissionRequest, PostToolUse, `--answers`); four interactive children with their flags.
- Repository: the `<Event>.<stem>-<n>.json` variant class (12 per version), 2.1.287 / 2.1.288 stamped at fourteen rows (was ten); the relayed set superseded and kept, `RELAYED.md` dated "Superseded".
- Contracts: CI `--dialogs`, four typed runs, fourteen rows; the tree's `typed.rs` and fixtures comments; the ledger module lists fourteen rows and `probe_body` / `ProbeAnswer` / `dialog_variants`.
**Why:** the chunk landed these resources; the answers flag and Runs C/D are the founder's live rulings of 2026-10-05 (M7 = A, STOP 7), relayed by the overseer and ratified by the operator at this wrap as his.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
