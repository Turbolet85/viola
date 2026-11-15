# Hold journal — entry 10 (`live-hold.py live-wrapped`), run 2026-11-15T09-11-10-implement

Written 2026-11-15T09:13:08Z (`date -u`), BEFORE entry 10 is fired and so before the hold's wait. The CLI compacts an
idle window after about 55 min: a window that resumes mid-hold re-reads this file first.

## What is running
- Entry 10, fired ONCE on the operator's word (inputs#I18), through
  `python -X utf8 {tools_dir}/gate.py run --chunk 2026-11-15-live-round-on-the-trimmed-product --run-dir .andromeda/runs/2026-11-15T09-11-10-implement --operator 10`,
  backgrounded (timeout 4500 s). The harness's completion notification is the wake; no poll, no `ScheduleWakeup`.
- One live start: the 7th of the founder's 9 (6 spent by entry 9). `claude` 2.1.325, mise's `latest` read `match`.
- Plain build `target/debug/viola` sha256 prefix `a0e247fd3a77f744`.
- Home: `target/e2e-home/viola-live-20261115T083805Z/home` (entry 9's kept home, stamp `pass` for 2.1.325, read by
  entry 21); instance `round`; `config.json` `dialog_hold_s` 3600; cwd `target/e2e-home/viola-live-20261115T083805Z/proj`
  (its own git repo, trust keys on `trust yes`).

## Expected timeline (durations, a prediction, not a reading; the rig's own journal holds the send's real time)
- Within about a minute of the firing: the wrapped start, the start readings, the two name refusals, the status-line
  reading.
- Then: `x` and DEL into the input box; the box read from the screen model and recorded; no further key; `viola
  release`; the send. The send's outcome is the second witness. The rig writes `evidence/hold-journal.md` at the send.
- The `question` line within 300 s of the send; then nothing typed until `dialog-expired`, about 3600 s after the
  question, bounded at 3660 s (the expiry's clock time is in the rig's own journal, written at the send).
- Then the killed-wrapper leg (SIGKILL of the wrapper, the child gone within 10 s), the census, the closing line.

## What a resumed window re-reads (in order)
1. This journal.
2. `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/hold-journal.md` — the send's UTC time
   and the expected expiry, written by the rig at the send.
3. The gate log of entry 10: `$TMPDIR/andromeda-gate/2026-11-15-live-round-on-the-trimmed-product/implement-2026-11-15T09-11-10/10.log`
   (the rig's ONE closing line arrives there at the end), and after the notification
   `gate.py show --run-dir .andromeda/runs/2026-11-15T09-11-10-implement --marker 2026-11-15-live-round-on-the-trimmed-product --n 10`.
4. The instance's `events.ndjson`: `target/e2e-home/viola-live-20261115T083805Z/home/instances/round/events.ndjson`
   (`question`, `dialog-expired`, `wheel` lines) — read only.
5. `evidence/live-sessions.ndjson` (entry 10's `start` row, then its `reading` row) and `evidence/round-state.json`.

## Rules while it runs
- Do NOT re-fire entry 10 whatever it reads; type nothing into the session; start nothing else live.
- If the background task is gone with no notification, read the gate log and the census
  (`gate.py census --args viola-live-20261115T083805Z`), and report; never fire again.
- After the notification: record the entry's line and summary in `evidence/`, report the start readings, the box
  reading beside the send's outcome, `dialog-expired` / `deadline_hit` / `duration_ms`, and the killed-wrapper reading,
  then STOP. Entry 11 waits for its own word.
