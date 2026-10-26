group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/services/viola-channel.md:25   class: stale
  leaf-text: «takes the liveness-only `viola_state::liveness::live_endpoint` read (`LiveEndpoint`: endpoint, pid, start time from one snapshot read), then compares»
  source: .andromeda/architecture-detail/established-decisions.md:110 «- The event and dialog paths share one connect: `read_snapshot`, then `Client::connect_by`, then `verify_server` (§Standard Contracts, Session liveness).»
  replace-with: «takes its one snapshot read (the CLI frame verbs and `viola-mcp`'s frame tools the liveness-only `viola_state::liveness::live_endpoint` read, `LiveEndpoint`: endpoint, pid, start time from one snapshot read; `viola hook`'s event and dialog paths one shared `read_snapshot` before `Client::connect_by`), then compares»
