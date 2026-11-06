# P5 controls — 2026-11-06-version-done-check

Each control was fired as its entry through `gate.py run --chunk 2026-11-06-version-done-check --docs-root . --entry {n}
--project-root $TMPDIR/{scratchpad}/clone-p5`. The root is a `git clone --shared` of the project at HEAD
`aec2ec95d3c241758bb79e592d43e9abc4de11ee`. The script is `$TMPDIR/{scratchpad}/p5-controls.sh`.

## Unmodified clone (reads as the tree did)
- 8 `cargo tree -i pin-utils …`: red, exit 0, the pin-utils ← nix 0.25.1 ← portable-pty 0.8.1 path.
- 9 `git grep … RUSTSEC-2017-0008 … =0.8.1`: red, exit 0, the two lines.
- 15 `cargo deny check`: red, exit 1, `advisories FAILED` on RUSTSEC-2026-0441.

## Step 1 applied in the clone (the must-pass input)
The input: `portable-pty = "=0.9.0"`, `cargo update -p portable-pty`, and the RUSTSEC-2017-0008 line removed from
`deny.toml`.
- 8: green, exit 101 (`did not match any packages`).
- 9: green, exit 1, no output.
- 15: green, exit 0 (`advisories ok, bans ok, licenses ok, sources ok`).
- 11 `cargo nextest run -p viola-pty`: green, exit 0, 33 tests run.

## Entry 10, the remnant guard
- Fail side: the real tree, red, 75 lines in 47 files (its baseline).
- Must-pass input: the clone with every matched line dropped from the matched files. Reading: green, exit 1, no
  output.

## Entries 37–39, the live-record readers
- Fail side on an absent input: the real tree, red, jq exit 2 (no file).
- Must-pass input: a control ledger with 2 `start` rows, 1 `reading` with verdict green, and a `census` with
  processes_left 0. Reading: 37, 38 and 39 all green.
- Must-fail input: a ledger with 1 `start` row, a red `reading`, and a census with 1 process left. Reading: 37 red
  (last line 1), 38 red (last line 1), 39 red (last line 1).
