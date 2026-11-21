# P5 progress — revision 1 of 2026-11-20-harness-prefix-and-identity-floor-ledger-rows (stopped at the context alarm)

Written 2026-11-21 by the revision window before its close; the next window resumes P5 here (Setup 3 offers it: a
`plan` row, no `matrix-{marker}.json` in this run dir).

## Done
- Setup: masters rows; one pending chunk; offer (b) taken on the operator's word (`inputs#I9`, `relay-1.md`).
- Snapped: `inputs#I9` (the operator's word), `inputs#I10` (relay-harness-split.md, the founder's answers 1-4 and the
  overseer's items 5-7).
- P3 premise closure: scope rev 3 (tx 3569); fork F-R1 appended to the take-up's `founder-forks.md` (three test-plan
  rows outside §5 stating the floor's count); research checkpoint (evolve tx 3570).
- P4: plan rev 7 (tx 3571), then rev 8 (tx 3574: the `matrix#v2-25 notes` line moved out of Expected amendments, the
  planlint check-2 HIT resolved); plan checkpoint (evolve tx 3572).
- List sync: research rev 3 (tx 3573) with `## Revision 1`; `scope sync: PASS · delta +4 −1`.
- P5 step 1 so far:
  - `gate.py run --dry-run`: 35 entries parse; `--live-legs --dry-run` selects 13-20, no pre-flight refusal;
  - planlint: 1 HIT (check 2) resolved by rev 8; check 12 (c) INDETERMINATE (the steps are lettered R1-R6, F1-F2,
    C1-C6, B1, D1-D3, E1 — read by hand: every cited step exists);
  - `gate.py baseline`: entry 5 NOT SEEN (needs `new = true` written); new (key) 3, 4, 8, 9, 10, 22, 23, 24, 25;
  - `inputs.py verify`: 10 entries, 0 unparsed (I5 uncited: information);
  - `gate.py scope --lists-only`: parsed 26 items.
- Baseline firings of 3, 4, 5, 8, 9, 10, 22, 23, 24, 25 started (`gate.py run --entry {n}`, this run dir) — their
  lines are in this run's gate trail and its logs dir.

## Baseline readings (on the tree as the stopped run left it; lines verbatim)
```
  3 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 3.log
  4 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.01s · 14744 B → 4.log
  5 unit        green · exit 0 · 1.79s · 154644 B → 5.log
  8 unit        green · exit 0 · 2.09s · 6129 B → 8.log
  9 integration red · exit 0 ✗ (exit 100) · exit 100 · 52.37s · 11446 B → 9.log
 10 integration green · exit 0 · 9.23s · 1146 B → 10.log
 22 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.03s · 541 B → 22.log
 23 probe       recorded · exit 1 · 0.03s · 526 B → 23.log
 24 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 24.log
 25 probe       red · exit 0 ✗ (exit 2) · exit 2 · 0.0s · 67 B → 25.log
```
Expected reds: 3 and 4 (the cut not yet made), 9 (`cli_verify` at 29 rows while the plan's oracles are 28 — its
cause still to be read by `gate.py show --n 9` before the baseline is written), 22/23/25 (no `HG` stamp or record
yet).

**NOT expected — a finding for the overseer before anything else (entry 24):** the live home's
`~/.viola/ledger/stamps.json` no longer equals the sha kept in `evidence/live-stamp.sha256` since `HF`'s round
(prefixes now `c464903cea48`, kept `598764d7579a`). Its mtime reads 2026-11-21T07:24:08Z, after `HF`'s after-read at
07:00Z read `live-stamp: unmoved`, in the window of the host's restart. This session wrote nothing under `~/.viola`
and started no `viola verify` there. Who wrote it (the pinned product's own start, the operator) is unmeasured
here. Until it is named, entry 24 (`live-stamp: unmoved` after `HG`) reads red by construction — the kept reading
needs the overseer's ruling (re-read the sha before `HG`, or a named cause), never a silent rewrite of the evidence
file.

## Resumed 2026-11-21T08:04Z (the next window) — owed items 1–4 DONE
- `inputs#I11` snapped (relay-harness-rev-p5.md): the stamp moved by the overseer's ordered verify of 2.1.330; F-R1
  answered (a) as a delegate's confirm, PROVISIONAL — written beside the fork in the take-up's `founder-forks.md`.
- Plan rev 9 (tx 3576) and rev 10 (tx 3577): entry 17 re-takes the sha into `live-stamp-HG.sha256`, entry 24 reads it;
  baselines written; entry 5 `new`. Entry 9's cause: 5 stamped-home cases failing on `cross-session-prefix` alone.
- Controls: `p5-controls.md`. Check set re-run on rev 10, all pass; validation-1 aligned.
- Card: `p5-card-1.md`, stopped there for the word. After the word: step 4 (v2-26 re-claim, v2-25 un-claim + note),
  step 5 land, the validate checkpoint.

## Closed 2026-11-21T08:07Z — the word taken, the phase ended
- The overseer's `yes` on `p5-card-1.md` (plan rev 10), no note: `inputs#I12`.
- Step 4: v2-26 re-claimed (`acceptance-v2-26.md`, `note-v2-26.md`); v2-25 un-claimed, noted (`note-v2-25.md`); audit:
  v2-26 cited in `## Acceptance Criteria`, no affordance cap.
- Step 5: `outputs land` — no field payload in the run (no P2 batch in a revision). Validate checkpoint: evolve tx 3582.
- Next: `/andromeda-implement` (not started here).

## Owed, in order (as the earlier window left it)
1. Read each baseline line (`gate.py show --run-dir {this run dir} --marker {marker} --n {n}`), write `baseline` on
   each new non-leg entry and `new = true` on entry 5 (plan working copy, then `put --expect-rev 8`).
2. The clone controls (validation.md check 4 (9)): a `git clone --shared` at HEAD `b3eb184c` in the session scratchpad;
   inputs by `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ba7939b8-3128-49f4-a739-0eb13ce1c82c/scratchpad/p5_rev1_inputs.py {clone} pass|fail|clean`
   (written, not yet run); fire entries 3, 4, 19, 20, 22, 24, 25 as themselves (`--docs-root . --entry {n}
   --project-root {clone}`); record the table in `{this run dir}/p5-controls.md`. No recursive delete of the clone.
3. Re-run the whole check set (any plan edit re-runs it), validation-1 against scope rev 3.
4. Write `p5-card-1.md` (the plan render at `.andromeda/cache/docs/{marker}/plan.md`; the concretization preview for
   v2-26 — `HG`, 28 rows, thirteen floor names, four non-identity names; v2-25 named for un-claim after the word; F-R1
   as its own question; `## Beyond the plan`: the new entry minted at the wrap for the cross-session row (standing
   rows of the take-up's card 4 read and marked), then stop at the card.
