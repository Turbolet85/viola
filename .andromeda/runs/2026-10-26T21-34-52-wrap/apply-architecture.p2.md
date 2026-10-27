# Apply — architecture · part 2 of 5

## Verdicts
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:17#1
- fanout-architecture--cascade-g1.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:79#1
- fanout-architecture--cascade-g1.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:81+#1
- fanout-architecture--cascade-g1.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:101#1
- fanout-architecture--cascade-g1.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:110#1
- fanout-architecture--cascade-g1.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:125+#1
- fanout-architecture--cascade-g1.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:151#1
- fanout-architecture--cascade-g1.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:207+#1
- fanout-architecture--cascade-g1.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:207+#2
- fanout-architecture--cascade-g1.md 10 · applied · .andromeda/architecture-detail/established-decisions.md:205#1, .andromeda/architecture-detail/established-decisions.md:207+#3
- fanout-architecture--cascade-g1.md 11 · applied · .andromeda/architecture-detail/established-decisions.md:207#1
- fanout-architecture--cascade-g1.md 12 · applied · .andromeda/architecture-detail/established-decisions.md:213#1
- fanout-architecture--cascade-g1.md 15 · applied · .andromeda/architecture-detail/established-decisions.md:229+#1
- fanout-architecture--cascade-g1.md 16 · applied · .andromeda/architecture-detail/established-decisions.md:229+#2
- cascade-compare-architecture.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:94+#1
- cascade-compare-architecture.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:199+#1

## Edits
- line: .andromeda/architecture-detail/established-decisions.md:17
  old: «An absent, tampered, unreadable or unloadable sideload fails open»
  new: «An absent, tampered, unreadable or unloadable sideload, or one whose `conpty/` directory or a companion fails its owner/DACL check ([Deployment / Distribution]), fails open»
- line: .andromeda/architecture-detail/established-decisions.md:79
  old: «as an absent one: every reader stays on `read_snapshot` (§Standard Contracts, Snapshot envelope), and the readers' move onto `read_snapshot_or_replay` is owed to the working-route entry "Home and code-bearing file integrity".»
  new: «as an absent one, through `replay::read_snapshot_or_replay` (§Standard Contracts, Snapshot envelope): a replayed snapshot holds no `endpoint`, so it gives no endpoint and no frame is written; no product reader under `src/` calls `read_snapshot`.»
- after: .andromeda/architecture-detail/established-decisions.md:81
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-message-broker-ipc -->»
  add: «»
  add: «- `viola hook statusline`'s `serve` reads the snapshot through the replay too: a replayed snapshot holds no `statusline_command`, so nothing runs and nothing is printed. The replay's `state.snapshot_recover` span sits under `run.start` before `run.collision_check`, and under `hook.handle` on the hook path.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:101
  old: «else, when the endpoint is not reached or its server check fails, a direct `try_lock` append»
  new: «else, once the instance check has passed ([Hook Contract]) and the endpoint is not reached or its server check fails, a direct `try_lock` append»
- line: .andromeda/architecture-detail/established-decisions.md:110
  old: «The event and dialog paths share one connect: `read_snapshot`, then»
  new: «The event and dialog paths, once their instance check has passed ([Hook Contract]), share one connect: `replay::read_snapshot_or_replay` (a replayed or absent snapshot is an endpoint not reached: the hook fails open with `channel-unreachable`), then»
- after: .andromeda/architecture-detail/established-decisions.md:125
  anchor: «- A dialog hook whose server check fails (the serving process is not the recorded wrapper, or the check cannot be made) sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human (its log lines: §Occupied Resources, `diagnostics/`).»
  add: «- Beside the absent-`VIOLA_NAME` exit, the event and dialog paths run the instance check (`viola_state::strict::check_instance` on the canonical instance directory) before the diagnostics level is read and obs is initialised: a refused instance, or none (a `VIOLA_DIR` naming no existing directory), exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The `--capture` arms run ahead of it, unchanged.»
- line: .andromeda/architecture-detail/established-decisions.md:151
  old: «on the home and `instances/<driven>` before `client::start`»
  new: «on the home and `instances/<driven>` through `client::refuse_untrusted` (`src/cmd/client.rs`, shared with the six frame verbs) before `client::start`»
- line: .andromeda/architecture-detail/established-decisions.md:205
  old: «`outcome` `loaded` · `hash-mismatch` · `unreadable` · `load-failed` · `not-built`, never a refusal»
  new: «`outcome` `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`, never a refusal»
- line: .andromeda/architecture-detail/established-decisions.md:207
  old: «returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`), and every frame-writing client takes the liveness-only pre-check first, then,»
  new: «returns them with the endpoint from one `replay::read_snapshot_or_replay` (`LiveEndpoint`, only on a real snapshot: a replayed or absent one gives none), and every frame-writing client runs the strict-modes instance check first (the CLI's `client::refuse_untrusted`, [Links]; `viola mcp`'s frame tools, [MCP]; the hook's, [Hook Contract]), then the liveness-only pre-check, then,»
- after: .andromeda/architecture-detail/established-decisions.md:207
  anchor: «- The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`), and every frame-writing client takes the liveness-only pre-check first, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness).»
  add: «- `run`'s collision check takes its caller's one snapshot read (`replay::read_snapshot_or_replay`): over a replayed snapshot, which holds no `pid` or `started_at`, it decides from the heartbeat alone, a beat at most `STALE_AFTER` old refusing `already-live` and an older one free; a failed read or no snapshot is free.»
- after: .andromeda/architecture-detail/established-decisions.md:207
  anchor: «- The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`), and every frame-writing client takes the liveness-only pre-check first, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness).»
  add: «- `viola revive`'s preflight reads the snapshot once and hands that read to both its collision check and its recorded-directory reading: one replay and one `state-recovered` line per revive.»
- after: .andromeda/architecture-detail/established-decisions.md:207
  anchor: «- The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`), and every frame-writing client takes the liveness-only pre-check first, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness).»
  add: «- Before its wrapper log opens, every start (`run`'s and a revived one) checks the home tree: `checked_exe` reads the exe's bytes once and takes their pin key, then `viola_state::strict::check_home_tree` checks the home, the instance's set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists; a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, no log opened, no line and no child.»
- line: .andromeda/architecture-detail/established-decisions.md:213
  old: «its fold or ratification is open; anyhow 1.0.104»
  new: «its fold or ratification is open. A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open; anyhow 1.0.104»
- after: .andromeda/architecture-detail/established-decisions.md:229
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-deployment-distribution -->»
  add: «»
  add: «- `viola_state::pin::pin_key(bytes)` names the pinned copy's key, `<version>-<16 hex>`, in one place, and `pin_bytes(home, bytes)` pins bytes already read; `pin_exe(home, exe)` delegates to it. `run`'s start reads its exe's bytes once, takes their key for its home-tree check ([Session Liveness]) and pins those same bytes through `pin_bytes`; `viola verify` calls `pin_exe`.»
- after: .andromeda/architecture-detail/established-decisions.md:229
  anchor: «<!-- /detail -->»
  add: «- On Windows x64, before the held open, `pin_companions` runs the owner/DACL check (`viola_state::strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists: a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit code unchanged ([PTY]).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:94
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-mcp -->»
  add: «»
  add: «- Each frame tool (`send`, `wait`, `last`, `answer`) runs `viola_state::strict::check_instance` on the home and the target's instance directory before `live_endpoint`; a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:199
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-cli-version-compatibility -->»
  add: «»
  add: «- Before its `cli` log opens, `viola verify` runs `viola_state::strict::check_home_tree(home, None, None)` (the home, `ledger/`, `bin/` and `plugin/`, each that exists); a refusal prints the home pair and exits 1 with no step line, no summary and no line. The one stamps writer, `update_stamps`, runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read; a refusal (`StampsError::StrictModes`, [Error Handling]) reads and writes nothing, and verify maps it to the same home pair.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = "tampered, unreadable or unloadable sideload fails open"

[[pattern]]
id = "arch2-02"
fixed = "every reader stays on `read_snapshot`"

[[pattern]]
id = "arch2-03"
fixed = "share one connect: `read_snapshot`, then"

[[pattern]]
id = "arch2-04"
fixed = "`loaded` · `hash-mismatch` · `unreadable`"

[[pattern]]
id = "arch2-05"
fixed = "takes the liveness-only pre-check first"
