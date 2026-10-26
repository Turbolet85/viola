# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:264) The exit-21 matrix row this chunk builds: a same-user impostor listener on the recorded endpoint whose pid or start time does not match `snapshot.json` must exit 21 on Linux and Windows, and the macOS case must be the loosened socket dir, the macOS same-dir impostor never asserted as a refusal.
  > server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On macOS the client checks only euid and the socket dir (security excerpt, Vector 1), so the `#[cfg(target_os = "macos")]` case loosens the verified per-user socket dir to 0755 and asserts exit 21. A same-user impostor inside a valid 0700 dir on macOS is an accepted upstream limitation, not asserted as a refusal.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:261) Each exit-21 case the chunk adds must assert the exit code, the `--json` document and a last `hint:` line naming its own cause, distinct from every other cause's hint.
  > Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause. A shared hint across causes fails the table, and exact strings are pinned with trycmd once the product defines them.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:263) The neighbouring exit-21 cause the strict-modes half of the check meets: if verification takes the strict-modes check with it (P4's lean), its refusal is this row's, kept distinct from the server-verification row.
  >     - the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:282) The hint of the new server-verification refusal must not contain `release`.
  >   - Driver-facing hints never contain `release`: this holds for every refusal row in the table (`human-typing`, `budget-paused`), not only in Paths 5 and 6.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:90) A verification mismatch must give the same document as an unreachable instance: CLI exit 21 with this body, MCP `is_error:true` with the CLI document less `v` and the text `refused: instance-unreachable`.
  >   - Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The verifying client and every test of it take the endpoint from `snapshot.json` alone.
  > The endpoint is read only from `snapshot.json`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:27) An impostor listener on the recorded endpoint is planted the way Path 1's squatter is, through the interprocess server API; Path 1's squatted-endpoint exit 1 must stay green.
  > Squat: in a new home H2, create a listener on `viola_channel` endpoint name(`builder`, H2) through the interprocess server API, then run `viola run builder --home H2 -- <fake>`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) The MCP tools' verification is driven E2E through an rmcp client over a `viola mcp` child the test owns, never `TokioChildProcess`.
  > rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The hook's fail-open-on-mismatch case is driven by assert_cmd under an `Instant` deadline over fixture stdin, re-adding `LLVM_PROFILE_FILE` after any `.env_clear()`.
  > Fixture stdin from `fixtures/claude/<ver>/`. `.env_clear()` must re-add `LLVM_PROFILE_FILE`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:244) The hook's verification mismatch is a fail-open case of the hook matrix beside its channel-failure case, each exit 0 with no body.
  > The hook fail-open matrix cases are oversize stdin, malformed JSON, clap error, channel failure, forced panic and missing `VIOLA_NAME`. The sweep also covers the `--home` strict-modes refusals.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:289) The Unix socket-directory negative names the outcomes a refused client check must keep: a CLI client verb exit 21, `viola hook` exit 0 with no body and no frame.
  > In each case `viola run` exits 1, a CLI client verb against an existing instance exits 21, and `viola hook <event>` exits 0 with no body and writes no frame.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:290) The Windows client-side server verification control: the viola client's identification-level open is asserted against a test-owned pipe server.
  >   - **Windows client SQOS** (`#[cfg(windows)]`, client-side server verification). A test-owned same-user pipe server accepts the viola client's connection, calls `ImpersonateNamedPipeClient`, then reads the thread token's `TokenImpersonationLevel`. It must be `SecurityIdentification`, never `SecurityImpersonation` or higher, which proves the client opened with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:290) An owed item this chunk's client work touches: the SQOS negative against the viola client itself, not only the open recipe.
  > The `security_negatives_*.rs` case against the viola client lands with `viola-channel`, its first consumer.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:294) The verifying clients must write neither `snapshot.json` nor `stamps.json`; against a stopped wrapper `wait` / `last` / `send` still exit 21 and the MCP tools change neither file.
  > With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle), run every other writer-capable entry point: every `hook <event>` including `hook statusline`, `list`, `wait` / `last` / `send` (which then fail with exit 21) and `mcp` tool calls. Neither file changes.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:316) The vanished-endpoint chaos case must stay exit 21 once the client verifies the server.
  >   - endpoint vanishes during `wait`: exit 21
- (.andromeda/test-plan-detail/e2e-test-strategy.md:250) Every new client error payload passes the post-E2E sanitization check: no absolute path, serde path or `Caused by`.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:235) The owed move of the five other snapshot readers to the replay is named against this entry; it is bound to the scope's held CARRY 7, so the plan goes up to it behind the founder's stop rule.
  > Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the five other snapshot readers taking the replay are owed to "Server verification before any frame" (the review card the founder approved).
- (.andromeda/test-plan.md:184) The coverage trigger this chunk discharges: one mismatch outcome per client kind.
  >     - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:193) The hook mismatch case meets the fail-open bar: exit 0, empty stderr, no body.
  > fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body.
- (.andromeda/test-plan.md:201) If the strict-modes half lands here, its refusals per verb kind are these.
  >     - `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:437) The strict-modes outcome per entry point the clients must keep: CLI 21 for every verb but `run` / `mcp` / `revive`, hook 0 with no output.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb.
- (.andromeda/test-plan.md:46) Server verification is a testable channel behaviour; only a peer of another euid is held at the peer-credential boundary.
  > server verification and the squatted-name refusal are testable, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:476) A cross-user impostor is not an E2E case: it is held by the peer-credential unit test.
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:435) The channel suites assert server verification at the integration tier beside the E2E cases.
  > - **Wrapper channel:** every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame, server verification,
- (.andromeda/test-plan.md:250) The OS branches of the check each get their own test: Unix socket and named pipe, macOS euid + dir only.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:767) The macOS runner carries the euid + dir-only verification leg.
  > `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves)
- (.andromeda/test-plan.md:873) Each OS branch of the verification is proved only on its own CI runner, never by a Linux local run.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:611) The CLI and MCP mismatch documents must be equal, MCP's the CLI's less `v`.
  >   - for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:834) The hook's verification sits inside its connect path and must keep it under the spine deadline; the provisional connect bound and its assertion stand.
  > The hook's own connect deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
- (.andromeda/test-plan.md:840) The perf rows time each hook through the real channel path, so the added check must keep every row's `max` under `SPINE_DEADLINE`.
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:590) The body's own record of the owed E5 steps against this entry.
  > - **As landed: in part.** The replay's product readers (`viola revive`, `viola list`), its budget and link legs and its `list --json` leg are covered; steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`, owed to the route entry "Server verification before any frame".
- (.andromeda/test-plan.md:592) If E5 is built here, its two signals are restated to what landed.
  > - Two of the signals above are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}`, so `wheel:"human"` does not hold after step 3, and the replay writes no file, so no snapshot is rebuilt from it.
- (.andromeda/test-plan.md:868) The pure match decision (pid and start time against the snapshot) is held at the unit tier; E2E asserts only the cross-process outcome.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:869) Every client surface the check reaches (CLI, hook, MCP) gets its own driver step and signal.
  > - NEVER assert a multi-surface critical path on one surface only. Every surface a §6 scenario lists needs its own driver step and transition signal.
- (.andromeda/test-plan.md:889) An impostor test never takes the endpoint by `try_overwrite`; it reads it from the wrapper's `snapshot.json`.
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:905) A mismatch case starts from a snapshot the wrapper wrote; no test hand-writes `snapshot.json` to fake a pid or start time.
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test.
- (.andromeda/test-plan.md:895) The impostor and exiting-wrapper cases wait on logged signals, never a sleep.
  > - NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.

## Bindings
- **Server-verification exit-21 row ↔ security.md §Local trust boundary (the "Before the FIRST frame" clause and the macOS gap)**: the security clause names pid + start time against a strict-modes-passed snapshot, macOS euid + dir only; the test row asserts that same split and leaves the macOS same-dir impostor unasserted (.andromeda/test-plan-detail/e2e-test-strategy.md:264), (.andromeda/test-plan.md:184), (.andromeda/test-plan.md:767)
- **Mismatch document ↔ security.md §Error handling (MCP `isError` codes `instance-unreachable`)**: the MCP mismatch result is the code-only `instance-unreachable` body, equal to the CLI document less `v` (.andromeda/test-plan-detail/e2e-test-strategy.md:90), (.andromeda/test-plan.md:611)
- **Hook fail-open case ↔ CLAUDE.md universal invariant (`viola hook` always exits 0, never writes stderr, fails open with no body)**: the mismatch joins the fail-open matrix (.andromeda/test-plan-detail/e2e-test-strategy.md:244), (.andromeda/test-plan.md:193)
- **Strict-modes half ↔ working route "Home and code-bearing file integrity"**: whether verification here takes the strict-modes check decides whether the strict-modes exit-21 row and the per-verb outcomes bind in this chunk or that entry (.andromeda/test-plan-detail/e2e-test-strategy.md:263), (.andromeda/test-plan.md:201), (.andromeda/test-plan.md:437)
- **E5 owed readers ↔ scope CARRY 7 / 8 (the founder's hold on replay-backed clients, `verification-matrix.json#v1-41`)**: the owed move is named against this entry but held behind the stop rule (.andromeda/test-plan-detail/e2e-test-strategy.md:235), (.andromeda/test-plan.md:590)
- **Hook connect path ↔ §10 perf gate (`SPINE_DEADLINE`, `CONNECT_DEADLINE`)**: the added check runs inside the timed hook rows (.andromeda/test-plan.md:834), (.andromeda/test-plan.md:840)
- **Windows client SQOS ↔ security.md (client opens with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`)**: the viola-client negative is the owed half of that control (.andromeda/test-plan-detail/e2e-test-strategy.md:290)
- **Single writers ↔ security.md (only the wrapper writes `snapshot.json`)**: the verifying clients read the snapshot and write nothing (.andromeda/test-plan-detail/e2e-test-strategy.md:294)
