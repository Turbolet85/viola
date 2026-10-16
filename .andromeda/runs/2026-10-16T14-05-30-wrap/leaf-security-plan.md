group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «The ledger has twenty-one rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`, the newline row `paste-newline`, and the three statusline rows `statusline-override` · `statusline-shell` · `statusline-rate-limits`)»
  source: .andromeda/security-plan.md:617 «the three statusline rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` and the resume row `resume-session`)»
  replace-with: «The ledger has twenty-two rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`, the newline row `paste-newline`, the three statusline rows `statusline-override` · `statusline-shell` · `statusline-rate-limits`, and the resume row `resume-session`)»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «(`LedgerRow::relied()`: all twenty-one on Unix; on Windows, where the status line is not wrapped, the first eighteen)»
  source: .andromeda/security-plan.md:617 «counter `/22` on Unix; on Windows, where the status line is not wrapped, the first eighteen and `resume-session`, counter `/19`)»
  replace-with: «(`LedgerRow::relied()`: all twenty-two on Unix; on Windows, where the status line is not wrapped, the first eighteen and `resume-session`)»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «(`17 pass  4 fail` on Unix»
  source: .andromeda/security-plan.md:617 «a home stamped `18 pass  4 fail` on Unix records `cli_verified:false`»
  replace-with: «(`18 pass  4 fail` on Unix»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «or on Unix one of the eighteen written before chunk 2026-10-13-owed-ledger-rows»
  source: .andromeda/security-plan.md:617 «or exactly the twenty-one of a stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows»
  replace-with: «or, on every OS, one of the eighteen written before chunk 2026-10-13-owed-ledger-rows or of the twenty-one written before chunk 2026-10-14-resume-and-compact-ledger-rows»

- leaf: .claude/docs/security-summary.md:75   class: stale
  leaf-text: «four interactive `claude` children after the print probe»
  source: .andromeda/security-plan.md:131 «spawns five interactive `claude` children under a PTY»
  replace-with: «five interactive `claude` children after the print probe»

- leaf: .claude/docs/security-summary.md:75   class: omission
  leaf-text: «settled then killed — never a byte typed into any dialog»
  source: .andromeda/security-plan.md:131 «Run E runs last, trusted in `<cwd>/.viola-verify-<pid>-resume/` (a 0700 dir its guard removes when verify passes, fails or is stopped)»
  replace-with: «settled then killed; Run E last, trusted in `<cwd>/.viola-verify-<pid>-resume/` (0700, removed like the others), with `--resume <id>` from `resume_args(id, false)`, the id the `session_id` of Run B's first SessionStart capture accepted by `is_session_id` (`ledger::resume_target`; with none it spawns nothing), no `--settings`, empty input, waiting for one SessionStart capture up to `PROBE_DEADLINE`, settled then killed (the founder's own answer to P4 fork 2 of chunk 2026-10-14-resume-and-compact-ledger-rows) — never a byte typed into any dialog»

- leaf: .claude/docs/security-summary.md:75   class: omission
  leaf-text: «(+2 per verify for C and D)»
  source: .andromeda/security-plan.md:291 «Run E, which resumes Run B's first session with `--resume <id>` from its own `-resume` dir with empty input, adds no transcript file»
  replace-with: «(+2 per verify for C and D; Run E, resuming Run B's first session with empty input, adds no transcript file, as measured on 2.1.287)»

- leaf: .claude/rules/security.md:19   class: stale
  leaf-text: «`viola verify`'s four interactive PTY children (founder-ratified live, 2026-10-05)»
  source: .andromeda/security-plan.md:131 «spawns five interactive `claude` children under a PTY»
  replace-with: «`viola verify`'s five interactive PTY children (founder-ratified live, 2026-10-05; Run E the founder's own answer to P4 fork 2 of chunk 2026-10-14-resume-and-compact-ledger-rows)»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «Run D's `plansDirectory` is a 0700 `plans/` inside its own dir;»
  source: .andromeda/security-plan.md:246 «A third source, inside `viola verify`: Run E takes the `session_id` of Run B's first SessionStart capture through `viola_agent_claude::ledger::resume_target`, only when `is_session_id` accepts it»
  replace-with: «Run D's `plansDirectory` is a 0700 `plans/` inside its own dir; Run E runs last with empty input (no key typed) and no `--settings`, its one `--resume <id>` built by `resume_args(id, false)` from the `session_id` of Run B's first SessionStart capture only when `is_session_id` accepts it (`ledger::resume_target`; with none it spawns nothing);»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «`<cwd>/.viola-verify-<pid>/`, `-dialogs/`, `-plan/` are 0700»
  source: .andromeda/security-plan.md:283 «and Run E's `<cwd>/.viola-verify-<pid>-resume/`. All are 0700 and removed when verify passes, fails, or is stopped by Ctrl-C or TERM»
  replace-with: «`<cwd>/.viola-verify-<pid>/`, `-dialogs/`, `-plan/`, `-resume/` are 0700»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «(the CLI's own Run B / C / D transcripts are the accepted residual)»
  source: .andromeda/security-plan.md:291 «Run E, which resumes Run B's first session with `--resume <id>` from its own `-resume` dir with empty input, adds no transcript file»
  replace-with: «(the CLI's own Run B / C / D transcripts are the accepted residual; Run E adds no transcript file)»
