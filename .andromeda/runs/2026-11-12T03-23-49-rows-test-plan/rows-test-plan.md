@row_none n=1 rec=fct-dqhtamw2@1 type=S
@row_none n=2 rec=fct-l5lhrwtf@1 type=0
what: an editing note about when the document itself may be changed
@row_add n=1 row=decision/agent-driven-test-layers area=test-plan-viola rec=fct-bmqjnwhn@1
name: Agent-driven test layers
rule: Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review.
except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.
@row_none n=3 rec=fct-uzc5osyb@1 type=S
@row_add n=2 row=decision/comprehensive-test-tier area=1-test-scope-summary rec=fct-cyzmyvps@2
name: Comprehensive test tier
rule: The project is tested at the Comprehensive tier (2).
why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
@row_add n=3 row=decision/tests-run-on-linux-ubuntu-latest area=1-test-scope-summary rec=fct-cyzmyvps@2,fct-oftlgaxh@2
name: Tests run on Linux
rule: The test suites run on Linux, with CI on the ubuntu-latest leg.
@row_add n=4 row=decision/chaos-tests-for-self-healing-state area=1-test-scope-summary rec=fct-eij3zkzc@1,fct-cyzmyvps@2
name: Chaos tests for self-healing state
rule: The crash-safe self-healing state is covered by chaos tests.
why: A driver of the Comprehensive tier.
@row_add n=5 row=decision/performance-budget-tests-for-hook-deadlines area=1-test-scope-summary rec=fct-zvdpu2dw@1,fct-cyzmyvps@2
name: Performance-budget tests for hook deadlines
rule: Hook deadlines are covered by performance-budget tests.
why: A driver of the Comprehensive tier.
@row_add n=6 row=decision/compat-tests-for-cli-versions area=1-test-scope-summary rec=fct-ih6wd2ud@1,fct-cyzmyvps@2
name: Compatibility tests across CLI versions
rule: Multi-version Claude CLI fixtures are covered by compatibility tests.
why: A driver of the Comprehensive tier.
@row_add n=7 row=decision/mutation-testing area=1-test-scope-summary rec=fct-4k247nk7@1,fct-cyzmyvps@2
name: Mutation testing
rule: Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit.
why: A mandated driver of the Comprehensive tier.
@row_add n=8 row=decision/no-compliance-verification area=1-test-scope-summary rec=fct-pvam5xzl@1
name: No compliance verification
rule: Compliance verification, the one Comprehensive-tier component, is not applied.
why: The security excerpt names no compliance triggers.
@row_none n=4 rec=fct-r5flmuox@1 type=S
@row_none n=5 rec=fct-bu2wibzw@1 type=5
@row_none n=6 rec=fct-pebopeci@1 type=5
@row_none n=7 rec=fct-njspb5nv@3 type=4
relied: The child's exit is detected on the process handle and never on EOF.
@row_add n=9 row=decision/cross-user-tested-at-peer-check area=1-test-scope-summary rec=fct-qg6j3l2y@2,fct-upolcsa3@1
name: Cross-user attacks tested at the peer-credential check
rule: Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test.
why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test.
@row_none n=8 rec=fct-ocrbiixg@1 type=5
@row_add n=10 row=contract/mcp-tools-list/tool-set area=1-test-scope-summary rec=fct-rmbr6xeu@1,fct-apiugjwf@1
boundary: MCP tools/list
item: tool set
promise: tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink.
@row_add n=11 row=contract/viola-hook/exit-code area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: exit code
promise: viola hook <event> and viola hook statusline always exit 0.
@row_add n=12 row=contract/viola-hook/stderr area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: stderr
promise: viola hook never writes to stderr.
@row_add n=13 row=contract/viola-hook/deadline area=1-test-scope-summary rec=fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: deadline
promise: Spine hooks finish within the spine deadline, and SessionEnd within about 1 s.
@row_add n=14 row=contract/viola-hook-statusline/budget-json area=1-test-scope-summary rec=fct-paewrdc5@2,fct-tcg4lsy4@2
boundary: viola hook statusline
item: budget.json
promise: rate_limits is written to budget.json only when the payload holds a rate_limits object.
@row_add n=15 row=contract/viola-hook-statusline/passthrough area=1-test-scope-summary rec=fct-paewrdc5@2,fct-tcg4lsy4@2
boundary: viola hook statusline
item: passthrough
promise: The user's statusline is passed through unchanged, after budget.json is written.
@row_none n=9 rec=fct-4rrynj5z@1 type=5
@row_none n=10 rec=fct-23i45jvh@1 type=5
@row_none n=11 rec=fct-np3dgpi7@1 type=5
@row_add n=16 row=decision/r8-claude-env-strip area=1-test-scope-summary rec=fct-u3dkdc2w@3,fct-otwmx5qk@1
name: R8 CLAUDE* environment strip
rule: Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor.
except: Persistent-environment names listed in config.json claude_env_keep are kept.
@row_none n=12 rec=fct-5uplt6g5@1 type=5
@row_none n=13 rec=fct-ydbukved@1 type=5
@row_add n=17 row=decision/cargo-deny-tokio-and-c-crate-bans area=1-test-scope-summary rec=fct-elcdxxgr@2
name: Tokio and C-crate bans
rule: The dependency policy bans tokio where it is not allowed and bans C crates.
guard: cargo deny
@row_add n=18 row=decision/clippy-deny-warnings area=1-test-scope-summary rec=fct-elcdxxgr@2
name: Clippy warnings are errors
rule: Clippy runs with warnings denied.
guard: clippy -D warnings
@row_add n=19 row=decision/cargo-modules-orphans-gate area=1-test-scope-summary rec=fct-elcdxxgr@2
name: No orphan modules
rule: Every lib and bin target is checked for orphan modules.
guard: cargo-modules orphans gate per lib/bin target
@row_add n=20 row=decision/acyclic-boundary-review area=1-test-scope-summary rec=fct-elcdxxgr@2
name: On-demand acyclic boundary review
rule: Module boundaries are reviewed for cycles on demand rather than gated on every run.
guard: cargo-modules --acyclic
@row_none n=14 rec=fct-hmbzhbf7@1 type=S
@row_add n=21 row=decision/cli-tests-as-subprocess area=1-test-scope-summary rec=fct-a6gfb3bv@2
name: CLI tested as a subprocess
rule: The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
@row_add n=22 row=contract/cli-json-output/stdout-document area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI --json output
item: stdout document
promise: Under --json a verb prints one JSON document on stdout.
@row_add n=23 row=contract/cli-json-output/answer-send-exit-2 area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI --json output
item: exit 2 of answer and send
promise: On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document.
@row_add n=24 row=contract/cli-human-output/no-sgr area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI human output
item: SGR
promise: Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
@row_add n=25 row=decision/no-child-screen-assertions area=1-test-scope-summary rec=fct-a6gfb3bv@2,fct-pbrzczk2@1
name: No assertions on the child's screen
rule: TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content.
except: The forced vt100 panic case is checked by a byte search of the outer terminal.
@row_add n=26 row=decision/readiness-gate-tested-by-refusals area=1-test-scope-summary rec=fct-pbrzczk2@1
name: Readiness gate tested by its refusals
rule: The readiness gate of viola run is tested through its refusal outputs.
@row_add n=27 row=decision/ipc-tests-schema-asserted area=1-test-scope-summary rec=fct-b7vg2zil@1,fct-apiugjwf@1
name: IPC surfaces tested by schema-asserting clients
rule: The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
@row_add n=28 row=decision/mcp-negative-inputs-violate-schema area=1-test-scope-summary rec=fct-apiugjwf@1
name: MCP negative inputs violate the schema
rule: Negative MCP test inputs must violate the advertised tool schema.
why: The advertised schema is advisory.
@row_add n=29 row=decision/single-test-only-crate area=1-test-scope-summary rec=fct-oftlgaxh@2
name: One test-only workspace member
rule: The workspace has exactly one test-only member, crates/viola-e2e, never published.
@row_none n=15 rec=fct-5fsff64v@1 type=S
@row_add n=30 row=decision/fake-agent-and-verify-fixtures-are-contract area=1-test-scope-summary rec=fct-l7edbh4k@2
name: Fake agent and verify fixtures are the contract
rule: The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to.
why: A binding founder direction.
@row_add n=31 row=decision/structured-json-process-logs area=1-test-scope-summary rec=fct-l7edbh4k@2,fct-yp3slbux@1
name: Structured JSON logs from every process
rule: Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/.
why: A binding founder direction.
@row_add n=32 row=contract/test-harness/boot area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: boot
promise: boot builds viola and starts the wrappers in a fresh home to readiness.
@row_add n=33 row=contract/test-harness/run area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: run
promise: run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
@row_add n=34 row=contract/test-harness/status area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: status
promise: status queries viola list --json.
@row_add n=35 row=contract/test-harness/cleanup area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: cleanup
promise: cleanup stops every wrapper and deletes the home, idempotently.
@row_add n=36 row=contract/test-harness/logs area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: logs
promise: logs prints the per-process logs as JSON lines.
@row_add n=37 row=contract/viola-list-json/items area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: ok.items
promise: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, budget_paused?, dialog_pending, cli_version?, cli_verified?}.
@row_add n=38 row=contract/viola-list-json/mirrors-channel area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: channel mirror
promise: The CLI --json document mirrors the channel list result.
@row_add n=39 row=contract/event-line/format area=1-test-scope-summary rec=fct-yp3slbux@1
boundary: the event line
item: format
promise: Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
@row_add n=40 row=contract/event-line/source area=1-test-scope-summary rec=fct-yp3slbux@1
boundary: the event line
item: source
promise: source is one of hook, wrapper or cli.
@row_add n=41 row=contract/heartbeat-file/interval area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance heartbeat file
item: touch interval
promise: The instance heartbeat file is touched every 1 s.
@row_add n=42 row=contract/instance-liveness/gone area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: gone
promise: An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
@row_add n=43 row=contract/instance-liveness/live area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: live
promise: An instance whose process runs and whose heartbeat is at most 5 s old is live.
@row_add n=44 row=contract/instance-liveness/stale area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: stale
promise: An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
@row_add n=45 row=decision/scrubbed-fixtures area=1-test-scope-summary rec=fct-yeykk4zl@1
name: Fixtures free of real data
rule: Fixtures never come from real prompts and are checked for absolute paths and usernames.
@row_add n=46 row=decision/property-fuzz-for-parsers area=1-test-scope-summary rec=fct-yeykk4zl@1,fct-cyzmyvps@2
name: Property and fuzz tests for parsers
rule: Property and fuzz generators cover the parser surfaces.
@row_none n=16 rec=fct-vdbzoexn@1 type=S
@row_add n=47 row=contract/viola-run/start-refusal-exit-1 area=1-test-scope-summary rec=fct-2ls6kelo@2
boundary: viola run start
item: exit 1
promise: A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
@row_add n=48 row=contract/driver-send/send-issued area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: send-issued
promise: A driver send logs a send-issued event.
@row_add n=49 row=contract/driver-send/typed-text area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: typed text
promise: The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
@row_add n=50 row=contract/driver-send/confirmation area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: confirmation
promise: A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
@row_add n=51 row=contract/driver-send/empty-text area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: empty-text
promise: An empty typed text is refused empty-text.
@row_add n=52 row=contract/driver-send/local-command area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: ledger-listed local command
promise: A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
@row_add n=53 row=contract/wait/wake-kinds area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
@row_add n=54 row=contract/wait/already-logged area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: after cursor
promise: wait returns at once when a waking event is already logged at or after the after cursor.
@row_add n=55 row=contract/wait/timeout area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: timeout
promise: wait times out with {timed_out:true}.
@row_add n=56 row=contract/wait/instance-unreachable area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: unreachable wrapper
promise: A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.
@row_add n=57 row=contract/last/result area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: last
item: result
promise: last returns {last_assistant_message, ts} from the newest turn-ended event.
@row_add n=58 row=contract/dialog/one-pending area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook dialogs
item: one pending dialog
promise: Exactly one dialog is pending per instance; a second concurrent dialog gets a null response.
@row_add n=59 row=contract/dialog/event-logged-once area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook dialogs
item: dialog event
promise: A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
@row_add n=60 row=contract/hook-decision-body/question area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: question
promise: A question is answered with PreToolUse allow plus updatedInput.answers, annotations included.
@row_add n=61 row=contract/hook-decision-body/permission area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: permission
promise: A permission is answered with PermissionRequest allow or deny.
@row_add n=62 row=contract/hook-decision-body/plan area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: plan
promise: A plan is approved through PreToolUse only and revised through PermissionRequest deny plus message.
@row_add n=63 row=contract/cli-exit-codes/13 area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: CLI exit codes
item: 13
promise: An unknown dialog_id given to answer exits 13 unknown-dialog.
@row_add n=64 row=decision/human-takes-wheel-on-edit area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Human editing takes the wheel
rule: A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
@row_add n=65 row=contract/driver-send/human-typing area=1-test-scope-summary rec=fct-nzaflfcl@1
boundary: driver send
item: human-typing
promise: A driver send while the human holds the wheel is refused human-typing.
@row_add n=66 row=decision/pause-forces-human-wheel area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Pause forces the human wheel
rule: pause moves the wheel to the human manually.
@row_add n=67 row=decision/wheel-returns-only-via-cli-release area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Wheel returns only through viola release
rule: The wheel returns from the human only through the CLI viola release, never through a release carrying from.
@row_add n=68 row=decision/budget-pause-thresholds area=1-test-scope-summary rec=fct-tcg4lsy4@2
name: Budget pause thresholds
rule: Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
@row_add n=69 row=decision/unverified-cli-gate area=1-test-scope-summary rec=fct-yl2sew2w@1
name: Unverified CLI version gate
rule: On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli.
why: A non-null dialog decision needs a viola verify stamp.
@row_add n=70 row=contract/hook-dialog/unverified-null area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: channel hook.dialog
item: unverified CLI
promise: On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty.
@row_add n=71 row=contract/cli-exit-codes/12 area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: CLI exit codes
item: 12
promise: answer on an unverified CLI version exits 12.
@row_add n=72 row=contract/mcp-answer/unverified-cli area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: MCP answer tool
item: unverified-cli
promise: MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
@row_none n=17 rec=fct-xk4vhilu@1 type=9
@row_add n=73 row=decision/unwrapped-hooks-silent-noop area=1-test-scope-summary rec=fct-54ym6y3r@1
name: Unwrapped session hooks are a no-op
rule: A viola hook fired in a session viola does not wrap is a silent no-op.
@row_none n=18 rec=fct-momocfjp@1 type=9
@row_none n=19 rec=fct-qyfqvwpo@1 type=9
@row_none n=20 rec=fct-udamboof@1 type=S
@row_add n=74 row=decision/ipc-vector-negative-tests area=1-test-scope-summary rec=fct-3znwz3ga@1
name: IPC security vector negative tests
rule: The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
@row_add n=75 row=contract/channel-errors/32600 area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: channel errors
item: -32600
promise: A frame over MAX_FRAME is answered with -32600 and the connection is closed.
@row_add n=76 row=contract/channel-errors/32602 area=1-test-scope-summary rec=fct-3znwz3ga@1,fct-bpxqtijx@1
boundary: channel errors
item: -32602
promise: A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper.
@row_add n=77 row=contract/unix-socket-permissions/directory area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: Unix socket permissions
item: directory
promise: The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
@row_add n=78 row=contract/unix-socket-permissions/file area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: Unix socket permissions
item: file
promise: The Unix socket file is mode 0600.
@row_add n=79 row=decision/peer-euid-checked area=1-test-scope-summary rec=fct-3znwz3ga@1
name: Peer of another euid dropped
rule: A peer of another euid is dropped by the wrapper before any frame, and a client reads server-verify-failed.
@row_add n=80 row=decision/paste-control-characters-rejected area=1-test-scope-summary rec=fct-tbtv5wyr@1
name: Control characters in pasted text rejected
rule: ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper.
except: LF, CR, TAB and multi-byte UTF-8 are accepted.
@row_add n=81 row=decision/hooks-fail-open area=1-test-scope-summary rec=fct-qtwskmiu@1
name: Hooks fail open
rule: A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
@row_add n=82 row=decision/statusline-command-needs-strict-home area=1-test-scope-summary rec=fct-qtwskmiu@1
name: Statusline command gated on strict home
rule: The configured statusline_command never runs when the home fails strict-modes.
@row_add n=83 row=decision/mcp-schema-violations-rejected area=1-test-scope-summary rec=fct-bia7axqu@1
name: MCP schema-violating tool calls rejected
rule: MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler.
@row_add n=84 row=contract/mcp-iserror/payload area=1-test-scope-summary rec=fct-bia7axqu@1
boundary: MCP isError payloads
item: payload
promise: An isError payload carries codes only.
@row_add n=85 row=decision/cli-env-vector-negative-tests area=1-test-scope-summary rec=fct-engo5ir7@2
name: CLI and env vector negative tests
rule: Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
@row_add n=86 row=decision/no-switch-disables-a-control area=1-test-scope-summary rec=fct-engo5ir7@2
name: No switch disables a security control
rule: No environment variable, flag or config key disables a security control.
@row_add n=87 row=decision/private-filesystem-permissions area=1-test-scope-summary rec=fct-55p24js3@1
name: Private filesystem permissions enforced
rule: A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
@row_add n=88 row=decision/widened-install-dirs-refused area=1-test-scope-summary rec=fct-55p24js3@1
name: Widened installed directories refused
rule: A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
@row_add n=89 row=decision/pinned-exe-hash-checked area=1-test-scope-summary rec=fct-55p24js3@1
name: Pinned executable hash checked
rule: A pinned executable whose hash does not match makes the command exit 1.
@row_add n=90 row=decision/plugin-rewritten-on-start area=1-test-scope-summary rec=fct-55p24js3@1
name: Plugin and settings rewritten on start
rule: The plugin directory and the instance's settings.json are rewritten on every start.
@row_add n=91 row=decision/agents-json-fallback-unknown area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Unreadable agents listing reads unknown
rule: Oversize or malformed claude agents --json output is read as unknown.
@row_add n=92 row=decision/row-name-never-a-target area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Agent row name never a target
rule: An agent row name is never usable as a target.
@row_add n=93 row=decision/human-output-escapes-control area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Human-mode output escapes control characters
rule: Human-mode list, wait and last output escapes C0 and C1 characters.
@row_add n=94 row=decision/no-advisory-ignored area=1-test-scope-summary rec=fct-hlx2zkbt@2
name: No dependency advisory ignored
rule: CI gates on cargo deny check with no advisory ignored.
@row_add n=95 row=decision/workflows-audited-and-sha-pinned area=1-test-scope-summary rec=fct-hlx2zkbt@2
name: Workflows audited and Actions SHA-pinned
rule: CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code.
@row_add n=96 row=decision/parser-property-fuzz-coverage area=1-test-scope-summary rec=fct-urt6oicn@1
name: Parser surfaces under property and fuzz tests
rule: Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the vt100 feed and prompt-submitted normalisation.
@row_add n=97 row=decision/chaos-tests-required area=1-test-scope-summary rec=fct-pw5tvowr@1
name: Chaos tests required
rule: Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
@row_add n=98 row=decision/hook-deadline-timing area=1-test-scope-summary rec=fct-2xjimxfu@1
name: Hook deadlines and heartbeat staleness asserted
rule: Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
@row_add n=99 row=decision/cli-mcp-identical-payloads area=1-test-scope-summary rec=fct-wmsiydy7@2
name: CLI and MCP return identical payloads
rule: MCP and CLI return identical ok and refusal payloads for the same channel result.
@row_add n=100 row=contract/cli-exit-codes/10-14 area=1-test-scope-summary rec=fct-wmsiydy7@2
boundary: CLI exit codes
item: 10-14
promise: A refused send shows the refused marker and exits with a code from 10 to 14.
@row_add n=101 row=decision/replay-every-cli-version-fixture area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Every CLI-version fixture set replayed
rule: Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
@row_add n=102 row=decision/unknown-events-counted-skipped area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Unknown events counted, not failed
rule: Unknown event kinds and fields are counted in skipped rather than failing.
@row_add n=103 row=decision/ledger-row-verify-probe area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Every ledger row has a verify probe
rule: Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
@row_add n=104 row=decision/fake-agent-matches-verify-fixtures area=1-test-scope-summary rec=fct-fvjvvfmg@1,fct-xagti75h@1
name: Fake agent held to recorded verify fixtures
rule: A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally.
why: guards against the fake agent drifting from the real CLI
@row_add n=105 row=decision/error-sanitization area=1-test-scope-summary rec=fct-kpze4wbb@1
name: Errors sanitized at every surface
rule: CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
@row_add n=106 row=contract/channel-errors/32603 area=1-test-scope-summary rec=fct-kpze4wbb@1
boundary: channel errors
item: -32603
promise: The message is exactly "internal error" and data is null.
@row_add n=107 row=decision/no-screen-scraping-in-e2e area=1-test-scope-summary rec=fct-shszmhpb@1
name: E2E asserts on events, never on screen content
rule: E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
@row_add n=108 row=decision/no-blanket-approval area=1-test-scope-summary rec=fct-shszmhpb@1
name: No blanket dialog approval
rule: No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
@row_add n=109 row=decision/waits-key-on-events area=1-test-scope-summary rec=fct-shszmhpb@1,fct-s5fluv4p@1
name: Test waits key on logged events
rule: Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
@row_add n=110 row=decision/test-tier-single-source area=1-test-scope-summary rec=fct-imyfbj3w@1
name: Test tier is the single source of rigor
rule: The test tier is the single source of truth for how much test rigor downstream skills apply.
@row_add n=111 row=decision/e2e-for-critical-paths area=2-test-strategy rec=fct-q3ukwekc@1
name: E2E for every critical path
rule: All seven critical paths and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
@row_add n=112 row=decision/property-and-fuzz-layer area=2-test-strategy rec=fct-oa2vhv4g@1
name: Property tests and fuzzing
rule: proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
@row_add n=113 row=decision/heartbeat-staleness-pure-function area=2-test-strategy rec=fct-egpbbwom@1
name: Heartbeat staleness checked without a clock
rule: The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
@row_add n=114 row=decision/fault-injection-sources area=2-test-strategy rec=fct-2yi4u2mc@1
name: Fault injection sources
rule: Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
@row_add n=115 row=decision/mutation-run-failure-conditions area=2-test-strategy rec=fct-cv73iuuc@1
name: Mutation run failure conditions
rule: A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
@row_none n=21 rec=fct-afnl6nvb@1 type=S
@row_none n=22 rec=fct-vbygawhs@1 type=S
@row_none n=23 rec=fct-nymxcmgt@1 type=S
@row_add n=116 row=decision/machine-parseable-test-output area=2-test-strategy rec=fct-elca5edh@2
name: Machine-parseable test output
rule: Every test layer produces machine-parseable output, and harness commands print one JSON document on stdout.
@row_add n=117 row=decision/no-human-in-loop-verification area=2-test-strategy rec=fct-gcer5i4i@2
name: No human-in-loop verification
rule: Verification has no human-in-loop step, no visual review service and no manual exploratory step.
@row_add n=118 row=decision/insta-check-mode-only area=2-test-strategy rec=fct-gcer5i4i@2
name: Snapshot tests in check mode only
rule: insta runs only in check mode, never updating snapshots.
@row_add n=119 row=decision/no-test-retries-or-sleeps area=2-test-strategy rec=fct-s5fluv4p@1
name: No test retries or sleep synchronisation
rule: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation.
@row_add n=120 row=decision/ci-never-real-claude area=2-test-strategy rec=fct-s5fluv4p@1
name: CI never touches the real agent CLI
rule: CI never touches the real claude.
@row_add n=121 row=decision/tests-create-own-home area=2-test-strategy rec=fct-qyzxqqix@1
name: Every test creates its own home
rule: Every test creates its own temporary --home, with no developer-seeded data.
@row_add n=122 row=decision/stamps-from-fake-agent-verify area=2-test-strategy rec=fct-qyzxqqix@1
name: Stamps only from verify against the fake agent
rule: Test stamps come only from viola verify run against the fake agent.
@row_add n=123 row=decision/budget-json-from-statusline area=2-test-strategy rec=fct-qyzxqqix@1
name: Budget file only from statusline stdin
rule: In every booted-wrapper test, budget.json comes only from viola hook statusline stdin.
except: a reader's own unit cases may plant a malformed one only by altering the writer's own bytes
@row_add n=124 row=decision/tokio-only-in-e2e-crate area=2-test-strategy rec=fct-h4mygvf3@1
name: Tokio test code confined to the E2E crate
rule: One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients.
why: so the root package and the sync crates carry no tokio dev-dependency
@row_add n=125 row=decision/shared-e2e-helpers area=2-test-strategy rec=fct-h4mygvf3@1
name: Shared E2E helpers never copied
rule: The sync E2E suites in the root share one set of helpers, which no root test file copies.
@row_none n=24 rec=fct-3p5vn3ea@1 type=S
@row_none n=25 rec=fct-m6h5pcif@1 type=S
@row_none n=26 rec=fct-ts43nhos@1 type=S
@row_none n=27 rec=fct-k2yzqcx6@1 type=S
@row_add n=126 row=decision/agent-driven-harness area=3-test-harness-contract rec=fct-kalxmzsn@1
name: Agent-driven harness development
rule: Development is agent-driven through a test harness whose contract the test plan specifies.
@row_add n=127 row=decision/harness-logic-in-rust area=3-test-harness-contract rec=fct-yqn2bvnu@2
name: Harness logic once, in Rust
rule: One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust.
@row_add n=128 row=contract/harness-session-record/v area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: v
promise: The session record carries its format version v.
@row_add n=129 row=contract/harness-session-record/session area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: session
promise: The session record carries the session id; the default session is default.
@row_add n=130 row=contract/harness-session-record/home area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: home
promise: The session record carries the session's home.
@row_add n=131 row=contract/harness-session-record/instances area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: instances
promise: The session record lists instances, each with name, wrapper_pid, started_at and fake_args.
@row_add n=132 row=contract/harness-session-record/supervisor-pid area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: supervisor_pid
promise: The session record carries the supervisor's pid.
@row_add n=133 row=contract/harness-session-record/supervisor-started-at area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: supervisor_started_at
promise: The session record carries the supervisor's start time.
@row_add n=134 row=contract/harness-session-record/agents-mode area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: agents_mode
promise: agents_mode is one of recorded, oversize or malformed, defaults to recorded, is written by boot and stands in supervise.json too.
@row_add n=135 row=decision/harness-records-tolerate-unknown-fields area=3-test-harness-contract rec=fct-bilmaai6@2
name: Harness records tolerate missing and unknown fields
rule: The harness crate never denies unknown fields, so an older session record lacking a newer field still parses.
@row_add n=136 row=contract/harness-cli/session area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness CLI
item: --session
promise: --session with an id selects a session, allowing parallel sessions.
@row_add n=137 row=contract/harness-exit-codes/0 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 0
promise: The harness command succeeded.
@row_add n=138 row=contract/harness-exit-codes/1 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 1
promise: The harness command failed.
@row_add n=139 row=contract/harness-exit-codes/2 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 2
promise: The harness command was given a usage error, and its document carries reason usage and a detail.
@row_add n=140 row=contract/harness-stdout/document area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness stdout
item: document
promise: Every harness command prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order.
@row_add n=141 row=contract/harness-usage-detail/invalid-session-id area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: invalid-session-id
promise: The session id given is not valid.
@row_add n=142 row=contract/harness-usage-detail/session-exists area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: session-exists
promise: The session named already exists.
@row_add n=143 row=contract/harness-usage-detail/invalid-instance area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: invalid-instance
promise: The instance named is not valid.
@row_add n=144 row=contract/harness-usage-detail/arguments area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: arguments
promise: The arguments do not parse, including a flag or selector whose surface is not built yet.
@row_add n=145 row=contract/harness-usage-detail/no-supervise-spec area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: no-supervise-spec
promise: No supervise spec exists for the request.
@row_add n=146 row=contract/harness-usage-detail/unknown-suite area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: unknown-suite
promise: A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.
@row_add n=147 row=decision/unbuilt-surface-is-usage-error area=3-test-harness-contract rec=fct-sqrvsbbz@1
name: Unbuilt harness surface is a usage error
rule: A harness flag or selector whose surface is not built yet is a usage error, never a vacuous pass.
@row_none n=28 rec=fct-tqclwfnu@2 type=S
@row_add n=148 row=decision/doctests-via-cargo-test area=4-unit-test-strategy rec=fct-uptme7ma@1
name: Doctests run through cargo test
rule: Doctests run through cargo test --workspace --doc.
why: nextest cannot run doctests
@row_add n=149 row=decision/llvm-cov-coverage area=4-unit-test-strategy rec=fct-uptme7ma@1
name: cargo-llvm-cov measures coverage
rule: Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
@row_none n=29 rec=fct-oiewcpcp@1 type=S
@row_add n=150 row=decision/rstest-fixtures-for-constructors area=4-unit-test-strategy rec=fct-xesm7yum@1,fct-o2g5knlj@1
name: rstest fixtures for shared constructors
rule: Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.
@row_add n=151 row=decision/one-test-per-recorded-hook-payload area=4-unit-test-strategy rec=fct-5h2dowad@1
name: One generated test per recorded hook payload
rule: Unit tests generate one test case per recorded hook payload fixture file through rstest's files attribute.
@row_add n=152 row=decision/proptest-parser-invariants area=4-unit-test-strategy rec=fct-klpawgfb@1
name: Proptest for parser invariants
rule: Parser invariants are held by proptest property blocks run at 512 cases.
@row_add n=153 row=decision/insta-snapshots-for-decision-bodies area=4-unit-test-strategy rec=fct-w3h7uvct@1
name: Insta snapshots for decision bodies
rule: Decision bodies are pinned by insta JSON snapshots with the dialog_id and ts fields redacted.
@row_none n=30 rec=fct-oyk2rij4@1 type=S
@row_none n=31 rec=fct-ymia7ofg@1 type=5
@row_add n=154 row=decision/pty-exit-from-wait-not-eof area=4-unit-test-strategy rec=fct-sbml5vi6@2
name: PTY exit reported from wait, never from reader EOF
rule: viola-pty reports a child's exit from wait on the handle, never from reader EOF; consumers are tested with a mockall double of the six-method seam whose reader never returns EOF while wait returns an exit status.
@row_none n=32 rec=fct-2hayha6u@2 type=5
@row_none n=33 rec=fct-7vfmbe3m@2 type=5
@row_add n=155 row=decision/identity-floor-tested-against-literal-copy area=4-unit-test-strategy rec=fct-nr357doe@2
name: Identity floor tested against a literal copy
rule: The R8 strip is tested against a literal copy of the identity floor, never an imported one.
@row_add n=156 row=decision/no-assertion-beyond-compiled-literals area=4-unit-test-strategy rec=fct-nr357doe@2
name: No assertion beyond compiled literals
rule: viola-agent-claude unit tests never assert agent content beyond the compiled literals.
@row_add n=157 row=contract/mcp-tools-list/tools-list area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP tools/list
item: tools/list
promise: tools/list is exactly send, wait, last, answer, list.
@row_add n=158 row=contract/mcp-tool-arguments/invalid-argument area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP tool arguments
item: invalid argument
promise: A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.
@row_add n=159 row=contract/mcp-stdin/line-cap area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP stdin
item: line cap
promise: A line of exactly MAX_FRAME bytes, its newline counted, passes whole; one byte more passes no byte, reads as end of stream, and serve ends the session.
@row_add n=160 row=contract/viola-mcp-exit/oversize-before-initialize area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: viola mcp exit
item: oversize line before initialize
promise: A stdin line past MAX_FRAME before initialize completes makes viola mcp exit 1 with both streams empty and one parse-rejected mcp-stdin / oversize line of count 1.
@row_add n=161 row=contract/reply-screen/refusal area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_add n=162 row=contract/reply-screen/detail area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: detail
promise: A string detail that is no code word reads null.
@row_add n=163 row=contract/reply-screen/peer-fault area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: peer fault
promise: A peer fault reads through sanitised_fault.
@row_none n=34 rec=fct-xcvr2ftc@1 type=5
@row_add n=164 row=decision/mocks-only-at-product-seams area=4-unit-test-strategy rec=fct-zaiak6ok@1
name: Mocks only at product seams
rule: Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
@row_none n=35 rec=fct-fljf6obl@1 type=S
@row_none n=36 rec=fct-ict4jqoa@1 type=S
@row_none n=37 rec=fct-gr2ozii7@1 type=S
@row_none n=38 rec=fct-h5z4y4ot@1 type=S
@row_none n=39 rec=fct-vmaqxk5t@1 type=S
@row_none n=40 rec=fct-obagkhyb@1 type=S
@row_add n=165 row=decision/wrapper-channel-integration-driver area=5-integration-test-strategy rec=fct-f5kgr4ed@1,fct-e73lmy47@1
name: Wrapper channel integration driver
rule: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the product's schemars schemas.
@row_add n=166 row=decision/cli-outputs-pinned-by-literal area=5-integration-test-strategy rec=fct-s42l2rwd@2,fct-e73lmy47@1
name: CLI outputs pinned by literal asserts
rule: CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
@row_add n=167 row=decision/human-layouts-pinned-by-literal area=5-integration-test-strategy rec=fct-klrvzf5j@2,fct-e73lmy47@1
name: Human-mode layouts pinned by literal
rule: Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files.
guard: HELP literal in cli_output_plain; cli_list board tests
@row_add n=168 row=decision/real-pty-in-integration area=5-integration-test-strategy rec=fct-5vfqglut@1,fct-e73lmy47@1
name: Real PTY in integration tests
rule: Integration tests use the real portable-pty through viola-pty.
@row_add n=169 row=decision/real-disk-in-integration area=5-integration-test-strategy rec=fct-jl2uwhfc@2,fct-e73lmy47@1
name: Real disk in integration tests
rule: Integration tests use a real tempfile filesystem.
@row_none n=41 rec=fct-nmpwd4ab@1 type=S
@row_add n=170 row=decision/module-integration-direct-calls area=5-integration-test-strategy rec=fct-crxvk5ao@1
name: Module integration by direct calls
rule: Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.
@row_add n=171 row=decision/on-disk-state-tested-on-real-files area=5-integration-test-strategy rec=fct-rhuz6eg7@1
name: On-disk state tested on real files
rule: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
@row_add n=172 row=decision/ipc-tested-on-real-endpoint area=5-integration-test-strategy rec=fct-ctentyeb@2
name: IPC tested on a real endpoint
rule: IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
@row_add n=173 row=decision/pty-tested-on-real-openpty area=5-integration-test-strategy rec=fct-nvtf2boy@2
name: PTY tested on real openpty spawns
rule: PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size.
@row_add n=174 row=decision/hook-exec-against-live-wrapper area=5-integration-test-strategy rec=fct-3aeso5ca@1
name: Hook exec tested against a live wrapper
rule: viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted.
@row_none n=42 rec=fct-xq6vpu6l@1 type=4
relied: The watch report carries a stalled wrapper's state at the readiness deadline, before the panic, and a passing boot adds nothing to it.
@row_none n=43 rec=fct-cl3ld5uh@1 type=S
@row_add n=175 row=contract/wrapper-channel/-32600 area=5-integration-test-strategy rec=fct-r2yuicrg@1
boundary: the wrapper channel
item: -32600
promise: An oversize frame is answered with -32600 and the connection closed.
@row_add n=176 row=contract/event-line/round-trip area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: the event line
item: event kind
promise: Every event kind round-trips the event line.
@row_add n=177 row=contract/event-tailing/ignored-entries area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: event tailing
item: symlinks and invalid names
promise: Tailing ignores symlinks and invalid names.
@row_add n=178 row=contract/strict-modes-refusal/run-mcp-revive-verify area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: run, mcp, viola revive, viola verify
promise: A strict-modes refusal exits 1.
@row_add n=179 row=contract/strict-modes-refusal/mcp-frame-tools area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: MCP frame tools
promise: A strict-modes refusal in the MCP frame tools gives instance-unreachable.
@row_add n=180 row=contract/strict-modes-refusal/21 area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: 21
promise: Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
@row_add n=181 row=contract/strict-modes-refusal/no-line area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: no line
promise: A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
@row_add n=182 row=contract/viola-hook/exit-and-stderr area=5-integration-test-strategy rec=fct-627s4o4i@1
boundary: viola hook
item: exit and stderr
promise: Every hook event exits 0 with empty stderr.
@row_add n=183 row=contract/viola-hook/viola-dir area=5-integration-test-strategy rec=fct-627s4o4i@1
boundary: viola hook
item: VIOLA_DIR
promise: A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
@row_add n=184 row=contract/cli-output/no-escape-without-tty area=5-integration-test-strategy rec=fct-w5xcngvx@3
boundary: CLI output
item: escape sequences
promise: Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
@row_add n=185 row=contract/cli-argv/leading-slash area=5-integration-test-strategy rec=fct-w5xcngvx@3
boundary: CLI argv
item: leading slash
promise: A leading-slash argument is never delivered as a prompt.
@row_add n=186 row=decision/board-on-the-read-rule area=5-integration-test-strategy rec=fct-w5xcngvx@3
name: Board held to the read rule
rule: viola list refuses a home another user may read, with and without an instance.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_add n=187 row=decision/test-started-viola-drops-env area=5-integration-test-strategy rec=fct-4jpum6e2@1
name: Test-started viola drops inherited env
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
@row_add n=188 row=decision/mcp-handlers-use-tokio-client area=5-integration-test-strategy rec=fct-gri7u5le@1
name: MCP handlers use the Tokio client
rule: MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
@row_add n=189 row=contract/mcp-frame-tools/socket-dir-refused area=5-integration-test-strategy rec=fct-gri7u5le@1
boundary: MCP frame tools
item: socket-dir-refused
promise: A loosened socket directory gives isError with socket-dir-refused, no frame and no line.
@row_none n=44 rec=fct-znwunjjb@1 type=S
@row_none n=45 rec=fct-nywojhlo@1 type=S
@row_none n=46 rec=fct-odgitjio@1 type=S
@row_none n=47 rec=fct-yo2sxb4f@1 type=5
@row_add n=190 row=decision/live-readings-not-a-test-layer area=6-e2e-test-strategy rec=fct-6ugas3oa@1
name: Live readings are not a test layer
rule: A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
@row_none n=48 rec=fct-at6p4owu@1 type=S
@row_none n=49 rec=fct-43gyt7hv@1 type=5
@row_add n=191 row=contract/run-start-records/first-three area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the event log at viola run start
item: first three records
promise: The first three records are wheel{cause:"start"}, budget-gate and the hook's session-start, and the child is spawned only after them.
@row_add n=192 row=contract/snapshot/run-fields area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the instance snapshot
item: pinned path, pids and endpoint
promise: The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
@row_add n=193 row=contract/viola-run-exit-codes/1 area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: viola run exit codes
item: 1
promise: A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
@row_add n=194 row=contract/generated-plugin/hook-commands area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the generated plugin
item: hook commands
promise: A restart rewrites plugin/ with absolute hook commands, never a PATH lookup, and the settings.json override.
@row_none n=50 rec=fct-57rdoxhz@1 type=5
@row_add n=195 row=contract/viola-send/confirmed area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: confirmed send
promise: A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
@row_add n=196 row=contract/viola-send/cr-lf-text area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: text ending in or holding CR or LF
promise: A text ending in or holding CR or LF is confirmed as its typed text.
@row_add n=197 row=contract/viola-send/not-delivered area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: not-delivered
promise: A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
@row_add n=198 row=contract/viola-send/unconfirmable area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: unconfirmable
promise: A listed local command is confirmed only by its post-condition, or else is unconfirmable.
@row_add n=199 row=contract/mcp-send/ok-keys area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: the MCP send tool
item: ok result keys
promise: The MCP send result carries the same ok key set as the CLI.
@row_none n=51 rec=fct-yckihvdn@1 type=5
@row_add n=200 row=contract/viola-wait/wake area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait
item: wake on turn-ended
promise: wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
@row_add n=201 row=contract/viola-wait/timed-out area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait
item: timed_out
promise: A wait past the log end times out with {timed_out:true}.
@row_add n=202 row=contract/viola-last/newest-message area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola last
item: newest turn message
promise: last returns the newest turn's message, and the MCP result is equal to the CLI's.
@row_add n=203 row=contract/wait-and-last/killed-wrapper area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait and last
item: killed wrapper
promise: A killed wrapper gives CLI exit 21 and MCP instance-unreachable.
@row_none n=52 rec=fct-zjvldsf3@1 type=5
@row_add n=204 row=contract/dialog-event/logged-once area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: the event log
item: dialog event
promise: Each dialog event is logged exactly once with a dialog_id, and wait returns it at once.
@row_add n=205 row=contract/hook-stdout/decision-body area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: hook stdout
item: decision body
promise: The hook prints the exact decision body per dialog kind, pinned by insta snapshots per CLI version.
@row_add n=206 row=contract/hook-stdout/concurrent-dialog area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: hook stdout
item: second concurrent dialog
promise: A second concurrent dialog gets an empty hook stdout.
@row_add n=207 row=decision/decision-body-gate area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
name: Decision body gate
rule: No dialog decision body is ever emitted unless the wheel is driver and the CLI version is stamped.
@row_none n=53 rec=fct-h73p3622@1 type=5
@row_add n=208 row=contract/wheel-record/human-input area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: the event log
item: wheel human-input
promise: A human key logs wheel{holder:"human",cause:"human-input"} and the human's prompt still lands.
@row_add n=209 row=contract/viola-send/exit-10 area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: viola send
item: 10
promise: A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
@row_add n=210 row=contract/viola-release/exit-20 area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: viola release
item: 20
promise: release returns the wheel; release with from exits 20.
@row_add n=211 row=decision/injected-turn-keeps-wheel area=6-e2e-test-strategy rec=fct-x4oyim7i@1
name: Harness-injected turn keeps the wheel
rule: A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
@row_add n=212 row=decision/no-release-in-driver-hints area=6-e2e-test-strategy rec=fct-x4oyim7i@1,fct-jtonrpse@2,fct-ndx7lm2p@2
name: No release in driver-facing hints
rule: Driver-facing hints, refusal hints included, never suggest or name viola release.
guard: no_hint_names_release
@row_none n=54 rec=fct-g6cib4bm@1 type=5
@row_add n=213 row=contract/hook-statusline/passthrough area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola hook statusline
item: statusline output
promise: The statusline output passes through byte for byte, and budget.json holds the fed reading.
@row_add n=214 row=contract/viola-send/exit-11 area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola send
item: 11
promise: A budget crossing logs budget-gate{paused:true} and send is refused exit 11 with its window word.
@row_add n=215 row=decision/budget-pause-threshold area=6-e2e-test-strategy rec=fct-jtonrpse@2
name: Budget pause threshold
rule: A budget reading of exactly 90 pauses the driver and 89 does not.
@row_add n=216 row=contract/viola-release/budget area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola release
item: --budget
promise: release --budget lifts the budget pause for its own instance only.
@row_add n=217 row=decision/statusline-command-after-strict-modes area=6-e2e-test-strategy rec=fct-jtonrpse@2
name: Status-line command gated by strict modes
rule: The configured statusline_command never runs when the strict-modes check fails.
@row_none n=55 rec=fct-f6lswl4x@1 type=5
@row_add n=218 row=decision/unverified-cli-send-only area=6-e2e-test-strategy rec=fct-v74hzzdp@1
name: Unverified CLI degrades to send only
rule: On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
@row_none n=56 rec=fct-irpt6eev@1 type=5
@row_add n=219 row=decision/unwrapped-name-never-a-target area=6-e2e-test-strategy rec=fct-nzkgjegi@1
name: Unwrapped row name is never a target
rule: A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target.
guard: e1_send_naming_an_unwrapped_row_is_unreachable
@row_none n=57 rec=fct-j7upv7f4@1 type=5
@row_none n=58 rec=fct-5ylrta5q@1 type=5
@row_add n=220 row=decision/child-env-strip area=6-e2e-test-strategy rec=fct-tenptmlu@2
name: Child environment strip
rule: No stripped environment name reaches the agent child.
guard: the secret scan of the canary values
except: a name listed in claude_env_keep survives
@row_add n=221 row=decision/child-holds-no-viola-paths area=6-e2e-test-strategy rec=fct-tenptmlu@2
name: Child holds no viola socket or home path
rule: The agent child is given no viola socket or home path.
@row_add n=222 row=contract/process-start/claude-child area=6-e2e-test-strategy rec=fct-tenptmlu@2
boundary: the event log
item: process-start{subject:"claude-child"}
promise: The record carries the strip counts and the stripped names only, never their values.
@row_none n=59 rec=fct-tdook54p@1 type=5
@row_add n=223 row=decision/link-is-idempotent area=6-e2e-test-strategy rec=fct-b5i2ykog@1,fct-lhyzlw5x@1
name: Link is idempotent
rule: Re-linking a driver to the same driven instance adds no second link record.
@row_add n=224 row=contract/event-log/unlink area=6-e2e-test-strategy rec=fct-b5i2ykog@1
boundary: the event log
item: unlink record
promise: viola unlink logs an unlink record.
@row_none n=60 rec=fct-qyyocm3q@1 type=5
@row_add n=225 row=decision/driver-name-self-reported area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Driver identity is self-reported
rule: The driver of a link is the self-reported VIOLA_NAME and need not be a booted instance.
@row_add n=226 row=decision/unlink-survives-restart area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Unlink survives a wrapper restart
rule: An unlink holds across a wrapper restart.
guard: links_an_unlink_survives_a_wrapper_restart
@row_add n=227 row=decision/implicit-link-on-send area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Implicit link on send or answer
rule: A send or answer from another instance records an implicit link.
@row_none n=61 rec=fct-giovpuyr@1 type=5
@row_add n=228 row=decision/start-hands-wheel-to-driver area=6-e2e-test-strategy rec=fct-uq5phmo2@1
name: Start hands the wheel to the driver
rule: Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
@row_none n=62 rec=fct-jiq73dx3@1 type=5
@row_add n=229 row=decision/hook-suites-in-integration-layer area=6-e2e-test-strategy rec=fct-guyemqyz@1
name: Non-path suites layer
rule: Non-path trigger-coverage suites run in the E2E layer.
except: root hook_* files run in the integration layer
@row_add n=230 row=contract/mcp-tools/invalid-params area=6-e2e-test-strategy rec=fct-4aawmv2h@2
boundary: MCP tools
item: invalid-params
promise: A schema-violating call is answered invalid-params naming its field, and no frame is written.
@row_add n=231 row=decision/panic-seam-test-only area=6-e2e-test-strategy rec=fct-4aawmv2h@2
name: Panic seam is test-only
rule: The forced-panic seam fires only under the fake-agent feature and never reaches the release build.
@row_add n=232 row=decision/error-payloads-sanitized area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Error payloads sanitized
rule: No error payload carries an absolute path, a serde path or a Caused by chain.
guard: tests/contract_error_bodies.rs census of writers
@row_add n=233 row=decision/diagnostics-files-owner-only area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Diagnostics files owner-only
rule: Diagnostics files are mode 0600 on Unix.
guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
@row_add n=234 row=decision/scan-hits-never-echo-bytes area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Scan hits never echo bytes
rule: A secret-scan hit is reported by place and class, never by the matched bytes.
@row_add n=235 row=decision/diagnostics-schema-conformance area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Diagnostics schema conformance
rule: After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step.
why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
@row_add n=236 row=decision/jsonschema-validator area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: jsonschema validates diagnostics
rule: jsonschema is what validates diagnostics lines against their schemas.
@row_add n=237 row=decision/non-json-lines-skipped area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Non-JSON diagnostics lines skipped
rule: A diagnostics line that is not JSON is skipped and counted, not failed.
why: chaos cases tear lines by design
@row_add n=238 row=decision/schema-failure-no-content area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Schema failure report without content
rule: A schema failure prints file, line number and the failing schema keyword, never the line's content.
@row_add n=239 row=decision/hint-names-its-cause area=6-e2e-test-strategy rec=fct-ndx7lm2p@2
name: Each exit cause has its own hint
rule: Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint.
guard: no_two_codes_share_a_hint
@row_add n=240 row=contract/exit-causes/socket-dir-refused area=6-e2e-test-strategy rec=fct-ndx7lm2p@2
boundary: exit causes
item: socket-dir-refused
promise: A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
@row_add n=241 row=decision/socket-dir-refusal area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Socket directory refusal
rule: A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named.
guard: security_negatives_socket_dir
@row_add n=242 row=decision/link-name-validation area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Raw-channel link name validation
rule: link and unlink on the raw channel validate their names.
guard: channel_links
@row_add n=243 row=decision/home-readable-by-others-refused area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Home readable by others refused
rule: A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_add n=244 row=decision/frame-verbs-refuse-widened-instance area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Frame verbs refuse a widened instance
rule: The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line.
guard: security_negatives_strict_modes
@row_add n=245 row=decision/list-board-refusal-opens-no-log area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Board refusal opens no log
rule: viola list's whole-board strict-modes refusal exits 21 and opens no log.
guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
@row_add n=246 row=decision/plugin-install-refusal-creates-nothing area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Plugin install refusal creates nothing
rule: viola plugin install refused by strict modes exits 21 and creates nothing.
guard: strict_modes_on_plugin_install_is_exit_21_and_creates_nothing
@row_add n=247 row=decision/list-names-escaped area=6-e2e-test-strategy rec=fct-nsd7seww@1
name: List names escaped
rule: Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell.
guard: cross_list_rows
@row_add n=248 row=contract/mcp-cli-parity/refusal area=6-e2e-test-strategy rec=fct-4hhqtfrz@1
boundary: MCP and CLI parity
item: refusal
promise: For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.
@row_add n=249 row=contract/mcp-cli-parity/exit-21 area=6-e2e-test-strategy rec=fct-4hhqtfrz@1
boundary: MCP and CLI parity
item: exit 21 causes
promise: For each cause of exit 21 the MCP isError body {error, detail} equals the CLI --json document less v, its text refused: instance-unreachable.
@row_add n=250 row=decision/torn-append-healed area=6-e2e-test-strategy rec=fct-47fi4ugg@2
name: Torn append healed
rule: A torn log append is counted and the next record starts on a fresh line.
@row_add n=251 row=contract/viola-wait/endpoint-lost area=6-e2e-test-strategy rec=fct-47fi4ugg@2
boundary: viola wait
item: endpoint-lost
promise: An endpoint vanishing during wait exits 21 with endpoint-lost.
@row_add n=252 row=contract/viola-run/stale-heartbeat area=6-e2e-test-strategy rec=fct-47fi4ugg@2
boundary: viola run
item: stale-heartbeat
promise: A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
@row_add n=253 row=decision/forward-compat-lines-skipped area=6-e2e-test-strategy rec=fct-47fi4ugg@2
name: Forward-compatible lines skipped
rule: Log lines of a newer form are counted in skipped rather than failing the read.
@row_add n=254 row=decision/fuzz-separate-workspace area=6-e2e-test-strategy rec=fct-advgjcvl@1
name: Fuzz targets in a separate workspace
rule: cargo-fuzz targets live in a separate fuzz workspace and join as their parsers land.
@row_add n=255 row=decision/fake-agent-matches-recordings area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-vnlgjdeq@1
name: Fake agent matches recordings byte for byte
rule: For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
@row_add n=256 row=decision/pinned-exe-hash-vectors area=6-e2e-test-strategy rec=fct-q6ltykaz@1
name: Pinned-exe hash pinned by vectors
rule: The pinned-exe content hash is pinned to its library by literal test vectors.
@row_add n=257 row=decision/no-unbounded-json-depth area=6-e2e-test-strategy rec=fct-q6ltykaz@1
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is banned.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_none n=63 rec=fct-r3lcwsm7@1 type=S
@row_none n=64 rec=fct-mmqn6p7s@1 type=S
@row_add n=258 row=decision/rstest-fixtures area=7-test-data-fixtures rec=fct-khyzsutg@1
name: rstest for fixtures
rule: rstest is what provides test fixtures and cases.
@row_add n=259 row=decision/tempfile-test-dirs area=7-test-data-fixtures rec=fct-khyzsutg@1
name: tempfile for test directories
rule: tempfile is what provides temporary test directories.
@row_add n=260 row=decision/proptest-strategies area=7-test-data-fixtures rec=fct-khyzsutg@1
name: proptest for generated data
rule: proptest strategies, with proptest-derive, are what generate property-test data.
@row_add n=261 row=decision/insta-snapshots area=7-test-data-fixtures rec=fct-khyzsutg@1
name: insta for snapshots
rule: insta is what holds snapshot expectations.
@row_add n=262 row=decision/fixtures-per-cli-version area=7-test-data-fixtures rec=fct-khyzsutg@1
name: Recorded fixtures per CLI version
rule: Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
@row_none n=65 rec=fct-mx7xzaet@1 type=S
@row_add n=263 row=decision/product-state-from-real-runs area=7-test-data-fixtures rec=fct-54gkcbby@2
name: Product state from real runs only
rule: On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written.
except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json
@row_add n=264 row=decision/stamps-written-only-by-verify area=7-test-data-fixtures rec=fct-4py23xr3@1
name: Stamps written only by verify
rule: ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it.
@row_add n=265 row=decision/budget-file-from-statusline-hook area=7-test-data-fixtures rec=fct-sgyzq76e@1
name: Budget file from the statusline hook
rule: budget.json comes from generated statusline stdin piped into viola hook statusline; the reader's own unit cases alter only bytes the writer wrote.
@row_add n=266 row=decision/statusline-source-read-only area=7-test-data-fixtures rec=fct-iwzuwruj@1
name: Statusline source is read-only to viola
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_add n=267 row=decision/fake-agent-on-test-path area=7-test-data-fixtures rec=fct-ew5v6fx5@1
name: Fake agent on a test-scoped PATH
rule: The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
@row_add n=268 row=decision/hook-payloads-recorded-locally area=7-test-data-fixtures rec=fct-tq5dzqhd@1
name: Hook payloads recorded locally, replayed in CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
@row_add n=269 row=decision/dirty-payload-refuses-recording area=7-test-data-fixtures rec=fct-tq5dzqhd@1,fct-auxau5hd@1
name: Dirty payload refuses the recording
rule: A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
@row_add n=270 row=decision/human-output-as-literals area=7-test-data-fixtures rec=fct-puo2hazj@2
name: Human-mode output as literals
rule: Expected human-mode output, such as help text and the list board, is held as literals in its test files.
@row_add n=271 row=decision/decision-bodies-snapshotted area=7-test-data-fixtures rec=fct-qx745xaf@1
name: Decision bodies snapshotted
rule: Dialog decision bodies are held as insta snapshots, regenerated only by the local fixture refresh.
@row_add n=272 row=decision/proptest-persisted-seeds area=7-test-data-fixtures rec=fct-ehbrsv74@1,fct-fpfimgp4@1
name: proptest with persisted seeds
rule: Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.
@row_add n=273 row=decision/injected-clock area=7-test-data-fixtures rec=fct-jrpg7it3@1,fct-sjtqshx5@1,fct-46mmbbun@1
name: injected clock for time-sensitive data
rule: Time-sensitive data is injected per test behind viola_core::Clock, which tests advance; the wall clock is read only through that clock seam.
except: A link line's since is stamped from the wall clock and asserted by its shape only, and the budget gate's three evaluation points read the wall clock.
@row_add n=274 row=decision/time-as-argument area=7-test-data-fixtures rec=fct-jrpg7it3@1,fct-46mmbbun@1
name: pure time decisions take their instant
rule: A pure time decision, such as the budget gate's override expiry, takes its instant as an argument instead of reading a clock.
@row_add n=275 row=contract/fake-agent-cli/cli-version area=7-test-data-fixtures rec=fct-izkbqvor@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --cli-version
promise: The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
@row_add n=276 row=contract/fake-agent-hooks/command-spawn area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: hook command
promise: The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
@row_add n=277 row=contract/fake-agent-hooks/stdin-payload area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: hook stdin payload
promise: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
@row_add n=278 row=contract/fake-agent-hooks/status-line-command area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: status line command
promise: The fake agent runs a status line command only when the command is absolute.
@row_add n=279 row=contract/fake-agent-events/session-start area=7-test-data-fixtures rec=fct-cv6ihlor@1,fct-gbrktuzh@1
boundary: viola-fake-agent fired events
item: SessionStart
promise: After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire.
@row_add n=280 row=contract/fake-agent-input/bracketed-paste area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: bracketed paste
promise: Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.
@row_add n=281 row=contract/fake-agent-input/framing-paste area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: --framing paste
promise: Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.
@row_add n=282 row=contract/fake-agent-input/clear area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: /clear
promise: Under --framing the probed /clear fires the recorded clear-1 SessionEnd then SessionStart and no UserPromptSubmit.
@row_add n=283 row=contract/fake-agent-input/compact area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: /compact
promise: Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
@row_add n=284 row=contract/fake-agent-input/keystroke area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: keystroke
promise: Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
@row_add n=285 row=decision/control-file-gating area=7-test-data-fixtures rec=fct-ktnlwgrn@1
name: control-file gating of fake agent steps
rule: Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep.
why: Deterministic ordering without sleeps.
@row_add n=286 row=contract/fake-agent-cli/script area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --script
promise: The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
@row_add n=287 row=contract/fake-agent-cli/control area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --control
promise: The control path is always <home>/fake/<name>.control, and the fake agent reads it by byte offset.
@row_add n=288 row=decision/synthetic-fixture-text area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-lq7tfxzv@1
name: synthetic fixture text only
rule: Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
@row_add n=289 row=contract/fake-agent-receipt/line area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1
boundary: viola-fake-agent receipt file
item: receipt line
promise: The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.
@row_add n=290 row=contract/fake-agent-receipt/kind area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1
boundary: viola-fake-agent receipt file
item: kind
promise: Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, statusline, step.
@row_add n=291 row=contract/fake-script/format area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: script format
promise: A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.
@row_add n=292 row=contract/fake-script/unreadable-exit area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: unreadable script
promise: An unreadable script makes the fake agent exit 2.
@row_add n=293 row=contract/fake-script/gated-step area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: gated step
promise: A gated step waits for one more newline-terminated control line past the byte offset.
@row_add n=294 row=contract/fake-agent-cli/argv-only-options area=7-test-data-fixtures rec=fct-yxk5mpg3@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: modes and options
promise: The fake agent's modes and options are taken from argv only, never from the environment.
@row_add n=295 row=contract/fake-agent-cli/unknown-arguments area=7-test-data-fixtures rec=fct-yxk5mpg3@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: unknown arguments
promise: Unknown arguments are ignored.
@row_add n=296 row=contract/fake-agent-exit/exit-triggers area=7-test-data-fixtures rec=fct-7o4nbra2@1,fct-gbrktuzh@1
boundary: viola-fake-agent exit
item: exit trigger
promise: The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.
@row_add n=297 row=decision/fake-agent-children-own-process-group area=7-test-data-fixtures rec=fct-7o4nbra2@1
name: fake agent children in their own process group
rule: On Unix every command the fake agent spawns, each hook and the status line command, runs in a process group of its own.
why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.
@row_add n=298 row=decision/fake-agent-exit-ignores-sighup area=7-test-data-fixtures rec=fct-7o4nbra2@1
name: fake agent exit ignores SIGHUP
rule: On Unix the fake agent ignores SIGHUP from the first step of its exit (\x03 or stdin's end), while a hangup landing before the exit begins still ends it.
why: The agent is its PTY's session leader and cannot leave that group, so a hangup during exit would cut its own exit-time coverage write.
@row_none n=66 rec=fct-zinytcdj@1 type=S
@row_add n=299 row=decision/fixture-hygiene-walk area=7-test-data-fixtures rec=fct-k6vdnnmh@1,fct-dviikzsc@1
name: fixture hygiene run-time walk
rule: One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema.
why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file.
guard: claude_fixtures in tests/contract_fixture_hygiene.rs
except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.
@row_add n=300 row=decision/hygiene-reports-class-only area=7-test-data-fixtures rec=fct-qyi324hl@1
name: hygiene checker reports class only
rule: The hygiene checker reports a violation class only (absolute-path, username, schema), never the offending content.
@row_add n=301 row=decision/planted-hygiene-violations area=7-test-data-fixtures rec=fct-qyi324hl@1,fct-hsf33llc@1
name: planted inputs prove hygiene classes red
rule: Every hygiene violation class is proven red by a planted input.
@row_add n=302 row=decision/recorder-scrub area=7-test-data-fixtures rec=fct-eu46zdvx@1,fct-dviikzsc@1
name: recorder scrubs home paths and usernames
rule: The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub.
except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
@row_add n=303 row=decision/record-only-at-zero-failing-rows area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: fixtures recorded only when nothing fails
rule: The viola verify recorder writes fixtures only at 0 failing rows.
@row_add n=304 row=decision/screen-fixture-scrub-as-detector area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: screen fixtures checked scrub-as-detector
rule: A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
@row_add n=305 row=decision/mcp-tap-refuses-dirty-exchange area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: MCP tap refuses a dirty exchange whole
rule: The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
@row_add n=306 row=contract/verify-recording-refusal/unable-message area=7-test-data-fixtures rec=fct-eu46zdvx@1
boundary: viola verify fixture recording refusal
item: unable message
promise: A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
@row_none n=67 rec=fct-les7ny3j@1 type=S
@row_add n=307 row=decision/tests-seed-own-data area=7-test-data-fixtures rec=fct-r66s5egb@1
name: self-bootstrapping tests
rule: Tests seed their own data; no developer pre-populates a home.
@row_none n=68 rec=fct-pb23i77m@1 type=4
relied: The boot command creates the home, stamps it via viola verify and starts the wrappers, which tests rely on to seed their own data.
@row_none n=69 rec=fct-qwtscuv3@1 type=S
@row_add n=308 row=decision/per-test-home area=7-test-data-fixtures rec=fct-io5pl4i6@1
name: isolated home per test
rule: Each test has its own home, so no endpoint, log or budget.json is shared between tests.
@row_add n=309 row=decision/env-per-child area=7-test-data-fixtures rec=fct-io5pl4i6@1
name: environment set per child process
rule: Test environment is set per child process with Command::env, never with std::env::set_var.
why: std::env::set_var is unsafe in edition 2024.
@row_none n=70 rec=fct-wok64pug@1 type=4
relied: Per session, cleanup removes the harness home, session.json and the supervisor.
@row_add n=310 row=decision/ci-homes-kept-until-gates area=7-test-data-fixtures rec=fct-lrldphvg@1
name: CI homes kept until gate steps
rule: In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run.
except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
@row_none n=71 rec=fct-6nl4t77j@1 type=S
@row_none n=72 rec=fct-galwepqe@1 type=S
@row_none n=73 rec=fct-m3e2zfxx@1 type=S
@row_add n=311 row=decision/no-http-mocking area=8-mocking-stubbing-discipline rec=fct-yqmk3k5s@1,fct-qta73xkp@1
name: no HTTP mock library
rule: No HTTP mock library is used.
why: viola makes no outbound HTTP calls.
@row_add n=312 row=decision/fakes-for-external-services area=8-mocking-stubbing-discipline rec=fct-azuqhcwj@1
name: fakes replace external services
rule: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
@row_add n=313 row=decision/mcp-paused-tokio-clock area=8-mocking-stubbing-discipline rec=fct-sjtqshx5@1
name: paused tokio clock for viola-mcp timers
rule: viola-mcp tests use tokio's paused clock for a timer it owns, and a paused clock in a test never reaches a spawned wrapper.
@row_add n=314 row=decision/filesystem-not-mocked area=8-mocking-stubbing-discipline rec=fct-hkmaq6ye@2,fct-rddvvdrx@1
name: real filesystem in tests
rule: The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere.
why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
@row_none n=74 rec=fct-ahdpqenp@1 type=S
@row_add n=315 row=decision/pure-functions-tested-directly area=8-mocking-stubbing-discipline rec=fct-6xqpyyhr@1
name: pure functions tested directly
rule: Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
@row_add n=316 row=decision/no-mocks-in-e2e area=8-mocking-stubbing-discipline rec=fct-dlsufsua@1,fct-kjqvkfwd@1
name: nothing mocked in E2E
rule: E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
@row_none n=75 rec=fct-jw45xvx5@1 type=S
@row_add n=317 row=decision/no-http-fake-servers area=8-mocking-stubbing-discipline rec=fct-qull53ri@1
name: no HTTP fake servers
rule: Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used.
@row_none n=76 rec=fct-5hz2d655@1 type=S
@row_none n=77 rec=fct-vj56oiq2@1 type=S
@row_add n=318 row=decision/github-actions-ci area=9-ci-integration rec=fct-3x7gvt6f@2
name: GitHub Actions for CI
rule: CI runs on GitHub Actions: one workflow on push and pull request, and a scheduled nightly workflow for fuzz and weekly advisories.
@row_add n=319 row=absence/deploy-stage area=9-ci-integration rec=fct-3x7gvt6f@2
capability: a deploy stage
@row_add n=320 row=decision/rust-cache-only-cache area=9-ci-integration rec=fct-3x7gvt6f@2
name: rust-cache as the only CI cache
rule: rust-cache is the only cache, used in the push and pull-request workflow only.
@row_none n=78 rec=fct-ocmnyevd@1 type=S
@row_add n=321 row=decision/ci-lint-gate area=9-ci-integration rec=fct-346brcpe@2
name: CI lint gate
rule: CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion.
except: The acyclic module review is not a gate.
@row_add n=322 row=decision/supply-chain-fail-closed area=9-ci-integration rec=fct-vuxpgop2@1
name: fail-closed supply-chain job
rule: One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
@row_add n=323 row=decision/single-instrumented-test-run area=9-ci-integration rec=fct-onifqrhg@2,fct-wxdsyw3l@1
name: one instrumented coverage run in CI
rule: CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.
@row_add n=324 row=decision/jobs-end-in-harness-gate area=9-ci-integration rec=fct-onifqrhg@2
name: CI jobs end in the harness gate
rule: Each CI test job ends in viola-harness gate requiring its suites.
@row_add n=325 row=decision/mutation-not-a-ci-gate area=9-ci-integration rec=fct-fo2372wn@2
name: mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
@row_add n=326 row=decision/perf-verdict-from-harness area=9-ci-integration rec=fct-3q2uf724@2
name: perf verdict from the harness gate
rule: Perf runs in its own CI job, and its verdict comes from viola-harness gate, never from a jq binary.
@row_add n=327 row=decision/msrv-check area=9-ci-integration rec=fct-bpswkwdk@1,fct-kumfcynn@2
name: MSRV check on the security toolchain floor
rule: An ubuntu CI job checks the workspace and runs the unit suite on Rust 1.96, the minimum supported Rust version.
why: Rust 1.96 is the security toolchain floor.
@row_add n=328 row=decision/fuzz-corpus-replay-in-ci area=9-ci-integration rec=fct-mq72t7wc@1
name: Fuzz corpus replay in CI
rule: Every push and pull request replays the fuzz corpus, and a nightly run fuzzes each target for a bounded time.
@row_add n=329 row=decision/quality-gate-command area=9-ci-integration rec=fct-bzvycnyj@1,fct-phqnbmfn@1
name: One quality gate over the test outputs
rule: The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.
@row_add n=330 row=decision/workflow-least-permissions area=9-ci-integration rec=fct-r5xptmoc@1
name: Least workflow permissions
rule: Every CI workflow grants no permissions at the top and only read access to contents per job.
@row_add n=331 row=decision/single-tool-version-source area=9-ci-integration rec=fct-r5xptmoc@1
name: One version source for each tool
rule: Each tool has exactly one version source, the CI workflow's pins, and the local pre-push gate checks the host's tools against them and installs nothing.
@row_add n=332 row=decision/no-ci-concurrency-groups area=9-ci-integration rec=fct-7mqjkvmt@1
name: No concurrency groups in CI
rule: No CI workflow has a concurrency block, and zizmor's concurrency-limits finding is declined and left visible.
why: A concurrency group cancels pending runs, which would drop a push's always-run gate and upload chain.
@row_add n=333 row=decision/linux-only-ci-matrix area=9-ci-integration rec=fct-shyrfs6v@2,fct-q4ch4yge@1
name: Linux-only CI matrix
rule: CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
@row_add n=334 row=decision/pinned-rust-toolchain area=9-ci-integration rec=fct-kumfcynn@2
name: Exactly pinned Rust toolchain
rule: The Rust toolchain is pinned to one exact stable version at or above the 1.96 floor and installed by rustup on both the CI runner and the host.
@row_add n=335 row=decision/machine-parseable-ci-reports area=9-ci-integration rec=fct-yruy4jpq@1
name: Machine-parseable CI test reports
rule: CI test reports are machine-parseable and leave CI as uploaded artifacts.
@row_add n=336 row=decision/artifacts-behind-secret-scan area=9-ci-integration rec=fct-mos7j36w@2,fct-ap7wweiw@1
name: Artifacts leave CI behind a secret scan
rule: Every test or perf artifact leaves CI only after its job's secret scan passed; when the scan fails only the scan's hit report is uploaded.
@row_add n=337 row=contract/ci-artifacts/harness area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: harness-${{ matrix.os }}
promise: Uploaded by the test job on failure after a passed secret scan: the agent-run output minus the mutation diff, with per-suite JUnit, run summary, coverage summary and the failure-only logs and status capture; no perf export.
@row_add n=338 row=contract/ci-artifacts/diag area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: diag-${{ matrix.os }}
promise: Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.
@row_add n=339 row=contract/ci-artifacts/junit area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: junit-${{ matrix.os }}
promise: Holds the one JUnit report of the test job's single instrumented coverage nextest invocation.
@row_add n=340 row=contract/ci-artifacts/secret-scan area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: secret-scan-${{ matrix.os }}
promise: Holds the test job's secret-scan hit report, uploaded only when the scan failed.
@row_add n=341 row=decision/ci-artifact-retention area=9-ci-integration rec=fct-mos7j36w@2
name: Seven-day artifact retention
rule: Every CI test artifact is kept for 7 days.
@row_add n=342 row=decision/per-suite-junit-copies area=9-ci-integration rec=fct-mos7j36w@2
name: Per-suite JUnit copies kept
rule: Each suite's JUnit report is kept as its own copy among the run artifacts.
why: Each nextest invocation overwrites the raw JUnit file.
@row_add n=343 row=decision/mutation-output-never-uploaded area=9-ci-integration rec=fct-2ymlhohm@2
name: Mutation output never uploaded
rule: The mutation-testing output directory is never uploaded from CI.
why: Its outcomes file holds absolute argv paths and its log holds test output.
@row_add n=344 row=contract/ci-artifacts/perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: perf-${{ matrix.os }}
promise: Holds the perf job's hyperfine exports, uploaded only when the perf job's secret scan succeeded.
@row_add n=345 row=contract/ci-artifacts/diag-perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: diag-perf-${{ matrix.os }}
promise: Holds the perf run homes' diagnostics NDJSON files, uploaded only when the perf job's secret scan succeeded.
@row_add n=346 row=contract/ci-artifacts/secret-scan-perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: secret-scan-perf-${{ matrix.os }}
promise: Holds the perf job's secret-scan hit report, uploaded only when that scan failed.
@row_add n=347 row=decision/agent-reads-ci-through-gh area=9-ci-integration rec=fct-kprehror@1
name: Agent reads CI through gh
rule: The agent reads CI job conclusions and fetches artifacts with the gh CLI, and locally gets the same files from agent-run run and agent-run logs.
@row_add n=348 row=decision/build-fails-on-test-failure area=9-ci-integration rec=fct-q4zg6vp5@2,fct-u7ux54l5@1
name: Build fails on any test failure
rule: The build fails when any unit, integration, E2E, contract or property test fails.
@row_add n=349 row=decision/build-fails-below-coverage area=9-ci-integration rec=fct-6iojik4n@2,fct-u7ux54l5@1
name: Build fails below coverage threshold
rule: The build fails when coverage falls below the quality-gate threshold.
@row_add n=350 row=decision/build-fails-on-perf-breach area=9-ci-integration rec=fct-y7sxtbos@1,fct-u7ux54l5@1
name: Build fails on a performance budget breach
rule: The build fails on any hyperfine performance budget breach.
@row_add n=351 row=decision/build-fails-on-fuzz-crash area=9-ci-integration rec=fct-y7sxtbos@1,fct-u7ux54l5@1
name: Build fails on a fuzz replay crash
rule: The build fails when the fuzz corpus replay crashes.
@row_add n=352 row=decision/fmt-and-clippy-deny-warnings area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1
name: Formatting and clippy warnings fail the build
rule: Any rustfmt difference or clippy warning, with warnings denied, fails the build.
@row_add n=353 row=decision/rmcp-feature-limit area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1
name: rmcp limited to server and transport-io
rule: The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested.
guard: rmcp-features
@row_add n=354 row=decision/cargo-deny-gate area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: cargo-deny supply-chain gate
rule: Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build.
guard: cargo deny; deny-probes, one probe per ban
@row_add n=355 row=decision/channel-tokio-free-by-default area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: viola-channel reaches tokio only by feature
rule: The build fails when viola-channel reaches tokio without its feature enabled.
guard: channel-tokio-free
@row_add n=356 row=decision/zizmor-gate area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: zizmor findings fail the build
rule: Any zizmor finding on the CI workflows fails the build.
@row_add n=357 row=decision/weekly-advisories-run area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: Weekly advisories run
rule: A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
@row_add n=358 row=decision/comprehensive-coverage-floors area=10-quality-gates-coverage-targets rec=fct-nwua7cus@1,fct-4ixdwl5o@1,fct-ryyndnld@2
name: Comprehensive-tier coverage floors
rule: viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job.
guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
@row_add n=359 row=decision/coverage-workspace-rollup area=10-quality-gates-coverage-targets rec=fct-2xpzggiy@1
name: Coverage rolled up across the workspace
rule: The coverage number rolls up across all workspace crates.
@row_add n=360 row=decision/coverage-exclusions area=10-quality-gates-coverage-targets rec=fct-3mj52n2x@2
name: Coverage exclusions
rule: The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness.
why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
@row_add n=361 row=decision/child-process-coverage area=10-quality-gates-coverage-targets rec=fct-3zrfkvpu@1
name: Child viola processes counted in coverage
rule: viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE.
why: LLVM_PROFILE_FILE propagates to child processes.
except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
@row_add n=362 row=decision/doctests-outside-coverage area=10-quality-gates-coverage-targets rec=fct-hxwzxyew@1
name: Doctests outside the coverage number
rule: Doctests are run but are not part of the coverage number.
@row_add n=363 row=decision/mutation-gate-cadence area=10-quality-gates-coverage-targets rec=fct-xnmc6vwq@1
name: Mutation gate at the epoch boundary only
rule: Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
@row_add n=364 row=decision/host-unreachable-mutants-owed area=10-quality-gates-coverage-targets rec=fct-xnmc6vwq@1
name: Host-unreachable mutants recorded as owed
rule: A mutant the host cannot compile or reach is recorded as owed, never as equivalent or caught.
@row_add n=365 row=decision/perf-gate-on-max area=10-quality-gates-coverage-targets rec=fct-3c7vh3a2@1
name: Performance gate on the max sample
rule: The performance gate judges the max sample; mean, median and stddev are reported for trend only, and no percentile is computed or gated.
why: Claude Code hook deadlines are hard cutoffs.
@row_add n=366 row=decision/perf-own-release-build area=10-quality-gates-coverage-targets rec=fct-db2dcqzo@1,fct-nzahfwbg@1
name: Perf times its own release build
rule: The perf run times its own non-instrumented release workspace build with the fake agent, never a debug or coverage build, and never with LLVM_PROFILE_FILE set.
@row_add n=367 row=decision/perf-real-hook-path area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf times hooks through the real path
rule: The perf run boots one unstamped session with a builder instance and times each hook through the real logging and channel path on synthetic payloads.
@row_add n=368 row=decision/perf-panic-check area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf run checks for hook panics
rule: Before cleanup the perf run asserts no panic line in the home's role files.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=369 row=decision/perf-failure-turns-red area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf failures turn the suite red
rule: A failed build, boot, timed row, panic check or cleanup turns suite perf red, never a missing or skipped row.
@row_add n=370 row=decision/perf-hyperfine-invocation area=10-quality-gates-coverage-targets rec=fct-lqftgban@2
name: Hyperfine invocation shape
rule: Every hyperfine call runs without a shell, with 3 warm-up runs and 30 timed runs, exporting JSON to the run artifacts; warm-up runs are not part of max.
why: Warm-up runs absorb first-execution costs.
@row_add n=371 row=decision/perf-no-retry area=10-quality-gates-coverage-targets rec=fct-5j2njdi5@1
name: No retry of a performance breach
rule: The gate evaluates the exported perf JSON once, with no rerun, outlier trimming or second attempt; a breach is fixed in the product or escalated through a Decisions Log entry.
@row_add n=372 row=decision/spine-deadline-single-source area=10-quality-gates-coverage-targets rec=fct-ggkcj6j7@1
name: Spine deadline from the product constant
rule: The perf gate reads the spine deadline from the product's own constant (1.0 s), never from a literal edited into CI or the test plan.
@row_add n=373 row=decision/hook-connect-deadline area=10-quality-gates-coverage-targets rec=fct-ggkcj6j7@1
name: Hook connect deadline below the spine deadline
rule: The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces.
guard: connect_deadline_is_below_the_spine_deadline
state: provisional
@row_add n=374 row=decision/perf-in-own-ci-job area=10-quality-gates-coverage-targets rec=fct-vxnii5yh@3
name: Perf runs in its own CI job
rule: CI runs perf in its own perf job ending in a gate that requires perf; the test job and the pre-push gate carry no perf step, and no async-tier hook is timed.
@row_add n=375 row=decision/fail-open-spine-bound-at-unit-speed area=10-quality-gates-coverage-targets rec=fct-vxnii5yh@3
name: Fail-open spine bound asserted at unit speed
rule: Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed.
guard: hook_fails_open_silently_within_the_spine_bound
@row_add n=376 row=contract/perf-budgets/session-end area=10-quality-gates-coverage-targets rec=fct-yoazurel@2
boundary: Performance budgets
item: viola hook session-end
promise: Timed on a synthetic payload through the live wrapper; mean, median and stddev reported for trend, gated on max under 1.0 s.
@row_add n=377 row=contract/perf-budgets/spine-hooks area=10-quality-gates-coverage-targets rec=fct-7bp7lvdw@1
boundary: Performance budgets
item: spine hooks session-start, user-prompt-submit, stop
promise: Timed the same way with VIOLA_NAME and VIOLA_DIR exported to hyperfine; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_add n=378 row=contract/perf-budgets/pre-tool-use area=10-quality-gates-coverage-targets rec=fct-ermnme7k@1
boundary: Performance budgets
item: viola hook pre-tool-use
promise: Timed on an unverified CLI whose dialog answer is null at once, a required perf row; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_add n=379 row=contract/heartbeat-staleness/age-threshold area=10-quality-gates-coverage-targets rec=fct-4tlnkrfv@1
boundary: Heartbeat staleness classification
item: heartbeat age
promise: For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
@row_add n=380 row=contract/heartbeat-staleness/identity-mismatch area=10-quality-gates-coverage-targets rec=fct-4tlnkrfv@1
boundary: Heartbeat staleness classification
item: pid or start-time mismatch
promise: A pid or start-time mismatch classifies the instance gone at any age.
@row_none n=79 rec=fct-nc7rbvnm@1 type=5
@row_none n=80 rec=fct-gs6kxojs@1 type=5
@row_none n=81 rec=fct-oz6bj3oy@1 type=5
@row_none n=82 rec=fct-3g23erif@1 type=S
@row_none n=83 rec=fct-kohfqcyc@1 type=S
@row_none n=84 rec=fct-vua4yfw6@1 type=S
@row_none n=85 rec=fct-xugh2lfn@1 type=S
@row_none n=86 rec=fct-mmlpb66v@1 type=0
what: the coverage floors of the Minimal tier, a tier this project does not use
@row_none n=87 rec=fct-4jvy3xbt@1 type=0
what: the coverage floors of the Standard tier, a tier this project does not use
@row_none n=88 rec=fct-pzyggn3l@1 type=S
@row_none n=89 rec=fct-wcemoat6@1 type=S
@row_none n=90 rec=fct-hdwvrmvb@1 type=S
@row_none n=91 rec=fct-xsdmwdxz@1 type=S
@row_add n=381 row=decision/send-and-wait-latency-not-timed area=10-quality-gates-coverage-targets rec=fct-7l3hnykk@1,fct-umz6vque@1
name: Send confirmation and wait wake latency not timed
rule: The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed.
why: Both are bounded by product constants that are still open.
@row_add n=382 row=decision/boot-deadline-not-a-budget area=10-quality-gates-coverage-targets rec=fct-6txqcbhi@1,fct-umz6vque@1
name: Boot deadline is not a performance budget
rule: The 20 s `boot` deadline is a harness readiness timeout, deliberately not a product performance budget in the quality gates.
@row_add n=383 row=decision/hyperfine-max-perf-gate area=10-quality-gates-coverage-targets rec=fct-rvltlmnj@2,fct-22efn7xx@1,fct-7vidmhnu@1
name: hyperfine max is the performance gate
rule: Performance budgets, hook deadlines among them, are gated by hyperfine on `max`, never on `mean`, and a budget regression fails the build; criterion 0.8.2 is not a gate.
why: criterion 0.8.2 does not exit non-zero on a regression.
@row_add n=384 row=decision/hook-deadlines-asserted-at-unit-speed area=10-quality-gates-coverage-targets rec=fct-rvltlmnj@2
name: Hook deadlines asserted at unit speed
rule: The hook deadlines are also asserted at unit speed inside the hook subprocess tests, with a monotonic `std::time::Instant` bound.
@row_add n=385 row=decision/zero-flakiness area=10-quality-gates-coverage-targets rec=fct-zrh4qt5s@1,fct-2cc4sa5w@1,fct-go7zgoyw@1,fct-nqu6lr5l@1
name: Zero-flakiness budget
rule: Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is.
why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
@row_add n=386 row=decision/no-ignore-parking area=10-quality-gates-coverage-targets rec=fct-zrh4qt5s@1
name: No #[ignore] parking
rule: `#[ignore]` is not allowed as a parking place for a failing or flaky test.
why: Agent-driven development cannot tell a flake from a real bug.
@row_add n=387 row=decision/test-failure-fails-build area=10-quality-gates-coverage-targets rec=fct-plbmhdnw@2,fct-bphyjtz7@1
name: Any test failure fails the build
rule: A test failure at any level is a build failure condition.
@row_add n=388 row=decision/coverage-threshold-fails-build area=10-quality-gates-coverage-targets rec=fct-2ztzeoct@2,fct-bphyjtz7@1
name: Coverage below tier threshold fails the build
rule: Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
@row_add n=389 row=decision/mutants-verdict-counted-by-harness area=10-quality-gates-coverage-targets rec=fct-wjz5endv@1,fct-n6iwafhi@1
name: Mutation verdict counted by the harness
rule: At a `run --mutants` the harness counts missed, timeout, unviable and caught outcomes itself, leaving host-excluded missed mutants out; any other missed mutant, any timed-out mutant, or more unviable than caught mutants is red.
why: The cargo-mutants exit code and summary cannot be trusted alone: exit 3 masks exit 2, and the summary counts a host-excluded mutant as missed.
@row_add n=390 row=decision/lint-gates-fail-build area=10-quality-gates-coverage-targets rec=fct-ikzozfqb@1,fct-bphyjtz7@1
name: Lint and audit gates fail the build
rule: A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
@row_add n=391 row=decision/no-test-binary-in-release area=10-quality-gates-coverage-targets rec=fct-ikzozfqb@1,fct-bphyjtz7@1
name: No test-only binary in a release build
rule: A release build carrying a test-only binary is a build failure condition.
@row_none n=92 rec=fct-suizqs5j@1 type=S
@row_none n=93 rec=fct-tli2yavh@1 type=S
@row_add n=392 row=decision/e2e-asserts-only-cross-process area=11-test-anti-patterns-never-do-these rec=fct-a34t7dfp@1
name: E2E asserts only cross-process behaviour
rule: A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour.
why: The test pyramid.
@row_add n=393 row=decision/no-over-mocking-integration area=11-test-anti-patterns-never-do-these rec=fct-l46kd4t2@1
name: No over-mocking at integration level
rule: Integration tests never over-mock.
why: Over-mocking stops testing the integration.
@row_add n=394 row=decision/no-real-cli-in-ci area=11-test-anti-patterns-never-do-these rec=fct-lbxoplwk@1
name: No real claude CLI in CI
rule: The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too.
guard: `agent-run run --local-live` refuses when `CI` is set
@row_add n=395 row=decision/fake-agent-not-proof-of-real-cli area=11-test-anti-patterns-never-do-these rec=fct-egysilpo@1
name: Fake-agent run is not proof of real-CLI behaviour
rule: A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite.
why: The fake agent must not drift from the real CLI.
@row_add n=396 row=decision/test-public-behaviour-only area=11-test-anti-patterns-never-do-these rec=fct-7rhfof6k@1
name: Test public API and observable behaviour
rule: Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
@row_add n=397 row=decision/assert-on-contract-not-fixture area=11-test-anti-patterns-never-do-these rec=fct-6im6ezik@1
name: Assert on contract, not fixture internals
rule: Assertions target the contract, never brittle fixture-internal values.
@row_add n=398 row=decision/literal-test-oracles area=11-test-anti-patterns-never-do-these rec=fct-jfqpalye@1
name: Test oracles written as literals
rule: Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
@row_add n=399 row=decision/no-debug-string-comparison area=11-test-anti-patterns-never-do-these rec=fct-ee3ranmx@1
name: No comparison by Debug string
rule: Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
@row_add n=400 row=decision/real-on-disk-state area=11-test-anti-patterns-never-do-these rec=fct-7gyjknnu@1
name: On-disk state is never mocked
rule: Tests never mock the on-disk state and use a real tempfile home instead.
why: The architecture has no database; this is the database-equivalent rule.
@row_add n=401 row=decision/cleanup-between-integration-tests area=11-test-anti-patterns-never-do-these rec=fct-fmecvcgv@1
name: Cleanup between integration tests
rule: Integration tests never skip cleanup between tests.
why: Order-dependent tests hide bugs.
@row_add n=402 row=decision/endpoint-from-snapshot-only area=11-test-anti-patterns-never-do-these rec=fct-aqwq345t@1
name: Endpoint taken only from the snapshot
rule: Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
@row_add n=403 row=decision/no-process-env-mutation-in-tests area=11-test-anti-patterns-never-do-these rec=fct-ieax375m@1
name: No process-wide env mutation in tests
rule: Tests never set process environment variables globally; environment is set per child command.
why: Setting the process environment is unsafe in edition 2024 and races across threads.
@row_add n=404 row=decision/no-sleep-synchronization area=11-test-anti-patterns-never-do-these rec=fct-cdb2tplq@1
name: No sleep for synchronization
rule: Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
@row_add n=405 row=decision/no-screen-parsing area=11-test-anti-patterns-never-do-these rec=fct-nh5slhtz@1
name: Rendered child screen is never parsed
rule: Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
@row_add n=406 row=decision/child-exit-by-wait area=11-test-anti-patterns-never-do-these rec=fct-ukay3vyw@2
name: Child exit detected by waiting on the handle
rule: Child exit is detected by waiting on the child handle, never by reading master EOF.
@row_add n=407 row=decision/wrapper-spawned-with-guard area=11-test-anti-patterns-never-do-these rec=fct-ajzn4txb@2
name: Long-lived wrapper spawned with a drop guard
rule: Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call.
why: A blocking call waits until the wrapper exits.
@row_add n=408 row=decision/no-production-data-in-tests area=11-test-anti-patterns-never-do-these rec=fct-ic7ldbxq@1
name: No production data in tests
rule: Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames.
except: A `workspace.repo` naming this repository's own remote is kept.
@row_add n=409 row=decision/no-shared-mutable-state area=11-test-anti-patterns-never-do-these rec=fct-x76wnqv5@1
name: No shared mutable state across parallel tests
rule: Parallel tests never share mutable state.
@row_add n=410 row=decision/seeded-generators area=11-test-anti-patterns-never-do-these rec=fct-lg3od2u6@1
name: Seed-controlled generators
rule: Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
@row_add n=411 row=decision/state-files-never-hand-written area=11-test-anti-patterns-never-do-these rec=fct-o75kb2kj@1
name: State files are never hand-written
rule: Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand.
except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
@row_add n=412 row=decision/no-committed-test-secrets area=11-test-anti-patterns-never-do-these rec=fct-ejn23r66@1
name: No test dirs, env files or tokens committed
rule: `--home` test directories, `.env` files and tokens are never committed.
@row_add n=413 row=decision/no-mocking-unowned-code area=11-test-anti-patterns-never-do-these rec=fct-z5k5vjqr@1
name: No mocking of what is not owned
rule: Libraries and the language runtime are never mocked without an abstraction layer.
@row_add n=414 row=decision/no-over-stubbing area=11-test-anti-patterns-never-do-these rec=fct-epqm6key@1
name: No over-stubbing
rule: Tests never over-stub.
why: Over-stubbing stops testing real behaviour.
@row_add n=415 row=decision/no-runtime-monkey-patching area=11-test-anti-patterns-never-do-these rec=fct-sw7m3jb3@1
name: No runtime monkey-patching
rule: Production code is never monkey-patched at runtime; dependency injection is used instead.
@row_add n=416 row=decision/no-tokio-failpoint-crate area=11-test-anti-patterns-never-do-these rec=fct-mbozqxei@1
name: No failpoint crate pulling tokio
rule: No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
@row_add n=417 row=decision/real-hook-in-e2e area=11-test-anti-patterns-never-do-these rec=fct-ud76yym4@1
name: Real viola hook in E2E
rule: `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
@row_add n=418 row=decision/tests-run-in-parallel area=11-test-anti-patterns-never-do-these rec=fct-yijholqq@1
name: Tests run in parallel
rule: Tests are never serialised with a single thread; isolation comes from the per-test home.
except: The fuzz stages are single jobs, by design.
@row_add n=419 row=decision/rust-cache-only-in-ci area=11-test-anti-patterns-never-do-these rec=fct-qk7zl643@1,fct-tam4bqkb@1
name: rust-cache is the only cache, CI only
rule: Swatinem/rust-cache is the only cache in `ci.yml`, and no release workflow uses it.
@row_add n=420 row=decision/coverage-ignore-regex-not-widened area=11-test-anti-patterns-never-do-these rec=fct-b4avhaux@2
name: Coverage ignore regex is never widened
rule: The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first.
why: Widening it is a back door to lowering the coverage thresholds.
except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
@row_add n=421 row=decision/coverage-threshold-never-lowered area=11-test-anti-patterns-never-do-these rec=fct-r52fahdc@1
name: Coverage threshold never lowered to pass
rule: The coverage threshold is never lowered to pass a build.
why: Raise quality, not the bar.
@row_add n=422 row=decision/no-skipping-quality-gates area=11-test-anti-patterns-never-do-these rec=fct-55sujg7u@1
name: Quality gates never skipped
rule: Quality gates are never skipped, not even once.
@row_add n=423 row=decision/no-human-reviewed-visual-regression area=11-test-anti-patterns-never-do-these rec=fct-neagcwig@1
name: No human-reviewed visual regression
rule: Percy, Chromatic, Applitools and other visual regression with human review are never used.
@row_add n=424 row=decision/no-manual-smoke-step area=11-test-anti-patterns-never-do-these rec=fct-vds3ppgv@1
name: No manual smoke step
rule: No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
@row_add n=425 row=decision/no-recorder-authored-tests area=11-test-anti-patterns-never-do-these rec=fct-xjawgeec@1
name: No recorder-authored tests
rule: Tests are never authored by Selenium IDE or the Playwright codegen recorder.
@row_add n=426 row=decision/no-human-screenshot-review area=11-test-anti-patterns-never-do-these rec=fct-dx5w5yd6@1
name: No human screenshot review
rule: No test step relies on a human reviewing screenshots.
@row_add n=427 row=decision/no-real-network-in-tests area=11-test-anti-patterns-never-do-these rec=fct-bom5r3of@1
name: No real network calls in tests
rule: Tests never make real network calls; they use mocks or localhost stubs.
@row_add n=428 row=decision/no-test-owned-real-time area=11-test-anti-patterns-never-do-these rec=fct-45jodiii@1
name: No test-owned real time
rule: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict.
except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
@row_add n=429 row=decision/no-in-process-clock-across-processes area=11-test-anti-patterns-never-do-these rec=fct-v6in7u4x@1
name: No in-process clock for another process
rule: An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case.
why: A global mock clock reaches only the test process's threads, never a spawned viola run.
@row_add n=430 row=decision/no-interactive-snapshot-review area=11-test-anti-patterns-never-do-these rec=fct-qxrlql3x@2
name: No interactive snapshot review as a gate
rule: Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
@row_add n=431 row=decision/no-blanket-approval-tests area=11-test-anti-patterns-never-do-these rec=fct-m765nh6l@1
name: No blanket-approval tests
rule: No test auto-approves a dialog.
@row_add n=432 row=decision/no-test-passing-unstamped-decision-body area=11-test-anti-patterns-never-do-these rec=fct-m765nh6l@1
name: No test passing an unguarded decision body
rule: No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
@row_add n=433 row=decision/harness-output-json-and-exit-code area=11-test-anti-patterns-never-do-these rec=fct-jyezwnnp@1
name: Harness commands always give JSON and exit code
rule: Every harness command prints its JSON document and exit code, never a human-only message alone.
@row_add n=434 row=decision/no-flaky-test-retries area=11-test-anti-patterns-never-do-these rec=fct-jpy6mjkf@1
name: No retries for flaky tests
rule: No retry-once policy or any retry budget exists for flaky tests.
why: The zero-flakiness budget.
@row_add n=435 row=decision/proptest-seeds-committed area=11-test-anti-patterns-never-do-these rec=fct-3bpkukik@1
name: Property tests replay from committed seeds
rule: Property tests replay deterministically: proptest regression seeds persist in committed regression files.
@row_add n=436 row=decision/chaos-faults-explicit area=11-test-anti-patterns-never-do-these rec=fct-3bpkukik@1
name: Chaos faults only from explicit operations
rule: Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
@row_none n=94 rec=fct-e3n7qfim@1 type=S
@row_none n=95 rec=fct-bg247fzm@1 type=S
@row_add n=437 row=decision/lint-fmt-and-clippy-deny-warnings area=ci-integration rec=fct-pk32flwi@2
name: Lint denies format drift and clippy warnings
rule: Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on.
guard: CI job lint
@row_add n=438 row=decision/sync-crates-compile-without-features area=ci-integration rec=fct-pk32flwi@2
name: Sync crates compile alone without features
rule: Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails.
guard: CI job lint, one cargo check per listed crate
@row_add n=439 row=decision/module-acyclicity-not-gated area=ci-integration rec=fct-pk32flwi@2
name: Module acyclicity is a review, not a gate
rule: Module dependency acyclicity is an on-demand boundary review, not a gate.
why: cargo-modules reads every type and its own inherent method as a cycle whatever the filters.
@row_add n=440 row=decision/rmcp-features-server-transport-io area=ci-integration rec=fct-pk32flwi@2
name: rmcp takes exactly server and transport-io
rule: rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features.
guard: lint step rmcp features (rmcp-features.sh)
@row_add n=441 row=decision/viola-channel-tokio-free area=ci-integration rec=fct-pk32flwi@2,fct-3gkvok74@1
name: viola-channel normal graph is tokio-free
rule: viola-channel's normal dependency graph holds no tokio; its Tokio client sits behind its tokio feature.
guard: channel-tokio-free.sh and deny probe channel-tokio-edge
@row_add n=442 row=decision/ci-single-ubuntu-leg area=ci-integration rec=fct-pk32flwi@2,fct-ijfdjbph@2,fct-sd3lvsrr@2,fct-bfdvmti3@2
name: CI runs on one ubuntu-latest leg
rule: CI jobs run on a single ubuntu-latest leg.
@row_add n=443 row=decision/cargo-deny-supply-chain-gate area=ci-integration rec=fct-3gkvok74@1
name: cargo deny supply-chain gate
rule: The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace.
guard: CI job supply-chain
@row_add n=444 row=decision/serde-json-unbounded-depth-banned area=ci-integration rec=fct-3gkvok74@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned from the dependency graph.
guard: deny probe serde_json-unbounded-depth
@row_add n=445 row=decision/tokio-banned-per-sync-crate area=ci-integration rec=fct-3gkvok74@1
name: tokio banned from each sync crate
rule: tokio is banned from each crate on the sync-crates list, checked with that crate as sole root.
guard: step Tokio ban per sync crate (sole root)
@row_add n=446 row=decision/deny-bans-probed area=ci-integration rec=fct-3gkvok74@1
name: Every deny ban has a negative probe
rule: Every cargo deny ban has a permanent negative probe that must fire, beside a clean control.
guard: deny-probes.sh
@row_add n=447 row=decision/workflows-audited-by-zizmor area=ci-integration rec=fct-3gkvok74@1
name: Workflows audited by zizmor
rule: The GitHub workflows are audited by zizmor in the supply-chain job.
guard: CI job supply-chain
@row_add n=448 row=decision/weekly-advisory-scan area=ci-integration rec=fct-3gkvok74@1
name: Weekly advisory scan
rule: Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch.
guard: nightly job advisories
@row_none n=96 rec=fct-4otx4vxd@2 type=6
@row_none n=97 rec=fct-a43jfmhh@1 type=6
@row_add n=449 row=decision/one-instrumented-ci-test-run area=ci-integration rec=fct-sd3lvsrr@2
name: One instrumented test run in CI
rule: In CI one instrumented coverage run under nextest replaces the unit, integration and e2e runs, followed by the doctest run; the separate runs are the local equivalents.
guard: CI job test
@row_add n=450 row=decision/ci-jobs-end-in-harness-gate area=ci-integration rec=fct-sd3lvsrr@2
name: CI test jobs end in the harness gate
rule: Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
@row_add n=451 row=decision/mutation-testing-off-ci area=ci-integration rec=fct-ixlw7x3t@2
name: Mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
@row_add n=452 row=decision/perf-verdict-from-harness-gate area=ci-integration rec=fct-bfdvmti3@2
name: Perf verdict from the harness gate
rule: The perf verdict comes from viola-harness gate, never from a jq binary; CI neither installs nor pins jq.
guard: CI job perf, gate --require perf
@row_add n=453 row=decision/hyperfine-measures-perf area=ci-integration rec=fct-bfdvmti3@2
name: hyperfine for perf measurement
rule: hyperfine is what measures perf, exporting the JSON the gate reads.
@row_add n=454 row=decision/msrv-1-96 area=ci-integration rec=fct-whs3hmpm@1
name: Minimum supported Rust 1.96
rule: The workspace is checked and unit-tested on Rust 1.96, its minimum supported version.
why: 1.96 is the security toolchain floor and the requested workspace rust-version; sysinfo needs 1.95.
guard: CI job msrv, gate --require nextest-unit
@row_add n=455 row=decision/fuzz-corpus-replay-on-push area=ci-integration rec=fct-j6sjgyk3@1
name: Fuzz corpus replay on every push
rule: Every push and pull request replays each fuzz target's corpus without fuzzing.
guard: CI job fuzz-replay, gate --require fuzz-replay
@row_add n=456 row=decision/nightly-fuzzing area=ci-integration rec=fct-j6sjgyk3@1
name: Nightly fuzzing
rule: Each fuzz target is fuzzed for 120 seconds nightly, its crash artifacts uploaded on failure.
guard: nightly job fuzz
@row_add n=457 row=decision/no-cache-in-release-workflow area=ci-integration rec=fct-2qyicniz@2
name: No build cache in a release workflow
rule: The Rust build cache is used in ci.yml only, never in a release workflow.
@row_add n=458 row=decision/harness-gate-fails-on-breach area=ci-integration rec=fct-k7t43itf@2
name: Harness gate fails on any quality breach
rule: viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
@row_add n=459 row=decision/least-workflow-permissions area=ci-integration rec=fct-k7t43itf@2
name: Least workflow permissions
rule: Every workflow has empty permissions at the top and contents read per job.
@row_add n=460 row=decision/actions-sha-pinned area=ci-integration rec=fct-k7t43itf@2
name: Actions pinned by SHA
rule: Every action a workflow uses is pinned by SHA.
guard: zizmor unpinned-uses
@row_add n=461 row=decision/one-version-source-per-tool area=ci-integration rec=fct-k7t43itf@2
name: One version source per tool
rule: Each CI tool has exactly one pinned version source; cargo-deny and zizmor run as pinned CLIs, never through their actions.
@row_add n=462 row=decision/jq-runner-provided area=ci-integration rec=fct-k7t43itf@2
name: jq taken from the runner
rule: jq is taken from the CI runner, never installed; the scripts that need it refuse with tool-missing: jq when it is absent.
@row_add n=463 row=decision/ripgrep-pinned-checksum-verified area=ci-integration rec=fct-k7t43itf@2
name: ripgrep pinned and checksum-verified
rule: ripgrep is installed from a pinned, checksum-verified official release.
@row_add n=464 row=decision/local-pre-push-gate-installs-nothing area=ci-integration rec=fct-k7t43itf@2
name: Local pre-push gate installs nothing
rule: The local pre-push gate installs nothing; it checks the host's tools against the pins parsed from ci.yml and refuses a missing or off-pin tool.
why: ci.yml stays the one version source.
@row_add n=465 row=decision/no-workflow-concurrency area=ci-integration rec=fct-k7t43itf@2
name: No workflow concurrency block
rule: No workflow has a concurrency block; zizmor's concurrency-limits finding is declined and stays visible.
why: A concurrency group cancels pending runs even without cancel-in-progress, which would drop a push's always() gate and upload chain.
@row_add n=466 row=decision/ci-stamped-home-kill-line area=ci-integration rec=fct-7d3yogoz@2
name: 20 s kill line for stamped-home tests
rule: On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
@row_add n=467 row=decision/no-ignored-advisories area=ci-integration rec=fct-zwsm4wxk@2
name: No advisory ignored
rule: cargo deny ignores no advisory.
guard: CI job supply-chain
@row_add n=468 row=decision/cli-e2e-via-process area=e2e-test-strategy rec=fct-4m5t4osf@5
name: CLI tested as a process
rule: CLI verbs are tested end to end by running the viola binary as a child process.
@row_add n=469 row=decision/cli-fixed-strings-pinned-by-literal area=e2e-test-strategy rec=fct-4m5t4osf@5
name: CLI fixed strings pinned by literals
rule: Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
@row_add n=470 row=decision/tty-styling-asserted-on-sgr area=e2e-test-strategy rec=fct-4m5t4osf@5
name: Terminal styling asserted on SGR sequences
rule: Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
@row_add n=471 row=contract/cli-json-failure/document area=e2e-test-strategy rec=fct-4m5t4osf@5
boundary: CLI --json failures
item: failure document
promise: Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
@row_add n=472 row=decision/hook-e2e-recorded-stdin area=e2e-test-strategy rec=fct-ennvbmje@2
name: Hook verbs fed recorded stdin
rule: The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
@row_add n=473 row=decision/tui-outer-pty-driver area=e2e-test-strategy rec=fct-tr6c4sml@3
name: Passthrough tested through an outer PTY
rule: The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
@row_add n=474 row=decision/outer-pty-reader-drained area=e2e-test-strategy rec=fct-tr6c4sml@3
name: Outer PTY reader drained on its own thread
rule: An outer-PTY test drains the master reader on its own thread.
@row_add n=475 row=decision/tui-verdicts-from-records area=e2e-test-strategy rec=fct-tr6c4sml@3
name: TUI verdicts from records
rule: TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status.
except: The forced vt100 panic case byte-searches what the drained master reader read.
@row_add n=476 row=decision/channel-e2e-driver area=e2e-test-strategy rec=fct-6q7xvkh4@1
name: Channel tested with the sync client
rule: The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
@row_add n=477 row=decision/channel-endpoint-from-snapshot area=e2e-test-strategy rec=fct-6q7xvkh4@1
name: Channel endpoint read from the snapshot
rule: Channel tests read the wrapper endpoint only from the instance snapshot.
@row_add n=478 row=decision/mcp-tests-own-server-child area=e2e-test-strategy rec=fct-thp7vui4@1
name: MCP tests own their server child
rule: MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport.
why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
@row_add n=479 row=decision/viola-e2e-rmcp-client-only area=e2e-test-strategy rec=fct-thp7vui4@1
name: viola-e2e takes rmcp client only
rule: The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
@row_add n=480 row=decision/library-surface-at-unit-tier area=e2e-test-strategy rec=fct-okzgoz3x@1
name: Library-only surface covered at unit tier
rule: Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
@row_none n=98 rec=fct-l7jcwkqi@1 type=5
@row_add n=481 row=contract/viola-run-start/event-order area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: first three events
promise: The first three events.ndjson records are wheel{cause:"start"}, budget-gate, then session-start{source:"hook"} with data.cause "startup", in that order.
@row_add n=482 row=contract/viola-run-start/child-after-events area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: child spawn
promise: The child agent is spawned only after the start events are written.
@row_add n=483 row=contract/instance-snapshot/pinned-bin area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: pinned_bin
promise: The instance snapshot holds pinned_bin.
@row_add n=484 row=contract/instance-snapshot/started-at area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: started_at
promise: The instance snapshot holds started_at.
@row_add n=485 row=contract/instance-snapshot/child-pid area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: child_pid
promise: The instance snapshot holds child_pid.
@row_add n=486 row=contract/instance-snapshot/endpoint area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: endpoint
promise: The instance snapshot holds the wrapper channel endpoint.
@row_add n=487 row=contract/viola-list-json/live-instance area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola list --json
item: running wrapped instance
promise: A running wrapped instance's item reads wrapped:true and liveness "live".
@row_add n=488 row=contract/viola-run-start/duplicate-run area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: duplicate run
promise: A second run of an instance already running exits 1.
@row_add n=489 row=contract/viola-run-start/squatted-endpoint area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: squatted endpoint
promise: A run whose channel endpoint is already held by another listener exits 1.
@row_add n=490 row=contract/viola-run-start/tampered-pinned-exe area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: tampered pinned exe
promise: A run whose pinned executable was altered exits 1.
@row_add n=491 row=contract/viola-run-start/plugin-hooks area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: plugin hooks.json
promise: Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.
@row_add n=492 row=contract/viola-run-start/settings-override area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: settings.json
promise: Each start rewrites settings.json whole at 0600 as a statusLine command of the absolute pinned path plus hook statusline, only when every path character is an ASCII letter, a digit or one of _ - . / :.
@row_add n=493 row=contract/viola-run-start/home-tree-check area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: home-tree strict-modes check
promise: The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
@row_add n=494 row=contract/viola-run-start/socket-dir-check area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: socket directory check
promise: The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
@row_none n=99 rec=fct-qdldsdqk@1 type=8
@row_none n=100 rec=fct-4n3xwm4s@1 type=5
@row_add n=495 row=contract/viola-send/turn-running area=e2e-test-strategy rec=fct-n6muav3b@2,fct-imixgk6n@1
boundary: viola send
item: turn-running refusal
promise: While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
@row_add n=496 row=contract/mcp-tools/result-body area=e2e-test-strategy rec=fct-n6muav3b@2,fct-qbytqn26@3,fct-mdqnbyad@2,fct-p2i2onck@2,fct-3r2a46vd@2
boundary: MCP tools
item: result and refusal body
promise: Each MCP tool's result and refusal equal the CLI's --json document less v.
@row_add n=497 row=contract/viola-send/ok-document area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: ok document
promise: A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
@row_add n=498 row=contract/viola-send/records area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: send records
promise: The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the typed text.
@row_add n=499 row=contract/viola-send/paste-framing area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: typed framing
promise: A sent text is typed as one bracketed paste followed by one CR, its newlines kept inside.
@row_add n=500 row=contract/viola-send/trailing-newlines area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: trailing newlines
promise: A text ending in LF, CR, CRLF or two CRs is typed without that ending and confirmed.
@row_add n=501 row=contract/viola-send/inner-cr area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: inner CR
promise: A CR or CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel.
@row_add n=502 row=contract/viola-send/no-prompt-submitted area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: unconfirmed send
promise: A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
@row_add n=503 row=contract/viola-send/unconfirmable-local-command area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: local command with no measured post-condition
promise: Exits 0 with ok {confirmed:false, detail:"unconfirmable", cursor}, logs send-issued then send-confirmed{confirmed:false}, no send-refused, no confirmation window.
@row_add n=504 row=contract/viola-send/compact-confirmed area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: /compact on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.
@row_add n=505 row=contract/viola-send/clear-confirmed area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: /clear on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause clear and a new agent_session_id, ending exit 0 {submitted_at, cursor}.
@row_add n=506 row=contract/viola-send/off-list-slash-text area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: slash text off the compiled list
promise: Exits 13 not-delivered/no-prompt-submitted.
@row_add n=507 row=contract/viola-send/human-key-during-gate area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: human key during the gate wait
promise: The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
@row_add n=508 row=contract/viola-send-human-mode/read-back area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: confirmed line
promise: A confirmed send prints the stdout line [RB] read back … cursor <n>.
@row_add n=509 row=contract/viola-send-human-mode/unable area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: refusal lines
promise: A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
@row_add n=510 row=contract/viola-send-human-mode/unconfirmable area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: unconfirmable line
promise: A local command with no measured post-condition prints one stdout line [  ] unconfirmable <name> local command, no measured post-condition, exit 0, nothing on stderr.
@row_none n=101 rec=fct-kh5kjsnv@1 type=5
@row_add n=511 row=contract/turn-ended/background-tasks area=e2e-test-strategy rec=fct-mdqnbyad@2
boundary: turn-ended event data
item: background_tasks
promise: The Stop array's length, or null.
@row_add n=512 row=contract/mcp-wait/timeout-ms area=e2e-test-strategy rec=fct-mdqnbyad@2
boundary: MCP wait
item: timeout_ms
promise: The bound sent is clamped to 50 000 ms, which is also its default.
@row_none n=102 rec=fct-lmiov3q7@1 type=5
@row_add n=513 row=contract/viola-wait/non-waking-records area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: activity and wheel records
promise: Activity and wheel records never wake a parked wait.
@row_add n=514 row=contract/viola-wait/turn-ended area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: turn-ended wake
promise: Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
@row_add n=515 row=contract/viola-wait/already-past area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: event already past the cursor
promise: A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
@row_add n=516 row=contract/viola-wait/timeout area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: timeout
promise: A wait that times out gives {timed_out:true} with exit 0.
@row_add n=517 row=contract/viola-last/document area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola last
item: last document
promise: Gives {last_assistant_message, ts} of the last turn.
@row_add n=518 row=contract/cli-exit-codes/21 area=e2e-test-strategy rec=fct-nb7ralop@1,fct-mdqnbyad@2
boundary: CLI exit codes
item: 21
promise: Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
@row_add n=519 row=contract/mcp-tools/instance-unreachable area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: MCP tools
item: instance-unreachable
promise: is_error:true with the CLI document less v, text refused: instance-unreachable.
@row_none n=103 rec=fct-sefkfuph@1 type=5
@row_add n=520 row=contract/viola-run/child-on-wrapper-kill area=e2e-test-strategy rec=fct-lhsyucqg@2
boundary: viola run wrapper
item: wrapper killed
promise: When the wrapper is killed, its child agent is gone within a bounded time.
@row_add n=521 row=contract/question-event/option-details area=e2e-test-strategy rec=fct-p2i2onck@2
boundary: question event
item: option_details
promise: One {description, preview} per option, each a string or null.
@row_add n=522 row=contract/viola-answer/allow-suggestion-fields area=e2e-test-strategy rec=fct-p2i2onck@2
boundary: viola answer
item: permission allow
promise: An allow's suggestion fields never reach the agent CLI.
@row_none n=104 rec=fct-lqbzryiq@1 type=5
@row_add n=523 row=contract/dialog-events/logged-once area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog events
item: question, permission, plan
promise: Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
@row_add n=524 row=contract/hook-process/exit area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook process
item: exit and stderr
promise: The hook process exits 0 with empty stderr.
@row_add n=525 row=contract/hook-decision/question area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: question answer
promise: PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included.
@row_add n=526 row=contract/hook-decision/permission area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: permission answer
promise: PermissionRequest decision.behavior "allow", or "deny" with message.
@row_add n=527 row=contract/hook-decision/plan-approve area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: plan approve
promise: PreToolUse allow with updatedInput as the tool's own input unchanged, and no PermissionRequest body.
@row_add n=528 row=contract/hook-decision/plan-revise area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: plan revise
promise: PermissionRequest deny with message.
@row_add n=529 row=contract/hook-decision/concurrent-dialog area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: second concurrent dialog
promise: A second dialog raised while one is pending gets an empty hook stdout.
@row_add n=530 row=contract/viola-answer/human-wheel area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: viola answer
item: wheel held by human
promise: answer exits 10 and the hook stdout stays empty.
@row_add n=531 row=decision/no-decision-body-without-driver-and-stamp area=e2e-test-strategy rec=fct-2zxurlqa@1
name: Decision bodies only for a driver on a stamped CLI
rule: No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
@row_none n=105 rec=fct-oecictpm@1 type=5
@row_add n=532 row=decision/terminal-replies-not-editing area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Terminal replies are not editing
rule: Terminal reply shapes on a closed list are read as not editing and never take the wheel; any other stdin input is typing.
guard: classifier_terminal_reply_is_not_editing, classifier_typing_is_editing
@row_add n=533 row=decision/focus-mouse-resize-keep-wheel area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Focus, mouse and resize never take the wheel
rule: Focus, mouse and resize input never takes the wheel from the driver.
guard: tui_focus_mouse_and_resize_never_take_the_wheel
@row_add n=534 row=contract/viola-run-passthrough/ctrl-z area=e2e-test-strategy rec=fct-3r2a46vd@2
boundary: viola run passthrough
item: Ctrl-Z
promise: Ctrl-Z reaches the child, and later keys still do.
@row_add n=535 row=decision/terminal-reply-reading-live-measured area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Terminal reply reading measured live
rule: The classifier's reading of terminal replies on a real terminal is a local live measurement, not a suite.
why: The outer-PTY tier writes no terminal reply of its own.
@row_none n=106 rec=fct-v76i6l5l@1 type=5
@row_add n=536 row=contract/viola-send/human-typing area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola send
item: human-typing refusal
promise: While the human holds the wheel, send exits 10 human-typing with detail null.
@row_add n=537 row=contract/mcp-send/human-typing area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: MCP send
item: human-typing refusal
promise: is_error:true with text refused: human-typing.
@row_add n=538 row=contract/viola-release/document area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: release document
promise: Gives {wheel:"driver", budget_paused:false}, after which a send exits 0.
@row_add n=539 row=contract/viola-send/manual-pause area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola send
item: manual-pause refusal
promise: After pause, send exits 10 with detail "manual-pause".
@row_add n=540 row=decision/hints-never-suggest-release area=e2e-test-strategy rec=fct-imixgk6n@1,fct-3r2a46vd@2
name: Driver hints never suggest release
rule: No driver-facing hint, MCP refusal text, structured content, result or tool description names release.
why: Drivers are refused release, so such a hint sends the driver into a dead end.
@row_add n=541 row=contract/viola-release/from-driver area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: release from a driver
promise: Exits 20 {"error":"wrapper-fault","detail":{"code":-32602,…}} carrying release-from-driver.
@row_add n=542 row=contract/viola-release/running-turn area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: turn left running
promise: Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
@row_none n=107 rec=fct-pyb4ohg2@1 type=5
@row_add n=543 row=contract/statusline-settings-source/source-file area=e2e-test-strategy rec=fct-bo4nzuwn@1
boundary: statusline settings source
item: source-file
promise: viola reads the user statusline command from <home>/statusline-source.json (the Claude settings shape) when it exists, else in the default home from the user's .claude/settings.json, and in any other home from none; viola only reads it.
@row_add n=544 row=contract/statusline-payload/resets-at area=e2e-test-strategy rec=fct-bo4nzuwn@1,fct-kn7ryhs7@2
boundary: statusline payload
item: resets_at
promise: The statusline reader takes resets_at as epoch seconds or RFC 3339; a malformed value is recorded as "unknown" in budget.json.
@row_add n=545 row=decision/override-expiry-tested-at-unit-level area=e2e-test-strategy rec=fct-bo4nzuwn@1
name: Budget override expiry tested at unit level
rule: Budget override expiry is asserted only in unit cases of the pure gate decision, which takes the instant as an argument, never end to end.
why: No in-process clock reaches the separately spawned wrapper, and waiting in real time would break the sleep(N) ban.
guard: gate_decide_an_expired_override_is_dropped, gate_release_override_ends_at_its_instant
@row_add n=546 row=contract/budget-paused-refusal/send-exit-11 area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget-paused refusal
item: send exit 11
promise: A send while the budget is paused exits 11 with detail naming the window, "five-hour" or "seven-day".
@row_add n=547 row=contract/statusline-hook/stdout area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: viola hook statusline
item: stdout
promise: The wrapped statusline prints the user's command output byte for byte, with exit 0 and empty stderr.
@row_add n=548 row=decision/statusline-command-shell-and-bound area=e2e-test-strategy rec=fct-kn7ryhs7@2
name: Statusline command run through sh under a bound
rule: The wrapped statusline arm runs the recorded user command as /bin/sh -c <command> under a 5 s bound.
state: provisional
@row_add n=549 row=contract/budget-json/write-condition area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget.json
item: write condition
promise: budget.json is written only when the statusline payload holds a rate_limits object.
@row_add n=550 row=contract/budget-gate-record/paused-true area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget-gate record
item: paused true
promise: A budget pause logs a budget-gate{paused:true, window} record.
@row_add n=551 row=contract/budget-gate-record/paused-false area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
boundary: budget-gate record
item: paused false
promise: A per-instance release logs one budget-gate{paused:false, override_until} record for that instance alone, its override_until later than its ts and equal to the instance snapshot's budget_override_until.
@row_add n=552 row=contract/release-budget-json/document area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: release --budget --json document
item: document
promise: release --budget gives {budget_paused:false, override_until} when a pause stood and {budget_paused:false} alone when none did; the next send exits 0.
@row_add n=553 row=decision/statusline-command-never-runs-unsafe area=e2e-test-strategy rec=fct-kn7ryhs7@2
name: Statusline command never runs over unsafe state
rule: The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing.
guard: statusline_over_a_replayed_snapshot_runs_nothing
@row_add n=554 row=contract/list-json/budget-paused area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: list --json
item: budget_paused
promise: The top-level .ok.budget.paused reports the budget reading's pause, while each item's budget_paused reports that instance's own gate, false under a per-instance override.
@row_add n=555 row=decision/budget-override-per-instance area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Budget override is per instance
rule: release --budget lifts the pause for the named instance alone; other instances under the same reading stay paused and are refused.
guard: path6_two_instance_release_over_a_harness_session, gate_release_on_one_instance_leaves_another_paused
@row_add n=556 row=contract/release-verb/driver-caller area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: release verb
item: driver caller
promise: A driver calling release is refused with -32602, exit 20.
@row_add n=557 row=decision/unknown-reset-does-not-pause area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Unknown or passed reset does not pause
rule: A budget window whose resets_at is "unknown", unparseable or already passed does not pause.
guard: path6_an_unknown_or_passed_reset_does_not_pause
@row_add n=558 row=decision/release-without-pause-records-nothing area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Release with no pause records nothing
rule: release --budget with no pause standing records nothing.
guard: path6_release_budget_lifts_the_pause
@row_add n=559 row=decision/budget-override-ends-with-wrapper area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Budget override ends with its wrapper
rule: A budget override does not outlive its wrapper, so a restart over the same reading pauses again.
guard: path6_a_restart_pauses_again
@row_add n=560 row=decision/budget-gate-covers-send-only area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Budget gate covers send only
rule: The budget gate refuses automated send alone; answer, wait, last and pause behave under a closed gate as under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=561 row=contract/mcp-tool-result/budget-paused area=e2e-test-strategy rec=fct-kylcwwrw@2
boundary: MCP tool result
item: budget-paused
promise: A budget-paused send is isError:true with {"refusal":"budget-paused","detail":"five-hour"} and no cursor, equal to the CLI's document less v.
@row_add n=562 row=contract/list-human/wheel-budget-paused area=e2e-test-strategy rec=fct-kylcwwrw@2
boundary: viola list human board
item: WHEEL cell
promise: A budget-paused instance's WHEEL cell reads "driver - budget-paused" on that instance's row alone, with no ESC byte.
@row_none n=108 rec=fct-nbvt5ht3@1 type=5
@row_none n=109 rec=fct-lwjugv35@1 type=5
@row_add n=563 row=decision/unverified-cli-gates-answer-only area=e2e-test-strategy rec=fct-5mlkcfqk@1
name: Unverified CLI version gates answer only
rule: Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
@row_add n=564 row=contract/cli-answer/unverified-cli area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: viola answer
item: unverified-cli
promise: answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
@row_add n=565 row=contract/mcp-tool-result/unverified-cli area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: MCP tool result
item: unverified-cli
promise: MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
@row_add n=566 row=contract/verify-output/last-line area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: viola verify output
item: last line
promise: A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
@row_none n=110 rec=fct-5eczp6gf@2 type=5
@row_none n=111 rec=fct-2rmlc6ik@1 type=5
@row_add n=567 row=decision/fd-test-reads-live-child-table area=e2e-test-strategy rec=fct-blbpbqaj@1
name: fd test reads the live child's fd table
rule: The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump.
why: The fake agent's own receipt and /dev/fd handle show as extra fds in its self-dump.
@row_add n=568 row=decision/claude-child-env-strip area=e2e-test-strategy rec=fct-ja62ftpj@3,fct-blbpbqaj@1
name: claude child environment strip
rule: The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive.
guard: tui_env_strip
@row_add n=569 row=contract/process-start-claude-child/env-stripped-count area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_stripped_count
promise: Carries the count of environment names stripped from the claude child.
@row_add n=570 row=contract/process-start-claude-child/env-stripped-known area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_stripped_known
promise: Carries the known stripped environment names, names only, never values.
@row_add n=571 row=contract/process-start-claude-child/env-kept area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_kept
promise: Carries the environment names kept for the claude child, names only, never values.
@row_add n=572 row=decision/claude-child-holds-nothing-of-viola area=e2e-test-strategy rec=fct-ja62ftpj@3
name: claude child holds nothing of viola's
rule: The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY.
guard: tui_channel_fds
@row_add n=573 row=decision/no-fd-count-assertion area=e2e-test-strategy rec=fct-ja62ftpj@3
name: No fd-count assertion on the child
rule: The child fd test asserts what the child holds, never a count of its descriptors.
why: portable-pty closes every inherited descriptor above 2 at spawn, so a planted leak cannot turn a count red, and the fake agent's own files add fds.
@row_add n=574 row=decision/secrets-never-in-logs area=e2e-test-strategy rec=fct-ja62ftpj@3,fct-iplgfuqy@2
name: Secrets never in logs or diagnostics
rule: No stripped-variable value and no Cookie header appears in any log or diagnostics file.
@row_none n=112 rec=fct-tmgp6fr5@1 type=5
@row_add n=575 row=decision/link-driver-self-reported area=e2e-test-strategy rec=fct-53es7vlo@2
name: Link driver is a self-reported name
rule: The driver of a link is a self-reported name and need not be a booted instance.
@row_add n=576 row=decision/send-from-another-instance-links-once area=e2e-test-strategy rec=fct-53es7vlo@2
name: Send from another instance links once
rule: An issued send from another instance appends one link{driver, driven} record with source wrapper before its send-issued, and a later send of the same pair adds none.
guard: links_a_send_from_another_instance_links_once
@row_add n=577 row=decision/refused-or-own-send-records-no-link area=e2e-test-strategy rec=fct-53es7vlo@2
name: Refused or own-name send records no link
rule: A refused send and a send to an instance's own name record no link.
guard: links_a_refused_send_and_an_own_name_send_record_none
@row_add n=578 row=contract/channel-link/already-linked area=e2e-test-strategy rec=fct-53es7vlo@2
boundary: wrapper link method
item: already_linked
promise: A second link of an already linked pair appends no record, keeps the first since, and answers already_linked: true.
@row_add n=579 row=contract/channel-unlink/unlink-record area=e2e-test-strategy rec=fct-53es7vlo@2
boundary: wrapper unlink method
item: unlink record
promise: unlink appends one unlink record (source: wrapper, {driver, driven}) and empties the snapshot's links; a second unlink appends nothing, and a later link is a new pair with a later since.
@row_add n=580 row=decision/links-survive-wrapper-restart area=e2e-test-strategy rec=fct-53es7vlo@2,fct-3svwp7h4@1
name: Links survive a wrapper restart
rule: A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line.
guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
@row_add n=581 row=decision/replay-writes-no-file area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Replay writes no file
rule: The snapshot replay writes no file, so no snapshot is rebuilt from it.
@row_add n=582 row=decision/replayed-snapshot-has-no-endpoint area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Replayed snapshot has no endpoint or cwd
rule: A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing.
guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
@row_add n=583 row=decision/collision-check-over-replay-reads-beat area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Collision check over a replay reads the beat
rule: Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free.
guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
@row_add n=584 row=decision/list-reads-live-through-replay area=e2e-test-strategy rec=fct-3svwp7h4@1
name: list reads a live wrapper through the replay
rule: list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line.
guard: e5_list_reads_a_live_wrapper_through_the_replay
@row_add n=585 row=contract/event-log/start-wheel-record area=e2e-test-strategy rec=fct-3svwp7h4@1
boundary: instance event log
item: first record of a start
promise: Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record.
@row_add n=586 row=decision/security-sweep-under-nextest area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Security sweep runs under nextest alone
rule: The security sweep runs under nextest alone.
@row_add n=587 row=decision/hook-instance-check-before-config area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Hook instance check before any config read
rule: The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
except: The two --capture arms stay ahead of the check.
@row_add n=588 row=decision/fake-agent-panic-seam-test-only area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Forced-panic seam is test-only
rule: The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it.
why: Ratified in the security-plan Decisions Log.
guard: cli_controls_not_disableable
@row_add n=589 row=decision/hook-panic-payload-to-detail-file area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Hook panic payload kept to the detail file
rule: A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only.
guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
@row_add n=590 row=contract/caught-panic-exit-code/hook area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: exit code on a caught panic
item: hook
promise: hook exits 0 on a caught panic.
@row_add n=591 row=contract/caught-panic-exit-code/run area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: exit code on a caught panic
item: run
promise: run exits 1 on a caught panic.
@row_add n=592 row=contract/mcp-tool-input/schema-violation area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: MCP tool input
item: schema violation
promise: A schema-violating tool call is invalid-params naming its field, revalidated before any frame, with no send-issued.
@row_add n=593 row=contract/mcp-tool-input/empty-or-control-text area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: MCP tool input
item: empty or control-character text
promise: A send text that is empty or holds a control character is not-delivered.
@row_add n=594 row=contract/hook-decision/server-verify-failed area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: hook-decision record
item: server-verify-failed
promise: A hook facing a server that is not the recorded wrapper logs one hook-decision line with detail "server-verify-failed", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_add n=595 row=contract/hook-decision/socket-dir-refused area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: hook-decision record
item: socket-dir-refused
promise: A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_add n=596 row=decision/error-bodies-held-by-surface-witnesses area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Error bodies held by surface witnesses
rule: No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
@row_add n=597 row=contract/cli-json/internal-error area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: CLI --json document
item: internal-error
promise: Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
@row_add n=598 row=contract/cli-human/internal-error area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: CLI human output
item: internal error line
promise: Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
@row_add n=599 row=contract/channel-error/-32603 area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: channel error
item: -32603
promise: Every -32603 is exactly "internal error" with data null.
@row_add n=600 row=decision/internal-error-held-at-unit-layer area=e2e-test-strategy rec=fct-iplgfuqy@2
name: -32603 held at the unit layer
rule: No -32603 is driven in a scanned home and no failpoint or counted panic stands for one; the unit layer holds it.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_add n=601 row=decision/diagnostics-files-0600 area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Diagnostics files are owner-only
rule: diagnostics/ files are mode 0600.
@row_add n=602 row=decision/synthetic-inputs-with-canary area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Synthetic test inputs carry a canary
rule: Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
@row_add n=603 row=decision/scan-hits-never-print-bytes area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Scan hits never print matched bytes
rule: A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes.
guard: error_body_check_reports_no_byte_of_what_it_read
@row_add n=604 row=decision/ci-scan-before-upload area=e2e-test-strategy rec=fct-iplgfuqy@2
name: CI secret scan before any upload
rule: The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload.
except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
@row_add n=605 row=decision/each-exit-cause-own-hint area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: Each exit cause has its own hint
rule: Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none.
guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
@row_add n=606 row=contract/exit-21-detail/strict-modes-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: strict-modes-failed
promise: The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
@row_add n=607 row=contract/exit-21-detail/instance-dead area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: instance-dead
promise: No instance is running for the name with its directory standing, a stale wrapper included; during connect.
@row_add n=608 row=contract/exit-21-detail/unwrapped-name area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: unwrapped-name
promise: The name holds no instance directory (an unwrapped session or a name never started); its own hint; during connect.
@row_add n=609 row=contract/exit-21-detail/endpoint-lost area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open; its own hint; during call.
@row_add n=610 row=contract/exit-21-detail/server-verify-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: server-verify-failed
promise: The listening server is not the recorded wrapper (pid or start time mismatch, or another euid); its own hint; logged as process-exit at WARN; during connect.
@row_add n=611 row=contract/exit-21-detail/socket-dir-refused area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: socket-dir-refused
promise: The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
@row_add n=612 row=contract/cli-json/instance-unreachable area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: CLI --json document
item: instance-unreachable
promise: Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
@row_add n=613 row=contract/start-exit-1/already-live area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: already-live
promise: The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
@row_add n=614 row=contract/start-exit-1/stale-heartbeat area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: stale-heartbeat
promise: The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
@row_add n=615 row=contract/start-exit-1/squatted-name area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: squatted-name
promise: The endpoint name is squatted.
@row_add n=616 row=contract/start-exit-1/pinned-hash-mismatch area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: pinned-hash-mismatch
promise: The pinned executable fails its SHA-256 re-hash.
@row_add n=617 row=contract/start-exit-1/strict-modes-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: strict-modes-failed
promise: The home or instance state could be written, or read, by another user; no log opened, no line, no child.
@row_add n=618 row=contract/start-exit-1/socket-dir-refused area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: socket-dir-refused
promise: The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
@row_add n=619 row=contract/revive-exit-1/no-session area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: no-session
promise: No logged session for the name, or an --id the log does not hold.
@row_add n=620 row=contract/revive-exit-1/session-live area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: session-live
promise: One claude agents --json read lists the logged session live in another claude; no child starts.
@row_add n=621 row=contract/revive-exit-1/cwd-missing area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: cwd-missing
promise: The recorded directory is gone, or the snapshot holds no cwd.
@row_add n=622 row=decision/revive-preflight-order area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: revive preflight order
rule: revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
@row_add n=623 row=decision/session-live-read-fails-open area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: session-live reading fails open
rule: revive --fork skips the session-live reading, and a failed claude agents read passes it.
guard: revive_passes_the_session_live_reading, revive_whose_agents_read_fails_starts
@row_add n=624 row=decision/revive-json-no-document-after-terminal area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: No revive document once the child holds the terminal
rule: A pump_child error under revive --json prints no document.
why: The child has held the terminal.
@row_add n=625 row=decision/strict-modes-before-any-log area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-hhwc4gim@2
name: Strict-modes check before any read or open
rule: Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found.
why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700.
guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
@row_add n=626 row=decision/one-negative-per-security-control area=e2e-test-strategy rec=fct-hhwc4gim@2
name: One negative test per security control
rule: Every security-plan control has a negative test of its own.
@row_add n=627 row=decision/socket-dir-check-never-chmods area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Socket directory refused, never repaired
rule: The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it.
guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
@row_add n=628 row=contract/mcp-frame/socket-dir-refused area=e2e-test-strategy rec=fct-hhwc4gim@2
boundary: MCP frame
item: socket-dir-refused
promise: A refused socket directory gives instance-unreachable with no line at MCP's frame.
@row_add n=629 row=contract/channel-link-names/invalid-name area=e2e-test-strategy rec=fct-hhwc4gim@2
boundary: channel link and unlink methods
item: invalid name
promise: A driver or driven that is not a valid name, a driven other than the serving wrapper's own name, or a driver equal to driven gets -32602 with data null and appends no record.
@row_add n=630 row=decision/home-readable-by-other-refused area=e2e-test-strategy rec=fct-hhwc4gim@2,fct-jaqoqtdh@1
name: Home another user may read is refused
rule: A pre-existing home another user may read is refused at run with exit 1.
guard: a_home_another_user_may_read_refuses_run
@row_add n=631 row=decision/single-writers area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json.
guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
@row_add n=632 row=decision/client-verifies-server-identity area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Client verifies the server before any frame
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output.
guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_add n=633 row=decision/list-rows-escape-control-chars area=e2e-test-strategy rec=fct-hdfefjry@2
name: list rows escape control characters
rule: viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode.
guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
@row_add n=634 row=contract/list-human/columns area=e2e-test-strategy rec=fct-hdfefjry@2
boundary: viola list human board
item: columns
promise: The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
@row_add n=635 row=contract/list-human/unwrapped-separator area=e2e-test-strategy rec=fct-hdfefjry@2
boundary: viola list human board
item: unwrapped separator
promise: Unwrapped sessions follow the separator line "-- UNWRAPPED - READ-ONLY --".
@row_add n=636 row=decision/stale-row-dim area=e2e-test-strategy rec=fct-3fk5wzsy@2
name: Stale row shown dim
rule: On a terminal a stale row is dim from first byte to last with its NAME dim and bold, shows stale, and no SGR touches the caption or separator.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_add n=637 row=decision/torn-append-heals area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Torn append heals on the next start
rule: A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged.
guard: chaos_torn_append
@row_add n=638 row=decision/torn-append-by-truncation area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Torn append simulated by truncation
rule: The torn-append chaos case stops the wrapper and shortens its log in place rather than killing it mid-append.
why: The file on disk is the same, and a killed instrumented process can leave a short coverage profile.
@row_add n=639 row=decision/exit-detected-on-handle area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Child exit detected on the handle
rule: The wrapper detects the child's exit on the process handle (exit_source "handle-wait"), not on output EOF.
@row_add n=640 row=contract/list-json/liveness-stale area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: list --json
item: liveness stale
promise: A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
@row_add n=641 row=decision/vt100-panic-keeps-passthrough area=e2e-test-strategy rec=fct-nhle4u5a@1,fct-gjbygtvj@1
name: vt100 panic keeps human passthrough
rule: After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=642 row=contract/cli-send/input-not-ready area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: viola send
item: input-not-ready
promise: send exits 13 with input-not-ready when the screen model is not ready.
@row_add n=643 row=decision/forward-compatible-log-reading area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Forward-compatible event log reading
rule: Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state.
guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
@row_add n=644 row=contract/list-json/skipped-counts area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: list --json
item: skipped
promise: skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
@row_none n=113 rec=fct-nwgyqyjs@2 type=5
@row_add n=645 row=decision/property-tests-with-proptest area=e2e-test-strategy rec=fct-gjbygtvj@1
name: proptest for property tests
rule: Input parsers and validators are property-tested with proptest at 512 cases, each with its regression file committed.
@row_add n=646 row=decision/fuzzing-with-cargo-fuzz area=e2e-test-strategy rec=fct-gjbygtvj@1
name: cargo-fuzz in a separate workspace
rule: Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
@row_add n=647 row=decision/fuzz-corpora-byte-exact area=e2e-test-strategy rec=fct-gjbygtvj@1
name: Fuzz corpora kept byte-exact
rule: Byte-sensitive fuzz corpora are marked binary in .gitattributes so their seeds stay byte-exact.
@row_add n=648 row=decision/vt100-fuzz-silent-panic-hook area=e2e-test-strategy rec=fct-gjbygtvj@1
name: vt100 fuzz target installs a silent panic hook
rule: The vt100_feed fuzz target installs a silent panic hook once before its first feed.
why: libfuzzer-sys's own hook aborts on every panic, a caught one included; an escaping panic still aborts through its catch.
@row_add n=649 row=decision/statusline-proptest-whole-seconds area=e2e-test-strategy rec=fct-gjbygtvj@1
name: Statusline property draws whole seconds
rule: The statusline reader's property strategy draws whole seconds across the date range.
why: As first written the property stayed green on a stub.
@row_add n=650 row=decision/agents-proptest-reparse-oracle area=e2e-test-strategy rec=fct-gjbygtvj@1
name: agents parser oracle re-parses its input
rule: The claude agents --json property oracle compares a row against the input re-parsed from its bytes.
why: A float does not round-trip through serde_json text.
@row_add n=651 row=decision/fixture-drift-byte-compare area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Fixture drift compared byte for byte
rule: The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails.
guard: contract_fake_agent_drift
@row_add n=652 row=decision/pinned-exe-hash-sha2 area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: sha2 for the pinned-exe content hash
rule: The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex.
guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
@row_add n=653 row=decision/contract-tables-are-literal area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Contract tables are literal
rule: Contract tests hold literal tables, never the product's own list.
@row_add n=654 row=decision/no-serde-json-unbounded-depth area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in deny.toml.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_add n=655 row=decision/error-writers-each-named area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Every product error writer is named
rule: Every product error writer is named in the error-body enumeration with a writer and a witness that stand.
guard: product_error_writers_are_each_named
@row_add n=656 row=decision/thiserror-messages-interpolate-nothing area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: thiserror messages interpolate nothing
rule: thiserror error messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=657 row=contract/channel-error/fault-bodies area=e2e-test-strategy rec=fct-sn6e6lxw@2
boundary: channel error
item: fault bodies
promise: The channel's faults are seven literal bodies.
@row_none n=114 rec=fct-bio6aaf2@1 type=5
@row_add n=658 row=decision/os-gated-tests-each-named area=e2e-test-strategy rec=fct-liulfbun@2
name: OS-gated tests each named with a reason
rule: Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named.
guard: os_gated_tests_are_each_named_with_their_reason
@row_none n=115 rec=fct-vgds3skp@2 type=5
@row_add n=659 row=decision/module-boundary-integration-tests area=integration-test-strategy rec=fct-jvtm6f2a@1
name: Module boundaries tested by direct calls
rule: The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
@row_add n=660 row=decision/on-disk-state-integration-tests area=integration-test-strategy rec=fct-hctsd5xl@2
name: On-disk state integration tests
rule: Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests.
why: The project has no database.
@row_add n=661 row=decision/concurrent-append-whole-lines area=integration-test-strategy rec=fct-hctsd5xl@2
name: Whole lines under concurrent append
rule: Separate processes appending at once to each shared diagnostics file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; a torn or interleaved line fails the test.
guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
@row_add n=662 row=decision/channel-tests-real-endpoint area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Channel tests on a real endpoint
rule: Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.
@row_add n=663 row=decision/squatter-via-server-api area=integration-test-strategy rec=fct-ouc4a2b7@2,fct-pwscsozw@2
name: Squatter listener through the server API
rule: A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
@row_add n=664 row=decision/owner-only-socket-modes area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Owner-only socket modes
rule: The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
@row_add n=665 row=decision/socket-dir-check area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Socket directory check before use
rule: The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods.
guard: security_negatives_socket_dir
@row_add n=666 row=decision/channel-client-no-dir-check area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Channel client carries no directory check
rule: The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check.
why: viola-channel has no viola-state edge.
guard: channel_link_names_are_validated_by_the_wrapper
@row_add n=667 row=decision/pty-tests-real-openpty area=integration-test-strategy rec=fct-vmzc7an2@3
name: PTY tests on a real openpty
rule: PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
@row_add n=668 row=decision/resize-baseline-is-spawn-size area=integration-test-strategy rec=fct-vmzc7an2@3
name: Pump resize baseline is the spawn size
rule: The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child.
why: A resize can land between the spawn sizing and the pump's first look.
guard: pump_forwards_a_resize_that_lands_before_its_first_look
@row_add n=669 row=decision/key-after-resize-waits-for-size area=integration-test-strategy rec=fct-vmzc7an2@3
name: Key after a resize waits for the new size
rule: A test that writes a key after a resize sends it only once the child reports the new size.
guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code
@row_add n=670 row=decision/hook-tests-real-binary area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook tests run the real binary
rule: Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json.
@row_add n=671 row=decision/hook-instance-check-first area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook instance check comes first
rule: The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
@row_add n=672 row=decision/hook-line-cases-in-made-instance-dir area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook line cases run in a viola-made instance dir
rule: Hook test cases that expect lines run in a viola-made instance directory.
why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.
@row_add n=673 row=decision/placed-copy-execs-at-once area=integration-test-strategy rec=fct-okfymegs@2
name: Placed copy closed before it is persisted
rule: A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement.
why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY).
guard: place_private_new_placed_copy_execs_at_once
@row_none n=116 rec=fct-5qxwlgko@3 type=5
@row_add n=674 row=decision/guarded-paths-start-stamped area=integration-test-strategy rec=fct-qmrx4mno@2
name: Check-guarded cases start stamped
rule: A test case that drives a check-guarded path starts from a stamped home.
@row_add n=675 row=decision/wrapper-drop-guard area=integration-test-strategy rec=fct-qmrx4mno@2
name: Test wrappers held in a Drop guard
rule: Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.
@row_add n=676 row=decision/private-dir-for-test-endpoints area=integration-test-strategy rec=fct-qmrx4mno@2,fct-pwscsozw@2
name: Test endpoint directories made private
rule: A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir.
why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
@row_add n=677 row=decision/readiness-stall-report area=integration-test-strategy rec=fct-qmrx4mno@2
name: Readiness stall report
rule: When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing.
why: So the report reaches the JUnit.
@row_add n=678 row=contract/wrapper-channel/params-v-higher area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: params.v higher than served
promise: A request whose params.v is higher than the served version is answered with error -32602.
@row_add n=679 row=contract/wrapper-channel/frame-over-max-frame area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: frame over MAX_FRAME
promise: A frame over MAX_FRAME is answered with error -32600 and the connection is closed; a pending dialog stays pending.
@row_add n=680 row=decision/server-identity-verified area=integration-test-strategy rec=fct-ksdwqwkf@2
name: Server identity verified before any frame
rule: A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed.
guard: security_negatives_server_identity
@row_add n=681 row=contract/mcp-unreachable-detail/strict-modes-failed area=integration-test-strategy rec=fct-ksdwqwkf@2,fct-bt3npe7x@2
boundary: MCP frame tools instance-unreachable detail
item: strict-modes-failed
promise: A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.
@row_add n=682 row=contract/mcp-unreachable-detail/socket-dir-refused area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: socket-dir-refused
promise: A refused socket directory check after the live endpoint is read gives detail socket-dir-refused, with no frame and no log line.
@row_add n=683 row=contract/mcp-unreachable-detail/server-verify-failed area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: server-verify-failed
promise: A peer euid the admission refuses, or a failed server verification, gives detail server-verify-failed, with no frame and no log line.
@row_add n=684 row=contract/mcp-unreachable-detail/instance-dead area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: instance-dead
promise: A failed connect gives detail instance-dead, with no frame and no log line.
@row_add n=685 row=contract/mcp-unreachable-detail/endpoint-lost area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: endpoint-lost
promise: A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost.
@row_add n=686 row=contract/mcp-frame-tools/refusal-result area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools
item: refusal result
promise: A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.
@row_add n=687 row=contract/mcp-frame-tools/free-text-refusal area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools
item: refusal of free text
promise: A wrapper refusal of free text reads as unknown, with no detail.
@row_add n=688 row=decision/paste-validated-at-both-ends area=integration-test-strategy rec=fct-ksdwqwkf@2
name: Sent text validated at client and wrapper
rule: Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper.
guard: channel_paste_validation
@row_add n=689 row=contract/wrapper-channel/empty-text area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: refusal detail empty-text
promise: An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.
@row_add n=690 row=contract/wrapper-channel/accepted-text area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: text with LF, CR, TAB and multi-byte characters
promise: Text with LF, a lone inner CR, TAB and multi-byte characters is accepted and typed as one prompt, its inner CR as one LF.
@row_add n=691 row=contract/cli-exit-codes/13-text-refused area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: CLI exit codes
item: 13 for refused text
promise: viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.
@row_add n=692 row=contract/viola-answer/oversize-stdin area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: viola answer
item: stdin over MAX_FRAME
promise: Exits 2 before connecting, with the fixed unusable-response line on stderr whatever --json says and no document; the dialog stays pending.
@row_add n=693 row=decision/bad-config-value-falls-back area=integration-test-strategy rec=fct-ksdwqwkf@2,fct-lzdm6e7w@6
name: Bad config value falls back to its default
rule: A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it.
guard: setting_does_not_disable_the_budget_gate
@row_add n=694 row=contract/channel-log/link-existing-pair area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel log
item: link of an existing pair
promise: Linking an already linked pair adds no log line.
@row_add n=695 row=contract/channel-log/unserved-method area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel log
item: response line of an unserved method
promise: The conn-less response line of an unserved method carries no method key; method names only the closed method list.
@row_add n=696 row=contract/event-line/keys area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event line
item: keys
promise: Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
@row_add n=697 row=contract/event-data/link area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: link
promise: data holds the keys driver and driven.
@row_add n=698 row=contract/event-data/unlink area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: unlink
promise: data holds the keys driver and driven.
@row_add n=699 row=contract/event-data/dialog-expired area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: dialog-expired
promise: data holds the keys dialog_id and dialog_kind.
@row_add n=700 row=contract/event-data/turn-ended area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: turn-ended
promise: data holds the keys last_assistant_message and background_tasks.
@row_add n=701 row=contract/event-data/question area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: question
promise: option_details is a known data key of a question line.
@row_add n=702 row=decision/tolerant-event-reader area=integration-test-strategy rec=fct-bt3npe7x@2
name: Tolerant event log reader
rule: The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
@row_add n=703 row=decision/snapshot-replayed-from-log area=integration-test-strategy rec=fct-bt3npe7x@2
name: Broken snapshot replayed from the log
rule: A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
@row_add n=704 row=decision/link-since-kept area=integration-test-strategy rec=fct-bt3npe7x@2
name: Link keeps its first since
rule: A linked pair keeps its first since until an unlink, and takes a later one after it.
@row_add n=705 row=contract/strict-modes-refusal/run area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola run
promise: Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
@row_add n=706 row=contract/strict-modes-refusal/mcp area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola mcp
promise: Exit 1.
@row_add n=707 row=contract/strict-modes-refusal/revive area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola revive
promise: Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
@row_add n=708 row=contract/strict-modes-refusal/verify area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola verify
promise: Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged.
@row_add n=709 row=contract/strict-modes-refusal/other-verbs area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: every other CLI verb
promise: Exit 21.
@row_add n=710 row=decision/instance-check-before-role-file area=integration-test-strategy rec=fct-bt3npe7x@2
name: Instance check before any role file or frame
rule: viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found.
why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home.
guard: link_verbs_refuse_a_widened_instance_before_any_log
@row_add n=711 row=contract/cli-json-error/instance-unreachable area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
@row_add n=712 row=decision/widened-home-tree-refused-at-start area=integration-test-strategy rec=fct-bt3npe7x@2
name: Widened home tree refused at start
rule: viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found.
guard: run_refuses_a_widened_home_tree_before_its_log
@row_add n=713 row=decision/home-read-rule area=integration-test-strategy rec=fct-bt3npe7x@2
name: Home readable by another user is refused
rule: A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing.
guard: a_home_another_user_may_read_refuses_run
@row_add n=714 row=contract/cli-json/one-document-per-verb area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: CLI --json
item: one document per run
promise: Each driver-facing verb prints one --json document and its literal exit code, for every exit code.
@row_add n=715 row=decision/human-output-pinned-by-literals area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Human output pinned by literals
rule: Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
@row_add n=716 row=decision/no-cli-snapshot-crates area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No CLI snapshot crates
rule: No manifest holds assert_cmd, predicates, trycmd or snapbox, and there is no command fixture directory.
@row_add n=717 row=decision/no-escape-in-plain-output area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No escape bytes in plain output
rule: No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE.
guard: cli_output_plain
@row_add n=718 row=decision/help-no-bold-underline area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Help carries no bold or underline
rule: viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF.
guard: cli_output_plain
@row_add n=719 row=contract/viola-help/verb-table area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola --help
item: grouped verb table
promise: Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty.
@row_add n=720 row=contract/verify-output/step-line area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify output
item: step line
promise: One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
@row_add n=721 row=contract/verify-output/summary-line area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify output
item: summary line
promise: The last stdout line is stamped <ver>  <n> pass  <m> fail.
@row_add n=722 row=contract/verify-stop/signal area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify interrupted
item: SIGINT or SIGTERM
promise: Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
@row_add n=723 row=contract/verify-exit-detail/program-not-found area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: program-not-found
promise: Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found.
@row_add n=724 row=contract/verify-exit-detail/pinned-hash-mismatch area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.
@row_add n=725 row=contract/verify-exit-detail/version-unreadable area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: version-unreadable
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.
@row_add n=726 row=contract/verify-exit-detail/strict-modes-failed area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses.
@row_add n=727 row=contract/verify-exit-detail/unclean-fixture area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: unclean-fixture
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
@row_add n=728 row=contract/verify-exit-detail/interrupted area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: interrupted
promise: Written on verify's process-exit when verify is stopped by a signal.
@row_add n=729 row=decision/verify-failing-row-no-refusal area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Failing ledger row is no refusal
rule: A failing ledger row in viola verify is no refusal and carries no code.
@row_add n=730 row=decision/verify-takes-no-json area=integration-test-strategy rec=fct-lzdm6e7w@6
name: viola verify takes no --json
rule: viola verify takes no --json flag.
@row_add n=731 row=decision/plugin-marketplace-mcp-only area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Plugin marketplace is MCP-only
rule: viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere.
guard: plugin_install_writes_the_mcp_only_marketplace
@row_add n=732 row=decision/plugin-install-owner-only area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Plugin install writes owner-only
rule: viola plugin install writes its tree at 0700 / 0600 whatever the umask, and refuses a widened home with exit 21, leaving the home as found.
guard: plugin_install_writes_owner_only_modes
@row_add n=733 row=contract/channel-response-detail/unsupported-version area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: unsupported-version
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_add n=734 row=contract/channel-response-detail/invalid-params area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: invalid-params
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_add n=735 row=contract/channel-response-detail/release-from-driver area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: release-from-driver
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_add n=736 row=decision/no-names-in-wrapper-log area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No session names in the wrapper log
rule: The wrapper's process log carries no driver or driven name, and channel lines carry no name or param.
guard: link_verbs_log_the_channel_pair_and_no_names
@row_add n=737 row=decision/home-resolution-order area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Home resolution order
rule: The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists.
why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening.
guard: home_resolution_takes_the_flag_then_the_session_then_the_default
@row_add n=738 row=decision/test-viola-drops-session-vars area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Test-started viola drops session variables
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
@row_add n=739 row=decision/ledger-probe-contract-suite area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Ledger probe contract suite
rule: For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list.
guard: contract_ledger_probes
@row_add n=740 row=decision/time-faked-through-seams area=mocking-stubbing-discipline rec=fct-je5xeiyl@1,fct-pskdyofh@1
name: Time faked through injected clocks
rule: Sync crates fake time with mock_instant or the injected Clock, MCP adapter timers use tokio's paused test clock, and a pure decision takes its instant as an argument.
@row_add n=741 row=decision/wrapper-deadline-injected-clock area=mocking-stubbing-discipline rec=fct-je5xeiyl@1
name: Wrapper deadlines driven by the injected clock
rule: The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.
@row_add n=742 row=decision/utc-now-through-clock-seam area=mocking-stubbing-discipline rec=fct-je5xeiyl@1
name: Wall clock read through the clock seam
rule: chrono Utc::now() is read only through the clock seam.
except: The budget gate's evaluation points read it and hand the instant to its pure decision; the link line's since is stamped directly, so tests assert since by shape only.
@row_add n=743 row=decision/external-processes-real-stubs area=mocking-stubbing-discipline rec=fct-sik7tvk4@2
name: External processes are real stub executables
rule: External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
@row_add n=744 row=decision/impostor-holder-real-process area=mocking-stubbing-discipline rec=fct-sik7tvk4@2
name: Impostor's held process is real
rule: A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard.
@row_add n=745 row=decision/real-process-faults area=mocking-stubbing-discipline rec=fct-63urz52n@3,fct-lzdm6e7w@6
name: Process faults are real
rule: A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
@row_add n=746 row=decision/impostor-server-real-listener area=mocking-stubbing-discipline rec=fct-pwscsozw@2
name: Impostor server is a real listener
rule: Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process.
@row_add n=747 row=decision/hook-unit-impostor-varies-start-time area=mocking-stubbing-discipline rec=fct-pwscsozw@2
name: Hook unit impostor varies the start time
rule: The hook's unit impostor varies the recorded start time, a second early or an unparseable started_at, instead of using a second process.
why: A root-bin unit test has no portable second process.
@row_add n=748 row=decision/real-filesystem-faults area=mocking-stubbing-discipline rec=fct-zsbxg2jw@2
name: Filesystem faults are real
rule: A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found.
except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.
@row_add n=749 row=decision/forced-window-hook-is-test-parameter area=mocking-stubbing-discipline rec=fct-7ilveif5@1
name: Forced-window hook is a test parameter
rule: A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.
@row_add n=750 row=decision/peer-admission-by-injection area=mocking-stubbing-discipline rec=fct-6xaoh2kl@2
name: Peer admission tested by injection
rule: The peer euid decision takes its euids as arguments and the private serve function takes its admission as an argument, with no env, flag or feature seam.
@row_add n=751 row=decision/coverage-includes-child-processes area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Coverage includes child viola processes
rule: Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE.
except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
@row_add n=752 row=decision/fake-agent-own-process-group area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Fake agent spawns in its own process group
rule: Every command the fake agent spawns runs in a process group of its own.
why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
@row_add n=753 row=decision/killed-test-child-uninstrumented area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Killed test child is uninstrumented
rule: A test that kills its child straight after the start uses an uninstrumented host program as that child.
why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
@row_add n=754 row=decision/profraw-census-dev-host-only area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Profraw census is a dev-host tool
rule: The profraw census script is a dev-host tool: no CI job, pre-push stage or harness command runs it.
@row_add n=755 row=decision/mutation-gate-at-epoch-boundary area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Mutation gate at the epoch boundary
rule: Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
@row_add n=756 row=contract/mutants-verdict/unviable-exceeds-caught area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
boundary: Mutation run verdict
item: unviable-exceeds-caught
promise: The run is red when unviable mutants outnumber caught ones.
@row_add n=757 row=decision/doctest-only-catch-is-missed area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Doctest-only catch counts as missed
rule: A mutant caught only by a doctest counts as missed, and the missing unit test must be added.
@row_add n=758 row=decision/host-unreachable-mutants area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Host-unreachable mutants recorded, not caught
rule: A mutant this host cannot compile or reach is recorded not measured here, owed to the route entry that measures it, never equivalent and never caught; one wholly under a cfg proved false for the host is left out of the count with its predicate.
@row_add n=759 row=decision/peer-admitted-survivor-known-limit area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Peer admission survivor is a known limit
rule: A mutant making a client seam's peer admission always admit survives every committed test and stands as a known limit, read at each epoch-boundary audit.
why: One account can make no foreign peer, and no test seam may be added.
@row_add n=760 row=decision/report-files-per-test-process area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Report files per test process
rule: Test report files carry the test process's id, so two concurrent jobs never share one.
@row_add n=761 row=decision/mutation-harness-tests-ungated area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Real cargo-mutants harness tests ungated
rule: The two real-cargo-mutants harness tests run ungated, never ignored, and the CI profile's 120 s kill and zero retries stay unchanged.
@row_add n=762 row=decision/perf-panic-check-before-cleanup area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf run panic check
rule: After the last perf row and before cleanup, the perf run fails as panic when the session home's role files hold any panic event line, with no seam exemption.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=763 row=decision/perf-failure-turns-suite-red area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf failures named, never skipped
rule: A failed build, boot, row, panic check or cleanup turns suite perf red with the failure named, never a missing or skipped row.
@row_add n=764 row=decision/perf-hooks-time-the-real-path area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf hooks time the real path
rule: Every timed hook runs with an instance name and directory set, so it takes the real logging and channel path, never the no-instance no-op, with its instance, server, socket-directory and peer checks inside the timed path.
@row_add n=765 row=decision/product-state-never-hand-written area=test-data-fixtures rec=fct-bxlupfu3@2
name: On-disk product state produced by viola
rule: On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written.
except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
@row_add n=766 row=decision/stamps-only-from-verify area=test-data-fixtures rec=fct-jg3hj3ma@1,fct-6upjfbpd@2
name: Stamps written only by verify
rule: The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it.
why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
@row_add n=767 row=decision/budget-seeded-through-statusline-hook area=test-data-fixtures rec=fct-4ejdt5qs@1,fct-6upjfbpd@2
name: Budget seeded through the statusline hook
rule: A test's budget file is seeded by piping generated statusline stdin into viola hook statusline.
except: The budget reader's own unit cases pad, replace or re-version bytes the writer wrote, because no writer produces an over-cap, unparseable or v 2 file.
@row_add n=768 row=decision/statusline-source-written-by-test area=test-data-fixtures rec=fct-652w7iir@2
name: Statusline source planted by the test
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_none n=117 rec=fct-jfjmhkwu@2 type=5
@row_none n=118 rec=fct-t5v77dkt@2 type=5
@row_add n=769 row=decision/socket-dir-check-leaves-dir-as-found area=test-data-fixtures rec=fct-2mxqmdgt@1,fct-n2arkmru@2
name: Socket-directory check never repairs
rule: The socket-directory check never chmods the directory it checks; a refused socket directory is left as found.
@row_add n=770 row=decision/foreign-owner-by-injected-uid area=test-data-fixtures rec=fct-2mxqmdgt@1
name: Foreign socket-directory owner by injected uid
rule: A socket directory with a foreign owner is not driven in tests; its verdict is taken by an injected uid.
@row_add n=771 row=decision/agents-stub-from-files-no-env area=test-data-fixtures rec=fct-g6w3aucc@1,fct-6upjfbpd@2
name: Agents list answered by a stub on PATH
rule: claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH.
why: The no-env answer was ratified.
@row_add n=772 row=decision/hook-payloads-recorded-by-verify area=test-data-fixtures rec=fct-myaghpwm@2,fct-6upjfbpd@2
name: Hook payloads recorded by verify, replayed by CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
@row_add n=773 row=contract/claude-fixtures/payload-path area=test-data-fixtures rec=fct-myaghpwm@2
boundary: Recorded Claude fixtures
item: payload path
promise: A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
@row_add n=774 row=contract/mcp-fixture-scrub-refusal/home-path area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: home-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=775 row=contract/mcp-fixture-scrub-refusal/absolute-path area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: absolute-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=776 row=contract/mcp-fixture-scrub-refusal/username area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: username
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=777 row=contract/mcp-fixture-scrub-refusal/email area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: email
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=778 row=decision/human-output-expected-as-literals area=test-data-fixtures rec=fct-6wjlyq3w@2
name: Human-mode expected output as literals
rule: Human-mode expected output is held as literals in the test files that check it.
@row_add n=779 row=decision/decision-bodies-as-insta-snapshots area=test-data-fixtures rec=fct-dvphj5bz@1
name: Decision bodies as insta snapshots
rule: Decision bodies are checked against insta snapshots, regenerated only by the local fixture refresh.
@row_add n=780 row=decision/fake-transcripts-compared-to-recordings area=test-data-fixtures rec=fct-moqhwhlq@1
name: Fake-agent transcripts compared byte for byte
rule: Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
@row_add n=781 row=decision/proptest-for-randomized-data area=test-data-fixtures rec=fct-tuwzhtch@1
name: proptest for randomized data
rule: Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
@row_add n=782 row=decision/mock-instant-behind-clock-seam area=test-data-fixtures rec=fct-ovluqlfu@1
name: Mocked time behind the clock seam
rule: Time in the sync crates is mocked with mock_instant's MockClock in global mode behind the core Clock seam.
@row_add n=783 row=decision/link-since-asserted-by-shape area=test-data-fixtures rec=fct-ovluqlfu@1
name: Link time asserted by shape
rule: The link line's since time is asserted by its shape only.
why: It is stamped outside the clock seam.
@row_add n=784 row=contract/mcp-wait-tool/timeout-bound area=test-data-fixtures rec=fct-ovluqlfu@1
boundary: MCP wait tool
item: wait bound
promise: The wait bound is the given timeout capped at 50 000 ms, min(given, 50 000).
@row_none n=119 rec=fct-7tiauhj7@1 type=4
relied: The error-body check reads a hit of a planted value as class planted and reports place and class, never bytes.
@row_none n=120 rec=fct-rgx56qfp@2 type=5
@row_none n=121 rec=fct-wmieczsn@2 type=5
@row_add n=785 row=decision/test-endpoint-dirs-made-private area=test-data-fixtures rec=fct-fvd26vjh@1,fct-6upjfbpd@2
name: Test endpoint directories are private
rule: A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir.
why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
@row_add n=786 row=decision/fake-agent-copies-planted-first area=test-data-fixtures rec=fct-q4qwgsv5@1
name: Fake-agent copies planted before any child
rule: A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child.
why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).
@row_add n=787 row=decision/fake-agent-replays-recorded-bytes area=test-data-fixtures rec=fct-q6tl7r52@1
name: Fake agent replays recorded bytes
rule: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the resume launch SessionStart for a set without the recorded resume variant and the compact replay's session id.
why: The rewritten resume payload was ratified.
@row_add n=788 row=contract/fake-agent-receipt/line-format area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: line format
promise: Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
@row_add n=789 row=contract/fake-agent-receipt/start area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: start
promise: Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
@row_add n=790 row=contract/fake-agent-receipt/size area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: size
promise: Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
@row_add n=791 row=contract/fake-agent-receipt/cwd area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: cwd
promise: Carries cwd.
@row_add n=792 row=contract/fake-agent-receipt/hold area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: hold
promise: Carries pid, written by the --exit-no-eof holder at its start.
@row_add n=793 row=contract/fake-agent-receipt/fds area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: fds
promise: Carries fds.
@row_add n=794 row=contract/fake-agent-receipt/key area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: key
promise: Carries hex, one line per byte received outside a paste.
@row_add n=795 row=contract/fake-agent-receipt/prompt area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: prompt
promise: Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
@row_add n=796 row=contract/fake-agent-receipt/hook area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: hook
promise: Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
@row_add n=797 row=contract/fake-agent-receipt/statusline area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: statusline
promise: Carries the hook kind's fields without event.
@row_add n=798 row=contract/fake-agent-receipt/step area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: step
promise: Carries index and event.
@row_add n=799 row=decision/fake-agent-argv-only area=test-data-fixtures rec=fct-vrevms4b@1
name: Fake agent configured by argv alone
rule: The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments.
@row_add n=800 row=decision/harness-cli-version-matches-fake area=test-data-fixtures rec=fct-vrevms4b@1
name: One default CLI version
rule: The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
@row_add n=801 row=decision/wrapper-boots-pass-trusted-screens area=test-data-fixtures rec=fct-vrevms4b@1
name: Test wrapper boots run trusted with screens
rule: Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd.
except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
@row_add n=802 row=decision/fixtures-walked-for-hygiene area=test-data-fixtures rec=fct-o7d2jylf@1
name: Fixture hygiene walk
rule: Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red.
guard: contract_fixture_hygiene
@row_add n=803 row=contract/process-exit-detail/unclean-fixture area=test-data-fixtures rec=fct-hc6txkos@1
boundary: process-exit detail
item: unclean-fixture
promise: verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
@row_add n=804 row=decision/cross-user-at-peer-check-boundary area=test-scope-summary rec=fct-jnkzde5v@2
name: Cross-user rejection asserted at the boundary
rule: Rejection of a peer with a different euid is asserted at the peer-credential check boundary, over injected euids and an injected decision, with no seam added to the client wiring.
why: Hosted CI runners have no second OS account, so a foreign peer cannot be driven.
guard: peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported
@row_add n=805 row=contract/wrapper-channel/max-frame area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Wrapper channel
item: MAX_FRAME
promise: A frame is at most 16 MiB.
@row_add n=806 row=contract/cli-frame-verbs/server-verify-failed area=test-scope-summary rec=fct-jnkzde5v@2
boundary: CLI frame verbs
item: server-verify-failed
promise: A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
@row_add n=807 row=contract/hook/impostor-server area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Hook
item: impostor server
promise: A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
@row_add n=808 row=contract/channel-fault-screen/known-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: known code
promise: Each of the five known codes takes its fixed message, with data null save the kept -32602 data.
@row_add n=809 row=contract/channel-fault-screen/-32602 area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: -32602
promise: Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null.
@row_add n=810 row=contract/channel-fault-screen/unknown-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: code outside the five
promise: Reads wrapper fault with data null.
@row_add n=811 row=contract/channel-fault-screen/no-integer-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: error without an integer code
promise: Reads nothing.
@row_add n=812 row=decision/real-claude-never-in-ci area=test-scope-summary rec=fct-zgqp2syf@2,fct-67zmkx76@3
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
@row_add n=813 row=contract/revive-refusal-code/strict-modes-failed area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: strict-modes-failed
promise: A closed code of the revive --json refusal document.
@row_add n=814 row=contract/revive-refusal-code/already-live area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: already-live
promise: A closed code of the revive --json refusal document.
@row_add n=815 row=contract/revive-refusal-code/no-session area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: no-session
promise: A closed code of the revive --json refusal document; also the document of an empty --list chain.
@row_add n=816 row=contract/revive-refusal-code/session-live area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: session-live
promise: A closed code of the revive --json refusal document.
@row_add n=817 row=contract/revive-refusal-code/cwd-missing area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: cwd-missing
promise: A closed code of the revive --json refusal document.
@row_add n=818 row=contract/revive-refusal-code/stale-heartbeat area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: stale-heartbeat
promise: A closed code of the revive --json refusal document.
@row_add n=819 row=contract/revive-refusal-code/pinned-hash-mismatch area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: pinned-hash-mismatch
promise: A closed code of the revive --json refusal document.
@row_add n=820 row=contract/revive-refusal-code/squatted-name area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: squatted-name
promise: A closed code of the revive --json refusal document.
@row_add n=821 row=contract/revive-refusal-code/socket-dir-refused area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: socket-dir-refused
promise: A closed code of the revive --json refusal document.
@row_add n=822 row=contract/revive-refusal-code/internal-error area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: internal-error
promise: A closed code of the revive --json refusal document.
@row_add n=823 row=contract/revive-json/list-document area=test-scope-summary rec=fct-xttglbra@2
boundary: revive --json
item: --list document
promise: revive --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_add n=824 row=contract/revive-json/pump-child-error area=test-scope-summary rec=fct-xttglbra@2
boundary: revive --json
item: child pump error
promise: An error while pumping the child prints no document, the child having held the terminal.
@row_add n=825 row=contract/revive-exit-codes/0 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 0
promise: Success.
@row_add n=826 row=contract/revive-exit-codes/1 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 1
promise: A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
@row_add n=827 row=contract/revive-exit-codes/2 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_add n=828 row=decision/frame-verbs-check-instance-first area=test-scope-summary rec=fct-xttglbra@2
name: Frame verbs check the instance first
rule: All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
@row_add n=829 row=decision/self-link-refused-by-wrapper area=test-scope-summary rec=fct-xttglbra@2,fct-5qkdel63@2
name: Self-link refused by the wrapper
rule: A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20.
@row_add n=830 row=absence/windows-and-macos-support area=test-scope-summary rec=fct-ze6ljnvw@1
capability: Windows and macOS support: code, tests, fixtures and CI jobs
@row_add n=831 row=decision/cli-json-one-document area=test-scope-summary rec=fct-fnukifof@4
name: One JSON document per verb
rule: Under --json a CLI verb signals by its exit code plus one JSON document on stdout.
except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
guard: NOT_ERROR_BODIES
@row_add n=832 row=decision/no-sgr-without-tty area=test-scope-summary rec=fct-fnukifof@4
name: No SGR without a terminal
rule: CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
@row_add n=833 row=decision/human-tty-through-outer-pty area=test-scope-summary rec=fct-fnukifof@4
name: Human TTY rendering through the outer PTY
rule: The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_add n=834 row=contract/cli-human-lines/read-back area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: read back
promise: [RB] read back … cursor <n>
@row_add n=835 row=contract/cli-human-lines/unable area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: unable
promise: unable  <name>  <reason>  <detail>, followed by a last-line hint:.
@row_add n=836 row=contract/cli-human-lines/stamped area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: stamped
promise: stamped <ver>  <n> pass  <n> fail
@row_add n=837 row=contract/revive-human/refusal area=test-scope-summary rec=fct-fnukifof@4
boundary: revive human output
item: refusal
promise: Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty.
@row_add n=838 row=contract/revive-human/list-row area=test-scope-summary rec=fct-fnukifof@4
boundary: revive human output
item: --list row
promise: One <ts>  <cause>  <id> row per logged session on stdout.
@row_add n=839 row=decision/never-parse-rendered-child-screen area=test-scope-summary rec=fct-b2j3wr36@3
name: Rendered child screen never parsed
rule: Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search.
why: Named a test anti-pattern by the creator.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=840 row=contract/wrapper-channel/success area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: success
promise: {"result":{"ok":…}}
@row_add n=841 row=contract/wrapper-channel/refusal area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: refusal
promise: {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
@row_add n=842 row=contract/wrapper-channel/protocol-fault area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: protocol fault
promise: {"error":{code,message,data}}.
@row_add n=843 row=contract/wrapper-channel/-32603 area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: -32603
promise: internal error with data null.
@row_add n=844 row=contract/wrapper-channel/link area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: link
promise: Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603.
@row_add n=845 row=contract/wrapper-channel/unlink area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: unlink
promise: Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603.
@row_add n=846 row=contract/channel-response-line/detail area=test-scope-summary rec=fct-5qkdel63@2
boundary: channel-response log line
item: detail
promise: On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param.
@row_add n=847 row=decision/channel-error-bodies-literal area=test-scope-summary rec=fct-5qkdel63@2
name: Channel error bodies are literals
rule: The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text.
guard: channel_error_bodies_hold_no_path_or_upstream_text
@row_add n=848 row=contract/mcp-tool-result/ok area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: ok
promise: isError false with structuredContent equal to the ok payload.
@row_add n=849 row=contract/mcp-tool-result/refusal area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: refusal
promise: isError true with {refusal, detail} and text "refused: <reason>".
@row_add n=850 row=contract/mcp-tool-result/instance-unreachable area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: instance-unreachable
promise: isError true with {"error":"instance-unreachable","detail":"<cause>"}, the CLI exit-21 document less v, text refused: <code>.
@row_add n=851 row=contract/instance-unreachable-cause/strict-modes-failed area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: strict-modes-failed
promise: A closed cause word of instance-unreachable's detail.
@row_add n=852 row=contract/instance-unreachable-cause/instance-dead area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: instance-dead
promise: A closed cause word of instance-unreachable's detail.
@row_add n=853 row=contract/instance-unreachable-cause/unwrapped-name area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: unwrapped-name
promise: A closed cause word of instance-unreachable's detail.
@row_add n=854 row=contract/instance-unreachable-cause/endpoint-lost area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: endpoint-lost
promise: A closed cause word of instance-unreachable's detail.
@row_add n=855 row=contract/instance-unreachable-cause/server-verify-failed area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: server-verify-failed
promise: A closed cause word of instance-unreachable's detail.
@row_add n=856 row=contract/instance-unreachable-cause/socket-dir-refused area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A closed cause word of instance-unreachable's detail.
@row_add n=857 row=contract/mcp-tool-result/wrapper-fault area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: wrapper-fault
promise: isError true with {"error":"wrapper-fault","detail":{code,message,data}}, the detail being the channel fault screen's.
@row_add n=858 row=contract/mcp-tool-result/invalid-params area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: invalid-params
promise: isError true with {"error":"invalid-params","detail":"<field>"}, no frame written.
@row_add n=859 row=contract/mcp-invalid-params-field/target area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: target
promise: A closed field word of invalid-params' detail.
@row_add n=860 row=contract/mcp-invalid-params-field/text area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: text
promise: A closed field word of invalid-params' detail.
@row_add n=861 row=contract/mcp-invalid-params-field/after area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: after
promise: A closed field word of invalid-params' detail.
@row_add n=862 row=contract/mcp-invalid-params-field/timeout-ms area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: timeout_ms
promise: A closed field word of invalid-params' detail.
@row_add n=863 row=contract/mcp-invalid-params-field/dialog-id area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: dialog_id
promise: A closed field word of invalid-params' detail.
@row_add n=864 row=contract/mcp-invalid-params-field/response area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: response
promise: A closed field word of invalid-params' detail.
@row_add n=865 row=contract/mcp-server/internal-fault area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: internal handler fault
promise: Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_add n=866 row=contract/mcp-server/unknown-tool area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: unknown tool
promise: The JSON-RPC error -32602.
@row_add n=867 row=contract/mcp-server/tools-list area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: tools/list
promise: Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private.
@row_add n=868 row=decision/mcp-no-control-tools area=test-scope-summary rec=fct-elurmfjv@1
name: No control tools over MCP
rule: The MCP server offers no release, pause, link or unlink tool.
@row_add n=869 row=decision/mcp-negatives-violate-schema area=test-scope-summary rec=fct-elurmfjv@1
name: MCP negatives violate the advertised schema
rule: MCP tests send negative inputs that violate the advertised tool schemas.
why: The schemas are advisory.
@row_add n=870 row=decision/mcp-frame-tools-check-instance-first area=test-scope-summary rec=fct-elurmfjv@1
name: MCP frame tools check the instance first
rule: The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line.
guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
@row_add n=871 row=contract/mcp-reply-screen/refusal area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_add n=872 row=contract/mcp-reply-screen/detail area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP reply screen
item: detail
promise: A string detail that is no code word reads null, and a non-string detail is no known reply.
@row_add n=873 row=contract/mcp-stdin/oversize-before-initialize area=test-scope-summary rec=fct-elurmfjv@1
boundary: viola mcp stdin
item: line past MAX_FRAME before initialize
promise: viola mcp exits 1 with both streams empty and one parse-rejected mcp-stdin oversize line.
@row_add n=874 row=absence/browser-suite area=test-scope-summary rec=fct-67zmkx76@3
capability: A browser test suite
@row_add n=875 row=decision/mutation-testing-on-demand area=test-scope-summary rec=fct-67zmkx76@3
name: Mutation testing on demand
rule: Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
@row_add n=876 row=contract/harness-run/exit area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness run command
item: exit
promise: Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
@row_add n=877 row=contract/harness-boot/readiness-failure area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness boot command
item: readiness failure
promise: Exits non-zero with a JSON reason on stdout; a run refusal to start is surfaced verbatim.
@row_add n=878 row=contract/harness-cleanup/idempotent area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness cleanup command
item: second run
promise: A second cleanup exits 0 and reports nothing to clean.
@row_add n=879 row=contract/harness-cleanup/endpoint-gone area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness cleanup command
item: endpoint check
promise: Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
@row_add n=880 row=decision/events-never-truncated area=test-scope-summary rec=fct-67zmkx76@3
name: Events never truncated
rule: Instance events are never truncated; their retention in tests is the lifetime of the temp home.
@row_add n=881 row=decision/per-test-fresh-home area=test-scope-summary rec=fct-6upjfbpd@2
name: Self-bootstrapping per-test home
rule: Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
@row_add n=882 row=decision/fuzz-the-parser-surfaces area=test-scope-summary rec=fct-6upjfbpd@2
name: Property and fuzz generators on parsers
rule: Property and fuzz generators cover the parser surfaces the security plan lists.
@row_add n=883 row=decision/fixtures-never-from-real-prompts area=test-scope-summary rec=fct-6upjfbpd@2
name: Fixtures never from real prompts
rule: Fixtures must not come from real prompts and must be checked for absolute paths and usernames.
why: A security anti-pattern.
@row_add n=884 row=decision/run-checks-before-child-spawn area=test-scope-summary rec=fct-n2arkmru@2
name: viola run checks before the child
rule: viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child.
guard: run_refuses_a_widened_home_tree_before_its_log
@row_add n=885 row=contract/run-start-events/order area=test-scope-summary rec=fct-n2arkmru@2
boundary: run start events
item: order
promise: The events log reads wheel with cause start, then budget-gate, then the agent's session-start with source hook.
@row_add n=886 row=contract/run-refusal/socket-dir-refused area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: socket-dir-refused
promise: A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
@row_add n=887 row=contract/run-refusal/stale-heartbeat area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: stale-heartbeat
promise: A stale holder refuses the start with exit 1.
@row_add n=888 row=contract/run-refusal/already-live area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: already-live
promise: A live holder or a fresh replayed beat refuses the start with exit 1.
@row_add n=889 row=contract/instance-settings/statusline-override area=test-scope-summary rec=fct-n2arkmru@2
boundary: Instance settings file
item: statusline override
promise: Written 0600 with the absolute pinned path, only for a pinned path of plain characters.
@row_add n=890 row=contract/send-event-log/send-issued area=test-scope-summary rec=fct-lhvxskln@1
boundary: the send event log
item: send-issued
promise: An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
@row_add n=891 row=contract/send-confirmation/prompt-submitted-driver area=test-scope-summary rec=fct-lhvxskln@1
boundary: send confirmation
item: prompt-submitted with origin driver
promise: A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
@row_add n=892 row=contract/cli-send-json/exit-0 area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: exit 0
promise: A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
@row_add n=893 row=contract/cli-send-json/no-prompt-submitted area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: not-delivered / no-prompt-submitted
promise: A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
@row_add n=894 row=contract/cli-send-json/empty-text area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: not-delivered / empty-text
promise: A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
@row_add n=895 row=decision/sent-text-typed-form area=test-scope-summary rec=fct-lhvxskln@1,fct-hpqcf7r7@3
name: Sent text is typed in normalised form
rule: A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched.
guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
@row_add n=896 row=contract/send-result/unconfirmable area=test-scope-summary rec=fct-lhvxskln@1
boundary: the send result
item: unconfirmable
promise: A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
@row_add n=897 row=decision/local-command-post-condition area=test-scope-summary rec=fct-lhvxskln@1
name: Local commands confirmed by post-condition only
rule: On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
@row_add n=898 row=decision/multi-line-send-one-prompt area=test-scope-summary rec=fct-lhvxskln@1
name: Multi-line text is one prompt
rule: A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
@row_add n=899 row=contract/mcp-send/success area=test-scope-summary rec=fct-lhvxskln@1
boundary: MCP send
item: success result
promise: A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
@row_add n=900 row=decision/human-input-takes-the-wheel area=test-scope-summary rec=fct-3oyxjfyz@1
name: Human input takes the wheel
rule: A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
@row_add n=901 row=contract/event-log/wheel-human-input area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: the event log
item: wheel
promise: A human taking the wheel logs wheel with holder human and cause human-input.
@row_add n=902 row=contract/cli-send-json/human-typing area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI send --json
item: exit 10
promise: A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
@row_add n=903 row=contract/mcp-send/refused-human-typing area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: MCP send
item: refused: human-typing
promise: An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
@row_add n=904 row=contract/cli-release/wheel-driver area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI release
item: wheel driver
promise: Release gives wheel driver, and the next send is accepted.
@row_add n=905 row=contract/cli-release/release-from-driver area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI release
item: release-from-driver
promise: A release carrying from is rejected with -32602 release-from-driver, exit 20.
@row_add n=906 row=decision/harness-turn-keeps-the-wheel area=test-scope-summary rec=fct-3oyxjfyz@1
name: Harness-injected turns keep the wheel
rule: A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel.
@row_add n=907 row=decision/statusline-passthrough area=test-scope-summary rec=fct-qobe5v26@2
name: Statusline passthrough
rule: The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0.
@row_add n=908 row=contract/budget-json/rate-limits area=test-scope-summary rec=fct-qobe5v26@2,fct-hpqcf7r7@3
boundary: budget.json
item: rate_limits
promise: The statusline hook writes rate_limits to budget.json only when the payload holds a rate_limits object; a malformed resets_at becomes "unknown".
@row_add n=909 row=decision/statusline-shell-out-frame-cap area=test-scope-summary rec=fct-qobe5v26@2
name: Statusline shell-out frame cap
rule: The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command.
guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap
@row_add n=910 row=contract/event-log/budget-gate area=test-scope-summary rec=fct-qobe5v26@2
boundary: the event log
item: budget-gate
promise: A budget pause logs budget-gate with paused true and its window.
@row_add n=911 row=contract/cli-send-json/budget-paused area=test-scope-summary rec=fct-qobe5v26@2
boundary: CLI send --json
item: exit 11
promise: A send refused by the budget gate exits 11 with detail five-hour or seven-day.
@row_add n=912 row=contract/cli-release/budget area=test-scope-summary rec=fct-qobe5v26@2
boundary: CLI release --budget
item: budget_paused false
promise: Release --budget gives budget_paused false, with override_until when a pause stood.
@row_add n=913 row=decision/torn-tail-healed-by-append area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Torn log tail healed by the next append
rule: A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal.
guard: torn_tail
@row_add n=914 row=decision/snapshot-replay-fields area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Snapshot replay recovers a fixed field set
rule: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd.
guard: replay_recovers, chaos_snapshot_replay
@row_add n=915 row=decision/snapshot-read-through-replay area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Every product snapshot reader goes through read-or-replay
rule: Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
@row_add n=916 row=contract/cli-wait/endpoint-lost area=test-scope-summary rec=fct-yxyv2kzp@2
boundary: CLI wait
item: endpoint-lost
promise: An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost.
@row_add n=917 row=decision/fake-agent-contract-suite area=test-scope-summary rec=fct-3wc25tvq@1,fct-vekwchud@2
name: Fake agent held to recorded fixtures
rule: A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations.
why: guards against the fake agent drifting from the real CLI
guard: contract_fake_agent_drift
@row_add n=918 row=decision/fixtures-recorded-by-live-verify area=test-scope-summary rec=fct-3wc25tvq@1
name: Fixtures refreshed by live verify
rule: The recorded agent fixtures are refreshed only by a live viola verify run, locally.
@row_add n=919 row=decision/nextest-is-the-runner area=test-strategy rec=fct-7osf2h3d@2
name: nextest is the runner
rule: nextest is the test runner; a plain cargo test run is not supported.
@row_add n=920 row=decision/e2e-over-fake-agent area=test-strategy rec=fct-pe3lhjgn@1
name: E2E over the fake agent
rule: Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
@row_add n=921 row=decision/fuzz-nightly-and-corpus-gate area=test-strategy rec=fct-akm552hs@1
name: Fuzzing nightly with a corpus replay gate
rule: cargo-fuzz targets run as a time-boxed nightly job, plus a corpus replay gate, with each parser getting a target as it lands.
@row_add n=922 row=decision/heartbeat-staleness-pure-classifier area=test-strategy rec=fct-zx23mmpw@2,fct-vbcc7bad@2
name: Heartbeat staleness by a pure classifier
rule: Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
@row_add n=923 row=decision/chaos-fault-sources area=test-strategy rec=fct-zinazvq2@1
name: Chaos fault sources
rule: Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
@row_add n=924 row=decision/mutation-run-verdict area=test-strategy rec=fct-cz5jn5fs@2
name: Mutation run verdict
rule: A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones.
except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
@row_add n=925 row=decision/bounded-remove-the-guard-witness area=test-strategy rec=fct-cz5jn5fs@2
name: Bounded remove-the-guard witness
rule: A control whose neutralised guard lets a product process start needs a witness bounded by a time limit.
why: nextest's default profile has no kill line, and an unbounded witness hung
@row_add n=926 row=decision/dialog-bodies-pinned-by-insta area=test-strategy rec=fct-vekwchud@2
name: Dialog decision bodies pinned by insta
rule: insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
@row_add n=927 row=decision/relayed-fixtures-kept-superseded area=test-strategy rec=fct-vekwchud@2
name: Relayed fixtures kept as superseded
rule: Fixtures relayed from the prototype are superseded by live recordings but stay committed with their provenance under a Superseded section.
@row_add n=928 row=decision/test-only-e2e-crate area=test-strategy rec=fct-hckuzazj@4
name: Test-only E2E crate
rule: One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root.
why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
@row_add n=929 row=decision/one-shared-test-helper area=test-strategy rec=fct-hckuzazj@4
name: One shared copy of each test helper
rule: Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
@row_add n=930 row=decision/tests-never-read-host-sessions area=test-strategy rec=fct-hckuzazj@4
name: Tests never read the host's claude sessions
rule: A test's list child runs with PATH set to an empty bin dir, and the fake agent is planted as claude only through one shared helper.
@row_add n=931 row=decision/e2e-layer-by-binary-prefix area=test-strategy rec=fct-hckuzazj@4
name: E2E layer selected by binary prefix
rule: The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
@row_none n=122 rec=fct-df3r2j7c@1 type=5
@row_add n=932 row=decision/ipc-vector-test-layers area=test-strategy rec=fct-34amt7nu@2
name: IPC vector test layers
rule: The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
@row_add n=933 row=decision/socket-directory-refusal area=test-strategy rec=fct-34amt7nu@2
name: Loosened socket directory refused
rule: A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
@row_add n=934 row=contract/channel-links/invalid-pair area=test-strategy rec=fct-34amt7nu@2
boundary: the wrapper channel
item: link and unlink with a bad name or pair
promise: A raw link or unlink frame with a bad name or pair gets -32602 and no line is written.
@row_add n=935 row=decision/paste-injection-test-layers area=test-strategy rec=fct-ggbcgrqb@1
name: Paste injection test layers
rule: Paste injection is tested by unit property tests, integration and E2E.
@row_add n=936 row=decision/hook-stdin-test-layers area=test-strategy rec=fct-yhevpdmp@2
name: Hook stdin test layer
rule: Hook stdin is tested at integration, through the hook as a subprocess.
@row_add n=937 row=decision/mcp-test-layer area=test-strategy rec=fct-wzml6tug@1
name: MCP test layer
rule: MCP is tested at E2E through an rmcp client.
@row_add n=938 row=decision/cli-env-test-layers area=test-strategy rec=fct-3o7lfwre@1
name: CLI and env test layers
rule: CLI and environment input is tested at unit and integration.
@row_add n=939 row=decision/filesystem-test-layers area=test-strategy rec=fct-mqdaf5bf@2
name: Filesystem test layers
rule: The filesystem vector is tested at integration and chaos, including strict-mode refusals.
@row_add n=940 row=decision/home-another-user-may-read-refused area=test-strategy rec=fct-mqdaf5bf@2
name: Readable home refused
rule: A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
@row_add n=941 row=decision/child-spawn-test-layers area=test-strategy rec=fct-svqwosl6@1
name: Child spawn test layers
rule: Child spawn is tested at integration and E2E.
@row_add n=942 row=decision/supply-chain-ci-gates area=test-strategy rec=fct-peucakqx@1
name: Supply chain held by CI gates
rule: The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
@row_add n=943 row=decision/property-trigger-layer area=test-strategy rec=fct-l3h5275l@1
name: Property-test trigger layer
rule: The property-test trigger is met by proptest and the fuzz job.
@row_add n=944 row=decision/chaos-trigger-layer area=test-strategy rec=fct-x65537y4@1
name: Chaos-test trigger layer
rule: The chaos-test trigger is met by a chaos suite.
@row_add n=945 row=decision/performance-trigger-layer area=test-strategy rec=fct-b6yqyazs@1
name: Performance-budget trigger layer
rule: The performance-budget trigger is met by perf budget tests.
@row_add n=946 row=decision/cross-surface-trigger-layer area=test-strategy rec=fct-s3ozhmcp@1
name: Cross-surface trigger layer
rule: Cross-surface coordination is tested at E2E, with cross-surface parity checks.
@row_add n=947 row=decision/single-os-ci-leg area=test-strategy rec=fct-gldxhydz@3
name: One CI leg on Linux
rule: The CI matrix builds on one leg, ubuntu-latest.
@row_add n=948 row=decision/os-gated-tests-registered area=test-strategy rec=fct-gldxhydz@3
name: OS-gated tests registered with a reason
rule: Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=949 row=decision/multi-version-trigger-layers area=test-strategy rec=fct-iwh5tatz@1
name: Multi-version compatibility layers
rule: Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
@row_add n=950 row=decision/sandbox-contract-trigger-layer area=test-strategy rec=fct-ffgiydwd@1
name: Sandbox contract trigger layer
rule: The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
@row_add n=951 row=decision/mutation-trigger-layer area=test-strategy rec=fct-2aejh4x2@1
name: Mutation trigger layer
rule: Mutation testing is met by the mutation run.
@row_add n=952 row=decision/discipline-at-every-layer area=test-strategy rec=fct-brzfiy4i@1
name: Discipline held at every layer
rule: No screen parsing, no blanket approval and no flaky tests hold at every test layer.
@row_add n=953 row=contract/code-word/format area=unit-test-strategy rec=fct-iqktwdfa@1
boundary: code words
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_add n=954 row=contract/unreachable-cause/socket-dir-refused area=unit-test-strategy rec=fct-iqktwdfa@1
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
@row_add n=955 row=contract/json-rpc-errors/-32700 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32700
promise: Parse error is reported as -32700.
@row_add n=956 row=contract/json-rpc-errors/-32600 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32600
promise: Invalid request is reported as -32600.
@row_add n=957 row=contract/json-rpc-errors/-32601 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32601
promise: Unknown method is reported as -32601.
@row_add n=958 row=contract/json-rpc-errors/-32602 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32602
promise: Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart.
@row_add n=959 row=contract/json-rpc-errors/wrapper-fault area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: a code outside the five
promise: A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code.
@row_add n=960 row=contract/channel-endpoint/socket-path area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: the wrapper channel endpoint
item: socket path
promise: The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
@row_add n=961 row=decision/defensive-log-reader area=unit-test-strategy rec=fct-vbcc7bad@2
name: Event log reader counts and never panics
rule: The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned.
guard: three_counts
@row_add n=962 row=decision/server-identity-check area=unit-test-strategy rec=fct-vbcc7bad@2
name: Server identity check
rule: A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint.
guard: server_verdict_rows
@row_add n=963 row=decision/strict-modes-two-rules area=unit-test-strategy rec=fct-vbcc7bad@2
name: Strict modes write and read rules
rule: Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule.
guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
@row_add n=964 row=decision/socket-directory-check area=unit-test-strategy rec=fct-vbcc7bad@2
name: Socket directory check
rule: The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes.
guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
@row_add n=965 row=decision/stamps-refuse-widened-ledger area=unit-test-strategy rec=fct-vbcc7bad@2
name: Stamps refuse a widened ledger
rule: Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing.
guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
@row_add n=966 row=decision/private-placement-mode-first area=unit-test-strategy rec=fct-vbcc7bad@2
name: Private copies placed mode-first
rule: A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name.
guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
@row_add n=967 row=decision/bounded-compare area=unit-test-strategy rec=fct-vbcc7bad@2
name: Bounded compare of a shared target
rule: Comparing a target to known bytes reads no more than one byte past their length, never the whole file.
guard: replace_private_shared_reads_no_more_than_one_byte_past_the_bytes
@row_add n=968 row=decision/hook-unknown-fields-tolerated area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Unknown hook payload fields tolerated
rule: Unknown fields in a hook payload are counted, never fatal.
@row_add n=969 row=decision/dialog-answer-mapping area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Dialog answer mapping
rule: Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny.
@row_add n=970 row=decision/claude-env-strip area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Agent identity environment stripped
rule: Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names.
guard: an R8 test against a literal copy of the floor, never imported from the product
@row_add n=971 row=contract/dialog-data/null-details area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: dialog and Stop event data
item: description, preview and background_tasks
promise: A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array.
@row_add n=972 row=decision/version-gate-needs-all-relied-rows area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Version gate needs every relied row
rule: A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified.
guard: stamps_verdict_needs_the_dialog_rows
@row_add n=973 row=decision/agents-parser-fails-to-unknown area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Agents list parse failure reads unknown
rule: An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
@row_add n=974 row=contract/prompt-submitted/origin-harness area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: the prompt-submitted event
item: origin harness
promise: A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human.
@row_add n=975 row=decision/screen-signatures-only area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Screen read only for compiled signatures
rule: The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
@row_add n=976 row=contract/readiness-gate/gate-max-wait area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: the readiness gate
item: GATE_MAX_WAIT
promise: On a verified CLI a quiet screen with no input-box literal waits up to 8 500 ms and reads input-not-ready from that bound on.
@row_add n=977 row=decision/vt100-feed-panic-safe area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Screen feed panic-safe
rule: The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready.
@row_add n=978 row=decision/mcp-stdin-line-cap area=unit-test-strategy rec=fct-vbsipq2o@1
name: MCP stdin line cap
rule: The MCP server's stdin yields whole lines of at most MAX_FRAME bytes; a longer line releases no byte, sets the cut flag and reads as end of stream, ending a session before initialize as the initialize error.
guard: capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more
@row_add n=979 row=contract/mcp-reply/screened-words area=unit-test-strategy rec=fct-vbsipq2o@1
boundary: MCP tool replies
item: refusal and detail words
promise: A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
@row_add n=980 row=decision/unknown-refusal-word-stays-visible area=unit-test-strategy rec=fct-id225cf7@2
name: Unknown refusal word stays visible
rule: A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit.
guard: reply_of_reads_the_three_shapes
@row_add n=981 row=contract/cli-arguments/json-flag area=unit-test-strategy rec=fct-id225cf7@2
boundary: CLI arguments
item: --json
promise: The --json flag is read from the arguments only before a `--` separator.
@row_add n=982 row=decision/budget-decision-takes-the-instant area=unit-test-strategy rec=fct-id225cf7@2
name: Budget decision takes the instant as an argument
rule: The budget decision is a pure function that takes the wall-clock instant as an argument, so its thresholds and override expiry are unit-tested with no clock seam and no mocked instant.
guard: gate_decide_*
@row_add n=983 row=decision/send-confirmation-exact-match area=unit-test-strategy rec=fct-id225cf7@2
name: Send confirmation by exact normalised match
rule: A send is confirmed only when the submitted prompt text equals the typed text, every CR LF and lone CR read as one LF and the trailing CR and LF removed; a submitted prompt that still holds a CR or the trailing newline claims nothing.
guard: send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing
@row_add n=984 row=decision/send-confirmation-window-expiry area=unit-test-strategy rec=fct-id225cf7@2
name: Send confirmation window expiry
rule: A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted.
@row_add n=985 row=contract/channel-wait/wake-kinds area=unit-test-strategy rec=fct-id225cf7@2
boundary: the channel wait method
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
@row_add n=986 row=contract/channel-wait/timed-out area=unit-test-strategy rec=fct-id225cf7@2
boundary: the channel wait method
item: timed_out
promise: wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process.
@row_add n=987 row=decision/send-refusal-order area=unit-test-strategy rec=fct-id225cf7@2
name: send refusal order
rule: send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
@row_add n=988 row=decision/empty-text-refused-before-the-wheel area=unit-test-strategy rec=fct-id225cf7@2
name: Empty text refused before the wheel is read
rule: An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read.
guard: send_empty_text_is_refused_first_after_control_character
@row_add n=989 row=decision/answer-refusal-order area=unit-test-strategy rec=fct-id225cf7@2
name: answer refusal order
rule: answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog.
@row_add n=990 row=decision/human-output-control-escaping area=unit-test-strategy rec=fct-id225cf7@2
name: Human output escapes control characters
rule: Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
@row_add n=991 row=decision/list-row-control-escaping area=unit-test-strategy rec=fct-id225cf7@2
name: list rows show every control character as hex
rule: In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text.
guard: escape_row_shows_every_control_character_as_hex_text
@row_add n=992 row=decision/dialog-hold-read-once area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog hold read once at start
rule: The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection.
guard: read_hold_falls_back_to_sixty_with_one_rejection
@row_add n=993 row=decision/dialog-hold-expiry area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog hold expiry
rule: An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line.
guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line
@row_add n=994 row=decision/pending-dialog-before-its-line area=unit-test-strategy rec=fct-id225cf7@2
name: Pending dialog on the snapshot before its line
rule: A held dialog is on the snapshot before its line is logged, and is cleared when the append fails.
guard: hook_dialog_append_that_failed_clears_the_pending_dialog
@row_add n=995 row=decision/dialog-timeouts-outlast-the-hold area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog timeouts outlast the longest hold
rule: The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold.
guard: hooks_json_dialog_timeouts_outlast_the_longest_hold
@row_add n=996 row=decision/scoped-capture-pinned-dispatch area=unit-test-strategy rec=fct-id225cf7@2
name: Scoped log captures through a pinned dispatch
rule: Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default.
why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost
guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
@row_add n=997 row=decision/one-global-tracing-install area=unit-test-strategy rec=fct-id225cf7@2
name: One global tracing install
rule: The binary installs a global tracing default in exactly one place, the gate.
@row_add n=998 row=decision/server-checked-before-any-frame area=unit-test-strategy rec=fct-id225cf7@2
name: Server checked before any frame
rule: Every client verb and the hook refuse an unverified server inside their own span with no frame written.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span
@row_add n=999 row=contract/unreachable-log-cause/instance-dead area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: instance-dead
promise: Logged during connect.
@row_add n=1000 row=contract/unreachable-log-cause/unwrapped-name area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: unwrapped-name
promise: Logged during connect.
@row_add n=1001 row=contract/unreachable-log-cause/endpoint-lost area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: endpoint-lost
promise: Logged during call.
@row_add n=1002 row=contract/unreachable-log-cause/server-verify-failed area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: server-verify-failed
promise: Logged during connect.
@row_add n=1003 row=contract/unreachable-log-cause/socket-dir-refused area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: socket-dir-refused
promise: Logged during connect when the socket directory is refused.
@row_add n=1004 row=decision/socket-dir-checked-before-any-frame area=unit-test-strategy rec=fct-id225cf7@2
name: Socket directory checked before any frame
rule: A loosened socket directory is refused inside the verb's or the hook's span before any frame is written.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_add n=1005 row=contract/human-hint-cause/not-running area=unit-test-strategy rec=fct-id225cf7@2
boundary: the human hint cause
item: not-running
promise: The hint cause of an instance-dead unreachable is not-running, a word apart from the log's.
@row_add n=1006 row=contract/human-hint-cause/socket-dir area=unit-test-strategy rec=fct-id225cf7@2
boundary: the human hint cause
item: socket-dir
promise: The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused.
@row_add n=1007 row=decision/hook-dialog-no-frame-to-impostor area=unit-test-strategy rec=fct-id225cf7@2
name: Hook writes no frame to an impostor server
rule: The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper.
guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_add n=1008 row=decision/session-end-direct-append area=unit-test-strategy rec=fct-id225cf7@2
name: SessionEnd falls back to a direct append
rule: A SessionEnd hook whose server check fails appends the session end directly.
@row_add n=1009 row=contract/link-method/link-twice area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: link of a linked pair
promise: Linking a pair twice writes one line and answers the same since.
@row_add n=1010 row=contract/link-method/unlink-then-link area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: unlink then link
promise: An unlink then a link takes a later since.
@row_add n=1011 row=contract/link-method/unlink-absent-pair area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: unlink of an absent pair
promise: Unlinking an absent pair writes nothing.
@row_add n=1012 row=contract/link-method/bad-pair area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: bad pair
promise: A bad pair is refused with invalid_params.
@row_add n=1013 row=contract/link-method/from area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: from
promise: A from on a link or unlink changes nothing.
@row_add n=1014 row=contract/link-method/append-failure area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: failed append
promise: An append that fails answers internal with the link set unchanged.
@row_add n=1015 row=contract/link-verb-line/since area=unit-test-strategy rec=fct-id225cf7@2
boundary: the link verb line
item: since
promise: The line's time is the reply's since as UTC clock time; a since that is no RFC 3339 instant prints escaped.
@row_add n=1016 row=decision/diagnostics-checked-before-narrowing area=unit-test-strategy rec=fct-id225cf7@2
name: Diagnostics path checked before narrowing
rule: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; the writer goes to a sink and a detail line is dropped.
guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
@row_add n=1017 row=decision/bounded-readers-keep-the-cap area=unit-test-strategy rec=fct-id225cf7@2
name: Bounded readers keep the frame cap
rule: The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
guard: the BOUNDARIES rows of contract_bounded_readers
@row_add n=1018 row=contract/e2e-home-refusal/message area=unit-test-strategy rec=fct-msk3nz6p@1
boundary: the e2e-home headroom refusal
item: message
promise: e2e-home backing: {free} MiB free, below the {floor} MiB floor
@row_add n=1019 row=contract/harness-boot/default-instances area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --instance
promise: With no --instance flag the instances are overseer and builder whatever other flags are given; one or more --instance flags replace that list.
@row_add n=1020 row=contract/harness-boot/agents-mode area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --agents-mode
promise: --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records.
@row_add n=1021 row=contract/harness-boot/statusline-echo area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --statusline-echo
promise: --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var and writes no budget file.
@row_add n=1022 row=decision/harness-builds-in-its-own-target area=5-command-implementation rec=fct-zxaumenb@5
name: Harness builds into its own target dir
rule: The harness build always uses its own target dir, never the target dir the shim's cargo run uses.
why: a running harness binary could not be relinked, and a workspace-wide build re-fingerprints it
@row_add n=1023 row=decision/e2e-homes-under-target-e2e-home area=5-command-implementation rec=fct-zxaumenb@5
name: E2E homes under target/e2e-home
rule: Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only.
why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
@row_add n=1024 row=decision/e2e-home-link-base area=5-command-implementation rec=fct-zxaumenb@5
name: E2E home base may be a link
rule: A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
@row_add n=1025 row=decision/harness-scoped-path area=5-command-implementation rec=fct-zxaumenb@5
name: Harness-scoped PATH
rule: The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
@row_add n=1026 row=decision/verify-sole-stamp-writer area=5-command-implementation rec=fct-zxaumenb@5
name: verify is the only stamp writer
rule: viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
@row_add n=1027 row=decision/supervisor-ordinary-child area=5-command-implementation rec=fct-zxaumenb@5
name: Supervisor is an ordinary child
rule: The supervisor is spawned as an ordinary child with no detach flags, outlives boot, and stops only through its stop request.
@row_add n=1028 row=decision/harness-never-reads-cargo-bin-exe area=5-command-implementation rec=fct-zxaumenb@5
name: Harness never reads CARGO_BIN_EXE
rule: The harness takes its binaries from a passed binary dir and never reads CARGO_BIN_EXE_*.
why: Cargo sets those only for the root package's own integration tests
@row_add n=1029 row=decision/workspace-root-absolute area=5-command-implementation rec=fct-zxaumenb@5
name: Workspace root passed absolute
rule: The harness resolves the workspace root and passes it as an absolute path.
why: nextest runs the e2e crate's tests with that crate as the cwd
@row_add n=1030 row=decision/boot-readiness-file-probe area=5-command-implementation rec=fct-hyltxe6w@1
name: Boot readiness is a file-state probe
rule: boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation.
@row_add n=1031 row=contract/boot-readiness/process-start area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: process-start
promise: The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited.
@row_add n=1032 row=contract/boot-readiness/snapshot area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: snapshot
promise: The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
@row_add n=1033 row=contract/boot-readiness/heartbeat area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: heartbeat
promise: The heartbeat mtime is less than 5 s old; missing reads heartbeat.
@row_add n=1034 row=contract/boot-readiness/events area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: events
promise: Event lines 1 to 3 are a wheel line with cause start, then budget-gate, then session-start with source hook; missing reads events.
@row_add n=1035 row=contract/harness-boot-exit/0 area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot exit codes
item: 0
promise: Every instance is ready within the timeout.
@row_add n=1036 row=contract/harness-boot-exit/1 area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot exit codes
item: 1
promise: Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
@row_add n=1037 row=contract/harness-boot-reason/build-failed area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: build-failed
promise: One of the four boot refusal reasons.
@row_add n=1038 row=contract/harness-boot-reason/verify-failed area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: verify-failed
promise: One of the four boot refusal reasons.
@row_add n=1039 row=contract/harness-boot-reason/run-exited area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: run-exited
promise: The wrapper exited; exit_code is the wrapper's exit code verbatim.
@row_add n=1040 row=contract/harness-boot-reason/readiness-timeout area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: readiness-timeout
promise: One of the four boot refusal reasons.
@row_add n=1041 row=contract/harness-boot/timeout area=5-command-implementation rec=fct-ybixbnbx@1
boundary: harness boot
item: timeout
promise: Readiness is bounded at 20 s per instance after the build.
@row_none n=123 rec=fct-56kgfapd@1 type=S
@row_add n=1042 row=contract/harness-run/default-selection area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: default selection
promise: With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
@row_add n=1043 row=contract/harness-run/mutants-only-flags area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: --file and --package
promise: --file and --package each require --mutants.
@row_add n=1044 row=decision/test-layer-filtersets area=5-command-implementation rec=fct-hgewdmj2@6
name: Test layers by nextest filterset
rule: The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
@row_add n=1045 row=decision/no-nested-harness-in-nextest area=5-command-implementation rec=fct-hgewdmj2@6
name: No test shells out to the harness
rule: No nextest test shells out to agent-run or the harness binary, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
@row_add n=1046 row=decision/e2e-boot-model-by-cleanup area=5-command-implementation rec=fct-hgewdmj2@6
name: E2E boot model chosen by cleanup
rule: A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
@row_add n=1047 row=decision/root-wait-bound area=5-command-implementation rec=fct-hgewdmj2@6
name: Root waits bounded below the kills
rule: Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first.
why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
@row_add n=1048 row=decision/killed-wrapper-endpoint-wait area=5-command-implementation rec=fct-hgewdmj2@6
name: A killed wrapper's endpoint is gone when unconnectable
rule: After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone.
why: a killed wrapper removes nothing, so its socket file stays
@row_add n=1049 row=contract/harness-session/session-id area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness session
item: session id
promise: The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
@row_add n=1050 row=contract/harness-session/cleanup-report area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness session
item: cleanup report
promise: A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command.
@row_add n=1051 row=decision/mutation-base-whole-chunk area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation base is the whole chunk
rule: The mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending record, so every run of a pass mutates the whole chunk; an unresolved base exits 1 base-missing and no empty diff is written.
why: an unbounded search moves mid-pass, and an empty diff reads as a false zero-mutant pass
@row_add n=1052 row=decision/chunk-diff-pins-prefixes area=5-command-implementation rec=fct-hgewdmj2@6
name: Chunk diff pins its prefixes
rule: The chunk diff covers the working tree and untracked files and always pins the a/ and b/ prefixes.
why: a host's diff prefix config would write headers the classification cannot read
guard: chunk_diff_pins_its_prefixes_under_a_hostile_config
@row_add n=1053 row=decision/mutation-own-target-dir area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation run builds in its own target dir
rule: The mutation run pre-builds the root package and builds every mutant in its own target dir, copied into each scratch tree.
why: a copied default target keeps the original tree's binary paths, so tests drove the unmutated binary
@row_add n=1054 row=decision/mutation-keeps-no-test-home area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation run keeps no test home
rule: A mutation run keeps no test home, overriding the workflow's keep-homes setting.
why: every home carries a pinned copy of the viola binary
@row_add n=1055 row=decision/mutants-output-live area=5-command-implementation rec=fct-hgewdmj2@6
name: cargo-mutants output streams live
rule: cargo-mutants' output goes live to the harness's stderr with every outcome line printed, never captured.
why: a stalled or cancelled run names the mutant it stopped at
@row_add n=1056 row=decision/mutant-build-timeout area=5-command-implementation rec=fct-hgewdmj2@6
name: Fixed mutant build timeout
rule: Each mutant's build is bounded by a fixed 400 s, five times the largest measured baseline, not a multiplier.
why: a multiplier derives a sub-second bound from a sub-second baseline
@row_add n=1057 row=decision/mutation-diff-classification area=5-command-implementation rec=fct-hgewdmj2@6
name: Diffs with no mutable Rust skip cargo-mutants
rule: A diff naming no Rust path passes as no-rust-delta and one whose Rust paths are all test targets passes as test-only-rust-delta, both without reaching cargo-mutants; any other deletes the old outcomes first and is red with none.
why: cargo-mutants exits 0 and writes no outcomes on such diffs
@row_add n=1058 row=decision/mutation-verdict-by-counts area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation verdict by counts
rule: A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide.
why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing
@row_add n=1059 row=decision/host-excluded-mutants area=5-command-implementation rec=fct-hgewdmj2@6
name: Host-excluded missed mutants
rule: A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
@row_add n=1060 row=contract/harness-run/mutants-exit-reason area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: mutants-exit-<code>
promise: cargo-mutants exit 1, 4, 5, 6 or 70 fails run --mutants with reason mutants-exit-<code>.
@row_add n=1061 row=decision/mutants-tested-by-owning-package area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutants tested by their own package
rule: Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
@row_add n=1062 row=decision/members-declare-fake-agent-feature area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutated members declare fake-agent
rule: A mutable workspace member declares a fake-agent feature, a no-op where it has none.
why: cargo-mutants passes the feature to package-scoped runs, and a missing feature fails the run
@row_add n=1063 row=contract/harness-run/package-refused area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: package-refused
promise: A --package that is not viola or a member declaring fake-agent, named in lowercase ASCII, digits, - and _, exits 2 package-refused before any cargo runs.
@row_add n=1064 row=contract/coverage-failure/llvm-cov-exit area=5-command-implementation rec=fct-zxoe3ae7@2
boundary: coverage suite failure codes
item: llvm-cov-exit-N
promise: A red llvm-cov exit over green tests, such as a floor breach.
@row_add n=1065 row=contract/coverage-failure/llvm-cov-summary-missing area=5-command-implementation rec=fct-zxoe3ae7@2
boundary: coverage suite failure codes
item: llvm-cov-summary-missing
promise: The coverage summary was not written.
@row_add n=1066 row=decision/perf-verdict-belongs-to-gate area=5-command-implementation rec=fct-hr3ukgwq@1
name: Perf verdict belongs to the gate
rule: The perf arm runs only when named and reports its suite; the deadline verdict belongs to the gate's perf requirement, never to run.
@row_add n=1067 row=decision/fuzz-replay-linux-only area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz replay on Linux only
rule: Fuzz replay runs on Linux only; elsewhere it exits 2 fuzz-linux-only.
@row_add n=1068 row=decision/fuzz-toolchain-single-source area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz toolchain from one source
rule: The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root.
@row_add n=1069 row=decision/fuzz-corpus-never-vacuous area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz replay never passes vacuously
rule: A missing or empty corpus, or no target at all, fails corpus-empty, never a vacuous pass.
@row_add n=1070 row=decision/fuzz-reproducers-committed area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz corpus committed with reproducers
rule: The replay corpus is committed, seeded from recorded payloads or synthetic seeds, and every reproducer is committed with its fix; nothing is cached between CI runs.
@row_add n=1071 row=decision/nightly-fuzz-fails-on-crash area=5-command-implementation rec=fct-ciziz6dy@1
name: Nightly fuzz fails on any crash
rule: The nightly time-boxed fuzz run uploads its artifacts and fails the job on any crash.
@row_add n=1072 row=contract/harness-run/tool-missing area=5-command-implementation rec=fct-ciziz6dy@1
boundary: harness run
item: tool-missing
promise: Exits 1 with a detail naming the missing tool or file, such as cargo-fuzz.
@row_add n=1073 row=decision/live-verify-never-in-ci area=5-command-implementation rec=fct-tuv4h7zt@3
name: Live verify never in CI
rule: verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
@row_add n=1074 row=decision/local-live-runner-seam area=5-command-implementation rec=fct-tuv4h7zt@3
name: Local-live through a runner seam
rule: The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
@row_add n=1075 row=contract/local-live-suite/pass area=5-command-implementation rec=fct-tuv4h7zt@3
boundary: the local-live suite
item: pass
promise: Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
@row_add n=1076 row=contract/local-live-suite/failure-codes area=5-command-implementation rec=fct-tuv4h7zt@3
boundary: the local-live suite
item: failure codes
promise: A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
@row_add n=1077 row=contract/harness-run-exit/0 area=5-command-implementation rec=fct-dwjsmtpg@1
boundary: harness run exit codes
item: 0
promise: Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
@row_add n=1078 row=contract/harness-run-exit/non-zero area=5-command-implementation rec=fct-dwjsmtpg@1
boundary: harness run exit codes
item: non-zero
promise: Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red.
@row_add n=1079 row=contract/run-document/suites area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: suites
promise: Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names.
@row_add n=1080 row=contract/run-document/mutants area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: mutants
promise: A --mutants run adds a mutants object with tested and a verdict of counted, no-rust-delta, test-only-rust-delta, scoped or package, with the full base sha except on package.
@row_add n=1081 row=contract/run-document/host-excluded area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: host_excluded
promise: Present only when a mutant was left out: an array of name and cfg as written in source, repo-relative.
@row_add n=1082 row=contract/run-document/archived area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: archived
promise: The repo-relative run-archive dir holding this run's JUnit files and read outcomes, the newest 10 kept and never uploaded; an absent source is skipped and named on stderr.
@row_add n=1083 row=contract/run-document/reason area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: reason
promise: A document refused by a tool arm carries reason and, where one applies, detail.
@row_add n=1084 row=decision/skipped-counts-real-skips-only area=5-command-implementation rec=fct-zujg4opw@3
name: skipped counts real skips only
rule: skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted.
why: so the gate's suite-skipped breach fires only on a real skip
@row_add n=1085 row=decision/doc-examples-never-ignore area=5-command-implementation rec=fct-zujg4opw@3
name: Doc examples never use ignore
rule: A doc example that must not run uses no_run or a text fence, never ignore.
why: ignored doctests count as skipped
@row_add n=1086 row=decision/scoped-mutants-not-the-verdict area=5-command-implementation rec=fct-wglylwog@1
name: Scoped mutation is the inner fix loop
rule: A file-scoped mutation run is the inner fix loop; the counted chunk mutation run stays the verdict.
@row_none n=124 rec=fct-zs4cl5pl@1 type=S
@row_add n=1087 row=decision/status-strips-bridge-env area=5-command-implementation rec=fct-aajvmd2t@1
name: Harness status strips the bridge env
rule: Harness status runs viola list with the session PATH and with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed.
why: so the read files no role line under a host bridge's name
@row_add n=1088 row=decision/status-bounded-read area=5-command-implementation rec=fct-aajvmd2t@1
name: Harness status reads a bounded stdout
rule: Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing.
@row_add n=1089 row=contract/agent-run-status-polled-fields/name area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: name
promise: A per-item field of the status document that the agent polls.
@row_add n=1090 row=contract/agent-run-status-polled-fields/wrapped area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: wrapped
promise: A per-item field of the status document that the agent polls.
@row_add n=1091 row=contract/agent-run-status-polled-fields/liveness area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: liveness
promise: A per-item field of the status document that the agent polls.
@row_add n=1092 row=contract/agent-run-status-polled-fields/status area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: status
promise: A per-item field of the status document that the agent polls.
@row_add n=1093 row=contract/agent-run-status-polled-fields/wheel area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: wheel
promise: A per-item field of the status document that the agent polls.
@row_add n=1094 row=contract/agent-run-status-polled-fields/budget-paused area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: budget_paused
promise: A per-item field of the status document that the agent polls.
@row_add n=1095 row=contract/agent-run-status-polled-fields/dialog-pending area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: dialog_pending
promise: A per-item field of the status document that the agent polls.
@row_add n=1096 row=contract/agent-run-status-polled-fields/cli-verified area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: cli_verified
promise: A per-item field of the status document that the agent polls.
@row_add n=1097 row=contract/agent-run-status-polled-fields/skipped area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: skipped
promise: An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
@row_add n=1098 row=contract/agent-run-status-polled-fields/budget area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: budget
promise: An envelope field of the status document that the agent polls.
@row_add n=1099 row=contract/agent-run-status-exit-codes/0 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 0
promise: Exit 0 when ok is true, which is only for state ready.
@row_add n=1100 row=contract/agent-run-status-exit-codes/1 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 1
promise: Exit 1 for degraded or down; the full status document is still printed, so an agent waiting for a stale transition reads exit 1 as a state, not a harness fault.
@row_add n=1101 row=contract/agent-run-status-exit-codes/2 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 2
promise: Exit 2 on a usage error, or when no session.json exists for the id, printing {"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}.
@row_add n=1102 row=decision/cleanup-ctrl-c-repeated-until-exit area=5-command-implementation rec=fct-beydn7zm@2
name: Cleanup stops agents with a repeated Ctrl-C
rule: Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline.
why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
@row_add n=1103 row=decision/e2e-home-deletion-through-link area=5-command-implementation rec=fct-beydn7zm@2,fct-x3qmgvn2@2
name: Test-home deletion through the dev-host link
rule: On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created.
why: A boundary widening, ratified.
@row_add n=1104 row=decision/ci-keeps-test-homes area=5-command-implementation rec=fct-beydn7zm@2,fct-xifaibmw@1,fct-x3qmgvn2@2
name: CI keeps every test home until the gates ran
rule: In CI, AGENT_RUN_KEEP_HOMES=1 is set workflow-wide (only run --mutants overrides it to 0), so every rstest home and every harness session home stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
why: obs-plan's gate steps and uploads read the homes; the ephemeral runner then discards them with the job.
@row_add n=1105 row=decision/cleanup-is-idempotent area=5-command-implementation rec=fct-rttkzq3u@1
name: Idempotent cleanup
rule: Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.
@row_add n=1106 row=contract/cleanup-document/processes-gone area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup document
item: processes_gone
promise: True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.
@row_add n=1107 row=contract/cleanup-document/endpoint-gone area=5-command-implementation rec=fct-tqogk4ey@1,fct-beydn7zm@2
boundary: cleanup document
item: endpoint_gone
promise: True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.
@row_add n=1108 row=contract/cleanup-document/home-removed area=5-command-implementation rec=fct-tqogk4ey@1,fct-beydn7zm@2
boundary: cleanup document
item: home_removed
promise: True when the session home was removed; reports "kept" when homes are kept, which is not a failure.
@row_add n=1109 row=contract/cleanup-document/killed area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup document
item: killed
promise: Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup.
@row_add n=1110 row=contract/cleanup-exit-codes/1 area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup exit codes
item: 1
promise: Any false field in the cleanup document means exit 1.
@row_add n=1111 row=contract/agent-run-logs-stream/events-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: events line
promise: Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
@row_add n=1112 row=contract/agent-run-logs-stream/diag-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: diag line
promise: Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
@row_add n=1113 row=contract/agent-run-logs-stream/detail-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: detail line
promise: Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances.
@row_add n=1114 row=contract/agent-run-logs-stream/torn-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: torn line
promise: A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped.
@row_add n=1115 row=decision/panic-counts-read-role-files-only area=5-command-implementation rec=fct-mfhdbyoe@1
name: Panic counts read role files only
rule: Panic counts over the logs stream use the role files only, excluding detail files.
why: A panic line appears both in a role file and in its detail file.
@row_add n=1116 row=decision/log-assertions-via-jq-or-jaq area=5-command-implementation rec=fct-mfhdbyoe@1,fct-z43sni6d@2
name: Log assertions through jq or jaq
rule: Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
@row_add n=1117 row=decision/session-home-retention area=5-command-implementation rec=fct-xifaibmw@1
name: Session home retention
rule: Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
@row_add n=1118 row=decision/e2e-homes-under-one-root area=5-command-implementation rec=fct-xifaibmw@1
name: All test homes under target/e2e-home
rule: Every harness and rstest home lives under target/e2e-home.
why: It is the root obs-plan's CI gates and upload read.
@row_add n=1119 row=decision/ci-uploads-only-after-secret-scan area=5-command-implementation rec=fct-xifaibmw@1,fct-5z3qj22n@2,fct-47aec6c3@3
name: CI uploads only after the secret scan
rule: On CI failure the home's diagnostics, events and harness artifacts leave CI only in scan-gated uploads, after the secret scan has passed on them; there is no unscanned upload.
@row_add n=1120 row=decision/dev-host-e2e-home-on-tmpfs area=5-command-implementation rec=fct-xifaibmw@1,fct-p6be5x2e@1,fct-x3qmgvn2@2
name: Dev-host test homes on tmpfs
rule: On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged.
@row_add n=1121 row=decision/internal-harness-subcommands-off-agent-surface area=5-command-implementation rec=fct-p5mffmqj@1
name: Internal harness subcommands
rule: The internal harness subcommands are implemented in viola-harness, forwarded unchanged by the shims, and are not part of the agent's 5-command surface.
@row_add n=1122 row=decision/supervisor-spawned-only-by-boot area=5-command-implementation rec=fct-r55p5ks7@1
name: Supervisor spawned only by boot
rule: The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.
@row_add n=1123 row=contract/supervisor-exit-file/exits area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervisor-exit.json
item: exits
promise: One entry per instance: {"name":<instance>,"exit_code":<int|null>,"killed":<bool>}, inside {"v":1,"cmd":"supervise","ok":<bool>,"exits":[…]}.
@row_add n=1124 row=contract/supervisor-exit-file/stdout area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervisor-exit.json
item: stdout
promise: The supervisor's stdout is discarded; the JSON document is the file.
@row_add n=1125 row=contract/supervise-exit-codes/0 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 0
promise: Exit 0 when no child needed kill().
@row_add n=1126 row=contract/supervise-exit-codes/1 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 1
promise: Exit 1 when any child needed kill().
@row_add n=1127 row=contract/supervise-exit-codes/2 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 2
promise: Exit 2 on a usage error.
@row_add n=1128 row=decision/diagnostics-schema-validation area=5-command-implementation rec=fct-7hikjies@1
name: Schema conformance check of diagnostics
rule: Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
@row_add n=1129 row=contract/schema-check-output/failures area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: failures
promise: Each failure is {file, line, keyword}, file relative to target/e2e-home; no line content is ever printed.
@row_add n=1130 row=contract/schema-check-output/torn area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: torn
promise: The count of non-JSON lines.
@row_add n=1131 row=contract/schema-check-output/skipped-non-file area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: skipped_non_file
promise: The count of directories at a diagnostics file path, which are never read.
@row_add n=1132 row=contract/schema-check-reason/empty-scope area=5-command-implementation rec=fct-7hikjies@1,fct-mihqkrxe@2
boundary: schema-check reason
item: empty-scope
promise: No diagnostics file was found.
@row_add n=1133 row=contract/schema-check-reason/schema-unreadable area=5-command-implementation rec=fct-7hikjies@1,fct-mihqkrxe@2
boundary: schema-check reason
item: schema-unreadable
promise: A schema could not be read.
@row_add n=1134 row=contract/schema-check-exit-codes/2 area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check exit codes
item: 2
promise: Exit 2 on a usage error.
@row_add n=1135 row=decision/secret-scan-scope area=5-command-implementation rec=fct-5z3qj22n@2
name: Secret scan scope
rule: The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload.
why: The run --mutants chunk diff is repository source text that hits the scan's own needles.
except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned.
@row_add n=1136 row=decision/canary-list-single-sourced area=5-command-implementation rec=fct-5z3qj22n@2
name: Single-sourced canary list
rule: Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table.
guard: crates/viola-e2e/tests/scan_patterns.rs
@row_add n=1137 row=contract/secret-scan-class/claude-stripped area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: claude-stripped
promise: A hit of the fixed CLAUDE* canaries the root tests plant.
@row_add n=1138 row=contract/secret-scan-class/cookie area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: cookie
promise: A hit of the cookie: header, case-insensitive.
@row_add n=1139 row=contract/secret-scan-class/content-canary area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: content-canary
promise: A content canary, failing only in a home-level role file.
@row_add n=1140 row=contract/secret-scan-class/mode area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: mode
promise: Unix only: a diagnostics file that is not 0600.
@row_add n=1141 row=contract/secret-scan-class/unreadable area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: unreadable
promise: A file in scope that could not be read.
@row_add n=1142 row=contract/secret-scan-output/hits area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan output
item: hits
promise: Each hit is {file, line, offset, class}; the matched bytes are never printed.
@row_add n=1143 row=contract/secret-scan-output/report area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan output
item: report
promise: "report":"unwritten" is added when the hit file cannot be written.
@row_add n=1144 row=contract/secret-scan-hits-file/hits-json area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan hits file
item: target/secret-scan/hits.json
promise: Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run.
@row_add n=1145 row=contract/secret-scan-reason/empty-scope area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan reason
item: empty-scope
promise: Nothing in scope was found.
@row_add n=1146 row=contract/secret-scan-exit-codes/1 area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan exit codes
item: 1
promise: Exit 1 on any hit.
@row_add n=1147 row=contract/secret-scan-exit-codes/2 area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan exit codes
item: 2
promise: Exit 2 on a usage error.
@row_add n=1148 row=contract/gate-require/suites area=5-command-implementation rec=fct-47aec6c3@3,fct-mihqkrxe@2
boundary: gate --require
item: suites
promise: Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.
@row_add n=1149 row=decision/junit-copied-after-each-invocation area=5-command-implementation rec=fct-47aec6c3@3
name: JUnit report copied after each nextest invocation
rule: The run command deletes the nextest junit report before each invocation and copies it per suite right after; a report missing afterwards is an artifact-missing breach, never a copy of an earlier run's file.
why: nextest overwrites that path on every invocation.
@row_add n=1150 row=decision/mutants-survived-count area=5-command-implementation rec=fct-47aec6c3@3
name: Mutants survived count
rule: The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes.
@row_add n=1151 row=decision/missing-artifact-is-breach area=5-command-implementation rec=fct-47aec6c3@3
name: A missing artifact is a breach
rule: A gate input that is missing or unreadable is a breach, never a pass.
@row_add n=1152 row=contract/gate-breach/suite-missing area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-missing
promise: A required suite is absent from the run summary.
@row_add n=1153 row=contract/gate-breach/suite-failed area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-failed
promise: A required suite has failed > 0.
@row_add n=1154 row=contract/gate-breach/suite-skipped area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-skipped
promise: A required suite has skipped > 0.
@row_add n=1155 row=contract/gate-breach/coverage area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: coverage
promise: Coverage below a floor, each floor its own breach.
@row_add n=1156 row=contract/gate-breach/mutants area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: mutants
promise: The mutants suite has survived > 0.
@row_add n=1157 row=contract/gate-breach/perf area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: perf
promise: A perf result's max is at or over its gate; detail reads {file} max {max} >= {bound}.
@row_add n=1158 row=contract/gate-breach/artifact-missing area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: artifact-missing
promise: A required artifact is missing, each absent perf row its own breach naming that file.
@row_add n=1159 row=contract/gate-output/detail area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate output
item: detail
promise: A fixed code or a count, never file content.
@row_add n=1160 row=contract/gate-exit-codes/0 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 0
promise: Exit 0 with no breaches.
@row_add n=1161 row=contract/gate-exit-codes/1 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 1
promise: Exit 1 on any breach.
@row_add n=1162 row=contract/gate-exit-codes/2 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 2
promise: Exit 2 on a usage error.
@row_add n=1163 row=decision/ci-gate-last-step area=5-command-implementation rec=fct-47aec6c3@3,fct-da3wgwes@2
name: Gate as the last step of every test job
rule: CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
@row_add n=1164 row=decision/no-ci-mutants-gate area=5-command-implementation rec=fct-47aec6c3@3
name: No CI mutants gate
rule: No CI job gates the mutants suite.
@row_add n=1165 row=decision/pre-push-local-linux-gate area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push local Linux gate
rule: The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
@row_add n=1166 row=decision/pre-push-clean-environment area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push children run in a clean environment
rule: Every pre-push child runs under env -i with HOME from the passwd entry and a PATH of constants only, inheriting no host environment; the harness's own HOME is never read.
why: No CLAUDE* value crosses.
@row_add n=1167 row=contract/pre-push-stage/host area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: host
promise: Off Linux, exit 2 with pre-push-linux-only.
@row_add n=1168 row=contract/pre-push-stage/tools area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: tools
promise: Checks the passwd home, cc, and rustc, cargo-nextest, cargo-mutants and cargo-llvm-cov at their pinned versions.
@row_add n=1169 row=contract/pre-push-stage/linux-tests area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: linux-tests
promise: Runs run --coverage then gate --require coverage,doctest; the last stage, ok:true when green.
@row_add n=1170 row=decision/pre-push-runs-no-mutation area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push runs no mutation
rule: No pre-push stage runs mutation testing.
why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests.
@row_add n=1171 row=contract/pre-push-output/paths area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push output
item: paths
promise: Codes, counts and repo-relative names only, never a home or repository path.
@row_add n=1172 row=contract/pre-push-exit-codes/0 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 0
promise: Exit 0 when every stage is green.
@row_add n=1173 row=contract/pre-push-exit-codes/1 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 1
promise: Exit 1 on a red stage or a stop reason.
@row_add n=1174 row=contract/pre-push-exit-codes/2 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 2
promise: Exit 2 on pre-push-linux-only or a usage error.
@row_add n=1175 row=decision/closed-enums-need-decisions-log area=5-command-implementation rec=fct-mihqkrxe@2,fct-k3mrcwuf@1
name: Closed enums change only by decision
rule: A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
@row_add n=1176 row=contract/run-suite/local-live area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run suite
item: local-live
promise: Appears only under --local-live and is not accepted by gate --require.
@row_add n=1177 row=contract/local-live-failure/build area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: build
promise: A closed local-live failure code.
@row_add n=1178 row=contract/local-live-failure/verify-exit-n area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: verify-exit-<n>
promise: A closed local-live failure code.
@row_add n=1179 row=contract/local-live-failure/verify-exit-none area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: verify-exit-none
promise: A closed local-live failure code.
@row_add n=1180 row=contract/local-live-failure/row-missing area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: row-missing
promise: A closed local-live failure code.
@row_add n=1181 row=contract/boot-readiness-missing/run-process-start area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:run-process-start
promise: The run process start record of an instance is missing.
@row_add n=1182 row=contract/boot-readiness-missing/child-process-start area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:child-process-start
promise: The child process start record of an instance is missing.
@row_add n=1183 row=contract/boot-readiness-missing/snapshot area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:snapshot
promise: The snapshot of an instance is missing.
@row_add n=1184 row=contract/boot-readiness-missing/endpoint area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:endpoint
promise: The endpoint of an instance is missing.
@row_add n=1185 row=contract/boot-readiness-missing/heartbeat area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:heartbeat
promise: The heartbeat of an instance is missing.
@row_add n=1186 row=contract/boot-readiness-missing/events area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:events
promise: The events of an instance are missing.
@row_add n=1187 row=contract/status-last-error/null area=5-command-implementation rec=fct-mihqkrxe@2,fct-cz5fdkae@1
boundary: status last_error
item: null
promise: No error.
@row_add n=1188 row=contract/status-last-error/list-exit-code area=5-command-implementation rec=fct-mihqkrxe@2,fct-cz5fdkae@1,fct-kr53tti7@1
boundary: status last_error
item: list-exit-<code>
promise: viola list exited with that code (for example list-exit-21); list-exit-0 when it exited 0 with no JSON object on stdout.
@row_add n=1189 row=contract/status-last-error/list-exit-none area=5-command-implementation rec=fct-mihqkrxe@2,fct-kr53tti7@1
boundary: status last_error
item: list-exit-none
promise: viola list gave no exit code, could not be spawned, or had no supervise record to run from.
@row_add n=1190 row=contract/mutants-verdict/scoped area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run --mutants verdict
item: scoped
promise: The run was scoped by --file.
@row_add n=1191 row=contract/run-reason/fuzz-linux-only area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: fuzz-linux-only
promise: Exit 2.
@row_add n=1192 row=contract/run-reason/tool-missing area=5-command-implementation rec=fct-mihqkrxe@2,fct-z43sni6d@2
boundary: run reason
item: tool-missing
promise: run --perf and --fuzz-replay check their tool is on PATH first; when missing they exit 1 with this reason and the tool name, never a silent pass.
@row_add n=1193 row=contract/run-reason/corpus-empty area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: corpus-empty
promise: A closed run reason.
@row_add n=1194 row=contract/run-reason/live-in-ci area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: live-in-ci
promise: Exit 2.
@row_add n=1195 row=contract/run-reason/package-refused area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: package-refused
promise: Exit 2.
@row_add n=1196 row=contract/run-usage-detail/unknown-suite area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run usage detail
item: unknown-suite
promise: The closed usage detail for a suite value outside the closed list.
@row_add n=1197 row=contract/pre-push-reason/pre-push-linux-only area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: pre-push-linux-only
promise: Exit 2.
@row_add n=1198 row=contract/pre-push-reason/tool-missing area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: tool-missing
promise: Reused from run; its detail names the missing tool or passwd-home.
@row_add n=1199 row=contract/pre-push-reason/tool-pin-mismatch area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: tool-pin-mismatch
promise: A closed pre-push reason.
@row_add n=1200 row=contract/pre-push-reason/linux-document-unreadable area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: linux-document-unreadable
promise: The detail, run or gate, names the harness command whose document was unreadable.
@row_add n=1201 row=contract/pre-push-detail/passwd-home area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: passwd-home
promise: Detail of tool-missing.
@row_add n=1202 row=contract/pre-push-detail/cc area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cc
promise: A closed pre-push detail.
@row_add n=1203 row=contract/pre-push-detail/rustc area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: rustc
promise: A closed pre-push detail.
@row_add n=1204 row=contract/pre-push-detail/cargo-nextest area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-nextest
promise: A closed pre-push detail.
@row_add n=1205 row=contract/pre-push-detail/cargo-mutants area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-mutants
promise: A closed pre-push detail.
@row_add n=1206 row=contract/pre-push-detail/cargo-llvm-cov area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-llvm-cov
promise: A closed pre-push detail.
@row_add n=1207 row=contract/pre-push-detail/pins-unreadable area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: pins-unreadable
promise: A closed pre-push detail.
@row_add n=1208 row=contract/pre-push-detail/run area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: run
promise: The run document was unreadable.
@row_add n=1209 row=contract/pre-push-detail/gate area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: gate
promise: The gate document was unreadable.
@row_add n=1210 row=decision/ci-profile-no-retries area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5,fct-da3wgwes@2
name: CI test profile without retries
rule: The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
@row_add n=1211 row=decision/verify-window-60s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 60 s kill for the verify_window_ class
rule: Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
why: Their cases wait out the gate's 8.5 s maximum several times by design.
guard: mutants_verify_window_override_stands_before_the_harness_package_override
@row_add n=1212 row=decision/verify-driven-tests-20s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Per-test kill bounds verify-driven tests
rule: A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound.
why: Ratified; sized from the measured verify floor plus a 3x runner tail.
@row_add n=1213 row=decision/within-7s-wait-bound area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: WITHIN bound for every other wait
rule: WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.
@row_add n=1214 row=decision/mutants-fail-fast-wait area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Mutants profile stops after one failure and waits
rule: The mutants nextest profile stops scheduling at the first failure and lets the tests already running finish (terminate = wait).
why: Running tests then drop their temp dirs and session guards; immediate termination left leftover temp dirs.
@row_add n=1215 row=decision/mutants-kill-below-cargo-mutants-timeout area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Kill lines below cargo-mutants' timeout
rule: Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout.
why: At or above that floor a hang grades Timeout instead of caught.
guard: test_deadlines_sit_below_the_nextest_kill_line
@row_add n=1216 row=decision/viola-e2e-30s-mutants-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 30 s mutants kill for the harness package
rule: Under the mutants profile the viola-e2e package takes a 30 s per-test kill.
why: The harness's own tests wait out its 20 s boot deadline by design.
@row_add n=1217 row=decision/send-window-paste-hint-30s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 30 s mutants kill for send-window classes
rule: Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill.
why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint, within a second of the 10 s default kill.
guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill
@row_add n=1218 row=decision/fixture-git-no-auto-maintenance area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Fixture git repos without auto maintenance
rule: Fixture git repositories in tests run git with maintenance.auto=false.
why: The detached auto maintenance each fixture commit spawned wrote into .git while the temp dir's drop walked it, leaving half-removed dirs.
@row_add n=1219 row=decision/rmcp-client-only area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: rmcp client only in the harness
rule: The harness uses rmcp with the client feature only, without transport-child-process.
why: The test owns the viola mcp child process.
@row_add n=1220 row=decision/syn-cfg-reader-for-host-exclusion area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: syn as the cfg reader
rule: syn and proc-macro2 are ordinary harness dependencies that read #[cfg] attributes for run --mutants' host exclusion.
@row_add n=1221 row=decision/agent-run-shim-over-harness area=bootstrap-phases-derive-for-route-setup-project rec=fct-piiirk6o@2,fct-p5mffmqj@1
name: agent-run shim over viola-harness
rule: The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs.
@row_add n=1222 row=decision/status-shape-ownership area=bootstrap-phases-derive-for-route-setup-project rec=fct-ftvtdczd@1,fct-weskv4ql@1,fct-4npcxfnu@1,fct-ahlkrnje@1
name: Ownership of the status shape
rule: The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.
@row_add n=1223 row=decision/harness-log-fields-owned-by-test-plan area=bootstrap-phases-derive-for-route-setup-project rec=fct-yu57xy4j@1,fct-weskv4ql@1,fct-ahlkrnje@1,fct-k3mrcwuf@1
name: Ownership of the harness-grepped log fields
rule: The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first.
why: The harness greps logs for assertions, so a format break is a harness break.
@row_add n=1224 row=decision/cargo-llvm-cov-coverage area=bootstrap-phases-derive-for-route-setup-project rec=fct-qacavs3x@1
name: cargo-llvm-cov for coverage
rule: cargo-llvm-cov 0.9.1, installed through taiki-e/install-action, produces coverage as lcov and a JSON summary.
@row_add n=1225 row=decision/ci-tools-version-pinned area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: Version-pinned CI tools
rule: Every CI-invoked binary is version-pinned.
@row_add n=1226 row=decision/g2-zero-panics area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: G2 zero panics
rule: CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs.
except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.
@row_add n=1227 row=decision/runner-provided-jq area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: Runner-provided jq in CI scripts
rule: CI scripts use the runner-provided jq, presence-checked and never installed, and refuse with tool-missing: jq without it.
@row_add n=1228 row=decision/ripgrep-verified-install area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: ripgrep for G1 and G3, checksum-verified
rule: ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256.
why: taiki-e/install-action has no ripgrep manifest.
@row_add n=1229 row=decision/ci-actions-sha-pinned area=bootstrap-phases-derive-for-route-setup-project rec=fct-da3wgwes@2
name: SHA-pinned actions, no default permissions
rule: Every CI action is SHA-pinned, with permissions: {} at the workflow's top level.
@row_add n=1230 row=decision/ci-exit-code-gates area=bootstrap-phases-derive-for-route-setup-project rec=fct-da3wgwes@2
name: CI exit-code gates
rule: CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
@row_add n=1231 row=contract/log-format/json-per-line area=log-format rec=fct-ehp4mr57@1,fct-gxb3q2be@1
boundary: log format
item: line format
promise: JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
@row_add n=1232 row=contract/log-streams/event-stream area=log-format rec=fct-ehp4mr57@1
boundary: log streams
item: event stream
promise: Product events in each instance's events.ndjson, exactly the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.
@row_add n=1233 row=decision/process-logs-via-tracing-subscriber area=log-format rec=fct-ehp4mr57@1
name: tracing-subscriber JSON process logs
rule: Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
@row_add n=1234 row=decision/codes-only-role-logs area=log-format rec=fct-ehp4mr57@1
name: Codes-only role logs, detail apart
rule: Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
@row_add n=1235 row=contract/log-files/modes area=log-format rec=fct-ehp4mr57@1
boundary: log files
item: modes and writes
promise: Files are 0600, dirs 0700, with one write per line.
@row_add n=1236 row=contract/log-line/timestamp area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=1237 row=contract/log-line/level area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_add n=1238 row=contract/log-line/target area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: target
promise: A required field.
@row_add n=1239 row=contract/log-line/message area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: message
promise: fields.message, or the flattened message; required.
@row_add n=1240 row=contract/log-line/event area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: event
promise: A closed kebab-case enum; each value fixes what its corr holds.
@row_add n=1241 row=contract/log-line/process area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
@row_add n=1242 row=contract/log-line/instance area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: instance
promise: A ViolaName or null.
@row_add n=1243 row=contract/log-line/corr area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
@row_add n=1244 row=contract/log-line/null-encoding area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: null encoding
promise: A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
@row_add n=1245 row=contract/log-event-corr/channel-request area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: channel-request
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification.
@row_add n=1246 row=contract/log-event-corr/channel-response area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: channel-response
promise: corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).
@row_add n=1247 row=contract/log-event-corr/dialog-raised area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: dialog-raised
promise: corr is the dialog_id.
@row_add n=1248 row=contract/log-event-corr/dialog-answered area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: dialog-answered
promise: corr is the dialog_id.
@row_add n=1249 row=contract/log-event-corr/hook-invoked area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
@row_add n=1250 row=contract/log-event-corr/hook-decision area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.
@row_add n=1251 row=contract/log-event-corr/send-issued area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-issued
promise: corr is the send cursor.
@row_add n=1252 row=contract/log-event-corr/send-confirmed area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-confirmed
promise: corr is the send cursor.
@row_add n=1253 row=contract/log-event-corr/send-refused area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-refused
promise: corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
@row_add n=1254 row=contract/log-line/conn-and-rpc-id area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: conn, rpc_id
promise: Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
@row_add n=1255 row=contract/log-event-corr/release-from-driver area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: release-from-driver
promise: corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.
@row_add n=1256 row=contract/log-event-corr/process-start area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: process-start
promise: corr is null.
@row_add n=1257 row=contract/log-event-corr/process-exit area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: process-exit
promise: corr is null.
@row_add n=1258 row=contract/log-event-corr/panic area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: panic
promise: corr is null.
@row_add n=1259 row=contract/log-event-corr/state-recovered area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: state-recovered
promise: corr is null.
@row_add n=1260 row=contract/log-event-corr/parse-rejected area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: parse-rejected
promise: corr is null.
@row_add n=1261 row=decision/diag-line-schema-requires-corr area=log-format rec=fct-k3mrcwuf@1
name: Schema requires corr where always defined
rule: The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
@row_add n=1262 row=decision/hook-trace-write-failure-swallowed area=log-format rec=fct-fqmaxw56@1
name: Hook trace write failures swallowed
rule: A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
@row_add n=1263 row=contract/log-line/hook-panic area=log-format rec=fct-gxb3q2be@1
boundary: process log line
item: hook panic
promise: A panic in a hook is caught and logged as one level ERROR, event panic line.
@row_add n=1264 row=decision/no-product-pid-file area=pid-file rec=fct-pskra4yu@1
name: No product pid file
rule: The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json.
@row_add n=1265 row=contract/session-json/supervisor-pid area=pid-file rec=fct-pskra4yu@1
boundary: harness session.json
item: supervisor_pid
promise: The supervisor's pid.
@row_add n=1266 row=contract/session-json/wrapper-pid area=pid-file rec=fct-pskra4yu@1
boundary: harness session.json
item: wrapper_pid
promise: The wrapper's pid.
@row_add n=1267 row=contract/session-json/lifecycle area=pid-file rec=fct-ooubxlv3@1,fct-iwx6e4zn@1
boundary: harness session.json
item: lifecycle
promise: Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup.
@row_add n=1268 row=decision/pid-start-time-verified-kill area=pid-file rec=fct-ooubxlv3@1,fct-iwx6e4zn@1
name: Pid and start time verified before a kill
rule: Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling.
why: A reused pid is never killed.
@row_add n=1269 row=contract/status-document/list area=status-endpoint-shape rec=fct-wemcdo6l@1,fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: list
promise: The parsed viola list --json document verbatim, the exit-21 error document included; null when list exits 0 with no JSON object on stdout.
@row_add n=1270 row=contract/status-document/ok area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: ok
promise: True only for state ready.
@row_add n=1271 row=contract/status-document/pid area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: pid
promise: Stays null.
@row_add n=1272 row=contract/status-document/uptime-ms area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: uptime_ms
promise: Stays null.
@row_add n=1273 row=contract/status-document/instances area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: instances
promise: One entry per booted instance: name, wrapper_pid, and alive by pid and start-time liveness.
@row_add n=1274 row=contract/status-state/ready area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: ready
promise: viola list exits 0 and every booted instance is an item wrapped:true, liveness:"live"; an unwrapped item is ignored, and a session that booted none reads ready.
@row_add n=1275 row=contract/status-state/degraded area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: degraded
promise: A booted instance is stale or absent from the list, or the document holds no items.
@row_add n=1276 row=contract/status-state/down area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: down
promise: viola list exits non-zero, gives no exit code, cannot be spawned, has no supervise record to run from, or exits 0 with no JSON object on stdout.
@row_add n=1277 row=decision/rstest-fixture-chain area=test-data-bootstrap rec=fct-p6be5x2e@1
name: rstest fixture chain
rule: Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
@row_add n=1278 row=decision/fake-agent-replay-for-stamps area=test-data-bootstrap rec=fct-p6be5x2e@1
name: Fake-agent replay for stamps
rule: Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
@row_add n=1279 row=decision/proptest-committed-seeds area=test-data-bootstrap rec=fct-p6be5x2e@1
name: proptest with committed seeds
rule: Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.
@row_add n=1280 row=decision/chaos-home-outside-scan area=test-data-bootstrap rec=fct-p6be5x2e@1,fct-x3qmgvn2@2
name: One chaos home outside the scans
rule: Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design.
why: A ratified carve-out.
@row_add n=1281 row=decision/e2e-home-base-checks area=test-data-bootstrap rec=fct-p6be5x2e@1
name: Checked base of the test homes
rule: The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
@row_add n=1282 row=decision/test-home-owner-record-sweep area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Owner-record sweep of test homes
rule: Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record.
why: A test the runner kills never drops its home.
@row_add n=1283 row=decision/owner-record-removed-last area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Owner record removed last
rule: Removing a test home deletes its owner record last.
why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
@row_add n=1284 row=decision/keep-failed-home-opt-in area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Failed test homes kept only on request
rule: A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1.
why: So that logs can inspect the home.
@row_none n=125 rec=fct-aet5iavj@1 type=S
@row_none n=126 rec=fct-aqbt7wnm@1 type=S
@row_none n=127 rec=fct-n3ryxatt@1 type=S
@row_none n=128 rec=fct-b5skp7pn@1 type=S
@row_none n=129 rec=fct-6ihplwew@1 type=5
@row_none n=130 rec=fct-jjhahrts@1 type=S
@row_none n=131 rec=fct-fje2sz6m@1 type=S
@row_add n=1285 row=decision/real-claude-local-only area=1-test-scope-summary rec=fct-wcpph3y4@2,fct-upolcsa3@1,fct-tfd2itkl@1,fct-dpjzmvrb@1,fct-6ugas3oa@1,fct-7bwho3ei@1
name: Real claude CLI tested local-only
rule: Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
@row_add n=1286 row=contract/viola-hook/stdout area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2,fct-627s4o4i@1
boundary: viola hook exec-form entry points
item: stdout
promise: stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.
@row_add n=1287 row=decision/sync-crates-without-tokio area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-hlx2zkbt@2,fct-7h3hdkfi@1,fct-ys3szg34@1,fct-u7ux54l5@1
name: Sync crates build without tokio
rule: Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build.
guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
@row_add n=1288 row=decision/release-build-viola-only area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-see5mjkn@1,fct-thnlptgc@1,fct-2qyicniz@2
name: Release build carries viola only
rule: The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step.
guard: release-check.sh and its probe
@row_add n=1289 row=decision/sha-pinned-actions area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-r5xptmoc@1,fct-qgvb7uk6@1
name: SHA-pinned Actions
rule: Every Action a workflow uses is SHA-pinned, never referenced by a mutable tag.
guard: zizmor unpinned-uses
@row_add n=1290 row=contract/viola-list-json/budget area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: budget
promise: The ok payload's budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or "unknown".
@row_add n=1291 row=contract/viola-list-json/skipped area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: skipped
promise: The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
@row_add n=1292 row=decision/logs-carry-no-secrets area=1-test-scope-summary rec=fct-yp3slbux@1,fct-gjahavf7@1
name: Logs carry no secrets
rule: No log contains the Cookie header or a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line.
guard: viola-harness secret-scan
@row_add n=1293 row=contract/instance-snapshot/pid area=1-test-scope-summary rec=fct-w7y4wg56@1,fct-4khqx2uf@2
boundary: instance snapshot.json
item: pid
promise: The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
@row_add n=1294 row=decision/self-bootstrapping-test-data area=1-test-scope-summary rec=fct-yeykk4zl@1,fct-s424obzs@1
name: Self-bootstrapping test data
rule: Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
@row_add n=1295 row=decision/human-never-blocked area=1-test-scope-summary rec=fct-nzaflfcl@1,fct-imixgk6n@1,fct-3oyxjfyz@1
name: The human is never blocked
rule: The human is never blocked: human typing still reaches the agent while automation is refused.
@row_add n=1296 row=contract/viola-verify/stamped-line area=1-test-scope-summary rec=fct-yl2sew2w@1,fct-v74hzzdp@1
boundary: viola verify
item: stamped line
promise: viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
@row_add n=1297 row=contract/mcp-stdin/oversize-line area=1-test-scope-summary rec=fct-bia7axqu@1,fct-fbkvffnw@2,fct-elurmfjv@1
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it.
@row_add n=1298 row=decision/leading-slash-never-a-prompt area=1-test-scope-summary rec=fct-engo5ir7@2,fct-lzdm6e7w@6
name: Leading-slash argument never a prompt
rule: A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered.
guard: cli_slash_args
@row_add n=1299 row=decision/tailing-ignores-symlinks area=1-test-scope-summary rec=fct-55p24js3@1,fct-bt3npe7x@2
name: Tailing ignores symlinks and invalid names
rule: Tailing ignores symlinks and invalid names, and an over-long ndjson line is counted in skipped.torn_lines.
@row_add n=1300 row=decision/vt100-panic-contained area=1-test-scope-summary rec=fct-vzvcqlaz@1,fct-47fi4ugg@2
name: vt100 panic contained
rule: A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
@row_add n=1301 row=decision/linux-only area=1-test-scope-summary rec=fct-5akis2u7@3,fct-ze6ljnvw@1
name: Linux only
rule: The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=1302 row=contract/cli-exit-codes/14 area=1-test-scope-summary rec=fct-bpxqtijx@1,fct-fn3ybwy5@2,fct-id225cf7@2
boundary: CLI exit codes
item: 14
promise: A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
@row_add n=1303 row=decision/no-secrets-in-logs area=1-test-scope-summary rec=fct-kpze4wbb@1,fct-fqmaxw56@1
name: No secrets in logs or diagnostics
rule: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0600.
guard: the secret-scan test
@row_add n=1304 row=decision/mutation-at-epoch-boundary area=1-test-scope-summary rec=fct-4ngbveqo@1,fct-cv73iuuc@1,fct-cz5jn5fs@2
name: Mutation testing at the epoch boundary
rule: cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
@row_add n=1305 row=decision/rstest-case-tables area=2-test-strategy rec=fct-njccdctv@2,fct-74ah5yxk@1,fct-gbcioyfo@1,fct-7osf2h3d@2
name: Table-driven unit cases in rstest
rule: Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label.
except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
why: so no manifest edge is added for rstest alone
@row_add n=1306 row=decision/unit-tests-inline area=2-test-strategy rec=fct-njccdctv@2,fct-ks7lvtf7@1,fct-7osf2h3d@2,fct-hckuzazj@4
name: Inline unit tests
rule: Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
@row_add n=1307 row=decision/integration-suite-per-contract area=2-test-strategy rec=fct-4ignlk4w@2,fct-hlf5klew@2
name: Integration suite for every standard contract
rule: Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
@row_add n=1308 row=decision/hook-deadlines-gated-by-hyperfine area=2-test-strategy rec=fct-egpbbwom@1,fct-zx23mmpw@2
name: Hook deadlines gated by hyperfine
rule: hyperfine gates the hook deadlines on max.
@row_add n=1309 row=decision/fixtures-schema-checked area=2-test-strategy rec=fct-xagti75h@1,fct-vekwchud@2
name: Recorded fixtures checked against their schema
rule: jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
@row_add n=1310 row=decision/complete-lines-reader area=2-test-strategy rec=fct-vvnlh7sf@1,fct-hckuzazj@4
name: Live-appended files read by complete lines
rule: Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader.
why: a Linux reader can observe an append part-way
@row_add n=1311 row=decision/integration-binary-surface-prefix area=2-test-strategy rec=fct-cyzxt2op@1
name: Integration binaries prefixed by surface
rule: Rust integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer.
why: so nextest filtersets can select layers
@row_add n=1312 row=decision/fixture-named-by-hook-event area=2-test-strategy rec=fct-cyzxt2op@1
name: Fixtures named by hook event
rule: Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
@row_add n=1313 row=decision/test-function-naming area=2-test-strategy rec=fct-ux4bzdth@1,fct-jzgpms4i@1
name: Test function naming
rule: Test functions are named <subject>_<condition>_<expected> in snake_case.
except: critical-path scenarios are named path<N>_<slug>
@row_add n=1314 row=decision/nextest-test-runner area=4-unit-test-strategy rec=fct-uptme7ma@1,fct-2j6rht5l@5
name: cargo-nextest runs the tests
rule: Tests run on libtest through cargo-nextest, process per test, with JUnit output.
except: Doctests run through cargo test, because nextest cannot run them.
@row_add n=1315 row=absence/database area=5-integration-test-strategy rec=fct-rhuz6eg7@1,fct-hctsd5xl@2
capability: A database
@row_add n=1316 row=absence/outbound-http area=5-integration-test-strategy rec=fct-jnilykpz@2,fct-yqmk3k5s@1,fct-qta73xkp@1,fct-b4mgg72y@2,fct-iyneo4vq@1,fct-pwscsozw@2
capability: Outbound HTTP
@row_add n=1317 row=decision/fake-agent-as-claude area=5-integration-test-strategy rec=fct-jnilykpz@2,fct-b4mgg72y@2
name: Fake agent serves the claude boundary
rule: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
@row_add n=1318 row=decision/fresh-home-per-test area=5-integration-test-strategy rec=fct-wwnk7u4j@2,fct-qmrx4mno@2,fct-p6be5x2e@1,fct-ea2dd5jv@1
name: Fresh home per test
rule: Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests.
why: so obs-plan's CI gates, the secret scan and the upload cover integration runs
except: The chaos feed panic test alone boots outside the scans.
@row_add n=1319 row=contract/wrapper-channel/cursor area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-ksdwqwkf@2
boundary: the wrapper channel
item: cursor
promise: A result's cursor equals the events.ndjson length at acceptance.
@row_add n=1320 row=contract/wrapper-channel/-32602 area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-5qkdel63@2
boundary: the wrapper channel
item: -32602
promise: A request with a higher v is too new and is answered -32602 with data {supported, wrapper}.
@row_add n=1321 row=contract/wrapper-channel/control-character area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-ksdwqwkf@2
boundary: the wrapper channel
item: control-character
promise: In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.
@row_add n=1322 row=contract/event-line/ts area=5-integration-test-strategy rec=fct-zpi22xot@1,fct-bt3npe7x@2
boundary: the event line
item: ts
promise: ts is RFC 3339 UTC with ms and Z.
@row_add n=1323 row=contract/strict-modes-refusal/hook area=5-integration-test-strategy rec=fct-zpi22xot@1,fct-bt3npe7x@2
boundary: strict-modes refusal
item: hook
promise: A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
@row_add n=1324 row=contract/viola-hook/no-viola-name area=5-integration-test-strategy rec=fct-627s4o4i@1,fct-nzkgjegi@1,fct-srb3tv7d@1
boundary: viola hook
item: no VIOLA_NAME
promise: With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.
@row_add n=1325 row=decision/controls-not-disableable area=5-integration-test-strategy rec=fct-w5xcngvx@3,fct-lzdm6e7w@6
name: Controls not disableable
rule: No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them.
guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
@row_add n=1326 row=decision/tty-asserted-only-on-verb-sgr area=6-e2e-test-strategy rec=fct-gnzxb6ex@2
name: TTY rendering asserted only on verb SGR
rule: CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@row_add n=1327 row=decision/hook-deadline-driver area=6-e2e-test-strategy rec=fct-iqthlzz5@2
name: viola hook driven under an Instant deadline
rule: viola hook and hook statusline are driven by std::process::Command with an Instant deadline over recorded fixture stdin.
@row_add n=1328 row=decision/env-clear-keeps-llvm-profile-file area=6-e2e-test-strategy rec=fct-iqthlzz5@2,fct-qacavs3x@1
name: env_clear tests re-add LLVM_PROFILE_FILE
rule: Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
@row_add n=1329 row=decision/tui-passthrough-never-parses-screen area=6-e2e-test-strategy rec=fct-ktlsbc3h@2
name: Passthrough verdicts never parse the rendered screen
rule: viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen.
except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
@row_add n=1330 row=decision/endpoint-read-from-snapshot area=6-e2e-test-strategy rec=fct-uz5vlm6m@1
name: Endpoint read only from the snapshot
rule: Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
@row_add n=1331 row=decision/mcp-stdio-test-owns-child area=6-e2e-test-strategy rec=fct-xwumruas@1
name: MCP stdio test owns the child
rule: MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
@row_add n=1332 row=decision/cross-user-peers-untestable area=6-e2e-test-strategy rec=fct-lkgqv65p@3,fct-7bwho3ei@1
name: Cross-OS-user peers skipped as untestable
rule: Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
@row_add n=1333 row=contract/viola-send/empty-text area=6-e2e-test-strategy rec=fct-waypbx6k@1,fct-qbytqn26@3
boundary: viola send
item: empty-text
promise: A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
@row_add n=1334 row=contract/viola-answer/unknown-dialog area=6-e2e-test-strategy rec=fct-zjfsmr2z@1,fct-2zxurlqa@1
boundary: viola answer
item: 13 unknown-dialog
promise: An answer to an unknown dialog id exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
@row_add n=1335 row=contract/viola-answer/unverified-cli area=6-e2e-test-strategy rec=fct-v74hzzdp@1,fct-p2i2onck@2
boundary: viola answer
item: 12 unverified-cli
promise: An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
@row_add n=1336 row=decision/snapshot-replay-from-log area=6-e2e-test-strategy rec=fct-uq5phmo2@1,fct-am65hify@1,fct-47fi4ugg@2,fct-3svwp7h4@1
name: Unreadable snapshot replayed from the log
rule: Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file.
guard: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_what_the_wrapper_wrote
@row_add n=1337 row=decision/start-collision-by-heartbeat area=6-e2e-test-strategy rec=fct-uq5phmo2@1,fct-47fi4ugg@2,fct-h6d4aowm@1
name: Start collision read from the heartbeat
rule: Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not.
guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
@row_add n=1338 row=decision/hook-fails-open area=6-e2e-test-strategy rec=fct-4aawmv2h@2,fct-fbkvffnw@2
name: Hook fails open silently
rule: The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper.
guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
@row_add n=1339 row=contract/json-rpc-errors/-32603 area=6-e2e-test-strategy rec=fct-gjahavf7@1,fct-o57nkgzg@2
boundary: JSON-RPC errors
item: -32603
promise: Every -32603 carries exactly the message "internal error" and data null.
@row_add n=1340 row=decision/umask-independent-modes area=6-e2e-test-strategy rec=fct-4uc5ehox@2,fct-hhwc4gim@2
name: Umask-independent modes
rule: Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private.
guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
@row_add n=1341 row=decision/single-writer-state-files area=6-e2e-test-strategy rec=fct-4uc5ehox@2,fct-mqdaf5bf@2
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
@row_add n=1342 row=decision/frame-verbs-reach-only-the-recorded-wrapper area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Frame verbs reach only the recorded wrapper
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper.
guard: security_negatives_server_identity
@row_add n=1343 row=contract/list-json/name area=6-e2e-test-strategy rec=fct-nsd7seww@1,fct-hdfefjry@2
boundary: list --json
item: name
promise: list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
@row_add n=1344 row=decision/property-tests-on-parsers area=6-e2e-test-strategy rec=fct-advgjcvl@1,fct-akm552hs@1
name: Property tests on every parser surface
rule: proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
@row_add n=1345 row=decision/fixtures-match-tolerant-schema area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-sn6e6lxw@2
name: Fixtures checked against the tolerant schema
rule: jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
@row_add n=1346 row=decision/bounded-readers area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-sn6e6lxw@2
name: Every external reader is bounded
rule: Every product reader of external input is bounded in-statement or named in the enumeration with a witness test.
except: Reads of viola's own executable take no external input and are listed as exceptions.
guard: product_readers_are_each_bounded_or_named
@row_add n=1347 row=decision/test-homes-made-private area=7-test-data-fixtures rec=fct-54gkcbby@2,fct-ubb4ig4c@2,fct-6upjfbpd@2
name: Test-made homes are private directories
rule: A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation.
why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
@row_add n=1348 row=decision/mcp-exchange-recorded-by-tap area=7-test-data-fixtures rec=fct-auxau5hd@1,fct-beozd4ox@1
name: MCP exchange recorded by a test-only tap
rule: The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
@row_add n=1349 row=contract/fake-agent-receipt/env area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1,fct-mw3qahin@1
boundary: viola-fake-agent receipt
item: env
promise: An env receipt holds variable names, sorted, names only, never a value.
@row_add n=1350 row=decision/mockall-on-seam-traits-only area=8-mocking-stubbing-discipline rec=fct-76bthv7a@1,fct-3a3m4jll@1,fct-sik7tvk4@2
name: mockall doubles on seam traits only
rule: Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
@row_add n=1351 row=decision/no-orphan-modules area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1,fct-pk32flwi@2
name: No orphan modules
rule: Any cargo-modules orphan in any library or binary target fails the build.
guard: orphans-check.sh with its probe of two planted orphans
@row_add n=1352 row=decision/mutation-gate-thresholds area=10-quality-gates-coverage-targets rec=fct-xnmc6vwq@1,fct-ocdr3xdn@3
name: Mutation gate thresholds
rule: A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out.
why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.
@row_add n=1353 row=decision/every-surface-asserted area=11-test-anti-patterns-never-do-these rec=fct-3wg6nwel@1
name: Every surface of a critical path asserted
rule: A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
@row_add n=1354 row=decision/env-clear-keeps-profile-file area=11-test-anti-patterns-never-do-these rec=fct-wxsgpjht@1,fct-ennvbmje@2
name: Cleared environment keeps LLVM_PROFILE_FILE
rule: A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`.
why: Without it the run silently drops out of coverage.
@row_add n=1355 row=decision/compact-confirm-window area=e2e-test-strategy rec=fct-qbytqn26@3,fct-lhvxskln@1,fct-id225cf7@2
name: Compact confirmation window
rule: `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor.
guard: send_compact_confirm_window_is_its_own_bound
except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send.
state: provisional
@row_add n=1356 row=contract/events/human-input-wheel area=e2e-test-strategy rec=fct-imixgk6n@1,fct-iyklz2mz@1
boundary: events.ndjson
item: human-input
promise: Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.
@row_add n=1357 row=contract/events/harness-prompt area=e2e-test-strategy rec=fct-imixgk6n@1,fct-iyklz2mz@1
boundary: events.ndjson
item: harness-injected turn
promise: A harness-injected turn logs `prompt-submitted{origin:"harness"}` and no `wheel` record.
@row_add n=1358 row=decision/five-hour-pause-threshold area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Five-hour pause threshold at 90
rule: A `five_hour` reading of exactly 90 pauses `send` (exit 11, `detail:"five-hour"`), and 89 does not.
guard: path6_the_boundaries_and_the_window_word
@row_add n=1359 row=decision/driver-hints-never-name-release area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Driver-facing hints never name release
rule: A driver-facing refusal hint (human-typing, budget-paused) never contains release.
why: Drivers are refused release.
guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
@row_add n=1360 row=decision/error-bodies-hold-no-path area=e2e-test-strategy rec=fct-iplgfuqy@2,fct-jbbspgvm@1
name: Error bodies hold no path or upstream text
rule: CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers.
guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
@row_add n=1361 row=contract/revive-json/refusal-document area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-xttglbra@2
boundary: revive --json
item: refusal document
promise: An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
@row_add n=1362 row=decision/peer-euid-admission area=mocking-stubbing-discipline rec=fct-6xaoh2kl@2,fct-o57nkgzg@2
name: Peer admitted only on the owning user's euid
rule: A channel peer is admitted only when its observed euid equals the server's own; an unreadable euid is refused, and a refused peer is dropped before any frame, with no line written.
guard: peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported, serve_drops_a_refused_peer_before_any_frame
@row_add n=1363 row=contract/mutants-verdict/no-rust-delta area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: no-rust-delta
promise: A closed mutants.verdict value: a diff naming no .rs path never reaches cargo-mutants and passes with tested 0, the diff and its file count.
@row_add n=1364 row=contract/mutants-verdict/test-only-rust-delta area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: test-only-rust-delta
promise: A closed mutants.verdict value: a diff whose .rs paths are all test targets passes only with this verdict, naming the diff, its file count and rust_files.
@row_add n=1365 row=contract/mutants-verdict/counted area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: counted
promise: A closed mutants.verdict value: any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.
@row_add n=1366 row=contract/mutants-verdict/package area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: package
promise: A closed mutants.verdict value: a whole member scored by run --mutants --package <member>, under the same missed == 0, timeout == 0 and unviable <= caught.
@row_add n=1367 row=decision/perf-gate-spine-deadline area=quality-gates-coverage-targets rec=fct-ftncprbj@2,fct-47aec6c3@3,fct-da3wgwes@2
name: Hook perf gate at the spine deadline
rule: The five hook perf rows are required by name, and every timed hook row's hyperfine max must be below the product's SPINE_DEADLINE (1.0 s); a missing row is a breach, never a pass.
@row_add n=1368 row=contract/cli-frame-verbs/strict-modes-failed area=test-scope-summary rec=fct-xttglbra@2,fct-mqdaf5bf@2
boundary: CLI frame verbs
item: strict-modes-failed
promise: A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
@row_add n=1369 row=decision/local-command-exact-match area=test-scope-summary rec=fct-lhvxskln@1,fct-id225cf7@2
name: Local commands matched by exact equality
rule: Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test.
@row_add n=1370 row=decision/budget-governor-thresholds area=unit-test-strategy rec=fct-id225cf7@2,fct-qobe5v26@2
name: Budget governor thresholds
rule: Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override instant has passed.
guard: gate_decide_an_expired_override_is_dropped
@row_add n=1371 row=decision/local-command-unconfirmable area=unit-test-strategy rec=fct-id225cf7@2
name: Local command without a post-condition is unconfirmable
rule: A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window.
guard: …_without_a_measured_post_condition_is_unconfirmable
@row_add n=1372 row=decision/clear-confirmed-by-new-session area=unit-test-strategy rec=fct-id225cf7@2
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor.
guard: …_clear_is_confirmed_by_a_new_session
@row_add n=1373 row=decision/compact-confirmed-by-same-session area=unit-test-strategy rec=fct-id225cf7@2
name: /compact confirmed by its same session
rule: /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing.
guard: …_compact_is_confirmed_by_its_same_session
@row_add n=1374 row=decision/e2e-home-headroom-floor area=test-data-bootstrap rec=fct-p6be5x2e@1,fct-msk3nz6p@1,fct-zxaumenb@5
name: Headroom floor of the e2e home base
rule: A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom.
why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB.
guard: harness::tests::e2e_home_*
@row_add n=1375 row=decision/fixture-chain-exists-twice area=5-command-implementation rec=fct-hgewdmj2@6
name: Fixture chain exists twice
rule: The home, fake_agent_path, stamped_home and booted_wrapper chain exists twice, as the sync root copy and the viola-e2e copy, against one contract.
why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
@row_add n=1376 row=decision/coverage-floors area=5-command-implementation rec=fct-zxoe3ae7@2,fct-da3wgwes@2,fct-47aec6c3@3
name: Coverage floors
rule: CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
