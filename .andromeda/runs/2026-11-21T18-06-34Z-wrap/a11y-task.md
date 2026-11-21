@task_prose part=session
The session added the 29th relied capability-ledger row, cross-session-prefix, read pass or fail by viola verify; the relied count moved 28 to 29, so verify's step lines now count [NN/29] and its summary reads 29 pass 0 fail.
Verify's Run B gains a cross-session turn: a print-mode sender child (diag subject verify-sender, 120 s deadline, 30 s arrival window) sends to an address verify composes from XDG_RUNTIME_DIR, read once and held in memory, printed on no output line, log line, stamp or error.
Without a runtime dir the turn does not run and the row reads not-run; every test and harness verify now gets its own short owner-only runtime dir.
viola hook gained a hidden --names-only option, valid only with --capture, that files closed tool words only; the fake agent resolves its sibling by the address and its start receipt gains pid and prompt.
The recorded 2.1.328 fixture set was re-recorded whole, 32 to 33 files, the new one a cross-session prompt arrival; measured live on 2.1.328 (XS-2, 29/29, delivered). Nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows and their readings, the probe prompts, the stamp merge and the fixture scrub and hygiene walk
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate's compiled literals (CLI flags, the socket scheme, directory, suffix and path bound)
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload reader: the prompt origin and the harness-prefix literals
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests (tool words, capture documents, the sender plugin hooks)
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's row-check unit tests (the cross-session row's pass and fail table, the address composition)
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the ledger's probe-framing unit tests
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp unit tests (the merged stamp's measured blocks)
@task_file n=8 path=src/cmd/hook.rs
what: the viola hook command: event and dialog calls and the capture arm
@task_file n=9 path=src/cmd/verify.rs
what: the viola verify command: its measure, its children and its step-line output
@task_file n=10 path=src/cmd/verify/typed.rs
what: verify's typed runs and their inputs
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: verify's typed run ends (how each probe run is ended)
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: verify's probe turns, Run B's cross-session turn and its sender child
@task_file n=13 path=src/run/version_gate.rs
what: the CLI version gate's tests
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the fake agent test double that replays the recorded CLI for tests
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's sibling-session lookup
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests (the sender and receiver)
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option tests
@task_file n=18 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, its subject list
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot that stamps a verify before a session
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run and its pinned live verify output
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its floor rows, kept stamps and summary lines
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests
@task_file n=23 path=tests/cli_verify.rs
what: the viola verify integration suite, its step-line and per-row assertions
@task_file n=24 path=tests/cli_verify/runs.rs
what: verify integration tests over the probe runs
@task_file n=25 path=tests/cli_verify/stamps.rs
what: verify integration tests over the ledger stamps
@task_file n=26 path=tests/cli_fake_agent.rs
what: the fake agent integration suite, its receipt key pin
@task_file n=27 path=tests/cli_send.rs
what: the viola send integration suite (a recorded session id literal)
@task_file n=28 path=tests/chaos_revive.rs
what: the revive chaos suite (a recorded session id literal)
@task_file n=29 path=tests/contract_diag_schema.rs
what: the diagnostic line schema contract check
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: the fake agent's drift check against every recorded dialog set
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene check over the recorded sets
@task_file n=32 path=tests/contract_ledger_probes.rs
what: the ledger probe contract check over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: the security negative tests on env and capture (no payload or canary byte kept)
@task_file n=34 path=tests/support/home.rs
what: the test support's test home and wrapper start as user
@task_file n=35 path=tests/support/start.rs
what: the test support's start helpers
@task_file n=36 path=tests/support/verify.rs
what: the test support's verify helpers and the test-owned runtime dir
@task_file n=37 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 hook payload fixture set, re-recorded whole
@task_file n=38 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: the recorded cross-session prompt arrival fixture (new)
@task_item n=1 part=touches
contract/viola-verify/step-line: verify's step lines now count to 29 ([NN/29]) and print one more row, cross-session-prefix, reading pass, fail or not-run; a row that holds the count or the row list moves.
@task_item n=2 part=touches
decision/critical-path-cli-board-output: the verify output on this critical path gains the 29th row and its 29 pass 0 fail summary.
@task_item n=3 part=touches
contract/interrupted-verify/exit, contract/interrupted-verify/stderr and contract/viola-verify/interrupted-pair: verify now runs one more child (the print-mode sender) inside Run B; an interrupt may land during the sender's turn, whose end the interrupted pair must still cover.
@task_item n=4 part=touches
decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human: two new bounds, the sender's 120 s deadline and the 30 s arrival window, fall on verify, never on the human.
@task_item n=5 part=touches
decision/run-diagnostics-off-terminal: a new diagnostic subject verify-sender joins the process log's subject list; its lines go to the log, never the terminal.
@task_item n=6 part=touches
decision/paths-not-carried: the runtime dir path and the composed socket address reach only the sender's argv, never a printed line, refusal, log line or stamp.
@task_item n=7 part=touches
decision/static-linear-output and decision/no-spinners: verify's wait on the sender adds up to two minutes inside one step with no progress line; the output stays linear and static.
@task_item n=8 part=touches
decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal and decision/cli-a11y-by-literal-tests: the new row's step line is pinned by literal in the verify suite, beside the [NN/29] count.
@task_item n=9 part=touches
contract/cli-help/served-verbs-only and contract/cli-help/usage-line: viola hook gained a hidden option --names-only (valid only with --capture) that help must not list.
@task_item n=10 part=touches
decision/single-vendor-content-boundary, decision/boundary-check-never-inspects-claude-content and decision/no-screen-content-parsing: the new row reads only whether an arrived prompt opens with the CLI's own cross-session prefix literal, and the sender's tool names as closed words; no other vendor content is read.
@task_item n=11 part=touches
decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: the new verify output assertions live inside the existing verify integration suite.
@task_item n=12 part=touches
decision/ci-test-gate-no-skips and decision/zero-flakiness: every test and harness verify now takes a test-owned runtime dir so the 29-row stamp holds under pre-push's empty env instead of reading not-run.
@task_item n=13 part=touches
decision/live-readings-never-gate: the live rig gained labels XS and XS-2 and summary lines cross-session-prefix, cross-session, sender-tools and send-result; they are readings, kept out of the gate.
@task_item n=14 part=touches
decision/no-hint-names-release and decision/one-hint-per-code: the new row's fail and not-run readings add no refusal code or hint; check no row lists the verify row words as closed.
@task_item n=1 part=untouched
The TUI passthrough rows (entry, human keystrokes, focus, mouse and resize sequences, viola terminal bytes, vt100 feed panic, refused peer): the session did not reach the wrapped session's terminal path.
@task_item n=2 part=untouched
The viola list board and caption rows, the SGR, colour, dim, bold, amber and NAME truncation rules: no board or colour output changed.
@task_item n=3 part=untouched
The refusal and failure families of run, revive, link, unlink, plugin install, send and answer (codes, exit codes, --json documents, unable and hint lines, socket-dir and strict-modes refusals): none gained or lost a code or a line.
@task_item n=4 part=untouched
The dialog, send-readback and human-takes-the-wheel critical paths and the wheel rules: no dialog, send or wheel behaviour changed.
@task_item n=5 part=untouched
The screen-reader targets and setups, the manual Orca and keyboard passes: unchanged.
@task_item n=6 part=untouched
The a11y CI rows (GitHub Actions workflow, coverage suite, no own artifact, gate breach fails the build, mutation audit): no workflow or gate value changed.
@task_item n=7 part=untouched
The N/A areas (keyboard navigation, visual design, contrast, focus management, structured violation JSON, WCAG mapping) and the absences (touch, web or graphical frontend, internationalization, user time limit, rendered page): the session adds no frontend or user-facing time limit.
@task_item n=8 part=untouched
config.json rows such as dialog_hold_s and the MCP wait tool's timeout_ms: no config key or MCP tool parameter was added.
