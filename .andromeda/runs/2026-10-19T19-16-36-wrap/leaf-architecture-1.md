group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/services/viola-channel.md:25   class: stale
  leaf-text: «Interim ratified exception (security-plan Decisions Log `2026-09-28`): `viola hook`'s `hook.event` goes without it until Epoch 6.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «is the liveness-only endpoint read shared by CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools»
  replace-with: «Interim ratified exception (security-plan Decisions Log `2026-09-28`): `viola hook`'s `hook.event` goes without it until Epoch 6; and CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools write their frame after only the liveness-only endpoint read (`viola_state::liveness::live_endpoint`), the server check owed to the Epoch 6 entry "Server verification before any frame".»

- leaf: .claude/docs/services/viola-agent-claude.md:25   class: omission
  leaf-text: «`normalise(event, bytes) -> Normalised{kind, data, drift}`»
  source: .andromeda/architecture.md:255 «`background_tasks` is the Stop payload's `background_tasks` array length, a count only (no element is read or carried), or `null` when the key is absent, `null` or not an array.»
  replace-with: «`normalise(event, bytes) -> Normalised{kind, data, drift}` (Stop's `turn-ended` data is `{last_assistant_message, background_tasks}`: `background_tasks` the payload array's length only, no element read or carried, `null` when the key is absent, `null` or not an array)»

- leaf: .claude/docs/services/viola-agent-claude.md:24   class: omission
  leaf-text: «`classify` (one dialog payload → its closed kind / tool; malformed → `AgentError::DialogMalformed`, the hook fails open)»
  source: .andromeda/architecture.md:256 «`option_details` is parallel to `options`, one per option, each field the option's string or `null` (a missing or non-string value never refuses the dialog)»
  replace-with: «`classify` (one dialog payload → its closed kind / tool; malformed → `AgentError::DialogMalformed`, the hook fails open; a `question`'s data is `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`, `option_details` one per option, each field the option's string or `null`, a missing or non-string value never refusing the dialog, the option text upstream content never written into a human line)»
