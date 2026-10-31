# Task — obs · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code across `--json`, MCP and the process log, no path and no pid in it, matching the human `hint:` line of the same cause.
Builds the exit-cause table `tests/cross_exit_causes.rs`: one negative rstest case per cause, asserting exit code, `--json` document and last stderr `hint:` line, a hint shared across causes failing the table.
Gives a cause the code holds and no process-log line carries its log form, or names its absence as a decision; the strict-modes refusals write no line by rule; no new exit code, and no refusal widened, moved or made to write a line it writes none of today unless P4 names it.
Closes the `viola revive --json` gap after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`) and builds the unwrapped-name hint, claiming `verification-matrix.json#v1-18` and `#v1-39`.
Re-reads whether `revive`'s strict-modes check still runs before its log opens, whether `Refusal::StrictModes.detail()` still stands, and whether `session-live` is landed; tallies the `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` WATCH on its CI reads.

## Reached files
- tests/cross_exit_causes.rs · edits the rstest `#[case]` table, one negative test per cause (new file)
- crates/viola-core/src/lib.rs
- src/cmd/client.rs · edits `refused_document`, `refuse_untrusted`, the liveness pre-check, the server check
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/mcp.rs
- src/cmd/run.rs · edits its start refusals, `checked_exe` / `refuse_tree`, the collision check
- src/cmd/revive.rs · edits the document after a passed preflight
- src/cmd/verify.rs · edits its refused home tree
- src/human/hints.rs · edits the unwrapped-name hint, each cause's own hint
- src/human/mirror.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-channel/src/client.rs · edits the vanished server read as `instance-unreachable`
- schemas/diag-line.v1.json · edits the `process-exit` `detail` values
- tests/cli_machine_contract.rs · edits the assertions the moved document reaches
- tests/cli_revive.rs · edits the assertions the moved document reaches
- tests/cli_list.rs · edits the assertions the moved document reaches
- tests/cli_links.rs · edits the assertions the moved document reaches
- tests/cli_verify.rs · edits the assertions the moved document reaches
- tests/cli_plugin_install.rs · edits the assertions the moved document reaches
- tests/security_negatives_strict_modes.rs · edits the assertions the moved document reaches
- tests/security_negatives_server_identity.rs · edits the assertions the moved document reaches
- tests/contract_error_bodies.rs · edits the census of error bodies and writers
- tests/cli_error_bodies.rs · edits the assertions the moved document reaches
- crates/viola-mcp/tests/handlers.rs · edits the assertions the moved document reaches
- crates/viola-e2e/tests/mcp_tools.rs · edits the assertions the moved document reaches
- .andromeda/test-plan.md · edits §6 Exit-cause matrix, §6 security-control table `viola list`'s row
- .claude/rules/security.md · re-reads its statement that `session-live` landed, its strict-modes refusal's `detail: null`

## Needs
- The closed `process-exit` `detail` catalogue: which codes it holds today for exits 21 and 1, its kebab-case and no-path-no-pid rule, and the new values this chunk adds to it and to `schemas/diag-line.v1.json`.
- The creator-explicit trigger behind this chunk (Founder Direction 4) and its Section 1 mirrors: a distinct `detail` per exit-21 and exit-1 cause, the alignment gap naming them, the run-start path's required fields; a change of contract reaches this text.
- The strict-modes carve-out: a check refused before any log opens writes no line and its cause is the stderr pair or the `--json` document, against the only strict-modes lines that exist (`ledger-stamps` in `run`'s version gate, a refused `conpty/` companion's `sideload_fallback`).
- The refused home or instance tree on the frame verbs `wait` and `last`, ending before any role file opens, and any plan text stating its exact `--json` document with `detail` null, which this chunk's contract choice can move.
- `server-verify-failed`: its `process-exit` shape at WARN with `during:"connect"` and no `channel-request` line on Linux and Windows, MCP's return with no line, and the recorded macOS gap with no peer pid comparison.
- A mid-call disconnect, the endpoint-lost cause: logged once by the client, read through `crates/viola-channel/src/client.rs`, which line carries its new code.
- The not-running and unwrapped-name causes on the frame verbs `send`, `answer`, `pause`, `release`, `link`, `unlink`: what their scenarios and spans log for exit 21 today, each verb's role file (`link` / `unlink` into the driven instance's `cli-<driven>.ndjson`).
- `viola run`'s start refusals (already live, stale heartbeat, squatted name, pinned-hash mismatch, script child, strict-modes or socket-directory refusal): their stderr pair through `human::refuse`, no path or pid, which come before the log and which reach a `process-exit`, and the refusal telemetry owed to a pinned-exe mismatch.
- `viola revive`'s refusals: its start arm logging as process `run`, its home-tree check before that log, the logged `detail` of `already-live`, `no-session`, `cwd-missing` (and `session-live` if landed), the `revive --list` pair, and `revive`'s line for a refusal `run`'s start makes after a passed preflight.
- `internal-error`: the `process-exit` line through `obs::internal_error_exit_line` for `Run` and `Cli` only, `viola::obs::report_internal_error`, and the fixed stderr message with no hint, which the table's rule of a last `hint:` line per cause must meet.
- `viola mcp`: `isError` results and rejected-call lines carrying codes only, `instance-unreachable` logged by no line with the `isError` code its only record, stdout holding MCP frames alone, the refused home check that is exit 1 with nothing on stdout or stderr, and the new codes passing through `crates/viola-mcp/src/result.rs`.
- `viola list`'s whole-board strict-modes refusal (exit 21), and `viola plugin install`'s exit 21: what they log, their `cli.list` and `cli.plugin_install` spans' place relative to the refusal, each joining the table or named outside it by a reason.
- `viola verify`'s refused tree: exit 1 with the home pair and no line before its log, against the `ledger/stamps.json` stamp-write refusal, same pair with its own `process-exit`, and its logging only when `VIOLA_NAME` is set.
- The error layer new codes cross: fixed `Display` strings, the replaced serde-sourced chain before stderr, `--json`, MCP `isError` and channel `error.data`, channel `-32603` with null `data`, read through `viola_core::is_code_word` and `viola_channel::sanitised_fault`.
- The human refusal writer in `src/human.rs`: its two fixed lines in one `write_all`, its call-site counts per verb in Section 3, should this chunk move a site.
- Section 6's level mapping as it applies to the new logged causes: `process-exit` with exit 1 at ERROR, with exit 21 at WARN.
- The module-boundary error rule: every typed error crossing a crate seam ends logged once with a closed `detail` or `error_code`, with MCP's no-line outcome and the pre-log refusals as its edges.
- Any row keyed on a word this chunk moves: a derived metric, a `jq` snippet, the `agent-run status` `last_error` enum or a scenario that groups or filters on `process-exit` `detail` or on `instance-unreachable`.
- The hook's edge: exit 0, no body, no stderr on a refused instance, its `hook-decision` detail its only cause record, so the catalogue adds no hook cause.
- The socket directory's refusal: a code only if the code holds that refusal today, its verified 0700 belonging to another route entry.

## Standing conventions
- S1 · Every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the upload cover it · §9 CI Integration
- S2 · A new log line goes through `obs_event!` with explicit typed fields, never a raw tracing macro, `corr` recorded unstringified · §11 Logs
- S3 · A field is logged only if named in the Section 6 catalogue, default-deny against `schemas/diag-line.v1.json`, gated by G4 · §8 PII Scrubbing, log-coverage.md
- S4 · A `detail` value is a closed kebab-case code, never a path or pid, low-class metadata going as typed fields · §6 Log Coverage, §11 PII Scrubbing
- S5 · No test retries or retry-once policy, and no log-based wait relying on sleeps · §10 SLO Invariants, §11 SLO
- S6 · Zero tolerance on `event:"panic"` lines, secret-scan hits and schema-conformance failures across test homes · §10 Error budget
- S7 · Every input a test feeds the product is synthetic, and the content canary never appears in a home-level file · §8 PII Scrubbing, pii-scrubbing-compliance.md
- S8 · No `print!`, `eprintln!` or `dbg!` on `hook`, `run` or `mcp` paths, output owned by the `--json`, human stderr and decision-body modules · §11 Logs
- S9 · A span added on a new path is `#[instrument(skip_all, ...)]` with an allow-list, a static name, no `err` or `ret` · §4 Span / Trace Coverage, §11 Spans / Traces
- S10 · Every verdict over telemetry is an assertion that sets an exit code, never human review · §2 Telemetry Strategy, §11 CI

## Not needed
- OTel SDK, OTLP exporter, W3C trace context and any error reporter: the chunk adds no exporter or egress.
- The logger stack's install (subscriber layers, timer, writer, per-module `Targets`, `diagnostics_level`): the chunk writes through the pipeline as built.
- The PTY pump, vt100 feed, readiness gate and paste-write telemetry: no exit-cause code arises there.
- The budget governor and `viola hook statusline`: exit 20's cause and the shell-out are not re-opened.
- Exits 10–14 and their `send-refused` reasons and details: already distinct and outside this catalogue.
- The dialog scenario's `dialog_id`, `hook.dialog` and `hook-invoked` timing: the catalogue moves none of them.
- `conn` and `corr` correlation joins and the merged-log join key: no correlation field changes.
- The `events.ndjson` product stream, `link` / `unlink` product lines, `wheel` events, snapshot replay and torn-line healing: no product event or recovery line changes.
- R8 env-strip counts, capability-ledger stamp contents, drift reports and `claude agents --json` parsing: no cause here is an exit-21 or exit-1 code.
- Hyperfine perf budgets and hook latency gates: the hook paths stay as they are.
- CI workflow step order, artifact retention, pinned Actions and the nightly jobs: the chunk edits no workflow.
- `events.ndjson` and `diagnostics/` retention and rotation: nothing here rotates a file.
- The frontend: deferred to 0.2.0, with no frontend scenario in the plan.
