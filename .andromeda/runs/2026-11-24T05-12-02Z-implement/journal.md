# Journal — implement 2026-11-24-viola-up

- 2026-11-24T05:27:52Z — firing entry 10 (the R4a rehearsal over the fakes) once, the script now written, before any
  graded use. PATH read by eye: under `--rehearse` every viola process runs with `PATH=$bin` alone (the scratch bin
  holding the fake `claude`, `foot`, `hyprctl`); the script's own compositor reads go through `"$hyprctl"` =
  `$bin/hyprctl`; the two bare `hyprctl` words stand only in the live `else` arm; `workspace_of` returns before its
  clients read under `--rehearse`. Backgrounded; waiting on its notification.
- 2026-11-24T05:28:43Z (read at the write, after the firing began) — entry 10 green (`r4: green phase=windows
  order=parents-first placed=2/2 lines=matched`, then the rehearse last line); entries 11 and 12 green. Fired just
  before: the four controls (`control-1.toml`: up-skip-live, up-order, up-boot, up-ended) on entry 4 with
  `--timeout 900`, backgrounded; waiting on its notification.
- 2026-11-24T05:29:20Z — controls: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced (evidence/control-1.txt).
  Firing entries 1-9 backgrounded; waiting on its notification.
- 2026-11-24T05:30:32Z — entries 1-9: two reds, both this chunk's own (1: fmt on two files a scripted edit left
  unformatted; 7: `help_lists_only_served_verbs`' served list lacked `up`), fixed (`cargo fmt`; `up` added to the
  list), re-fired `--only 1,7` green. Host read `hostwatch.py read --last 15`: QUIET, 0 s stalled; `df -m /tmp` 31071
  MB free. Firing entries 13-15 (the three whole layers) backgrounded; waiting on its notification.
- 2026-11-24T05:32:35Z — 13, 14, 15 green. Host QUIET (5 s stalled, this repository's own layers alive); /tmp 30649
  MB free. Firing the whole block once on the final tree, backgrounded; waiting on its notification.
- 2026-11-24T05:35:09Z — whole block: green 12 · red 2 (14, 15) · recorded 1, on the tree 13-15 had read green on
  minutes before, no edit between. RECORDED before any re-fire:
  - 14 (`run --integration`, log 14.2.log): `cli_up up_without_foot_refuses_terminal_not_found_and_spawns_nothing`,
    `left: "gone  no-session\n" right: ""` (tests/cli_up.rs:646). Cause read: `viola open` returns at the live
    endpoint, before the fake's `session-start` line; the case killed the wrapper with no session logged. This
    chunk's own test race; the fix waits for the `session-start` line after every open.
  - 15 (`run --e2e`, log 15.2.log): `tui_channel_fds tui_channel_fds_the_child_holds_nothing_of_violas`, `fd 4 holds a
    viola home file: … plugin/0.1.0-48d6da3c62f2489e/hooks/hooks.json` (tests/tui_channel_fds.rs:89). Cause read:
    the test's own comment names the fake agent's launch opening `hooks.json` after its start receipt; it retries
    only a link that vanished mid-read, never one read while still open. Neither the test, the fake's launch path
    nor the plugin is in this chunk's diff. Not re-fired for green.
- 2026-11-24T05:35:52Z — `cli_up`'s open now waits for the `session-start` line. Host QUIET (5 s stalled, this
  repository's own builds alive); /tmp 30648 MB free. Re-firing 1, 2, 6, 14, 15 backgrounded (15 as a measurement:
  its recorded red stays open whatever it reads); waiting on its notification.
- 2026-11-24T05:41:02Z — 1, 2, 6, 14, 15 green. The scope read is clean; the census finds none of this run's
  processes; the report landed (rev 1); hygiene clean; the chunk's files landed. STOPPED before the operator pass on
  the open `tui_channel_fds` red (CLAUDE.md's revision-door rule, `inputs#I4`'s stop). No pre-push, commit, push or
  CI read; no R4 entry fired.
