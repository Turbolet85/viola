@task_prose part=session
A test-only cleanup: no product API, verb, exit code, refusal, output line, SGR rule, dependency, schema or CI step changed.
The three census tests now share one masking scanner (comments, or comments and literals) in test support; the hand scanners each census carried are retired, census verdicts unchanged (84/84).
The largest test files and the fake agent's bin were split into module files with names and bodies kept; every test function name and case row stands.
Dead code retired: the strict-modes crate's unused presence check, the e2e harness's non-unix twins, and the e2e secret scan's cookie class (a cookie line now reads as no hit).
Named mutation survivors closed by 25 controls that hold (no cargo-mutants run), with new cases for the pin, the private-file placer, the MCP stdin reader, the MCP oversize-line WARN, the MCP tap and the secret scan's unreadable class.
Test homes make their private directories through one shared test helper; the census of error bodies and bounded readers leaves out the fake agent's whole module directory.
@task_file n=1 path=tests/support/source_scan.rs
what: the one shared test masking scanner the census checks read source through
@task_file n=2 path=tests/support/home.rs
what: the test home helper that makes test homes and their private directories
@task_file n=3 path=tests/contract_error_bodies.rs
what: the error-bodies census check that every error body has a writer and a witness
@task_file n=4 path=tests/contract_bounded_readers.rs
what: the bounded-readers census check over the product's input readers
@task_file n=5 path=tests/contract_os_gated_tests.rs
what: the OS census check of the test cases gated to one platform (Linux only)
@task_file n=6 path=tests/cli_verify.rs
what: the viola verify CLI suite root, holding its unix-gated cases
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify CLI cases for the ledger stamp, step lines and verify refusals
@task_file n=8 path=tests/cli_verify/runs.rs
what: the viola verify CLI cases for record screens, modal stops and run dirs
@task_file n=9 path=src/cmd/hook.rs
what: the viola hook event, dialog and statusline call handler and its refusal tests
@task_file n=10 path=src/run/mod.rs
what: the viola run TUI passthrough driver module
@task_file n=11 path=src/run/links.rs
what: the viola run link handling of the passthrough driver
@task_file n=12 path=src/run/snapshot.rs
what: the viola run instance snapshot of the passthrough driver
@task_file n=13 path=crates/viola-state/src/strict.rs
what: the strict-modes check over the home and socket directory
@task_file n=14 path=crates/viola-state/src/fs.rs
what: the private-file and private-directory placer of the state crate
@task_file n=15 path=crates/viola-state/src/pin.rs
what: the pinned copy of the claude binary the wrapper runs
@task_file n=16 path=crates/viola-mcp/src/lib.rs
what: the MCP server and its parse-rejected WARN log line
@task_file n=17 path=crates/viola-mcp/src/stdin.rs
what: the MCP server's bounded stdin line reader
@task_file n=18 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the verify ledger's test module root
@task_file n=19 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the verify ledger cases for its rows, the capture plugin and the stamp
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the verify ledger cases for the scrub, the input-box signature and screen checks
@task_file n=21 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the verify ledger cases for agents, resume, paste, local commands and statusline rows
@task_file n=22 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the verify ledger cases for framing prompts, dialog rows and probe answers
@task_file n=23 path=src/bin/viola-fake-agent.rs
what: the fake agent test bin that stands in for claude in the boundary tests
@task_file n=24 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's claude agents output and session registry
@task_file n=25 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's stdin reader of keys, pastes and submits
@task_file n=26 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline command runner
@task_file n=27 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's own test module root
@task_file n=28 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's cases for its options, input and hook replay
@task_file n=29 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's cases for statusline, dialog and resume replay modes
@task_file n=30 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only MCP tap that records and scrubs an MCP exchange
@task_file n=31 path=crates/viola-e2e/tests/mcp_tap.rs
what: the e2e cases that spawn the MCP tap
@task_file n=32 path=crates/viola-e2e/src/harness/mod.rs
what: the e2e harness and its free-space read of the test home
@task_file n=33 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run of the live ledger rows
@task_file n=34 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the e2e artifact secret scan and its class list
@task_file n=35 path=.claude/docs/services/viola-state.md
what: the state crate's service doc
@task_item n=1 part=touches
decision/error-bodies-have-writer-and-witness: its census now masks source through the shared scanner and leaves out the fake agent's whole module directory (the bin and every file under it), not the bin file alone.
@task_item n=2 part=touches
Any row that names the census checks as the guard of an output-discipline rule (no bare error text, thiserror messages interpolate nothing, no stack trace as error text): the census reads code through the one shared scanner now, verdicts unchanged.
@task_item n=3 part=touches
decision/linux-only and decision/ci-test-gate-no-skips: the OS census masks comments through the shared scanner and now skips block comments too; the e2e harness's non-unix items are retired, the unix forms stand unconditional; the unix-gated verify cases stay in the suite root.
@task_item n=4 part=touches
decision/a11y-inside-existing-suites, decision/a11y-runs-in-existing-test-suites and decision/a11y-cases-inside-coverage-suite: the suites that hold the a11y literal cases were split into module files, every case name and case row kept.
@task_item n=5 part=touches
decision/fixed-cli-strings-pinned-by-literal, decision/fixed-cli-strings-pinned and decision/refusal-layout-literal-pins: the verify step-line, refusal and stamp cases holding those literals moved into new module files unchanged.
@task_item n=6 part=touches
contract/viola-verify/step-line, contract/viola-verify/interrupted-pair and contract/interrupted-verify/*: their witnessing verify cases moved with the suite split, no string changed.
@task_item n=7 part=touches
decision/boundary-tests-sequenced-on-records, decision/live-readings-never-gate and decision/no-screen-content-parsing: the verify ledger's input-box signature and screen-check cases moved into split test modules unchanged.
@task_item n=8 part=touches
decision/stdin-classifier-non-editing-list, decision/statusline-output-passed-byte-for-byte and decision/claude-agents-read-fails-open: the fake agent that stands in for claude in their boundary tests had its input reader, statusline runner and agents output split into modules, behaviour unchanged.
@task_item n=9 part=touches
decision/tui-boundary-driver-pin, decision/zero-viola-bytes-on-the-terminal and contract/tui-passthrough/*: the passthrough driver's modules gained only a test-only snapshot helper; no driver behaviour changed.
@task_item n=10 part=touches
decision/strict-modes-refusal-before-any-write, decision/unstattable-home-is-strict-modes-refusal and contract/start-refusal/home-strict-modes: the strict-modes crate lost a dead presence check with no caller; refusals unchanged.
@task_item n=11 part=touches
contract/hook-call/server-verify-failed, contract/hook-call/socket-dir-refused and contract/hook-call/strict-modes-failed: the hook handler gained a shared test helper for both hook paths refusing; the refusals are unchanged.
@task_item n=12 part=touches
decision/run-diagnostics-off-terminal and contract/log-identity/*: the MCP server gained a case that its oversize stdin line writes one WARN parse-rejected log line, under a scoped test subscriber with no global install.
@task_item n=13 part=touches
decision/mutation-audit-covers-crate-invariants and decision/zero-flakiness: named mutation survivors were closed by controls that hold, with no cargo-mutants run and no retry.
@task_item n=14 part=touches
decision/gate-requires-coverage-doctest and decision/a11y-gate-breach-fails-build: every gate ran green, coverage included; no gate threshold, floor or step changed.
@task_item n=15 part=touches
contract/structured-violation-json/*: test totals moved (state 285, MCP 87, e2e 302) but no row holds a count; flag only where a cell holds one.
@task_item n=16 part=untouched
The colour and SGR rows (word beside every SGR, amber DIALOG, bold NAME, dim stale row, no SGR off a terminal): no rendering code changed.
@task_item n=17 part=untouched
The viola list board, caption, unwrapped separator and NAME truncation rows: no board output changed.
@task_item n=18 part=untouched
The CLI exit-code rows, the revive refusal and revive --json rows, the link and unlink rows, plugin install and --help rows: no verb, code word, hint or exit code changed.
@task_item n=19 part=untouched
The manual screen-reader and keyboard pass rows, their targets and setups: no manual pass ran or changed.
@task_item n=20 part=untouched
The a11y tier, compliance, WCAG and scope rows, and the absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit): nothing in scope changed.
@task_item n=21 part=untouched
The config.json, budget gate, dialog hold, wait timeout and send refusal rows: no value or bound changed.
@task_item n=22 part=untouched
The CI workflow rows (a11y checks in GitHub Actions, no own a11y artifact, no separate a11y driver): no CI step or workflow changed.
