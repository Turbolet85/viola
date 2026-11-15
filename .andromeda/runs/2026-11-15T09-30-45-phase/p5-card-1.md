# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's fourth revision (rev 8 → rev 10)

Phase run `.andromeda/runs/2026-11-15T09-30-45-phase`. The word that started it: the operator's by the delegate overseer
(inputs#I20). It rests on the founder's own word on the last two starts, «Диагноз, затем повтор (Recommended)», with the
overseer's bindings (inputs#I21), and on the overseer's revision brief (inputs#I22). No row read, no research, no live
start. Starts spent: 7 of 9.

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md`.
No plan stands in the chunk's folder. Writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-live-round-on-the-trimmed-product`.

## What the revision does
1. **The order and the cap's wording (the founder's word, inputs#I21).** Entry 11 (`live-unwrapped`) fires next, as start
   8. It keeps its screen, and that screen is the diagnosis of the dialog 2.1.325 raises in a fresh cwd. **Entry 12 now
   runs `live-hold.py live-wrapped`.** It is entry 10's RE-FIRE on the fixed rig, start 9, its own reading, in place of
   the MCP tap. This is the one bend of «a red step is not re-fired», and entry 12 itself is never re-fired. Entry 10
   stays RED as fired. The tap's `live` is built but no entry fires it. CARRY 1 goes to «The 2.1.325 recorded fixture
   set». The cap stays 9: verify 6 · wrapped 1 (red) · unwrapped 1 · re-fire 1.
   Entry numbers do not move. Entry 12's slot takes the re-fire, and entry 7 stays the tap's no-start rehearsal.
2. **The gate before entry 12 (the overseer's binding).** This is the new step 3d. Entry 12 has its own word, given
   only after two things: the overseer has read entry 11's kept frames, and the rig's trust step has been rehearsed
   under the fake agent against that screen's form. The same rehearsal reads whether the kept home, with entry 10's
   killed `round` instance in it, lets the re-fire start; a refusal there is a rig finding, not a spent start.
   **Start 9 is not spent** if either holds:
   - the frames show a dialog the pty word cannot pass (at most 2 down-arrows and 1 Enter, only on the trust literal);
   - the trust step reads `unknown-modal`.
3. **My three lines:**
   - **Step 3:** «only while the drained bytes hold the compiled trust literal» becomes «only while the screen the
     drained bytes build (the rig's VT screen model) holds the compiled trust literal, the Enter only on a `selected`
     read off that screen».
   - **Implementation note** (was line 478): a confirmed send reads `empty`; `not-delivered`/`input-not-ready` reads
     `unread`; another `not-delivered` or a `wheel` line `human` reads `x-stood`.
   - **The kept screens:** step 3c, below.
4. **The kept screens: a content rule (step 3c).**
   - Frames are written only under `target/live-round-screens/`: 0700/0600, gitignored by `.gitignore:16` `target/`.
     They never go in the work dir or a run dir, and are never landed or committed. They are the operator's to read and
     to delete.
   - What lands is `evidence/screens-summary.ndjson`: one line per frame, closed words only.
     - `found` is one of `trust-dialog` · `input-box` · `question` · `none` · `unknown-modal`.
     - `selected` is the `❯` row's option index or `null`.
     - Each line also carries `rows`, `sha256`, the subcommand, the arm and the rig's own label.
   - A new no-start entry 22, `live-read.py screens-check`, checks the summary's closed form. It also checks that no
     `screens-*.json` stands in the work dir or a run dir, and that each local frame still standing hashes to its line.
     A frame the operator deleted reads `deleted`, never red.
   - The split is rehearsed under the fake agent: frames present locally and the summary clean. Must-fail controls: a
     screen-text value in the summary, and a planted `screens-*.json`.
   - The rule lands in «New files» (local-only row), in a security criterion and in a Constraints line. Entry 18
     (hygiene) is fired with the frames in place.
5. **The criteria name which firing they read.** v2-03 and v2-02 now read entry 12 (the re-fire), with entry 10's first
   firing red as fired named beside it. A new criterion covers two cases: entry 12 not spent, or entry 12 red. Either
   way v2-02, v2-03 and CARRY 3's wrapped side stay unread and gated. The CARRY 1 criterion becomes «leaves the round».
   CARRY 3 reads entries 11 and 12.

/implement re-enters at step 3c. It builds the split and `screens-check` and takes their readings. Then it re-fires
entries 1–8, 21 and 22 and stops before entry 11. After entry 11 it stops again, for step 3d.

Also moved, as consequences:
- the Goal;
- Metadata «Revised (3)» and «Revised (4)»;
- step 7's re-entry line and step 8's recording sentence;
- the gate notes of entries 7, 10, 11, 12 and 15;
- the live-start counts note;
- a Constraints line on the re-fire.

The scope (rev 6) gains two `[inferred, closed]` bullets (inputs#I21, inputs#I22), and CARRY 1 gains a
`premise-corrected`. Research (rev 3) has its two lists rewritten to the revised write set:
- new: the evidence records, `screens-summary.ndjson`, `target/live-round-screens/` (local);
- modify: `crates/viola-core/src/lib.rs` only.

## Top acceptance criteria
- (arch) matrix#v2-03: entry 12 `live-wrapped: green` (entry 10's re-fire, start 9), entry 10's first firing red as fired.
  The rest is unchanged: the stamp on entry 21, the start order, 0 `unable: `/`hint: `, the `x` and the wheel, the name
  refusals, the child gone in 10 s.
- (arch) matrix#v2-02: entry 12's hold reading near 3600 s, then the `hold:` line settles the mark.
- (arch) Entry 12 not spent, or red: v2-02, v2-03 and CARRY 3's wrapped side unread, both caps gated, no start past 9.
- (security) Frames only under `target/live-round-screens/` (local). Entry 22 `screens-check: clean` and entry 18 `hygiene: clean`, both with
  the frames in place.
- (tests) CARRY 1 leaves the round: no tap start, the 2.1.302 fixture untouched.

## Test commands
22 entries. The only new one is 22 (`screens-check`). Its baseline was read this run: `red · exit 1`, `live-read.py`
prints its usage, the red the plan predicts. Entry 12's `run` is now `live-hold.py live-wrapped` (leg operator, timeout
4500).

## Mechanical checks (rev 10)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 22 entries |
| `gate.py baseline` | exit 0: seen green 1 · not seen 0 · new 14 · leg 7 |
| entry 22 baseline | fired, red as predicted, recorded |
| `planlint.py check` | `hits: 0`. 12 (d)'s five claims each hold, including line 487's «not touched» on `fixtures/claude/2.1.302/Mcp.tools-list-1.json`: step 4's rehearsal only reads it |
| `inputs.py verify` | 22 entries · drifted 0 · unparsed 0. I20–I22 are snapped and cited. The uncited rows (I2, I7, I8, I13–I15, I18) are words the trail and claim notes carry |
| `gate.py scope --lists-only` | parsed 3 items |

Absence assertions were run over the plan's own names:
- `screens-*.json` under the chunk's work dir and every run dir: 0;
- `target/live-round-screens/` ignored (`git check-ignore` exit 0);
- a sweep of every sentence naming entry 10, entry 12, the tap, `Mcp.tools-list` or CARRY 1 found no stale sentence
  left.

Validation-1: aligned. The intent's wrapped start and near-ceiling hold are still what v2-03 and v2-02 read, now on the
re-fire.

## Concretization preview (re-claims after the word, each with a dated `--note-file`)
- **v2-03** `acceptance`: «One live wrapped start … (plan entry 10, `live-wrapped: green`), …»
  → «… (plan entry 12, `live-wrapped: green` — plan entry 10's re-fire on the fixed rig, start 9, the founder's own
  word inputs#I21; plan entry 10's first firing red as fired, no `session-start`), …». Every other clause is unchanged,
  word for word.
- **v2-02** `acceptance`: «… stands on record (plan entry 10, on the CLI whose …» → «… stands on record (plan entry 12 —
  plan entry 10's re-fire, start 9, the founder's own word inputs#I21; plan entry 10's first firing red as fired, no
  hold — on the CLI whose …». Every other clause is unchanged.
- Neither is weakened: the same assertions are read on the re-fire. Both stay gated and keep their `gated` text.
  Nothing is released.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | «The 2.1.325 recorded fixture set» now owns CARRY 1 WHOLE: the live `claude -p` re-record through `viola-mcp-tap` on 2.1.325 (its one live start), the `list` description read without «and the usage budget», and its `Mcp.tools-list-1.json` placed in the new set. The round makes no tap start (the founder's own word, inputs#I21). CARRY 2's recording stays as card 1 of `2026-11-15T08-48-39-phase` widened it | the existing CARRY row, widened again | the wrap's route-resolve |

Apply? (yes / review / cancel)
