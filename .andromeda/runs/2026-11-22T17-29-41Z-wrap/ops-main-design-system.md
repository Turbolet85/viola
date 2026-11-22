@row_add n=1 row=decision/permission-request-question-held area=surface-cli detector=none
name: PermissionRequest question held for an answer
rule: A question first raised by a PermissionRequest is held as a pending dialog and answered by `viola answer` like any dialog; a human key, an expired hold or an unverified CLI leave it to the human.
basis: adaptation:6
because: A question first raised by PermissionRequest is held for `viola answer` as built, and the CLI surface master states which dialogs that verb reaches.
