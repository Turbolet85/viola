group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:42   class: stale
  leaf-text: «of the twenty-three rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-three on Unix; on Windows, where the status line is not wrapped, twenty, the first eighteen, `resume-session` and `agents-join`) gates `cli_verified`»
  source: .andromeda/architecture.md:87 «so a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS»
  replace-with: «of the twenty-four rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-four on Unix; on Windows, where the status line is not wrapped, twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) gates `cli_verified`»
- leaf: CLAUDE.md:42   class: omission
  leaf-text: «the `resume-session` row and the `agents-join` row included»
  source: .andromeda/architecture.md:87 «`statusline-rate-limits` · `resume-session` · `agents-join` · `local-command-compact`»
  replace-with: «the `resume-session` row, the `agents-join` row and the `local-command-compact` row included»
- leaf: CLAUDE.md:42   class: retired
  leaf-text: «`/compact` is on the local-command list with no post-condition, so a `send` of it reads `unconfirmable`, and its post-condition row is owed on the route entry "Compact post-condition ledger row"»
  source: .andromeda/architecture.md:410 «`local-command-compact` (cause `compact`, the same id, its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`), probed by Run B's sixth paste, relied on every OS»
  replace-with: «`/compact` on the local-command list has the post-condition a SessionStart of source `compact` under the session's own id, the row `local-command-compact`, probed by Run B's sixth paste, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"), its confirm window its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`, so a `send` of it is confirmed on a verified CLI and reads `unconfirmable` only on an unverified one»
