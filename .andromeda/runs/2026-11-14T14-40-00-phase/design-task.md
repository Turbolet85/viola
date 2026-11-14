@task_prose part=chunk
A REMOVAL chunk: the budget gate leaves viola whole. viola reads no rate limits and gates nothing on budget, so an automated send is never refused on budget.
Leaving: the wrapper's gate, the budget-paused send refusal, the budget step of the start order, viola release --budget, the budget.json state, the board's budget header and cells, the budget fields in list --json and MCP list, the budget hints and verb lines, the config threshold keys, the budget log fields, the rate-limits reading and its ledger row, and the Path 6 suite.
Staying: the driver release guard, the plain release, the status-line pass-through (that is the next entry's work), and the readers of old disk state. Exit code 11 is retired and never renumbered or reused.
P3 re-measures each coordinate, how the config reader treats an unknown key, how old logs and snapshots replay with budget fields, and who else calls Reading<T>.
@task_file n=1 path=src/run/budget.rs mark=edits
what: the wrapper's budget gate: threshold reading, the decide step, the release --budget lift and the run.budget_eval span over start, send and turn-ended (the whole file is deleted)
@task_file n=2 path=src/run/send/budget.rs mark=edits
what: the send context's budget-paused check (the whole file is deleted)
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's send handling and its refusal order — edits: the budget-paused rung of the refusal order
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot within the send refusal order — edits: the budget-paused rung
@task_file n=5 path=src/run/wheel.rs mark=edits
what: the wrapper's wheel (pause and release control transfer) — edits: its budget references
@task_file n=6 path=src/run/wait.rs mark=edits
what: the wrapper's wait handling — edits: its budget references
@task_file n=7 path=src/run/mod.rs mark=edits
what: the wrapper run's start order and first state — edits: the budget-gate start record, the budget_paused and budget_override_until in first(), the turn-ended re-evaluation
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run verb — edits: its budget references
@task_file n=9 path=src/cmd/revive.rs mark=edits
what: the viola revive verb — edits: its budget references
@task_file n=10 path=crates/viola-state/src/budget.rs mark=edits
what: the budget state file's writer and readers, write_budget, read_budget and read_budget_stamped (the whole file is deleted)
@task_file n=11 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: pub mod budget
@task_file n=12 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the budget_paused and budget_override_until fields
@task_file n=13 path=crates/viola-state/src/replay.rs mark=edits
what: the event-log replay that recovers the snapshot — edits: its recovery of budget_paused and budget_override_until
@task_file n=14 path=crates/viola-state/src/board.rs mark=edits
what: the board's data model behind viola list — edits: the board item's budget_paused and the board's budget reading
@task_file n=15 path=crates/viola-core/src/lib.rs mark=edits
what: the core event and refusal types — edits: EventKind::BudgetGate, RefusalReason::BudgetPaused, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=16 path=src/cmd/release.rs mark=edits
what: the viola release verb and its channel release param — edits: the --budget option, the budget param, the human lines write_budget_lifted and write_budget_open
@task_file n=17 path=src/cmd/send.rs mark=edits
what: the viola send verb's refusal-to-exit-code mapping — edits: the budget-paused refusal's exit 11
@task_file n=18 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tools' result payloads — edits: the list payload's budget field
@task_file n=19 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tool list and its descriptions — edits: the list tool description naming the budget
@task_file n=20 path=src/human/board.rs mark=edits
what: the viola list board rendered for the human — edits: its budget header and cells
@task_file n=21 path=src/human/board/bay.rs mark=edits
what: the board's BAY header line — edits: its budget figures
@task_file n=22 path=src/human/board/cells.rs mark=edits
what: the board's strip cells — edits: the budget cells
@task_file n=23 path=src/human/board/rows.rs mark=edits
what: the board's strip rows — edits: the budget cells
@task_file n=24 path=src/human/hints.rs mark=edits
what: the human hint lines under refusals — edits: the budget hints
@task_file n=25 path=src/human/verbs.rs mark=edits
what: the human verb output lines — edits: the budget verb lines
@task_file n=26 path=src/human.rs mark=edits
what: the human output module — edits: its budget references
@task_file n=27 path=src/cmd/mod.rs mark=edits
what: the CLI verb definitions and help text — edits: the list verb's help text naming the budget
@task_file n=28 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the Claude status-line payload reader — edits: the rate_limits parse (reading)
@task_file n=29 path=src/cmd/hook/statusline.rs mark=edits
what: the hook statusline arm — edits: its budget.json write and its hook-decision budget_written field
@task_file n=30 path=src/cmd/hook.rs mark=edits
what: the hook verb's decision line — edits: the budget_written field
@task_file n=31 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude ledger of relied rows and recorded fixture names — edits: the statusline-rate-limits row, whole_rate_limits, the STATUSLINE_RATE_LIMITS record name, the relied count
@task_file n=32 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that replays recorded Claude behaviour in CI — edits: the rate-limits replay after the first Stop
@task_file n=33 path=schemas/diag-line.v1.json mark=edits
what: the diag log-line schema — edits: its budget fields
@task_file n=34 path=tests/cli_budget.rs mark=edits
what: the CLI budget test suite (the whole file is deleted)
@task_file n=35 path=crates/viola-e2e/tests/path6_budget_harness.rs mark=edits
what: the Path 6 budget end-to-end suite (the whole file is deleted)
@task_file n=36 path=tests/cli_list.rs mark=edits
what: the viola list board and --json tests — edits: the budget cases
@task_file n=37 path=tests/cli_machine_contract.rs mark=edits
what: the CLI machine contract tests (exit codes, --json documents) — edits: the budget cases
@task_file n=38 path=tests/hook_statusline.rs mark=edits
what: the hook statusline arm tests — edits: the budget cases
@task_file n=39 path=crates/viola-channel/src/client.rs mark=edits
what: the channel client — edits: the tests that use budget-paused as a sample refusal
@task_file n=40 path=.config/nextest.toml mark=edits
what: the test-runner configuration — edits: the Path 6 entry
@task_item n=1 part=needs
The send refusal order loses its budget-paused rung, and budget-paused leaves the closed list of refusal words. An automated send is never refused on budget.
@task_item n=2 part=needs
Exit code 11 is retired: no refusal maps to it, it is never renumbered or reused, and the codes 10, 12, 13 and 14 keep their numbers and meanings.
@task_item n=3 part=needs
viola release --budget leaves, along with the channel release budget param, the override time and the lifted and open human lines.
@task_item n=4 part=needs
The plain release (the wheel handed back) stays, and so does the driver release guard. A release whose from is a string is refused -32602 release-from-driver; a from of any other type is refused -32602 with data null. viola release stays a human verb.
@task_item n=5 part=needs
The viola list board loses its budget reading: the BAY header's window figures, the budget cells, the expired-window display, and the human holder under a budget pause. The minimum-fuel budget-paused strip state leaves with them.
@task_item n=6 part=needs
The list payload loses its budget field, both under --json and in the MCP list tool.
@task_item n=7 part=needs
The budget-paused refusal hint and the budget verb lines leave the human output. The hints that remain keep their fixed refusal-line form.
@task_item n=8 part=needs
Neither the list verb's help text nor the MCP list tool description names the budget any more. The recorded MCP tools-list fixture that carries that description follows, with how it is re-recorded measured at P3.
@task_item n=9 part=needs
The hook statusline arm no longer parses rate_limits or writes budget.json, and its hook-decision line loses budget_written. The arm keeps only its pass-through to the user's command until the next entry.
@task_item n=10 part=needs
Budget log lines and fields leave: the budget-gate event kind (paused, window, override_until), the run.budget_eval span, the diag-line schema's budget fields, and the parse-rejected config-json lines for the thresholds.
@task_item n=11 part=needs
The config.json five_hour and seven_day threshold keys (Percent, defaults 90 and 85) leave. config.json and dialog_hold_s stay. What a config.json still carrying either key reads after the removal is measured at P3 against how the config reader treats an unknown key.
@task_item n=12 part=needs
The snapshot's and the board item's budget_paused and budget_override_until leave, and so does the budget.json state. A budget.json left in a home is read by nothing.
@task_item n=13 part=needs
Readers of old disk state keep working. An event log that holds budget-gate lines, or a snapshot that holds the budget fields, still replays and still reaches the board: readers skip and count unknown kinds and fields and never deny unknown fields. Replay and the board are measured at P3.
@task_item n=14 part=needs
The core budget types leave: BudgetGate, BudgetPaused, BudgetWindowKind, Percent, BudgetWindow and BudgetReading. Reading<T> stays only if a non-budget caller remains, which P3 measures.
@task_item n=15 part=needs
The statusline-rate-limits ledger row leaves, together with its recorded rate-limits fixtures and the fake agent's rate-limits replay, and the relied count falls from 24 to 23. P3 settles where the boundary falls against the next entry; a reading that moves the row to the next entry is a P4 fork.
@task_item n=16 part=needs
A test that uses budget-paused only as a sample refusal word keeps its case with another closed refusal word. P3 names each such case.
@task_item n=17 part=needs
A guard test's one-off remove-the-guard control runs through gate.py control under a kill deadline. Its case is the test's name alone, and it holds only when that case ran and FAILED. No mutation run, local or dispatched.
@task_item n=18 part=needs
Every statement in the plan about how a tool, a crate or the CLI behaves is measured at P3 and cited, or marked unmeasurable.
@task_item n=19 part=needs
These are not touched: the pinned product binary, the live home ~/.viola/ and its budget.json. There is no install, no re-pin, no live claude start (a print-and-exit claude --version is allowed) and no live desktop step. Each of these is a named operator step that waits.
@task_item n=20 part=needs
The status-line interception stays as it is: the pass-through and its shell-out, the settings override, statusline-source.json, the snapshot's statusline_command, the statusline-override and statusline-shell ledger rows, the default status-line fixture, and the fake agent's status-line modes.
@task_item n=21 part=needs
The phase edits no master and no harness text. The budget rows in the masters (census F-08) retire at this chunk's wrap.
@task_item n=22 part=needs
The capability v2-04 is claimed at P5 only if the plan proves it whole: no budget reading, file, paused state, release option, board field, ledger row or log field remains, and exit 11 is retired, not reused.
@task_standing n=1
convention: Every human line the chunk keeps or rewords stays ASCII-only, has its control characters escaped, is never wrapped, and carries no internals.
home: surface-cli
@task_standing n=2
convention: Every verb keeps data on stdout and messages on stderr, and its --json result stays one clean stdout document with no hint line.
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
The web-page surface: viola has no web or graphical frontend, and the chunk adds none.
@task_item n=2 part=not_needed
Rendered-page tokens (radius, depth and spacing tokens): viola renders no page.
@task_item n=3 part=not_needed
The viola verify output and its refusals: the chunk does not reach verify.
@task_item n=4 part=not_needed
The viola link and unlink output and their --json ok documents: the chunk does not reach linking.
@task_item n=5 part=not_needed
The self-validation protocol's squint, swap and domain checks: these are design-review tests, not something this removal builds or proves.
