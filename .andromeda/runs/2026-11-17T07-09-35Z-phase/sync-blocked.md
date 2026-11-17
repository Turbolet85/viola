# Revision 1 — stopped at the list sync (2026-11-17T07:15Z)

State: scope rev 5 and plan rev 7 are landed (the revision's text is complete, `gate.py run --dry-run` parses 17
entries). Research is untouched. P5 has not started; no card has been shown and no word taken.

Blocker: `gate.py scope --sync --chunk 2026-11-17-standing-live-rig --run-dir .andromeda/runs/2026-11-17T07-09-35Z-phase`
exits 2 with `not a {ISO}-phase run dir`. Its run-dir grammar (`~/.claude/skills/andromeda-tools/scripts/gate.py:2023`,
`RUN_NAME`) reads `YYYY-MM-DDTHH-MM-SS-{kind}`, with no `Z`. This chunk's take-up (`2026-11-17T05-54-05Z-phase`), its
stopped implement run (`2026-11-17T06-49-10Z-implement`) and this revision were all named with a `Z`. So renaming this
revision's dir alone still fails: the sync finds the stopped run by name (`sync_base`, the `*-implement` filter), and
`06-49-10Z-implement` does not read as an implement run.

What the sync would read: the plan's `## Codebase touchpoints` lists are unchanged between rev 6 and rev 7. Every
revision-1 edit names files already listed (`crates/viola-e2e/src/rig/{keys,screen,mod}.rs`,
`crates/viola-e2e/src/bin/viola-live-rig.rs`). The expected delta is `+0 −0`, and no file outside the lists is read.
That expectation is not a reading from the tool.

Options for the operator:
1. Fix the tool's grammar to take an optional `Z` (an Andromeda pipeline change, founder/tool side), then re-fire the
   sync here. (Recommended: it fixes the cause, and the next revision would hit the same refusal.)
2. Rename the two Z-named runs this sync reads (the stopped implement dir and this one) to the tool's form. This moves
   another run's record, and every citation of its path (the outcome file, plan rev 7, the trails) goes stale.
3. Waive check 13 for this revision on the operator's word: the delta stands as the hand reading above, the review
   card says so, and the research gets its `## Revision 1` section by hand.
