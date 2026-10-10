
## 2026-10-10-statusline-pass-through — counts, crate edges and one order finding: the fake agent, the seeds, the snapshot readers, revive's log before its check
**Section:** §Occupied Resources (Binary and test-only binaries; Repository: `fuzz/`, proptest regressions; Filesystem: the instance files) · §Standard Contracts (Snapshot envelope) · §Conventions (Data model conventions, Percentages) · §Infrastructure Patterns → Crate dependency direction · §Infrastructure Patterns → Project directory structure
**Change:**
- `viola-fake-agent` has twelve argv options (was ten, per the 2026-10-10-viola-revive entry on the root wait count and the ten fake-agent options; the count's rule is architecture's own list): `--settings <file>` and `--statusline-stdin <file>` serve the statusline tests, and it takes one mode word, `statusline-echo`. It files a `statusline` receipt.
- `fuzz/corpus/hook_stdin` has 12 seeds (was 10; rule `ls fuzz/corpus/hook_stdin | wc -l`), and that target also feeds the statusline payload reader. `crates/viola-agent-claude/proptest-regressions/` holds `statusline.txt` beside `hook.txt`.
- `read_snapshot` has five product callers (was four): `src/cmd/hook/statusline.rs` joins. The rule: `read_snapshot(` under `src/`, outside test modules.
- Percentages: an external budget reading's `used_percentage` is a `Reading<f64>` in `viola_core::BudgetWindow`, kept from 0 to 100 and otherwise `"unknown"`; the `Percent` newtype stays viola's own.
- Crate dependency direction: `viola-agent-claude` names `chrono` (the workspace pin, features `clock` and `std`) for its pure `statusline` module; it gained no `viola-state` edge.
- Project directory structure: `src/cmd/hook/statusline.rs`, `viola-state`'s `budget.rs`, `viola-core`'s budget reading types and the agent crate's `statusline` module are named.
- The instance files: `revive`'s start arm opens its wrapper log before its instance check, and opening a log sets an existing home to 0700, so on `revive` the check cannot refuse a home for a group- or other-writable mode. Written as read in source and not measured on `revive`.
**Why:** each count moved with the chunk's code. The order on `revive` was found while measuring the same mechanism on the statusline arm; the fix is owed to the working-route entry "Budget governor" (the operator's direction at this wrap, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/
