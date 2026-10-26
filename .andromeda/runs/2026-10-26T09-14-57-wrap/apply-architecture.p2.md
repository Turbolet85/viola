# Apply — architecture · part 2 of 2

## Verdicts
- fanout-architecture.md 4 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:67#1
- fanout-architecture.md 6 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:32#1
- fanout-architecture.md 5 · applied · .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5#1

## Edits
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:67
  old: «liveness-only endpoint read (`liveness::live_endpoint`, shared by the CLI frame verbs and viola-mcp's frame tools), the»
  new: «live-endpoint read (`liveness::live_endpoint` → `LiveEndpoint`: the endpoint, pid and start time from one snapshot read, shared by the CLI frame verbs and viola-mcp's frame tools), the server identity check (`ServerVerdict`, `server_verdict`, `verify_server`: the recorded pid and its start time against the connected server's, made before the first frame by the CLI frame verbs, `viola hook` and viola-mcp's frame tools), the»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:32
  old: «client.rs (a helper, no subcommand): the channel client send / wait / last share;»
  new: «client.rs (a helper, no subcommand): `connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens (the live endpoint, then the server identity check, then the client), the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`, and the channel client those verbs share;»
- line: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5
  old: «sysinfo (the pid + start-time liveness check, shared by CLI `list` and `mcp`; `liveness::live_endpoint(instance_dir)`, moved unchanged from `src/cmd/client.rs` at chunk 2026-10-17-mcp-server-for-drivers, is the liveness-only endpoint read shared by CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools)»
  new: «sysinfo (the pid + start-time liveness check, shared by CLI `list` and `mcp`, and the start-time read of the server identity check `liveness::verify_server`, made before the first frame by CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook` and `viola-mcp`'s frame tools; `liveness::live_endpoint(instance_dir)` returns `LiveEndpoint` (the endpoint, pid and start time) from one snapshot read, shared by those CLI verbs and `viola-mcp`'s frame tools)»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = 'liveness-only endpoint read'
ignore_case = true

[[pattern]]
id = "arch2-02"
fixed = 'channel client send / wait / last share'

[[pattern]]
id = "arch2-03"
fixed = 'is the liveness-only endpoint read shared by'
