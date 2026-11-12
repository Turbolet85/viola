@row_add n=1 row=decision/perf-panic-check-before-cleanup area=quality-gates-coverage-targets rec=1
name: Perf run panic check
rule: After the last perf row and before cleanup, the perf run fails as panic when the session home's role files hold any panic event line, with no seam exemption.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=2 row=decision/perf-failure-turns-suite-red area=quality-gates-coverage-targets rec=1
name: Perf failures named, never skipped
rule: A failed build, boot, row, panic check or cleanup turns suite perf red with the failure named, never a missing or skipped row.
@row_add n=3 row=decision/perf-hooks-time-the-real-path area=quality-gates-coverage-targets rec=1
name: Perf hooks time the real path
rule: Every timed hook runs with an instance name and directory set, so it takes the real logging and channel path, never the no-instance no-op, with its instance, server, socket-directory and peer checks inside the timed path.
@row_add n=4 row=decision/perf-gate-spine-deadline area=quality-gates-coverage-targets rec=1
name: Hook perf gate
rule: The gate on each timed hook row is SPINE_DEADLINE.
@row_add n=5 row=decision/product-state-never-hand-written area=test-data-fixtures rec=2
name: On-disk product state produced by viola
rule: On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written.
except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
@row_add n=6 row=decision/stamps-only-from-verify area=test-data-fixtures rec=3,37
name: Stamps written only by verify
rule: The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it.
why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
@row_add n=7 row=decision/budget-seeded-through-statusline-hook area=test-data-fixtures rec=4,37
name: Budget seeded through the statusline hook
rule: A test's budget file is seeded by piping generated statusline stdin into viola hook statusline.
except: The budget reader's own unit cases pad, replace or re-version bytes the writer wrote, because no writer produces an over-cap, unparseable or v 2 file.
@row_add n=8 row=decision/statusline-source-written-by-test area=test-data-fixtures rec=5
name: Statusline source planted by the test
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_none n=1 rec=6 type=5
@row_none n=2 rec=7 type=5
@row_add n=9 row=decision/socket-dir-check-leaves-dir-as-found area=test-data-fixtures rec=8,38
name: Socket-directory check never repairs
rule: The socket-directory check never chmods the directory it checks; a refused socket directory is left as found.
@row_add n=10 row=decision/foreign-owner-by-injected-uid area=test-data-fixtures rec=8
name: Foreign socket-directory owner by injected uid
rule: A socket directory with a foreign owner is not driven in tests; its verdict is taken by an injected uid.
@row_add n=11 row=decision/agents-stub-from-files-no-env area=test-data-fixtures rec=9,37
name: Agents list answered by a stub on PATH
rule: claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH.
why: The no-env answer was ratified.
@row_add n=12 row=decision/hook-payloads-recorded-by-verify area=test-data-fixtures rec=10,37
name: Hook payloads recorded by verify, replayed by CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
@row_add n=13 row=contract/claude-fixtures/payload-path area=test-data-fixtures rec=10
boundary: Recorded Claude fixtures
item: payload path
promise: A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
@row_add n=14 row=decision/mcp-exchange-recorded-by-tap area=test-data-fixtures rec=11
name: MCP exchange recorded by a test-only tap
rule: The recorded MCP exchange is taken by a test-only tap between a live claude start and viola mcp; the raw capture is 0600 and never committed, and the kept fixture passes the record scrub and refusal.
@row_add n=15 row=contract/mcp-fixture-scrub-refusal/home-path area=test-data-fixtures rec=11
boundary: MCP fixture scrub refusal codes
item: home-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=16 row=contract/mcp-fixture-scrub-refusal/absolute-path area=test-data-fixtures rec=11
boundary: MCP fixture scrub refusal codes
item: absolute-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=17 row=contract/mcp-fixture-scrub-refusal/username area=test-data-fixtures rec=11
boundary: MCP fixture scrub refusal codes
item: username
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=18 row=contract/mcp-fixture-scrub-refusal/email area=test-data-fixtures rec=11
boundary: MCP fixture scrub refusal codes
item: email
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_add n=19 row=decision/human-output-expected-as-literals area=test-data-fixtures rec=12
name: Human-mode expected output as literals
rule: Human-mode expected output is held as literals in the test files that check it.
@row_add n=20 row=decision/decision-bodies-as-insta-snapshots area=test-data-fixtures rec=13
name: Decision bodies as insta snapshots
rule: Decision bodies are checked against insta snapshots, regenerated only by the local fixture refresh.
@row_add n=21 row=decision/fake-transcripts-compared-to-recordings area=test-data-fixtures rec=14
name: Fake-agent transcripts compared byte for byte
rule: Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
@row_add n=22 row=decision/proptest-for-randomized-data area=test-data-fixtures rec=15
name: proptest for randomized data
rule: Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
@row_add n=23 row=decision/mock-instant-behind-clock-seam area=test-data-fixtures rec=16
name: Mocked time behind the clock seam
rule: Time in the sync crates is mocked with mock_instant's MockClock in global mode behind the core Clock seam.
@row_add n=24 row=decision/link-since-asserted-by-shape area=test-data-fixtures rec=16
name: Link time asserted by shape
rule: The link line's since time is asserted by its shape only.
why: It is stamped outside the clock seam.
@row_add n=25 row=contract/mcp-wait-tool/timeout-bound area=test-data-fixtures rec=16
boundary: MCP wait tool
item: wait bound
promise: The wait bound is the given timeout capped at 50 000 ms, min(given, 50 000).
@row_none n=3 rec=17 type=4
relied: The error-body check reads a hit of a planted value as class planted and reports place and class, never bytes.
@row_none n=4 rec=18 type=5
@row_none n=5 rec=19 type=5
@row_add n=26 row=decision/test-homes-made-private area=test-data-fixtures rec=20,37
name: Test-made homes are private directories
rule: A home a test makes before viola runs is made as a private directory through the state crate's private-directory helper, never by plain recursive creation.
why: The strict-modes read rule refuses a home made by plain recursive creation, its 0755 mode failing the rule.
@row_add n=27 row=decision/test-endpoint-dirs-made-private area=test-data-fixtures rec=21,37
name: Test endpoint directories are private
rule: A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir.
why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
@row_add n=28 row=decision/fake-agent-copies-planted-first area=test-data-fixtures rec=22
name: Fake-agent copies planted before any child
rule: A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child.
why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).
@row_add n=29 row=decision/fake-agent-replays-recorded-bytes area=test-data-fixtures rec=23
name: Fake agent replays recorded bytes
rule: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the resume launch SessionStart for a set without the recorded resume variant and the compact replay's session id.
why: The rewritten resume payload was ratified.
@row_add n=30 row=contract/fake-agent-receipt/line-format area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: line format
promise: Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
@row_add n=31 row=contract/fake-agent-receipt/start area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: start
promise: Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
@row_add n=32 row=contract/fake-agent-receipt/size area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: size
promise: Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
@row_add n=33 row=contract/fake-agent-receipt/cwd area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: cwd
promise: Carries cwd.
@row_add n=34 row=contract/fake-agent-receipt/hold area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: hold
promise: Carries pid, written by the --exit-no-eof holder at its start.
@row_add n=35 row=contract/fake-agent-receipt/env area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: env
promise: Carries names, sorted, names only, never a value.
@row_add n=36 row=contract/fake-agent-receipt/fds area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: fds
promise: Carries fds.
@row_add n=37 row=contract/fake-agent-receipt/key area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: key
promise: Carries hex, one line per byte received outside a paste.
@row_add n=38 row=contract/fake-agent-receipt/prompt area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: prompt
promise: Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
@row_add n=39 row=contract/fake-agent-receipt/hook area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: hook
promise: Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
@row_add n=40 row=contract/fake-agent-receipt/statusline area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: statusline
promise: Carries the hook kind's fields without event.
@row_add n=41 row=contract/fake-agent-receipt/step area=test-data-fixtures rec=24
boundary: Fake-agent receipt
item: step
promise: Carries index and event.
@row_add n=42 row=decision/fake-agent-argv-only area=test-data-fixtures rec=25
name: Fake agent configured by argv alone
rule: The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments.
@row_add n=43 row=decision/harness-cli-version-matches-fake area=test-data-fixtures rec=25
name: One default CLI version
rule: The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
@row_add n=44 row=decision/wrapper-boots-pass-trusted-screens area=test-data-fixtures rec=25
name: Test wrapper boots run trusted with screens
rule: Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd.
except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
@row_add n=45 row=decision/fixtures-walked-for-hygiene area=test-data-fixtures rec=26
name: Fixture hygiene walk
rule: Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red.
guard: contract_fixture_hygiene
@row_add n=46 row=contract/process-exit-detail/unclean-fixture area=test-data-fixtures rec=27
boundary: process-exit detail
item: unclean-fixture
promise: verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
@row_add n=47 row=decision/cross-user-at-peer-check-boundary area=test-scope-summary rec=28
name: Cross-user rejection asserted at the boundary
rule: Rejection of a peer with a different euid is asserted at the peer-credential check boundary, over injected euids and an injected decision, with no seam added to the client wiring.
why: Hosted CI runners have no second OS account, so a foreign peer cannot be driven.
guard: peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported
@row_add n=48 row=contract/wrapper-channel/max-frame area=test-scope-summary rec=28
boundary: Wrapper channel
item: MAX_FRAME
promise: A frame is at most 16 MiB.
@row_add n=49 row=contract/cli-frame-verbs/server-verify-failed area=test-scope-summary rec=28
boundary: CLI frame verbs
item: server-verify-failed
promise: A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
@row_add n=50 row=contract/hook/impostor-server area=test-scope-summary rec=28
boundary: Hook
item: impostor server
promise: A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
@row_add n=51 row=contract/channel-fault-screen/known-code area=test-scope-summary rec=28
boundary: Channel fault screen
item: known code
promise: Each of the five known codes takes its fixed message, with data null save the kept -32602 data.
@row_add n=52 row=contract/channel-fault-screen/-32602 area=test-scope-summary rec=28
boundary: Channel fault screen
item: -32602
promise: Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null.
@row_add n=53 row=contract/channel-fault-screen/unknown-code area=test-scope-summary rec=28
boundary: Channel fault screen
item: code outside the five
promise: Reads wrapper fault with data null.
@row_add n=54 row=contract/channel-fault-screen/no-integer-code area=test-scope-summary rec=28
boundary: Channel fault screen
item: error without an integer code
promise: Reads nothing.
@row_add n=55 row=decision/real-claude-never-in-ci area=test-scope-summary rec=29,36
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
@row_add n=56 row=contract/revive-json/refusal-document area=test-scope-summary rec=30
boundary: revive --json
item: refusal document
promise: An exit-1 refusal of the strict-modes check, the preflight or the shared start prints {"v":1,"error":"<code>","detail":null} on stdout.
@row_add n=57 row=contract/revive-refusal-code/strict-modes-failed area=test-scope-summary rec=30
boundary: revive refusal codes
item: strict-modes-failed
promise: A closed code of the revive --json refusal document.
@row_add n=58 row=contract/revive-refusal-code/already-live area=test-scope-summary rec=30
boundary: revive refusal codes
item: already-live
promise: A closed code of the revive --json refusal document.
@row_add n=59 row=contract/revive-refusal-code/no-session area=test-scope-summary rec=30
boundary: revive refusal codes
item: no-session
promise: A closed code of the revive --json refusal document; also the document of an empty --list chain.
@row_add n=60 row=contract/revive-refusal-code/session-live area=test-scope-summary rec=30
boundary: revive refusal codes
item: session-live
promise: A closed code of the revive --json refusal document.
@row_add n=61 row=contract/revive-refusal-code/cwd-missing area=test-scope-summary rec=30
boundary: revive refusal codes
item: cwd-missing
promise: A closed code of the revive --json refusal document.
@row_add n=62 row=contract/revive-refusal-code/stale-heartbeat area=test-scope-summary rec=30
boundary: revive refusal codes
item: stale-heartbeat
promise: A closed code of the revive --json refusal document.
@row_add n=63 row=contract/revive-refusal-code/pinned-hash-mismatch area=test-scope-summary rec=30
boundary: revive refusal codes
item: pinned-hash-mismatch
promise: A closed code of the revive --json refusal document.
@row_add n=64 row=contract/revive-refusal-code/squatted-name area=test-scope-summary rec=30
boundary: revive refusal codes
item: squatted-name
promise: A closed code of the revive --json refusal document.
@row_add n=65 row=contract/revive-refusal-code/socket-dir-refused area=test-scope-summary rec=30
boundary: revive refusal codes
item: socket-dir-refused
promise: A closed code of the revive --json refusal document.
@row_add n=66 row=contract/revive-refusal-code/internal-error area=test-scope-summary rec=30
boundary: revive refusal codes
item: internal-error
promise: A closed code of the revive --json refusal document.
@row_add n=67 row=contract/revive-json/list-document area=test-scope-summary rec=30
boundary: revive --json
item: --list document
promise: revive --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_add n=68 row=contract/revive-json/pump-child-error area=test-scope-summary rec=30
boundary: revive --json
item: child pump error
promise: An error while pumping the child prints no document, the child having held the terminal.
@row_add n=69 row=contract/revive-exit-codes/0 area=test-scope-summary rec=30
boundary: revive exit codes
item: 0
promise: Success.
@row_add n=70 row=contract/revive-exit-codes/1 area=test-scope-summary rec=30
boundary: revive exit codes
item: 1
promise: A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
@row_add n=71 row=contract/revive-exit-codes/2 area=test-scope-summary rec=30
boundary: revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_add n=72 row=decision/frame-verbs-check-instance-first area=test-scope-summary rec=30
name: Frame verbs check the instance first
rule: All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
@row_add n=73 row=contract/cli-frame-verbs/strict-modes-failed area=test-scope-summary rec=30
boundary: CLI frame verbs
item: strict-modes-failed
promise: An instance refused by the strict-modes check exits 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_add n=74 row=decision/self-link-refused-by-wrapper area=test-scope-summary rec=30,34
name: Self-link refused by the wrapper
rule: A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20.
@row_add n=75 row=decision/linux-only area=test-scope-summary rec=31
name: Linux only
rule: viola targets Linux only: the core on any current distribution, CI on ubuntu-latest, and the window features on Hyprland.
@row_add n=76 row=absence/windows-and-macos-support area=test-scope-summary rec=31
capability: Windows and macOS support: code, tests, fixtures and CI jobs
@row_add n=77 row=decision/cli-json-one-document area=test-scope-summary rec=32
name: One JSON document per verb
rule: Under --json a CLI verb signals by its exit code plus one JSON document on stdout.
except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
guard: NOT_ERROR_BODIES
@row_add n=78 row=decision/no-sgr-without-tty area=test-scope-summary rec=32
name: No SGR without a terminal
rule: CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
@row_add n=79 row=decision/human-tty-through-outer-pty area=test-scope-summary rec=32
name: Human TTY rendering through the outer PTY
rule: The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_add n=80 row=contract/cli-human-lines/read-back area=test-scope-summary rec=32
boundary: CLI human lines
item: read back
promise: [RB] read back … cursor <n>
@row_add n=81 row=contract/cli-human-lines/unable area=test-scope-summary rec=32
boundary: CLI human lines
item: unable
promise: unable  <name>  <reason>  <detail>, followed by a last-line hint:.
@row_add n=82 row=contract/cli-human-lines/stamped area=test-scope-summary rec=32
boundary: CLI human lines
item: stamped
promise: stamped <ver>  <n> pass  <n> fail
@row_add n=83 row=contract/revive-human/refusal area=test-scope-summary rec=32
boundary: revive human output
item: refusal
promise: Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty.
@row_add n=84 row=contract/revive-human/list-row area=test-scope-summary rec=32
boundary: revive human output
item: --list row
promise: One <ts>  <cause>  <id> row per logged session on stdout.
@row_add n=85 row=decision/never-parse-rendered-child-screen area=test-scope-summary rec=33
name: Rendered child screen never parsed
rule: Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search.
why: Named a test anti-pattern by the creator.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=86 row=contract/wrapper-channel/success area=test-scope-summary rec=34
boundary: Wrapper channel
item: success
promise: {"result":{"ok":…}}
@row_add n=87 row=contract/wrapper-channel/refusal area=test-scope-summary rec=34
boundary: Wrapper channel
item: refusal
promise: {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
@row_add n=88 row=contract/wrapper-channel/protocol-fault area=test-scope-summary rec=34
boundary: Wrapper channel
item: protocol fault
promise: {"error":{code,message,data}}.
@row_add n=89 row=contract/wrapper-channel/-32602 area=test-scope-summary rec=34
boundary: Wrapper channel
item: -32602
promise: Invalid params, with data {supported, wrapper} for a too-new request.
@row_add n=90 row=contract/wrapper-channel/-32603 area=test-scope-summary rec=34
boundary: Wrapper channel
item: -32603
promise: internal error with data null.
@row_add n=91 row=contract/wrapper-channel/link area=test-scope-summary rec=34
boundary: Wrapper channel
item: link
promise: Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603.
@row_add n=92 row=contract/wrapper-channel/unlink area=test-scope-summary rec=34
boundary: Wrapper channel
item: unlink
promise: Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603.
@row_add n=93 row=contract/channel-response-line/detail area=test-scope-summary rec=34
boundary: channel-response log line
item: detail
promise: On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param.
@row_add n=94 row=decision/channel-error-bodies-literal area=test-scope-summary rec=34
name: Channel error bodies are literals
rule: The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text.
guard: channel_error_bodies_hold_no_path_or_upstream_text
@row_add n=95 row=contract/mcp-tool-result/ok area=test-scope-summary rec=35
boundary: MCP tool result
item: ok
promise: isError false with structuredContent equal to the ok payload.
@row_add n=96 row=contract/mcp-tool-result/refusal area=test-scope-summary rec=35
boundary: MCP tool result
item: refusal
promise: isError true with {refusal, detail} and text "refused: <reason>".
@row_add n=97 row=contract/mcp-tool-result/instance-unreachable area=test-scope-summary rec=35
boundary: MCP tool result
item: instance-unreachable
promise: isError true with {"error":"instance-unreachable","detail":"<cause>"}, the CLI exit-21 document less v, text refused: <code>.
@row_add n=98 row=contract/instance-unreachable-cause/strict-modes-failed area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: strict-modes-failed
promise: A closed cause word of instance-unreachable's detail.
@row_add n=99 row=contract/instance-unreachable-cause/instance-dead area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: instance-dead
promise: A closed cause word of instance-unreachable's detail.
@row_add n=100 row=contract/instance-unreachable-cause/unwrapped-name area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: unwrapped-name
promise: A closed cause word of instance-unreachable's detail.
@row_add n=101 row=contract/instance-unreachable-cause/endpoint-lost area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: endpoint-lost
promise: A closed cause word of instance-unreachable's detail.
@row_add n=102 row=contract/instance-unreachable-cause/server-verify-failed area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: server-verify-failed
promise: A closed cause word of instance-unreachable's detail.
@row_add n=103 row=contract/instance-unreachable-cause/socket-dir-refused area=test-scope-summary rec=35
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A closed cause word of instance-unreachable's detail.
@row_add n=104 row=contract/mcp-tool-result/wrapper-fault area=test-scope-summary rec=35
boundary: MCP tool result
item: wrapper-fault
promise: isError true with {"error":"wrapper-fault","detail":{code,message,data}}, the detail being the channel fault screen's.
@row_add n=105 row=contract/mcp-tool-result/invalid-params area=test-scope-summary rec=35
boundary: MCP tool result
item: invalid-params
promise: isError true with {"error":"invalid-params","detail":"<field>"}, no frame written.
@row_add n=106 row=contract/mcp-invalid-params-field/target area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: target
promise: A closed field word of invalid-params' detail.
@row_add n=107 row=contract/mcp-invalid-params-field/text area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: text
promise: A closed field word of invalid-params' detail.
@row_add n=108 row=contract/mcp-invalid-params-field/after area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: after
promise: A closed field word of invalid-params' detail.
@row_add n=109 row=contract/mcp-invalid-params-field/timeout-ms area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: timeout_ms
promise: A closed field word of invalid-params' detail.
@row_add n=110 row=contract/mcp-invalid-params-field/dialog-id area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: dialog_id
promise: A closed field word of invalid-params' detail.
@row_add n=111 row=contract/mcp-invalid-params-field/response area=test-scope-summary rec=35
boundary: MCP invalid-params fields
item: response
promise: A closed field word of invalid-params' detail.
@row_add n=112 row=contract/mcp-server/internal-fault area=test-scope-summary rec=35
boundary: MCP server
item: internal handler fault
promise: Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_add n=113 row=contract/mcp-server/unknown-tool area=test-scope-summary rec=35
boundary: MCP server
item: unknown tool
promise: The JSON-RPC error -32602.
@row_add n=114 row=contract/mcp-server/tools-list area=test-scope-summary rec=35
boundary: MCP server
item: tools/list
promise: Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private.
@row_add n=115 row=decision/mcp-no-control-tools area=test-scope-summary rec=35
name: No control tools over MCP
rule: The MCP server offers no release, pause, link or unlink tool.
@row_add n=116 row=decision/mcp-negatives-violate-schema area=test-scope-summary rec=35
name: MCP negatives violate the advertised schema
rule: MCP tests send negative inputs that violate the advertised tool schemas.
why: The schemas are advisory.
@row_add n=117 row=decision/mcp-frame-tools-check-instance-first area=test-scope-summary rec=35
name: MCP frame tools check the instance first
rule: The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line.
guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
@row_add n=118 row=contract/mcp-reply-screen/refusal area=test-scope-summary rec=35
boundary: MCP reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_add n=119 row=contract/mcp-reply-screen/detail area=test-scope-summary rec=35
boundary: MCP reply screen
item: detail
promise: A string detail that is no code word reads null, and a non-string detail is no known reply.
@row_add n=120 row=contract/mcp-stdin/oversize-line area=test-scope-summary rec=35
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: Passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one parse-rejected mcp-stdin oversize WARN line; earlier lines are answered and viola mcp exits 0 with stderr empty.
@row_add n=121 row=contract/mcp-stdin/oversize-before-initialize area=test-scope-summary rec=35
boundary: viola mcp stdin
item: line past MAX_FRAME before initialize
promise: viola mcp exits 1 with both streams empty and one parse-rejected mcp-stdin oversize line.
@row_add n=122 row=absence/browser-suite area=test-scope-summary rec=36
capability: A browser test suite
@row_add n=123 row=decision/mutation-testing-on-demand area=test-scope-summary rec=36
name: Mutation testing on demand
rule: Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
@row_add n=124 row=contract/harness-run/exit area=test-scope-summary rec=36
boundary: Harness run command
item: exit
promise: Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
@row_add n=125 row=contract/harness-boot/readiness-failure area=test-scope-summary rec=36
boundary: Harness boot command
item: readiness failure
promise: Exits non-zero with a JSON reason on stdout; a run refusal to start is surfaced verbatim.
@row_add n=126 row=contract/harness-cleanup/idempotent area=test-scope-summary rec=36
boundary: Harness cleanup command
item: second run
promise: A second cleanup exits 0 and reports nothing to clean.
@row_add n=127 row=contract/harness-cleanup/endpoint-gone area=test-scope-summary rec=36
boundary: Harness cleanup command
item: endpoint check
promise: Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
@row_add n=128 row=decision/events-never-truncated area=test-scope-summary rec=36
name: Events never truncated
rule: Instance events are never truncated; their retention in tests is the lifetime of the temp home.
@row_add n=129 row=decision/per-test-fresh-home area=test-scope-summary rec=37
name: Self-bootstrapping per-test home
rule: Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
@row_add n=130 row=decision/fuzz-the-parser-surfaces area=test-scope-summary rec=37
name: Property and fuzz generators on parsers
rule: Property and fuzz generators cover the parser surfaces the security plan lists.
@row_add n=131 row=decision/fixtures-never-from-real-prompts area=test-scope-summary rec=37
name: Fixtures never from real prompts
rule: Fixtures must not come from real prompts and must be checked for absolute paths and usernames.
why: A security anti-pattern.
@row_add n=132 row=decision/run-checks-before-child-spawn area=test-scope-summary rec=38
name: viola run checks before the child
rule: viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child.
guard: run_refuses_a_widened_home_tree_before_its_log
@row_add n=133 row=contract/run-start-events/order area=test-scope-summary rec=38
boundary: run start events
item: order
promise: The events log reads wheel with cause start, then budget-gate, then the agent's session-start with source hook.
@row_add n=134 row=contract/run-refusal/socket-dir-refused area=test-scope-summary rec=38
boundary: run and revive start refusals
item: socket-dir-refused
promise: A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
@row_add n=135 row=contract/run-refusal/stale-heartbeat area=test-scope-summary rec=38
boundary: run and revive start refusals
item: stale-heartbeat
promise: A stale holder refuses the start with exit 1.
@row_add n=136 row=contract/run-refusal/already-live area=test-scope-summary rec=38
boundary: run and revive start refusals
item: already-live
promise: A live holder or a fresh replayed beat refuses the start with exit 1.
@row_add n=137 row=contract/instance-settings/statusline-override area=test-scope-summary rec=38
boundary: Instance settings file
item: statusline override
promise: Written 0600 with the absolute pinned path, only for a pinned path of plain characters.
