# Apply — architecture · part 2 of 4

## Verdicts
- fanout-architecture--cascade-g1.md 8 · applied · .andromeda/architecture-detail/conventions.md:29+#1
- fanout-architecture--cascade-g1.md 9 · applied · .andromeda/architecture-detail/conventions.md:35#1
- fanout-architecture--cascade-g1.md 10 · applied · .andromeda/architecture-detail/conventions.md:36#1
- fanout-architecture--cascade-g1.md 11 · applied · .andromeda/architecture-detail/conventions.md:36#2, .andromeda/architecture-detail/conventions.md:36#3, .andromeda/architecture-detail/conventions.md:36#4
- fanout-architecture--cascade-g1.md 12 · applied · .andromeda/architecture-detail/conventions.md:38#1
- fanout-architecture--cascade-g1.md 13 · applied · .andromeda/architecture-detail/conventions.md:46#1
- fanout-architecture--cascade-g1.md 14 · applied · .andromeda/architecture-detail/conventions.md:48+#1
- fanout-architecture--cascade-g1.md 15 · applied · .andromeda/architecture-detail/conventions.md:48+#2
- fanout-architecture--cascade-g1.md 16 · applied · .andromeda/architecture-detail/conventions.md:64#1, .andromeda/architecture-detail/conventions.md:64#2
- fanout-architecture--cascade-g1.md 17 · applied · .andromeda/architecture-detail/conventions.md:66+#1
- fanout-architecture--cascade-g1.md 18 · applied · .andromeda/architecture-detail/conventions.md:66+#2
- fanout-architecture--cascade-g1.md 19 · applied · .andromeda/architecture-detail/conventions.md:70#1
- fanout-architecture--cascade-g1.md 20 · applied · .andromeda/architecture-detail/conventions.md:72+#1
- fanout-architecture--cascade-g1.md 21 · applied · .andromeda/architecture-detail/conventions.md:76#1
- fanout-architecture--cascade-g1.md 23 · applied · .andromeda/architecture-detail/conventions.md:89#1
- cascade-compare-architecture.md 2 · applied · .andromeda/architecture-detail/conventions.md:60+#1

## Edits
- after: .andromeda/architecture-detail/conventions.md:29
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-refusal-reason -->»
  add: «»
  add: «- **Unreachable cause:** `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`) is exit 21's closed cause set. It has no serde path: the two clients write every word. It is no `RefusalReason`.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/conventions.md:35
  old: «`detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`, or `unable: <name> is still running but not answering` with the stale hint)»
  new: «a live holder or a fresh replayed beat `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`; a stale holder `detail:"stale-heartbeat"`, `unable: <name> is still running but not answering` with the stale hint)»
- line: .andromeda/architecture-detail/conventions.md:36
  old: «`detail:"already-live"` (`run`'s own collision check»
  new: «`detail:"already-live"` or `detail:"stale-heartbeat"` (`run`'s own collision check»
- line: .andromeda/architecture-detail/conventions.md:36
  old: «each of these revive refusals, the strict-modes one included, is instead one stdout document»
  new: «each of these revive refusals, the strict-modes one included, and every refusal of the start it shares with `run` is instead one stdout document»
- line: .andromeda/architecture-detail/conventions.md:36
  old: «`session-live` · `cwd-missing`; `client::refused_document`»
  new: «`session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `client::refused_document`»
- line: .andromeda/architecture-detail/conventions.md:36
  old: «the same log lines, while a refusal `run`'s start makes after a passed preflight prints no document (owed to "Exit-cause code catalogue"; chunk 2026-10-17-cli-machine-contract)»
  new: «the same log lines (chunk 2026-10-17-cli-machine-contract), and a `pump_child` error, the child having held the terminal, prints none»
- line: .andromeda/architecture-detail/conventions.md:38
  old: «no hint — no dispatch arm prints it»
  new: «no hint — no dispatch arm prints it; under `--json` before a `--` the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in that line's place, stderr then empty»
- line: .andromeda/architecture-detail/conventions.md:46
  old: «(`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)»
  new: «(`--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, no log opened)»
- after: .andromeda/architecture-detail/conventions.md:48
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- The catch site reads `--json` through `src/main.rs` `json_asked`, from argv before a `--`, as `role_of` reads argv; where `prints_internal_error` is true it prints `src/cmd/mod.rs` `internal_error_document` in place of the human line. `run`, `revive`'s start, `mcp` and `hook` stay silent at that site.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:48
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- An absence: the socket-directory refusal has no code; `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site. OPEN — the socket-directory refusal's code, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I3`) · owner: the route entry "Unix endpoint and home hardening".»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:60
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- `viola verify`'s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME` (`src/cmd/verify.rs` `Ended::Refused`): `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`. A refused home tree writes no line (above), a failing row is no refusal and carries no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/conventions.md:64
  old: «the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`»
  new: «the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json`»
- line: .andromeda/architecture-detail/conventions.md:64
  old: «and the cause has no exit code, `--json` code or `RefusalReason` of its own»
  new: «and the cause has no exit code or `RefusalReason` of its own, its `--json` word the document's `detail`»
- after: .andromeda/architecture-detail/conventions.md:66
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `21`'s human hints: `src/human/hints.rs` holds the unwrapped-name hint and the endpoint-lost hint, each last on stderr and plain ASCII (their wording: design-system, surface cli-send). `src/cmd/client.rs` `hint_cause` keys the causes `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`; `not-running` is the key of the cause the log reads `instance-dead`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:66
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- An exception: `server-verify-failed` is claimed off macOS only; `v1-18`'s named carve-out leaves macOS's server check with the route entry "Unix endpoint and home hardening" (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I2` Q4).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/conventions.md:70
  old: «exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty»
  new: «exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
- after: .andromeda/architecture-detail/conventions.md:72
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `21`'s CLI mapping: `src/cmd/client.rs` `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`; the six no-endpoint arms (`send`, `wait`, `last`, `answer`, `pause`, `link`; `release` and `unlink` reach them through `wheel_call` and `link_call`) read `Unreachable::without_endpoint`.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/conventions.md:76
  old: «(`instance-unreachable` with `detail: null`;»
  new: «(`instance-unreachable` with `detail` its cause word, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`;»
- line: .andromeda/architecture-detail/conventions.md:89
  old: «`isError` result with `detail: null`,»
  new: «`isError` result with `detail: "strict-modes-failed"`,»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = 'is already live` / `hint: viola list`, or `unable: <name> is still running'

[[pattern]]
id = "arch2-02"
regex = 'detail:"already-live"` \(.run.s own collision check'

[[pattern]]
id = "arch2-03"
fixed = '`session-live` · `cwd-missing`; `client::refused_document`'

[[pattern]]
id = "arch2-04"
fixed = 'start makes after a passed preflight prints no document'

[[pattern]]
id = "arch2-05"
fixed = 'instance-unreachable","detail":null}`, no log opened'

[[pattern]]
id = "arch2-06"
fixed = 'the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "arch2-07"
fixed = 'has no exit code, `--json` code or `RefusalReason`'

[[pattern]]
id = "arch2-08"
fixed = 'exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty'

[[pattern]]
id = "arch2-09"
fixed = '`instance-unreachable` with `detail: null`'

[[pattern]]
id = "arch2-10"
fixed = '`isError` result with `detail: null`'
