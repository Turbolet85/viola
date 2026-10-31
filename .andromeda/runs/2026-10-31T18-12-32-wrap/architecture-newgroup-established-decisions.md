# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade-g1.md 2 · contradiction · lines 124–124 · «a refusal is the `instance-unreachable` result with `detail: null`» · the standing line gives a refused `check_instance` a null detail, the proposal gives it the cause `StrictModes`, and it adds after the line without changing it
- fanout-architecture--cascade-g1.md 3 · distinct · no group names the codes `viola verify` writes on its `process-exit`; line 249's lineless home refusal agrees with the proposal
- fanout-architecture--cascade-g1.md 4 · contradiction · lines 305–305 · «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed`» · revive's listed preflight codes hold `already-live` and no `stale-heartbeat`, while line 255 has revive refuse a stale name through the same collision check, which the proposal makes print `stale-heartbeat`
- fanout-architecture--cascade-g1.md 5 · distinct · no group names `without_endpoint` or its split of a missing endpoint into `InstanceDead` and `UnwrappedName`
- cascade-compare-architecture.md 3 · contradiction · lines 305–305 · «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed`» · the same conflict as fanout-architecture--cascade-g1.md 4, which states the same fact: revive's listed preflight codes hold no `stale-heartbeat`, while line 255 sends a stale name through the same collision check
- cascade-compare-architecture.md 4 · distinct · line 75 names `instance-unreachable` with no detail, so `endpoint-lost` extends that sentence without opposing it, and fanout-architecture--cascade-g1.md 2 gives the channel mapping but not this `wait` outcome
