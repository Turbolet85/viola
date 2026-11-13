@task_prose part=chunk
Epoch 8 cleanup: test support, test-file splits, dead code and behaviour-keeping refactors; no product behaviour changes and every verdict kept.
It builds one shared source-masking scanner, one viola-made-home (0700) test helper, one remove-the-guard control helper and one named temp root for the host.
It merges two duplicated function pairs (the hook verb's pair; the run's links and snapshot pair), deletes or wires the dead strict-modes presence check, and deletes the dead non-unix constants of the end-to-end harness.
It splits three over-800-line files with bodies byte-identical and every test name kept, closes the named mutant survivors by tests (no mutation run), and removes the secret scan's cookie class.
It re-measures every triage coordinate at HEAD; no live claude start in this chunk.
@task_file n=1 path=tests/contract_error_bodies.rs mark=edits
what: the error-bodies contract test, its verdicts unchanged — edits: its hand scanner `blanked`, replaced by the shared scanner
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract test, its verdicts unchanged — edits: its hand scanner `code_only`, replaced by the shared scanner
@task_file n=3 path=tests/contract_os_gated_tests.rs mark=edits
what: the OS-gated tests census, its verdicts unchanged — edits: its census scanner, replaced by the shared scanner
@task_file n=4 path=tests/support/ mark=edits
what: the shared test support — edits: the new source-masking scanner and its own tests, the viola-made-home helper, the remove-the-guard control helper, the one named test temp root
@task_file n=5 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI tests, split to at most 800 lines per file, bodies byte-identical, every test name kept
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the Claude agent ledger's test cases, split to at most 800 lines per file, case count unchanged
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary the tests drive in place of the real CLI, split to at most 800 lines per file, behaviour unchanged
@task_file n=8 path=src/cmd/hook.rs mark=edits
what: the hook verb — edits: the duplicated function pair, merged into one, behaviour unchanged
@task_file n=9 path=src/run/links.rs mark=edits
what: the run's link handling — edits: the function duplicated with the snapshot, merged into one
@task_file n=10 path=src/run/snapshot.rs mark=edits
what: the run's snapshot — edits: the function duplicated with the links, merged into one
@task_file n=11 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes checks of the state store — edits: the dead presence check `check_present`, deleted or wired and tested
@task_file n=12 path=.claude/docs/services/viola-state.md mark=edits
what: the state service's doc — edits: the one mention of the presence check
@task_file n=13 path=crates/viola-state/src/fs.rs mark=edits
what: the state store's file access — edits: tests for the error-kind guard survivor
@task_file n=14 path=crates/viola-state/src/pin.rs mark=edits
what: the state store's pin — edits: tests for the error-kind guard survivor
@task_file n=15 path=crates/viola-mcp/src/lib.rs mark=edits
what: the mcp server — edits: a test asserting the WARN parse-rejected log line
@task_file n=16 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the mcp stdin reader — edits: a test with a buffer smaller than the pending line; the CHUNK survivor marked equivalent
@task_file n=17 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the end-to-end test-only mcp tap — edits: its tests, or its recorded exclusion from the mutants scope
@task_file n=18 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the end-to-end harness — edits: tests for the free_bytes survivors; the temp root named in one place
@task_file n=19 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the end-to-end harness's run driver — edits: the three dead non-unix constants, deleted
@task_file n=20 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan — edits: the cookie class (the generic cookie header and the GUI cookie pair), removed with its tests; every other class still caught
@task_item n=1 part=needs
The hook verb's exit code and stderr: the merged pair keeps them exactly as they are.
@task_item n=2 part=needs
The viola link and unlink output and its --json ok documents: the merged links and snapshot pair keeps every line and document the run's link handling gives.
@task_item n=3 part=needs
The strict-modes refusals and the home-tree check, as they reach the presence check: if it is wired rather than deleted, its refusal takes the fixed instance-state or home-state pair, exit 21 and the strict-modes-failed --json cause, and never a path in the message.
@task_item n=4 part=needs
The viola list board's strict-modes scan and refusal, which the presence check would join if wired: a strict-modes failure refuses the whole board.
@task_item n=5 part=needs
The viola verify output and refusals the split verify tests assert: step line, summary line, fail word, stamps.json update refused, CLI not found, version unreadable, unclean fixture, interrupted run, home-tree check, --help.
@task_item n=6 part=needs
The CLI verification state (verified and unverified-cli) the fake agent stands in for: the split binary answers the verify run exactly as before.
@task_item n=7 part=needs
The viola-made home at 0700 the shared helper builds: the home the strict-modes and home-tree checks accept, so tests stop reddening on a 0755 home.
@task_item n=8 part=needs
The mcp verb's stdout and the stdout/stderr split: the WARN parse-rejected line goes to the log, never to the mcp verb's stdout, and the new test asserts it there.
@task_item n=9 part=needs
The CLI streams and catch sites the moved and new tests assert: data on stdout, messages on stderr, silent verbs (hook, mcp, run) kept silent.
@task_item n=10 part=needs
The CLI exit codes (0, 1, 2, 10 to 14, 20, 21) and the exit-21 causes the split and new tests assert, each unchanged.
@task_item n=11 part=needs
The CLI colour decision order for viola hook, viola mcp and viola run: the merged and deleted code keeps those verbs uncoloured.
@task_item n=12 part=needs
The wrapper-fault --json detail and the wrapper socket's refused peer, as far as the mcp stdin and lib tests assert them: unchanged.
@task_standing n=1
convention: every test the chunk moves or adds that asserts a verb's output asserts the fixed forms: data on stdout, messages on stderr, the unable and hint lines in their fixed shapes, the exit code of the contract
home: surface-cli · surface
@task_item n=1 part=not_needed
Brand identity's strip-bay direction, flight-progress strip fields, readback box and handoff display: the chunk adds no output and changes no display.
@task_item n=2 part=not_needed
Colour palette: the arrival amber accent and its guards; no colour is added or changed.
@task_item n=3 part=not_needed
Typography: bold NAME column, terminal font, dim stale rows; no type changes.
@task_item n=4 part=not_needed
Spacing and iconography: two-space columns, text glyphs, list columns, arrows; no column or glyph is drawn by this chunk.
@task_item n=5 part=not_needed
The web-page surface and rendered-page absences, depth strategy and border radius: viola renders no page and this chunk adds none.
@task_item n=6 part=not_needed
Self-validation protocol's squint, swap, domain and readback-box signature tests: design reviews of a display this chunk does not touch.
@task_item n=7 part=not_needed
Send readback and its mirror, viola wait, viola last, the revive --list output, release and pause outputs, refusal hints for sends: no code of those verbs is reached.
