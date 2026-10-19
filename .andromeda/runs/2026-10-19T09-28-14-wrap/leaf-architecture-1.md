group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/conventions.md:45   class: omission
  leaf-text: «`claude_env_keep` (the Unix R8 pass-list of `CLAUDE*` names, default empty).»
  source: .andromeda/architecture.md:419 «and `dialog_hold_s` (the dialog hold as a `DialogHold`, whole seconds 60–3600, default 60)»
  replace-with: «`claude_env_keep` (the Unix R8 pass-list of `CLAUDE*` names, default empty), `dialog_hold_s` (the dialog hold as a `DialogHold`, whole seconds 60–3600, default 60, read once at the wrapper's start; any other value takes 60 with one `parse-rejected` line, so no value makes the hold zero or unbounded).»

- leaf: .claude/docs/services/viola-core.md:6   class: omission
  leaf-text: «the nutype newtypes `ViolaName` and `Percent`,»
  source: .andromeda/architecture.md:106 «nutype newtypes `ViolaName`, `Percent` and `DialogHold`.»
  replace-with: «the nutype newtypes `ViolaName`, `Percent` and `DialogHold` (whole seconds 60–3600),»

- leaf: .claude/docs/services/viola-core.md:6   class: retired
  leaf-text: «`DIALOG_DEADLINE` (60 s PROVISIONAL, the dialog hooks' wait, below the `hooks.json` `timeout` of 75)»
  source: .andromeda/architecture.md:66 «`config.json`'s `dialog_hold_s`: whole seconds from 60 to 3600, default 60 (`viola_core::DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL, beside `SPINE_DEADLINE`)»
  replace-with: «`DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX` (60 s · 60 s · 3600 s, PROVISIONAL: the dialog hold's default and bounds, `config.json`'s `dialog_hold_s` read once at the wrapper's start into a `DialogHold`; the hook bounds its reply read at `DIALOG_HOLD_MAX + 5 s`, below the `hooks.json` `timeout` of 3615)»

- leaf: .claude/docs/services/viola-core.md:11   class: omission
  leaf-text: «nutype 0.8.0 (`ViolaName`, `Percent`)»
  source: .andromeda/architecture.md:25 «| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100), `DialogHold` (whole seconds 60–3600) |»
  replace-with: «nutype 0.8.0 (`ViolaName`, `Percent`, `DialogHold`)»

- leaf: .claude/docs/services/viola-core.md:20   class: stale
  leaf-text: «`EventKind` holds 15 kinds (the length of `EventKind::ALL`), the dialog kinds `question` / `permission` / `plan` among them and `link` / `unlink` (`data` keys `driver`, `driven`) the last two;»
  source: .andromeda/architecture.md:159 «`send-issued`, `send-confirmed`, `send-refused`, `dialog-expired`. New kinds are added only in `viola-core`, never with Claude-specific names.»
  replace-with: «`EventKind` holds 16 kinds (the length of `EventKind::ALL`), the dialog kinds `question` / `permission` / `plan` among them, `link` / `unlink` (`data` keys `driver`, `driven`) and, last, `dialog-expired` (`data` keys `dialog_id`, `dialog_kind`; `source: wrapper`, appended once when a held dialog's hold ends);»

- leaf: .claude/docs/services/viola-core.md:20   class: stale
  leaf-text: «`EventKind::WAIT_WAKE` is the one wake set (`turn-ended`, `question`, `permission`, `plan`, `session-end`)»
  source: .andromeda/architecture.md:237 «Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake (`viola_core::EventKind::WAIT_WAKE`)»
  replace-with: «`EventKind::WAIT_WAKE` is the one wake set (`turn-ended`, `question`, `permission`, `plan`, `session-end`, `dialog-expired`)»
