# arch · Standard Contracts — category read

## Carried
- (.andromeda/architecture.md:156) The verification must compare the connected server against the snapshot's `pid` and `started_at` in this recorded form (sysinfo OS start time, whole seconds, RFC 3339), and must take the endpoint only from the snapshot the wrapper wrote.
  > `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`. `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
- (.andromeda/architecture.md:156) The snapshot stays wrapper-written: a client's verification reads it and must never write or rewrite it.
  > (`instances/<ViolaName>/snapshot.json`, written only by that instance's wrapper, rewritten on every change to a field below)
- (.andromeda/architecture.md:157) The pair the check matches must be the snapshot's wrapper identity, `pid` + `started_at`, never `child_pid`.
  > - `pid` and `started_at` identify the wrapper process for the liveness check.
- (.andromeda/architecture.md:243) The server match must keep the liveness rule's sense: a pid alive with another start time is not the recorded wrapper (this bears on CARRY 2's exiting-wrapper window).
  > When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age. Otherwise a beat at most 5 s old is `live`, and an older or absent beat is `stale`.
- (.andromeda/architecture.md:59) The check must never rest on a bare pid; the start time is part of the identity.
  > Rationale: the prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
- (.andromeda/architecture-detail/standard-contracts.md:24) A replayed snapshot carries no `endpoint`, `pid` or `started_at`, so no verification can be made against one; the client outcome on an unreadable or newer snapshot stays as today under CARRY 7's hold.
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`. Replay recovers only log-derived fields: `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`. Fields no event carries (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) are treated as absent. Liveness then rests on the heartbeat alone, so the instance is `live` or gone and never `stale`.
- (.andromeda/architecture-detail/standard-contracts.md:24) A product reader of the replay fallback must sit after `check_instance` has passed, as the two landed readers do.
  > As landed the fallback is library code in `viola-state` with two product readers, each after `viola_state::strict::check_instance` has passed:
- (.andromeda/architecture-detail/standard-contracts.md:24) A client moving to the classified read must keep its four cases apart through the `MAX_FRAME` bound.
  > `snapshot::read_snapshot_classified` tells four cases apart through the same `MAX_FRAME` bound: the snapshot, no file, an unreadable one, and an unsupported one with the `v` it saw.
- (.andromeda/architecture-detail/standard-contracts.md:25) The five `read_snapshot` callers named here (CARRY 8) are the readers this chunk re-reads; any that moves changes this count, which keeps its rule, and the move is bound to CARRY 7's hold.
  > `read_snapshot` keeps its results, and its five product callers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still read every such snapshot as absent.
- (.andromeda/architecture-detail/standard-contracts.md:25) If a client takes the replay, it must be this one-pass, no-write fallback with its one `state-recovered` line; an absent snapshot is neither replayed nor logged.
  > `replay::read_snapshot_or_replay` replays the log for an unreadable or unsupported snapshot, in one pass, and logs one `state-recovered` line; an absent snapshot is neither replayed nor logged, and the replay writes no file.
- (.andromeda/architecture-detail/standard-contracts.md:59) `hook.event` is an id-less notification with no reply, so its verification must be complete before the frame is written; the hook can learn nothing back from the server.
  > - `hook.event`, sent id-less by `hook` (a notification, no reply): `{ts, event:{kind, data}}`.
- (.andromeda/architecture.md:41) Server verification must add no handshake frame to the channel; the identity must come from the endpoint, not from a new method or field.
  > Every `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake.
- (.andromeda/architecture.md:176) `conn` must stay outside identity: the check must not read a client- or server-supplied frame field as proof.
  > A client may add `conn` (obs-plan D-10) to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
- (.andromeda/architecture.md:39) On Windows the verifying client must keep connecting only through the SQOS Identification open.
  > on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
- (.andromeda/architecture.md:263) On Unix the client must keep connecting to the snapshot's recorded `endpoint`, never a recomputed or abstract name.
  > every other process connects to the `endpoint` recorded in the instance snapshot.
- (.andromeda/architecture.md:6) A wrapper started by `viola revive` must be verifiable by the same check, since it records its identity through the one start path.
  > Every wrapper owns its own local-socket endpoint, whether it was started by `viola run` or by a `viola revive` whose preflight passed (both go through the one start path).
- (.andromeda/architecture.md:125) `viola revive` writes no channel frame, so it is no client this check is added to.
  > `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments) send no channel frame.
- (.andromeda/architecture-detail/standard-contracts.md:65) `list` (CLI and MCP) contacts no wrapper, so it writes no frame and takes no server verification.
  > `list` is not a channel method. The calling process reads `instances/*/` (heartbeat and snapshot) and `claude agents --json` itself, so `list` needs no live wrapper.
- (.andromeda/architecture.md:224) A CLI verb's mismatch or unverifiable server has no channel `result`, so it must print the `error` document, not a refusal.
  > Outcomes that have no channel `result` use `error`:
- (.andromeda/architecture.md:225) The CLI mismatch outcome must be exactly the `instance-unreachable` document with exit 21.
  > `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
- (.andromeda/architecture.md:100) Exit 21 is the CLI code for an unreachable instance; a strict-modes refusal added to the frame verbs would extend this row's text.
  > `21` instance unreachable, and a refused strict-modes check of `viola list` or `viola plugin install`.
- (.andromeda/architecture.md:104) The MCP frame tools' mismatch must be an `isError: true` result shaped like CLI `--json`, never a JSON-RPC error.
  > a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`; JSON-RPC errors are only the MCP connection's own faults
- (.andromeda/architecture.md:102) The hook client's mismatch must exit 0, as every hook failure does.
  > - **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
- (.andromeda/architecture.md:238) A dialog hook that refuses the server must print nothing, the no-decision form.
  > - No decision is exit 0 with an empty stdout. Async-tier hooks always print nothing.
- (.andromeda/architecture.md:197) A `hook.dialog` never sent for a mismatch must read like a `null` answer: the dialog renders for the human.
  > `null` means no decision, so the hook prints nothing.
- (.andromeda/architecture.md:45) A `hook.event` withheld on a mismatch loses the event only, never the turn.
  > A lost endpoint loses the event, not the turn.
- (.andromeda/architecture.md:405) Any failure of the hook's verification must fail open toward the human.
  > Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human.
- (.andromeda/architecture.md:410) The check on the `hook`, `send` and other sync CLI paths must be std/blocking; only the `mcp` path may run it under Tokio.
  > New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.
- (.andromeda/architecture.md:291) `viola-channel` holds both a sync client and the feature-gated Tokio client; the check must reach both, and the crate's normal graph must stay tokio-free.
  > `viola-channel` left that list at chunk 2026-10-17-mcp-server-for-drivers: it declares `tokio = ["dep:tokio", "interprocess/tokio"]` (its Tokio client, enabled by `viola-mcp`) and `fake-agent = ["tokio"]`, so its package-scoped mutation build compiles and tests the Tokio client; its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh`.
- (.andromeda/architecture.md:106) A new verification error must fold into the owning crate's one error enum (e.g. `ChannelError`), not a new enum.
  > **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`)
- (.andromeda/architecture.md:325) If the chunk adds a strict-modes check to a frame client, it must run before any log opens, the order the three landed checkers hold.
  > `viola list`, `viola revive` and `hook statusline` check these files before they use them, ahead of any log, because opening a log narrows an existing home.
- (.andromeda/architecture-detail/standard-contracts.md:79) The landed precedent for a hook-side snapshot reader: the instance check comes before the log, and the snapshot is read only after it.
  > The arm's order, as landed (chunk 2026-10-10-statusline-pass-through): it canonicalises the instance directory, runs the instance check (`viola_state::strict::check_instance`), opens its log, reads stdin through `take(MAX_FRAME + 1)`, writes `budget.json` when the payload holds a `rate_limits` object, reads the snapshot, then runs the command. The check comes before the log because opening the log sets an existing home to 0700, and a check made after that passes a home at 0770 (as measured at that chunk's `evidence/red-green.md` §3).
- (.andromeda/architecture.md:89) Verification must read existing snapshot fields; a field added for it is additive with no `v` bump, and changing a field's meaning bumps `v`.
  > Adding a field or event kind is additive and needs no bump. Removing or changing a field's meaning bumps `v`.
- (.andromeda/architecture.md:420) The per-connection proof is the security specialist's to define; this plan binds only its placement and outcomes.
  > Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.

## Bindings
- **Snapshot identity fields ↔ security-plan IPC client-side server verification / security.md's "verify the server: pid (+ sysinfo start time) against a `snapshot.json` that passed strict-modes"**: the security clause names the check; this plan fixes the values it matches (`pid`, `started_at` in whole-second RFC 3339) and their writer, (.andromeda/architecture.md:156), (.andromeda/architecture.md:157)
- **Replay fallback lacks `pid` / `started_at` ↔ CARRY 7's held founder fork and security.md's strict-modes half ("Home and code-bearing file integrity")**: a replayed snapshot cannot be verified against, and a replay reader sits after `check_instance`; whether a client takes it is the founder's, (.andromeda/architecture-detail/standard-contracts.md:24), (.andromeda/architecture-detail/standard-contracts.md:25)
- **Unreachable outcomes ↔ Conventions (exit codes, MCP results) and security.md's hook fail-open invariant**: CLI exit 21 `instance-unreachable` document, MCP `isError` shaped like `--json`, hook exit 0 with no body, (.andromeda/architecture.md:225), (.andromeda/architecture.md:104), (.andromeda/architecture.md:102), (.andromeda/architecture.md:405)
- **No handshake / `conn` never identity ↔ obs-plan D-10 (`conn`) and the channel frame contract**: the proof comes from the endpoint, not a frame field, (.andromeda/architecture.md:41), (.andromeda/architecture.md:176)
- **Sync and Tokio clients ↔ Established Decisions KEYSTONE Tokio containment and `scripts/channel-tokio-free.sh` / `deny-sync.toml` CI gates**: (.andromeda/architecture.md:410), (.andromeda/architecture.md:291)
- **Liveness rule ↔ CARRY 2 (Windows pipe outliving the observed exit) and test-plan §3 `run` step 2**: the pid + start-time match is what a hook in the exiting-wrapper window meets, (.andromeda/architecture.md:243)
- **Check-before-log order ↔ security.md §Input validation (statusline / revive check ordering)**: (.andromeda/architecture.md:325), (.andromeda/architecture-detail/standard-contracts.md:79)
