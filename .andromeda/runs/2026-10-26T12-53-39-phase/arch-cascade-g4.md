# arch · cascade read · group 4 of 4

## Carried
- (.andromeda/architecture-detail/conventions.md:21) `viola link` / `viola unlink` must exit 0 on success, 2 on a clap usage error, 20 on a wrapper protocol fault, 21 on an unreachable instance, and join the `cli` role's verbs whose dispatch `Err` or caught panic prints `error: internal error` once from the `main` catch site, the `--json` documents matching.
  > - **CLI exit codes:**
  >   - `0` ok
  (.andromeda/architecture-detail/conventions.md:26)
  > so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list` and `plugin install`), and a dispatch `Err` or a caught panic in any of them prints exactly `error: internal error` on stderr once, from the `main` catch site, no hint
  (.andromeda/architecture-detail/conventions.md:27)
  >   - `2` usage error (clap, a second positional to `viola send` included; never produced by `viola hook`)
  (.andromeda/architecture-detail/conventions.md:33)
  >   - `20` protocol fault from the wrapper
  (.andromeda/architecture-detail/conventions.md:34)
  >   - `21` instance unreachable (no live endpoint for the `ViolaName`)
  (.andromeda/architecture.md:100)
  > - **CLI exit codes:** `0` ok · `1` internal error, and the start, revive-preflight and verify refusals · `2` usage error · `10` `human-typing` · `11` `budget-paused` · `12` `unverified-cli` · `13` `not-delivered` · `14` `unknown` refusal · `20` protocol fault from the wrapper · `21` instance unreachable, and a refused strict-modes check of `viola list` or `viola plugin install`.
  (.andromeda/architecture.md:224)
  > **CLI `--json` output** mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code. Outcomes that have no channel `result` use `error`:
  > - `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
  > - `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` (exit 20)
- (.andromeda/architecture-detail/conventions.md:40) The two verbs take the server identity check through `client::connect`, so a server that is not the recorded wrapper must exit 21 with no frame written, the same document or `unable` line and hint, and the verb list this line names must gain `link` and `unlink`.
  > - Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper (the connected peer's pid or OS start time is not the snapshot's, or the check cannot be made): no frame is written, and the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty, or the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording: design-system, surface cli-send). The client's own refusals (exit 13) come first, and the cause has no exit code, `--json` code or `RefusalReason` of its own.
- (.andromeda/architecture-detail/conventions.md:46) `v1-13`'s `link` half claims the `--json` / MCP result carrying the same facts while no `link` MCP tool is built; the plan must prove that half against this MCP result shape or un-claim it with a dated note, as P3 reads it.
  > - **MCP:** a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`
  (.andromeda/architecture.md:104)
  > - **MCP:** a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`
- (.andromeda/architecture-detail/standard-contracts.md:24) The served `unlink` method appends the first product `unlink` lines, which the replay's link half must remove from a rebuilt `links`; the verbs read the snapshot only inside `live_endpoint`.
  > Replay recovers only log-derived fields: `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`.
  (.andromeda/architecture-detail/standard-contracts.md:25)
  > `links` is recovered in the same pass through the public rule `viola_state::replay::apply_link_line` (chunk 2026-10-10-session-links): a `link` line whose `data.driver` and `data.driven` are strings adds `{driver, driven, since}` with `since` that line's `ts` unless the set holds the pair, an `unlink` line removes the pair, and a line of another kind or with a name that is no string changes nothing.
  (.andromeda/architecture-detail/standard-contracts.md:25)
  > the CLI frame verbs and `viola mcp` read the snapshot inside `viola_state::liveness::live_endpoint`
  (.andromeda/architecture.md:152)
  > **Snapshot envelope** (atomically replaced JSON): `{v, written_at, writer, data}`. A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.
- (.andromeda/architecture-detail/standard-contracts.md:41) The `link` and `unlink` methods must change `LinkSet` (its `remove` gaining its product caller) only after the line has landed, rewrite the snapshot's `links` through `Snapshots::update`, and leave the set as it was on a failed append; `since` is the `link` line's `ts` in the RFC 3339 millisecond `Z` form.
  > - `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.
  (.andromeda/architecture.md:164)
  > - `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`, `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and the set follows the log: a failed append leaves it as it was.
  (.andromeda/architecture.md:130)
  > Always RFC 3339 UTC with milliseconds and a `Z` suffix, for example `2026-09-23T19:43:58.123Z` (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`).
  (.andromeda/architecture.md:411)
  > - **Crash-safe disk writes.** Append-only logs use one `write` per line. Mutable files are replaced atomically. Every guarded file has its own `.lock` sibling.
- (.andromeda/architecture-detail/standard-contracts.md:47) The CLI clients of `link` and `unlink` must compare the connected peer's pid and start time with the snapshot's before their first frame, and this client list, with the channel method contract that today answers both `-32601`, moves to the served methods and their `-32602` / `-32603` faults on the sync server.
  > - Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, compares the connected stream's peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and stays a recorded gap.
  (.andromeda/architecture.md:91)
  > Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method.
  (.andromeda/architecture.md:96)
  > with codes `-32700` parse error, `-32600` invalid request, `-32601` unknown method, `-32602` bad params / unsupported `v`, `-32603` internal error.
  (.andromeda/architecture.md:178)
  > Every `params` also carries `v` and `sender`. `from` is the caller's `VIOLA_NAME` when set, and is omitted otherwise. The frame envelope does not type `from`: each method that reads it (`send`, `wait`, `last`, `answer`, `pause`) takes it absent, `null` or a valid `ViolaName` and answers anything else `-32602` `invalid params` with `data: null`, never `-32600`; `release` refuses any string `from` (below).
  (.andromeda/architecture.md:193)
  > - `link` and `unlink` `{driver, driven}`, sent to the driven instance's wrapper → `{}`. Neither is built as of chunk 2026-10-10-session-links, which landed the implicit link alone: the wrapper answers both `-32601`, and the explicit verbs are held for the founder's word on the boundary widening they need ([Links]).
  (.andromeda/architecture.md:410)
  > - **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.
- (.andromeda/architecture-detail/standard-contracts.md:59) The `unlink` line the wrapper now appends must be log-only like `link`, never appended through `WaitFeed::appending` and never waking a parked `wait`, and the paragraph saying no product path appends `unlink` ends with this chunk.
  > Hook lines signal, and of the wrapper's own records only `dialog-expired`, appended through `WaitFeed::appending` (the wrapper's `send-*`, `link` and start records never wake).
  (.andromeda/architecture.md:212)
  > - `link` / `unlink`: `{driver, driven}`.
  (.andromeda/architecture.md:220)
  > Since chunk 2026-10-10-session-links the driven instance's wrapper also appends `link` (`source: wrapper`, `{driver, driven}`), once per new `driver → driven` pair on an implicit link (an issued `send` or a taken `answer` from another instance, [Links]). It is log-only and never wakes `wait` either, and it is how the audit trail and a replay learn the link set. No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served.
- (.andromeda/architecture-detail/standard-contracts.md:93) The chunk re-reads the client path at HEAD, so the verbs must take the liveness-only pre-check first and the server check after it, from the one snapshot read `live_endpoint` makes, with macOS on the recorded gap.
  > - The server identity read is `viola_state::liveness`'s. `live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call, so a frame-writing client takes the liveness-only pre-check first and the server check after it. The closed decision enum `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares the snapshot's `pid` and `started_at` (whole seconds, read through its RFC 3339 form) with the observed peer pid and that process's OS start time; with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap. The joined `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` reads the observed start time through `process_start_time` (a zombie reads none) and returns `server_verdict`'s verdict.
  (.andromeda/architecture.md:157)
  > - `pid` and `started_at` identify the wrapper process for the liveness check and, on Linux and Windows, for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame (macOS reports no peer pid, the recorded gap).
  (.andromeda/architecture.md:243)
  > - The wrapper touches `<instance dir>/heartbeat` every 1 s. When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age. Otherwise a beat at most 5 s old is `live`, and an older or absent beat is `stale`.
- (.andromeda/architecture-detail/occupied-resources.md:8) The verbs must connect to the driven instance's `endpoint` as recorded in its snapshot, never a path rebuilt from their own environment while the snapshot is readable; that recorded value is what crossing (c) reads without strict-modes.
  > `run` resolves the path once and records it as `endpoint` in the instance snapshot. Every other process (`hook`, `mcp`, CLI verbs, and `run`'s collision check) connects to the recorded `endpoint`, because processes spawned inside the session may run with a different environment from the wrapper. It rebuilds the path by the same rule only when the snapshot is missing or unreadable.
  (.andromeda/architecture.md:263)
  > - **Linux / macOS:** Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root, chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot.
  (.andromeda/architecture.md:420)
  > - **Local endpoint trust boundary.** Any process that can open an instance's endpoint can `send` prompts and `answer` dialogs, including PermissionRequest. v1 trusts every process of the same OS user and no other.
- (.andromeda/architecture-detail/occupied-resources.md:14) The two verbs connect to the recorded `endpoint` and must check the serving process before their first frame, so this list of checking processes must gain them, a failed check sending no frame and exiting 21.
  > - Before its first frame each process that connects to the recorded `endpoint`, but `run`'s collision check (`hook`'s event and dialog paths, the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release`, and `mcp`'s four frame tools), checks that the process serving it is the snapshot's wrapper: the connected stream's peer pid (`viola_channel::Client::server_pid` / `TokioClient::server_pid`) and its OS start time against the snapshot's `pid` and `started_at` (§Standard Contracts, Session liveness). A mismatch or a check that cannot be made sends no frame: CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open. macOS reports no peer pid and stays on the recorded gap.
- (.andromeda/architecture-detail/occupied-resources.md:29) The clap dispatch the chunk edits must serve `link` and `unlink` with arguments mirroring the channel `params`, each name a `ViolaName`, and the CLI adding `from` from its own `VIOLA_NAME`; the `viola --help` table gains its `handoff:` row only once they are served.
  > - Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.
  (.andromeda/architecture.md:270)
  > - Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp` (hidden from `viola --help`, run by the plugin), `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.
  (.andromeda/architecture.md:125)
  > `viola link <driver> <driven>`, `viola unlink <driver> <driven>`.
  (.andromeda/architecture.md:125)
  > Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
  (.andromeda/architecture.md:128)
  > The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, enforced by nutype.
- (.andromeda/architecture-detail/occupied-resources.md:59) The two new verb arms must resolve their home through `resolved_home`, the `VIOLA_DIR` grandparent taken only for a pair that passes `instance_of`'s shape check, so a verb run inside a session started with `--home` reaches that home.
  > As landed (chunk 2026-10-17-cli-machine-contract), `hook` and every CLI verb, `run` and `revive` included, read it: the verbs through `resolved_home`, which takes the `VIOLA_DIR` grandparent only when `VIOLA_NAME` and `VIOLA_DIR` pass `instance_of`'s shape check, a pair that fails falling through to the default, never refusing (§Cross-cutting Patterns, Config management)
  (.andromeda/architecture.md:298)
  > - `VIOLA_DIR`: the absolute path of the instance's state directory, set by `viola run` in the child; `hook`, `mcp` and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home.
- (.andromeda/architecture-detail/occupied-resources.md:97) The two verbs become producers of `cli-<name>.ndjson` (`<name>` the driven instance), writing the same process-start, client channel lines and process-exit, with `server-verify-failed` + `during:"connect"` and no `channel-request` line on a failed server check, and this producer list must name them.
  > `cli-<name>.ndjson`: `send` / `wait` / `last` / `answer` / `pause` / `release` — `<name>` the target instance — write `process-start`, the client `channel-request` / `channel-response` lines (`wait`'s carry `after` / `timeout_ms` / `outcome`) and `process-exit{subject:"self"}`, exit 21 with `detail:"instance-dead"` + `during`, or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line), at WARN, with no pid, start time, path or endpoint, exit 20 with `detail:"wrapper-fault"`
- (.andromeda/architecture-detail/occupied-resources.md:105) The scope's second form takes `check_instance` ahead of the snapshot read as `viola revive` does: under it the check must come before any log opens, the reader count and the `check_instance(` call-site count with its rule move, and the Windows DACL case stays owed to "Home and code-bearing file integrity"; the unlink appends never truncate `events.ndjson`.
  > v1 never rotates or truncates `events.ndjson`, because `wait`'s `after` and `send`'s `cursor` are byte offsets into it. Bounding its size is an open item, and any scheme must keep existing offsets valid. Three readers check these files before they use them.
  (.andromeda/architecture-detail/occupied-resources.md:106)
  > `viola revive` (the start arm and `--list`) runs `viola_state::strict::check_instance` over the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order, before its wrapper log opens
  (.andromeda/architecture-detail/occupied-resources.md:106)
  > Both checks come first because opening a log sets an existing home to 0700 (as measured on the statusline arm at chunk 2026-10-10-statusline-pass-through, `evidence/red-green.md` §3), and a check made after it passes a home at 0770.
  (.andromeda/architecture-detail/occupied-resources.md:106)
  > `check_instance` has these two product call sites (the rule: `check_instance(` under `src/`).
  (.andromeda/architecture-detail/occupied-resources.md:107)
  > The check is proven on Unix (five widened modes). No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry "Home and code-bearing file integrity".
  (.andromeda/architecture.md:325)
  > `viola list`, `viola revive` and `hook statusline` check these files before they use them, ahead of any log, because opening a log narrows an existing home.
- (.andromeda/architecture-detail/occupied-resources.md:150) The scope re-reads at P3 that no `fixtures/claude` file is touched: the committed sets must keep their counts by their own rules, no recording added.
  > `fixtures/claude/<cli-version>/`: hook-payload fixtures recorded by `viola verify --record` (only when 0 rows fail) and replayed by the fake agent in CI
  (.andromeda/architecture-detail/occupied-resources.md:152)
  > 34 files, the count's rule `ls fixtures/claude/2.1.287 | wc -l`)
  (.andromeda/architecture-detail/occupied-resources.md:152)
  > 30 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) The two new arms of `dispatch` must read the home through `resolved_home` like every verb arm, `--home` first, then the `VIOLA_DIR` grandparent, then the default.
  > Viola home: a global `--home <dir>` flag on every subcommand overrides the default `<user home>/.viola/`. Resolution order: `--home`, then the grandparent of `VIOLA_DIR` when it is set, then the default.
  (.andromeda/architecture-detail/cross-cutting-patterns.md:9)
  > `resolved_home` reads the two variables and `std::env::home_dir()` once, at every verb arm of `dispatch`
  (.andromeda/architecture.md:401)
  > The viola home resolves `--home`, then the grandparent of `VIOLA_DIR`, then `<user home>/.viola/`; `config.json` is parsed tolerantly and carries `v`.
- (.andromeda/architecture-detail/cross-cutting-patterns.md:18) The scope's boundary (no ledger row, no `viola verify` child, no re-stamp, no live `claude` start, re-read at P3) holds only while a link is recorded from viola's own parameters with no new dependence on a `claude` behaviour.
  > Any new dependence on an undocumented `claude` behaviour is added as a ledger row in `viola-agent-claude`, with a `viola verify` probe that has a post-condition check. It is never hard-coded as an unconditional assumption.
  (.andromeda/architecture.md:408)
  > One ruled limit: a CLI shape no `send` relies on needs no probe.

## Bindings
- **conventions exit 21 ↔ standard-contracts client check ↔ occupied-resources checking processes and `cli-<name>` producers**: the two verbs join one list in four places together — the exit-21 server-mismatch verbs (.andromeda/architecture-detail/conventions.md:40), the clients that check before their first frame (.andromeda/architecture-detail/standard-contracts.md:47), the processes that verify the serving wrapper (.andromeda/architecture-detail/occupied-resources.md:14) and the `cli-<name>.ndjson` producers with `server-verify-failed` (.andromeda/architecture-detail/occupied-resources.md:97); a master that names one list without the others drifts.
- **standard-contracts links set and replay ↔ standard-contracts `wait` wake ↔ occupied-resources `events.ndjson`**: the `unlink` line the served method appends is removed by the replay's link half (.andromeda/architecture-detail/standard-contracts.md:25), follows the log in `LinkSet` and the snapshot (.andromeda/architecture-detail/standard-contracts.md:41), never wakes `wait` (.andromeda/architecture-detail/standard-contracts.md:59) and lands by append in a log never truncated (.andromeda/architecture-detail/occupied-resources.md:105).
- **standard-contracts server identity read ↔ occupied-resources recorded `endpoint` ↔ the P4 form**: `live_endpoint`'s one snapshot read (.andromeda/architecture-detail/standard-contracts.md:93) is where the verbs get the recorded `endpoint` (.andromeda/architecture-detail/occupied-resources.md:8), `pid` and `started_at` — the read crossing (c) makes without strict-modes under the first form, or after `check_instance` under the second (.andromeda/architecture-detail/occupied-resources.md:106).
- **occupied-resources subcommands ↔ occupied-resources `VIOLA_DIR` ↔ cross-cutting config**: the new clap arms (.andromeda/architecture-detail/occupied-resources.md:29) resolve their home through `resolved_home` (.andromeda/architecture-detail/occupied-resources.md:59, .andromeda/architecture-detail/cross-cutting-patterns.md:9) and add `from` from `VIOLA_NAME` (.andromeda/architecture.md:125).
- **conventions MCP results ↔ the MCP tool list**: `v1-13`'s MCP half reads an MCP `ok` result (.andromeda/architecture-detail/conventions.md:46) that no tool of `send · wait · last · answer · list` carries for a link, so the claim's MCP half is proven or un-claimed, never refined down (.andromeda/architecture.md:104).
- **cross-cutting capability ledger ↔ occupied-resources fixtures**: no new `claude` dependence (.andromeda/architecture-detail/cross-cutting-patterns.md:18) means no recording, so the committed fixture sets keep their counts by their rules (.andromeda/architecture-detail/occupied-resources.md:152).
