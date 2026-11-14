# Control — entry 9 (the fuzz lock read under `--locked`), check 4 (9)

Clone: `git clone --shared` of the project at HEAD `e8d7406045d094ed4ad1aa470224f75526647397`, at
`$TMPDIR/{scratchpad}/clone9`; fired as the entry through `gate.py run --chunk 2026-11-14-budget-gate-removed
--docs-root . --entry 9 --project-root $TMPDIR/{scratchpad}/clone9`.

- unmodified (the clone as HEAD): `9 probe green · exit 0 · 0.08s · 291 B` — the same reading as the tree's.
- input it must fail on: `chrono.workspace = true` removed from `crates/viola-agent-claude/Cargo.toml`, `fuzz/Cargo.lock`
  left as committed: `9 probe red · exit 0 ✗ (exit 101) · exit 101 · 0.06s · 316 B` — a stale fuzz lock reads red.

The must-pass side (the manifest edit with the fuzz lock refreshed) is implement's step 15 reading.
