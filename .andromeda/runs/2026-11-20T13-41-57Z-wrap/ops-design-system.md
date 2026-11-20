@row_add n=1 row=decision/permission-request-question-held area=surface-cli fact=F1 detector=none
name: PermissionRequest question held for an answer
rule: A question first raised by a PermissionRequest is held as a pending dialog and answered by `viola answer` like any dialog; only a human key, an expired hold or an unverified CLI leave it to the human.
basis: fct-xnoawzki
because: The chunk removed the rule that left a PermissionRequest question to the human, so the CLI surface now holds it for `viola answer` and the master must state that decision.
