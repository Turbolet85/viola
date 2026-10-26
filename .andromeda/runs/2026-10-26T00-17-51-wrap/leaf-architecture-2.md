group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola.md:17   class: omission
  leaf-text: «never in `cargo build --release --bin viola`.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:14 «The `viola` root bin's test-only Windows dev-dependency: windows-sys (the workspace pin `=0.61.2`) under `[target.'cfg(windows)'.dev-dependencies]`, with both features `Wdk_Storage_FileSystem` and»
  replace-with: «never in `cargo build --release --bin viola`. Its one test-only Windows dev-dependency is windows-sys (the workspace pin `=0.61.2`) under `[target.'cfg(windows)'.dev-dependencies]` with both features `Wdk_Storage_FileSystem` and `Win32_System_IO` (`NtQueryInformationFile` and its `IO_STATUS_BLOCK` argument), for the pipe-type read-back of `tests/security_negatives_channel.rs`; both features already reach the Windows graph by unification, so `Cargo.lock` does not move, and under `resolver = "3"` no release build carries them.»
