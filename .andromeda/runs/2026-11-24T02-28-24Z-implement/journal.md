# Implement journal — 2026-11-24-viola-focus

Each long run, named before its wait (the directive `inputs#I5`).

- 2026-11-24T02:38Z — entry 8 alone (`gate.py run --entry 8`): the R3 rehearsal over the fakes, its must-pass
  reading before any graded use. PATH read by eye first: every `hyprctl` the script calls is `"$HYPRCTL"`, set to
  `$scratch/bin/hyprctl` (the fake copy) under `--rehearse`; every viola child starts under `env PATH=$scratch/bin`
  alone; board reads under an empty-dir PATH; the bare `command -v hyprctl` is on the live branch only.
  Read: `8 e2e green · exit 0 · 31.46s`; its log `r3: green hand=focused opened=focused` then
  `r3: rehearsed — not a live reading`. Census after: no foot, viola, fake agent or sleep of this run alive.
  Left on disk: `$TMPDIR/tmp.FmH6CggFOf/r3` (the entry's own `mktemp -d`).
- 2026-11-24T02:41Z — the block's entries 1-9 (`--only 1-9`), backgrounded: fmt, clippy, the root units whole and
  filtered, the new integration binary, the readers, `bash -n`, the rehearsal again, the host read. The three whole
  layers (10-12) wait for my own hostwatch read after it.
  Read: entries 1-8 green, 9 recorded (`entries 19 · green 8 · red 0 · recorded 1`).
- 2026-11-24T02:41Z — my host read (`hostwatch.py read --last 15`, 02:25:03Z..02:40:03Z): `CONTENDED - 70 s stalled
  on IO: 02:34:13Z..02:34:18Z 5 s; 02:38:33Z..02:39:38Z 65 s … build tools alive in them: escher 65 s, viola 50 s`;
  `df -m /tmp` 31370 MB free of 32095. A neighbour (escher) alive in the stalled seconds: entries 10, 11 and 12
  WAIT for the overseer's quiet window (directive `inputs#I5`, The host).
- 2026-11-24T02:42Z — the three controls (`control-1.toml`: focus-reuse, focus-order, focus-no-client) on entry 4,
  `--timeout 900`, backgrounded: filtered runs, which the directive lets run as they come.
  Read: `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`; lines in
  `evidence/control-1.txt`.
- 2026-11-24T02:40:53Z — host re-read (02:25:53Z..02:40:53Z): still `CONTENDED - 70 s stalled on IO … escher 65 s,
  viola 50 s`. STOPPED here: entries 10, 11, 12 (the three whole layers) and the operator pass wait for the
  overseer's quiet window. Nothing of this run is running.
- 2026-11-24T02:46Z — the overseer's quiet window (`inputs#I6`): host read 02:46:08Z..02:46:21Z `QUIET - 0 s stalled
  on IO`, no build tool seen; `df -m /tmp` 31370 MB free. Entries 10, 11, 12 (`--only 10,11,12`) backgrounded now.
  Read: `entries 19 · green 3 · red 0` (10 unit 12.62 s, 11 integration 70.83 s, 12 e2e 11.52 s).
- 2026-11-24T02:50Z — a fix after green: `scripts/r3-focus.sh` wrote `hand=not-live` for a wrapper never live, a
  word outside the plan's (`focused`, the focus's code, `unread`); now `hand=unread`. Entries 7 and 8 re-fired
  (`--only 7,8`), backgrounded.
  Read: 7 green, 8 green (`entries 19 · green 2 · red 0`). Report landed rev 2; hygiene clean; chunk landed tx 4455.
- 2026-11-24T02:51Z — the operator pass: host read 02:46:08Z..02:51:04Z `QUIET - 0 s stalled on IO`, `df -m /tmp`
  30657 MB free. Entry 14 (`agent-run.sh pre-push`) fired through `gate.py run --operator 14`, backgrounded.
