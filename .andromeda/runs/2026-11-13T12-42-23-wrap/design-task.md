@task_prose part=session
A cleanup pass over tests and test support. No verb, output line, exit code, refusal, hint, colour or column changed, and no product API was added or changed.
One shared masking scanner (comments, or comments and literals) replaced the hand scanners of the three census tests. The census verdicts are unchanged.
The largest test files were split into module files with every test name kept: the fake agent's bin, the claude ledger's test cases and the verify CLI tests. A private-dir test helper now makes test homes, and 14 root test files were swept onto it.
Retired: the e2e secret scan's cookie class (a cookie line now reads as no hit), the dead strict-modes function check_present (no callers), and the harness's non-unix twins.
Added: test cases only. These cover the mcp tap, the mcp oversize-line WARN, the stdin line reader, the pinned-copy placement, the private-file placement, writable bytes and an unreadable file in the secret scan. Control runs against named mutant survivors all hold.
@task_file n=1 path=tests/support/source_scan.rs
what: the shared masking scanner the census tests use to blank comments, or comments and literals, before they read code (test support)
@task_file n=2 path=tests/support/home.rs
what: the test home helper, which makes isolated viola homes and private dirs for the CLI tests
@task_file n=3 path=tests/contract_error_bodies.rs
what: the census check that error and refusal bodies carry no internals. Its filter now leaves out the fake agent's whole module dir
@task_file n=4 path=tests/contract_bounded_readers.rs
what: the census check that every reader of outside input is bounded. Its filter now leaves out the fake agent's whole module dir
@task_file n=5 path=tests/contract_os_gated_tests.rs
what: the census check of OS-gated (unix-only) test cases and their GATES list. It now masks comments through the shared scanner
@task_file n=6 path=src/cmd/hook.rs
what: the hook verb, which is silent on stdout and fails open. It gained a test helper only
@task_file n=7 path=src/run/mod.rs
what: viola run, which wraps a claude session as an instance. It gained a test-only first snapshot helper
@task_file n=8 path=src/run/links.rs
what: viola run's link handling between instances (test lines only)
@task_file n=9 path=src/run/snapshot.rs
what: viola run's instance snapshot, the reading behind the board (test lines only)
@task_file n=10 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the claude behaviour ledger's tests (the rows verify measures and stamps, including the readback box signature checks), now split into four files
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger tests for the stamps.json envelope, version parsing and the verified/unverified reading (moved here)
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger tests for the input box signature, the modal and screen checks and the scrub (moved here)
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger tests for the resume, paste, local command and statusline checks (moved here)
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the ledger tests for framing, dialog and probe answers (moved here)
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: the fake agent, the test stand-in for the claude CLI. Its bin root is now split into modules
@task_file n=16 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's agents listing and session registry (moved here)
@task_file n=17 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input box byte reader, which turns keys, pastes and submits into actions (moved here)
@task_file n=18 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline command replay (moved here)
@task_file n=19 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's test helpers (moved here)
@task_file n=20 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option, input and hook-command tests (moved here)
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's statusline, agents-output, replay and resume tests (moved here)
@task_file n=22 path=tests/cli_verify.rs
what: the viola verify CLI tests (the verify output and refusals). Its root keeps the unix-gated cases, and the rest moved into two module files
@task_file n=23 path=tests/cli_verify/stamps.rs
what: the viola verify tests for the step line, stamps, refusals (version unreadable, pinned copy tampered, missing program, home-tree check) and logged spawns (moved here)
@task_file n=24 path=tests/cli_verify/runs.rs
what: the viola verify tests for record screens, run dirs, the --help text, modal stops and hook capture (moved here)
@task_file n=25 path=crates/viola-state/src/strict.rs
what: the strict-modes check of the home and instance state, behind the strict-modes refusals. Its dead check_present was removed
@task_file n=26 path=crates/viola-state/src/fs.rs
what: private file and dir placement in the viola home (a test case was added)
@task_file n=27 path=crates/viola-state/src/pin.rs
what: the pinned copy of the claude binary, behind the pinned-hash-mismatch refusal (a test case was added)
@task_file n=28 path=crates/viola-mcp/src/lib.rs
what: the mcp verb's server, with stdout as its protocol stream. A test case was added for the one WARN parse-rejected log line on an oversize line
@task_file n=29 path=crates/viola-mcp/src/stdin.rs
what: the mcp verb's capped stdin line reader (a test case was added)
@task_file n=30 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only mcp tap, which records an mcp exchange for fixtures (test cases were added)
@task_file n=31 path=crates/viola-e2e/tests/mcp_tap.rs
what: the e2e tests that spawn the mcp tap
@task_file n=32 path=crates/viola-e2e/src/harness/mod.rs
what: the e2e harness: test homes, free-space check and the ledger row count
@task_file n=33 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness's run of the fake agent (non-unix twins removed)
@task_file n=34 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the e2e artifact secret scan and its classes. The cookie class is retired, and claude-stripped, content-canary, mode and unreadable remain
@task_file n=35 path=.claude/docs/services/viola-state.md
what: the service doc of the viola-state crate (home, strict modes, pinned copy, private placement)
@task_item n=1 part=touches
The viola verify output and refusal contracts (step line, summary, fail word, --help, stamps update, version unreadable, CLI not found, home-tree check, interrupted, unclean fixture). Their CLI tests moved into two module files with names kept, and no output changed. Any row whose guard cell names a verify test case by its file now points at a moved case.
@task_item n=2 part=touches
The readback box signature check. The ledger's input box signature tests moved into a split file with their names kept, and the rule is unchanged. A guard cell naming them by file moved.
@task_item n=3 part=touches
The strict-modes refusal rows (list board strict-modes refusal and scan, CLI strict-modes refusal, frame verb refusals, revive and run strict-modes). Under them, the dead function check_present was removed from the strict-modes check. No refusal changed.
@task_item n=4 part=touches
The pinned-hash-mismatch refusals of run, revive and verify. A test case pins that placing the pinned copy over an unreadable name fails and places nothing. Behaviour is unchanged.
@task_item n=5 part=touches
The hook verb contracts (exit code, stderr) and silent catch sites. The hook verb gained a test helper only, and it stays silent and fails open.
@task_item n=6 part=touches
The mcp verb's stdout contract and the hidden mcp verb. A new test case covers the one WARN parse-rejected line on an oversize stdin line, and stdout is unchanged. The test-only mcp tap is now tested.
@task_item n=7 part=touches
The no-internals-in-errors and no-stack-traces rules and any guard naming the error-bodies census or the bounded-readers census. Both censuses now leave out the fake agent's whole module dir. The OS-gated census masks comments through the shared scanner and also skips block comments.
@task_item n=8 part=touches
viola run's rules (run prints nothing while wrapped, child started, start refusals). Its instance, links and snapshot gained test-only lines, and no output changed.
@task_item n=9 part=touches
The viola list board rows that read the agents listing (failed agents read, status, reading age). The fake agent's agents output and live sessions moved into a module, with the bytes unchanged.
@task_item n=10 part=touches
The send readback and the input-not-ready refusal hint. The fake agent's input box byte reader moved into a module, with behaviour unchanged.
@task_item n=11 part=touches
Any row whose cell holds a secret-scan class list that includes cookie, or a never-logged cookie clause. The cookie class is retired.
@task_item n=1 part=untouched
Brand identity, the strip vocabulary, the readback box as a signature element and the strip-board density rules: no rendered element changed.
@task_item n=2 part=untouched
Colour palette, typography, spacing, iconography and border radius: no colour, glyph, separator, column or width changed.
@task_item n=3 part=untouched
The CLI colour decision order, colour depth and colour tokens: no styling path changed.
@task_item n=4 part=untouched
The command names (name/command rows) and the --help verb groups: no verb was added, renamed or removed.
@task_item n=5 part=untouched
The CLI exit codes, the exit-21 causes, the revive --json refusal codes and the refusal hints: no code, cause or hint text changed.
@task_item n=6 part=untouched
The web-page surface and rendered-page absences, the anti-patterns and the self-validation tests (squint and swap): nothing reaches them.
@task_item n=7 part=untouched
The link, unlink, pause, release, answer, wait and last verb outputs: none changed.
