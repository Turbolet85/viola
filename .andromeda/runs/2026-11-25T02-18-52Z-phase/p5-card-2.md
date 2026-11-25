# P5 review card 2 — «Placing without moving the view» (`2026-11-25-placing-without-moving-the-view`)

Run `.andromeda/runs/2026-11-25T02-18-52Z-phase/` · it answers the review that lifted card 1's hold, and folds the
operator's own answers to forks M, V, W and L (his dialog 2026-11-25T06:22:26Z, relayed verbatim by overseer1,
snapped `inputs#I3`). The plan's render to open in an editor:
`.andromeda/cache/docs/2026-11-25-placing-without-moving-the-view/plan.md` (plan rev 4). No plan stands in a chunk's
folder; writing in that file changes no record until a `put`; `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-placing-without-moving-the-view` is a diff of
the chunk's records.

## The re-read (I sat idle)
Read from disk before anything was touched:
- the run dir: card 1, `founder-forks.md`, `relay-1.md`, `control-entry-13.md`, the trails;
- the store: plan rev 3, scope rev 3, research rev 2;
- the chunk record: `pending`.

Nothing had moved.

## His answers, and what each arm built
Every answer is the arm plan rev 3 was built on, so nothing returns to P4.
- **M → M1.** The eight placement words stay; no ninth word. This discharges T3's PROVISIONAL.
- **V → V1.** `untrusted-folder`, no stamp.
- **W → W1.** One window.
- **L → R7 this evening, with him at the host, unlocked.** There is no word for a run without him. v2-07's claim stays
  GATED on R7. His granted fixture re-record stays unspent.

His answers are appended under each fork in `founder-forks.md`.

## What changed, plan rev 3 → rev 4
Only whose word the plan cites; no step, touchpoint or gate `run` moved.
- **Metadata**: «each built on its PROVISIONAL arm … until his word» → his own answers M1 · V1 · W1 and R7 this
  evening, `inputs#I3`, «none of them is PROVISIONAL».
- **Goal**:
  - M1 and V1 now cite his answer;
  - the eight words read «no ninth word (… his M1 answer discharging its PROVISIONAL)»;
  - R7 reads «which he times for this evening at his desk, unlocked».
- **Step 5**: the module doc comment states the restore as his fork M answer, no longer «PROVISIONAL standing».
- **Step 17**: the report names each arm as built on his own answer.
- **Test Commands prose**: the unspent re-record cites his fork L answer.
- **Entry 13 `defer`** (the gated block): «… its moment his to name (founder-forks L)» → «… fired this evening with him at
  the host (his fork L answer, inputs#I3)». Its `run`, `expect` and `baseline` are unchanged.
- **Provenance**: `inputs#I3` added.
- **Constraints**:
  - the ninth word, and V2 / V3, are now «rejected by the operator»;
  - a W2 line was added.
- **Expected amendments (wrap)**:
  - the window-step widening now reads escalate `rul-bznresq5`: his OWN answer to fork M, one answer card confirmed by
    the overseer, written as his in the security-plan sidecar with the relay named;
  - the V1 rows now read ratified rows amended on his own answer, on the same card.

## The checks, re-run whole on rev 4
- `gate.py run --dry-run`: 22 entries, all parse.
- `gate.py scope --lists-only`: `scope lists: parsed 8 items (new 1 · modify 7 · derived 0)`.
- `inputs.py verify`: 3 entries. I1 and I3 unchanged, I2 n/a (a message); 0 uncited, 0 unparsed.
- `gate.py baseline`: 4 seen green, 0 not seen, 9 new (key), 9 leg. The baselines of plan rev 3 stand, because no
  `run` changed.
- Hand checks: 2, 3, 4 (2) (the boot smoke, entries 14–16), 4 (6)–(8), 8, 11 and 12 all read as on card 1.
  Validation-1 is aligned.
- planlint, whole:
  ```
  planlint v1.25 · 1a2cb894
  fence: 22 entries · plan: doc:2026-11-25-placing-without-moving-the-view/plan · research: doc:2026-11-25-placing-withou…
  check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none fetched (no network r…`
  check 9 · runner-only scope bullet: not read — the letter's
  check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: filled · ``.claude/rules/verification-harness.md` — r…`
  check 4 (5) · leg entries with a content atom: 5 · note without `atom from`: 0
  check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
  check 4 (13) · job-log reads by pattern: 0
  check 4 (14) · base-less git change reads outside the operator and round legs: 0
  check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
  check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
  check 4 (18) · entries reading a master's or a base's path the store now holds: 0
  check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
  check 4 (17) · entries reading a path the wrap writes: 0 of 12 asserting — listed, P5's read
  check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
  check 2 · ledger-note entries (matrix#): 0
  check 2 · entries naming no master and no matrix#: 0
  check 5 · placeholders outside code: 0
  check 12 (c) · step references the plan's steps lack: 0 of 6
  check 12 (d) · «unchanged» claims on a name another part of the plan names: 4 of 8 — listed, P5's read
    line 73 · «unchanged» · tests/cli_up.rs (Test Commands) · tests/cli_revive.rs (Test Commands)
    line 83 · «unchanged» · stamps.json (step 13)
    line 178 · «unchanged» · src/run/window.rs (Implementation Steps)
    line 386 · «unchanged» · window_step_moves_on_when_the_monitors_read_fails (step 6)
        window_step_keeps_placed_when_the_restore_fails (step 6)
    a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
  check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 1
  hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
  ```
  Each 12 (d) claim holds, as on card 1.

## Goal · criteria · touchpoints · test commands
As on card 1, unchanged except for whose word they cite:
- **New:** `scripts/r7-place.sh`.
- **Modify:** `src/run/window.rs`, `tests/cli_open.rs`, `src/cmd/verify.rs`, `tests/cli_verify.rs`,
  `tests/cli_version_gate.rs`, `tests/cross_exit_causes.rs`, `schemas/diag-line.v1.json`.
- **Gates:** 22 entries; controls `win-restore`, `win-stayed` and `vfy-confound`, `--timeout 900` each.

## Concretization preview — matrix#v2-07 (never weakened)
- **Now:** v2-07's acceptance as card 1 quoted it: the open clauses, the fakes' clauses, «Gated on R2 alone … a
  stand-in never verifies it».
- **Proposed:** every clause kept, word for word, and these added:
  - «… with the operator's active workspace unchanged;
  - over the fakes, the window step asks the focused monitor's active workspace before and after its one move and
    focuses that workspace back only when the move took it (`src/run/window.rs`
    `window_step_restores_the_active_workspace_the_move_took`, `window_step_restores_nothing_when_the_view_stayed`);
  - gated on R7 for the active-workspace clause: this chunk's `evidence/r7.md` holds
    `r7: green placement=placed active=unchanged` (R2's placement reading stands as cleared).»
- **The gated criterion's block, verbatim** (rev 4): «R7, ONE viola open placing a window live at the operator host,
  unlocked, the active workspace read before and after, fired this evening with him at the host (his fork L answer,
  inputs#I3)». A stand-in, stub or recorded artifact never verifies it.

## The claim's lines
Not fired. `matrix.py claim` and its gated file are P5 step 4's, after the word on this card.

## Every non-zero exit and refusal since card 1
None. Every call of the fold read exit 0.

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | Card 1's row 1 (his answers to M, V, L, W owed): **ended**. He answered, `inputs#I3`. | — |
| 2 | The granted ONE live re-record of the move's fixture is NOT spent by this chunk (M1 changes no form). It stays his to spend. (standing — `.andromeda/runs/2026-11-25T02-18-52Z-phase/p5-card-1.md`) | his grant, unspent |
| 3 | Card 1's row 3 (a new route entry «A silent move form, measured», only on M3): **ended**. He took M1. | — |
| 4 | The red stamp already kept for 2.1.331 on his own viola home is not touched by this chunk; clearing it is his at the desk. (standing — `.andromeda/runs/2026-11-25T02-18-52Z-phase/p5-card-1.md`) | his, at the desk |
| 5 | R7 is fired this evening by the operator entries 21–22, with him at the host, unlocked, and on no other word. | his, this evening |

Apply? (yes / review / cancel)
