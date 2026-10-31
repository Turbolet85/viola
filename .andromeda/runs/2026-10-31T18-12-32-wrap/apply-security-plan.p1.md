# Apply — security-plan · part 1 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 4 · applied · .andromeda/security-plan.md:413#1
- fanout-security-plan.md 1 · folded into fanout-security-plan--cascade-g2.md 4
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:99#1
- cascade-compare-security-plan.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:59+#1
- cascade-compare-security-plan.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:122+#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#1
- fanout-security-plan--cascade-g1.md 3 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#2
- fanout-security-plan--cascade-g1.md 4 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#3
- fanout-security-plan--cascade-g1.md 5 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#4
- fanout-security-plan--cascade-g1.md 6 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#1
- fanout-security-plan--cascade-g1.md 7 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#2
- fanout-security-plan--cascade-g1.md 8 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#3
- fanout-security-plan--cascade-g1.md 9 · applied · .andromeda/security-plan-detail/authentication-authorization.md:73#1
- fanout-security-plan--cascade-g1.md 10 · applied · .andromeda/security-plan-detail/authentication-authorization.md:73#2
- fanout-security-plan--cascade-g1.md 11 · applied · .andromeda/security-plan-detail/authentication-authorization.md:70#1
- fanout-security-plan--cascade-g1.md 12 · applied · .andromeda/security-plan-detail/authentication-authorization.md:18#1
- fanout-security-plan--cascade-g1.md 13 · applied · .andromeda/security-plan-detail/authentication-authorization.md:68+#1
- fanout-security-plan--cascade-g1.md 14 · applied · .andromeda/security-plan-detail/authentication-authorization.md:74+#1
- fanout-security-plan--cascade-g1.md 15 · applied · .andromeda/security-plan-detail/authentication-authorization.md:79#1

## Edits
- line: .andromeda/security-plan.md:413
  old: «`instance-unreachable` with `detail: null`, `wrapper-fault`»
  new: «`instance-unreachable` with `detail` its closed cause, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, `wrapper-fault`»
- line: .andromeda/security-plan-detail/threat-model-summary.md:99
  old: «a refusal is `instance-unreachable` with `detail: null`, no frame and no line»
  new: «a refusal is `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line»
- after: .andromeda/security-plan-detail/threat-model-summary.md:59
  anchor: «    - That limit is the wrapper's read of a frame. A client's read of the answer is screened on its own: `reply_of` and `of_reply` take a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`) and a fault only through `viola_channel::sanitised_fault`, so nothing of the peer's but closed values reaches a CLI `--json` document or an MCP result; a code word outside the closed list stays visible by design.»
  add: «    - An `instance-unreachable` on CLI `--json` or an MCP result names its cause in `detail`, one of five closed words from `viola_core::UnreachableCause` (no serde path, every word written by the two clients, no path, pid or upstream text): `strict-modes-failed`; `instance-dead` (no endpoint with the instance directory standing, or a failed connect); `unwrapped-name` (no endpoint with anything else at that path: absent, a regular file or a symlink); `endpoint-lost` (the wrapper lost with the call open); `server-verify-failed` (off macOS). The no-endpoint reading is `viola_state::liveness::without_endpoint`, a `symlink_metadata` that opens nothing and writes no line, with the strict-modes check first at every caller (its status at § Authentication & Authorization, the no-endpoint reading). MCP writes no line for any cause, `isError` the record.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:122
  anchor: «    - Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance directory or `diagnostics/` another user can read, is refused.»
  add: «    - `src/main.rs`'s `json_asked` reads `--json` from argv ahead of clap, as `role_of` reads its role, taking the flag only among the arguments before a `--`. It chooses only the form of the catch site's output where `prints_internal_error` is true: `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, in place of the human line `error: internal error`. `run`, `revive`'s start, `mcp` and `hook` print nothing at the catch site.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:18
  old: «without writing any frame. **Secondary control:**»
  new: «without writing any frame. **Socket-directory failure at bind:** a failure at `bind_endpoint`'s `create_private_dir` has no refusal code: it is an anyhow error at the catch site, and its code is owed to the route entry "Unix endpoint and home hardening". **Secondary control:**»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «(`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`)»
  new: «(`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":"<cause>"}`, the cause `without_endpoint`'s reading, `instance-dead` or `unwrapped-name`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «MCP returns `instance-unreachable` with `detail: null` and writes no line»
  new: «MCP returns `instance-unreachable` with `detail` `server-verify-failed` and writes no line»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «(under `--json` the `instance-unreachable` document, `detail: null`)»
  new: «(under `--json` the `instance-unreachable` document, `detail` `strict-modes-failed`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «MCP returns `instance-unreachable` with `detail: null`. **Collision check:**»
  new: «MCP returns `instance-unreachable` with `detail` `strict-modes-failed`. **Collision check:**»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty»
  new: «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty»
  new: «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «a refusal returning `instance-unreachable` with `detail: null`, no frame and no line»
  new: «a refusal returning `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line»
- after: .andromeda/security-plan-detail/authentication-authorization.md:68
  anchor: «  - **Stack reference:** `viola-state` liveness; sysinfo 0.39.6»
  add: «- **Requirement:** IPC client-side server verification, the no-endpoint reading»
  add: «  - **Implementation:** With no `LiveEndpoint`, `viola_state::liveness::without_endpoint` tells a dead instance from an unwrapped name by `symlink_metadata` on the instance directory alone: a directory reads `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line, and the strict-modes check stays first at every caller. OPEN — this reading is PROVISIONAL · owner: how a name with no endpoint tells a dead instance from an unwrapped name.»
  add: «  - **Stack reference:** `viola-state` liveness (`without_endpoint`), called by the root bin's `src/cmd/client.rs` (`Unreachable::without_endpoint`) and `crates/viola-mcp/src/tools.rs` (`frame`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:70
  old: «(`live_endpoint`; none is exit 21 with the not-running hint)»
  new: «(`live_endpoint`; none is exit 21 through `Unreachable::without_endpoint`, `instance-dead` with the not-running hint or `unwrapped-name` with the unwrapped-name hint)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:73
  old: «under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty»
  new: «under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` on stdout with stderr empty»
- line: .andromeda/security-plan-detail/authentication-authorization.md:73
  old: «or under `--json` the same `instance-unreachable` document, opening nothing under the home»
  new: «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, opening nothing under the home»
- after: .andromeda/security-plan-detail/authentication-authorization.md:74
  anchor: «  - **Stack reference:** root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`, `refuse_untrusted`); `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs`; `src/human/hints.rs` (the `server-verify` cause)»
  add: «- **Requirement:** IPC client-side server verification, the MCP refusal»
  add: «  - **Implementation:** `viola mcp`'s `frame` maps a refused `check_instance` to `strict-modes-failed`, no `live_endpoint` to `without_endpoint`'s reading and a failed `server_verified` to `server-verify-failed`, each the `instance-unreachable` result with its cause in `detail`, with no frame and no line on any of them, `isError` the only record.»
  add: «  - **Stack reference:** `crates/viola-mcp/src/tools.rs` (`frame`); `crates/viola-mcp/src/result.rs` (`unreachable`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:79
  old: «the per-user socket directory loosened to 0755 giving exit 21 and the client euid check, are a CARRY»
  new: «the per-user socket directory loosened to 0755 giving exit 21, the client euid check and the socket-directory refusal's code, are a CARRY»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = '`instance-unreachable` with `detail: null`'

[[pattern]]
id = "sec1-02"
fixed = 'a refusal is `instance-unreachable` with `detail: null`'

[[pattern]]
id = "sec1-03"
fixed = 'else `{"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "sec1-04"
fixed = 'MCP returns `instance-unreachable` with `detail: null` and writes no line'

[[pattern]]
id = "sec1-05"
fixed = 'the `instance-unreachable` document, `detail: null`'

[[pattern]]
id = "sec1-06"
fixed = 'MCP returns `instance-unreachable` with `detail: null`.'

[[pattern]]
id = "sec1-07"
fixed = 'prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty'

[[pattern]]
id = "sec1-08"
fixed = 'prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty'

[[pattern]]
id = "sec1-09"
fixed = 'a refusal returning `instance-unreachable` with `detail: null`'

[[pattern]]
id = "sec1-10"
fixed = 'none is exit 21 with the not-running hint'

[[pattern]]
id = "sec1-11"
fixed = 'the document `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "sec1-12"
fixed = 'the same `instance-unreachable` document'
