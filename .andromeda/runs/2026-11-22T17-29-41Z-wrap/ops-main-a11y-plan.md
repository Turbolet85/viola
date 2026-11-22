@row_set n=1 row=contract/tui-passthrough/own-bytes cell=promise seen=1 detector=none
value: viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.
basis: adaptation:4
because: A start with verify_new_cli on writes its automatic-verify lines and lets verify's output through ahead of the child's bytes, so the zero-bytes promise must name them.
@row_set n=2 row=decision/zero-viola-bytes-on-the-terminal cell=except seen=1 detector=none
value: A keyed start's automatic-verify lines and verify's own output, which precede the child's bytes.
basis: adaptation:4
because: The outer-PTY stream of a start with verify_new_cli on holds its automatic-verify lines and verify's output ahead of the child's bytes, a named exception to the zero-bytes rule.
@row_set n=3 row=decision/wheel-ignores-terminal-events cell=except seen=1 detector=none
value: Before the child starts, a keyed start's automatic-verify lines and verify's own output reach the human's terminal.
basis: adaptation:4
because: A start with verify_new_cli on writes its automatic-verify lines and lets verify's output through before the child starts, an exception to zero originated bytes the row must name.
@row_set n=4 row=decision/portable-pty-outer-pty-driver cell=except seen=1 detector=none
value: The checks hold from the child's start: before it, a keyed start's automatic-verify lines and verify's own output reach the terminal, and a key waits in the terminal's own buffer.
basis: adaptation:4
because: Before the child starts, a start with verify_new_cli on writes its own lines and runs verify on null stdin, so the boundary checks hold only from the child's start.
@row_set n=5 row=decision/time-bounds-fall-on-the-driver cell=rule seen=5 detector=none
value: Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
basis: adaptation:4
because: The compiled paste bound now falls on the driver's send and the start's verify stop grace on viola's own child; the bound holds paste write by name with every word kept, and the stop grace stands in its own row.
@row_set n=6 row=decision/time-bounds-never-on-human cell=rule seen=5 detector=none
value: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
basis: adaptation:4
because: The compiled paste bound now falls on the driver's send and the start's verify stop grace on viola's own child; the bound holds paste write by name with every word kept, and the stop grace stands in its own row.
@row_set n=7 row=decision/critical-path-dialog-answer cell=rule seen=1 detector=none
value: A pending dialog answered with viola answer (question, permission, plan, and a question first raised by PermissionRequest), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
basis: adaptation:6
because: A question first raised by PermissionRequest is held and answered by the driver like any dialog, so the dialog-answer critical path names it.
@row_add n=1 row=decision/start-verify-stop-grace-off-the-human area=cognitive-accessibility detector=none
name: Start's verify stop grace never falls on the human
rule: The start's verify stop grace, the wait before a stopped start's verify child is killed, falls on viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
basis: adaptation:4
because: A stop during a start's automatic verify waits a grace on the verify child before its kill, a time bound the two time-bound lists cannot hold beside paste write within their bound, so it stands in its own row.
@row_add n=2 row=decision/compiled-shapes-over-signatures area=11-a11y-anti-patterns-never-do-these detector=none
name: Compiled shapes over the screen signatures
rule: The compiled shapes the rendered `claude` TUI screen is parsed for over its compiled signatures are a modal label row's option shape and the box's rows below it; no row text is kept.
basis: adaptation:5
because: The screen-parsing rule names the compiled shapes but its bound cannot hold what they are with every member whole, so that reading stands in its own row.
