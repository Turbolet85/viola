@row_set n=1 row=decision/dialog-answer-mapping cell=rule seen=1 fact=F1 detector=none
value: Questions: allow with updatedInput answers and annotations, through PreToolUse, or PermissionRequest where first raised there; permissions: PermissionRequest allow, or deny with a message; plans: approved only via PreToolUse allow with the input unchanged, revised via PermissionRequest deny.
basis: fct-xnoawzki, fct-njitszmf
because: A question first raised by PermissionRequest is now held and answered with a PermissionRequest allow body carrying updatedInput, so the mapping that names only PreToolUse for questions must say the second route.
@row_set n=2 row=contract/hook-decision-body/question cell=promise seen=1 detector=none
value: A question is answered allow plus updatedInput.answers, annotations included: through PreToolUse, or through PermissionRequest when the question is first raised there.
basis: fct-xnoawzki, fct-u5kjf7gw, fct-njitszmf
because: The decision body for a question now has a PermissionRequest form, so the promise that names PreToolUse alone is incomplete.
@row_set n=3 row=contract/hook-decision/question cell=promise seen=1 detector=none
value: PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included; a question first raised by PermissionRequest takes PermissionRequest decision.behavior "allow" with updatedInput, the tool input plus answers and annotations.
basis: fct-xnoawzki, fct-u5kjf7gw, fct-njitszmf
because: The hook now emits a PermissionRequest allow body with updatedInput for a question first raised there, a body shape the promise must name.
@row_none n=1 fact=F2 type=5
what: one relied ledger row's own probe id, prompt and Run C order — an item of the ledger's inventory the code holds; the standing rule that every ledger row has a verify probe already covers it.
