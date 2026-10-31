# Task — a11y · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code across `--json`, MCP and the process log, carrying no path and no pid, each code matching the human `hint:` line of the same cause.
Builds the new table `tests/cross_exit_causes.rs`, one negative rstest case per cause, each asserting the exit code, the `--json` document and a last stderr `hint:` line naming that cause; a hint shared across causes fails the table.
Closes the owed gap of `viola revive --json`: a refusal `run`'s start makes after a passed preflight prints a document, its codes joining the catalogue, and builds the unwrapped-name hint.
Moves the one `instance-unreachable` document contract, with the code carried in `detail` or in `error` left to P3 and P4, and amends the plan text naming that document where the choice lands.
Re-reads whether `session-live` is landed, the mechanism of the strict-modes refusal writing no log line, and `Refusal::StrictModes.detail()`; tallies the fake harness `wait_for` WATCH on its CI reads.

## Reached files
- tests/cross_exit_causes.rs · edits the new rstest `#[case]` exit-cause table, one negative test per cause
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
- src/human/hints.rs · edits the unwrapped-name hint and each cause's own hint
- src/human/mirror.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-channel/src/client.rs · edits the vanished server read as `instance-unreachable`
- schemas/diag-line.v1.json · edits the `process-exit` `detail` values
- tests/cli_machine_contract.rs
- tests/cli_revive.rs
- tests/cli_list.rs
- tests/cli_links.rs
- tests/cli_verify.rs
- tests/cli_plugin_install.rs
- tests/security_negatives_strict_modes.rs
- tests/security_negatives_server_identity.rs
- tests/contract_error_bodies.rs · edits the census of error bodies and writers
- tests/cli_error_bodies.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- .andromeda/test-plan.md · edits §6 Exit-cause matrix, `viola list`'s row of the security-control table
- .claude/rules/security.md · re-reads `session-live` as landed and the strict-modes refusal's `detail: null`
- tests/support/fake.rs · re-reads `wait_for`'s 7 s `WITHIN`, the WATCH on `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`

## Needs
- CLI refusal layout on every exit-21 and exit-1 cause: the `unable <reason> <detail>` stderr line, `hint:` as the last stderr line, the typed exit code; the table's cases assert this layout.
- The one-line refusal form: `viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair print one `unable: <text>` line then the hint, and the table's rows for them read that form.
- `--json` mode gives one document and no hint: the distinct code travels without hint text, whichever field carries it.
- The plan's Owed line under the CLI verbs entity: under `viola revive --json`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and a `claude` not on `PATH` as `internal-error`, refused after a passed preflight, print no document today; this chunk discharges it.
- Each cause's own recovery hint: the recovery suggestion rule (SC 3.3.3 stand-in) over the hints this chunk adds or splits, including the unwrapped-name hint, each matching its code.
- C0/C1 escaping on the CLI surface: the unwrapped-name hint prints a caller-supplied `<name>`, so its bytes pass under the rule for control characters.
- A kebab-case code on a human stream sits beside the plain word `unable`, never alone; no bare status number and no stack trace as the only error text, which reaches `internal-error` most.
- The carve-out that faults are not refusals and carry no hint, against the table's per-case hint assertion: where it falls on `internal-error` and the mid-call causes.
- The carve-out for the exit-2 paths of `viola answer` and `viola send` (a fixed stderr line, no document): the moved document and the edits to those verbs leave it standing.
- The carve-out for `unknown` (exit 14, no hint, its detail a code word or nothing), and the refusal copy of exits 10 to 14 and 20, which stay as they are while `src/human/hints.rs` and `src/human/mirror.rs` change.
- SGR rules on refusal output: no SGR under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `unable` and `fail` never coloured; this holds for every new or split hint.
- The `viola run` TUI boundary, which a passed `viola revive` enters too: a start refusal or a new `revive --json` document is written only for a start that never reached passthrough; past it viola writes zero bytes of its own.
- `viola run` diagnostics go to `diagnostics/`, never to the terminal: a cause given a new process-log form stays a log line and adds nothing to a terminal stream.
- `viola revive`'s rows: `strict-modes-failed`, `already-live`, `no-session` (two stderr pairs), `cwd-missing`, and `session-live` if P3 reads it landed, each with its own hint and its error-recovery text on the cli surface.
- `viola list`'s whole-board strict-modes refusal (exit 21): the refusal's text and document only, no board row, joining the matrix and the security-control table.
- `viola plugin install`'s exit 21 refusal: its printed text and document as a CLI verb's refusal, its copy pinned by literal.
- The frame verbs' refusals (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`): not running, unwrapped name, strict modes and server verification told apart, in linear static output.
- Endpoint lost mid-call: the refusal comes after a verb may already have written to stderr (`viola wait`'s TTY-only `waiting: <name>`), and the hint stays the last stderr line.
- The server-verification case and its hint, pinned by literal: the plan runs the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only because macOS reports no peer pid; the table's row follows that, the macOS gap staying as recorded.
- The not-assertable carve-out for MCP, the plugin hooks and the channel: machine-to-machine, no UI surface, so codes there owe no output-discipline assertion; `viola mcp`'s refused home check (exit 1, nothing on stdout or stderr) and `hook` (exit 0, no body, no stderr) sit against the hint rule and each is named in the table or outside it by a reason.
- `viola verify`'s refused home tree: its refusal text beside the step lines of verify's linear output.
- No driver-facing hint on any row names `viola release`, per the scope and test-plan's matrix.

## Standing conventions
- S1 · each new CLI case runs inside `nextest-integration` / `nextest-e2e` locally and the per-OS `coverage` suite in CI, through `scripts/agent-run.*` `run`, with no new `suite` value and no separate a11y driver · `## 9. CI Integration` and `## 11. A11y Anti-Patterns` § CI
- S2 · a new output assertion uses assert_cmd, predicates, trycmd or the portable-pty outer PTY, stands in for SC 1.3.2, 1.4.1 and 3.3.1 only, and claims no WCAG conformance · `## 4. ARIA Patterns & Roles` and `## 11. A11y Anti-Patterns` § Strategy
- S3 · each new fixed CLI string is pinned by trycmd or by literal, so a copy change fails CI until the pin moves on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S4 · output-discipline invariants pass on all three OS legs, with `retries: 0`, no retry-once policy and `#[ignore]` banned · `## 10. SLO Invariants & A11y Budgets` and `## 11. A11y Anti-Patterns` § SLO
- S5 · no new test parses the rendered `claude` TUI screen or takes a screenshot as evidence · `## 11. A11y Anti-Patterns` § Universal

## Not needed
- The manual NVDA, VoiceOver and Orca passes and the manual keyboard pass: a founder-owned supplement this chunk does not run.
- Keystroke delivery and wheel-taking in the passthrough (focus, mouse and resize sequences, F-W3, the atomic paste): the chunk changes no input path.
- The vt100 feed panic containment and its Windows byte reading, and the outer-PTY witness of dim on stale rows: owed to "Windows-only live measurements" and "Linux and macOS parity".
- The time-bound inventory under Timeout extensions: the chunk adds and moves no time bound.
- The 0.2.0 frontend's a11y tooling, a browser leg, contrast and ARIA conformance: deferred, no page rendered.
- The triggers marked not triggered (wcag-mapping, regulated-compliance, cognitive-accessibility, motion-sensitive, multi-language, target-size): nothing in this chunk raises one.
