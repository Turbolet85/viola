# P5 review — 2026-11-18-trust-record-row-read-live · Revision 1 (changed lines only)

The plan's render: `.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md` (rev 4). No plan stands in the
chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live` diffs the chunk's
records.

**Stop:** implement stopped before the operator pass on S0R RED: `7/23`; Runs B, C, D, F read `none` on one static,
folder-independent screen; Run A read `trust`. **Word:** inputs#I3 (the operator), inputs#I4 (the overseer's relay),
inputs#I5 (the analysis).

**Changed:**
- **Scope rev 4.** The common-cause hypothesis is closed `[premise-corrected]` against S0R. The named cause is the
  rig's first-run private config; its mend is the rig's seed, this chunk's. Unmeasured: (i) the notice stack laid out
  at 80×24, or (ii) an account dialog raised since 2026-11-17T16:39Z. S0R-2 is the first spare, and S0R-3 is left for
  the founder. Finer closed facts are added, and F2 is recorded.
- **Plan rev 4.**
  - A new section, «Revision 1», says what is done (steps 1–10, with evidence), what the start read, and the cause.
    It holds the table of six seed markers, each with the 2.1.325 bundle line that tests it and the value written:

    | Key | Value |
    |---|---|
    | `hasSeenAutoDefaultNotice` | `true` |
    | `hasSeenAutoModeEntryWarning` | `true` |
    | `lastReleaseNotesSeen` | the running version |
    | `lastOnboardingVersion` | the running version |
    | `lastClawdEntranceVersion` | the running version |
    | `migrationVersion` | `15` (`Ge=15` in the same module) |

  - Implement re-enters at **step 12**. Steps 12–15:
    - `seed_used_markers` in the V arm only, the running version read from the child program by path, and
      `seed-markers: written` read back from the disk;
    - per run, the four finer facts: each box literal's first row, rule-above-prompt, the first row's family
      (`rule` · `banner` · `text`), and the last column;
    - the facts carried into the stamp's `first_facts` and printed on the rig's `facts-<x>` line;
    - the cases for each.
  - Step 16 is **S0R-2**, under `--spare 1`, so the rig's ledger refuses S0R-3. Its ends: the floor held, then the
    operator pass; still `none` with the markers seeded, then a STOP with F1 updated and S0R-3 not spent; any other
    red, then a STOP.
  - **Test Commands:** entry 11 is now `live S0R-2 … --spare 1`, with the added atom `contains seed-markers: written`.
    Two new one-shot controls: `seed-markers` (entry 4) and `fs-adjacent` (entry 3), each `--timeout 900`. Entries
    1–10 and 12–19 are unchanged and keep their baselines.
  - **Acceptance:** four lines for Revision 1 (the seed, the facts, the two controls, S0R-2); S0R's reading recorded.
  - **Constraints:** no markers on other arms and no value from the founder's file; no product mend (that is F2);
    no S0R-3 tonight.
- **F2** is appended beside F1 in `.andromeda/runs/2026-11-18T01-02-09Z-implement/founder-forks.md`, F1 gaining
  this revision's update.

**Research:** lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2. The
`## Revision 1` section records the closure's reads: the bundle, the rig's kept frames by row family, the key names
of `~/.claude.json`.

**Checks:**
- The fence parses (19 entries). Baseline: 2 seen green, 5 new (carried), 12 legs.
- planlint: 0 hits. Check 12 (d) lists four «unchanged» claims, each read true.
- `inputs.py verify`: 5 entries, none drifted.
- `scope --sync` PASS, `--lists-only` parsed 14.
- Validation-1 aligned. Size warns at about 470 lines of prose, the steps already done included.

**Claims:** none (no coverage row; `matrix#v2-33` stays pooled).

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T00-06-23Z-phase/p5-card-2.md`) none
- F2, the product finding (verify in a first-run config at 80×24 reads its interactive rows `fail`): its options are
  the founder's, no owner route entry proposed. It is recorded in `founder-forks.md` only.
- S0R-3 is left for the founder's word (R6's second spare), not spent by this chunk tonight.

Apply? (yes / review / cancel)
