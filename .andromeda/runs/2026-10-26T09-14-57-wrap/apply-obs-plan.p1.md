# Apply — obs-plan · part 1 of 3

## Verdicts
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:361#1
- fanout-obs-plan--cascade-g2.md 10 · folded into fanout-obs-plan.md 1
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:537#1
- fanout-obs-plan--cascade-g1.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:123#1
- fanout-obs-plan--cascade-g1.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:327#1
- fanout-obs-plan--cascade-g1.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:454+#1
- fanout-obs-plan--cascade-g1.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:485+#1
- cascade-compare-obs-plan.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:370+#1
- fanout-obs-plan--cascade-g1.md 6 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:30#1

## Edits
- line: .andromeda/obs-plan.md:361
  old: «`strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".»
  new: «on Linux and Windows a serving process that is not the recorded wrapper ends a CLI `wait` / `last` before any frame, with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, and MCP's returns `instance-unreachable` with no line (macOS compares no peer pid, the recorded gap); `strict-modes-failed` cannot occur on them, its check owed to the route entry "Home and code-bearing file integrity".»
- line: .andromeda/obs-plan.md:537
  old: «MCP rejects are `isError` codes only, mirrored by codes-only log lines.»
  new: «MCP rejects are `isError` codes only; a logged reject's line is codes-only, and `instance-unreachable` — the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows — is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:123
  old: «Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.»
  new: «Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:327
  old: «`refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12)»
  new: «`refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12, `server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint)»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:370
  anchor: «  - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).»
  add: «  - A server-verify mismatch (Linux and Windows) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:454
  anchor: «  - ConPTY vs openpty in `pty` spans.»
  add: «  - the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:485
  anchor: «  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).»
  add: «  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:30
  old: «asserted by absence (chunk 2026-10-10-budget-governor) |»
  new: «asserted by absence (chunk 2026-10-10-budget-governor). The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux and Windows (macOS compares no peer pid, the recorded gap) |»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = 'cannot occur on `wait` / `last` until client-side server verification lands'

[[pattern]]
id = "obs1-02"
fixed = 'mirrored by codes-only log lines'
