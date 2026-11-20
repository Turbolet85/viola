@task_prose part=session
The session narrowed when a dialog label on the child's screen refuses a send: a label quoted inside a transcript row on an idle session no longer refuses viola send; a standing dialog still does.
A dialog now counts as standing only when its label row is shaped as a dialog option (optional selector glyph, optional numbering, the label, nothing after) or when no input-box row stands below it.
The same readiness predicate decides for viola run's pre-send gate, viola verify's paste guard, the ledger's input-box post-condition and the rig's box read; no caller, signature, refusal word, hint text, exit code, log line or schema changed.
Tests added: unit cases for the option shape and the box-below clause, two gate cases, a synthetic quoting idle screen for the fake agent, and one integration send past a quoted label; all green, CI green on the pushed head.
Nothing retired; no count, ledger row, stamp or fixture moved; no live run.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the claude agent - the readiness predicate that reads the child's screen for the input box and for a standing dialog against compiled literals only
@task_file n=2 path=src/run/gate.rs
what: viola run's pre-send readiness gate - the quiet period, the at-once input-not-ready refusal and the unverified-cli arm before a send is issued
@task_file n=3 path=tests/support/fake.rs
what: the fake agent's test support - its synthetic screens, now one idle screen quoting a dialog label above the input box
@task_file n=4 path=tests/cli_send.rs
what: the viola send integration tests - the send readback and refusals over a verified CLI, now one send that goes past a quoted dialog label
@task_item n=1 part=touches
The input-not-ready refusal of a send (refusal hints not-delivered input-not-ready): it now fires only for a standing dialog or a missing input box, no longer for a dialog label quoted in a transcript row; the hint text itself is unchanged.
@task_item n=2 part=touches
The send readback unable form and the viola send readback mirror unable line: fewer sends on an idle session end unable with input-not-ready; the forms are unchanged.
@task_item n=3 part=touches
The rule that nothing announces success before confirmation and no optimistic send confirmation: a send past a quoted label still goes through send-issued, prompt-submitted and send-confirmed before any readback; check it holds as written.
@task_item n=4 part=touches
The dialog-pending cocked strip and the flight-progress strip DIALOG field: a quoted dialog label in a transcript row is not a pending dialog for the send gate; the strip's own dialog reading is unchanged (it reads raw presence by design).
@task_item n=5 part=touches
The CLI refusal line and fixed unable form for a not-delivered send: the same words and exit code, reached in fewer cases.
@task_item n=6 part=touches
viola verify output rows (step line, summary, browser-tools line): verify's paste guard reads the moved predicate; its output words, stamps and settle end are unchanged.
@task_item n=7 part=touches
Rules that the CLI never parses the child's rendered screen content or quotes text: the readiness read stays a boolean over compiled literals and compiled glyphs; it now also reads a row's shape and its position above the input box, keeping no row text.
@task_item n=8 part=touches
The unverified-cli state and its refusal hint: the gate's unverified arm is unchanged; only the verified path's dialog refusal narrowed.
@task_item n=1 part=untouched
Colour, typography, spacing, iconography, brand-identity expression rows: no colour token, glyph, column, width or SGR span changed.
@task_item n=2 part=untouched
viola list board and table rows, the BAY line, NAME cut, skip counts and the WHEEL column: no board output changed.
@task_item n=3 part=untouched
revive, run start, link, unlink, wait, last, hook and mcp verb output and refusal rows: none of those paths changed.
@task_item n=4 part=untouched
exit 21 causes, strict-modes refusals, socket-directory refusals, wrapper fault details and peer reply screening: no exit code, cause or detail word changed.
@task_item n=5 part=untouched
The command names sheet and the absences (rendered page, web surface, tape): no verb or name was taken or freed and no surface added.
@task_item n=6 part=untouched
The self-validation protocol checks (squint, swap, readback box signature, domain design): no visual element changed.
