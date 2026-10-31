group: architecture-3
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
- leaf: .claude/docs/services/viola.md:20   class: stale
  leaf-text: «A `live` or `stale` name → exit 1 (`already-live`)»
  source: .andromeda/architecture-detail/conventions.md:41 «a live holder or a fresh replayed beat `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`; a stale holder `detail:"stale-heartbeat"`»
  replace-with: «A live holder or a fresh replayed beat → exit 1 `already-live`, a `stale` holder → exit 1 `stale-heartbeat` (the `run.collision_check` span's `outcome` reads `already-live` for both)»
- leaf: .claude/docs/services/viola.md:21   class: stale
  leaf-text: «`already-live` → `no-session` → `session-live` → `cwd-missing`, each one»
  source: .andromeda/architecture-detail/conventions.md:42 «`detail:"already-live"` or `detail:"stale-heartbeat"` (`run`'s own collision check and its two pairs, unchanged)»
  replace-with: «`already-live` or `stale-heartbeat` → `no-session` → `session-live` → `cwd-missing`, each one»
- leaf: .claude/docs/services/viola.md:21   class: stale
  leaf-text: «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; `client::refused_document`) on stdout in place of its pair»
  source: .andromeda/architecture-detail/established-decisions.md:315 «every refusal of its start, a refused strict-modes check and a preflight refusal among them, prints `{"v":1,"error":"<code>","detail":null}`»
  replace-with: «every refusal of its start, a refused strict-modes check and a preflight refusal among them, prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, a `claude` not on `PATH` or an anyhow error of the start being `internal-error`; `client::refused_document`) on stdout in place of its pair where it has one»
- leaf: .claude/docs/services/viola.md:21   class: retired
  leaf-text: «a refusal `run`'s start makes after a passed preflight prints no document (owed to the working-route entry "Exit-cause code catalogue")»
  source: .andromeda/architecture-detail/standard-contracts.md:123 «a `pump_child` error prints no document, because the child has held the terminal.»
  replace-with: «`Launch` carries `json: bool` (`run` false, `revive` `args.json`), and with it true `refused_with` (`src/cmd/run.rs`) prints the document in place of the human pair before `spawn_child`; a `pump_child` error prints no document, because the child has held the terminal»
- leaf: .claude/docs/services/viola.md:34   class: omission
  leaf-text: «after its `process-exit{detail:"internal-error"}` line when an instance resolves»
  source: .andromeda/architecture-detail/conventions.md:44 «under `--json` before a `--` the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in that line's place, stderr then empty»
  replace-with: «after its `process-exit{detail:"internal-error"}` line when an instance resolves; under `--json` before a `--` (`src/main.rs` `json_asked`) it prints `cmd::internal_error_document`, `{"v":1,"error":"internal-error","detail":null}`, on stdout in that line's place, stderr then empty; `run`, `revive`'s start, `mcp` and `hook` stay silent at that site»
- leaf: .claude/docs/services/viola.md:35   class: omission
  leaf-text: «and its self exit reads `exit_code:1, detail:"interrupted"`»
  source: .andromeda/architecture-detail/occupied-resources.md:137 «the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`»
  replace-with: «and its self exit reads `exit_code:1, detail:"interrupted"`; a refused verify's self exit under a parsed `VIOLA_NAME` carries its `Ended::Refused` code in `detail` (`batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`), a refused home tree writing no line and a failing row carrying no code»
- leaf: .claude/docs/services/viola.md:42   class: stale
  leaf-text: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty»
  source: .andromeda/architecture-detail/conventions.md:106 «`--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
  replace-with: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
- leaf: .claude/docs/services/viola.md:42   class: stale
  leaf-text: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no line written»
  source: .andromeda/architecture-detail/standard-contracts.md:111 «prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout (exit 21), with stderr empty and no line in any log»
  replace-with: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, no line written»
- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an `UnreachableCause` through `cause()`) and `CallError`, `unreachable_document`»
  replace-with: «the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an `UnreachableCause` through `cause()`; `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`, a missing live endpoint reads `Unreachable::without_endpoint`, so a stale instance reads `instance-dead`; `hint_cause` keys the human hint) and `CallError`; `unreachable_document`, the one writer of the exit-21 document, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «or the `instance-unreachable` document under `--json`; nothing opened under the home»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «or the `instance-unreachable` document with `detail` `strict-modes-failed` under `--json`; nothing opened under the home, no line»
  replace-with: «or the `instance-unreachable` document with `detail` `strict-modes-failed` under `--json`; nothing opened under the home»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «the latter at WARN, a failed server check logged `server-verify-failed` with `during:"connect"` and no `channel-request` line»
  source: .andromeda/architecture-detail/occupied-resources.md:122 «exit 21 with `detail:"unwrapped-name"` + `during:"connect"`, or `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open»
  replace-with: «the latter at WARN with `detail:"instance-dead"` + `during:"connect"`, or `detail:"server-verify-failed"` + `during:"connect"` and no `channel-request` line for a failed server check; exit 21 also with `detail:"unwrapped-name"` + `during:"connect"`, or `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «or `instance-unreachable` for the board's refusal»
  source: .andromeda/architecture-detail/standard-contracts.md:99 «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one»
  replace-with: «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «exit 21 on a refusal (nothing created, no log opened)»
  source: .andromeda/architecture.md:230 «its refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`»
  replace-with: «exit 21 on a refusal (the home-state pair, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed` through `client::unreachable_document`; nothing created, no log opened)»
- leaf: .claude/docs/services/viola.md:49   class: omission
  leaf-text: «root `tests/{cli,hook,tui,channel,chaos,contract}_*.rs`»
  source: .andromeda/architecture-detail/occupied-resources.md:225 «`tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary»
  replace-with: «root `tests/{cli,hook,tui,channel,chaos,contract,cross}_*.rs` (`tests/cross_exit_causes.rs` the exit-cause table)»
