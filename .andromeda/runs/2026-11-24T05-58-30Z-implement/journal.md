# Journal — implement 2026-11-24-viola-up (re-entry, plan rev 10)

- 2026-11-24T05:59:01Z — directive read whole and snapped (`inputs#I9`); the invocation's word written to
  `relay-1.md` and snapped (`inputs#I10`). Step 21 done in `tests/tui_channel_fds.rs` (a whole table holding a non-fixture
  home file is read again inside the case's 2 s deadline; at the deadline the last whole table is asserted as before).
  Entries 1 and 16 green (16: 1 test across 1 binary, passed). The `fds-held-red` control (`control-1.toml`) fired on
  entry 16 with `--timeout 1800`, backgrounded; waiting on its notification.
- 2026-11-24T06:00:03Z — control: 1 holds (in place green 48.22 s; removed red, `fd 4 holds a viola home file`, the
  case's own assertion; clone removed) → `evidence/control-2.txt`. Host read `hostwatch.py read --last 15`: QUIET,
  0 s stalled, only this repository's own control build alive; `df -m /tmp` 31070 MB free. Firing entries 13-15 (the
  three whole layers) backgrounded; waiting on its notification.
- 2026-11-24T06:06:15Z — 13-15 green; clippy green; scope clean; census none of this run's; report rev 2 landed
  (tx 4557); hygiene clean; files landed (tx 4559). Operator pass: host QUIET (06:04:07Z); pre-push RED, exit 1, all
  4320 tests passed, coverage merge refused a corrupt fake-agent profile `viola-3061597-…_29.profraw`; host QUIET over
  the firing; cause unknown; RECORDED, not re-run. Pass STOPPED: no hygiene entry, commit, push or CI read; no R4
  entry.
- 2026-11-24T06:08:25Z — the overseer's answer `inputs#I11` (relay-up-profraw.md, snapped): measure with
  `scripts/profraw-census.sh --case`. The header names no counts for the case form (commands.md names `4800 48` for
  the root-bin form only), so every case runs 192 over 16 workers. Cases: the 11 `up_*`, the two `cli_revive` origin
  cases, `tui_channel_fds`; control: `cli_open`'s `open_spawns_the_terminal_with_the_closed_argv` and
  `open_records_the_window_found_by_the_terminal_pid` (the same SIGKILL-by-pid-and-start-time guard as `cli_up`'s
  `Terminals`). Host QUIET, /tmp 31068 MB. Driver `census-run.sh` backgrounded; waiting on its notification.
- 2026-11-24T06:15:54Z — census done, every run passed. Refused: 5, 0, 6, 16, 2, 2, 0, 14, 0, 0, 9 for the eleven
  `up_*`; 0 and 0 for `cli_revive`; 0 for `tui_channel_fds`; the control's `cli_open` cases 9 and 6. Every refused file
  carries the fake agent's signature. The control truncates, so this is `inputs#I11` step 4: not shown to be this
  chunk's. STOPPED, no mend (`evidence/profraw-census.md`).
