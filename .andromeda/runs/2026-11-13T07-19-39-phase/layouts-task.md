@task_prose part=chunk
Epoch 8 cleanup: test support, test files, dead code and verdict-keeping refactors; no product behaviour changes.
Builds one shared source-masking scanner in test support for three contract tests, one viola-made-home (0700) test helper, one remove-the-guard control helper, and one named temp root for the host.
Splits the three largest over-800-line files (test files first) with bodies byte-identical and every test name kept; merges two duplicated function pairs.
Deletes or wires the dead strict-modes presence check, removes the dead non-unix constants, and drops the secret scan's cookie class.
Closes the named mutant survivors by new tests or a recorded mutants-scope exclusion, with no mutation run and no live claude start; P3 re-measures every coordinate at HEAD.
@task_file n=1 path=tests/contract_error_bodies.rs mark=edits
what: the contract test over error bodies, its verdicts unchanged — edits: its hand scanner `blanked`, replaced by the shared scanner
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the contract test over bounded readers, its verdicts unchanged — edits: its hand scanner `code_only`, replaced by the shared scanner
@task_file n=3 path=tests/contract_os_gated_tests.rs mark=edits
what: the census contract test of OS-gated tests, its verdicts unchanged — edits: its census scanner, replaced by the shared scanner
@task_file n=4 path=tests/support/ mark=edits
what: the shared test support — the new source-masking scanner (comments, and string literals by a flag) with its own tests, the viola-made-home helper, the remove-the-guard control helper and the host's temp root
@task_file n=5 path=tests/cli_verify.rs mark=edits
what: the test file of the viola verify verb's output and refusals, split to at most 800 lines per file, every test name kept
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the ledger's test cases, split to at most 800 lines per file, moved bodies byte-identical, case count unchanged
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary the tests drive in place of claude, split to at most 800 lines per file, behaviour unchanged
@task_file n=8 path=src/cmd/hook.rs mark=edits
what: the hook verb's command code — edits: the duplicated function pair, merged into one, behaviour unchanged
@task_file n=9 path=src/run/links.rs mark=edits
what: the run's link handling — edits: the function duplicated with the snapshot code, merged into one, behaviour unchanged
@task_file n=10 path=src/run/snapshot.rs mark=edits
what: the run's snapshot code — edits: the function duplicated with the link code, merged into one, behaviour unchanged
@task_file n=11 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes checks over viola's home and state files — edits: the dead presence check `check_present`, deleted or wired and tested
@task_file n=12 path=.claude/docs/services/viola-state.md mark=edits
what: the viola-state service doc — edits: its one mention of the dead presence check
@task_file n=13 path=crates/viola-state/src/fs.rs mark=edits
what: viola-state's file handling — edits: the error-kind guard a mutant survives, closed by a test
@task_file n=14 path=crates/viola-state/src/pin.rs mark=edits
what: viola-state's pin handling — edits: the error-kind guard a mutant survives, closed by a test
@task_file n=15 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp server — edits: a test asserting the WARN parse-rejected line
@task_file n=16 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the viola mcp server's stdin reader — edits: a test with a buffer smaller than the pending line; the CHUNK constant marked equivalent
@task_file n=17 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the end-to-end test-only MCP tap binary — tests, or a recorded exclusion from the mutants scope
@task_file n=18 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the end-to-end harness — edits: `free_bytes`, its two survivors closed by tests
@task_file n=19 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the end-to-end harness's run driver — edits: the three dead non-unix constants, removed
@task_file n=20 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan — edits: its cookie class (the cookie header and the GUI cookie pair), removed with its tests; every other class still caught
@task_item n=1 part=needs
The viola verify verb's output and refusals: the tests that assert them move between files unchanged, every assertion kept, so the verify output and refusal rows stay what those moved tests pin.
@task_item n=2 part=needs
Strict-modes refusal across verbs: the dead presence check sits in the strict-modes checks, so whether it is deleted or wired must leave every verb's strict-modes refusal and its strict-modes-failed --json detail unchanged.
@task_item n=3 part=needs
Viola's home kept private: the viola-made-home helper makes a test home the way viola makes one (0700), so the home-not-private and home-tree refusals stay the behaviour those tests rely on.
@task_item n=4 part=needs
The viola mcp server has no human surface and logs to stderr: the new test asserts its WARN parse-rejected line, a log line the server already writes.
@task_item n=5 part=needs
The hook verb's and the run's code is merged, not changed: any human or --json output those paths produce, including CLI refusal lines and exit codes, stays byte-for-byte.
@task_item n=6 part=needs
The fake agent stands in for claude in the CLI tests: splitting it must keep every behaviour the run, send, wait, list and revive output tests read from it.
@task_item n=7 part=needs
Human output is a contract: refactors and splits in this chunk keep every verdict, so tests asserting verb output keep their exact expected text.
@task_item n=8 part=needs
No live claude start: the viola verify run against a real claude CLI is not exercised by this chunk; its tests stay on the fake agent.
@task_standing n=1
convention: every test that asserts a verb's human or --json output asserts the exact contracted text, never a looser match
home: surface-cli
@task_item n=1 part=not_needed
A web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
Colour, styling and token rendering rows (SGR styling, colour depth, token roles, spacing, motion): no styling code is touched.
@task_item n=3 part=not_needed
The board's layout rows (column widths, ordering, truncation, escaping, stale rows, the BAY line): the board's rendering is not edited.
@task_item n=4 part=not_needed
Hint wording rows (a hint per cause, hints name the next verb, hint position): no hint text is added or changed.
@task_item n=5 part=not_needed
Send readback and progress rows (readback mirror, issue line, no spinner, appended send lines): the send path is not edited.
@task_item n=6 part=not_needed
The MCP tool set and its tool results (answer, last, list, send, wait; hidden verbs): only a log-line test and a buffer test are added, no tool changes.
@task_item n=7 part=not_needed
Verb-set and parsing rows (flat verbs, clap derive, --help groups, global flags, prompt from stdin or file): no verb, flag or usage changes.
@task_item n=8 part=not_needed
Wheel, handoff, link and release-budget output rows: none of those verbs' output is touched.
