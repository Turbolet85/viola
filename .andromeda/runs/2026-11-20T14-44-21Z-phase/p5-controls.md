# P5 controls — 2026-11-20-a-new-cli-version-stamped-automatically

Entry 23 (`live-stamp: unmoved`, a literal the entry asserts, red on an absent input at baseline) — must-pass reading,
fired AS THE ENTRY through `gate.py run --chunk 2026-11-20-a-new-cli-version-stamped-automatically --docs-root .
--entry 23 --project-root $TMPDIR/{scratchpad}/clone23`, no `--run-dir`:

- clone `$TMPDIR/{scratchpad}/clone23`, `git clone --shared` of the project at HEAD `816f9a77`.
- unmodified: `23 probe red · exit 0 ✗ (exit 1) · exit 1` — the same reading as the tree's baseline (the evidence sha
  file absent).
- input: `.andromeda/cache/chunks/2026-11-20-a-new-cli-version-stamped-automatically/evidence/live-stamp.sha256` minted
  in the clone holding the live home stamps' sha (64 hex, read from `~/.viola/ledger/stamps.json` by sha alone):
  `23 probe green · exit 0 · 0.01s · 20 B` — the line `live-stamp: unmoved`.

Entries 6, 7, 9 (filters to cases this chunk writes): red, exit 4, nextest «no tests to run», each binary term
selecting its binary (cli_version_gate 9 tests, rig_rehearsal 38) and the name term nothing — the subject this chunk
builds; their red-before-green is implement's.

Entries 4, 5: green on the untouched tree (275 and 1141 tests).
