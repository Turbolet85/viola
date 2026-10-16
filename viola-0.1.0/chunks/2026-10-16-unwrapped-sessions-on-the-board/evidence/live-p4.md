# The P4 / P12 leg (plan.md step 17)

**On `claude` 2.1.287 a second interactive `claude --resume <id>` of an id another live `claude` holds is neither
refused nor copied: both sessions run under the one id, two writers on one transcript (P4). After `/clear` the
`claude agents --json` row of the session's pid names the new id (P12 follows). With the two writers standing,
`viola revive` of the first instance was refused `session-live` with the founder's pair and started nothing.**

Two live starts were made, the last two of the founder's cap of 8 for this chunk (`inputs#I1` fork 3; the record
round made the other six), each ledgered with the clock's time before it was made, both on the headless pty rig.
No compositor was started, no window was opened and no desktop key was typed (`hyprctl locked` read `false` before
the first start; the rig needs neither). Each session was stopped through the rig and left no process. Times are
UTC, read from the clock, on 2026-10-16.

No prompt text, assistant text, title, directory content or session id is in this file or in the ledger: a reading
is a count, a code, a cause, a status word or an equality. No acceptance criterion of the plan rests on this file
but its own (the live-start count).

- The ledger: `live-sessions.ndjson`, 3 rows (two `start` rows, the final `census`), `live-ledger.py` its one
  writer; each row's `written_at` is the clock's. A read of it for the 8-4-4-4-12 hex id shape counts 0.
- The rig: `live-pty.py`, `live-ledger.py` and `live-drive.py`, byte for byte the copies of chunk
  2026-10-10-viola-revive (`cmp` exit 0 on each; `live-drive.py selftest`: 18 cases, 0 mismatches). New here:
  `live-p4-read.py`, the reader of equalities (one `claude agents --json` read through the CLI by path, with every
  `CLAUDE*` and `VIOLA*` name removed; a read opens no session).
- The product verbs ran with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given; the rig's host
  declares the session's environment (8 names, no `CLAUDE*` and no `VIOLA*` name); the 2.1.287 install directory
  stood first on every `PATH` that looked `claude` up (the bare name resolves to 2.1.296 on this host).
- The home: `target/e2e-home/viola-record-20261016T204612Z/vhome`, the record round's, stamped for 2.1.287 at
  20:47:12Z, one version, 23 rows `pass`, mode 0600. Its backing: a link to a `tmpfs` directory.
- The build: `target/release-check/release/viola`, built from this chunk's tree at 21:10 through
  `scripts/release-check.sh` (last line `release-check: viola only`), sha256 prefix `51573384480c0029`,
  4 416 320 B. No file under `src` or `crates` was edited after that build.
- The two session directories: `target/p4-live-20261016T211033Z/a` (where `p4a` was typed) and `.../b` (where
  `p4b` and the revive were typed), under the repository's ignored build directory, inside the tree the founder
  trusted. No trust dialog appeared: each start logged its `session-start`.
- The private directory, outside the tree, mode 0700: `p4-rig/` under the implementing session's scratchpad. It holds
  the rig's status files, the `/clear` text, the drive journal, the start-2 script, the revive's two streams and the
  list captures. It stands on the tmpfs behind `/tmp` and is gone at a reboot.

## The sessions

| row | ledgered | started | verb, typed in | came up | stop |
|---|---|---|---|---|---|
| start 7, instance `p4a` | 21:10:50 | 21:10:56.704 | `run p4a -- claude --model haiku`, `a` | `session-start` cause `startup`; `cli_verified` true | three Ctrl-C presses from 21:12:13.620, exit 0, not killed |
| start 8, instance `p4b` | 21:11:49 | 21:11:59.957 | `run p4b -- claude --resume <p4a's latest logged id> --model haiku`, `b` | `session-start` cause `resume` | three presses from 21:12:41.359, exit 0, not killed |

## (a) The board with `p4a` live

`viola list --json` (exit 0): one wrapped item, `p4a`, `live`, STATUS `idle`, `cli_verified` true; 11 unwrapped
items (the host's other live sessions, counted, never named here), their statuses among `busy`, `idle` and
`unknown`. The human form (exit 0): 15 lines — BAY, the caption, `p4a  live  idle  driver …`, the separator
`-- UNWRAPPED - READ-ONLY --` at line 4, then the 11 read-only rows; printable ASCII only, no line ending in a space.

The join: one `claude agents --json` row carries `p4a`'s child pid; its id equals `p4a`'s logged id and the
snapshot's `agent_session_id`; status `idle`.

## (b) P12: `/clear`

`viola send p4a` with `/clear` on stdin: exit 0, read back (a local command confirmed by its post-condition). A
second `session-start`, cause `clear`, was logged. Then: `p4a`'s latest logged id differs from its first; exactly one
agents row carries `p4a`'s child pid, and its id **equals the latest (post-`/clear`) id, not the first**; status
`idle`; the snapshot's `agent_session_id` equals the latest id. **P12: follows.** A session-id join therefore
survives a `/clear` as the wrapper writes it on each `session-start`.

## (c) P4: a second `--resume` of the live id

`p4b`'s `session-start`: cause `resume`, one record, its id **equal** to `p4a`'s latest id (`p4a~p4b` equal). The
agents read listed 13 rows: one carrying `p4a`'s child pid and one carrying `p4b`'s, **both with that one id**
(`rows_with_latest_id` 2). The CLI neither refused the start (a `session-start` came) nor copied the session (the id
is the same). **P4: two writers.**

## (d) The revive, on `two writers`

`p4a` was stopped through the rig. Read before typing anything: `p4a` had no row of its own, and one live row
(`p4b`'s) carried `p4a`'s latest id. Then `viola revive p4a`, typed in `b` with the 2.1.287 directory first on
`PATH`:

- exit 1; stdout empty (0 B);
- stderr exactly `unable: p4a's session is open in another claude` then
  `hint: close it there first, or start a copy: viola revive p4a --fork`, the hint last;
- `run-p4a.ndjson` gained 4 lines: `process-start{subject:"self"}`, the `agents-probe` pair, and
  `process-exit{subject:"self", exit_code:1, detail:"session-live"}`; no `claude-child` start;
- no session started (the cap stood at 8 of 8).

Since P4 read `two writers`, `session-live` stays a refusal; no conversion to a warning is owed (`inputs#I1` fork 3).

## (e) Stop and census

`p4b` was stopped through the rig (exit 0, not killed). The census at 21:12:42Z: no `claude` process whose cwd is
under `target/p4-live-*`, `target/e2e-home/viola-record-*` or a `.viola-verify-*` directory, and no running process
whose executable lies under this repository. Ledger row `census` written at 21:12:55Z.
