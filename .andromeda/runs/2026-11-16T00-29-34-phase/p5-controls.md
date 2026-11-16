# P5 controls — 2026-11-16-wait-wake-condition (validation.md check 4 (9))

Clone: `git clone --shared` of the project, detached at `df3bad61` (HEAD), at
`$TMPDIR/{scratchpad}/p5-clone`. Each entry fired as itself through
`gate.py run --chunk 2026-11-16-wait-wake-condition --docs-root . --entry {n} --project-root {clone}`.

- entry 10 (no recorded fixture added or changed):
  - unmodified → green, last line 0 (run-20261116T012037Z, the same as the tree's baseline);
  - an untracked `fixtures/claude/2.1.287/Stop.held-1.json` minted → red, last line 1 (run-20261116T012038Z).
- entry 9 (the class measurement's record):
  - must-pass: a planted `class-wall.json` with after_wall 150.4 < before_wall 410.2, after_max 6.2 < 20, tests
    330 = 330 → green, last line True (run-20261116T012045Z);
  - must-fail: the same record with after_max 21.0 → red, last line False (run-20261116T012046Z).
