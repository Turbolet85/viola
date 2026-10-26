# tests · cascade read · group 1 of 5

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The pid / start-time server verification is a testable same-user part of `viola-channel`, so the chunk's client-side check in the sync and Tokio clients and the endpoint's server-pid read must be asserted in tests, beside the squatted-name refusal.
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
  (.andromeda/test-plan-detail/test-scope-summary.md:11)
  > The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary.
  (.andromeda/test-plan.md:46)
  > same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
  (.andromeda/test-plan.md:88)
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan-detail/test-scope-summary.md:32) Every edited CLI verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) must give exit 21 on a mismatch or a check that cannot be made, inside the closed exit-code set of this contract.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21;
  (.andromeda/test-plan.md:62)
  > **Entity:** CLI verbs and the `--json` / exit-code contract (the verbs, global `--home`, exits 0/1/2/10–14/20/21, `revive`'s `--json` documents on both arms), plus the human TTY output (`viola list`, the readback mirror, C0/C1 escaping) — testable.
- (.andromeda/test-plan-detail/test-scope-summary.md:41) The CLI verbs' unreachable outcome must be asserted as a subprocess with a per-test `--home` and `--json`, its signal the exit code plus one `{"v":1,"error":…}` document on stdout.
  > **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  (.andromeda/test-plan-detail/test-scope-summary.md:42)
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
  (.andromeda/test-plan-detail/test-scope-summary.md:43)
  > **Boundary:** cross-surface (verbs talk to the `run` wrapper over the IPC channel, and read disk for `list`)
  (.andromeda/test-plan.md:94)
  > **Surface:** cli (`viola` verbs) — driven as a subprocess by `assert_cmd` with a per-test `--home` and `--json`; its signal is the exit code plus one JSON document on stdout
- (.andromeda/test-plan-detail/test-scope-summary.md:57) The wrapper channel is the surface whose client connect the chunk edits: its server-verification tests are driven by the language-native IPC client, the `viola-channel` sync client or a raw `interprocess` client, against a real endpoint.
  > **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  (.andromeda/test-plan-detail/test-scope-summary.md:60)
  > **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.
  (.andromeda/test-plan.md:104)
  > **Surface:** ipc-internal (the wrapper channel, JSON-RPC 2.0 over ndjson) — driven by a language-native IPC client with JSON schema assertion; its signal is the success, refusal or protocol-error payload, cross-checked against the `events.ndjson` cursor; cross-user peer rejection is flagged in section 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:64) The four edited MCP tools must return exactly `isError:true` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable` on a mismatch or an unmakeable check, writing no frame.
  > `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`.
  (.andromeda/test-plan-detail/test-scope-summary.md:65)
  > **Boundary:** cross-surface (MCP → wrapper channel → run wrapper → fake agent)
  (.andromeda/test-plan.md:106)
  > its signal is the tool result (`isError` with the `ok` payload, a refusal or a coded error)
- (.andromeda/test-plan-detail/test-scope-summary.md:122) The `pid` and `started_at` that `run`'s start records in `snapshot.json` are the values the new check matches against the connected server, so a test's expected pair must be the serving wrapper's own, and a squatted endpoint's `run` exit 1 stands beside the impostor case.
  > `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on)
  (.andromeda/test-plan-detail/test-scope-summary.md:124)
  > A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
  (.andromeda/test-plan.md:135)
  > a second `run` of the same name, a squatted endpoint or a tampered pinned exe exits 1.
  (.andromeda/test-plan.md:128)
  > The wrapper `pid`, `started_at` and `child_pid` live in `<home>/instances/<name>/snapshot.json`, with liveness in `<home>/instances/<name>/heartbeat` (touched every 1 s): when pid + start time no longer name a running process the instance is `gone` whatever the beat's age; otherwise a beat at most 5 s old is `live`, and an older or absent one `stale`.
- (.andromeda/test-plan-detail/test-scope-summary.md:172) An endpoint vanishing during `wait` must still read `instance-unreachable` through the new check, and the stale-heartbeat / live-pid distinction must stand beside the pid / start-time mismatch reading.
  > endpoint vanishes during `wait`: `instance-unreachable`
  (.andromeda/test-plan-detail/test-scope-summary.md:173)
  > stale-heartbeat / live-pid distinction
  (.andromeda/test-plan-detail/test-scope-summary.md:170)
  > corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`.
  (.andromeda/test-plan.md:145)
  > Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
  (.andromeda/test-plan.md:237)
  > **Trigger type:** chaos-test — required: a torn last ndjson line counted in `torn_lines` and healed by the next append, a corrupt or unsupported-`v` snapshot recovered by replay, a child exit without EOF detected on the handle, an endpoint vanishing during `wait` read `instance-unreachable`, and the stale-heartbeat / live-pid distinction.
  (.andromeda/test-plan.md:905)
  > A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version
- (.andromeda/test-plan-detail/test-strategy.md:10) The new cause of exit 21 must be covered in the root bin's inline exit-code `#[case]` table, and the channel's server-identity decision and the liveness match in their own crates' inline unit modules, on all three OSes.
  > **Tier-specific depth (Comprehensive, viola):** Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes
  (.andromeda/test-plan.md:285)
  > **Unit:** inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices
- (.andromeda/test-plan-detail/test-strategy.md:14) The server-verification negatives must run in the integration layer over a real `interprocess` endpoint and a real tempfile home, the Wrapper channel and CLI contracts each in a crate-level `tests/` suite.
  > **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:287)
  > **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan-detail/test-strategy.md:17) Path 3's killed-wrapper signal and the MCP mismatch must stay green at E2E against harness-booted wrappers over the fake agent, through the rmcp client.
  > **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:289)
  > **E2E:** required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:23) The verification added to the hook client must keep the hook deadlines green on hyperfine `max`, and the pid / start-time mismatch must read `gone` in the pure liveness classifier fed directly, with no clock.
  > hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10). The heartbeat staleness flip at >5 s is checked by `viola_state::liveness::classify`, a pure function fed the beat's age (no clock).
  (.andromeda/test-plan.md:293)
  > **Performance / Load:** required — hyperfine gates the hook deadlines on `max`; the heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
- (.andromeda/test-plan-detail/test-strategy.md:26) A vanished endpoint, a frozen heartbeat and a corrupt snapshot reaching the new check must be driven only by explicit process and file faults or existing mockall seams.
  > Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
  (.andromeda/test-plan.md:295)
  > **Chaos / Fault:** required — faults come from process and file operations and from mockall faults on the `viola-pty` seam.
- (.andromeda/test-plan-detail/test-strategy.md:58) The server pid / start-time mismatch negative is the IPC vector's, owed in the integration and E2E layers and in the root `security_negatives_channel.rs` beside the Windows admission cases, giving CLI exit 21, MCP `instance-unreachable` and a hook exit 0 with no body.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) |
  (.andromeda/test-plan.md:184)
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan-detail/test-strategy.md:60) A verification failure in `viola hook`'s `hook.event` and `hook.dialog` clients must join channel failure in the hook fail-open tests, each exit 0, empty stderr and no body.
  > | Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
  (.andromeda/test-plan.md:193)
  > **Required test type:** fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body.
  (.andromeda/test-plan.md:98)
  > **Signal:** exit code is always 0. stderr is always empty. stdout is either empty (no decision / async hooks) or exactly the PreToolUse / PermissionRequest decision body.
- (.andromeda/test-plan-detail/test-strategy.md:61) The MCP tools' new `instance-unreachable` outcome must be asserted through the rmcp client and carry codes only.
  > | MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
  (.andromeda/test-plan.md:196)
  > `isError` payloads carry codes only.
- (.andromeda/test-plan-detail/test-strategy.md:62) If the plan takes the strict-modes half with the verification (the P4 boundary question), a home or `VIOLA_DIR` failing strict-modes must give the CLI's exit and a hook exit 0, in unit and integration cases.
  > | CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
  (.andromeda/test-plan.md:201)
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan-detail/test-strategy.md:63) The snapshot reads the chunk reaches are the filesystem vector's: a refused home or instance dir, and E5's replay recovery, which is built here only if the replay ruling lands.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
  (.andromeda/test-plan.md:209)
  > group/world-writable home or instance dir → refused
  (.andromeda/test-plan.md:174)
  > Snapshot corruption → replay recovery.
- (.andromeda/test-plan-detail/test-strategy.md:67) The vanished-endpoint case must stay in the §6 Chaos suite and read unreachable through the new check.
  > | chaos-test | chaos | §6 Chaos suite |
- (.andromeda/test-plan-detail/test-strategy.md:68) The hook client's added verification must stay inside the §10 performance budgets.
  > | performance-budget | perf | §10 Performance budgets |
  (.andromeda/test-plan.md:241)
  > **Required test type:** timing assertions that spine hooks (SessionStart, UserPromptSubmit, Stop) exit within the spine deadline and SessionEnd within ~1 s, and that heartbeat staleness flips at >5 s.
- (.andromeda/test-plan-detail/test-strategy.md:69) For the same unreachable outcome the MCP and CLI payloads must agree, asserted at E2E in the Cross-surface parity suite.
  > | cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
  (.andromeda/test-plan.md:247)
  > MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan-detail/test-strategy.md:70) Each OS branch of the server-pid read (Unix peer credentials, the Windows pipe server process id, macOS euid + dir only) must be tested on its own CI matrix runner.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
  (.andromeda/test-plan.md:250)
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan-detail/test-strategy.md:73) The new unreachable and fail-open outcomes must carry codes and fixed messages only, asserted in the integration and E2E error-sanitization checks: no pid, path or start time in a `--json` document, an MCP `isError` or channel `error.data`.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
  (.andromeda/test-plan.md:265)
  > **Required test type:** assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`.
- (.andromeda/test-plan.md:873) The Windows pipe server process id and the macOS euid + dir-only branches the chunk adds are proved only on their own runners, never by the Linux dev-host run.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) The impostor or squatter server a mismatch test needs must be bound through the server API with no `try_overwrite` and no reused endpoint name, and the client under test must take its endpoint only from `snapshot.json`.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.

## Bindings
- **test-scope-summary (viola-channel server verification) ↔ unit-test-strategy**: the testable pid/start-time verification (.andromeda/test-plan-detail/test-scope-summary.md:11) is held at unit level by extending the peer-credential decision with injected euids (.andromeda/test-plan.md:376) to the server pid / start-time decision, with the cross-user peer asserted only at that boundary.
- **test-strategy (IPC V1 row) ↔ integration-test-strategy and e2e-test-strategy**: the V1 mismatch negative (.andromeda/test-plan-detail/test-strategy.md:58) lands in the wrapper channel suites (.andromeda/test-plan.md:435) and the root security negatives, and its CLI exit 21 is one cause of the exit-cause matrix (.andromeda/test-plan.md:604).
- **test-strategy (Hook stdin V4 row) ↔ e2e-test-strategy (Security sweep)**: the verification failure joins the hook fail-open matrix the security sweep runs (.andromeda/test-plan-detail/test-strategy.md:60) (.andromeda/test-plan.md:598).
- **test-strategy (cross-surface row) ↔ e2e-test-strategy (Cross-surface parity)**: the MCP `instance-unreachable` (.andromeda/test-plan-detail/test-scope-summary.md:64) and the CLI exit-21 document (.andromeda/test-plan-detail/test-scope-summary.md:42) agree in the parity suite (.andromeda/test-plan.md:610).
- **test-strategy (multi-os row) ↔ ci-integration (Matrix builds)**: the OS-branch server-pid read (.andromeda/test-plan-detail/test-strategy.md:70) is verified per runner, macOS by euid + dir only (.andromeda/test-plan.md:767), never by a Linux run (.andromeda/test-plan.md:873).
- **test-strategy (perf row) ↔ quality-gates-coverage-targets (Performance budgets)**: the hook client's added check (.andromeda/test-plan-detail/test-strategy.md:23) stays under the spine deadline the gate reads from `viola_core::SPINE_DEADLINE` (.andromeda/test-plan.md:834).
- **test-scope-summary (chaos snapshot corruption) ↔ test-data-fixtures and e2e-test-strategy (E5)**: the corrupt or newer-`v` snapshot case (.andromeda/test-plan-detail/test-scope-summary.md:170) is planted only by altering the writer's bytes (.andromeda/test-plan.md:905) and its E5 replay is built only if the replay ruling lands (.andromeda/test-plan-detail/test-strategy.md:63).
