group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
- leaf: .claude/docs/conventions.md:8   class: omission
  leaf-text: «root `tests/<surface>_<topic>.rs` (`cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_`)»
  source: .andromeda/architecture-detail/occupied-resources.md:225 «`tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary»
  replace-with: «root `tests/<surface>_<topic>.rs` (`cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_`, `cross_`)»
- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «CLI exit codes: 0 · 1 (also `viola run`'s and `viola revive`'s start, and `viola verify`, refusing the home tree before any log opens:»
  source: .andromeda/architecture-detail/conventions.md:44 «under `--json` before a `--` the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in that line's place, stderr then empty»
  replace-with: «CLI exit codes: 0 · 1 (a `cli` verb's internal error: `error: internal error` on stderr, or under `--json` before a `--` the catch site's `{"v":1,"error":"internal-error","detail":null}` on stdout, stderr then empty, `run`, `revive`'s start, `mcp` and `hook` silent at that site; also `viola run`'s and `viola revive`'s start, and `viola verify`, refusing the home tree before any log opens:»
- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «no line written, no child started) · 2 ·»
  source: .andromeda/architecture-detail/conventions.md:82 «`viola verify`'s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME`»
  replace-with: «no line written, no child started; `viola verify`'s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`; a failing row is no refusal and carries no code, and `verify` takes no `--json`) · 2 ·»
- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «21 instance unreachable (also, on»
  source: .andromeda/architecture-detail/standard-contracts.md:112 «The exit-21 document's `detail` is one of five closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`»
  replace-with: «21 instance unreachable (under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, written by `client::unreachable_document` (the board builds its own), `<cause>` one of `viola_core::UnreachableCause`'s closed words `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, no `RefusalReason`; with no live endpoint `viola_state::liveness::without_endpoint` reads an instance directory as `instance-dead` and anything else as `unwrapped-name` (PROVISIONAL), so a stale instance reads `instance-dead`, and a wrapper lost with the call open reads `endpoint-lost`; also, on»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «no frame written, `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, after the client's own exit-13 refusals, with no code or `RefusalReason` of its own;»
  source: .andromeda/architecture-detail/conventions.md:88 «the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` with stderr empty»
  replace-with: «no frame written, `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` (claimed off macOS only, `v1-18`'s named carve-out), after the client's own exit-13 refusals, with no exit code or `RefusalReason` of its own, its `--json` word the document's `detail`;»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «the instance-state `unable:` / `hint:` pair, or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, no log opened, no line written)»
  source: .andromeda/architecture-detail/conventions.md:106 «`--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
  replace-with: «the instance-state `unable:` / `hint:` pair, or `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`, no log opened, no line written)»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «or a preflight refusal (`already-live` · `no-session` · `session-live` · `cwd-missing`, read in that order, each one `process-exit` line) or 2 for a usage error»
  source: .andromeda/architecture-detail/occupied-resources.md:30 «1 for a refused strict-modes check, a preflight refusal or a refusal of the start after a passed preflight and 2 for a clap usage error»
  replace-with: «or a preflight refusal (`already-live` or `stale-heartbeat` · `no-session` · `session-live` · `cwd-missing`, read in that order, each one `process-exit` line) or a refusal of the start after a passed preflight, or 2 for a usage error»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «where a refused strict-modes check or a preflight refusal is instead one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), stderr empty, exit 1,»
  source: .andromeda/architecture-detail/conventions.md:42 «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  replace-with: «where a refused strict-modes check and every refusal of its start is instead one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, by `client::refused_document`; a `claude` not on `PATH` or an anyhow error of the start is `internal-error`), stderr empty, exit 1,»
- leaf: .claude/docs/conventions.md:33   class: retired
  leaf-text: «(a refusal `run`'s start makes after a passed preflight prints no document; owed to "Exit-cause code catalogue")»
  source: .andromeda/architecture.md:229 «A `pump_child` error prints no document: the child has held the terminal.»
  replace-with: «(a `pump_child` error prints no document: the child has held the terminal)»
- leaf: .claude/docs/conventions.md:36   class: omission
  leaf-text: «and holds that a `thiserror` message interpolates nothing.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:35 «Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.»
  replace-with: «and holds that a `thiserror` message interpolates nothing; every exit-21 cause and exit-1 code passes `is_code_word` and is a row of it, carrying no path, pid or upstream text.»
- leaf: .claude/docs/services/viola-channel.md:25   class: omission
  leaf-text: «a mismatch or a check that cannot be made writes no frame (CLI exit 21 / MCP `instance-unreachable`, the hook fails open with no body).»
  source: .andromeda/architecture-detail/established-decisions.md:125 «`of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`»
  replace-with: «a mismatch or a check that cannot be made writes no frame (CLI exit 21 / MCP `instance-unreachable`, its `detail` `server-verify-failed`, the hook fails open with no body); a refused strict-modes check's `detail` is `strict-modes-failed`, a client with no live endpoint reads `viola_state::liveness::without_endpoint`, and the CLI's `answer_of` and MCP's `of_reply` read `Connect` as `instance-dead` and `Closed | Io` as `endpoint-lost`.»
