# tests · cascade read · group 1 of 6

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The chunk's peer euid check on both sides must be asserted at the peer-credential check boundary, because a peer of another euid cannot be driven on a hosted runner, while `run`'s squatted-name exit 1 stays driven as same-user behaviour.
  > The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary.
  (.andromeda/test-plan-detail/test-scope-summary.md:11)
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
  (.andromeda/test-plan.md:46)
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
  (.andromeda/test-plan.md:88)
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan-detail/test-scope-summary.md:12) The chunk must deliver the macOS cases this line owes it, without reading the match with no comparison as a comparison, and every impostor negative it names must still reach its verdict past the new directory and euid checks on the same surfaces.
  > the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable` with `detail` `server-verify-failed`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed to the route entry "Unix endpoint and home hardening".
  (.andromeda/test-plan.md:184)
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
  (.andromeda/test-plan.md:193)
  > fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body. `statusline_command` must not run when the home fails strict-modes.
  (.andromeda/test-plan.md:254)
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan-detail/test-scope-summary.md:34) Each new refusal must take an exit inside the typed set, and a socket-directory refusal of the start `revive` shares with `run` must reach this closed `<code>` list in its own word, so the plan names how the list records it.
  > exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`;
  (.andromeda/test-plan.md:62)
  > **Entity:** CLI verbs and the `--json` / exit-code contract (the verbs, global `--home`, exits 0/1/2/10–14/20/21, `revive`'s `--json` documents on both arms)
  (.andromeda/test-plan.md:201)
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
  (.andromeda/test-plan.md:204)
  > no env / flag / config disables a control
- (.andromeda/test-plan-detail/test-scope-summary.md:37) The client-side directory and euid refusals on the frame verbs must agree with this reading: exit 21, its closed `--json` document and no line, and a test must hold each in that form.
  > **As landed (the instance check):** all eight CLI frame verbs run `viola_state::strict::check_instance` on the named instance (for `link` / `unlink` the driven one) through the shared `src/cmd/client.rs` `refuse_untrusted`, before any role file or log opens: a refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and writes no line (§5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
- (.andromeda/test-plan-detail/test-scope-summary.md:45) Every new CLI refusal must be asserted against this signal: its integer exit, one `--json` document, and in human mode the `unable` line with a last-line `hint:`, `revive` holding its own form and its counted codes.
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
  (.andromeda/test-plan-detail/test-scope-summary.md:45)
  > Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`.
  (.andromeda/test-plan-detail/test-scope-summary.md:43)
  > `revive` without `--json` has the human signal: exit 0 / 1 / 2, on a refusal one `unable:` line and a last-line `hint:` on stderr with empty stdout, for `--list` one `<ts>  <cause>  <id>` row per logged session on stdout; under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document (its ten codes)
  (.andromeda/test-plan.md:94)
  > its signal is the exit code plus one JSON document on stdout (the exit-2 unusable paths of `answer` and `send` excepted: a fixed stderr line and no document)
- (.andromeda/test-plan-detail/test-scope-summary.md:63) The wrapper's euid check against a connecting client stands on this surface, so both of its test clients must still be admitted as the same user, its cross-user leg held at the boundary, and any reading it gives a refused peer kept inside the closed fault shapes, the socket and directory modes asserted beside it.
  > **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.
  (.andromeda/test-plan-detail/test-scope-summary.md:60)
  > **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  (.andromeda/test-plan-detail/test-scope-summary.md:61)
  > Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`).
  (.andromeda/test-plan.md:104)
  > cross-user peer rejection is flagged in section 1.
  (.andromeda/test-plan.md:186)
  > Unix socket dir 0700 and file 0600 asserted
- (.andromeda/test-plan-detail/test-scope-summary.md:69) Each new cause of exit 21 must join this closed `<cause>` list with its MCP body the CLI document less `v`, must run ahead of any frame with no line, and must carry no absolute path of the refused directory and no anyhow chain.
  > `isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}` (`<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`)
  (.andromeda/test-plan-detail/test-scope-summary.md:72)
  > **As landed (the instance check):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).
  (.andromeda/test-plan.md:269)
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`.
- (.andromeda/test-plan-detail/test-scope-summary.md:101) The chunk must measure the hypothesis against this rule, since a force-killed wrapper leaves its Unix socket file, and whatever the measurement finds must leave the endpoint-gone reading and `cleanup`'s idempotence true or be named as the rule's change.
  > Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
  (.andromeda/test-plan-detail/test-scope-summary.md:100)
  > Stops every wrapper started by `boot`. Exit is confirmed on the process handle, not EOF.
  (.andromeda/test-plan-detail/test-scope-summary.md:102)
  > Must be idempotent: a second run exits 0 and reports nothing to clean.
  (.andromeda/test-plan.md:118)
  > `cleanup` stops every wrapper and deletes the home, idempotently
- (.andromeda/test-plan-detail/test-scope-summary.md:128) The directory verification must take its place before the exclusive endpoint bind in this sequence with the child spawn still last, the squatted endpoint and second `run` still exit 1 beside a takeover of a leftover file, and a refused home held to this reading on the paths the chunk's strict modes reach.
  > **Path:** `run` start sequence. The home-tree strict-modes check before the wrapper log opens (`check_home_tree`: the home under the read rule, the instance check's set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists), the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  (.andromeda/test-plan-detail/test-scope-summary.md:134)
  > A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).
  (.andromeda/test-plan.md:51)
  > Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`.
  (.andromeda/test-plan.md:135)
  > a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
  (.andromeda/test-plan.md:185)
  > squatted endpoint → `run` exit 1
  (.andromeda/test-plan.md:209)
  > group/world-writable home or instance dir → refused
  (.andromeda/test-plan.md:210)
  > a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID)
- (.andromeda/test-plan-detail/test-scope-summary.md:183) A wrapper killed under a parked `wait` must still read `endpoint-lost` with the client-side checks and the leftover socket file standing, its named case unchanged in verdict.
  > endpoint vanishes during `wait`: `instance-unreachable`, `detail` `endpoint-lost` (`tests/cross_exit_causes.rs` `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`)
  (.andromeda/test-plan.md:145)
  > Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
  (.andromeda/test-plan.md:241)
  > an endpoint vanishing during `wait` read `instance-unreachable`
- (.andromeda/test-plan.md:345) The `cleanup` the chunk measures must be read by this exit and document, so its `endpoint_gone:false` hypothesis is judged on exit 1 and `ok`, never on a message.
  > **Exit codes:** every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}` (key order held by serde_json `preserve_order`).
  (.andromeda/test-plan.md:955)
  > NEVER let a harness command print a human-only message without the JSON document and exit code.
- (.andromeda/test-plan.md:879) Any live Linux run `v1-53` draws on this chunk must stay local and outside CI, a live `claude` start itself being the scope's P4 fork.
  > NEVER run the real `claude` CLI or `viola verify` against the real CLI in CI. The suites reach the real CLI only through `agent-run run --local-live`, which refuses when `CI` is set; a route entry's own live measurement of a real session is local too and never a CI step.
- (.andromeda/test-plan.md:897) The squatter and the takeover cases the chunk re-reads must keep binding through the server API and read the endpoint from the snapshot, so no test proves a leftover file's takeover by overwriting through `try_overwrite`.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:921) The injected euids that stand in for a foreign peer and a foreign-owned directory must come through a dependency-injected decision, never a runtime patch of the check.
  > NEVER monkey-patch production code at runtime (use dependency injection)
- (.andromeda/test-plan.md:927) The widened per-user directory stands outside a test's `--home`, so a case must re-point it for its own child and never lean on serial execution to keep a parallel wrapper from meeting it.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.

## Bindings
- **The CLI's exit-21 document ↔ MCP's closed causes**: one new cause lands on both, the MCP body the CLI document less `v`, (.andromeda/test-plan-detail/test-scope-summary.md:37) and (.andromeda/test-plan-detail/test-scope-summary.md:69), its parity row at (.andromeda/test-plan.md:618).
- **The `run` start sequence ↔ `revive`'s closed codes**: a directory refusal of the shared start reads on both, the exit-1 sequence at (.andromeda/test-plan-detail/test-scope-summary.md:128) and the counted list at (.andromeda/test-plan-detail/test-scope-summary.md:34).
- **`cleanup`'s endpoint-gone rule ↔ the killed-wrapper readings**: the socket file a killed wrapper leaves is what (.andromeda/test-plan-detail/test-scope-summary.md:101) rules on and what (.andromeda/test-plan-detail/test-scope-summary.md:183) must read past.
- **The peer-credential boundary ↔ the unit layer**: the cross-user leg of (.andromeda/test-plan-detail/test-scope-summary.md:11) is held by the injected-euid decision at (.andromeda/test-plan.md:380) and the skip at (.andromeda/test-plan.md:484).
- **The owed macOS cases ↔ the exit-cause matrix and the matrix row**: (.andromeda/test-plan-detail/test-scope-summary.md:12) is the debt that (.andromeda/test-plan.md:610) names and (.andromeda/test-plan.md:775) runs on `macos-latest`.
