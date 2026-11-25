@row_add n=1 row=contract/viola-verify-refusals/untrusted-folder area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola verify refusals
item: untrusted folder
promise: When Claude met the trust or imports form at the first screen of Runs B to E: exit 1, code `untrusted-folder`, `unable: claude did not start ready in this folder: it is not trusted` / `hint: run viola verify from a folder you trust in Claude Code`, no step line, no summary, no stamp.
basis: fct-yf2imaoq, fct-py27nrcw, fct-r7kjslfe
because: verify gains a new refusal pair with its own code, a sibling of the stopped refusal, its wording provisional on a delegate's word for the epoch batch, and no standing row states it.
@row_set n=1 row=contract/viola-verify-output/failing-row cell=promise seen=1 fact=F2 detector=none
value: A run with a failing row exits 1 and still writes the stamp; a run whose first screen read trust or imports is a refusal before any row is read, not a failing row, and writes no stamp.
basis: fct-yf2imaoq, fct-py27nrcw
because: a confounded run is now a refusal that stamps nothing while a failing relied row still stamps, so the failing-row promise must name where it stops.
@row_add n=2 row=decision/placing-start-keeps-the-view area=surface-cli fact=F4 detector=none
name: A placing start keeps the view
rule: A placing start (viola open, viola up, viola revive --origin, viola adopt --origin) leaves the active workspace unchanged, sending the view back once when the move switched it, with no new placement word, field or line.
guard: window_step_restores_the_active_workspace_the_move_took
basis: fct-yf2imaoq, fct-i4yzedmo, fct-r7kjslfe
because: the operator's answer makes every placing start leave the user's view where it was, a decision the cli surface rows do not yet state.
@row_none n=1 fact=F3 type=4
relied: the automatic verify at a start reads the untrusted-folder refusal as no stamp entry, printing the start's existing no-stamp unstamped line and leaving the version un-attempted; the code holds it, its branch unchanged.
