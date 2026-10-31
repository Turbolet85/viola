group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/api.md:14   class: omission
    leaf-text: «on an interpolating `thiserror` message.»
    source: .andromeda/architecture-detail/cross-cutting-patterns.md:35 «Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.»
    replace-with: «on an interpolating `thiserror` message. Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of that binary, carrying no path, pid or upstream text; `tests/cross_exit_causes.rs` holds the exit-21 and exit-1 cause table (`every_code_is_a_code_word`, `no_two_codes_share_a_hint`, `no_hint_names_release`).»
- leaf: .claude/rules/api.md:20   class: omission
    leaf-text: «or `instance-unreachable` for the board's refusal.»
    source: .andromeda/architecture-detail/standard-contracts.md:99 «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one»
    replace-with: «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one.»
- leaf: .claude/rules/api.md:23   class: stale
    leaf-text: «MCP `instance-unreachable` with `detail: null` and no log line»
    source: .andromeda/security-plan-detail/error-handling.md:12 «`isError` `instance-unreachable` with `detail` `server-verify-failed`, text `refused: instance-unreachable`»
    replace-with: «MCP `instance-unreachable` with `detail` `server-verify-failed` and no log line»
- leaf: .claude/rules/api.md:28   class: stale
    leaf-text: «a refused strict-modes check or a preflight refusal is the exit-1»
    source: .andromeda/architecture.md:229 «`viola revive --json` refusing its strict-modes check, its preflight or the start it shares with `run`»
    replace-with: «a refused strict-modes check, a preflight refusal or any refusal of the start it shares with `run` is the exit-1»
- leaf: .claude/rules/api.md:28   class: stale
    leaf-text: «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` through `client::refused_document`»
    source: .andromeda/architecture.md:229 «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`»
    replace-with: «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error` through `client::refused_document`»
- leaf: .claude/rules/api.md:28   class: retired
    leaf-text: «a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue"»
    source: .andromeda/architecture-detail/standard-contracts.md:123 «An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error»
    replace-with: «`Launch.json` makes `refused_with` (`src/cmd/run.rs`) print the document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and a program not found `internal-error`, each before `spawn_child`; an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal»
- leaf: .claude/rules/api.md:28   class: stale
    leaf-text: «its refused strict-modes check the `instance-unreachable` document, exit 21, through `client::refused_document`»
    source: .andromeda/architecture.md:230 «its refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`»
    replace-with: «its refused strict-modes check the `instance-unreachable` document with `detail` `strict-modes-failed`, exit 21, through `client::unreachable_document`»
- leaf: .claude/rules/api.md:28   class: stale
    leaf-text: «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}`, exit 21»
    source: .andromeda/architecture-detail/standard-contracts.md:111 «prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout (exit 21)»
    replace-with: «prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, exit 21»
- leaf: .claude/rules/api.md:28   class: omission
    leaf-text: «`{"v":1,"error":"instance-unreachable"|"wrapper-fault",…}`»
    source: .andromeda/architecture.md:226 «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (exit 21), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`»
    replace-with: «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (`viola_core::UnreachableCause`, no serde path and no `RefusalReason`), written by `client::unreachable_document`, the board's `strict-modes-failed` built by `src/cmd/list.rs`'s `refused_document` / `{"v":1,"error":"wrapper-fault",…}`»
- leaf: .claude/rules/api.md:28   class: omission
    leaf-text: «read through `viola_channel::sanitised_fault`).»
    source: .andromeda/architecture.md:228 «`{"v":1,"error":"internal-error","detail":null}` (exit 1): a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`)»
    replace-with: «read through `viola_channel::sanitised_fault`; `{"v":1,"error":"internal-error","detail":null}`, exit 1, for a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`), printed on stdout by `cmd::internal_error_document` in place of `error: internal error`, stderr empty, `run`, `revive`'s start, `mcp` and `hook` staying silent there).»
- leaf: .claude/rules/api.md:29   class: stale
    leaf-text: «a `revive` strict-modes, home-tree or preflight refusal»
    source: .andromeda/architecture-detail/occupied-resources.md:30 «1 for a refused strict-modes check, a preflight refusal or a refusal of the start after a passed preflight»
    replace-with: «a `revive` strict-modes, home-tree, preflight or start refusal»
- leaf: .claude/rules/api.md:29   class: omission
    leaf-text: «each the home-state pair, included)»
    source: .andromeda/architecture-detail/conventions.md:82 «`viola verify`'s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME` (`src/cmd/verify.rs` `Ended::Refused`)»
    replace-with: «each the home-state pair, included; a refused home tree writes no line, a failing row carries no code, and every other refusal carries its `Ended::Refused` code, `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`, on the `process-exit` it writes under a parsed `VIOLA_NAME`; `verify` takes no `--json`)»
- leaf: .claude/rules/api.md:29   class: omission
    leaf-text: «`viola link` / `viola unlink`'s refusal reply or an `ok` of another shape»
    source: .andromeda/architecture-detail/conventions.md:44 «for `viola wait` / `viola last` exit 1 also covers a refusal reply, a reply of no known shape, and any channel error but a failed connect or a closed / reset connection»
    replace-with: «`viola wait` / `viola last`'s refusal reply, reply of no known shape or channel error other than a failed connect or a closed / reset connection, `viola link` / `viola unlink`'s refusal reply or an `ok` of another shape»
- leaf: .claude/rules/api.md:29   class: omission
    leaf-text: «21 instance unreachable (on `send`,»
    source: .andromeda/architecture-detail/standard-contracts.md:140 «a directory reads `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`»
    replace-with: «21 instance unreachable (the `--json` document's `detail` its closed cause: no live endpoint reads `viola_state::liveness::without_endpoint`, a `symlink_metadata` behind the strict-modes check that opens nothing and writes no line, `instance-dead` for an instance directory and `unwrapped-name` for anything else, a stale wrapper giving no endpoint and so `instance-dead`; `answer_of` reads a failed connect as `instance-dead` and a wrapper lost with the call open as `endpoint-lost`; `src/human/hints.rs` holds the unwrapped-name and endpoint-lost hints, each last on stderr in plain ASCII; on `send`,»
- leaf: .claude/rules/api.md:29   class: stale
    leaf-text: «the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty»
    source: .andromeda/architecture-detail/conventions.md:88 «the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` with stderr empty»
    replace-with: «`{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` with stderr empty»
- leaf: .claude/rules/api.md:29   class: stale
    leaf-text: «and the cause has no exit code, `--json` code or `RefusalReason` of its own)»
    source: .andromeda/architecture-detail/conventions.md:88 «and the cause has no exit code or `RefusalReason` of its own, its `--json` word the document's `detail`»
    replace-with: «and the cause has no exit code or `RefusalReason` of its own, its `--json` word the document's `detail`)»
- leaf: .claude/rules/api.md:29   class: stale
    leaf-text: «(`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)»
    source: .andromeda/architecture-detail/conventions.md:52 «(`--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, no log opened)»
    replace-with: «(`--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, no log opened)»
- leaf: .claude/rules/api.md:30   class: omission
    leaf-text: «the client euid check and macOS owed to "Unix endpoint and home hardening"»
    source: .andromeda/security-plan-detail/authentication-authorization.md:85 «the client euid check and the socket-directory refusal's code, are a CARRY owed to the route entry "Unix endpoint and home hardening"»
    replace-with: «the client euid check, macOS and the socket-directory refusal's code (none today: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) owed to "Unix endpoint and home hardening"»
- leaf: .claude/rules/api.md:30   class: stale
    leaf-text: «a refusal of either `{"error":"instance-unreachable","detail":null}` with no frame»
    source: .andromeda/security-plan-detail/error-handling.md:13 «`detail` reading `strict-modes-failed` for a refused check and, with no live endpoint, `viola_state::liveness::without_endpoint`'s cause: `instance-dead` for an instance directory»
    replace-with: «a refusal of either `{"error":"instance-unreachable","detail":"<cause>"}` with no frame, `<cause>` `strict-modes-failed` for a refused check and, with no live endpoint, `without_endpoint`'s `instance-dead` or `unwrapped-name`»
- leaf: .claude/rules/api.md:34   class: stale
    leaf-text: «a refusal being `isError:true` `{"error":"instance-unreachable","detail":null}`, text»
    source: .andromeda/architecture-detail/conventions.md:131 «a refusal returns the `instance-unreachable` `isError` result with `detail: "strict-modes-failed"`, text `refused: instance-unreachable`, and writes no frame and no line»
    replace-with: «a refusal being `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}`, text»
- leaf: .claude/rules/api.md:35   class: omission
    leaf-text: «refusal, unreachable, fault or invalid params: `isError:true` + codes only, text `refused: <code>`;»
    source: .andromeda/architecture-detail/established-decisions.md:125 «`of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board` reads the board's refusal as `StrictModes`»
    replace-with: «refusal, unreachable, fault or invalid params: `isError:true` + codes only, text `refused: <code>`; `instance-unreachable` carries its cause in `detail` through `result::unreachable(cause)`: `of_reply` reads `Connect` as `instance-dead` and `Closed | Io` as `endpoint-lost`, `of_board` the board's refusal as `strict-modes-failed`, and `tools.rs` `frame` a refused `check_instance` as `strict-modes-failed`, no `live_endpoint` by `without_endpoint` and a failed `server_verified` as `server-verify-failed`, `isError` the only record;»
