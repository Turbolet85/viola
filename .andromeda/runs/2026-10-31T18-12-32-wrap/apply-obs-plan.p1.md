# Apply — obs-plan · part 1 of 4

## Verdicts
- fanout-obs-plan--cascade-g1.md 10 · applied · .andromeda/obs-plan.md:215#1
- fanout-obs-plan--cascade-g3.md 7 · applied · .andromeda/obs-plan.md:361#1, .andromeda/obs-plan.md:361#2
- fanout-obs-plan.md 1 · folded into fanout-obs-plan--cascade-g3.md 7
- fanout-obs-plan--cascade-g4.md 16 · applied · .andromeda/obs-plan.md:523#1
- fanout-obs-plan.md 5 · applied · .andromeda/obs-plan.md:140#1
- fanout-obs-plan--cascade-g1.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:36+#1
- fanout-obs-plan--cascade-g1.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:159#1
- fanout-obs-plan--cascade-g1.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:163#1
- fanout-obs-plan--cascade-g1.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:165#1
- fanout-obs-plan--cascade-g1.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:215#1
- fanout-obs-plan--cascade-g1.md 6 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:302#1
- fanout-obs-plan--cascade-g1.md 7 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:329#1
- fanout-obs-plan--cascade-g1.md 8 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:381#1
- fanout-obs-plan--cascade-g1.md 9 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:399#1, .andromeda/obs-plan-detail/obs-scope-summary.md:399#2
- fanout-obs-plan--cascade-g1.md 11 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:445#1
- fanout-obs-plan--cascade-g1.md 12 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:490#1, .andromeda/obs-plan-detail/obs-scope-summary.md:490#2
- fanout-obs-plan--cascade-g1.md 13 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:491+#1
- cascade-compare-obs-plan.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:320#1
- fanout-obs-plan--cascade-g1.md 14 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:43#1

## Edits
- line: .andromeda/obs-plan.md:215
  old: «`run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged);»
  new: «`run`'s version gate, on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged) and on `verify`'s own `process-exit` for a refused stamp write (`detail` `strict-modes-failed`, exit 1);»
- line: .andromeda/obs-plan.md:361
  old: «and MCP's returns `instance-unreachable` with no line (macOS compares»
  new: «and MCP's returns `instance-unreachable` with `detail` `server-verify-failed` and no line (macOS compares»
- line: .andromeda/obs-plan.md:361
  old: «`{"v":1,"error":"instance-unreachable","detail":null}` under `--json`; no line), and MCP's returns `instance-unreachable` with no line.»
  new: «`{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`; no line), and MCP's returns `instance-unreachable` with `detail` `strict-modes-failed` and no line.»
- line: .andromeda/obs-plan.md:523
  old: «in one `write_all`, with no hint, and exit 1;»
  new: «in one `write_all`, with no hint, or under `--json` the `{"v":1,"error":"internal-error","detail":null}` document on stdout with stderr empty, and exit 1;»
- line: .andromeda/obs-plan.md:140
  old: «exit 1 can be an additive `detail` field.»
  new: «exit 1 are an additive `detail` field on `process-exit` (`schemas/diag-line.v1.json`).»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:36
  anchor: «- **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.»
  add: «  - `viola_core::UnreachableCause` is the closed set of exit 21's five cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), with no serde path. The two clients write each word into their two documents (the CLI's `--json` exit-21 document, MCP's `isError` body); the type writes no log line of its own, and a refused strict-modes check writes no line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:159
  old: «or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home»
  new: «or under `--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document, opens nothing under the home»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:163
  old: «(design excerpt, CLI refusals / CLI faults).»
  new: «(design excerpt, CLI refusals / CLI faults). Without `--json` a catch-site fault prints the fixed `error: internal error` line; under `--json` stderr stays empty and the `{"v":1,"error":"internal-error","detail":null}` document goes to stdout (`internal_error_document` in `src/cmd/mod.rs`, when `json_asked` in `src/main.rs` reads `--json` among the arguments before a `--`).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:165
  old: «`run`, `viola revive` and `viola verify`, exit 1).»
  new: «`run`, `viola revive` and `viola verify`, exit 1). The socket-directory cause and macOS's server check are the named carve-outs from this rule, both with the route entry "Unix endpoint and home hardening" (the Founder Direction 4 trigger below states them).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:215
  old: «`detail`, `instance-unreachable`, `wrapper-fault`»
  new: «`detail`, `instance-unreachable` (its `detail` one closed cause: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`; the CLI's document less `v`, its text `refused: instance-unreachable`), `wrapper-fault`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:302
  old: «a distinct `detail` code: already-live, squatted-name,»
  new: «a distinct `detail` code: already-live (a live holder or a fresh replayed beat), stale-heartbeat (a stale holder), squatted-name,»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:329
  old: «or the `instance-unreachable` document under `--json`;»
  new: «or the `instance-unreachable` document with `detail` `strict-modes-failed` under `--json`;»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:381
  old: «One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.»
  new: «These rejected calls write no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail` `strict-modes-failed` and writes no frame; one with no `live_endpoint` returns `without_endpoint`'s reading, `instance-dead` or `unwrapped-name`, and one whose server check fails returns `server-verify-failed`, each likewise with no frame.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:399
  old: «has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64»
  new: «has a line in three places: the `ledger-stamps` read in `run`'s version gate, on Windows x64»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:399
  old: «the exit unchanged; a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`) leaves only verify's own `process-exit` (with an instance; exit 1, no `detail`).»
  new: «the exit unchanged; and a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`), which leaves only verify's own `process-exit` (with an instance; exit 1, `detail` `strict-modes-failed`).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:445
  old: «endpoint vanished during `wait` (exit 21 / `instance-unreachable`);»
  new: «endpoint vanished during `wait` (exit 21 / `instance-unreachable`, `detail` `endpoint-lost`, `during` `call`);»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:490
  old: «exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets»
  new: «exit 21 (`instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`) and exit 1 (`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and `viola revive`'s `no-session`, `session-live` and `cwd-missing`) gets»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:490
  old: «`{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.»
  new: «`{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document, exit 21.»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:491
  anchor: «  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).»
  add: «  - Two causes stand outside the distinct-code rule, both with the route entry "Unix endpoint and home hardening": the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check. OPEN — the socket-directory cause has no code, PROVISIONAL · owner: leaving its code to that entry.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:320
  old: «`exit_code` (21 when the wrapper is killed mid-wait)»
  new: «`exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`)»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:43
  old: «writes no line and carries no `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document (the frame verbs) or `strict-modes-failed` (`revive`)»
  new: «writes no line and carries no logged `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document with `detail` `strict-modes-failed` (the frame verbs and `link` / `unlink`) or the `strict-modes-failed` document (`revive`)»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = 'version gate and on a `conpty/` companion'

[[pattern]]
id = "obs1-02"
fixed = 'returns `instance-unreachable` with no line'

[[pattern]]
id = "obs1-03"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "obs1-04"
fixed = 'with no hint, and exit 1'

[[pattern]]
id = "obs1-05"
fixed = 'can be an additive `detail` field'

[[pattern]]
id = "obs1-06"
fixed = 'distinct `detail` code: already-live, squatted-name'

[[pattern]]
id = "obs1-07"
fixed = 'the `instance-unreachable` document under `--json`'

[[pattern]]
id = "obs1-08"
fixed = 'with `detail: null` and writes no frame'

[[pattern]]
id = "obs1-09"
fixed = 'has a line in two places'

[[pattern]]
id = "obs1-10"
fixed = 'exit 1, no `detail`'

[[pattern]]
id = "obs1-11"
fixed = 'dead instance, strict-mode fail, server-verify fail'

[[pattern]]
id = "obs1-12"
fixed = 'closed `instance-unreachable` document (the frame verbs)'
