# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T08-33-57)

**Outcome: stuck — four plan lines must move (the phase door), section «After entry 9» below.** Entry 9 fired once
and read red. The cause is a rig fault, not the CLI. It was not re-fired and is not re-labelled. The overseer's
technical answer (inputs#I9) was then carried out: the state save is fixed, and `round-state.json` is derived from
the kept home by a rig read. Entries 10 and 11 now read ready, but entry 10 waits for its own word and for the phase
door.

Setup's approval line, quoted: `2026-11-15T08-23-45-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json · open · 2979 B`
(plan rev 4). The operator's word for entry 9 is snapped as inputs#I8.

## What entry 9 read
- **Stamp line:** `stamped 2.1.325  21 pass  0 fail`.
- **Relied set:** 21 pass, 0 fail. Every one of the 21 relied rows reads `pass` in the test home's
  `ledger/stamps.json` (listed in `evidence/live-round.md`). Measured: `box_only_guards: 3`.
- **`box-only`:** unread. verify refused the whole recording before writing any file:
  `unable: a recorded fixture is not clean: SessionStart.default.json absolute-path` ·
  `hint: record with a viola home under your user home`. `verify_exit=1`, `recorded_files=0`.
- **Starts spent:** 6 of the founder's 9. The `start` row was ledgered from the clock before the spawn (08:38:05Z). The
  rig confirmed `latest` 2.1.325 before spawning.
- **Gate line:**
  `9 e2e red · exit 0 ✗ (exit 1) · exit 1 · 46.15s · 1616 B → 9.log · history: unmoved` ·
  `entries 20 · green 0 · red 1 (9) · …` (`evidence/operator-9.txt`).

## Why it is red: two rig faults
1. **The recording refusal.** The rig names the test home's directory `home` (plan step 3:
   `target/e2e-home/viola-live-{stamp}/home`). So the print probe's cwd `…/viola-live-…/home/ledger/probes/<pid>`
   carries `/home/`, which the record scrub's absolute-path test (`ledger::has_absolute_path`, `/home/` substring)
   refuses. The fake-agent rehearsal replays fixture payloads and cannot reach this. My pre-fire reading checked only
   the `/tmp` prefix and missed the dir name.
2. **The crash after it.** The rig's green needs exit 0 and a recording, so it read red and wrote its `reading` row.
   It then raised in `save_state`: `rel()` resolves the `target/e2e-home` link to `$TMPDIR/viola-e2e-home-1000/…`,
   outside the root. `round-state.json` was never written, so as the rigs stand, entries 10 and 11 refuse
   («no green live-verify for this cli»).

## What this means for the plan's criteria
The plan's unread clause is «a red entry 9 (a 2.1.325 verify that does not stamp `0 fail`) leaves v2-02 and v2-03
unread». This verify DID stamp `0 fail` over the relied set: v2-03's stamp premise holds on the kept home. What failed
is the recording (CARRY 2's evidence and its box reading) and the rig's own bookkeeping.

**Proposed: for the operator's word, not done.** No rig was changed after the live red.
- (a) Fix `rel()` to the unresolved path.
- (b) Split the rig's verify verdict into the stamp (what 10 and 11 need) and the recording (CARRY 2).
- (c) Write `round-state.json` from this run's own ledger row and kept home, without a re-fire — or treat entry 9 as
  red under the plan's rule.
- (d) Rename the home dir for any future recording.

CARRY 2's recording is spent for this round under the no-re-fire rule unless the operator rules otherwise.

## After entry 9 — the overseer's technical answer (inputs#I9), items 1–6
Entry 9 stands as fired: **red**. Its halves, stated apart:
- **STAMP:** `stamped 2.1.325  21 pass  0 fail`, all 21 relied rows `pass` in the kept home's `ledger/stamps.json`.
  Entries 10 and 11 need this.
- **RECORDING:** refused whole, a rig fault (the dir named `home`). CARRY 2 needed this; it is spent for this round,
  and CARRY 2 stays a CARRY onto «The 2.1.325 recorded fixture set». `box-only` is UNREAD; `box_only_guards: 3` is
  verify's measurement.

1. **The state save, fixed and rehearsed through the link.** `rel()` keeps the path as spelled, never resolved;
   `save_state` takes a path, and `rehearse-verify` writes and reads back its own state. Gate entry 4 (this run's firing
   2): `… · state_saved=True · census_held=0`; the block 1–8 `green 8 · red 0`, ending on the plain build
   `a0e247fd3a77f744`. A future `live-verify` writes the stamp and the recording apart in the state
   (`stamp` · `recording` · `entry`), and entries 10 and 11 read `stamp == pass` for their cli. Their refusal literal
   `refused: no green live-verify for this cli` is kept.
2. **`round-state.json` derived by a rig read, never by hand.** `live-hold.py derive-state <base>` reads the kept home's
   `ledger/stamps.json` (mode 0600, `v` 1) for the `cli:` version, every relied row read from the source
   (`LedgerRow::relied()` = `ALL` and the `id()` arms of `crates/viola-agent-claude/src/ledger.rs`, its git blob
   recorded), and this run's own `live-verify` start row (before the stamp) and its reading row. It writes the state
   only when every relied row reads `pass`, records where each value came from, and spends no start.
   - Rehearsed under the fake agent in both arms:
     `rehearse-derive: green — stamped=True · pass_arm=written,stamp=pass,recording=refused,entry=red,home_is_dir=True,via_link=True · provenance_keys=reading_row,relied,stamps,start_row · fail_arm=refused:relied row local-command-compact reads fail · fail_arm_wrote=False · relied_count=21 · census_held=0`.
   - Live, over the kept home: `derive-state: written — stamp pass · cli 2.1.325 · recording refused · entry red · home target/e2e-home/viola-live-20261115T083805Z/home`.
   - Its provenance: stamps sha256 `86f763d8f43f5281`, verified_at 2026-11-15T08:38:51.417Z · relied 21 rows, git
     blob `96c1c5d085dc`, each `pass` · start row `live-sessions.ndjson` line 5, written_at 2026-11-15T08:38:05Z,
     starts 6, latest 2.1.325 · reading row verdict red, stamped 2.1.325/21/0, recorded_files 0, verify_exit 1.
3. **(b), as a statement:** above. Entry 9's verdict is not changed.
4. **(d), for the future:** not done now. Nothing is re-recorded in this chunk. Step 3's literal home path is the line
   it will move (item 5).
5. **Plan lines — four must move, so it goes through the phase door** (plan rev 4; line numbers of its render):
   - **Acceptance, CARRY 2** (lines 352–353): «entry 9's recording read by `live-read.py box` and recorded as read
     (`box-only: yes|no` per phase)» cannot be met. No `Screen.*.json` exists, and `box-only` is unread. It becomes:
     the recording refused (a rig fault), `box-only` unread, carried.
   - **Step 8** (lines 120–124): «the `Screen.*.json` of entry 9's `--record` — stay in the round's private dir and are
     summarised … (their `box` reading …)» names files that do not exist. Only entry 12's `Mcp.tools-list-1.json` can
     stay.
   - **Acceptance, the `latest` check** (lines 330–331): «every live `start` row and closing line of entries 9–12
     records `latest` as 2.1.325». Entry 9's start row and reading row record it, but its closing line was never
     printed (the rig raised in `save_state`).
   - **Step 3** (lines 50–57):
     - «Writes the home's path relative to the root into `evidence/round-state.json`»: this round's state came from
       `derive-state`, a subcommand the step's list does not name (nor `rehearse-derive`), and the state now carries
       the stamp and the recording apart.
     - Its literal home path `…/viola-live-{UTC stamp}/home` is what (d) moves for any later recording.
   - **Lines checked that do not move:**
     - Step 7 line 117 («10 and 11 need 9's green stamp»): the stamp is green.
     - Test Commands prose line 165 and the acceptance line «A red entry 9 — a 2.1.325 verify that does not stamp
       `0 fail` — leaves … unread»: their condition, no `0 fail` stamp, is not this case. The refusal literal is kept.
     - The v2-03 criterion's stamp clause: it holds on the kept home.
     - Entry 9's `expect`: it stays red as fired.
     - Entries 10–12's `run`/`expect`: unchanged.
6. **Readiness of entries 10 and 11** (read by the rig's own functions, nothing started): `entry 10: … → ready`,
   `entry 11: … → ready` (founder lines and cap 6 + 1 ≤ 9 pass, `latest` 2.1.325, state stamp `pass` for 2.1.325,
   kept home present). Starts are still 6 of 9.

For the wrap, named plainly:
- I built step 3a (the `latest` check) before entry 9 and re-ran entries 1–8.
- A work-dir land was chained onto a hygiene call that had refused, so `live-round.md` landed once with a host path
  (tx 1373) before the respelled one (tx 1374).

## Files changed (work dir only; scope: `clean — changed 0`)
- `evidence/live-hold.py` — the state-save fix (`rel`, `save_state(path)`, the stamp and recording apart in the state,
  entries 10 and 11 reading `stamp`) and the new `derive-state` / `rehearse-derive` (inputs#I9).
- `evidence/round-state.json` — written by `derive-state` (never by hand).
- `evidence/live-read.py` — step 3a: `latest [<link>]` and the `latest()` function.
- `evidence/live-hold.py` — step 3a: the `latest` refusal in `words_or_refuse`, `latest` in every live `start` row and
  closing line.
- `evidence/live-tap.py` — step 3a: the `latest` refusal and record in `live`.
- `evidence/rehearsal.md` — step 3a's readings: `match`; `refused: latest 2.1.326 is not 2.1.325` exit 3;
  `refused: latest unread` exit 3; each of the four live subcommands exit 3 with nothing made.
- `evidence/live-round.md`, `evidence/operator-9.txt` — entry 9's record.
- `evidence/live-sessions.ndjson` — entry 9's `start` and `reading` rows (appended by the rig).

## Deviations, with justifications
1. **The word said resume at the live steps; plan rev 4 binds step 3a first.** Its constraint: no live start without
   the `latest` refusal. That refusal was unbuilt, so 3a was built, its readings taken, and entries 1–8 re-fired
   (`green 8 · red 0`, run `2026-11-15T08-33-57-implement`, fake agent only, no live start) before entry 9.
2. **3a's live-subcommand refusals were reached through the module's link.** `latest-refusal.py` in the session
   scratchpad sets `READ.LATEST`; the CLI has no link seam. The real link `./2.1.325` was never moved.

## Matrix
No ref. v2-02 and v2-03 stay gated and unread.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entry 9's `viola verify` and its six `claude` 2.1.325 sessions and two non-session spawns | this run (operator entry 9) | terminated — verify exited 1 at 46 s; `gate.py census --comm claude --comm viola --args viola-live-20261115T083805Z --args installs/claude/2.1.325` lists none of them; its probe dir and `.viola-verify-*` dirs are gone |
| entries 4–7's rehearsal wrappers and fake agents | this run | terminated — `census_held=0` in each closing line |
| the 14 processes the census lists | NOT DERIVED — not this run's: the overseer sessions, the prototype tree's builders, `~/.viola/bin/0.1.0-…/viola mcp` | left running — their owners' |

Kept on purpose: the live test home `target/e2e-home/viola-live-20261115T083805Z/` (its stamp, for the later entries).

## Decisions & corrections
- The operator's word: entry 9 only, no re-fire, then stop (inputs#I8). Entry 9 was fired once and is red; it is not
  re-fired.
- A pre-fire reading was wrong. I read the scrub's absolute-path test against the tmpfs prefix and concluded no
  refusal; the plan's own `home` dir name trips it. Recorded as `contract.instrument-validity`.
- A hygiene refusal: `gate.py hygiene` refused 2 lines carrying a `/tmp/…` host path (`evidence/live-round.md:26`, this
  file's line 29). I had chained `rec.py land` onto that same call, so `evidence/live-round.md` landed once with the host
  path before the respell (tx 1373; a landed byte stays in the journal). Both lines were respelled `$TMPDIR/…`, hygiene
  re-run bare, and the file landed again.
