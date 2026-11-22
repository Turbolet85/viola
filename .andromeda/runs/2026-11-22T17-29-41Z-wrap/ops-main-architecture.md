@row_set n=1 row=decision/viola-home-resolution cell=rule seen=1 detector=none
value: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it.
basis: adaptation:4
because: The start's automatic verify child takes the home by --home with VIOLA_DIR removed while the claude child still takes it through VIOLA_DIR alone, so the row's only-through-VIOLA_DIR claim must name each child as built.
@row_set n=2 row=contract/hook-to-kind-map/permissionrequest cell=promise seen=1 detector=none
value: PermissionRequest maps to `permission`, or to `question` for an AskUserQuestion call with no matching armed dialog, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
basis: adaptation:6
because: A question first raised by PermissionRequest with no matching armed dialog is now raised and held as a question of its own, so the map must name that arm beside permission and the continuation.
@row_set n=3 row=decision/verify-never-types-into-dialog cell=name seen=1 detector=none
value: viola verify never types into a CLI dialog
basis: adaptation:6
because: Verify's Run C answers its dialogs through the capture hook's closed probe ids, so the name saying verify never answers a CLI dialog is false while the rule, no byte typed into a dialog, holds.
@row_add n=1 row=decision/permission-request-question-ledger-row area=established-decisions detector=none
name: Permission-request-question ledger row and its probe
rule: The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option; it passes only when that request is raised and the answer reaches the tool's answers.
basis: adaptation:6
because: The relied ledger row and its Run C probe are built, and each relied row with its probe stands as its own decision beside the other ledger-row decisions.
