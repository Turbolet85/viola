# The live session lost the wheel at its start, and which terminal reply is read as typing (2026-10-08)

**Outcome: the live run stopped at its first `send`.** Start 6 came up verified, and 237 ms after the child
started the log shows a `wheel` record with holder `human` and cause `human-input`. No key was typed by anyone.
The planned first `send` was refused `human-typing`, exit 10. The operator ruled the stop (inputs#I15).

## The live session, start 6

The product build (`target/release-check/release/viola`) over `claude` 2.1.287 by path, in the home the round
stamped, in a foot window (class `viola.livetest`, font size 8) on the chunk's own compositor. Records:
`evidence/live-run-start6.ndjson`.

| when (UTC) | record | reading |
|---|---|---|
| 07:28:40.939 | `wheel` | holder `driver`, cause `start` |
| 07:28:40.939 | `budget-gate` | not paused |
| 07:28:40.939 | the `claude-child` start line | `pty_backend` `openpty`, `cli_version` 2.1.287, `cli_verified` true |
| 07:28:41.176 | `wheel` | holder `human`, cause `human-input`: 237 ms after the child's start line |
| 07:28:41.536 | `session-start` | cause `startup`, from the hook: 360 ms after the wheel moved |
| 07:28:41 | the snapshot | `cli_version` 2.1.287, `cli_verified` true, `wheel` `human` |
| 07:30:29 | the child's terminal (`stty size` on its own pts) | 45 rows by 210 columns |
| 07:30:29.540 | `send` of the skill line | exit 10; stderr `[/ ] unable  livetest  human-typing`, then `hint: the human has the wheel; send again after the human hands it back`; one `send-refused`, no `send-issued`; nothing typed |
| 07:32:36.906 | `session-end` (the close) | from the hook, its decision line `channel-unreachable`; the wrapper read gone 0.14 s after the TERM to the window's foot process, the child 0.52 s after it |

No `key` call was made between the start and the close: the only `key` call of the chunk is step 0's, into the
probe window, which was closed at 07:26:37. The desktop was locked throughout. No modal was seen on the records
(`session-start` landed).

So the driver could not send: every `send` is refused while the human holds the wheel, and the wheel's return is
the human's `release`, which nobody runs in this chunk (plan.md, Constraints).

## Why: the terminal answers the CLI on stdin

`viola run` reads its host stdin through the observer (`src/run/wheel.rs:306-313`). Every byte is typing except
the sequences on the closed list (`:473-474`): focus reports, mouse reports, DA1, DA2, CPR, DECRPM, the kitty
flags reply, and an OSC or DCS string of at most 64 payload bytes. A real terminal answers a program's queries
on that same stdin. Under the fake agent no query is sent, so step 0 saw none of it.

## The measurement (inputs#I15): one query per window, no live start, no key

`evidence/reply-probe.sh` with `evidence/reply-probe-child.py`, 07:33:44Z to 07:34:47Z, in the same start of
the own compositor. For each query: one foot window (class `viola.replyprobe`), the product build wrapping the
child script, which sends ONE query, records what the terminal answered, and exits. The instance's log then
shows whether a `wheel` record of cause `human-input` followed. 23 windows; nothing calls `key`. Rows:
`evidence/reply-probe.ndjson`.

Controls: with no query sent, no `wheel` record follows (the window, its focus and the child's raw mode move
nothing). With DA1, which is on the list, the reply arrives and no `wheel` record follows.

| query | foot 1.28.0 answered | read as typing |
|---|---|---|
| (none) | nothing | no |
| DA1 `CSI c` | `CSI ? 62;4;22;28;52 c` | no |
| DA2 `CSI > c` | `CSI > 1;012800;0 c` | no |
| XTVERSION `CSI > 0 q` | `DCS > | foot(1.28.0) ST` | no |
| kitty flags `CSI ? u` | `CSI ? 0 u` | no |
| DECRQM 2026 | `CSI ? 2026;2 $ y` | no |
| CPR `CSI 6 n` | `CSI 1;1 R` | no |
| **DSR `CSI 5 n`** | `CSI 0 n` | **yes** |
| OSC 10, OSC 11, OSC 4 colour queries | `OSC n ; rgb:… ST` (25 or 26 bytes) | no |
| **colour-scheme query `CSI ? 996 n`** | `CSI ? 997;1 n` | **yes** |
| mode 2031 set (theme updates) | nothing at once | no |
| **window size in pixels `CSI 14 t`** | `CSI 4;675;1260 t` | **yes** |
| **cell size `CSI 16 t`** | `CSI 6;15;6 t` | **yes** |
| **text area in cells `CSI 18 t`** | `CSI 8;45;210 t` | **yes** |
| **in-band resize, mode 2048 set** | `CSI 48;45;210;675;1260 t` | **yes** |
| **modifyOtherKeys query `CSI ? 4 m`** | `CSI > 4;1 m` | **yes** |
| XTGETTCAP `TN`, `RGB` | `DCS 1 + r … ST` | no |
| DECRQSS cursor style | `DCS 1 $ r 2 SP q ST` | no |
| focus reporting, mode 1004 set | `CSI I` | no |
| kitty graphics query | nothing | no |

Seven answers of this terminal are read as typing. Each has a CSI final byte that the closed list does not
name: `n` (other than nothing: the list has no `n` form), `t`, and `m` after `>`.

## What is not measured

- **Which query the live CLI sends at its start.** No live start was made for it. The seven shapes above are
  what this terminal would answer if asked; the one (or more) that 2.1.287 asks for 237 ms after its start is
  not known. A read of the CLI's binary for the literal query strings is not conclusive: it holds `?997;` (the
  colour-scheme reply's parameter prefix) on 4 lines and none of the seven query literals, which a program that
  builds its sequences from numbers would also show.
- Whether the CLI asks again later in a session (at a resize, a focus change, a theme change). An in-band
  resize report and a colour-scheme report are sent by the terminal on its own once their mode is set.
- Any other terminal. The list was measured on the Windows terminal path; foot is the first real terminal a
  live CLI ran in under the product on Linux.

## What follows

- The fix is in `src/run/wheel.rs`, outside this chunk's files and under its preservation guard: on the
  operator's word (inputs#I15) it is a plan revision through the phase door, in this chunk, with its own
  red-green case, and the live run is retried on the same compositor with the 2 live starts left (6 of 8 spent).
- The closed list is F-W2's, a founder ruling: which replies join it is his word to give or to confirm.
- A retry that names only some of the shapes risks start 7 on the one it did not name. The CLI's own start-up
  input can be read at start 7 only if the revision plans a record for it.
