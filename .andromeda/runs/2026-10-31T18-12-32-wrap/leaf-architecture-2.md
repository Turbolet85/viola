group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
- leaf: .claude/docs/services/viola-core.md:6   class: omission
  leaf-text: «`RefusalReason` and its closed detail sets (`HumanTyping { ManualPause }` among them),»
  source: .andromeda/architecture-detail/conventions.md:33 «`viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`) is exit 21's closed cause set.»
  replace-with: «`RefusalReason` and its closed detail sets (`HumanTyping { ManualPause }` among them), `UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`: exit 21's closed cause set, with no serde path, the two clients writing every word, and no `RefusalReason`),»
- leaf: .claude/docs/services/viola-mcp.md:6   class: omission
  leaf-text: «the board's refusal returned as `instance-unreachable`)»
  source: .andromeda/architecture-detail/standard-contracts.md:99 «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one;»
  replace-with: «the board's refusal, its only one, returned as `instance-unreachable` with `detail` `strict-modes-failed`)»
- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «`{"error":"instance-unreachable"|"wrapper-fault","detail":…}` (`wrapper-fault` with»
  source: .andromeda/architecture-detail/conventions.md:118 «(`instance-unreachable` with `detail` its cause word, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `wrapper-fault` with»
  replace-with: «`{"error":"instance-unreachable"|"wrapper-fault","detail":…}` (`instance-unreachable` with `detail` its cause word, built by `result::unreachable(cause)` from a `viola_core::UnreachableCause`: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, `of_reply` reading `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board` the board's refusal as `StrictModes`, and `tools.rs` `frame` no `live_endpoint` through `viola_state::liveness::without_endpoint`; `wrapper-fault` with»
- leaf: .claude/docs/services/viola-mcp.md:22   class: stale
  leaf-text: «(a refusal returns the `instance-unreachable` result with `detail: null`,»
  source: .andromeda/architecture-detail/conventions.md:131 «a refusal returns the `instance-unreachable` `isError` result with `detail: "strict-modes-failed"`, text `refused: instance-unreachable`, and writes no frame and no line.»
  replace-with: «(a refusal returns the `instance-unreachable` result with `detail: "strict-modes-failed"`,»
- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «returns the `instance-unreachable` result with no frame written and no log line»
  source: .andromeda/architecture-detail/established-decisions.md:125 «and a failed `server_verified` as `ServerVerify`, each with no frame and no line, `isError` the record.»
  replace-with: «returns the `instance-unreachable` result with `detail: "server-verify-failed"`, no frame written and no log line»
- leaf: .claude/docs/services/viola-pty.md:14   class: omission
  leaf-text: «The root bin's `run` pump (`src/run/`) and the tests' outer-PTY driver»
  source: .andromeda/architecture.md:314 «`COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read (the board's width is `viola_pty::host_size()`).»
  replace-with: «The root bin's `run` pump (`src/run/`), `viola verify`'s five interactive PTY runs (`src/cmd/verify/typed/`), `viola list`'s board width (`viola_pty::host_size()`) and the tests' outer-PTY driver»
- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «shared by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` and `viola-mcp`'s frame tools;»
  source: .andromeda/architecture-detail/standard-contracts.md:140 «Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`: a directory reads `InstanceDead`»
  replace-with: «shared by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` and `viola-mcp`'s frame tools, and giving none for a stale wrapper too; `liveness::without_endpoint(instance_dir)` → `UnreachableCause`, where `live_endpoint` gives none: read through `symlink_metadata`, a directory is `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`, opening nothing and writing no line, behind the strict-modes check at both callers, `client::Unreachable::without_endpoint` and `viola-mcp`'s `frame` (PROVISIONAL);»
- leaf: .claude/docs/services/viola-state.md:14   class: stale
  leaf-text: «The wrapper, `hook`, CLI verbs (`list` reads disk directly), `viola-agent-claude`, `viola-mcp`.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «The ledger chunk added no `viola-state` dependency: the stamps file I/O stays in `viola-state` and the root bin, so the crate stays pure.»
  replace-with: «The wrapper, `hook`, CLI verbs (`list` reads disk directly), `viola-mcp`; `viola-agent-claude` takes no `viola-state` dependency, the stamps file I/O staying here and in the root bin.»
