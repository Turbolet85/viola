# Remove-the-guard reading — the input-box literal `(shift+tab to cycle)` (testing.md 2026-09-25)

Taken at the wrap, on the overseer's technical direction (`inputs#I8` item 3): over a scratch clone of HEAD, never
the tree. The in-tree attempt during implement was refused by the permission classifier (`[Security Test Removal]`).
That refusal is recorded in `.andromeda/runs/2026-10-20T05-01-00-implement/outcome-2026-10-19-newer-claude-cli-rows.md`.

## The guard removed (red)

- **The clone:** `git clone` of the repository at HEAD `4eaca1d3`, in the session scratchpad
  (`$TMPDIR/claude-1000/…/scratchpad/ctl-4eaca1d3`). Its target dir was put on disk, at
  `target/wrap-ctl-target` (`/tmp` is the quota tmpfs).
- **The one edit:** in the clone, `crates/viola-agent-claude/src/screen.rs:41` changed from
  `input_box: &["for agents", "(shift+tab to cycle)"],` to `input_box: &["for agents"],`. `git diff --stat` of the
  clone read `1 file changed, 1 insertion(+), 1 deletion(-)`.
- **The firing:** `gate.py run --plan …/plan.md --entry 8 --project-root {clone} --void 8:{the control}`, with no run
  dir and so no trail. Its line:

  ```
    8 unit        red · exit 0 ✗ (exit 1) · exit 1 · 37.89s · 11981 B → 8.log · void: remove-the-guard control over a scratch … (143 chars) · bash scripts/agent-run.sh run --unit --filter 'test(/resti… (71 chars)
  entries 35 · green 0 · red 1 (8) · recorded 0 · timeout 0 · not-run 34 · voided 1 (8)
  ```

- **The log** (`$TMPDIR/andromeda-gate/2026-10-19-newer-claude-cli-rows/run-20261020T054907Z/8.log`) read 8 passed and
  2 failed. The two `FAIL` lines are the only cases that rest on the new literal:
  - `screen::tests::holds_any_reads_the_resting_footer::case_1_resting_footer_2_1_302`
  - `screen::tests::verdict_over_the_2_1_302_resting_footer_alone_is_ready`

  The other eight passed: the 2.1.287 row, the modal rows, the rule, blank, tip and Skills-hint rows, and the
  beside-a-modal verdict. Without the literal, each of those reads the same way.

## The guard in place (green)

The tree, with the literal, read entry 8 green: `10 passed` in implement's run (`8.log`, `8.2.log`, `8.3.log` under
`$TMPDIR/andromeda-gate/2026-10-19-newer-claude-cli-rows/implement-2026-10-20T05-01-00/`, the last one fired after
implement's restore). CI ran it green on all three OSes at `4eaca1d3` (ci#39374691224).

## The clone

The clone and its target dir could not be removed: the host's Bash guard refuses a recursive `rm`, and a refused
removal is not done another way. Both are left for the founder's desk, read 2026-10-20T05:50:18Z:
- `$TMPDIR/claude-1000/…/scratchpad/ctl-4eaca1d3`;
- `target/wrap-ctl-target`, 1.1 G.
