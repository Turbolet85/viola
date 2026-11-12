@row_add n=1 row=decision/one-test-per-recorded-hook-payload area=4-unit-test-strategy rec=1
name: One generated test per recorded hook payload
rule: Unit tests generate one test case per recorded hook payload fixture file through rstest's files attribute.
@row_add n=2 row=decision/proptest-parser-invariants area=4-unit-test-strategy rec=2
name: Proptest for parser invariants
rule: Parser invariants are held by proptest property blocks run at 512 cases.
@row_add n=3 row=decision/insta-snapshots-for-decision-bodies area=4-unit-test-strategy rec=3
name: Insta snapshots for decision bodies
rule: Decision bodies are pinned by insta JSON snapshots with the dialog_id and ts fields redacted.
@row_none n=1 rec=4 type=S
@row_none n=2 rec=5 type=5
@row_add n=4 row=decision/pty-exit-from-wait-not-eof area=4-unit-test-strategy rec=6
name: PTY exit reported from wait, never from reader EOF
rule: viola-pty reports a child's exit from wait on the handle, never from reader EOF; consumers are tested with a mockall double of the six-method seam whose reader never returns EOF while wait returns an exit status.
@row_none n=3 rec=7 type=5
@row_none n=4 rec=8 type=5
@row_add n=5 row=decision/identity-floor-tested-against-literal-copy area=4-unit-test-strategy rec=9
name: Identity floor tested against a literal copy
rule: The R8 strip is tested against a literal copy of the identity floor, never an imported one.
@row_add n=6 row=decision/no-assertion-beyond-compiled-literals area=4-unit-test-strategy rec=9
name: No assertion beyond compiled literals
rule: viola-agent-claude unit tests never assert agent content beyond the compiled literals.
@row_add n=7 row=contract/mcp-tools-list/tools-list area=4-unit-test-strategy rec=10
boundary: MCP tools/list
item: tools/list
promise: tools/list is exactly send, wait, last, answer, list.
@row_add n=8 row=contract/mcp-tool-arguments/invalid-argument area=4-unit-test-strategy rec=10
boundary: MCP tool arguments
item: invalid argument
promise: A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.
@row_add n=9 row=contract/mcp-stdin/line-cap area=4-unit-test-strategy rec=10
boundary: MCP stdin
item: line cap
promise: A line of exactly MAX_FRAME bytes, its newline counted, passes whole; one byte more passes no byte, reads as end of stream, and serve ends the session.
@row_add n=10 row=contract/viola-mcp-exit/oversize-before-initialize area=4-unit-test-strategy rec=10
boundary: viola mcp exit
item: oversize line before initialize
promise: A stdin line past MAX_FRAME before initialize completes makes viola mcp exit 1 with both streams empty and one parse-rejected mcp-stdin / oversize line of count 1.
@row_add n=11 row=contract/reply-screen/refusal area=4-unit-test-strategy rec=10,11
boundary: the reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_add n=12 row=contract/reply-screen/detail area=4-unit-test-strategy rec=10,11
boundary: the reply screen
item: detail
promise: A string detail that is no code word reads null.
@row_add n=13 row=contract/reply-screen/peer-fault area=4-unit-test-strategy rec=10,11
boundary: the reply screen
item: peer fault
promise: A peer fault reads through sanitised_fault.
@row_add n=14 row=contract/cli-exit-codes/14 area=4-unit-test-strategy rec=11
boundary: CLI exit codes
item: 14
promise: An unknown refusal maps to exit 14, never a panic and never 20.
@row_none n=5 rec=12 type=5
@row_add n=15 row=decision/mocks-only-at-product-seams area=4-unit-test-strategy rec=13
name: Mocks only at product seams
rule: Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
@row_none n=6 rec=14 type=S
@row_none n=7 rec=15 type=S
@row_none n=8 rec=16 type=S
@row_none n=9 rec=17 type=S
@row_add n=16 row=decision/real-claude-local-only area=4-unit-test-strategy rec=18,53,52
name: Real claude CLI tested local-only
rule: The real claude CLI and Anthropic backend are mocked at unit level, untestable in CI, and reached only locally through viola verify.
@row_none n=10 rec=19 type=S
@row_none n=11 rec=20 type=S
@row_none n=12 rec=21 type=S
@row_add n=17 row=decision/wrapper-channel-integration-driver area=5-integration-test-strategy rec=22,21
name: Wrapper channel integration driver
rule: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the product's schemars schemas.
@row_add n=18 row=decision/cli-outputs-pinned-by-literal area=5-integration-test-strategy rec=23,21
name: CLI outputs pinned by literal asserts
rule: CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
@row_add n=19 row=decision/human-layouts-pinned-by-literal area=5-integration-test-strategy rec=24,21
name: Human-mode layouts pinned by literal
rule: Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files.
guard: HELP literal in cli_output_plain; cli_list board tests
@row_add n=20 row=decision/real-pty-in-integration area=5-integration-test-strategy rec=25,21
name: Real PTY in integration tests
rule: Integration tests use the real portable-pty through viola-pty.
@row_add n=21 row=decision/real-disk-in-integration area=5-integration-test-strategy rec=26,21
name: Real disk in integration tests
rule: Integration tests use a real tempfile filesystem.
@row_none n=13 rec=27 type=S
@row_add n=22 row=decision/module-integration-direct-calls area=5-integration-test-strategy rec=28
name: Module integration by direct calls
rule: Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.
@row_add n=23 row=absence/database area=5-integration-test-strategy rec=29
capability: A database
@row_add n=24 row=decision/on-disk-state-tested-on-real-files area=5-integration-test-strategy rec=29
name: On-disk state tested on real files
rule: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
@row_add n=25 row=absence/outbound-http area=5-integration-test-strategy rec=30
capability: Outbound HTTP
@row_add n=26 row=decision/fake-agent-as-claude area=5-integration-test-strategy rec=30
name: Fake agent serves the claude boundary
rule: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
@row_add n=27 row=decision/ipc-tested-on-real-endpoint area=5-integration-test-strategy rec=31
name: IPC tested on a real endpoint
rule: IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
@row_add n=28 row=decision/pty-tested-on-real-openpty area=5-integration-test-strategy rec=32
name: PTY tested on real openpty spawns
rule: PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size.
@row_add n=29 row=decision/hook-exec-against-live-wrapper area=5-integration-test-strategy rec=33
name: Hook exec tested against a live wrapper
rule: viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted.
@row_add n=30 row=decision/fresh-home-per-test area=5-integration-test-strategy rec=34
name: Fresh home per test
rule: Each test gets a fresh home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests.
why: so obs-plan's CI gates, the secret scan and the upload cover integration runs
except: The chaos feed panic test alone boots outside the scans.
@row_none n=14 rec=35 type=4
relied: The watch report carries a stalled wrapper's state at the readiness deadline, before the panic, and a passing boot adds nothing to it.
@row_none n=15 rec=36 type=S
@row_add n=31 row=contract/wrapper-channel/cursor area=5-integration-test-strategy rec=37
boundary: the wrapper channel
item: cursor
promise: The cursor equals the log length at acceptance.
@row_add n=32 row=contract/wrapper-channel/-32602 area=5-integration-test-strategy rec=37
boundary: the wrapper channel
item: -32602
promise: A request with a higher v is answered with -32602.
@row_add n=33 row=contract/wrapper-channel/-32600 area=5-integration-test-strategy rec=37
boundary: the wrapper channel
item: -32600
promise: An oversize frame is answered with -32600 and the connection closed.
@row_add n=34 row=contract/wrapper-channel/control-character area=5-integration-test-strategy rec=37
boundary: the wrapper channel
item: control-character
promise: In wrapper-side paste validation, control-character wins over every other refusal.
@row_add n=35 row=contract/event-line/ts area=5-integration-test-strategy rec=38
boundary: the event line
item: ts
promise: ts is RFC 3339 UTC with milliseconds.
@row_add n=36 row=contract/event-line/round-trip area=5-integration-test-strategy rec=38
boundary: the event line
item: event kind
promise: Every event kind round-trips the event line.
@row_add n=37 row=contract/event-tailing/ignored-entries area=5-integration-test-strategy rec=38
boundary: event tailing
item: symlinks and invalid names
promise: Tailing ignores symlinks and invalid names.
@row_add n=38 row=contract/strict-modes-refusal/run-mcp-revive-verify area=5-integration-test-strategy rec=38
boundary: strict-modes refusal
item: run, mcp, viola revive, viola verify
promise: A strict-modes refusal exits 1.
@row_add n=39 row=contract/strict-modes-refusal/hook area=5-integration-test-strategy rec=38
boundary: strict-modes refusal
item: hook
promise: A strict-modes refusal in the hook exits 0 with no output.
@row_add n=40 row=contract/strict-modes-refusal/mcp-frame-tools area=5-integration-test-strategy rec=38
boundary: strict-modes refusal
item: MCP frame tools
promise: A strict-modes refusal in the MCP frame tools gives instance-unreachable.
@row_add n=41 row=contract/strict-modes-refusal/21 area=5-integration-test-strategy rec=38
boundary: strict-modes refusal
item: 21
promise: Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
@row_add n=42 row=contract/strict-modes-refusal/no-line area=5-integration-test-strategy rec=38
boundary: strict-modes refusal
item: no line
promise: A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
@row_add n=43 row=contract/viola-hook/exit-and-stderr area=5-integration-test-strategy rec=39
boundary: viola hook
item: exit and stderr
promise: Every hook event exits 0 with empty stderr.
@row_add n=44 row=contract/viola-hook/stdout area=5-integration-test-strategy rec=39
boundary: viola hook
item: stdout
promise: A hook prints a decision body or empty stdout; async hooks print nothing.
@row_add n=45 row=contract/viola-hook/no-viola-name area=5-integration-test-strategy rec=39
boundary: viola hook
item: VIOLA_NAME
promise: With no VIOLA_NAME the hook exits 0 immediately.
@row_add n=46 row=contract/viola-hook/viola-dir area=5-integration-test-strategy rec=39
boundary: viola hook
item: VIOLA_DIR
promise: A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
@row_add n=47 row=contract/cli-output/no-escape-without-tty area=5-integration-test-strategy rec=40
boundary: CLI output
item: escape sequences
promise: Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
@row_add n=48 row=contract/cli-argv/leading-slash area=5-integration-test-strategy rec=40
boundary: CLI argv
item: leading slash
promise: A leading-slash argument is never delivered as a prompt.
@row_add n=49 row=decision/controls-not-disableable area=5-integration-test-strategy rec=40
name: Controls not disableable
rule: No setting disables a control, the socket directory check among them.
guard: setting_does_not_disable_the_socket_dir_check
@row_add n=50 row=decision/board-on-the-read-rule area=5-integration-test-strategy rec=40
name: Board held to the read rule
rule: viola list refuses a home another user may read, with and without an instance.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_add n=51 row=decision/test-started-viola-drops-env area=5-integration-test-strategy rec=41
name: Test-started viola drops inherited env
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
@row_add n=52 row=decision/mcp-handlers-use-tokio-client area=5-integration-test-strategy rec=42
name: MCP handlers use the Tokio client
rule: MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
@row_add n=53 row=contract/mcp-frame-tools/socket-dir-refused area=5-integration-test-strategy rec=42
boundary: MCP frame tools
item: socket-dir-refused
promise: A loosened socket directory gives isError with socket-dir-refused, no frame and no line.
@row_none n=16 rec=43 type=S
@row_none n=17 rec=44 type=S
@row_none n=18 rec=45 type=S
@row_add n=54 row=decision/tty-asserted-only-on-verb-sgr area=6-e2e-test-strategy rec=46
name: TTY rendering asserted only on verb SGR
rule: CLI E2E pins --json documents and human layouts by literals, and asserts TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@row_add n=55 row=decision/hook-e2e-deadline-driver area=6-e2e-test-strategy rec=47
name: Hook E2E under a deadline
rule: viola hook and hook statusline are driven as processes with an Instant deadline over recorded fixture stdin.
@row_add n=56 row=decision/env-clear-keeps-llvm-profile-file area=6-e2e-test-strategy rec=47
name: Cleared env keeps LLVM_PROFILE_FILE
rule: A test that clears a child's environment must re-add LLVM_PROFILE_FILE.
@row_add n=57 row=decision/tui-e2e-never-parses-screen area=6-e2e-test-strategy rec=48
name: TUI E2E never parses the rendered screen
rule: viola run passthrough E2E detects exit only via child wait, drains the master reader on its own thread, and takes verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen.
except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
@row_add n=58 row=decision/endpoint-read-from-snapshot area=6-e2e-test-strategy rec=49
name: Endpoint read only from the snapshot
rule: Wrapper channel E2E drives the sync client, a raw interprocess stream and jsonschema, and reads the endpoint only from snapshot.json.
@row_add n=59 row=decision/mcp-e2e-test-owns-child area=6-e2e-test-strategy rec=50
name: MCP E2E test owns the child
rule: MCP stdio E2E runs an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
@row_none n=19 rec=51 type=5
@row_none n=20 rec=52 type=S
@row_add n=60 row=decision/live-readings-not-a-test-layer area=6-e2e-test-strategy rec=53
name: Live readings are not a test layer
rule: A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
@row_add n=61 row=decision/cross-user-peers-untestable area=6-e2e-test-strategy rec=54,52
name: Cross-OS-user peers untestable end to end
rule: Cross-OS-user peers are skipped as untestable in E2E, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
@row_none n=21 rec=55 type=S
