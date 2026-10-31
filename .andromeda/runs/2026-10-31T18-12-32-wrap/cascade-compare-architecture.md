# Cascade compare — architecture · wrap · 2026-10-31-exit-cause-code-catalogue
E items 43 · held 17 · lost at classify 2 · lost at strong 1 · beside a placed fact 4 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### conventions#5.1 · lost at classify · beside a placed fact F2 (near 15, conventions#1.1) · .andromeda/architecture-detail/conventions.md:52
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 52
    section: '## Conventions'
    change: 'New item following line 52: - `viola revive --json` prints one exit-1 document `{"v":1,"error":"<code>","detail":null}` for every refusal of the start it shares with `run`, stderr empty: `refused_with` prints `client::refused_document` in place of the pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and for a program not found `internal-error` (no human pair), all before `spawn_child`; `run` itself passes `json` false and its human pairs are unmoved. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints the `internal-error` document and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal. The codes of `viola revive --json`: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Conventions: `viola revive --json` start refusals each print a document; the ten codes listed.'
    rationale: 'Report **Symbols / APIs**, `run`''s start under `--json` and `viola revive --json`. Its own item, because the exit-code block :33-46 stands at 7545 B of the 8 KB entity bound and line 36 at 1857 B of the 2 000 line bound.'
    basis: src/cmd/revive.rs:119-134

### conventions#6.1 · lost at classify · .andromeda/architecture-detail/conventions.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 58
    section: '## Conventions'
    change: 'New item following line 58: - `viola verify`''s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME` (`Ended::Refused`): `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless, a failing row is no refusal and carries no code, the fixture''s `<file>` enters no code, and `verify` takes no `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Conventions: `viola verify`''s refusal codes on its `process-exit` line.'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes; **Schema / config**: the `process-exit` `detail` enum admits them; **Counts**, `verify-codes`.'
    basis: schemas/diag-line.v1.json:78

### established-decisions#29.3 · lost at classify · beside a placed fact F13 (near 249, established-decisions#28.1) · .andromeda/architecture-detail/established-decisions.md:260
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 260
    section: '## Established Decisions'
    change: '- `collision_check` (`src/cmd/run.rs`) prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat. The `outcome` of the `run.collision_check` span stays `already-live` for a live or a stale holder: its closed set says a holder stands, and it did not move. A frame verb against a stale instance still reads `instance-dead`, since `live_endpoint` gives none for a stale wrapper: a stale heartbeat is the cause of exit 1 alone.'
    sidecar: '[Session Liveness] added the code of a stale holder, `stale-heartbeat`, and what stays `already-live` and `instance-dead`.'
    rationale: 'Report **Symbols / APIs**, a stale holder code; **Retired** names already-live as a stale holder refusal; **Reverted / negative API facts**: the span outcome was not moved. Line 260 stands: it speaks of a fresh replayed beat.'

### established-decisions#9.1 · lost at classify · beside a placed fact F9 (near 258, established-decisions#29.1) · .andromeda/architecture-detail/established-decisions.md:75
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: '## Established Decisions'
    change: 'In the `wait` sentence: With no `timeout_ms` it blocks until an event arrives, and if the endpoint vanishes first the caller gets `instance-unreachable`, its `detail` `endpoint-lost`.'
    sidecar: '[Message Broker / IPC] a `wait` whose endpoint vanishes names `endpoint-lost`.'
    rationale: 'Report **Symbols / APIs**: `answer_of` and `of_reply` read `Closed | Io` as the lost cause; **A process-exit `during`**: a wrapper lost with the call open is `endpoint-lost`; witnesses `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost` and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`.'

### occupied-resources#19.1 · lost at classify · beside a placed fact F16 (near 217, occupied-resources#33.1) · .andromeda/architecture-detail/occupied-resources.md:132
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 132
    section: '## Occupied Resources'
    change: 'New record in its own `<!-- detail: values · res-filesystem -->` group: - `verify`''s refusal codes: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless; a failing row stays no refusal and no code; the fixture''s `<file>` enters no code; `verify` takes no `--json`.'
    sidecar: 'Occupied Resources, Filesystem: `verify`''s `process-exit` refusal codes registered.'
    rationale: 'Report Symbols / APIs, "`verify`''s codes", and Schema / config; lines 122-124 name only `interrupted` for that exit.'
    basis: schemas/diag-line.v1.json:78

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: The exit-21 document names its cause since chunk 2026-10-31-exit-cause-code-catalogue (the founder's own word, that chunk's `inputs#I2` Q1), reading `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with `v` still 1 and `error` still `instance-unreachable`, no exit moved and no `RefusalReason` or exit code added. The five closed causes are `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the words of `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, through `as_str`, no serde path). `client::unreachable_document(cause)` is the one writer of the document, with `refuse_untrusted` and `viola plugin install` writing `StrictModes` through it; `viola list`'s `refused_document` (a `Value` for `board_document`) builds `strict-modes-failed`, the board's only refusal; `client::refused_document` stays exit 1's.
    sidecar: Adds the exit-21 document's cause form, its five closed causes, UnreachableCause and the one writer unreachable_document.
    rationale: Report Symbols / APIs, the exit-21 document moved, `viola_core::UnreachableCause` and `src/cmd/client.rs`; Expected amendments carries architecture §Standard Contracts CLI `--json` output.
    basis: crates/viola-core/src/lib.rs:181

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: Which cause a CLI frame verb reads is `client::Unreachable`'s, now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified` (`cause()`, `detail()`). In `answer_of` a `Connect` failure is `Dead` (`instance-dead`) and `Closed | Io` is `Lost` (`endpoint-lost`, a wrapper lost with the call open); a failed server check is `ServerUnverified` (`server-verify-failed`). The six no-endpoint arms of `send`, `wait`, `last`, `answer`, `pause` and `link` read `Unreachable::without_endpoint(&instance_dir)`, with `release` and `unlink` reaching them through `wheel_call` and `link_call`; the unwrapped reading by the instance directory is the delegate's word, PROVISIONAL (that chunk's `inputs#I2` Q2).
    sidecar: Adds how a CLI frame verb maps a failed connect, a lost call, a failed server check and a missing endpoint to its cause.
    rationale: Report Symbols / APIs, `src/cmd/client.rs` and the six no-endpoint arms; Decisions carries Q2 as the delegate's word, PROVISIONAL.
    basis: src/cmd/client.rs:114

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: MCP's `isError` result for an unreachable instance is the same body less `v`, its text unchanged and the cause in `detail`, built by `viola_mcp`'s `result::unreachable(cause)`. In `of_reply` a `Connect` failure is `InstanceDead` and `Closed | Io` is `EndpointLost`; in `of_board` the board's refusal is `StrictModes`; in the frame tools a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading and a failed `server_verified` is `ServerVerify`. None of them writes a frame or a line, `isError` the record; JSON-RPC errors did not move and `-32603` keeps its fixed body.
    sidecar: Adds the MCP unreachable result's cause in detail and the path each of its causes comes from.
    rationale: Report Symbols / APIs, the exit-21 document moved and the MCP bullet; the founder's own word `inputs#I2` Q1.

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: Since chunk 2026-10-31-exit-cause-code-catalogue `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, its codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. `Launch` gains `json` (`run` passes false, `revive` passes `args.json`), and `refused_with(json, detail, human)` prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and, for a program not found, `internal-error`, all before `spawn_child`; the human pairs stand byte for byte. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line. A `pump_child` error prints no document, because the child has held the terminal and viola writes nothing past it.
    sidecar: Adds revive --json's start documents, closing the post-preflight gap, with refused_with and the pump_child exception.
    rationale: Report Symbols / APIs, `run`'s start under `--json` and `viola revive --json`; Retired names the post-preflight gap.
    basis: src/cmd/revive.rs:119

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: A `stale` holder is refused by `collision_check` (`src/cmd/run.rs`) with the code `stale-heartbeat`, printed and logged, while a live holder and a fresh replayed beat stay `already-live`. Stale heartbeat is exit 1's cause alone, because a frame verb against a stale instance reads `instance-dead`.
    sidecar: Adds stale-heartbeat as a stale holder's own code, separate from already-live.
    rationale: Report Symbols / APIs, a stale holder's code; Retired names `already-live` as a stale holder's refusal.
    basis: tests/cli_instance_state.rs:193

### standard-contracts#16.3 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: At the `cli` catch site, where `prints_internal_error` is true and `src/main.rs` `json_asked(args)` finds `--json` among the arguments before a `--` (read from argv as `role_of` is), `cmd::internal_error_document()` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of the human internal-error line, stderr then empty. Without `--json` the line stands byte for byte; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
    sidecar: Adds the catch site's internal-error document under --json, with json_asked and internal_error_document.
    rationale: Report Symbols / APIs, the `cli` catch site under `--json`; Counts catch-site has a site stating the output under `--json` take the document.
    basis: src/main.rs:154

### standard-contracts#18.1 · lost at strong · .andromeda/architecture-detail/standard-contracts.md:135
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 135
    section: "## Standard Contracts"
    change: Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`, a directory reading `InstanceDead` and anything else (absent, a regular file, a symlink) `UnwrappedName`; it opens nothing and writes no line, and the strict-modes check stays first at every caller. Its callers are `client::Unreachable::without_endpoint` and the `frame` of `viola-mcp`'s tools. `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.
    sidecar: Adds without_endpoint, the directory reading behind instance-dead and unwrapped-name, to the session liveness contract.
    rationale: Report Symbols / APIs, `viola_state::liveness::without_endpoint` and a stale holder's code; Decisions carries the unwrapped reading as `inputs#I2` Q2, PROVISIONAL.
    basis: crates/viola-state/src/liveness.rs:107

## Held

- conventions#10.1
- conventions#11.1
- conventions#4.1
- conventions#7.1
- conventions#8.1
- conventions#9.1
- cross-cutting-patterns#4.1
- established-decisions#16.1
- established-decisions#29.2
- established-decisions#35.1
- occupied-resources#18.1
- occupied-resources#22.1
- occupied-resources#25.1
- occupied-resources#4.1
- occupied-resources#6.1
- standard-contracts#15.1
- standard-contracts#16.1

## Cascade only

- conventions#1.1
- established-decisions#28.1
- established-decisions#29.1
- established-decisions#29.5
- standard-contracts#13.1
