@row_set n=1 row=decision/dialog-hold cell=state seen=1 detector=D-arch-decisions
value: standing
basis: fct-ur4aozib, fct-rbnf2n7i
because: The 3600 s ceiling is now measured live at the ceiling itself and ruled to stand, so the provisional mark leaves the dialog hold decision.
@row_set n=2 row=decision/dialog-hold-bound cell=state seen=1 detector=D-arch-decisions
value: standing
basis: fct-ur4aozib, fct-rbnf2n7i
because: The 3600 s ceiling is now measured live at the ceiling itself and ruled to stand, so the provisional mark leaves the dialog hold bound decision.
@row_set n=3 row=decision/wheel-starts-with-driver cell=rule seen=1 fact=F1 detector=none
value: The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key, a key the human types into the folder-trust dialog before the session starts included.
basis: fct-k3ry5p3p, fct-rdvnj325
because: Trust keys passed through the wrapper take the wheel before session-start, so the row that says when the driver's start-time wheel ends must name that case.
@row_set n=4 row=contract/dialog-hold-end/null-reply cell=promise seen=1 fact=F2 detector=none
value: At the hold's end the wrapper replies `null` and the hook exits 0 with no body; the expiry ends only the hook's wait, the CLI's dialog staying drawn for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
basis: fct-cffasnck
because: Entry 12 read the question dialog still drawn after dialog-expired, so the promise at the hold's end must say the expiry ends the wait, not the CLI's dialog.
@row_none n=1 fact=F3 type=4
relied: a send after a human erase relies on the real CLI emptying its input box on DEL, a behaviour the fake agent does not reproduce, so only a live run reads it.
