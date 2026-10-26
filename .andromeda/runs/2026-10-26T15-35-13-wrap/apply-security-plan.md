# Apply — security-plan

## Verdicts
- fanout-security-plan.md 1 · applied · .andromeda/security-plan.md:114#1
- fanout-security-plan.md 2 · folded into fanout-security-plan--cascade.md 3
- fanout-security-plan--cascade-new-data-protection.md 1 · applied · .andromeda/security-plan-detail/data-protection.md:22+#1
- fanout-security-plan--cascade-new-data-protection.md 2 · applied · .andromeda/security-plan-detail/data-protection.md:22+#2
- fanout-security-plan--cascade-new-error-handling.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:10+#1
- fanout-security-plan--cascade-new-error-handling.md 2 · folded into fanout-security-plan--cascade.md 5
- fanout-security-plan--cascade.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:55#1, .andromeda/security-plan-detail/threat-model-summary.md:55#2
- fanout-security-plan--cascade.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:45+#1
- fanout-security-plan--cascade.md 3 · applied · .andromeda/security-plan-detail/authentication-authorization.md:12#1
- fanout-security-plan--cascade.md 4 · applied · .andromeda/security-plan-detail/authentication-authorization.md:64+#1
- fanout-security-plan--cascade.md 5 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#1
- fanout-security-plan--cascade.md 6 · applied · .andromeda/security-plan-detail/authentication-authorization.md:66#1, .andromeda/security-plan-detail/authentication-authorization.md:66#2
- fanout-security-plan--cascade.md 7 · applied · .andromeda/security-plan-detail/authentication-authorization.md:67#1
- fanout-security-plan--cascade.md 8 · applied · .andromeda/security-plan-detail/authentication-authorization.md:73+#1
- fanout-security-plan--cascade.md 9 · applied · .andromeda/security-plan-detail/input-validation.md:17+#1
- fanout-security-plan--cascade.md 10 · applied · .andromeda/security-plan-detail/input-validation.md:31+#1
- fanout-security-plan--cascade.md 11 · applied · .andromeda/security-plan-detail/input-validation.md:50#1
- fanout-security-plan--cascade.md 12 · applied · .andromeda/security-plan-detail/input-validation.md:51#1
- fanout-security-plan--cascade.md 13 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:8#1, .andromeda/security-plan-detail/security-anti-patterns.md:8+#1
- cascade-compare-security-plan.md 1 · applied · .andromeda/security-plan-detail/bootstrap-phases.md:15#1

## Edits
- line: .andromeda/security-plan.md:114
  old: «the snapshot read without the strict-modes check is a dated gap owed to "Home and code-bearing file integrity", and»
  new: «the snapshot read without the strict-modes check is a dated gap owed to "Home and code-bearing file integrity", except for CLI `link` / `unlink`, which read it only after the instance's strict-modes check and take no gap, and»
- after: .andromeda/security-plan-detail/data-protection.md:22
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Logs (`events.ndjson`, the link lines):** the wrapper's served `link` method appends one `link` line through `LinkSet::record`, as an implicit link does, and its served `unlink` method one `unlink` line through `LinkSet::remove`, each to the instance's `events.ndjson` with `data: {driver, driven}`: the two instance names only, no user content. A held pair's second `link` and an absent pair's `unlink` append nothing. The replies carry only `{since, already_linked}` (`link`) and `{}` (`unlink`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/data-protection.md:22
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Logs (the wrapper's process log) and the link verbs' output:** the wrapper's `link` / `unlink` methods open no span and write no process-log line of their own; its `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no name, so no driver name reaches the wrapper's process log. `viola link` / `viola unlink` print only their own parsed `ViolaName` arguments, never a name read back from the reply or the log.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/error-handling.md:10
  anchor: «- A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; it adds no error word and writes no log line.»
  add: «- The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.»
- line: .andromeda/security-plan-detail/threat-model-summary.md:55
  old: «(CLI `send`, `wait`, `last`, `answer`, `pause`, `release`;»
  new: «(CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`;»
- line: .andromeda/security-plan-detail/threat-model-summary.md:55
  old: «The snapshot is read without the strict-modes check (owed to "Home and code-bearing file integrity"), and macOS»
  new: «The snapshot is read without the strict-modes check (owed to "Home and code-bearing file integrity"), except by CLI `link` / `unlink`, which read it only after `check_instance` passed on the driven instance, and macOS»
- after: .andromeda/security-plan-detail/threat-model-summary.md:45
  anchor: «    - Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user. They can also read the whole conversation (`wait`, `last`).»
  add: «    - Whoever can connect can also add or remove a driver link (`link` / `unlink`), whose effect is a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; neither method reads the self-reported `from`.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:12
  old: «Any caller that passes the OS check (IPC) gets the full surface.»
  new: «Any caller that passes the OS check (IPC) gets the full surface: the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows below and its own params check (§ Input Validation, Channel frames).»
- after: .andromeda/security-plan-detail/authentication-authorization.md:64
  anchor: «  - **Stack reference:** `viola-state` liveness; sysinfo 0.39.6»
  add: «- **Requirement:** IPC client-side server verification, `viola link` / `viola unlink`»
  add: «  - **Implementation:** No dated gap: both verbs run `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read; then the liveness-only pre-check (`live_endpoint`; none is exit 21 with the not-running hint); then, on Linux and Windows, the server check through `cmd::client::connect` before their one `link` / `unlink` frame (macOS the recorded gap). Their reference values come from a snapshot whose instance passed the strict-modes check in the same process, so they borrow none of the eight dated gaps.»
  add: «  - **Stack reference:** root bin `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`); `viola-state` `strict::check_instance` and liveness (`live_endpoint`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «proven by `cfg(unix)` cases only), `mcp` (as landed»
  new: «proven by `cfg(unix)` cases only), `link` / `unlink` (`viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read; a refusal exits 21: the human form prints the fixed instance-state `unable:` / `hint:` pair on stderr through `human::refuse`, `--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty; it opens no log, writes no line and leaves the directory's mode as found; it is none of the dated interim gaps; proven by `cfg(unix)` cases only), `mcp` (as landed»
- line: .andromeda/security-plan-detail/authentication-authorization.md:66
  old: «(`send`, `wait`, `last`, `answer`, `pause`, `release`) meeting»
  new: «(`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting»
- line: .andromeda/security-plan-detail/authentication-authorization.md:66
  old: «The client's own exit-13 refusals come first.»
  new: «The client's own exit-13 refusals come first, and `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:67
  old: «`src/cmd/{send,wait,last,answer,pause,release}.rs`»
  new: «`src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs`»
- after: .andromeda/security-plan-detail/authentication-authorization.md:73
  anchor: «  - **Stack reference:** `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
  add: «- **Requirement:** `viola link` / `viola unlink`, the tests holding their controls»
  add: «  - **Implementation:** `channel_link_names_are_validated_by_the_wrapper` sends raw bad-name, foreign-`driven` and self-pair `link` / `unlink` frames past every CLI check and reads `-32602` with no line written; `link_verbs_refuse_a_widened_instance_before_any_log` (Unix only) reads a group-writable driven instance refused before any log, its mode as found; the server-identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server on Linux and Windows, compiled out on macOS with the file.»
  add: «  - **Stack reference:** `tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`»
- after: .andromeda/security-plan-detail/input-validation.md:17
  anchor: «  - **What to validate:** **Size cap:** wrap the stream reader in `Read::take(MAX_FRAME)` before `read_line`; an over-limit frame gets `-32600` and the connection is closed. **Depth:** serde_json's default recursion limit of 128 (never `unbounded_depth`). **Version:** the `v` check (`-32602`). **Closed types:** `behavior` is a closed enum (`allow`/`deny`, `approve`/`revise`), `dialog_id` is `u64`, and `link`/`unlink` `driver`/`driven` go through `ViolaName::try_new`. **`send` params (wrapper):** `text` is a string and `from` is absent, `null` or a string that passes `ViolaName::try_new`; anything else is `-32602` `"invalid params"` with `data: null`, before `validate_paste_text` runs. **`wait` / `last` params (wrapper):** `after` and `timeout_ms` are each absent or a non-negative integer that fits `u64` (`null`, a negative, a fraction or a string is `-32602` `"invalid params"` with `data: null`), `from` follows the `send` rule (one shared check), and unknown fields are ignored. The frame envelope types no method param, so a mistyped `from` reaches the method's `-32602`, never an envelope `-32600`. **`hook.dialog` params (wrapper):** `kind` is a closed dialog kind, `data` is an object, `hook_event` is closed, and `tool`, `input` and `continuation` are closed or absent; an invalid frame is not registered. **`answer` params (wrapper):** `dialog_id` is `u64`, `from` follows the `send` rule, `response` maps into the closed per-kind enums, then `validate_paste_text` runs on its free text. **`pause` / `release` params (wrapper):** `pause`'s `from` follows the `send` rule; `release`'s `budget` is absent or a bool (anything else, `null` included, is `-32602` `"invalid params"` with `data: null`), and its `from` is absent or `null` — a string `from` is `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}`, any other type `-32602` with `data: null`.»
  add: « **`link` / `unlink` params (wrapper):** `driver` and `driven` are strings that pass `ViolaName::try_new` (one shared check); a missing, non-string or invalid name, a `driven` other than the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` `"invalid params"` with `data: null`, no line written; neither method reads `from`, so a `from` of any shape changes nothing.»
- after: .andromeda/security-plan-detail/input-validation.md:31
  anchor: « **`pause` / `release` arguments:** `<name>` through `ViolaName::try_new`; their `from` comes from `VIOLA_NAME` only. **`revive` arguments:** `<name>` through `ViolaName::try_new` (the verbs' shared name parser); `--id <ID>` through a clap value parser that accepts only the session-id shape (anything else is a usage error, exit 2); `--list` conflicts with `--id`, `--fork` and the trailing arguments; the words after `--` go to the child as argv elements of a direct spawn; `--json` is accepted on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal then prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` (through `client::refused_document`), and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (the escaped `ts`, the closed cause word, the session-shaped id) — codes, closed words and session-shaped ids only, never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue"); `--list` under a parsed `VIOLA_NAME` logs `process-start` / `process-exit` in the caller's `cli-<VIOLA_NAME>.ndjson` only after its strict-modes check passed. Both arms run the strict-modes check on the instance's files before any snapshot or log value is used, so `viola revive` is not among this row's interim gaps. **`list` arguments:** `[--json]` only, no name.»
  add: « **`link` / `unlink` arguments:** `<driver>` and `<driven>` through `ViolaName::try_new` (the verbs' shared name parser), and `--json`; the verbs' instance is the driven one, `instances/<driven>` joined only from that parsed name and checked by `check_instance` before any snapshot read, so neither verb is among this row's interim gaps. A `driver` equal to `driven` is not refused client-side: the wrapper refuses it (`-32602`, the Channel frames row).»
- line: .andromeda/security-plan-detail/input-validation.md:50
  old: «the driver from a `from` that passed `ViolaName::try_new`, the driven its own name»
  new: «the driver from a `from` or from a served `link` / `unlink` method's `params.driver`, each having passed `ViolaName::try_new`, the driven its own name»
- line: .andromeda/security-plan-detail/input-validation.md:51
  old: «(its restore at every start and its record of an implicit link)»
  new: «(its restore at every start, its record of an implicit link or of a served `link` method, and its removal for a served `unlink` method)»
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «`pause`/`release`, and `hook.event` notifications»
  new: «`pause`/`release`, `link`/`unlink`, and `hook.event` notifications»
- after: .andromeda/security-plan-detail/security-anti-patterns.md:8
  anchor: «- NEVER write any channel frame (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, and `hook.event` notifications, which carry prompt and reply text) before verifying the server's pid (Windows: the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end) or euid + pid (Linux/FreeBSD `peer_creds()`; the client-side euid check owed to the route entry "Unix endpoint and home hardening") plus sysinfo start time against `snapshot.json` (on macOS, where `peer_creds()` gives no pid: euid + the verified 0700 socket directory only, per the recorded known gap; the euid check is never skipped), and NEVER take those reference values (`endpoint`, `pid`, `started_at`) from a `snapshot.json` that has not passed the strict-modes check in the same process. The dated exceptions hold only the strict-modes half — the reference values taken from a `snapshot.json` read without the strict-modes check — owed to the Epoch 6 entry "Home and code-bearing file integrity"; on Linux and Windows each frame below verifies the serving process first (macOS: the recorded gap): `viola hook`'s `hook.event` (Decisions Log `2026-09-28`, ratified by the founder), CLI `viola send`'s `send` frame, CLI `viola wait` / `viola last`'s `wait` / `last` frames and CLI `viola answer`'s `answer` frame and CLI `viola pause` / `viola release`'s `pause` / `release` frames, each after the liveness pre-check and the server check, and `viola hook pre-tool-use|permission-request`'s `hook.dialog` frame after its payload shape check (the founder's rulings, live, 2026-10-04, relayed by the overseer; `answer` / `hook.dialog` the sixth dated gap; `pause` / `release` the seventh, F-W1), and `viola mcp`'s `send` / `wait` / `last` / `answer` tool frames after the liveness pre-check and the server check (the eighth dated gap: the founder's own answer to P4 fork Q1, 2026-10-17, relayed by the overseer, `inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers); no other frame or process may borrow any of them.»
  add: « CLI `viola link` / `viola unlink`'s `link` / `unlink` frames meet this ban in full: they take `endpoint`, `pid` and `started_at` from a `snapshot.json` read after `check_instance` passed in the same process and verify the server through `cmd::client::connect` on Linux and Windows; only every client's euid check and macOS's pid and start-time check stay owed to "Unix endpoint and home hardening", and they borrow no dated exception.»
- line: .andromeda/security-plan-detail/bootstrap-phases.md:15
  old: «the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity".»
  new: «the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity" for every client but CLI `link` / `unlink`, which run `viola_state::strict::check_instance` on the home and the driven instance before any role file or log opens and before any snapshot value is read, then the liveness pre-check and the server check.»

## Patterns
[[pattern]]
id = "sec-01"
fixed = 'snapshot read without the strict-modes check is a dated gap'

[[pattern]]
id = "sec-02"
fixed = 'every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`;'

[[pattern]]
id = "sec-03"
fixed = 'The snapshot is read without the strict-modes check'

[[pattern]]
id = "sec-04"
fixed = 'A CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`)'

[[pattern]]
id = "sec-05"
fixed = 'the driver from a `from` that passed'

[[pattern]]
id = "sec-06"
fixed = 'its restore at every start and its record of an implicit link'

[[pattern]]
id = "sec-07"
fixed = '`pause`/`release`, and `hook.event` notifications'

[[pattern]]
id = "sec-08"
fixed = 'the snapshot read after the strict-modes check is owed to the route entry'
