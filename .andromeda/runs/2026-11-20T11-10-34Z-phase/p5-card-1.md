# P5 review — 2026-11-20-permissionrequest-question-answered: «PermissionRequest question answered»

**The plan to read:** `.andromeda/cache/docs/2026-11-20-permissionrequest-question-answered/plan.md` (rev 3).
- It is the render of `doc:2026-11-20-permissionrequest-question-answered/plan`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- To see the chunk's records as a diff:
  `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-permissionrequest-question-answered`.

**Basis:** the founder's answers Q1, Q2, Q3, the cap and «verified», plus the overseer's technical answers 7–12
(`inputs#I3`, appended to `founder-forks.md`, snapped as `inputs#I4`).

## Goal
A question the CLI first raises through PermissionRequest, with no armed PreToolUse dialog matching it, is held and
answered. This applies on a verified CLI with the driver at the wheel. The driver's `answer` makes the hook print
PermissionRequest `allow` + `updatedInput` (the input plus `answers` and `annotations`). A human key, an expired hold
and an unverified CLI leave it to the human, as today.

The body is relied on through a 27th ledger row, `permission-request-question`:
- Its probe is a fourth compiled prompt, last in Run C. Its PreToolUse is left unanswered, and its PermissionRequest
  is answered by a sixth closed probe id.
- `question-answer`'s clause is scoped to its own turn.

2.1.328 alone is re-recorded by ONE live start. 2.1.287, 2.1.302 and 2.1.325 go drift-only, and both default
constants move to 2.1.328.

## Phases (the order matters)
- **A — the product change on the 26-row tree,** whole workspace green:
  - `dialog.rs`: the new arm;
  - `run/dialog.rs`: `unanswerable` removed;
  - Path 4 rewritten;
  - control `pr-question-arm`.
- **B — the row and its probe:**
  - `ledger.rs`, Run C's list, the count literals;
  - the rig's `PQ` label with its kept `question-stamp.json` and a summary line for the new row;
  - lint, crate-scoped units, control `pq-row-answers`.
- **C — the live start `S1`,** fired under the rig label `PQ`. The rig's own `S1` label is bound to its `TS` arm
  (mod.rs:192).
  - claude 2.1.328 by path, `viola verify --record`, the VR arm, the rig's private config and scratch home.
  - Only verify's compiled pastes are typed. No key and no byte goes into any CLI dialog.
  - `--cap 1 --spare 1`. The spare `PQ-2` opens only through the revision door, after S1's red cause is read and
    mended.
  - Before the start: the host read by hand, the install, the census, the build hash, the kept record and stamp
    absent, and the live home stamp sha.
  - After the start: the operator census, the kept stamp at `27 ok`, `live-stamp: unmoved`, and the record's names
    differing from the committed set by exactly 4 lines.
- **D — the landing:**
  - the record copied whole over `fixtures/claude/2.1.328/`, keeping `Mcp.tools-list-1.json`;
  - the lists and constants moved, and every §V site;
  - the drift and hygiene tests re-keyed;
  - then the whole unit, integration and E2E layers and the boot smoke at 2.1.328.
- **E — the operator pass:** pre-push, hygiene, push, CI.

## Top acceptance criteria
- **`matrix#v2-20`:**
  - Path 4 answers the arm-less question by id with the `allow` + `updatedInput` body (phase A, and again on 2.1.328's
    `question-permission-1`).
  - The human-wheel, unverified and pending cases leave it to the human.
  - The row reads `pass` in `contract_ledger_probes` over 2.1.328, and live in `S1` (`pass-count: 27/27`).
- The body is pinned by literal and snapshot. Both controls read «holds».
- `S1` is green on its atoms within the cap, and every after-read holds. A red `S1` stops implement before the landing
  and is recorded as read.
- `diff -r` of the record against the landed set prints nothing.
- `permission_mode` of the recorded question PermissionRequest is recorded as a READING, with no atom.

## Touchpoints
- **New files (4):** the insta snapshot and the three `question-permission-1` variants.
- **Files to modify (38):**
  - product: `dialog.rs`, `run/dialog.rs`, `ledger.rs`, `verify/typed/runs.rs`, `verify.rs`;
  - test-only: the rig and the harness;
  - the ledger test files and the count-literal files;
  - the §V sites, the two fake scripts, and the drift and hygiene tests.
- No master is a touchpoint.

## The version move (`inputs#I3` item 8), counted by file (§V)
- **runs on 2.1.328 — 18 files**, the two fake scripts among them;
- **drift-only reading, still asserting — 4 files:**
  - `contract_ledger_probes.rs`;
  - `dialog.rs`'s tests pinned to 2.1.287's relayed files;
  - `contract_fixture_hygiene.rs`;
  - `contract_fake_agent_drift.rs`;
- **unchanged — 37 files.**
- **Would silently stop asserting:** three `tests/cli_revive.rs` tests, but only if `FIXTURE_SESSION` were left at the
  2.1.287 id. Step 24 moves it to the landed id, and every other affected test fails loudly.

## Test commands
34 entries.
- Phase A: entries 1–6.
- Phase B: 7–11.
- The round: 12–16, then `S1` (17), then the operator census (18).
- The after-reads: 19–22.
- The landing: 23–27.
- The smoke: 28–30, operator.
- The operator pass: 31–34.

Baselines on the untouched tree:
- 10 is green (95 run) and 11 is green (1 093 run).
- 19–23 read red on absent inputs.
- 24 is `recorded`.
- 19–22 each took a must-pass green control in a scratch clone (`p5-controls.md`).
- The other twelve non-leg entries are `seen green` at the last wrap.

## Concretization preview — `v2-20` (method e2e)
- **Now:** «A question first raised by a permission request is answered by the driver with a body carrying the
  answers, never left to the human; the new ledger row has its probe, and the re-stamped CLI versions pass it.»
- **Proposed:**
  - On a verified CLI with the driver at the wheel, a question first raised by PermissionRequest (no matching armed
    dialog) is held. `viola answer` by its id with `answers`/`annotations` makes the hook print
    `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow","updatedInput":<input + answers + annotations>}}}`.
  - Path 4 `path4_permission_is_logged_once_woken_and_answered_by_id` proves it over the fake agent, on 2.1.328's
    recorded `question-permission-1`.
  - A human key, an expired hold or an unverified CLI leaves it to the human: run/dialog.rs
    `continuation_raised_as_a_question_is_left_to_the_human_when`.
  - The relied row `permission-request-question` has its Run C probe. It reads `pass` in `contract_ledger_probes` over
    every stamped set (2.1.328, the version the founder chose re-stamped, `inputs#I3` item 3) and live in `S1` on
    2.1.328 (`permission-request-question: pass`).
- **The never-weaken read:** «the re-stamped CLI versions» narrows to the one version the founder's Q3 named.
  «never left to the human» keeps the human's own key, an expiry and an unverified CLI as today: Q1 option 1 as he
  chose it.

## Leans, and where a letter was wrong or silent
- **The overseer's item 10 cannot hold whole.** It asks for green before the start, but no committed set can stamp
  the 27th row before `S1`'s record lands, and the founder declined synthetic variants. So whole-workspace green is
  taken in phase A, phase B is lint, crate units and controls, and everything re-runs after the landing.
- **My P3 block said `contract_fake_agent_drift` keeps replaying the three sets unchanged. That was wrong.** Its two
  verify-exit-0 assertions (:262-266, :362-367) and the hygiene walk's fixed twelve-name list (:791-834) are re-keyed
  in steps 25a–25b. Found at P4 by a delegated read of the tree, then spot-checked.
- **The 2.1.287-only relayed variants are used by `path4.json`, `path4-permission.json` and six test sites.** These
  re-point to 2.1.328 variants, the question ones to the new recording.
- **The strict-kind residual** is written as an `except` on the architecture row replacing
  `decision/permission-request-question-stays-null`. No route entry would consume a CARRY. The alternative is a CARRY
  on «Version done-check». Say which.
- **The rig label is `PQ`.** It is a lean on the code line `Arm::of_label`.
- **Size:** the plan's prose is about 560 lines, past the template's ~400. That is the §V list and the four phases.
  Splitting was not proposed, since one implement window holds the code. The live start is one leg.
- **Carry `cry-3qv2r3iy`:** acted on only at its next red. No entry runs the rehearsals before the landing.

## Tool lines, verbatim (directive item 11)
```
architecture: rows 968 · packs 10 · picked 293 (176 by every draw) · standing 31 · 81362 B
security-plan: rows 623 · packs 7 · picked 273 (193 by every draw) · standing 7 · 74622 B
design-system: rows 318 · packs 4 · picked 87 (63 by every draw) · standing 20 · 21313 B
layout-templates: rows 223 · packs 3 · picked 72 (53 by every draw) · standing 2 · 15406 B
test-plan: rows 1355 · packs 14 · picked 335 (203 by every draw) · standing 73 · 82018 B
obs-plan: rows 1030 · packs 11 · picked 362 (217 by every draw) · standing 55 · 86892 B
a11y-plan: rows 235 · packs 3 · picked 109 (73 by every draw) · standing 15 · 28478 B
rows-union: 7 master(s) · picked 1531 of 4752 · rule any
arch-pick-p2.d2 · [RETYPED] line 88: decision/links-are-events-in-log is no row of the pack — read as decision/links-are-log-events, the pack's row 88
security-pick-p2.d3 · [RETYPED] line 2: contract/strict-modes-refusal/cli-frame-verb is no row of the pack — read as contract/strict-modes-refusal/cli-frame-verbs, the pack's row 2
tests-pick-p9.d2 · [RETYPED] line 64: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 64
tests-pick-p9.d2 · [RETYPED] line 70: decision/proptest-for-generated-data is no row of the pack — read as decision/proptest-strategies, the pack's row 70
tests-pick-p11.d3 · [RETYPED] line 8: decision/no-test-passing-unguarded-decision-body is no row of the pack — read as decision/no-test-passing-unstamped-decision-body, the pack's row 8
```

- **Rows read by search, not whole:** the rows files sum to 390 091 B, past 250 000 B.
- **Every cited row** is in its rows file or standing list, except `(test-plan · decision/verify-window-80s-kill)`,
  looked up by `rec.py find`.
- **Re-spawns:** none. Dead draws: 0.
- **Non-zero exits:** baselines 19–23 red on absent inputs, and 24 exit 1 (report-only). No `UNPARSED`,
  `INDETERMINATE` or `NOT DERIVED` line.
- **Refused calls:** two Bash-guard refusals (a pipe after a tool, a `cd` into the run dir), each re-issued once in
  the guard's form.

## Beyond the plan
1. **A new route entry right after this chunk:** a new `claude` version on the host is stamped automatically in the
   product, as its own entry (the founder's direction, `inputs#I3` item 6). Proposed title «A new CLI version stamped
   automatically». This chunk's wrap takes it at route-resolve.

Apply? (yes / review / cancel)
