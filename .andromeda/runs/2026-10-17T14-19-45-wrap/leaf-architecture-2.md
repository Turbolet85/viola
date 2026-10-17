group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/stack.md:14   class: stale
  leaf-text: «(sync API; Tokio flavour only in `viola-mcp`)»
  source: .andromeda/architecture.md:18 «interprocess 2.4.4 `local_socket` (sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone,»
  replace-with: «(sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`)»

- leaf: .claude/docs/stack.md:26   class: omission
  leaf-text: «`send · wait · last · answer · list` |»
  source: .andromeda/architecture.md:30 «The release graph's rmcp features are exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process`»
  replace-with: «`send · wait · last · answer · list`. The release graph's rmcp features are exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process` (which kills the server 3 s after a close), its E2E owning the `viola mcp` child and awaiting its exit (chunk 2026-10-17-mcp-server-for-drivers) |»

- leaf: .claude/docs/stack.md:47   class: stale
  leaf-text: «in `viola-e2e`: rmcp `=3.4.1` (`client`, `transport-child-process`), tokio 1.53.1 `test-util`»
  source: .andromeda/architecture.md:30 «the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process` (which kills the server 3 s after a close), its E2E owning the `viola mcp` child and awaiting its exit»
  replace-with: «in `viola-e2e`: rmcp `=3.4.1` (`client` alone, never `transport-child-process`), tokio 1.53.1 (`rt` + `process`, architecture.md's [Module Boundaries])»

- leaf: .claude/docs/services/viola.md:34   class: stale
  leaf-text: «`role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`,»
  source: .andromeda/architecture.md:129 «`role_of` files every first word but `run`, `revive`, `mcp`, `hook` and a leading `-` flag under `cli` (a first word `mcp`, after any `--home <dir>` / `--home=<dir>`, is `Role::Mcp`,»
  replace-with: «`role_of` files every first word but `run`, `revive`, `mcp`, `hook` and a leading `-` flag under `cli` (a first word `mcp`, after any `--home <dir>` / `--home=<dir>`, is `Role::Mcp`, whose exit maps like `Other` and which never prints `error: internal error` — `prints_internal_error` stays `Cli`-only),»

- leaf: .claude/docs/services/viola.md:41   class: stale
  leaf-text: «shared by `list`, `revive` and verify's Run B;»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:36 «shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`);»
  replace-with: «shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`);»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «exits 0 / 21 / 1 / 2)»
  source: .andromeda/architecture.md:237 «MCP `list` (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read → document, the provider `viola mcp` hands `viola_mcp::serve`)»
  replace-with: «exits 0 / 21 / 1 / 2; `board_document` (scan → read → document) is the provider `viola mcp` hands `viola_mcp::serve` for the `list` tool, which returns the document's `ok`, or `instance-unreachable` for the board's refusal, while CLI `list` keeps its own order: scan, its log opened, read, document)»

- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «the `cli` obs start, the liveness-only endpoint, the reply shapes»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «`liveness::live_endpoint(instance_dir)`, moved unchanged from `src/cmd/client.rs` at chunk 2026-10-17-mcp-server-for-drivers, is the liveness-only endpoint read shared by CLI `send` / `wait` /»
  replace-with: «the `cli` obs start, the reply shapes (the liveness-only endpoint read is `viola_state::liveness::live_endpoint`, moved there unchanged from `src/cmd/client.rs` and shared with `viola-mcp`'s frame tools)»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «the message-mode escaper in `src/human/escape.rs`»
  source: .andromeda/architecture.md:301 «`mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`;»
  replace-with: «the message-mode escaper in `src/human/escape.rs`; **MCP verb:** `src/cmd/mcp.rs` (the hidden `viola mcp`, `#[command(hide = true)]`, no argument but the global `--home`: its home and `diagnostics/` (each that exists) through `viola_state::strict::check_path` before anything opens, exit 1 with nothing on stdout or stderr and no role file on a refusal; then `viola_mcp::serve`, the five tools over stdio until stdin closes)»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «`liveness::start_times`, one process-table read)»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «`liveness::live_endpoint(instance_dir)`, moved unchanged from `src/cmd/client.rs` at chunk 2026-10-17-mcp-server-for-drivers, is the liveness-only endpoint read shared by CLI `send` / `wait` /»
  replace-with: «`liveness::start_times`, one process-table read; `liveness::live_endpoint(instance_dir)`, moved unchanged from `src/cmd/client.rs`, the liveness-only endpoint read shared by CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools)»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «the CLI verbs' step is not ratified, the founder's word owed)»
  source: .andromeda/architecture.md:322 «`viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) resolves its home through the same `resolved_home` and reads `VIOLA_NAME` for its own name (the `from` it adds and its detail sink).»
  replace-with: «`viola mcp` resolves its home through the same `resolved_home`; the CLI verbs' step, and `mcp`'s, are not ratified, the founder's word owed)»
