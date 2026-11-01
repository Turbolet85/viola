# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · no · the session adds no config key, setting or product environment read; the test helper's TMPDIR read is test-only, outside the product's variable list
cross-cutting-patterns#2.1 · no · no ledger row, probe, stamp or claude dependence moved, and the agents-join read stays unstamped and non-fatal
cross-cutting-patterns#3.1 · no · the new check reads metadata and a peer euid, not external bytes, so no product bounded reader is added and the census claim holds
cross-cutting-patterns#4.1 · yes · add-to · F12, the socket-directory hint, names only the ViolaName with no path, pid or uid and is plain ASCII, so it joins the sanitised surfaces
