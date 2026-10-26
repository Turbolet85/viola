# New groups — obs-plan · span-trace-coverage

## Verdicts
- fanout-obs-plan--cascade.md 7 · distinct · no standing group names `cli.link` / `cli.unlink` as dispatch spans (line 8's CLIENT list and line 18's verb spans hold neither; line 18's generic `cli.<verb>` names no verb)
- fanout-obs-plan--cascade.md 11 · contradiction · lines 177–177 · «neither is served (both answer `-32601`)» · the standing E3 says the explicit `link` / `unlink` methods are not served and answer `-32601`, where the proposal has the driven wrapper serving them with name validation and `-32602` / `-32603` errors
- fanout-obs-plan--cascade.md 12 · contradiction · lines 177–177 · «The `method:"link"|"unlink"` pair above belongs to the explicit methods, which are not built» · the standing E3 ties the `method:"link"|"unlink"` channel pair to methods it says are not built, where the proposal makes that pair the trace of served methods that open no span and write no line
- fanout-obs-plan--cascade.md 13 · contradiction · lines 177–177 · «no product path appends an `unlink` line» · the standing E3 says no product path appends an `unlink` line, where the proposal has `unlink` append one when a pair is removed
