# Apply — architecture · part 1 of 2

## Verdicts
- fanout-architecture.md 3 · already stands · .andromeda/architecture.md:157–157 · «for the server check every frame-writing client makes before its first frame»
- fanout-architecture--cascade-new-standard-contracts.md 1 · already stands · .andromeda/architecture-detail/standard-contracts.md:93–93 · «`LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call»
- fanout-architecture--cascade.md 13 · already stands · .andromeda/architecture-detail/standard-contracts.md:25–25 · «`read_snapshot` keeps its results, and its three product callers»
- fanout-architecture--cascade.md 14 · already stands · .andromeda/architecture-detail/standard-contracts.md:47–47 · «compares the connected stream's peer pid and that process's OS start time»
- fanout-architecture--cascade.md 15 · already stands · .andromeda/architecture-detail/standard-contracts.md:65–65 · «sent id-less by `hook` once its server check has passed»
- fanout-architecture--cascade.md 1 · already stands · .andromeda/architecture-detail/stack-and-technologies.md:39–39 · «through interprocess's `StreamCommon::peer_creds().pid()`»
- fanout-architecture--cascade.md 2 · already stands · .andromeda/architecture-detail/stack-and-technologies.md:45–45 · «sysinfo's process start time is also the observed side of the server check»
- fanout-architecture--cascade.md 3 · already stands · .andromeda/architecture-detail/established-decisions.md:67–67 · «return the connected stream's peer pid through interprocess's `peer_creds().pid()`»
- fanout-architecture--cascade.md 4 · already stands · .andromeda/architecture-detail/established-decisions.md:73–73 · «returns `Matches` with no comparison: the recorded gap»
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:79#1
- fanout-architecture--cascade.md 6 · already stands · .andromeda/architecture-detail/established-decisions.md:110–110 · «The event and dialog paths share one connect»
- fanout-architecture--cascade.md 7 · already stands · .andromeda/architecture-detail/established-decisions.md:101–101 · «when the endpoint is not reached or its server check fails, a direct `try_lock` append»
- fanout-architecture--cascade.md 8 · already stands · .andromeda/architecture-detail/established-decisions.md:110–110 · «gets no frame: the hook records `server-verify-failed` and exits 0 with no body»
- fanout-architecture--cascade.md 10 · already stands · .andromeda/architecture-detail/established-decisions.md:204–204 · «The snapshot's `pid` and `started_at` are also the server identity»
- fanout-architecture--cascade.md 11 · already stands · .andromeda/architecture-detail/conventions.md:40–40 · «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`»
- fanout-architecture--cascade.md 12 · already stands · .andromeda/architecture-detail/conventions.md:53–53 · «returns the `instance-unreachable` `isError` result, text `refused: instance-unreachable`»
- fanout-architecture--cascade.md 16 · already stands · .andromeda/architecture-detail/occupied-resources.md:97–97 · «`detail:"server-verify-failed"` + `during:"connect"`»
- fanout-architecture--cascade.md 17 · already stands · .andromeda/architecture-detail/occupied-resources.md:99–99 · «a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN»
- cascade-compare-architecture.md 1 · already stands · .andromeda/architecture-detail/occupied-resources.md:14–14 · «checks that the process serving it is the snapshot's wrapper»

## Edits
- line: .andromeda/architecture-detail/established-decisions.md:79
  old: «as an absent one (the founder's ruling): every reader»
  new: «as an absent one: every reader»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = "as an absent one (the founder's ruling)"
