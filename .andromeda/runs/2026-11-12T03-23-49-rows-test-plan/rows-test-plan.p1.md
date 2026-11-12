@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=0
what: an editing note about when the document itself may be changed
@row_add n=1 row=decision/agent-driven-test-layers area=test-plan-viola rec=3
name: Agent-driven test layers
rule: Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review.
except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.
@row_none n=3 rec=4 type=S
@row_add n=2 row=decision/comprehensive-test-tier area=1-test-scope-summary rec=5
name: Comprehensive test tier
rule: The project is tested at the Comprehensive tier (2).
why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
@row_add n=3 row=decision/tests-run-on-linux-ubuntu-latest area=1-test-scope-summary rec=5,34
name: Tests run on Linux
rule: The test suites run on Linux, with CI on the ubuntu-latest leg.
@row_add n=4 row=decision/chaos-tests-for-self-healing-state area=1-test-scope-summary rec=6,5
name: Chaos tests for self-healing state
rule: The crash-safe self-healing state is covered by chaos tests.
why: A driver of the Comprehensive tier.
@row_add n=5 row=decision/performance-budget-tests-for-hook-deadlines area=1-test-scope-summary rec=7,5
name: Performance-budget tests for hook deadlines
rule: Hook deadlines are covered by performance-budget tests.
why: A driver of the Comprehensive tier.
@row_add n=6 row=decision/compat-tests-for-cli-versions area=1-test-scope-summary rec=8,5
name: Compatibility tests across CLI versions
rule: Multi-version Claude CLI fixtures are covered by compatibility tests.
why: A driver of the Comprehensive tier.
@row_add n=7 row=decision/mutation-testing area=1-test-scope-summary rec=9,5
name: Mutation testing
rule: Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit.
why: A mandated driver of the Comprehensive tier.
@row_add n=8 row=decision/no-compliance-verification area=1-test-scope-summary rec=10
name: No compliance verification
rule: Compliance verification, the one Comprehensive-tier component, is not applied.
why: The security excerpt names no compliance triggers.
@row_none n=4 rec=11 type=S
@row_none n=5 rec=12 type=5
@row_none n=6 rec=13 type=5
@row_none n=7 rec=14 type=4
relied: The child's exit is detected on the process handle and never on EOF.
@row_add n=9 row=decision/cross-user-tested-at-peer-check area=1-test-scope-summary rec=15,27
name: Cross-user attacks tested at the peer-credential check
rule: Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test.
why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test.
@row_none n=8 rec=16 type=5
@row_add n=10 row=decision/real-claude-local-only area=1-test-scope-summary rec=17,27,37
name: Real claude CLI tested locally only
rule: Claude-facing behaviour is tested in CI against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are reached only locally, through viola verify and a route entry's own live measurement, never in CI.
@row_add n=11 row=contract/mcp-tools-list/tool-set area=1-test-scope-summary rec=18,33
boundary: MCP tools/list
item: tool set
promise: tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink.
@row_add n=12 row=contract/viola-hook/exit-code area=1-test-scope-summary rec=19,30
boundary: viola hook exec-form entry points
item: exit code
promise: viola hook <event> and viola hook statusline always exit 0.
@row_add n=13 row=contract/viola-hook/stderr area=1-test-scope-summary rec=19,30
boundary: viola hook exec-form entry points
item: stderr
promise: viola hook never writes to stderr.
@row_add n=14 row=contract/viola-hook/stdout area=1-test-scope-summary rec=19,30
boundary: viola hook exec-form entry points
item: stdout
promise: stdout is either empty (no decision, async hooks) or exactly the PreToolUse or PermissionRequest decision body.
@row_add n=15 row=contract/viola-hook/deadline area=1-test-scope-summary rec=30
boundary: viola hook exec-form entry points
item: deadline
promise: Spine hooks finish within the spine deadline, and SessionEnd within about 1 s.
@row_add n=16 row=contract/viola-hook-statusline/budget-json area=1-test-scope-summary rec=19,48
boundary: viola hook statusline
item: budget.json
promise: rate_limits is written to budget.json only when the payload holds a rate_limits object.
@row_add n=17 row=contract/viola-hook-statusline/passthrough area=1-test-scope-summary rec=19,48
boundary: viola hook statusline
item: passthrough
promise: The user's statusline is passed through unchanged, after budget.json is written.
@row_none n=9 rec=20 type=5
@row_none n=10 rec=21 type=5
@row_none n=11 rec=22 type=5
@row_add n=18 row=decision/r8-claude-env-strip area=1-test-scope-summary rec=23,52
name: R8 CLAUDE* environment strip
rule: Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor.
except: Persistent-environment names listed in config.json claude_env_keep are kept.
@row_none n=12 rec=24 type=5
@row_none n=13 rec=25 type=5
@row_add n=19 row=decision/cargo-deny-tokio-and-c-crate-bans area=1-test-scope-summary rec=26
name: Tokio and C-crate bans
rule: The dependency policy bans tokio where it is not allowed and bans C crates.
guard: cargo deny
@row_add n=20 row=decision/sync-crates-without-tokio area=1-test-scope-summary rec=26
name: Sync crates build without tokio
rule: The sync crates must build without tokio.
guard: cargo check of the sync crates without tokio
@row_add n=21 row=decision/clippy-deny-warnings area=1-test-scope-summary rec=26
name: Clippy warnings are errors
rule: Clippy runs with warnings denied.
guard: clippy -D warnings
@row_add n=22 row=decision/cargo-modules-orphans-gate area=1-test-scope-summary rec=26
name: No orphan modules
rule: Every lib and bin target is checked for orphan modules.
guard: cargo-modules orphans gate per lib/bin target
@row_add n=23 row=decision/acyclic-boundary-review area=1-test-scope-summary rec=26
name: On-demand acyclic boundary review
rule: Module boundaries are reviewed for cycles on demand rather than gated on every run.
guard: cargo-modules --acyclic
@row_add n=24 row=decision/release-build-viola-only area=1-test-scope-summary rec=26
name: Release build carries viola only
rule: The release build ships the viola binary only.
@row_add n=25 row=decision/sha-pinned-actions area=1-test-scope-summary rec=26
name: SHA-pinned GitHub Actions
rule: CI Actions are pinned by SHA.
guard: zizmor
@row_none n=14 rec=28 type=S
@row_add n=26 row=decision/cli-tests-as-subprocess area=1-test-scope-summary rec=29
name: CLI tested as a subprocess
rule: The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
@row_add n=27 row=contract/cli-json-output/stdout-document area=1-test-scope-summary rec=29
boundary: CLI --json output
item: stdout document
promise: Under --json a verb prints one JSON document on stdout.
@row_add n=28 row=contract/cli-json-output/answer-send-exit-2 area=1-test-scope-summary rec=29
boundary: CLI --json output
item: exit 2 of answer and send
promise: On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document.
@row_add n=29 row=contract/cli-human-output/no-sgr area=1-test-scope-summary rec=29
boundary: CLI human output
item: SGR
promise: Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
@row_add n=30 row=decision/no-child-screen-assertions area=1-test-scope-summary rec=29,31
name: No assertions on the child's screen
rule: TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content.
except: The forced vt100 panic case is checked by a byte search of the outer terminal.
@row_add n=31 row=decision/readiness-gate-tested-by-refusals area=1-test-scope-summary rec=31
name: Readiness gate tested by its refusals
rule: The readiness gate of viola run is tested through its refusal outputs.
@row_add n=32 row=decision/ipc-tests-schema-asserted area=1-test-scope-summary rec=32,33
name: IPC surfaces tested by schema-asserting clients
rule: The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
@row_add n=33 row=decision/mcp-negative-inputs-violate-schema area=1-test-scope-summary rec=33
name: MCP negative inputs violate the schema
rule: Negative MCP test inputs must violate the advertised tool schema.
why: The advertised schema is advisory.
@row_add n=34 row=decision/single-test-only-crate area=1-test-scope-summary rec=34
name: One test-only workspace member
rule: The workspace has exactly one test-only member, crates/viola-e2e, never published.
@row_none n=15 rec=35 type=S
@row_add n=35 row=decision/fake-agent-and-verify-fixtures-are-contract area=1-test-scope-summary rec=36
name: Fake agent and verify fixtures are the contract
rule: The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to.
why: A binding founder direction.
@row_add n=36 row=decision/structured-json-process-logs area=1-test-scope-summary rec=36,39
name: Structured JSON logs from every process
rule: Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/.
why: A binding founder direction.
@row_add n=37 row=contract/test-harness/boot area=1-test-scope-summary rec=37
boundary: test harness commands
item: boot
promise: boot builds viola and starts the wrappers in a fresh home to readiness.
@row_add n=38 row=contract/test-harness/run area=1-test-scope-summary rec=37
boundary: test harness commands
item: run
promise: run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
@row_add n=39 row=contract/test-harness/status area=1-test-scope-summary rec=37
boundary: test harness commands
item: status
promise: status queries viola list --json.
@row_add n=40 row=contract/test-harness/cleanup area=1-test-scope-summary rec=37
boundary: test harness commands
item: cleanup
promise: cleanup stops every wrapper and deletes the home, idempotently.
@row_add n=41 row=contract/test-harness/logs area=1-test-scope-summary rec=37
boundary: test harness commands
item: logs
promise: logs prints the per-process logs as JSON lines.
@row_add n=42 row=contract/viola-list-json/items area=1-test-scope-summary rec=38
boundary: viola list --json
item: ok.items
promise: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, budget_paused?, dialog_pending, cli_version?, cli_verified?}.
@row_add n=43 row=contract/viola-list-json/budget area=1-test-scope-summary rec=38
boundary: viola list --json
item: ok.budget
promise: ok.budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or the string "unknown".
@row_add n=44 row=contract/viola-list-json/skipped area=1-test-scope-summary rec=38
boundary: viola list --json
item: ok.skipped
promise: ok.skipped holds the counts unknown_kinds, unknown_fields and torn_lines.
@row_add n=45 row=contract/viola-list-json/mirrors-channel area=1-test-scope-summary rec=38
boundary: viola list --json
item: channel mirror
promise: The CLI --json document mirrors the channel list result.
@row_add n=46 row=contract/event-line/format area=1-test-scope-summary rec=39
boundary: the event line
item: format
promise: Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
@row_add n=47 row=contract/event-line/source area=1-test-scope-summary rec=39
boundary: the event line
item: source
promise: source is one of hook, wrapper or cli.
@row_add n=48 row=decision/logs-carry-no-secrets area=1-test-scope-summary rec=39
name: Logs carry no secrets
rule: Logs never contain the Cookie header or stripped CLAUDE* values.
@row_add n=49 row=contract/instance-snapshot/pid area=1-test-scope-summary rec=40
boundary: instance snapshot.json
item: pid, started_at, child_pid
promise: The wrapper pid, started_at and child_pid live in the instance's snapshot.json; there is no standalone pid file.
@row_add n=50 row=contract/heartbeat-file/interval area=1-test-scope-summary rec=40
boundary: instance heartbeat file
item: touch interval
promise: The instance heartbeat file is touched every 1 s.
@row_add n=51 row=contract/instance-liveness/gone area=1-test-scope-summary rec=40
boundary: instance liveness
item: gone
promise: An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
@row_add n=52 row=contract/instance-liveness/live area=1-test-scope-summary rec=40
boundary: instance liveness
item: live
promise: An instance whose process runs and whose heartbeat is at most 5 s old is live.
@row_add n=53 row=contract/instance-liveness/stale area=1-test-scope-summary rec=40
boundary: instance liveness
item: stale
promise: An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
@row_add n=54 row=decision/self-bootstrapping-test-data area=1-test-scope-summary rec=41
name: Self-bootstrapping test data
rule: Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
@row_add n=55 row=decision/scrubbed-fixtures area=1-test-scope-summary rec=41
name: Fixtures free of real data
rule: Fixtures never come from real prompts and are checked for absolute paths and usernames.
@row_add n=56 row=decision/property-fuzz-for-parsers area=1-test-scope-summary rec=41,5
name: Property and fuzz tests for parsers
rule: Property and fuzz generators cover the parser surfaces.
@row_none n=16 rec=42 type=S
@row_add n=57 row=contract/viola-run/start-refusal-exit-1 area=1-test-scope-summary rec=43
boundary: viola run start
item: exit 1
promise: A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
@row_add n=58 row=contract/driver-send/send-issued area=1-test-scope-summary rec=44
boundary: driver send
item: send-issued
promise: A driver send logs a send-issued event.
@row_add n=59 row=contract/driver-send/typed-text area=1-test-scope-summary rec=44
boundary: driver send
item: typed text
promise: The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
@row_add n=60 row=contract/driver-send/confirmation area=1-test-scope-summary rec=44
boundary: driver send
item: confirmation
promise: A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
@row_add n=61 row=contract/driver-send/empty-text area=1-test-scope-summary rec=44
boundary: driver send
item: empty-text
promise: An empty typed text is refused empty-text.
@row_add n=62 row=contract/driver-send/local-command area=1-test-scope-summary rec=44
boundary: driver send
item: ledger-listed local command
promise: A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
@row_add n=63 row=contract/wait/wake-kinds area=1-test-scope-summary rec=45
boundary: wait
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
@row_add n=64 row=contract/wait/already-logged area=1-test-scope-summary rec=45
boundary: wait
item: after cursor
promise: wait returns at once when a waking event is already logged at or after the after cursor.
@row_add n=65 row=contract/wait/timeout area=1-test-scope-summary rec=45
boundary: wait
item: timeout
promise: wait times out with {timed_out:true}.
@row_add n=66 row=contract/wait/instance-unreachable area=1-test-scope-summary rec=45
boundary: wait
item: unreachable wrapper
promise: A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.
@row_add n=67 row=contract/last/result area=1-test-scope-summary rec=45
boundary: last
item: result
promise: last returns {last_assistant_message, ts} from the newest turn-ended event.
@row_add n=68 row=contract/dialog/one-pending area=1-test-scope-summary rec=46
boundary: hook dialogs
item: one pending dialog
promise: Exactly one dialog is pending per instance; a second concurrent dialog gets a null response.
@row_add n=69 row=contract/dialog/event-logged-once area=1-test-scope-summary rec=46
boundary: hook dialogs
item: dialog event
promise: A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
@row_add n=70 row=contract/hook-decision-body/question area=1-test-scope-summary rec=46
boundary: hook decision body
item: question
promise: A question is answered with PreToolUse allow plus updatedInput.answers, annotations included.
@row_add n=71 row=contract/hook-decision-body/permission area=1-test-scope-summary rec=46
boundary: hook decision body
item: permission
promise: A permission is answered with PermissionRequest allow or deny.
@row_add n=72 row=contract/hook-decision-body/plan area=1-test-scope-summary rec=46
boundary: hook decision body
item: plan
promise: A plan is approved through PreToolUse only and revised through PermissionRequest deny plus message.
@row_add n=73 row=contract/cli-exit-codes/13 area=1-test-scope-summary rec=46
boundary: CLI exit codes
item: 13
promise: An unknown dialog_id given to answer exits 13 unknown-dialog.
@row_add n=74 row=decision/human-takes-wheel-on-edit area=1-test-scope-summary rec=47
name: Human editing takes the wheel
rule: A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
@row_add n=75 row=contract/driver-send/human-typing area=1-test-scope-summary rec=47
boundary: driver send
item: human-typing
promise: A driver send while the human holds the wheel is refused human-typing.
@row_add n=76 row=decision/pause-forces-human-wheel area=1-test-scope-summary rec=47
name: Pause forces the human wheel
rule: pause moves the wheel to the human manually.
@row_add n=77 row=decision/wheel-returns-only-via-cli-release area=1-test-scope-summary rec=47
name: Wheel returns only through viola release
rule: The wheel returns from the human only through the CLI viola release, never through a release carrying from.
@row_add n=78 row=decision/human-never-blocked area=1-test-scope-summary rec=47
name: Human never blocked
rule: The human at the terminal is never blocked by automation.
@row_add n=79 row=decision/budget-pause-thresholds area=1-test-scope-summary rec=48
name: Budget pause thresholds
rule: Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
@row_add n=80 row=decision/unverified-cli-gate area=1-test-scope-summary rec=49
name: Unverified CLI version gate
rule: On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli.
why: A non-null dialog decision needs a viola verify stamp.
@row_add n=81 row=contract/hook-dialog/unverified-null area=1-test-scope-summary rec=49
boundary: channel hook.dialog
item: unverified CLI
promise: On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty.
@row_add n=82 row=contract/cli-exit-codes/12 area=1-test-scope-summary rec=49
boundary: CLI exit codes
item: 12
promise: answer on an unverified CLI version exits 12.
@row_add n=83 row=contract/mcp-answer/unverified-cli area=1-test-scope-summary rec=49
boundary: MCP answer tool
item: unverified-cli
promise: MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
@row_add n=84 row=contract/viola-verify/stamped-line area=1-test-scope-summary rec=49
boundary: viola verify output
item: stamped line
promise: A passing viola verify ends with the line "stamped <ver>  <n> pass  0 fail".
@row_none n=17 rec=50 type=9
@row_add n=85 row=decision/unwrapped-hooks-silent-noop area=1-test-scope-summary rec=51
name: Unwrapped session hooks are a no-op
rule: A viola hook fired in a session viola does not wrap is a silent no-op.
@row_none n=18 rec=53 type=9
@row_none n=19 rec=54 type=9
@row_none n=20 rec=55 type=S
