# Task — security · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code across `--json`, MCP and the process log, carrying no path and no pid, each code matching the human `hint:` line of the same cause; the typed exit set and every exit code stay as they are.
Builds the new exit-cause table `tests/cross_exit_causes.rs`, one negative case per cause asserting exit code, `--json` document and last stderr `hint:` line, a hint shared across causes failing the table.
Closes the owed gap of `verification-matrix.json#v1-39`: a refusal `run`'s start makes after a passed preflight prints its document under `viola revive --json`; claims `verification-matrix.json#v1-18`.
Folds in `viola list`, `viola plugin install`'s exit 21 and `viola mcp`'s refused home check, each joining the table or named outside it by a reason; a cause no log line carries gets its log form or a named decision, the strict-modes refusals writing no line.
Re-reads at HEAD whether the strict-modes check still runs before the log opens, whether `Refusal::StrictModes.detail()` still stands, and whether `session-live` is built; tallies the 7 s `wait_for` margin watch on its CI reads, no bound moved.

## Reached files
- tests/cross_exit_causes.rs · edits the exit-cause rstest `#[case]` table (a new file)
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
- src/cmd/list.rs · edits its refusal's document
- src/cmd/plugin.rs · edits its exit 21 document
- src/cmd/mcp.rs · edits its refused home check
- src/cmd/run.rs · edits its start refusals, `checked_exe` / `refuse_tree`, the collision check
- src/cmd/revive.rs · edits the document after a passed preflight
- src/cmd/verify.rs · edits its refused home tree
- src/human/hints.rs · edits the unwrapped-name hint, each cause's own hint
- src/human/mirror.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-channel/src/client.rs · edits the vanished server read as `instance-unreachable`
- schemas/diag-line.v1.json · edits the `process-exit` `detail` values
- tests/cli_machine_contract.rs
- tests/cli_revive.rs
- tests/cli_list.rs · edits `e1_send_naming_an_unwrapped_row_is_unreachable`, with the assertions the moved document reaches
- tests/cli_links.rs
- tests/cli_verify.rs
- tests/cli_plugin_install.rs
- tests/security_negatives_strict_modes.rs
- tests/security_negatives_server_identity.rs
- tests/contract_error_bodies.rs · edits the census of error bodies and writers
- tests/cli_error_bodies.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- .andromeda/test-plan.md · edits §6's Exit-cause matrix, the security-control table's `viola list` row, the text naming the moved document
- .andromeda/architecture.md · edits the text naming the moved document
- .andromeda/design-system.md · edits the text naming the moved document, the cli pattern 2 hints
- .andromeda/security-plan.md · edits the text naming the moved document
- .claude/rules/security.md · re-reads the strict-modes refusal's `detail: null` rule and its description of `session-live`

## Needs
- The exit-21 causes, each a distinct closed code: not running, unwrapped name, strict modes, server verification, endpoint lost mid-call; told apart inside exit 21, no new exit code.
- The exit-1 causes, each a distinct closed code: already live, stale heartbeat, squatted name, pinned-hash mismatch, script child, strict-modes or socket-directory refusal, internal error, and `revive`'s `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`; P3 closes the list against the code.
- The shape of the external error bodies a new code enters: the CLI `--json` document, the MCP `isError` `structuredContent` and its `refused: <code>` text, today's `instance-unreachable` with a null `detail`; the rule text naming them is what the contract choice amends.
- A new code's passage through `viola_core::is_code_word` and `viola_channel::sanitised_fault`: a word that fails reads as `unknown` or null on `of_reply` and `reply_of`, so every new code must pass as a code word.
- What no code, document or hint may hold: an absolute path, the OS username, a pid, upstream text, an internal type name, a serde or anyhow chain, on `--json`, MCP `isError`, channel `error.data` or stderr (the NEVER-log floor and the Logging bans, over the scope's own no-path no-pid obligation).
- The fixed `Display` messages on `Refused`, `McpError`, `StateError`, `PinError` and `ChannelError`, wherever a cause's code is taken from such a variant whose fields hold a path or payload.
- Whether a distinct code tells a same-user process more than a null `detail` does today, read against the same-OS-user trust boundary of the IPC vector and the error rules.
- The strict-modes check's place: before any snapshot, `statusline_command` or ledger read and before any log opens, a refusal reading and writing nothing; a code given to it moves no check and adds no line.
- The client's order on every method: the strict-modes check, then server verification on one snapshot read, before the first frame; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect, and telling those causes apart adds no frame, retry or connect.
- The dated gaps owed to Unix endpoint and home hardening: macOS's server check and every client's euid check; a server-verification code claims neither.
- The Unix socket-directory check: a failed check refuses `run`, reports `instance-unreachable` to clients and fails `hook` open; it gets a code only if the code holds that refusal today.
- `hook`'s carve-out: exit 0, no body, no stderr on every failure, security checks included; its `hook-decision` detail stays its only cause record, so no catalogue code reaches it.
- The rule that a security refusal never blocks the human, over any refusal the code-giving touches: none is widened or moved.
- The unwrapped name: a `claude agents --json` row name is never a `ViolaName` nor a valid `target`; its new hint prints only the name the caller typed and what the hint's own rule lets it print, no row text unescaped.
- `revive`'s output rules: `revive --json` prints codes, closed words and session-shaped ids only; no refusal line or hint holds the recorded directory, a pid or a logged id; `no-session` comes before any spawn; the recorded `cwd` is read after the strict-modes check.
- The start refusals gaining a document under `revive --json` and a code: `batch-script-child` (a `.cmd`/`.bat` child), `pinned-hash-mismatch` (the pinned binary's re-hash), `squatted-name`, and a `claude` not on `PATH` as `internal-error`; no hash, resolved path or pid in the code or the document.
- `viola list`'s exit 21 whole-board strict-modes refusal and its row in the security-control table; a failed `claude agents --json` read still fails open rather than becoming a cause.
- `viola plugin install`'s exit 21 and `viola mcp`'s refused home check, the latter exit 1 with nothing on stdout or stderr today; each joins the table or is named out by a reason, its exit and its silence unchanged unless P4 names it.
- `viola verify`'s refused home tree as an exit-cause row: the refusal's code only.
- The channel client's mid-call reading of a vanished server as a distinct code, beside the `wrapper-fault` `{code,message,data}` closure and the fixed `-32603` `internal error` with null `data`, which stay.
- The process log's closed `process-exit` `detail` list: a new value joins `schemas/diag-line.v1.json`, holds no path or pid, and a cause carrying no line is a named decision.
- Each cause's own human `hint:` line matching its code, holding no path or pid, and no `viola release` on any row; the wrapper's refusal of a `release` carrying `from` stays an affordance guard, untouched.
- The census in `tests/contract_error_bodies.rs`: every new code and writer named there, none outside it.
- The security negatives that the moved document reaches, `tests/security_negatives_strict_modes.rs` and `tests/security_negatives_server_identity.rs`: their assertions move while they still prove no frame and no line on refusal.
- The security plan's own text on the document: its MCP and auth rows name `instance-unreachable`, amended only through the choice P4 raises; a boundary widening is the founder's live word.

## Standing conventions
- S1 · each new test runs on its own `--home`, never the user's real home · Secret Management, Development
- S2 · a child launched by the pre-push gate gets exactly `HOME` and `PATH`; a host-sourced assignment needs its own Decisions Log entry · Secret Management, Development
- S3 · every home and file a test builds is owner-only, 0700 and 0600, never the default umask · Security Anti-Patterns, Data Protection
- S4 · each new log line holds nothing on the NEVER-log floor · Bootstrap phases, `logging-redaction-wire` (.andromeda/security-plan-detail/bootstrap-phases.md)
- S5 · each new test's artefacts under `target/agent-run/` are synthetic and stay readable by the artifact canary scan · Secret Management (.andromeda/security-plan-detail/secret-management.md)
- S6 · a test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns, Code Patterns
- S7 · test-only features `fake-agent` and `test-support` stay out of the release build · Dependency Security, CI integration
- S8 · no new test seam or environment read without its own Decisions Log entry · Security Anti-Patterns, Universal
- S9 · no new exemption to the G2 zero-panics gate without its own Decisions Log entry · Dependency Security, CI integration

## Not needed
- GitHub Actions workflow hardening (SHA pins, workflow permissions, zizmor, the nightly and windows-mutants workflows): the scope changes no workflow.
- ConPTY vendoring, its four pins and the sideload with its DLL search order: a sideload failure degrades silently and is no exit cause.
- The v1.x release prerequisites and signing (dist, cargo-auditable, zipsign, `self_update`): v1 has no release.
- The causes of exits 10–14 and 20: their reasons and details are already distinct and the scope does not re-open them.
- Listener construction: the Windows DACL, handle inheritance, `try_overwrite` and the accept-time peer check are server-side and unchanged.
- The statusline shell-out and its integrity: no exit-21 or exit-1 cause runs through it.
- The vt100 feed, PTY output bytes and the human terminal's stdin classifier: no cause or code arises there.
- The R8 strip and the persistent-environment names: the child's environment is unchanged.
- The ledger stamps, the version gate and non-`null` dialog decisions: dialog answering is not a surface this chunk changes.
- `events.ndjson` retention, tailed paths and `budget.json` reads: no cause in the catalogue is decided by them.
- The accepted risks and residuals of verify's trusted runs, the scratch footer reading and a live `viola plugin install` round: the chunk runs no such round.
- The `run --mutants` wipe ban and its Windows mutation scratch: no mutation tooling is touched.
