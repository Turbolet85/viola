# Apply — architecture · part 3 of 4

## Verdicts
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:99#1
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:111#1
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:111+#1
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:117#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:117+#1
- fanout-architecture--cascade-g2.md 9 · applied · .andromeda/architecture-detail/standard-contracts.md:119+#1
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture-detail/standard-contracts.md:119+#2
- cascade-compare-architecture.md 12 · applied · .andromeda/architecture-detail/standard-contracts.md:135+#1
- fanout-architecture--cascade-g2.md 11 · applied · .andromeda/architecture-detail/occupied-resources.md:30#1, .andromeda/architecture-detail/occupied-resources.md:30#2
- fanout-architecture--cascade-g2.md 12 · applied · .andromeda/architecture-detail/occupied-resources.md:42#1
- fanout-architecture--cascade-g2.md 13 · applied · .andromeda/architecture-detail/occupied-resources.md:122#1
- fanout-architecture--cascade-g2.md 14 · applied · .andromeda/architecture-detail/occupied-resources.md:122#2
- fanout-architecture--cascade-g2.md 15 · applied · .andromeda/architecture-detail/occupied-resources.md:124+#1
- fanout-architecture--cascade-g2.md 16 · applied · .andromeda/architecture-detail/occupied-resources.md:148#1
- fanout-architecture--cascade-g2.md 17 · applied · .andromeda/architecture-detail/occupied-resources.md:149#1
- fanout-architecture--cascade-g2.md 18 · applied · .andromeda/architecture-detail/occupied-resources.md:168#1
- fanout-architecture--cascade-new-occupied-resources.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:217+#1
- cascade-compare-architecture.md 5 · applied · .andromeda/architecture-detail/occupied-resources.md:42#2, .andromeda/architecture-detail/occupied-resources.md:132+#1
- fanout-architecture--cascade-g2.md 19 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:34+#1
- fanout-architecture.md 5 · applied · .andromeda/registries/contracts/architecture/build-system.md:4#1

## Edits
- line: .andromeda/architecture-detail/standard-contracts.md:99
  old: «or `instance-unreachable` for the board's refusal;»
  new: «or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one;»
- line: .andromeda/architecture-detail/standard-contracts.md:111
  old: «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21)»
  new: «prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout (exit 21)»
- after: .andromeda/architecture-detail/standard-contracts.md:111
  anchor: «- `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21), with stderr empty and no line in any log.»
  add: «- The exit-21 document's `detail` is one of five closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`; its `v` is 1 and its `error` `instance-unreachable` on all five.»
- line: .andromeda/architecture-detail/standard-contracts.md:117
  old: «print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21)»
  new: «print the same `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout under `--json` (exit 21)»
- after: .andromeda/architecture-detail/standard-contracts.md:117
  anchor: «- `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.»
  add: «- `client::unreachable_document` is the one writer of the exit-21 CLI document; `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through it, and `src/cmd/list.rs`'s `refused_document` builds `strict-modes-failed` as a `Value` for `board_document`, the board's only refusal.»
- after: .andromeda/architecture-detail/standard-contracts.md:119
  anchor: «- A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP, and the human line reads `error: wrapper fault  <code>`.»
  add: «- `Launch` carries `json: bool` (`run` passes false, `revive` passes `args.json`). With it true, `refused_with` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and a program not found, which has no human pair, prints `internal-error`; each returns before `spawn_child`.»
- after: .andromeda/architecture-detail/standard-contracts.md:119
  anchor: «- A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP, and the human line reads `error: wrapper fault  <code>`.»
  add: «- `viola revive --json` prints one exit-1 document for every refusal of its start. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site logs its chain and its line; a `pump_child` error prints no document, because the child has held the terminal.»
- after: .andromeda/architecture-detail/standard-contracts.md:135
  anchor: «- The server identity read is `viola_state::liveness`'s. `live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot (a replayed or absent one, or a failed read, gives `None`), so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check. The closed decision enum `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares the snapshot's `pid` and `started_at` (whole seconds, read through its RFC 3339 form) with the observed peer pid and that process's OS start time; with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap. The joined `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` reads the observed start time through `process_start_time` (a zombie reads none) and returns `server_verdict`'s verdict.»
  add: «- Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`: a directory reads `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`. It opens nothing and writes no line, and the strict-modes check comes first at every caller: `client::Unreachable::without_endpoint` and `frame` in `viola-mcp`'s `tools.rs`. `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.»
- line: .andromeda/architecture-detail/occupied-resources.md:30
  old: «under `--json` a refused strict-modes check or a preflight refusal is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes,»
  new: «under `--json` a refused strict-modes check and every refusal of its start is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the ten codes,»
- line: .andromeda/architecture-detail/occupied-resources.md:30
  old: «1 for a refused strict-modes check or a preflight refusal and 2 for a clap usage error»
  new: «1 for a refused strict-modes check, a preflight refusal or a refusal of the start after a passed preflight and 2 for a clap usage error»
- line: .andromeda/architecture-detail/occupied-resources.md:42
  old: «`--json`: the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout»
  new: «`--json`: the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout»
- line: .andromeda/architecture-detail/occupied-resources.md:122
  old: «exit 21 with `detail:"instance-dead"` + `during`, or»
  new: «exit 21 with `detail:"instance-dead"` + `during:"connect"`, or»
- line: .andromeda/architecture-detail/occupied-resources.md:122
  old: «with no pid, start time, path or endpoint, exit 20»
  new: «with no pid, start time, path or endpoint, exit 21 with `detail:"unwrapped-name"` + `during:"connect"`, or `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open, exit 20»
- after: .andromeda/architecture-detail/occupied-resources.md:124
  anchor: « with no name, or on a refused scan, `list` opens no `cli-*.ndjson`; `mcp.ndjson` (chunk 2026-10-17-mcp-server-for-drivers), written by `viola mcp` once its home and `diagnostics/` passed `check_path` (a refusal opens no role file and writes no line): `process-start{subject:"self"}`, the Tokio client's `channel-request` / `channel-response` lines (none for a frame tool whose instance check or server check fails, and a refused instance check writes no line at all), one client-side `send-refused` per refused `send` text, the `list` tool's `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines (the `config-json` rejections included), one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line), and no self `process-exit`: none on stdin EOF or at that cut (`command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs`), and none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk; `hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected` (a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN, on the dialog path after `hook-invoked{corr:null}` and with `corr:null, decision_emitted:false`; a refused or absent instance writes no line on any path, its check (`trusted`) coming before `read_diagnostics_level` and obs init); `hook`'s drift reports go only to the instance's `diagnostics/detail-hook.ndjson`). One line per `write`; dir 0700, files 0600 on Unix; line format owned by obs and fixed by `schemas/diag-line.v1.json`. Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (the instance files, below).»
  add: « `schemas/diag-line.v1.json`'s `process-exit` `detail` enum holds `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture` among its words.»
- line: .andromeda/architecture-detail/occupied-resources.md:148
  old: «(`--json`: `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty;»
  new: «(`--json`: `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout, stderr empty;»
- line: .andromeda/architecture-detail/occupied-resources.md:149
  old: «(`--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout, stderr empty;»
  new: «(`--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout, stderr empty;»
- line: .andromeda/architecture-detail/occupied-resources.md:168
  old: «(`instance-unreachable` with `detail: null`, no frame)»
  new: «(`instance-unreachable` with `detail: "strict-modes-failed"`, no frame)»
- after: .andromeda/architecture-detail/occupied-resources.md:217
  anchor: «- Test-only: `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes. No after-every-test helper is built: the three surface witnesses, the enumeration and the census of writers replace it, reading this check.»
  add: «- Test-only: `tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary, with four checks over the table itself: `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.»
- line: .andromeda/architecture-detail/occupied-resources.md:42
  old: «`verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, or `update_stamps` refuses `ledger/stamps.json`.»
  new: «`verify` exits 1 with the home-state pair: before its `cli` log and any step line when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, and, when `update_stamps` refuses `ledger/stamps.json`, with `detail:"strict-modes-failed"` on its `process-exit` under a parsed `VIOLA_NAME`.»
- after: .andromeda/architecture-detail/occupied-resources.md:132
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `verify`'s refusal codes: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside `interrupted`. A refused home tree writes no line; a failing row is no refusal and carries no code; the fixture's `<file>` enters no code; `verify` takes no `--json`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:34
  anchor: «  - The E2E contract binary `tests/contract_error_bodies.rs` (run under `run --e2e`, carrying `andromeda:walks-tree`) holds it: its tables `CLI_DOCUMENTS`, `MCP_RESULTS` and `CHANNEL_FAULTS` enumerate the error bodies, and `error_bodies_each_name_a_writer_and_a_witness_that_stand` checks that each body's writer and witness stand; `NOT_ERROR_BODIES` names what is no error body, and `PLUMBING` the census lines that build or route a document a row names, so a row stays one body. A census, `product_error_writers_are_each_named`, covers the product's error writers, and `thiserror_messages_interpolate_nothing` holds that a `thiserror` message interpolates nothing.»
  add: «  - Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.»
- line: .andromeda/registries/contracts/architecture/build-system.md:4
  old: «which prints `error: internal error` for every `cli` verb;»
  new: «which prints `error: internal error` for every `cli` verb without `--json`, and under `--json` the `internal-error` document on stdout in its place (`cmd::internal_error_document`), stderr empty;»

## Patterns
[[pattern]]
id = "arch3-01"
fixed = 'or `instance-unreachable` for the board'

[[pattern]]
id = "arch3-02"
fixed = 'prints exactly `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "arch3-03"
fixed = 'print the same `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "arch3-04"
fixed = 'a refused strict-modes check or a preflight refusal is the exit-1'

[[pattern]]
id = "arch3-05"
fixed = 'one of the five codes'

[[pattern]]
id = "arch3-06"
fixed = '1 for a refused strict-modes check or a preflight refusal'

[[pattern]]
id = "arch3-07"
fixed = '`--json`: the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout'

[[pattern]]
id = "arch3-08"
fixed = 'or `update_stamps` refuses `ledger/stamps.json`'

[[pattern]]
id = "arch3-09"
fixed = 'detail:"instance-dead"` + `during`,'

[[pattern]]
id = "arch3-10"
fixed = '(`--json`: `{"v":1,"error":"instance-unreachable","detail":null}` on stdout'

[[pattern]]
id = "arch3-11"
fixed = '(`--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout'

[[pattern]]
id = "arch3-12"
fixed = '`instance-unreachable` with `detail: null`, no frame'

[[pattern]]
id = "arch3-13"
fixed = 'prints `error: internal error` for every `cli` verb;'
