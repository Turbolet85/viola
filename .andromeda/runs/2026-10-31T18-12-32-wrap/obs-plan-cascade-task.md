# Task — obs-plan · 2026-10-31-exit-cause-code-catalogue

## The session
Gave every exit-21 and exit-1 cause a distinct code across `--json`, MCP and the process log, matching its human hint; no exit moved, no `RefusalReason` and no exit code was added, and exit 21's `error` stays `instance-unreachable`.
Changed exit 21's document from `detail` null to one closed cause (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), `v` still 1; MCP's `isError` result is the same less `v`, and MCP's text stays `refused: instance-unreachable`.
Split the log codes: a stale holder now logs `stale-heartbeat` rather than `already-live`, a wrapper lost with a call open logs `endpoint-lost` with `during` `call`, `instance-dead` no longer carries `during` `call`, and `verify` writes a code per refusal on its `process-exit`.
Closed two `--json` gaps: `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, and the `cli` catch site prints the `internal-error` document on stdout with stderr empty.
Measured: `process-exit` `detail` enum 13 to 19 members, `ci` kill line 24 to 25 `binary(` operands, `human::refuse(` still 18 sites, the new table `tests/cross_exit_causes.rs` 39 tests on Linux, pre-push 3403 passed, CI green on the final HEAD.
The change reached 41 files; the list below is capped at 40, and the one left off is `tests/cli_controls_not_disableable.rs`, a one-line test change.

## Reached files
- tests/cross_exit_causes.rs
- .config/nextest.toml
- Cargo.toml
- schemas/diag-line.v1.json
- crates/viola-core/src/lib.rs
- crates/viola-state/src/liveness.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- src/main.rs
- src/human/hints.rs
- src/cmd/mod.rs
- src/cmd/client.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/link.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- tests/support/start.rs
- tests/contract_error_bodies.rs
- tests/chaos_wait_vanish.rs
- tests/cli_send.rs
- tests/cli_list.rs
- tests/cli_machine_contract.rs
- tests/cli_wait_last.rs
- tests/cli_plugin_install.rs
- tests/cli_links.rs
- tests/cli_error_bodies.rs
- tests/cli_instance_state.rs
- tests/cli_revive.rs
- tests/cli_verify.rs
- tests/security_negatives_server_identity.rs
- tests/security_negatives_strict_modes.rs

## Touches
- Any text giving exit 21's `--json` document with `detail` null, on any frame verb, `list` or `plugin install`: it now names its cause; a strict-modes refusal reads `strict-modes-failed`, a not-running one its own cause (the `wait` / `last` scenario at `.andromeda/obs-plan.md:361` is one).
- Any text saying MCP returns a bare `instance-unreachable`: the `isError` body now carries the cause as `detail`, still with no line and no frame; `isError` stays the only record (the MCP reject rule in Error Capture).
- The `detail` code catalog (`.andromeda/obs-plan.md:451`, `.andromeda/obs-plan-detail/log-coverage.md:39`): the catalog's list grows by the six new `process-exit` words.
- The count of `process-exit` `detail` values at `.andromeda/obs-plan-detail/log-coverage.md:28`: 13 becomes 19; `.andromeda/obs-plan-detail/log-coverage.md:37` names the catalog without a count and stays.
- `already-live` as a stale holder's code (`.andromeda/obs-plan-detail/log-coverage.md:26`): retired; it stays the code of a live holder and of a fresh replayed beat.
- The `run.collision_check` span's `outcome` (`.andromeda/obs-plan-detail/span-trace-coverage.md:45`): re-read and unmoved, still `already-live` for a live or a stale holder, while the printed and logged code differs for a stale one.
- `instance-dead` with `during` `call` (`.andromeda/obs-plan-detail/span-trace-coverage.md:95`, `:112`): retired; that case now reads `endpoint-lost` with `during` `call`.
- The `cli` catch site in Error Capture (`.andromeda/obs-plan.md:519`, `:523`, `:527`; `.andromeda/obs-plan-detail/error-capture-reporting.md:16`): the human line `error: internal error` stands byte for byte, and a statement of the output under `--json` too takes the document; `mcp`, `hook`, `run` and `revive`'s start stay silent there.
- A `detail` null that stays true: the catch site's `internal-error` document and other exit-1 documents keep `detail` null, so only exit 21's null is retired.
- The cli surface's stderr statement in Obs Scope: under `--json` a catch-site fault now leaves stderr empty and writes its document on stdout, through `internal_error_document` in `src/cmd/mod.rs`.
- The print-lint exception for output modules that own `--json` in Obs Anti-Patterns: the cli catch site gains a stdout document, so check the exception still names its owner.
- Any statement that a refusal `run`'s start makes after a passed preflight prints no document under `revive --json`: no longer true; the post-preflight gap is retired.
- Any list of `revive --json` codes: now `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`.
- The `run` start-refusal statement in Obs Scope and the `run` start scenario: the human pairs are byte for byte as they stood, `run` passes `json` false, and the `human::refuse(` count of 18 sites was re-read unmoved.
- The `verify` edge flow in Span / Trace Coverage: its `process-exit` now carries a code per refusal; a refused home tree stays lineless and a failing row stays no refusal and no code.
- The Vector 7 trigger in Obs Scope, which says a strict-modes failure has a line only on named sites: read it against `verify`'s `strict-modes-failed` code on its own `process-exit`; the report measures that no refused check gained a line or moved.
- The Founder Direction 4 trigger and alignment gap (a) in Obs Scope, each cause behind exit 21 and exit 1 a distinct `detail` code: now built, with named carve-outs.
- The `viola run` must-trace path's required field in Obs Scope, exit-1 causes with a distinct `detail` code: the start's codes are now built and witnessed; no path or pid enters a code.
- Metric Coverage's grouping by the closed `detail` enum, and any derived metric or query filtering `process-exit` on `already-live`, `instance-dead` or `during`: reached through the six new values and the moved codes.
- Log levels mapping: the new codes fall under its standing `process-exit` rows by exit code, exit 1 at error and exit 21 at warn.
- Gate G4 and the schema-conformance budget: they read `schemas/diag-line.v1.json`, whose only moved list is `process-exit` `detail` at line 78; `channel-response`'s three words and `hook-decision`'s seven did not move, and no event value or field was added.
- The SLO rule on module-boundary error logging: its closed `detail` set is the grown one.
- The stale-heartbeat chaos class and the no-logged-transition rule in SLO: a stale holder's `stale-heartbeat` is a start refusal's code, never a liveness transition line, and a frame verb against a stale instance still reads `instance-dead`.
- The `viola-core` scope entity in Obs Scope: it now also supplies `UnreachableCause`, and it stays pure with no line of its own.
- The `viola-state` and `viola-mcp` scope entities in Obs Scope: `viola-state` gains `without_endpoint`, and `viola-mcp` maps each refused frame to a cause.
- The rule that every integration-test home lives under `target/e2e-home/`: the new root binary `tests/cross_exit_causes.rs` and the setups moved to `tests/support/start.rs` fall under it.
- The `ci` kill line in CI Integration, if named: it gains `binary(cross_exit_causes)`; no obs text was found to state the count of 25.
- Line citations and test names into the reached files, chiefly `src/cmd/client.rs`, `src/cmd/run.rs`, `src/cmd/revive.rs`, `src/cmd/verify.rs`, `src/main.rs`, `crates/viola-mcp/src/result.rs`, `crates/viola-mcp/src/tools.rs`, `tests/cli_instance_state.rs`, `tests/contract_error_bodies.rs`: lines moved.

## New facts
- F1 · log-coverage · The `process-exit` `detail` catalog gains `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture`.
- F2 · log-coverage · `verify` writes its refusal's code on the `process-exit` under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`; the fixture's file name enters no code, and `verify` takes no `--json`.
- F3 · log-coverage · `stale-heartbeat` is exit 1's cause alone, written by `collision_check` in `src/cmd/run.rs` for a stale holder; `live_endpoint` gives none for a stale wrapper, so a frame verb reads `instance-dead`.
- F4 · span-trace-coverage · A process-exit's `during` is `call` for `endpoint-lost` only; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.
- F5 · span-trace-coverage · `viola_state::liveness::without_endpoint` reads a no-endpoint instance by `symlink_metadata`: a directory is `instance-dead`, anything else `unwrapped-name`; it opens nothing, writes no line, and the strict-modes check stays first at every caller.
- F6 · error-capture-reporting · Under `--json` the `cli` catch site prints the `internal-error` document on stdout in place of the human line, stderr then empty, decided by `json_asked`, which reads `--json` from argv before any `--`.
- F7 · error-capture-reporting · Under `revive --json`, an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site still logs its chain and its line.
- F8 · error-capture-reporting · A `pump_child` error under `revive --json` prints no document: the child has held the terminal, and viola writes nothing past it.
- F9 · error-capture-reporting · `refused_with` prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and `internal-error` for a program not found (no human pair today, none added), all before `spawn_child`.
- F10 · error-capture-reporting · MCP maps its refusals to causes: a refused `check_instance` or board is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `ServerVerify`, `Connect` is `InstanceDead`, and `Closed` or `Io` is `EndpointLost`.
- F11 · obs-scope-summary · `viola_core::UnreachableCause` is the closed set of exit 21's five cause words; it has no serde path, and every word is written by the two clients.
- F12 · obs-scope-summary · Exception to the distinct-code rule, by the founder's named carve-outs: the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check, both left with the route entry Unix endpoint and home hardening; the row leaving the socket-directory code out is the delegate's word, PROVISIONAL.

## Untouched
- The hook role's fail-open, silent no-op and stdout rules: `src/cmd/hook.rs` is not in the diff, and `hook_fail_open` stands green.
- The channel wire and its logging: nothing in `viola-channel`, so `channel-request` / `channel-response`, the `conn` join key and `-32603`'s fixed body are not reached; a mention of `instance-unreachable` near them is not a channel change.
- CI Integration's G1 to G4 commands, step order, artifact uploads, secret scan and retention: nothing in `.github` or `scripts`, and no harness verb, status shape or CI step moved.
- SLO performance budgets and the error budget's three zero counts: no hook, perf row or threshold moved.
- PII Scrubbing's data classes, veil, sink separation and `schemas/diag-detail.v1.json`: detail-file lines gained no field and the schema is not in the diff; the new codes hold no path, pid or upstream text.
- Exporter and platform picks (no OTel, no sentry, file sink only): no dependency added or bumped.
- The `viola-pty` and `viola-agent-claude` scope entities, and the fixtures and plugin entities: none of their files is in the diff.
- Snapshot replay and torn-line healing (`state-recovered`): the replay and appender code is not in the diff, and the session adds no reader.
- Metric naming and the rule of no in-process instruments: every metric stays derived, and no metric was added.
