group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 39 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «the human/`--json` output modules (`src/human.rs` today:»
  source: .andromeda/registries/contracts/architecture/build-system.md:4 «its only human writer is the `human` module, `src/human.rs` (which holds `write_refusal`, `write_internal_error`, `write_result` and their wrappers) and its phases under `src/human/`.»
  replace-with: «the human/`--json` output modules (the `human` module: `src/human.rs`, with its phases under `src/human/` — mirror, escape, board, wait, verbs, hints —; `src/human.rs` holds»

- leaf: .claude/docs/services/viola.md:41   class: stale
  leaf-text: «`send.rs` the `send` method, one in flight with the remembered session id,»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:34 «send.rs: the `send` module root: the wrapper's `send` method on `Ctx` with its refusal order, re-exporting `SendSlot`, `append_hook_event` and `parse_from` so every caller keeps its path»
  replace-with: «`send.rs` the `send` module root — the `send` method on `Ctx` with its refusal order, re-exporting `SendSlot`, `append_hook_event` and `parse_from` —, one in flight with the remembered session id,»

- leaf: .claude/docs/services/viola.md:41   class: stale
  leaf-text: «the local-command decision over `LOCAL_COMMANDS`, the driver relabel, the `session-start` post-condition claim, the hook-event append;»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:35 «slot.rs: the one-in-flight slot (`SendSlot`) with the remembered session id, `append_hook_event` with the driver relabel, and the `session-start` post-condition claim;»
  replace-with: «and its phases under `send/`: `slot.rs` the one-in-flight `SendSlot`, `append_hook_event` with the driver relabel and the `session-start` post-condition claim; `classify.rs` the local-command decision over `LOCAL_COMMANDS` (`classify`); `params.rs` `parse` / `parse_from`; `budget.rs` `Ctx::budget_paused`;»

- leaf: .claude/docs/services/viola.md:42   class: stale
  leaf-text: «`write_paused` / `write_released` in `src/human.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:24 «verbs.rs: the `answer` line, the `pause` / `release` lines and the budget lift / open lines»
  replace-with: «`write_paused` / `write_released` in `src/human/verbs.rs`»

- leaf: .claude/docs/services/viola.md:43   class: stale
  leaf-text: «`write_answered` / `answer_hint` in `src/human.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:24 «verbs.rs: the `answer` line, the `pause` / `release` lines and the budget lift / open lines; hints.rs: the hints (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint`)»
  replace-with: «`write_answered` in `src/human/verbs.rs`, `answer_hint` in `src/human/hints.rs`»

- leaf: .claude/docs/services/viola.md:44   class: stale
  leaf-text: «the readback mirror via `src/human.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:22 «human/                  # the human module's phases: mirror.rs: `send`'s readback mirror (`[  ] open` / `[RB] read back` / `[  ] unconfirmable` with its one fixed note / `[/ ] unable`)»
  replace-with: «the readback mirror via `src/human/mirror.rs`»

- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «human lines and the message-mode escaper in `src/human.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:23 «escape.rs: the message-mode escaper (`escape_message`) and the row-mode escaper (`escape_row`)»
  replace-with: «human lines in `src/human/wait.rs` and the message-mode escaper in `src/human/escape.rs`»
