# The live readings (plan.md step 12): three starts of the cap of three

**On `claude` 2.1.287 on the Linux dev host, all three behaviours the chunk relies on read as the plan
supposed.** The CLI ran a statusline command string through a shell it started as `/bin/sh`. The override viola
writes took the status line's place for the session, and its unquoted `<pinned path> hook statusline` string
ran. `rate_limits` was absent from the first payload of a session and then carried two windows, each a whole
number `used_percentage` and a whole number `resets_at` that reads as epoch seconds.

Three live starts were made, the founder's number for this chunk (`inputs#I4`), each ledgered with the clock's
time before it was made, all on the headless pty rig. No compositor was started, no window was opened and no
desktop key was typed (`hyprctl locked` read `true` before the first start, and the rig needs neither). Each
session was stopped through the rig and left no process. Times are UTC, read from the clock, on 2026-10-10.

No prompt text, assistant text or payload value is in this file or in the ledger: a reading of a payload is a key
name, a JSON type, a count or an equality. No reading here became a ledger row or a fixture, and no acceptance
criterion of the plan rests on this file.

## One departure from the step as written

Step 12 item 1 names the recorder as the session's **user-scope** statusline command. The user-scope file is the
founder's own `~/.claude/settings.json`; it holds a `statusLine` of type `command` today (presence and type
read, nothing else), and every session he has open reads it. No word covers editing it, so it was not opened for
writing. The recorder was named by the **project settings of the rig's own directory** instead
(`target/sl-live-20261010/a/.claude/settings.json`, under the ignored build directory). What that changes in the
readings is said at each one.

## The rig

- The ledger: `live-sessions.ndjson`, 5 rows (the rehearsal, three `start` rows, the final `census`).
  `live-ledger.py` is its one writer, byte for byte the copy of chunk 2026-10-10-viola-revive (`cmp` exit 0).
- The host: `live-pty.py`, that chunk's copy plus one addition, said in its header: when
  `<private dir>/<label>.needles` exists it counts each listed ASCII literal in the drained stream. The stream is
  still discarded and nothing is written to the session while it runs.
- New here: `live-recorder.sh` (the statusline recorder), `live-sl-setup.py` (the two directories and the private
  directory's files; every JSON document written by `json.dumps`), `live-sl-start.sh` (the launcher; it passes
  no `--settings` of its own) and `live-sl-read.py` (the reader and the bounded waits; it prints codes only).
- The recorder is named as `<private dir>/live-recorder.sh <label> "$0" "${BASH_VERSION:+bash}"
  "${ZSH_VERSION:+zsh}"`. The three quoted words are expanded by whatever shell runs the string, so they say
  which shell it was. It files its stdin whole and a few facts about its ancestors, and prints `SLMARK<label>`.
- The host declares the session's environment (8 names, no `CLAUDE*` and no `VIOLA*` name); every launch ran
  with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed.
- The CLI: `claude` 2.1.287 named by path (`--version` printed `2.1.287 (Claude Code)` at 19:22:41Z), its install
  directory first on the host's `PATH`, on the model alias `haiku`, with one short prompt as its argument.
- The build: `target/release-check/release/viola`, built from this chunk's tree at 19:19:26Z through
  `scripts/release-check.sh` (last line `release-check: viola only`), sha256 prefix `ee7f6565675202f1`,
  4 131 032 B, read again at 19:22:41Z. No file under `src` or `crates` was edited after that build.
- The home: `target/e2e-home/viola-live-sl-20261010/home`, new, made by the rehearsal's first start. **It is not
  stamped**: every session read `cli_version` 2.1.287 and `cli_verified` false. The stamped live home was left
  alone, so no source file and no `budget.json` were put into it. Its backing: a link to a `tmpfs` directory.
- The host at 19:22:41Z (`hostwatch.py read --last 5 --for viola`): verdict `QUIET`, 0 s stalled on IO.
- The private directory, outside the tree, mode 0700: `sl-rig/` under the implementing session's scratchpad,
  `/tmp/claude-<uid>/<project-dir>/ee3aa00f-8953-40bb-96e6-f1ea969eb1d8/scratchpad/sl-rig`, where
  `<project-dir>` is the repository's absolute path with every `/` written as `-`. It holds the recorder, every
  payload it filed, the waits, the ledger's row files and each host's status. It stands on the tmpfs behind
  `/tmp` and is gone at a reboot.

## The sessions

| row | ledgered | started | what ran, typed in | stop |
|---|---|---|---|---|
| rehearsal 1, fake agent, instance `slreh` | after it, in the rehearsal row | 19:21:59.107 | `viola run`, `a`, no source | one Ctrl-C, exit 0, not killed |
| rehearsal 2, the same instance | the same row | 19:22:08.820 | `viola run`, `a`, the recorder planted as the home's source, the fake agent given the rig's synthetic payload | one Ctrl-C, exit 0, not killed |
| start 1 | 19:22:41 | 19:22:50.635 | `claude` unwrapped, `a` (the directory's settings name the recorder, label `direct`) | three presses from 19:23:02.448, exit 0, not killed |
| start 2, instance `slsrc` | 19:23:16 | 19:23:26.188 | `viola run -- <claude 2.1.287 by path>`, `a`, the recorder planted as the home's source (label `wrapped`) | three presses from 19:23:46.547, exit 0, not killed |
| start 3, instance `slbare` | 19:24:11 | 19:24:22.706 | `viola run -- <claude 2.1.287 by path>`, `b`, no source | three presses from 19:24:32.739, exit 0, not killed |

Each `start` row was written 9 to 12 s before its start. No trust dialog appeared: every session ran its status
line or its hooks.

## The rehearsal (no live start)

The release build under the pty host with the harness build's fake agent, named by path.

- Rehearsal 1: the start made the home, wrote `instances/slreh/settings.json` at mode 0600 with a `statusLine`
  whose command ends `viola hook statusline`, and recorded no `statusline_command`.
- Rehearsal 2: the fake agent ran the override's command with the 149-byte synthetic payload. The recorder ran
  once, started by a shell named `/bin/sh`, its ancestors `viola` < `viola-fake-agent` < `viola`; it was handed
  149 bytes; the fake agent receipted exit 0 and the 13 bytes `SLMARKwrapped`; `budget.json` was written at mode
  0600 and equals the payload's reading; the role file holds `hook-invoked`, the `statusline-shell` pair with
  exit 0, and `hook-decision` with `budget_written` true.

## Start 1: the shell, and the payload

| reading | read |
|---|---|
| recorder runs | 3, at 19:22:51.681, 19:22:52.044 and 19:22:55.928 |
| the shell's own name (`$0` as the launching shell expanded it) | `/bin/sh`, on all three |
| is that shell bash, is it zsh | `BASH_VERSION` set, `ZSH_VERSION` not: on this host `/bin/sh` is a link to bash |
| the recorder's parent process | `claude` itself, then the rig's host: the shell replaced itself with the command |
| the words reached the recorder expanded | yes, 4 arguments: a shell ran the string, the CLI did not split it itself |
| `COLUMNS` and `LINES` | both set, on all three |
| payload size | 1 157, 1 209 and 1 797 bytes; each one JSON object |
| top-level key names, first run (13) | `context_window`, `cost`, `cwd`, `exceeds_200k_tokens`, `fast_mode`, `model`, `output_style`, `scratchpad_dir`, `session_id`, `thinking`, `transcript_path`, `version`, `workspace` |
| added on the second run | `prompt_id` |
| added on the third run | `prompt_cache`, `rate_limits`, `session_name` |
| `rate_limits` | absent on the first two runs; present on the third, 4.2 s after the second |
| its windows | `five_hour` and `seven_day`, no other |
| each window's keys | `resets_at`, `used_percentage`, no other |
| `used_percentage` | a whole number, from 0 to 100, in both windows |
| `resets_at` | a whole number of 10 digits in both windows. Read as epoch seconds it lies ahead of the run, within 5 hours for `five_hour` and within 7 days for `seven_day` |
| the recorder's output on the screen | the literal `SLMARKdirect` occurred once in the session's 6 701 drained bytes |

**The shell is `/bin/sh` by the name the CLI started it under.** The flag it was given was not read: the
recorder sees the shell's name and the expanded words, not the shell's own arguments. The plan's `/bin/sh -c`
holds for the program; `-c` is the only way a POSIX shell takes a command string, and it stays an inference.

Because the recorder was named by project settings, this is the shell and the payload of a project-scope status
line. Whether a user-scope one runs through another shell or gets another payload was not measured.

## Start 2: the override, and the pass-through

The directory still named the recorder with label `direct`; the home's source named it with label `wrapped`.

| reading | read |
|---|---|
| recorder runs with label `direct` | 0 |
| recorder runs with label `wrapped` | 3, at 19:23:26.969, 19:23:27.320 and 19:23:29.147 |
| their ancestors | `viola` (the hook) < `claude` < `viola` (the wrapper), on all three |
| the shell viola started | named `/bin/sh`, `BASH_VERSION` set |
| payload size as the recorder got it | 1 157, 1 341 and 1 796 bytes |
| key names | the union is start 1's 17 names, none more and none less; 13, 15 and 17 on the three runs |
| `rate_limits` | absent on the first run, present on the second (0.35 s later) and the third; the same two windows and the same types as in start 1 |
| `COLUMNS` and `LINES` in the recorder's environment | both set |
| `VIOLA_NAME` in the recorder's environment | set: the user's command gets the hook's environment unchanged |
| `budget.json` | `v` 1, keys `five_hour`, `seven_day`, `read_at`, `v`; both windows objects with `used_percentage` a number and `resets_at` a string; mode 0600; both windows equal the newest payload's reading (the percentage as it stood, `resets_at` as that second in RFC 3339 UTC) |
| the hook's role lines for `statusline` | 3 `hook-invoked`, 3 `process-start` and 3 `process-exit` with subject `statusline-shell` and `shell_exit_status` 0, 3 `hook-decision`: one with `budget_written` false (the first payload, no `rate_limits`) and two with true; no `corr`, no `detail`; 13 field names, all the schema's |
| the hook's whole run | 18 to 20 ms; the shell-out inside it 17 to 20 ms |
| the recorder's output on the screen | `SLMARKwrapped` occurred once in 5 843 drained bytes; `SLMARKdirect` did not occur |
| the session's other hooks | `session-start`, `user-prompt-submit`, `stop` and `session-end` each logged; the event log reads `wheel`, `budget-gate`, `session-start`, `prompt-submitted`, `wheel`, `turn-ended`, `session-end` |

**The override replaced the lower-scope status line for the session, and its unquoted string ran.** Measured
against the directory's project settings: with them naming the recorder as `direct`, no `direct` run happened and
every run came through `viola hook statusline`. That the same holds over a **user-scope** status line is read in
start 3, where the user's own was the lower scope.

**`rate_limits` on the second run of a session is not what the fetched documentation says** ("only after the
first API response in the session"): here it arrived 0.35 s after the first run, in a session started 22 s
after start 1 ended. Why is not known. It does not touch what the chunk built: the arm writes whenever the
key is present.

## Start 3: the override alone

Directory `b` has no settings of its own, so the lower scope was the founder's own user-scope status line.

| reading | read |
|---|---|
| the snapshot's `statusline_command` | absent: a home named by `--home` reads no user settings, and no source stood in it |
| the hook's role lines for `statusline` | 3 `hook-invoked`, 3 `hook-decision` (two with `budget_written` false, one with true), no `statusline-shell` pair |
| the hook's whole run | 0 ms on each, as rounded |
| `budget.json` | the same shape; its bytes differ from the copy taken after start 2: a new reading was written |
| recorder runs | 0 of either label |
| on the screen | neither `SLMARKdirect` nor `SLMARKwrapped` occurred in 6 817 drained bytes |

**With no source the override still ran and recorded the reading, and the session showed no status line text
from any command viola knows.** The hook was the session's status line three times over, so the user's own
user-scope command was not the one in effect. That it did not also run was not observed directly: nothing in
this rig watches the founder's own command.

What the status line row itself looked like (empty, or absent) was not read: the rig keeps no screen. The row
held neither marker, and the hook wrote nothing to print.

This is the cost the P4 fork's option named and the founder's word on W3 took (`inputs#I3`, `inputs#I4`): **a
user who starts viola with `--home` loses their own status line in that session until they plant the home's
source file.**

## Every wait, its length and what it settled on

| wait | on | bound | began | waited | settled on |
|---|---|---|---|---|---|
| rehearsal 2 | a `wrapped` recorder run | 20 s | 19:22:15.291 | 0.000 s | found: 1 run, at 19:22:08.850 |
| start 1 up | a `direct` recorder run | 60 s | 19:22:50.708 | 1.003 s | found: 19:22:51.681 |
| start 1 reading | a `direct` run whose payload holds `rate_limits` | 120 s | 19:22:51.731 | 4.216 s | found: 19:22:55.928 |
| start 2 hook | a `statusline` `hook-decision` of `slsrc` | 60 s | 19:23:31.766 | 0.000 s | found: 3 already, the first at 19:23:26.981 |
| start 2 recorder | a `wrapped` recorder run | 60 s | 19:23:31.784 | 0.000 s | found: 3 already, the first at 19:23:26.969 |
| start 2 written | a decision with `budget_written` true | 120 s | 19:23:31.802 | 0.000 s | found: 2 already, the first at 19:23:27.331 |
| start 2 reading | a `wrapped` run whose payload holds `rate_limits` | 30 s | 19:23:31.819 | 0.000 s | found: 2 already, the first at 19:23:27.320 |
| start 3 hook | a `statusline` `hook-decision` of `slbare` | 60 s | 19:24:22.808 | 0.601 s | found: 19:24:23.406 |
| start 3 written | a decision with `budget_written` true | 120 s | 19:24:23.429 | 4.210 s | found: 19:24:27.551 |

Start 2's four waits began 5.6 s after its start (the calls before them took that long), so each found its
record standing. Each stop was waited for through the host's status file, bound 25 s; from the stop's first
Ctrl-C to the host's end the status files read 1.3 s, 1.1 s and 1.4 s. No wait reached its bound.

## What was not measured

- any CLI version but 2.1.287, any host but the Linux dev host, and a host whose `/bin/sh` is not bash;
- a user-scope status line as the recorder (the departure above), and the founder's own command under any start;
- the flag the CLI hands its shell, and how it finds that shell;
- how the CLI cancels a statusline script still running, and whether the user's command then outlives the hook:
  the longest hook run here was 20 ms, and nothing forced an overlap;
- the status line row's own appearance with an empty output;
- `rate_limits` for an account without one, a window whose `resets_at` has passed, and the `spend_limit` window;
- the 5 s bound on the user's command, on a real session;
- macOS and Windows.

## The census

After start 3 (19:24:34Z), over the host's process list by executable and command line: 0 rig hosts, 0
processes of the product build, 0 of the fake agent, 0 of `claude` 2.1.287 by path. Each host's status: 0 bytes
written to the session before the stop; 6 701, 5 843 and 6 817 bytes drained and discarded.

Left standing, none of it in a commit:

- the rig home `target/e2e-home/viola-live-sl-20261010/` on the tmpfs, with three instance directories
  (`slreh`, `slsrc`, `slbare`) and its `budget.json`; the planted source file was moved out of it into the
  private directory before start 3;
- `target/sl-live-20261010/` with `a` (its one settings file) and `b` (empty), ignored by git;
- the private directory;
- the CLI's own files for the three sessions under its project directories for `a` and `b` (two entries and
  one, names not read); viola wrote nothing there.

The three sessions ran three short model turns on the user's own subscription, on the model alias `haiku`.
`.claude/session-handoff.md` reads the same after them as before (its one changed line is the one this
session's own start found).
