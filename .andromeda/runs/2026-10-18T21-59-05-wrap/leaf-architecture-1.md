group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/gotchas.md:84   class: stale
  leaf-text: «`/compact` is listed with no post-condition and no probe types it (its post-condition row owed to the working-route entry "Compact post-condition ledger row"), so a `send` of it is `unconfirmable` at once on a verified and an unverified CLI alike.»
  source: .andromeda/architecture.md:46 «`/compact` on a verified version waits its own window for its post-condition only, and no `prompt-submitted` claims it»
  replace-with: «`/compact` is listed with the post-condition a SessionStart with source `compact` and the session's own `session_id` (row `local-command-compact`, probed by Run B's `/compact` paste and stamped on 2.1.287 on the Linux dev host at chunk 2026-10-18-compact-post-condition-ledger-row, its answer read 21 413 ms after the paste): on a verified version a `send` of it waits its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) for a `session-start` of cause `compact` whose id equals the remembered one, no `prompt-submitted` claims it, and a missed post-condition expires at 40 s as `not-delivered` / `no-prompt-submitted`; on an unverified CLI it is `unconfirmable` at once. No live driver `send` of `/compact` has been made.»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: omission
  leaf-text: «the compiled `QUIET_PERIOD` 300 ms / `GATE_MAX_WAIT` 8.5 s / `CONFIRM_WINDOW_FALLBACK` 10 s, validated per CLI version by the stamped rows»
  source: .andromeda/architecture.md:46 «a `/compact` send waiting for its post-condition has its own, `COMPACT_CONFIRM_WINDOW` = 40 s (PROVISIONAL, validated per CLI version by the `local-command-compact` row against the measured `compact_latency_ms`)»
  replace-with: «the compiled `QUIET_PERIOD` 300 ms / `GATE_MAX_WAIT` 8.5 s / `CONFIRM_WINDOW_FALLBACK` 10 s / `COMPACT_CONFIRM_WINDOW` 40 s (PROVISIONAL, a `/compact` send's own window, validated by the `local-command-compact` row against the measured `compact_latency_ms`), validated per CLI version by the stamped rows»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`LedgerRow`, twenty-three closed rows»
  source: .andromeda/architecture.md:87 «the ledger (`viola_agent_claude::ledger`, a closed set) holds twenty-four rows (the members of `LedgerRow::ALL`, in its order)»
  replace-with: «`LedgerRow`, twenty-four closed rows»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «· `agents-join`; the per-OS relied set»
  source: .andromeda/architecture.md:87 «`resume-session` · `agents-join` · `local-command-compact`. The relied set is `LedgerRow::relied()`»
  replace-with: «· `agents-join` · `local-command-compact`; the per-OS relied set»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS»
  source: .andromeda/architecture.md:87 «a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS»
  replace-with: «all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «(`agents_joined`), and fails with no capture);»
  source: .andromeda/architecture.md:87 «the compact row only when the first SessionStart capture of source `compact` after the newline turn's prompt (`compact_start`) has a non-empty `session_id`»
  replace-with: «(`agents_joined`), and fails with no capture; the `local-command-compact` row passes only when the first SessionStart capture of source `compact` after the newline turn's prompt (`compact_start`) has a non-empty `session_id` equal to that of Run B's first SessionStart capture, no UserPromptSubmit carries `/compact`, and `compact_latency_ms` is within `COMPACT_CONFIRM_WINDOW` (`local_command_compacted`));»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «Run B's compiled paste texts (`PROBE_PROMPT` and `PROBE_NEWLINE_PASTE` among them: 113 bytes, ASCII, exactly one LF at neither end, no CR);»
  source: .andromeda/architecture.md:87 «`PROBE_NEWLINE_PASTE` (113 bytes, ASCII, exactly one LF at neither end, no CR), `PROBE_COMPACT_COMMAND` (the list's `/compact`) and `PROBE_LOCAL_COMMAND` (the list's `/clear`)»
  replace-with: «Run B's six compiled paste texts (`PROBE_PROMPT`, `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, exactly one LF at neither end, no CR) and `PROBE_COMPACT_COMMAND` (the list's `/compact`) among them);»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`FRAMING_TURNS` (in paste order `paste-1` · `paste-2` · `paste-3` · `clear-1`)»
  source: .andromeda/architecture.md:87 «(`ledger::FRAMING_TURNS`, in paste order `paste-1` · `paste-2` · `paste-3` · `compact-1` · `clear-1`)»
  replace-with: «`FRAMING_TURNS` (in paste order `paste-1` · `paste-2` · `paste-3` · `compact-1` · `clear-1`)»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «every row of `LedgerRow::relied()` `pass`: all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`;»
  source: .andromeda/architecture.md:198 «version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)»
  replace-with: «every row of `LedgerRow::relied()` `pass`: all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`;»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «or exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds, all `pass`, reads unverified on every OS, and so does one holding the eighteen and `resume-session` on Windows; one holding the eighteen, `resume-session` and `agents-join` reads verified on Windows only; so every home stamped before that chunk reads `cli_verified:false` until its next `viola verify`»
  source: .andromeda/architecture.md:87 «or exactly the twenty-three ids every stamp written before chunk 2026-10-18-compact-post-condition-ledger-row holds, all `pass`, reads unverified on every OS»
  replace-with: «or exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds, or exactly the twenty-three ids every stamp written before chunk 2026-10-18-compact-post-condition-ledger-row holds, all `pass`, reads unverified on every OS, and so do one holding the eighteen and `resume-session` and one holding the eighteen, `resume-session` and `agents-join` on Windows; one holding the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only; so every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false` until its next `viola verify`»

- leaf: .claude/docs/services/viola-agent-claude.md:33   class: stale
  leaf-text: «`/compact` is listed with none (its post-condition row owed to "Compact post-condition ledger row"), so its `send` is `unconfirmable` at once on a verified and an unverified CLI alike.»
  source: .andromeda/architecture.md:84 «so a `send` of it is confirmed on a verified CLI and `unconfirmable` at once only on an unverified one.»
  replace-with: «`/compact`'s is a SessionStart `compact` with the session's own `session_id` (row `local-command-compact`, its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`), so its `send` is confirmed on a verified CLI and `unconfirmable` at once only on an unverified one.»
