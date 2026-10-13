group: architecture+security-plan+design-system+layout-templates+test-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «the S3/S7/S8 dialog bodies and the paste-framing and `/clear` rows included: seventeen rows gate `cli_verified`»
  source: .andromeda/architecture.md:91 «the ledger (`viola_agent_claude::ledger`, a closed set) holds eighteen rows (the members of `LedgerRow::ALL`, in its order)»
  replace-with: «the S3/S7/S8 dialog bodies, the paste-framing and `/clear` rows and the `paste-newline` row included: eighteen rows (`LedgerRow::ALL`) gate `cli_verified`»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «three relied-on dependences have no row yet: an LF typed inside a paste and `claude --resume <id>` reopening a session, both owed on the route entry "Paste newline ledger row", and the statusline's `rate_limits` payload (which the `budget-paused` send refusal now rests on), `--settings` override and `/bin/sh` shell, owed on "Paste newline ledger row" and, for Windows, on "Windows-only live measurements")»
  source: .andromeda/architecture.md:472 «Two relied-on dependences have no row yet and are not under that limit: one of `viola revive`'s, and the statusline pass-through's three shapes.»
  replace-with: «an LF typed inside a paste is gated by the `paste-newline` row, stamped on 2.1.287 on the Linux dev host only (its Windows reading owed on "Windows-only live measurements"); two relied-on dependences have no row yet: `claude --resume <id>` reopening a session, and the statusline's `rate_limits` payload (which the `budget-paused` send refusal now rests on), `--settings` override and `/bin/sh` shell, both owed on the route entry "Owed ledger rows" and, for Windows, on "Windows-only live measurements")»
