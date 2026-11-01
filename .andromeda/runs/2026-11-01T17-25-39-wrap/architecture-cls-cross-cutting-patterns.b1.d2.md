# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · no · the session adds no config key and reads no new variable, and hook's instance check still runs before the config read; the new socket-dir check sits in connect after hook-invoked, so the claim holds
cross-cutting-patterns#2.1 · no · no claude behaviour is depended on and viola-agent-claude is not reached, so the ledger rule and the no-stamp join read stand
cross-cutting-patterns#3.1 · no · the new socket-dir check reads metadata only and adds no byte reader, so the census and its BOUNDARIES claim stand
cross-cutting-patterns#4.1 · yes · add-to · F12: the new socket-dir unable/hint pair names only the ViolaName, with no path, pid or uid, and joins the no-path error surfaces
