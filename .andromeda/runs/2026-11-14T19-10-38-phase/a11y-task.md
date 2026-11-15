@task_prose part=chunk
Removes viola's status-line interception whole: a wrapped session's status line is the user's own command, run by the CLI exactly as unwrapped.
Leaves: the `hook statusline` arm and its capture probe, the pure statusline module and its `/bin/sh -c` shell-out, the start's settings override and `--settings` argument, the snapshot's `statusline_command` field, the fake agent's status-line modes, fixtures, schema, fuzz seeds, tests and log subjects.
The ledger loses its two interception rows (status-line override, status-line shell); `relied()` goes from 23 rows to 21 and `viola verify`'s Run B loses only its status-line probe and capture.
Re-reads every statusline test case and sorts it: leaves (it tests the interception) or is rewritten (it tests something that stays and only spells the interception); every coordinate re-tested.
Adds no code beyond the removal: no new settings, flag, env var or ledger row; no live `claude` start, no install, no write under the live home.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the `hook statusline` arm — instance check, replay read, the `/bin/sh -c` run under its deadline, the stdout pass-through of the status-line output; deleted whole
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the `viola hook` dispatch for event and dialog calls and the hidden `--capture` probe forms — edits: `mod statusline`, `STATUSLINE_DEADLINE`, the `statusline::WORD` branch, the `statusline_deadline_is_five_seconds` test, `capture_statusline` and its four tests, the `--shell` arg doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs
what: the pure statusline module of the claude agent crate — `BIN_SH`, `shell_argv`, `user_command`, `override_document`, `USER_SETTINGS`, `SETTINGS_FLAG`, `WORD` and their tests; deleted whole
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude agent crate's module list — edits: `pub mod statusline`
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the proptest seeds of the statusline module; deleted
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the ledger of relied rows `viola verify` probes — edits: `LedgerRow::StatusLineOverride` / `StatusLineShell` in `ALL`, their ids and claims, the `ShellVerdict` / `StatusLineCapture` / `statusline_variants` / `StatusLine.<k>` helpers
@task_file n=7 path=src/cmd/verify/typed.rs mark=edits
what: `viola verify`'s typed runs — edits: Run B's status-line probe `--settings typed/statusline.json` and its capture
@task_file n=8 path=src/cmd/verify/typed/runs.rs mark=edits
what: `viola verify`'s run definitions — edits: Run B's status-line probe and capture
@task_file n=9 path=src/run/version_gate.rs mark=edits
what: the version gate over the wrapped CLI's version — edits: its statusline row group
@task_file n=10 path=src/cmd/run.rs mark=edits
what: `viola run`'s start of a wrapped session — edits: the statusline source read (`STATUSLINE_SOURCE` / `statusline-source.json`), the instance `settings.json` override written every start, the snapshot recording of the status-line command, and their tests
@task_file n=11 path=src/run/mod.rs mark=edits
what: the wrapped session's run module — edits: `SETTINGS_FLAG` (the `--settings <instance dir>/settings.json` argument), its `statusline_command: None` literal
@task_file n=12 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot on disk — edits: the `statusline_command` field and its tests
@task_file n=13 path=crates/viola-state/src/board.rs mark=edits
what: the state crate's board of instances — edits: its `statusline_command: None` literals, its `"statusline"` literal
@task_file n=14 path=crates/viola-state/src/liveness.rs mark=edits
what: the state crate's instance liveness read — edits: its `statusline_command: None` literals
@task_file n=15 path=crates/viola-state/src/replay.rs mark=edits
what: the state crate's event-log replay — edits: its `statusline_command: None` literals
@task_file n=16 path=crates/viola-state/src/strict.rs mark=edits
what: the state crate's strict-modes check of the home — edits: its `statusline_command: None` literals
@task_file n=17 path=src/cmd/revive.rs mark=edits
what: `viola revive`'s start of a recorded session — edits: its `statusline_command: None` literals
@task_file n=18 path=src/run/dialog.rs mark=edits
what: the wrapped session's dialog handling — edits: its `statusline_command: None` literals
@task_file n=19 path=src/run/send/slot.rs mark=edits
what: the wrapped session's send slot — edits: its `statusline_command: None` literals
@task_file n=20 path=src/run/wheel.rs mark=edits
what: the wrapped session's wheel between driver and human — edits: its `statusline_command: None` literals
@task_file n=21 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the MCP server's handler tests — edits: their `statusline_command: None` literals
@task_file n=22 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status-line modes — the `--framing` StatusLine replay, `--settings` + `--statusline-stdin`, the `statusline-echo` mode and receipt kind `statusline`; deleted whole
@task_file n=23 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness's boot of a wrapped session — edits: `boot --statusline-echo`
@task_file n=24 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture of the wrapped CLI 2.1.287; deleted
@task_file n=25 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture of the wrapped CLI 2.1.302; deleted
@task_file n=26 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the StatusLine fixtures, validated by fixture hygiene; deleted
@task_file n=27 path=fuzz/corpus/hook_stdin/statusline-absent
what: a fuzz corpus seed of the hook stdin reader for an absent status line; deleted
@task_file n=28 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a fuzz corpus seed of the hook stdin reader for a status-line epoch; deleted
@task_file n=29 path=schemas/diag-line.v1.json mark=edits
what: the diag-line schema of run diagnostics — edits: the `subject` value `statusline-shell`
@task_file n=30 path=tests/hook_statusline.rs
what: the `hook statusline` contract tests; deleted whole
@task_file n=31 path=Cargo.toml mark=edits
what: the workspace manifest's test targets — edits: the `[[test]]` entry of `hook_statusline`
@task_file n=32 path=.config/nextest.toml mark=edits
what: the nextest configuration — edits: the `hook_statusline` filter term
@task_file n=33 path=tests/support/statusline.rs
what: the test support helpers for the status line; deleted whole
@task_file n=34 path=tests/tui_passthrough.rs mark=edits
what: the TUI passthrough boundary tests of zero viola bytes on the terminal — edits: `passthrough_with_a_statusline_source_adds_no_viola_bytes`
@task_file n=35 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene check — edits: the StatusLine fixture schema validation
@task_file n=36 path=tests/contract_fake_agent_drift.rs mark=edits
what: the fake agent's drift check against recorded fixtures — edits: the drift check over StatusLine variants
@task_file n=37 path=tests/contract_ledger_probes.rs mark=edits
what: the ledger probe contract tests — edits: their statusline cases
@task_file n=38 path=tests/cli_verify.rs mark=edits
what: `viola verify`'s CLI tests — edits: their statusline cases
@task_file n=39 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open tests — edits: their statusline cases
@task_file n=40 path=tests/contract_diag_schema.rs mark=edits
what: the diag-line schema contract tests — edits: their statusline cases
@task_item n=1 part=needs
The `viola hook statusline` command leaves: its dispatch word, its five-second deadline, its exit status and its hidden `--capture <DIR> [--shell <WORD>]` probe form; the rows stating that command's exit status are expected amendments of the wrap.
@task_item n=2 part=needs
The pass-through of the status-line command's output byte for byte leaves: viola no longer stands between the CLI and the user's status-line command; the CLI runs the user's own command and viola emits, reads and alters nothing on that path.
@task_item n=3 part=needs
Zero viola bytes on the terminal still holds for a wrapped session with a user status line: the passthrough test that spelled a status-line source is sorted (leaves or rewritten), and a wrapped session's TUI still reads as the unwrapped one.
@task_item n=4 part=needs
viola runs no shell for any purpose: the `/bin/sh -c` spawn of the user's command leaves with its process-start and process-exit diag lines; the no-tmux-or-shell rule is closed once the shell-out is gone.
@task_item n=5 part=needs
The start writes no status-line settings override, passes no `--settings` argument, reads no `statusLine` from the user's settings and records no status-line command; the user's settings file is never written.
@task_item n=6 part=needs
The snapshot loses its `statusline_command` field; a snapshot or event log written by an older binary that holds it keeps reading (unknown fields skipped and counted, never denied); the own-format `v` is not decided here.
@task_item n=7 part=needs
`viola verify`'s ledger drops the two interception rows, 23 relied rows to 21: verify's step lines lose those two rows' lines, and Run B keeps its other pastes, probes and its `claude agents --json` read as they are.
@task_item n=8 part=needs
Log subjects leave: the diag-line subject value `statusline-shell`, the span area `statusline`, the subject tag `contract-hook-statusline`; run diagnostics still never reach the terminal.
@task_item n=9 part=needs
The fake agent's status-line replay, `--settings` / `--statusline-stdin` and `statusline-echo` mode, its `statusline` receipt kind and the harness's `boot --statusline-echo` leave; the removal is proved by the fake agent in CI and by the code's own tests.
@task_item n=10 part=needs
The StatusLine fixtures, their schema and its hygiene validation, the drift check over StatusLine variants and the hook-stdin fuzz seeds for the status line leave; no box-only screen fixture is hand-written — it waits on a live recording, an operator step.
@task_item n=11 part=needs
Every statusline test case is sorted: one that tests the interception leaves; one that tests what stays and only spells the interception is rewritten, keeping its own assertion inside its existing suite; the removed test target leaves the manifest and the nextest filter, so no case is skipped.
@task_item n=12 part=needs
Removed words and identifiers: the retired-words fixture may take them (P3 reads its rule); an acceptance probe over identifiers is checked against the plan's own test cases that must spell an old word, kept in fixture files outside the probed paths.
@task_item n=13 part=needs
A guard test's one-off remove-the-guard control runs through the gate's control under a kill deadline, named by the test's name alone, and holds only when that case ran and failed; no mutation run.
@task_item n=14 part=needs
Every other `hook` arm — `hook <event>`, the dialog arms and their `--capture` / `--answers` probe forms — keeps its behaviour and its refusals; the dispatch edit removes only the status-line branch.
@task_standing n=1
convention: every new or rewritten a11y assertion lives inside an existing test suite and fails as a nextest failure; no separate a11y driver or suite
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: every test the chunk keeps or rewrites runs in CI with no skip, no retry and zero flakiness; a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: every fixed CLI string a rewritten test reads is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=4
convention: every TUI boundary test is sequenced on records and never inspects the wrapped CLI's content
home: keyboard-test-harness
@task_standing n=5
convention: every a11y case runs inside the coverage suite of the GitHub Actions workflow, the gate requiring coverage and doctest
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
`viola link` and `viola unlink` output, exit codes and result lines: the chunk prints nothing through them.
@task_item n=2 part=not_needed
`viola plugin install` human and `--json` output: untouched by the removal.
@task_item n=3 part=not_needed
`viola --help` group labels, usage line, handoff row and served verbs: the status-line hook arm was hidden and never listed there.
@task_item n=4 part=not_needed
SGR and colour rules (amber, bold, dim, word beside every SGR, colour decision inputs): the chunk adds no coloured output.
@task_item n=5 part=not_needed
Screen-reader test setups for the cli and the tui (Orca targets): the chunk changes no screen-reader target or setup.
@task_item n=6 part=not_needed
The structured violation JSON schema and its rows: the chunk adds no violation output.
@task_item n=7 part=not_needed
Absences of a touch surface, a web or graphical frontend, a rendered page and internationalization: nothing here adds one.
@task_item n=8 part=not_needed
`viola wait` waiting line and timed_out output: untouched by the removal.
@task_item n=9 part=not_needed
Reading-level gate and AAA cognitive escalation decisions: the chunk writes no new user-facing copy.
