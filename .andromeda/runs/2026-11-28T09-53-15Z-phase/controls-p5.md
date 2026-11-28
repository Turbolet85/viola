# P5 controls — 2026-11-28-fleet-console-view (validation.md check 4 (9))

Entry 10 (the console-frame arm's required list, read by jq), fired as the entry through
`gate.py run --chunk 2026-11-28-fleet-console-view --docs-root . --entry 10 --project-root {clone}`, the clone
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/d15015c9-e9e2-42cc-ade5-82c7927a852a/scratchpad/clone-e10`
(`git clone --shared` of the project at HEAD `be4a06e2ae2528d1d45148ccdf27323b2b8d8c49`):

- unmodified clone (the interim grep form, before the re-spell): `10 probe red · exit 0 ✗ (exit 1) · exit 1` — the
  same reading as the tree.
- clone carrying step 8 (`schemas/diag-line.v1.json`'s console-frame arm with `shown` and `hidden` in its properties
  and its required list): the grep form read `last line '1' ✗ (last: '2')` — two lines hold `"shown"`, so the entry
  was re-spelt to the jq read of the required list.
- the re-spelt jq entry on the tree: `10 probe red · last line 'cols,rows,fits,shown,hidden' ✗ (last: 'cols,rows,fits')`.
- the re-spelt jq entry over the clone carrying step 8: `10 probe green · exit 0` — the must-pass reading.

The clone is left in the session scratchpad, not removed (no recursive delete, inputs#I1 item 7).
