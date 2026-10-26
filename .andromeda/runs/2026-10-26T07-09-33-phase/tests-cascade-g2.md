# tests · cascade read · group 2 of 5

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:12) The edited viola-channel connect and endpoint server-pid read must have unit cases of a pure decision fed injected values, the peer-credential decision with injected euids extended to the server pid / start-time decision, the cross-user boundary asserted at that decision only.
  > The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.
  (.andromeda/test-plan.md:376)
  > the peer-credential decision with injected euids, and Windows handle non-inheritance
- (.andromeda/test-plan-detail/unit-test-strategy.md:13) The sync client's connect is a unit the chunk edits to read the server's identity, so on Windows the connected client handle must still report `HANDLE_FLAG_INHERIT` clear after the change.
  > `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear.
- (.andromeda/test-plan-detail/unit-test-strategy.md:20) `live_endpoint` and the pid + start-time match are edited, so the pure classifier must read a pid or start-time mismatch as `gone` at any beat age, fed directly with no clock, with the 4.9 / 5.0 / 5.1 s rows standing beside it.
  > The liveness classifier (live / stale / gone) is the pure `classify(beat_age, same_process)`, fed ages directly (no clock): a pid that is dead or carries another start time is `gone` at any age; for the same process 4.9 s and 5.0 s are live, 5.1 s and an absent beat `stale`; `same_process` compares the snapshot's `started_at` with the pid's OS start time.
  (.andromeda/test-plan.md:378)
  > the pure liveness classifier fed ages directly (no clock)
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) The chunk re-reads the classified snapshot read and holds the five readers' move to it behind the founder's replay ruling, so the classified read's four outcomes and the named five `read_snapshot` callers are the state the plan must keep unless that ruling lands.
  > Snapshot envelope: the classified read tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first; an unsupported `v` or a parse failure goes to replay through the read-or-replay function, which logs one `state-recovered` line for each and none for a present or an absent snapshot (unit cases `snapshot_cause`, `replay_recovers`).
  (.andromeda/test-plan-detail/unit-test-strategy.md:18)
  > The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:23) The chunk re-reads the strict-modes checks for its P4 question (the verification taking the strict-modes half or leaving it to "Home and code-bearing file integrity"), so the instance check's order and its Windows gap owed to that entry are what the plan reads against.
  > The instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order.
  (.andromeda/test-plan-detail/unit-test-strategy.md:23)
  > Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".
  (.andromeda/test-plan-detail/unit-test-strategy.md:21)
  > Unix strict-modes checks `mode & 0o077`.
  (.andromeda/test-plan.md:378)
  > Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.
- (.andromeda/test-plan-detail/unit-test-strategy.md:48) A verification mismatch is a new cause of exit 21, so the root bin's exit-code mapping unit cases must cover it, never mapping it to a panic or to 20.
  > The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21.
  (.andromeda/test-plan.md:386)
  > the exit-code mapping (an unknown refusal maps to exit 14, never a panic and never 20)
- (.andromeda/test-plan-detail/unit-test-strategy.md:60) `send`'s pre-frame check is edited, so its client refusals must still come before any frame and the new verification must not reorder them (a control character or an empty text stays exit 13, never 21).
  > the client's `deliver_empty_text_is_refused_before_any_frame` and `refusal_of_reads_a_refused_character_before_an_empty_text` in `src/cmd/send.rs`
- (.andromeda/test-plan-detail/unit-test-strategy.md:62) The `hook.dialog` client path in `src/cmd/hook.rs` is edited, so its bound must stay the longest hold plus the reply grace with the verification added before its frame.
  > the hook's bound is `DIALOG_HOLD_MAX` plus the reply grace (`dialog_deadline_is_the_longest_hold_plus_the_reply_grace`, `src/cmd/hook.rs`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The impostor server of the mismatch case must be a real endpoint bound in a temp home through the server API, never `try_overwrite`, its endpoint taken only from `snapshot.json`.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema. The squatter listener is created first via the interprocess server API, never `try_overwrite`.
  (.andromeda/test-plan.md:421)
  > a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`)
  (.andromeda/test-plan.md:889)
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:25) The hook client's verification failure must be driven as `viola hook <event>` with fixture stdin against a live test wrapper, its event line asserted absent and its hook trace read.
  > `viola hook <event>` fed fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper. The event line, the `diagnostics/hook-<name>.ndjson` trace and `budget.json` are asserted.
  (.andromeda/test-plan.md:425)
  > `viola hook <event>` fed fixture stdin against a live test wrapper, its event line, hook trace and `budget.json` asserted.
  (.andromeda/test-plan.md:440)
  > exit 0 and empty stderr
  (.andromeda/test-plan.md:441)
  > decision body or empty stdout
- (.andromeda/test-plan-detail/integration-test-strategy.md:47) The chunk must build this case per client family: a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21, the hook exit 0 with no body, and the MCP `send` an `isError` `instance-unreachable` with a null detail, on the OS runners the case names.
  > a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21, and the hook exits 0 with no body (Linux and Windows; macOS verifies euid + dir only, see the §6 exit-cause matrix). The MCP half is the same impostor case in `cross_exit_causes.rs`, the crate with the rmcp client: `call_tool("send")` gets `is_error:true` with `{"error":"instance-unreachable","detail":null}`.
  (.andromeda/test-plan.md:435)
  > `-32600` and a close on an oversize frame, server verification, and wrapper-side paste validation
  (.andromeda/test-plan.md:449)
  > adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client
  (.andromeda/test-plan.md:873)
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/integration-test-strategy.md:56) The outcomes the verification reports are the strict-modes refusal outcomes (hook 0 with no output, CLI 21), so whichever way the strict-modes fork goes they must stay those, `mcp` keeping exit 1 at its start.
  > hook 0 with no output (the statusline arm reads and opens nothing under the home after a refused check and writes no line, however often it is invoked), CLI 21 for every other verb
  (.andromeda/test-plan.md:437)
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb.
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) If the verification takes the strict-modes check, every frame-writing client becomes a check-guarded path, and its Windows cases must then start from `stamped_home`, never a seeded home.
  > Such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).
  (.andromeda/test-plan.md:427)
  > Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, and `tests/chaos_feed_panic.rs` alone boots outside the scans.
- (.andromeda/test-plan-detail/integration-test-strategy.md:63) The new exit-21 cause must have its `--json` document asserted in the root CLI suites.
  > root `tests/cli_*.rs` covers the `--json` document per exit code
  (.andromeda/test-plan.md:445)
  > root `tests/cli_*.rs` cover the `--json` document per exit code
- (.andromeda/test-plan-detail/integration-test-strategy.md:80) The driver-facing verbs whose pre-frame check the chunk edits must keep printing one document and their literal exit over a live wrapper's life, over a home with no instance and over a home another user could write.
  > Home resolution and the machine contract (chunk 2026-10-17-cli-machine-contract): `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance (`verbs_over_a_home_with_no_instance_print_their_documents`) and over a home another user could write (`verbs_over_a_home_another_user_could_write_print_their_documents`)

## Bindings
- **unit liveness classifier ↔ integration impostor case**: the pure `classify` decision reading a start-time mismatch as `gone` (.andromeda/test-plan-detail/unit-test-strategy.md:20) is the unit half of the cross-process mismatch case that gives CLI exit 21, hook exit 0 and MCP `instance-unreachable` (.andromeda/test-plan-detail/integration-test-strategy.md:47); the impostor in that case is bound through the server API, never `try_overwrite` (.andromeda/test-plan-detail/integration-test-strategy.md:18).
- **unit peer-credential decision ↔ integration OS split**: the injected-value decision (.andromeda/test-plan-detail/unit-test-strategy.md:12) holds the cross-user boundary no hosted runner can stage, while the integration case runs on Linux and Windows and macOS stays euid + dir only (.andromeda/test-plan-detail/integration-test-strategy.md:47), each OS branch on its own runner (.andromeda/test-plan.md:873).
- **unit exit-code mapping ↔ integration CLI documents**: the new exit-21 cause in the mapping (.andromeda/test-plan-detail/unit-test-strategy.md:48) is the same cause whose `--json` document the CLI suites assert (.andromeda/test-plan-detail/integration-test-strategy.md:63) and the machine contract reads per verb (.andromeda/test-plan-detail/integration-test-strategy.md:80).
- **unit instance check ↔ integration strict-modes outcomes ↔ home lifecycle**: if the verification takes the strict-modes half, `check_instance` (.andromeda/test-plan-detail/unit-test-strategy.md:23) gives the refusal outcomes hook 0 / CLI 21 (.andromeda/test-plan-detail/integration-test-strategy.md:56), and the Windows cases then start from `stamped_home` (.andromeda/test-plan-detail/integration-test-strategy.md:33); otherwise its Windows gap stays owed to "Home and code-bearing file integrity".
- **unit snapshot envelope ↔ the replay hold**: the five `read_snapshot` callers (.andromeda/test-plan-detail/unit-test-strategy.md:18) stay as they are, an unreadable or newer snapshot reading as absent, unless the founder's replay ruling lands.
- **unit hook dialog deadline ↔ integration hook exec**: the verification added to the edited `hook.dialog` path keeps it inside its bound (.andromeda/test-plan-detail/unit-test-strategy.md:62), and the hook's fail-open on a mismatch is driven against a live test wrapper (.andromeda/test-plan-detail/integration-test-strategy.md:25).
