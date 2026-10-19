group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:27   class: omission
  leaf-text: «prompts, `last_assistant_message`, tool `input`, plans, dialog answers, drift reports»
  source: .andromeda/security-plan.md:28 «`question` carries questions, options and each option's description and preview text (`option_details`), which also go out over CLI `wait --json` and MCP `wait`.»
  replace-with: «prompts, `last_assistant_message`, tool `input`, plans, dialog questions, options and option text (`option_details`: each option's description and preview), dialog answers, drift reports»

- leaf: .claude/docs/security-summary.md:72   class: omission
  leaf-text: «Run E, resuming Run B's first session with empty input, adds no transcript file, as measured on 2.1.287)»
  source: .andromeda/security-plan.md:261 «The print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript)»
  replace-with: «Run E, resuming Run B's first session with empty input, adds no transcript file, as measured on 2.1.287; the print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript), accepted into the same class by the founder's own word, live, 2026-10-19, relayed by the overseer, as measured on 2.1.296 and 2.1.302 on the Linux dev host)»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «(the CLI's own Run B / C / D transcripts are the accepted residual; Run E adds no transcript file)»
  source: .andromeda/security-plan.md:261 «The print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript)»
  replace-with: «(the CLI's own Run B / C / D transcripts are the accepted residual, and so is the print-mode probe's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript; the founder's own word, live, 2026-10-19); Run E adds no transcript file)»

- leaf: .claude/rules/security.md:25   class: omission
  leaf-text: «serde_json default depth (never `unbounded_depth`).»
  source: .andromeda/security-plan.md:202 «The option text crossing into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait` is a boundary widening the founder answered himself»
  replace-with: «serde_json default depth (never `unbounded_depth`). In a hook payload each question's `option_details` keeps an option's `description` / `preview` only as strings (else `null`, never refusing the dialog), and the Stop's `background_tasks` is read as `IgnoredAny` elements and carried only as its length (`null` when absent, `null` or not an array); the option text crossing into `events.ndjson`, `hook.dialog`, CLI `wait --json` and MCP `wait` is a boundary widening the founder answered himself (P4 fork Q4 of chunk 2026-10-19-overseer-switch-to-viola).»
