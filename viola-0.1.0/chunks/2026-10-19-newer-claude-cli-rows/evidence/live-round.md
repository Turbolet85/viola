# The gated live round — chunk 2026-10-19-newer-claude-cli-rows

The block cleared on the founder's own answers (`inputs#I6`, read 2026-10-20T04:59:04Z):
- **Fork 1, option 1:** 19 live starts. That is 1 footer reading, 6 for the record round and 6 for `--local-live`,
  with 6 held for one re-run, spent only if the record round reads red. Step 13 (the option-2 2.1.296 verify, entry 18)
  does not run.
- **Fork 2, option 1:** one scratch start, footer rows only.

The operator's invocation (`inputs#I7`) asks for every live start to be ledgered before it is made, with the clock's
time. Each row below was written before its firing, with the `date -u` read just before it.

## Start ledger

| start | what | entry | written at (`date -u`) |
|---|---|---|---|
| 1 | the footer reading: one `viola run` of claude 2.1.302 in a detached tmux pane (`footer-reading.sh`) | 11, fired with `--live-legs --entry 11` (entries 9 and 10 before it start nothing) | 2026-10-20T05:05:22Z |
| 2–7 | the record round: one `viola verify --record` of claude 2.1.302 (the print probe, then Runs A–E, in that order), after step 9's literal | 13, fired with `--live-legs --entry 13` (entry 12 before it starts nothing) | 2026-10-20T05:11:56Z |
| 8–13 | `--local-live` with the 2.1.302 install first on `PATH`: one `viola verify` stamping the harness-built `target/e2e-home/viola-live-<pid>/home` (the print probe, then Runs A–E) | 16, fired with `--live-legs --entry 16` (entries 14 and 15 before it start nothing) | 2026-10-20T05:14:00Z |

**Starts made: 13 of the 19.** The 6 held for a re-run were not spent, because the record round was green on its first
firing. Step 13 (entry 18, the 2.1.296 verify) did not run, because Fork 1 is option 1.

## The round's readings

Each firing's lines are in `round-050522Z.txt`, `round-051156Z.txt` and `round-051400Z.txt`, verbatim and without the
header.

- **Entry 11, the footer reading** — `green · exit 0 · 28.9s`. The rows are in `footer-2.1.302.md`: the
  `for agents` hint was gone at ~15 s and ~25 s, and the resting row was `  ⏵⏵ auto mode on (shift+tab to cycle)`.
- **Entry 13, the record round** — `green · exit 0 · 69.2s`.
  - Its log holds the 24 `[NN/24]` lines and then `stamped 2.1.302  24 pass  0 fail`.
  - `fixtures/claude/2.1.302/` was copied whole (29 files).
  - Its `Screen.ready.json` and `Screen.turn.json` keep one row, `  ⏵⏵ auto mode on (shift+tab to cycle) · ← for
    agents`: the hint before it expires, joined with the measured resting text. `Screen.modal.json` keeps
    `   Yes, I trust this folder`.
  - The record home's `ledger/stamps.json` holds 2.1.302 with all 24 rows `pass`. `stamps_verdict` reads such a stamp
    as `(true, None)`, which is `cli_verified:true` (its unit case `stamps_verdict_reads_every_arm`). That was read
    from the file, with no further start.
- **Entry 16, `--local-live`** — `green · exit 0 · 63.71s`.
  - Its verify printed `stamped 2.1.302  24 pass  0 fail`.
  - Its document was
    `{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"local-live","passed":1,"failed":0,"skipped":0,"survived":0,"artifact":null,"failures":[]}]}`.
- **Censuses.** Entries 12 and 14 were green with no output, and every firing's round census left no survivor line.
  - Entry 17 (the census after `--local-live`) did not run through the tool. In block order it sits between leg 16
    and leg 18, so `--entry` fires it only together with leg 18, and leg 18 is not run under option 1. Its loop was
    run by hand right after the firing and printed nothing. No `.viola-footer-*` or `.viola-verify-*` dir was left
    at the root.
- **Transcripts** (`~/.claude/projects/*-viola-*/*.jsonl`, entries 10 and 15):
  - 107 before the round, 112 after the record round. That is +5: the record verify's Run B 3 (its session, a backup
    file beside it and the `/clear` session), Run C 1 and Run D 1, and Run E 0.
  - The footer reading wrote no transcript, since no prompt was typed. That is below the one the plan expected for it.
  - `--local-live`'s verify then added the same 3 + 1 + 1 class (read by name after the round; no entry counts it).
  - Every file is in the accepted class (security-plan §Data Protection, the residual of verify's trusted runs).
- **Left on the tmpfs (the founder's desk):** the round's homes `target/e2e-home/viola-footer-20261020T050534Z/`,
  `target/e2e-home/viola-record-20261020T051203Z/` and the `--local-live` harness home `viola-live-<pid>/`.

Host facts read at the first ledger row:
- `hyprctl locked` read `true`. No step of this round focuses a window or types a key with `wtype`: the pane is a
  detached tmux session, and no key is typed.
- `/tmp` (tmpfs) had 15 G free.
