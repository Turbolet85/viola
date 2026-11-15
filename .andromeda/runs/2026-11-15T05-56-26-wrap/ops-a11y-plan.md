@row_set n=1 row=decision/time-bounds-never-on-human cell=rule seen=2 detector=none
value: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read) falls on the driver or viola's child; no key is blocked, refused or delayed by it.
basis: fct-vey5bl3d, fct-pkysvchx
because: The status-line deadline and the user's status-line command it bounded are removed whole, so the row's list of time bounds must drop them.
@row_set n=2 row=decision/time-bounds-fall-on-the-driver cell=rule seen=2 detector=none
value: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-vey5bl3d, fct-pkysvchx
because: The 5 s status-line deadline is removed with the viola hook statusline arm, so the row's list of time bounds must drop it.
@row_retire n=1 row=decision/statusline-output-passed-byte-for-byte seen=1 detector=none
why: viola no longer runs or passes through the user's status-line command; the viola hook statusline arm and its passthrough case are removed.
basis: fct-vey5bl3d
because: The status-line interception is retired whole (arm, shell-out, TUI statusline passthrough case), so the decision has no subject left.
@row_retire n=2 row=contract/viola-hook-statusline/exit seen=1 heir=contract/viola-hook-event-word/statusline detector=none
why: viola hook statusline no longer runs a command under a deadline; the word is now an unknown hook event.
basis: fct-yeasicmk
because: The statusline arm and its 5 s deadline are removed and the word takes the unknown-event exit, so the contract's promise is false and its heir states the new one.
@row_add n=1 row=decision/no-settings-layer-on-child area=keyboard-test-harness fact=F1 detector=none
name: No settings layer on the child
rule: viola adds no settings layer to the child and writes nothing into its status line, so a wrapped session's status line is the user's own, exactly as unwrapped.
basis: fct-6hiffgpi, fct-vey5bl3d
because: The per-session settings override and the child's --settings are removed, and the wrapped status line now equals the unwrapped one by construction.
@row_add n=2 row=contract/viola-hook-event-word/statusline area=cognitive-accessibility fact=F2 detector=none
boundary: viola hook event words
item: statusline
promise: statusline is no hook event: the word takes the unknown-event exit 0, reading and writing nothing, and holds no deadline.
basis: fct-yeasicmk, fct-ozynmuep
because: The statusline arm is removed and the retired word now maps to no hook event, a promise at the hook boundary that replaces the retired exit contract.
@row_add n=3 row=absence/status-line-interception area=1-a11y-scope-summary fact=F3 detector=none
capability: Status-line interception: viola runs no shell and no status-line command, timed or passed through
basis: fct-vey5bl3d
because: The status-line interception and its /bin/sh -c shell-out are retired whole, leaving viola with no shell run at all.
