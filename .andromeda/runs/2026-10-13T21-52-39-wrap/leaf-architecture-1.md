group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»

- leaf: .claude/docs/services/viola-agent-claude.md:25   class: stale
  leaf-text: «every multi-line `send` relies on it and it has no ledger row yet (owed to `v1-34`, working-route entry "Paste newline ledger row")»
  source: .andromeda/architecture.md:49 «Every multi-line `send` relies on that shape, and it is gated by the ledger row `paste-newline` and its `viola verify` probe, Run B's `PROBE_NEWLINE_PASTE` ([CLI Version Compatibility])»
  replace-with: «every multi-line `send` relies on it, and it is gated by the ledger row `paste-newline` and its `viola verify` probe, Run B's `PROBE_NEWLINE_PASTE` (landed at chunk 2026-10-13-paste-newline-ledger-row, stamped on live 2.1.287 on the Linux dev host only; its Windows reading owed to "Windows-only live measurements")»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`LedgerRow`, seventeen closed rows `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload` · `modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window` · `question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency` · `long-paste-wrapper` · `tag-escaping` · `local-command-clear`; `TypedRun`, `Probes { print, typed, trusted, dialogs }` (`trusted`: Run B's captures in claim order); `PROBE_PROMPT`;»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «the pure `ledger` module: the eighteen rows (`LedgerRow::ALL`), Run B's compiled paste texts (`PROBE_NEWLINE_PASTE` among them)»
  replace-with: «`LedgerRow`, eighteen closed rows (the members of `LedgerRow::ALL`, in its order) `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload` · `modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window` · `question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency` · `long-paste-wrapper` · `tag-escaping` · `local-command-clear` · `paste-newline`; `TypedRun`, `Probes { print, typed, trusted, dialogs }` (`trusted`: Run B's captures in claim order); Run B's compiled paste texts (`PROBE_PROMPT` and `PROBE_NEWLINE_PASTE` among them: 113 bytes, ASCII, exactly one LF at neither end, no CR); `FRAMING_TURNS` (in paste order `paste-1` · `paste-2` · `paste-3` · `clear-1`) and `framing_stem`;»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`verified` (every row `pass`)»
  source: .andromeda/architecture.md:91 «or exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds, all `pass`, reads unverified, so every home stamped before that chunk reads `cli_verified:false`»
  replace-with: «`verified` (every one of the eighteen rows `pass`; a stamp holding exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds reads unverified, so such a home reads `cli_verified:false` until its next `viola verify`)»
