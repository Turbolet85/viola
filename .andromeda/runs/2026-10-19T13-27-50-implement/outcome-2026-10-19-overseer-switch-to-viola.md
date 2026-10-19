# Implement outcome — 2026-10-19-overseer-switch-to-viola (INTERIM: under the product, paused at step 12's clear)

**Outcome word:** not yet. Steps 10–11 are done under the product. This turn ends asking for the clear (step 12). The
first run's record is `.andromeda/runs/2026-10-19T10-56-42-implement/outcome-2026-10-19-overseer-switch-to-viola.md`
(Parts A and B, the gate trail of entries 1–19, the code and fix-loop evolve checkpoints).

## For the next entry, after `/clear` (this window is gone then: read this file, the first run's outcome,
## `evidence/switch-round.md`, `evidence/switch-readings.ndjson` and `evidence/profraw-red.md` first)
- **Step 12** records three acts in `evidence/switch-readings.ndjson`, read from `~/.viola/instances/viola-builder/events.ndjson`
  and `~/.viola/diagnostics/run-viola-builder.ndjson`. Kinds, cursors and durations only; never text.
  - `turn-waited`: the `turn-ended` line that ends THIS turn (the first `turn-ended` after cursor 4981), its cursor
    and its `background_tasks` count, then the overseer's `wait` `channel-request` / `channel-response` that woke on it.
  - `final-read`: the `last` `channel-request` / `channel-response` after that turn end. The overseer reports the
    exit code (0 expected).
  - `clear-confirmed`: the `/clear` send's `send-issued` / `send-confirmed` and `session-start{cause:"clear"}`, whose
    `agent_session_id` is new (not `6e59945a-f08d-4221-8d42-ba26deefbf1b`). Then the next send (the re-entry).
  - Read the log with `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/6e59945a-f08d-4221-8d42-ba26deefbf1b/scratchpad/events_shape.py <log> [after]`
    (it prints offsets, kinds and non-text fields only). It may not survive; if it is gone, write the same reader again.
- **Step 13:** `switch-round.md` gains:
  - the act table;
  - the live-start ledger (7 of 8 spent, unless a start is added);
  - the CLI residual (baseline 2026-10-19T11:00:50Z: `~/.claude/projects` 111, `*viola-verify*` transcripts 70; after
    the verify 115 / 75);
  - the count of `event:"panic"` lines in `~/.viola/diagnostics/*.ndjson` (read 0 at 13:29Z).

  Then fire the readers, entries 23 and 24 (`gate.py run … --only 23,24`, this run dir): 23 expects `last line 7`
  (seven acts), 24 expects `last line 1`. Then:
  - the evolve smoke checkpoint;
  - the matrix: v1-33 is gated, so it gets NO ref until step 14 clears;
  - P4's scope read and census;
  - this outcome, rewritten whole.
- **Step 14 (gated):** the founder's key and his own `release`; never typed by an agent. Entry 25 stays deferred
  unless the block clears.

## Readings this entry (steps 10–11), in `evidence/switch-readings.ndjson`
- `verify-stamped`: 2.1.296, 24 pass, 0 fail, sha256 `231c0225…db52e3755`; home `755` after the verify, `700` after
  the switched start.
- `switched`: `wheel{driver,start}` @0 → `budget-gate{paused:false}` @144 → `session-start{cause:resume}` @275
  under the step-9 id.
  - Snapshot: `cli_version` 2.1.296, `cli_verified` true.
  - Exec-form commands: 11, all absolute and pinned. `hooks.json` 9/9, `.mcp.json` 1/1, `settings.json` 1/1.
  - The child's `env_stripped_count` is 11; the names are recorded.
  - The overseer's O1–O6 reports are recorded.
- `skill-sent`: `send-issued` @465 → `prompt-submitted{origin:driver}` @594 → `send-confirmed` @1458, 108 ms.
- `review-answered`:
  - The question @3700 (dialog 1) carries `option_details` with `description` and `preview` present for both options.
  - `dialog-answered{corr 1, question, duration_ms 27345, deadline_hit false}`, then `hook-decision{decision_emitted:true}`
    on the verified CLI.

## Deviations and gaps so far (to surface at P4)
- **`send-confirmed{confirmed:true}`:** the plan and v1-33 write it this way. The line reads `{cursor}` with no
  `confirmed` key, which events.md's contract (`confirmed?`, set only for an unconfirmable send) makes the confirmed
  form. A wording gap between the acceptance and the contract.
- **`dialog-answered` with `from`:** v1-33 names it. The overseer's answering process carried no `VIOLA_NAME`, so no
  `from` was sent and the line holds none. Its notes: "the record stands … record what it reads". Recorded as read.
  The same holds for the skill send (`send-issued` without `from`), so no implicit link was recorded.
- **The dialog hold:** 27.3 s, far below the 3000 s near-ceiling reading. The CARRY stays owed.
- From the first run: step 8's mode premise (`755`, not `700`, until the first `run` start); step 5's fixtures (null
  previews, a count of 0); one ledger clock for the verify's six starts; `pre-push` red once, open
  (`evidence/profraw-red.md`).
- `.claude/session-handoff.md`'s last section was rewritten by the prototype session's `/exit` ("Completed normally
  at 2026-10-19 15:26:36"). This is the carried note "A live `claude` session ending in this tree rewrites this
  file's last section"; implement does not touch the file.

## Live-start ledger
- 1–6: the verify (11:09:31Z).
- 7: O6's switched start (13:26:43Z, the overseer's).
- 7 of 8 spent.
