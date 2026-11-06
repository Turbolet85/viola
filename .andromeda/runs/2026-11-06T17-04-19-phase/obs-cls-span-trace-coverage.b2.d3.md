@answer n=1 entity=span-trace-coverage#5.1 v=yes
why: run.pin_copy placement is part of the live viola run start sequence (item 10), and its PinError::HashMismatch outcome rides on the PinError divergence item 3 folds or ratifies.
@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the send must-trace chain includes pty.paste_write, the PTY seam span item 6 keeps, and the overseer's live send exercises it.
@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write carries text_bytes and paste_mode and never text, the paste-as-text_bytes-only rule of item 6 and the no-send-text rule of item 11.
@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: send-issued and send-refused log fields carry text_bytes never text (item 11), and the client socket-dir-refused process-exit code is the detail code item 14 wording covers.
@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: the send.client cleanup lists the exit 21 detail socket-dir-refused that item 14 ratifies or re-words.
@answer n=6 entity=span-trace-coverage#7.1 v=yes
why: the wait and last must-trace chain is what the spare overseer session (item 10) uses to drive a builder, a reading the scope leaves open.
@answer n=7 entity=span-trace-coverage#7.2 v=yes
why: wait.block and run.wait_dispatch attributes (after, timeout_ms, woken_kind) are carried by the overseer's wait in the live session (item 10 fork).
@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: states last_assistant_message never appears (item 11) and carries the socket-dir-refused exit 21 detail code (item 14).
@answer n=9 entity=span-trace-coverage#7.4 v=yes
why: the wait cleanup names the socket-directory refusal outcome line through process-exit, the start-refusal pair item 14 covers.
@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog to answer must-trace chain includes run.dialog_await, the hold item 12 reads live near the 3600 s ceiling.
@answer n=11 entity=span-trace-coverage#8.2 v=yes
why: run.dialog_await deadline_ms (ceiling 3 600 000) and deadline_hit are the values the near-ceiling dialog hold read (item 12) records.
@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: hook-invoked written only after the hook.dialog response, and question, answer and tool input never logged, are the evidence item 12 names.
@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the hold's end, dialog-expired line and the hook's 3605 s backstop (DIALOG_HOLD_MAX + 5 s) are the near-ceiling behaviour item 12 reads live.
@answer n=14 entity=span-trace-coverage#9.1 v=no
why: the human-takes-the-wheel transition is not named by the scope, the headless rig has no human typing, and the chunk touches no wheel code.
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: run.wheel_transition span attributes concern the wheel, which the scope does not name and the live read does not exercise.
@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: the pause and release socket-directory refusal carries process-exit detail socket-dir-refused, the code item 14 ratifies or re-words.
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the wheel product event as audit record concerns the wheel, which no scope item names.
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the budget governor and statusline path is untouched (task not_needed item 2).
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: budget_write, shell_out and budget_eval attributes concern budget.json and the statusline, which the chunk does not touch (not_needed item 2).
@answer n=20 entity=span-trace-coverage#10.3 v=no
why: statusline hook log fields concern the budget governor, untouched by the chunk (not_needed item 2), and carry none of the values item 11 names.
@answer n=21 entity=span-trace-coverage#10.4 v=no
why: the statusline shell-out cleanup concerns the budget governor, untouched by the chunk (not_needed item 2).
@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: E2 keeps names only on process-start claude-child at spawn (item 6, the portable-pty seam) and no CLAUDE* value in any log (item 11).
@answer n=23 entity=span-trace-coverage#12.1 v=yes
why: the implicit link is recorded inside the overseer's from-bearing send and the taken answer, a path the live read exercises (fork over the scope's open reading).
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the explicit link and unlink methods are channel methods the chunk does not add (not_needed item 3) and the live read does not exercise.
@answer n=25 entity=span-trace-coverage#14.1 v=no
why: E5 snapshot replay and state-recovered lines are untouched, no state-store reader is touched (not_needed item 5), and no live read names snapshot corruption.
