proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 124
    section: '## Established Decisions'
    change: '- Each frame tool (`send`, `wait`, `last`, `answer`) runs `viola_state::strict::check_instance` on the home and the target instance directory before `live_endpoint`; a refusal is the `instance-unreachable` result with `detail` `strict-modes-failed`, no frame and no line.'
    sidecar: '[MCP] a frame tool strict-modes refusal now reads `detail` `strict-modes-failed`, no longer `detail: null`.'
    rationale: 'Report **Retired** names the exit-21 document `detail: null`; **Symbols / APIs**, MCP: a refused `check_instance` is `StrictModes`; **Expected amendments** lists established-decisions.md:124.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 124
    section: '## Established Decisions'
    change: '- The `isError` result of an unreachable instance names its cause through `viola_mcp` `result::unreachable(cause: UnreachableCause)`: the CLI document less `v`, its text still `refused: instance-unreachable`. In `frame` a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is the reading of `viola_state::liveness::without_endpoint` ([Session Liveness]) and a failed `server_verified` is `server-verify-failed` (off macOS), no frame and no line on any of the three; `of_reply` reads `Connect` as `instance-dead` and `Closed | Io` as `endpoint-lost`; `of_board` reads the refusal of the board as `strict-modes-failed`. `isError` is the record. JSON-RPC errors did not move, and `-32603` keeps its fixed body (the founder''s own answer, chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1).'
    sidecar: '[MCP] added the cause mapping of the unreachable result: `frame`, `of_reply`, `of_board`.'
    rationale: 'Report **Symbols / APIs**, MCP bullet; **Decisions & corrections** carries Q1 as the founder''s own word; **Expected amendments** names §Established Decisions [MCP].'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 305
    section: '## Established Decisions'
    change: 'Replace the `viola revive` clauses of the [CLI Conventions] entry, to its end, with: `viola revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check, a preflight refusal or a refusal of the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, a `claude` not on `PATH` being the last) in place of its `unable:` / `hint:` pair, exit 1, and `<name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order; without `--json` the pairs and the `--list` rows are byte-identical. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints the `internal-error` document and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal (since chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: '[CLI Conventions] the `revive --json` post-preflight gap closed: four start codes and `stale-heartbeat` joined the list, the owed sentence dropped.'
    rationale: 'Report **Retired** names the `revive --json` post-preflight gap and already-live as a stale holder refusal; **Symbols / APIs** says the "prints no document" sentence is no longer true and lists the ten codes; **Expected amendments** lists established-decisions.md:305. The clause owing the document to "Exit-cause code catalogue" is spent by this chunk (`title`).'
    basis: src/cmd/revive.rs:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 305
    section: '## Established Decisions'
    change: '- The exit-21 document names its cause: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `v` still 1, `<cause>` one of the five of the closed `viola_core::UnreachableCause` (no serde path; the two clients write each word): `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`. `client::unreachable_document(cause)` writes it, `viola list` building its one refusal into its board document. No exit moved, no `RefusalReason` and no exit code was added, and `error` stays `instance-unreachable` (the founder''s own answer, chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1).'
    sidecar: '[CLI Conventions] added the exit-21 document per cause and its writer.'
    rationale: 'Report **Symbols / APIs**, the exit-21 document, `UnreachableCause` and `unreachable_document` bullets; the machine-parseable refusal is the stated rationale of this entry.'
    basis: src/cmd/client.rs:222
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 305
    section: '## Established Decisions'
    change: '- Where the `cli` catch site prints its line (`prints_internal_error`) and `--json` stands among the arguments before a `--` (`json_asked` in `src/main.rs`, read from argv as `role_of` is), `cmd::internal_error_document` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line is byte for byte as it stood, and `run`, the start of `viola revive`, `mcp` and `hook` stay silent there.'
    sidecar: '[CLI Conventions] added the catch-site document under `--json`.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`; `catch-site` in **Counts / qualifiers moved**.'
    basis: src/main.rs:154
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 259
    section: '## Established Decisions'
    change: '- A client with no `LiveEndpoint` reads its cause from `viola_state::liveness::without_endpoint(instance_dir)`: `symlink_metadata` reads a directory `instance-dead` and anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line, and the strict-modes check stays first at every caller. Its callers are `Unreachable::without_endpoint` in `src/cmd/client.rs` and `frame` in `crates/viola-mcp/src/tools.rs` (the overseer as delegate, PROVISIONAL, chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q2).'
    sidecar: '[Session Liveness] added the no-endpoint reading, PROVISIONAL.'
    rationale: 'Report **Symbols / APIs**, `without_endpoint`; **Decisions & corrections** carries Q2 as the delegate word, PROVISIONAL; **Expected amendments** names §Established Decisions [Session Liveness].'
    basis: crates/viola-state/src/liveness.rs:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 260
    section: '## Established Decisions'
    change: '- `collision_check` (`src/cmd/run.rs`) prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat. The `outcome` of the `run.collision_check` span stays `already-live` for a live or a stale holder: its closed set says a holder stands, and it did not move. A frame verb against a stale instance still reads `instance-dead`, since `live_endpoint` gives none for a stale wrapper: a stale heartbeat is the cause of exit 1 alone.'
    sidecar: '[Session Liveness] added the code of a stale holder, `stale-heartbeat`, and what stays `already-live` and `instance-dead`.'
    rationale: 'Report **Symbols / APIs**, a stale holder code; **Retired** names already-live as a stale holder refusal; **Reverted / negative API facts**: the span outcome was not moved. Line 260 stands: it speaks of a fresh replayed beat.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: '## Established Decisions'
    change: 'In the `wait` sentence: With no `timeout_ms` it blocks until an event arrives, and if the endpoint vanishes first the caller gets `instance-unreachable`, its `detail` `endpoint-lost`.'
    sidecar: '[Message Broker / IPC] a `wait` whose endpoint vanishes names `endpoint-lost`.'
    rationale: 'Report **Symbols / APIs**: `answer_of` and `of_reply` read `Closed | Io` as the lost cause; **A process-exit `during`**: a wrapper lost with the call open is `endpoint-lost`; witnesses `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost` and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`.'
note: 'Outside this file, for the owner of the body: architecture.md:229 still names `client::refused_document` as the writer of the `plugin install` exit-21 document, which the report moves to `unreachable_document`, and the report lists only :226 and :228 in the body; D-arch-resources and D-arch-decisions found no drift here (no dependency, crate or registry resource in this category).'
