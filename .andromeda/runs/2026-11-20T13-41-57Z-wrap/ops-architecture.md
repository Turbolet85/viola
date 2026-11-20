@row_retire n=1 row=decision/permission-request-question-stays-null seen=4 heir=decision/permission-request-question-answered detector=D-arch-decisions
why: A question first raised by PermissionRequest is now held for the driver and answered allow with updatedInput, its relied ledger row and probe landed and the versions re-stamped.
basis: fct-sbkniw6r
because: The unanswerable rule the row describes was removed and the plan's Expected amendments retire and replace this row.
@row_add n=1 row=decision/permission-request-question-answered area=established-decisions fact=F1 detector=D-arch-decisions
name: A question first raised by PermissionRequest is answered
rule: A question first raised by PermissionRequest with no armed PreToolUse is held as a pending dialog; the driver's answer returns as PermissionRequest allow with updatedInput, the tool's input plus answers and annotations; a human key, an expired hold or an unverified CLI leave it to the human.
why: A PermissionRequest allow with updatedInput satisfies the question, now backed by its own relied ledger row and verify probe.
basis: fct-xnoawzki
because: The decision body maps PermissionRequest plus Question to allow with updatedInput and the run's dialog hold no longer leaves such a question unanswerable, so the replaced row's decision must stand under its heir.
@row_add n=2 row=decision/permission-request-question-ledger-row area=established-decisions fact=F2 detector=none
name: Permission-request-question ledger row and its probe
rule: The permission-request-question relied ledger row is probed by verify's Run C pasting a compiled prompt that raises a question through PermissionRequest, answered by the closed probe id permission-question-first-option; the row passes only when the answer lands in the tool's input.
basis: fct-xnoawzki
because: A new relied ledger row and its Run C probe landed, and each relied row with its probe stands as its own decision beside the other ledger-row decisions.
@row_set n=1 row=contract/hook-contract/permission-request cell=promise seen=1 detector=none
value: PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`; for a question the decision is `{"behavior":"allow","updatedInput":<the tool's input plus answers and annotations>}`.
basis: fct-u5kjf7gw
because: The decision-body shape for a PermissionRequest question is new, and the Expected amendments name this contract as gaining the question's decision form.
@row_set n=2 row=contract/answer-refusals/unknown-dialog cell=promise seen=1 detector=none
value: A late `answer`, to a dialog already answered or after its hold expired, is refused `unknown-dialog`.
basis: fct-njitszmf
because: A question first raised by PermissionRequest is now held and answerable, so only a late answer or one after an expired hold is refused unknown-dialog, as the Expected amendments carry.
@row_set n=3 row=contract/hook-to-kind-map/permissionrequest cell=promise seen=1 detector=none
value: PermissionRequest maps to `permission`, or to `question` for an AskUserQuestion call with no armed PreToolUse dialog; one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input` is that dialog's continuation and raises no event.
basis: fct-xnoawzki
because: A question first raised by PermissionRequest with no armed PreToolUse is now held as a dialog of its own, so the map must name that arm beside permission and the continuation.
@row_set n=4 row=decision/verify-never-types-into-dialog cell=name seen=1 detector=none
value: viola verify never types into a CLI dialog
basis: fct-xnoawzki
because: Verify's Run C now answers a question through a closed probe id's hook body, so the name saying verify never answers a CLI dialog is false while its rule, no byte typed into a dialog, still holds.
