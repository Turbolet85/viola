# Reproduction — the red-before reading (plan step 4)

**Taken:** 2026-10-19T23:51Z (UTC), by /implement, run dir `.andromeda/runs/2026-10-19T23-48-08-implement/`, gate
entry 22 (`bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal/)'`),
its first firing: trail `gate-2026-10-19-newer-claude-cli-rows.json`, log `$TMPDIR/andromeda-gate/2026-10-19-newer-claude-cli-rows/implement-2026-10-19T23-48-08/22.log`
(9091 B, sha8 `cab57f12`).

**Tree state:** plan steps 1–3 built (`TypedRun::long_turn`, Run B's `framing_turns` filling it, the fake agent's
`--long-turn-screen`), step 4's case written with the predicted green-side literal, and step 2's `check` arm NOT yet
extended (`input-box-signature` still read `ready` and `turn` only).

**Reading:** the case read red at its stdout assertion (`tests/cli_verify.rs:1521`), exit 1, 3.70 s. The fake agent's
verify printed, verbatim (the `stdout` the assertion's message carries):

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
[16/24] tag-escaping tag-like text un-escapes to the text as pasted  fail
[17/24] local-command-clear /clear starts a new session and submits no prompt  fail
[18/24] paste-newline an LF inside a paste stays inside the one prompt submitted  fail
[19/24] statusline-override a --settings statusLine runs in the status line's place  pass
[20/24] statusline-shell a statusline command runs through /bin/sh  pass
[21/24] statusline-rate-limits the statusline payload carries two whole-number rate_limits windows  pass
[22/24] resume-session --resume <id> reopens the session under that id  pass
[23/24] agents-join claude agents --json names the live session by pid and session id  pass
[24/24] local-command-compact /compact keeps the session id and submits no prompt  fail
stamped 2.1.0  20 pass  4 fail
```

**Against the overseer's 2.1.302 verdict**
(`viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/inputs/I20-verify-2.1.302-20261019.out.txt`, read the same
minute): all 24 row lines are identical, word for word and verdict for verdict. The only difference is the version
in the summary: `stamped 2.1.0  20 pass  4 fail` here, `stamped 2.1.302  20 pass  4 fail` there. The four red rows
are the same set: `tag-escaping`, `local-command-clear`, `paste-newline`, `local-command-compact`.

So the stop reproduces without a live start. A screen after the long paste's turn that shows no input box Run B can
type into ends Run B's pastes, and only those four rows read it. `input-box-signature` passes, because it read only
the `ready` and `turn` screens.

**Green side:** with step 2's arm extended, the same case asserts `stamped 2.1.0  19 pass  5 fail` on Unix, with
`input-box-signature` the fifth red row. It read green at entry 22's firings 2 and 3 (`22.2.log`, `22.3.log`, 3.66 s
and 4.56 s). The Unix literal is the run's own, from firing 1's assertion message: the `right` side, which that
firing's red was set against, printed `stamped 2.1.0  19 pass  5 fail`. The Windows literal `16 pass  5 fail`
remains the forecast (24 − 3 statusline rows − 5). CI's `windows-2025` job reads it first.

## Remove-the-guard control on the `long_turn` unit cases

Taken before 2026-10-20T00:00:52Z, the `date -u` read just after it was recorded (testing.md 2026-09-25). `check(InputBoxSignature)`'s third operand
`&& input_box_ready(typed.long_turn.as_ref())` was removed (re-read at `ledger.rs:1011-1012` before the run), and
`bash scripts/agent-run.sh run --unit --filter 'test(/long_turn/)'` read exit 1, `5 tests run: 3 passed, 2 failed`:
- FAIL `ledger::tests::check_input_box_signature_long_turn_holding_a_modal_fails`
- FAIL `ledger::tests::check_input_box_signature_long_turn_of_none_fails`
- PASS `check_input_box_signature_long_turn_holding_the_literal_passes` and the two fake-agent parse cases. These are
  the cases the arm does not decide.

The operand was then restored (re-read at `ledger.rs:1011-1013`). Entries 1, 2, 6, 7, 22 and 24 re-fired green on
the restored tree (gate trail, firings `1.2`, `2.2`, `6.2`, `7.2`, `22.3`, `24.2`).
