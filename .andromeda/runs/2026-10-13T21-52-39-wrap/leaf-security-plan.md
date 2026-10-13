group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:19   class: stale
  leaf-text: «Run B's four pastes are compiled literals (the probe prompt, a long text, a tag-like text, `/clear`), each added one only into a settled input box, waited for up to the probe deadline (the founder's live answers, 2026-10-06)»
  source: .andromeda/security-plan.md:131 «gets five bracketed pastes, all compiled literals: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE`»
  replace-with: «Run B's five pastes are compiled literals (the probe prompt, a long text, a tag-like text, a newline text, `/clear`), each added one only into a settled input box, waited for up to the probe deadline (the founder's live answers, 2026-10-06; the newline paste his own answer to P4 fork 1 of chunk 2026-10-13-paste-newline-ledger-row)»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «The stamp holds seventeen rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`), so a decision flows only when every row passes (`13 pass  4 fail`, or a stamp of the fourteen older ids all `pass`, → `cli_verified:false`, `answer` exit 12)»
  source: .andromeda/security-plan.md:617 «A stamp holds the eighteen ledger rows (`LedgerRow::ALL`, counter `/18`: the six spine rows, the four screen and timing rows, the four dialog-tier rows»
  replace-with: «The stamp holds eighteen rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`, and the newline row `paste-newline`), so a decision flows only when every row passes (`14 pass  4 fail`, or a stamp of the fourteen older ids or of the seventeen written before chunk 2026-10-13-paste-newline-ledger-row all `pass`, → `cli_verified:false`, `answer` exit 12)»

- leaf: .claude/docs/security-summary.md:75   class: stale
  leaf-text: «one `PROBE_PROMPT` paste, Ctrl-C ×2 then kill»
  source: .andromeda/security-plan.md:131 «gets five bracketed pastes, all compiled literals: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE`»
  replace-with: «five compiled-literal pastes (`PROBE_PROMPT`, a long text, a tag-like text, a newline text, `/clear`), each added one only into a settled input box, Ctrl-C ×2 then kill»
