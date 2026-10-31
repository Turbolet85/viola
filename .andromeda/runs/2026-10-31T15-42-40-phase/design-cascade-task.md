# Task — design · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code across `--json`, MCP and the process log, carrying no path and no pid, and matching the human `hint:` line of the same cause.
Builds the exit-cause table `tests/cross_exit_causes.rs`, one negative case per cause, each asserting the exit code, the `--json` document and a last stderr `hint:` line naming that cause; a hint shared across causes fails the table.
Builds the owed unwrapped-name hint, the `--json` document for `viola revive`'s refusals after a passed preflight, and a log form or a named absence for each cause no process-log line carries.
Moves the document every exit-21 cause prints today, so the plans' text naming it is amended once the contract choice is made; adds no exit code, changes no exit, and widens or moves no refusal unless P4 names it.
Re-reads, on this chunk's CI reads, the margin watch on `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`.

## Reached files
- tests/cross_exit_causes.rs
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
- tests/support/fake.rs · re-reads the 7 s `WITHIN` of `wait_for`, the margin the watch tallies
- .andromeda/test-plan.md · edits the Exit-cause matrix, the security-control table's `viola list` row, the text naming the document
- .andromeda/architecture.md · edits the text naming the document
- .andromeda/security-plan.md · edits the text naming the document
- .andromeda/design-system.md · edits the text naming the document, the two pending lines this chunk owns

## Needs
- The CLI exit-code phraseology, exits 1 and 21: each one's human stderr form and its `--json` document, the text this chunk's moved document amends.
- The exit-21 human form: `unable  instance-unreachable` then the cause's hint, and for a refused strict-modes check the fixed home-state or instance-state `unable:` / `hint:` pair instead.
- The exit-21 `--json` rule: one document with stderr empty, and no `hint:` line under `--json`; the table's cases assert both the document and the human hint of one cause.
- The pending line in the send pattern: a per-cause `detail` code, owned by the arch amendment naming them; this chunk settles it.
- The pending line in the revive pattern: a refusal `run`'s start makes after a passed preflight prints no `--json` document, owned by this entry; this chunk closes it (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH`).
- The unwrapped-name hint's exact text in the send pattern (a name not wrapped, only sessions started with viola run take commands), and an unwrapped session never being a target: built here.
- One `hint:` line per reason, detail or cause, directly after its refusal, and the per-cause lines listed under the send hints: the source each table case's hint is matched against.
- The exit-1 start refusals of `run` and `revive`: a fixed-message `unable:` line and one hint per cause, for example already live with its `viola list` hint; `revive`'s causes read in a fixed order that stops at the first.
- `viola revive`'s `--json` under a refused strict-modes check or preflight: one stdout document, no hint; `no-session`, `session-live`, `cwd-missing` among its causes.
- The rule that faults and `unknown` get no hint, and exit 1's `error: internal error` form, against the table's assertion that each cause ends on a `hint:` line naming it: internal error must be cased or named outside by a reason.
- `viola list`'s whole-board strict-modes refusal (exit 21) in the fixed `unable:` / `hint:` form, and under `--json` the `instance-unreachable` document, joining the matrix.
- `viola plugin install`'s refused strict-modes check (exit 21): the home-state pair, or under `--json` the document with stderr empty.
- The instance-state pair of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` before any frame (exit 21): names that instance; no frame and no line are written, which this chunk must keep.
- `viola verify`'s refused home tree: exit 1, no step line, no summary, only its fixed pair; a refused `stamps.json` update prints the home-state pair.
- The `hook` and `mcp` platform notes: `hook` writes no stderr and exits 0; `mcp` writes only MCP frames on stdout, so its refused home check stays exit 1 with nothing printed, joining the table or named outside it by a reason.
- The cli ban on upstream text, paths, pids and anyhow chains in errors and hints, and the rule that a hint never quotes sent or upstream text: every new code and hint stays inside it.
- The rule that no hint names `viola release`, since a driver's `release` is refused: every table row's hint honours it.
- The rule that full detail goes only to `instances/<name>/diagnostics/`, errors staying fixed messages: the boundary for what a new `process-exit` `detail` code may carry.
- The frontend's deferral to 0.2.0 (Surface: web-spa): the ruling that names `v1-18`'s `ui` start refusal out if the code holds none.

## Standing conventions
- S1 · Each new refusal or hint line is ASCII only and is never wrapped · ## Surface: cli, Width
- S2 · Each new refusal or hint line carries no colour, glyph or SGR, and no such byte reaches stdout or stderr under `--json` · ## Anti-Patterns, Per-Surface Bans, cli
- S3 · Each new message is fixed text with no stack trace, and results never mix with refusals and hints across stdout and stderr · ## Anti-Patterns, Per-Surface Bans, cli
- S4 · Each new human word or code is short fixed phraseology with a typed reason, no filler and no guessing · ## Brand Identity, Domain anchors

## Not needed
- The colour palette's surface scale, text hierarchy, semantic and domain status colours and border progression: 0.2.0 frontend inputs, and this chunk renders no page.
- Typography, spacing, border radius, motion and iconography values: 0.2.0 frontend inputs or motion levels no refusal line uses.
- The Token Test's colour, length and duration tracing: it binds a 0.2.0 stylesheet this chunk does not write.
- The board's columns, its BAY line, its ordering, its width truncation and its empty line: the chunk touches only `viola list`'s refusal, not its table.
- `wait`'s and `last`'s result lines and message-mode escaping: the chunk moves their exit-21 refusal, not what they print on success.
- The readback mirror's delivered and open states, and the send refusals of exits 10 to 14: their reasons and details are already distinct and are not re-opened.
- The success lines of `pause`, `release`, `link`, `unlink`, `answer` and `plugin install`: no cause of exit 21 or exit 1 lives in them.
- `verify`'s step-counter pass and fail lines and its summary line: the chunk reaches only its refusal.
- The `viola --help` verb rows: the chunk adds and renames no verb.
