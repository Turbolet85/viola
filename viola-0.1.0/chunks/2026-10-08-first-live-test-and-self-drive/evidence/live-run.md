# The live run (2026-10-08): start 7, completed

**Completed.** Start 7, the product build (`target/release-check/release/viola`, sha256 `2bf1b8ab19e16c8e`)
over `claude` 2.1.287 by path, in the home the round stamped, in a foot window (class `viola.livetest`, font
size 8, 210 columns by 45 rows) on the chunk's own compositor. The session kept the wheel with the driver from
its start to the takeover key. Records: `evidence/live-run.ndjson` (17 step lines); the driver is
`evidence/live-drive.py`, its verbs run by the implementing session with the text on stdin.

No prompt text, assistant text or event payload text is in this file: a reading of text is a count or a boolean.

## The start

| when (UTC) | record | reading |
|---|---|---|
| 08:39:33 | the desktop lock, read right before the start | shell `true`, desktop instance `true` |
| 08:39:33 | the own compositor | alive by pid and start time, own instance not locked, 0 windows |
| 08:39:33.569 | `wheel` | holder `driver`, cause `start` |
| 08:39:33.569 | `budget-gate` | |
| 08:39:34.427 | `session-start` | cause `startup` |
| 08:39:39 | the snapshot, 5 s after `session-start` | `cli_version` 2.1.287, `cli_verified` true, `wheel` `driver`; 0 `wheel` records with holder `human` |

Start 6 had a `wheel` record of cause `human-input` between `budget-gate` and `session-start`, 237 ms after its
child started (`evidence/terminal-replies.md`). Start 7, on the build with the closed list extended, has none. No
modal: `session-start` landed 0.86 s after the start.

## The steps

| step | verb | exit | what the records hold | time (UTC) |
|---|---|---|---|---|
| `skill-sent` | `send` of the skill line | 0 | `[RB] read back`, cursor 454; `send-issued`, `prompt-submitted`, `send-confirmed` | 08:39:51.6 |
| `permission-1` | `answer` by id 1 | 0 | a `permission` the builder raised by itself (tool `Bash`, one of the skill's own scripts): `allow`; decision emitted, deadline not hit, 3 ms after it was raised | 08:39:55.0 |
| `turn-ended` | `wait --after 454` | 0 | woke on `turn-ended`, cursor 6879, 124 s after the send | 08:41:55.4 |
| `last-read` | `last` | 0 | non-empty, 2198 bytes, 324 words, holds the skill's closing question | 08:42:08.0 |
| `review-answered` | `send` of the declining reply | 0 | read back; `wait` woke on `turn-ended`; `last` is the one word | 08:42:08.1 |
| `clear` | `send` of `/clear` | 0 | read back; `session-end`, then `session-start` with cause `clear` and a new session id; 0 `prompt-submitted` | 08:42:16.4 |
| `skill-2` | `send` of the skill line again | 0 | read back, cursor 8067; woke on `turn-ended` 203 s later | 08:42:23.6 |
| `permission-2` to `-4` | `answer` by ids 2, 3, 4 | 0 each | three `permission` dialogs the builder raised by itself (tool `Bash`, none a read or a skill script by the driver's rule): `deny` each; each decision emitted, no deadline hit | 08:42:28 to 08:42:42 |
| `skill-2-review` | `send` of the declining reply | 0 | read back; `turn-ended`; `last` is the one word | 08:45:46.3 |
| `dialog-question` | `send`, `wait`, `answer` by id 5, `wait` | 0, 0, 0, 0 | a `question`, asked as planned, raised 1.6 s after the send; decision emitted, deadline not hit; `turn-ended`; `last` is the chosen word; 0 `wheel` records, 0 keys | 08:45:53.5 |
| `dialog-permission` | `send`, `wait`, `answer` by id 6, `wait` | 0, 0, 0, 0 | a `permission` for tool `Bash`, the command `true`: `deny`; decision emitted, deadline not hit; `turn-ended`; 0 `wheel` records, 0 keys | 08:45:56.4 |
| (the three readings of step 8) | | | `evidence/live-readings.ndjson`; see below | 08:46:09 to 08:46:29 |
| `dialog-plan` | `send`, `wait`, `answer` by id 7, `wait` | 0, 0, 0, 0 | a `plan`, raised 10.4 s after the send: `approve`; decision emitted, deadline not hit; `turn-ended`; `last` is the one word; 0 `wheel` records, 0 keys | 08:46:37.4 |
| `takeover` | `guard`, `key viola.livetest x`, then `send` of `probe` | guard 0, key 0, send **10** | see below | 08:47:05.0 |
| `close` | `close viola.livetest` | 0 | `session-end` (from the hook, its decision line `channel-unreachable`); the wrapper gone 0.03 s and the child 0.36 s after the close call began; the `session-end` line landed before the child was gone | 08:47:28.5 |

All three dialog kinds `v1-31` names were raised by the live CLI on the driver's request and decided by their
ids. With the four permissions the builder raised by itself, seven dialogs were decided in the session: each
reads `dialog-raised`, `dialog-answered` and a `hook-decision` with `decision_emitted` true and `deadline_hit`
false on its own id. Nothing was typed into any dialog, and no `wheel` record stands between the start and the
takeover key.

## The takeover

| when (UTC) | reading | result |
|---|---|---|
| 08:47:04.9 | the last record | `turn-ended`: no turn running, no dialog pending |
| 08:47:04.940 | `guard viola.livetest` | ok: own compositor alive, own instance not locked, one window, of that class, active address equal |
| 08:47:04.9 | `wheel` records before and after the guard | 1 and 1: the window's focus wrote none (`wheel_after_focus` 0) |
| 08:47:04.989 | `key viola.livetest x` | the guard read again immediately before the key: ok; `wtype` on the own socket, exit 0, one character |
| 08:47:04.983 | the new `wheel` record (its own `ts`) | holder `human`, cause `human-input`; the snapshot's `wheel` reads `human` |
| 08:47:05.021 | a driver `send` of `probe` | exit 10; stderr is two lines: `[/ ] unable  livetest2  human-typing`, then one `hint:` line, last, which does not name `release`; one `send-refused`, no `send-issued` |

The wheel is left with the human. No `release` and no `pause` was run by anyone.

## The readings taken in this session (step 8)

| reading | exit | result |
|---|---|---|
| `send-under-hint` | 0 | a long paste (2564 bytes) confirmed; its turn ended; the short text (32 bytes) was sent 4.2 s after the paste was issued, inside the hint's 8 s. The gate held it: it was issued 8.3 s after the paste and confirmed, `send-confirmed.duration_ms` 4081 against the 8 500 ms bound. Its turn ended; no `wheel` record |
| `ends-in-newlines` | 0 | 33 bytes sent, two of them the trailing LF; `text_bytes` 31; the prompt filed `driver`; confirmed; 0 `send-refused`, 0 `wheel` records |
| `clear-then-newline` | 0 | `/clear` and one LF: `text_bytes` 6; confirmed by `session-end` and a `session-start` of cause `clear` with a new session id; 0 `prompt-submitted` |

The two readings of unknown outcome were taken in start 8 (`evidence/live-readings.ndjson`, `only-newlines` and
`trailing-cr`): a text of only newlines is refused `not-delivered` / `no-prompt-submitted` after 10 s, exit 13,
with the wheel unmoved; a text ending in one CR is delivered and answered, but the CLI submits it without the
CR, so the prompt is filed `human`, the wheel moves to the human 33 ms after the send was issued, and the send
ends `not-delivered` / `no-prompt-submitted`, exit 13.

## Not as the plan forecast

- The CLI's plan file did not land in the 0700 `plans/` beside the home: that directory is empty after the run,
  and one file of 584 bytes, written at 08:46:37Z and holding the test plan's wording (a boolean read), stands
  under the CLI's default plans directory in the user's home. The `plansDirectory` key of the launcher's
  settings object did not move it. viola wrote nothing there; the file is the CLI's own. It is not removed by
  this chunk.
- The live child's environment holds no `CLAUDE*` name and three `VIOLA*` names (`VIOLA_BIN`, `VIOLA_DIR`,
  `VIOLA_NAME`), names only.

## Start 6, earlier the same day

Start 6 lost the wheel at its start and its first `send` was refused; its records are
`evidence/live-run-start6.ndjson` and its cause is `evidence/terminal-replies.md`.
