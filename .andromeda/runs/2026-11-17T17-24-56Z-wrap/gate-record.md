# Light gate — 2026-11-17-first-readings-on-the-standing-rig (wrap P7.1)

`gate.py run --chunk 2026-11-17-first-readings-on-the-standing-rig --run-dir .andromeda/runs/2026-11-17T17-24-56Z-wrap`
(no `--skip`, no `--void`): `entries 24 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 15` (trail
`gate-2026-11-17-first-readings-on-the-standing-rig.json`).

ASSERT, per entry:
- 1–8, 20 — green by their `expect`, re-run here.
- 9, 10, 11, 12, 13 (`leg = 'round'` / `'live'`) — green, recorded in `evidence/round-163909Z.txt` (`round: COMPLETE`).
- 14 (`R3`, `leg = 'live'`) — red · `contains 'user-config: unchanged'` ✗, recorded in `evidence/round-163940Z.txt`
  (`round: STOPPED at 14`), never re-fired: red — the plan's wording: entry 14 against the mark witness the chunk was
  obliged to build (plan step 12, the founder's answer inputs#I6 item 1; the old stamp witness could not attribute a
  host writer's change, `evidence/user-config-writer.md`) · word: "The four spent starts stand as read; none is
  re-fired. `R3`'s `user-config: changed` is recorded as it was read, with its measured cause" — the overseer (technical
  word), 2026-11-17, inputs#I6 item 4; re-stated by the wrap directive inputs#I8 item 3 («stands as read … not
  re-worded»). Its readings stand (R3a, R3c).
- 15 (`R3-326`) — green, `evidence/round-164338Z.txt`.
- 16 (`IM`) — green, its round lines in `evidence/live-starts.md` (`round: COMPLETE · legs fired 1/1`).
- 17, 18, 19 (`R3D`'s round) — green, `evidence/round-170712Z.txt` (`round: COMPLETE · legs fired 1/1`).
- 21–24 (`leg = 'operator'`) — green at the final HEAD 0e1373d7: `evidence/operator-21.txt` (pre-push `"ok":true`),
  `evidence/operator-22-23.txt` (hygiene clean; the push), `evidence/operator-pass.md` (ci#44134805079 green 7/7).
- No `defer`, no deferral, no `timeout`, no `INDETERMINATE:`.
