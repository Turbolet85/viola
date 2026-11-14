@task_prose part=chunk
Removes the budget gate whole: viola reads no rate limits and gates nothing on budget; every surface of the gate leaves the product and its tests.
Leaves: the wrapper's gate and its start, send and turn-ended evaluation, the budget-paused refusal, the budget-gate event, budget.json and its state module, the snapshot and board budget fields, the config thresholds, release --budget, the list budget field, the rate-limits reading in the statusline arm, the statusline-rate-limits ledger row, the Path 6 suite.
Keeps: the driver release guard, the plain release, exit code 11 retired and never renumbered or reused, the statusline pass-through (the next entry's), config.json's other keys, readers of old disk state.
Re-measures at P3: a config.json still carrying the threshold keys, old logs and snapshots read by replay and the board, Reading<T>'s remaining callers, the ledger row boundary, whether the MCP tools-list fixture can be re-recorded without a live claude.
@task_file n=1 path=src/run/budget.rs mark=edits
what: the wrapper's budget gate — threshold read, decide, the release --budget lift and the run.budget_eval span over start, send and turn-ended; removed whole
@task_file n=2 path=src/run/send/budget.rs mark=edits
what: the send path's budget-paused check (Ctx::budget_paused); removed whole
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's channel send method and its refusal order — edits: the budget-paused rung
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot of the wrapper's send method — edits: the budget-paused rung
@task_file n=5 path=src/run/send/test_support.rs mark=edits
what: test support for the wrapper's send method — edits: the budget-paused fixtures
@task_file n=6 path=src/run/wheel.rs mark=edits
what: the wheel decided in wrapper memory and its release — edits: budget references
@task_file n=7 path=src/run/wait.rs mark=edits
what: the wrapper's channel wait method and its wake kinds — edits: budget references
@task_file n=8 path=src/run/dialog.rs mark=edits
what: the wrapper's dialog hold and hook.dialog handling — edits: budget references
@task_file n=9 path=src/run/version_gate.rs mark=edits
what: the wrapper's capability-ledger version gate — edits: budget references
@task_file n=10 path=src/run/mod.rs mark=edits
what: the wrapper's start order and its first state — edits: budget_paused and budget_override_until in first(), the budget step of the start order, the turn-ended re-evaluation
@task_file n=11 path=src/cmd/run.rs mark=edits
what: the viola run verb's entry — edits: budget references
@task_file n=12 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, which resumes through the start path — edits: budget references
@task_file n=13 path=crates/viola-state/src/budget.rs mark=edits
what: the budget.json state — write_budget, read_budget, read_budget_stamped; removed whole
@task_file n=14 path=crates/viola-state/src/lib.rs mark=edits
what: the viola-state crate's module list — edits: pub mod budget
@task_file n=15 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot's fields — edits: budget_paused and budget_override_until
@task_file n=16 path=crates/viola-state/src/replay.rs mark=edits
what: snapshot replay, recovering log-derived fields — edits: recovery of budget_paused and budget_override_until
@task_file n=17 path=crates/viola-state/src/board.rs mark=edits
what: the board read model behind CLI list and MCP list — edits: the item's budget_paused and the board's budget reading
@task_file n=18 path=crates/viola-core/src/lib.rs mark=edits
what: the core normalised types — edits: EventKind::BudgetGate, RefusalReason::BudgetPaused, BudgetWindowKind, Percent, BudgetWindow, BudgetReading and their tests
@task_file n=19 path=src/cmd/release.rs mark=edits
what: the viola release verb — edits: the --budget option and the human lines write_budget_lifted and write_budget_open
@task_file n=20 path=src/cmd/send.rs mark=edits
what: the viola send verb and its exit codes — edits: the budget-paused refusal's exit 11
@task_file n=21 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results, the list payload — edits: the list payload's budget field
@task_file n=22 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tools and their descriptions — edits: the list tool description naming the budget
@task_file n=23 path=src/human/board.rs mark=edits
what: the human board of viola list, its header, rows and cells — edits: the budget header and budget cells
@task_file n=24 path=src/human/hints.rs mark=edits
what: the human hint lines — edits: the budget hints
@task_file n=25 path=src/human/verbs.rs mark=edits
what: the human verb lines — edits: the budget verb lines
@task_file n=26 path=src/human.rs mark=edits
what: the one human writer in the root bin — edits: budget lines
@task_file n=27 path=src/cmd/mod.rs mark=edits
what: the CLI verbs and their help text — edits: the list verb's help text naming the budget
@task_file n=28 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the Claude status-line payload shape — edits: the rate_limits parse (reading)
@task_file n=29 path=src/cmd/hook/statusline.rs mark=edits
what: the hook statusline arm — edits: the budget.json write
@task_file n=30 path=src/cmd/hook.rs mark=edits
what: the viola hook command and its hook-decision lines — edits: the hook-decision budget_written field
@task_file n=31 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger's rows — edits: LedgerRow::StatusLineRateLimits, whole_rate_limits, the STATUSLINE_RATE_LIMITS record fixture name, the relied row count
@task_file n=32 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent and its status-line modes — edits: the rate-limits replay after the first Stop
@task_file n=33 path=tests/cli_budget.rs mark=edits
what: the CLI budget test suite; removed whole
@task_file n=34 path=crates/viola-e2e/tests/path6_budget_harness.rs mark=edits
what: the Path 6 end-to-end budget suite; removed whole
@task_file n=35 path=.config/nextest.toml mark=edits
what: the nextest test-group configuration — edits: the Path 6 entry
@task_file n=36 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics line schema — edits: the budget fields
@task_file n=37 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the recorded Claude status-line fixture schema — edits: the rate-limits shape
@task_file n=38 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json mark=edits
what: a recorded Claude status-line fixture carrying rate limits, held by the statusline-rate-limits ledger row; removed whole, with its 2.1.287 twin
@task_file n=39 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: the recorded claude MCP tools/list exchange the MCP tools list is held to — edits: the list tool description naming the budget
@task_file n=40 path=scripts/profraw-census.sh mark=edits
what: the dev-host profraw census witness — edits: its budget mention
@task_item n=1 part=needs
The wrapper's budget gate leaves: no rate-limit thresholds read, no decision, no budget step in the start order, no re-evaluation at turn end, no run.budget_eval span; an automated send is never refused on budget.
@task_item n=2 part=needs
Send's refusal order loses its budget-paused rung; the remaining rungs keep their order and only the first refusal is returned; the channel send method's budget-paused result is gone.
@task_item n=3 part=needs
RefusalReason loses budget-paused; refusal details stay a closed set; exit code 11 is retired, no other refusal takes 11 and the remaining CLI exit codes keep their numbers.
@task_item n=4 part=needs
The budget-gate event kind leaves the normalised event kinds, the per-kind event data and the wrapper's event appends; the wrapper appends no budget-gate line; new event kinds are added only in viola-core.
@task_item n=5 part=needs
Old disk state keeps reading: an event log holding budget-gate lines and a snapshot holding budget_paused or budget_override_until are read by replay and the board, the readers skipping and counting unknown kinds and fields, never denying unknown fields; mixed binary versions are real; measured at P3.
@task_item n=6 part=needs
The instance snapshot and its replay lose budget_paused and budget_override_until; replay still recovers only log-derived fields and writes no file.
@task_item n=7 part=needs
budget.json leaves: no writer (the statusline hook was its single writer), no reader, no shared budget reading, its envelope fields gone; a budget.json left in a home is read by nothing; the live home is never written.
@task_item n=8 part=needs
config.json's five_hour and seven_day keys leave with their parse-rejected config-json lines; dialog_hold_s and the other keys stay; the config keys stay equal to the config schema; a config.json still carrying either key reads by the config reader's unknown-key handling, measured at P3.
@task_item n=9 part=needs
release --budget and the channel release budget param leave, with budget release durability and per-instance budget release; the plain release stays; the driver release guard stays — a string from is refused -32602 release-from-driver, a from of another type -32602 with data null; release stays a CLI-only human verb.
@task_item n=10 part=needs
The list payload's budget field and the list item's budget_paused leave from CLI list --json and MCP list; the human board loses its budget header and cells; the list verb's help and the MCP list tool description stop naming the budget.
@task_item n=11 part=needs
The MCP tools/list is held to a recorded claude exchange: the recorded tools-list fixture must agree with the changed list tool description; whether its record can run with no live claude is measured at P3, and a live record is a named operator step that waits.
@task_item n=12 part=needs
The hook statusline arm loses the rate_limits parse, the budget.json write and the hook-decision budget_written field, keeping only its pass-through, stdout and exit status until the next entry.
@task_item n=13 part=needs
The capability ledger loses the statusline-rate-limits row, its record fixture name and its recorded fixtures; the relied row count goes 24 to 23; a CLI version is still verified only when every relied row passes; P3 settles the row's boundary against the next entry, a move being a P4 fork.
@task_item n=14 part=needs
The fake agent's rate-limits replay after the first Stop leaves; its other status-line modes stay; CI runs against the fake agent with no live claude.
@task_item n=15 part=needs
The core budget types leave (BudgetWindowKind, Percent, BudgetWindow, BudgetReading); Reading<T> stays only if a non-budget user remains, measured at P3; tolerant resets_at parsing goes with its last user.
@task_item n=16 part=needs
The diagnostics line schema loses its budget fields; process logs stay codes-only and the diag schema contract test agrees with the schema.
@task_item n=17 part=needs
Tests: the CLI budget suite, the Path 6 suite and its nextest entry leave; budget cases leave the hook statusline, list, wheel, instance state, machine contract, controls, run start, chaos replay, contract, MCP tools, MCP handlers and state tests; a test using budget-paused only as a sample refusal word keeps its case with another closed refusal word.
@task_item n=18 part=needs
The removal leaves the build clean under warnings denied: no orphan module, no dead budget code, no lint ban weakened; coverage floors still hold over what remains.
@task_item n=19 part=needs
A guard test's one-off remove-the-guard control runs through the gate's control step under a kill deadline, its case the test's name alone, holding when that case ran and failed; no mutation run.
@task_item n=20 part=needs
The fuzz corpus statusline-epoch seed carrying rate_limits stays: external payloads are parsed tolerantly and the arm still receives that content.
@task_item n=21 part=needs
Capability v2-04: no budget reading, file, paused state, release option, board field, ledger row or log field remains; claimed only if the plan proves it whole.
@task_standing n=1
convention: each test runs in its own isolated viola home, cleaned by its owner
home: established-decisions
@task_standing n=2
convention: tests run against the fake agent in CI, never a live claude
home: ci-cd-approach
@task_standing n=3
convention: a recorded fixture is scrubbed and hygiene-walked, recorded only from a clean verify
home: occupied-resources
@task_standing n=4
convention: an OS-gated test names its reason
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The status-line interception's own rows — the per-session settings override, the statusLine source and its precedence, the statusline command shell-out and deadline, the statusline_command snapshot field, the statusline-override and statusline-shell ledger rows: the next entry's.
@task_item n=2 part=not_needed
The Hyprland compositor seam, its errors and window features: the chunk touches no compositor code.
@task_item n=3 part=not_needed
The PTY seam, the vt100 readiness gate, bracketed paste and typed-text normalisation: nothing about how a send is typed changes.
@task_item n=4 part=not_needed
Link and unlink recording, the link set and their channel methods: no link behaviour changes.
@task_item n=5 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json: the chunk installs nothing and changes no plugin content.
@task_item n=6 part=not_needed
Socket directory checks, endpoint naming and hashing, peer euid admission and server verification before a frame: untouched.
@task_item n=7 part=not_needed
Release tooling, dist, crates.io publishing and licensing rows: no release or packaging change.
@task_item n=8 part=not_needed
The child's environment strip of CLAUDE names and the child argv flags other than the status-line settings: unchanged.
@task_item n=9 part=not_needed
Absences of a database, broker, daemon, listening port, graphical or mobile front-end and hosting: the removal adds none.
@task_item n=10 part=not_needed
The question, permission and plan answer shapes and their hook contracts: no dialog answer changes.
