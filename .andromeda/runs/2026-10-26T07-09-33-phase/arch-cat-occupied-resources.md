# arch · Occupied Resources — category read

## Carried
- (.andromeda/architecture-detail/occupied-resources.md:8) Every client this chunk verifies must connect to the `endpoint` the snapshot records, the snapshot it verifies the server against; the clause that rebuilds the path when the snapshot is missing or unreadable must stay unused by a frame-writing client unless the founder's word on the held self-healing CARRY rules otherwise.
  > `run` resolves the path once and records it as `endpoint` in the instance snapshot. Every other process (`hook`, `mcp`, CLI verbs, and `run`'s collision check) connects to the recorded `endpoint`, because processes spawned inside the session may run with a different environment from the wrapper. It rebuilds the path by the same rule only when the snapshot is missing or unreadable.
- (.andromeda/architecture-detail/occupied-resources.md:8) The Unix endpoint the verification reads the peer of is a socket file in a per-user 0700 dir with a sibling start lock; the verification adds no endpoint, dir or lock beside it.
  > - **Linux / macOS:** Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (security-plan arch amendment 2): `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, and `/tmp/viola-<uid>/` only as a fallback; never the `/tmp` or `$TMPDIR` root. The socket is chmod 0600 after the bind. The start arbiter is the sibling `<socket dir>/viola-<h12>.lock`: `run` holds an exclusive lock on it, and a held lock is the taken-name refusal.
- (.andromeda/architecture-detail/occupied-resources.md:23) The `viola mcp` client keeps its existing home check and exit shape; a verification failure is a per-tool outcome, never this process-level exit.
  > `mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`; it serves the five tools over stdio until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`.
- (.andromeda/architecture-detail/occupied-resources.md:53) The hook, MCP and CLI clients must find the snapshot they verify through `VIOLA_DIR` as today, the hook's SessionEnd direct-append fallback (the CARRY on the exiting wrapper's pipe) included; no new variable is added.
  > It is read by `hook` (for the recorded `endpoint`, the SessionEnd direct-append fallback, the statusline wrapper and the diagnostics directory) and by `mcp` and every CLI verb run inside the session.
- (.andromeda/architecture-detail/occupied-resources.md:91) The CLI verbs' role-file lines are fixed: a verification mismatch reported as exit 21 must fit this line set (an `instance-dead`-class WARN exit), any new detail word an amendment here at the wrap.
  > `cli-<name>.ndjson`: `send` / `wait` / `last` / `answer` / `pause` / `release` — `<name>` the target instance — write `process-start`, the client `channel-request` / `channel-response` lines (`wait`'s carry `after` / `timeout_ms` / `outcome`) and `process-exit{subject:"self"}`, exit 21 with `detail:"instance-dead"` + `during` at WARN, exit 20 with `detail:"wrapper-fault"`;
- (.andromeda/architecture-detail/occupied-resources.md:93) The MCP client's role-file lines are fixed; a refused verification writes no `channel-request` for a frame never sent, and any new line is an amendment here.
  > `mcp.ndjson` (chunk 2026-10-17-mcp-server-for-drivers), written by `viola mcp` once its home and `diagnostics/` passed `check_path` (a refusal opens no role file and writes no line): `process-start{subject:"self"}`, the Tokio client's `channel-request` / `channel-response` lines,
- (.andromeda/architecture-detail/occupied-resources.md:93) The hook client's role-file line set is closed; its fail-open on a mismatch must use these kinds, any content-bearing detail going only to the instance detail file.
  > `hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected`; `hook`'s drift reports go only to the instance's `diagnostics/detail-hook.ndjson`).
- (.andromeda/architecture-detail/occupied-resources.md:99) If this chunk takes the strict-modes half with the verification, each client that checks `snapshot.json` is a new reader of these files and the reader count here moves at the wrap.
  > Three readers check these files before they use them.
- (.andromeda/architecture-detail/occupied-resources.md:100) Any strict-modes check a client adds must run before its role log opens, the order this file records for the existing readers.
  > Both checks come first because opening a log sets an existing home to 0700 (as measured on the statusline arm at chunk 2026-10-10-statusline-pass-through, `evidence/red-green.md` §3), and a check made after it passes a home at 0770.
- (.andromeda/architecture-detail/occupied-resources.md:100) A count with its rule: a `check_instance(` call added under `src/` by a client moves this count, read back by the rule at the wrap.
  > `check_instance` has these two product call sites (the rule: `check_instance(` under `src/`).
- (.andromeda/architecture-detail/occupied-resources.md:101) An owed item on the boundary: the Windows instance-tree DACL case stays owed to "Home and code-bearing file integrity" unless this chunk takes it.
  > No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry "Home and code-bearing file integrity".
- (.andromeda/architecture-detail/occupied-resources.md:132) The test-side endpoint waits the exiting-wrapper CARRY rests on: a verification test of a stopped or killed wrapper must keep these waits' rules (unconnectable endpoint after stop, holder gone after kill).
  > One of the waits is `tests/support/home.rs` `wait_endpoint_gone`, which `Wrapper::stop` / `stop_keep` run after the wrapper's exit until the snapshot's recorded `endpoint` is unconnectable; after a kill, `Wrapper::kill` waits on `holder_gone` instead (on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found), because a killed wrapper removes nothing and its socket file stays on Unix
- (.andromeda/architecture-detail/occupied-resources.md:132) A count with its rule: a root test wait added on the `WITHIN` bound moves this count, read back by its pattern at the wrap.
  > `Instant::now() + WITHIN` reads 28 sites in 20 files under `tests/`, as measured at chunk 2026-10-10-budget-governor's wrap on the Linux dev host's final tree
- (.andromeda/architecture.md:6) The server a client verifies is the one wrapper that owns the instance's endpoint, a `run` or a passed `revive`, through the one start path.
  > Every wrapper owns its own local-socket endpoint, whether it was started by `viola run` or by a `viola revive` whose preflight passed (both go through the one start path).
- (.andromeda/architecture.md:39) The Windows client's open stays the SQOS Identification open; the verification is added beside it, never through interprocess's default connect.
  > on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
- (.andromeda/architecture.md:59) The verification must compare pid and start time together, never a bare pid.
  > Rationale: the prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
- (.andromeda/architecture.md:100) A CLI verb's mismatch or unverifiable server reports exit 21, the existing instance-unreachable code.
  > `21` instance unreachable, and a refused strict-modes check of `viola list` or `viola plugin install`.
- (.andromeda/architecture.md:102) The hook client's mismatch path must exit 0.
  > - **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
- (.andromeda/architecture.md:104) An MCP tool's mismatch is an `isError: true` result shaped like CLI `--json`, never a JSON-RPC error.
  > a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`;
- (.andromeda/architecture.md:152) A snapshot that fails to parse or carries a higher `v` is ignored and only log-derived fields are replayed; the replay holds no `pid` or `started_at` and no `endpoint`, so a client cannot verify against it (bound to the held self-healing CARRY).
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.
- (.andromeda/architecture.md:156) The value the client compares: `started_at` is sysinfo's start time at whole seconds, so the client's reading of the connected server's start time must use the same source and precision.
  > `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`. `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
- (.andromeda/architecture.md:157) The snapshot's `pid` and `started_at` are the wrapper's identity the verification matches.
  > - `pid` and `started_at` identify the wrapper process for the liveness check.
- (.andromeda/architecture.md:176) Identity must come from the endpoint and the snapshot, never from a frame field.
  > so no method takes it as input and it is never identity.
- (.andromeda/architecture.md:225) The CLI `--json` document for a refused verification is this existing one.
  > - `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
- (.andromeda/architecture.md:243) The liveness rule the pre-check already applies: pid dead or alive with another start time is `gone`; the verification applies the same pid + start-time identity to the connected server.
  > When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age.
- (.andromeda/architecture.md:260) The Windows endpoint whose server process the client reads is the instance's named pipe; no new pipe is occupied.
  > - **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
- (.andromeda/architecture.md:261) Any test endpoint this chunk's tests bind (a squatting or mismatched server) must stay outside the `viola-<h12>` namespace, unique per test, and be recorded here if it is a new name class.
  > Both are outside the `viola-<h12>` namespace (the `test-` segment is never 12 lowercase hex characters), unique per test and never bound by `viola`.
- (.andromeda/architecture.md:291) The verification code in `viola-channel` must keep the crate's normal graph tokio-free, the Tokio client's half living behind its `tokio` feature.
  > `viola-channel` left that list at chunk 2026-10-17-mcp-server-for-drivers: it declares `tokio = ["dep:tokio", "interprocess/tokio"]` (its Tokio client, enabled by `viola-mcp`) and `fake-agent = ["tokio"]`, so its package-scoped mutation build compiles and tests the Tokio client; its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh`.
- (.andromeda/architecture.md:296) The hook client keeps its silent exit 0 without `VIOLA_NAME`, before any verification.
  > its absence makes every hook event path a silent exit 0
- (.andromeda/architecture.md:325) The body's reader list: a client that adds a strict-modes check of these files joins it, ahead of any log.
  > `viola list`, `viola revive` and `hook statusline` check these files before they use them, ahead of any log, because opening a log narrows an existing home.
- (.andromeda/architecture.md:401) No env var, flag or config key may switch the verification off.
  > environment variables are not a configuration channel: a dependency or feature that reads one breaks this rule like a direct read does.
- (.andromeda/architecture.md:405) The hook's mismatch outcome is the fail-open one: exit 0, no body.
  > - **Fail open toward the human.** Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human.
- (.andromeda/architecture.md:410) The verification on the hook and CLI paths is std-thread, blocking code; only the MCP path runs it under Tokio.
  > - **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime. CLI `wait` is a blocking read on `viola-channel`'s sync client.
- (.andromeda/architecture.md:420) The per-connection proof this chunk builds is the security specialist's to define; the body's boundary is same-OS-user only.
  > Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.

## Bindings
- **Recorded endpoint ↔ the body's Instance snapshot contract**: the client connects to the snapshot's `endpoint` (.andromeda/architecture-detail/occupied-resources.md:8) and verifies the server against the same snapshot's `pid` and `started_at` (.andromeda/architecture.md:156) (.andromeda/architecture.md:157); both sides read one snapshot in one process, the security plan's clause.
- **Endpoint rebuild on an unreadable snapshot ↔ the body's Snapshot envelope replay ↔ the held self-healing CARRY**: the rebuild rule (.andromeda/architecture-detail/occupied-resources.md:8) and the replay that recovers no `pid`, `started_at` or `endpoint` (.andromeda/architecture.md:152) together mean no verification can be made on a replayed state; the outcome stays today's (exit 21 / fail open) behind the founder's stop rule.
- **Strict-modes readers ↔ the security plan's strict-modes check ↔ the route entry "Home and code-bearing file integrity"**: the reader count and call-site count (.andromeda/architecture-detail/occupied-resources.md:99) (.andromeda/architecture-detail/occupied-resources.md:100) and the body's reader list (.andromeda/architecture.md:325) move only if this chunk takes the strict-modes half; the Windows DACL case stays owed there (.andromeda/architecture-detail/occupied-resources.md:101).
- **Role-file lines ↔ the obs plan's event catalog and `diag-line` schema**: the CLI, MCP and hook line sets (.andromeda/architecture-detail/occupied-resources.md:91) (.andromeda/architecture-detail/occupied-resources.md:93) bind any new line or detail word to an obs amendment on both sides.
- **Endpoint-gone test waits ↔ the test plan's §3 `run` step 2 ↔ the exiting-wrapper CARRY**: `wait_endpoint_gone` / `holder_gone` (.andromeda/architecture-detail/occupied-resources.md:132) are the test side of the window in which a hook reached an exiting wrapper's pipe; the server's pid + start time stay its own until it is gone (.andromeda/architecture.md:243).
- **Outcome codes ↔ the body's Conventions and CLI `--json` / MCP contracts**: exit 21 and the `instance-unreachable` document (.andromeda/architecture.md:100) (.andromeda/architecture.md:225), MCP `isError` results (.andromeda/architecture.md:104), hook exit 0 (.andromeda/architecture.md:102) (.andromeda/architecture.md:405).
- **`viola-channel` placement ↔ the body's Tokio containment and the supply-chain gates**: the sync and Tokio clients' verification keeps the crate's normal graph tokio-free (.andromeda/architecture.md:291) (.andromeda/architecture.md:410), proven by `scripts/channel-tokio-free.sh` and the `cargo deny` bans.
