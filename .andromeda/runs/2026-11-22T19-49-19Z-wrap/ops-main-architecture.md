@row_set n=1 row=contract/hook-to-kind-map/permissionrequest cell=promise seen=2 detector=none
value: PermissionRequest maps to `permission`, to `question` for an AskUserQuestion call with no matching armed dialog, or to `plan` for such an ExitPlanMode call (unmeasured live), except one repeating the armed `question` / `plan` dialog's tool with an equal `tool_input`: a continuation, no event.
basis: fct-xo6tfs76
because: The built map also raises an ExitPlanMode PermissionRequest with no matching armed dialog as its own plan dialog, and the operator's word names that arm beside the question arm, marked unmeasured live, with no behaviour change.
@row_set n=2 row=contract/human-hint-lines/paste-cut cell=promise seen=1 detector=none
value: Reads "<name> did not take the whole text in time; a human must look at the session before the next send".
basis: fct-xo6tfs76
because: The operator's word lifts the provisional clause from the paste-cut hint promises; the built words stand unchanged.
