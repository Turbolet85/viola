# P5 review card 1 — 2026-11-15-live-round-on-the-trimmed-product (REVISION)

**Mode:** revision of a pending chunk's plan (Setup 3 offer b), on the operator's word by the delegate overseer
(`relay-1.md`, inputs#I6). Run dir `.andromeda/runs/2026-11-15T08-23-45-phase/`; take-up run `2026-11-15T06-55-05-phase`.
**Cause:** /implement (`2026-11-15T07-52-35-implement`) built the rigs and read entries 1–8 green twice, then stopped
before entry 9. The founder answered fork 3 `cli: 2.1.325` (inputs#I5), which is not the plan's 2.1.302 lean.
**Not run, by the word:** the row read (P2) and research; the `research` document stands as written (its lists still
name the two 2.1.302 fixture files the plan now drops — a wider allowance, nothing new).

Open the plan in an editor: `.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md` (rev 4, the
store's render). No plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 {tools_dir}/rec.py diff --root . 2026-11-15-live-round-on-the-trimmed-product` diffs the chunk's records.

## What changed (plan rev 3 → 4; scope rev 3 → 4)
- **Scope (P3 premise closure):** the CARRY 1 and CARRY 2 bullets and the `--record` surface are
  `[premise-corrected]`: the 2.1.325 recordings are evidence only, and a CARRY mints the new-set entry at the wrap. The
  founder's `latest` binding is added as an `[inferred]` bullet, closed in the same pass.
- **Metadata / Goal:** the revision is stamped; the round runs on 2.1.325; the forks answered are named (fork 4 waits).
- **New step 3a — the `latest` check:** `live-read.py latest [<link>]` reads mise's `latest` link. Every live
  subcommand (`live-hold.py live-*`, `live-tap.py live`) refuses with exit 3, making and starting nothing, unless it
  resolves to the `cli:` version. It records the resolved version in its `start` row and closing line. The refusal is
  rehearsed through the function's link argument, never by moving the real link.
- **Step 7:** steps 1–7 stand done. /implement re-enters at 3a, takes the `latest` readings, re-fires entries 1–8 and
  stops before 9.
- **Step 8 / entry 13:** on 2.1.325 `place` places nothing (`cli_places: false`, every recording `carried`). The
  recordings stay in the round's private dir and are summarised in `evidence/live-round.md`.
- **Files to modify:** the two `fixtures/claude/2.1.302/` rows leave the list. `crates/viola-core/src/lib.rs`
  (on `hold: stands`) stays.
- **Gate notes:** 9 (2.1.325 unstamped, the `latest` refusal, a red leaves v2-02/v2-03 unread), 13, 14, 15, 16 and 20
  now say «as committed / nothing placed». No `run`, `expect` or key other than `note` changed.
- **Acceptance criteria:**
  - New: a criterion for the `latest` check.
  - v2-03 names the relied set (`stamped 2.1.325  {n} pass  0 fail`, `LedgerRow::relied()`), not «21 pass».
  - New: a red entry 9 leaves v2-02 and v2-03 unread (entries 10 and 11 refuse, both caps recorded `unread`, both
    stay gated).
  - CARRY 1 and CARRY 2 read as evidence and carried.
  - The last criterion is over the fixtures as committed.
- **Constraints:** two lines added — the 2.1.325 set is not landed here; no live start off `latest` or past 2.1.325.
- **Implementation notes:** the untaken placing arm; the steps the stopped run did, with inputs#I3 and inputs#I4, and
  «the real CLI erases on DEL» stated NOT measured. The test-plan §7 placement amendment falls away. The security-plan
  key/read amendment is now answered by the founder's own selections (inputs#I5).

## Goal (as revised, in short)
The first live round on the trimmed product, on `claude` 2.1.325. It covers v2-03 (a verified wrapped start), v2-02
(a hold read near 3600 s), CARRY 1 (the tools/list re-record), CARRY 2 (a box-only screen read) and CARRY 3
(status line wrapped vs unwrapped). Both recordings are evidence, never a fixture here. The rigs are rehearsed under
the fake agent, and the live entries 9–12 are fired by the operator only, one word each.

## Top acceptance criteria
1. Entries 1–8 green, and the round stops before entry 9 with no `start` row.
2. The `latest` check reads `match` on the real link and refuses on a scratch link pointing at `./2.1.326`. Every live
   `start` row and closing line records `latest` 2.1.325.
3. matrix#v2-03 — entry 10 `live-wrapped: green` on the home entry 9 stamped `0 fail` over the relied set for 2.1.325.
4. matrix#v2-02 — entry 10's hold: `dialog-expired` and `dialog-answered{deadline_hit:true}` near 3 600 000 ms.
5. A red entry 9 leaves v2-02 and v2-03 unread and gated, and says so in `evidence/live-round.md`.
6. CARRY 1 / 2 / 3 as read; entry 16 and CI green over the fixtures as committed.

## Touchpoints
- New: the chunk's `evidence/` (the rigs; `live-read.py` gains `latest`; `live-hold.py` and `live-tap.py` call it).
- Modify: `crates/viola-core/src/lib.rs` — `DIALOG_HOLD_MAX`'s doc comment, only on `hold: stands`.

## Test commands (20 entries, unchanged but for notes)
1–8 /implement's (rigs, rehearsals, the two builds) · 9–12 LIVE operator entries (6 + 1 + 1 + 1 starts) · 13 `place`
· 14–16 the suites · 17 fmt · 18–20 the operator pass (hygiene, push, CI).

## Mechanical checks (P5 step 1)
- `gate.py run --dry-run`: the fence parses, 20 entries, no refusal.
- `gate.py baseline`: seen green 1 · not seen 0 · new (key) 12 · leg 7. Every entry is carried unchanged, so each keeps
  its baseline (§Revision item 4).
- `planlint.py check`: `hits: 0`.
  - 12 (d)'s four «unchanged» claims, each read and holding: the cmp'd rigs; `DIALOG_HOLD_MAX`'s value; mise's link
    (only read); the committed 2.1.302 exchange (only read).
  - 4 (17): 0 of 13 asserting.
- `inputs.py verify`: 6 entries — unchanged 2 · n/a 4 · drifted 0 · unparsed 0 (I2 uncited: information).
- `gate.py scope --lists-only`: `scope lists: parsed 4 items`.
- Absence assertions over the plan's own names (P4), each discriminating:
  - live-tap's `budget_words` finds «usage budget» in the committed 2.1.302 exchange (`True`).
  - The `--settings` reading goes red on a planted argv and green without one.
  - `live-read.py place` on 2.1.325 writes nothing: `cli_places: false`, `git status fixtures/` empty.
  - The `latest` refusal is built at step 3a; its readings are implement's one-shot run.
- Size: ~240 prose lines outside the fence. Graph: the take-up's `tree-query-…json` stands. Host reach: the rigs
  signal only the pids they started; the `latest` check reads a link and moves none.
- **Validation-1:** aligned with scope rev 4.

## Concretization preview (§Revision item 5)
- **v2-03** (claimed, gated; stays claimed): its acceptance reads «a test home that plan entry 9's `viola verify`
  stamped `21 pass  0 fail` for the round's CLI». **Proposed:** «a test home that plan entry 9's `viola verify`
  stamped `0 fail` for 2.1.325, the round's CLI (`stamped 2.1.325  {n} pass  0 fail`, every row of the relied set
  `LedgerRow::relied()` passing, `{n}` as verify prints it)». Every other clause is unchanged.
  - Never weaker: the relied set is the same predicate, the number was only its count on 2.1.302 and the fake agent.
  - Re-claimed after the word with a dated note; the gated block is unchanged.
- **v2-02** (claimed, gated): unchanged — the revised plan still proves it as written.
- The gated criterion stands: the block «the founder's own words on forks 1-5 … (cap, keys, cli, hold)» is still
  open (fork 4 waits on entry 10's reading). A stand-in, stub or recorded artifact never verifies a gated cap.

## Beyond the plan
| # | proposal | kind | where |
|---|---|---|---|
| 1 | A new fixture-set entry for `claude` 2.1.325: the recorded `Mcp.tools-list-1.json` (CARRY 1's re-record without «and the usage budget») and the recorded `Screen.*.json` (CARRY 2's box-only screen, if it reads `box-only: yes`), landed as a whole `fixtures/claude/2.1.325/` set with `tests/contract_ledger_probes.rs`'s `STAMPED` and the drift suites — a CARRY the wrap mints at its route-resolve (inputs#I5, inputs#I6) | new route entry (CARRY) | proposed title «The 2.1.325 recorded fixture set», placed by the wrap's route-resolve |

Apply? (yes / review / cancel)
