group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/rules/events.md:31   class: stale
  leaf-text: «takes the liveness-only pre-check first, then compares the connected peer's pid and its OS start time with them before its first frame»
  source: .andromeda/architecture-detail/established-decisions.md:110 «- The event and dialog paths share one connect: `read_snapshot`, then `Client::connect_by`, then `verify_server` (§Standard Contracts, Session liveness).»
  replace-with: «takes its snapshot read first (the CLI frame verbs and the MCP tools the liveness-only pre-check through `live_endpoint`, `viola hook`'s event and dialog paths one shared `read_snapshot`, then `Client::connect_by`), then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame»

- leaf: .claude/docs/stack.md:14   class: omission
  leaf-text: «the Windows open leaves the server able to identify, never impersonate, the client (measured)»
  source: .andromeda/architecture-detail/stack-and-technologies.md:39 «- Wrapper IPC: both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()`»
  replace-with: «the Windows open leaves the server able to identify, never impersonate, the client (measured); both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS), the identity each frame-writing client checks before its first frame»

- leaf: .claude/docs/stack.md:25   class: omission
  leaf-text: «Enriches the primary signal, the heartbeat file»
  source: .andromeda/architecture-detail/stack-and-technologies.md:45 «- Process liveness: sysinfo's process start time is also the observed side of the server check each frame-writing client makes before its first frame»
  replace-with: «Enriches the primary signal, the heartbeat file; sysinfo's process start time is also the observed side of the server check each frame-writing client makes before its first frame (the connected peer's pid and its start time against the snapshot's `pid` and `started_at`)»
