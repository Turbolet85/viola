group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «21 instance unreachable; `viola hook` always 0.»
  source: .andromeda/architecture.md:147 «`21` instance unreachable (no live endpoint for the `ViolaName`); also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board»
  replace-with: «21 instance unreachable (also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board, no log opened); `viola hook` always 0.»

- leaf: .claude/docs/conventions.md:40   class: stale
  leaf-text: «`viola pause|release <target> [--budget]`»
  source: .andromeda/architecture.md:176 «`viola pause <target>`, `viola release <target> [--budget]`, `viola link <driver> <driven>`, `viola unlink <driver> <driven>`»
  replace-with: «`viola pause <target>`, `viola release <target> [--budget]`»

- leaf: .claude/docs/conventions.md:40   class: omission
  leaf-text: «`viola link|unlink <driver> <driven>`.»
  source: .andromeda/architecture.md:176 «`viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments) send no channel frame.»
  replace-with: «`viola link|unlink <driver> <driven>`; `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame.»

- leaf: .claude/docs/conventions.md:45   class: omission
  leaf-text: «GUI port (47319), `diagnostics_level` (obs D-15).»
  source: .andromeda/architecture.md:465 «the GUI port (default 47319), `diagnostics_level` (`info` by default, or `debug`) and `claude_env_keep` (the Unix R8 pass-list of `CLAUDE*` names, default empty)»
  replace-with: «GUI port (47319), `diagnostics_level` (`info` by default, or `debug`; obs D-15), `claude_env_keep` (the Unix R8 pass-list of `CLAUDE*` names, default empty).»

- leaf: .claude/docs/services/viola-core.md:11   class: stale
  leaf-text: «Third-party: nutype 0.8.0, serde, serde_json, chrono, thiserror (via `[workspace.dependencies]`).»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:2 «Third-party: nutype (`ViolaName`, `Percent`) and serde (`derive`, for the kebab-case `RefusalReason` / `NotDelivered` / `BudgetWindowKind`), with the dev-dependency serde_json.»
  replace-with: «Third-party: nutype 0.8.0 (`ViolaName`, `Percent`) and serde (`derive`), with the dev-dependency serde_json (via `[workspace.dependencies]`).»

- leaf: .claude/docs/services/viola-channel.md:6   class: stale
  leaf-text: «It provides a sync client and server, plus a Tokio client behind the `tokio` feature that only `viola-mcp` enables.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:4 «As landed it is sync and tokio-free, with a sync client and server; the Tokio client arrives behind the `tokio` feature with `viola-mcp`.»
  replace-with: «It provides a sync client and server; as landed it is sync and tokio-free, and the Tokio client arrives behind the `tokio` feature with `viola-mcp`, the one crate that enables it.»

- leaf: .claude/docs/services/viola-channel.md:11   class: stale
  leaf-text: «libc (Unix: the uid for the socket dir); sysinfo 0.39.6 (start-time check, with the client verification).»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:4 «`viola-channel` → `viola-core`, interprocess, serde, serde_json, thiserror, tracing, veil, windows-sys (Windows only: the SQOS client open adopted by `Stream::try_from`»
  replace-with: «libc (Unix: the uid for the socket dir).»

- leaf: .claude/docs/services/viola-channel.md:26   class: stale
  leaf-text: «The wrapper serves `hook.event` (re-validated, appended with `source:"hook"`); every other method is still `-32601`.»
  source: .andromeda/architecture.md:288 «Neither is built as of chunk 2026-10-10-session-links, which landed the implicit link alone: the wrapper answers both `-32601`»
  replace-with: «The wrapper serves `hook.event` (re-validated, appended with `source:"hook"`), `hook.dialog`, `send`, `wait`, `last`, `answer`, `pause` and `release`; it answers `link` and `unlink` `-32601` (neither is built).»

- leaf: .claude/docs/services/viola-agent-claude.md:6   class: stale
  leaf-text: «parses the statusline `rate_limits` and `claude agents --json`»
  source: .andromeda/architecture.md:53 «not a channel request (as landed it reads the disk only: the `claude agents --json` read is owed to "Unwrapped sessions on the board")»
  replace-with: «parses the statusline `rate_limits` (the `claude agents --json` read is owed to "Unwrapped sessions on the board")»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: omission
  leaf-text: «As landed: `viola-core`, serde, serde_json, serde_path_to_error»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «As landed it depends on `viola-core`, serde, serde_json, chrono (the workspace pin `=0.4.45`, features `clock` and `std`»
  replace-with: «As landed: `viola-core`, serde, serde_json, chrono (the workspace pin `=0.4.45`, for the pure `statusline` module), serde_path_to_error»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: omission
  leaf-text: «thiserror (`IDENTITY_FLOOR`, `plan_strip`, `resolve_program`, `Refusal`, and the pure `ledger` module)»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «thiserror (`IDENTITY_FLOOR`, `plan_strip`, `resolve_program`, `Refusal`, `PLUGIN_DIR_FLAG`, `plugin_files` over the `include_str!`-embedded `plugin/` templates, and the pure `ledger` module»
  replace-with: «thiserror (`IDENTITY_FLOOR`, `plan_strip`, `resolve_program`, `Refusal`, `PLUGIN_DIR_FLAG`, `plugin_files` over the `include_str!`-embedded `plugin/` templates, the pure `ledger` module and the pure `dialog` module)»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: omission
  leaf-text: «dev-deps proptest, rstest.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «with dev-dependencies proptest, rstest and insta (`=1.48.0`, the dialog decision-body snapshots in `src/snapshots/`)»
  replace-with: «dev-deps proptest, rstest and insta (`=1.48.0`, the dialog decision-body snapshots in `src/snapshots/`).»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: omission
  leaf-text: «`list` / `mcp` / `ui` (`claude agents --json` rows)»
  source: .andromeda/architecture.md:334 «Its readers (`list`, `mcp` and `ui`, through `viola-agent-claude`) spawn it directly (none does yet: CLI `list` landed without it, chunk 2026-10-11-the-board-viola-list»
  replace-with: «`list` / `mcp` / `ui` (`claude agents --json` rows; none reads them yet: CLI `list` landed without the read, owed to "Unwrapped sessions on the board")»

- leaf: .claude/docs/services/viola-agent-claude.md:23   class: stale
  leaf-text: «statusline source + shell (planned; none landed)»
  source: .andromeda/architecture.md:79 «Statusline `rate_limits.*`, the settings-override mechanism, and the shell Claude Code runs a statusline command through on each OS. None of the three has a row yet»
  replace-with: «statusline `rate_limits.*`, the settings-override mechanism and the shell (none of the three has a row yet)»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «ten closed rows `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload` · `modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window`»
  source: .andromeda/architecture.md:91 «the ledger (`viola_agent_claude::ledger`, a closed set) holds seventeen rows: `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload`»
  replace-with: «seventeen closed rows `shim-resolution` · `spine-hooks` · `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload` · `modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window` · `question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency` · `long-paste-wrapper` · `tag-escaping` · `local-command-clear`»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: stale
  leaf-text: «`TypedRun`, `Probes { print, typed }`»
  source: .andromeda/architecture.md:91 «`check(row, &Probes)` reads `Probes { print, typed, trusted, dialogs }` (`trusted`: Run B's captures in claim order)»
  replace-with: «`TypedRun`, `Probes { print, typed, trusted, dialogs }` (`trusted`: Run B's captures in claim order)»

- leaf: .claude/docs/services/viola-mcp.md:31   class: tree
  leaf-text: «**Tool handlers:** `crates/viola-mcp/src/`»
  source: .andromeda/architecture.md:378 «Landed so far: `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-e2e`.»
  replace-with: «**Tool handlers:** `crates/viola-mcp/src/` (not in the tree yet: the crate lands with the route entry "MCP server for drivers")»

- leaf: .claude/docs/services/viola-mcp.md:32   class: tree
  leaf-text: «stdio E2E in `crates/viola-e2e/tests/mcp_*.rs` with the rmcp client»
  source: .andromeda/registries/contracts/architecture/build-system.md:12 «has no subject until `viola-mcp` exists; it lands with the "MCP server for drivers" chunk.»
  replace-with: «stdio E2E in `crates/viola-e2e/tests/mcp_*.rs` with the rmcp client (none in the tree yet: they land with "MCP server for drivers")»

- leaf: .claude/docs/services/viola-mcp.md:36   class: tree
  leaf-text: «**E2E:** `mcp_schema_violations.rs`, Paths 2–5 MCP legs, `cross_exit_causes.rs` (CLI/MCP parity)»
  source: .andromeda/architecture.md:378 «Landed so far: `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-e2e`.»
  replace-with: «**E2E:** `mcp_schema_violations.rs`, Paths 2–5 MCP legs, `cross_exit_causes.rs` (CLI/MCP parity), none in the tree yet: they land with "MCP server for drivers"»

- leaf: .claude/docs/services/viola-pty.md:39   class: tree
  leaf-text: «**Tests:** crate `tests/` spawning `viola-fake-agent` through real ConPTY/openpty (no-EOF mode, resize, `.cmd` refusal)»
  source: .andromeda/architecture.md:415 «viola-pty's real-PTY tests have their child stream its report to the first, one line per step»
  replace-with: «**Tests:** the crate's inline `mod tests` (the real-PTY rig) and the root `tests/tui_pty_seam.rs`, which spawns `viola-fake-agent` through real ConPTY/openpty (no-EOF mode, resize); the `.cmd` refusal in the root `tests/cli_program_resolution.rs`»
