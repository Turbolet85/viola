# Outcome — 2026-11-18-trust-record-row-read-live (implement run 2026-11-18T06-24-01Z, plan rev 6, Revision 2)

**Outcome: surfaced.** Re-entry at step 17. Steps 17–19 are built and green on the fake, and the `vc-nokey` control
holds. The one naming start **S0C** was ledgered at 2026-11-18T06:32:18Z under its own ledger (`--cap 1 --spare 0`)
and read **GREEN by its predicate** (`round: COMPLETE · green 4 · red 0`). **It met NO dialog:** the rig's own plain
`claude` 2.1.325 reached the input box in about 1 s at 80×24, under the same private config, the same markers, the same
environment and Run F's argv. So the dialog verify's runs B–F met is **verify's own doing**. It is not the config's
and not the account's. Per plan step 20, implement **stopped before the operator pass**, and F1 is updated with the
three roads. **S0R-3 is not spent**, and the R6 ledger was not touched. The start was not refused for the token
window. No timer or scheduled wake was set.

Next → the founder's answer to F1's S0C update in `.andromeda/runs/2026-11-18T01-02-09Z-implement/founder-forks.md`:
(h) recommended, a revision whose rig arm reproduces the trust path B–E take; (e′) S0R-3 keeping Run B's frame; or
(g) wrap as it stands. Then `/andromeda-phase` (a revision) or `/andromeda-wrap-session`. The wrap authors the chunk
report: a surfaced run writes none.

## Files changed this run (4, all in research's lists; scope: clean — changed 16 · listed 16 · recorded 0)
- `crates/viola-e2e/src/rig/mod.rs` — `Arm::VC` (word `vc`, label `S0C` alone; unwrapped, no viola home, its cwd
  trusted), `Arm::verify_seed` (V and VC take the git root and the six markers, and print `git-root:` and
  `seed-markers:`), `Arm::pty_size` (VC spawns at `Size::DEFAULT`, 80×24; every other arm at 210×45). The summary
  gains `pty: <cols>x<rows>` from the spawn spec, on every arm. `Session` keeps its spawn instant and counts fed
  bytes, and builds its model at the spec's size. `arm_vc` does `first_frame`, then `record_reading`, then
  `screen:` (`SIGNATURES.first_screen`) and `facts:` (the stamp's `first_facts` row shape, through `facts_line`), then
  `keys: none`. VC stops through `stop_no_key`. The unit case `the_arm_words_and_labels_parse` gains `vc`, `S0C`,
  `S0C-2` → none, and the VC/V pty sizes.
- `crates/viola-e2e/src/rig/screen.rs` — `Model::sized(rows, cols)`; `rows()` now reads at the model's own width;
  `new()` keeps 45×210. Case `a_sized_model_reads_its_rows_at_its_own_width`.
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — the `live` verb gives VC `--model haiku` (Run F's argv); the doc
  names `vc` and `S0C`.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — `rig_rehearsal_vc_reads_its_first_frame_at_80x24_and_stops_with_no_key`.
(Steps 1–16's files from runs 2026-11-18T01-02-09Z and 2026-11-18T01-39-23Z stand uncommitted beside these.)

## Gates (trail `.andromeda/runs/2026-11-18T06-24-01Z-implement/gate-2026-11-18-trust-record-row-read-live.json`)
- Block, first firing: `entries 18 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 11`, with one iteration and
  no red. In entry 4: `PASS rig::screen::tests::a_sized_model_reads_its_rows_at_its_own_width` and
  `PASS rig::tests::the_arm_words_and_labels_parse`. In entry 5: `PASS [1.126s] … rig_rehearsal_vc_reads_its_first_frame_at_80x24_and_stops_with_no_key`
  and `PASS [52.290s] … verify_window_without_screens_fails_every_interactive_row` (under the 80 s kill).
- Control (`evidence/control-3.txt`): `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed`. `vc-nokey` fired on entry 5 with `--timeout 900`. With the guard removed, the named case alone FAILED
  (`left: Some("ctrl-c 1") right: Some("killed-no-key")`), with no TIMEOUT line.
- Scope `clean`, hygiene `clean`, and the land (tx 2609) all came before the start, as step 19 asks.
- Live round (`evidence/round-063211Z.txt`): `round: COMPLETE · legs fired 1/1 · entries 18 · green 4 · red 0`. The
  install was read by path (`2.1.325 (Claude Code)`), and the census read 0 before and after. `mise ls claude` read
  2.1.327 as `latest`; it was not used.
- No `gate.py guard` entry. No deferral. No matrix ref: the chunk owns no coverage row, and no capability is claimed.
- Smoke (P3): skipped. No product boot-path changed (the rig is test-only), and the plan's smoke entries are operator
  entries, not run because implement stops before the operator pass.

## What was surfaced (`evidence/live-starts.md` §S0C)
- **The reading:** `screen: box`, `box: up`, `modal: none`, `trust: absent`, `facts: bytes=1390 alive=yes exit=null
  rows=10 box=yes last=661 lit=footer-agents@23,footer-cycle@23,rule@20,prompt@21 adj=yes first=banner col=80`,
  `settle-ms: 1062`. In closed words, the dialog is **none**: a banner, one non-modal `model-notice`, then the box
  holding the CLI's own placeholder hint. Options 0, selected none, owner action no, and our compiled literals match
  (as `box`). Nothing of the account owner's stood.
- **Set beside S0R-2 B and F:** the screens differ in every literal field. S0C has the box at rows 20–23 under a
  banner. B and F have one rule on row 1, with no prompt and no footer, which is the trust dialog's frame.
- **What is left of verify's shape:** the sequence (the print probe, then Run A killed at its trust dialog, before B)
  and the trust path (S0C's cwd sat at the seed's exact key; B–E are trusted only through the parent walk, and F only
  by verify's own record). Timing is argued against. Branch (ii), an account dialog, is argued against.

## Deviations from the plan, with justifications
- The `vc` rehearsal's `facts:` suffix oracle (` lit=footer-agents@23 adj=no first=text col=33`) was taken from the
  fake's measured B–F reading at 80×24 (the V rehearsal's first-run values), not typed from prose. The case passed on
  its first run with it, so the value is the fake's own output (testing.md 2026-10-16).
- `pty:` is printed on every arm, not on VC alone. The plan says «the summary gains `pty:`»; it is a closed fact from
  the spawn spec and no rehearsal pins the summary's line set.
- `the_arm_words_and_labels_parse` also asserts the arms' pty sizes and VC's trusted / unwrapped / no-vhome shape, so
  `pty_size` and `trusted` have an observable effect (testing.md 2026-09-24).
- Scope record: none needed (scope `clean`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig live S0C` and its one `claude` 2.1.325 child | a leg (entry 10) | terminated: the summary read `census: 0` and `stop: killed-no-key`; entry 11's census read `census: 0`; `gate.py census` at 06:34:01Z showed no 2.1.325 `claude` and no rig |
| gate entries 1–7 and 8–11 (cargo, nextest, harness, rig census) | this run | terminated (each entry's tree censused by the tool) |
| control clone `~/.cache/andromeda-control/0ebce078` and its firings | this run (`gate.py control`) | terminated; `clone removed` |
| 16 host rows (`claude` on the `latest` install: overseers, builders, sessions; `viola` from `~/.local/viola-product`, the viola-lab prototype and `~/.viola/bin`) | NOT DERIVED — the host's own sessions and other trees | left running: not this run's |

## Decisions & corrections
- The directive was followed: ONE start (S0C), its own ledger under `--cap 1 --spare 0`, ledgered with the clock's
  time first, no byte into any dialog (`keys: none`), stopped by the no-key group kill. S0R-3 was not made. Implement
  stopped before the operator pass. No scheduled-wake or timer tool was called.
- The newest install moved to 2.1.327 (`latest`) between the plan and the start. The start stayed on 2.1.325 by
  path, as named (verification-harness.md 2026-10-10, extended 2026-11-13).
