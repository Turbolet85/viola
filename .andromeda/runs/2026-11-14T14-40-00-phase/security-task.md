@task_prose part=chunk
A REMOVAL chunk: the budget gate leaves the product whole; viola reads no rate limits and refuses or pauses nothing on budget.
Leaving: the wrapper's gate and its start step, the turn-ended re-check, the budget-paused send refusal, the budget state file and its writer, the two config threshold keys, the budget core types and event kind, the release budget option, the board and list budget fields, the rate-limits parse, ledger row and fixtures, the Path 6 suite and budget tests, and the budget fields in the diagnostics schema.
Staying: the driver release guard (release-from-driver), the plain release, exit code 11 retired and never reused, the status-line pass-through (the next entry's), config.json with its other keys, and old event logs and snapshots that still hold budget lines, which keep reading.
P3 measures how a leftover threshold key in config.json is read, how replay and the board read old budget lines, which callers Reading<T> still has, and where the rate-limits ledger row ends up. No live home write, no install, no live claude start, no mutation run.
@task_file n=1 path=src/run/budget.rs
what: the wrapper's budget gate: threshold read, decide, the release budget lift and the budget evaluation span over start, send and turn-ended. Deleted whole.
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget-paused check (Ctx::budget_paused). Deleted whole.
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's send handler and its refusal order — edits: the budget-paused rung of send's refusal order
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot — edits: its budget-paused references
@task_file n=5 path=src/run/mod.rs mark=edits
what: the wrapper's start order and its start records — edits: the budget step of the start order (the budget-gate record), budget_paused and budget_override_until in first()
@task_file n=6 path=src/run/wheel.rs mark=edits
what: the wrapper's wheel (who holds the input) — edits: its budget references and the turn-ended budget re-check
@task_file n=7 path=src/run/version_gate.rs mark=edits
what: viola run's version gate — edits: its budget references
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run verb — edits: its budget references
@task_file n=9 path=src/cmd/revive.rs mark=edits
what: the viola revive verb — edits: its budget references
@task_file n=10 path=src/cmd/release.rs mark=edits
what: the CLI viola release verb and the channel release budget param — edits: the --budget option, the human lines write_budget_lifted and write_budget_open
@task_file n=11 path=src/cmd/send.rs mark=edits
what: the CLI viola send verb and its refusal-to-exit-code mapping — edits: the budget-paused refusal's exit 11
@task_file n=12 path=src/cmd/mod.rs mark=edits
what: the CLI verb table and help text — edits: the list verb's help text naming the budget
@task_file n=13 path=src/cmd/hook/statusline.rs mark=edits
what: the viola hook statusline arm — edits: its budget.json write and the hook-decision budget_written field
@task_file n=14 path=src/cmd/hook.rs mark=edits
what: viola hook's event dispatch and hook-decision log line — edits: the budget_written field
@task_file n=15 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the statusline payload parser for Claude's hook stdin — edits: the rate_limits parse (reading)
@task_file n=16 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the agent ledger of relied upstream rows and their recorded fixtures — edits: the statusline-rate-limits row, whole_rate_limits, the STATUSLINE_RATE_LIMITS record fixture name
@task_file n=17 path=src/bin/viola-fake-agent.rs mark=edits
what: the test-only fake agent and its replay modes — edits: the rate-limits replay after the first Stop
@task_file n=18 path=crates/viola-state/src/budget.rs
what: the budget state file's writer and readers (write_budget, read_budget, read_budget_stamped). Deleted whole.
@task_file n=19 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: pub mod budget
@task_file n=20 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot written by the wrapper — edits: the budget_paused and budget_override_until fields
@task_file n=21 path=crates/viola-state/src/replay.rs mark=edits
what: event-log replay that recovers snapshot state — edits: its recovery of budget_paused and budget_override_until
@task_file n=22 path=crates/viola-state/src/board.rs mark=edits
what: the board item and the board's reading of the home — edits: the board item's budget_paused and the board's budget reading
@task_file n=23 path=crates/viola-core/src/lib.rs mark=edits
what: the core crate's shared closed types and validators — edits: EventKind::BudgetGate, RefusalReason::BudgetPaused, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=24 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results — edits: the list payload's budget field
@task_file n=25 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tool list and its descriptions — edits: the list tool's description naming the budget
@task_file n=26 path=src/human/board.rs mark=edits
what: the human-mode board printed by viola list — edits: its budget header
@task_file n=27 path=src/human/board/cells.rs mark=edits
what: the human-mode board's cells — edits: the budget cells
@task_file n=28 path=src/human/hints.rs mark=edits
what: the human-mode stderr hints — edits: the budget hints
@task_file n=29 path=src/human/verbs.rs mark=edits
what: the human-mode verb lines — edits: the budget verb lines
@task_file n=30 path=tests/cli_budget.rs
what: the CLI budget gate test suite. Deleted whole.
@task_file n=31 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 end-to-end budget suite. Deleted whole.
@task_file n=32 path=tests/hook_statusline.rs mark=edits
what: the hook statusline arm's tests — edits: its budget cases
@task_file n=33 path=tests/cli_controls_not_disableable.rs mark=edits
what: the check that no config, env var or flag switches off a control — edits: its budget cases
@task_file n=34 path=tests/cli_machine_contract.rs mark=edits
what: the CLI machine contract tests (exit codes, --json documents) — edits: its budget cases
@task_file n=35 path=crates/viola-channel/src/client.rs mark=edits
what: the channel client — edits: its tests that use budget-paused as a sample refusal
@task_file n=36 path=.config/nextest.toml mark=edits
what: the test runner's configuration — edits: the Path 6 entry
@task_file n=37 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics log line schema — edits: its budget fields
@task_file n=38 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the schema of recorded Claude statusline fixtures — edits: its rate-limits shape
@task_file n=39 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: a recorded Claude statusline fixture carrying rate limits, one of the ledger row's fixtures. Deleted whole.
@task_file n=40 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: the recorded MCP tools-list fixture, taken through the test-only tap — edits: the list tool's description naming the budget
@task_item n=1 part=needs
An automated send is never refused on budget: the budget-paused word leaves send's refusal order and the closed refusal set, and the other refusals keep their order and words.
@task_item n=2 part=needs
CLI exit codes: exit 11 is retired and not renumbered. No other refusal gets 11, and every remaining code keeps its number.
@task_item n=3 part=needs
Readers of old disk state stay tolerant. An event log with budget-gate lines and a snapshot with budget_paused or budget_override_until fields still read: unknown kinds and fields are skipped and counted, never refused (no deny_unknown_fields), because mixed binary versions are real. The replayed snapshot stays inert and the board still reads.
@task_item n=4 part=needs
Clients read reply refusals only as closed code words. Once budget-paused leaves the closed set, an older wrapper that still answers budget-paused must reach the client as an unknown code, handled the way the client handles any code outside its closed values, and never as text it acts on.
@task_item n=5 part=needs
The budget state file loses its only writer (hook statusline) and every reader. Its single-writer rule, its fields (window, used_percentage, resets_at) and the failed-read and fail-open handling of that untrusted file all go with it. A budget.json left in a home is read by nothing, and the live one is not touched.
@task_item n=6 part=needs
The hook statusline arm keeps only its pass-through. It stays silent and always exits 0, fails open to the human, checks the instance and strict modes before any read, and parses its stdin tolerantly. A payload that still carries rate_limits is received as content and ignored. Its hook-decision log line loses the budget_written field.
@task_item n=7 part=needs
Config: the five_hour and seven_day keys leave config.json along with their parse-rejected lines. The tolerant config parse has to handle a config.json that still carries either key (P3 measures this). dialog_hold_s and the other keys stay, and no config key can switch off a control.
@task_item n=8 part=needs
The driver release guard stays. A channel release with a string from is refused -32602 release-from-driver, and a from of another type is refused -32602 with null data. The plain release (the wheel handed back) stays. The release params lose the budget param, and the closed-type check on release params still refuses anything else.
@task_item n=9 part=needs
The CLI viola release verb loses --budget and stays a human verb. The from value of pause and release is still taken from VIOLA_NAME.
@task_item n=10 part=needs
The list payload under --json and the MCP list result lose the budget field. The MCP list tool description stops naming the budget. The tool list stays an affordance only, never enforcement.
@task_item n=11 part=needs
The human board loses its budget header and cells, and the hints and verb lines lose their budget lines. Human output still escapes control characters. The board still refuses the whole home under the read rule.
@task_item n=12 part=needs
The home-write-rule checkers: budget read, mcp start and plugin install check the home. With the budget read gone, mcp start and plugin install keep their strict-modes checks, and the start order still checks the home tree before any log opens.
@task_item n=13 part=needs
The census of bounded product readers loses the budget file reader. Every reader that remains is still bounded or named, and reads stay capped at the shared frame cap.
@task_item n=14 part=needs
The core crate's budget types and the budget-gate event kind leave. Reading<T> stays only if a caller outside the budget remains. The shared validators stay in the core crate, used the same way by the CLI, MCP and the wrapper.
@task_item n=15 part=needs
The diagnostics log line schema loses its budget fields and the budget evaluation span leaves. Lines that remain still follow the never-log floor: no user content outside diagnostics, no paths or ids in refusals.
@task_item n=16 part=needs
The list of product error writers loses the release budget human lines (budget lifted, budget open). Every remaining error writer is still named, and error messages still interpolate nothing.
@task_item n=17 part=needs
The statusline-rate-limits ledger row, its recorded fixtures and the fake agent's rate-limits replay leave, and the relied row count drops from 24 to 23 (P3 settles where the row goes). The fake agent's remaining seams stay under the fake-agent feature and are compiled out of release.
@task_item n=18 part=needs
Parser fuzz scope: the rate_limits parse leaves the parser surfaces. The hook stdin fuzz seed that carries rate_limits stays as content the arm still receives.
@task_item n=19 part=needs
Re-recording the MCP tools-list fixture goes only through the test-only tap and is scrubbed before commit. If it cannot run without a live claude start, it is a named operator step that waits.
@task_item n=20 part=needs
Tests that leave or change: the budget suites are deleted. A test that used budget-paused only as a sample refusal keeps its case with another closed refusal word. The controls-not-disableable check loses its budget case and still covers the remaining controls.
@task_item n=21 part=needs
The release-from-driver guard's test is kept and proven by a one-off remove-the-guard control, not by a mutation run. The control holds when that case ran and failed.
@task_item n=22 part=needs
Nothing is written under the live viola home: no install, no re-pin, no live claude start without a cap. Tests run against their own homes.
@task_standing n=1
convention: every test runs against its own home, never the live home, and removes only the directories it created
home: secret-management
@task_standing n=2
convention: every recorded fixture added or re-recorded is scrubbed of paths, usernames and emails and reviewed before commit
home: data-protection
@task_standing n=3
convention: every test seam stays under the fake-agent feature and is compiled out of the release build
home: input-validation
@task_item n=1 part=not_needed
Crate version floors, advisory ignores and exact dependency pins: the chunk adds, removes or bumps no dependency.
@task_item n=2 part=not_needed
GitHub workflow action SHA pinning, workflow permissions and checkout credentials: the chunk edits no workflow.
@task_item n=3 part=not_needed
Socket bind, socket file chmod, socket-directory checks and peer euid checks: the chunk does not touch the channel endpoint.
@task_item n=4 part=not_needed
viola verify's probe runs, probe directories, real-CLI transcripts and screen signature reads: the chunk does not touch verify.
@task_item n=5 part=not_needed
Plugin install rounds, plugin file rewrites and the marketplace tree: the chunk does not touch plugin install.
@task_item n=6 part=not_needed
link and unlink, their channel methods and their event log lines: nothing in the chunk reaches them.
@task_item n=7 part=not_needed
The Hyprland recorder and its reply parsing: nothing in the chunk reaches them.
@task_item n=8 part=not_needed
The CLAUDE credential strip and the child's hermetic environment: the chunk does not change the child's environment.
@task_item n=9 part=not_needed
The absences (hosting, network listeners, outbound calls, model API calls, databases, webhooks, accounts): the chunk adds no such surface.
