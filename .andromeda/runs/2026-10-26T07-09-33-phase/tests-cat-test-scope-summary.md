# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The server verification this chunk builds must be tested same-user in the channel's own scope, alongside framing, dispatch and refusal order.
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
- (.andromeda/test-plan-detail/test-scope-summary.md:11) Any cross-user (different euid) leg of the verification must be asserted at the peer-credential check boundary, because hosted runners cannot drive it end to end.
  > The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary.
- (.andromeda/test-plan-detail/test-scope-summary.md:32) The verified CLI clients (`send`, `wait`, `last`, `answer`, `pause`, `release`) must keep the closed exit-code set; a verification failure has to map to a code inside it.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21;
- (.andromeda/test-plan-detail/test-scope-summary.md:41) Tests of a CLI verb's verification outcome must drive the real binary as a subprocess with a per-test home and `--json`.
  > subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
- (.andromeda/test-plan-detail/test-scope-summary.md:42) A mismatch refusal must be asserted on its exit code plus exactly one JSON document on stdout.
  > exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
- (.andromeda/test-plan-detail/test-scope-summary.md:43) The CLI surface counts as cross-surface: the verb-to-wrapper hop that verification guards has to be exercised against a real wrapper.
  > cross-surface (verbs talk to the `run` wrapper over the IPC channel, and read disk for `list`)
- (.andromeda/test-plan-detail/test-scope-summary.md:56) The set of channel methods whose clients must verify before the first frame includes `hook.dialog` and `hook.event`. `link` / `unlink` are not served by this chunk.
  > methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`)
- (.andromeda/test-plan-detail/test-scope-summary.md:57) Channel-level verification tests must use the language-native IPC client (the sync client, or a raw `interprocess` client) with schema assertion.
  > language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
- (.andromeda/test-plan-detail/test-scope-summary.md:63) The MCP tools' verification must be tested through a `viola mcp --home <tmp>` stdio client.
  > language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
- (.andromeda/test-plan-detail/test-scope-summary.md:64) An MCP tool's verification failure must surface as the coded `instance-unreachable` result, never as a new shape. Any other fault stays `-32603` `internal error` / `null`.
  > `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`. An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is the JSON-RPC error `-32603` `internal error`, `data: null`;
- (.andromeda/test-plan-detail/test-scope-summary.md:65) The MCP verification must be tested across the whole cross-surface chain down to a real wrapper.
  > cross-surface (MCP → wrapper channel → run wrapper → fake agent)
- (.andromeda/test-plan-detail/test-scope-summary.md:76) The snapshot fields the verification reads (`pid`, `started_at`, `endpoint`) are the harness `boot` readiness fields and must keep being written as they are now.
  > Readiness, per instance: `instances/<name>/snapshot.json` exists with `pid`, `started_at`, `child_pid` and `endpoint`.
- (.andromeda/test-plan-detail/test-scope-summary.md:92) The harness `cleanup` check (a CLI verb against a gone name exits 21) must still hold once the verb verifies the server. This touches CARRY 2's window where an exiting wrapper's pipe is still open.
  > Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
- (.andromeda/test-plan-detail/test-scope-summary.md:105) Each verification test must bootstrap its own fresh home.
  > Self-bootstrapping per test: a fresh `--home` temp dir.
- (.andromeda/test-plan-detail/test-scope-summary.md:112) A mismatch case must not rely on developer-seeded state. The mismatched snapshot has to come from a real run's own bytes.
  > No developer-seeded state.
- (.andromeda/test-plan-detail/test-scope-summary.md:118) `run`'s collision check reads the snapshot (CARRY 8). Its duplicate-name and squatted-endpoint exits must stay 1.
  > A second `run` of the same name, or a squatted endpoint, exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:126) Client-side refusals must still come "before any frame". Where verification sits relative to them in that pre-frame order has to be fixed and tested.
  > A text whose typed text is empty is refused `not-delivered`/`empty-text` (exit 13) right after `control-character` and before anything is issued, by the client before any frame and by the wrapper.
- (.andromeda/test-plan-detail/test-scope-summary.md:170) The snapshot-corruption chaos case is where a client's reading of an unreadable or newer snapshot would show. Under CARRY 7's hold, the client outcome stays unreachable / fail-open.
  > corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`.
- (.andromeda/test-plan-detail/test-scope-summary.md:172) An endpoint that vanishes during `wait` must still read `instance-unreachable` with verification in place.
  > endpoint vanishes during `wait`: `instance-unreachable`
- (.andromeda/test-plan-detail/test-scope-summary.md:173) The pid + start-time identity check must not blur the stale-heartbeat / live-pid distinction.
  > stale-heartbeat / live-pid distinction
- (.andromeda/test-plan.md:24) Tier Comprehensive sets the breadth: negative tests per vector, multi-OS, chaos.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:46) The channel entity's broad line lists server verification as same-user testable and the cross-user peer as boundary-asserted only.
  > `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets — partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:48) The verification's pid + start-time reading and any strict-modes step it takes are `viola-state` surfaces, partially testable in a temp home.
  > `viola-state`: ndjson logs, atomic snapshots, `File::lock` on `.lock` siblings, torn-line healing, tailing, pid + start-time liveness, rebuild by replay, Unix modes and strict-modes
- (.andromeda/test-plan.md:58) The hook client's verification must keep the hook entity's exit-0, no-stderr contract.
  > They always exit 0, never write stderr, emit decision bodies,
- (.andromeda/test-plan.md:88) A second-user endpoint squatter is an untestable zone. It must not be claimed as E2E-verified.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:90) The macOS euid + dir-only leg can be verified on CI only.
  > macOS local runs. Security excerpt, Context: "macOS is CI-only in v1".
- (.andromeda/test-plan.md:97) The hook client's mismatch case must be driven with recorded fixture stdin and the instance env pair.
  > subprocess + `assert_cmd` feeding recorded hook payloads from `fixtures/claude/<cli-version>/` on stdin, with and without `VIOLA_NAME` / `VIOLA_DIR`
- (.andromeda/test-plan.md:98) On a mismatch the hook must show exit 0, empty stderr, and empty stdout (no decision body).
  > exit code is always 0. stderr is always empty. stdout is either empty (no decision / async hooks) or exactly the PreToolUse / PermissionRequest decision body.
- (.andromeda/test-plan.md:99) The hook's `hook.dialog` / `hook.event` hop is the boundary the verification guards. It has to be tested cross-surface down to disk, with no event line written on a mismatch.
  > cross-surface (hook → IPC `hook.dialog` / `hook.event` → wrapper → disk)
- (.andromeda/test-plan.md:128) The verification oracle is the snapshot's `pid` / `started_at`. The liveness classification (`gone` on a pid or start-time mismatch at any beat age) must stay consistent with it.
  > The wrapper `pid`, `started_at` and `child_pid` live in `<home>/instances/<name>/snapshot.json`, with liveness in `<home>/instances/<name>/heartbeat` (touched every 1 s): when pid + start time no longer name a running process the instance is `gone` whatever the beat's age; otherwise a beat at most 5 s old is `live`, and an older or absent one `stale`.
- (.andromeda/test-plan.md:145) A wrapper killed mid-wait must still give exit 21 / `instance-unreachable` once verification is in place.
  > Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
- (.andromeda/test-plan.md:184) This is the chunk's own required negative test: a server pid / start-time mismatch gives exit 21 on every CLI verb, `instance-unreachable` on every MCP tool, and exit 0 with no body on every hook.
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:193) The hook fail-open matrix's channel-failure case must keep passing, and the verification failure joins it as a fail-open cause.
  > fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body.
- (.andromeda/test-plan.md:201) If verification takes the strict-modes half with it, a failing home must map to these codes: 1 for `run` / `mcp`-class starts, 21 for CLI verbs, 0 for hooks.
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:241) The hook's added pre-frame verification must keep spine hooks inside the spine deadline and SessionEnd inside about 1 s.
  > timing assertions that spine hooks (SessionStart, UserPromptSubmit, Stop) exit within the spine deadline and SessionEnd within ~1 s, and that heartbeat staleness flips at >5 s.
- (.andromeda/test-plan.md:247) A verification failure must read the same across MCP and CLI for the same channel result.
  > MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan.md:250) Every OS branch of the server-pid read must be tested on its own runner: the named pipe, the Unix socket, and macOS's euid + dir-only verification.
  > full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:265) A verification refusal's `--json`, `isError` and `error.data` must not carry a pid, a path or any upstream text.
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`.
- (.andromeda/test-plan.md:271) Mismatch and endpoint-gone tests must synchronise on logged events and offsets, never sleeps. This includes the CARRY 2 exiting-wrapper window.
  > Waits must key on logged events and byte offsets, not sleeps.

## Bindings
- **pid / start-time mismatch negatives ↔ security-plan / security.md §Local trust boundary**: security requires verification "Before the FIRST frame of any method (including `hook.event`)". Tests carry the three outcomes (CLI 21, MCP `instance-unreachable`, hook 0 with no body) (.andromeda/test-plan.md:184).
- **macOS euid + dir-only gap ↔ security.md (the recorded macOS gap) and §9 CI matrix**: tested on CI only, on its own runner (.andromeda/test-plan.md:250), (.andromeda/test-plan.md:90).
- **cross-user peer ↔ security-plan Vector 1 / the peer-credential check (viola-channel)**: asserted only at the peer-credential boundary (.andromeda/test-plan-detail/test-scope-summary.md:11).
- **strict-modes half ↔ route entry "Home and code-bearing file integrity" / viola-state strict checks**: if this chunk takes it, the codes are 1 / 21 / 0 (.andromeda/test-plan.md:201). Otherwise it stays with that entry.
- **hook pre-frame verification ↔ obs / §10 perf gate (`SPINE_DEADLINE`, hook connect deadline)**: spine hooks must stay inside the deadline (.andromeda/test-plan.md:241).
- **unreadable or newer snapshot outcome ↔ CARRY 7 hold (the founder's word) / architecture On-disk replay**: today's outcome is kept in the snapshot-corruption chaos case (.andromeda/test-plan-detail/test-scope-summary.md:170).
- **harness `cleanup` / `boot` ↔ §3 `run` / `boot` contracts (registries/test-plan-contracts.toml)**: readiness fields and the gone-endpoint exit 21 must still hold (.andromeda/test-plan-detail/test-scope-summary.md:76), (.andromeda/test-plan-detail/test-scope-summary.md:92).
- **error sanitization ↔ security-plan §Error Handling**: no pid, path or upstream text in a verification refusal (.andromeda/test-plan.md:265).
