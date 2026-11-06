@answer n=1 entity=span-trace-coverage#5.1 v=no
why: the run.pin_copy concurrent-placement rule concerns the claude pin copy race, which the portable-pty bump, the Windows-remnant edits and the live run's stated scope do not reach.

@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the confirmed-send span tree's must-trace pty.paste_write span sits on the PTY seam that the portable-pty bump reaches and item 6 says must still paste as text_bytes only.

@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write's required text_bytes and paste_mode="bracketed" attributes are the paste value item 6 requires the seam to keep after the bump.

@answer n=4 entity=span-trace-coverage#6.3 v=no
why: the send-issued, send-confirmed, channel and hook log fields are wrapper and hook lines the chunk neither changes nor names; the scope's paste check is the seam span of 6.2.

@answer n=5 entity=span-trace-coverage#6.4 v=no
why: the send.client process-exit codes and the confirm-window cleanup are the driver CLI's own exit path, not the child's handle-wait process-exit or the PTY seam that the chunk touches.

@answer n=6 entity=span-trace-coverage#7.1 v=no
why: the wait and last span tree (wait.block, channel.dispatch, run.wait_dispatch) is a channel path the chunk neither changes nor names in its scope.

@answer n=7 entity=span-trace-coverage#7.2 v=no
why: the wait.block and run.wait_dispatch span attributes (after, timeout_ms, woken_kind) belong to the wait verb, which the chunk does not touch.

@answer n=8 entity=span-trace-coverage#7.3 v=no
why: the wait and last channel-request, channel-response and process-exit log fields are the channel's own records, not named by the live-session or PTY scope.

@answer n=9 entity=span-trace-coverage#7.4 v=no
why: the wait.block cleanup on response, connection error or refusal is the client's wait path, which no chunk item reaches.

@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog scenario's must-trace spans (hook.handle, run.dialog_register, run.dialog_await, hook.decision_emit) are the lines item 12 names as the evidence of the live dialog-hold read.

@answer n=11 entity=span-trace-coverage#8.2 v=yes
why: run.dialog_await's deadline_ms bounds (60 000 to 3 600 000 ms, the dialog_hold_s hold) and deadline_hit are the value the live read near the 3600 s ceiling measures.

@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: the dialog hook's hook-invoked written only after the hook.dialog response, and the rule that question, answer and tool input never appear, are the evidence item 12 names for the live read.

@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the hold's end (dialog-expired, dialog-answered with deadline_hit) and the hook's DIALOG_HOLD_MAX + 5 s (3605 s) backstop are the ceiling behaviour the live dialog-hold read observes.

@answer n=14 entity=span-trace-coverage#9.1 v=no
why: the pause and release wheel-transition spans are the human-wheel path, which the chunk neither changes nor names.

@answer n=15 entity=span-trace-coverage#9.2 v=no
why: run.wheel_transition's wheel_from, wheel_to and cause attributes are the wheel's own span, outside the chunk's scope.

@answer n=16 entity=span-trace-coverage#9.3 v=no
why: the send-refused human-typing, release-from-driver and budget-gate log lines are wheel and budget records no chunk item names.

@answer n=17 entity=span-trace-coverage#9.4 v=no
why: run.wheel_transition is a point-in-time span and the wheel product event is an audit record, both on a path the chunk does not reach.

@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the budget governor's hook.statusline, state.budget_write and statusline.shell_out spans are excluded by the chunk's not-needed item 2, which says no change touches the statusline shell-out, budget.json or budget-gate.

@answer n=19 entity=span-trace-coverage#10.2 v=no
why: the budget span attributes (five_hour_pct, shell_exit_status, trigger, window) belong to the budget governor that the not-needed item 2 excludes.

@answer n=20 entity=span-trace-coverage#10.3 v=no
why: the statusline hook-invoked, parse-rejected and hook-decision log fields are the budget governor's lines, excluded by the not-needed item 2.

@answer n=21 entity=span-trace-coverage#10.4 v=no
why: the statusline.shell_out bound (STATUSLINE_DEADLINE) and its fail-open cleanup are the budget governor's shell-out, excluded by the not-needed item 2.

@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: the E2 strip's rule that process-start{subject:"claude-child"} carries names only and never a CLAUDE* value is the rule item 11 names for the live real-claude session's diagnostics, and its IDENTITY_FLOOR is the Windows-measured constant the remnant review reaches.

@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the E3 implicit link record inside send and answer is the driven wrapper's link set, which the chunk does not change and no needs item names.

@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the explicit link and unlink methods add no channel method or verb in the chunk, as not-needed item 3 states.

@answer n=25 entity=span-trace-coverage#14.1 v=no
why: the E5 snapshot-replay state-recovered lines and their spans are a state-store reader the chunk does not touch, as not-needed item 5 states.
