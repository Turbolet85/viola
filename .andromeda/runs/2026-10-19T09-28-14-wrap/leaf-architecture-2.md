group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola.md:41   class: stale
  leaf-text: «the Condvar await on the injected clock up to `DIALOG_DEADLINE`»
  source: .andromeda/architecture.md:66 «waits on `hook.dialog` for at most the instance's dialog hold, `config.json`'s `dialog_hold_s`: whole seconds from 60 to 3600, default 60»
  replace-with: «the Condvar await on the injected clock up to the instance's dialog hold (`config.json`'s `dialog_hold_s`, a `DialogHold` of whole seconds 60–3600, default 60, read once at the wrapper's start through `run::dialog::read_hold` right after the thresholds), at whose end the wrapper replies `null` and appends one `dialog-expired` line (`{dialog_id, dialog_kind}`, `source: wrapper`) through `WaitFeed::appending`, waking a parked `wait`»

- leaf: .claude/docs/services/viola.md:43   class: stale
  leaf-text: «read their reply under `DIALOG_DEADLINE + 5 s` and write stdout once»
  source: .andromeda/architecture.md:66 «the hook bounds its reply read at the ceiling, `DIALOG_HOLD_MAX + 5 s` (3605 s), never at each start's hold, and writes stdout once»
  replace-with: «read their reply under the ceiling `DIALOG_HOLD_MAX + 5 s` (3605 s), never at each start's hold, and write stdout once»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «(`config.json` is read by the root bin, `run::budget::read_thresholds`, not here)»
  source: .andromeda/architecture.md:354 «`dialog_hold_s`: the dialog hold, a JSON whole number of seconds from 60 to 3600, default 60, read by the wrapper once at its start through `run::dialog::read_hold` right after the thresholds»
  replace-with: «(`config.json` is read by the root bin, `run::budget::read_thresholds` and `run::dialog::read_hold`, not here)»

- leaf: .claude/docs/services/viola-state.md:45   class: stale
  leaf-text: «the 15 kinds round-trip»
  source: .andromeda/architecture.md:159 «`send-issued`, `send-confirmed`, `send-refused`, `dialog-expired`. New kinds are added only in `viola-core`, never with Claude-specific names.»
  replace-with: «the 16 kinds round-trip»
