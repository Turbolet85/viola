group: architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «the `paste-newline` row and the three statusline rows included: of the twenty-one rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows, where the status line is not wrapped) gates `cli_verified`»
  source: .andromeda/architecture.md:91 «so a Unix host relies on all twenty-two rows and a Windows host on nineteen, the first eighteen and `resume-session`, which is relied on every OS»
  replace-with: «the `paste-newline` row, the three statusline rows and the `resume-session` row included: of the twenty-two rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-two on Unix; on Windows, where the status line is not wrapped, nineteen, the first eighteen and `resume-session`) gates `cli_verified`»

- leaf: CLAUDE.md:43   class: retired
  leaf-text: «one relied-on dependence has no row yet: `claude --resume <id>` reopening a session, owed with `/compact`'s on the route entry "Resume and compact ledger rows")»
  source: .andromeda/architecture.md:473 «`resume-session`, probed by verify's Run E ([CLI Version Compatibility]), relied on every OS and stamped live on 2.1.287 on the Linux dev host»
  replace-with: «`claude --resume <id>` reopening a session is the row `resume-session`, probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"); `/compact` is on the local-command list with no post-condition, so a `send` of it reads `unconfirmable`, and its post-condition row is owed on the route entry "Compact post-condition ledger row")»

- leaf: CLAUDE.md:15   class: omission
  leaf-text: «`<Event>.<stem>-<n>.json` dialog and framing variants»
  source: .andromeda/architecture.md:422 «Run E's first SessionStart is written as `SessionStart.resume-<n>.json` (stem `resume`; `ledger::resume_variants`)»
  replace-with: «`<Event>.<stem>-<n>.json` dialog, framing and resume variants»
