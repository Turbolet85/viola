# Live round — 2026-10-18-compact-post-condition-ledger-row

The founder's cap (`inputs#I3`, relayed again with the implement direction, `inputs#I5`): at most twelve live
`claude` 2.1.287 starts, two `viola verify --record` rounds, the second only after a red in the first; no plugin start.
The unit gates read green before any start (implement run `.andromeda/runs/2026-10-18T21-07-38-implement/`, firing 1:
`entries 26 · green 9 · red 0`, entry 6 `nextest-unit 16 passed 0 failed`).

## Round 1 — the only round
- **UTC start:** 2026-10-18T21:24:14Z (`date -u` in the call before the firing). **UTC end:** 2026-10-18T21:25:47Z
  (`date -u` in the first call after the round's notification).
- **Fired by:** `gate.py run --plan … --live-legs` (entries 7–11); the listing is `round-212414Z.txt` beside this file.
  Verdict: `round: COMPLETE · legs fired 1/1 · … red 0`.
- **Starts spent: 6 of 12** — the print probe and Runs A, B, C, D, E of one `viola verify --record` against
  `~/.local/share/mise/installs/claude/2.1.287/claude` (entry 9, 78.34 s, the `cargo build -q` included), from the
  repository root, home `target/e2e-home/viola-record-20261018T212421Z/vhome`, `VIOLA_NAME` / `VIOLA_DIR` /
  `VIOLA_BIN` removed. No second round: the first read green, so the STOP rule's re-run was not taken. No plugin start.
- **The binary probe (entry 7):** `test -x` green, starting no process.

### stdout (entry 9's log, whole)
```
[01/24] shim-resolution claude resolves to a real executable  pass
[02/24] spine-hooks spine hooks fire through the plugin dir  pass
[03/24] session-start-fields SessionStart carries session_id and source  pass
[04/24] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass
[05/24] stop-message Stop carries last_assistant_message  pass
[06/24] largest-hook-payload every hook payload fits the frame cap  pass
[07/24] modal-signature an untrusted start shows a compiled modal literal  pass
[08/24] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass
[09/24] quiet-period the screen settles within the gate's maximum wait  pass
[10/24] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass
[11/24] question-answer a question answered through PreToolUse takes effect  pass
[12/24] plan-approve-revise a plan revise and approve each take effect  pass
[13/24] question-notes free text and notes reach the question  pass
[14/24] dialog-concurrency two parallel questions each raise a dialog  pass
[15/24] long-paste-wrapper a long paste unwraps to the text as pasted  pass
[16/24] tag-escaping tag-like text un-escapes to the text as pasted  pass
[17/24] local-command-clear /clear starts a new session and submits no prompt  pass
[18/24] paste-newline an LF inside a paste stays inside the one prompt submitted  pass
[19/24] statusline-override a --settings statusLine runs in the status line's place  pass
[20/24] statusline-shell a statusline command runs through /bin/sh  pass
[21/24] statusline-rate-limits the statusline payload carries two whole-number rate_limits windows  pass
[22/24] resume-session --resume <id> reopens the session under that id  pass
[23/24] agents-join claude agents --json names the live session by pid and session id  pass
[24/24] local-command-compact /compact keeps the session id and submits no prompt  pass
stamped 2.1.287  24 pass  0 fail
```

### The stamp's `measured` (the record home's `ledger/stamps.json`, version 2.1.287)
- `typed_probe`: `ready_settle_ms` 2050 · `turn_settle_ms` 384 · `prompt_latency_ms` 267 · `max_turn_gap_ms` 233 ·
  **`compact_latency_ms` 21413** (from the `/compact` paste to the instant its SessionStart was read; under the 40 s
  `COMPACT_CONFIRM_WINDOW`; the second compaction measured on 2.1.287, after the revive chunk's 18.8 s).
- `largest_hook_payload`: SessionStart 375 · UserPromptSubmit 500 · Stop 523 · SessionEnd 422.
- `dialog_probe.parallel_both_before_first_post`: false.

### The recorded variant
- `SessionStart.compact-1.json` was recorded and copied ALONE into `fixtures/claude/2.1.287/` (the entry's `cp`;
  `cmp` of the record's copy and the committed one: identical). Its `source` is `compact`; its `session_id` equals
  the recorded `SessionStart.resume-1.json`'s, which is Run B's first session id (Run E resumes that id), so the
  compaction kept the session id. Its keys: `cwd`, `hook_event_name`, `model`, `prompt_id`, `scratchpad_dir`,
  `session_id`, `source`, `transcript_path`.
- The record's set holds 29 files (the committed 2.1.287 set held 33 before this chunk; with `compact-1` it holds 34).
- **A SessionEnd for `/compact`:** not readable from the recorded set. `--record` keeps no capture between the newline
  turn and `/clear` other than `compact-1`, and the probe's captures dir is removed when verify ends. What the set
  shows: `SessionEnd.clear-1.json`, the capture directly before `/clear`'s SessionStart, reads `reason` `clear`, so
  it is `/clear`'s own SessionEnd and not a `/compact` one. Whether the CLI fired a SessionEnd before the compact
  SessionStart is still unmeasured.

### Census (entry 10)
`green · exit 0 · no output`: no `claude` process left whose cwd is the record dir, a repository-root
`.viola-verify-*` dir, or a `$TMPDIR/viola-verify-*` dir (Run A's, under the OS temp dir). Afterwards, read by
hand: no `.viola-verify-*` dir under the repository root and none under the OS temp dir.

### Transcripts (entries 8 and 11)
- `viola-verify` transcripts under `~/.claude/projects/` were **60 before and 65 after: a delta of 5**, against the
  security-plan §Data Protection accepted risk's "two per real-CLI verify". The five new files by project dir, ids
  elided:
  - `…--viola-verify-<pid>/<id>-backup-<n>.jsonl` (380 222 B) — a backup the CLI wrote at the compaction;
  - `…--viola-verify-<pid>/<id>.jsonl` (2 754 B) and `…--viola-verify-<pid>/<id>.jsonl` (421 179 B) — Run B's
    directory, two session transcripts (inferred, not read: the second is the session `/clear` starts);
  - `…--viola-verify-<pid>-dialogs/<id>.jsonl` (399 326 B) — Run C;
  - `…--viola-verify-<pid>-plan/<id>.jsonl` (325 501 B) — Run D.
  Run E added none, as before. The CLI writes every one of these files itself; viola writes nothing under
  `~/.claude`. The `-backup-` file is new with the `/compact` paste. The plan names this delta as an expected
  amendment to security-plan §Data Protection when it is other than two.

### Where it stands
The record dir `target/e2e-home/viola-record-20261018T212421Z/` (home `vhome`, set `out/2.1.287/`) is on the tmpfs
backing of `target/e2e-home` and is left on disk, for the founder's desk with the earlier record dirs.
