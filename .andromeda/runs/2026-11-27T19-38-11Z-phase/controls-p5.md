# P5 controls — 2026-11-27-epoch-5-cleanup (check 4 (9) must-pass readings)

Clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/9ef63c91-382d-4fd3-9e83-88fd34b22892/scratchpad/r2clone`,
role the scratch clone (`git clone --shared` of the project at HEAD `ec8de48d9f95eec183bdc30f14ab563e546a42b3`),
fired through `gate.py run --chunk 2026-11-27-epoch-5-cleanup --docs-root <root> --entry N --project-root <clone>`.

Unmodified (must read what the tree read):
- ` 18 probe       red · contains 'r2: red precondition=scratch' ✗ · exit 1 · 0.04s · 149 B → 18.log` (run-20261127T203902Z)
- ` 19 probe       red · contains 'r2: red precondition=locked' ✗ · exit 1 · 0.05s · 149 B → 19.log` (run-20261127T203904Z)

Input: the plan's step 16 applied to the clone's `scripts/r2-open.sh` alone, by `scratchpad/r2edit.py` (a
`precondition` function printing `r2: red precondition=<word>` and exiting 1; `scratch` when the `<scratch home>`
argument or `$XDG_RUNTIME_DIR/viola-r2` exists, ahead of the tool reads; `locked` when `hyprctl locked` is not `false`,
after them). Must-pass (the literal as the plan builds it):
- ` 18 probe       green · exit 1 · 0.04s · 85 B → 18.log` (run-20261127T203904Z)
- ` 19 probe       green · exit 1 · 0.04s · 84 B → 19.log` (run-20261127T203905Z)

The addendum entry (26, `leg = 'operator'`): no `outcomes.json` stands on the host to read its atom against
(`find ~/dev/projects/viola-mutants-scratch ~/.cache <session scratch> -maxdepth 6 -name outcomes.json`: none). Its
must-pass reading is its first firing after step 8, before any graded use, as the plan's prose names it.
