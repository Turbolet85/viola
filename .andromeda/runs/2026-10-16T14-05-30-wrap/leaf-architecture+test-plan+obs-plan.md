group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «drive one print-mode probe (Haiku) and four interactive PTY runs»
  source: .andromeda/architecture.md:91 «It then drives five interactive PTY runs (`src/cmd/verify/typed.rs`).»
  replace-with: «drive one print-mode probe (Haiku) and five interactive PTY runs»

- leaf: .claude/docs/commands.md:26   class: omission
  leaf-text: «Run D's plan in `-plan/`; no key into any dialog»
  source: .andromeda/architecture.md:100 «and Run E, trusted in `<cwd>/.viola-verify-<pid>-resume/`, loading the capture plugin from `ledger/probes/<pid>/resume/plugin/` and resuming Run B's first session with `--resume <id>`, no key typed.»
  replace-with: «Run D's plan in `-plan/`, then Run E in `-resume/` resuming Run B's first session with `--resume <id>` (input empty, no `--settings`); no key into any dialog»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows)»
  source: .andromeda/architecture.md:91 «so a Unix host relies on all twenty-two rows and a Windows host on nineteen, the first eighteen and `resume-session`, which is relied on every OS.»
  replace-with: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-two on Unix; nineteen on Windows, the first eighteen and `resume-session`)»

- leaf: .claude/docs/commands.md:26   class: stale
  leaf-text: «the last `[21/21] statusline-rate-limits …` on Unix, `[18/18] paste-newline …` on Windows»
  source: .andromeda/architecture.md:91 «22 on Unix, the last `[22/22] resume-session …`; 19 on Windows, the last `[19/19] resume-session …`»
  replace-with: «the last `[22/22] resume-session …` on Unix, `[19/19] resume-session …` on Windows»

- leaf: .claude/docs/commands.md:26   class: omission
  leaf-text: «Run B's status line variants `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json` and signature-only»
  source: .andromeda/architecture.md:91 «Run E's first SessionStart as `SessionStart.resume-1.json` (`ledger::resume_variants`) and the signature-only `Screen.<phase>.json` screens the fake agent replays, only when 0 rows fail.»
  replace-with: «Run B's status line variants `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json`, Run E's first SessionStart as `SessionStart.resume-1.json` and signature-only»

- leaf: .claude/docs/commands.md:27   class: stale
  leaf-text: «every relied row (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows) `pass`»
  source: .andromeda/architecture.md:253 «reads `pass` (all twenty-two on Unix; on Windows nineteen, the first eighteen and `resume-session`)»
  replace-with: «every relied row (`LedgerRow::relied()`: all twenty-two on Unix; nineteen on Windows, the first eighteen and `resume-session`) `pass`»

- leaf: .claude/docs/commands.md:27   class: stale
  leaf-text: «and so on Unix does an eighteen-row stamp written before chunk 2026-10-13-owed-ledger-rows (on Windows it reads verified)»
  source: .andromeda/architecture.md:253 «or of the twenty-one pre-2026-10-14-resume-and-compact-ledger-rows ids reads unverified on every OS, and one of the eighteen and `resume-session` reads verified on Windows only.»
  replace-with: «and so on every OS does an eighteen-row stamp written before chunk 2026-10-13-owed-ledger-rows or a twenty-one-row stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows (a stamp of the eighteen and `resume-session` reads verified on Windows only)»
