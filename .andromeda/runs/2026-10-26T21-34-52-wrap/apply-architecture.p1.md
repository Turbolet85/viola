# Apply — architecture · part 1 of 5

## Verdicts
- fanout-architecture--cascade-g1.md 13 · applied · .andromeda/architecture.md:65#1
- fanout-architecture--cascade-g1.md 14 · applied · .andromeda/architecture.md:106#1
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:325#1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:100#1, .andromeda/architecture.md:100#2
- fanout-architecture--cascade-g1.md 20 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:318+#1
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:319#1
- fanout-architecture.md 5 · applied · .andromeda/architecture.md:323#1
- fanout-architecture--cascade-g2.md 9 · folded into fanout-architecture.md 5
- fanout-architecture.md 6 · applied · .andromeda/architecture.md:152+#1
- fanout-architecture.md 11 · applied · .andromeda/architecture.md:296#1

## Edits
- line: .andromeda/architecture.md:65
  old: «and two open divergences, `viola-agent-claude`'s `Refusal` and `viola-mcp`'s `ServeError`, the rule standing»
  new: «and three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, the rule standing»
- line: .andromeda/architecture.md:106
  old: «beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are the two open divergences named there»
  new: «beside `AgentError`, `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences named there»
- line: .andromeda/architecture.md:325
  old: «`viola list`, `viola revive`, `hook statusline`, `viola link` and `viola unlink` check these files before they use them, ahead of any log, because opening a log narrows an existing home.»
  new: «`viola list`, `viola revive`, `viola run`'s start, `hook statusline`, `viola hook`'s event and dialog paths, `viola link` / `viola unlink`, the six CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) and `viola mcp`'s frame tools check these files before they use them, ahead of any log, because opening a log narrows an existing home: the home, the instance directory and its `diagnostics/` to the read rule (Unix no group or other bit; Windows no read right granted to another SID), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check writes no line.»
- line: .andromeda/architecture.md:100
  old: «`1` internal error, and the start, revive-preflight and verify refusals»
  new: «`1` internal error, and the start, revive-preflight and verify refusals (a refused home-tree check of `run`, `viola revive`'s start or `verify` among them)»
- line: .andromeda/architecture.md:100
  old: «a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink`.»
  new: «a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`).»
- after: .andromeda/architecture.md:318
  anchor: «- `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary that `viola run` creates if it is absent (0700 on Unix), where `<hash>` is the first 16 hex of the SHA-256 of the exe's content; an existing copy is re-hashed on every start, and a mismatch refuses the start (exit 1, `pinned-hash-mismatch`) and leaves the file as found. v1 never deletes pinned copies or plugin folders. Each instance snapshot records the pinned path it uses, so any later cleanup (an open item) can skip copies that a live or `stale` instance still calls.»
  add: «- Home tree: before any log, `viola run` and `viola revive`'s start check the home tree (`viola_state::strict::check_home_tree`): the home to the read rule, then the instance's files (as `instances/<ViolaName>/` below), `ledger/`, `bin/`, `plugin/`, `bin/<version>-<hash>/`, the pinned copy and `plugin/<version>-<hash>/` with its `hooks/` and `.claude-plugin/` to the write rule, each that exists; a refusal exits 1 with the home-state `unable:` / `hint:` pair (the instance-state pair for a refusal inside the instance), opens no log, writes no line and starts no child, and `run` pins the very exe bytes it checked. `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` the same way before its log (exit 1, the home-state pair).»
- line: .andromeda/architecture.md:319
  old: «at every start each is held with `FILE_SHARE_READ` only and fully re-hashed against its compiled SHA-256 pin until the spawn. A mismatch leaves the file as found, never loads it, never refuses the start, and runs the child on the inbox ConPTY (`sideload_fallback` `hash-mismatch`).»
  new: «at every start `bin/<version>-<hash>/conpty/` and each companion that exists first pass the strict-modes check (owner and DACL), then each is held with `FILE_SHARE_READ` only and fully re-hashed against its compiled SHA-256 pin until the spawn. A refused check or a mismatch writes nothing, leaves the file as found, never loads it, never refuses the start, and runs the child on the inbox ConPTY (`sideload_fallback` `strict-modes-failed` or `hash-mismatch`).»
- line: .andromeda/architecture.md:323
  old: «Content-bearing detail stays in `instances/<ViolaName>/diagnostics/`.»
  new: «Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (format fixed by `schemas/diag-detail.v1.json`). Every open of a role file or a detail file first checks the existing `diagnostics/` folder and the file, before anything is narrowed: a symbolic link refused, the folder to the read rule, the file to the read rule on Unix and the write rule on Windows, an absent path passing; a refusal is a failed open (the role file's writer ends at a sink, a detail line is dropped).»
- after: .andromeda/architecture.md:152
  anchor: «**Snapshot envelope** (atomically replaced JSON): `{v, written_at, writer, data}`. A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.»
  add: «- The replay's product readers are `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`, `viola_state::liveness::live_endpoint` (the CLI frame verbs, `viola link` / `viola unlink` and `viola mcp`'s frame tools), `viola hook`'s connect, `hook statusline` and `run`'s collision check; no product code under `src/` calls `read_snapshot`. A replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `live_endpoint` and the hook's connect read the instance as unreachable, `hook statusline` runs nothing, and the collision check judges liveness from the heartbeat alone (a beat within `STALE_AFTER` refuses `already-live`).»
- line: .andromeda/architecture.md:296
  old: «and its absence makes every hook event path a silent exit 0 (the hidden `hook --capture` arm reads no `VIOLA_*` at all, [Hook Contract]).»
  new: «and its absence, a `VIOLA_DIR` naming no existing instance directory, or an instance its strict-modes check refuses makes every hook event and dialog path a silent exit 0: no `config.json` read, no stdin read, no line, no frame and no SessionEnd append (the hidden `hook --capture` arms, taken first, read no `VIOLA_*` at all, [Hook Contract]).»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'two open divergences'

[[pattern]]
id = "arch1-02"
fixed = 'are the two open divergences named there'

[[pattern]]
id = "arch1-03"
fixed = '`hook statusline`, `viola link` and `viola unlink` check these files'

[[pattern]]
id = "arch1-04"
fixed = '`viola plugin install`, `viola link` or `viola unlink`'

[[pattern]]
id = "arch1-05"
fixed = '(`sideload_fallback` `hash-mismatch`)'
