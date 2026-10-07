# Step 12 — the by-path re-verify of `claude` 2.1.287: GREEN, fired once

**Outcome: `stamped 2.1.287  17 pass  0 fail`, exit 0, `round: COMPLETE · legs fired 1/1`. Five live starts, the
whole of the cap. The round was fired once and not again.**

Times are `date -u`, 2026-10-07.

## Before the round
- The plan's sixteen local entries read green in the second whole-block run (12:40Z to 12:43Z), `pre-push` among
  them; the ledger reader (entry 9) read red, as its baseline says it does until this record exists.
- `cargo build -q` exit 0 at 12:43Z: `target/debug/viola` is the tree those gates read.
- The census before, 12:44:01Z (`live-sessions.ndjson`, line 1): eleven `.viola-verify-*` dirs at the root, all
  standing before this chunk (the operator's); none under the OS temp dir; no `claude` process with a probe-dir
  cwd.
- The five `start` lines (`print`, `A`, `B`, `C`, `D`, each `cli` `2.1.287`) were written at 12:44:01Z, each with
  the clock read at its own write. The five starts are children of the one `viola verify` the round runs, so all
  five lines stand before the fire.
- The backing, read before the fire: `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/` read
  `tmpfs`.

## The round
Fired once through `gate.py run --plan <the plan> --live-legs`, 12:44:07Z to 12:44:54Z (wall 47 s). Its entry
lines and summary are in `round-124407Z.txt`, verbatim.

- Entry 17, the non-priming version probe: green, exit 0, 0.06 s. It printed `2.1.287 (Claude Code)`.
- Entry 18, the verify entry: green, exit 0, 46.29 s. No survivor was reported and no ref moved.
- Summary: `round: COMPLETE · legs fired 1/1 · entries 21 · green 2 · red 0 · recorded 0 · timeout 0 ·
  indeterminate 0 · not-run 19`.

The seventeen step lines and the summary, as printed:

```
[01/17] shim-resolution claude resolves to a real executable  pass
[02/17] spine-hooks spine hooks fire through the plugin dir  pass
[03/17] session-start-fields SessionStart carries session_id and source  pass
[04/17] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass
[05/17] stop-message Stop carries last_assistant_message  pass
[06/17] largest-hook-payload every hook payload fits the frame cap  pass
[07/17] modal-signature an untrusted start shows a compiled modal literal  pass
[08/17] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass
[09/17] quiet-period the screen settles within the gate's maximum wait  pass
[10/17] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass
[11/17] question-answer a question answered through PreToolUse takes effect  pass
[12/17] plan-approve-revise a plan revise and approve each take effect  pass
[13/17] question-notes free text and notes reach the question  pass
[14/17] dialog-concurrency two parallel questions each raise a dialog  pass
[15/17] long-paste-wrapper a long paste unwraps to the text as pasted  pass
[16/17] tag-escaping tag-like text un-escapes to the text as pasted  pass
[17/17] local-command-clear /clear starts a new session and submits no prompt  pass
stamped 2.1.287  17 pass  0 fail
```

## The stamp
The round's home is `target/e2e-home/viola-reverify-20261007T124408Z/vhome` (git-ignored, on the tmpfs behind the
`target/e2e-home` link, so it does not survive a reboot). It is left where it is: not removed, not moved. Its
`ledger/stamps.json` (`v` 1, writer `verify`, `written_at` `2026-10-07T12:44:54.473Z`) names one version,
`2.1.287`, with seventeen rows:

| row | verdict |
|---|---|
| `shim-resolution` | pass |
| `spine-hooks` | pass |
| `session-start-fields` | pass |
| `prompt-verbatim` | pass |
| `stop-message` | pass |
| `largest-hook-payload` | pass |
| `modal-signature` | pass |
| `input-box-signature` | pass |
| `quiet-period` | pass |
| `confirm-window` | pass |
| `question-answer` | pass |
| `plan-approve-revise` | pass |
| `question-notes` | pass |
| `dialog-concurrency` | pass |
| `long-paste-wrapper` | pass |
| `tag-escaping` | pass |
| `local-command-clear` | pass |

`measured.typed_probe`, the two settles the `quiet-period` row is checked against, each beside the bound:

| settle | measured | bound (`GATE_MAX_WAIT`) | what it settled on |
|---|---|---|---|
| `ready_settle_ms` | 1 103 ms | 8 500 ms | a compiled input-box literal, no modal (row `input-box-signature` passes on the same screen) |
| `turn_settle_ms` | 617 ms | 8 500 ms | the screen after the probe turn's Stop |

Both are far inside the old 5 000 ms bound too: the row's check was loosened by this chunk, and the measured
values did not need the room. The same record's `prompt_latency_ms` is 30 and `max_turn_gap_ms` is 274. What the
turn settle settled on is read from the row verdicts, not from a recorded screen: this is a re-verify, not a
`--record` round, so no screen was written.

## After the round (12:45:19Z to 12:45:45Z)
- The round's probe dirs carried pid 253926 (read from the directory names of the CLI's own transcripts). No
  `.viola-verify-253926*` dir is left at the root, and none under the OS temp dir. The root's eleven dirs are the
  same eleven as before.
- No `claude` process has a probe-dir cwd (`pgrep -x claude`, each cwd read). No `viola` process of this
  repository is running (`/proc/<pid>/exe` read).
- The home's `ledger/probes/` is empty.
- Residual, the accepted class: under `~/.claude/projects/` the CLI left 4 transcripts written in the round's
  window, 2 for Run B (the session and the one `/clear` opened), 1 for Run C and 1 for Run D. Counted by name and
  modification time; none was opened.
- The ledger's `round` line (`fired` 1, `exit` 0, the last stdout line) and its `census` line (`when` `after`,
  `own_left` 0) were appended at 12:45:45Z.

STOP 1 did not fire. No product source changed after the round at the time of this record (STOP 6 stands for
what follows).
