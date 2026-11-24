# Session Handoff

**Last Updated:** 2026-11-24T14:43:28Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** feat(2026-11-24-switching-a-session-s-model-and-effort): `viola switch` — a session's model and effort switched in place

## Position
- Done: 2026-11-24-switching-a-session-s-model-and-effort (`chk-ke32dhty`), wrap `2026-11-24T13-56-22Z-wrap`.
  `viola switch` is built and proven over the fake agent alone. No capability is claimed: v2-30 and v2-31 stay
  unclaimed, each with its dated note. No epoch closed.
- Still GATED, STANDING, each waiting on the operator's evening at the host:
  - «`viola focus`» (`chk-fw7hhw4r`): R3;
  - «`viola up`» (`chk-eq4ejvc6`): R4a and R4b;
  - «`viola adopt`» (`chk-ep3sqyx7`): R5a and R5b.
- Epoch 5 holds 9 taken-up chunks (6 complete, 3 gated) and no markerless entry. It closes at the last gated wrap.
- Next: «Console scaffold» (`chk-eg54zv5h`, Epoch 6). It is named and NOT started: Epoch 5's boundary comes first,
  and the overseer says when (`inputs#I8`).

## Work done
- P1: the report closed (tx 4783–4784) from the outcome file and the pass record. Directive `inputs#I6`.
- P2: 62 row operations landed over 7 masters (tx 4788–4795):
  - 25 duplicate adds rejected (check 2), plus 3 fit verdicts (1 not_master, 2 stands) and 1 route-carry add;
  - Q1 (the F1 widening, 7 operations) confirmed by the delegate overseer, PROVISIONAL (`inputs#I7`);
  - 50 leaf findings applied, 1 held (a time bound's figure in a leaf).
- P3: testing.md got 1 new entry and 1 extension.
- P5: 2 carries landed (`inputs#I8`):
  - R6a and R6b on «The fleet on the founder's host» (`cry-vqkz3zqo`);
  - v2-31's console leg on «Console actions» (`cry-mf2w3ioc`).

## Drift resolved
- The report named `cry-s4qhj2e3` as R6a and R6b's carry. That is «Model and effort at launch»'s launch-pair carry,
  so R6a and R6b got a carry of their own (above).
- test-plan: `decision/unverified-cli-gates-answer-only` was retired into `decision/unverified-cli-send-only`, which
  now names switch.

## Notes
- **Founder forks still owed**, every row resting on one landed PROVISIONAL and named with its fork in its sidecar
  entry:
  - this chunk's F1 (the widening: a `switch` channel method, a wrapper ending its own child), F3, F5, F6 and F7;
  - the earlier chunks' forks, as before.
  A reversal of F1 takes out the verb layer of `6281e1a5`
  (`.andromeda/runs/2026-11-24T13-56-22Z-wrap/escalation-card-q1.md`).
- **Deferred learnings:**
  - recurrence-despite-learning: CLAUDE.md «A timestamp written into any record … is read from `date -u`». Implement
    typed two stamps from estimate, a third recurrence.
  - Carried from earlier wraps: the same entry, and testing.md 2026-09-24 «wait on the exact line asserted».
  Review with `/andromeda-wrap-session --review` if any should be applied.
- **Known gap left by the overseer's word:** a11y-plan's two «Eight time bounds» list rows stay untouched
  (`inputs#I6` item 3), and the 10 s reap bound is a row of its own.
- **Carries:** `cry-z6ucncyv` (R5), `cry-tzxxzdkd` (R4) and `cry-fqzcimun` (R3) stand. `cry-vqkz3zqo` (R6a, R6b) is new.
- **Desk:** as before (the previous handoff's list). This wrap added nothing outside the tree.
- **Last failed command:** none.
- **Host:** PATH's claude is 2.1.330. Read `hostwatch.py read` before any stall red.
