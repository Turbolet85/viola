# P5 controls — 2026-11-16-send-confirmation

Entry 11 (`{ git diff --name-only 86fc64d0b4fb -- fixtures/claude; git ls-files -o --exclude-standard -- fixtures/claude; } | wc -l`),
fired as the entry through `gate.py run --entry 11 --project-root` over a scratch clone
(`git clone --shared` of the project at HEAD `86fc64d0b4fb`, at `$TMPDIR/{scratchpad}/ctl11`):

- clone, unmodified: `11 probe green · exit 0 · 0.01s · 2 B → 11.log` (last line 0), as on the real tree.
- clone, input `fixtures/claude/2.1.302/UserPromptSubmit.minted-1.json` minted untracked:
  `11 probe red · last line '0' ✗ (last: '1') · exit 0` — the guard reads a new fixture.

Entries 6, 7 and 9 read red on the untouched tree with `error: no tests to run` (exit 4): the red the plan predicts on
an absent input (their tests are not written yet). Their must-pass and must-fail readings are implement's
red-before-green run and the `locked-turn-read`, `trail-trim` and `fake-trim` controls (plan step 10).
