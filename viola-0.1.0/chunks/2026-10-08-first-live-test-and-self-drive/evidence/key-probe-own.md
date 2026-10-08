# Key probe in the chunk's own compositor — step 0 (2026-10-08, 07:26Z)

**Outcome: passed, before any live `claude` start (0 of 8).** A key typed with `wtype` into the probe window on
the chunk's own compositor moved the wheel to the human, and the next driver `send` was refused.

The probe ran inside the ONE start of the own compositor (`evidence/live-compositor.sh start`, 07:26:12Z; the
`compositor` line of `evidence/live-sessions.ndjson`). It is `evidence/key-probe-own.sh 8`: the harness `viola`
(`target/harness/debug/viola`) over the fake agent, in a new home under `target/e2e-home/`. Every `hyprctl` call
went through `live-compositor.sh hc`, which names the own instance; the desktop instance was asked one thing
only, `locked`, by `lock-read`.

## Before the start, and right after it

| when (UTC) | reading | result |
|---|---|---|
| 07:26:12.3 | `lock-read pre`: the shell's answer, the desktop instance's `locked` | `true`, `true` |
| 07:26:12.4 | `start` | own instance up in 0.2 s; its signature is not the desktop's; control socket and Wayland socket in the private runtime dir; one output `WAYLAND-1` 1261x688 at scale 1; own instance `locked` false; 0 windows |
| 07:26:14.5 | `lock-read after-start`, first reading (+2.1 s) | shell `true`, desktop instance `true` |
| 07:26:27.5 | `lock-read after-start`, second reading (+15.1 s) | shell `true`, desktop instance `true`; the shell's pid is the one read before the start (`shell_relaunched` false) |

## The terminal size

| font size | `size` receipt (columns by rows) | floor 90 by 30 |
|---|---|---|
| 8 | 210 by 45 | reached at the first size tried |

The forecast was about 97 by 49 on an output of 621x688. The output the locked desktop gave this start is
1261x688, so the columns are about twice the forecast; the rows read 45. Font size 8 is the font size of every
later window. Sizes 6 and 4 were not tried.

## The reading, in order

| when (UTC) | reading | result |
|---|---|---|
| 07:26:34.8 | the window, class `viola.keyprobe`, font size 8 | listed on the own instance, `mapped` true, 1261x688 |
| 07:26:34.8 | the three start records | `wheel` (cause `start`), `budget-gate`, `session-start` |
| 07:26:36.9 | focus, 1.5 s after the window was read | it holds focus by itself: the own instance's active address equals its address |
| 07:26:36.9 | `wheel` records before and after the focus | 1 and 1, delta 0: the focus wrote no `wheel` record |
| 07:26:36.9 | `key viola.keyprobe x` | the guard passed immediately before the key (own compositor alive by pid and start time, own instance not locked, one window, of that class, active address equal); `wtype x` on the own socket, exit 0 |
| 07:26:36.9 | the `wheel` records after the key | one new record: holder `human`, cause `human-input` (its `ts` 07:26:36.926Z) |
| 07:26:36.9 | the fake agent's receipt | one `key` line, hex `78` |
| 07:26:36.96 | a driver `send` of `probe` to the probe instance | exit 10; stderr `[/ ] unable  keyprobe  human-typing`, then `hint: the human has the wheel; send again after the human hands it back`; the log holds one `send-refused` (`human-typing`) and 0 `send-issued` records |
| 07:26:37.1 | `close viola.keyprobe` | the window's own foot process ended by pid and read gone; the wrapper read gone; 0 windows on the own instance |

The own compositor stays up for the round and the live sessions.

## Left

- The probe home `target/e2e-home/viola-keyprobe-20261008T072634Z/` (a tmpfs behind the link, gone at a reboot).
- No process and no window of the probe (read by pid and by `hc clients`).
