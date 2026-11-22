# Outcome — 2026-11-21-key-after-a-resize · implement run 2026-11-22T08-19-57Z-implement

**Outcome: stuck** — the live round stopped at entry 15 (`KR8`, claude 2.1.328), exit 1, `dirty-box: 8`,
`kr-reading: unverified`. No `lost` word stands in either start. Per the overseer's directive (`inputs#I4`, «Stops»;
`inputs#I5`) a start that exits 1 stops implement: the spare `KR8-2` is a plan revision's entry on the overseer's word,
never a re-fire; nothing was retried. No report written (a stuck run writes none); no matrix ref written.

## Files changed (5, every one in research's «Files to modify»)
- `crates/viola-e2e/src/rig/keys.rs` — `Grant.resize`, the `resize` word, `Keys::may_resize`, `Keys::may_type_box`, the grant case `the_resize_word_grants_a_resize_and_no_key`.
- `crates/viola-e2e/src/rig/screen.rs` — `Model::resize` (vt100 `set_size` in place, poison kept), the case `a_resized_model_keeps_its_box_and_reads_at_the_new_width`.
- `crates/viola-e2e/src/rig/mod.rs` — `Arm::KR` (word `kr`, labels `KR` · `KR-2` · `KR8` · `KR8-2`, HB's seeds), `Session::resize` / `pump_for`, `arm_kr`, `kr_control_round`, `kr_rounds`, `KR_ROUNDS`, `kr_delay`, `kr_size`, `kr_control`, `kr_round_word`, `kr_reading`, `kr_whole` in `finish`, `KrTally`, the module doc, four cases (`the_kr_labels_take_the_unwrapped_trusted_marker_seeded_arm`, `the_kr_schedule_interleaves_its_delays_four_each_and_changes_width_every_round`, `each_kr_round_and_reading_word_comes_from_attribute_words`, `the_kr_tally_puts_its_closed_words_and_the_whole_start_rule_reads_them`).
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — `kr` in `rehearse`'s words, `KR` · `KR8` in `live`'s doc line with its sentence.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — `rig_rehearsal_kr_reads_each_key_kept_after_its_own_resize`, `rig_rehearsal_kr_without_the_resize_grant_types_nothing`, the helper `sizes`.

No product file changed; scope read `scope: clean — changed 5 · listed 5 · recorded 0`.

## Deviations, each with its why
- `kr_control`: a typed word read `unread …` reads `confounded` as well as typed equal to hint (the plan names only the
  equality). Why: an `x` never drawn cannot tell the box apart; the hint word may be `unread (no-cell)` (the fake
  agent's empty box), so only the typed side is held.
- `lost-at` lists each lost round's delay in round order (`20ms,20ms`), the plan saying «the delays a `lost` fell on,
  comma-joined».
- A key refused or a resize failed inside a round is the summary key `key-refusal` (words `key-not-ready` ·
  `key-not-granted` · `key-write-failed` · `resize-failed`), never `refused:`, so it cannot trip the live entries'
  `lacks refused:` atom; the round still counts `unread`.
- `Keys::may_type_box` added beside `may_resize` for `arm_kr`'s first check (the plan: «without `may_resize` and the
  `box` grant»).
- The `KR` rehearsal ran 12.4 s (probe) against the rig's default `--deadline 15` and the case bound `WITHIN` 18 s; no
  bound moved. A loaded host could cut it: named, not changed.

## Gates
- Phase A, entries 1–5: `green` 5/5 (trail `.andromeda/runs/2026-11-22T08-19-57Z-implement/gate-2026-11-21-key-after-a-resize.json`); entry 4 read `passed 2`.
- The round (entries 6–19), one `--live-legs` call fired 2026-11-22T08:26:30Z after hostwatch read `QUIET`
  (08:26:05Z–08:26:25Z): `round: STOPPED at 15 (red · exit 0 ✗ (exit 1)) · legs fired 2/2 · … green 7 · red 1 (15) · recorded 2 · … not-run 19`
  — `evidence/round-082630Z.txt`.
  - Entry 14 `KR` (2.1.330): green — `kr-reading: kept`, 12/12, 4/4 at each delay.
  - Entry 15 `KR8` (2.1.328): red — atoms `keys: x 13, del 13` ✗ and `rounds: 12` ✗; read `rounds: 7`, `kept: 7/12`,
    `lost: 0/12`, `unread: 0/12`, `dirty-box: 8`, `kr-reading: unverified`, `keys: x 8, del 8`.
- Entry 20 (census, `--operator 20`): `green · exit 0 · history: unmoved`.
- Entry 21 (ledger read-back): `recorded` — copied into `evidence/live-round.md` from the typed fields alone.
- Entries 16, 17 (after-reads; not fired by the stopped round) run by hand with their exact text at 2026-11-22T08:27:40Z:
  `kept-products: unchanged`, `live-stamp: unmoved`.
- Not run: 18, 19 (the readings; `KR8`'s would read `unverified`), 22 (the whole integration layer), 23–29 (the operator pass).

## What stuck
`KR8` round 8 read the box's attribute word not the hint word before its resize (`dirty-box: 8`). The settle times
(`settle-ms`, closed numbers) show round 7's x settle at 729 ms, its DEL settle at 50 ms, round 8's box settle at 63 ms.
HYPOTHESIS, not measured: the rig's `settle` reads «quiet» from the last byte, which can predate the key just typed, so
round 7's DEL settle returned on a frame from before the DEL's redraw (or with none) — a rig-side read, not a key loss
reading. Whether the DEL's redraw came late or never is not read: no kept frame was read. The fix (e.g. a settle that
waits for a byte after the key, or a spare `KR8-2`) moves a plan entry: the overseer's word / a plan revision
(`/andromeda-phase`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig` + one `claude` 2.1.330 (entry 14) | a leg | terminated — `census: 0` in its summary; the round's census found no survivor |
| `viola-live-rig` + one `claude` 2.1.328 (entry 15) | a leg | terminated — `census: 0`, `stop: ctrl-c 4`, `child: stopped` |
| the two `KR` rehearsals' rig and fake agent (entry 4 and the probe run) | this run | terminated — `assert_torn_down` held |
`gate.py census --comm claude --comm viola-live-rig --comm viola-fake-agent --args projects/viola/target/debug/viola`
read 8 running, every one a host session under `claude/latest` (overseers and builders), none this run's.

## Decisions & corrections
- The overseer's word to fire the round, hostwatch read from 2026-11-22T08:26:05Z, one call, any `lost` stops
  implement — `inputs#I5` (relay-1.md).
- The stop at a start that exits 1, no retry, the spare only on the overseer's word — `inputs#I4` «Stops», applied.
- A wrong oracle of mine (a vt100 row reads only its drawn cells, so a row's char count is no width) fixed at the
  first unit run; the case reads the width by the past-screen cell instead.
