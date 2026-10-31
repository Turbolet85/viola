# security · Error Handling — category read

## Carried
- (.andromeda/security-plan-detail/error-handling.md:8) `viola hook` must stay outside the enumeration as a surface with no error body: the sanitising work must give it no stderr, no body and no non-zero exit.
  > `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human.
- (.andromeda/security-plan-detail/error-handling.md:12) The MCP surface's enumeration and witness must hold this exact unreachable result for a failed server check, with no added error word and no log line.
  > A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; it adds no error word and writes no log line.
- (.andromeda/security-plan-detail/error-handling.md:13) A refused strict-modes check or a replayed or absent snapshot on an MCP frame tool must return the same fixed unreachable result, so the MCP enumeration names one body for all these causes.
  > A `viola mcp` frame tool runs the instance's strict-modes check (`check_instance`) before its liveness pre-check; a refused check, like a snapshot that is replayed or absent (no live endpoint), sends no frame and returns the same unreachable result: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`, and no log line.
- (.andromeda/security-plan-detail/error-handling.md:14) The channel surface must answer a failed `link` / `unlink` record or remove with the fixed `-32603` body, and the CLI verbs must keep these exits for a wrapper error reply or an off-shape reply.
  > The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.
- (.andromeda/security-plan-detail/error-handling.md:15) The CLI `--json` enumeration must hold this document byte for byte for a strict-modes refusal on a frame verb, with its exit, and the refusal must open nothing and write no line.
  > a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.
- (.andromeda/security-plan-detail/error-handling.md:16) The code `revive --json` prints for a strict-modes refusal must stay this closed word, read by the enumeration and not reshaped here (the revive start-refusal documents stand with "Exit-cause code catalogue").
  > a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`).
- (.andromeda/security-plan-detail/error-handling.md:17) The `thiserror` audit must hold `StampsError` and `PinError::StrictModes` to these fixed messages, with no path or payload, and must keep the `strict-modes-failed` outcome mapping.
  > `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.
- (.andromeda/security-plan-detail/error-handling.md:18) The owner-readable detail destination that full error detail goes to must fail closed in this way, for the owner-readable detail leg of the candidate claim `v1-47`.
  > A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open, checked before anything is narrowed: the role writer ends at `std::io::sink` and the detail line is dropped.
- (.andromeda/security-plan.md:29) No error body on any surface may carry the snapshot's recorded `cwd`; a witness must assert its absence.
  > The snapshot's recorded `cwd`, an absolute host path that can carry the OS username, stands in the 0600 snapshot only, on no log line, error body or stdout.
- (.andromeda/security-plan.md:116) The channel enumeration must keep `release-from-driver` as the `-32602` cause beside the other `-32602` causes, which the `ProtocolError::InvalidParams` freight must tell apart.
  > the wrapper refuses a `release` whose params carry a string `from` (`-32602`, `release-from-driver`), before any budget is acted on
- (.andromeda/security-plan.md:140) An MCP input failure must carry `invalid-params` with a closed field word and no frame, so the field word set is enumerable.
  > each handler checks the raw arguments itself (`ViolaName`, `u64`, the closed response enums, `validate_paste_text` on text and free-text answers), a failure being `invalid-params` with a closed field word and no frame.
- (.andromeda/security-plan.md:142) The PREREQ witness of an over-long `viola mcp` stdin line before `initialize` must hold this clause at that edge: refused whole, session ended, one `parse-rejected` line with no byte of it.
  > MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines) — each line capped at `MAX_FRAME`, its `\n` counted, by the reader viola hands rmcp, which reads stdin lines unbounded; a longer line releases no byte, is refused whole, never truncated or skipped, and ends the session as a stdin close does, recorded as one `parse-rejected` line holding no byte of it.
- (.andromeda/security-plan.md:144) Every `revive --json` body the CLI enumeration names must hold codes, closed words and session-shaped ids only.
  > `revive --json` prints codes, closed words and session-shaped ids only, never a path or pid.
- (.andromeda/security-plan.md:178) The cap-driving cases owed to the five `BOUNDARIES` rows must drive these reads past this one value, the status-line command's output, the child output and `config.json` among them.
  > `MAX_FRAME` = 16 MiB, one const in `viola-core`, caps every external read (channel frames, hook stdin, the statusline source and command output, `answer` input, `viola mcp`'s stdin lines, sent text, child output, ndjson lines, `config.json` and the stamps read; viola's reads of its own executable are no external input)
- (.andromeda/security-plan.md:205) Drift reports must reach only `diagnostics/`, for the owner-readable detail leg of `v1-47`.
  > serde_path_to_error drift reports, which can quote tool `input`, are written only to `diagnostics/`.
- (.andromeda/security-plan.md:337) The secret-scan leg of `v1-47` must be this canary scan, its matched bytes never printed or written and each scan-gated upload waiting on it.
  > the CI `test` job runs obs-plan §9's artifact canary scan against this plan's NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass
- (.andromeda/security-plan.md:340) Every product `Display` that can reach a surface, in the bootstrap wire's own words, must interpolate no upstream text and no absolute path.
  > The `Display` impls on the `<Crate>Error` enums (`thiserror`, or hand-written for `PtyError`) must not interpolate upstream text or absolute paths.
- (.andromeda/security-plan.md:341) MCP `structuredContent` on the `isError` results the chunk enumerates must carry codes only.
  > MCP `structuredContent` carries only `refusal`/`detail`/`error` codes.
- (.andromeda/security-plan.md:342) A failure that joins an anyhow chain must be mapped to a fixed message first, with any serde source stopped before it, and full detail must go only to `diagnostics/` when an instance is resolved.
  > anyhow chains stop at the `viola` bin's human-mode stderr. A chain printed there never includes a serde_json or serde_path_to_error source, or any other source whose `Display` can quote upstream text (serde errors quote the offending value). Map it to a fixed message before it joins the chain, and write the full error only to `instances/<name>/diagnostics/` when an instance is resolved (NEVER-log floor, `logging-redaction-wire`).
- (.andromeda/security-plan.md:343) Any wording this chunk moves on the floor must stay undiverged from this one consolidated list.
  > This list is the consolidated NEVER-log floor. Data Protection § Logs, Error Handling and Anti-Patterns § Secrets / § Logging restate parts of it and must not diverge from it:
- (.andromeda/security-plan.md:344) The planted stripped `CLAUDE*` value of each witness must be absent from every one of these destinations as well as the error bodies.
  > Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.
- (.andromeda/security-plan.md:345) The witnesses' upstream text must be absent from these four destinations, while the content-bearing contract payloads the floor's scope exempts stay unrestricted.
  > Outside those payloads, user content (prompts, `last_assistant_message`, plan, question and option text), tool `input`, the `statusline_command` string, and serde_path_to_error drift reports (paths and values) go only to `instances/<name>/diagnostics/`. They never go to stderr log or error lines, CLI `--json` error output, MCP `isError` results or channel `error.data`.
- (.andromeda/security-plan.md:346) The three surfaces must carry no absolute path and no internal type name, the two values a witness must assert absent beside the upstream text.
  > Operational metadata (TMS §1): absolute paths, which carry the OS username (the only PII in scope), and internal type names never go to CLI `--json` error output, MCP `isError` results or channel `error.data`. Pids, `agent_session_id`, `started_at` and `budget.json` values may be written to `diagnostics/`.
- (.andromeda/security-plan.md:347) The detail files that receive full error detail must be 0600, for the owner-readable leg of `v1-47`.
  > `diagnostics/` files are 0600.
- (.andromeda/security-plan.md:372) A witness planting a stripped value must find it unserialised, and the strip must keep handling names only, with no value read added for the proof.
  > removed from the child by the R8 rule (the identity floor whatever the persistent set says) and never serialised into events, snapshots, diagnostics or the ledger; the strip handles names only and never reads a value.
- (.andromeda/security-plan.md:374) The planted value must be a test's own synthetic assignment, never a value taken from the host environment, since a host-sourced assignment widens this boundary.
  > launches every child under `env -i` with exactly `HOME` and `PATH`, so no host environment value crosses into a child; any assignment whose value comes from the host is a boundary widening needing its own Decisions Log entry, and no uid-0 launch exists.
- (.andromeda/security-plan.md:381) The canary classes the scan holds must include the stripped `CLAUDE*` needles the witnesses' planted value is checked against.
  > the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
- (.andromeda/security-plan.md:409) The rule's scope must be exactly the chunk's three surfaces.
  > **External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.
- (.andromeda/security-plan.md:410) Each error body on the three surfaces must be free of all five of these classes.
  > - No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
- (.andromeda/security-plan.md:411) These `Display` impls must use fixed messages with path and payload fields excluded, and `viola-agent-claude`'s `Refusal` must meet the same rule whether it folds into `AgentError` or is ratified apart.
  > - The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
- (.andromeda/security-plan.md:412) Channel `-32603` must be exactly this message and data, on every path a wrapper answers it.
  > - Channel `-32603` uses a fixed `message` ("internal error") with `data: null`.
- (.andromeda/security-plan.md:413) The MCP enumeration must be this closed set: each `isError` shape, the `refused: <code>` text, and `-32603` for any other fault, a reply of no known shape or a panicked call.
  > - MCP `isError` results carry only the `structuredContent` codes from Conventions (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`, `invalid-params` with a closed field word), and every `isError` text is `refused: <code>`. Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null`
- (.andromeda/security-plan.md:416) Full error detail must go only to these destinations, and stderr must carry nothing on the floor.
  > - Full error detail goes only to `instances/<name>/diagnostics/` (files 0600; format owned by obs) or, for short-lived CLI verbs, to stderr. stderr never carries upstream text, tool `input`, drift-report values or any other value on the NEVER-log floor (Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:421) The channel enumeration must keep this split between JSON-RPC `error` protocol faults and `result.refusal` codes.
  > - **Channel:** JSON-RPC `error` for protocol faults, including `-32600` for frames over `MAX_FRAME` and `-32602` for `release`-with-`from`, and `result.refusal` for `not-delivered`/`control-character` and `not-delivered`/`empty-text`.
- (.andromeda/security-plan.md:476) The planted value must also be absent from stderr and `diagnostics/`.
  > NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:482) No drift report or anyhow chain may reach MCP `structuredContent` or channel `error.data`, and no stderr chain may keep a serde source.
  > NEVER echo serde_path_to_error drift reports or anyhow chains into MCP `structuredContent` or channel `error.data`, and NEVER print an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source (its `Display` quotes input values such as tool `input`).
- (.andromeda/security-plan.md:487) This ban must hold on every external error the chunk enumerates.
  > NEVER expose absolute paths in any external error.
- (.andromeda/security-plan.md:519) A witness must drive its failures with no new environment read or product seam unless that seam has its own Decisions Log entry.
  > NEVER add another such seam or environment read without its own Decisions Log entry.

## Bindings
- **Fixed error bodies ↔ the body's NEVER-log floor**: the external-response rule and the floor name the same three surfaces and must not diverge; the floor is the consolidated list, (.andromeda/security-plan.md:410) and (.andromeda/security-plan.md:343).
- **MCP `isError` codes ↔ architecture Conventions**: the MCP code set stands in Conventions and this plan restates it, so the two must name one closed set, (.andromeda/security-plan.md:413).
- **`Refusal` enum ↔ architecture [Error Handling]**: architecture's one-enum rule decides the fold or the ratified exception, and this plan's fixed-message rule binds the enum either way, (.andromeda/security-plan.md:411).
- **`-32602` causes ↔ obs-plan §5 and the Metric Coverage detail**: telling `release-from-driver`, unsupported-version and invalid-params apart on the `channel-response` line is a contract read against obs-plan, and the channel's fixed error bodies must not change for it, (.andromeda/security-plan.md:116).
- **Secret-scan leg of `v1-47` ↔ obs-plan §9 and `verification-matrix.json#v1-47`**: obs owns the scan, this plan owns the floor it checks, and the matrix's acceptance reads the two together, (.andromeda/security-plan.md:337) and (.andromeda/security-plan.md:381).
- **Full error detail ↔ obs's diagnostics format**: obs owns the format, this plan owns the destination, its 0600 mode and the failed-open drop, (.andromeda/security-plan.md:416) and (.andromeda/security-plan-detail/error-handling.md:18).
- **Cap-driving witnesses ↔ test-plan §4 and §6 and the Input Validation detail**: test-plan names this chunk owner of the cases, and the cap they drive is this plan's one const, (.andromeda/security-plan.md:178) and (.andromeda/security-plan.md:142).
- **Strict-modes refusal documents ↔ Authentication & Authorization and "Unix endpoint and home hardening"**: this chunk enumerates the refusal bodies and leaves the checks' hardening to that entry, (.andromeda/security-plan-detail/error-handling.md:15) and (.andromeda/security-plan-detail/error-handling.md:16).
