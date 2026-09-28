# Seam remove-the-guard run (testing.md 2026-09-25)

The guard is the seam's one panicking statement, `src/cmd/hook/seam.rs:19`:
`panic!("{}", "forced-hook-panic ".repeat(256));`. Run 2026-09-28 at implement through
`bash scripts/agent-run.sh run --integration --filter 'test(/hook_forced_panic|hook_panics_append/)'`.

## Neutralised

Line 19 replaced with `let _ = "forced-hook-panic ".repeat(256);` (confirmed by `grep -n` on the file before
the run). Harness exit 1, both cases red:

- `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` FAIL at `tests/hook_fail_open.rs:285`
  (`left: 2`, `right: 1`): with no panic the hook ran `handle`, so the role file held two lines (the ordinary
  `hook-invoked` + `hook-decision`) in place of the one panic line.
- `hook_panics_append_whole_lines_over_4_kib_side_by_side` FAIL at `tests/hook_fail_open.rs:449` (`left: 16`,
  `right: 8`): sixteen ordinary role lines from eight processes in place of eight panic lines.

## Restored

Line 19 restored (confirmed by `grep -n`; `git diff` shows the file as newly added only). Harness exit 0, both cases
PASS (0.129 s and 0.207 s).
