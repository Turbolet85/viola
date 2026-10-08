# The chunk's own compositor — its one start, and where it stands (2026-10-08)

**One start, 07:26:12Z; one end, 08:49:30Z, after the live work (step 9a). It stood 4 998 s. The desktop lock
read locked at every reading, the two after the end included. The desktop shell exited and relaunched once, at
07:35:05Z, and re-took the lock 2.2 s later; it did not exit at the end.**

Script and config: `evidence/live-compositor.sh`, `evidence/live-compositor.lua`. Ledger:
`evidence/live-sessions.ndjson` (one `compositor` line with `event: "start"` and one with `event: "end"`).

## The start

| when (UTC) | reading | result |
|---|---|---|
| 07:20:49 | `lock-read pre`, an early read before the helpers were finished | shell `true`, desktop instance `true` |
| 07:26:12.3 | `lock-read pre`, right before the start | shell `true`, desktop instance `true` |
| 07:26:12.4 | `start` | nested: one client window of the desktop compositor; `env -i`, no session bus, the seat backend forced to an absent `seatd`; own instance up in 0.2 s, signature not the desktop's, control and Wayland sockets in the private runtime dir (`$XDG_RUNTIME_DIR/vcomp`); output `WAYLAND-1` 1261x688 at scale 1; own instance `locked` false; 0 windows |
| 07:26:12.581 | the desktop shell's journal | `idle-monitor: active`, 0.2 s after the start. No wake line, no exit, no relaunch followed it |
| 07:26:14.5 | `lock-read after-start`, first reading (+2.1 s) | shell `true`, desktop instance `true` |
| 07:26:27.5 | `lock-read after-start`, second reading (+15.1 s) | shell `true`, desktop instance `true`; the shell's pid unchanged |

The start did not end the shell this time (the revision's one nested run did, 4 s after its start).

## Every later reading of the desktop lock (all a read only)

| when (UTC) | after | shell | desktop instance | shell pid against the reading before |
|---|---|---|---|---|
| 07:28:32 | the round | `true` | `true` | unchanged |
| 07:32:38 | the live session's close | `true` | `true` | unchanged |
| 07:35:49 | the reply probe | `true` | `true` | **changed** (see below) |
| 07:36:45 | the same, re-read with the corrected relaunch reading | `true` | `true` | changed against the start's |

`hyprctl --instance <desktop signature> locked` is the only call of the chunk that names the desktop instance.
No key and no dispatch went to it. Every other `hyprctl` call went through `live-compositor.sh hc`, which sets
the private runtime dir and the own signature and removes the desktop's signature and socket name from the
call's environment.

A defect of the script, found and corrected at 07:36Z: `lock-read pre` compared the shell's pid with itself, so
its `shell_relaunched` could not read a relaunch between two calls; the 07:35:49 reading printed `false`. It
now compares with the pid of the reading before. The relaunch below is read from the journal.

## What the desktop shell did (its journal, as times and codes)

| when (UTC) | line |
|---|---|
| 07:26:12.581 | `idle-monitor: active` (the start) |
| 07:28:42.581 | `idle-monitor: idle`; `idle-cycle-start: screensaver=150 lock=300` |
| 07:31:12.591 | `lock-system: lock-timeout`; its `lock` process started and exited 0 |
| 07:35:01.268 | `idle-monitor: active`. **Not attributed**: no call of this chunk touched a compositor at that second (the reply probe's last window closed at 07:34:47Z; the call running then read a file) |
| 07:35:02.6, 07:35:04.5 | `There are no outputs - creating placeholder screen`, twice: the desktop's outputs went away and came back; a monitor-control tool ran at 07:35:03Z |
| 07:35:05.111 | the desktop compositor: `error in client communication` for the shell's pid |
| 07:35:05.705 | `The Wayland connection experienced a fatal error: Invalid argument`; the shell exited 255 and was relaunched |
| 07:35:07.350 | the new shell: `lock-stranded: recovering`, `lock-requested` |
| 07:35:07.911 | `secure=true`, 2.2 s after the exit |
| 07:35:43 to 07:35:44 | the outputs gone and back again; no exit followed |

This is the sequence `evidence/comp-probe.md` recorded four times earlier today (a wake, the outputs flapping,
the shell's exit 255, `lock-stranded: recovering`, `secure=true` 2.2 s later), three of them with no compositor
of this chunk running. Here it came 8 min 53 s after the start. The state of the lock inside those 2.2 s was
not read; the desktop instance's `locked` read `true` at the readings before and after.

The own compositor stood through it (alive by pid and start time at 07:35:49Z and 07:36:45Z). Its log grew
from 255 KB to 1 080 KB at that minute: the desktop re-sent its buffer format lists each time its outputs came
back. The log is on the private runtime dir (a tmpfs with 6.3 GiB free).

## What ran inside the one start

| when (UTC) | window class | what | key typed |
|---|---|---|---|
| 07:26:34 to 07:26:37 | `viola.keyprobe` | step 0: the harness build over the fake agent | one, `x`, after its guard |
| 07:28:41 to 07:32:37 | `viola.livetest` | start 6: the product build over `claude` 2.1.287 | none |
| 07:33:44 to 07:34:47 | `viola.replyprobe` | 23 windows, the product build over a script child, one terminal query each | none |
| 08:37:01 to 08:38:00 | `viola.replyprobe` | step 5c: 23 windows, the rebuilt product build over the script child, one terminal query each | none |
| 08:38:08 to 08:38:16 | `viola.keyprobe` | step 5c: three windows, the harness build over the fake agent | one each, `n`, `t`, `m`, each after its guard |
| 08:39:33 to 08:47:28 | `viola.livetest` | start 7: the product build over `claude` 2.1.287, the live run | one, `x`, the takeover, after its guard |
| 08:48:15 to 08:49:22 | `viola.readings` | start 8: the product build over `claude` 2.1.287, the two readings | none |

Between the first three the round (`run --local-live`, 07:27Z to 07:28Z) ran outside the compositor.

Five keys were typed in the whole start, each with `wtype` on the own socket and each after a guard that read
the chunk's one window active on the own instance: `x` into the probe window of step 0, `n`, `t` and `m` into
the probe windows of step 5c, and `x` into the live window at the takeover. No key and no dispatch went to the
desktop compositor.

## Between the plan revision and the end

The compositor stood with no window from 07:34:47Z to 08:37:01Z. The desktop shell's journal holds no idle,
lock, output or exit line between 07:41Z and 08:49:30Z.

| when (UTC) | reading | shell | desktop instance | shell pid against the reading before |
|---|---|---|---|---|
| 08:36:55 | `lock-read pre`, before step 5c | `true` | `true` | unchanged |
| 08:39:33 | `lock-read pre`, right before start 7 | `true` | `true` | unchanged |
| 08:48:14 | `lock-read pre`, right before start 8 | `true` | `true` | unchanged |

The bound of inputs#I15 (no live start by 10:32Z ends the compositor) was not reached: start 7 was made at
08:39:33Z.

## The end (step 9a)

| when (UTC) | reading | result |
|---|---|---|
| 08:49:30.1 | windows on the own instance before the end | 0 |
| 08:49:30.141 | `end` | TERM to the own compositor's pid, its runtime dir read from its environment first |
| 08:49:30.276 | the own compositor | gone; `own_left` 0 (no compositor, no window process of the chunk); one `Hyprland` process on the host, the desktop's |
| 08:49:30.230 | the desktop shell's journal | `idle-monitor: active`, 0.1 s after the TERM. No exit and no relaunch followed |
| 08:49:32.2 | `lock-read after-end`, first reading (+2.1 s) | shell `true`, desktop instance `true` |
| 08:49:45.2 | `lock-read after-end`, second reading (+15.1 s) | shell `true`, desktop instance `true`; the shell's pid unchanged (`shell_relaunched` false) |
| 08:52:00.229 | the desktop shell's journal | `idle-monitor: idle`; `idle-cycle-start: screensaver=150 lock=300`; its `screensaver` process started and exited 0 |

Ending the compositor did to the desktop what its start did at 07:26Z: the idle monitor read activity once. The
shell did not lose its connection.

## Left

- Not removed (a tmpfs, gone at a reboot): `$XDG_RUNTIME_DIR/vcomp/` (four instance dirs, a `dconf/` dir), the
  state dir `target/e2e-home/viola-comp-20261008T072612Z/`, and this chunk's scratch dirs under
  `target/e2e-home/` (listed in the report).
- No process of the chunk: the own compositor and the four window classes' terminal processes are read gone by
  pid and start time.
