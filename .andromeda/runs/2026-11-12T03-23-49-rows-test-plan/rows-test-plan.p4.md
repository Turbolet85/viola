@row_none n=1 rec=1 type=5
@row_add n=1 row=contract/run-start-records/first-three area=6-e2e-test-strategy rec=2
boundary: the event log at viola run start
item: first three records
promise: The first three records are wheel{cause:"start"}, budget-gate and the hook's session-start, and the child is spawned only after them.
@row_add n=2 row=contract/snapshot/run-fields area=6-e2e-test-strategy rec=2
boundary: the instance snapshot
item: pinned path, pids and endpoint
promise: The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
@row_add n=3 row=contract/viola-run-exit-codes/1 area=6-e2e-test-strategy rec=2
boundary: viola run exit codes
item: 1
promise: A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
@row_add n=4 row=contract/generated-plugin/hook-commands area=6-e2e-test-strategy rec=2
boundary: the generated plugin
item: hook commands
promise: A restart rewrites plugin/ with absolute hook commands, never a PATH lookup, and the settings.json override.
@row_none n=2 rec=3 type=5
@row_add n=5 row=contract/viola-send/confirmed area=6-e2e-test-strategy rec=4
boundary: viola send
item: confirmed send
promise: A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
@row_add n=6 row=contract/viola-send/cr-lf-text area=6-e2e-test-strategy rec=4
boundary: viola send
item: text ending in or holding CR or LF
promise: A text ending in or holding CR or LF is confirmed as its typed text.
@row_add n=7 row=contract/viola-send/empty-text area=6-e2e-test-strategy rec=4
boundary: viola send
item: empty-text
promise: An empty typed text is refused empty-text.
@row_add n=8 row=contract/viola-send/not-delivered area=6-e2e-test-strategy rec=4
boundary: viola send
item: not-delivered
promise: A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
@row_add n=9 row=contract/viola-send/unconfirmable area=6-e2e-test-strategy rec=4
boundary: viola send
item: unconfirmable
promise: A listed local command is confirmed only by its post-condition, or else is unconfirmable.
@row_add n=10 row=contract/mcp-send/ok-keys area=6-e2e-test-strategy rec=4
boundary: the MCP send tool
item: ok result keys
promise: The MCP send result carries the same ok key set as the CLI.
@row_none n=3 rec=5 type=5
@row_add n=11 row=contract/viola-wait/wake area=6-e2e-test-strategy rec=6
boundary: viola wait
item: wake on turn-ended
promise: wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
@row_add n=12 row=contract/viola-wait/timed-out area=6-e2e-test-strategy rec=6
boundary: viola wait
item: timed_out
promise: A wait past the log end times out with {timed_out:true}.
@row_add n=13 row=contract/viola-last/newest-message area=6-e2e-test-strategy rec=6
boundary: viola last
item: newest turn message
promise: last returns the newest turn's message, and the MCP result is equal to the CLI's.
@row_add n=14 row=contract/wait-and-last/killed-wrapper area=6-e2e-test-strategy rec=6
boundary: viola wait and last
item: killed wrapper
promise: A killed wrapper gives CLI exit 21 and MCP instance-unreachable.
@row_none n=4 rec=7 type=5
@row_add n=15 row=contract/dialog-event/logged-once area=6-e2e-test-strategy rec=8
boundary: the event log
item: dialog event
promise: Each dialog event is logged exactly once with a dialog_id, and wait returns it at once.
@row_add n=16 row=contract/hook-stdout/decision-body area=6-e2e-test-strategy rec=8
boundary: hook stdout
item: decision body
promise: The hook prints the exact decision body per dialog kind, pinned by insta snapshots per CLI version.
@row_add n=17 row=contract/hook-stdout/concurrent-dialog area=6-e2e-test-strategy rec=8
boundary: hook stdout
item: second concurrent dialog
promise: A second concurrent dialog gets an empty hook stdout.
@row_add n=18 row=contract/viola-answer/unknown-dialog area=6-e2e-test-strategy rec=8
boundary: viola answer
item: 13 unknown-dialog
promise: An answer to an unknown dialog id exits 13 unknown-dialog.
@row_add n=19 row=decision/decision-body-gate area=6-e2e-test-strategy rec=8
name: Decision body gate
rule: No dialog decision body is ever emitted unless the wheel is driver and the CLI version is stamped.
@row_none n=5 rec=9 type=5
@row_add n=20 row=contract/wheel-record/human-input area=6-e2e-test-strategy rec=10
boundary: the event log
item: wheel human-input
promise: A human key logs wheel{holder:"human",cause:"human-input"} and the human's prompt still lands.
@row_add n=21 row=contract/viola-send/exit-10 area=6-e2e-test-strategy rec=10
boundary: viola send
item: 10
promise: A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
@row_add n=22 row=contract/viola-release/exit-20 area=6-e2e-test-strategy rec=10
boundary: viola release
item: 20
promise: release returns the wheel; release with from exits 20.
@row_add n=23 row=decision/injected-turn-keeps-wheel area=6-e2e-test-strategy rec=10
name: Harness-injected turn keeps the wheel
rule: A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
@row_add n=24 row=decision/no-release-in-driver-hints area=6-e2e-test-strategy rec=10,12,35
name: No release in driver-facing hints
rule: Driver-facing hints, refusal hints included, never suggest or name viola release.
guard: no_hint_names_release
@row_none n=6 rec=11 type=5
@row_add n=25 row=contract/hook-statusline/passthrough area=6-e2e-test-strategy rec=12
boundary: viola hook statusline
item: statusline output
promise: The statusline output passes through byte for byte, and budget.json holds the fed reading.
@row_add n=26 row=contract/viola-send/exit-11 area=6-e2e-test-strategy rec=12
boundary: viola send
item: 11
promise: A budget crossing logs budget-gate{paused:true} and send is refused exit 11 with its window word.
@row_add n=27 row=decision/budget-pause-threshold area=6-e2e-test-strategy rec=12
name: Budget pause threshold
rule: A budget reading of exactly 90 pauses the driver and 89 does not.
@row_add n=28 row=contract/viola-release/budget area=6-e2e-test-strategy rec=12
boundary: viola release
item: --budget
promise: release --budget lifts the budget pause for its own instance only.
@row_add n=29 row=decision/statusline-command-after-strict-modes area=6-e2e-test-strategy rec=12
name: Status-line command gated by strict modes
rule: The configured statusline_command never runs when the strict-modes check fails.
@row_none n=7 rec=13 type=5
@row_add n=30 row=decision/unverified-cli-send-only area=6-e2e-test-strategy rec=14
name: Unverified CLI degrades to send only
rule: On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
@row_add n=31 row=contract/viola-answer/unverified-cli area=6-e2e-test-strategy rec=14
boundary: viola answer
item: 12 unverified-cli
promise: answer on an unverified CLI exits 12 unverified-cli, and MCP answer refuses alike.
@row_add n=32 row=contract/viola-verify/stamped-line area=6-e2e-test-strategy rec=14
boundary: viola verify
item: stamped line
promise: verify at a stamped version exits 0 with its line stamped <ver>  <n> pass  0 fail.
@row_none n=8 rec=15 type=5
@row_none n=9 rec=16 type=5
@row_add n=33 row=contract/viola-hook/no-viola-name area=6-e2e-test-strategy rec=17,16
boundary: viola hook
item: run with no VIOLA_NAME
promise: Every hook event run with no VIOLA_NAME exits 0 with empty stdout and stderr, and no instances directory is created.
@row_add n=34 row=decision/unwrapped-name-never-a-target area=6-e2e-test-strategy rec=17
name: Unwrapped row name is never a target
rule: A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target.
guard: e1_send_naming_an_unwrapped_row_is_unreachable
@row_none n=10 rec=18 type=5
@row_none n=11 rec=19 type=5
@row_add n=35 row=decision/child-env-strip area=6-e2e-test-strategy rec=20
name: Child environment strip
rule: No stripped environment name reaches the agent child.
guard: the secret scan of the canary values
except: a name listed in claude_env_keep survives
@row_add n=36 row=decision/child-holds-no-viola-paths area=6-e2e-test-strategy rec=20
name: Child holds no viola socket or home path
rule: The agent child is given no viola socket or home path.
@row_add n=37 row=contract/process-start/claude-child area=6-e2e-test-strategy rec=20
boundary: the event log
item: process-start{subject:"claude-child"}
promise: The record carries the strip counts and the stripped names only, never their values.
@row_none n=12 rec=21 type=5
@row_none n=13 rec=22 type=5
@row_add n=38 row=decision/link-is-idempotent area=6-e2e-test-strategy rec=23,22
name: Link is idempotent
rule: Re-linking a driver to the same driven instance adds no second link record.
@row_add n=39 row=contract/event-log/unlink area=6-e2e-test-strategy rec=23
boundary: the event log
item: unlink record
promise: viola unlink logs an unlink record.
@row_none n=14 rec=24 type=5
@row_add n=40 row=decision/driver-name-self-reported area=6-e2e-test-strategy rec=25
name: Driver identity is self-reported
rule: The driver of a link is the self-reported VIOLA_NAME and need not be a booted instance.
@row_add n=41 row=decision/unlink-survives-restart area=6-e2e-test-strategy rec=25
name: Unlink survives a wrapper restart
rule: An unlink holds across a wrapper restart.
guard: links_an_unlink_survives_a_wrapper_restart
@row_add n=42 row=decision/implicit-link-on-send area=6-e2e-test-strategy rec=25
name: Implicit link on send or answer
rule: A send or answer from another instance records an implicit link.
@row_none n=15 rec=26 type=5
@row_none n=16 rec=27 type=5
@row_add n=43 row=decision/snapshot-replay-from-log area=6-e2e-test-strategy rec=28,27,39
name: Unreadable snapshot replayed from the log
rule: A start over a truncated or unknown-version snapshot replays links, the budget pause and the agent session id from the log, with no dialog pending, and writes no file.
guard: a_start_over_an_unreadable_snapshot_replays_the_log
@row_add n=44 row=decision/start-hands-wheel-to-driver area=6-e2e-test-strategy rec=28
name: Start hands the wheel to the driver
rule: Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
@row_add n=45 row=decision/start-collision-by-heartbeat area=6-e2e-test-strategy rec=28,39
name: Start collision read from the heartbeat
rule: Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not.
guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
@row_none n=17 rec=29 type=5
@row_none n=18 rec=30 type=5
@row_add n=46 row=decision/hook-suites-in-integration-layer area=6-e2e-test-strategy rec=31
name: Non-path suites layer
rule: Non-path trigger-coverage suites run in the E2E layer.
except: root hook_* files run in the integration layer
@row_add n=47 row=contract/mcp-tools/invalid-params area=6-e2e-test-strategy rec=32
boundary: MCP tools
item: invalid-params
promise: A schema-violating call is answered invalid-params naming its field, and no frame is written.
@row_add n=48 row=decision/hook-fails-open area=6-e2e-test-strategy rec=32
name: Hook fails open
rule: The hook exits 0 with no body on oversize stdin, malformed JSON, a parse error, a channel failure, a panic, a missing VIOLA_NAME, a refused instance or socket directory, or a server that is not the recorded wrapper.
guard: the hook fail-open matrix
@row_add n=49 row=decision/panic-seam-test-only area=6-e2e-test-strategy rec=32
name: Panic seam is test-only
rule: The forced-panic seam fires only under the fake-agent feature and never reaches the release build.
@row_add n=50 row=decision/error-payloads-sanitized area=6-e2e-test-strategy rec=33
name: Error payloads sanitized
rule: No error payload carries an absolute path, a serde path or a Caused by chain.
guard: tests/contract_error_bodies.rs census of writers
@row_add n=51 row=contract/json-rpc-errors/-32603 area=6-e2e-test-strategy rec=33
boundary: JSON-RPC errors
item: -32603
promise: Every -32603 is exactly "internal error" with null data.
@row_add n=52 row=decision/logs-carry-no-secrets area=6-e2e-test-strategy rec=33
name: Logs carry no secrets
rule: No log holds a Cookie header or a stripped variable's value, and the fixed canary appears in no home-level diagnostics line.
guard: viola-harness secret-scan
@row_add n=53 row=decision/diagnostics-files-owner-only area=6-e2e-test-strategy rec=33
name: Diagnostics files owner-only
rule: Diagnostics files are mode 0600 on Unix.
guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
@row_add n=54 row=decision/scan-hits-never-echo-bytes area=6-e2e-test-strategy rec=33
name: Scan hits never echo bytes
rule: A secret-scan hit is reported by place and class, never by the matched bytes.
@row_add n=55 row=decision/diagnostics-schema-conformance area=6-e2e-test-strategy rec=34
name: Diagnostics schema conformance
rule: After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step.
why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
@row_add n=56 row=decision/jsonschema-validator area=6-e2e-test-strategy rec=34
name: jsonschema validates diagnostics
rule: jsonschema is what validates diagnostics lines against their schemas.
@row_add n=57 row=decision/non-json-lines-skipped area=6-e2e-test-strategy rec=34
name: Non-JSON diagnostics lines skipped
rule: A diagnostics line that is not JSON is skipped and counted, not failed.
why: chaos cases tear lines by design
@row_add n=58 row=decision/schema-failure-no-content area=6-e2e-test-strategy rec=34
name: Schema failure report without content
rule: A schema failure prints file, line number and the failing schema keyword, never the line's content.
@row_add n=59 row=decision/hint-names-its-cause area=6-e2e-test-strategy rec=35
name: Each exit cause has its own hint
rule: Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint.
guard: no_two_codes_share_a_hint
@row_add n=60 row=contract/exit-causes/socket-dir-refused area=6-e2e-test-strategy rec=35
boundary: exit causes
item: socket-dir-refused
promise: A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
@row_add n=61 row=decision/socket-dir-refusal area=6-e2e-test-strategy rec=36
name: Socket directory refusal
rule: A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named.
guard: security_negatives_socket_dir
@row_add n=62 row=decision/umask-independent-modes area=6-e2e-test-strategy rec=36
name: Umask-independent modes
rule: File and socket modes do not depend on the umask: a clean start under an open umask keeps the socket private.
@row_add n=63 row=decision/link-name-validation area=6-e2e-test-strategy rec=36
name: Raw-channel link name validation
rule: link and unlink on the raw channel validate their names.
guard: channel_links
@row_add n=64 row=decision/home-readable-by-others-refused area=6-e2e-test-strategy rec=36
name: Home readable by others refused
rule: A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_add n=65 row=decision/single-writer-state-files area=6-e2e-test-strategy rec=36
name: Single writers of stamps and snapshot
rule: stamps.json and snapshot.json each have a single writer: every other writer-capable entry point leaves them byte-identical.
@row_add n=66 row=decision/frame-verbs-refuse-widened-instance area=6-e2e-test-strategy rec=36
name: Frame verbs refuse a widened instance
rule: The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line.
guard: security_negatives_strict_modes
@row_add n=67 row=decision/list-board-refusal-opens-no-log area=6-e2e-test-strategy rec=36
name: Board refusal opens no log
rule: viola list's whole-board strict-modes refusal exits 21 and opens no log.
guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
@row_add n=68 row=decision/plugin-install-refusal-creates-nothing area=6-e2e-test-strategy rec=36
name: Plugin install refusal creates nothing
rule: viola plugin install refused by strict modes exits 21 and creates nothing.
guard: strict_modes_on_plugin_install_is_exit_21_and_creates_nothing
@row_add n=69 row=decision/server-identity-check area=6-e2e-test-strategy rec=36
name: Server identity check
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper.
guard: security_negatives_server_identity
@row_add n=70 row=decision/list-names-escaped area=6-e2e-test-strategy rec=37
name: List names escaped
rule: Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell.
guard: cross_list_rows
@row_add n=71 row=contract/list-json/name area=6-e2e-test-strategy rec=37
boundary: list --json
item: name
promise: list --json keeps a session's original name, unescaped.
@row_add n=72 row=contract/mcp-cli-parity/refusal area=6-e2e-test-strategy rec=38
boundary: MCP and CLI parity
item: refusal
promise: For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.
@row_add n=73 row=contract/mcp-cli-parity/exit-21 area=6-e2e-test-strategy rec=38
boundary: MCP and CLI parity
item: exit 21 causes
promise: For each cause of exit 21 the MCP isError body {error, detail} equals the CLI --json document less v, its text refused: instance-unreachable.
@row_add n=74 row=decision/torn-append-healed area=6-e2e-test-strategy rec=39
name: Torn append healed
rule: A torn log append is counted and the next record starts on a fresh line.
@row_add n=75 row=contract/viola-wait/endpoint-lost area=6-e2e-test-strategy rec=39
boundary: viola wait
item: endpoint-lost
promise: An endpoint vanishing during wait exits 21 with endpoint-lost.
@row_add n=76 row=contract/viola-run/stale-heartbeat area=6-e2e-test-strategy rec=39
boundary: viola run
item: stale-heartbeat
promise: A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
@row_add n=77 row=decision/vt100-panic-contained area=6-e2e-test-strategy rec=39
name: Screen-parser panic contained
rule: A vt100 panic refuses send input-not-ready while passthrough continues on the outer terminal.
@row_add n=78 row=decision/forward-compat-lines-skipped area=6-e2e-test-strategy rec=39
name: Forward-compatible lines skipped
rule: Log lines of a newer form are counted in skipped rather than failing the read.
@row_add n=79 row=decision/property-tests-on-parsers area=6-e2e-test-strategy rec=40
name: Property tests on parsers
rule: proptest at 512 cases covers paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation.
@row_add n=80 row=decision/fuzz-separate-workspace area=6-e2e-test-strategy rec=40
name: Fuzz targets in a separate workspace
rule: cargo-fuzz targets live in a separate fuzz workspace and join as their parsers land.
@row_add n=81 row=decision/fake-agent-matches-recordings area=6-e2e-test-strategy rec=41,55
name: Fake agent matches recordings byte for byte
rule: For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
@row_add n=82 row=decision/fixtures-match-tolerant-schema area=6-e2e-test-strategy rec=41
name: Fixtures checked against the tolerant schema
rule: Every recorded fixture is checked against the tolerant parser schema.
@row_add n=83 row=decision/pinned-exe-hash-vectors area=6-e2e-test-strategy rec=41
name: Pinned-exe hash pinned by vectors
rule: The pinned-exe content hash is pinned to its library by literal test vectors.
@row_add n=84 row=decision/bounded-readers area=6-e2e-test-strategy rec=41
name: Bounded readers
rule: Every product reader of external input is bounded in-statement or named in the enumeration with a witness test; reads taking no external input are listed as exceptions.
guard: product_readers_are_each_bounded_or_named
@row_add n=85 row=decision/no-unbounded-json-depth area=6-e2e-test-strategy rec=41
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is banned.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_none n=19 rec=42 type=S
@row_none n=20 rec=43 type=S
@row_add n=86 row=decision/rstest-fixtures area=7-test-data-fixtures rec=44
name: rstest for fixtures
rule: rstest is what provides test fixtures and cases.
@row_add n=87 row=decision/tempfile-test-dirs area=7-test-data-fixtures rec=44
name: tempfile for test directories
rule: tempfile is what provides temporary test directories.
@row_add n=88 row=decision/proptest-strategies area=7-test-data-fixtures rec=44
name: proptest for generated data
rule: proptest strategies, with proptest-derive, are what generate property-test data.
@row_add n=89 row=decision/insta-snapshots area=7-test-data-fixtures rec=44
name: insta for snapshots
rule: insta is what holds snapshot expectations.
@row_add n=90 row=decision/fixtures-per-cli-version area=7-test-data-fixtures rec=44
name: Recorded fixtures per CLI version
rule: Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
@row_none n=21 rec=45 type=S
@row_add n=91 row=decision/product-state-from-real-runs area=7-test-data-fixtures rec=46
name: Product state from real runs only
rule: On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written.
except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json
@row_add n=92 row=decision/test-homes-made-private area=7-test-data-fixtures rec=46
name: Test homes made private
rule: A test that makes a home itself makes it through the product's private-directory creation, never a plain recursive create.
why: a plain home is one the product refuses by the read rule
@row_add n=93 row=decision/stamps-written-only-by-verify area=7-test-data-fixtures rec=47
name: Stamps written only by verify
rule: ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it.
@row_add n=94 row=decision/budget-file-from-statusline-hook area=7-test-data-fixtures rec=48
name: Budget file from the statusline hook
rule: budget.json comes from generated statusline stdin piped into viola hook statusline; the reader's own unit cases alter only bytes the writer wrote.
@row_add n=95 row=decision/statusline-source-read-only area=7-test-data-fixtures rec=49
name: Statusline source is read-only to viola
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_add n=96 row=decision/fake-agent-on-test-path area=7-test-data-fixtures rec=50
name: Fake agent on a test-scoped PATH
rule: The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
@row_add n=97 row=decision/hook-payloads-recorded-locally area=7-test-data-fixtures rec=51
name: Hook payloads recorded locally, replayed in CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
@row_add n=98 row=decision/dirty-payload-refuses-recording area=7-test-data-fixtures rec=51,52
name: Dirty payload refuses the recording
rule: A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
@row_add n=99 row=decision/mcp-exchange-recorded-by-tap area=7-test-data-fixtures rec=52
name: MCP exchange recorded by a test-only tap
rule: MCP exchange fixtures are recorded by the test-only viola-mcp-tap through the same scrub and refusal, schema-validated and replayed.
@row_add n=100 row=decision/human-output-as-literals area=7-test-data-fixtures rec=53
name: Human-mode output as literals
rule: Expected human-mode output, such as help text and the list board, is held as literals in its test files.
@row_add n=101 row=decision/decision-bodies-snapshotted area=7-test-data-fixtures rec=54
name: Decision bodies snapshotted
rule: Dialog decision bodies are held as insta snapshots, regenerated only by the local fixture refresh.
