# Task — obs · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Adds the home strict-modes check (`viola_state::strict::check_instance`, Unix mode/owner, Windows owner + DACL) BEFORE any read or open of the home at every entry point: CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s nine events (its `VIOLA_DIR` snapshot read, failing open); `viola mcp`'s frame tools (`live_endpoint`); `verify`'s `update_stamps`; `run` (home, `instances/<name>/`, `bin/`, `plugin/`, `ui/`, trusted files, Windows `conpty/` companions); and `client::start`'s role-file open — closing the dated strict-modes exceptions of security.md.
Makes the owner-only home whole (0700 dirs / 0600 files on Unix, protected owner + SYSTEM DACL on Windows) and puts the Windows DACL check before any `diagnostics/detail-*.ndjson` or role file is opened, with a Windows proof of `check_instance` and a reading of the DACL of a test-seeded (`seed_conpty`) home.
Pinned binary: `bin/<version>-<hash>/viola` re-hashed before reuse, the `pin_exe` first-start race closed, and two clean release builds of one tree measured by file and section; `plugin/` rewritten atomically each start; the `marketplace/` tree's integrity; single writers of `snapshot.json` and `ledger/stamps.json` proven.
Re-reads the CI verdict of `99df45362ac6` at P3; the readers' move onto `read_snapshot_or_replay` is the founder's P4 fork, neither planned in nor out.

## Reached files
- crates/viola-state/src/strict.rs · edits check_instance (its Windows proof, the DACL cases)
- crates/viola-state/src/fs.rs
- crates/viola-state/src/stamps.rs · edits update_stamps (the stamps read)
- crates/viola-state/src/liveness.rs · edits live_endpoint
- crates/viola-state/src/pin.rs · edits pin_exe (the home copy, its race, the re-hash before reuse)
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/replay.rs
- src/cmd/client.rs · edits start (the role-file open)
- src/obs.rs
- src/cmd/hook.rs · edits the snapshot read through VIOLA_DIR
- src/cmd/hook/statusline.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/verify.rs
- src/cmd/run.rs · edits start (the home, instances/<name>/, bin/, plugin/, ui/ and trusted-file checks; the collision check)
- src/conpty.rs
- src/cmd/plugin.rs
- tests/cli_controls_not_disableable.rs · edits the 0770 --home negative (exit 21), the table's completeness case
- tests/support/home.rs
- tests/hook_statusline.rs

## Needs
- Order of init: the strict-modes check runs before any role file (`run-`, `hook-`, `cli-<name>.ndjson`, `mcp.ndjson`) or detail file is opened, so a refused check falls in the pre-step-4 window where no file exists and the panic hook writes nothing.
- A refused strict-modes check writes no process-log line on either arm (the §10 exemption for a refused check), except the `ledger-stamps` read in `run`'s version gate; whether `verify`'s `update_stamps` read now gets a line, and which, binds through the Vector 7 trigger.
- `process-exit{subject:"self", exit_code:21}` at WARN with a closed `detail` (`strict-modes-failed`) for the CLI frame verbs newly refused, codes only, no path, pid or endpoint, and its `detail` code in the closed catalog.
- `hook` fail-open on a refused check: exit 0, no body, stderr empty, `hook-decision` with a fail-open `detail` only where a role file may still be opened — the check precedes the open, so the refused arm may log nothing.
- MCP frame tools refused by the new check return `instance-unreachable` with `detail: null` and write no line, the outcome kept inside `mcp.tool_call`.
- The `wait` / `last` scenario's statement that `strict-modes-failed` cannot occur on them, its check owed to this route entry, becomes stale once the check lands.
- `diagnostics/` files 0600 in 0700 directories, Windows DACL-protected; a `diagnostics/` file not 0600 (or not DACL-protected on Windows) is a build failure condition.
- Refusal telemetry for a pinned-exe hash mismatch (tampered pinned copy): `run`'s two fixed stderr lines through `human::refuse`, no path or hash, and its `process-exit` detail code; the `pinned_bin` path is never logged.
- The `human::refuse` site counts recorded in §3 (run 5, verify 5, list 2, revive 1, plugin install 1, link 1) move if the chunk adds a refusal site.
- `run` start-sequence scenario: never the `pinned_bin` path, the ConPTY companions' paths or hashes; a sideload outcome is never a refusal or exit change — the `conpty/` companions joining the Windows strict-modes set must keep that.
- Capability ledger edge flow: a strict-modes-refused `ledger/stamps.json` counts as `cli_verified:false`, path and stamp contents never logged; `verify` with `VIOLA_NAME` logs `process-start` / `process-exit` to `cli-<name>.ndjson`, none from a plain terminal.
- `plugin install` logging (`cli.plugin_install` span, opened after its log) where the `marketplace/` integrity work touches it.
- E5: only `read_snapshot_or_replay` writes `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}`, one line per call; the other snapshot readers write none — binds the readers if the P4 fork moves them.
- Hook perf budgets (spine hooks under the spine deadline, `session-end` under 1.0 s, hyperfine `max` through `gate --require perf`): the strict-modes check added on every hook's path counts inside them.
- Server-verify lines already standing on these verbs (`server-verify-failed`, `during:"connect"`) keep their order after the new check.
- Panic hook and `catch_unwind` sites stay first in `main` and around dispatch; no new check path ends a role through an uncaught unwind.

## Standing conventions
- S1 · every new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, no raw tracing macros · ## 11. Obs Anti-Patterns › ### Logs
- S2 · every new field or `detail` code is in the §6 catalog and validates against `schemas/diag-line.v1.json` (G4, default-deny) · ## 6. Log Coverage / log-coverage.md
- S3 · a `detail` code is closed kebab-case and never holds a path or pid · ## 6. Log Coverage / ## 11. Obs Anti-Patterns › ### PII Scrubbing
- S4 · every new integration-test home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · ## 9. CI Integration / ci-integration.md
- S5 · zero unlogged panics over home-level role files (G2), no retry-once policy · ## 10. SLO Invariants & Telemetry Budgets / slo-invariants-telemetry-budgets.md
- S6 · no telemetry on stdout on any role, nothing on `hook`'s stderr, `with_writer` explicit · ## 3. Observability Harness Contract / ## 11. Obs Anti-Patterns › ### Logs
- S7 · the secret-scan floor over every `diagnostics/` file (no `CLAUDE*` value, no content canary in a home-level line, 0600) · ## 8. PII Scrubbing & Compliance / pii-scrubbing-compliance.md
- S8 · spans named `<area>.<operation>`, static, no instance name or path · ## 4. Span / Trace Coverage / span-trace-coverage.md

## Not needed
- Metric Coverage's derived `viola.*` metrics and their cardinality rules: the chunk adds no metric.
- Error-reporting platform choice (sentry, reporter preconditions): no reporter is added.
- veil / type-level redaction of channel, hook and MCP payload types: no payload type changes.
- Confirmed-`send` CL-1 readback, `/compact` confirm window and paste `text_bytes`: the send path after the frame is untouched.
- Dialog → `answer` content rules (question, option text, `hook-invoked` timing): no dialog flow changes.
- Wheel / `release-from-driver` scenario: the wheel is untouched beyond the pre-frame check.
- Budget governor shell-out (`STATUSLINE_DEADLINE`, `budget-gate`, `budget.json` writer): the statusline arm already checks before its open.
- E2 R8 strip fields and E3 `link` / `unlink` traces: `link` / `unlink` already run `check_instance` first.
- Heartbeat liveness-transition logging and its 5 s flip: unchanged.
- vt100 feed `parse-rejected` and torn-line healing: unchanged.
- `claude agents --json` parse-rejected handling: no agents read is added.
- OTel SDK, OTLP exporter, trace-context propagation and W3C traceparent rules: nothing of the pipeline changes.
- CI artifact retention and upload step order: no upload or job is added.
