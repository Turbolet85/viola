# Apply — obs-plan · part 2 of 4

## Verdicts
- fanout-obs-plan--cascade-g2.md 1 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:30#1
- fanout-obs-plan--cascade-g2.md 2 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:33#1
- fanout-obs-plan--cascade-g2.md 3 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:33#2
- fanout-obs-plan--cascade-g2.md 4 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:57#1
- fanout-obs-plan--cascade-g2.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:57#2
- fanout-obs-plan--cascade-g2.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:78+#1
- fanout-obs-plan--cascade-g2.md 7 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:95#1
- fanout-obs-plan--cascade-g2.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:96+#1
- fanout-obs-plan--cascade-g3.md 1 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:112#1
- fanout-obs-plan--cascade-g3.md 2 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:112#2, .andromeda/obs-plan-detail/span-trace-coverage.md:112#3
- fanout-obs-plan--cascade-g3.md 3 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:136#1, .andromeda/obs-plan-detail/span-trace-coverage.md:136#2
- fanout-obs-plan--cascade-g3.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:230#1
- fanout-obs-plan--cascade-g4.md 1 · applied · .andromeda/obs-plan-detail/metric-coverage.md:37#1

## Edits
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  old: «exactly `{"v":1,"error":"instance-unreachable","detail":null}`) opens no process log»
  new: «exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`) opens no process log»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:33
  old: «(a refusal returns `instance-unreachable` with `detail: null`, no frame and no line)»
  new: «(a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:33
  old: «returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does»
  new: «returns the codes-only `isError` `instance-unreachable` with `detail` `server-verify-failed`, no frame and no log line, as the unreachable path does with its own cause»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:57
  old: «`already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4»
  new: «`already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child` (`already-live` a live holder or a fresh replayed beat, `stale-heartbeat` a stale holder; Founder Direction 4»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:57
  old: «(and `already-live`, from the same collision check)»
  new: «(and `already-live` or `stale-heartbeat`, from the same collision check)»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:78
  anchor: «  - The instance check comes first: CLI `send` runs `client::refuse_untrusted` after its text read and before any role file opens, and MCP `send` runs `check_instance` before `live_endpoint`; a refusal sends no frame and writes no line. The server check runs before `channel.request`, inside `send.client` (CLI) or `mcp.tool_call` (MCP): the peer pid and its start time against the one snapshot read, `live_endpoint`'s `read_snapshot_or_replay` (a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so the path ends as unreachable). A server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request`, and the path ends there (Linux and Windows; macOS on the recorded gap).»
  add: «  - `viola_state::liveness::without_endpoint` reads a no-endpoint instance by `symlink_metadata`: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line of its own, and the instance check stays first at every caller (CLI `Unreachable::without_endpoint`, MCP `frame`).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:95
  old: «(no live snapshot, a replayed one included, or the connect failed) or `call` (the connection lost after it), or»
  new: «(no live snapshot, a replayed one included, or the connect failed), `detail:"unwrapped-name"` with `during` `connect` (no instance directory), `detail:"endpoint-lost"` with `during` `call` (the connection lost after it), or»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:96
  anchor: «  - A `send` its instance check refuses (`client::refuse_untrusted`: exit 21, the instance pair, or under `--json` the `instance-unreachable` document) is refused before any role file opens: no `send.client` span, no `process-exit` and no line.»
  add: «  - An exit-21 `process-exit`'s `during` is `call` for `endpoint-lost` alone; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:112
  old: «`detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`), or»
  new: «`detail:"instance-dead"` or `detail:"unwrapped-name"` with `during:"connect"`, `detail:"endpoint-lost"` with `during:"call"` (endpoint vanished mid-wait, the wrapper lost with the call open), or»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:112
  old: «exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and sends no frame»
  new: «exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and sends no frame»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:112
  old: «refused by `check_instance` return `instance-unreachable` with no line»
  new: «refused by `check_instance` return that body less `v` (`error` `instance-unreachable`, `detail` `strict-modes-failed`), their text `refused: instance-unreachable`, with no line»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:136
  old: «or under `--json` the `instance-unreachable` document, no `channel-request`»
  new: «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, no `channel-request`»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:136
  old: «returns `instance-unreachable` with `detail: null` and writes no line»
  new: «returns `instance-unreachable` with `detail` `strict-modes-failed` and writes no line»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:230
  old: «(`StampsError::StrictModes`) to the same home pair.»
  new: «(`StampsError::StrictModes`) to the same home pair, and under a parsed `VIOLA_NAME` its `process-exit` carries `detail:"strict-modes-failed"`. Every other refusal past the tree check likewise writes its own code on that `process-exit` (the Section 6 catalog); a failing row is no refusal and carries no code.»
- line: .andromeda/obs-plan-detail/metric-coverage.md:37
  old: «(`instance-unreachable`, `detail: null`) writes none either»
  new: «(`instance-unreachable`, `detail: "strict-modes-failed"`) writes none either»

## Patterns
[[pattern]]
id = "obs2-01"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "obs2-02"
fixed = 'returns `instance-unreachable` with `detail: null`'

[[pattern]]
id = "obs2-03"
fixed = '`instance-unreachable` with no frame and no log line'

[[pattern]]
id = "obs2-04"
fixed = '`already-live|squatted-name|pinned-hash-mismatch|batch-script-child`'

[[pattern]]
id = "obs2-05"
fixed = '(and `already-live`, from the same collision check)'

[[pattern]]
id = "obs2-06"
fixed = 'or `call` (the connection lost after it)'

[[pattern]]
id = "obs2-07"
fixed = '`detail:"instance-dead"` plus `during`'

[[pattern]]
id = "obs2-08"
fixed = 'refused by `check_instance` return `instance-unreachable` with no line'

[[pattern]]
id = "obs2-09"
fixed = '(`instance-unreachable`, `detail: null`)'
