# P5 controls — 2026-11-16-wait-wake-condition, revision (validation.md check 4 (9))

Entry 9 re-worded (inputs#I7, inputs#I8). Clone: `git clone --shared` of the project, detached at `df3bad61`, at
`$TMPDIR/{scratchpad}/p5r-clone`. The entry fired as itself through
`gate.py run --chunk 2026-11-16-wait-wake-condition --entry 9 --project-root {clone} --docs-root .` (no `--run-dir`:
a clone's control takes none), over a planted `class-wall.json` in the clone's evidence folder.

- baseline on the tree (this run dir's gate trail): red, exit 1, `KeyError: 'before_path1_s'` — the record carries
  the old keys only; step 8's split record not written yet (the red the plan predicts on an absent input).
- must-pass — the stop's measured split: after_rest_max 10.913 < 20, after_window_max 43.358 < 60, window tests 3,
  tests 321 = 321, wall 56.069/48.381 and path1 6.593/2.385 printed → green, last line True (run-20261116T014307Z).
- must-fail, the rest arm — after_rest_max 21.0 → red, last line False (run-20261116T014312Z).
- must-fail, the window arm — after_window_max 61.0, rest back at 10.913 → red, last line False
  (run-20261116T014318Z).
