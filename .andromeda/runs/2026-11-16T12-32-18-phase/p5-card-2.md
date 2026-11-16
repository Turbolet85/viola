# P5 review — 2026-11-16-survey-key-keeps-the-wheel · «Survey key keeps the wheel»

**Plan to open in an editor:** `.andromeda/cache/docs/2026-11-16-survey-key-keeps-the-wheel/plan.md` (rev 5,
rendered byte-equal from the store).

## What changed since `p5-card-1.md` (the operator's review, `inputs#I4`) — nothing else
- **Step 1, before each live start, the token window.** The credentials copy must never refresh its token. The
  hypothesis, unmeasured: a refresh from the copy rotates the refresh token and logs out every live session on this
  host.
  - The rig reads only the copy's `claudeAiOauth.expiresAt` (ms since the epoch) and keeps, prints or writes no
    token.
  - With ≥ 20 minutes left it prints `token-window: ok`.
  - Under 20 minutes, or with the field absent or not a number, it prints `token-window: REFUSED` and ends the start
    uncounted: before the ledger line, the folder and the spawn.
  - Each start is bounded well inside the window by its entry's `timeout` (300 s for S1–S3, 420 s for S4).
- **The rehearsal (entry 12) gains the case:** `expiresAt` 25 minutes out → `ok`; 19 minutes out, absent, and a
  string → `REFUSED`, each with no ledger line, folder or spawn.
- **The expect line:** entries 13, 14, 15 and 17 each add `'contains token-window: ok'`. No plan stands in a chunk's folder. Writing in that file changes no record until
a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-survey-key-keeps-the-wheel`
diffs the chunk's records.

## Goal
A key the human presses to answer or dismiss the claude survey leaves the wheel with its holder; every other key takes
it as today.
- The screen model's feed thread sets a survey flag while the survey's compiled literals stand on a screen that is not
  poisoned. The stdin observer reads it with no wait.
- An unknown, poisoned or stale screen fails toward the human.
- The rule holds on any CLI version, the exact literals its own guard.
- The ledger row is owed to «The 2.1.325 recorded fixture set».

These are the founder's own answers to P4 forks 1–3 (`inputs#I3`).

**Order of work:**
1. S1–S3 measure the survey's grammar live.
2. Implement HOLDS on a grammar card for his own word (step 4). He sees the exact keys before they are compiled.
3. Implement codes the rule.
4. S4 proves it end to end under `viola run` on the chunk's build.

The cap is 4 starts, keys `0` · Enter · `3` · Ctrl-C, the login by a credentials copy removed on every exit path
(`inputs#I2`).

## Top acceptance criteria
1. `matrix#v2-21`: in S4 (entry 17), after the key `0`, no `wheel{human,human-input}` follows; the next `viola send` is
   confirmed. In CI, entry 6's tables hold.
2. Only the ratified grammar joins; every other key, under the flag or not, still takes the wheel (entries 6, 8).
3. A poisoned or overflowed model clears the flag; the feed-panic passthrough stays green (entries 6, 8).
4. The stdin side never waits: one atomic load, bytes returned whole (entry 6).
5. Each live start reads `token-window: ok`, `folder: absent` and its ledger count; no fifth start (entries 13–15,
   17).

## Touchpoints
`src/run/wheel.rs` · `src/run/gate.rs` · `src/cmd/run.rs` (`pump_child`, the observer's argument only) ·
`crates/viola-agent-claude/src/screen.rs`. The rig is gitignored evidence.

## Test commands
21 entries:
- lint 1–3;
- unit 4–7 and 11;
- integration 8;
- probes 9 (no fixture change), 10 (the product never names `CLAUDE_FORCE_DISPLAY_SURVEY`) and 12 (the rig's
  rehearsal);
- operator 13–15 and 17 (S1–S4, each once);
- build 16;
- pre-push 18;
- the operator pass's hygiene, push and CI read (19–21).

No mutation run (`inputs#I1`). Four one-shot controls, each `--timeout 300` on its filtered entry:
`survey-flag`, `survey-exact`, `survey-poison`, `survey-options`.

**Baselines:**
- 4: 304/304 green.
- 5: 572/572 green.
- 6, 7: red, no tests to run yet (an absent input).
- 8: 76/76 green.
- 9, 10: green, each with a clone control read red on a minted input (`p5-controls.md`).
- 12: red, the rig is not written yet.
- 16: green.

## Concretization preview — `matrix#v2-21` (method `integration`)
- **Now:** «A key answering the CLI's session-feedback survey leaves the wheel with its holder once the survey's
  grammar is measured live; a prompt shape not measured still takes the wheel.»
- **Proposed claim-exit form:** «Measured live on claude 2.1.325 (S1–S3, `evidence/live-starts.md`) and ratified by the
  founder's own word (plan step 4):
  - in S4, under `viola run` on this chunk's build, the survey key `0` writes no `wheel{holder:"human",cause:"human-input"}`
    after its offset in `events.ndjson`, and the driver's next `viola send` is confirmed;
  - in CI, `observed_a_survey_key_under_the_flag_keeps_the_wheel` holds each ratified key form under the flag, while
    `observed_a_survey_key_without_the_flag_takes_the_wheel` and
    `observed_a_neighbour_of_a_survey_key_under_the_flag_takes_the_wheel` move the wheel for every other shape.»
- **Not weakened:** it keeps «measured live» and «a shape not measured still takes the wheel».
- **If S1 and S2 both show no survey:** the claim is un-made at the stop (step 3), with a dated note.

## Validation
- **Mechanical checks:** all clear, re-run whole after the review's edit (rev 5):
  - sections, paths and parse: dry-run 21 entries, exit 0;
  - baseline listing: seen green 5 · not seen 0 · new 9 · leg 7; no `run` changed, so the baselines stand;
  - planlint: `hits: 0`;
  - inputs: 4 entries, unchanged 2, n/a 2, uncited 0, unparsed 0;
  - scope lists: `parsed 4 items`;
  - check 7: the graph trace present.
- **Check 4 (2):** no smoke entry, stated in the prose. `pump_child` changes only the observer's argument, and entry 8's
  booted cases start through it.
- **Check 11 (b):** the 2026-11-15 rig's report and evidence were read. No recorded harm to the host. One recorded slip:
  a `/tmp` host path landed once in its journal (`fct-hgzo674a`). Step 1 now forbids any host path in a record.
- **Validation-1:** aligned with scope rev 4. That rev carries the fork answers and the live cap; the P3 closure
  corrected 2 premises.
- **Size:** 479 lines with the fence, about 310 of prose.

## Directive block (`inputs#I1` item 8)
- **The union's per-master lines, verbatim:**
  ```
  architecture: rows 947 · packs 10 · picked 145 (97 by every draw) · standing 16 · 42540 B
  security-plan: rows 603 · packs 7 · picked 86 (54 by every draw) · standing 11 · 25497 B
  design-system: rows 315 · packs 4 · picked 51 (33 by every draw) · standing 13 · 12476 B
  layout-templates: rows 222 · packs 3 · picked 34 (17 by every draw) · standing 0 · 6667 B
  test-plan: rows 1331 · packs 14 · picked 206 (117 by every draw) · standing 59 · 49478 B
  obs-plan: rows 1022 · packs 11 · picked 188 (95 by every draw) · standing 53 · 46551 B
  a11y-plan: rows 235 · packs 3 · picked 52 (39 by every draw) · standing 16 · 15055 B
  rows-union: 7 master(s) · picked 762 of 4675 · rule any
  ```
- **Rows read whole:** the seven rows files (198 264 B, under the 250 000 B bound), each standing list and each
  history file, once at P2.
- **Every cited row is in the union:** 18 handles checked by a scripted grep against the rows files and standing
  lists (all `in`); the other 10 Provenance handles were read in the whole rows files (architecture lines 4, 10, 22,
  37, 40, 47, 48, 141; obs 186; a11y 61; test-plan 104, 195). None is cited from outside them.
- **`[RETYPED]` lines, verbatim:**
  - `arch-pick-p4 · [RETYPED] line 16: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 16`
  - `tests-pick-p7 · [RETYPED] line 4: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 4`
  - `tests-pick-p9 · [RETYPED] line 73: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 73`
- **Re-spawns:** one. `obs-task · [NOT_NEEDED_REACHED] 'Revive, the claude agents probe, session-live checks and link or
  unlink: the chu… (104 chars)' names fixtures/claude`, re-spawned as `task-r1`: `VERDICT: 1 of 1 pass`. No dead draw.
- **Non-zero exits and refusals:**
  - the bash guard refused `cd` three times (re-issued as subshells or absolute paths), and refused a pipe of `gate.py` output into `grep`
    once (re-issued bare);
  - `rec.py get cap:…/v2-21` exit 1 inside a chained call — the `grep` of the working route for `v2-21` had no
    match, so no entry names it;
  - the graph query went to the background at 120 s while the rust plane regenerated (286 s); it then exited 0;
  - baselines: entry 10 first read red (4 existing `CLAUDE_CONFIG_DIR` test lines), and its probe was narrowed to
    the survey variable; entries 6, 7 read exit 4 and entry 12 exit 2 (absent inputs).
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none printed.
- **Where a letter was wrong or silent:**
  - The working entry named no capability id. `v2-21` was found by `git log -S` on the route commit (filed as
    take-up friction).
  - The phase letter has no form for a planned mid-implement HOLD on the founder's word. Step 4 writes it as a stop
    rule, after `rule rul-p2zl2kgl`.
  - The letter's live-leg form (`leg = 'live'`, `env`, `stop`, a round probe) does not fit a rig that starts its own
    capped processes. The starts are `operator` entries, fired once each.

## Beyond the plan
- An owner for the part left out (`inputs#I3`, fork 3). A CARRY on «The 2.1.325 recorded fixture set»:
  - the survey's capability-ledger row;
  - its `viola verify` Run F: a forced survey, signature read, no key;
  - the recorded `Screen.survey`;
  - the fake agent's survey-screen option;
  - an outer-PTY `tui_wheel` survey case.

Apply? (yes / review / cancel)
