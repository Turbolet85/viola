@row_add n=1 row=absence/database area=integration-test-strategy rec=1
capability: A database

@row_add n=2 row=decision/on-disk-state-integration-tests area=integration-test-strategy rec=1
name: On-disk state integration tests
rule: Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests.
why: The project has no database.

@row_add n=3 row=decision/concurrent-append-whole-lines area=integration-test-strategy rec=1
name: Whole lines under concurrent append
rule: Separate processes appending at once to each shared diagnostics file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; a torn or interleaved line fails the test.
guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side

@row_add n=4 row=absence/outbound-http area=integration-test-strategy rec=2,13,17
capability: Outbound HTTP, so no HTTP mocking library and no HTTP fake server

@row_add n=5 row=decision/fake-agent-as-claude area=integration-test-strategy rec=2
name: Fake agent stands in for the claude CLI
rule: The child-process boundary to the agent CLI is tested by copying viola-fake-agent to a test-scoped PATH as claude, serving claude agents --json in recorded, oversize and malformed modes.

@row_add n=6 row=decision/channel-tests-real-endpoint area=integration-test-strategy rec=3
name: Channel tests on a real endpoint
rule: Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.

@row_add n=7 row=decision/squatter-via-server-api area=integration-test-strategy rec=3,17
name: Squatter listener through the server API
rule: A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.

@row_add n=8 row=decision/owner-only-socket-modes area=integration-test-strategy rec=3
name: Owner-only socket modes
rule: The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.

@row_add n=9 row=decision/socket-dir-check area=integration-test-strategy rec=3
name: Socket directory check before use
rule: The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods.
guard: security_negatives_socket_dir

@row_add n=10 row=decision/channel-client-no-dir-check area=integration-test-strategy rec=3
name: Channel client carries no directory check
rule: The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check.
why: viola-channel has no viola-state edge.
guard: channel_link_names_are_validated_by_the_wrapper

@row_add n=11 row=decision/pty-tests-real-openpty area=integration-test-strategy rec=4
name: PTY tests on a real openpty
rule: PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.

@row_add n=12 row=decision/resize-baseline-is-spawn-size area=integration-test-strategy rec=4
name: Pump resize baseline is the spawn size
rule: The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child.
why: A resize can land between the spawn sizing and the pump's first look.
guard: pump_forwards_a_resize_that_lands_before_its_first_look

@row_add n=13 row=decision/key-after-resize-waits-for-size area=integration-test-strategy rec=4
name: Key after a resize waits for the new size
rule: A test that writes a key after a resize sends it only once the child reports the new size.
guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code

@row_add n=14 row=decision/hook-tests-real-binary area=integration-test-strategy rec=5
name: Hook tests run the real binary
rule: Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json.

@row_add n=15 row=decision/hook-instance-check-first area=integration-test-strategy rec=5
name: Hook instance check comes first
rule: The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing

@row_add n=16 row=decision/hook-line-cases-in-made-instance-dir area=integration-test-strategy rec=5
name: Hook line cases run in a viola-made instance dir
rule: Hook test cases that expect lines run in a viola-made instance directory.
why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.

@row_add n=17 row=decision/placed-copy-execs-at-once area=integration-test-strategy rec=6
name: Placed copy closed before it is persisted
rule: A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement.
why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY).
guard: place_private_new_placed_copy_execs_at_once

@row_none n=1 rec=7 type=5

@row_add n=18 row=decision/fresh-home-per-test area=integration-test-strategy rec=8
name: Fresh viola-made home per test
rule: Each test gets a fresh, not-yet-existing home path inside a temp parent under the workspace's e2e-home target directory, which viola creates itself; nothing is shared between tests.
why: So obs-plan's CI gates, secret scan and upload cover integration runs too.
except: The chaos feed panic test alone boots in a home under the system temp dir, outside G2, G4 and the secret scan, because its forced panic writes a G2-counted panic line it asserts itself.

@row_add n=19 row=decision/guarded-paths-start-stamped area=integration-test-strategy rec=8
name: Check-guarded cases start stamped
rule: A test case that drives a check-guarded path starts from a stamped home.

@row_add n=20 row=decision/wrapper-drop-guard area=integration-test-strategy rec=8
name: Test wrappers held in a Drop guard
rule: Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.

@row_add n=21 row=decision/private-dir-for-test-endpoints area=integration-test-strategy rec=8,17
name: Test endpoint directories made private
rule: A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir.
why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.

@row_add n=22 row=decision/readiness-stall-report area=integration-test-strategy rec=8
name: Readiness stall report
rule: When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing.
why: So the report reaches the JUnit.

@row_add n=23 row=contract/wrapper-channel/params-v-higher area=integration-test-strategy rec=9
boundary: Wrapper channel
item: params.v higher than served
promise: A request whose params.v is higher than the served version is answered with error -32602.

@row_add n=24 row=contract/wrapper-channel/frame-over-max-frame area=integration-test-strategy rec=9
boundary: Wrapper channel
item: frame over MAX_FRAME
promise: A frame over MAX_FRAME is answered with error -32600 and the connection is closed; a pending dialog stays pending.

@row_add n=25 row=contract/wrapper-channel/cursor area=integration-test-strategy rec=9
boundary: Wrapper channel
item: cursor
promise: A result's cursor equals the events.ndjson length at acceptance.

@row_add n=26 row=decision/server-identity-verified area=integration-test-strategy rec=9
name: Server identity verified before any frame
rule: A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed.
guard: security_negatives_server_identity

@row_add n=27 row=contract/mcp-unreachable-detail/strict-modes-failed area=integration-test-strategy rec=9,10
boundary: MCP frame tools instance-unreachable detail
item: strict-modes-failed
promise: A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.

@row_add n=28 row=contract/mcp-unreachable-detail/socket-dir-refused area=integration-test-strategy rec=9
boundary: MCP frame tools instance-unreachable detail
item: socket-dir-refused
promise: A refused socket directory check after the live endpoint is read gives detail socket-dir-refused, with no frame and no log line.

@row_add n=29 row=contract/mcp-unreachable-detail/server-verify-failed area=integration-test-strategy rec=9
boundary: MCP frame tools instance-unreachable detail
item: server-verify-failed
promise: A peer euid the admission refuses, or a failed server verification, gives detail server-verify-failed, with no frame and no log line.

@row_add n=30 row=contract/mcp-unreachable-detail/instance-dead area=integration-test-strategy rec=9
boundary: MCP frame tools instance-unreachable detail
item: instance-dead
promise: A failed connect gives detail instance-dead, with no frame and no log line.

@row_add n=31 row=contract/mcp-unreachable-detail/endpoint-lost area=integration-test-strategy rec=9
boundary: MCP frame tools instance-unreachable detail
item: endpoint-lost
promise: A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost.

@row_add n=32 row=contract/mcp-frame-tools/refusal-result area=integration-test-strategy rec=9
boundary: MCP frame tools
item: refusal result
promise: A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.

@row_add n=33 row=contract/mcp-frame-tools/free-text-refusal area=integration-test-strategy rec=9
boundary: MCP frame tools
item: refusal of free text
promise: A wrapper refusal of free text reads as unknown, with no detail.

@row_add n=34 row=decision/paste-validated-at-both-ends area=integration-test-strategy rec=9
name: Sent text validated at client and wrapper
rule: Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper.
guard: channel_paste_validation

@row_add n=35 row=contract/wrapper-channel/control-character area=integration-test-strategy rec=9
boundary: Wrapper channel
item: refusal detail control-character
promise: send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, ahead of every other refusal, with no send-issued record and no prompt typed.

@row_add n=36 row=contract/wrapper-channel/empty-text area=integration-test-strategy rec=9
boundary: Wrapper channel
item: refusal detail empty-text
promise: An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.

@row_add n=37 row=contract/wrapper-channel/accepted-text area=integration-test-strategy rec=9
boundary: Wrapper channel
item: text with LF, CR, TAB and multi-byte characters
promise: Text with LF, a lone inner CR, TAB and multi-byte characters is accepted and typed as one prompt, its inner CR as one LF.

@row_add n=38 row=contract/cli-exit-codes/13-text-refused area=integration-test-strategy rec=9
boundary: CLI exit codes
item: 13 for refused text
promise: viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.

@row_add n=39 row=contract/viola-answer/oversize-stdin area=integration-test-strategy rec=9
boundary: viola answer
item: stdin over MAX_FRAME
promise: Exits 2 before connecting, with the fixed unusable-response line on stderr whatever --json says and no document; the dialog stays pending.

@row_add n=40 row=decision/bad-config-value-falls-back area=integration-test-strategy rec=9,11
name: Bad config value falls back to its default
rule: A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it.
guard: setting_does_not_disable_the_budget_gate

@row_add n=41 row=contract/channel-log/link-existing-pair area=integration-test-strategy rec=9
boundary: Wrapper channel log
item: link of an existing pair
promise: Linking an already linked pair adds no log line.

@row_add n=42 row=contract/channel-log/unserved-method area=integration-test-strategy rec=9
boundary: Wrapper channel log
item: response line of an unserved method
promise: The conn-less response line of an unserved method carries no method key; method names only the closed method list.

@row_add n=43 row=contract/event-line/keys area=integration-test-strategy rec=10
boundary: Event line
item: keys
promise: Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.

@row_add n=44 row=contract/event-line/ts area=integration-test-strategy rec=10
boundary: Event line
item: ts
promise: RFC 3339 UTC with milliseconds and Z.

@row_add n=45 row=contract/event-data/link area=integration-test-strategy rec=10
boundary: Event data keys
item: link
promise: data holds the keys driver and driven.

@row_add n=46 row=contract/event-data/unlink area=integration-test-strategy rec=10
boundary: Event data keys
item: unlink
promise: data holds the keys driver and driven.

@row_add n=47 row=contract/event-data/dialog-expired area=integration-test-strategy rec=10
boundary: Event data keys
item: dialog-expired
promise: data holds the keys dialog_id and dialog_kind.

@row_add n=48 row=contract/event-data/turn-ended area=integration-test-strategy rec=10
boundary: Event data keys
item: turn-ended
promise: data holds the keys last_assistant_message and background_tasks.

@row_add n=49 row=contract/event-data/question area=integration-test-strategy rec=10
boundary: Event data keys
item: question
promise: option_details is a known data key of a question line.

@row_add n=50 row=decision/tolerant-event-reader area=integration-test-strategy rec=10
name: Tolerant event log reader
rule: The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.

@row_add n=51 row=decision/snapshot-replayed-from-log area=integration-test-strategy rec=10
name: Broken snapshot replayed from the log
rule: A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.

@row_add n=52 row=decision/link-since-kept area=integration-test-strategy rec=10
name: Link keeps its first since
rule: A linked pair keeps its first since until an unlink, and takes a later one after it.

@row_add n=53 row=decision/tailing-ignores-symlinks area=integration-test-strategy rec=10
name: Tailing ignores symlinks and invalid names
rule: Event tailing ignores symlinks and invalid names.

@row_add n=54 row=contract/strict-modes-refusal/run area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: viola run
promise: Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.

@row_add n=55 row=contract/strict-modes-refusal/mcp area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: viola mcp
promise: Exit 1.

@row_add n=56 row=contract/strict-modes-refusal/revive area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: viola revive
promise: Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.

@row_add n=57 row=contract/strict-modes-refusal/verify area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: viola verify
promise: Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged.

@row_add n=58 row=contract/strict-modes-refusal/hook area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: viola hook
promise: Exit 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.

@row_add n=59 row=contract/strict-modes-refusal/other-verbs area=integration-test-strategy rec=10
boundary: Strict-modes refusal
item: every other CLI verb
promise: Exit 21.

@row_add n=60 row=decision/instance-check-before-role-file area=integration-test-strategy rec=10
name: Instance check before any role file or frame
rule: viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found.
why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home.
guard: link_verbs_refuse_a_widened_instance_before_any_log

@row_add n=61 row=contract/cli-json-error/instance-unreachable area=integration-test-strategy rec=10
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.

@row_add n=62 row=decision/widened-home-tree-refused-at-start area=integration-test-strategy rec=10
name: Widened home tree refused at start
rule: viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found.
guard: run_refuses_a_widened_home_tree_before_its_log

@row_add n=63 row=decision/home-read-rule area=integration-test-strategy rec=10
name: Home readable by another user is refused
rule: A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing.
guard: a_home_another_user_may_read_refuses_run

@row_add n=64 row=contract/cli-json/one-document-per-verb area=integration-test-strategy rec=11
boundary: CLI --json
item: one document per run
promise: Each driver-facing verb prints one --json document and its literal exit code, for every exit code.

@row_add n=65 row=decision/human-output-pinned-by-literals area=integration-test-strategy rec=11
name: Human output pinned by literals
rule: Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.

@row_add n=66 row=decision/no-cli-snapshot-crates area=integration-test-strategy rec=11
name: No CLI snapshot crates
rule: No manifest holds assert_cmd, predicates, trycmd or snapbox, and there is no command fixture directory.

@row_add n=67 row=decision/no-escape-in-plain-output area=integration-test-strategy rec=11
name: No escape bytes in plain output
rule: No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE.
guard: cli_output_plain

@row_add n=68 row=decision/help-no-bold-underline area=integration-test-strategy rec=11
name: Help carries no bold or underline
rule: viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF.
guard: cli_output_plain

@row_add n=69 row=contract/viola-help/verb-table area=integration-test-strategy rec=11
boundary: viola --help
item: grouped verb table
promise: Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty.

@row_add n=70 row=contract/verify-output/step-line area=integration-test-strategy rec=11
boundary: viola verify output
item: step line
promise: One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.

@row_add n=71 row=contract/verify-output/summary-line area=integration-test-strategy rec=11
boundary: viola verify output
item: summary line
promise: The last stdout line is stamped <ver>  <n> pass  <m> fail.

@row_add n=72 row=contract/verify-stop/signal area=integration-test-strategy rec=11
boundary: viola verify interrupted
item: SIGINT or SIGTERM
promise: Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.

@row_add n=73 row=contract/verify-exit-detail/program-not-found area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: program-not-found
promise: Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found.

@row_add n=74 row=contract/verify-exit-detail/pinned-hash-mismatch area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.

@row_add n=75 row=contract/verify-exit-detail/version-unreadable area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: version-unreadable
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.

@row_add n=76 row=contract/verify-exit-detail/strict-modes-failed area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses.

@row_add n=77 row=contract/verify-exit-detail/unclean-fixture area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: unclean-fixture
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.

@row_add n=78 row=contract/verify-exit-detail/interrupted area=integration-test-strategy rec=11
boundary: viola verify process-exit detail
item: interrupted
promise: Written on verify's process-exit when verify is stopped by a signal.

@row_add n=79 row=decision/verify-failing-row-no-refusal area=integration-test-strategy rec=11
name: Failing ledger row is no refusal
rule: A failing ledger row in viola verify is no refusal and carries no code.

@row_add n=80 row=decision/verify-takes-no-json area=integration-test-strategy rec=11
name: viola verify takes no --json
rule: viola verify takes no --json flag.

@row_add n=81 row=decision/plugin-marketplace-mcp-only area=integration-test-strategy rec=11
name: Plugin marketplace is MCP-only
rule: viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere.
guard: plugin_install_writes_the_mcp_only_marketplace

@row_add n=82 row=decision/plugin-install-owner-only area=integration-test-strategy rec=11
name: Plugin install writes owner-only
rule: viola plugin install writes its tree at 0700 / 0600 whatever the umask, and refuses a widened home with exit 21, leaving the home as found.
guard: plugin_install_writes_owner_only_modes

@row_add n=83 row=contract/channel-response-detail/unsupported-version area=integration-test-strategy rec=11
boundary: channel-response line detail
item: unsupported-version
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.

@row_add n=84 row=contract/channel-response-detail/invalid-params area=integration-test-strategy rec=11
boundary: channel-response line detail
item: invalid-params
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.

@row_add n=85 row=contract/channel-response-detail/release-from-driver area=integration-test-strategy rec=11
boundary: channel-response line detail
item: release-from-driver
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.

@row_add n=86 row=decision/no-names-in-wrapper-log area=integration-test-strategy rec=11
name: No session names in the wrapper log
rule: The wrapper's process log carries no driver or driven name, and channel lines carry no name or param.
guard: link_verbs_log_the_channel_pair_and_no_names

@row_add n=87 row=decision/leading-slash-never-a-prompt area=integration-test-strategy rec=11
name: Leading-slash argument is never a prompt
rule: A leading-slash argument in the target or text position of send or answer is never delivered as a prompt: it is a usage error, exit 2.
guard: cli_slash_args

@row_add n=88 row=decision/controls-not-disableable area=integration-test-strategy rec=11
name: Controls are not disableable
rule: No VIOLA_* variable, global flag or config.json key disables a control: each control negative gets the same verdict under every disabling-shaped setting.
guard: every_flag_variable_and_config_key_is_a_row_of_the_table

@row_add n=89 row=decision/home-resolution-order area=integration-test-strategy rec=11
name: Home resolution order
rule: The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists.
why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening.
guard: home_resolution_takes_the_flag_then_the_session_then_the_default

@row_add n=90 row=decision/test-viola-drops-session-vars area=integration-test-strategy rec=11
name: Test-started viola drops session variables
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.

@row_add n=91 row=decision/ledger-probe-contract-suite area=integration-test-strategy rec=11
name: Ledger probe contract suite
rule: For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list.
guard: contract_ledger_probes

@row_add n=92 row=decision/time-faked-through-seams area=mocking-stubbing-discipline rec=12,14
name: Time faked through injected clocks
rule: Sync crates fake time with mock_instant or the injected Clock, MCP adapter timers use tokio's paused test clock, and a pure decision takes its instant as an argument.

@row_add n=93 row=decision/wrapper-deadline-injected-clock area=mocking-stubbing-discipline rec=12
name: Wrapper deadlines driven by the injected clock
rule: The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.

@row_add n=94 row=decision/utc-now-through-clock-seam area=mocking-stubbing-discipline rec=12
name: Wall clock read through the clock seam
rule: chrono Utc::now() is read only through the clock seam.
except: The budget gate's evaluation points read it and hand the instant to its pure decision; the link line's since is stamped directly, so tests assert since by shape only.

@row_add n=95 row=decision/mockall-on-seam-traits-only area=mocking-stubbing-discipline rec=15
name: mockall on seam traits only
rule: mockall doubles are generated only on seam traits, such as the Pty, liveness and Clock traits, and only under test.

@row_add n=96 row=decision/external-processes-real-stubs area=mocking-stubbing-discipline rec=15
name: External processes are real stub executables
rule: External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.

@row_add n=97 row=decision/impostor-holder-real-process area=mocking-stubbing-discipline rec=15
name: Impostor's held process is real
rule: A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard.

@row_add n=98 row=decision/real-process-faults area=mocking-stubbing-discipline rec=16,11
name: Process faults are real
rule: A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.

@row_add n=99 row=decision/impostor-server-real-listener area=mocking-stubbing-discipline rec=17
name: Impostor server is a real listener
rule: Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process.

@row_add n=100 row=decision/hook-unit-impostor-varies-start-time area=mocking-stubbing-discipline rec=17
name: Hook unit impostor varies the start time
rule: The hook's unit impostor varies the recorded start time, a second early or an unparseable started_at, instead of using a second process.
why: A root-bin unit test has no portable second process.

@row_add n=101 row=decision/real-filesystem-faults area=mocking-stubbing-discipline rec=18
name: Filesystem faults are real
rule: A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found.
except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.

@row_add n=102 row=decision/forced-window-hook-is-test-parameter area=mocking-stubbing-discipline rec=19
name: Forced-window hook is a test parameter
rule: A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.

@row_add n=103 row=decision/peer-euid-admission area=mocking-stubbing-discipline rec=20
name: Peer admitted only on its own euid
rule: A channel peer is admitted only when its observed euid equals the server's own; an unreadable euid is refused.

@row_add n=104 row=decision/peer-admission-by-injection area=mocking-stubbing-discipline rec=20
name: Peer admission tested by injection
rule: The peer euid decision takes its euids as arguments and the private serve function takes its admission as an argument, with no env, flag or feature seam.

@row_add n=105 row=decision/coverage-includes-child-processes area=quality-gates-coverage-targets rec=21
name: Coverage includes child viola processes
rule: Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE.
except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.

@row_add n=106 row=decision/fake-agent-own-process-group area=quality-gates-coverage-targets rec=21
name: Fake agent spawns in its own process group
rule: Every command the fake agent spawns runs in a process group of its own.
why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.

@row_add n=107 row=decision/killed-test-child-uninstrumented area=quality-gates-coverage-targets rec=21
name: Killed test child is uninstrumented
rule: A test that kills its child straight after the start uses an uninstrumented host program as that child.
why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.

@row_add n=108 row=decision/profraw-census-dev-host-only area=quality-gates-coverage-targets rec=21
name: Profraw census is a dev-host tool
rule: The profraw census script is a dev-host tool: no CI job, pre-push stage or harness command runs it.

@row_add n=109 row=decision/mutation-gate-at-epoch-boundary area=quality-gates-coverage-targets rec=22
name: Mutation gate at the epoch boundary
rule: Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.

@row_add n=110 row=decision/mutation-gate-thresholds area=quality-gates-coverage-targets rec=22
name: Mutation gate thresholds
rule: A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out.
why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.

@row_add n=111 row=contract/mutants-verdict/no-rust-delta area=quality-gates-coverage-targets rec=22
boundary: Mutation run verdict
item: no-rust-delta
promise: A diff naming no .rs path never reaches cargo-mutants and passes with tested 0, the diff and its file count.

@row_add n=112 row=contract/mutants-verdict/test-only-rust-delta area=quality-gates-coverage-targets rec=22
boundary: Mutation run verdict
item: test-only-rust-delta
promise: A diff whose .rs paths are all test targets passes only with this verdict, naming the diff, its file count and rust_files.

@row_add n=113 row=contract/mutants-verdict/counted area=quality-gates-coverage-targets rec=22
boundary: Mutation run verdict
item: counted
promise: Any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.

@row_add n=114 row=contract/mutants-verdict/unviable-exceeds-caught area=quality-gates-coverage-targets rec=22
boundary: Mutation run verdict
item: unviable-exceeds-caught
promise: The run is red when unviable mutants outnumber caught ones.

@row_add n=115 row=contract/mutants-verdict/package area=quality-gates-coverage-targets rec=22
boundary: Mutation run verdict
item: package
promise: A whole member scored by run --mutants --package <member>, under the same missed, timeout and unviable thresholds.

@row_add n=116 row=decision/doctest-only-catch-is-missed area=quality-gates-coverage-targets rec=22
name: Doctest-only catch counts as missed
rule: A mutant caught only by a doctest counts as missed, and the missing unit test must be added.

@row_add n=117 row=decision/host-unreachable-mutants area=quality-gates-coverage-targets rec=22
name: Host-unreachable mutants recorded, not caught
rule: A mutant this host cannot compile or reach is recorded not measured here, owed to the route entry that measures it, never equivalent and never caught; one wholly under a cfg proved false for the host is left out of the count with its predicate.

@row_add n=118 row=decision/peer-admitted-survivor-known-limit area=quality-gates-coverage-targets rec=22
name: Peer admission survivor is a known limit
rule: A mutant making a client seam's peer admission always admit survives every committed test and stands as a known limit, read at each epoch-boundary audit.
why: One account can make no foreign peer, and no test seam may be added.

@row_add n=119 row=decision/report-files-per-test-process area=quality-gates-coverage-targets rec=22
name: Report files per test process
rule: Test report files carry the test process's id, so two concurrent jobs never share one.

@row_add n=120 row=decision/mutation-harness-tests-ungated area=quality-gates-coverage-targets rec=22
name: Real cargo-mutants harness tests ungated
rule: The two real-cargo-mutants harness tests run ungated, never ignored, and the CI profile's 120 s kill and zero retries stay unchanged.
