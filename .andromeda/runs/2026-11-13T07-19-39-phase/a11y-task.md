@task_prose part=chunk
Epoch 8 cleanup: a behaviour-neutral pass over test support, test files, dead code and two refactor merges; every verdict and every product output stays as it is.
It adds one shared, separately tested source-masking scanner (comments, and by a flag string literals) in test support, replacing the three hand scanners of the error-body, bounded-reader and OS-gated-test census checks, whose verdicts stay unchanged.
It merges two duplicated function pairs (inside the hook command, and between the run's link set and its snapshot store), deletes or wires the dead strict-modes presence check, deletes three dead non-unix constants in the e2e run harness, and removes the e2e secret scan's cookie class.
It splits three files over 800 lines (the claude ledger test cases, the fake agent binary, the verify CLI tests) with bodies byte-identical and every test name kept, and adds one viola-made-home (0700) test helper, one remove-the-guard control helper and one named temp root.
It closes named mutant survivors by tests only (or the MCP tap's recorded exclusion from the mutants scope); no mutation run and no live claude start happen in this chunk; P3 re-measures every triage coordinate at HEAD.
@task_file n=1 path=tests/contract_error_bodies.rs mark=edits
what: the error-body witness check that every error body has its writer and fixed text — edits: its hand scanner `blanked`, replaced by the shared scanner
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract check — edits: its hand scanner `code_only`, replaced by the shared scanner
@task_file n=3 path=tests/contract_os_gated_tests.rs mark=edits
what: the census check of OS-gated tests on a Linux-only tree — edits: its census scanner, replaced by the shared scanner
@task_file n=4 path=tests/support/mod.rs mark=edits
what: the shared test-support module that holds the one shared test helpers (new scanner, viola-made-home helper, remove-the-guard control helper, the named temp root)
@task_file n=5 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI tests pinning verify's step lines and interrupted pair by literal — split to at most 800 lines per file, every test name kept
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the claude ledger's test cases — split to at most 800 lines per file, case count unchanged
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary standing in for claude in the boundary and CLI tests — split to at most 800 lines per file, behaviour unchanged
@task_file n=8 path=src/cmd/hook.rs mark=edits
what: the viola hook event, dialog and statusline calls and their refusals — edits: one duplicated function pair merged into one
@task_file n=9 path=src/run/links.rs mark=edits
what: the run's link set behind viola link and viola unlink — edits: the function duplicated with the snapshot store, merged into one
@task_file n=10 path=src/run/snapshot.rs mark=edits
what: the run's instance snapshot store — edits: the function duplicated with the link set, merged into one
@task_file n=11 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes checks behind the strict-modes-failed refusals — edits: the dead presence check `check_present`, deleted or wired and tested
@task_file n=12 path=.claude/docs/services/viola-state.md mark=edits
what: the viola-state service doc — edits: its one mention of the strict-modes presence check
@task_file n=13 path=crates/viola-state/src/fs.rs mark=edits
what: viola-state's file-system writes — edits: the error-kind guard whose mutant survives, closed by a test
@task_file n=14 path=crates/viola-state/src/pin.rs mark=edits
what: viola-state's pinned-hash reads — edits: the error-kind guard whose mutant survives, closed by a test
@task_file n=15 path=crates/viola-mcp/src/lib.rs mark=edits
what: the MCP server — edits: a test asserting its WARN parse-rejected log line
@task_file n=16 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the MCP server's stdin line reader — edits: a test with a buffer smaller than the pending line; the CHUNK constant marked equivalent
@task_file n=17 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the test-only MCP tap of the end-to-end harness — tests for it, or its recorded exclusion from the mutants scope
@task_file n=18 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the end-to-end harness — edits: `free_bytes`, its two surviving mutants closed by tests
@task_file n=19 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the end-to-end harness's run driver — edits: the three dead `#[cfg(not(unix))]` constants, deleted
@task_file n=20 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan of captured output — edits: its cookie class (the generic cookie header and the GUI cookie pair), removed; every other class still caught
@task_item n=1 part=needs
The error-body check that every error body has a writer and a witness keeps its verdict when its hand scanner gives way to the shared scanner.
@task_item n=2 part=needs
The literal pins that hold fixed CLI strings and the refusal layout (unable line, hint line, exit code) stay byte-identical when their test files are split or re-pointed at shared helpers.
@task_item n=3 part=needs
viola verify's output — the step line and the interrupted pair (stderr lines and exit 1) — keeps every test that pins it, under the same names, after the verify tests are split.
@task_item n=4 part=needs
The tree is Linux only: the OS-gated test census keeps its verdict, and the dead non-unix constants in the e2e run driver leave with no non-unix path left behind.
@task_item n=5 part=needs
The hook event and dialog calls keep their refusals (strict-modes-failed, socket-dir-refused, server not the recorded wrapper) and the statusline call its exit status and byte-for-byte output through the hook merge.
@task_item n=6 part=needs
viola link and viola unlink keep their result line (one write, no SGR), exit 20 and 21, and --json output, and link pairs stay validated by the wrapper, through the link-set and snapshot merge.
@task_item n=7 part=needs
The strict-modes refusal comes before any write and an unstattable home stays a strict-modes refusal: deleting or wiring the dead presence check leaves every strict-modes-failed refusal and its detail unchanged.
@task_item n=8 part=needs
The run, revive-start and verify home strict-modes refusal is what the 0755 test homes tripped: the shared helper makes a test home at 0700 the way viola makes one, so those standing tests read the real refusal rule.
@task_item n=9 part=needs
The mutation audit covers invariants held in crate tests: each named survivor is closed by a test the plan writes, or for the MCP tap by a recorded exclusion, never by a mutation result produced here.
@task_item n=10 part=needs
The MCP server's WARN parse-rejected log line, with its log identity tags (process, service_name), is the value the new viola-mcp test asserts.
@task_item n=11 part=needs
thiserror messages interpolate nothing and refusal messages are fixed: the error-kind guards in viola-state's file writes and pinned-hash reads are closed by tests that check the fixed kind, not a formatted text.
@task_item n=12 part=needs
The boundary tests stay sequenced on records and never inspect claude content: the fake agent standing in for claude keeps its behaviour after its split, and no live claude start runs in this chunk.
@task_item n=13 part=needs
Live readings never gate: no check added here depends on a live claude session or a mutation run.
@task_item n=14 part=needs
The e2e harness's capture of viola terminal bytes and run diagnostics keeps its verdicts while the secret scan drops its cookie class and keeps every other class.
@task_item n=15 part=needs
The one shared test helper decision rules the new scanner, home helper, remove-the-guard control and named temp root: each lives once in test support and the callers use it.
@task_item n=16 part=needs
Code words and their detail (strict-modes-failed, socket-dir-refused, server-verify-failed) in instance-unreachable and frame-verb refusals stay as they are; this chunk's product-source edits change no printed word.
@task_standing n=1
convention: every new or moved test runs as an ordinary nextest case in the existing suites; its failure is a nextest failure and the CI test gate allows no skip of it
home: bootstrap-phases-derive-for-route-setup-project · 1-a11y-scope-summary
@task_standing n=2
convention: every new test is deterministic — zero flakiness, no retries, and a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: new checks run inside the coverage suite of the push/PR workflow, with no separate driver, suite value or artifact of their own
home: 9-ci-integration
@task_standing n=4
convention: the CI gate a new test joins requires coverage and doctest to pass alongside it
home: ci-integration
@task_item n=1 part=not_needed
Colour and SGR rows (amber dialog word, bold names, dim stale rows, colour decision inputs, NAME truncation): the chunk changes no styled output.
@task_item n=2 part=not_needed
The viola list board and caption rows (order, empty board, unwrapped separator, caption order): the list command is not touched.
@task_item n=3 part=not_needed
Manual screen-reader and keyboard passes (Orca targets, manual supplements, screen-reader test setup): nothing manual is added or re-run.
@task_item n=4 part=not_needed
The absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit) and the browser a11y leg: no such surface arises.
@task_item n=5 part=not_needed
The budget gate on sends, config.json fields and the dialog-hold bounds: the dialog-hold ceiling is explicitly not this entry's and no send path changes.
@task_item n=6 part=not_needed
viola revive --json and human refusal rows and revive --list output: revive is not touched.
@task_item n=7 part=not_needed
viola send refusals (input-not-ready, no-prompt-submitted) and the human-takes-the-wheel rules: the send and wheel paths are not touched.
@task_item n=8 part=not_needed
The structured violation JSON schema and WCAG criteria mapping: no violation report is produced or reshaped.
@task_item n=9 part=not_needed
viola --help rows (usage line, group labels, handoff row, served verbs) and the CLI exit-code table beyond the codes named in the needs: no verb or help text changes.
