group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «with four compiled pastes — the probe prompt, then the long text, the tag-like text and `/clear`»
  source: .andromeda/architecture.md:91 «gets five bracketed pastes, all compiled texts: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes»
  replace-with: «with five compiled pastes — the probe prompt, then the long text, the tag-like text, the newline text (`PROBE_NEWLINE_PASTE`, 113 bytes, one inner LF) and `/clear`»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «checks the seventeen ledger rows»
  source: .andromeda/architecture.md:91 «It checks the eighteen rows, prints one stdout step line per row»
  replace-with: «checks the eighteen ledger rows (`LedgerRow::ALL`, the last `paste-newline`)»

- leaf: .claude/docs/workflow.md:49   class: stale
  leaf-text: «all four typed runs and all seventeen ledger rows»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20 «all four typed runs and all eighteen ledger rows (`LedgerRow::ALL`; `tests/cli_verify.rs`, harness `boot` step 4)»
  replace-with: «all four typed runs and all eighteen ledger rows (`LedgerRow::ALL`)»
