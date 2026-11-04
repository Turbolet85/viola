@task_prose part=chunk
Brings Linux and macOS to parity with Windows: the viola run wrapper runs its child under portable-pty's openpty backend through the same viola-pty seam as ConPTY (spawn, read, write, resize, wait, kill), as far as the fake agent can read them.
Makes the full fake-agent suite the same suite on windows-2025, ubuntu-latest and macos-latest; a test left cfg(windows), cfg(unix) or cfg(target_os) states where it stands why its behaviour belongs to one OS (which of the 14 cfg(windows) test files are parity gaps is P3's reading).
Re-reads the obs gates G1 to G4 and the CI test, perf and lint jobs, which must be green on ubuntu-latest and macos-latest beside windows-2025.
Holds one diagnostics schema on every OS: process-log lines, events.ndjson, snapshots and every --json document carry the same fields and closed words, with a documented null or closed word where one OS cannot fill a field.
Picks up the v1-40 integration-tier witness (a stale row of viola list reads dim under the outer PTY) as the first consumer of the Unix fake-agent suite, and the Unix reading of a wrapper's child ending with its killed wrapper; no live claude session, and Windows stays the live-supported target.
@task_file n=1 path=.github/workflows/ci.yml mark=re-reads
what: the test, perf and lint jobs over matrix.os read green on ubuntu-latest and macos-latest beside windows-2025
@task_file n=2 path=schemas/diag-line.v1.json mark=re-reads
what: G4 schema conformance, one process-log line schema on every OS
@task_file n=3 path=tests/support/home.rs mark=re-reads
what: Wrapper::kill and the kill-and-revive case read on Unix with the fake agent
@task_file n=4 path=tests/run_cli.rs
@task_item n=1 part=needs
The viola-pty seam on openpty under the Unix wrapper: spawn, read, write, resize, wait and kill behave as the ConPTY wrapper's do, exit is read on the process handle and never on reader EOF, and the Module to PTY integration boundary (a resize propagated, a key written after a resize once the child reports the new size) runs over real openpty spawns of the fake agent.
@task_item n=2 part=needs
The multi-os-compat trigger: full workspace and fake-agent E2E on all three runners, with OS-branch tests for named pipe vs Unix socket, ConPTY vs openpty, Windows DACL vs Unix modes, and macOS euid plus directory-only server verification.
@task_item n=3 part=needs
The per-OS gating of tests: which root and crate tests compile out on one OS (cfg(windows), cfg(unix), cfg(target_os)), including the unit cases already compiled out on macOS (the real cargo-mutants harness tests, the server check's captured-span cases), each gate carrying its named reason.
@task_item n=4 part=needs
The obs gate checks run in the per-OS test job on ubuntu-latest and macos-latest: G1 bare instrument, G2 zero panics, G3 no abort panic strategy, and G4 schema conformance, which validates every home-level diagnostics line under target/e2e-home against diag-line.v1.json and every instance detail line against diag-detail.v1.json.
@task_item n=5 part=needs
The CI matrix and per-OS jobs: the test job's one instrumented run --coverage and its gate --require, the per-OS perf job timing the hook rows against SPINE_DEADLINE on max, the per-OS lint job, and the per-OS uploads behind the secret scan (harness, diag and junit artifacts named by matrix.os).
@task_item n=6 part=needs
The per-OS coverage gate (lines 85, functions 95, regions 80), enforced in each OS job because cfg(unix) and cfg(windows) code compiles only on its own runner; a test moved onto Unix runners changes what each Unix job counts.
@task_item n=7 part=needs
One diagnostics schema across OSes: the event line shape in events.ndjson, the snapshot envelope and instance snapshot, the viola list --json status document and every --json document hold the same keys and closed words on every OS, an unfillable field reading its documented null or closed word, never a missing key or an OS-only word.
@task_item n=8 part=needs
The macOS carve-out: macOS reports no peer pid, so the server check there compares no pid or start time (server_verdict_rows), the server-identity negatives in tests/security_negatives_server_identity.rs run on Linux and Windows only, and no test reads the macOS check as a comparison.
@task_item n=9 part=needs
The Unix child-ends-with-its-wrapper reading: a killed wrapper's recorded child gone by pid and start time inside 7 s, read with the fake agent, whose Unix process groups (each spawned hook and the status line in a group of its own) and SIGHUP-ignoring exit decide what a hangup from a killed wrapper reaches; the standalone pin run_child_ends_when_its_wrapper_is_terminated stays cfg(windows).
@task_item n=10 part=needs
The v1-40 integration-tier witness: under the outer-PTY driver a viola list beside a stale wrapper (a heartbeat older than 5 s) reads SGR dim from the row's first byte to its last, NAME dim and bold; TTY rendering is asserted only on the SGR sequences of verb output, never the child's screen, and staleness rests on the 1 s heartbeat and 5 s threshold.
@task_item n=11 part=needs
The mutants profile's 10 s kill, which kept the v1-40 witness at the unit tier: a test that waits out the 5 s stale threshold beside the about 4 s stamp must stay inside that bound.
@task_item n=12 part=needs
The outer-PTY driver on Unix for the tui surface: the master reader drained on its own thread, exit only via child.wait(), and the forced vt100 panic chaos case reading by byte search that the child's own bytes reach the outer terminal on Unix and no viola literal appears.
@task_item n=13 part=needs
The OS-specific behaviours that justify a one-OS test: the statusline pass-through on Unix only (the Windows arm runs no command), the instance settings.json written on Unix only, the claude_env_keep survival on Unix in the R8 strip, Unix socket directory 0700 and socket 0600 modes, and the Unix socket directory refusals in tests/security_negatives_socket_dir.rs on Linux and macOS.
@task_item n=14 part=needs
The exit-code contract stays closed: the typed set 0, 1, 2, 10 to 14, 20, 21 gains no code, and hook keeps exit 0 with no body and no stderr on every OS, which every Unix-run test of the exit-cause matrix and the hook fail-open matrix asserts.
@task_item n=15 part=needs
The peer-euid invariant over any Unix test the chunk adds: no env, flag or feature seam forces a refusing peer-euid decision, the decision is held only over injected euids and the real euid read on Linux and macOS, and a Unix-only test seam needs a Decisions Log entry.
@task_item n=16 part=needs
Windows stays the live-supported target through the shared suite: no windows-2025 behaviour, pin or Windows-only test moves to make a Unix case pass, and the Windows-only live measurements entry keeps its carries.
@task_item n=17 part=needs
The wheel's closed non-editing list (the human takes the wheel, Path 5) as the CARRY names it: replies to the cursor-position query and the kitty graphics query count as typing; a shape joins the list only when measured by its exact grammar on the founder's word, and what of that is doable under the fake agent is P4's.
@task_item n=18 part=needs
The fake agent as the only child in this chunk: every Unix reading runs on viola-fake-agent and recorded fixtures, CI never touches the real claude, and a green fake-agent run is not claimed as proof of real-CLI behaviour (v1-53's live Linux leg is P4's).
@task_item n=19 part=needs
The anti-pattern that a Linux-only local run is no proof for Windows or macOS: macOS is CI-only, so every macOS parity reading comes from the macos-latest runner.
@task_standing n=1
convention: a new integration test binary takes the surface_topic prefix (cli_, hook_, tui_, channel_, chaos_, contract_ in root; path_, mcp_, cross_ in viola-e2e) so the layer filtersets select it
home: test-strategy
@task_standing n=2
convention: a new test function is named subject_condition_expected in snake_case, a critical-path scenario pathN_slug
home: ## 2. Test Strategy
@task_standing n=3
convention: each test makes its own home at a not-yet-existing path under target/e2e-home (through create_private_dir, never create_dir_all), started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests
home: integration-test-strategy
@task_standing n=4
convention: waits key on events.ndjson byte offsets, viola wait --after or a status field, never a sleep; a live-appended file is read only through the complete-lines reader tests/support/ndjson.rs
home: ## 2. Test Strategy
@task_standing n=5
convention: env is set per child with Command::env, never std::env::set_var, and any env_clear re-adds LLVM_PROFILE_FILE; a test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN
home: ## 11. Test Anti-Patterns
@task_standing n=6
convention: zero flakiness: nextest retries stay 0, a flaking test keeps the chunk red until its root cause is fixed, and #[ignore] is no parking place
home: quality-gates-coverage-targets
@task_standing n=7
convention: verdicts come from hook events, events.ndjson, channel payloads and the fake-agent receipt, never a parse of the rendered child screen; child exit is read only via child.wait(); assert_cmd .assert() is never used on a long-lived viola run wrapper
home: ## 11. Test Anti-Patterns
@task_standing n=8
convention: stamps.json, snapshot.json and budget.json are never hand-written: stamps come from viola verify against the fake agent, snapshots from the wrapper, budget from viola hook statusline
home: test-data-fixtures
@task_standing n=9
convention: shared root-test helpers live once in tests/support and no root test file copies them; viola-e2e tables are labelled case tables, not rstest
home: ## 2. Test Strategy
@task_standing n=10
convention: a case is asserted at the lowest layer that can hold it, and each surface a scenario lists gets its own driver step and signal
home: ## 11. Test Anti-Patterns
@task_standing n=11
convention: a CI job or step keeps permissions {} at the top and contents read per job, SHA-pinned uses, no concurrency block, rust-cache as the only cache, and its uploads behind the secret scan with retention-days 7
home: ci-integration
@task_item n=1 part=not_needed
Recording or refreshing real-CLI fixtures (the viola verify recorder's scrub, viola-mcp-tap, agent-run run --local-live): the chunk has no live session and records no fixture.
@task_item n=2 part=not_needed
Scenario E4 and any frontend testing: the frontend stays deferred to 0.2.0.
@task_item n=3 part=not_needed
The nightly time-boxed cargo fuzz runs and the weekly advisories re-run in nightly.yml: the chunk adds no parser and no fuzz target.
@task_item n=4 part=not_needed
The viola-core newtype unit tables (ViolaName, Percent, DialogHold bounds, EventKind counts, v constants): OS-agnostic pure values the chunk does not change.
@task_item n=5 part=not_needed
The viola-mcp handler re-validation cases and the tools/list exactness: transport-free handler logic with no per-OS branch.
@task_item n=6 part=not_needed
Insta decision-body snapshots and their regeneration by the local fixture refresh: no dialog mapping changes.
@task_item n=7 part=not_needed
The test tier selection and its justification (Comprehensive): a plan-level choice the chunk does not revisit.
@task_item n=8 part=not_needed
The harness command grammar, its usage-error details and the session.json record shape: the chunk adds no harness command or flag.
@task_item n=9 part=not_needed
The budget governor's threshold values (90 and 85) and its override expiry rule: pure decisions with no OS branch, unchanged here.
