@row_set n=1 row=contract/cli-refusal/unable-line cell=promise seen=2 fact=F1 detector=none
value: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola adopt`'s three own refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
basis: fct-x4brbvla, fct-p367ntgk
because: viola adopt gained a third own refusal, not-resumed, with its fixed unable:/hint: pair, so the row's count of adopt's own refusals moves from two to three.
@row_set n=2 row=contract/cli-fixed-message/form cell=promise seen=2 detector=none
value: viola revive's exit-1 refusals, viola adopt's strict-modes-failed, session-live and not-resumed refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
basis: fct-x4brbvla
because: adopt's new not-resumed refusal prints the same one-line unable:/hint: form, so the row naming adopt's fixed-message refusals must name it too.
@row_set n=3 row=contract/start-json-refusal/document cell=promise seen=2 fact=F2 detector=none
value: Every refusal of the start revive and adopt share with run, and adopt's own strict-modes-failed, session-live and not-resumed, gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
basis: fct-x4brbvla, fct-blcs6o5x
because: under --json adopt's not-resumed refusal is exactly {"v":1,"error":"not-resumed","detail":null} with stderr empty, so the row listing adopt's own document-giving refusals must include it.
@row_add n=1 row=contract/tui-passthrough/adopt-not-resumed-end area=a11y-scope-summary fact=F3 detector=none
boundary: TUI passthrough
item: a not-resumed viola adopt's end
promise: A `viola adopt` whose child ended without its session resuming writes its refusal pair, or under `--json` its document, only after the child ended and released the terminal; `viola run` and `viola revive` write nothing past the child, and a pump error under adopt still prints no document.
basis: fct-x4brbvla, fct-j2uatzhk
because: adopt's not-resumed end now writes after pump_end returns, the one named exception to viola writing nothing past a child that held the terminal, and the standing terminal-bytes row cannot take it within its bound.
@row_set n=4 row=decision/wheel-ignores-terminal-events cell=except seen=2 detector=none
value: Before the child starts, a keyed start's automatic-verify lines and verify's own output reach the human's terminal; after the child ends, a not-resumed viola adopt's refusal pair or document does.
basis: fct-x4brbvla, fct-j2uatzhk
because: the rule that viola originates zero bytes on the human's terminal now has a second named exception, adopt's not-resumed end written after its child released the terminal.
