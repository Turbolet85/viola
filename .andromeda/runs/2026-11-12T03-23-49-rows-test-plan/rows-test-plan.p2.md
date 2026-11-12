@row_add n=1 row=decision/ipc-vector-negative-tests area=1-test-scope-summary rec=1
name: IPC security vector negative tests
rule: The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
@row_add n=2 row=contract/channel-errors/32600 area=1-test-scope-summary rec=1
boundary: channel errors
item: -32600
promise: A frame over MAX_FRAME is answered with -32600 and the connection is closed.
@row_add n=3 row=contract/channel-errors/32602 area=1-test-scope-summary rec=1,14
boundary: channel errors
item: -32602
promise: A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper.
@row_add n=4 row=contract/unix-socket-permissions/directory area=1-test-scope-summary rec=1
boundary: Unix socket permissions
item: directory
promise: The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
@row_add n=5 row=contract/unix-socket-permissions/file area=1-test-scope-summary rec=1
boundary: Unix socket permissions
item: file
promise: The Unix socket file is mode 0600.
@row_add n=6 row=decision/peer-euid-checked area=1-test-scope-summary rec=1
name: Peer of another euid dropped
rule: A peer of another euid is dropped by the wrapper before any frame, and a client reads server-verify-failed.
@row_add n=7 row=decision/paste-control-characters-rejected area=1-test-scope-summary rec=2
name: Control characters in pasted text rejected
rule: ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper.
except: LF, CR, TAB and multi-byte UTF-8 are accepted.
@row_add n=8 row=decision/hooks-fail-open area=1-test-scope-summary rec=3
name: Hooks fail open
rule: A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
@row_add n=9 row=decision/statusline-command-needs-strict-home area=1-test-scope-summary rec=3
name: Statusline command gated on strict home
rule: The configured statusline_command never runs when the home fails strict-modes.
@row_add n=10 row=decision/mcp-schema-violations-rejected area=1-test-scope-summary rec=4
name: MCP schema-violating tool calls rejected
rule: MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler.
@row_add n=11 row=contract/mcp-iserror/payload area=1-test-scope-summary rec=4
boundary: MCP isError payloads
item: payload
promise: An isError payload carries codes only.
@row_add n=12 row=contract/mcp-stdin/oversize-line area=1-test-scope-summary rec=4
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A stdin line past MAX_FRAME is refused whole and ends the session: earlier lines are answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin/oversize WARN line holding no byte of it.
@row_add n=13 row=decision/cli-env-vector-negative-tests area=1-test-scope-summary rec=5
name: CLI and env vector negative tests
rule: Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
@row_add n=14 row=decision/no-switch-disables-a-control area=1-test-scope-summary rec=5
name: No switch disables a security control
rule: No environment variable, flag or config key disables a security control.
@row_add n=15 row=decision/leading-slash-never-a-prompt area=1-test-scope-summary rec=5
name: Leading-slash argument never a prompt
rule: A leading-slash argument is never taken as a prompt.
@row_add n=16 row=decision/private-filesystem-permissions area=1-test-scope-summary rec=6
name: Private filesystem permissions enforced
rule: A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
@row_add n=17 row=decision/widened-install-dirs-refused area=1-test-scope-summary rec=6
name: Widened installed directories refused
rule: A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
@row_add n=18 row=decision/pinned-exe-hash-checked area=1-test-scope-summary rec=6
name: Pinned executable hash checked
rule: A pinned executable whose hash does not match makes the command exit 1.
@row_add n=19 row=decision/plugin-rewritten-on-start area=1-test-scope-summary rec=6
name: Plugin and settings rewritten on start
rule: The plugin directory and the instance's settings.json are rewritten on every start.
@row_add n=20 row=decision/tailing-ignores-symlinks area=1-test-scope-summary rec=6
name: Tailing ignores symlinks
rule: Tailing ignores symlinks, and an over-long ndjson line is counted in skipped.torn_lines.
@row_add n=21 row=decision/agents-json-fallback-unknown area=1-test-scope-summary rec=7
name: Unreadable agents listing reads unknown
rule: Oversize or malformed claude agents --json output is read as unknown.
@row_add n=22 row=decision/row-name-never-a-target area=1-test-scope-summary rec=7
name: Agent row name never a target
rule: An agent row name is never usable as a target.
@row_add n=23 row=decision/human-output-escapes-control area=1-test-scope-summary rec=7
name: Human-mode output escapes control characters
rule: Human-mode list, wait and last output escapes C0 and C1 characters.
@row_add n=24 row=decision/vt100-panic-contained area=1-test-scope-summary rec=7
name: Terminal emulator panic contained
rule: A vt100 panic yields not-delivered/input-not-ready while passthrough continues: the child's own bytes reach the outer terminal and no viola literal appears on it.
@row_add n=25 row=decision/no-advisory-ignored area=1-test-scope-summary rec=8
name: No dependency advisory ignored
rule: CI gates on cargo deny check with no advisory ignored.
@row_add n=26 row=decision/sync-crates-without-tokio area=1-test-scope-summary rec=8
name: Sync crates build without tokio
rule: CI gates on a cargo check of the sync crates without tokio.
why: holds the tokio ban
@row_add n=27 row=decision/workflows-audited-and-sha-pinned area=1-test-scope-summary rec=8
name: Workflows audited and Actions SHA-pinned
rule: CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code.
@row_add n=28 row=decision/parser-property-fuzz-coverage area=1-test-scope-summary rec=9
name: Parser surfaces under property and fuzz tests
rule: Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the vt100 feed and prompt-submitted normalisation.
@row_add n=29 row=decision/chaos-tests-required area=1-test-scope-summary rec=10
name: Chaos tests required
rule: Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
@row_add n=30 row=decision/hook-deadline-timing area=1-test-scope-summary rec=11
name: Hook deadlines and heartbeat staleness asserted
rule: Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
@row_add n=31 row=decision/cli-mcp-identical-payloads area=1-test-scope-summary rec=12
name: CLI and MCP return identical payloads
rule: MCP and CLI return identical ok and refusal payloads for the same channel result.
@row_add n=32 row=contract/cli-exit-codes/10-14 area=1-test-scope-summary rec=12
boundary: CLI exit codes
item: 10-14
promise: A refused send shows the refused marker and exits with a code from 10 to 14.
@row_add n=33 row=decision/linux-only area=1-test-scope-summary rec=13
name: Linux-only product
rule: The product is Linux-only: the full workspace and the fake-agent E2E run on ubuntu-latest, and a test gate whose predicate names another OS is a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=34 row=decision/replay-every-cli-version-fixture area=1-test-scope-summary rec=14
name: Every CLI-version fixture set replayed
rule: Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
@row_add n=35 row=decision/unknown-events-counted-skipped area=1-test-scope-summary rec=14
name: Unknown events counted, not failed
rule: Unknown event kinds and fields are counted in skipped rather than failing.
@row_add n=36 row=contract/cli-exit-codes/14 area=1-test-scope-summary rec=14
boundary: CLI exit codes
item: 14
promise: A refusal whose reason is not known falls back to RefusalReason unknown and exits 14.
@row_add n=37 row=decision/ledger-row-verify-probe area=1-test-scope-summary rec=14
name: Every ledger row has a verify probe
rule: Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
@row_add n=38 row=decision/fake-agent-matches-verify-fixtures area=1-test-scope-summary rec=15,29
name: Fake agent held to recorded verify fixtures
rule: A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally.
why: guards against the fake agent drifting from the real CLI
@row_add n=39 row=decision/error-sanitization area=1-test-scope-summary rec=16
name: Errors sanitized at every surface
rule: CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
@row_add n=40 row=contract/channel-errors/32603 area=1-test-scope-summary rec=16
boundary: channel errors
item: -32603
promise: The message is exactly "internal error" and data is null.
@row_add n=41 row=decision/no-secrets-in-logs area=1-test-scope-summary rec=16
name: No secrets in logs or diagnostics
rule: No log and nothing in diagnostics holds a Cookie or a stripped CLAUDE variable value, and diagnostics files are mode 0600.
@row_add n=42 row=decision/mutation-at-epoch-boundary area=1-test-scope-summary rec=17,28
name: Mutation testing at the epoch boundary
rule: Mutation testing runs in the epoch-boundary code audit and on demand through run --mutants, never in the default run, --all, a chunk's run, the pre-push or CI.
@row_add n=43 row=decision/no-screen-scraping-in-e2e area=1-test-scope-summary rec=18
name: E2E asserts on events, never on screen content
rule: E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
@row_add n=44 row=decision/no-blanket-approval area=1-test-scope-summary rec=18
name: No blanket dialog approval
rule: No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
@row_add n=45 row=decision/waits-key-on-events area=1-test-scope-summary rec=18,33
name: Test waits key on logged events
rule: Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
@row_add n=46 row=decision/test-tier-single-source area=1-test-scope-summary rec=19
name: Test tier is the single source of rigor
rule: The test tier is the single source of truth for how much test rigor downstream skills apply.
@row_add n=47 row=decision/rstest-case-tables area=2-test-strategy rec=22,52,55
name: Table-driven unit cases in rstest
rule: Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label.
except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
why: so no manifest edge is added for rstest alone
@row_add n=48 row=decision/unit-tests-inline area=2-test-strategy rec=22,50
name: Inline unit tests
rule: Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
@row_add n=49 row=decision/integration-suite-per-contract area=2-test-strategy rec=23
name: Integration suite for every standard contract
rule: Every standard contract has a crate-level integration suite over a real interprocess endpoint, a real temporary home and real processes.
@row_add n=50 row=decision/e2e-for-critical-paths area=2-test-strategy rec=24
name: E2E for every critical path
rule: All seven critical paths and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
@row_add n=51 row=decision/property-and-fuzz-layer area=2-test-strategy rec=25
name: Property tests and fuzzing
rule: proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
@row_add n=52 row=decision/hook-deadlines-gated-by-hyperfine area=2-test-strategy rec=26
name: Hook deadlines gated by hyperfine
rule: hyperfine gates the hook deadlines on the maximum time.
@row_add n=53 row=decision/heartbeat-staleness-pure-function area=2-test-strategy rec=26
name: Heartbeat staleness checked without a clock
rule: The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
@row_add n=54 row=decision/fault-injection-sources area=2-test-strategy rec=27
name: Fault injection sources
rule: Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
@row_add n=55 row=decision/mutation-run-failure-conditions area=2-test-strategy rec=28
name: Mutation run failure conditions
rule: A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
@row_add n=56 row=decision/fixtures-schema-checked area=2-test-strategy rec=29
name: Fixtures checked against their schema
rule: Every recorded fixture is checked against its schema.
@row_none n=1 rec=20 type=S
@row_none n=2 rec=21 type=S
@row_none n=3 rec=30 type=S
@row_add n=57 row=decision/machine-parseable-test-output area=2-test-strategy rec=31
name: Machine-parseable test output
rule: Every test layer produces machine-parseable output, and harness commands print one JSON document on stdout.
@row_add n=58 row=decision/no-human-in-loop-verification area=2-test-strategy rec=32
name: No human-in-loop verification
rule: Verification has no human-in-loop step, no visual review service and no manual exploratory step.
@row_add n=59 row=decision/insta-check-mode-only area=2-test-strategy rec=32
name: Snapshot tests in check mode only
rule: insta runs only in check mode, never updating snapshots.
@row_add n=60 row=decision/no-test-retries-or-sleeps area=2-test-strategy rec=33
name: No test retries or sleep synchronisation
rule: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation.
@row_add n=61 row=decision/ci-never-real-claude area=2-test-strategy rec=33
name: CI never touches the real agent CLI
rule: CI never touches the real claude.
@row_add n=62 row=decision/tests-create-own-home area=2-test-strategy rec=34
name: Every test creates its own home
rule: Every test creates its own temporary --home, with no developer-seeded data.
@row_add n=63 row=decision/stamps-from-fake-agent-verify area=2-test-strategy rec=34
name: Stamps only from verify against the fake agent
rule: Test stamps come only from viola verify run against the fake agent.
@row_add n=64 row=decision/budget-json-from-statusline area=2-test-strategy rec=34
name: Budget file only from statusline stdin
rule: In every booted-wrapper test, budget.json comes only from viola hook statusline stdin.
except: a reader's own unit cases may plant a malformed one only by altering the writer's own bytes
@row_add n=65 row=decision/tokio-only-in-e2e-crate area=2-test-strategy rec=36
name: Tokio test code confined to the E2E crate
rule: One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients.
why: so the root package and the sync crates carry no tokio dev-dependency
@row_add n=66 row=decision/shared-e2e-helpers area=2-test-strategy rec=36
name: Shared E2E helpers never copied
rule: The sync E2E suites in the root share one set of helpers, which no root test file copies.
@row_add n=67 row=decision/complete-lines-reader area=2-test-strategy rec=37
name: Live-appended files read by complete lines
rule: Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader.
why: a Linux reader can observe an append part-way
@row_add n=68 row=decision/integration-binary-surface-prefix area=2-test-strategy rec=38
name: Integration binaries prefixed by surface
rule: Rust integration test binaries are named surface_topic, with the surface prefix naming the layer.
why: so nextest filtersets can select layers
@row_add n=69 row=decision/fixture-named-by-hook-event area=2-test-strategy rec=38
name: Fixtures named by hook event
rule: Fixtures are named Event.variant.json, where Event is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
@row_add n=70 row=decision/test-function-naming area=2-test-strategy rec=39,51
name: Test function naming
rule: Test functions are named subject_condition_expected in snake_case.
except: critical-path scenarios are named pathN_slug
@row_none n=4 rec=35 type=S
@row_none n=5 rec=40 type=S
@row_none n=6 rec=41 type=S
@row_none n=7 rec=42 type=S
@row_add n=71 row=decision/agent-driven-harness area=3-test-harness-contract rec=43
name: Agent-driven harness development
rule: Development is agent-driven through a test harness whose contract the test plan specifies.
@row_add n=72 row=decision/harness-logic-in-rust area=3-test-harness-contract rec=44
name: Harness logic once, in Rust
rule: One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust.
@row_add n=73 row=contract/harness-session-record/v area=3-test-harness-contract rec=45
boundary: harness session record
item: v
promise: The session record carries its format version v.
@row_add n=74 row=contract/harness-session-record/session area=3-test-harness-contract rec=45
boundary: harness session record
item: session
promise: The session record carries the session id; the default session is default.
@row_add n=75 row=contract/harness-session-record/home area=3-test-harness-contract rec=45
boundary: harness session record
item: home
promise: The session record carries the session's home.
@row_add n=76 row=contract/harness-session-record/instances area=3-test-harness-contract rec=45
boundary: harness session record
item: instances
promise: The session record lists instances, each with name, wrapper_pid, started_at and fake_args.
@row_add n=77 row=contract/harness-session-record/supervisor-pid area=3-test-harness-contract rec=45
boundary: harness session record
item: supervisor_pid
promise: The session record carries the supervisor's pid.
@row_add n=78 row=contract/harness-session-record/supervisor-started-at area=3-test-harness-contract rec=45
boundary: harness session record
item: supervisor_started_at
promise: The session record carries the supervisor's start time.
@row_add n=79 row=contract/harness-session-record/agents-mode area=3-test-harness-contract rec=45
boundary: harness session record
item: agents_mode
promise: agents_mode is one of recorded, oversize or malformed, defaults to recorded, is written by boot and stands in supervise.json too.
@row_add n=80 row=decision/harness-records-tolerate-unknown-fields area=3-test-harness-contract rec=45
name: Harness records tolerate missing and unknown fields
rule: The harness crate never denies unknown fields, so an older session record lacking a newer field still parses.
@row_add n=81 row=contract/harness-cli/session area=3-test-harness-contract rec=45
boundary: harness CLI
item: --session
promise: --session with an id selects a session, allowing parallel sessions.
@row_add n=82 row=contract/harness-exit-codes/0 area=3-test-harness-contract rec=46
boundary: harness exit codes
item: 0
promise: The harness command succeeded.
@row_add n=83 row=contract/harness-exit-codes/1 area=3-test-harness-contract rec=46
boundary: harness exit codes
item: 1
promise: The harness command failed.
@row_add n=84 row=contract/harness-exit-codes/2 area=3-test-harness-contract rec=46
boundary: harness exit codes
item: 2
promise: The harness command was given a usage error, and its document carries reason usage and a detail.
@row_add n=85 row=contract/harness-stdout/document area=3-test-harness-contract rec=46
boundary: harness stdout
item: document
promise: Every harness command prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order.
@row_add n=86 row=contract/harness-usage-detail/invalid-session-id area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: invalid-session-id
promise: The session id given is not valid.
@row_add n=87 row=contract/harness-usage-detail/session-exists area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: session-exists
promise: The session named already exists.
@row_add n=88 row=contract/harness-usage-detail/invalid-instance area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: invalid-instance
promise: The instance named is not valid.
@row_add n=89 row=contract/harness-usage-detail/arguments area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: arguments
promise: The arguments do not parse, including a flag or selector whose surface is not built yet.
@row_add n=90 row=contract/harness-usage-detail/no-supervise-spec area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: no-supervise-spec
promise: No supervise spec exists for the request.
@row_add n=91 row=contract/harness-usage-detail/unknown-suite area=3-test-harness-contract rec=46
boundary: harness usage error detail
item: unknown-suite
promise: A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.
@row_add n=92 row=decision/unbuilt-surface-is-usage-error area=3-test-harness-contract rec=46
name: Unbuilt harness surface is a usage error
rule: A harness flag or selector whose surface is not built yet is a usage error, never a vacuous pass.
@row_none n=8 rec=47 type=S
@row_add n=93 row=decision/nextest-test-runner area=4-unit-test-strategy rec=48
name: cargo-nextest runs the tests
rule: Tests run on libtest through cargo-nextest, process per test, with JUnit output, on a pinned stable toolchain.
@row_add n=94 row=decision/doctests-via-cargo-test area=4-unit-test-strategy rec=48
name: Doctests run through cargo test
rule: Doctests run through cargo test --workspace --doc.
why: nextest cannot run doctests
@row_add n=95 row=decision/llvm-cov-coverage area=4-unit-test-strategy rec=48
name: cargo-llvm-cov measures coverage
rule: Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
@row_none n=9 rec=49 type=S
@row_none n=10 rec=53 type=S
@row_add n=96 row=decision/rstest-fixtures-for-constructors area=4-unit-test-strategy rec=54,53
name: rstest fixtures for shared constructors
rule: Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.
