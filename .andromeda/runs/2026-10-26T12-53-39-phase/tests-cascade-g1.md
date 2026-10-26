# tests · cascade read · group 1 of 3

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The two served methods must keep the channel's result, refusal and error shapes, and the two new frame verbs join the server verification recorded as landed for the CLI frame verbs, so the "six" verbs it names must read eight once they land.
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  (.andromeda/test-plan-detail/test-scope-summary.md:12)
  > the pid/start-time server verification is asserted on Linux and Windows: the six CLI frame verbs by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives),
  (.andromeda/test-plan.md:46)
  > same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable
- (.andromeda/test-plan-detail/test-scope-summary.md:33) `viola link` / `viola unlink` with `--json` are verbs of this contract, so their exit codes and documents must fit its exit set, and their human result lines take its C0/C1 escaping.
  > - **Entity:** CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21;
  (.andromeda/test-plan.md:62)
  > - **Entity:** CLI verbs and the `--json` / exit-code contract (the verbs, global `--home`, exits 0/1/2/10–14/20/21, `revive`'s `--json` documents on both arms), plus the human TTY output (`viola list`, the readback mirror, C0/C1 escaping) — testable.
- (.andromeda/test-plan-detail/test-scope-summary.md:43) The two verbs' tests and the `v1-13` `link` half must read this signal: one JSON document per exit code, the human `link` / `unlink` lines by line match, and no SGR off a TTY, under `NO_COLOR` or `TERM=dumb`.
  > - **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`). Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`. There must be no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`.
  (.andromeda/test-plan.md:94)
  > - **Surface:** cli (`viola` verbs) — driven as a subprocess by `assert_cmd` with a per-test `--home` and `--json`; its signal is the exit code plus one JSON document on stdout, human mode checked by line match with no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`; the TTY rendering goes through the outer-PTY driver, never by inspecting the child agent's screen.
- (.andromeda/test-plan-detail/test-scope-summary.md:57) `link` and `unlink` are methods of this surface, now served: `tests/channel_links.rs` must assert their `ok` results (`{since, already_linked}`, the empty object) and their `-32602` faults on this signal, cross-checked against the `events.ndjson` cursor.
  > - **Surface:** ipc-internal (wrapper channel: JSON-RPC 2.0 over ndjson on a named pipe / Unix socket; methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`)
  >   - **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  >   - **Signal:** IPC response payload check. Success is `{"result":{"ok":…}}`. Refusal is `{"result":{"refusal":…,"detail":…}}` with the first-applicable ordering. Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`). This is cross-checked against the `events.ndjson` byte-offset `cursor`.
  (.andromeda/test-plan.md:104)
  > - **Surface:** ipc-internal (the wrapper channel, JSON-RPC 2.0 over ndjson) — driven by a language-native IPC client with JSON schema assertion; its signal is the success, refusal or protocol-error payload, cross-checked against the `events.ndjson` cursor; cross-user peer rejection is flagged in section 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:65) The scope adds no MCP link tool, so `tools/list` must hold no `link` / `unlink` after the chunk, and whether `v1-13`'s MCP half can be proved without one is the fork P3 reads.
  > There must be no `release` / `pause` / `link` / `unlink` tools in `tools/list`.
  (.andromeda/test-plan.md:106)
  > `tools/list` is exactly the five tools, never `release` / `pause` / `link` / `unlink`,
- (.andromeda/test-plan-detail/test-scope-summary.md:171) The chunk's product path appends the first `unlink` line, so the replay and the start-time restore must recover `links` without the removed pair.
  > As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive)
  (.andromeda/test-plan.md:237)
  > a corrupt or unsupported-`v` snapshot recovered by replay
- (.andromeda/test-plan-detail/test-strategy.md:10) The two verbs' refusal ordering and exit-code mapping must be rstest `#[case]` tables at the unit level, and their server check's captured-span cases compile out on macOS like every other frame verb's.
  > rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`
  (.andromeda/test-plan.md:285)
  > - **Unit:** inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices, on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
- (.andromeda/test-plan-detail/test-strategy.md:14) The two methods extend the Wrapper channel contract and the two verbs the CLI contract, so their root cases must run over a real `interprocess` endpoint, a real tempfile home and `assert_cmd` against the built `viola`.
  > - **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:287)
  > - **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan-detail/test-strategy.md:17) Scenario E3 (`link` / `unlink`) is one of the expansion scenarios this level requires, so its steps 1 and 3 must run against booted wrappers over the fake agent, with every surface the scenario lists given its own driver step.
  > - **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:289)
  > - **E2E:** required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
  (.andromeda/test-plan.md:173)
  > - `link` / `unlink`.
  (.andromeda/test-plan.md:869)
  > - NEVER assert a multi-surface critical path on one surface only. Every surface a §6 scenario lists needs its own driver step and transition signal.
- (.andromeda/test-plan-detail/test-strategy.md:58) The two new frame verbs take the server identity check through `client::connect`, so the CLI frame-verb impostor case in `security_negatives_server_identity.rs` must cover them off macOS: no frame and exit 21 `instance-unreachable`.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC + Wrapper channel (the MCP impostor case, `crates/viola-mcp/tests/handlers.rs`); §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper) + Security sweep (the hook fail-open impostor rows, off macOS) |
  (.andromeda/test-plan.md:184)
  > - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan-detail/test-strategy.md:62) The verbs take `<driver> <driven>` names and the raw-channel `link` / `unlink` name negative is owed, so a non-`ViolaName` name must be refused at the unit and integration layers this row maps.
  > | CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
  (.andromeda/test-plan.md:200)
  > - path-traversal names (`../x`, uppercase, `/`) rejected by `parse_viola_name`
- (.andromeda/test-plan-detail/test-strategy.md:63) If the founder's P4 form puts the instance check ahead of the verbs' snapshot read, a widened home or instance tree must be refused at integration with the CLI's exit 21, the layer this row maps.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
  (.andromeda/test-plan.md:201)
  > - `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
  (.andromeda/test-plan.md:209)
  > - group/world-writable home or instance dir → refused
- (.andromeda/test-plan-detail/test-strategy.md:69) Scenario E3 is this trigger's E2E row, and the `v1-13` `link` half rests on the CLI `--json` result carrying the same facts as the channel result, the parity this trigger requires.
  > | cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
  (.andromeda/test-plan.md:247)
  > - MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan-detail/test-strategy.md:70) The two verbs' server-identity cases hold on Linux and Windows and compile out on macOS, so their Windows leg must be read on the Windows CI runner and the macOS gap stays recorded, never claimed from the Linux host.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
  (.andromeda/test-plan.md:250)
  > - **Required test type:** full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
  (.andromeda/test-plan.md:873)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/test-strategy.md:73) The new `-32602` faults, any `-32603`, and the two verbs' `--json` documents and exit-21 hints must carry codes and fixed messages only, asserted at the integration and E2E layers this row maps.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
  (.andromeda/test-plan.md:265)
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:10) The chunk names `-32601` (the methods' answer today) and `-32602` (a foreign `driven`, a `driver` equal to `driven`), so the new invalid-params faults must keep the mapping this case pins, `-32603` exact.
  > - JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}` / -32603 with the exact `"internal error"` and `data:null`.
  (.andromeda/test-plan.md:376)
  > the JSON-RPC error mapping
- (.andromeda/test-plan-detail/unit-test-strategy.md:20) `LinkSet::remove` gains its product caller and the wrapper restores its set from the log at start, so the public link rule's `unlink` arm must hold for the line the new method appends, and an `unlink` of an absent pair must leave the set and the log unchanged.
  > Since chunk 2026-10-10-session-links the replay derives `links` in its one pass through the public rule `viola_state::replay::apply_link_line`: a `link` line whose `data.driver` and `data.driven` are strings adds `{driver, driven, since}` with `since` that line's `ts` unless the set holds the pair, an `unlink` line removes the pair, and any other line changes nothing (inline cases `link_rule_*`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:19) The two new frame verbs must read their snapshot through `live_endpoint`, whose one read feeds the server check, like the other CLI frame verbs.
  > the CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint`, whose one `read_snapshot` call feeds the server check.
  (.andromeda/test-plan-detail/unit-test-strategy.md:22)
  > `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.
- (.andromeda/test-plan-detail/unit-test-strategy.md:25) The P4 fork's second option is this instance check run ahead of the verbs' snapshot read; if the founder picks it, the verbs must take it as `viola revive` does, with its root refusal cases beside `cli_revive.rs`'s.
  > - The instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order.
  (.andromeda/test-plan.md:378)
  > Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.
- (.andromeda/test-plan-detail/unit-test-strategy.md:50) The two verbs' exits, `-32602` faults and unreachable cases included, must enter the root bin's exit-code mapping, its expected values written as literals.
  > - The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21.
  (.andromeda/test-plan.md:881)
  > - NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan-detail/unit-test-strategy.md:62) The task owes the two verbs' refusal ordering as root-bin unit cases, beside the `send` and `answer` orderings this case pins.
  > - Refusal ordering: `send` goes control-character → empty-text → human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted,
- (.andromeda/test-plan-detail/unit-test-strategy.md:63) A logged link name shown in a `link` / `unlink` result line must be escaped in human mode, like the `wait` / `last` and `list` rows.
  > - Human-mode C0/C1 escaping in `wait`/`last`, keeping `\n` and `\t`; in `list` rows (`escape_row`, chunk 2026-10-11-the-board-viola-list) every control character, `\n`, `\t`, DEL and C1 included, prints as `\xHH` text (unit cases `escape_row_shows_every_control_character_as_hex_text`, `board_row_cli_cell`).
  (.andromeda/test-plan.md:222)
  > - human-mode `list` / `wait` / `last` escape C0/C1
- (.andromeda/test-plan-detail/unit-test-strategy.md:68) The two new frame verbs must each carry the server check's captured-span case through `cmd::client::refused_inside`, and the list of verb files this case names must grow by them.
  > - The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, the server cause `during` `connect`).
  (.andromeda/test-plan.md:386)
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged, human-mode C0/C1 escaping

## Bindings
- **test-scope-summary (the channel surface's `-32602`) ↔ unit-test-strategy (the JSON-RPC error mapping)**: the surface signal fixes `-32602` with `data:{supported, wrapper}` (.andromeda/test-plan-detail/test-scope-summary.md:59) and the unit mapping pins the same shape (.andromeda/test-plan-detail/unit-test-strategy.md:10); the two new invalid-params faults (a foreign `driven`, `driver` equal to `driven`) must be read against both, so a `data` they carry is settled once.
- **test-strategy (the V1 IPC row) ↔ unit-test-strategy (the server check's span cases) ↔ test-scope-summary (as landed)**: the integration impostor case (.andromeda/test-plan-detail/test-strategy.md:58), the per-verb unit span case (.andromeda/test-plan-detail/unit-test-strategy.md:68) and the landed record of "the six CLI frame verbs" (.andromeda/test-plan-detail/test-scope-summary.md:12) each grow by `link` and `unlink` together, all off macOS (.andromeda/test-plan-detail/test-strategy.md:10).
- **test-scope-summary (the CLI contract and signal) ↔ unit-test-strategy (exit codes, escaping) ↔ test-strategy (cross-surface parity)**: the `v1-13` `link` half reads the one-document-per-exit signal (.andromeda/test-plan-detail/test-scope-summary.md:43), the unit exit-code mapping (.andromeda/test-plan-detail/unit-test-strategy.md:50) and escaping (.andromeda/test-plan-detail/unit-test-strategy.md:63), and the CLI / channel parity row (.andromeda/test-plan-detail/test-strategy.md:69).
- **test-scope-summary (the MCP surface) ↔ test-strategy (cross-surface parity)**: `tools/list` holds no `link` / `unlink` (.andromeda/test-plan-detail/test-scope-summary.md:65) while the parity row asks identical CLI and MCP payloads (.andromeda/test-plan.md:247), which is why `v1-13`'s MCP half is P3's fork, not a build.
- **test-scope-summary (the replay's links) ↔ unit-test-strategy (the link rule)**: the replay's recovery from `link` / `unlink` lines (.andromeda/test-plan-detail/test-scope-summary.md:171) is the public rule's `unlink` arm (.andromeda/test-plan-detail/unit-test-strategy.md:20), which the first product `unlink` line now feeds.
- **test-strategy (the V7 row) ↔ unit-test-strategy (the instance check)**: the P4 fork's second option ties the integration refusal (.andromeda/test-plan-detail/test-strategy.md:63) to `check_instance` (.andromeda/test-plan-detail/unit-test-strategy.md:25); under the first option neither moves for the verbs.
- **test-strategy (E2E level) ↔ the e2e-test-strategy category (Scenario E3)**: the level's expansion scenarios (.andromeda/test-plan-detail/test-strategy.md:17) include `link` / `unlink` (.andromeda/test-plan.md:173), whose steps 1 and 3 are another group's scenario entity to build.
