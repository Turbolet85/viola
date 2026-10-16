# Scope — 2026-10-14-resume-and-compact-ledger-rows · Resume and compact ledger rows

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, seventh under
`### Epoch 5 — Driver surface`, directly after "Owed ledger rows"): Resume and compact ledger rows —
`claude --resume <id>` reopening a session and `/compact` become probed, stamped capability-ledger rows (v1-34) —
plus three CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run dir's trail: three
rows on the entry, 1368 · 1538 · 861 chars, no abstention).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). No external
input is snapshotted at P1: every fact below is read from the entry or from a file of this repository (the earlier
chunks' `inputs/` and `evidence/` included).

**CI read at Setup 5a** (the last wrap's flip `880b056` = HEAD, `ci.py conclusion`, this run dir's trail):
- `880b056377f4`: **in progress** — `CI 880b056377f4: verdict not yet available`, checks 15/15 listed, run
  `ci#38526582400` (push) open, its oldest running job `test (windows-2025)` at 103 s when read. Not folded and not
  read as green at Setup. Re-read at P3 (`ci.py conclusion`, this run dir's trail): **green**, checks 15/15, wall
  579 s, `ci#38526582400` completed/success. No red or `not green` to disposition.

## What this chunk builds

Two relied-on CLI shapes, each a capability-ledger row with its `viola verify` probe and a recorded fixture, stamped
live. Today `LedgerRow::ALL` holds 21 rows (`crates/viola-agent-claude/src/ledger.rs:46`, re-read at P1); the relied
set (`LedgerRow::relied()`, `ledger.rs:145`) is 21 on Unix and 18 on Windows.

### 0. The cost frame (CARRY 1, minted at the 2026-10-13-owed-ledger-rows wrap)
- Minted directly after "Owed ledger rows" on the founder's own answer to that chunk's P4 fork 1 («Тройка
  статуслайна», `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/inputs/I1-relay-1.md.txt`), its P5 card row 1
  (`…/inputs/I2-relay-2.md.txt`, the operator's `yes`) and the operator's word at its wrap (`…/inputs/I4-relay-1.md.txt`)
  — all three re-read at P1 and matching the CARRY.
- Verbatim in substance: **each shape needs a widening ruling on the `viola verify` child set and a live-start cap of
  its own at take-up.** One `viola verify` round is five live starts; a fifth Run B paste added none.
- A new row leaves every home stamped before it `cli_verified:false` until re-stamped (the 18th row did so on every
  home, the 21st on every Unix home).
- A sweep of the row count's literals missed five readers that pin the row set by shape (a verdict vector, a stamp's
  rows object, an older-stamp oracle, the dialog-less summary, the framing-variant list).
- As read at chunk 2026-10-13-owed-ledger-rows: `/compact` needs a product change to `send`'s claim and `--resume` a
  sixth verify child and a new id source. The relied set is per OS, so **each new row states where it is relied**.

### 1. `claude --resume <id>` reopens the session (CARRY 2, chunk 2026-10-10-viola-revive)
- A second owed row of `v1-34`: `viola revive` relies on `claude --resume <id>` reopening the same session and firing
  a SessionStart of source `resume` with that id. Measured once at that chunk on 2.1.287 on the Linux dev host
  (`viola-0.1.0/chunks/2026-10-10-viola-revive/evidence/live-revive.md`, starts 2 and 3, re-read at P1); under the
  fake agent on CI. No ledger row, probe or recorded fixture covers it.
- The founder's word at that chunk (`…/2026-10-10-viola-revive/inputs/I3-relay-2.md.txt`, answer 1, «Строить, строка
  в долг»): revive landed with no row, the row owed on a route entry.
- The operator's answer at that wrap (`…/inputs/I8-relay-3.md.txt`, answer 3): one ruling, one re-stamp and one count
  move for both rows (§1, §2), **and this phase proposes a cut if the entry no longer fits one window**.
- Its probe needs a session to resume — a new shape for the verify child set (a sixth child, and a new id source:
  the id of a session a verify run itself started), under a founder ruling not yet given.
- With the row, the recorded SessionStart of source `resume` replaces the fake agent's rewritten payload: the live
  payload holds 10 key names, the rewritten one 5 (`cwd`, `hook_event_name`, `session_id`, `source`,
  `transcript_path`; the five more: `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`,
  `scratchpad_dir`, `seconds_since_last_response`) — `live-revive.md` §The payload's key names, matching.
- Not measured at that chunk (`live-revive.md` §What was not measured): `--fork` on the real CLI, a resume of a killed
  session on the real CLI, whether a resumed session held its earlier turns, project settings on a resume from
  another directory. Whether this chunk measures any of them is a P4 question, not a commitment here.

### 2. `/compact` as a local command (CARRY 3, chunk 2026-10-10-viola-revive)
- `viola send` of `/compact` exits 13 `not-delivered` / `no-prompt-submitted` after 10 s while the command runs, and
  the log shows a `session-start` of cause `compact` with the same session id 18.8 s after the send was issued
  (`live-revive.md` §Start 2, continued; the `compact` payload holds 8 key names: the five, `model`, `prompt_id`,
  `scratchpad_dir`). The operator's answer at that wrap (`I8-relay-3.md.txt`, answer 4): same ruling and rig, the
  size of this entry read at its take-up.
- `/compact` is not on the local-command list (re-read at P1: `LOCAL_COMMANDS` holds two entries, `/clear` with
  `PostCondition::NewSession` and a second with none, `ledger.rs:193`; `PostCondition` holds `NewSession`,
  `ledger.rs:186`).
- The CARRY's own marker, kept verbatim: "hypothesis: the send waited for a `prompt-submitted` that a local command
  never fires, as `/clear` fires none." Verified at P3 in its code half (research.md §Measured facts 2:
  `classify` returns `Confirms::Prompt` for an unlisted text, `classify.rs:27`–`:33`; `SendSlot::session_started`,
  `slot.rs:98`–`:118`, claims only cause `clear` with a NEW id). Its CLI half stays that chunk's one live reading.
- `[premise-corrected: research.md §Measured facts 3 — CONFIRM_WINDOW_FALLBACK is 10 s (screen.rs:20) and the one
  measured compaction answered 18.8 s after issue]` A ledger row, a Run B paste and a claim arm do NOT by themselves
  confirm a `/compact` send: under today's single 10 s window the post-condition arrives after the send has already
  ended exit 13. Confirming `/compact` also needs a window of its own — a new bound, whose length a compaction's
  growth with the session makes open — or `/compact` is listed `unconfirmable`, or it stays owed. That is P4's fork.
- "Listing it with that post-condition is a ledger row and one more Run B paste, and a compaction inside
  `viola verify` costs a live turn."

## Boundaries
- Not in scope: the harness-prefix row and the R8 identity-floor row of `v1-34` (owed, no entry minted; the founder,
  2026-10-07T09:43Z) — so **`v1-34` stays unclaimable by this chunk** (verified at P3: `matrix.py show --id v1-34`,
  research.md §Measured facts 12).
- Not in scope: the `claude agents --json` join row (moved onto "Unwrapped sessions on the board", fork 2 of
  2026-10-13-owed-ledger-rows).
- Not in scope: the Windows legs of anything this chunk stamps live on the Linux dev host — the owner is
  "Windows-only live measurements" (verified at P3: the entry stands markerless with its `BLOCKED-ON` at an annotation
  position, `route.py cursor` / `pins`).
- No `--json` for revive (owned by "CLI machine contract").

## Premises (closed at P3 against HEAD)
- Both rows are relied on every OS — verified (research.md §Measured facts 11: `src/cmd/revive.rs` has no
  top-level `cfg`; `classify` / `LOCAL_COMMANDS` none). The relied set moves 21 → 23 on Unix and 18 → 20 on Windows
  for each row built, and the Windows count waits on a Windows stamp.
- `[premise-corrected: the stopped implement run, .andromeda/runs/2026-10-16T06-46-40-implement/outcome-2026-10-14-resume-and-compact-ledger-rows.md §Deviations — ten readers moved outside the plan's list, the dialog-less summary's second reader `dialogless_home` in tests/support/home.rs among them]`
  Every literal reader of the row count or row set moves, by the function the row set decides (membership, the
  verdict vector, `merge_stamp`'s rows object, the older-stamp oracle, the framing-variant list, the dialog-less
  summary, the harness `LEDGER_ROWS`), and also by the run count Run E moves (the stop-less and no-screens sets, the
  starts, the spawn and receipt counts). P3's sweep keyed on row-count literals listed the dialog-less summary in
  `tests/cli_verify.rs` only; its second reader sits in `tests/support/home.rs` (`dialogless_home`, 17/14 → 18/15).
- A `/compact` row needs a new `PostCondition` member (cause `compact`, the SAME id) and a second claim arm in the
  send slot — verified (research.md §Measured facts 2), and extended by the corrected bullet in §2: it also needs a
  confirm window of its own.
- `[premise-corrected: research.md §Measured facts 5 — Run B's session after /clear is a new, empty one]` The
  `--resume` probe's session is one a verify run itself started; the session that holds turns is Run B's FIRST id
  (before `/clear`), so a resume child resumes that id, read from Run B's own first SessionStart capture — a new id
  source onto a child's argv (a crossing). Whether it is a sixth start or a shape of an existing run is P4's fork.
- The entry does not fit one shape: the two rows' costs diverge (research.md §Measured facts 3, 9 — `--resume` is a
  sixth start and a crossing; `/compact` is a live compaction inside Run B plus a new send bound). P4 proposes a cut.

## Gates
- **The founder's ruling on the verify child set** (a sixth child for `--resume`; a `/compact` paste in Run B) and
  **a live-start cap for this chunk** are P4 forks, his own word — never a decisive lean. Without them no probe ships
  and no live round starts.

## The cut taken at P4 (amended at P5 validation-1: intent-incomplete, the forks' answers)
The founder answered the three P4 forks himself, relayed verbatim by the operator (`inputs#I1`):
- Fork 1 «Resume; /compact в список»: this chunk builds the `--resume` row only. `/compact` joins the local-command
  list with no post-condition, so a `send` of it reads `unconfirmable` at once. It needs no row and no probe (the
  ruled limit), and runs no live compaction. The `/compact` post-condition row stays owed. So §2's row is not built
  here, and the relied set moves 21 → 22 on Unix and 18 → 19 on Windows (one row), not the two-row forecast in
  §Premises.
- Fork 2 «Run E, шестой старт»: the resume probe is a new sixth start, Run E, which resumes Run B's first id read from
  Run B's own first SessionStart capture. This is the boundary widening of the verify child set, ratified by him.
- Fork 3 «Двенадцать»: this chunk's cap is twelve live starts (two six-start rounds).

## The revision (2026-10-16, after `/andromeda-implement` stopped on the plan)
- The stop: implement run `.andromeda/runs/2026-10-16T06-46-40-implement/` read every gate green but plan entry 15,
  the preservation guard, red on `tests/support/home.rs` alone, a companion recorded in `scope-record.md`
  (``- `tests/support/home.rs` · companion · serves step 9 · self``). The overseer's answer there (founder-delegated,
  `inputs#I3`): re-plan the guard, stop with no commit and no push.
- The operator's word for this revision (`inputs#I4`): the guard excludes `tests/support/home.rs`. Implement re-enters
  after the revision at entry 15, then the operator pass. No live start remains (12 of 12 spent). Verified at this
  revision's P3: `git diff 880b056377f4 -- tests/support/home.rs` is the `dialogless_home` doc line and its
  `passed` literal only (3 lines changed, 17/14 → 18/15); the guard as written prints only that path under
  `git diff --name-only` with its own exclusions and reads exit 1; with `:!tests/support/home.rs` added it reads exit 0
  on the stopped run's tree.
- Nothing else in the chunk's scope moves: the write set gains that one companion, and no live round, step or
  criterion is added.
