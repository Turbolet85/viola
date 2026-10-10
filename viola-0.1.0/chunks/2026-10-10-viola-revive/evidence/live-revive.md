# The live readings (plan.md step 12): three starts of the cap of three

**On `claude` 2.1.287, `viola revive` reopened the session the dead instance had logged: the new
`session-start` read cause `resume` with the first life's id, and the child ran in the directory the first
wrapper recorded, not the one the revive was typed in.** `/compact` inside that session kept the id. The same
id resumed by hand from another directory was found, ran in that other directory and loaded that directory's
project file.

Three live starts were made, the founder's number for this chunk (`inputs#I5`), each ledgered with the clock's
time before it was made, all on the headless pty rig. No compositor was started, no window was opened and no
desktop key was typed (`hyprctl locked` read `true` before the first start, and the rig needs neither). Each
session was stopped through the rig and left no process. Times are UTC, read from the clock, on 2026-10-10.

No prompt text, assistant text or payload text is in this file or in the ledger: a reading of text is a count,
a code, a key name or an equality. No reading here became a ledger row or a fixture, and no acceptance
criterion of the plan rests on this file.

- The ledger: `live-sessions.ndjson`, 5 rows (the rehearsal, three `start` rows, the final `census`).
  `live-ledger.py` is its one writer; each row's `written_at` is the clock's.
- The rig: `live-pty.py`, `live-ledger.py` and `live-drive.py`, byte for byte the copies of chunk
  2026-10-09-inner-cr-and-crlf-in-a-sent-text (`cmp` exit 0 on each; `live-drive.py selftest`: 18 cases,
  0 mismatches). New here: `live-setup.sh` (the two session directories), `live-wait.py` (a bounded wait on an
  instance's event log) and `live-keys.py` (a recorder of the SessionStart payload's key names).
- The product verbs ran with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given; the rig's
  host declares the session's environment (8 names, no `CLAUDE*` and no `VIOLA*` name).
- The home: `target/e2e-home/viola-live-4043089/home`, stamped for 2.1.287 on 2026-10-08, one version, 17 rows
  `pass`, mode 0600, read at 14:47:42Z. Its backing: a link to a `tmpfs` directory.
- The build: `target/release-check/release/viola`, built from this chunk's tree at 14:44:58Z through
  `scripts/release-check.sh` (last line `release-check: viola only`), sha256 prefix `59ffbec884180d9c`,
  4 091 184 B, read again at 14:47:42Z. No file under `src` or `crates` was edited after that build.
- The host at 14:47:42Z (`hostwatch.py read --last 5 --for viola`): verdict `QUIET`, 0 s stalled on IO.
- The private directory, outside the tree, mode 0700: `rev-rig/` under the implementing session's scratchpad,
  `/tmp/claude-<uid>/<project-dir>/4c55e77e-193f-48b5-a38e-21449aedccec/scratchpad/rev-rig`, where
  `<project-dir>` is the repository's absolute path with every `/` written as `-`. It holds the four sent
  texts, the drive journal, the waits, the recorder's lines, the rig's recorder plugin and each product call's
  stdout and stderr. It stands on the tmpfs behind `/tmp` and is gone at a reboot.

## The two directories

`live-setup.sh target/rev-live-911840` made `a` and `b` under the repository's ignored build directory, inside
the tree the founder trusted. Each holds one project file, a `CLAUDE.md` of one line naming a marker word of its
own (`a`: ALPHA, `b`: BRAVO). The first session was typed in `a`; the revive and the by-hand resume were typed
in `b`. No trust dialog appeared in either: every start logged its `session-start`.

## The sessions

| row | ledgered | started | verb, typed in | came up | stop |
|---|---|---|---|---|---|
| rehearsal 1, fake agent as `claude`, instance `revreh` | 14:47:55 (after it, its readings in the row) | 14:46:43.396 | `run`, `a` | `session-start` cause `startup` 0.02 s after the start; one turn read back | one Ctrl-C, exit 0, not killed |
| rehearsal 2, the same instance | the same row | 14:47:01.068 | `revive`, `b` | `session-start` cause `resume`, the first id, 0.03 s after the start; the fake agent's `cwd` receipt and the snapshot's `cwd` both `a`; `--list` two rows; a second `revive` on the live name exit 1 | one Ctrl-C, exit 0, not killed |
| start 1, instance `revlive` | 14:47:55 | 14:48:04.958 | `run -- <claude 2.1.287 by path> --model haiku`, `a` | `wheel` (start), `budget-gate`, `session-start` cause `startup` 0.88 s after the start; `cli_version` 2.1.287, `cli_verified` true; snapshot `cwd` `a` | three Ctrl-C presses from 14:48:21.917, `session-end` at 14:48:22.687, exit 0, not killed |
| start 2, the same instance | 14:48:34 | 14:48:43.760 | `revive revlive -- --model haiku --plugin-dir <the rig's recorder>`, `b`, `claude` by name with the 2.1.287 install directory first on the `PATH` | the same three records, `session-start` cause `resume` 0.60 s after the start; `cli_version` 2.1.287, `cli_verified` true; snapshot `wheel` `driver`, `cwd` `a` | three presses from 14:49:41.649, exit 0, not killed |
| start 3, instance `revhand` | 14:49:52 | 14:50:00.929 | `run -- <claude 2.1.287 by path> --resume <the first id> --model haiku --plugin-dir <the rig's recorder>`, `b` | the same three records, `session-start` cause `resume` 0.60 s after the start; snapshot `cwd` `b` | three presses from 14:50:29.034, `session-end` at 14:50:29.805, exit 0, not killed |

Each `start` row was written 9 to 10 s before its start. The bare name `claude` resolves to 2.1.289 on this
host's own `PATH`, so start 2 put the 2.1.287 directory first for the rig's host alone; `command -v claude`
under that `PATH` printed the 2.1.287 file and `claude --version` printed `2.1.287 (Claude Code)`.

## Start 1: the session to resume

One turn on a synthetic text of 30 bytes: the send exit 0 and read back, `prompt-submitted` 29 ms after
`send-issued`, `turn-ended` 1.72 s after it; the last message is one word and equals the word asked for. The
session's id as logged is a UUID of the committed fixtures' form; it is called the first id below.

## Start 2: the resume itself

| reading | read |
|---|---|
| the first record past the first life's log (offset 1 254) | `wheel` with holder `driver`, cause `start` |
| the new `session-start` | cause `resume`; `agent_session_id` equal to the first id |
| `viola revive revlive --list`, run beside the live session | exit 0, two rows: `startup` then `resume`, one id |
| `viola revive revlive` on the live name | exit 1, stdout 0 bytes, stderr the two lines `unable: revlive is already live` / `hint: viola list` |
| the payload's `cwd` (the rig's recorder) | `a`, the recorded directory; not `b`, where the revive was typed |
| the snapshot after the revive | `wheel` `driver`, `cwd` `a` |
| a turn in the resumed session (33 bytes) | send exit 0, read back; `turn-ended` 1.26 s later; the last message is the one word asked for |

### The payload's key names, beside the fake agent's

The rig's recorder is a second plugin folder, passed after `--`, whose one SessionStart hook writes the
payload's key names. It ran beside viola's own hook, and viola's `session-start` record landed as above.

| payload | keys | names |
|---|---|---|
| the fake agent under `--resume` (plan step 5: the recorded `SessionStart.default` with two fields set) | 5 | `cwd`, `hook_event_name`, `session_id`, `source`, `transcript_path` |
| the real SessionStart of start 2, `source` `resume` | 10 | those five, and `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`, `scratchpad_dir`, `seconds_since_last_response` |
| the real SessionStart of start 3, `source` `resume` | 10 | the same ten |
| the real SessionStart after `/compact`, `source` `compact` | 8 | the five, and `model`, `prompt_id`, `scratchpad_dir` |

**The real resume payload carries five key names the fake agent's rewritten payload does not.** The two fields
viola reads (`source`, `session_id`) are in both, and viola filed the real payload as cause `resume` with the
id. The other five are not read by viola. A startup payload was not recorded in this chunk (start 1 ran without
the recorder), so whether `scratchpad_dir` is also in today's startup payload is not known; the committed
2.1.287 fixture of 2026-10-04 holds the five names only.

## Start 2, continued: `/compact` (the study's P6, its `/compact` half)

| reading | read |
|---|---|
| `viola send` of the 8-byte text `/compact` (issued 14:49:11.194) | exit 13 after 10.0 s: `not-delivered`, `no-prompt-submitted`, one `hint:` line that does not name `release`; the log holds `send-issued` then `send-refused` |
| the wheel after that refusal | no `wheel` record; the snapshot's `wheel` read `driver` |
| the next `session-start` (14:49:29.973, 18.8 s after the send was issued) | cause `compact`; `agent_session_id` equal to the first id |

**`/compact` did not rotate the id.** The command ran although its send was refused: it fires no
UserPromptSubmit, so the send had nothing to read back.

## Start 3: the same id resumed by hand from another directory (the study's P7)

| reading | read |
|---|---|
| found? | yes: `session-start` cause `resume`, `agent_session_id` equal to the first id, 0.60 s after the start |
| where the session runs | the payload's `cwd` is `b`; the snapshot's `cwd` is `b` |
| the payload's `transcript_path` | its directory's name ends as `b`'s project directory would |
| which directory's project file loads | one turn on a synthetic text of 85 bytes asking for the marker word: send exit 0, read back, `turn-ended` 2.69 s later; the last message is one word, equal to BRAVO and not to ALPHA: `b`'s |
| the CLI's own files afterwards (names' shapes, sizes and line counts only, read at 14:50:42Z) | one project directory for these sessions, `a`'s; none for `b`. In it the file named for the first id: 746 212 B, 99 lines, last modified 14:50:29Z, during start 3's stop; and one other file, not a UUID-named one, 335 075 B, 43 lines, last modified 14:49:05Z |

So a resume from another directory is found and takes the other directory's project file, while the one
transcript file on disk stayed under the first directory. The payload named a path under `b`'s directory and no
such directory existed 12 s after the session ended; why is not known.

## Every wait, its length and what it settled on

| wait | on | bound | began | waited | settled on |
|---|---|---|---|---|---|
| `reh1-up` | `revreh`, a `session-start` from offset 0 | 20 s | 14:46:43.483 | 0.000 s | found: cause `startup`, 14:46:43.419 |
| `reh2-up` | `revreh`, from offset 1 138 | 20 s | 14:47:01.161 | 0.000 s | found: cause `resume`, 14:47:01.093 |
| `live1-up` | `revlive`, from offset 0 | 60 s | 14:48:05.030 | 0.852 s | found: cause `startup`, 14:48:05.839 |
| `live2-up` | `revlive`, from offset 1 254 | 60 s | 14:48:43.845 | 0.552 s | found: cause `resume`, 14:48:44.356 |
| `live2-compacted` | `revlive`, from offset 2 264 | 180 s | 14:49:21.213 | 8.776 s | found: cause `compact`, 14:49:29.973 |
| `live3-up` | `revhand`, from offset 0 | 45 s | 14:50:01.005 | 0.552 s | found: cause `resume`, 14:50:01.527 |

The four product waits on a turn (`viola wait --after <cursor>`, bound 120 s each, 20 s in the rehearsal)
settled on `turn-ended` after 0.003 s (rehearsal), 1.717 s (start 1), 1.260 s (start 2) and 2.689 s (start 3).
Each stop was waited for through the host's status file, bound 20 s: 1.1 s, 1.2 s and 1.2 s. No wait reached
its bound.

## What was not measured

- any CLI version but 2.1.287, and any host but the Linux dev host;
- a resume of a session killed rather than stopped: start 1 was stopped through the rig. The kill is the fake
  agent case's (`tests/chaos_revive.rs`);
- `--fork` on the real CLI: no start was left for it. `--fork-session` is in the 2.1.287 help text;
- whether the resumed sessions held the earlier turns in context: the recorder kept key names and four values,
  not `context_tokens`;
- the study's P4 and P12 (a resume of an id live elsewhere, the agents rows): they left with `session-live`;
- which project settings load on a resume from another directory: only the project file `CLAUDE.md` was read;
- a startup payload's key names on this day's CLI;
- `/compact` on a long session: the session held two short turns when it was compacted.

## The census

After start 3 (14:50:30Z), over the host's process list by executable and command line: 0 rig hosts, 0
processes of the product build, 0 of the fake agent as `claude`, 0 of `claude` 2.1.287 by path. Each host's
status: 0 bytes written to the master before the stop; 6 100, 22 667 and 7 464 bytes drained and discarded. The
`plans/` directory beside the home is still empty.

Three instance directories were added to the stamped home (`revreh`, `revlive`, `revhand`); the home stands on
the tmpfs and was not removed. The two session directories stand too: `target/rev-live-911840/` with `a` and
`b`, each holding its one-line `CLAUDE.md`. Their removal was asked for once and denied by the permission
layer, so it was not done another way; the directory is ignored by git and its deletion is the founder's. The live sessions ran four short model turns and one compaction on the user's
own subscription, on the model alias `haiku`. The CLI kept its own transcript of them under its project
directory for `a`; viola wrote nothing there.
