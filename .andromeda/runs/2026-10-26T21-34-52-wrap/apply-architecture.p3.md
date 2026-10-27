# Apply — architecture · part 3 of 5

## Verdicts
- fanout-architecture--cascade-g1.md 17 · applied · .andromeda/architecture-detail/conventions.md:36+#1
- fanout-architecture--cascade-g1.md 18 · applied · .andromeda/architecture-detail/conventions.md:37+#1
- fanout-architecture--cascade-g1.md 19 · applied · .andromeda/architecture-detail/conventions.md:42+#1
- fanout-architecture--cascade-g1.md 21 · applied · .andromeda/architecture-detail/conventions.md:40#1
- fanout-architecture--cascade-g1.md 22 · applied · .andromeda/architecture-detail/conventions.md:55+#1
- fanout-architecture--cascade-g1.md 23 · applied · .andromeda/architecture-detail/standard-contracts.md:24#1, .andromeda/architecture-detail/standard-contracts.md:24#2
- fanout-architecture--cascade-g1.md 24 · applied · .andromeda/architecture-detail/standard-contracts.md:25#1
- fanout-architecture--cascade-g1.md 25 · applied · .andromeda/architecture-detail/standard-contracts.md:37+#1
- fanout-architecture--cascade-g1.md 26 · applied · .andromeda/architecture-detail/standard-contracts.md:53+#1
- fanout-architecture--cascade-g1.md 27 · applied · .andromeda/architecture-detail/standard-contracts.md:75#1
- fanout-architecture--cascade-g1.md 28 · applied · .andromeda/architecture-detail/standard-contracts.md:95+#1
- fanout-architecture--cascade-g1.md 29 · applied · .andromeda/architecture-detail/standard-contracts.md:101#1, .andromeda/architecture-detail/standard-contracts.md:101#2
- fanout-architecture--cascade-g1.md 30 · applied · .andromeda/architecture-detail/standard-contracts.md:109#1

## Edits
- after: .andromeda/architecture-detail/conventions.md:36
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `1` also covers `viola run`'s start and `viola revive`'s start arm refusing the home tree, checked before the wrapper log opens (`checked_exe`, then `refuse_tree`, in `src/cmd/run.rs`): the home, the instance's `check_instance` set, `ledger/`, `bin/` and `plugin/`, then for the running binary's pin key `bin/<key>/`, the pinned `viola`, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists. A refusal prints the home-state `unable:` / `hint:` pair, or the instance-state pair when the refused path is the instance's, opens no log, writes no line and starts no child; under `viola revive --json` it is the `strict-modes-failed` document.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:37
  anchor: «»
  next: «<!-- detail: values · conv-exit-codes -->»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `1` also covers `viola verify` refusing the home tree (`check_home_tree` with no name and no key: the home, `ledger/`, `bin/` and `plugin/`, each that exists), checked before its `cli` log opens: the home-state `unable:` / `hint:` pair on stderr, no step line, no summary and no log line. A strict-modes refusal from `update_stamps` (`StampsError::StrictModes`) in its measuring step prints the same pair.»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/architecture-detail/conventions.md:40
  old: «The client's own refusals (exit 13), and for `link` / `unlink` the instance check of the driven instance (exit 21), come first, before any connect»
  new: «The client's own refusals (exit 13) and the strict-modes instance check (exit 21, `client::refuse_untrusted`: the target instance for `send`, `wait`, `last`, `answer`, `pause` and `release`, the driven instance for `link` / `unlink`) come first, before any connect»
- after: .andromeda/architecture-detail/conventions.md:42
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`, the verb's first step, after its text read for `send`): human mode prints `link`'s instance-state `unable:` / `hint:` pair, and `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; nothing under the home is opened and no line is written.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:55
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-mcp-results -->»
  add: «»
  add: «- A frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint`: a refusal returns the `instance-unreachable` `isError` result with `detail: null`, text `refused: instance-unreachable`, and writes no frame and no line.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:24
  old: «with two product readers, each after `viola_state::strict::check_instance` has passed:»
  new: «with six product readers, each after the instance's strict-modes check has passed:»
- line: .andromeda/architecture-detail/standard-contracts.md:24
  old: «an absent snapshot or a failed replay gives no item.»
  new: «an absent snapshot or a failed replay gives no item. The other four are `viola_state::liveness::live_endpoint` (Session liveness, below); `viola hook`'s `connect`, which reads a replayed or absent snapshot as unreachable, so the hook fails open; `hook statusline`'s `serve`, which finds no `statusline_command` in a replayed snapshot; and `run`'s collision check, which takes its caller's one read and judges a replayed snapshot by the heartbeat alone (`already-live` when the beat is at most `STALE_AFTER` old, else free; an absent snapshot or a failed read is free). `viola revive`'s preflight hands its one read to both its collision check and its recorded-directory reading.»
- line: .andromeda/architecture-detail/standard-contracts.md:25
  old: «`read_snapshot` keeps its results, and its three product callers (`src/cmd/hook.rs` once, the one read its event and dialog paths share; `run`'s collision check; `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md`; the CLI frame verbs and `viola mcp` read the snapshot inside `viola_state::liveness::live_endpoint`) still read every such snapshot as absent.»
  new: «`read_snapshot` keeps its results and has no product caller (the rule: `read_snapshot(` under `src/`, outside test modules, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).»
- after: .andromeda/architecture-detail/standard-contracts.md:37
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-ledger-stamps -->»
  add: «»
  add: «- `viola_state::stamps::update_stamps` runs `strict::check_stamps` on the home before it creates `ledger/` and again under its lock before its capped read; a refusal reads and writes nothing and returns `StampsError::StrictModes`. Its error is `StampsError` (`StrictModes` · `State`), each variant with a fixed message.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:53
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-channel-methods -->»
  add: «»
  add: «- Before that comparison each client runs the instance's strict-modes check, and a refused check writes no frame and no line: `client::refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the hook's `trusted` for `hook.event` and `hook.dialog`, and `strict::check_instance` in the MCP frame tools. Its one snapshot read is `replay::read_snapshot_or_replay`, and only a real snapshot goes on to the comparison.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:75
  old: «sent id-less by `hook` once its server check has passed (a notification, no reply; a failed check sends nothing and the hook fails open)»
  new: «sent id-less by `hook` once its instance check (`trusted`) and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot or a failed server check sends nothing and the hook fails open)»
- after: .andromeda/architecture-detail/standard-contracts.md:95
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-cli-json-output -->»
  add: «»
  add: «- `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:101
  old: «it canonicalises the instance directory, runs the instance check (`viola_state::strict::check_instance`), opens its log»
  new: «it canonicalises the instance directory and runs the instance check (`canonical` and `trusted` in `src/cmd/hook.rs`, shared with the event and dialog paths; `trusted` runs `viola_state::strict::check_instance`), opens its log»
- line: .andromeda/architecture-detail/standard-contracts.md:101
  old: «reads the snapshot, then runs the command»
  new: «reads the snapshot through `replay::read_snapshot_or_replay` (a replayed one holds no `statusline_command`, so no command runs), then runs the command»
- line: .andromeda/architecture-detail/standard-contracts.md:109
  old: «all from its one `read_snapshot` call, so a frame-writing client takes the liveness-only pre-check first and the server check after it»
  new: «all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot (a replayed or absent one, or a failed read, gives `None`), so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check»

## Patterns
[[pattern]]
id = "arch3-01"
fixed = 'and for `link` / `unlink` the instance check of the driven instance'

[[pattern]]
id = "arch3-02"
fixed = 'with two product readers'

[[pattern]]
id = "arch3-03"
fixed = 'still read every such snapshot as absent'

[[pattern]]
id = "arch3-04"
fixed = 'its one `read_snapshot` call'

[[pattern]]
id = "arch3-05"
fixed = 'takes the liveness-only pre-check first'
