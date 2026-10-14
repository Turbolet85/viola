group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»

- leaf: .claude/docs/services/viola-agent-claude.md:22   class: stale
  leaf-text: «Its three CLI shapes have no ledger row yet (architecture, Capability ledger).»
  source: .andromeda/architecture.md:79 «Rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk 2026-10-13-owed-ledger-rows, relied only where the status line is wrapped (`statusline::WRAPPED_HERE`, Unix)»
  replace-with: «Its three CLI shapes are the ledger rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` (since chunk 2026-10-13-owed-ledger-rows), relied only where the status line is wrapped (`WRAPPED_HERE`, Unix); their Windows legs are owed to "Windows-only live measurements" (architecture, Capability ledger).»

- leaf: .claude/docs/services/viola-agent-claude.md:22   class: omission
  leaf-text: «`override_document` and `shell_argv` (`/bin/sh -c`; both `None` on Windows), `SETTINGS_FLAG`, `USER_SETTINGS`.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «`shell_argv`, `SETTINGS_FLAG`, `USER_SETTINGS`, `WRAPPED_HERE`, `BIN_SH`, `SHELL_FLAG`, `probe_document`, which reads no file, clock or environment and adds no error enum»
  replace-with: «`override_document` and `shell_argv` (`/bin/sh -c`; both `None` on Windows), `SETTINGS_FLAG`, `USER_SETTINGS`, `WRAPPED_HERE` (the status line is wrapped: Unix), `BIN_SH`, `SHELL_FLAG` and `probe_document` (the document behind Run B's probe `--settings`, `typed/statusline.json`).»

- leaf: .claude/docs/services/viola-agent-claude.md:23   class: stale
  leaf-text: «(none of the three has a row yet)»
  source: .andromeda/architecture.md:79 «Rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk 2026-10-13-owed-ledger-rows, relied only where the status line is wrapped (`statusline::WRAPPED_HERE`, Unix)»
  replace-with: «(rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk 2026-10-13-owed-ledger-rows, relied on Unix only)»

- leaf: .claude/docs/services/viola-agent-claude.md:23   class: omission
  leaf-text: «`claude agents --json` join field, plugin precedence»
  source: .andromeda/architecture.md:86 «`claude agents --json` join field: whether a row exposes the child pid or the session id that matches a wrapped instance. Its row and probe are owed, with the read itself, to "Unwrapped sessions on the board"»
  replace-with: «`claude agents --json` join field (its row and probe owed, with the read itself, to "Unwrapped sessions on the board"), plugin precedence»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`LedgerRow`, eighteen closed rows»
  source: .andromeda/architecture.md:91 «the ledger (`viola_agent_claude::ledger`, a closed set) holds twenty-one rows (the members of `LedgerRow::ALL`, in its order)»
  replace-with: «`LedgerRow`, twenty-one closed rows»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`local-command-clear` · `paste-newline`; `TypedRun`, `Probes { print, typed, trusted, dialogs }` (`trusted`: Run B's captures in claim order);»
  source: .andromeda/architecture.md:91 «`check(row, &Probes)` reads `Probes { print, typed, trusted, dialogs, statusline }` (`trusted`: Run B's captures in claim order; `statusline`: Run B's status line captures in `k` order)»
  replace-with: «`local-command-clear` · `paste-newline` · `statusline-override` · `statusline-shell` · `statusline-rate-limits`; the per-OS relied set `LedgerRow::relied()` (`ALL` filtered in its order by `relied_on(row, statusline::WRAPPED_HERE)`: all twenty-one on Unix, the first eighteen on Windows) and `relied_here`; `ShellVerdict`, `StatusLineCapture`, `statusline_variants` (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the `StatusLine.<k>` file-name helpers; `TypedRun`, `Probes { print, typed, trusted, dialogs, statusline }` (`trusted`: Run B's captures in claim order; `statusline`: Run B's status line captures in `k` order);»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`verified` (every one of the eighteen rows `pass`; a stamp holding exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds reads unverified, so such a home reads `cli_verified:false` until its next `viola verify`)»
  source: .andromeda/architecture.md:253 «verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-one on Unix, the first eighteen on Windows), so an all-`pass` stamp of the eighteen pre-2026-10-13-owed-ledger-rows ids reads»
  replace-with: «`verified` (every row of `LedgerRow::relied()` `pass`: all twenty-one on Unix, the first eighteen on Windows; a stamp holding exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds reads unverified, and on Unix so does one holding exactly the eighteen ids every stamp written before chunk 2026-10-13-owed-ledger-rows holds, which reads verified on Windows; such a home reads `cli_verified:false` until its next `viola verify`)»

- leaf: .claude/docs/services/viola-agent-claude.md:39   class: omission
  leaf-text: «the committed `fixtures/claude/*/*.json` are walked for hygiene by a run-time directory walk in `tests/contract_fixture_hygiene.rs`»
  source: .andromeda/architecture.md:426 «`tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/StatusLine.*.json` against it and every other hook payload against `claude-fixture.v1.json`.»
  replace-with: «the committed `fixtures/claude/*/*.json` are walked for hygiene by a run-time directory walk in `tests/contract_fixture_hygiene.rs`, which validates every `StatusLine.*.json` against `schemas/claude-statusline-fixture.v1.json` and every other hook payload against `claude-fixture.v1.json`»
