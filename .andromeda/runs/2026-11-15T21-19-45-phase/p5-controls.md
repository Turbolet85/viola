# P5 controls — chunk 2026-11-15-api-error-turn-end

Entry 13 (`{ git diff --name-only aa6bb5abea6b -- fixtures/claude; git ls-files -o --exclude-standard --
fixtures/claude; } | wc -l`, expect `last line 0`): a guard reading a change, read on both sides. Each firing went
through `gate.py run --chunk 2026-11-15-api-error-turn-end --docs-root . --entry 13 --project-root
$TMPDIR/{scratchpad}/ctl13`, a `git clone --shared` of the project checked out at `aa6bb5abea6b` (HEAD
`aa6bb5abea6b5bf858ffcf65fe6e602f0cd3eae0`); each input was made in the clone and undone after its read.

- The tree, untouched: green · last line 0 (an empty input; no must-pass by itself).
- The clone, unmodified: green · last line 0 (reads what the tree reads).
- A — an untracked new file `fixtures/claude/2.1.325/StopFailure.default.json`: red · last line '1'.
- B — a tracked fixture changed (`fixtures/claude/2.1.302/Stop.default.json`, one byte appended): red · last line '1'.
- C — a change outside the guarded path (`plugin/hooks/hooks.json`, one byte appended), the must-pass: green · last
  line 0.

The clone's status read empty after the last undo.
