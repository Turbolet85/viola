# Apply — security-plan · part 2 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:12#1
- fanout-security-plan--cascade-g2.md 2 · applied · .andromeda/security-plan-detail/error-handling.md:13#1
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:22#1
- fanout-security-plan--cascade-g2.md 9 · applied · .andromeda/security-plan-detail/error-handling.md:24#1
- fanout-security-plan--cascade-g2.md 10 · applied · .andromeda/security-plan-detail/error-handling.md:22+#1
- fanout-security-plan--cascade-g2.md 11 · applied · .andromeda/security-plan-detail/error-handling.md:22+#2
- fanout-security-plan--cascade-g2.md 12 · applied · .andromeda/security-plan-detail/error-handling.md:24+#1
- fanout-security-plan--cascade-g2.md 13 · applied · .andromeda/security-plan-detail/error-handling.md:24+#2
- fanout-security-plan--cascade-g2.md 14 · applied · .andromeda/security-plan-detail/error-handling.md:22+#3
- fanout-security-plan--cascade-g2.md 15 · applied · .andromeda/security-plan-detail/error-handling.md:29+#1
- fanout-security-plan--cascade-g2.md 5 · applied · .andromeda/security-plan-detail/input-validation.md:37#1
- fanout-security-plan--cascade-g2.md 6 · applied · .andromeda/security-plan-detail/input-validation.md:37#2
- fanout-security-plan--cascade-g2.md 7 · applied · .andromeda/security-plan-detail/input-validation.md:38#1, .andromeda/security-plan-detail/input-validation.md:38#2, .andromeda/security-plan-detail/input-validation.md:38#3
- fanout-security-plan--cascade-g2.md 8 · applied · .andromeda/security-plan-detail/input-validation.md:75#1
- fanout-security-plan--cascade-g2.md 16 · applied · .andromeda/security-plan-detail/input-validation.md:38+#1
- fanout-security-plan--cascade-g2.md 17 · applied · .andromeda/security-plan-detail/input-validation.md:38+#2
- fanout-security-plan--cascade-g2.md 18 · applied · .andromeda/security-plan-detail/data-protection.md:73+#1

## Edits
- line: .andromeda/security-plan-detail/error-handling.md:12
  old: «returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; it adds no error word and writes no log line.»
  new: «returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail` `server-verify-failed`, text `refused: instance-unreachable`; its `error` is `instance-unreachable` as for every cause, and it writes no log line.»
- line: .andromeda/security-plan-detail/error-handling.md:13
  old: «returns the same unreachable result: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`, and no log line.»
  new: «returns the unreachable result: `isError` `instance-unreachable`, text `refused: instance-unreachable`, and no log line, `detail` reading `strict-modes-failed` for a refused check and, with no live endpoint, `viola_state::liveness::without_endpoint`'s cause: `instance-dead` for an instance directory, `unwrapped-name` for anything else.»
- line: .andromeda/security-plan-detail/error-handling.md:22
  old: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/security-plan-detail/error-handling.md:24
  old: «and a refusal there, or `update_stamps`'s `StampsError::StrictModes`, exits 1 with the home pair, no step line, no summary and no line.»
  new: «and a refusal there exits 1 with the home pair, no step line, no summary and no line; `update_stamps`'s `StampsError::StrictModes` exits 1 with the home pair, no step line and no summary, and its `process-exit`, under a parsed `VIOLA_NAME`, carries `strict-modes-failed`.»
- after: .andromeda/security-plan-detail/error-handling.md:22
  anchor: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance's strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.»
  add: «- Exit 21's `detail` is a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`: the CLI's `--json` document reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, and MCP's `isError` result the same less `v`, its text `refused: instance-unreachable`.»
- after: .andromeda/security-plan-detail/error-handling.md:22
  anchor: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance's strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.»
  add: «- `viola_core::UnreachableCause` has no serde path; every cause word is written by the two clients: `client::unreachable_document` is the one writer of the CLI's exit-21 document (`refuse_untrusted` and `src/cmd/plugin.rs` through it; `src/cmd/list.rs`'s `refused_document` builds the board's `strict-modes-failed` as a `Value` for `board_document`), and `unreachable` in `crates/viola-mcp/src/result.rs` is MCP's.»
- after: .andromeda/security-plan-detail/error-handling.md:22
  anchor: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance's strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.»
  add: «- The `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, their `<name>` the parsed `ViolaName`.»
- after: .andromeda/security-plan-detail/error-handling.md:24
  anchor: «- `viola run` and `viola revive`'s start run the home tree's strict-modes check (`check_home_tree`, over the instance and the pinned copy's key) before the wrapper log opens: a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`). `viola verify` runs the check on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and a refusal there, or `update_stamps`'s `StampsError::StrictModes`, exits 1 with the home pair, no step line, no summary and no line. On Unix a home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair); on Windows that path stats `NotFound` and the `cli` catch-site line stands.»
  add: «- Where the `cli` catch site prints its internal-error line (`prints_internal_error`) and `json_asked` (`src/main.rs`) reads `--json` among the arguments before a `--`, `{"v":1,"error":"internal-error","detail":null}` prints on stdout in place of `error: internal error`, stderr empty; `run`, `revive`'s start, `mcp` and `hook` stay silent there.»
- after: .andromeda/security-plan-detail/error-handling.md:24
  anchor: «- `viola run` and `viola revive`'s start run the home tree's strict-modes check (`check_home_tree`, over the instance and the pinned copy's key) before the wrapper log opens: a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`). `viola verify` runs the check on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and a refusal there, or `update_stamps`'s `StampsError::StrictModes`, exits 1 with the home pair, no step line, no summary and no line. On Unix a home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair); on Windows that path stats `NotFound` and the `cli` catch-site line stands.»
  add: «- `collision_check` prints and logs `stale-heartbeat` for a stale holder and `already-live` for a live holder and a fresh replayed beat; a frame verb against a stale instance reads `instance-dead` (`live_endpoint` gives none for a stale wrapper), stale heartbeat being exit 1's cause alone.»
- after: .andromeda/security-plan-detail/error-handling.md:29
  anchor: «- `tests/contract_error_bodies.rs`, carrying `andromeda:walks-tree`, enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions); `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.»
  add: «- `tests/cross_exit_causes.rs`, a root `cross_` binary with `required-features = ["fake-agent"]`, holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.»
- line: .andromeda/security-plan-detail/input-validation.md:37
  old: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/security-plan-detail/input-validation.md:37
  old: «a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.»
  new: «a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line.»
- line: .andromeda/security-plan-detail/input-validation.md:38
  old: «a refused strict-modes check or a preflight refusal then prints exactly»
  new: «a refused strict-modes check or any refusal of the start it shares with `run` then prints exactly»
- line: .andromeda/security-plan-detail/input-validation.md:38
  old: «`<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` (through `client::refused_document`)»
  new: «`<code>` one of the closed codes that **`revive --json` codes:** below holds (through `client::refused_document`)»
- line: .andromeda/security-plan-detail/input-validation.md:38
  old: «never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue"); `--list` under»
  new: «never a path or pid; `--list` under»
- line: .andromeda/security-plan-detail/input-validation.md:75
  old: «and the two fixed `unable:`/`hint:` stderr lines with no path or pid — because»
  new: «and the two fixed `unable:`/`hint:` stderr lines with no path or pid (under `viola revive --json`, through `refused_with`, the document `{"v":1,"error":"batch-script-child","detail":null}` on stdout in place of the pair, stderr empty, and a `claude` not found prints `internal-error`, each before `spawn_child`) — because»
- after: .andromeda/security-plan-detail/input-validation.md:38
  anchor: « **`pause` / `release` arguments:** `<name>` through `ViolaName::try_new`; their `from` comes from `VIOLA_NAME` only. **`revive` arguments:** `<name>` through `ViolaName::try_new` (the verbs' shared name parser); `--id <ID>` through a clap value parser that accepts only the session-id shape (anything else is a usage error, exit 2); `--list` conflicts with `--id`, `--fork` and the trailing arguments; the words after `--` go to the child as argv elements of a direct spawn; `--json` is accepted on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal then prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` (through `client::refused_document`), and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (the escaped `ts`, the closed cause word, the session-shaped id) — codes, closed words and session-shaped ids only, never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue"); `--list` under a parsed `VIOLA_NAME` logs `process-start` / `process-exit` in the caller's `cli-<VIOLA_NAME>.ndjson` only after its strict-modes check passed. Both arms run the strict-modes check on the instance's files before any snapshot or log value is used, and the start arm also runs `check_home_tree` (through `run`'s `checked_exe`) before its wrapper log: a refusal is exit 1 with the home or the instance pair, no log and no child (under `--json`, `strict-modes-failed`). **`list` arguments:** `[--json]` only, no name.»
  add: « **`revive --json` codes:** `viola revive --json`'s closed exit-1 codes are `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.»
- after: .andromeda/security-plan-detail/input-validation.md:38
  anchor: « **`pause` / `release` arguments:** `<name>` through `ViolaName::try_new`; their `from` comes from `VIOLA_NAME` only. **`revive` arguments:** `<name>` through `ViolaName::try_new` (the verbs' shared name parser); `--id <ID>` through a clap value parser that accepts only the session-id shape (anything else is a usage error, exit 2); `--list` conflicts with `--id`, `--fork` and the trailing arguments; the words after `--` go to the child as argv elements of a direct spawn; `--json` is accepted on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal then prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` (through `client::refused_document`), and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (the escaped `ts`, the closed cause word, the session-shaped id) — codes, closed words and session-shaped ids only, never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue"); `--list` under a parsed `VIOLA_NAME` logs `process-start` / `process-exit` in the caller's `cli-<VIOLA_NAME>.ndjson` only after its strict-modes check passed. Both arms run the strict-modes check on the instance's files before any snapshot or log value is used, and the start arm also runs `check_home_tree` (through `run`'s `checked_exe`) before its wrapper log: a refusal is exit 1 with the home or the instance pair, no log and no child (under `--json`, `strict-modes-failed`). **`list` arguments:** `[--json]` only, no name.»
  add: « **`revive --json` errors:** an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal, and viola writes nothing past it.»
- after: .andromeda/security-plan-detail/data-protection.md:73
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Logs (`verify`'s refusal codes):** `verify` writes `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` (`Ended::Refused`) on the `process-exit` it writes under a parsed `VIOLA_NAME`; a refused home tree is lineless, a failing row is no refusal and no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "sec2-01"
fixed = 'adds no error word'

[[pattern]]
id = "sec2-02"
fixed = 'returns the same unreachable result'

[[pattern]]
id = "sec2-03"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "sec2-04"
fixed = '`StampsError::StrictModes`, exits 1 with the home pair'

[[pattern]]
id = "sec2-05"
fixed = 'a refusal returns `instance-unreachable` with `detail: null`'

[[pattern]]
id = "sec2-06"
fixed = 'a preflight refusal then prints'

[[pattern]]
id = "sec2-07"
fixed = '`session-live` · `cwd-missing` (through `client::refused_document`)'

[[pattern]]
id = "sec2-08"
fixed = 'after a passed preflight prints no document'
