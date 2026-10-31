# Apply — architecture · part 4 of 4

## Verdicts
- fanout-architecture.md 6 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:32#1, .andromeda/registries/contracts/architecture/project-directory-structure.md:32#2
- fanout-architecture.md 7 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:58#1
- fanout-architecture.md 8 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:67#1

## Edits
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:32
  old: «or the `instance-unreachable` document under `--json`»
  new: «or the `instance-unreachable` document with `detail` `strict-modes-failed` under `--json`»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:32
  old: «the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`, and the channel client those verbs share;»
  new: «the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an `UnreachableCause` through `cause()`) and `CallError`, `unreachable_document`, the one writer of the exit-21 document (the board builds its own in list.rs; exit 1's is `refused_document`), and the channel client those verbs share;»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:58
  old: «`BudgetWindowKind` (`five-hour` · `seven-day`)»
  new: «`BudgetWindowKind` (`five-hour` · `seven-day`), `UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word from `as_str`, no serde path)»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:67
  old: «frame tools), the server identity check»
  new: «frame tools), the no-endpoint read (`liveness::without_endpoint` → `UnreachableCause`: with no live endpoint a directory reads `InstanceDead` and anything else `UnwrappedName`, opening nothing and writing no line, shared by the same callers), the server identity check»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = 'closed `Unreachable` (`instance-dead` · `server-verify-failed`)'
