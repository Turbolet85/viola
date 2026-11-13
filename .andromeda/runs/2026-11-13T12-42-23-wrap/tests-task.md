@task_prose part=session
Epoch 8 cleanup: test-side refactor with no product API, schema, dependency or CI-step change.
One shared source-masking scanner (comments, or comments and literals) replaced the hand scanners of the three source census tests (OS-gated tests, bounded readers, error bodies); census verdicts unchanged.
The secret scan's cookie class is retired: a Cookie header line now reads as no hit; the remaining classes claude-stripped, content-canary, mode and unreadable each hit a planted case (unreadable newly planted).
The largest test modules split (the fake agent binary into a module directory, the ledger test cases, the verify CLI tests); every test name and case row kept.
Dead code removed (a pub strict-modes presence check with no caller, the harness's non-unix 21-row ledger twin); named mutant survivors closed by 25 controls, with no cargo-mutants run.
Test homes and endpoint dirs are now made through one shared private-dir test helper; the MCP tap got its own e2e cases; the viola-mcp oversize-line WARN is pinned with a test-local scoped subscriber.
@task_file n=1 path=tests/support/source_scan.rs
what: the one shared source-masking scanner of the census tests (mask comments, or comments and literals), with its own case table
@task_file n=2 path=tests/support/home.rs
what: the shared test-home support: per-test home and the private-dir helper through which test homes and endpoint directories are made private
@task_file n=3 path=tests/contract_os_gated_tests.rs
what: the OS-gated tests census: every cfg-gated test registered with a reason in GATES, comments masked through the shared scanner
@task_file n=4 path=tests/contract_bounded_readers.rs
what: the bounded-readers census: every external reader keeps the frame cap; the fake agent's module directory left out
@task_file n=5 path=tests/contract_error_bodies.rs
what: the error-bodies census: every product error writer named, bodies hold no path; the fake agent's module directory left out
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the fake agent binary root (viola-fake-agent serving the claude boundary), now over its module directory
@task_file n=7 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's agents listing: the session registry, planted rows and live sessions it answers
@task_file n=8 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input reader: keystrokes, bracketed paste and submit
@task_file n=9 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status line command execution (words, shell expansion, echo)
@task_file n=10 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's unit test module and its shared case helpers
@task_file n=11 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: fake agent unit cases: command-line options, input, turn script, hook commands, resume
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: fake agent unit cases: statusline replay, dialog replay, hooks, agents listing modes, compact replay
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger probe unit test module root (verify ledger rows and stamps)
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: ledger unit cases: the 24 ledger rows, capture plugin, stamp merge and the version gate's relied rows
@task_file n=15 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: ledger unit cases: recorder scrub, screen signatures and box rows, dialog and agents checks
@task_file n=16 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: ledger unit cases: agents join, resume session, paste and local-command post-conditions, statusline checks
@task_file n=17 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: ledger unit cases: framing variants, dialog probes and the probe answer mapping
@task_file n=18 path=tests/cli_verify.rs
what: the viola verify CLI integration binary root; its cfg(unix) gated cases stay here
@task_file n=19 path=tests/cli_verify/stamps.rs
what: viola verify integration cases: stamps, refusals, identity strip, spawn pairs, the recorder
@task_file n=20 path=tests/cli_verify/runs.rs
what: viola verify integration cases: recorded screens, run dirs, dialog and framing rows, hook capture, concurrent verify
@task_file n=21 path=crates/viola-e2e/tests/mcp_tap.rs
what: e2e cases of the test-only MCP tap: usage refusal, fixture writing, dirty-capture refusal, record of a viola mcp exchange
@task_file n=22 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only MCP tap that records the MCP exchange and writes its scrubbed fixture
@task_file n=23 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan and its hit classes (cookie class retired; unreadable now planted)
@task_file n=24 path=crates/viola-e2e/src/harness/mod.rs
what: the harness core, including the e2e home headroom check (free bytes as blocks times block size)
@task_file n=25 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command (non-unix ledger row twin removed)
@task_file n=26 path=crates/viola-state/src/strict.rs
what: the strict-modes write and read rules (dead presence check removed)
@task_file n=27 path=crates/viola-state/src/fs.rs
what: private placement of files and directories (create_private_dir, mode-first placement); a too-long-name case added
@task_file n=28 path=crates/viola-state/src/pin.rs
what: the pinned executable copy placement; an unreadable-name case added
@task_file n=29 path=crates/viola-mcp/src/lib.rs
what: the viola mcp server; a test pinning one WARN parse-rejected line for an oversize stdin line under a test-local scoped subscriber
@task_file n=30 path=crates/viola-mcp/src/stdin.rs
what: the viola mcp stdin line reader under the line cap; a short-buffer case added
@task_file n=31 path=src/cmd/hook.rs
what: the viola hook exec-form entry points; a shared test helper for both hook paths refusing
@task_file n=32 path=src/run/mod.rs
what: the viola run wrapper; a test-only instance snapshot builder
@task_file n=33 path=src/run/links.rs
what: the wrapper's link and unlink records; test homes made through the shared helper
@task_file n=34 path=src/run/snapshot.rs
what: the instance snapshot of the wrapper; test homes made through the shared helper
@task_file n=35 path=tests/run_cli.rs
what: viola run CLI integration tests; endpoint directories made through the shared private-dir helper
@task_file n=36 path=tests/hook_fail_open.rs
what: hook fail-open integration tests; test homes made through the shared helper
@task_file n=37 path=tests/security_negatives_server_identity.rs
what: server identity security negative tests; test homes made through the shared helper
@task_file n=38 path=tests/cli_controls_not_disableable.rs
what: controls-not-disableable integration tests; test homes made through the shared helper
@task_file n=39 path=tests/cross_exit_causes.rs
what: cross-surface exit-cause integration tests; test homes made through the shared helper
@task_file n=40 path=.claude/docs/services/viola-state.md
what: the viola-state service doc (strict-modes, private placement and pin APIs)
@task_item n=1 part=touches
contract/secret-scan-class/cookie: the cookie hit class is retired; the row goes.
@task_item n=2 part=touches
decision/logs-carry-no-secrets and decision/no-secrets-in-logs: their Cookie clause drops with the retired class.
@task_item n=3 part=touches
decision/secrets-never-in-logs: its Cookie clause drops with the retired class.
@task_item n=4 part=touches
decision/secret-scan-scope and the remaining contract/secret-scan-class rows (claude-stripped, content-canary, mode, unreadable): the class list loses cookie; unreadable now has its planted case.
@task_item n=5 part=touches
decision/one-shared-test-helper: the shared source-masking scanner and the private-dir helper join the shared helpers.
@task_item n=6 part=touches
decision/test-homes-made-private, decision/test-endpoint-dirs-made-private, decision/private-dir-for-test-endpoints: test homes and endpoint dirs are made through one shared helper over create_private_dir (still true).
@task_item n=7 part=touches
decision/os-gated-tests-registered and decision/os-gated-tests-each-named: the census now masks comments through the shared scanner and skips block comments; it walks only files directly under tests/ and crates/*/tests/.
@task_item n=8 part=touches
decision/bounded-readers-keep-the-cap and decision/bounded-readers: the census leaves out the fake agent's whole module directory.
@task_item n=9 part=touches
decision/error-writers-each-named, decision/error-bodies-held-by-surface-witnesses, decision/error-bodies-hold-no-path: the error-bodies census leaves out the fake agent's whole module directory.
@task_item n=10 part=touches
decision/mcp-exchange-recorded-by-tap and decision/mcp-tap-refuses-dirty-exchange: the tap is now exercised by its own e2e cases.
@task_item n=11 part=touches
decision/mcp-stdin-line-cap, contract/mcp-stdin/line-cap, contract/log-event-corr/parse-rejected: an oversize line now pinned to write one WARN parse-rejected line.
@task_item n=12 part=touches
decision/scoped-capture-pinned-dispatch and decision/one-global-tracing-install: the viola-mcp WARN case carries a test-local scoped subscriber, no global install, no tracing-subscriber dev-dependency.
@task_item n=13 part=touches
decision/e2e-home-headroom-floor and contract/e2e-home-refusal/message: free bytes restated as available blocks times block size (behaviour unchanged).
@task_item n=14 part=touches
decision/peer-admitted-survivor-known-limit, decision/host-unreachable-mutants, decision/mutation-run-verdict: named mutant survivors closed by controls; no cargo-mutants run this session.
@task_item n=15 part=touches
decision/fake-agent-as-claude and the contract/fake-agent-* rows: the fake agent binary is split into a module directory with behaviour unchanged.
@task_item n=16 part=touches
decision/integration-binary-surface-prefix and decision/test-function-naming: the verify CLI binary now has submodules in a directory under tests/; every test name kept.
@task_item n=17 part=touches
decision/strict-modes-two-rules: a dead pub presence check removed from the strict-modes component (no rule change).
@task_item n=18 part=touches
decision/private-placement-mode-first and decision/placed-copy-execs-at-once: new cases on private placement and the pinned copy (too-long name, unreadable name).
@task_item n=19 part=touches
absence/windows-and-macos-support and decision/linux-only: the harness's cfg(not(unix)) items are removed.
@task_item n=20 part=touches
decision/ledger-row-verify-probe, decision/version-gate-needs-all-relied-rows, decision/stamps-refuse-widened-ledger: their ledger unit cases moved into split modules, unchanged.
@task_fact n=1 slug=test-strategy
fact: Every source census test (OS-gated tests, bounded readers, error bodies) masks source through the one shared scanner, comments only or comments and literals; no census carries its own scanner.
@task_fact n=2 slug=test-strategy
fact: The OS-gated census walks only files directly under tests/ and crates/*/tests/, so a binary split into a tests/<bin>/ directory keeps its gated cases in its root file.
@task_fact n=3 slug=test-strategy
fact: The bounded-readers and error-bodies censuses leave out the fake agent binary and every file of its module directory.
@task_fact n=4 slug=5-command-implementation
fact: The secret scan reads a Cookie header line as no hit; its hit classes are claude-stripped, content-canary, mode and unreadable, each proved by a planted case.
@task_fact n=5 slug=7-test-data-fixtures
fact: The test-only MCP tap is itself tested by e2e cases: usage refusal, fixture of a clean capture, refusal of a dirty capture by file name and code alone, and a recorded viola mcp exchange both ways.
@task_item n=1 part=untouched
CLI exit codes and the --json documents of the verbs (send, wait, last, list, release, revive, answer): no output or code changed.
@task_item n=2 part=untouched
Wrapper channel and JSON-RPC error contracts, link methods and MCP tool result bodies: no surface changed.
@task_item n=3 part=untouched
Budget governor thresholds, statusline payload and budget.json contracts: untouched.
@task_item n=4 part=untouched
Performance budgets, hyperfine gating, coverage floors and mutation gate thresholds: no value moved.
@task_item n=5 part=untouched
CI workflows, artifacts, supply-chain gates (cargo-deny, zizmor, SHA pins, toolchain) and fuzzing: no workflow, manifest or lockfile changed.
@task_item n=6 part=untouched
Harness CLI contracts (boot, status, cleanup, gate, pre-push documents, exit codes and reasons): no agent-run verb or status shape changed.
@task_item n=7 part=untouched
Dialog decision bodies, send confirmation and the wheel rules: no product behaviour changed.
