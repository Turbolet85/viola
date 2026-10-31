proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 124
    section: '## Established Decisions'
    change: 'Replace `a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line` with `a refusal is the `instance-unreachable` result with `detail: "strict-modes-failed"`, no frame and no line`.'
    sidecar: 'dec-mcp: a frame tool''s strict-modes refusal reads `detail: "strict-modes-failed"`, no longer null.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**, MCP: a refused `check_instance` is `StrictModes`, no frame and no line (`inputs#I2` Q1, the founder''s own word).'
    basis: .andromeda/architecture-detail/established-decisions.md:124
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 124
    section: '## Established Decisions'
    change: 'Add after the line: `- The MCP unreachable mapping (chunk 2026-10-31-exit-cause-code-catalogue): `of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board`''s refusal is `StrictModes`, and `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, no `live_endpoint` by `without_endpoint`, and a failed `server_verified` as `ServerVerify`.`'
    sidecar: 'dec-mcp: adds the mapping of each channel and frame failure to its unreachable cause.'
    rationale: 'Report **Symbols / APIs**, MCP bullet: `crates/viola-mcp/src/result.rs` `unreachable(cause)`, `of_reply`, `of_board` and `tools.rs` `frame`.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 249
    section: '## Established Decisions'
    change: 'Add after the line: `- `viola verify` writes `Ended::Refused`''s code on the `process-exit` it logs under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted` (chunk 2026-10-31-exit-cause-code-catalogue). A refused home tree stays lineless, a failing row has no code, the fixture''s `<file>` enters no code, and `verify` takes no `--json`.`'
    sidecar: 'dec-cli-version-compatibility: adds verify''s `process-exit` refusal codes and what stays without one.'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes: `src/cmd/verify.rs` `Ended::Refused`, and **Schema / config**: `schemas/diag-line.v1.json:78` admits the new words.'
    basis: schemas/diag-line.v1.json:78
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 258
    section: '## Established Decisions'
    change: 'Add after the line: `- `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder or a fresh replayed beat (chunk 2026-10-31-exit-cause-code-catalogue). The `run.collision_check` span''s `outcome` stays `already-live` for both, its closed set saying a holder stands. Stale heartbeat is exit 1''s cause alone: a frame verb against a stale instance reads `instance-dead`, `live_endpoint` giving none for a stale wrapper.`'
    sidecar: 'dec-session-liveness: a stale holder refuses with its own `stale-heartbeat` code; the span outcome is unmoved.'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code; **Retired** names `already-live` as a stale holder''s refusal; **Reverted / negative API facts**: the span''s `outcome` was not moved.'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 259
    section: '## Established Decisions'
    change: 'Add after the line: `- `viola_state::liveness::without_endpoint` reads the instance directory by `symlink_metadata` where a client has no live endpoint: a directory is `InstanceDead`, anything else (absent, a regular file, a symlink) is `UnwrappedName`. It opens nothing, writes no line and stays behind the strict-modes check at every caller (`Unreachable::without_endpoint`, `viola mcp`''s `frame`). The reading is the delegate''s word, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I2` Q2).`'
    sidecar: 'dec-session-liveness: adds `without_endpoint`, which splits a missing endpoint into `instance-dead` and `unwrapped-name`.'
    rationale: 'Report **Symbols / APIs**: `viola_state::liveness::without_endpoint` and its two callers; **Decisions & corrections** carries Q2 as the delegate''s word, PROVISIONAL.'
    basis: crates/viola-state/src/liveness.rs:107-115
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 262
    section: '## Established Decisions'
    change: 'Replace `a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, no log opened, no line and no child` with `a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, or under `viola revive --json` the `strict-modes-failed` document in place of either pair, no log opened, no line and no child`.'
    sidecar: 'dec-session-liveness: a refused home-tree check of a revived start under `--json` prints its document in place of the pair.'
    rationale: 'Report **Symbols / APIs**, `viola revive --json`: one exit-1 document for every refusal of its start, `strict-modes-failed` among its codes; the table''s `strict_modes_on_a_start_is_exit_1_and_opens_no_log` witnesses it.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 305
    section: '## Established Decisions'
    change: 'Replace `a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) in place` with `every refusal of its start, a refused strict-modes check and a preflight refusal among them, prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, a `claude` not on `PATH` being `internal-error`) in place`; replace the last sentence, from `A refusal `run`''s start makes` to `"Exit-cause code catalogue".`, with `A `pump_child` error prints no document: the child has held the terminal (chunk 2026-10-31-exit-cause-code-catalogue).`'
    sidecar: 'dec-cli-conventions: `viola revive --json` holds ten codes; the post-preflight gap and its owed pointer are spent.'
    rationale: 'Report **Retired** names the `revive --json` post-preflight gap; **Symbols / APIs** lists the ten codes, says the no-document sentence is no longer true, and gives the `pump_child` exception.'
    basis: .andromeda/architecture-detail/established-decisions.md:305
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: '## Conventions'
    change: 'Add after the line: `- `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, with `as_str`) has no serde path: every word is written by the two clients, and it is no `RefusalReason`.`'
    sidecar: 'conv-refusal-reason: adds `UnreachableCause`, the exit-21 cause set that is no refusal.'
    rationale: 'Report **Symbols / APIs**: `viola_core::UnreachableCause`, no serde path; no `RefusalReason` was added.'
    basis: crates/viola-core/src/lib.rs:181-190
    fact: F2
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 35
    section: '## Conventions'
    change: 'Replace `([Session Liveness]; `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`, or `unable: <name> is still running but not answering` with the stale hint)` with `([Session Liveness]; a live holder or a fresh replayed beat `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`; a stale holder `detail:"stale-heartbeat"`, `unable: <name> is still running but not answering` with the stale hint)`.'
    sidecar: 'conv-exit-codes: exit 1''s collision refusal reads `stale-heartbeat` for a stale holder, its human pair unchanged.'
    rationale: 'Report **Retired** names `already-live` as a stale holder''s refusal; **Counts / qualifiers moved**, `stale-live`, lists `conventions.md:35` as changed.'
    basis: .andromeda/architecture-detail/conventions.md:35
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 36
    section: '## Conventions'
    change: 'Replace `` `detail:"already-live"` (`run`''s own collision check and its two pairs, unchanged) `` with `` `detail:"already-live"` or `detail:"stale-heartbeat"` (`run`''s own collision check and its two pairs, unchanged) ``.'
    sidecar: 'conv-exit-codes: revive''s first preflight reading refuses with either collision code.'
    rationale: 'Report **Symbols / APIs**: `collision_check` prints and logs `stale-heartbeat` for a stale holder, and `viola revive --json` lists it among its codes; the human pairs are byte for byte as they stood.'
    basis: .andromeda/architecture-detail/conventions.md:36
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 36
    section: '## Conventions'
    change: 'Replace `each of these revive refusals, the strict-modes one included, is instead one stdout document` with `each of these revive refusals, the strict-modes one included, and every refusal of the start it shares with `run` is instead one stdout document`; extend the code list to `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; replace `, while a refusal `run`''s start makes after a passed preflight prints no document (owed to "Exit-cause code catalogue"; chunk 2026-10-17-cli-machine-contract)` with `, and a `pump_child` error prints none (chunk 2026-10-17-cli-machine-contract)`.'
    sidecar: 'conv-exit-codes: revive''s `--json` document covers every refusal of its start; the owed pointer is spent.'
    rationale: 'Report **Retired** names the `revive --json` post-preflight gap; **Symbols / APIs** lists the ten codes and the `pump_child` exception. The edit is sized to keep the line under the 2 000-char entity limit.'
    basis: .andromeda/architecture-detail/conventions.md:36
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 38
    section: '## Conventions'
    change: 'After `prints exactly `error: internal error` on stderr once, from the `main` catch site, no hint — no dispatch arm prints it` add `; under `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout in that line''s place, stderr then empty`.'
    sidecar: 'conv-exit-codes: the `cli` catch site prints the `internal-error` document under `--json`.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`; **Counts / qualifiers moved**, `catch-site`, lists `conventions.md:38`. Without `--json` the line stands byte for byte.'
    basis: src/cmd/mod.rs:99-102
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 46
    section: '## Conventions'
    change: 'Replace `(`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)` with `(`--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, no log opened)`; the later `same exit and document` clauses then read that form.'
    sidecar: 'conv-exit-codes: the board, `plugin install` and `link` / `unlink` strict-modes refusals read `strict-modes-failed`.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**: `src/cmd/list.rs`''s `refused_document` builds `strict-modes-failed`, `src/cmd/plugin.rs` writes `StrictModes` through `unreachable_document`, and `refuse_untrusted` too.'
    basis: .andromeda/architecture-detail/conventions.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 46
    section: '## Conventions'
    change: 'Add after the line: `- `src/main.rs` `json_asked` reads `--json` from argv before a `--`. Where `prints_internal_error` is true it prints `internal_error_document` on stdout in place of the human line, stderr empty; `run`, `revive`''s start, `mcp` and `hook` stay silent at that site.`'
    sidecar: 'conv-exit-codes: adds how the catch site reads `--json` and which roles stay silent there.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`: `json_asked` and `internal_error_document`.'
    basis: src/main.rs:154-161
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 46
    section: '## Conventions'
    change: 'Add after the line: `- An absence: the socket-directory refusal has no code. `bind_endpoint`''s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening", the delegate''s word, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I3`).`'
    sidecar: 'conv-exit-codes: records that the socket-directory cause has no code and who owns it.'
    rationale: 'Report **Spec claims disproved by measurement**, `v1-18`, and the Beyond-the-plan row under `inputs#I3`.'
    fact: F18
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: '## Conventions'
    change: 'Replace `the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`` with `the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json``; replace `and the cause has no exit code, `--json` code or `RefusalReason` of its own` with `and the cause has no exit code or `RefusalReason` of its own, its `--json` word the document''s `detail``.'
    sidecar: 'conv-exit-codes: the failed server check reads `server-verify-failed` in the exit-21 document.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs** lists `server-verify-failed` among the five causes, with no exit, `RefusalReason` or exit code added.'
    basis: .andromeda/architecture-detail/conventions.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: '## Conventions'
    change: 'Add after the line: `- The human hints: `src/human/hints.rs` holds an unwrapped-name hint and a new endpoint-lost hint, last on stderr and plain ASCII. The hint cause keys are `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`, `not-running` kept unrenamed. The endpoint-lost wording is the delegate''s word, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I2` Q3).`'
    sidecar: 'conv-exit-codes: adds the unwrapped-name and endpoint-lost hints and the four hint cause keys.'
    rationale: 'Report **Symbols / APIs**: `hint_cause()`; **Reverted / negative API facts**: `not-running` stays the hint cause''s key; **Coverage of new surfaces**: the two hints.'
    basis: src/human/hints.rs:50-55
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: '## Conventions'
    change: 'Add after the line: `- An exception: `server-verify-failed` does not arise on macOS, which reports no peer pid. `v1-18`''s carve-out leaves macOS''s server check with the route entry "Unix endpoint and home hardening" (chunk 2026-10-31-exit-cause-code-catalogue, its `inputs#I2` Q4, the founder''s own word).`'
    sidecar: 'conv-exit-codes: records that `server-verify-failed` does not arise on macOS, and the carve-out''s owner.'
    rationale: 'Report **Outcome**: `v1-18` met with named carve-outs, macOS''s server check among them; MCP''s criterion reads `server-verify-failed` off macOS.'
    fact: F19
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 70
    section: '## Conventions'
    change: 'Replace `exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty` with `exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty`.'
    sidecar: 'conv-exit-codes: a frame verb''s refused strict-modes check reads `strict-modes-failed`.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**: `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/architecture-detail/conventions.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 70
    section: '## Conventions'
    change: 'Add after the line: `- The CLI unreachable mapping (`src/cmd/client.rs`): `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`, and the six no-endpoint arms (`send`, `wait`, `last`, `answer`, `pause`, `link`, with `release` and `unlink` reaching them through `wheel_call` and `link_call`) read `Unreachable::without_endpoint`.`'
    sidecar: 'conv-exit-codes: adds how the CLI maps a channel failure and a missing endpoint to its cause.'
    rationale: 'Report **Symbols / APIs**: `src/cmd/client.rs` `answer_of`, and the six no-endpoint arms.'
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 76
    section: '## Conventions'
    change: 'Replace `(`instance-unreachable` with `detail: null`;` with `(`instance-unreachable` with `detail` its cause word, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`;`.'
    sidecar: 'conv-mcp-results: an unreachable result''s `detail` is its cause word, no longer null.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**: MCP''s `isError` result is the document less `v`, the five closed causes (`inputs#I2` Q1, the founder''s own word).'
    basis: .andromeda/architecture-detail/conventions.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 83
    section: '## Conventions'
    change: 'Add after the line: `- MCP''s unreachable `isError` body is the CLI document less `v`, and its text stays `refused: instance-unreachable`. No cause writes a frame or a line, `isError` being the record; JSON-RPC errors are unmoved and `-32603` keeps its fixed body (chunk 2026-10-31-exit-cause-code-catalogue).`'
    sidecar: 'conv-mcp-results: adds that every cause keeps the same text, no frame, no line and unmoved JSON-RPC errors.'
    rationale: 'Report **Symbols / APIs**: the exit-21 document moved, and the MCP bullet: no frame and no line, JSON-RPC errors unmoved, `-32603` its fixed body.'
    fact: F5
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 89
    section: '## Conventions'
    change: 'Replace `returns the `instance-unreachable` `isError` result with `detail: null`,` with `returns the `instance-unreachable` `isError` result with `detail: "strict-modes-failed"`,`.'
    sidecar: 'conv-mcp-results: a frame tool''s strict-modes refusal reads `strict-modes-failed`, no longer null.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**, MCP: a refused `check_instance` is `StrictModes`.'
    basis: .andromeda/architecture-detail/conventions.md:89
note: 'The body''s broad lines of Stack, Established Decisions and Conventions still hold; the exit-21 and exit-1 lines at .andromeda/architecture.md:226, :228 and :229 are Standard Contracts, not this group''s, and no new IPC, env var, crate or dependency fires D-arch-resources or D-arch-decisions here.'
