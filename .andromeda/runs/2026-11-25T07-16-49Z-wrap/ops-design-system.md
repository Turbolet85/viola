@row_set n=1 row=contract/auto-verify-line/did-not-verify cell=promise seen=1 detector=D-design-cli-wording
value: A start whose automatic verify ran and left a stamp that is not green prints the fixed did-not-verify line naming the parsed claude version.
basis: fct-yf2imaoq, fct-froyousk
because: An automatic verify refused for an untrusted folder leaves no stamp and prints the no-stamp `unstamped` line, so the did-not-verify line belongs only to a run that left a stamp that is not green.
@row_set n=2 row=contract/auto-verify-line/no-stamp cell=promise seen=1 fact=F2 detector=D-design-cli-wording
value: A start whose automatic verify left no stamp, a verify refused for an untrusted folder included, prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`; the version stays unattempted.
basis: fct-yf2imaoq, fct-froyousk
because: The untrusted-folder refusal of the automatic verify now ends in the no-stamp `unstamped` line with the version left unattempted, which the row must say.
@row_add n=1 row=contract/viola-verify-refusals/untrusted-folder area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola verify refusals
item: untrusted-folder
promise: When any of verify's probe Runs B to E first reads Claude Code's trust form or imports form: `unable: claude did not start ready in this folder: it is not trusted` then `hint: run viola verify from a folder you trust in Claude Code`, exit 1, no step line, no summary, `stamps.json` untouched.
basis: fct-yf2imaoq, fct-froyousk
because: The chunk adds a new fixed refusal pair to `viola verify` (wording provisional, delegate overseer, for the epoch batch) that no design-system row states.
@row_add n=2 row=contract/log-detail-codes/untrusted-folder area=surface-cli fact=F3 detector=none
boundary: log detail codes
item: untrusted-folder
promise: Written to the log as the `process-exit` detail by `viola verify` at its refusal for a folder Claude Code does not trust.
basis: fct-wnswac6i, fct-vmrlleio
because: The `process-exit` detail codes gain `untrusted-folder`, a code at the log boundary no row names.
@row_add n=3 row=decision/placing-keeps-the-view area=surface-cli fact=F4 detector=none
name: Placing keeps the view
rule: A placing verb (`open`, `up`, `revive --origin`, `adopt` under `--origin`) whose move of its own window changed the focused monitor's active workspace returns to the earlier one once; the placement word is set by the move alone, and a failed read or return changes no word, field or line.
guard: window_step_restores_the_active_workspace_the_move_took
basis: fct-yf2imaoq, fct-gs7z5go5
because: The window step now returns the view to the earlier active workspace after its move, a behaviour every placing verb carries that no row states.
