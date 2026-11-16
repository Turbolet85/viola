# P5 review — revision 2 of 2026-11-16-survey-key-keeps-the-wheel (changed lines only)

**Why:** `/andromeda-implement` stopped before the operator pass.
- S6 read red (`second-send: refused-human-typing`), and the cap was spent at 6 of 6
  (`.andromeda/runs/2026-11-16T15-28-30-implement/outcome-2026-11-16-survey-key-keeps-the-wheel.md`).
- The operator's word is `inputs#I10`.
- The founder's ruling since, «Выкинуть фичу» (2026-11-16T15:45:58Z, relayed verbatim by the overseer), is
  `inputs#I11`: the exemption is NOT built. It supersedes `inputs#I3` (forks 1–3) and `inputs#I6` («Только 0 и 3»).

**The plan, to open in an editor:** `.andromeda/cache/docs/2026-11-16-survey-key-keeps-the-wheel/plan.md` (rev 10).
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-survey-key-keeps-the-wheel`
  diffs the chunk's records.

## What changed
- **Goal.** The chunk is now the survey's grammar on claude 2.1.325, measured live and recorded. That covers S1–S6 as
  read, and S4–S6's two shape findings:
  - a driver send's typing closes the forced survey;
  - an argument-started turn is filed `origin: human`.

  No flag, no literals, no exemption, no ledger row. «run never reads screen content» stands untouched. A survey key
  takes the wheel like every key.
- **Scope (rev 6).**
  - Four bullets are `[premise-corrected]` against `inputs#I11`: the exemption, the compiled literals, the flag, and the
    live starts (now 6 of 6, spent).
  - Two `[inferred → verified]` bullets are added: the ruling, and the S4–S6 findings.
  - One bullet is verified: the stopped run's four-file diff (`git diff --stat -- src crates`).
  - The surfaces now read none in the product.
- **Steps.** A done block covers everything through S6. `/andromeda-implement` re-enters at step 1:
  1. **the rollback** — `git restore --` the four files (`screen.rs`, `run.rs`, `gate.rs`, `wheel.rs`); if a tool refuses
     it, stop and say so;
  2. entries 1–12;
  3. **the record** — a `## Revision 2` section in `evidence/live-starts.md`, then `rec.py land`;
  4. the operator pass (entries 13–15).

  No live start.
- **Touchpoints.** New: none. Modify: none (the chunk ends byte-equal to HEAD).
- **Gate fence:** 22 entries → 15.
  - **Left the block:** the survey tables (old 6, 7), the rehearsal (12), S1–S3 (13–15), the build (16), and S5 and S6
    (17, 18). The four survey controls are gone too.
  - **New entry 1 — the rollback probe.** `git diff --name-only 498b45333fd9` plus untracked files over `src crates
    tests fixtures`, then `| wc -l`, expecting `last line 0`. Its baseline was run now: red as predicted, last line `4`.
    It names the base sha, which clears check 4 (14).
  - **New entry 10 — the starts ledger.** `grep -c` of the start lines in `survey-starts.ndjson`, expecting
    `last line 6`. Baseline green, `6`: no seventh start.
  - **Carried entries.** The `run` strings are byte-equal: lint 2–4, unit 5–6, integration 7, the fixture and
    `CLAUDE_FORCE_DISPLAY_SURVEY` probes 8–9, `agent-run --unit` 11, `pre-push` 12, hygiene 13, push 14, CI 15. Their
    baselines are kept, and their notes no longer name the survey steps.
- **Acceptance.** Seven lines, no `matrix#` claim:
  - the rollback (entry 1);
  - green on the existing tests (2–7);
  - nothing recorded or forced (8–9);
  - the live rig: six starts, all folders absent, hygiene before the commit (10, 13);
  - the record (step 3);
  - the harness gates (11–12);
  - CI green (15).
- **Expected amendments (wrap):** none. The take-up's list of master widenings (plan rev 7, Implementation notes) is withdrawn. No master row names the
  survey (`rec.py find --text survey` over the 7 masters: 0 hits each), and no route `block` row names it (0 rows).
- **Constraints.** Each is rejected or not chosen, citing `inputs#I11`:
  - Epoch 4 with the code (not chosen);
  - S7 with `release` (not chosen);
  - any part of the exemption;
  - the fixture-set CARRY (withdrawn);
  - deleting the rig or the readings;
  - a cancel (not available).

## Checks (step 1, re-run whole after the last edit)
- dry-run: 15 entries parse. baseline: 5 seen green, 7 `new` (1 and 10 baseline-run now; 5–9 carried with their
  baselines), 3 operator.
- planlint: 0 hits. The two 12 (d) «byte-equal» lines (109: the carried `run` strings; 224: the four files after
  step 1) were read, and each holds after the chunk.
- inputs verify: 11 entries, 0 drifted. I4, I5 and I9 are uncited, as earlier runs left them.
- The lists parse (0 items). `scope --sync`: **PASS**, delta +0 −7 (plan rev 7 → 10; research rev 5).
- Check 7: skipped, nothing to modify. Check 11: no process or path is touched beyond the restore.
- Validation-1: aligned with scope rev 6. The frozen working line still reads «… joining the closed non-editing list».
  Scope's corrected bullets carry the ruling; the line is frozen and the wrap does not edit it.

Research: lists synced to the revision's delta (+0 −7), rev 5; not re-derived since the take-up's rev 2.

## `matrix#v2-21` — no longer claimed
The revised plan proves no part of its acceptance: no flag, no tables, no live key. After the word, at P5 step 4:
1. `matrix.py unclaim --id v2-21`, then
2. a dated `matrix.py note` naming the founder's word (`inputs#I11`) and this card.

**Where the `defer` runs** (`inputs#I11` item 3: «v2-21 уходит в deferred твоим словом»):
- Phase's letter places no `defer`.
- The wrap applies it, as this plan's own recorded disposition (`andromeda-wrap-session/references/route-resolve.md`,
  the «Unproved claim» row: «the plan's own — defer · un-claim · routed elsewhere — is applied instead»):
  `matrix.py defer --dir viola-0.2.0 --id v2-21`, its note citing `inputs#I11`.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-16T15-18-54-phase/p5-card-1.md`) An owner for the part left out
  (`inputs#I3`, fork 3). A CARRY on «The 2.1.325 recorded fixture set»:
  - the survey's capability-ledger row;
  - its `viola verify` Run F: a forced survey, signature read, no key;
  - the recorded `Screen.survey`;
  - the fake agent's survey-screen option;
  - an outer-PTY `tui_wheel` survey case.

  **ENDED by this revision** (`inputs#I11` item 3): withdrawn, since nothing is relied on. The wrap writes no such
  CARRY.
- **A deferral:** `matrix#v2-21` → `deferred` at the wrap, on the founder's word (`inputs#I11`), after this phase's
  un-claim (above).
- **What the wrap does with what still speaks of the exemption as coming:**
  - **No master row:** 0 hits, so nothing to amend.
  - **No route carry:** 0 `block` rows.
  - **The working entry's frozen line** («… joining the closed non-editing list»): left as written, since a frozen line
    is never edited. The wrap's report states that the chunk closed with the exemption not built.
  - **`v2-21`'s requirement line and the intent fact `fct-fthk66ll`** (the founder's word of 2026-11-08: the survey
    joins once measured): both stand as written. The cap is deferred, not rewritten. A deferred cap's re-wording is
    the next version's Phase A (`matrix.py refine` refuses `deferred`). The report names `inputs#I11` as the word
    that holds them open.

Apply? (yes / review / cancel)
