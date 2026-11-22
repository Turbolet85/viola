@row_set n=1 row=decision/time-bounds-never-on-human cell=rule seen=5 detector=none
value: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
basis: fct-shrbvdzl
because: The paste write now ends within the compiled PASTE_BOUND, a new time bound that falls on the driver's send, so the row's list of bounds must name it.
@row_set n=2 row=decision/time-bounds-fall-on-the-driver cell=rule seen=5 detector=none
value: Every time bound (MCP wait cap and call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-shrbvdzl
because: The paste write now ends within the compiled PASTE_BOUND, a new time bound that falls on the driver's send, so the row's list of bounds must name it.
@row_set n=3 row=decision/human-keystrokes-never-blocked cell=rule seen=2 fact=F2 detector=none
value: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or dropped: it shares one write queue with the paste in arrival order and, typed during a paste, waits behind at most the in-flight chunk of 4096 bytes and the closing marker.
basis: fct-z6owtgsx
because: The shared writer lock and its one-write atomic paste gave way to one writer thread with a bounded queue in arrival order, so a key now waits behind at most one chunk and the closing marker rather than the whole atomic paste.
@row_set n=4 row=contract/tui-passthrough/human-keystrokes cell=promise seen=2 detector=none
value: Human keystrokes are never blocked, refused or dropped once the wrapped child holds the terminal: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker; the one exception is a start's automatic verify window, before the child starts.
basis: fct-z6owtgsx
because: The paste is no longer one atomic write under a shared lock but chunks on one queued writer thread, so the promise's bound on a key's delay is the in-flight chunk and the closing marker, not the current atomic paste.
@row_add n=1 row=decision/paste-cut-mark-holds-sends area=cognitive-accessibility fact=F1 detector=none
name: Paste-cut mark holds later sends
rule: After a cut paste every later send is refused not-delivered / paste-cut with nothing typed until a human editing key moves the wheel or a prompt-submitted, session-start or session-end record clears the mark; a release with no key or a turn end leaves the mark, held in memory only.
basis: fct-z6owtgsx
because: The chunk adds the cut mark, which refuses every later send until a human looks at the session, a decision no standing row of the area says.
@row_add n=2 row=decision/cut-paste-never-submitted area=cognitive-accessibility fact=F3 detector=none
name: A cut paste is never submitted
rule: A paste cut at its bound closes its bracket with no Enter, so a partial text is never submitted for the human; the paste bound is compiled and no config key, env var or flag sets it.
basis: fct-wxrpelpd
because: The chunk bounds the paste and closes a cut paste with no CR, a decision no standing row of the area says.
