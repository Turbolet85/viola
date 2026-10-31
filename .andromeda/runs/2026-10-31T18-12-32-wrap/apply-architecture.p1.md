# Apply — architecture · part 1 of 4

## Verdicts
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture.md:226#1
- fanout-architecture.md 1 · folded into fanout-architecture--cascade-g2.md 1
- fanout-architecture--cascade-g2.md 2 · applied · .andromeda/architecture.md:228#1, .andromeda/architecture.md:228#2
- fanout-architecture.md 2 · folded into fanout-architecture--cascade-g2.md 2
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture.md:229#1
- fanout-architecture.md 3 · folded into fanout-architecture--cascade-g2.md 3
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:227+#1
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:124#1
- fanout-architecture--cascade-g1.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:124+#1
- fanout-architecture--cascade-g1.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:249+#1
- fanout-architecture--cascade-g1.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:258+#1
- fanout-architecture--cascade-g1.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:259+#1, .andromeda/architecture-detail/established-decisions.md:264+#1
- fanout-architecture--cascade-g1.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:262#1
- fanout-architecture--cascade-g1.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:305#1, .andromeda/architecture-detail/established-decisions.md:305#2
- cascade-compare-architecture.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:75#1

## Edits
- line: .andromeda/architecture.md:226
  old: «`{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)»
  new: «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (exit 21), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (`viola_core::UnreachableCause`), written by `client::unreachable_document`; MCP's `isError` result is the same less `v`, its text `refused: instance-unreachable`»
- after: .andromeda/architecture.md:227
  anchor: «- `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` (exit 20): `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`»
  add: «- `{"v":1,"error":"internal-error","detail":null}` (exit 1): a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`), printed on stdout by `cmd::internal_error_document` in place of `error: internal error`, stderr empty; without `--json` that human line stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.»
- line: .andromeda/architecture.md:228
  old: «`viola revive --json` refusing its strict-modes check or its preflight, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, written by»
  new: «`viola revive --json` refusing its strict-modes check, its preflight or the start it shares with `run`, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, written by»
- line: .andromeda/architecture.md:228
  old: «A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".»
  new: «A `pump_child` error prints no document: the child has held the terminal.»
- line: .andromeda/architecture.md:229
  old: «prints the `instance-unreachable` document (exit 21) through `client::refused_document`.»
  new: «prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`.»
- line: .andromeda/architecture-detail/established-decisions.md:75
  old: «if the endpoint vanishes first the caller gets `instance-unreachable`.»
  new: «if the endpoint vanishes first the caller gets `instance-unreachable`, its `detail` `endpoint-lost`.»
- line: .andromeda/architecture-detail/established-decisions.md:124
  old: «a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line.»
  new: «a refusal is the `instance-unreachable` result with `detail: "strict-modes-failed"`, no frame and no line.»
- after: .andromeda/architecture-detail/established-decisions.md:124
  anchor: «- Each frame tool (`send`, `wait`, `last`, `answer`) runs `viola_state::strict::check_instance` on the home and the target's instance directory before `live_endpoint`; a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line.»
  add: «- `viola_mcp`'s `result::unreachable(cause)` builds the `instance-unreachable` result from a `viola_core::UnreachableCause`: `of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board` reads the board's refusal as `StrictModes`, and `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, no `live_endpoint` by `without_endpoint` ([Session Liveness]) and a failed `server_verified` as `ServerVerify`, each with no frame and no line, `isError` the record.»
- after: .andromeda/architecture-detail/established-decisions.md:249
  anchor: «- Before its `cli` log opens, `viola verify` runs `viola_state::strict::check_home_tree(home, None, None)` (the home, `ledger/`, `bin/` and `plugin/`, each that exists); a refusal prints the home pair and exits 1 with no step line, no summary and no line. The one stamps writer, `update_stamps`, runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read; a refusal (`StampsError::StrictModes`, [Error Handling]) reads and writes nothing, and verify maps it to the same home pair.»
  add: «- `viola verify` writes `Ended::Refused`'s code on the `process-exit` it logs under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`. A refused home tree writes no line, a failing row is no refusal and has no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.»
- after: .andromeda/architecture-detail/established-decisions.md:258
  anchor: « A gone instance's directory is reused at once, and its `events.ndjson` is appended to, never truncated.»
  add: «- `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder or a fresh replayed beat. The `run.collision_check` span's `outcome` reads `already-live` for a live or a stale holder, its closed set saying a holder stands. Stale heartbeat is exit 1's cause alone: a frame verb against a stale instance reads `instance-dead`, `live_endpoint` giving none for a stale wrapper.»
- after: .andromeda/architecture-detail/established-decisions.md:259
  anchor: «- The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `replay::read_snapshot_or_replay` (`LiveEndpoint`, only on a real snapshot: a replayed or absent one gives none), and every frame-writing client runs the strict-modes instance check first (the CLI's `client::refuse_untrusted`, [Links]; `viola mcp`'s frame tools, [MCP]; the hook's, [Hook Contract]), then the liveness-only pre-check, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness).»
  add: «- `viola_state::liveness::without_endpoint` reads the instance directory by `symlink_metadata` where a client has no live endpoint: a directory is `InstanceDead`, anything else (absent, a regular file, a symlink) is `UnwrappedName`. It opens nothing, writes no line and stays behind the strict-modes check at every caller (`client::Unreachable::without_endpoint`, `viola mcp`'s `frame`).»
- line: .andromeda/architecture-detail/established-decisions.md:262
  old: «or the instance pair for an instance-level refusal, no log opened»
  new: «or the instance pair for an instance-level refusal, or under `viola revive --json` the `strict-modes-failed` document in place of either pair, no log opened»
- after: .andromeda/architecture-detail/established-decisions.md:264
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: pending · dec-session-liveness -->»
  add: «»
  add: «- OPEN — `without_endpoint`'s reading of a missing endpoint by the instance directory, `instance-dead` for a directory and `unwrapped-name` for anything else, is PROVISIONAL · owner: the open decision on the unwrapped-name reading, `inputs#I2` Q2 of chunk 2026-10-31-exit-cause-code-catalogue.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:305
  old: «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) in place of its `unable:` / `hint:` pair, exit 1,»
  new: «every refusal of its start, a refused strict-modes check and a preflight refusal among them, prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, a `claude` not on `PATH` or an anyhow error of the start being `internal-error`) in place of its `unable:` / `hint:` pair where it has one, exit 1,»
- line: .andromeda/architecture-detail/established-decisions.md:305
  old: «A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".»
  new: «A `pump_child` error prints no document: the child has held the terminal.»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "arch1-02"
fixed = 'refusing its strict-modes check or its preflight'

[[pattern]]
id = "arch1-03"
fixed = 'prints no document as landed'

[[pattern]]
id = "arch1-04"
fixed = '(exit 21) through `client::refused_document`'

[[pattern]]
id = "arch1-05"
fixed = 'the caller gets `instance-unreachable`.'

[[pattern]]
id = "arch1-06"
fixed = 'result with `detail: null`, no frame and no line'

[[pattern]]
id = "arch1-07"
fixed = 'or the instance pair for an instance-level refusal, no log opened'

[[pattern]]
id = "arch1-08"
fixed = 'a refused strict-modes check or a preflight refusal prints'

[[pattern]]
id = "arch1-09"
fixed = 'its document is owed to the working-route entry "Exit-cause code catalogue"'
