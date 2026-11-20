@row_add n=1 row=contract/viola-run-auto-verify/stderr-lines area=1-a11y-scope-summary fact=F2 detector=none
boundary: viola run start automatic verify output
item: stderr lines L1 to L5
promise: L1 verifying, L2 did not verify, L3 earlier red, L4 verified elsewhere, L5 no stamp with spawn-failed, unstamped or lock-refused: each one stderr write, ASCII, no SGR, only the parsed version and closed word; with verify's output they precede the child's bytes; none under revive --json.
basis: fct-zfntgo5a
because: The chunk adds the start's automatic verify lines as a new fixed human output surface on stderr, and no row of a11y-plan states their discipline.
@row_add n=2 row=contract/config-json/verify-new-cli area=cognitive-accessibility fact=F3 detector=none
boundary: config.json
item: verify_new_cli
promise: A JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line; v must be 1; turns the start's automatic verify on and lifts no gate by itself.
basis: fct-xgc7qjiw
because: The chunk adds a config.json key beside dialog_hold_s whose contract no row of a11y-plan carries.
@row_set n=1 row=decision/human-keystrokes-never-blocked cell=except seen=1 fact=F1 detector=none
value: a run or revive start's automatic verify window (verify_new_cli on, a claude version with no stamp entry): no wrapped child yet, verify on null stdin; it ends with verify, before the child starts
basis: fct-uidyyps4
because: The start's automatic verify opens one named window before the wrapped child exists, and the decision must name it as its exception.
@row_set n=2 row=contract/tui-passthrough/human-keystrokes cell=promise seen=1 detector=none
value: Human keystrokes are never blocked, refused or delayed past the current atomic paste once the wrapped child holds the terminal; the one exception is a start's automatic verify window, before the child starts.
basis: fct-uidyyps4
because: The start's automatic verify runs before the wrapped child exists with null stdin, so the never-blocked promise holds from the child's start, not across that window.
@row_set n=3 row=contract/tui-passthrough/viola-terminal-bytes cell=promise seen=1 detector=none
value: Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.
basis: fct-uidyyps4
because: A start's automatic verify now writes its lines and verify's output to the terminal before the child's bytes, so zero viola bytes holds only from the child's start.
@row_set n=4 row=contract/tui-passthrough/own-bytes cell=promise seen=1 detector=none
value: viola writes zero bytes to the terminal except the child's output and, before it, a start's automatic verify lines L1 to L5 and verify's own output; a passed viola revive writes no line of its own while the child holds the terminal.
basis: fct-uidyyps4
because: A start's automatic verify now writes its lines and verify's output ahead of the child's bytes, so the zero-bytes promise must name them.
@row_set n=5 row=decision/zero-viola-bytes-on-the-terminal cell=except seen=1 detector=none
value: a start's automatic verify lines L1 to L5 and verify's own output, which precede the child's bytes
basis: fct-uidyyps4
because: The outer-PTY stream of a start that runs its automatic verify now holds viola's L1 to L5 lines ahead of the child's bytes, a named exception to the zero-bytes rule.
@row_set n=6 row=decision/wheel-ignores-terminal-events cell=rule seen=1 detector=none
value: On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal once the wrapped child holds it.
basis: fct-uidyyps4
because: A start's automatic verify writes lines L1 to L5 before the child starts, so zero originated bytes holds only while the child holds the terminal.
@row_set n=7 row=decision/portable-pty-outer-pty-driver cell=rule seen=1 detector=none
value: The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed while the wrapped child holds the terminal.
basis: fct-uidyyps4
because: Before the child starts, a start's automatic verify writes its own lines and runs with null stdin, so the boundary checks hold from the child's start.
@row_set n=8 row=decision/time-bounds-fall-on-the-driver cell=rule seen=4 fact=F4 detector=none
value: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait, the start's verify stop grace) falls on the driver, viola's own child or viola's own start, never delaying a human key and never moving the wheel.
basis: fct-zfntgo5a
because: A stop during a start's automatic verify TERMs the verify child and waits a 30 s grace on it before KILL, a new time bound that falls on the child, never the human.
@row_set n=9 row=decision/time-bounds-never-on-human cell=rule seen=4 detector=none
value: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait, the start's verify stop grace) falls on the driver, viola's child or viola's own start; no key is blocked by it.
basis: fct-zfntgo5a
because: The start's automatic verify adds a 30 s stop grace on the verify child, a time bound the list of bounds must now name.
