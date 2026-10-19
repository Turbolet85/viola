# The switch round — 2026-10-19-overseer-switch-to-viola

Kinds, cursors, exit codes, durations and hashes only: no prompt, skill or dialog text.

## The driving binary (step 7)

- `target/release/viola` from gate entry 12 (`cargo build --release --bin viola`, green, 8.38 s) over the finished Part
  A tree (HEAD `f23586fbda77` plus the six Part A files uncommitted). Read at 2026-10-19T11:09:14Z.
- sha256 `231c0225dac912d9ab987e19d96adca281c3e481bac68e079810026db52e3755`, 8 100 512 B, mtime 2026-10-19T11:02:18Z.
- Part A's gates, entries 1–19 of the block, read green. Entry 19 (`pre-push`) went red once on a coverage merge
  refusal (`evidence/profraw-red.md`, open, its cause owed), then green on its second firing.

## Live-start ledger (cap 8, the founder's, `inputs#I14`)

Each start is ledgered with the clock read by `date -u` just before it. `claude --version` and `claude agents
--json` reads are not starts.

| # | start | ledgered at (UTC) | by |
|---|---|---|---|
| — | — | 2026-10-19T11:09:14Z | 0 of 8 spent |
| 1–6 | the one `viola verify` (the print probe, then Runs A, B, C, D, E) | 2026-10-19T11:09:31Z | the builder, step 8 |

Starts 1–6 are one command. The clock was read just before it, and the six spawns inside it are verify's own: this
home logs no spawn pair for a verify run without a `VIOLA_NAME`. **6 of 8 spent.**

## The verify of claude 2.1.296 (step 8)

- `claude --version` by path: `~/.local/share/mise/installs/claude/latest/claude` (→ `…/claude/2.1.296/claude`)
  reads `2.1.296 (Claude Code)`, and so does `~/.local/bin/claude`. The bare `claude` resolves to the first of them.
- The run: `target/release/viola verify` from the repository root, no `--home` (the default `~/.viola`), with
  `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed from its environment. 11:09:31Z → 11:10:29Z (58 s), exit 0,
  stderr 0 B, rows `[01/24]`–`[24/24]` all `pass`. The last line:

  stamped 2.1.296  24 pass  0 fail

- `ledger/` 700, `ledger/stamps.json` 600, `bin/` 700. The verify's scratch dirs `.viola-verify-635187*` are gone.
- **Mode reading (a premise of step 8 falsified):** `stat -c %a ~/.viola` reads `755`, not the `700` the plan
  expected. The verify opened no home log (the home holds no `diagnostics/`), so nothing narrowed it. The narrowing
  `src/obs.rs:43-46` performs comes with the first `run` start's log open, so it is read at step 10 on the switched
  start. `sessions/` and `archive/` are still listed beside `bin/` and `ledger/`, untouched.
- **CLI residual under `~/.claude`** (counted, never removed; baseline read 11:00:50Z):
  - `~/.claude/projects/` went from 111 to 115 entries. Three of the four are this verify's
    (`*viola-verify-635187`, `-dialogs`, `-plan`); the fourth is not attributable to it.
  - Transcripts under `*viola-verify*` went from 70 to 75, the five of this run (Runs A–D's and one CLI `-backup-`
    file). Run E added none.

## The switch point (step 9)

- Hand-off: binary sha256 `231c0225dac912d9ab987e19d96adca281c3e481bac68e079810026db52e3755` · the stamp line of step 8
  (2.1.296, 24 pass, 0 fail) · prototype session id `6e59945a-f08d-4221-8d42-ba26deefbf1b` (read from
  `~/.viola/sessions/viola-builder/session_id`) · ledger 6 of 8 spent.
- Nothing further runs under the prototype. Next: the overseer's O1–O6. O6's `run … --resume <the id above>` is
  live start 7.

## The overseer's switch (O1–O6, its own reports, step 10)

- O1: the binary copied, 0700, sha256 equal to step 7's. O2: `config.json` `dialog_hold_s` 3600, 0600.
- O3: `vb.sh` / `vbr.sh --product` through `vstart.sh --product` (andromedaV3 `0f551a2`); the prototype forms kept.
- O4: the `up.sh` row text. O5: `pwait.sh` (`wait --json`, `turn-ended` `background_tasks`); `waitm` / `waitlong`
  kept. O6: the prototype ended by `/exit`; the product start at 13:26:43Z with `--resume` of the step-9 id.

## The act table (steps 10–12)

Read from `~/.viola/instances/viola-builder/events.ndjson` (cursor = the line's byte offset) and
`~/.viola/diagnostics/run-viola-builder.ndjson`. One line each in `switch-readings.ndjson`.

| act | records (cursor) | reading |
|---|---|---|
| `verify-stamped` | `ledger/stamps.json` 2.1.296 | 24 pass, 0 fail; exit 0, 58 s |
| `switched` | `wheel{driver,start}` @0 · `budget-gate{paused:false}` @144 · `session-start{resume}` @275 | `cli_version` 2.1.296, `cli_verified` true; 11 exec-form commands, 11 absolute |
| `skill-sent` | `send-issued` @465 · `prompt-submitted{driver}` @594 · `send-confirmed` @1458 | 108 ms; the diag line reads `confirmed:true` |
| `review-answered` | `question` @3700 (`option_details` present) · `dialog-answered{corr 1}` | 27 345 ms, `deadline_hit` false, no `from` |
| `turn-waited` | `turn-ended` @5489 (ends @7895) | `background_tasks` 0; `wait --after 4981` woke on it after 74 499 ms, outcome `turn-ended` |
| `final-read` | `last` request/response, 13:31:37.756Z and 13:31:52.368Z | both `result_class` ok, 0 ms; the exit code is not in the relay |
| `clear-confirmed` | `send-refused{budget-paused, seven-day}` @8046 · `budget-gate{paused:false, override_until}` @9519 · `send-issued` @9694 · `session-end` @9824 · `session-start{clear}` @9938 · `send-confirmed` @10127 | a new `agent_session_id`; 123 ms; the next send @10260 → `prompt-submitted{driver}` @10391 → `send-confirmed` @11069, 68 ms |

Between the turn end and the clear (`inputs#I16`, the `restarts` line):
- The first `/clear` was refused `budget-paused` (seven-day at 99 %).
- A host restart ended the first run: `session-end` @8321 at 17:29:38Z, with no `process-exit{subject:"self"}` line.
- Start 8 at 18:07:34Z resumed the step-9 id on claude 2.1.302 with `cli_verified` false.
- The overseer's `viola verify` of 2.1.302 stamped `20 pass  4 fail` at 18:21:41Z. The failing rows are
  `tag-escaping`, `local-command-clear`, `paste-newline` and `local-command-compact`. The 2.1.296 stamp beside it
  still reads 24 pass.
- Start 8 ended at 18:26:19Z: claude child exit 143, the run's own exit 0.
- Start 15 at 18:26:23Z resumed the same id on claude 2.1.296, through the overseer's PATH shim, with
  `cli_verified` true.
- The founder lifted the gate with his own `viola release --budget` @9519, at 18:31:54Z.

## Live-start ledger, closed (the founder's cap 8 → 14 → 15, `inputs#I14`, `inputs#I16`)

| # | start | at (UTC) | by |
|---|---|---|---|
| 1–6 | the `viola verify` of 2.1.296 | 11:09:31Z | the builder, step 8 |
| 7 | O6's switched start (2.1.296) | 13:26:43Z | the overseer |
| 8 | `vb.sh --continue` after the host restart (2.1.302) | 18:07:33Z | the overseer |
| 9–14 | the `viola verify` of 2.1.302 | 18:18:43Z → 18:21:41Z | the overseer, on the founder's «Verify 2.1.302, +6 стартов» |
| 15 | `vb.sh --continue` under the 2.1.296 pin | 18:26:23Z | the overseer, on the founder's «Закрепить 2.1.296, +1 старт» |

**15 of 15 spent.** The `/clear` and the sends that followed started no `claude`. Starts 8–15 are recorded from the
relay `inputs#I16` and the run log's `process-start{subject:"claude-child"}` lines (18:07:34Z with 2.1.302, 18:26:23Z
with 2.1.296). The builder read them; it did not make them.

## CLI residual under `~/.claude` (counted, never removed)

| reading | `~/.claude/projects/` entries | files under `*viola-verify*` |
|---|---|---|
| baseline, 11:00:50Z | 111 | 70 |
| after the 2.1.296 verify | 115 | 75 |
| step 13, 2026-10-19T18:34Z | 120 | 78 |

- Of the 5 new entries, 4 carry the 2.1.302 verify's pid 3090040: `*-viola-ledger-probes-3090040`,
  `*viola-verify-3090040`, `-dialogs` and `-plan`. The fifth cannot be attributed.
- The same pid match corrects step 8's reading. The fourth entry it could not attribute is
  `*-viola-ledger-probes-635187`, the 2.1.296 verify's own.
- The 2.1.302 verify's scratch dirs `.viola-verify-3090040*` are gone from the repository root.

## Panic lines (obs-plan §10)

`event:"panic"` lines in `~/.viola/diagnostics/*.ndjson` (`cli-`, `hook-`, `run-viola-builder.ndjson`, `mcp.ndjson`):
**0** in each file, read at step 13.
