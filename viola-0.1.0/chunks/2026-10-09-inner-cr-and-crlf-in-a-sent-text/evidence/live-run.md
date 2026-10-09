# The live work (2026-10-09): one rehearsal and one start of the cap of two

**The control held: on `claude` 2.1.287 an LF typed inside a bracketed paste is submitted unchanged.** The send of
`rule-inner-lf` (one LF inside, no CR) was confirmed, its prompt filed `driver` and equal to the sent text, with no
`wheel` record. Every later reading of this session rests on that shape.

**One live start was made, of the founder's cap of two (`inputs#I1`, `inputs#I3`), on `claude` 2.1.287 by path,
ledgered before it was made, headless.** No compositor was started, no window was opened and no desktop key was
typed. The session was stopped through the rig and left no process. The spare start was not made: start 1 took all
six readings. Times are UTC, read from the clock.

No prompt text, assistant text or event payload text is in this file or in the record files: a reading of text is
a count, a code or an equality. The sent texts stand in the private directory outside the tree
(`live-preconditions.md`).

- The ledger: `live-sessions.ndjson` (4 rows: the rehearsal, the `start` row, the `census` row after the start and
  the final `census` row). `live-ledger.py` is its one writer; each row's `written_at` is the clock's.
- The readings: `live-readings.ndjson` (six `reading` lines, written by `live-read.py`).
- The rig: `live-pty.py`, `live-start.sh`, `live-drive.py`, `live-ledger.py` (byte-for-byte the last chunk's) and
  `live-read.py` (extended, `live-preconditions.md`). The product verbs ran with `VIOLA_NAME`, `VIOLA_DIR` and
  `VIOLA_BIN` removed and `--home` given.
- The home: `target/e2e-home/viola-live-4043089/home`, stamped for 2.1.287 on 2026-10-08, 17 rows `pass`.
- The build: `target/release-check/release/viola`, sha256 prefix `b4659b98029d0f94`, made from the changed tree
  by the block's first run and read again right before the start's ledger row. The block's later rebuild does
  not carry this hash; what differs and what does not is in `live-preconditions.md`, its last section.

## The sessions

| row | ledgered | started | build (sha256 prefix) | came up | stop | left |
|---|---|---|---|---|---|---|
| rehearsal, fake agent, instance `icrrehearse` | 19:04:03 (after it, its readings in the row) | 19:03:12.198 | `b4659b98029d0f94` | `wheel` (start), `budget-gate`, `session-start` (startup) within 0.03 s; `cli_verified` true | one Ctrl-C through the pty, exit 0, not killed | 0 processes |
| start 1, instance `icrlive` | 19:04:12 | 19:04:21.269 | `b4659b98029d0f94` | the same three records, `session-start` 0.96 s after the start; `cli_version` 2.1.287, `cli_verified` true; no modal; 0 `wheel` records of cause `human-input` before the first send | three Ctrl-C presses from 19:05:21.828, `session-end` recorded at 19:05:22.607, exit 0, not killed | 0 processes |

The start's row was written 9 s before the start. The session held the wheel with the driver for 5.0 s after its
start records, with no new record, before its first send. Stop rule S4 did not fire.

## Start 1: six sends on the build after the change (plan.md step 10)

One session, the plan's order, each text synthetic and under 100 bytes with no newline ending, each sent only
after the turn before it had ended.

| # | reading (sent at) | sent (bytes, CR, LF) | typed (`text_bytes`) | exit | the records after the send | the prompt | rule equality | `wheel` records |
|---|---|---|---|---|---|---|---|---|
| 1 | `rule-inner-lf` (19:04:34.559) | 69, 0, 1 | 69 | 0, read back | `send-issued`, `prompt-submitted`, `send-confirmed`, `turn-ended` | origin `driver`, 69 bytes, 0 CR, 1 LF; equal to the sent text unchanged | true | 0 |
| 2 | `rule-inner-crlf` (19:04:43.158) | 70, 1, 1 | 69 | 0, read back | the same four | origin `driver`, 69 bytes, 0 CR, 1 LF | true | 0 |
| 3 | `rule-inner-cr` (19:04:49.416) | 71, 1, 0 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 1 LF | true | 0 |
| 4 | `rule-inner-cr-cr` (19:04:56.858) | 71, 2, 0 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 2 LF | true | 0 |
| 5 | `rule-inner-lf-cr` (19:05:03.180) | 71, 1, 1 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 2 LF | true | 0 |
| 6 | `rule-crlf-lines` (19:05:09.380) | 82, 2, 2 | 80 | 0, read back | the same four | origin `driver`, 80 bytes, 0 CR, 2 LF | true | 0 |

After each send the snapshot's `wheel` read `driver`. The wrapper's own lines agree: six `send-issued` lines with
`text_bytes` 69, 69, 71, 71, 71 and 80, and six `send-confirmed` lines with `confirmed` true, each on its send's
`corr`. The `prompt-submitted` record followed `send-issued` by 42, 23, 20, 34, 22 and 32 ms, and each turn ended
0.7 to 2.0 s after it.

### The readings beside their predictions

| reading | predicted | read | holds |
|---|---|---|---|
| `rule-inner-lf`, the control (`inputs#I2`) | confirmed, filed `driver`, no `wheel` record | confirmed, filed `driver`, 0 `wheel` records, the prompt equal to the sent text | yes |
| `rule-inner-crlf` (before the change: `after-inner-crlf`, exit 13, the wheel to the human) | confirmed, filed `driver`, no CR, the rule equality true | as predicted | yes |
| `rule-inner-cr` (before the change: `after-inner-cr`, exit 13, the wheel to the human) | the same | as predicted | yes |
| `rule-inner-cr-cr` (not measured before) | the same | as predicted: the two LFs it was typed with came back as two LFs | yes |
| `rule-inner-lf-cr` (not measured before) | the same | as predicted: two LFs came back | yes |
| `rule-crlf-lines` (not measured before) | the same | as predicted: three lines joined by one LF each | yes |

No prediction was contradicted, so there is no finding to report against the rule and stop rule S5 did not fire.
The block's live readings reader grades the first three ids; the three ids not measured before are recorded here
as read and are not graded by an entry.

What the six readings show together, on 2.1.287: with the rule no CR reaches the CLI, the CLI submits each LF
typed inside a paste as that LF, one LF or two in a row, and the prompt it submits equals the typed text, so the
exact match holds and the driver keeps the wheel.

The session's last record before the stop is the sixth `turn-ended` (19:05:10.092). No `release` and no `pause` was
run by anyone. The stop's own Ctrl-C left one `wheel` record of cause `human-input` (19:05:21.828), after every
reading.

## What was not measured

- any CLI version but 2.1.287;
- a long or wrapped multi-line text (every text was under 100 bytes and of at most three lines; a long paste
  starts the CLI's paste hint);
- an LF typed inside a paste on Windows (the dev host is Linux);
- what the CLI does with a typed CR CR or LF CR: with the rule no `send` types a CR, so neither was typed and
  neither is leaned on.

The shape the control read has no ledger row and no `viola verify` probe; the row stays owed to `v1-34`
(`inputs#I2`).

## The census

After the start (19:05:23Z): the wrapper and the child gone by pid, the rig's host gone, 0 processes left in the
pty's session or the child's, 0 bytes written to the master before the stop, 43 924 bytes drained and discarded,
and the `plans/` directory beside the home still empty. By executable and by command line over the host's process
list: 0 rig hosts, 0 processes of the product build, 0 of the harness's fake agent, 0 of `claude` 2.1.287 by path.

One desktop session process stands on the host, the desktop's own, started 2026-10-04; this chunk started none,
opened no window and typed no desktop key. Two instance directories were added to the stamped home
(`icrrehearse`, `icrlive`); the home stands on the tmpfs and was not removed.

The live session ran six short turns on the user's own subscription, one a send. The CLI kept its own transcript
of them.
