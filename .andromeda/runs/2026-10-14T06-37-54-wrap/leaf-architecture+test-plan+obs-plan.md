group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «check the eighteen ledger rows (`LedgerRow::ALL`), print `[NN/18]` step lines (the last `[18/18] paste-newline …`)»
  source: .andromeda/architecture.md:91 «`MM` being `LedgerRow::relied().len()`: 21 on Unix, the last `[21/21] statusline-rate-limits …`; 18 on Windows, the last `[18/18] paste-newline …`»
  replace-with: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows), print `[NN/MM]` step lines (`MM` the relied count; the last `[21/21] statusline-rate-limits …` on Unix, `[18/18] paste-newline …` on Windows)»

- leaf: .claude/docs/commands.md:26   class: omission
  leaf-text: «Run B trusted in `<cwd>/.viola-verify-<pid>/` with five compiled pastes»
  source: .andromeda/architecture.md:91 «On a wrapped OS (Unix), and only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
  replace-with: «Run B trusted in `<cwd>/.viola-verify-<pid>/` (on Unix, for shell-plain paths, also one probe `--settings <probe dir>/typed/statusline.json` whose status line the hidden `hook statusline --capture` arm files; none on Windows) with five compiled pastes»

- leaf: .claude/docs/commands.md:26   class: omission
  leaf-text: «Run B's `paste-<n>` / `clear-<n>` framing variants and signature-only»
  source: .andromeda/architecture.md:91 «Run B's status line captures as `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json` (`ledger::statusline_variants`»
  replace-with: «Run B's `paste-<n>` / `clear-<n>` framing variants, Run B's status line variants `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json` and signature-only»

- leaf: .claude/docs/commands.md:27   class: stale
  leaf-text: «a version is verified only when its stamp holds every landed row (`LedgerRow::ALL`, eighteen) `pass`: a seventeen-row stamp written before chunk 2026-10-13-paste-newline-ledger-row reads unverified (`cli_verified:false`) until the next `viola verify`»
  source: .andromeda/architecture.md:253 «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-one on Unix, the first eighteen on Windows), so an all-`pass` stamp of the eighteen pre-2026-10-13-owed»
  replace-with: «a version is verified only when its stamp holds every relied row (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows) `pass`: a seventeen-row stamp written before chunk 2026-10-13-paste-newline-ledger-row reads unverified (`cli_verified:false`) until the next `viola verify`, and so on Unix does an eighteen-row stamp written before chunk 2026-10-13-owed-ledger-rows (on Windows it reads verified)»
