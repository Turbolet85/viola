# The live round — one `viola verify --record` on claude 2.1.287

Chunk 2026-10-13-paste-newline-ledger-row, plan step 8. The founder's cap (`inputs#I2`, fork 2): five live `claude`
starts, one round, fired once, never retried. This round was that one firing. The cap is now spent (5 of 5) and no
further `claude` start of any kind is made in this chunk. The starts were ledgered before the firing in
`live-starts.md`; the round's entry lines, verbatim, are in `round-213213Z.txt`.

## Times (`date -u`)
- Ledgered, before the firing: 2026-10-13T21:32:02Z.
- Start: 2026-10-13T21:32:13Z, the record dir's own name `viola-record-20261013T213213Z`, read by `date -u` inside
  the record entry before its build.
- End: the entry ran 57.13 s, so about 21:33:10Z by the entry's own duration. Read after the completion notice:
  2026-10-13T21:33:13Z.

## The five starts
The print probe, Run A (untrusted, OS temp dir), Run B (trusted, `.viola-verify-<pid>/` under the repository root,
the probe prompt then the five pastes: the long text, the tag-like text, the newline text, `/clear`), Run C
(dialogs) and Run D (plan). They ran as one `viola verify --record` against
`~/.local/share/mise/installs/claude/2.1.287/claude` (by path), home `target/e2e-home/viola-record-20261013T213213Z/vhome`,
`VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed, from the repository root. The probe entry before it ran
`test -x` and started nothing.

## The step lines and the stamp (the record entry's stdout, whole)

```
[01/18] shim-resolution claude resolves to a real executable  pass
[02/18] spine-hooks spine hooks fire through the plugin dir  pass
[03/18] session-start-fields SessionStart carries session_id and source  pass
[04/18] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass
[05/18] stop-message Stop carries last_assistant_message  pass
[06/18] largest-hook-payload every hook payload fits the frame cap  pass
[07/18] modal-signature an untrusted start shows a compiled modal literal  pass
[08/18] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass
[09/18] quiet-period the screen settles within the gate's maximum wait  pass
[10/18] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass
[11/18] question-answer a question answered through PreToolUse takes effect  pass
[12/18] plan-approve-revise a plan revise and approve each take effect  pass
[13/18] question-notes free text and notes reach the question  pass
[14/18] dialog-concurrency two parallel questions each raise a dialog  pass
[15/18] long-paste-wrapper a long paste unwraps to the text as pasted  pass
[16/18] tag-escaping tag-like text un-escapes to the text as pasted  pass
[17/18] local-command-clear /clear starts a new session and submits no prompt  pass
[18/18] paste-newline an LF inside a paste stays inside the one prompt submitted  pass
stamped 2.1.287  18 pass  0 fail
```

The recording was not refused (no `unable:` line; the entry exited 0). The out dir holds 24 files.

## The recorded `paste-3` against the compiled text
- Only `UserPromptSubmit.paste-3.json` was copied into `fixtures/claude/2.1.287/`. It is byte-equal to the
  recording (`cmp`), 621 bytes, sha256 prefix `97317863f8649e2e`, one raw newline at the end (`}\n`).
- Its `prompt` equals `PROBE_NEWLINE_PASTE` exactly: 113 bytes each, one LF, no CR. The CLI submitted the LF
  inside the one prompt, the post-condition the row reads.
- Keys: `cwd`, `hook_event_name`, `permission_mode`, `prompt`, `prompt_id`, `scratchpad_dir`, `session_id`,
  `transcript_path`. `cwd` reads `~/dev/projects/viola/.viola-verify-1474175`, scrubbed to `~`.

## Census after the round
- The census entry (plan entry 9) read green, with no output. No `claude` process had a cwd under
  `target/e2e-home/viola-record-*`, the repository root's `.viola-verify-*` or the OS temp dir's
  `viola-verify-*` (Run A's).
- The round's own termination read reported no survivors.
- Re-read by hand after the round: none such, and no `.viola-verify-*` dir under the repository root and no
  `viola-verify-*` dir under the OS temp dir left.

## Where the round's home stands
`target/e2e-home/viola-record-20261013T213213Z/` (`vhome/` and `out/`) stands on the tmpfs backing
(`target/e2e-home` → an owner-only tmpfs dir), left in place. It is gone at the next reboot.
