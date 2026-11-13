@task_prose part=session
An epoch-8 cleanup: no product API, crate, dependency, config, schema, CI step or wire shape changed.
Added one shared test-support masking scanner (comments, or comments and literals) that the three source census contract checks now take, replacing their four hand scanners; the bounded-readers and error-body censuses now leave out the fake agent's whole module, and the OS-gated census skips block comments too.
Split the largest files into module files inside existing crates: the fake agent binary into a module dir, the capability ledger's unit tests into four files, the verify integration tests into a module dir; gated cases stay in the root test file because the OS-gated census walks only top-level test files.
Retired the artifact secret scan's cookie class (a Cookie header line now reads as no hit; the remaining classes are claude-stripped, content-canary, mode, unreadable), the dead viola-state strict-modes helper check_present, and the e2e harness's non-unix twins (the 21-row ledger form); the harness keeps its one 24-row form.
Closed named mutation survivors with new tests (pinned copy placement under an unreadable name, private file placement with a too-long name, MCP stdin capped reads, the MCP oversize-line WARN parse-rejected line through a scoped test-only tracing subscriber, the e2e-home writable bytes, the tap binary's own e2e cases); 25 controls hold, no cargo-mutants run.
Added a test-home private_dir helper as the one test call of the private-dir creator; the founder ruled that a print-and-exit read of the claude binary (version or help) is no live start.
@task_file n=1 path=tests/support/source_scan.rs
what: test support: the one shared source masking scanner (comments, or comments and literals) the source census contract checks take
@task_file n=2 path=tests/contract_bounded_readers.rs
what: the source census contract check that every boundary reader is bounded, now excluding the fake agent's whole module
@task_file n=3 path=tests/contract_error_bodies.rs
what: the source census contract check over error bodies and error message text, now excluding the fake agent's whole module
@task_file n=4 path=tests/contract_os_gated_tests.rs
what: the census contract check that OS-gated tests name their reason and that no test gate names another OS
@task_file n=5 path=src/bin/viola-fake-agent.rs
what: the root of the test-only viola-fake-agent binary that stands in for claude in CI and tests
@task_file n=6 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's emulation of claude agents listing and its live session registry
@task_file n=7 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input reader turning typed bytes, pastes and submits into actions
@task_file n=8 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's replay of the per-session statusline command
@task_file n=9 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's unit tests (shared helpers and agents listing)
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's unit tests of its options, input and resume behaviour
@task_file n=11 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of its statusline, dialog replay and session modes
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the root of the capability ledger's unit tests in viola-agent-claude
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: capability ledger unit tests of ledger rows, the stamps envelope merge and the verified-version read
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: capability ledger unit tests of fixture scrubbing, screen signature checks and agents join checks
@task_file n=15 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: capability ledger unit tests of the resume, paste, local-command and statusline row checks
@task_file n=16 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: capability ledger unit tests of framing variants, dialog checks and the probe answers table
@task_file n=17 path=tests/cli_verify.rs
what: the root of viola verify's integration tests, holding its unix-gated cases
@task_file n=18 path=tests/cli_verify/stamps.rs
what: viola verify integration tests of stamping every row, its refusals and its process logs
@task_file n=19 path=tests/cli_verify/runs.rs
what: viola verify integration tests of its probe runs, recorded screens and probe dir removal
@task_file n=20 path=src/cmd/hook.rs
what: the viola hook subcommand; only a test helper changed
@task_file n=21 path=src/run/mod.rs
what: the run wrapper's start path; a test-only first-snapshot helper added
@task_file n=22 path=src/run/links.rs
what: the run wrapper's link set; only its tests changed
@task_file n=23 path=src/run/snapshot.rs
what: the run wrapper's instance snapshot writing; only its tests changed
@task_file n=24 path=crates/viola-state/src/strict.rs
what: viola-state's strict-modes read rule checks; the dead check_present removed
@task_file n=25 path=crates/viola-state/src/fs.rs
what: viola-state's private file and directory helpers (mode set before write, no-clobber placement, private dir creation)
@task_file n=26 path=crates/viola-state/src/pin.rs
what: placement of the pinned exe copy, placed once no-clobber and never replaced
@task_file n=27 path=crates/viola-mcp/src/lib.rs
what: the rmcp stdio MCP server; a test of its oversize-line WARN parse-rejected log added
@task_file n=28 path=crates/viola-mcp/src/stdin.rs
what: the MCP stdin bounded reader under the line cap
@task_file n=29 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only viola-mcp-tap binary that records an MCP exchange and writes the tools/list fixture
@task_file n=30 path=crates/viola-e2e/tests/mcp_tap.rs
what: the end-to-end tests of the viola-mcp-tap binary
@task_file n=31 path=crates/viola-e2e/src/harness/mod.rs
what: the end-to-end harness, including the e2e-home base headroom check
@task_file n=32 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness run against the fake agent and its ledger rows
@task_file n=33 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the artifact secret scan before upload and its class list
@task_file n=34 path=tests/support/home.rs
what: test support for each test's own viola home, now with the one private-dir helper
@task_item n=1 part=touches
The artifact secret scan's class list lost its cookie class: any row naming the scan's classes, a Cookie header among what never reaches an uploaded artifact, or the upload gate on the scan.
@task_item n=2 part=touches
The source census contract checks now mask through one shared test-support scanner: rows on bounded readers at every boundary, sanitised error surfaces and thiserror messages that interpolate nothing, where they say how the census reads the source.
@task_item n=3 part=touches
The bounded-readers and error-body censuses now exclude the whole test-only fake agent module, not its root file alone: an exception to the bounded-readers and error-surface rules for a test binary.
@task_item n=4 part=touches
The OS-gated census now skips block comments and walks only top-level test files: rows that OS-gated tests name their reason and that no test gate names another OS.
@task_item n=5 part=touches
The e2e harness's non-unix items were removed, the unix twins standing unconditional: the rule that no test gate names another OS.
@task_item n=6 part=touches
The project directory tree: the fake agent is now a module directory, the capability ledger's unit tests split into four files, viola verify's integration tests into a module directory, and a shared source scanner joins test support.
@task_item n=7 part=touches
viola-state's strict-modes check lost the dead helper check_present: any strict-modes read rule row or guard naming it.
@task_item n=8 part=touches
Pinned copy placement: a new test shows a no-clobber placement over a directory at the name reads present either way, so it pins the folder's mtime; rows on pinned copy placed once, no-clobber, never replaced.
@task_item n=9 part=touches
MCP stdin: new tests of capped lines read across short reads and of the one WARN parse-rejected line for an oversize line; rows on the MCP stdin line cap, oversize line refused whole and the parse-rejected process log.
@task_item n=10 part=touches
The MCP oversize test carries a scoped tracing subscriber in its own test module, no global install: rows on tracing process logs and the one event emitter.
@task_item n=11 part=touches
The e2e-home headroom check now computes writable bytes through a private helper; the floor's value is unchanged: rows on the e2e-home headroom floor and base checks.
@task_item n=12 part=touches
The test-only viola-mcp-tap binary is now tested end to end: rows on the tap's recorded exchange holding MCP tools/list, viola-e2e being test-only and test binaries never released.
@task_item n=13 part=touches
Test homes get their private dir through one test-support helper, the one test call of the private-dir creator: rows on each test in its own viola home and test homes cleaned by owner.
@task_item n=14 part=touches
The fake agent split keeps every item's name and only widens visibility inside its module: rows on fake-agent test seams, the fake-agent feature and no orphan modules.
@task_item n=15 part=touches
Named mutation survivors were closed by tests with controls, no cargo-mutants run: rows on mutation testing at the epoch boundary.
@task_item n=16 part=touches
The founder's word that a print-and-exit read of the claude binary is no live start: rows on real claude only locally and no live claude in CI, where they bound live starts.
@task_item n=17 part=touches
The e2e harness keeps the capability ledger's one 24-row form: any row holding the ledger row count.
@task_fact n=1 slug=cross-cutting-patterns
fact: The source census contract checks (bounded readers, error bodies, OS-gated tests) read the source through one shared test-support masking scanner, which masks comments, or comments and literals; the bounded-readers and error-body censuses leave out the whole test-only fake agent module.
@task_fact n=2 slug=cross-cutting-patterns
fact: The OS-gated census reads only the top-level test files of each test directory, so when a test binary splits into a module directory its OS-gated cases stay in the binary's root test file.
@task_fact n=3 slug=occupied-resources
fact: The artifact secret scan has no cookie class: its classes are claude-stripped, content-canary, mode and unreadable, and a Cookie header line reads as no hit.
@task_fact n=4 slug=project-directory-structure
fact: The test-only fake agent is a module directory beside its root file, and large test suites (the capability ledger's unit tests, viola verify's integration tests) split into sibling module files beside their root.
@task_item n=1 part=untouched
The wrapper channel's JSON-RPC methods, frames, fault codes and refusal reasons: no wire shape changed.
@task_item n=2 part=untouched
CLI verbs, CLI exit codes and the --json documents of list, revive, verify and plugin install: no output or exit code moved.
@task_item n=3 part=untouched
Normalised event kinds, event data and the ndjson event line: no event kind or field moved.
@task_item n=4 part=untouched
The stack, dependency policy, cargo-deny bans, toolchain pin, MSRV and rmcp features: no manifest or lockfile changed.
@task_item n=5 part=untouched
config.json keys, budget.json, the budget gate, the wheel and the human controls: none was reached.
@task_item n=6 part=untouched
Deployment, release build contents and CI workflow steps: no workflow, release or CI step changed.
@task_item n=7 part=untouched
Crate names, crate dependency direction and the occupied binary, command and environment names: no crate was added or removed and no name taken or freed.
