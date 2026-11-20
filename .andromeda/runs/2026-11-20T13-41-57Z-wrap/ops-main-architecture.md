@row_retire n=1 row=decision/permission-request-question-stays-null seen=4 heir=decision/permission-request-question-answered detector=D-arch-decisions
why: A question first raised by PermissionRequest is now held for the driver and answered allow with updatedInput, through its own relied ledger row and verify probe.
basis: fct-sbkniw6r
because: The unanswerable rule this row describes was removed, and the plan's Expected amendments retire and replace this row under the founder's W1.
@row_add n=1 row=decision/permission-request-question-answered area=established-decisions fact=F1 detector=D-arch-decisions
name: A question first raised by PermissionRequest is answered
rule: A question first raised by PermissionRequest with no matching armed dialog is held for the driver; the answer returns as PermissionRequest allow with updatedInput, the tool input plus answers and annotations; a human key, an expired hold or an unverified CLI leave it to the human.
why: Ratified to end the rule that left such a question null; the body now has its own relied ledger row and verify probe.
except: Unmeasured: a user argument to viola run selecting the CLI's strict session kind, which refuses a changed input; the human then sees a denied tool call and no dialog.
basis: fct-xnoawzki
because: The decision body maps a PermissionRequest question to allow with updatedInput and the dialog hold no longer leaves it unanswerable; the plan names the strict-kind residual as this row's except.
