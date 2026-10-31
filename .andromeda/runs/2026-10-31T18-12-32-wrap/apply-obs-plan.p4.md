# Apply — obs-plan · part 4 of 4

## Verdicts
- fanout-obs-plan.md 2 · applied · .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:25#1
- fanout-obs-plan.md 4 · applied · .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:20#1
- fanout-obs-plan.md 3 · applied · .andromeda/registries/contracts/obs-plan/logging-stack.md:12#1

## Edits
- line: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:25
  old: «{"v":1,"error":"instance-unreachable","detail":null}»
  new: «{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}»
- line: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:20
  old: «(`already-live`, `no-session`, `session-live`, `cwd-missing`)»
  new: «(`already-live`, `stale-heartbeat`, `no-session`, `session-live`, `cwd-missing`)»
- line: .andromeda/registries/contracts/obs-plan/logging-stack.md:12
  old: «(for example `detail:"already-live"`, Scenario 1)»
  new: «(`detail:"already-live"` for a live holder or a fresh replayed beat, Scenario 1; `detail:"stale-heartbeat"` for a `stale` holder)»

## Patterns
[[pattern]]
id = "obs4-01"
fixed = 'the fixed instance-state pair, or `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "obs4-02"
fixed = 'exit-1 refusals (`already-live`, `no-session`'

[[pattern]]
id = "obs4-03"
fixed = 'exit-1 `process-exit` (for example `detail:"already-live"`'
