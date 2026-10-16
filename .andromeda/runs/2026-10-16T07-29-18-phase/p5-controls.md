# P5 controls — 2026-10-14-resume-and-compact-ledger-rows, the plan revision (validation check 4 (9))

Entry 15, the preservation guard, revised to exclude `tests/support/home.rs` (`inputs#I4`). It is the only entry the
revision changes, so it is the only one baseline-run and controlled here.

The baseline was fired on the real tree, which carries the stopped implement run's edits, through `gate.py run --plan
viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md --run-dir .andromeda/runs/2026-10-16T07-29-18-phase
--marker 2026-10-14-resume-and-compact-ledger-rows --entry 15`. The line, as printed:

` 15 probe       green · exit 0 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`

## The clone controls

Each control was fired as the entry: `gate.py run --plan
viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md --entry 15 --project-root {clone}`, with no run dir.

- The clone: `git clone --shared` of the project, checked out at `880b056377f4894669397e086a01c2b5c306f411` (the plan's
  base), in the session scratchpad (`$TMPDIR/claude-1000/…/scratchpad/clone-rev`).
- Each input was made in the clone, read, then undone. The clone read 0 status rows afterwards.
- The lines are copied as printed.

- **Unmodified.** It must read green, as at the take-up's control (empty change):
  ` 15 probe       green · exit 0 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`
- **Must-pass.** The input:
  - `tests/support/home.rs` edited;
  - `crates/viola-agent-claude/src/ledger.rs` edited;
  - the resume variant added, untracked;
  - a chunk `evidence/live-round.md` written;
  - `.andromeda/friction-log.ndjson` appended.

  ` 15 probe       green · exit 0 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`
- **Must-fail.** `src/run/send/slot.rs` edited, beside the must-pass input:
  ` 15 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`
- **Must-fail.** `tests/support/mod.rs` edited, beside the must-pass input. A sibling under `tests/support/` shows the
  exclusion is the one file, not the directory:
  ` 15 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`
- **Must-fail.** An untracked `src/p5_control.rs`, beside the must-pass input:
  ` 15 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 15.2.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (721 chars)`
