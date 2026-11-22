# Outcome — 2026-11-21-key-after-a-resize · implement run 2026-11-22T08-49-22Z-implement

**Outcome: green** — re-entry after plan Revision 1 (plan rev 5) at its step 7. Phase A green 6/6 on its first
firing; both remove-the-guard controls `holds`; the round `COMPLETE` in one call, `KR8-2` (claude 2.1.328, the
spare, `--cap 2 --spare 1`) `kr-reading: kept` 12/12; the whole integration layer green. `matrix#v2-27` →
`implemented`. Next → the OPERATOR PASS (entries 24–30), in this window per the overseer's directive (`inputs#I9`).

## Files changed (5, every one in research's «Files to modify»; this run's edits in 3 of them)
- `crates/viola-e2e/src/rig/mod.rs` — `Session.quiet`, `Session::mark`, `Session::settle_since`, `settled_after`,
  `kr_box_word`, `kr_del`, `KrSeam` (`parse`, `withholds`), `quiet_of`, `Plan.quiet` / `Plan.kr_seam`, `KrTally.late_redraws`
  and its `late-redraw` key; `kr_control_round` and `kr_rounds` on `settle_since` (each key on its mark, each box
  settle and re-read on none), the box re-read; cases `the_kr_box_word_tells_a_late_redraw_from_a_dirty_box`,
  `a_byte_drained_before_the_key_never_settles_it`, `a_byte_after_the_key_settles_once_quiet`,
  `no_mark_settles_on_quiet_alone`, `the_kr_seam_reads_withheld_and_late_rounds_and_refuses_any_other_form`,
  `the_quiet_option_reads_one_to_four_hundred_ms`; `the_kr_tally_puts_its_closed_words_and_the_whole_start_rule_reads_them`
  gains `late-redraw: 0` and a one-late-redraw tally. `Session::settle` unchanged (no line).
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — `Verb::Rehearse` gains `--quiet-ms` and `--kr-del-seam` (`kr` alone,
  else `refused: usage` exit 2; a bad value the same); `plan_of` sets the defaults; the module doc sentence.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — `--quiet-ms 100` and `late-redraw: 0` on the whole KR rehearsal; new
  `rig_rehearsal_kr_reads_a_late_redraw_and_keeps_the_round`, `rig_rehearsal_kr_stops_on_a_box_still_dirty_on_its_re_read`,
  `rig_rehearsal_kr_seams_refuse_a_live_start_and_another_arm`.
- `crates/viola-e2e/src/rig/keys.rs`, `crates/viola-e2e/src/rig/screen.rs` — the stopped run's (steps 1–2), unchanged here.

No product file changed. Scope read: `scope: clean — changed 5 · listed 5 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 248`.

## Deviations, each with its why
- `rig_rehearsal_kr_seams_refuse_a_live_start_and_another_arm` reads «nothing spawned» by the case's own checks (its
  ledger absent, its receipt empty, no process the rig started standing), not by `assert_torn_down` as the plan's
  step 11 names: `assert_torn_down` reads the summary's `census` / `credentials` / `scratch` / `user-config` words,
  which a `refused: usage` exit 2 never prints, so it would read red on a correct refusal.
- `KrSeam` is pub in `rig` (the bin parses it); `quiet_of` returns `QUIET` for no option so `plan_of` takes the
  default through one function. `Session.quiet` is set from `Plan.quiet` at `arm_kr`'s start, past `first_frame`
  (which stays on `settle`).
- The stopped run's four recorded deviations (its outcome file, «Deviations») stand.

## Gates
- Phase A, entries 1–6 (`--only 1,2,3,4,5,6`): green 6/6 — `cargo fmt --all --check` · clippy · the rig library
  cases · the KR rehearsals (5 passed) · `binary(rig_rehearsal)` · `run --unit`.
- KR rehearsal times (entry 4's JUnit, `target/run-archive/572`): the whole 5.110 s · the late 5.296 s · the withheld
  2.220 s · no-resize 0.920 s · seams 0.146 s — each under the 10 s predicted, none at 13 s.
- Controls (`control-1.toml`, `--timeout 1200`): `control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
  (`evidence/control-1.txt`).
- The round, entries 7–20, one `--live-legs` call fired 2026-11-22T09:18:17Z after hostwatch read `QUIET` from
  2026-11-22T09:13:00Z to 09:18:13Z (0 s stalled, no build tool seen): `round: COMPLETE · legs fired 1/1 · entries 30 · green 12 · red 0 · recorded 2 · timeout 0 · indeterminate 0 · not-run 16`
  (`evidence/round-091817Z.txt`). Entry 15 `KR8-2` green, every atom held; entries 16–18 `kept-products: unchanged`
  · `rig-dirs: one new, none gone` · `live-stamp: unmoved`; 19 `KR kr-reading=kept`; 20 `KR8-2 kr-reading=kept`.
- Entry 21 (`--operator 21`): `green · exit 0 · history: unmoved` — `census: 0`.
- Entry 22: `recorded` — the three summaries copied into `evidence/live-round.md` from typed fields.
- Entry 23 (`run --integration`): green.
- The carried `rig_rehearsal_reads_the_box_empty_after_x_and_del` (`cry-234cbfoa`) ran green in entries 5 and 23 —
  not met red; re-pinned whole per its rule.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig` + one `claude` 2.1.328 (entry 15, `KR8-2`) | a leg | terminated — `stop: ctrl-c 3`, `child: stopped`, `census: 0`; entry 21 `census: 0` |
| the rehearsals' rigs and fake agents (entries 4, 5, 23) | this run | terminated — `assert_torn_down` and the seam case's process check held |
| the controls' clone builds (entry 3 ×3) | this run | terminated — `clone removed` |
`gate.py census --comm claude --comm viola-live-rig --comm viola-fake-agent --args projects/viola/target/debug/viola`
read `census: 8 running`, every one a host session under `claude/latest` (overseers, builders, the worker), none this run's.

## Decisions & corrections
- The overseer's word to fire the round: hostwatch from 2026-11-22T09:13:00Z, one call, `KR8-2 --cap 2 --spare 1`;
  a lost, a still-dirty box or any exit 1 stops — `inputs#I10` (relay-1.md).
- The directive's stop before the round, honoured (Phase A and both controls green first) — `inputs#I9`.
- An evolve append refused `REFUSED ts:` (a ts typed ahead of the clock); re-fired with the read `date -u`.
