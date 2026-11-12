@row_add n=1 row=decision/proptest-persisted-seeds area=7-test-data-fixtures rec=1,34
name: proptest with persisted seeds
rule: Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.

@row_add n=2 row=decision/injected-clock area=7-test-data-fixtures rec=2,32,42
name: injected clock for time-sensitive data
rule: Time-sensitive data is injected per test behind viola_core::Clock, which tests advance; the wall clock is read only through that clock seam.
except: A link line's since is stamped from the wall clock and asserted by its shape only, and the budget gate's three evaluation points read the wall clock.

@row_add n=3 row=decision/time-as-argument area=7-test-data-fixtures rec=2,42
name: pure time decisions take their instant
rule: A pure time decision, such as the budget gate's override expiry, takes its instant as an argument instead of reading a clock.

@row_none n=1 rec=3 type=S

@row_add n=4 row=contract/fake-agent-cli/cli-version area=7-test-data-fixtures rec=4,3
boundary: viola-fake-agent command line
item: --cli-version
promise: The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.

@row_add n=5 row=contract/fake-agent-hooks/command-spawn area=7-test-data-fixtures rec=5,3
boundary: viola-fake-agent hook execution
item: hook command
promise: The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.

@row_add n=6 row=contract/fake-agent-hooks/stdin-payload area=7-test-data-fixtures rec=5,3
boundary: viola-fake-agent hook execution
item: hook stdin payload
promise: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.

@row_add n=7 row=contract/fake-agent-hooks/status-line-command area=7-test-data-fixtures rec=5,3
boundary: viola-fake-agent hook execution
item: status line command
promise: The fake agent runs a status line command only when the command is absolute.

@row_add n=8 row=contract/fake-agent-events/session-start area=7-test-data-fixtures rec=6,3
boundary: viola-fake-agent fired events
item: SessionStart
promise: After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire.

@row_add n=9 row=contract/fake-agent-input/bracketed-paste area=7-test-data-fixtures rec=7,3
boundary: viola-fake-agent input
item: bracketed paste
promise: Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.

@row_add n=10 row=contract/fake-agent-input/framing-paste area=7-test-data-fixtures rec=7,3
boundary: viola-fake-agent input
item: --framing paste
promise: Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.

@row_add n=11 row=contract/fake-agent-input/clear area=7-test-data-fixtures rec=7,3
boundary: viola-fake-agent input
item: /clear
promise: Under --framing the probed /clear fires the recorded clear-1 SessionEnd then SessionStart and no UserPromptSubmit.

@row_add n=12 row=contract/fake-agent-input/compact area=7-test-data-fixtures rec=7,3
boundary: viola-fake-agent input
item: /compact
promise: Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.

@row_add n=13 row=contract/fake-agent-input/keystroke area=7-test-data-fixtures rec=7,3
boundary: viola-fake-agent input
item: keystroke
promise: Every byte typed outside a paste is a keystroke, and a CR submits what was typed.

@row_add n=14 row=decision/control-file-gating area=7-test-data-fixtures rec=8
name: control-file gating of fake agent steps
rule: Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep.
why: Deterministic ordering without sleeps.

@row_add n=15 row=contract/fake-agent-cli/script area=7-test-data-fixtures rec=8,3
boundary: viola-fake-agent command line
item: --script
promise: The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.

@row_add n=16 row=contract/fake-agent-cli/control area=7-test-data-fixtures rec=8,3
boundary: viola-fake-agent command line
item: --control
promise: The control path is always <home>/fake/<name>.control, and the fake agent reads it by byte offset.

@row_add n=17 row=decision/synthetic-fixture-text area=7-test-data-fixtures rec=8,18
name: synthetic fixture text only
rule: Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.

@row_add n=18 row=contract/fake-agent-receipt/line area=7-test-data-fixtures rec=9,3
boundary: viola-fake-agent receipt file
item: receipt line
promise: The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.

@row_add n=19 row=contract/fake-agent-receipt/kind area=7-test-data-fixtures rec=9,3
boundary: viola-fake-agent receipt file
item: kind
promise: Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, statusline, step.

@row_add n=20 row=contract/fake-agent-receipt/env area=7-test-data-fixtures rec=9,3
boundary: viola-fake-agent receipt file
item: env receipt
promise: An env receipt holds variable names only, never a value.

@row_add n=21 row=contract/fake-script/format area=7-test-data-fixtures rec=10
boundary: fake agent turn script
item: script format
promise: A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.

@row_add n=22 row=contract/fake-script/unreadable-exit area=7-test-data-fixtures rec=10
boundary: fake agent turn script
item: unreadable script
promise: An unreadable script makes the fake agent exit 2.

@row_add n=23 row=contract/fake-script/gated-step area=7-test-data-fixtures rec=10
boundary: fake agent turn script
item: gated step
promise: A gated step waits for one more newline-terminated control line past the byte offset.

@row_add n=24 row=contract/fake-agent-cli/argv-only-options area=7-test-data-fixtures rec=11,3
boundary: viola-fake-agent command line
item: modes and options
promise: The fake agent's modes and options are taken from argv only, never from the environment.

@row_add n=25 row=contract/fake-agent-cli/unknown-arguments area=7-test-data-fixtures rec=11,3
boundary: viola-fake-agent command line
item: unknown arguments
promise: Unknown arguments are ignored.

@row_add n=26 row=contract/fake-agent-exit/exit-triggers area=7-test-data-fixtures rec=12,3
boundary: viola-fake-agent exit
item: exit trigger
promise: The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.

@row_add n=27 row=decision/fake-agent-children-own-process-group area=7-test-data-fixtures rec=12
name: fake agent children in their own process group
rule: On Unix every command the fake agent spawns, each hook and the status line command, runs in a process group of its own.
why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.

@row_add n=28 row=decision/fake-agent-exit-ignores-sighup area=7-test-data-fixtures rec=12
name: fake agent exit ignores SIGHUP
rule: On Unix the fake agent ignores SIGHUP from the first step of its exit (\x03 or stdin's end), while a hangup landing before the exit begins still ends it.
why: The agent is its PTY's session leader and cannot leave that group, so a hangup during exit would cut its own exit-time coverage write.

@row_none n=2 rec=13 type=S

@row_add n=29 row=decision/fixture-hygiene-walk area=7-test-data-fixtures rec=14,19
name: fixture hygiene run-time walk
rule: One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema.
why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file.
guard: claude_fixtures in tests/contract_fixture_hygiene.rs
except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.

@row_add n=30 row=decision/hygiene-reports-class-only area=7-test-data-fixtures rec=15
name: hygiene checker reports class only
rule: The hygiene checker reports a violation class only (absolute-path, username, schema), never the offending content.

@row_add n=31 row=decision/planted-hygiene-violations area=7-test-data-fixtures rec=15,16
name: planted inputs prove hygiene classes red
rule: Every hygiene violation class is proven red by a planted input.

@row_add n=32 row=decision/recorder-scrub area=7-test-data-fixtures rec=17,19
name: recorder scrubs home paths and usernames
rule: The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub.
except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.

@row_add n=33 row=decision/record-only-at-zero-failing-rows area=7-test-data-fixtures rec=17
name: fixtures recorded only when nothing fails
rule: The viola verify recorder writes fixtures only at 0 failing rows.

@row_add n=34 row=decision/screen-fixture-scrub-as-detector area=7-test-data-fixtures rec=17
name: screen fixtures checked scrub-as-detector
rule: A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.

@row_add n=35 row=decision/mcp-tap-refuses-dirty-exchange area=7-test-data-fixtures rec=17
name: MCP tap refuses a dirty exchange whole
rule: The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.

@row_add n=36 row=contract/verify-recording-refusal/unable-message area=7-test-data-fixtures rec=17
boundary: viola verify fixture recording refusal
item: unable message
promise: A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.

@row_none n=3 rec=20 type=S

@row_add n=37 row=decision/tests-seed-own-data area=7-test-data-fixtures rec=21
name: self-bootstrapping tests
rule: Tests seed their own data; no developer pre-populates a home.

@row_none n=4 rec=22 type=4
relied: The boot command creates the home, stamps it via viola verify and starts the wrappers, which tests rely on to seed their own data.

@row_none n=5 rec=23 type=S

@row_add n=38 row=decision/per-test-home area=7-test-data-fixtures rec=24
name: isolated home per test
rule: Each test has its own home, so no endpoint, log or budget.json is shared between tests.

@row_add n=39 row=decision/env-per-child area=7-test-data-fixtures rec=24
name: environment set per child process
rule: Test environment is set per child process with Command::env, never with std::env::set_var.
why: std::env::set_var is unsafe in edition 2024.

@row_none n=6 rec=25 type=4
relied: Per session, cleanup removes the harness home, session.json and the supervisor.

@row_add n=40 row=decision/ci-homes-kept-until-gates area=7-test-data-fixtures rec=26
name: CI homes kept until gate steps
rule: In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run.
except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.

@row_none n=7 rec=27 type=S

@row_none n=8 rec=28 type=S

@row_none n=9 rec=29 type=S

@row_add n=41 row=absence/outbound-http area=8-mocking-stubbing-discipline rec=30,41
capability: outbound HTTP calls

@row_add n=42 row=decision/no-http-mocking area=8-mocking-stubbing-discipline rec=30,41
name: no HTTP mock library
rule: No HTTP mock library is used.
why: viola makes no outbound HTTP calls.

@row_add n=43 row=decision/fakes-for-external-services area=8-mocking-stubbing-discipline rec=31
name: fakes replace external services
rule: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.

@row_add n=44 row=decision/mcp-paused-tokio-clock area=8-mocking-stubbing-discipline rec=32
name: paused tokio clock for viola-mcp timers
rule: viola-mcp tests use tokio's paused clock for a timer it owns, and a paused clock in a test never reaches a spawned wrapper.

@row_add n=45 row=decision/filesystem-not-mocked area=8-mocking-stubbing-discipline rec=33,37
name: real filesystem in tests
rule: The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere.
why: Modes, file locking, torn lines and symlink handling are the behaviour under test.

@row_none n=10 rec=35 type=S

@row_add n=46 row=decision/pure-functions-tested-directly area=8-mocking-stubbing-discipline rec=36
name: pure functions tested directly
rule: Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

@row_add n=47 row=decision/no-mocks-in-e2e area=8-mocking-stubbing-discipline rec=38,39
name: nothing mocked in E2E
rule: E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.

@row_none n=11 rec=40 type=S

@row_add n=48 row=decision/mockall-on-seam-traits-only area=8-mocking-stubbing-discipline rec=43,45
name: mockall doubles on seam traits only
rule: Test doubles are mockall automock on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.

@row_add n=49 row=decision/no-http-fake-servers area=8-mocking-stubbing-discipline rec=44
name: no HTTP fake servers
rule: Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used.

@row_none n=12 rec=46 type=S

@row_none n=13 rec=47 type=S

@row_add n=50 row=decision/github-actions-ci area=9-ci-integration rec=48
name: GitHub Actions for CI
rule: CI runs on GitHub Actions: one workflow on push and pull request, and a scheduled nightly workflow for fuzz and weekly advisories.

@row_add n=51 row=absence/deploy-stage area=9-ci-integration rec=48
capability: a deploy stage

@row_add n=52 row=decision/rust-cache-only-cache area=9-ci-integration rec=48
name: rust-cache as the only CI cache
rule: rust-cache is the only cache, used in the push and pull-request workflow only.

@row_none n=14 rec=49 type=S

@row_add n=53 row=decision/ci-lint-gate area=9-ci-integration rec=50
name: CI lint gate
rule: CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion.
except: The acyclic module review is not a gate.

@row_add n=54 row=decision/supply-chain-fail-closed area=9-ci-integration rec=51
name: fail-closed supply-chain job
rule: One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.

@row_add n=55 row=decision/single-instrumented-test-run area=9-ci-integration rec=53,52
name: one instrumented coverage run in CI
rule: CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.

@row_add n=56 row=decision/jobs-end-in-harness-gate area=9-ci-integration rec=53
name: CI jobs end in the harness gate
rule: Each CI test job ends in viola-harness gate requiring its suites.

@row_add n=57 row=decision/mutation-not-a-ci-gate area=9-ci-integration rec=54
name: mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

@row_add n=58 row=decision/perf-verdict-from-harness area=9-ci-integration rec=55
name: perf verdict from the harness gate
rule: Perf runs in its own CI job, and its verdict comes from viola-harness gate, never from a jq binary.
