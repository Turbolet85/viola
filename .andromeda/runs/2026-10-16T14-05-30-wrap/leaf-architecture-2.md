group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «then the four interactive PTY runs»
  source: .andromeda/architecture.md:91 «It then drives five interactive PTY runs (`src/cmd/verify/typed.rs`).»
  replace-with: «then the five interactive PTY runs»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «Run D's plan in `-plan/`;»
  source: .andromeda/architecture.md:100 «Run E, trusted in `<cwd>/.viola-verify-<pid>-resume/`, loading the capture plugin from `ledger/probes/<pid>/resume/plugin/` and resuming Run B's first session with `--resume <id>`, no key typed.»
  replace-with: «Run D's plan in `-plan/`, then last Run E trusted in `-resume/`, resuming Run B's first session with `--resume <id>` (`viola_agent_claude::resume_args(id, false)`, the id Run B's first SessionStart `session_id` through `ledger::resume_target`; empty input, no `--settings`, ended by a kill);»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «on Unix all twenty-one of `LedgerRow::ALL`, the last `statusline-rate-limits`; on Windows the first eighteen, the last `paste-newline`»
  source: .andromeda/architecture.md:91 «`MM` being `LedgerRow::relied().len()`: 22 on Unix, the last `[22/22] resume-session …`; 19 on Windows, the last `[19/19] resume-session …`)»
  replace-with: «on Unix all twenty-two of `LedgerRow::ALL`, the last `resume-session`; on Windows nineteen, the first eighteen and `resume-session`, the last `resume-session`»

- leaf: .claude/docs/services/viola.md:35   class: stale
  leaf-text: «`verify`'s six child spawns are each logged as a `process-start`/`process-exit` pair at the call site: the `--version` read as `version-probe`, the print-mode probe as `verify-probe`, and each of the four interactive PTY runs as `verify-pty-probe`»
  source: .andromeda/architecture.md:406 «seven spawn pairs between them — `version-probe` around the `--version` read, `verify-probe` around the print-mode probe and one `verify-pty-probe` pair around each interactive run»
  replace-with: «`verify`'s seven child spawns are each logged as a `process-start`/`process-exit` pair at the call site: the `--version` read as `version-probe`, the print-mode probe as `verify-probe`, and each of the five interactive PTY runs (Run A, Run B, Run C, Run D, then Run E) as `verify-pty-probe`»

- leaf: .claude/docs/services/viola.md:35   class: stale
  leaf-text: «a child killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and no `child_exit_status`»
  source: .andromeda/architecture.md:406 «a child of an interactive PTY run killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and a `child_exit_status`, the PTY pump's exit code»
  replace-with: «a child of an interactive PTY run killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and a `child_exit_status`, the PTY pump's exit code (portable-pty reports a code for a signalled child; a print-mode or `--version` probe child killed at the stop is not measured)»

- leaf: .claude/docs/stack.md:14   class: stale
  leaf-text: «the screen of `viola verify`'s four interactive probe runs. `run` never reads content; `Screen::rows()` is the only row-text exit, read by `verify` alone for signature-only screen fixtures»
  source: .andromeda/architecture.md:18 «the screen of `viola verify`'s five interactive probe runs. `run` never reads content; `Screen::rows()` is the only row-text exit, read by `verify` alone for its signature-only `--record` screen fixtures»
  replace-with: «the screen of `viola verify`'s five interactive probe runs. `run` never reads content; `Screen::rows()` is the only row-text exit, read by `verify` alone for its signature-only `--record` screen fixtures»
