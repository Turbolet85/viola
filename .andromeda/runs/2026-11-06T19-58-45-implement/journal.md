# Journal — /implement 2026-11-06-version-done-check

Each long wait is written here before it begins (the operator's word on this run). Times UTC, read from `date -u`.

- 2026-11-06T20:09:17Z — wait 1: the gate block's first batch, entries 1–28, 31–36 and 50, fired backgrounded
  through `gate.py run --only`. Its end is the completion notice; the verdict is the printed summary line. Left out
  for now: 29–30 (perf), 37–39 (the live round's readers, which have nothing to read yet), 40 (pre-push), and the
  operator entries 41–49.
- 2026-11-06T20:10Z — the batch was first fired with `--only 1-28,…`; `--only` takes `N,N` only, and gate.py answered
  with a ValueError traceback (no entry ran). Re-fired in the documented form; the batch read green 33, recorded 2,
  red 0 (summary line).
- 2026-11-06T20:12:45Z — wait 2: live session 1 of 2, one `viola verify` against claude 2.1.287 by path, about 1–2 min,
  in the foreground under the Bash bound. Readings before: `hyprctl locked` false; census 0; `claude --version`
  `2.1.287 (Claude Code)`; `target/debug/viola` sha256 prefix `69e1a7d92c040450`; `~/.claude/projects` 557 entries, 39
  named `*viola-verify*`; no `.viola-verify-*` dir at the root. Round dir `target/done-check-live-20261106T201225Z/`
  (plain btrfs under `target/`, not the e2e-home tmpfs), `vhome/`, `priv/`, `priv/cwd/` all 0700.
- 2026-11-06T20:14:06Z — session 1 ended RED: exit 1, `stamped 2.1.287  23 pass  1 fail`, row `statusline-rate-limits`;
  not re-fired. This session's own wrapped status line reads five_hour `unknown`, seven_day present (2 s old). Ledgered
  (reading red, census 0). The overseer's answers: the hold reads a question dialog (`inputs#I5`, before the verify);
  one unverified session 2 spends the second slot (`inputs#I6`, a departure from step 9's stop).
- 2026-11-06T20:16:18Z — wait 3: live session 2 of 2, one unverified wrapped start under `live-pty.py`, cwd
  `priv/cwd` (its trusted ancestor is the repository root, read through the CLI's own walk to the git root). Readings
  before: `hyprctl locked` false; census 0; `claude --version` 2.1.287; build sha256 prefix `69e1a7d92c040450`
  (unchanged since session 1). No `config.json` written: no hold is read on an unverified start. The waits inside it:
  `session-start` up to 90 s, then the start and fd readings, then `kill -KILL` of the wrapper and its child gone
  within 10 s.
- 2026-11-06T20:17:19Z — session 2 ended: readings green (start, fd table, killed wrapper: child gone 0.401 s), hold
  unread; census 0. Ledgered. The round is closed: 2 starts of 2.
- 2026-11-06T20:19Z — wait 4: the gate block's second batch, entries 29, 30, 37, 38, 39 and 40, backgrounded: perf
  (bound 3600 s), the three ledger readers, then pre-push (bound 7200 s). No source changed since the first batch.
  Read: green 5, red 1 (39, `last: 2`, the live round's own reading); pre-push coverage 3367 passed, 0 failed.
- 2026-11-06T20:2xZ — implement's report written (`outcome-2026-11-06-version-done-check.md`: SURFACED); hygiene
  clean; `rec.py land` tx 461. The operator pass begins, on the operator's word in the /implement invocation.
- wait 5: the operator pass's native gate, pre-push again (entry 40) on the final uncommitted tree, backgrounded.
  Later waits (the push and the seven CI reads) are journaled after the push lands (ci.md 2026-10-13: nothing is
  written into the tree between the pre-CI commit and the push).
