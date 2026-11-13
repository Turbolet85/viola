@task_prose part=chunk
Epoch 8 cleanup, the corrective head of 0.2.0: test support, test files, dead code and verdict-keeping refactors, with no change to product behaviour.
Builds one separately tested source-masking scanner in test support that masks comments (and, by a flag, string literals) out of Rust source; it replaces three hand-written scanners, and its callers keep their verdicts. Adds one test-home helper that makes homes at 0700 the way viola does, one remove-the-guard control helper, and one named temp root for the host.
Merges two duplicated function pairs (in the hook command, and between link recording and the snapshot), splits three files of over 800 lines (bodies moved byte-identical, every test name kept), deletes or wires the dead strict-modes presence check, and deletes the dead non-unix constants in the e2e harness run.
Closes the named mutant survivors with new tests only (no mutation run). Removes the cookie class from the end-to-end secret scan, which must still catch every other class.
Re-reads every merge site, survivor and file length at HEAD. Starts no live claude.
@task_file n=1 path=tests/support/mod.rs mark=edits
what: the shared test-support module of the root integration tests — edits: the module list (the shared source-masking scanner and the new helpers join it)
@task_file n=2 path=tests/support/home.rs mark=edits
what: the test-support helper that makes an isolated viola home for each test — edits: the one viola-made-home helper (0700, made the way viola makes a home)
@task_file n=3 path=tests/contract_error_bodies.rs mark=edits
what: the source-scan contract test over error bodies and sanitised error surfaces — edits: the `blanked` hand scanner, replaced by the shared scanner
@task_file n=4 path=tests/contract_bounded_readers.rs mark=edits
what: the source-scan contract test that every boundary reader is bounded — edits: the `code_only` hand scanner, replaced by the shared scanner
@task_file n=5 path=tests/contract_os_gated_tests.rs mark=edits
what: the census test that OS-gated tests name their reason and name no other OS — edits: the census scanner, replaced by the shared scanner
@task_file n=6 path=tests/cli_verify.rs mark=edits
what: the CLI integration tests of viola verify — edits: split to 800 lines or fewer, bodies moved byte-identical, every test name kept
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the test cases of the Claude capability ledger in the agent crate — edits: split to 800 lines or fewer, bodies moved byte-identical, every test name kept
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake-agent test binary that CI runs against in place of claude — edits: split to 800 lines or fewer, bodies moved byte-identical
@task_file n=9 path=src/cmd/hook.rs mark=edits
what: the viola hook command, which turns hook payloads into channel frames and hook output — edits: the duplicated function pair, merged into one function with behaviour unchanged
@task_file n=10 path=src/run/links.rs mark=edits
what: the wrapper's driver-to-driven link recording — edits: the function duplicated in the snapshot module, merged into one
@task_file n=11 path=src/run/snapshot.rs mark=edits
what: the wrapper's instance snapshot writer — edits: the function duplicated in link recording, merged into one
@task_file n=12 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes read rule and check in viola-state — edits: `check_present`, the dead presence check (deleted, or wired and tested)
@task_file n=13 path=.claude/docs/services/viola-state.md mark=edits
what: the viola-state service doc — edits: its one mention of `check_present`
@task_file n=14 path=crates/viola-state/src/fs.rs mark=edits
what: viola-state's crash-safe file helpers — edits: tests that close the error-kind guard survivor
@task_file n=15 path=crates/viola-state/src/pin.rs mark=edits
what: viola-state's pinned exe copy (placed once, re-hashed before reuse) — edits: tests that close the error-kind guard survivor
@task_file n=16 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp server on rmcp over stdio — edits: a test that asserts the WARN `parse-rejected` process-log line
@task_file n=17 path=crates/viola-mcp/src/stdin.rs mark=edits
what: viola's own MCP stdin reader, which reads under a line cap — edits: a test with a buffer smaller than the pending line; the `CHUNK` survivor marked equivalent
@task_file n=18 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the viola-mcp-tap test-only binary of the e2e crate — edits: tests, or its recorded exclusion from the mutants scope
@task_file n=19 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the end-to-end harness and its home-base headroom check — edits: `free_bytes`, tests that close its 2 survivors
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the end-to-end harness run — edits: the three dead `#[cfg(not(unix))]` constants, removed
@task_file n=21 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan that gates uploads — edits: its `cookie` class (the generic cookie header and the GUI cookie pair) and the tests of that class, removed
@task_item n=1 part=needs
The shared source-masking scanner must keep the verdicts of the three source-scan checks it serves: error bodies interpolate nothing and stay sanitised, every boundary reader is bounded, and every OS-gated test names its reason and names no other OS.
@task_item n=2 part=needs
The census of OS-gated tests and the ban on test gates that name another OS. The removed non-unix constants are dead because the tree is Linux-only, and nothing new may add such a gate.
@task_item n=3 part=needs
Each test runs in its own viola home. The new helper makes that home at 0700, the way viola makes one, so the home-tree and strict-modes refusals of run, revive and verify do not redden the 19 tests that use it.
@task_item n=4 part=needs
Test homes are removed only by their owner and only outside the working directory, and test evidence survives a killed test. The single host temp root and the remove-the-guard helper must stay inside these rules.
@task_item n=5 part=needs
The dev host's test homes sit on a tmpfs link. The linked e2e-home base must be a private real directory with an owner-only headroom floor, and the headroom refusal message (served by `free_bytes`) must keep its text.
@task_item n=6 part=needs
The strict-modes read rule and its check order: clients check strict modes before any frame and before any log opens. Removing or wiring the dead presence check must not change any strict-modes-failed verdict.
@task_item n=7 part=needs
Hook behaviour stays unchanged across the merge: the hook fails open, writes nothing to stderr, logs only to diagnostics, exits 0 silently with no trusted instance, and keeps its exit and body contract.
@task_item n=8 part=needs
Link recording and the instance snapshot stay unchanged across the merge: links are recorded as log events, the link set follows the log, and snapshots go through the one shared atomic persist helper and keep their envelope.
@task_item n=9 part=needs
The MCP stdin reader is bounded under a line cap, refuses an oversize line whole, and logs the `parse-rejected` process-log line at WARN. The new tests assert these behaviours, including a buffer smaller than the pending line.
@task_item n=10 part=needs
The pinned copy is placed once without clobbering and re-hashed before every reuse, and crash-safe disk writes classify their error kinds. The new tests pin the error-kind guards of these helpers.
@task_item n=11 part=needs
Mutation testing runs at the epoch boundary, in its own target dir, with NOCOW scratch. This chunk produces no mutation result. The tap's survivors are closed by tests or by a recorded exclusion from the mutants scope, and the next boundary audit reads them.
@task_item n=12 part=needs
The secret scan gates uploads and skips only the chunk diff. Without the cookie class it must still catch every other class it catches today.
@task_item n=13 part=needs
The fake-agent test binary keeps its test seam variables and its no-op feature on the named member crates, and stays out of release builds. The split moves its bodies without changing behaviour.
@task_item n=14 part=needs
The Claude capability ledger's test cases keep their count and names when split. The agent crate does no file I/O and stays the one home of Claude payload shapes.
@task_item n=15 part=needs
Files that the splits and the shared scanner add must each be declared in their module tree, since orphan modules are banned. Test-only crates (syn, proc-macro2) stay in tests and never reach the product.
@task_item n=16 part=needs
No live claude runs in this chunk or in CI: tests run against the fake agent on ubuntu-latest, and the real CLI stays local-only behind the founder's per-chunk cap.
@task_item n=17 part=needs
The e2e crate is test-only and the test-support feature stays out of product builds. The tap binary and the harness changes stay inside that boundary.
@task_item n=18 part=needs
Lint and format gates: clippy denies warnings, print and dbg macros are banned, tracing level macros are banned and obs_event! is the only event emitter. This holds for the new helpers, the scanner and the moved bodies.
@task_item n=19 part=needs
The fixed bounds that root tests share (one wait bound, and freezing a wrapper through one SIGSTOP guard) bind the split test files and the remove-the-guard control helper.
@task_standing n=1
convention: each new test runs in its own isolated viola home, which only its owner removes
home: established-decisions · occupied-resources
@task_standing n=2
convention: each new OS-gated test names its reason and names no other OS
home: cross-cutting-patterns
@task_standing n=3
convention: each new test file or test-support module is declared in its module tree, compiles under clippy with warnings denied and the print bans, and stays out of product builds
home: build-system
@task_standing n=4
convention: each new test runs in CI against the fake agent, never against a live claude
home: ci-cd-approach
@task_standing n=5
convention: each new test keeps its evidence when it is killed, and its removals stay outside the working directory
home: occupied-resources
@task_item n=1 part=not_needed
The wheel, its byte-source claim and the human-take rules. No item touches who holds the wheel.
@task_item n=2 part=not_needed
Budget governor, budget gate, budget.json and release-budget rules. No budget behaviour changes.
@task_item n=3 part=not_needed
The dialog-hold ceiling and its configured bound. The scope names them as outside this entry, carried by 0.2.0.
@task_item n=4 part=not_needed
Send delivery: atomic bracketed paste, send confirmation, send refusal order and text normalisation. No send path is edited.
@task_item n=5 part=not_needed
CLI verbs, CLI exit codes and the --json error documents of list, link, revive and plugin install. No CLI surface changes.
@task_item n=6 part=not_needed
Revive: in-place resume, the refusal codes of its --json document and its program resolution.
@task_item n=7 part=not_needed
Release tooling, dist, crates.io publishing, licence allowlist, cargo-deny sources and dependency pins. No dependency is added or changed.
@task_item n=8 part=not_needed
The fuzz workspace, its nightly toolchain, weekly fuzzing and the corpus replay.
@task_item n=9 part=not_needed
Statusline wrapper, per-session settings override and the statusline source. Nothing in the statusline path is edited.
@task_item n=10 part=not_needed
Plugin install, the plugin's embedded hooks and the marketplace tree.
@task_item n=11 part=not_needed
Deployment model, hosting, containers and the listening-port absences. These are product-shape rules that no cleanup item reaches.
@task_item n=12 part=not_needed
Fleet console, window features and graphical-frontend absences.
