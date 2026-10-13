# Live claude starts — ledger

Chunk 2026-10-13-paste-newline-ledger-row. The founder's cap (`inputs#I2`, fork 2): exactly five live `claude`
starts, one `viola verify --record` round on 2.1.287 (`~/.local/share/mise/installs/claude/2.1.287/claude`, by path)
on the headless PTY, fired once and never retried; a red round stops and reports. Each start is ledgered here with
the `date -u` time read before it is made. One `viola verify` invocation makes the five in sequence. The gate
entry is plan entry 8, fired through `gate.py run --live-legs`. So all five carry the one time read just before
that firing.

Cap used before this ledger: 0 of 5.

| # | start | ledgered at (`date -u`, before the start) |
|---|---|---|
| 1 | print probe (`claude -p`, the spine) | 2026-10-13T21:32:02Z |
| 2 | Run A (untrusted, OS temp dir) | 2026-10-13T21:32:02Z |
| 3 | Run B (trusted, `.viola-verify-<pid>/`, five pastes) | 2026-10-13T21:32:02Z |
| 4 | Run C (dialogs, `.viola-verify-<pid>-dialogs/`) | 2026-10-13T21:32:02Z |
| 5 | Run D (plan, `.viola-verify-<pid>-plan/`) | 2026-10-13T21:32:02Z |

Verify's own `<program> --version` read is part of its round, as in every earlier round (plan §Test Commands);
the probe entry before it runs `test -x` and starts nothing. The round's outcome and the census are in
`live-round.md`.
