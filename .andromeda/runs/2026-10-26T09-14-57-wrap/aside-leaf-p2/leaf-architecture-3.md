group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/rules/events.md:30   class: stale
  leaf-text: «the other five snapshot readers (`hook statusline` the fifth) still call `read_snapshot`»
  source: .andromeda/architecture-detail/standard-contracts.md:25 «`read_snapshot` keeps its results, and its three product callers (`src/cmd/hook.rs` once, the one read its event and dialog paths share; `run`'s collision check; `src/cmd/hook/statusline.rs`;»
  replace-with: «the other snapshot readers still call `read_snapshot`: its three product callers (`src/cmd/hook.rs` once, the one read its event and dialog paths share; `run`'s collision check; `hook statusline`) and `viola_state::liveness::live_endpoint`, inside which the CLI frame verbs and `viola mcp` read it»

- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «Never a bare pid.»
  source: .andromeda/architecture.md:157 «`pid` and `started_at` identify the wrapper process for the liveness check and, on Linux and Windows, for the server check every frame-writing client makes before its first frame»
  replace-with: «Never a bare pid. The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`), and every frame-writing client (CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `hook.event` / `hook.dialog`, the MCP `send` / `wait` / `last` / `answer` tools) takes the liveness-only pre-check first, then compares the connected peer's pid and its OS start time with them before its first frame (`verify_server` → the closed `ServerVerdict` `Matches` · `Mismatch` · `Unverifiable`); a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid, so `server_verdict` returns `Matches` with no comparison: the recorded gap.»

- leaf: .claude/docs/stack.md:34   class: omission
  leaf-text: «are folded into architecture.md (chunk `2026-09-24-three-os-ci-headless-harness-skeleton`).»
  source: .andromeda/architecture-detail/stack-and-technologies.md:39 «- Wrapper IPC: both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()`»
  replace-with: «are folded into architecture.md (chunk `2026-09-24-three-os-ci-headless-harness-skeleton`). Below the table the section adds two notes. Wrapper IPC: both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS), the identity each frame-writing client checks before its first frame. Process liveness: sysinfo's process start time is also the observed side of that server check (the connected peer's pid and its start time against the snapshot's `pid` and `started_at`).»
