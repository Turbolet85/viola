# The live work (2026-10-09): one rehearsal and four starts of the cap of five

**Four live starts were made, of the founder's cap of five (`inputs#I1`), each on `claude` 2.1.287 by path, each
ledgered before it was made, each headless.** No compositor was started, no window was opened and no desktop key
was typed (`inputs#I4`). Every session was stopped through the rig and left no process. Times are UTC. The fourth
start is the plan's revision (`inputs#I7`), made in the re-entry (`inputs#I8`); no fifth was made.

No prompt text, assistant text or event payload text is in this file or in the two record files: a reading of text
is a count, a code or an equality. The sent texts stand in a private directory outside the tree.

- The ledger: `live-sessions.ndjson` (11 rows: the rehearsal, four `start` rows, a `census` row after each start,
  the final `census` row written after start 3, and the new last one written after start 4; the ledger is
  append-only, so the earlier final row stays). `live-ledger.py` is its one writer; each row's `written_at` is the
  clock's.
- The readings: `live-readings.ndjson` (eight `reading` lines and two `equality` lines). The `reading` lines are
  written by `live-read.py`; each `equality` line was computed beside its reading and appended through a
  parse-and-re-serialise step.
- The rig: `live-pty.py` (the pty host), `live-start.sh` and `live-drive.py` (both byte-for-byte the 2026-10-08
  chunk's). The product verbs ran with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given.
- The home: `target/e2e-home/viola-live-4043089/home`, stamped for 2.1.287 on 2026-10-08, 17 rows `pass`
  (`live-preconditions.md`).

## The sessions

| row | ledgered | started | build (sha256 prefix) | came up | stop | left |
|---|---|---|---|---|---|---|
| rehearsal, fake agent, instance `rehearse` | 15:52:25 (after it, its four readings in the row) | 15:50:33.935 | `2bf1b8ab19e16c8e` | `wheel` (start), `budget-gate`, `session-start` (startup) within 0.06 s; `cli_verified` true | one Ctrl-C through the pty, exit 0, not killed | 0 processes |
| start 1, instance `e3crlf` | 15:52:25 | 15:52:33.042 | `2bf1b8ab19e16c8e` | the same three records, `session-start` 1.33 s after the start; no modal; 0 `wheel` records of cause `human-input` before the send | three Ctrl-C presses, `session-end` recorded, exit 0, not killed | 0 processes |
| start 2, instance `e3crcr` | 15:53:22 | 15:53:30.018 | `2bf1b8ab19e16c8e` | the same, `session-start` 1.40 s after the start; no modal; 0 before the send | three presses, `session-end`, exit 0, not killed | 0 processes |
| start 3, instance `e3after` | 16:17:25 | 16:17:33.178 | `62bf6028d95fe34e` | the same, `session-start` 1.18 s after the start; no modal; 0 before the first send | three presses, `session-end`, exit 0, not killed | 0 processes |
| start 4, instance `e3innercr` | 16:46:29 | 16:46:33.109 | `62bf6028d95fe34e` | the same, `session-start` 0.59 s after the start; no modal; 0 before the send | three presses, `session-end`, exit 0, not killed | 0 processes |

The rows of starts 1 to 3 were each written 8 s before the start, and start 4's 4 s before it. The rehearsal's
four readings are in `rehearsal.md`. Each live session held the wheel with the driver for 5 s after its start
records, with no new record, before its first send.

`claude` 2.1.287 under `viola run` on a plain pty, with no terminal emulator behind it, came up four times of
four: the input box was reached, the snapshot read `cli_version` 2.1.287 and `cli_verified` true, and no reply of
a terminal took the wheel. Start 1 was the first reading of the real CLI on such a host.

## Starts 1 and 2: the build before the change (plan.md step 2)

One reading a start: a prompt the wrapper does not claim hands the wheel to the human.

| reading | sent | typed (`text_bytes`) | exit | the records after the send | the prompt the CLI submitted | wheel |
|---|---|---|---|---|---|---|
| `trailing-crlf` (start 1, 15:52:45.728) | 33 bytes, ending CR LF | 32 (the LF dropped, the CR typed) | 13, `not-delivered` / `no-prompt-submitted`, one hint line | `send-issued`, `prompt-submitted`, `wheel` (human-input), `turn-ended`, `send-refused` (10 s after `send-issued`) | 31 bytes, origin `human`; equal to the sent text without its whole CR LF ending; holds no CR and no LF | to the human, 78 ms after `send-issued` |
| `trailing-cr-cr` (start 2, 15:53:41.376) | 33 bytes, ending CR CR | 33 | 13, the same pair, one hint line | the same five | 31 bytes, origin `human`; equal to the sent text without both CRs | to the human, 96 ms after `send-issued` |

Both read as the one-CR reading of 2026-10-08 did: the CLI submits the text without its CR and LF ending, whatever
that ending's length, so the exact match on the typed text fails, the prompt is filed `human`, and the send that was
delivered and answered is reported not delivered. These two endings are the labelled cases of `typed_text`'s table
that name this chunk.

## Start 3: the build after the change (plan.md step 13)

One session, five sends in the plan's order.

| reading | sent | typed (`text_bytes`) | exit | the records after the send | the prompt | wheel records |
|---|---|---|---|---|---|---|
| `after-cr` (16:17:45.426) | 32 bytes, ending CR | 31 | 0, read back | `send-issued`, `prompt-submitted`, `send-confirmed`, `turn-ended` | 31 bytes, origin `driver`, equal to the sent text without its ending | 0 |
| `after-crlf` (16:17:53.054) | 33 bytes, ending CR LF | 31 | 0, read back | the same four | 31 bytes, origin `driver`, equal to the sent text without its ending | 0 |
| `after-cr-cr` (16:17:59.429) | 33 bytes, ending CR CR | 31 | 0, read back | the same four | 31 bytes, origin `driver`, equal to the sent text without its ending | 0 |
| `after-empty-text` (16:18:08.015) | 2 bytes, CR LF | none | 13, `not-delivered` / `empty-text`, refused by the client in 1 ms | none: 0 records, no `send-issued`; the client's own `send-refused` line, `side` `client` | none | 0 |
| `after-inner-crlf` (16:18:16.476) | 32 bytes, one CR LF inside, no newline at the end | 32 | 13, `not-delivered` / `no-prompt-submitted`, one hint line | `send-issued`, `prompt-submitted`, `wheel` (human-input), `turn-ended`, `send-refused` | 31 bytes, origin `human`; 0 CR and 1 LF | 1, to the human |

The three predictions hold as forecast: each ending is typed without it, confirmed, filed `driver`, with no
`wheel` record and the snapshot's `wheel` still `driver`. The stop rule on them did not fire.

`after-empty-text` holds as forecast: stderr is two lines, the struck mirror line with `not-delivered` and
`empty-text`, then one `hint:` line last; no ESC byte; nothing reached the wrapper.

### A finding: an inner CRLF is not submitted unchanged

`after-inner-crlf` was the reading of unknown outcome, taken last and recorded as read. The CLI submits a pasted
text's inner CR LF as one LF: the submitted prompt equals the sent text with its CR LF read as one LF (the
`equality` line: `prompt_equals_sent_with_each_crlf_as_one_lf` true, `prompt_equals_sent_unchanged` false). The
strip leaves an inner CR in place by the ruling, so the typed text still holds it, the exact match fails, the prompt
is filed `human`, the wheel goes to the human 24 ms after `send-issued`, and the send ends `not-delivered` /
`no-prompt-submitted`, exit 13. The text was delivered and its turn ended.

So a driver text that holds a CR LF inside it, as a text written on Windows does on every line, is today
delivered, answered, reported not delivered, and takes the wheel. It is the same shape the trailing CR had before
this chunk. This chunk does not change it: the ruling of 2026-10-09 covers the ending, the plan forbids stripping
an inner newline, and a tolerant match is a rejected approach. It is reported for the founder; not measured in
start 3 was an inner CR standing alone, with no LF after it (start 4, below, reads it).

It was the session's last send, so no later reading was taken under the human's wheel. No `release` and no `pause`
was run by anyone.

## Start 4: the lone inner CR on the build after the change (plan.md step 16)

One session and one send, on the build start 3 ran on (`live-preconditions.md`, "Before start 4"). The reading
carried no prediction and is recorded as read. No product file and no test file changed with it.

| reading | sent | typed (`text_bytes`) | exit | the records after the send | the prompt | wheel records |
|---|---|---|---|---|---|---|
| `after-inner-cr` (16:46:49.521) | 31 bytes, one CR inside, no LF anywhere, no newline at the end | 31 | 13, `not-delivered` / `no-prompt-submitted`, one hint line | `send-issued`, `wheel` (human-input), `prompt-submitted`, `turn-ended`, `send-refused` (10 s after `send-issued`) | 31 bytes, origin `human`; 0 CR and 1 LF | 1, to the human |

### The reading: a lone inner CR is submitted as one LF

The CLI submits a pasted text's lone inner CR as one LF. The `equality` line: `sent_cr_count` 1, `sent_lf_count`
0, `prompt_cr_count` 0, `prompt_lf_count` 1; `prompt_equals_sent_with_its_cr_as_one_lf` true,
`prompt_equals_sent_unchanged` false, `prompt_equals_sent_without_its_cr` false. One `prompt-submitted` record
followed the send, and no second one.

Set beside `after-inner-crlf`:

| | `after-inner-crlf` (start 3) | `after-inner-cr` (start 4) |
|---|---|---|
| sent, inside the text | one CR LF (32 bytes) | one CR (31 bytes) |
| the submitted prompt | 31 bytes, 0 CR, 1 LF: the CR LF as one LF | 31 bytes, 0 CR, 1 LF: the CR as one LF |
| equal to the sent text unchanged | no | no |
| `prompt-submitted` origin | `human` | `human` |
| the wheel | to the human, 24 ms after `send-issued` | to the human, 37 ms after `send-issued` |
| the send | exit 13, `not-delivered` / `no-prompt-submitted` | exit 13, `not-delivered` / `no-prompt-submitted` |
| the turn | ended | ended, 1.4 s after `send-issued` |
| record order after `send-issued` | `prompt-submitted`, then `wheel` | `wheel`, then `prompt-submitted` (both stamped the same millisecond) |

So both shapes of an inner CR read the same way on 2.1.287: the CLI turns it into one LF, the typed text still
holds the CR, the exact match fails, the prompt is filed `human`, the wheel goes to the human, and a send that was
delivered and answered is reported not delivered. Neither reading changes what the product does: the ruling of
2026-10-09 covers the ending, the plan forbids stripping an inner newline, a tolerant match is a rejected approach,
no test pins either reading and the fake agent gains no shape from them. Both are reported for the founder.

Not measured: several inner CRs, an inner CR beside an inner LF in the other order (LF CR), and any of these on a
CLI version other than 2.1.287.

It was the session's only send. No `release` and no `pause` was run by anyone; the session was stopped through the
rig with the wheel at the human.

## The census

After each start: the wrapper and the child gone by pid, the rig's host gone, 0 processes left in the pty's
session or the child's, 0 `claude` 2.1.287 processes on the host, 0 bytes written to the master before the stop,
and the `plans/` directory beside the home still empty.

The reading of 16:19:18Z, the ledger's final row after start 3: 0 rig hosts, 0 processes of the product build, 0
of the harness's fake agent, 0 of `claude` 2.1.287. One compositor process stands on the host, the desktop's own,
started 2026-10-04; this chunk started none. Four instance directories were added to the stamped home (`rehearse`,
`e3crlf`, `e3crcr`, `e3after`); the home stands on the tmpfs and was not removed.

The last reading, 16:47:27Z, the ledger's new last row after start 4: the same zeros, the same one compositor
process, none started by this chunk, no window opened and no desktop key typed. A fifth instance directory,
`e3innercr`, was added to the stamped home, which still stands on the tmpfs.

Each live session ran one or more short turns on the user's own subscription: start 1 one, start 2 one, start 3
four (the three confirmed sends and the inner-CRLF text), start 4 one. The CLI kept its own transcripts of them.
