@task_prose part=chunk
Epoch 8 cleanup, the corrective head of 0.2.0: test support, test files, dead code and verdict-keeping refactors only; no product behaviour changes.
Builds one separately tested source-masking scanner in test support (masks comments, and by a flag string literals, out of Rust source) replacing three hand scanners whose callers keep their verdicts; merges two duplicated function pairs (hook entry points; links and snapshot) into one each.
Splits three files over 800 lines (the ledger test cases, the fake agent binary, the verify CLI tests) to at most 800 lines each, bodies byte-identical, every test name and the case count kept.
Deletes or wires the dead strict-modes presence check and the dead non-unix constants of the e2e run harness; removes the secret scan's cookie class (header and GUI cookie pair) while every other class still catches; adds one viola-made (0700) test-home helper, one remove-the-guard control helper and one named host temp root.
Closes named mutant survivors by tests the plan writes (or, for the test-only MCP tap, a recorded exclusion from the mutants scope), never by a mutation result: no mutation run and no live claude start in this chunk; P3 re-reads every coordinate at HEAD.
@task_file n=1 path=tests/support/mod.rs mark=edits
what: the shared integration-test support helpers, home of the one shared copy of each test helper — edits: the new source-masking scanner, the viola-made home helper, the remove-the-guard control helper, the named test temp root
@task_file n=2 path=tests/contract_error_bodies.rs mark=edits
what: the error-body surface witness check over product source, reporting place and class — edits: its hand scanner `blanked`, replaced by the shared scanner
@task_file n=3 path=tests/contract_bounded_readers.rs mark=edits
what: the check that every external reader is bounded and keeps the frame cap — edits: its hand scanner `code_only`, replaced by the shared scanner
@task_file n=4 path=tests/contract_os_gated_tests.rs mark=edits
what: the census that every OS-gated test is registered and named with a reason — edits: its census scanner, replaced by the shared scanner
@task_file n=5 path=src/cmd/hook.rs mark=edits
what: the viola hook exec-form entry points (fail-open hooks, dialog decision bodies, statusline) — edits: one duplicated function pair merged into one
@task_file n=6 path=src/run/links.rs mark=edits
what: the wrapper's served link and unlink methods and their link records — edits: the function duplicated with the snapshot module, merged into one
@task_file n=7 path=src/run/snapshot.rs mark=edits
what: the instance snapshot writer and its read-or-replay from the event log — edits: the function duplicated with the links module, merged into one
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the ledger's table-driven test cases (every ledger row and its verify probe) — edits: split into files of at most 800 lines, bodies and names unchanged
@task_file n=9 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary serving the claude boundary (argv modes, receipts, turn scripts, hook execution) — edits: split into files of at most 800 lines, behaviour unchanged
@task_file n=10 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI integration tests (step and summary lines, process-exit details, fixture recording) — edits: split into files of at most 800 lines, every test name kept
@task_file n=11 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes write and read rules of the state crate — edits: the dead presence check `check_present`, deleted or wired and tested
@task_file n=12 path=.claude/docs/services/viola-state.md mark=edits
what: the state crate's service doc — edits: its one mention of the presence check
@task_file n=13 path=crates/viola-state/src/fs.rs mark=edits
what: the state crate's private file placement and reads — edits: tests for the error-kind guard at its named survivor
@task_file n=14 path=crates/viola-state/src/pin.rs mark=edits
what: the pinned executable copy and its content hash — edits: tests for the error-kind guard at its named survivor
@task_file n=15 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp server (tools, tool results, parse rejection) — edits: a test asserting the WARN parse-rejected log line
@task_file n=16 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the viola mcp stdin line reader and its MAX_FRAME line cap — edits: a test with a buffer smaller than the pending line; the CHUNK constant marked equivalent
@task_file n=17 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the test-only MCP tap that records and scrubs the MCP exchange — edits: tests, or its recorded exclusion from the mutants scope
@task_file n=18 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the e2e harness core, including the headroom check of the e2e home base — edits: tests for `free_bytes`
@task_file n=19 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the harness run command (suites, mutants run, gate) — edits: the three dead `#[cfg(not(unix))]` constants removed
@task_file n=20 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan and its hit classes — edits: the cookie class removed with its tests
@task_item n=1 part=needs
The secret-scan hit classes: cookie leaves (generic cookie header and the GUI cookie pair), and claude-stripped, content-canary, mode and unreadable still catch, with the scan's output, exit codes and hits file unchanged.
@task_item n=2 part=needs
One shared copy of each test helper, never copied: the scanner, the home helper and the remove-the-guard helper each live once in test support and the three scanner callers use the one copy.
@task_item n=3 part=needs
The shared scanner is separately tested and the three checks it serves keep their verdicts: error bodies held by surface witnesses and reported by place and class, bounded readers keeping the frame cap, OS-gated tests registered with a reason.
@task_item n=4 part=needs
Mutation scope and survivors: survivors close by tests the plan writes or a recorded exclusion with a reason (host-excluded or out-of-scope recording), never by a mutation result; no mutation run in this chunk, the next epoch-boundary audit reads them.
@task_item n=5 part=needs
Test homes made the way viola makes them: private 0700 directories, one per test, so standing tests no longer redden on a 0755 home; the home read rule refusing a home another user may read binds the helper.
@task_item n=6 part=needs
The bounded remove-the-guard witness: the control that removes a guard and expects red becomes one project script or test helper.
@task_item n=7 part=needs
The test temp root named once for the host: e2e and test homes under one root, the dev-host tmpfs home base and its link, the headroom floor of that base and its free-bytes reading, and the headroom refusal message.
@task_item n=8 part=needs
Strict-modes two rules: the presence check is deleted with its doc mention or wired into the rules and tested; the strict-modes refusals at every surface keep their verdicts.
@task_item n=9 part=needs
MCP stdin line cap: a line past MAX_FRAME and a read buffer smaller than the pending line both keep the cap and the oversize behaviour.
@task_item n=10 part=needs
The parse-rejected process log line: its WARN level, event and corr fields are what the new viola-mcp test asserts.
@task_item n=11 part=needs
The test-only MCP tap: the MCP exchange recorded by the tap and the tap refusing a dirty exchange whole, with its scrub refusal codes, are what its new tests exercise.
@task_item n=12 part=needs
The fake agent split keeps the fake agent as the contract held to recorded fixtures: its command line, receipts, turn script, hook execution and input handling unchanged byte for byte.
@task_item n=13 part=needs
The verify CLI tests split keeps every viola verify output, process-exit detail and recording-refusal assertion and the cases' count; new integration binaries it creates take the surface prefix.
@task_item n=14 part=needs
The ledger test cases split keeps every ledger row's verify probe and the table-driven cases unchanged.
@task_item n=15 part=needs
The hook merge keeps hook behaviour: hooks fail open, deadlines, exit code, stderr and stdout, decision bodies and the statusline passthrough unchanged.
@task_item n=16 part=needs
The links and snapshot merge keeps link records and snapshot reading: link idempotent and keeping its first since, unlink surviving a restart, every snapshot reader through read-or-replay and replay writing no file.
@task_item n=17 part=needs
Linux only: the dead non-unix constants go because the tree carries no Windows or macOS code, tests or CI jobs.
@task_item n=18 part=needs
The error-kind guards of the state crate's file placement and pinned copy: private placement mode-first and the pinned executable hash check are what the new tests pin.
@task_item n=19 part=needs
Real claude never started: no live claude in this chunk and none in CI; all new tests run over the fake agent.
@task_standing n=1
convention: every new test function follows the test function naming rule and every test name is kept across a split
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: every new integration test file takes the root binary surface prefix and runs under nextest by layer filterset
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: every new test creates and owns its private home, writes no hand-made product state and leaves no shared mutable state
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: every new test asserts literal oracles on contracts, with no sleep synchronisation, no retries, no Debug-string comparison and no process-wide env mutation
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: unit tests sit inline in their module and table-driven cases use rstest
home: 2-test-strategy · 4-unit-test-strategy
@task_item n=1 part=not_needed
Budget governor rows (pause thresholds, budget.json, budget override, release --budget): the chunk changes no budget behaviour.
@task_item n=2 part=not_needed
Dialog hold ceiling and dialog hold expiry: the scope names the dialog-hold ceiling as not this entry's.
@task_item n=3 part=not_needed
Send confirmation, wheel and human-typing rows (send refusals, confirmation windows, human input taking the wheel): no send or wheel behaviour changes.
@task_item n=4 part=not_needed
Revive refusal codes, revive exit codes and revive JSON documents: revive is untouched.
@task_item n=5 part=not_needed
viola list human board and list --json rows: the board and its document are untouched.
@task_item n=6 part=not_needed
Fuzz targets, fuzz corpus replay and nightly fuzzing: no parser or fuzz target changes.
@task_item n=7 part=not_needed
pre-push stage, detail, reason and exit-code contracts: the local pre-push gate is not changed.
@task_item n=8 part=not_needed
Harness boot, status, supervise and cleanup contracts and the agent-run status document: those commands are not changed.
@task_item n=9 part=not_needed
CI workflow rows (SHA-pinned actions, zizmor, artifact retention, rust-cache, workflow permissions): no workflow changes.
@task_item n=10 part=not_needed
Real-claude live verify and local-live suite rows: no live claude start and no fixture recording in this chunk.
@task_item n=11 part=not_needed
The run --mutants verdict contracts (counted, package, scoped, unviable-exceeds-caught): no mutation run happens here.
@task_item n=12 part=not_needed
Performance budgets and hyperfine invocation rows: no hook timing is measured or changed.
