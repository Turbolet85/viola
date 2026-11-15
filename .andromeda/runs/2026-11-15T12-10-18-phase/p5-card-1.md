# P5 card 1 — 2026-11-15-live-round-on-the-trimmed-product, the plan's eighth revision (rev 15 → rev 18)

Under a ONE-TIME override of the revision guard (pre-CI commit `4c02d2af` in HEAD), on the founder's own words
(inputs#I39): «Пусть еще проверяет уже в папке которой доверяет…» (12:01:48Z) and «Давай разовый обход чтоб не тянуть
резину» (12:08:07Z). Bound by inputs#I40 (only adds) and inputs#I41 (the tenth start as ruled). No row read, no
research, no live start. The plan's render is at `.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md`
(open it in an editor). No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-live-round-on-the-trimmed-product`
shows a diff of the chunk's records.

## What is added
- **Step 3g:**
  - (a) `founder-words.md` changes to `cap: 10 starts`.
  - (b) `live-hold.py live-trusted` (entry 27, LIVE, start 10 of 10, never re-fired):
    - It runs on the stamped kept home, with a fresh instance `trusted`, in `proj-20261115T095240Z`.
    - The rig reads the CLI's own trust record in `~/.claude.json` and nothing else there: one key, held as a boolean,
      never written. An absent or `false` record is refused, and nothing starts.
    - No key is typed before the `x`. If a dialog or modal stands, no key is typed and the entry reads red.
    - It reads its own `events.ndjson` lines 1–2, the strip fields' names, and the `x`'s own `human` wheel line, then
      the DEL and `release` → `driver`.
    - No send and no hold. The wrapper is ended as entry 12 ended it, under a 300 s kill deadline.
  - (c) `rehearse-trusted` (entry 26, no start), under the fake agent:
    - the granted arm reads green;
    - a record absent or `false`, and a name that isn't fresh, each mean no start;
    - a standing modal means no key typed, and the arm reads red.
- **Entries 26–30:**
  - 26 is the rehearsal; its baseline reads red, as predicted (the subcommand isn't built yet).
  - 27 is the live start.
  - 28–30 are the hygiene, the SECOND pre-CI commit pushed fast-forward, and its CI read (inputs#I40 (c)). Entries
    1–25 stand as fired.
- **Re-entry:** /implement re-enters at step 3g. It re-fires the no-start entries 1–8 and 21–25, fires 26, then
  re-fires 8 last. It STOPS before 27, which runs on its own word.
- **Criteria added:** entry 26 green; entry 27 fired once (its `start` row `starts` 10, no rig write into
  `~/.claude.json`); entries 28–30.
- **Constraint added:** entry 27 is never re-fired, and nothing past cap 10. It is the ONE start allowed in an earlier
  start's cwd, by the founder's word.
- **Expected amendment (wrap), added:** the security-plan line for the rig's read-only trust-record check, under pattern
  «Boundary widening». The read itself is the overseer's binding, so it is a delegate's answer and the wrap escalates it
  if it reads it as a widening.

## Existing lines this revision touches (all inside inputs#I40 (a))
1. **The Goal** names both capabilities again. v2-03 rests on entry 27 plus entry 12; a red 27 keeps revision 7's
   un-claim and CARRY.
2. **The v2-03 criterion is returned**, and each clause names the entry it rests on (below).
3. **The entry-12-red criterion**, count words only: «start 9 of 9 spent» became «start 9 spent», and «No start past 9»
   became «No start past 10: the tenth is entry 27's alone».
4. **Implementation notes**, the live-start count: a sentence was appended (+1 = 10).
5. **One scope bullet** gets a second `[premise-corrected]` tag (inputs#I39), and one closed `[inferred]` bullet is
   added.

**One standing line this revision meets but does not edit:** the constraint «A live start reusing, removing or
renaming what an earlier start left in the kept home (its `proj/`, its instance) — never».
- Entry 27 runs in entry 11's `proj-20261115T095240Z`, as the founder ruled, so it uses an earlier start's cwd.
- The new constraint line names entry 27 as the one exception, and the old line stands for every other start.
- If you read this as touching an existing constraint beyond the override, say so, and this stops.

## A finding behind the clause split
The rig never read «names only in its strip fields» at entry 12:
- `live-read.py:279–291` reads `cli_verified` and `pty_backend`, but no strip field;
- `process-start{claude-child}` carries `env_stripped_known` and `env_kept` (`src/run/mod.rs:60–80`).

So that clause rests on entry 27 too, not on entry 12. It was recorded as an evolve friction (`contract.premise-falsified`).

## Concretization preview — matrix#v2-03 (re-claimed after the word, with a dated note; it stays gated)
**Current:** one live wrapped start (entry 12, `live-wrapped: green`) reads every clause.

**Proposed:** proved across two entries on the test home whose 2.1.325 stamp entry 21 reads `pass`.
- **Entry 27** (`live-trusted: green`, start 10, the founder's word inputs#I39):
  - `events.ndjson` lines 1–2 read `wheel{cause:start}` then `session-start`, with no `budget-gate` kind;
  - the `x` is echoed within 2 s, and the file's first `wheel` line holder `human` is the `x`'s own;
  - `process-start{claude-child}` holds names only in `env_stripped_known` and `env_kept`.
- **Entry 12** (start 9, red as fired on those clauses alone; each clause here read green there):
  - `cli_verified:true` and `pty_backend:"openpty"`;
  - the pinned copy, and `plugin/<key>/` written at the start;
  - 0 `unable: ` and 0 `hint: ` bytes;
  - `already-live` and `stale-heartbeat`, with no child spawned;
  - the child gone within 10 s of the wrapper's SIGKILL.
- **Fake-agent witnesses:** entries 5, 16 and 26.
- **If entry 27 reads red:** no ref is written, v2-03 is un-claimed at the wrap with a dated note, and revision 7's
  CARRY owns it.

The outcome isn't weakened: every clause is still read live, and one more clause is read than entry 12 read.
matrix#v2-02 is unchanged.

## Mechanical checks (rev 18, re-run whole after the last edit)
| check | result |
|---|---|
| `gate.py run --dry-run` | exit 0, 30 entries |
| `gate.py baseline` | exit 0; 26 `new`, baseline-run red as predicted, key written |
| `planlint.py check` | `hits: 0`; 12 (d)'s six claims each hold (none on a name step 3g changes) |
| `inputs.py verify` | 41 entries · drifted 0 · unparsed 0; I39–I41 cited |
| `gate.py scope --lists-only` | parsed 3 items |
| 11 host reach | entry 27 kills only the wrapper it started (pid and start time); it removes nothing |
| 12 step order | entry 8 re-fired after 26, so the plain build stands before 27 |

**Validation-1: aligned.** The working entry's intent («v1-27's wrapped start proved») returns to the plan.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | A CARRY on a later live entry (the founder's own word, inputs#I33): «a wrapped start in a cwd trusted beforehand, so no trust key is typed — lines 1–2 and the `x`'s own wheel line read». matrix#v2-03 is un-claimed at this chunk's wrap with a dated note naming the word, and the CARRY owns its reading (standing — .andromeda/runs/2026-11-15T11-17-37-phase/p5-card-1.md) | a new CARRY, v2-03 un-claimed | the wrap's route-resolve and its ledger write |
| 2 | Row 1 now holds ONLY if entry 27 reads red. On a green entry 27, row 1 ends: no CARRY, no un-claim, and v2-03's ref is written (inputs#I41 item 6) | a condition on a standing row | the wrap's route-resolve and its ledger write |
| 3 | After entry 30, the halted wrap resumes in its own run dir `.andromeda/runs/2026-11-15T11-38-30-wrap/`, its report amended for entry 27, and the plain gate flips v2-02 and v2-03 (inputs#I40 (d)) | a resume of the halted wrap | `/andromeda-wrap-session` |

Apply? (yes / review / cancel)
