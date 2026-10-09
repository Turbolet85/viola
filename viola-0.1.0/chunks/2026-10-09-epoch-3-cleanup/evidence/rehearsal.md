# The rehearsal of the headless rig under the fake agent (plan.md step 1; inputs#I4)

**Held.** All four readings hold, so stop rule S4 did not fire and the live starts were made. No live `claude` was
started by this rehearsal. Times are UTC, 2026-10-09.

The rig: `evidence/live-pty.py` opens a plain pty itself (210 columns by 45 rows), starts the command on it as the
session leader, counts and discards what the master yields, and writes nothing to the master until its stop. No
compositor was started, no window opened, no desktop key typed. The command was the product build
(`target/release-check/release/viola`, sha256 prefix `2bf1b8ab19e16c8e`) wrapping the harness-built fake agent
(`target/harness/debug/viola-fake-agent`) in the stamped home, instance `rehearse`, the fake agent started as the
harness's supervise starts it: `--cli-version 2.1.287 --fixtures <root>/fixtures/claude --screens --trusted-root
<root>`. The readings were taken by `evidence/live-read.py` (codes, counts and times only).

| # | reading | what was read | holds |
|---|---|---|---|
| 1 | the session comes up | started 15:50:33.935; the first three records are `wheel` (cause `start`), `budget-gate` (both 15:50:33.990) and `session-start` (cause `startup`, 15:50:33.995); the snapshot is live: `cli_version` 2.1.287, `cli_verified` true, `wheel` `driver`, an endpoint, the wrapper and the child alive by pid | yes |
| 2 | it holds for 60 s | 60.07 s; 0 new records, so no `wheel` record after the start's (1 in total); both processes alive at every 500 ms read | yes |
| 3 | one probe text is confirmed | `live-drive.py … send` of a 30-byte text with no ending: exit 0, `[RB] read back`, cursor 451; the records are `send-issued`, `prompt-submitted` (origin `driver`, 30 bytes, equal to the sent text), `send-confirmed`; `text_bytes` 30; 0 new `wheel` records; the snapshot's `wheel` still `driver` | yes |
| 4 | the stop ends everything | the stop began 15:51:51.505: one Ctrl-C through the pty; the wrapper exited 0 at 15:51:51.506, not killed; the wrapper, the child and the rig's host gone; 0 processes left in the pty's session or the child's; 0 bytes written to the master before the stop; 91 bytes drained and discarded | yes |

The stop's Ctrl-C is a key from the wrapper's terminal, so it left one `wheel` record of cause `human-input` as the
instance's last record (seven records in all). That is the stop's own record, after every reading.

The ledger's `rehearsal` row (`live-sessions.ndjson`, line 1) holds the same four readings and `held` true.

## What the rehearsal does not show

It proves the rig, not the real CLI on it: the fake agent sends no terminal query. Start 1 was the first reading of
`claude` 2.1.287 under `viola run` on a plain pty; its start is in `live-run.md`.
