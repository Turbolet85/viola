group: architecture+test-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «Run B trusted in `<cwd>/.viola-verify-<pid>/` with four compiled pastes (the probe prompt, a long text, a tag-like text, `/clear`)»
  source: .andromeda/architecture.md:91 «gets five bracketed pastes, all compiled texts: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes, ASCII»
  replace-with: «Run B trusted in `<cwd>/.viola-verify-<pid>/` with five compiled pastes (the probe prompt, a long text, a tag-like text, a 113-byte text holding one inner LF, `/clear`)»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «check the seventeen ledger rows, print `[NN/17]` step lines»
  source: .andromeda/architecture.md:91 «It checks the eighteen rows, prints one stdout step line per row (`[NN/MM] <row id> <row words>  pass|fail`, `MM` being `LedgerRow::ALL.len()`, 18, the last `[18/18] paste-newline …`)»
  replace-with: «check the eighteen ledger rows (`LedgerRow::ALL`), print `[NN/18]` step lines (the last `[18/18] paste-newline …`)»

- leaf: .claude/docs/commands.md:27   class: omission
  leaf-text: «a version is verified only when every row reads `pass`»
  source: .andromeda/architecture.md:91 «exactly the seventeen ids every stamp written before chunk 2026-10-13-paste-newline-ledger-row holds, all `pass`, reads unverified, so every home stamped before that chunk reads `cli_verified:false`»
  replace-with: «a version is verified only when its stamp holds every landed row (`LedgerRow::ALL`, eighteen) `pass`: a seventeen-row stamp written before chunk 2026-10-13-paste-newline-ledger-row reads unverified (`cli_verified:false`) until the next `viola verify`»
