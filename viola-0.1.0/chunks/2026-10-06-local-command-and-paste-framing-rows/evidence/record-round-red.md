# Step 10 — the record round on `claude` 2.1.287: RED, not retried

**Outcome: `stamped 2.1.287  15 pass  2 fail`, exit 1. `tag-escaping` and `local-command-clear` failed. Nothing
was recorded and nothing was copied into `fixtures/claude/2.1.287/`. The round was fired once and is not retried;
it returns to the founder (inputs#I2).**

## How it ran
- The five ledger rows (2-6 of `live-sessions.md`) were written at 2026-10-06T20:34:33Z, before the round.
- Fired once through `gate.py run --live-legs`: entry 6, the version probe, green (`2.1.287 (Claude Code)`, 0.02 s);
  entry 7, the record entry, red after 44.4 s. Its lines and the summary are in `round-203444Z.txt`:
  `round: STOPPED at 7 (red · exit 0 ✗ (exit 1)) · legs fired 1/1`.
- The record home is `target/e2e-home/viola-record-20261006T203444Z/` (git-ignored). Its stamp names 2.1.287 with
  15 rows `pass` and 2 `fail`; no home outside it was written.
- Step lines 1-15 read `pass`: the six print rows, the modal and input-box signatures, the two timing rows, the
  four dialog rows and `long-paste-wrapper`. Lines 16 and 17 read `fail`.

## What the CLI received in Run B (measured: the CLI's own transcript of the run)
Read after the round from the one transcript the CLI wrote for Run B's dir under `~/.claude/projects/`, by a
scratch script that prints entry types, lengths and the synthetic prompt text only:

| time (UTC) | entry |
|---|---|
| 20:34:52.465 | user, 49 chars: the probe prompt |
| 20:34:54.501 | assistant `ok`; the turn's stop summary at .557 |
| 20:34:55.171 | user, 1 558 chars: the long text, wrapped — two newlines, the pair, one newline; the id is `31a3` |
| 20:34:56.593 | assistant `ok`; the turn's stop summary at .628 |

- There is no third prompt and no second transcript. The tag-like text and the probed local command were never
  pasted. Step 0's Run B dir holds two transcripts (the session and the one its `/clear` opened); this run's holds
  one.
- The long paste's id was `7ccf` at step 0 and `31a3` here: the same text, a different id.
- Run C's project dir was created about 12 s after Run B's, so Run B lasted about 12 s: no wait ran out the 120 s
  probe deadline.

## Why the two pastes were not made (two measurements joined; the cause is not measured inside the round)
- Read from the code: after each added turn Run B settles the screen and pastes the next text only when the
  settled rows hold the compiled input-box literal `for agents` and no modal literal
  (`src/cmd/verify/typed.rs`, `framing_turns`). A quiet screen with no compiled literal settles at
  `GATE_MAX_WAIT`, 5 s after the turn's Stop capture, and returns its rows as they are.
- Measured at step 0 (its drive log, kept in the earlier session's scratch dir): after the long-paste turn the
  input box settled **5.8 s** after that turn's Stop, against 1.3 s after the tag-like turn and 1.3 s after the
  local command. Step 0 judged "settled" as the input-box literal on screen and 1 s of quiet.
- Measured from step 0's raw PTY bytes, fed prefix by prefix through viola's own `Screen` (the scratch helper
  step 0 built): from the long paste on, row 23 reads `paste again to expand` and **no row holds `for agents`**;
  the footer `⏸ manual mode on · ← for agents` is drawn again only near the end of that window (raw byte 7 500 of
  the 7 621 read at the settle). The paste was written at byte 1 584.
- So at step 0 the hint stood for about 8 s from the paste: the turn took 3.4 s, and the footer came back about
  4.8 s after its Stop, inside the 5 s maximum.
- In the record round the same turn took 1.5 s (paste 20:34:55.171, stop 20:34:56.628). If the hint stands the
  same 8 s from the paste, the footer was still replaced 5 s after the Stop: the settle returned rows without the
  literal, and the guard pasted nothing more. Run B's length, about 12 s, fits that: ready, the first turn, the
  long turn, then one 5 s wait.
- **Not measured in the round itself:** Run B's captures and screen bytes are in the probe dir, which `verify`
  removes on every exit path. The hint's duration is inferred from one run (step 0), not measured as a timer.

## What follows from it
- `local-command-clear` failed with `tag-escaping` for the same reason: neither text was pasted. Its arm also reads
  the tag-like turn's capture to place the `clear` SessionStart after it, so it could not pass without that turn.
- The unwrap change is confirmed live a second time: `long-paste-wrapper` read `pass` on a prompt framed by two
  newlines and one, with an id the code had not seen.
- A reading for the founder, not measured end to end: the product's readiness gate reads the same input-box
  literal (architecture [Screen Model]). For several seconds after a long paste this CLI's footer does not show
  it, so a `send` issued in that window on a verified CLI would meet the gate's maximum wait.

## After the round (2026-10-06T20:36Z)
- No probe dir of this chunk is left at the repository root. `.viola-verify-2095228/` predates the chunk (the
  operator desk, the founder's word: leave it). No `viola-verify-*` dir is left under the OS temp dir.
- No `claude` process has a probe-dir cwd (`pgrep -x claude`, each cwd read: none under a record home or a
  `.viola-verify-*` dir).
- `fixtures/claude/` is unchanged (`git status --short fixtures`: no line).
- Residual: the CLI wrote one transcript each for Run B, Run C and Run D under `~/.claude/projects/`, the accepted
  class. Run B's is one lower than planned, because no `/clear` session was opened.
- Sessions: 6 of the cap of 8 are used. The 2 left are the spare step-0 sessions, unspent.
