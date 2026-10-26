# arch · cascade read · group 2 of 3

## Carried
- (.andromeda/architecture-detail/established-decisions.md:175) The client check must match both the snapshot's `pid` and its start time against the connected server, never a bare pid, a pid alive with another start time reading as a mismatch.
  > pid + process start time decide `gone`, and `claude agents --json` only enriches it.** Rationale: the prototype's heartbeat proved reliable. A bare pid is unsafe because pids get reused, which is why start time is matched too: a snapshot whose pid is dead, or alive with another start time, is `gone` whatever the heartbeat says (Session liveness).
  (.andromeda/architecture-detail/established-decisions.md:175)
  > A `viola run` whose `ViolaName` is `live` or `stale` refuses to start (exit 1, with a message naming the instance); once the endpoint exists, a name with an answering endpoint refuses the same way.
  (.andromeda/architecture-detail/established-decisions.md:175)
  > The strict-modes check on the home and the instance's files runs ahead of them, before the wrapper log opens (chunk 2026-10-10-budget-governor).
  (.andromeda/architecture-detail/established-decisions.md:176)
  > exclusive bind of the endpoint (a bind that finds the name taken is an exit-1 refusal, `squatted-name`, so two concurrent starts cannot both win), first snapshot (with `endpoint`, `pinned_bin`, and `statusline_command` when the named source gave a command; `pid`, `started_at`) and heartbeat, the start `wheel` and `budget-gate` events, then the child spawn, after which the snapshot is rewritten with `child_pid`.
  (.andromeda/architecture-detail/established-decisions.md:177)
  > → spawn, so the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
  (.andromeda/architecture.md:59)
  > Rationale: the prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
- (.andromeda/architecture-detail/established-decisions.md:61) The server's identity must be read on the stream the `channel` seam connected, on Windows the handle the SQOS Identification open adopted (never a default connect), and an endpoint that vanishes or fails the check must still read `instance-unreachable`.
  > On Windows the client connects only through the SQOS Identification open (windows-sys `CreateFileW` with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`, adopted by `Stream::try_from`), never interprocess's default connect, so a squatted or rogue server cannot impersonate the client; a non-overlapped handle adopted this way hangs (as measured at chunk 2026-09-25-security-prerequisites).
  (.andromeda/architecture-detail/established-decisions.md:61)
  > With no `timeout_ms` it blocks until an event arrives, and if the endpoint vanishes first the caller gets `instance-unreachable`.
  (.andromeda/architecture-detail/established-decisions.md:61)
  > Endpoint names are a short hash of the instance, never the raw `VIOLA_NAME` + path.
  (.andromeda/architecture-detail/established-decisions.md:60)
  > **[Message Broker / IPC] interprocess 2.4.4 `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam.** `hook` blocks on the channel for a dialog answer.
  (.andromeda/architecture.md:39)
  > on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
- (.andromeda/architecture-detail/established-decisions.md:8) The verification on the `hook` and CLI paths must stay std and blocking, the MCP tools' check going through the Tokio client in `viola-mcp` alone, so no runtime enters `hook`.
  > `hook` spawns once per hook event and must not pay for runtime startup. rmcp and interprocess-async require Tokio, so it is confined to the subcommand that uses them. CI enforces this with a sole-root `cargo deny` `tokio` ban (`deny-sync.toml`, run once per crate listed in `scripts/sync-crates.txt`), evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3). `viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in the same CI supply-chain step.
  (.andromeda/architecture-detail/established-decisions.md:190)
  > Among the product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature (`["dep:tokio", "interprocess/tokio"]`, the `TokioClient`), which `viola-mcp` enables and, in a package-scoped test build, the channel's own test-only `fake-agent = ["tokio"]` (chunk 2026-10-17-mcp-server-for-drivers); the test-only `viola-e2e` takes `rt` + `process`. No other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`).
  (.andromeda/architecture.md:23)
  > CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes.
- (.andromeda/architecture-detail/established-decisions.md:82) The hook's connect and server check together must finish inside the 750 ms `CONNECT_DEADLINE` below `SPINE_DEADLINE`, a check that fails or overruns recorded in `diagnostics/` with exit 0 and the event lost, and SessionEnd must still reach its direct-append fallback within its budget when the channel is refused.
  > Each writes its `hook.event` and exits 0 within viola's spine deadline: one built-in value for every event, below the `timeout` (5 s) those hooks get in `hooks.json`. That value is the product constant `viola_core::SPINE_DEADLINE` = 1 s, the bound the perf gate reads (test-plan §10). The connect deadline `viola hook` passes to `Client::connect_by` is the hook-local, crate-private `CONNECT_DEADLINE` = 750 ms (provisional), asserted below `SPINE_DEADLINE` by a unit test. If the endpoint cannot be reached or written within the deadline, the hook records the failure in `diagnostics/` and exits 0. The event is lost rather than the turn delayed, and a driver recovers through `wait`'s `timeout_ms`.
  (.andromeda/architecture-detail/established-decisions.md:83)
  > SessionEnd: sync within viola's own ~1 s budget: the channel under the same provisional 750 ms `CONNECT_DEADLINE`, else a direct `try_lock` append (`viola_state::try_append_event` with `Source::Hook`, which writes nothing while `events.ndjson.lock` is held), else nothing; `hooks.json` gives it no `timeout` key.
  (.andromeda/architecture-detail/established-decisions.md:86)
  > The 1.5 s SessionEnd budget is never raised.
  (.andromeda/architecture-detail/established-decisions.md:46)
  > every appender (`append_event`, `append_event_at`, `try_append_event`) reads the log's last byte under the lock it already holds and, when the log is not empty and that byte is not LF, writes one LF and its line in a single write.
  (.andromeda/architecture.md:45)
  > A lost endpoint loses the event, not the turn.
- (.andromeda/architecture-detail/established-decisions.md:92) A mismatch or an unmakeable check in `viola hook` must fail open, exit 0 with no body and nothing on stderr, a dialog hook then leaving the dialog to the human, and the hidden capture arm must open no channel and so take no check.
  > **[Hook Contract] Hooks fail open.** Exit 0 with a JSON body is a decision. Exit 0 with no body is no decision (the `unverified-cli` degrade: the dialog renders for the human). Exit 2 is never produced. `hook` exits 0 at once when `VIOLA_NAME` is absent.
  (.andromeda/architecture-detail/established-decisions.md:92)
  > it reads no `VIOLA_*`, does no obs init and opens no channel;
  (.andromeda/architecture-detail/established-decisions.md:93)
  > `hook` never writes to stderr. Its codes-only process log goes to the home-level `diagnostics/hook-<name>.ndjson`, and content-bearing detail goes only to the instance's `diagnostics/detail-hook.ndjson` (format owned by obs). A failure to write either is ignored. Rationale: exit 2 is a blocking error in Claude Code's contract, and a viola bug must never block the user's session.
  (.andromeda/architecture.md:47)
  > Exit 0 with a JSON body is a decision, with no body no decision; exit 2 is never produced, and `hook` never writes to stderr.
- (.andromeda/architecture-detail/established-decisions.md:67) The edited MCP `send` / `wait` / `last` / `answer` handlers must report a failed check as an `isError` result with no frame, keeping their argument re-validation, their `invalid-params` and `-32603` paths and their `from`, while `list` writes no frame and takes no check.
  > Refusals are `isError: true` tool results mapped from `result.refusal`.
  (.andromeda/architecture-detail/established-decisions.md:74)
  > `target` is a `ViolaName`, and the server adds `from` from its own `VIOLA_NAME` only when it has a name of its own. As landed (chunk 2026-10-17-mcp-server-for-drivers): each handler takes the raw argument object and re-validates it, a schema violation being the `invalid-params` result with a closed field `detail` and no frame (§Conventions, Error handling schema), and an internal handler fault being the JSON-RPC error `-32603` `internal error` with `data: null`;
  (.andromeda/architecture-detail/established-decisions.md:74)
  > `list` builds the CLI's board and makes its one `claude agents --json` read
  (.andromeda/architecture.md:43)
  > Refusals are `isError: true` tool results mapped from `result.refusal`;
- (.andromeda/architecture-detail/established-decisions.md:211) Every CLI verb's failed check must surface as its typed exit code and, under `--json`, a machine-parseable refusal document.
  > Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument.** viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths (brief §6), and agents need machine-parseable refusals.
- (.andromeda/architecture-detail/established-decisions.md:33) `src/cmd/send.rs` is reached whole, so the client's `empty-text` refusal must keep exit 13 and stay ahead of any frame, the new check adding no frame before it.
  > A text whose typed text is empty (the empty text, or a text of only CR and LF characters) is refused `not-delivered` / `empty-text` (exit 13) and is never issued: by the client before any frame (`refusal_of` in `src/cmd/send.rs`, the second product caller of `typed_text`, beside the wrapper's `send`)
- (.andromeda/architecture-detail/established-decisions.md:117) `src/cmd/pause.rs` and `src/cmd/release.rs` are reached whole, so the human's `pause` and `release` must still reach a wrapper that passes the check, the recovery of a stuck turn resting on them.
  > CLI `viola pause` moves the wheel to the human without a keystroke, after which automated `send` is refused `human-typing` with detail `manual-pause`. The wheel returns only through CLI `viola release`;
  (.andromeda/architecture-detail/established-decisions.md:116)
  > so a turn left running under a driver-held wheel is recovered without a keystroke by `viola pause` then `viola release`.
- (.andromeda/architecture-detail/established-decisions.md:125) The chunk must serve no `link` / `unlink` and add no client for them, the hold standing, while the `send` and `answer` clients keep passing `from` in their `params`.
  > They are held: the boundary widening they carry is not ratified (the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check), and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, is that no explicit verb is built before Epoch 6, the three crossings staying unbuilt and unratified.
  (.andromeda/architecture-detail/established-decisions.md:124)
  > The CLI and the MCP server pass that name as `from` in the request `params`, and the driven instance's wrapper appends a `link` event to its `events.ndjson` only when that `driver → driven` pair is not already in its link set.
- (.andromeda/architecture-detail/established-decisions.md:53) The clients' `snapshot.json` read must hold the file open only for the read itself, never across the connect and the identity check, so the wrapper's bounded Windows `persist` retry is not exhausted by a verifying client.
  > On Windows a `persist` refused with raw OS error 5 (a reader holding the target open, as measured at chunk 2026-09-27-hooks-to-normalised-events research.md M1 under every share mode) is retried every 10 ms, re-persisting the same temp file, at most `viola_state::fs::REPLACE_ATTEMPTS` = 100 attempts (an attempt count, never a time bound); any other error returns at once.

## Bindings
- **Session Liveness ↔ standard contracts (the snapshot's `pid` / `started_at`)**: the check compares the connected server with the `pid` and `started_at` the first snapshot records, and the start order puts that snapshot after the endpoint bind and before the spawn, so a client between bind and first snapshot finds no snapshot to verify against (.andromeda/architecture-detail/established-decisions.md:175), (.andromeda/architecture-detail/established-decisions.md:176)
- **Message Broker / IPC ↔ conventions (exit 21, `instance-unreachable`)**: a vanished endpoint and a failed check share one outcome, the CLI's typed exit with its `--json` document and the MCP's `isError` result, codes only (.andromeda/architecture-detail/established-decisions.md:61), (.andromeda/architecture-detail/established-decisions.md:211), (.andromeda/architecture-detail/established-decisions.md:67)
- **Hook Transport ↔ Hook Contract**: the check sits inside the hook's connect deadline, and its failure is the same fail-open exit as an unreachable endpoint, with SessionEnd falling to its direct append (.andromeda/architecture-detail/established-decisions.md:82), (.andromeda/architecture-detail/established-decisions.md:83), (.andromeda/architecture-detail/established-decisions.md:92)
- **Concurrency ↔ Module Boundaries**: the identity read lands in `viola-channel` / `viola-state` without pulling tokio into the channel's normal graph, the Tokio flavour behind the feature only `viola-mcp` enables (.andromeda/architecture-detail/established-decisions.md:8), (.andromeda/architecture-detail/established-decisions.md:190)
- **Session Liveness ↔ security plan (strict-modes in the same process)**: the strict-modes check runs before any log opens, the ordering the clients inherit if P4 takes the strict-modes half here (.andromeda/architecture-detail/established-decisions.md:175)
- **Links ↔ the scope's boundary**: the explicit verbs' hold names the liveness-only pre-check this chunk closes, and it stays held (.andromeda/architecture-detail/established-decisions.md:125)
