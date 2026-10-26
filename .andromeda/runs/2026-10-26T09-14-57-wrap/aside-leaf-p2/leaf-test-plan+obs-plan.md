group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «(the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13).»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`server-verify-failed` is written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows)»
  replace-with: «(the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13). A failed server check (Linux and Windows; macOS compares no peer pid, the recorded gap) sends no frame and opens no `channel.request`: the six CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) end `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside their dispatch span with no `channel-request` line; `viola hook` writes `hook-decision{detail:"server-verify-failed"}` at WARN (on a dialog hook `hook-invoked{corr:null}` first, then `decision_emitted:false`); MCP's frame tools return `instance-unreachable` with no line, as the liveness pre-check's; no pid, start time, path or endpoint in any line.»
