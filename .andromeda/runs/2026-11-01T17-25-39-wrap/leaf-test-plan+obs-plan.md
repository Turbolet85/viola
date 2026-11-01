group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
- leaf: .claude/rules/observability.md:27   class: omission
  leaf-text: «as on the hold's expiry line).»
  source: .andromeda/obs-plan.md:479 «On Unix a peer the wrapper's `accept_loop` refuses by euid is dropped on the accept thread, before its worker thread and before any frame, and writes no line by rule»
  replace-with: «as on the hold's expiry line). On Unix a peer the wrapper's `accept_loop` refuses by euid (`peer_admitted`) is dropped on the accept thread, before its worker thread and before any frame, and writes no line by rule (no frame to key a line on); the client reads end-of-file.»
- leaf: .claude/rules/observability.md:28   class: stale
  leaf-text: «closes `process-exit`'s at 19 values»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 20 values»
  replace-with: «closes `process-exit`'s at 20 values»
- leaf: .claude/rules/observability.md:28   class: stale
  leaf-text: «exit 21's closed causes are `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed` (»
  source: .andromeda/obs-plan-detail/log-coverage.md:31 «`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.»
  replace-with: «exit 21's closed causes are `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused` (»
- leaf: .claude/rules/observability.md:28   class: stale
  leaf-text: «`instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:105 «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`.»
  replace-with: «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`»
- leaf: .claude/rules/observability.md:38   class: stale
  leaf-text: «closes at 19 values»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 20 values»
  replace-with: «closes at 20 values»
- leaf: .claude/rules/observability.md:40   class: stale
  leaf-text: «A failed server check (Linux and Windows; macOS compares no peer pid, the recorded gap) sends no frame and opens no `channel.request`»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:29 «`server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone: macOS reports no peer pid, its named carve-out.»
  replace-with: «A refused peer euid (Unix) or a failed server check (the peer pid and start-time comparison on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out) sends no frame and opens no `channel.request`»
- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «MCP's frame tools return `instance-unreachable` with `detail` `server-verify-failed` and no line, as the liveness pre-check's does with»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:39 «then after `live_endpoint` run the socket-directory check `check_endpoint_dir` (a refusal returns `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no line)»
  replace-with: «MCP's frame tools return `instance-unreachable` with `detail` `server-verify-failed` and no line, as their refused `check_endpoint_dir` (made after `live_endpoint`) does with `detail` `socket-dir-refused` and no frame, and as the liveness pre-check's does with»
- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «no pid, start time, path or endpoint in any line.»
  source: .andromeda/obs-plan-detail/log-coverage.md:42 «The check takes the read verdict of the strict-modes checks above yet writes lines under its own word: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`,»
  replace-with: «no pid, start time, path or endpoint in any line. A refused socket directory (`check_socket_dir`, the strict-modes read verdict) is none of these exemptions and writes under its own closed word `socket-dir-refused`, with no path, pid or uid in any line: `viola run` and `viola revive`'s start (`socket_dir_ready`, no span, before any snapshot, child or `Server::bind`; a failed create, a lost race and a refused check read one code, and a `Server::bind` failure is not this refusal) one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, under `viola revive --json` the same-shaped `socket-dir-refused` document; the eight CLI frame verbs one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside their dispatch span, no `channel-request` line and no frame; `viola hook` one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle` (on a dialog hook after `hook-invoked{corr:null}`, with `decision_emitted:false`), exit 0, empty streams and no frame.»
