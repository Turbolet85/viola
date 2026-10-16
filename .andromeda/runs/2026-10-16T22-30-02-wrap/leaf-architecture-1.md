group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/services/viola-agent-claude.md:6   class: stale
  leaf-text: «parses the statusline `rate_limits` (the `claude agents --json` read is owed to "Unwrapped sessions on the board")»
  source: .andromeda/architecture.md:280 «parsed by the pure `viola_agent_claude::agents::parse` into rows or a closed `AgentsFailure` (`oversize` · `malformed`)»
  replace-with: «parses the statusline `rate_limits` and, in the pure `agents` module, a `claude agents --json` answer (`parse` into rows or a closed `AgentsFailure`, `oversize` · `malformed`; the read itself is the root bin's `run::agents`)»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: omission
  leaf-text: «the pure `ledger` module and the pure `dialog` module)»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «the pure `agents` module (`KEYS`, `AgentStatus`, `AgentsRow` and its `joins`, `AgentsList`, `AgentsFailure`, `parse`; no I/O, re-exported from `lib.rs`»
  replace-with: «the pure `agents` module (`KEYS`, `AgentStatus`, `AgentsRow` and its `joins`, `AgentsList`, `AgentsFailure`, `parse`; no I/O, re-exported from `lib.rs`), the pure `ledger` module and the pure `dialog` module)»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: stale
  leaf-text: «`list` / `mcp` (`claude agents --json` rows; none reads them yet: CLI `list` landed without the read, owed to "Unwrapped sessions on the board")»
  source: .andromeda/architecture.md:280 «it is spawned directly by CLI `list`, by `viola revive`'s preflight and by `viola verify`'s Run B, through the root bin's `run::agents`»
  replace-with: «CLI `list`, `viola revive`'s preflight and `viola verify`'s Run B (`claude agents --json` rows through `agents::parse`, the read spawned by the root bin's `run::agents`; `mcp`'s `list` is not built)»

- leaf: .claude/docs/services/viola-agent-claude.md:23   class: stale
  leaf-text: «`claude agents --json` join field (its row and probe owed, with the read itself, to "Unwrapped sessions on the board")»
  source: .andromeda/architecture.md:83 «`claude agents --json` join field (row `agents-join`): a row exposes both the child pid (`pid`) and the session id (`sessionId`) that match a wrapped instance, as measured on 2.1.287»
  replace-with: «`claude agents --json` join field (row `agents-join` since chunk 2026-10-16-unwrapped-sessions-on-the-board: both `pid` and `sessionId` exposed, relied on every OS)»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`LedgerRow`, twenty-two closed rows»
  source: .andromeda/architecture.md:88 «the ledger (`viola_agent_claude::ledger`, a closed set) holds twenty-three rows (the members of `LedgerRow::ALL`, in its order)»
  replace-with: «`LedgerRow`, twenty-three closed rows»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`statusline-rate-limits` · `resume-session`; the per-OS relied set»
  source: .andromeda/architecture.md:88 «`statusline-rate-limits` · `resume-session` · `agents-join`. The relied set is `LedgerRow::relied()`»
  replace-with: «`statusline-rate-limits` · `resume-session` · `agents-join`; the per-OS relied set»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «all twenty-two on Unix; on Windows nineteen, the first eighteen and `resume-session`, which is relied on every OS) and `relied_here`»
  source: .andromeda/architecture.md:88 «a Unix host relies on all twenty-three rows and a Windows host on twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS»
  replace-with: «all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS) and `relied_here`»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`resume_variants` (`SessionStart.resume-1.json`); `TypedRun`, `Probes { print, typed, trusted, dialogs, statusline, resumed }`»
  source: .andromeda/architecture.md:88 «`check(row, &Probes)` reads `Probes { print, typed, trusted, dialogs, statusline, resumed, agents }`»
  replace-with: «`resume_variants` (`SessionStart.resume-1.json`); `AgentsCapture` / `agents_variant` (Run B's agents capture and its `Agents.recorded-1.json` filter); `TypedRun`, `Probes { print, typed, trusted, dialogs, statusline, resumed, agents }`»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`resumed`: Run E's captures in claim order)»
  source: .andromeda/architecture.md:88 «`agents`: Run B's one agents capture, an `AgentsCapture`): the `agents-join` row passes only when that capture parses and holds exactly one row whose `pid` is Run B's child pid»
  replace-with: «`resumed`: Run E's captures in claim order; `agents`: Run B's one agents capture, an `AgentsCapture`, on which `agents-join` passes only when it parses and holds exactly one row whose `pid` is Run B's child pid, whose `sessionId` is the `session_id` of Run B's first SessionStart capture and whose status is idle or busy (`agents_joined`), and fails with no capture)»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «all twenty-two on Unix; on Windows nineteen, the first eighteen and `resume-session`;»
  source: .andromeda/architecture.md:199 «(all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`)»
  replace-with: «all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`;»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «all `pass`, reads unverified on every OS, and one holding the eighteen and `resume-session` reads verified on Windows only; so every home stamped before that chunk reads»
  source: .andromeda/architecture.md:88 «or exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds, all `pass`, reads unverified on every OS, and so does one holding the eighteen»
  replace-with: «or exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds, all `pass`, reads unverified on every OS, and so does one holding the eighteen and `resume-session` on Windows; one holding the eighteen, `resume-session` and `agents-join` reads verified on Windows only; so every home stamped before that chunk reads»

- leaf: .claude/docs/services/viola-agent-claude.md:39   class: omission
  leaf-text: «`schemas/claude-statusline-fixture.v1.json` and every other hook payload against `claude-fixture.v1.json`»
  source: .andromeda/architecture.md:356 «validated by `schemas/claude-agents-fixture.v1.json` and hygiene-walked (`agents_fixtures_pass_hygiene`)»
  replace-with: «`schemas/claude-statusline-fixture.v1.json`, every `Agents.*.json` against `schemas/claude-agents-fixture.v1.json` (`agents_fixtures_pass_hygiene`) and every other hook payload against `claude-fixture.v1.json`»

- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «(`already-live` · `no-session` · `cwd-missing`, read in that order»
  source: .andromeda/architecture.md:129 «whose four readings (the name's holder, the logged session, `session-live`, the recorded directory; chunk 2026-10-16-unwrapped-sessions-on-the-board added the third) are taken in a fixed order»
  replace-with: «(`already-live` · `no-session` · `session-live` · `cwd-missing`, read in that order»
