group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/services/viola-agent-claude.md:6   class: omission
  leaf-text: «It also holds the resume shapes `viola revive` launches with:»
  source: .andromeda/architecture.md:372 «then `--resume <id>` built by `viola_agent_claude::resume_args(id, false)`, its second caller beside `viola revive`»
  replace-with: «It also holds the resume shapes `viola revive` launches with (`viola verify`'s Run E is the second caller of `resume_args(id, false)`):»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: stale
  leaf-text: «`viola verify`'s four interactive runs feed their own `Screen` the same way.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «and `viola verify`'s five interactive probe runs feed their own»
  replace-with: «`viola verify`'s five interactive runs feed their own `Screen` the same way.»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`LedgerRow`, twenty-one closed rows»
  source: .andromeda/architecture.md:91 «holds twenty-two rows (the members of `LedgerRow::ALL`, in its order)»
  replace-with: «`LedgerRow`, twenty-two closed rows»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`statusline-rate-limits`; the per-OS relied set»
  source: .andromeda/architecture.md:91 «`statusline-rate-limits` · `resume-session`. The relied set is»
  replace-with: «`statusline-rate-limits` · `resume-session`; the per-OS relied set»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «by `relied_on(row, statusline::WRAPPED_HERE)`: all twenty-one on Unix, the first eighteen on Windows) and `relied_here`»
  source: .andromeda/architecture.md:91 «so a Unix host relies on all twenty-two rows and a Windows host on nineteen, the first eighteen and `resume-session`, which is relied on every OS.»
  replace-with: «by `relied_on(row, statusline::WRAPPED_HERE)`: all twenty-two on Unix; on Windows nineteen, the first eighteen and `resume-session`, which is relied on every OS) and `relied_here`»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`Probes { print, typed, trusted, dialogs, statusline }` (`trusted`: Run B's captures in claim order; `statusline`: Run B's status line captures in `k` order)»
  source: .andromeda/architecture.md:91 «reads `Probes { print, typed, trusted, dialogs, statusline, resumed }`»
  replace-with: «`Probes { print, typed, trusted, dialogs, statusline, resumed }` (`trusted`: Run B's captures in claim order; `statusline`: Run B's status line captures in `k` order; `resumed`: Run E's captures in claim order)»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: omission
  leaf-text: «`statusline_variants` (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the `StatusLine.<k>` file-name helpers;»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «`resume_target` / `resume_variants` (Run E's target id and its `resume-1` variant)»
  replace-with: «`statusline_variants` (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the `StatusLine.<k>` file-name helpers; `resume_target` (Run E's target id: the `session_id` of Run B's first SessionStart capture, only when `is_session_id` accepts it) and `resume_variants` (`SessionStart.resume-1.json`);»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`verified` (every row of `LedgerRow::relied()` `pass`: all twenty-one on Unix, the first eighteen on Windows; a stamp holding exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds reads unverified, and on Unix so does one holding exactly the eighteen ids every stamp written before chunk 2026-10-13-owed-ledger-rows holds, which reads verified on Windows; such a home reads `cli_verified:false` until its next `viola verify`)»
  source: .andromeda/architecture.md:91 «or exactly the twenty-one ids every stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows holds, all `pass`, reads unverified on every OS, and one holding the eighteen and `resume-session` reads verified»
  replace-with: «`verified` (every row of `LedgerRow::relied()` `pass`: all twenty-two on Unix; on Windows nineteen, the first eighteen and `resume-session`; a stamp holding exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds, or exactly the eighteen ids every stamp written before chunk 2026-10-13-owed-ledger-rows holds, or exactly the twenty-one ids every stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows holds, all `pass`, reads unverified on every OS, and one holding the eighteen and `resume-session` reads verified on Windows only; so every home stamped before that chunk reads `cli_verified:false` until its next `viola verify`)»

- leaf: .claude/docs/services/viola-agent-claude.md:33   class: omission
  leaf-text: «`/clear`'s post-condition is SessionStart `clear` + a new `session_id`.»
  source: .andromeda/architecture.md:88 «`/compact` (listed since chunk 2026-10-14-resume-and-compact-ledger-rows) has none either and no probe types it, so a `send` of it is `unconfirmable` at once on a verified and an unverified CLI alike»
  replace-with: «`/clear`'s post-condition is SessionStart `clear` + a new `session_id`; `/compact` is listed with none (its post-condition row owed to "Compact post-condition ledger row"), so its `send` is `unconfirmable` at once on a verified and an unverified CLI alike.»

- leaf: .claude/docs/gotchas.md:84   class: omission
  leaf-text: «Skills are not local commands.»
  source: .andromeda/architecture.md:88 «`/compact` (listed since chunk 2026-10-14-resume-and-compact-ledger-rows) has none either and no probe types it, so a `send` of it is `unconfirmable` at once on a verified and an unverified CLI alike»
  replace-with: «`/compact` is listed with no post-condition and no probe types it (its post-condition row owed to the working-route entry "Compact post-condition ledger row"), so a `send` of it is `unconfirmable` at once on a verified and an unverified CLI alike. Skills are not local commands.»
