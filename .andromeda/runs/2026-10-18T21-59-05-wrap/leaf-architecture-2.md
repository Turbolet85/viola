group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «with five compiled pastes — the probe prompt, then the long text, the tag-like text, the newline text (`PROBE_NEWLINE_PASTE`, 113 bytes, one inner LF) and `/clear`, each added one only into a settled input box, waited for after an added turn up to `PROBE_DEADLINE`»
  source: .andromeda/architecture.md:87 «It gets six bracketed pastes, all compiled texts: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes, ASCII»
  replace-with: «with six compiled pastes — the probe prompt, then the long text, the tag-like text, the newline text (`PROBE_NEWLINE_PASTE`, 113 bytes, one inner LF), `/compact` (`PROBE_COMPACT_COMMAND`) and `/clear`, each added one only into a settled input box, waited for after an added text turn and after the `/compact` turn up to `PROBE_DEADLINE`»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «on Unix all twenty-three of `LedgerRow::ALL`, the last `agents-join`; on Windows twenty, the first eighteen, `resume-session` and `agents-join`, the last `agents-join`»
  source: .andromeda/architecture.md:87 «a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`»
  replace-with: «on Unix all twenty-four of `LedgerRow::ALL`, the last `local-command-compact`; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last `local-command-compact`»
