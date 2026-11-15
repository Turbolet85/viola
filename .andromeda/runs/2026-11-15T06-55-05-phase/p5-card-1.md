# P5 review card 1 — 2026-11-15-live-round-on-the-trimmed-product «Live round on the trimmed product»

Written 2026-11-15T07:44:57Z. The plan to read in an editor: `.andromeda/cache/docs/2026-11-15-live-round-on-the-trimmed-product/plan.md`
(rendered from `doc:2026-11-15-live-round-on-the-trimmed-product/plan` rev 3). No plan stands in the chunk's folder;
writing in that file changes no record until a `put`; `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py
diff --root . 2026-11-15-live-round-on-the-trimmed-product` is a diff of the chunk's records.

Per the operator directive (`inputs#I1` item 2) this phase stops at this card: no word is taken in this window, no
implement follows. The founder's five forks are in `.andromeda/runs/2026-11-15T06-55-05-phase/founder-forks.md`.

## Goal
The first live round on the trimmed product: v2-03's verified wrapped start and v2-02's hold read near 3600 s, plus
the three CARRYs' live readings. No product behaviour changes; the rigs are Python scripts in the chunk's evidence,
each rehearsed under the fake agent. /implement builds and rehearses them (entries 1–8) and STOPS before the first live
step; entries 9–12 are named operator steps, each fired on its own word.

## Top acceptance criteria
- Entries 1–8 green; no live start made before entry 9.
- matrix#v2-03 — entry 10: on a home entry 9 stamped green, `cli_verified:true`, `openpty`, `wheel` → `session-start`
  (no budget step), pinned copy + fresh plugin, 0 viola bytes on the pty, the human key passed (on `input-box yes`),
  `already-live` and `stale-heartbeat` refused with no child, the child gone ≤ 10 s after the wrapper's SIGKILL.
- matrix#v2-02 — entry 10's hold: `question`, then ≈ 3600 s later `dialog-expired` + `dialog-answered{deadline_hit:true}`;
  the mark settled on the founder's `hold:` line.
- CARRY 1 (entry 12, then 15) · CARRY 2 (entry 9's recording read box-only or not; placed only on `cli: 2.1.302`) ·
  CARRY 3 (entries 10 and 11: the statusLine command run on both sides, no `--settings`, no instance `settings.json`,
  `~/.claude/settings.json` unchanged).
- `cargo nextest run --workspace --features fake-agent` 0 after placement; CI `verdict: green`.

## Touchpoints
- New: `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/` (the rigs and the records).
- Modify, each conditional: `fixtures/claude/2.1.302/Mcp.tools-list-1.json` (on `cli: 2.1.302`),
  `fixtures/claude/2.1.302/Screen.ready.json` or `Screen.turn.json` (on `cli: 2.1.302` and `box-only: yes`),
  `crates/viola-core/src/lib.rs` (the `DIALOG_HOLD_MAX` doc comment, on `hold: stands`).

## The live round
| item | entry | live starts | rig | what it records |
|---|---|---|---|---|
| `viola verify --record` on a fresh tmpfs test home | 9 | 6 (print probe + Runs A–E; + 2 non-session spawns) | `live-hold.py live-verify` (hosts nothing; a direct spawn from the repository root) | the stamp line for the round's CLI, the recorded set in a private dir, `box-only: yes/no` |
| the wrapped start: readings, refusals, keys, ~3600 s hold, killed wrapper | 10 | 1 | `live-hold.py live-wrapped` over `live-pty.py` | v2-03 whole, v2-02's hold lines, CARRY 3's wrapped side |
| the unwrapped start | 11 | 1 | `live-hold.py live-unwrapped` over `live-pty.py` | CARRY 3's unwrapped side |
| the MCP tap | 12 | 1 | `live-tap.py live` (`claude -p` → `viola-mcp-tap record` → `viola mcp`) | CARRY 1's re-recorded `Mcp.tools-list-1.json` |
| **total** | | **9** | | |

## Concretization preview (claims after the word; both GATED on the founder's own words — the epoch's last chunk)
- **v2-03** — now: «One live wrapped start of the unmodified claude on Linux shows the child's screen with no byte of
  viola's own and passes the human's keys unchanged, runs the pinned copy through a freshly written plugin, follows the
  documented start order with no budget step, refuses a live or stale name and detects the child's end on the process,
  on a ledger that reads green for the founder's account.» → proposed: entry 10 `live-wrapped: green` on the home entry
  9 stamped `21 pass 0 fail` for the round's CLI: `process-start{subject:"claude-child"}` `cli_verified:true`
  `pty_backend:"openpty"`; `events.ndjson` lines 1–2 `wheel{cause:start}` → `session-start`, no `budget-gate`; the pinned
  copy under `<home>/bin/` and `plugin/<key>/` written at the start; 0 `unable: `/`hint: ` bytes in the rig's drained
  pty stream; `x` echoed with a `wheel` line `human`; `already-live` and `stale-heartbeat` exit 1 with no child; the
  child gone by pid and start time ≤ 10 s after its wrapper's SIGKILL. Gated: fork 1 (`cap:` ≥ 7), fork 2
  (`input-box yes`; on `no` the cap is NOT claimed — stays pooled with a note), fork 3 (`cli:`).
- **v2-02** — now: «A live dialog hold read close to the 3600 s ceiling stands on record, and the ceiling either stays on
  that reading or moves to the value read, with no provisional mark left on it.» → proposed: entry 10's hold reading —
  one `question`, one `dialog-expired` and `dialog-answered{deadline_hit:true}` with `duration_ms` within 60 s of
  3 600 000, no human key during the hold — and the `hold:` line answered: `stands` removes the mark (plan step 9).
  Gated: forks 1, 3 and 4.
- Not claimed: v2-05 (verified; CARRY 3 adds a matrix note at the wrap).

## The directive's block (`inputs#I1` item 10), verbatim where a tool printed it
- **The union**, `rec.py rows-union`:
  ```
  architecture: rows 938 · packs 10 · picked 291 (169 by every draw) · standing 16 · 76777 B
  security-plan: rows 595 · packs 6 · picked 178 (124 by every draw) · standing 5 · 46945 B
  design-system: rows 313 · packs 4 · picked 69 (52 by every draw) · standing 12 · 16583 B
  layout-templates: rows 221 · packs 3 · picked 72 (53 by every draw) · standing 0 · 14904 B
  test-plan: rows 1325 · packs 14 · picked 437 (255 by every draw) · standing 42 · 99422 B
  obs-plan: rows 1021 · packs 11 · picked 346 (231 by every draw) · standing 43 · 77374 B
  a11y-plan: rows 235 · packs 3 · picked 72 (45 by every draw) · standing 12 · 18587 B
  rows-union: 7 master(s) · picked 1465 of 4648 · rule any
  ```
- **Every row the plan cites is in the union:** yes — 24 of 24 (21 architecture rows, 2 test-plan, 1 security-plan),
  each found as a picked row in its `{specialty}-rows.md`; none rests on a `*-standing.md` alone.
- **`[RETYPED]` lines**, as `fanout.py check` printed them:
  ```
  security-pick-p3.d2 · [RETYPED] line 81: decision/max-frame-checked-against-data is no row of the pack — read as decision/max-frame-checked-against-ledger, the pack's row 81
  security-pick-p6.d3 · [RETYPED] line 58: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 58
  tests-pick-p11.d3 · [RETYPED] line 55: decision/mutation-verdict-counted-by-harness is no row of the pack — read as decision/mutants-verdict-counted-by-harness, the pack's row 55
  tests-pick-p12.d3 · [RETYPED] line 93: decision/no-human-visual-regression is no row of the pack — read as decision/no-human-reviewed-visual-regression, the pack's row 93
  tests-pick-p13 · [RETYPED] line 27: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 27
  tests-pick-p13.d3 · [RETYPED] line 97: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 97
  ```
- **`[EVERY_ROW]` re-spawns:** `security-pick-p2.d3` and `security-pick-p6.d3` (batch `pick-p1-r1`), both passing on
  their one re-spawn: `pick: 2 agents · 2 pass · yes 47 of 197`. Dead draws: 0.
- **P4 self-check, checks 4 (2), 4 (6) and 8:**
  - 4 (2): no boot-path touchpoint — the modify-set holds two fixture files and one doc comment, none on a boot path;
    the absence is stated in the Test Commands prose («No `smoke` entry: the chunk touches no boot path»). PASS.
  - 4 (6): every gate a criterion names is an entry (entries 9–16, 20 by number; `cargo nextest run --workspace
    --features fake-agent` is entry 16); no source-mutating control is named (no guard test). PASS.
  - 8: the sites a criterion turns on, read at HEAD: the hold's conditions `src/run/dialog.rs:251` (`cli_verified &&
    pending.is_none() && !unanswerable && holder == Driver` — hence the stamp first and `release` before the send),
    the expiry lines `src/run/dialog.rs:372–394`, `--record`'s whole-set write `src/cmd/verify.rs:454–471` and its three
    phases `:575–582`. Sites this chunk CREATES (the rigs) are out of the check's scope; their controls are named
    as implement's one-shot run in the Test Commands prose. PASS.
- **Sealed-run reads made:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py runs --root . --kind
  '2026-11-14T14-40-00-phase' --file 'p5-card-1.md'` and `… --kind '2026-11-14T19-10-38-phase' --file 'p5-card-2.md'`
  (each listed `· sealed`, existence only, no `--show`); and in new-session `rec.py runs --root . --kind
  '*-evolve-diagnose' --file proposals.md --head 1`.
- **Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
  - the Bash guard refused three of my calls, each re-issued once in its passing form: a bare `grep` with a bounded
    repetition (re-issued as `/usr/bin/grep`), a `cd` into the run dir (re-issued with absolute paths), a pipeline
    tool's output piped into `head` (re-issued bare);
  - the stamp-ahead hook refused the plan's first `Generated:` stamp (typed 07:40:00Z at 07:37:06Z) — re-stamped from
    `date -u` (a recurrence of the handoff's deferred learning on clock-read stamps);
  - `inputs.py verify`: `UNPARSED: inputs#I5 — no entry (research.md:83)` — another chunk's snapshot cited in this
    chunk's form; re-pointed to `chunks/2026-11-06-version-done-check/inputs/I5-relay-1.md.txt`, re-verified
    `unparsed 0`;
  - P5 baselines, all expected: entries 1, 2, 4, 5, 6, 7, 13 red (exit 2 / `cmp` absent — the rigs are not written
    yet); 3, 8, 14, 15, 16 green; no `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.
  - CI at Setup 5a: `56c997145c53 verdict: in progress · checks 7/7 · runs ci#43687184962` — not re-read since; it is
    the operator's to read before the round.
- **Where a letter was wrong or silent:**
  - the away form (`inputs-contract.md` §snap) says the step STOPS at `founder-forks.md`; the directive says go on
    planning around the forks — the directive was followed, recorded as a deviation in the plan checkpoint;
  - the phase letter's whole read of every rows file and history file (≈ 500 KB) does not fit one window beside
    P3–P5: `arch-rows.md` and four history files and four standing lists were read whole, the other six rows files and
    three history files by targeted search (every cited row then checked present, above);
  - the 0.1.0 round put its homes outside `target/e2e-home/` so no CI upload could reach them; the directive puts them
    on `target/e2e-home/viola-live-*` — on the dev host that path is a link to a tmpfs outside the tree and reaches no
    CI run, so both hold;
  - the handoff's «claude 2.1.323 is `latest`» is stale: `mise ls claude` reads 2.1.325 `latest`;
  - the live home's 2.1.302 stamp (2026-11-09) reads four relied rows red — a fact no master or handoff carries.
- **Host work in this phase (directive item 9), each with its wall time:** no cargo build or test ran in research; at
  P5 the baseline runs fired the gate tool: entry 3 `cargo build --features fake-agent …` 5.41 s; entry 8 `cargo build
  --bin viola …` 2.31 s; entry 14 `contract_fixture_hygiene` 0.34 s (106 passed); entry 15 `mcp_recorded_exchange`
  1.36 s (1 passed); entry 16 `cargo nextest run --workspace --features fake-agent` 86.94 s (3280 passed). No live
  `claude` start was made.

## Validation-1
Aligned with the scope (rev 3) and the working entry: v2-03, v2-02 and the three CARRYs each have a step, an entry and
a criterion. One narrowing, surfaced for the review: files land only on `cli: 2.1.302`; another CLI's recordings are
read and carried (a new version's fixture set is whole and not this chunk's to land).

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | On `cli:` other than 2.1.302: a new route entry, proposed «A newer CLI's fixture set recorded whole», after «Live round on the trimmed product», owning that version's whole `--record` set, its `STAMPED` row and CARRY 1's and CARRY 2's files for it | route insertion (conditional) |
| 2 | On `hold: moves-to-read`: a new route entry, proposed «The dialog-hold ceiling moved to its live reading», after «Live round on the trimmed product», owning `DIALOG_HOLD_MAX`'s six references and the two 3615 timeouts | route insertion (conditional) |
| 3 | Each item a smaller `cap:` drops (fork 1's options 2–4), and CARRY 3's comparison on `statusline-read no`: carried on to «Live round on the trimmed product»'s successor in this version's tail, named at the wrap | deferral (conditional) |

Apply? (yes / review / cancel)
