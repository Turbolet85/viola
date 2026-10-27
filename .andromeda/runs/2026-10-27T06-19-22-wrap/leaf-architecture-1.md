group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »

- leaf: .claude/docs/services/viola-agent-claude.md:21   class: omission
  leaf-text: «every external reader bounded by `Read::take(MAX_FRAME)`.»
  source: .andromeda/architecture.md:422 «- **Bounded readers.** Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounde»
  replace-with: «every product reader of external bytes bounded in its own statement (`take`) or named as an exception (the own-executable reads), held by the census in `tests/contract_bounded_readers.rs`; serde_json's `unbounded_depth` is banned in `deny.toml`, so every parse keeps the default recursion limit.»
