# The live round (plan.md steps 12–13): one `viola verify` and one wrapped start, both green

**Status (2026-10-19T08:51Z):** on claude 2.1.287 on the Linux dev host, a dialog the wrapper held under
`dialog_hold_s` 3600 was answered by `viola answer` **671.3 s** after its `question` line, and the driver's answer
went into the CLI. The turn ended with `last` equal to the chosen word, and no `dialog-expired` line was written.

- Cap (`inputs#I2` Q3, «Verify + 1 старт, 11 мин»): **1 verify of 1, 1 wrapped start of 1**. Each was ledgered
  before it was made (`live-sessions.ndjson`, `written_at` read from the clock, `date -u` read first). Reds: 0, none
  re-fired. No key was typed into any dialog or into the session: the rig writes nothing to its pty before the stop.
- The reading is for Linux. Its Windows leg is owed to "Windows-only live measurements". The 3600 s ceiling stays
  PROVISIONAL: this round read it only to about 671 s.

No prompt text, question text, assistant text, session id, `CLAUDE*` value, absolute path or `detail-*.ndjson` line is
in this file. Times are UTC, read from the clock, on 2026-10-19.

## Setup

- **The rig:** `live-pty.py` and `live-ledger.py`, copied byte for byte from
  `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/` (`cmp` exit 0 on each). No other rig
  script was written. Scratch readers outside the tree printed codes, counts and booleans only.
- **The build:** `cargo build --bin viola` (gate entry 13, green on the finished tree). `target/debug/viola`
  sha256 prefix `5152f377a903d8ed`.
- **Before (08:37:13Z):** `target/e2e-home` is a link, its backing `tmpfs`. `hyprctl locked` read `true` (also at
  08:39:13Z). It is recorded and did not stop the round: the rig types no desktop key. `claude --version` by path:
  `2.1.287 (Claude Code)`. `~/.claude/projects/`: 107 entries. `*viola-verify*` transcripts: 65.
- **Census before:** none running (this repository's `target/debug/viola`, and args naming `viola-live-hold` or
  `hold-live-`). A first census keyed on the bare text `target/debug/viola` listed the overseer pair's prototype bridge
  processes (another tree's build of the same name, under `~/dev/projects/additional/viola-lab/`). They are not this
  round's, and nothing was done to them.
- **The environment:** every viola verb and the rig ran under a declared environment (`HOME`, `USER`, `LOGNAME`,
  `SHELL`, `LANG`, `XDG_RUNTIME_DIR`, `PATH` with the 2.1.287 dir first, and `TERM` for the rig): no `CLAUDE*` and no
  `VIOLA*` name, with `--home` given.
- **Dirs:** the home `target/e2e-home/viola-live-hold-20261019T083728Z/vhome/` and `target/hold-live-20261019T083728Z/priv/`,
  both fresh and 0700.

## The verify (cap item 1) — green

- Ledgered at 08:37:43Z. From the repository root, 08:37:47Z–08:38:55Z (68 s):
  `target/debug/viola --home <vhome> verify -- <claude 2.1.287>`. Exit 0, stderr empty, and stdout ends
  `stamped 2.1.287  24 pass  0 fail`. No `.viola-verify-*` dir was left at the root.
- `<vhome>/config.json`: 0600, `{"v":1,"dialog_hold_s":3600}` (the overseer's opt-in form, `inputs#I2` Q1).

## The trust guard

A scratch reader read `~/.claude.json`'s verdict for the resolved path of rig B's `proj/`
(`target/e2e-home/viola-live-plugin-20261019T061245Z/proj/`) as the word alone: `trusted`. Its
`.claude/settings.local.json` holds `enabledPlugins` `{}`. Its git root is `proj/` itself.

## The start (cap item 2) — green

- Ledgered at 08:39:13Z. From rig B's `proj/`: `live-pty.py <priv> s1 -- target/debug/viola --home <vhome> run
  builder -- <claude 2.1.287> --model haiku`.
- `session-start` (cause `startup`) at 08:39:18.724Z, inside the 90 s bound. `wheel{holder:"driver", cause:"start"}`
  is the only wheel line before the stop. The snapshot reads `cli_verified` true, `cli_version` 2.1.287.

### The dialog

- `send --file <priv>/prompt.txt --json` (08:39:27Z): exit 0, `ok`, cursor 448. The prompt is one fixed file
  (sha256 prefix `b547c8699ad7b0d2`) asking for one AskUserQuestion call with one question and the two options
  `alpha` and `beta`, then a reply of only the chosen word.
- `wait --after 448 --json` woke on `question`: ts **08:39:30.791Z**, `dialog_id` 1, cursor 1358, one question, and
  both option labels present. The snapshot's `pending_dialog` is `{dialog_id: 1, kind: question}`.

### The hold

- **At 300 s** (08:44:31Z): one `list --json`. The builder row reads `dialog_pending` **true**, `live`, wheel `driver`.
- **At 671.3 s** (08:50:42.074Z, no less than 660 s, waited in the background): `snapshot.json`'s `child_pid` is
  alive, and `pending_dialog` is still held. `answer builder 1 --json` with `{"answers": {<the question>: "beta"}}`
  on stdin, built in memory from the saved `wait` document (no file holds it): exit 0, `{"v":1,"ok":{}}`.
- `wait --after 1358 --json` woke on `turn-ended` (08:50:43.652Z). `last --json`: exit 0, and the message equals
  **`beta`**.

### Readings (codes, counts and booleans)

| reading | value |
|---|---|
| seconds from the `question` `ts` to the answer | 671.3 |
| `run-builder.ndjson` `dialog-answered`, `corr` 1 | `deadline_hit` **false**, `duration_ms` **671 301** (≥ 660 000), `dialog_kind` `question`, no `from` (the answer carried none); 1 such line |
| `hook-builder.ndjson` `hook-decision`, `corr` 1 | `hook_event` `pre-tool-use`, `decision_emitted` **true**, `deadline_hit` **false**, `duration_ms` **671 302** (≥ 660 000), no `detail` |
| `dialog-expired` lines | **0** |
| `last` equals `beta` | **true** |
| `child_pid` alive at the answer | true |
| `list`'s `dialog_pending` at 300 s | **true** |
| `parse-rejected` lines (run) | **0** |
| `panic` lines (run, hook) | 0, 0 |

`events.ndjson` kinds in order: `wheel`, `budget-gate`, `session-start`, `send-issued`, `prompt-submitted`,
`send-confirmed`, `question`, `activity`, `turn-ended`, `budget-gate`.

### The stop

- `<priv>/s1.stop` at 08:51:07Z: 3 Ctrl-C presses, exit 0, not killed, nothing written to the pty before the stop,
  277 899 bytes drained. The wrapper's `process-exit{subject:"self"}` exit 0.
- **Census after:** none running.
- Ledgered at 08:51:24Z (`census`: 1 verify, 1 start, 0 left, 0 reds).

## After

- `~/.claude/projects/`: 111 entries (+4). `*viola-verify*` transcripts: 70 (+5). These are the CLI's own writes
  (the accepted residual). Nothing under `~/.claude` or in `~/.claude.json` was removed or edited.
- The `vhome`, the rig dir and `priv/` stay on disk (tmpfs) for the founder's desk.
