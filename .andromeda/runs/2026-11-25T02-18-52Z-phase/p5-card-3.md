# P5 review card 3 — «Placing without moving the view» (`2026-11-25-placing-without-moving-the-view`)

Run `.andromeda/runs/2026-11-25T02-18-52Z-phase/` · it answers the review of card 2: the overseer's three technical
points (delegate Viola-overseer, `inputs#I4`, snapped from the relay read whole). The plan's render to open in an
editor: `.andromeda/cache/docs/2026-11-25-placing-without-moving-the-view/plan.md` (plan rev 6). No plan stands in a
chunk's folder; writing in that file changes no record until a `put`; `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-placing-without-moving-the-view` is a diff of
the chunk's records.

## What changed, plan rev 4 → rev 6
No step's subject, touchpoint or gate `run` moved, so every baseline stands.

1. **Point 1 — strip by pattern (step 15's frame).** «`VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` unset first» →
   every inherited `VIOLA_*` name is unset first BY PATTERN. The loop is
   `for name in $(compgen -e); do case $name in VIOLA_*) unset "$name" ;; esac; done`, as
   `scripts/r5-adopt.sh:48-52` and `scripts/r3-focus.sh:36-40` do (both read). It is never named by hand. The script's
   header says so.
2. **Point 2 — what `open` does with the never-live stand-in (step 15's reading)**, stated from the code:
   - **The live test:** `open` waits at most `OPEN_LIVE_BOUND`, 30 s (`src/cmd/open.rs:30`), polling `live_endpoint`
     (`came_up`, `open.rs:319-333`). "Live" means:
     - the snapshot's pid and start time are alive;
     - the heartbeat is at most 5 s old;
     - an endpoint is recorded.
   - **Why the stand-in still reads live:** the wrapper's start writes all three before it spawns the child. The window
     step runs before that snapshot write (`run.rs:445-448`, then `start_state` at `run.rs:520-534`). No session
     start is needed, as «`viola focus`»'s R3a read over the same kind of stand-in.
   - **What R7 reads, by outcome:**
     - **Live:** exit 0, `{"v":1,"ok":{"name","parent","window","placement"}}` (`open.rs:173-182`). `placement` is
       always present (the recorded word, or `unknown` with no window), and R7 reads it there.
     - **`not-live`:** exit 1, `{"v":1,"error":"not-live","detail":null}` with no `placement`. R7 reads
       `snapshot.json`'s `data.window.placement`, the window step's word written before the snapshot. It reads
       `unread` only when no snapshot records a window.
   - **The rehearsal** reads green by that same path, and no other.
   - **One firing's time on his screen**, bounded: the window wait (≤ 5 s), plus the move's three seam reads
     (≤ 5 s each), plus the live wait (ending at the first live poll, ≤ 30 s), plus the close. One window; the view
     may flash once.
   - **Also added, the close's `not-live` arm:** with no snapshot, the foot `open` spawned is ended by TERM. Its pid is
     read from the `process-start{subject:"terminal"}` line's `child_pid` in the scratch home's
     `diagnostics/cli-r7place.ndjson` (`open.rs:346-351`), while `/proc/<pid>/comm` reads `foot`. This keeps
     check 11 (a): only what the script's own open started.
3. **Point 3 — the pair's words are PROVISIONAL (step 9).**
   - Step 9 now marks the pair's WORDS PROVISIONAL on a delegate's word: his V1 answer covers the refusal, the code
     and «a fixed pair naming no path», not this wording. It is built as written.
   - `founder-forks.md` gains a wording item for the epoch batch, quoting the pair verbatim.
   - A new Expected amendment: layout-templates · contract/viola-verify-refusals — a sibling row of
     `contract/viola-verify-refusals/stopped` (found by `rec.py find`), landed PROVISIONAL, delegate overseer, the
     wording owed to him. The rest of V1 lands as his.
   - Rev 5 named no master on that line; planlint read it as a HIT. Rev 6 names the master.

## The checks, re-run whole on rev 6
- `gate.py run --dry-run`: 22 entries parse; no `run` text changed.
- `gate.py scope --lists-only`: `scope lists: parsed 8 items (new 1 · modify 7 · derived 0)`.
- `inputs.py verify`: 4 entries. I1, I3 and I4 unchanged, I2 n/a; 0 uncited, 0 unparsed.
- `gate.py baseline`: 4 seen green, 0 not seen, 9 new (key), 9 leg. The baselines stand.
- Hand checks 2, 3, 4 (2), 4 (6)–(8), 8, 11 and 12 read as on cards 1–2. Check 11 (a) is re-read for the new close
  arm: it ends only a pid the script's own `open` recorded, matched by its comm. Validation-1 is aligned.
- planlint, whole (rev 6):
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
  check 12 (c) · step references the plan's steps lack: 0 of 7
  check 12 (d) · «unchanged» claims on a name another part of the plan names: 4 of 8 — listed, P5's read
    line 73 · «unchanged» · tests/cli_up.rs (Test Commands) · tests/cli_revive.rs (Test Commands)
    line 87 · «unchanged» · stamps.json (step 13)
    line 209 · «unchanged» · src/run/window.rs (Implementation Steps)
    line 417 · «unchanged» · window_step_moves_on_when_the_monitors_read_fails (step 6)
        window_step_keeps_placed_when_the_restore_fails (step 6)
    a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
  check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 1
  hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
  ```
  Each 12 (d) claim holds, as before.

## Every non-zero exit and refusal since card 2
- planlint on rev 5 printed `check 2 · entries naming no master and no matrix#: 1 → HIT` (line 494, the new
  pair-words amendment) and `hits: 1`. It was fixed at rev 6 by naming layout-templates. Its exit was 0.
- Nothing else.

## Concretization preview — matrix#v2-07 (never weakened)
As on card 2, unchanged:
- every clause of the current acceptance kept, word for word;
- added: the active workspace unchanged, the window step's request sequence over the fakes, and R7's
  `r7: green placement=placed active=unchanged`.

The gated criterion's block, verbatim: «R7, ONE viola open placing a window live at the operator host, unlocked, the
active workspace read before and after, fired this evening with him at the host (his fork L answer, inputs#I3)». A
stand-in never verifies it.

## The claim's lines
Not fired. They come after the word on this card.

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | The granted ONE live re-record of the move's fixture is NOT spent by this chunk. (standing — `.andromeda/runs/2026-11-25T02-18-52Z-phase/p5-card-1.md`) | his grant, unspent |
| 2 | The red stamp already kept for 2.1.331 on his own viola home is not touched by this chunk; clearing it is his at the desk. (standing — `.andromeda/runs/2026-11-25T02-18-52Z-phase/p5-card-1.md`) | his, at the desk |
| 3 | R7 is fired this evening by the operator entries 21–22, with him at the host, unlocked, and on no other word. (standing — `.andromeda/runs/2026-11-25T02-18-52Z-phase/p5-card-2.md`) | his, this evening |
| 4 | Step 9's pair WORDS go to him in the epoch batch as a wording item. Until then they and their row stand PROVISIONAL, delegate overseer (`inputs#I4`). | his word, the epoch batch |

Apply? (yes / review / cancel)
