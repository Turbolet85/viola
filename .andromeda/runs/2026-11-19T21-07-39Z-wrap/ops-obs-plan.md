@row_add n=1 row=contract/confirm-window-close/context area=span-trace-coverage fact=F1 detector=none
boundary: run.confirm_window close
item: /context
promise: On a verified CLI, closes on a new local-command-stdout tag in the session transcript within the fallback window, with no prompt submitted; on an unverified CLI, or before the session transcript exists, the send reads unconfirmable.
basis: fct-62lgvvsj
because: /context became a listed local command confirmed by its stdout tag, so the closed list of run.confirm_window closes gains its item beside /clear, /compact and the ordinary send.
@row_none n=1 fact=F2 type=4
relied: verify's Run B pastes /context among its compiled probe literals and reads its stdout tag from the transcript end with no line or span of its own, recording the relied row local-command-context; the ledger and verify code hold it.
