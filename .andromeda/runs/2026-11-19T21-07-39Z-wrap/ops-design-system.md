@row_add n=1 row=contract/send-readback/context area=brand-identity fact=F1 detector=none
boundary: Send readback
item: /context
promise: On a verified CLI a `/context` send counts, and reads back, only once one new local-command stdout tag appears in the session transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads `unconfirmable`, exit 0.
basis: fct-62lgvvsj
because: `/context` became a listed local command with its own post-condition, a new promise of the send readback beside `/clear`, `/compact` and `/mcp`.
@row_set n=1 row=contract/send-readback/unconfirmable cell=promise seen=2 detector=none
value: A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.
basis: fct-62lgvvsj
because: `/context` now reads `unconfirmable` in exactly those two cases, and the row that closes the list of unconfirmable sends must name them.
@row_set n=2 row=contract/cli-verification/verified cell=promise seen=1 detector=none
value: A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.
basis: fct-tsyju4v4
because: A stamp of the earlier relied rows alone now reads unverified, so being stamped by `viola verify` is no longer enough to read `verified`.
@row_set n=3 row=contract/cli-verification/unverified-cli cell=promise seen=1 detector=none
value: A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
basis: fct-xj7egmxk
because: A version stamped before the relied set grew reads unverified until re-stamped, so the unverified state is no longer only an unstamped build's.
@row_none n=1 fact=F2 type=5
what: Run B's eighth compiled paste is the verify probe inventory the code and architecture hold; its row reading `pass` or `fail` on the step line is already said by contract/viola-verify-output/step-line.
