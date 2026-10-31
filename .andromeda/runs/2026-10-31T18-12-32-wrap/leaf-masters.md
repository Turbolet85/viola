group: masters
read: CLAUDE.md · 129 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:20   class: omission
  leaf-text: «`BudgetWindowKind`, `obs_event!`»
  source: .andromeda/architecture-detail/conventions.md:33 «`viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`) is exit 21's closed cause set.»
  replace-with: «`BudgetWindowKind`, `UnreachableCause` (exit 21's closed cause set, each word through `as_str`, no serde path), `obs_event!`»
- leaf: CLAUDE.md:23   class: omission
  leaf-text: «`LiveEndpoint`: endpoint, pid and start time from one snapshot read)»
  source: .andromeda/architecture-detail/standard-contracts.md:140 «`viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`: a directory reads `InstanceDead`, anything else»
  replace-with: «`LiveEndpoint`: endpoint, pid and start time from one snapshot read, none for a stale wrapper; where it gives none `without_endpoint` → `UnreachableCause`: a directory `InstanceDead`, anything else `UnwrappedName`, opening nothing and writing no line, behind the strict-modes check at every caller)»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «subcommand dispatch, the `run` pump and its vt100 feed thread»
  source: .andromeda/architecture.md:228 «a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`), printed on stdout by `cmd::internal_error_document` in place of `error: internal error`»
  replace-with: «subcommand dispatch and the `main` catch site (a `cli` verb's `error: internal error`, or with `--json` before a `--` (`json_asked`) the `internal-error` document on stdout, stderr empty; `run`, `revive`'s start, `mcp` and `hook` silent there), the `run` pump and its vt100 feed thread»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «skipped under `--fork`, a failed read passing)»
  source: .andromeda/architecture-detail/standard-contracts.md:123 «`viola revive --json` prints one exit-1 document for every refusal of its start.»
  replace-with: «skipped under `--fork`, a failed read passing; under `--json` one exit-1 document for every refusal of the start it shares with `run`, an anyhow error of the start `internal-error`, a `pump_child` error none, the child having held the terminal)»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «and `release` run first on `instances/<name>`)»
  source: .andromeda/architecture-detail/standard-contracts.md:119 «`client::unreachable_document` is the one writer of the exit-21 CLI document; `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through it»
  replace-with: «and `release` run first on `instances/<name>`; `client::unreachable_document` the one writer of the exit-21 CLI document, the board building its own in `src/cmd/list.rs`; no live endpoint reads `without_endpoint`, a failed connect `instance-dead`, a wrapper lost with the call open `endpoint-lost`)»
- leaf: CLAUDE.md:33   class: stale
  leaf-text: «never block, refuse or delay a human keystroke»
  source: .andromeda/architecture.md:5 «Human keystrokes are never blocked, refused or delayed past the current atomic paste.»
  replace-with: «never block or refuse a human keystroke, or delay it past the current atomic paste»
- leaf: CLAUDE.md:36   class: omission
  leaf-text: «`tests/contract_error_bodies.rs` names every error body and product error writer, and a `thiserror` message interpolates nothing.»
  source: .andromeda/architecture-detail/standard-contracts.md:112 «The exit-21 document's `detail` is one of five closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`»
  replace-with: «exit 21's `detail` is one of five closed `viola_core::UnreachableCause` words (`strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, that last off macOS), written only by the two clients; `tests/contract_error_bodies.rs` names every error body and product error writer, every exit-21 cause and exit-1 code one of its rows, `tests/cross_exit_causes.rs` is the exit-cause table, and a `thiserror` message interpolates nothing.»
- leaf: CLAUDE.md:40   class: stale
  leaf-text: «stdout is reserved (`--json` results, the hook decision body, MCP frames, the child's screen)»
  source: .andromeda/obs-plan.md:131 «stdout is reserved on `hook`, `mcp` and `run`»
  replace-with: «stdout is reserved on `hook`, `mcp` and `run` (the hook decision body or the status line pass-through, MCP frames, the child's screen); the other verbs' stdout carries their `--json` or human result»
- leaf: CLAUDE.md:40   class: stale
  leaf-text: «no `print!`/`eprintln!`/`dbg!` in product crates»
  source: .andromeda/obs-plan.md:717 «NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates»
  replace-with: «no `print!`/`println!`/`eprint!`/`eprintln!`/`dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (clippy at `deny`; the output modules that own `--json`, human CLI stderr and the hook decision body excepted)»
- leaf: CLAUDE.md:82   class: omission
  leaf-text: «every send is confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed.»
  source: .andromeda/architecture.md:4 «The one exception is a ledger-listed local command whose post-condition is "none" or not yet measured on this CLI version: it returns `ok` with `confirmed: false` and detail `unconfirmable`.»
  replace-with: «every send is confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed, the one exception a ledger-listed local command whose post-condition is none or not yet measured on this CLI version: `ok` with `confirmed: false`, detail `unconfirmable`.»
