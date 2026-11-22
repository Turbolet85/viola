# Entry 15's must-pass control (validation.md check 4 (9)) — `rec.py check --root . --no-heal`

Fired as the entry, `gate.py run --chunk 2026-11-22-epoch-4-cleanup --docs-root . --entry 15 --project-root
$TMPDIR/{scratchpad}/ctl-clone`, over a `git clone --shared` of the project at HEAD `a77798371f932d6afd0ae9fc97b29126f354a3d7`:

- clone with `.claude/rules/testing.md` given one appended LF (the input it must fail on): ` 15 probe       red ·
  contains 'check: clean' ✗ · exit 0 · 61.87s` — the log's lines `render: .claude/rules/testing.md differs from the
  journal …`, `records: harness · engine 1.5.6 · render DIFFERS: 1 · …`, `check: 1 failure(s)`.
- clone unmodified (the edit checked out; the input it must pass): ` 15 probe       green · exit 0 · 62.74s`.
- on the tree: ` 15 probe       green · exit 0 · 62.81s` (plan rev 3).

The clone `$TMPDIR/{scratchpad}/ctl-clone` is left in place (no recursive delete), named on the P5 card.
