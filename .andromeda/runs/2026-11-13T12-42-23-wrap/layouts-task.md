@task_prose part=session
An epoch cleanup chunk: no product verb, output line, exit code, refusal, hint, JSON shape, flag or token rendering changed.
The three source censuses now share one masking scanner (comments, or comments and literals) in place of four hand scanners; the error-body and bounded-reader censuses leave out the fake agent's whole module dir, and the OS-gated census also skips block comments.
The largest test files were split into modules with every test name and case kept: the fake agent's bin root, the claude ledger's test cases, and the viola verify CLI tests.
Dead code was removed (a strict-modes presence check with no caller, the harness's non-unix 21-row ledger form) and named mutant survivors were closed with new tests in the state, mcp server, mcp tap and e2e harness code.
Test homes now make their private dirs through one shared helper; the e2e secret scan retired its cookie class and gained a planted unreadable-file case.
Measured: a pinned-copy test needs a parent-mtime witness because the no-clobber persist reads a directory at the name as already present.
@task_file n=1 path=tests/support/source_scan.rs
what: the shared test-support masking scanner the source censuses read code through
@task_file n=2 path=tests/support/home.rs
what: the shared test home helper, now with the one private-dir maker every test home uses
@task_file n=3 path=tests/contract_error_bodies.rs
what: the error-body census check over the product source (refusal and internal-error bodies)
@task_file n=4 path=tests/contract_bounded_readers.rs
what: the bounded-readers census check over the product source
@task_file n=5 path=tests/contract_os_gated_tests.rs
what: the census of OS-gated tests and their GATES list
@task_file n=6 path=src/cmd/hook.rs
what: the hidden hook verb's handler, its capture and fail-open paths; a test helper only changed
@task_file n=7 path=src/run/mod.rs
what: the viola run wrapper's core; a test-only snapshot builder added
@task_file n=8 path=src/run/links.rs
what: the run wrapper's link handling behind viola link and unlink; its tests take the shared snapshot builder
@task_file n=9 path=src/run/snapshot.rs
what: the run wrapper's instance snapshot; its tests take the shared snapshot builder
@task_file n=10 path=crates/viola-state/src/strict.rs
what: the strict-modes checks behind the strict-modes refusals; a dead presence check removed
@task_file n=11 path=crates/viola-state/src/fs.rs
what: the private file and directory writes of the viola home; a test added
@task_file n=12 path=crates/viola-state/src/pin.rs
what: the pinned copy of the claude binary and its hash; a test added
@task_file n=13 path=crates/viola-mcp/src/lib.rs
what: the viola mcp server; a test of the oversize-line WARN added
@task_file n=14 path=crates/viola-mcp/src/stdin.rs
what: the viola mcp server's capped stdin line reader; a test added
@task_file n=15 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only mcp tap that records a viola mcp exchange; unit tests added
@task_file n=16 path=crates/viola-e2e/tests/mcp_tap.rs
what: the e2e tests of the mcp tap (usage refusal, fixture, record)
@task_file n=17 path=crates/viola-e2e/src/harness/mod.rs
what: the e2e harness core, its free-space read restated through one helper
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness's run of the ledger rows; the non-unix 21-row form removed
@task_file n=19 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the e2e artifact secret scan and its classes; the cookie class retired
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for the claude CLI in tests, its bin root
@task_file n=21 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's agents listing and session registry, moved out unchanged
@task_file n=22 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input box byte reader, moved out unchanged
@task_file n=23 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status line replay, moved out unchanged
@task_file n=24 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's own test helpers
@task_file n=25 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's tests of its options, input box and hooks
@task_file n=26 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's tests of its status line, dialog replay, agents and resume modes
@task_file n=27 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the claude ledger's tests behind viola verify, now a root over split modules
@task_file n=28 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger tests of the rows, the version read and the verify stamp
@task_file n=29 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger tests of the record scrub, the input-box signature and screen checks
@task_file n=30 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger tests of the resume, paste, local-command and status line checks
@task_file n=31 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the ledger tests of the framing prompts, dialog checks and probe answers
@task_file n=32 path=tests/cli_verify.rs
what: the viola verify CLI tests' root, keeping its unix-gated cases
@task_file n=33 path=tests/cli_verify/stamps.rs
what: the viola verify CLI tests of steps, stamps, refusals and the record
@task_file n=34 path=tests/cli_verify/runs.rs
what: the viola verify CLI tests of the record screens, run dirs, modals and hook capture
@task_file n=35 path=.claude/docs/services/viola-state.md
what: the viola-state service doc
@task_item n=1 part=touches
Every viola verify output and refusal contract (step line, summary, record, stop, fixture-not-clean, home-not-private, version-unreadable): the CLI tests guarding them moved into split modules with names and cases kept; a guard cell naming one of those tests now lives in a split file; no output changed.
@task_item n=2 part=touches
viola verify --record and fixture-not-clean: the ledger's scrub and screen-check tests moved into split modules, unchanged; a guard naming them moved.
@task_item n=3 part=touches
The verify ledger's row count: the e2e harness keeps the one 24-row form and its non-unix 21-row form is gone; a row whose cell holds 21 ledger rows would be stale.
@task_item n=4 part=touches
The strict-modes refusals (frame verbs, viola list, list --json, revive --json strict-modes-failed): a dead presence check under them removed, no refusal or detail changed.
@task_item n=5 part=touches
viola run and viola revive start refusals over the home tree and the pinned copy (home-tree-refused, pinned-hash-mismatch): new tests pin the private writes and the pinned copy; no refusal changed.
@task_item n=6 part=touches
The viola mcp server rows (mcp-server-without-human-surface, mcp tool results): new tests of the oversize-line WARN and the capped stdin reader; no tool or result shape changed.
@task_item n=7 part=touches
The hidden viola mcp verb: the test-only mcp tap that records its exchange is now tested end to end; no verb changed.
@task_item n=8 part=touches
Rules guarded by the error-body census (a hint for every refusal cause, internal-error bodies): the census now reads code through the shared masking scanner and leaves out the whole fake agent module dir; verdicts unchanged.
@task_item n=9 part=touches
Rules guarded by the bounded-readers census: same scanner and the same fake agent exclusion; verdicts unchanged.
@task_item n=10 part=touches
The OS-gated test census: block comments are now skipped, and gated cases named in its GATES list must stay in a root test file, not a split module.
@task_item n=11 part=touches
The hidden hook verb: a test helper over both hook paths refusing inside the hook handle; no hook behaviour changed.
@task_item n=12 part=touches
The viola list board's agents read and the status line rows, indirectly: the fake agent's agents listing, input box and status line replay moved into modules, bodies unchanged.
@task_item n=13 part=touches
viola link and unlink output, indirectly: the run wrapper's link and snapshot tests take one shared snapshot builder; no output changed.
@task_item n=14 part=touches
Any row naming the e2e secret scan classes: the cookie class is retired, the remaining classes are claude-stripped, content-canary, mode and unreadable.
@task_item n=1 part=untouched
The web surface absence and the CLI deep-links absence.
@task_item n=2 part=untouched
CLI token rendering, colour, SGR styling, spacing and motion rows.
@task_item n=3 part=untouched
The viola list board's columns, widths, truncation, ordering, escaping and the BAY line.
@task_item n=4 part=untouched
viola send output, the readback mirror and its vocabulary, and the send refusals before any request.
@task_item n=5 part=untouched
Wheel and handoff verb output, viola release budget output, viola answer and viola wait output.
@task_item n=6 part=untouched
CLI exit codes, their --json form, the global flags, --help usage and verb groups.
@task_item n=7 part=untouched
viola plugin install output and refusals.
@task_item n=8 part=untouched
The output conventions: no banner, spinner, footer, pager or progress glyphs; stdout for results, stderr for the rest.
