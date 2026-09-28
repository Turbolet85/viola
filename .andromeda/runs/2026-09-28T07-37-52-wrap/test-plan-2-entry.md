## 2026-09-28-hook-perf-gate — forced panic on the real binary, the over-4 KiB concurrent half, the controls table's interim shape
**Section:** §1 Test Scope Summary (hook stdin); §5 Module ↔ DB concurrent-append check, CLI `cli_controls_not_disableable.rs`; §6 Security sweep (the fail-open matrix and its summary)
**Change:**
- `hook_fail_open.rs` gains `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` and `hook_panics_append_whole_lines_over_4_kib_side_by_side` (was "11 cases and no forced panic", the forced panic pending); the seam `src/cmd/hook/seam.rs` carries its Decisions Log entry and is a row of the controls table.
- The concurrent-append check's over-4 KiB half landed: 8 forced panics, 8 whole role lines and 8 whole detail lines over 4 096 B, 3 OSes.
- `cli_controls_not_disableable.rs` as landed: the `FAKE_AGENT_HOOK_PANIC` rows `0` · `false` · `off` · empty × the oversize-stdin and malformed-json refusals, no panic line; the verb negatives and the completeness case join with `send` / `answer`.
**Why:** measured green at implement and in CI `test` on three OSes (ci#36390764600); the forced-panic case took 0.414 s on the Windows debug build; operator P4 fork 3 set the interim shape.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
