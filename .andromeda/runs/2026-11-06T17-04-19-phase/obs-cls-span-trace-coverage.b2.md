@answer n=1 entity=span-trace-coverage#5.1 v=no
why: run.pin_copy's placement under a concurrent first start is not named by the chunk; the live run is a single start and the portable-pty bump does not touch the pin
@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the send must-trace chain runs through pty.paste_write, which the scope names (paste as text_bytes only through the PTY seam), and its socket-directory refusal opens no channel.request
@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write's text_bytes and paste_mode attributes are the paste behaviour the scope names, and "never text" is the send-text rule the scope names
@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: send-issued's text_bytes, the one-line-per-side never-the-text rule (named by the send-text obligation), and the client socket-dir-refused process-exit (named socket-dir code) bind
@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: send.client's process-exit list carries exit 21 detail socket-dir-refused, the detail code the chunk ratifies or re-words
@answer n=6 entity=span-trace-coverage#7.1 v=no
why: the wait/last must-trace chain and its check ordering are not named by the scope and carry no socket-dir wording or code
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: wait.block and run.wait_dispatch span attributes (after, timeout_ms, woken_kind) are not touched or named by the chunk
@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: "last_assistant_message never appears" is a send-text-class obligation the scope names (no send text or last_assistant_message in a home-level file), and wait's socket-dir-refused process-exit carries the named code
@answer n=9 entity=span-trace-coverage#7.4 v=no
why: the wait.block cleanup on refusal states span close timing only; it carries neither the socket-dir wording nor its detail code
@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the scope names the dialog → answer scenario as the live hold read's subject, and this entity states that scenario's must-trace path (hook.handle, run.dialog_await, hook.decision_emit)
@answer n=11 entity=span-trace-coverage#8.2 v=yes
why: the hold read at the 3600 s ceiling re-measures dialog_hold_s bounds (60 000–3 600 000 ms, default 60 000) and the deadline_ms and deadline_hit attributes on run.dialog_await
@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: the scope names this scenario's lines as the read's evidence, including hook-invoked written after the hook.dialog response and question, answer and tool input never appearing; its socket-dir-refused lines carry the named code
@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the hold's end (dialog-expired, dialog-answered with deadline_hit true, hook-decision deadline_hit false, backstop 3605 s) is exactly the behaviour the live read at the ceiling exercises
@answer n=14 entity=span-trace-coverage#9.1 v=no
why: the wheel must-trace chain and pause/release check ordering are not named by the scope
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: run.wheel_transition span attributes (wheel_from, wheel_to, cause) are not touched or named by the chunk
@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: pause/release's socket-dir-refused process-exit and channel lines carry the socket-directory detail code the chunk ratifies or re-words
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the wheel cleanup (point-in-time span, wheel event as audit record) is not named by the scope
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the budget governor scenario is marked not-needed (no change touches the statusline shell-out, budget.json or budget-gate)
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: budget governor span attributes sit under the not-needed budget scenario the chunk does not touch
@answer n=20 entity=span-trace-coverage#10.3 v=no
why: statusline hook log fields sit under the not-needed budget and statusline scenario
@answer n=21 entity=span-trace-coverage#10.4 v=no
why: statusline shell-out cleanup and stdout pass-through sit under the not-needed budget and statusline scenario
@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: E2 states process-start{subject:"claude-child"} field content on spawn, which the scope names, and that the strip logs names only and never CLAUDE* values, which the live real-claude diagnostics obligation names
@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the implicit link record on send and answer is not named by the scope and the chunk adds no channel method or cross-surface call
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the explicit link and unlink methods are not named by the scope and the chunk does not touch them
@answer n=25 entity=span-trace-coverage#14.1 v=no
why: E5 snapshot replay and state-recovered lines are not-needed, since no state-store reader or appender is touched
