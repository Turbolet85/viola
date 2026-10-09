# The rehearsal of the headless rig under the fake agent, on the changed build (plan.md step 9)

**Held.** The session came up, all six texts were confirmed with the rule equality true, no CR in any prompt and
no `wheel` record, and the stop left no process. So stop rule S3 did not fire. No live `claude` was started by this
rehearsal. Times are UTC, 2026-10-09, read from the clock.

The rig: `evidence/live-pty.py` opens a plain pty itself (210 columns by 45 rows), starts the command on it as the
session leader, counts and discards what the master yields, and writes nothing to the master until its stop. No
compositor was started, no window opened, no desktop key typed. The command was the product build
(`target/release-check/release/viola`, sha256 prefix `b4659b98029d0f94`, the build the block's first run made from
the changed tree) wrapping the harness-built fake agent (`target/harness/debug/viola-fake-agent`) in the stamped
home, instance `icrrehearse`. The fake agent was started as the last chunk's rehearsal started it, plus
`--turn-stop`, so each turn ends: `--cli-version 2.1.287 --fixtures <root>/fixtures/claude --screens
--trusted-root <root> --turn-stop`. Each send went through `evidence/live-drive.py` with `VIOLA_NAME`, `VIOLA_DIR`
and `VIOLA_BIN` removed and `--home` given, and was read by `evidence/live-read.py reading … --append
evidence/rehearsal-readings.ndjson` (codes, counts, times and equalities only).

## The session

| reading | what was read | holds |
|---|---|---|
| it comes up | started 19:03:12.198; the first three records are `wheel` (cause `start`), `budget-gate` (both 19:03:12.218) and `session-start` (cause `startup`, 19:03:12.221); the snapshot is live: `cli_version` 2.1.287, `cli_verified` true, `wheel` `driver`, an endpoint, the wrapper and the child alive by pid | yes |
| it holds 5 s | 5.0 s; 0 new records, 1 `wheel` record in total (the start's); both processes alive at every 500 ms read | yes |
| the stop ends everything | the stop began 19:03:35.328: one Ctrl-C through the pty; the wrapper exited 0 at 19:03:35.329, not killed; the wrapper, the child and the rig's host gone; 0 processes left in the pty's session or the child's; 0 bytes written to the master before the stop; 637 bytes drained and discarded | yes |

## The six sends, in the live order

Each was sent after the turn before it had ended (`turn-ended` read before the next send).

| id | sent (bytes, CR, LF) | typed (`text_bytes`) | exit | the records after the send | the prompt | rule equality | `wheel` records |
|---|---|---|---|---|---|---|---|
| `rule-inner-lf` | 69, 0, 1 | 69 | 0, read back | `send-issued`, `prompt-submitted`, `send-confirmed`, `turn-ended` | origin `driver`, 69 bytes, 0 CR, 1 LF; equal to the sent text unchanged | true | 0 |
| `rule-inner-crlf` | 70, 1, 1 | 69 | 0, read back | the same four | origin `driver`, 69 bytes, 0 CR, 1 LF | true | 0 |
| `rule-inner-cr` | 71, 1, 0 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 1 LF | true | 0 |
| `rule-inner-cr-cr` | 71, 2, 0 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 2 LF | true | 0 |
| `rule-inner-lf-cr` | 71, 1, 1 | 71 | 0, read back | the same four | origin `driver`, 71 bytes, 0 CR, 2 LF | true | 0 |
| `rule-crlf-lines` | 82, 2, 2 | 80 | 0, read back | the same four | origin `driver`, 80 bytes, 0 CR, 2 LF | true | 0 |

After the six the snapshot's `wheel` was still `driver`. The instance's log holds 6 `send-issued`, 6
`prompt-submitted`, 6 `send-confirmed` and 6 `turn-ended` records between the three start records and the stop's
own `wheel` record (cause `human-input`, 19:03:35.328: the stop's Ctrl-C is a key from the wrapper's terminal).

The block's rehearsal reader (the `jq` entry over `rehearsal-readings.ndjson`) reads these six lines.

## The census after it (19:03:49Z)

By executable and by command line, over the host's process list: 0 rig hosts, 0 processes of the product build, 0
of the harness's fake agent, 0 of `claude` 2.1.287 by path. The `plans/` directory beside the home is empty.

The ledger's `rehearsal` row (`live-sessions.ndjson`, line 1, written 19:04:03Z) holds the same readings and
`held` true.

## What the rehearsal does and does not show

It is the reader's second known verdict and the rig's proof on the changed build: under the fake agent the prompt
is the typed bytes, so the rule equality reads true here because the product now types the LF. It says nothing
about the real CLI: the fake agent submits what it is typed, whatever that is. The live readings are in
`live-run.md`.

The equality's other side under the fake agent, false on the build before the change, was not taken as a session:
the rebuilt binary had already replaced the earlier one when the rehearsal ran. Its false side is read by
`live-read.py selftest` (eight literal rows that must read false, a prompt that still holds the CR among them)
and, across processes, by the red side of `red-green.md`, where the fake agent receipted the CR on the present
rule.
