# tests · cascade read · group 3 of 5

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) Each CLI verb that gains the check (`send`, `wait`, `last`, `answer`, `pause`, `release`) must have its mismatch negative (exit 21, its `--json` document) driven directly by assert_cmd with predicates; E1's unwrapped-row `send` must keep exiting 21 through the new check.
  > | cli (verbs + `--json`) | assert_cmd 2.2.2 + predicates 3.1.4;
  (.andromeda/test-plan.md:545)
  > A `send` to the unwrapped row's name exits 21: a row `name` is never a target. As landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21; the unwrapped-name hint not built, owed to "Exit-cause code catalogue").
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The hook client's `hook.event` and `hook.dialog` paths gain the check, so its verification-failure case (exit 0, empty stdout and stderr, no frame, no event line) must be driven by assert_cmd under an `Instant` deadline over fixture stdin, joining the fail-open matrix that runs in the integration layer.
  > | cli (`viola hook` / `hook statusline`) | assert_cmd 2.2.2 with `Instant` deadline | direct | Fixture stdin from `fixtures/claude/<ver>/`.
  (.andromeda/test-plan.md:598)
  > the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic and a missing `VIOLA_NAME`, each exit 0 with no body
  (.andromeda/test-plan.md:596)
  > run in the E2E layer, except root `hook_*` files such as `hook_fail_open.rs`, which the §3 layer filtersets run in the integration layer
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The chunk edits the sync and Tokio clients' connect and the endpoint's server-pid read, so every wrapper-channel test of the check, the impostor or squatter server included, must take the endpoint only from `snapshot.json`, never `try_overwrite` and never a reused endpoint name; the OS-branch server-pid read (Unix peer credentials, Windows pipe server process id, macOS euid + dir) is proven only on its own runner, the cross-user peer only at the unit boundary, and the control's negative stands beside the Windows SQOS case it builds on.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |
  (.andromeda/test-plan.md:468)
  > - **ipc-internal (wrapper channel):** the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
  (.andromeda/test-plan.md:889)
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
  (.andromeda/test-plan.md:873)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
  (.andromeda/test-plan.md:476)
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
  (.andromeda/test-plan.md:606)
  > - **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal, Windows client SQOS at the identification level,
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) The MCP `send` / `wait` / `last` / `answer` tools gain the check over the Tokio client, so their end-to-end unreachable outcome must be driven by an rmcp `client`-only test in `crates/viola-e2e` over a `viola mcp` child the test spawns and owns, never `TokioChildProcess`.
  > | ipc-internal (MCP stdio) | rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema | direct | Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.
  (.andromeda/test-plan.md:470)
  > - **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:26) `run`'s collision check (its snapshot read edited under the replay hold) must keep refusing a duplicate `run` and a squatted endpoint exit 1, and the squat's listener bound through the interprocess server API in a fresh home is the shape an impostor server for the verification negatives takes.
  > 3. Run a second `viola run builder --home H -- <fake>` with std::process.
  >   4. Squat: in a new home H2, create a listener on `viola_channel` endpoint name(`builder`, H2) through the interprocess server API, then run `viola run builder --home H2 -- <fake>`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:32) The snapshot's `pid` and `started_at` are the recorded values every client must match against the connected server, and the duplicate and squat exits must still read 1 after the collision check's edit.
  > The snapshot has `pinned_bin`, `pid`, `started_at`, `child_pid`, and `endpoint` from "Wrapper channel" on.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:34)
  > The duplicate `run` exits 1. The squatted endpoint exits 1.
  (.andromeda/test-plan.md:488)
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:46) `send`'s pre-frame check is edited, so the landed CLI and MCP `send` paths must stay green as the positive witnesses that a server matching its snapshot still receives the frame on all three CI OSes.
  > As landed (chunk 2026-10-04-confirmed-send-with-cl-1-records), `tests/cli_send.rs` `path2_send_confirms_with_cl1_events` covers cli, ipc-internal (the wrapper channel) and tui (the receipt) on all three CI OSes; the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli` (a harness session, `binary(mcp_tools)`), its result compared with the CLI's `--json` document less `v`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:74) `wait` and `last` gain the check, so the landed `wait` / `last` cases must pass against a matching server, `last_survives_a_wrapper_restart` must reach the restarted wrapper whose new `pid` / `started_at` its snapshot records, and the vanished-wrapper chaos case must still exit 21 through the new check.
  > `tests/cli_wait_last.rs` covers cli and ipc-internal (the wrapper channel) on all three CI OSes, over the gated fake script `fixtures/fake-scripts/path3.json` (a gated `PostToolUse`, then a gated `Stop`): `path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn` and `last_survives_a_wrapper_restart`; the vanished-wrapper half is `tests/chaos_wait_vanish.rs` `chaos_wait_parked_wrapper_killed_exits_21`. The MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, each result compared with the CLI's `--json` document less `v`;
  (.andromeda/test-plan.md:613)
  > an endpoint vanishing during `wait` (exit 21)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:83) Path 3's step 7 (a parked CLI `wait` whose wrapper is killed, then MCP `wait` on the dead instance) must still be driven through the new check, with the CLI `wait` of step 2 as its client.
  > 7. Start a new `wait` and kill the wrapper (the chaos helper); run rmcp `wait` on the dead instance.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:78)
  > 2. Run `viola send builder --json` as overseer to get cursor C. Spawn `viola wait builder --after C --json --timeout-ms 30000` with std::process.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:90) A verification mismatch, like a killed wrapper, must give CLI exit 21 with exactly this code-only document and MCP `is_error:true` with the same body and text, no pid, path or start time in either; as a new cause of exit 21 it must take its own exit-cause matrix case whose last `hint:` names it.
  > Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
  (.andromeda/test-plan.md:504)
  > a killed wrapper gives CLI exit 21 and MCP `instance-unreachable`.
  (.andromeda/test-plan.md:604)
  > - **Exit-cause matrix** (`cross_exit_causes.rs`): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table;

## Bindings
- **e2e wrapper-channel driver ↔ integration (the server-verification pattern) and unit (the peer-credential and liveness decisions)**: the impostor server the end-to-end and integration negatives bind must come through the server API with its endpoint read only from `snapshot.json` (.andromeda/test-plan-detail/e2e-test-strategy.md:13), (.andromeda/test-plan.md:889), while the cross-user peer and the pid / start-time decision are held at the unit boundary (.andromeda/test-plan.md:476).
- **e2e hook driver ↔ quality gates (perf)**: the hook client's added verification must keep its `Instant`-deadline runs (.andromeda/test-plan-detail/e2e-test-strategy.md:11) inside the hook perf rows gated on hyperfine `max` (.andromeda/test-plan.md:933).
- **Path 3 step 7 ↔ exit-cause matrix ↔ cross-surface parity**: the unreachable document a mismatch yields is the same CLI / MCP pair the killed wrapper yields (.andromeda/test-plan-detail/e2e-test-strategy.md:90), and its cause takes its own row with its own hint (.andromeda/test-plan.md:604).
- **Path 1 snapshot ↔ test data**: the `pid` / `started_at` the check matches come from the wrapper's own `snapshot.json` (.andromeda/test-plan-detail/e2e-test-strategy.md:32), which a test never hand-writes (.andromeda/test-plan.md:905).
- **e2e OS-branch verification ↔ CI**: the Unix, Windows and macOS legs of the server-pid read are each proven on their own runner (.andromeda/test-plan.md:873).
