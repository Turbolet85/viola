# P5 review — 2026-11-18-trust-record-row-read-live · «Trust-record row read live» · card 2

The plan is `doc:2026-11-18-trust-record-row-read-live/plan` **rev 3**. It was revised on the overseer's review of
card 1, `inputs#I2`, which was snapped at P5. Its render is `.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md`.
No plan stands in a chunk's folder, and writing in that file changes no record until a `put`. To diff the chunk's
records, run `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live`.

## What the revision changed (inputs#I2)

### 1. The difference table is in the plan
It sits in the plan's `## Difference table`, built from the code at `f862f193` and the 2.1.325 bundle, with no start.
It compares S0 with the starts that reached the input box: R3 and R3D (wrapped, same private config, 210×45) and the
live home's own verify (80×24, the user's own config).

| # | The difference | Where | Predicts on B–F |
|---|---|---|---|
| D1 | argv: verify's capture plugins vs run's viola plugin; C and D add `--settings`; flag order | `typed.rs:77-78`, `runs.rs:96-194`, `run/mod.rs:137-148` | a plugin-load prompt: `none` |
| D2 | env: the same `plan_strip` over the same keep list; run adds only `VIOLA_*` | `verify.rs:181`, `isolate.rs:23-31` | none on its face. Excluded: the print probe passed under it |
| D3 | a near-empty private config | `isolate.rs:252-263` | a first-run notice: `none` |
| D4 | B–E in subfolders, trusted through the ancestor | `runs.rs:284-290` | `trust` on B–E |
| D5 | 80×24 vs R3's 210×45 | `run.rs:85` | together with D3: a box the predicate does not accept |
| D6 | read at 8.5 s from spawn; R3 had no bound on its start | `pty.rs:145-163` | a start slower than 8.5 s |
| D7 | Run A killed before B | `runs.rs:36-37` | a killed sibling's leftover blocks: nothing drawn |

**Likely cause on its face:** none of D1–D7. D2 is excluded; D5 alone is excluded by the live home's verify, and D3
alone by R3. What remains is a combination unique to S0.

**Testable before a start:** none of the causes can be tested on the fake agent or by a unit case. What IS tested
first is that each fact reads its known value on the fake (step 5).

### 2. `none` speaks
Beside each run's word and settle wall, verify keeps one row of facts in `measured.first_facts`. All of them come from
state the run already holds, and none is screen text:

| Fact | What it reads |
|---|---|
| `bytes` | bytes the child's terminal produced before the settle |
| `alive` | whether the child was alive at the read |
| `exit` | its exit status, recorded only when it had exited before the read |
| `rows` | count of non-empty rows of the settled screen |
| `box_any` | whether any input-box-family literal stood: a footer, a rule row or a prompt row |
| `last_byte_ms` | spawn to the last byte before the read |

- The modal-family fact is dropped: on a `none` word it always reads false, so it separates nothing.
- How the facts separate the candidates:
  - `box_any` splits K1 (a box the predicate refused) from K3 (no box: an unknown modal or notice).
  - `bytes` and `rows` mark K4 (nothing drawn).
  - `last_byte_ms` marks K5 (still drawing near 8.5 s).
  - `alive` and `exit` mark K6 (exited).
  - `rows: null` marks K7 (a poisoned screen).
- The V arm prints one `facts-x:` line per run.
- The fake cases read them first:
  - With screens: bytes above 0, alive, rows above 0, `box_any` on B–F.
  - Without screens: the fake draws nothing, which is the known shape of the K4 row.
- A new control, `fs-boxany`, guards the rule-row arm of `box_any`.

### 3. What a red start then does (step 11)
A red names ONE candidate from the table below, or «none of the listed» with its facts as read, before anything else.

| Candidate | word | box_any | bytes / rows | alive | last_byte | The mend's owner |
|---|---|---|---|---|---|---|
| K1 box refused at 80×24 (D3+D5) | none | yes | >0 / >0 | yes | early | the founder's: the predicate `run` shares, or the ratified 80×24 |
| K2 ancestor not honoured (D4) | trust on B–E | no | >0 | yes | early | the founder's: verify writing more records |
| K3 unknown modal/notice (D1/D3) | none | no | >0 / >0 | yes | early | the founder's: a new literal moves `run`'s gate, and naming the screen needs its text |
| K4 nothing drawn (D7) | none | no | 0 or rows 0 | yes | null/early | the founder's, but this chunk's if the leftover is in the rig's setup |
| K5 slow start (D6) | none | any | >0 | yes | ≥ ~8 000 ms | this chunk's: a verify-only first-screen bound |
| K6 exited (D1/D2) | exited | any | any | no + code | any | this chunk's if it is the rig's env list; else the founder's |
| K7 poisoned screen | none | no | rows null | yes | any | this chunk's: verify's feed |

- **A mend that is this chunk's** goes through a plan revision. Its re-entry spends `S0R-2`, and a second named and
  mended cause spends `S0R-3`.
- **A mend that is the founder's** is written to `founder-forks.md`. No spare is spent on it: the chunk wraps with the
  cause named.
- **«None of the listed»** spends no spare either.
- **A start lost to its shape** spends the next spare with no mend.
- **No spare is ever spent on an unnamed cause.**

### 4. A missing failing-rows line reads red
- The rig prints `failing: unread` and `rows: unread` when the stamp or its version entry cannot be read.
- The live entry adds three atoms beside the 21 floor `lacks`, so a missing line, an unread one, or a short stamp all
  read RED and never green by absence:
  - `contains failing:`
  - `lacks failing: unread`
  - `contains rows: 23`
- A rig unit case, `stamp_reading_reads_unread_for_an_absent_version`, holds it.

### The git root, measured from the bundle
- The CLI's `jt` (`find_git_root`) and `xvn` take a folder as a root when its `.git` stats as a directory or a file, or
  as a symbolic link resolving to one outside itself. `t4o` refuses a link. No content of `.git` is read.
- The real `.git/` directory the plan writes is therefore a root.
- If the CLI did not read it as one, B–E's trust reading is unchanged: the `SS` walk climbs to the seeded cwd anyway.
  The root moves only the imports approval and project-local state.

## Unchanged from card 1
- The six words and walls in `measured`.
- The git root as files, not a `git` spawn.
- The 80 s kill with its planted-hang pair.
- The `--record` witness.
- `-3` spares.
- `S0R`, `S0R-2` and `S0R-3`.
- The 21-row floor.
- The 14 touchpoints.
- The 19 gate entries and their baselines on the untouched tree:

  | Entry | Reading |
  |---|---|
  | 3 | green, 591 passed |
  | 4 | green, 2675 passed |
  | 5 | green, 73 passed |
  | 6 | green, 524 passed |
  | 7 | green, 320 passed |

  Their `run` text is unchanged, so the baselines stand.
- No capability is claimed, and `matrix#v2-33` stays pooled.

## Founder forks
None at planning. The red-start table routes each founder-owned mend to `founder-forks.md` at implement.

## Directive item 12 — verbatim (card 1's block stands; additions since)
- **Union:** the seven per-master lines and `rows-union: 7 master(s) · picked 836 of 4715 · rule any`, as on card 1.
- **Rows:**
  - All seven rows files were read whole (223 771 B).
  - Standing lists: four read whole, three by their `- S` lines.
  - History files: four read whole, three through a 260-character line cut.
- **Cited rows:** all 24 stand in the union. rev 3 cites no new row.
- **`[RETYPED]` lines and re-spawns:** none printed; one re-spawn, `design-pick-p2.d3 [EVERY_ROW]`, which passed.
- **Non-zero exits and refusals:**
  - The 4 bash-guard refusals.
  - `rec.py put --edit-file` exit 3 (0 hits).
  - The stamp-ahead hook refusal.
  - None new since card 1.
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none.
- **planlint on rev 3:** `hits: 0`. Its 12 (d) listings are consistent: `merge_stamp`'s signature is unchanged while
  `measured` grows, and the plan never asserts `~/.claude.json` untouched.
- **Where a letter was wrong or silent:**
  - (a) Setup 5a's pickaxe path now lies in the store.
  - (b) The phase skill has its own `evolve-system.md`, byte-equal to new-session's copy.
  - (c) The rig ledger's spare rule, which only P4's code read found.
  - (d) The new-session and phase letters are silent on the fan-out's three-part picker batch running past ten
    minutes a part. Not a fault; noted.

## Beyond the plan
none

Apply? (yes / review / cancel)
