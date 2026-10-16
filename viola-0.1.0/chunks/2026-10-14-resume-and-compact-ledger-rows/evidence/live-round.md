# Live round — 2026-10-14-resume-and-compact-ledger-rows

This file holds codes, counts, key names and equalities only, with no prompt or payload text. The host is the
Linux dev host, and the CLI is `claude` 2.1.287, named by its install path. The cap is the founder's twelve starts
(`inputs#I1`, fork 3).

Each round was ledgered with `date -u` before it was fired, in the implement run dir's `journal.md`. Each round was
fired once and neither was re-fired. The round listings are `round-071816Z.txt` and `round-071942Z.txt`.

## Starts

Round 1 is the record entry: gate 8, `viola verify --record` with `--home …/vhome`, ledgered at
2026-10-16T07:17:12Z.

| Start | What | Rows it measured | Reading |
|---|---|---|---|
| 1 | print probe (`verify-probe`) | 01–06 | pass |
| 2 | Run A, untrusted | 07 | pass |
| 3 | Run B, trusted, five pastes, probe `--settings` | 08–10, 15–21 | pass |
| 4 | Run C, dialogs | 11, 13, 14 | pass |
| 5 | Run D, plan | 12 | pass |
| 6 | Run E, `--resume <Run B's first id>`, `-resume` dir | 22 | pass |

- Summary line: `stamped 2.1.287  22 pass  0 fail`. Gate 8 was green in 52.67 s.

Round 2 is `run --local-live`: gate 11, with the 2.1.287 install dir heading `PATH`. It was ledgered at
2026-10-16T07:18:40Z.

| Start | What | Reading |
|---|---|---|
| 7–12 | print probe and Runs A–E, the same six shapes | every row pass |

- Summary line: `stamped 2.1.287  22 pass  0 fail`.
- The harness suite read `"suite":"local-live","passed":1,"failed":0`. Gate 11 was green in 48.43 s.
- The tmpfs `--local-live` home (`target/e2e-home/viola-live-2894516/`) holds 22 rows under 2.1.287, all `pass`,
  `resume-session` included.

Starts used: **12 of 12**.

## Run E's SessionStart (round 1, the recorded variant)
- `source`: `resume`.
- `session_id`: equal to Run B's first SessionStart id (the id the paste variants carry), and not equal to the id
  after `/clear`.
- Key names (10): `context_tokens`, `cwd`, `estimated_cache_write_usd`, `hook_event_name`,
  `prompt_cache_likely_expired`, `scratchpad_dir`, `seconds_since_last_response`, `session_id`, `source`,
  `transcript_path`. These match the key-name list in `live-revive.md`.
- Its `transcript_path` and `cwd` name Run E's own `-resume` project directory. So `--resume <id>` found Run B's
  session from another project directory. This is the first live reading of the cross-directory lookup that P5 read
  from the CLI's code.
- `fixtures/claude/2.1.287/SessionStart.resume-1.json`: 578 B, one line ending in LF, byte-equal to the recording.
- The 2.1.287 set now holds 32 files (`ls fixtures/claude/2.1.287 | wc -l`).

## CLI transcripts (the security-plan accepted-risk count)
- The count is `*.jsonl` directly under `~/.claude/projects/*/`. Names were compared by shape: the project
  directory with the user and the id masked.

  | When | Count |
  |---|---|
  | before round 1 | 643 |
  | after round 1 | 647 |
  | after round 2 | 651 |

- Each round added four files: two under Run B's project dir (the first session, and the one after `/clear`), one
  under `-dialogs` and one under `-plan`.
- Run E added **no transcript file** in either round, although its payload names a `-resume` path.
- The print probe's `--no-session-persistence` added none.

## Census
- After gate 8 the round tool's own census found no survivor.
- Gate 9 (census after round 1): green, no output.
- Gate 12 (census after round 2) runs only in a whole round, so it was run by hand with the plan's exact command at
  2026-10-16T07:19:50Z. It printed no pid.
- At the same reading no `.viola-verify-*` and no `/tmp/viola-verify-<suffix>` dir was left.
