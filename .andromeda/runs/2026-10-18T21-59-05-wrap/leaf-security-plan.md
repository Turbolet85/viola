group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:36   class: stale
  leaf-text: «The ledger has twenty-three rows (six spine»
  source: .andromeda/security-plan.md:543 «The ledger has twenty-four rows (`LedgerRow::ALL`: the six spine rows,»
  replace-with: «The ledger has twenty-four rows (six spine»

- leaf: .claude/docs/security-summary.md:36   class: omission
  leaf-text: «the resume row `resume-session` and the agents row `agents-join`), and a stamp is read»
  source: .andromeda/security-plan.md:543 «the resume row `resume-session`, the agents row `agents-join` and the compact row `local-command-compact`), and a stamp is read over the relied set»
  replace-with: «the resume row `resume-session`, the agents row `agents-join` and the compact row `local-command-compact`), and a stamp is read»

- leaf: .claude/docs/security-summary.md:36   class: stale
  leaf-text: «all twenty-three on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session` and `agents-join`)»
  source: .andromeda/security-plan.md:543 «counter `/24` on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, counter `/21`)»
  replace-with: «all twenty-four on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)»

- leaf: .claude/docs/security-summary.md:36   class: stale
  leaf-text: «(`19 pass  4 fail` on Unix, or a stamp»
  source: .andromeda/security-plan.md:543 «a home stamped `20 pass  4 fail` on Unix (`17 pass  4 fail` on Windows) records `cli_verified:false`»
  replace-with: «(`20 pass  4 fail` on Unix or `17 pass  4 fail` on Windows, or a stamp»

- leaf: .claude/docs/security-summary.md:36   class: omission
  leaf-text: «of the twenty-one written before chunk 2026-10-14-resume-and-compact-ledger-rows or of the twenty-two written before chunk 2026-10-16-unwrapped-sessions-on-the-board, →»
  source: .andromeda/security-plan.md:543 «or exactly the twenty-two of a stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board, or exactly the twenty-three of a stamp written before chunk 2026-10-18-compact-post-condition-ledger-row;»
  replace-with: «of the twenty-one written before chunk 2026-10-14-resume-and-compact-ledger-rows, of the twenty-two written before chunk 2026-10-16-unwrapped-sessions-on-the-board or of the twenty-three written before chunk 2026-10-18-compact-post-condition-ledger-row, →»

- leaf: .claude/docs/security-summary.md:72   class: stale
  leaf-text: «five compiled-literal pastes (`PROBE_PROMPT`, a long text, a tag-like text, a newline text, `/clear`), each added one only into a settled input box»
  source: .andromeda/security-plan.md:115 «Run B gets six bracketed pastes, all compiled literals: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, one inner LF, no CR), `PROBE_COMPACT_COMMAND` (`/compact`»
  replace-with: «six compiled-literal pastes (`PROBE_PROMPT`, a long text, a tag-like text, a newline text, `/compact` — which compacts Run B's session in place and keeps its `session_id`, the founder's own answer to P4 fork 1 of chunk 2026-10-18-compact-post-condition-ledger-row, adding no run, start or spawn pair — and `/clear`), each added one only into a settled input box»

- leaf: .claude/docs/security-summary.md:72   class: omission
  leaf-text: «Accepted residual: Runs B, C and D each leave the CLI's synthetic-prompt transcript under `~/.claude/projects/` (+2 per verify for C and D;»
  source: .andromeda/security-plan.md:261 «Run B leaves three files per real-CLI verify: its session transcript, the `<id>-backup-<n>.jsonl` the CLI writes at its pasted `/compact`'s compaction, and the session its pasted `/clear` opens inside the same process»
  replace-with: «Accepted residual: Runs B, C and D each leave the CLI's synthetic-prompt transcript under `~/.claude/projects/` (+2 per verify for C and D; Run B leaves three files per real-CLI verify — its session transcript, the `<id>-backup-<n>.jsonl` the CLI writes at its pasted `/compact`'s compaction and the session its pasted `/clear` opens — the compaction backup the residual of the founder's own answer to P4 fork 1 of chunk 2026-10-18-compact-post-condition-ledger-row, the wrap's card confirm provisional;»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «Run B's five pastes are compiled literals (the probe prompt, a long text, a tag-like text, a newline text, `/clear`)»
  source: .andromeda/security-plan.md:115 «Run B gets six bracketed pastes, all compiled literals: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, one inner LF, no CR), `PROBE_COMPACT_COMMAND` (`/compact`»
  replace-with: «Run B's six pastes are compiled literals (the probe prompt, a long text, a tag-like text, a newline text, `/compact`, `/clear`)»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «the newline paste his own answer to P4 fork 1 of chunk 2026-10-13-paste-newline-ledger-row)»
  source: .andromeda/security-plan.md:115 «The `/compact` paste is the founder's own answer to P4 fork 1 of chunk 2026-10-18-compact-post-condition-ledger-row («Шестая вставка в Run B», relayed verbatim by the operator, that chunk's `inputs#I3`)»
  replace-with: «the newline paste his own answer to P4 fork 1 of chunk 2026-10-13-paste-newline-ledger-row; the `/compact` paste his own answer to P4 fork 1 of chunk 2026-10-18-compact-post-condition-ledger-row, adding no run, start or spawn pair)»
