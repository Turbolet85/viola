@task_prose part=chunk
A removal chunk: the budget gate leaves viola whole, so viola reads no rate limits and refuses or pauses nothing on budget.
Leaving: the send gate and its budget-paused refusal, the budget step of the start order and the turn-ended re-evaluation, the config thresholds, the budget state file and the snapshot's paused fields, the release --budget option, the board's budget header and cells, and the list payload's budget field.
Also leaving: the status-line rate-limits reading and its budget.json write, the budget-gate event and the budget log fields, the statusline-rate-limits ledger row with its fixtures, and the Path 6 suite.
Staying: the driver release guard and the plain release; exit code 11 is retired and never reused; the status-line pass-through is the next entry's.
P3 measures how config.json reads a leftover threshold key, how replay and the board read old budget-gate lines and paused fields, and every claim about how a tool behaves.
@task_file n=1 path=src/run/budget.rs
what: the wrapper's budget gate (threshold reading, the decision, the release --budget lift, the budget_eval span) — deleted whole
@task_file n=2 path=src/run/send/budget.rs
what: send's budget-paused check — deleted whole
@task_file n=3 path=src/run/send.rs mark=edits
what: the send verb's refusal order inside the wrapper — edits: the budget-paused rung of the refusal order
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: send's slot step in the refusal order — edits: its budget-paused references
@task_file n=5 path=src/run/send/test_support.rs mark=edits
what: send's test support — edits: its budget-paused references
@task_file n=6 path=src/run/mod.rs mark=edits
what: the wrapper's run loop and start order — edits: budget_paused and budget_override_until in first(), the budget step of the start order
@task_file n=7 path=src/run/wheel.rs mark=edits
what: the wheel (who holds the turn, human or driver) — edits: its budget references
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run verb's start — edits: its budget references
@task_file n=9 path=src/cmd/revive.rs mark=edits
what: the viola revive verb's start — edits: its budget references
@task_file n=10 path=crates/viola-state/src/budget.rs
what: the budget state file reader and writer (write_budget, read_budget, read_budget_stamped) — deleted whole
@task_file n=11 path=crates/viola-state/src/lib.rs mark=edits
what: the state crate's module list — edits: pub mod budget
@task_file n=12 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the budget_paused and budget_override_until fields
@task_file n=13 path=crates/viola-state/src/replay.rs mark=edits
what: event-log replay into the snapshot — edits: the recovery of budget_paused and budget_override_until
@task_file n=14 path=crates/viola-state/src/board.rs mark=edits
what: the board reading behind viola list — edits: the board item's budget_paused, the board's budget reading
@task_file n=15 path=crates/viola-core/src/lib.rs mark=edits
what: the core types: event kinds, refusal reasons — edits: EventKind::BudgetGate, RefusalReason::BudgetPaused, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=16 path=src/cmd/release.rs mark=edits
what: the viola release verb (the human hands the wheel back) — edits: the --budget option, the channel release budget param, the human lines write_budget_lifted and write_budget_open
@task_file n=17 path=src/cmd/send.rs mark=edits
what: the viola send verb's refusal-to-exit-code mapping — edits: the budget-paused refusal's exit 11
@task_file n=18 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool result payloads — edits: the list payload's budget field
@task_file n=19 path=src/human/board/bay.rs mark=edits
what: the viola list board's header block — edits: the budget header
@task_file n=20 path=src/human/board/cells.rs mark=edits
what: the viola list board's cells — edits: the budget cells
@task_file n=21 path=src/human/board/rows.rs mark=edits
what: the viola list board's rows — edits: the budget cells
@task_file n=22 path=src/human/board.rs mark=edits
what: the viola list board renderer — edits: its budget header and cells
@task_file n=23 path=src/human/hints.rs mark=edits
what: the refusal hint lines, one per code — edits: the budget hints
@task_file n=24 path=src/human/verbs.rs mark=edits
what: the human verb result lines — edits: the budget verb lines
@task_file n=25 path=src/human.rs mark=edits
what: the human output module — edits: its budget references
@task_file n=26 path=src/cmd/mod.rs mark=edits
what: the viola --help verb table — edits: the list verb's help text naming the budget
@task_file n=27 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool descriptions — edits: the list tool description naming the budget
@task_file n=28 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the claude status-line payload reader — edits: the rate_limits parse (reading)
@task_file n=29 path=src/cmd/hook/statusline.rs mark=edits
what: the viola hook statusline arm — edits: the budget.json write, the hook-decision budget_written field
@task_file n=30 path=src/cmd/hook.rs mark=edits
what: the viola hook dispatcher — edits: the hook-decision budget_written field
@task_file n=31 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude ledger of relied rows — edits: the statusline-rate-limits row, whole_rate_limits, the STATUSLINE_RATE_LIMITS record fixture name
@task_file n=32 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent used by tests — edits: its rate-limits replay after the first Stop
@task_file n=33 path=tests/cli_budget.rs
what: the budget CLI test suite — deleted whole
@task_file n=34 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget end-to-end suite — deleted whole
@task_file n=35 path=.config/nextest.toml mark=edits
what: the nextest suite configuration — edits: the Path 6 entry
@task_file n=36 path=schemas/diag-line.v1.json mark=edits
what: the diag log line schema — edits: its budget fields
@task_file n=37 path=tests/cli_list.rs mark=edits
what: the viola list board and --json test suite — edits: its budget cases
@task_file n=38 path=tests/cli_machine_contract.rs mark=edits
what: the CLI machine contract tests (exit codes, --json documents) — edits: its budget cases
@task_file n=39 path=tests/hook_statusline.rs mark=edits
what: the viola hook statusline tests — edits: its budget cases
@task_file n=40 path=tests/cli_controls_not_disableable.rs mark=edits
what: the tests that no control can be switched off — edits: its budget cases
@task_item n=1 part=needs
An automated send is never refused on budget: the budget-paused refusal leaves the send refusal order, and the remaining refusals (input-not-ready, no-prompt-submitted and the rest) keep their order, words and hints.
@task_item n=2 part=needs
Exit code 11 is retired: no refusal maps to it, it is never renumbered or reused, and every remaining CLI exit code keeps its number.
@task_item n=3 part=needs
config.json loses its five_hour and seven_day keys; dialog_hold_s stays as it is. What a config.json that still carries either key reads (ignored silently, or a line) is measured at P3.
@task_item n=4 part=needs
The viola list board loses its budget header and cells, and the list payload (under --json and the MCP list tool) loses its budget field. The board's caption, order, colour and empty-board form stay otherwise as they are.
@task_item n=5 part=needs
The list verb's --help text and the MCP list tool description no longer name the budget, and the recorded MCP tools-list fixture follows them if it can be re-recorded without a live claude.
@task_item n=6 part=needs
viola release loses --budget and the channel release loses its budget param, along with the human budget-lifted and budget-open lines. The driver release guard stays (a string from refused with -32602 release-from-driver, another type with -32602 and null data). The plain release stays a human verb.
@task_item n=7 part=needs
The budget hints and verb lines leave. The one-hint-per-code rule and every remaining refusal's unable line and hint line stay, held by their literal pins.
@task_item n=8 part=needs
The hook statusline arm keeps only its pass-through: no rate_limits parse, no budget.json write, and no budget_written field on its hook-decision line. Its exit status and its byte-for-byte pass-through of the user's status-line output stay unchanged.
@task_item n=9 part=needs
The budget-gate event kind, the budget_eval span and every budget log field leave the event and diag lines and the diag-line schema. No log line names the budget after the removal.
@task_item n=10 part=needs
Old disk state keeps reading: an event log holding budget-gate lines and a snapshot holding the paused fields are read by skipping and counting unknown kinds and fields, never by refusing them. Measured at P3 on replay and the board.
@task_item n=11 part=needs
A newer client facing an older wrapper may still receive the budget-paused word. Peer replies stay screened to code words, so an unlisted code word stays visible and never becomes a bare or blank error.
@task_item n=12 part=needs
The wrapper's start order loses its budget step and budget-gate start record. The run and revive start refusal documents and lines name no budget step.
@task_item n=13 part=needs
The statusline-rate-limits ledger row, its recorded rate-limits fixtures and the fake agent's rate-limits replay leave, and the relied row count drops from 24 to 23. Whether the row leaves here or with the next entry is settled at P3, and a split is a P4 fork.
@task_item n=14 part=needs
A test that uses budget-paused only as a sample refusal word keeps its case with another closed refusal word, so the refusal format, error-body and fixture-hygiene checks keep the same coverage.
@task_item n=15 part=needs
The Path 6 suite is removed from the suite configuration, not skipped. The CI test gate still allows no skips, and no remaining case is marked ignored or retried.
@task_item n=16 part=needs
A guard test's one-off remove-the-guard control runs through the gate control under a kill deadline, with the case named by the test's name alone, and holds only when that case ran and failed. No mutation run, local or dispatched.
@task_item n=17 part=needs
No live claude start, no live desktop step, no install or re-pin, and no write under the live home. A budget.json left in a home is read by nothing, and removing the live one is a named operator step.
@task_item n=18 part=needs
The fixed CLI strings that change (help text, board header, hints, verb lines) still come from the design copy and are pinned by their literal in the existing suites.
@task_standing n=1
convention: every fixed CLI string a changed or replacement test asserts is pinned by its literal
home: cognitive-accessibility
@task_standing n=2
convention: a11y assertions stay inside the existing suites, with no separate a11y suite or driver
home: 1-a11y-scope-summary
@task_standing n=3
convention: every changed test is deterministic: zero flakiness, no retries, no skips
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: every error body a changed test witnesses has its writer and its witness
home: cognitive-accessibility
@task_item n=1 part=not_needed
The TUI passthrough contracts and decisions (human keystrokes, focus, mouse and resize sequences, vt100 feed panic, zero viola terminal bytes): the outer PTY is untouched by the removal.
@task_item n=2 part=not_needed
viola link and viola unlink contracts (result line, exit 20 and 21, --json output): no link verb changes.
@task_item n=3 part=not_needed
viola plugin install contracts: the plugin installer is untouched.
@task_item n=4 part=not_needed
viola verify output and the interrupted-verify contracts: verify is untouched.
@task_item n=5 part=not_needed
Socket-directory, strict-modes and server-verify refusals on hook calls, frame verbs and MCP frame tools, and the instance-unreachable details: no transport or permission check changes.
@task_item n=6 part=not_needed
Screen-reader targets and setups (Orca with GNOME Terminal or foot) and the manual keyboard pass: no manual pass is run or changed.
@task_item n=7 part=not_needed
The absences (web or graphical frontend, touch surface, internationalization, rendered page) and the N/A harness areas (contrast, focus management, WCAG mapping): nothing in the chunk reaches them.
@task_item n=8 part=not_needed
Log identity tags (process, service_name): the removal drops budget fields only, never the identity tags.
@task_item n=9 part=not_needed
Structured violation JSON rows and the a11y artifact decisions: the chunk adds no a11y report.
