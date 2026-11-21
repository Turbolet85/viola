# P5 controls — 2026-11-21-cross-session-prefix-ledger-row

Each control fired AS THE ENTRY through `gate.py run --chunk 2026-11-21-cross-session-prefix-ledger-row --docs-root
<root> --entry {n} --project-root <clone>`, no `--run-dir`, over a `git clone --shared` of the project at HEAD
`3d281c17` in the session scratchpad (`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/<session>/scratchpad/ctl.IHtzha/clone`,
then `ctl2.RHTBlB/clone`); the inputs were minted in the clone only. Lines as the tool printed them (log names
trimmed).

## Run 1 (`ctl.IHtzha`, plan rev 1)
- 27 unmodified (no stamp): `red · exit 0 ✗ (exit 1)`
- 27 must-pass (29 rows, 28 pass and `browser-tools-prompt` absent): `green · exit 0`
- 27 must-fail (one row `fail`): `red · last line '29 ok' ✗ (last: '29 short')`
- 29 unmodified (no stamps, no sha file): `green · exit 0` — **a vacuity: empty equals empty.** Entry 29 (and entry 30,
  same shape) re-spelt at P5 with `test -s` on the sha file and `test -f` on each stamp; plan rev 2.
- 29 must-pass (sha file matches): `green` · must-fail (one stamp moved): `red · exit 0 ✗ (exit 1)`
- 30 unmodified (no sha file): `red · exit 0 ✗ (exit 1)` · must-pass (sha of the live stamp now): `green` ·
  must-fail (another sha): `red · exit 0 ✗ (exit 1)`
- 31 unmodified (no before file): `red · exit 0 ✗ (exit 2)` · must-pass (before = the count now, 141): `green` ·
  must-fail (before = count − 1): `red · exit 0 ✗ (exit 1)`
- 34 unmodified (no record): `red · exit 0 ✗ (exit 2)` · must-pass (the record a plain copy of the set, the tap
  fixture left out): `green · exit 0` · must-fail (one byte appended to one file): `red · exit 0 ✗ (exit 1)`

## Run 2 (`ctl2.RHTBlB`, plan rev 2, entries 29 and 30 re-spelt)
- 29 must-fail (stamps and sha file all absent): `red · exit 0 ✗ (exit 1)`
- 29 must-pass (sha file matches): `green · exit 0`
- 29 must-fail (one stamp moved): `red · exit 0 ✗ (exit 1)`
- 29 must-fail (stamps removed, sha file standing): `red · exit 0 ✗ (exit 1)`
- 30 must-fail (sha file absent): `red · exit 0 ✗ (exit 1)`
- 30 must-pass (sha of the live stamp now): `green · exit 0`
- 30 must-fail (another sha): `red · exit 0 ✗ (exit 1)`

## Run 3 (`ctl4.lkbkrf3j`, plan rev 4: entries 29–31 re-keyed to the round leg, `inputs#I3` items 3–4)
`gate.py` drives a `round` entry only inside a live round (`--entry 31` under `--live-legs` is refused, exit 2: «--entry
names a `leg = 'live'` entry»; without it each reads `not run — leg round`), so each entry's own `run` was typed as the
tool types an entry, `/usr/bin/bash -o pipefail -c '{run}'`, in a fresh clone (`controls4.py` in the session scratchpad),
the clone's HEAD `3d281c17`:
- 29 (kept-floor) all absent: exit 1 · must-pass: exit 0, last line `kept-floor: unchanged` · one stamp moved: exit 1
- 30 (live-stamp) sha file absent: exit 1 · must-pass: exit 0, last line `live-stamp: unmoved` · another sha: exit 1
- 31 (stale-sockets, a READING) before file absent: exit 1 · must-pass: exit 0, last line `stale-sockets: before 7
  after 141` (both counts printed; no atom compares them)

The three control entries 16–18 (plan rev 4) baseline red, exit 4, 0 selected each on the untouched tree.

The live home's stamp was read for its sha alone; nothing was written under `~/.viola` or `~/.claude*`. The runtime
dir's `cc-socks/` was read for a count of names alone (141 stale at the read).
