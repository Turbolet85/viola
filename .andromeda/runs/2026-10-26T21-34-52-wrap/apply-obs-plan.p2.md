# Apply — obs-plan · part 2 of 6

## Verdicts
- fanout-obs-plan--cascade-g1.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:123#1
- fanout-obs-plan--cascade-g1.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:133+#1
- fanout-obs-plan--cascade-g1.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:158#1
- fanout-obs-plan--cascade-g1.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:158#2
- fanout-obs-plan--cascade-g1.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:164#1
- fanout-obs-plan--cascade-g1.md 6 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:186#1
- fanout-obs-plan--cascade-g1.md 7 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:189#1
- fanout-obs-plan--cascade-g1.md 8 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:190#1
- fanout-obs-plan--cascade-g1.md 9 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:232#1
- fanout-obs-plan--cascade-g1.md 10 · folded into fanout-obs-plan--cascade-g2.md 1
- fanout-obs-plan--cascade-g1.md 11 · folded into fanout-obs-plan--cascade-g1.md 9
- fanout-obs-plan--cascade-g2.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:292#1
- fanout-obs-plan--cascade-g2.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:299#1
- fanout-obs-plan--cascade-g2.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:300#1
- fanout-obs-plan--cascade-g2.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:327#1
- fanout-obs-plan--cascade-g2.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:370#1
- fanout-obs-plan--cascade-g2.md 8 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:379#1
- fanout-obs-plan--cascade-g2.md 9 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:397#1
- fanout-obs-plan--cascade-g2.md 10 · folded into fanout-obs-plan--cascade-g2.md 1
- fanout-obs-plan--cascade-g2.md 13 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:464#1
- fanout-obs-plan--cascade-g2.md 14 · folded into fanout-obs-plan--cascade-g2.md 3
- cascade-compare-obs-plan.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:326#1

## Edits
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:123
  old: «not the OS internals.»
  new: «not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:133
  anchor: «  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.»
  add: «  - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:158
  old: «instance check of it has passed; a refused check opens none.»
  new: «instance check of it has passed; a refused check opens none. `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home and writes no line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:158
  old: «the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`).»
  new: «the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`). `viola verify` logs as `cli` (with an instance) once its home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) has passed; a refused check exits 1 with the home pair, opens no log and writes no line, and a refused `update_stamps` (`StampsError::StrictModes`) exits 1 with the same pair.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:164
  old: «each cause behind exit 21 and exit 1 gets a distinct detail code.»
  new: «each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:186
  old: «Vector 4 "missing `VIOLA_NAME`").»
  new: «Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The two `--capture` arms run ahead of the check; the `FAKE_AGENT_HOOK_PANIC` seam runs after obs init.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:189
  old: «Logger init must not add blocking I/O beyond one file open and append.»
  new: «Logger init must not add blocking I/O beyond one file open and append; the instance check ahead of it (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path, and `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms (as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:190
  old: «That case gets no log and no stderr, by contract.»
  new: «That case gets no log and no stderr, by contract. A `VIOLA_DIR` naming an instance directory that does not exist resolves none, so the hook writes nothing.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:232
  old: «and the instance (`VIOLA_NAME` or an argument).»
  new: «and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:292
  old: «and the tests secret-scan test enforces it.»
  new: «and the tests secret-scan test enforces it. The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:299
  old: «`run.start` › `run.collision_check`»
  new: «`run.start` › `state.snapshot_recover` › `run.collision_check`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:300
  old: «the check precedes the log (chunk 2026-10-10-budget-governor).»
  new: «the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:326
  old: «`hook.handle` › `channel.request(hook.dialog)`»
  new: «`hook.handle` › `state.snapshot_recover` › `channel.request(hook.dialog)`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:327
  old: «The question, answer and tool `input` text never appear.»
  new: «The question, answer and tool `input` text never appear. A refused instance check writes no line and so carries none of these fields: CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document under `--json`; the dialog hook exits 0 at once with empty streams, no frame and no line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:370
  old: «`viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line»
  new: «every `viola hook` arm but the two `--capture` arms — `hook statusline`, the event paths and the dialog paths — runs an instance check before obs init, and a refused check leaves no line»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:379
  old: «with no upstream text, absolute paths or type names.»
  new: «with no upstream text, absolute paths or type names. One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:397
  old: «As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Five strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the checked read»
  new: «As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`) leaves only verify's own `process-exit` (with an instance; exit 1, no `detail`). Every other strict-modes refusal leaves no line: `viola revive`'s instance check, `run`'s and a revived start's home-tree check (`check_home_tree`, exit 1, before the wrapper log opens), `viola verify`'s home-tree check (exit 1), the instance check of every `viola hook` arm but the two `--capture` arms (exit 0), the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`refuse_untrusted`, exit 21), `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the `viola mcp` frame tools' instance check (`instance-unreachable`), the `diagnostics/` opens (`check_diagnostics`: a refusal is a failed open), the checked read»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:464
  old: «and `hook` still exits 0.»
  new: «and `hook` still exits 0. A process whose role-file open fails — `check_diagnostics` refusing the `diagnostics/` folder or the role file included — has no panic sink, so a panic in it writes no record.»

## Patterns
[[pattern]]
id = "obs2-01"
regex = 'each cause behind exit 21 and exit 1 gets a distinct'
ignore_case = true

[[pattern]]
id = "obs2-02"
fixed = '`run.start` › `run.collision_check`'

[[pattern]]
id = "obs2-03"
fixed = '`hook.handle` › `channel.request(hook.dialog)`'

[[pattern]]
id = "obs2-04"
fixed = 'is the one arm that runs an instance check'

[[pattern]]
id = "obs2-05"
fixed = 'has a line only on the `ledger-stamps` read'

[[pattern]]
id = "obs2-06"
fixed = 'Five strict-modes refusals leave no line'

[[pattern]]
id = "obs2-07"
fixed = 'Rejected tool calls are logged with codes only'

[[pattern]]
id = "obs2-08"
fixed = 'Each panic produces exactly one single-line'
