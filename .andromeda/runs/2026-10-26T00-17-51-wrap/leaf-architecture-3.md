group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/stack.md:47   class: omission
  leaf-text: «mock_instant 0.6.1 (in no manifest yet);»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:14 «windows-sys (the workspace pin `=0.61.2`) under `[target.'cfg(windows)'.dev-dependencies]`, with both features `Wdk_Storage_FileSystem` and `Win32_System_IO`»
  replace-with: «mock_instant 0.6.1 (in no manifest yet); the root bin's test-only Windows dev-dependency windows-sys `=0.61.2` under `[target.'cfg(windows)'.dev-dependencies]`, features `Wdk_Storage_FileSystem` + `Win32_System_IO` (the pipe-type read-back of `tests/security_negatives_channel.rs`; `Cargo.lock` unmoved, in no release build);»
