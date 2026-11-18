# Outcome — 2026-11-18-trust-record-row-read-live (implement run 2026-11-18T01-39-23Z, plan rev 4, Revision 1)

**Outcome: surfaced.** Re-entry at step 12. Steps 12–15 are built and green on the fake. Both new controls hold.
**S0R-2** (the first spare, `--spare 1`) was ledgered at 01:51:56Z and read **RED**: the floor is broken, exactly
as at S0R. **`seed-markers: written` stood**, so the rig's first-run config is NOT the cause. The finer facts exclude
K1 and name **K3**: a dialog in the CLI's own dialog frame, whose confirm label is neither compiled modal literal.
Per plan step 16's second end, implement **stopped before the operator pass**. F1 is updated, and **S0R-3 is NOT
spent**: it is the founder's. The run was not refused for the token window.

Next → the founder's answer to F1's update, options (e) / (f) / (g), in
`.andromeda/runs/2026-11-18T01-02-09Z-implement/founder-forks.md`. Then either `/andromeda-phase` (a revision, e.g.
for S0R-3 under (e)) or `/andromeda-wrap-session` (g). The wrap authors the chunk report: a surfaced run writes none.

## Files changed this run (7, all in research's lists; scope: clean — changed 14 · listed 14 · recorded 0)
- `crates/viola-e2e/src/rig/isolate.rs` — `Scratch::seed_used_markers(version)` (merges the six markers into the
  private `.claude.json`, keeping every seeded key; one `write_private`), `Scratch::seed_markers_word` (read from the
  disk), the named constant `USED_MARKERS` with its `Marker` forms (the 2.1.325 bundle named as their source),
  `read_json` (bounded at `MAX_FRAME`); case `seed_used_markers_writes_the_six_bundle_markers_and_keeps_the_seed`.
- `crates/viola-e2e/src/rig/mod.rs` — the V arm only, after `seed_config`: the version read from the child program
  by path (`cli_version` + `version_token`), the markers written (an unreadable version or failed write refuses
  `seed`), `seed-markers: <word>` read back from the disk. The `facts-<x>` line gains `lit= adj= first= col=`
  (`literals_word`, `LITERAL_IDS`). The unit case
  `stamp_reading_puts_each_runs_screen_settle_and_facts_and_the_failing_ids` is extended (a literal list, `none`,
  `unread`).
- `crates/viola-agent-claude/src/screen.rs` — `RowFacts` gains `literals` (per `BOX_LITERAL_IDS`, the first row),
  `rule_above_prompt` (`holds_box`), `first_row` (`RowFamily`: rule · banner · text) and `last_col`; tables
  `first_screen_facts_name_each_literal_its_row_and_the_first_family` (12 cases) and
  `box_literal_ids_and_row_families_are_kebab_case`. The existing facts table now compares `(rows, box_any)`.
- `crates/viola-agent-claude/src/ledger.rs` — `FirstReading` gains the four facts; `first_facts` rows carry
  `literals` (keyed by id), `rule_above_prompt`, `first_row` and `last_col` (`measured` only).
- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` — the merge case asserts the four new keys.
- `src/cmd/verify/typed/run.rs` — `reading_of` fills the four facts; `reading_of_reads_the_feed_and_the_rows_at_the_settle`
  reads them (a text row, a box screen, a poisoned screen).
- `tests/cli_verify/runs.rs` — the fake case's and the no-screen case's oracles for the four facts.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — `seed-markers: written` and each `facts-<x>` line's finer suffix.
(Steps 1–10's 14 files from run 2026-11-18T01-02-09Z stand uncommitted beside these.)

## Gates (trail `.andromeda/runs/2026-11-18T01-39-23Z-implement/gate-2026-11-18-trust-record-row-read-live.json`)
- First firing: `entries 19 · green 5 · red 2 (5,6)`. Both reds came from step 15's predicted oracles against the
  fake's real screens: `verify_first_screens_read_trust_then_box_on_the_fake` and
  `rig_rehearsal_v_reads_the_trust_record_row_and_no_record_left`. Recorded before the re-fire in
  `evidence/gate-reds-rev1.md`.
- Fix: the first run's values were copied in as the oracles, as step 15 directs (testing.md 2026-10-16). Then
  `--only 1,2,5` read green, and the full block read `entries 19 · green 7 · red 0 · recorded 0 · timeout 0 ·
  not-run 12`.
- The 52 s case under the 80 s kill: `PASS [  52.184s]` (entry 5) and `PASS [  51.788s]` (entry 6).
- Controls (`evidence/control-2.txt`): `control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed`. `seed-markers` (entry 4, removed red exit 1) and `fs-adjacent` (entry 3, case
  `…::case_05_rule_above_prompt`, removed red exit 100).
- Hygiene `clean` and the land (tx 2594, 2 new · 12 kept) before the start, as step 16 asks.
- Live round (`evidence/round-015151Z.txt`): `round: STOPPED at 11 (red · lacks 'input-box-signature' ✗) · legs
  fired 1/1`. The install was read by path (`2.1.325 (Claude Code)`). The build hash `6728062b1afb6ca4` equals the
  summary's `viola-build`. `mise ls claude` read 2.1.326 `latest`, not used.
- No `gate.py guard` entry. No deferral. No matrix ref: the chunk owns no coverage row, and no capability is claimed.

## What was surfaced (`evidence/live-starts.md` §S0R-2)
- **The reading:** `verify-exit 1 · 7/23 · trust-record fail`; the same 16 failing ids as S0R. Run A `trust` at
  461 ms. Runs B, C, D, F `none` at 8 500 ms on a live child: 9–10 rows, 1167–1319 bytes, quiet by 411–425 ms, each
  `lit=rule@1 adj=no first=rule col=80`. Run E started no child.
- **What it shows:** with the six bundle markers on the disk, the screen is unchanged, so the first-run config is
  not the cause. The only box-family literal on B–F is one full-width rule as the first row (row 1). There is no `❯`
  row and no footer, which is the frame of Run A's trust dialog (`lit=rule@1 adj=no first=rule col=80`). K1 is
  excluded. K3 (an unknown modal) is named. Branch (i), the notice stack, is argued against. Branch (ii), an account
  dialog, is consistent but unmeasured; the Grove consumer-terms dialog is one candidate.
- **The mend is the founder's:** naming the dialog needs its text, or the founder's own eyes. F1 now offers (e) S0R-3
  keeping Run B's first screen locally, recommended; (f) the founder opens 2.1.325 in a fresh config; (g) wrap as it
  stands. F2's premise was not confirmed.

## Deviations from the plan, with justifications
- **Step 15's fake-agent oracles.** The plan said A reads `literals.rule` non-null, and B–F read `rule_above_prompt`
  true with `literals.rule` and `literals.prompt` non-null. The rehearsal was to read `adj=yes` on B–F. The fake's
  screens draw none of that: its trust form has no box-family literal, and its ready screen has only `for agents` on
  row 23. Step 15 itself says the oracles are copied from the first run (testing.md 2026-10-16), so they now read: A
  `lit=none adj=no first=text col=27`, B–F `lit=footer-agents@23 adj=no first=text col=33`. The plan's acceptance
  line «over the fake, B–F read `rule_above_prompt` true» is therefore disproved as written. This is a test premise,
  not product behaviour, and the wrap resolves its wording. Adjacency stays proven at the unit tier (the
  `fs-adjacent` control holds), and the live start read it as `no` on every run.
- Added beyond the plan's named cases: `box_literal_ids_and_row_families_are_kebab_case` (screen.rs). It pins the
  closed ids and the family words, so the `as_str` arms have an observable effect (testing.md 2026-09-24).
- The existing table `first_screen_facts_count_rows_and_any_box_literal` now compares `(rows, box_any)` rather than
  the whole struct, so its rstest case names are unchanged (fs-boxany's `case_2_rule_alone`).
- Scope record: none needed (scope `clean`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig live S0R-2`, its `viola verify` and 7 `claude` 2.1.325 children | a leg (entry 11) | terminated: the rig's summary read `census: 0`; the rig census by hand at 01:52:57Z read `census: 0`; `gate.py census` at 01:52:57Z showed no 2.1.325 `claude` and no rig. The round reported no survivors |
| gate entries 1–7 and 8–10 (cargo, nextest, harness) | this run | terminated (each entry's tree censused by the tool) |
| control clone `~/.cache/andromeda-control/fd236a33` and its firings | this run (`gate.py control`) | terminated; `clone removed` |
| 16 host rows (`claude` on the `latest` install: overseers, builders, sessions; `viola` from `~/.local/viola-product`, the viola-lab prototype and `~/.viola/bin`) | NOT DERIVED — the host's own sessions and other trees | left running: not this run's |

## Decisions & corrections
- The directive's floor held. The live entry read RED, so implement stopped before the operator pass, recorded the
  facts and updated F1. S0R-3 was not made, and no timer or scheduled wake was set.
- The step 15 oracle premise was disproved by the first run. The values were copied in under the plan's own rule
  and recorded in `evidence/gate-reds-rev1.md`.
- The first control file named the case `case_5_rule_above_prompt`. rstest pads it to `case_05_…` (read from entry
  3's log), and it was corrected before the control fired.
