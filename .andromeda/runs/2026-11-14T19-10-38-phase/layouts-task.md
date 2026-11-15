@task_prose part=chunk
Removes viola's status-line interception whole: the user's status-line command runs exactly as it does unwrapped, with no override, no settings flag for it, no recorded command, no hook statusline arm and no viola shell for any purpose.
The ledger loses its two interception rows (statusline-override, statusline-shell), so viola verify's relied rows go from 23 to 21 and Run B loses its status-line probe and capture.
Everything that existed only to build, test, record or log the interception leaves: the pure statusline module, the start's override and snapshot recording, the snapshot field, the fake agent's status-line modes, fixtures, schema, fuzz seeds, tests and log subjects.
Re-reads at P3 the no-tmux-or-shell decision (closed once the shell-out is gone), whether Run B passes a settings flag for anything else, and whether a verify --record run after the removal can reach 0 fail.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the hook statusline arm: instance check, replay read, the shell run under its deadline and the stdout pass-through (deleted)
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the hidden hook verb's dispatch over its arms and probe forms — edits: the statusline module declaration, the STATUSLINE_DEADLINE constant, the statusline WORD branch, the statusline_deadline_is_five_seconds test, capture_statusline and its four tests, the --shell arg doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs
what: the pure statusline module: the shell argv, the user command, the override document, the settings flag and its word (deleted)
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the agent crate's module list — edits: the pub mod statusline line
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the statusline module's proptest seeds (deleted)
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the verify ledger: its rows in order, their ids and claims, the relied rows, and the probe helpers — edits: LedgerRow StatusLineOverride and StatusLineShell in ALL, their ids and claims, ShellVerdict, StatusLineCapture, statusline_variants, the StatusLine variants helpers
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the ledger's test-case table — edits: the statusline rows
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs mark=edits
what: the ledger's check test cases — edits: the statusline rows
@task_file n=9 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs mark=edits
what: the ledger's verify-run test cases — edits: the statusline rows
@task_file n=10 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's typed runs and their probes — edits: Run B's status-line probe settings and capture
@task_file n=11 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's five runs — edits: Run B's status-line probe settings and capture
@task_file n=12 path=src/run/version_gate.rs mark=edits
what: the version gate's row groups over the ledger — edits: the statusline row group
@task_file n=13 path=src/cmd/run.rs mark=edits
what: the viola run start — edits: the statusline source read, the instance settings override written every start, the settings argument, the snapshot recording and their tests
@task_file n=14 path=src/run/mod.rs mark=edits
what: the run module's start constants and snapshot constructors — edits: SETTINGS_FLAG, the statusline_command literal
@task_file n=15 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot record — edits: the statusline_command field and its tests
@task_file n=16 path=crates/viola-state/src/board.rs mark=edits
what: the board's state read behind viola list — edits: the statusline_command literal, the "statusline" literal
@task_file n=17 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status-line replay mode (deleted)
@task_file n=18 path=src/bin/viola-fake-agent/test_cases.rs mark=edits
what: the fake agent's test-case table — edits: the statusline rows
@task_file n=19 path=src/bin/viola-fake-agent/test_cases_modes.rs mark=edits
what: the fake agent's mode test cases — edits: the statusline-echo mode and statusline receipt rows
@task_file n=20 path=src/bin/viola-fake-agent/test_cases_opts.rs mark=edits
what: the fake agent's option test cases — edits: the --framing StatusLine, --settings and --statusline-stdin rows
@task_file n=21 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded status-line stdin fixture (deleted)
@task_file n=22 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded status-line stdin fixture (deleted)
@task_file n=23 path=schemas/claude-statusline-fixture.v1.json
what: the status-line fixture schema (deleted)
@task_file n=24 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook-stdin fuzz corpus seed for the status line (deleted)
@task_file n=25 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook-stdin fuzz corpus seed for the status line (deleted)
@task_file n=26 path=tests/hook_statusline.rs
what: the hook statusline integration test file (deleted)
@task_file n=27 path=Cargo.toml mark=edits
what: the package manifest — edits: the hook_statusline [[test]] entry, the chrono dependency if nothing else uses it
@task_file n=28 path=.config/nextest.toml mark=edits
what: the nextest profile filters — edits: the hook_statusline filter term
@task_file n=29 path=tests/support/statusline.rs
what: the test support helpers for the status line (deleted)
@task_file n=30 path=schemas/diag-line.v1.json mark=edits
what: the diag log line schema — edits: the statusline-shell subject value
@task_file n=31 path=tests/tui_passthrough.rs mark=edits
what: the terminal passthrough tests — edits: passthrough_with_a_statusline_source_adds_no_viola_bytes
@task_item n=1 part=needs
The viola verify ledger: two rows leave (statusline-override, statusline-shell), the ledger order closes up without them, and the relied rows go from 23 to 21; no ledger row is added.
@task_item n=2 part=needs
viola verify's printed output: its step lines, failing rows, summary line and counts no longer carry a status-line row; the summary's relied count reads 21.
@task_item n=3 part=needs
viola verify --record: it still writes only at 0 fail; whether a run after this removal reaches 0 fail on this account is measured at P3 without a live start or marked unmeasurable; the box-only fixture stays a carry if the operator's cap does not come.
@task_item n=4 part=needs
viola verify's five runs stay; Run B keeps its other pastes, probes and its agents read, losing only the status-line settings probe and its capture.
@task_item n=5 part=needs
The hidden hook verb: the statusline arm and its capture and shell probe form leave; every other hook arm, the dialog arms, and the capture and answers probe forms of the other events stay unchanged.
@task_item n=6 part=needs
viola run and viola revive starts: no status-line source read, no instance settings override written, no settings argument passed for the status line, no status-line command recorded; the passed start output and start refusals otherwise unchanged.
@task_item n=7 part=needs
The instance snapshot loses its statusline_command field; viola list's board and its JSON document read the snapshot without it, and old snapshots holding the field keep reading (unknown fields skipped and counted).
@task_item n=8 part=needs
viola runs no shell for any purpose: the shell-out leaves, closing the no-tmux-or-shell dependency decision.
@task_item n=9 part=needs
No new settings, flag, environment variable, ledger row or verb; the change is removal only, and no help or usage line gains anything.
@task_item n=10 part=needs
Help output: any status-line wording in viola --help, per-verb help and viola verify --help leaves with the interception; the hidden hook verb stays out of the help verb groups.
@task_item n=11 part=needs
Log subjects: the statusline-shell diag subject, the statusline span area, the contract-hook-statusline subject tag and the shell-spawn process lines leave; no log line is added.
@task_item n=12 part=needs
The fake agent: the status-line framing replay, settings and stdin options, the statusline-echo mode and the statusline receipt kind leave; the CI proof of the removal runs through the fake agent and the code's own tests, never a live claude start.
@task_item n=13 part=needs
Tests that only spelled the interception are rewritten to test what stays; tests of the interception leave; a test that must spell an old word keeps it only where the acceptance probe over identifiers allows, old shapes living in fixture files outside the probed paths.
@task_item n=14 part=needs
A guard test's one-off remove-the-guard control is run through the gate's control step under a kill deadline, its case the test name alone; holds means that case ran and failed.
@task_item n=15 part=needs
The own-format version of the snapshot and diag line stays as it stands; whether this removal keeps v at 1 is the founder's fork with measured compatibility facts, not decided here.
@task_item n=16 part=needs
The retired-words fixture stays; whether the removed words join it is decided by its rule, read at P3.
@task_item n=17 part=needs
The user's status-line command and their settings file are never read, written or run; the live home and the pinned binary are not touched.
@task_standing n=1
convention: each new or rewritten test of verify output asserts the human output as a contract, plain ASCII, through the fake agent
home: surface-cli
@task_standing n=2
convention: each change to viola verify keeps the summary, step line and failing row forms of its output contracts
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page; nothing in this removal reaches a web surface.
@task_item n=2 part=not_needed
Board styling, colour tokens and column layout of viola list: the removal changes no column, colour, width or truncation.
@task_item n=3 part=not_needed
viola send and its readback mirror, send refusals and send output words: the send path does not carry the status line.
@task_item n=4 part=not_needed
viola wait, viola last and the wheel verbs pause and release: their output forms are untouched.
@task_item n=5 part=not_needed
viola link and unlink output: no status-line value reaches them.
@task_item n=6 part=not_needed
The viola mcp server, its tools and tool results: the MCP surface carries no status-line value.
@task_item n=7 part=not_needed
viola answer output and dialog attention: dialogs are not the status line.
@task_item n=8 part=not_needed
viola plugin install output: the plugin install path does not write the status-line override.
@task_item n=9 part=not_needed
Deep links and URL coupling in the CLI: nothing here adds or removes a link.
@task_item n=10 part=not_needed
CLI exit codes and the instance-unreachable details: no exit code or detail is added or removed.
