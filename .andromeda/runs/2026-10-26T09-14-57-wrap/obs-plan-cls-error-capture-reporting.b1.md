# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook and its detail-line capture are untouched (Untouched: panic hooks, not reached), and the new lines go to the cli and hook logs, not the detail sink
error-capture-reporting#2.1 · no · the catch sites and role_of are untouched (Untouched: catch sites, not reached), and the new refusal ends in a logged process-exit as no-session already does, not in a panic or the internal-error path
error-capture-reporting#3.1 · no · the new check sits in the hook connect and the verb and tool spans, not in the pre-init window, and the window's listed checks are unchanged
error-capture-reporting#4.1 · no · the vt100 feed thread in src/run/gate.rs is not reached, and a real wrapper passes the new check before send's request, so the input-not-ready refusal and exit 13 still come from the wrapper
