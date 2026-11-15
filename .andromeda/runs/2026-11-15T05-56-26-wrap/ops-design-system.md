@row_add n=1 row=absence/status-line-interception area=surface-cli fact=F1 detector=none
capability: Interception of the agent's status line: a wrapped agent's status line is its own, exactly as unwrapped
basis: fct-vey5bl3d
because: The chunk retired the status-line interception whole (settings override, child --settings, source read, shell-out), so the cli surface now holds the absence of that whole capability.
@row_add n=2 row=contract/hook-verb/unknown-event area=surface-cli fact=F2 detector=none
boundary: The hook verb
item: unknown event word
promise: `viola hook` takes nine hook event words; any other word, the retired `statusline` included, is an unknown event: exit 0, nothing read, nothing written.
basis: fct-yeasicmk
because: The `statusline` arm was removed and its word now takes the unknown-event exit, a promise at the hook verb's boundary no standing row states.
