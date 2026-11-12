@row_add n=1 row=contract/send-event-log/send-issued area=test-scope-summary rec=1
boundary: the send event log
item: send-issued
promise: An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
@row_add n=2 row=contract/send-confirmation/prompt-submitted-driver area=test-scope-summary rec=1
boundary: send confirmation
item: prompt-submitted with origin driver
promise: A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
@row_add n=3 row=contract/cli-send-json/exit-0 area=test-scope-summary rec=1
boundary: CLI send --json
item: exit 0
promise: A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
@row_add n=4 row=contract/cli-send-json/no-prompt-submitted area=test-scope-summary rec=1
boundary: CLI send --json
item: not-delivered / no-prompt-submitted
promise: A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
@row_add n=5 row=contract/cli-send-json/empty-text area=test-scope-summary rec=1
boundary: CLI send --json
item: not-delivered / empty-text
promise: A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
@row_add n=6 row=decision/sent-text-typed-form area=test-scope-summary rec=1,37
name: Sent text is typed in normalised form
rule: A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched.
guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
@row_add n=7 row=decision/local-command-exact-match area=test-scope-summary rec=1
name: Local commands matched by exact equality
rule: Send recognises a ledger-listed local command only by exact equality of the typed text: nothing else trimmed, no case folding, never a leading-slash test.
@row_add n=8 row=contract/send-result/unconfirmable area=test-scope-summary rec=1
boundary: the send result
item: unconfirmable
promise: A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
@row_add n=9 row=decision/local-command-post-condition area=test-scope-summary rec=1
name: Local commands confirmed by post-condition only
rule: On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
@row_add n=10 row=decision/compact-confirm-window area=test-scope-summary rec=1
name: Compact confirmation window
rule: The /compact post-condition is awaited inside its own window, COMPACT_CONFIRM_WINDOW, of 40 s.
state: provisional
@row_add n=11 row=decision/multi-line-send-one-prompt area=test-scope-summary rec=1
name: Multi-line text is one prompt
rule: A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
@row_add n=12 row=contract/mcp-send/success area=test-scope-summary rec=1
boundary: MCP send
item: success result
promise: A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
@row_add n=13 row=decision/human-input-takes-the-wheel area=test-scope-summary rec=2
name: Human input takes the wheel
rule: A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
@row_add n=14 row=contract/event-log/wheel-human-input area=test-scope-summary rec=2
boundary: the event log
item: wheel
promise: A human taking the wheel logs wheel with holder human and cause human-input.
@row_add n=15 row=contract/cli-send-json/human-typing area=test-scope-summary rec=2
boundary: CLI send --json
item: exit 10
promise: A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
@row_add n=16 row=contract/mcp-send/refused-human-typing area=test-scope-summary rec=2
boundary: MCP send
item: refused: human-typing
promise: An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
@row_add n=17 row=contract/cli-release/wheel-driver area=test-scope-summary rec=2
boundary: CLI release
item: wheel driver
promise: Release gives wheel driver, and the next send is accepted.
@row_add n=18 row=contract/cli-release/release-from-driver area=test-scope-summary rec=2
boundary: CLI release
item: release-from-driver
promise: A release carrying from is rejected with -32602 release-from-driver, exit 20.
@row_add n=19 row=decision/harness-turn-keeps-the-wheel area=test-scope-summary rec=2
name: Harness-injected turns keep the wheel
rule: A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel.
@row_add n=20 row=decision/human-never-blocked area=test-scope-summary rec=2
name: The human is never blocked
rule: Human typing always reaches the agent, whoever holds the wheel.
@row_add n=21 row=decision/budget-governor-thresholds area=test-scope-summary rec=3
name: Budget governor thresholds
rule: Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
@row_add n=22 row=decision/statusline-passthrough area=test-scope-summary rec=3
name: Statusline passthrough
rule: The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0.
@row_add n=23 row=contract/budget-json/rate-limits area=test-scope-summary rec=3,37
boundary: budget.json
item: rate_limits
promise: The statusline hook writes rate_limits to budget.json only when the payload holds a rate_limits object; a malformed resets_at becomes "unknown".
@row_add n=24 row=decision/statusline-shell-out-frame-cap area=test-scope-summary rec=3
name: Statusline shell-out frame cap
rule: The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command.
guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap
@row_add n=25 row=contract/event-log/budget-gate area=test-scope-summary rec=3
boundary: the event log
item: budget-gate
promise: A budget pause logs budget-gate with paused true and its window.
@row_add n=26 row=contract/cli-send-json/budget-paused area=test-scope-summary rec=3
boundary: CLI send --json
item: exit 11
promise: A send refused by the budget gate exits 11 with detail five-hour or seven-day.
@row_add n=27 row=contract/cli-release/budget area=test-scope-summary rec=3
boundary: CLI release --budget
item: budget_paused false
promise: Release --budget gives budget_paused false, with override_until when a pause stood.
@row_add n=28 row=decision/torn-tail-healed-by-append area=test-scope-summary rec=4,36
name: Torn log tail healed by the next append
rule: A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal.
guard: torn_tail
@row_add n=29 row=decision/snapshot-replay-fields area=test-scope-summary rec=4,36
name: Snapshot replay recovers a fixed field set
rule: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd.
guard: replay_recovers, chaos_snapshot_replay
@row_add n=30 row=decision/snapshot-read-through-replay area=test-scope-summary rec=4,36
name: Every product snapshot reader goes through read-or-replay
rule: Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
@row_add n=31 row=contract/cli-wait/endpoint-lost area=test-scope-summary rec=4
boundary: CLI wait
item: endpoint-lost
promise: An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost.
@row_add n=32 row=decision/fake-agent-contract-suite area=test-scope-summary rec=5,13
name: Fake agent held to recorded fixtures
rule: A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations.
why: guards against the fake agent drifting from the real CLI
guard: contract_fake_agent_drift
@row_add n=33 row=decision/fixtures-recorded-by-live-verify area=test-scope-summary rec=5
name: Fixtures refreshed by live verify
rule: The recorded agent fixtures are refreshed only by a live viola verify run, locally.
@row_add n=34 row=decision/unit-tests-inline area=test-strategy rec=6,14
name: Unit tests inline
rule: Unit tests live inline in a cfg(test) tests module at the bottom of each source file, in every crate and the root bin.
@row_add n=35 row=decision/rstest-case-tables area=test-strategy rec=6
name: rstest for case tables
rule: rstest case tables cover the refusal-order matrices, the exit-code table and the per-CLI-version fixture replay.
@row_add n=36 row=decision/nextest-is-the-runner area=test-strategy rec=6
name: nextest is the runner
rule: nextest is the test runner; a plain cargo test run is not supported.
@row_add n=37 row=decision/integration-suite-per-contract area=test-strategy rec=7
name: Integration suite per standard contract
rule: Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite using a real IPC endpoint, a real temporary home and the built binary as a subprocess.
@row_add n=38 row=decision/e2e-over-fake-agent area=test-strategy rec=8
name: E2E over the fake agent
rule: Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
@row_add n=39 row=decision/property-tests-on-parsers area=test-strategy rec=9
name: Property tests on every parser surface
rule: proptest covers every parser surface named by the property-test trigger, with its regressions directory committed.
@row_add n=40 row=decision/fuzz-nightly-and-corpus-gate area=test-strategy rec=9
name: Fuzzing nightly with a corpus replay gate
rule: cargo-fuzz targets run as a time-boxed nightly job, plus a corpus replay gate, with each parser getting a target as it lands.
@row_add n=41 row=decision/hook-deadlines-gated-by-hyperfine area=test-strategy rec=10
name: Hook deadlines gated by hyperfine
rule: hyperfine gates the hook deadlines on the max measurement, in a dedicated perf CI job.
@row_add n=42 row=decision/heartbeat-staleness-pure-classifier area=test-strategy rec=10,36
name: Heartbeat staleness by a pure classifier
rule: Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
@row_add n=43 row=decision/chaos-fault-sources area=test-strategy rec=11
name: Chaos fault sources
rule: Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
@row_add n=44 row=decision/mutation-at-epoch-boundary area=test-strategy rec=12
name: Mutation testing at the epoch boundary
rule: cargo-mutants runs at the epoch boundary and on demand by name only, never in the default or all run; there is no per-chunk, pre-push or CI mutation gate.
@row_add n=45 row=decision/mutation-run-verdict area=test-strategy rec=12
name: Mutation run verdict
rule: A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones.
except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
@row_add n=46 row=decision/bounded-remove-the-guard-witness area=test-strategy rec=12
name: Bounded remove-the-guard witness
rule: A control whose neutralised guard lets a product process start needs a witness bounded by a time limit.
why: nextest's default profile has no kill line, and an unbounded witness hung
@row_add n=47 row=decision/dialog-bodies-pinned-by-insta area=test-strategy rec=13
name: Dialog decision bodies pinned by insta
rule: insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
@row_add n=48 row=decision/fixtures-schema-checked area=test-strategy rec=13
name: Recorded fixtures checked against schemas
rule: jsonschema checks every recorded fixture against its kind's schema: status line, agents, MCP exchange, screen, and hook payload for every other one.
@row_add n=49 row=decision/relayed-fixtures-kept-superseded area=test-strategy rec=13
name: Relayed fixtures kept as superseded
rule: Fixtures relayed from the prototype are superseded by live recordings but stay committed with their provenance under a Superseded section.
@row_add n=50 row=decision/test-only-e2e-crate area=test-strategy rec=14
name: Test-only E2E crate
rule: One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root.
why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
@row_add n=51 row=decision/one-shared-test-helper area=test-strategy rec=14
name: One shared copy of each test helper
rule: Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
@row_add n=52 row=decision/complete-lines-reader area=test-strategy rec=14
name: Live-appended files read by complete lines
rule: Every test reader of a live-appended file parses only newline-terminated lines.
why: a Linux reader can observe an append part-way
@row_add n=53 row=decision/tests-never-read-host-sessions area=test-strategy rec=14
name: Tests never read the host's claude sessions
rule: A test's list child runs with PATH set to an empty bin dir, and the fake agent is planted as claude only through one shared helper.
@row_add n=54 row=decision/e2e-layer-by-binary-prefix area=test-strategy rec=14
name: E2E layer selected by binary prefix
rule: The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
@row_none n=1 rec=15 type=5
@row_add n=55 row=decision/ipc-vector-test-layers area=test-strategy rec=16
name: IPC vector test layers
rule: The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
@row_add n=56 row=decision/socket-directory-refusal area=test-strategy rec=16
name: Loosened socket directory refused
rule: A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
@row_add n=57 row=contract/channel-links/invalid-pair area=test-strategy rec=16
boundary: the wrapper channel
item: link and unlink with a bad name or pair
promise: A raw link or unlink frame with a bad name or pair gets -32602 and no line is written.
@row_add n=58 row=decision/paste-injection-test-layers area=test-strategy rec=17
name: Paste injection test layers
rule: Paste injection is tested by unit property tests, integration and E2E.
@row_add n=59 row=decision/hook-stdin-test-layers area=test-strategy rec=18
name: Hook stdin test layer
rule: Hook stdin is tested at integration, through the hook as a subprocess.
@row_add n=60 row=decision/mcp-test-layer area=test-strategy rec=19
name: MCP test layer
rule: MCP is tested at E2E through an rmcp client.
@row_add n=61 row=decision/cli-env-test-layers area=test-strategy rec=20
name: CLI and env test layers
rule: CLI and environment input is tested at unit and integration.
@row_add n=62 row=decision/filesystem-test-layers area=test-strategy rec=21
name: Filesystem test layers
rule: The filesystem vector is tested at integration and chaos, including strict-mode refusals.
@row_add n=63 row=contract/cli-frame-verbs/strict-modes-failed area=test-strategy rec=21
boundary: CLI frame verbs
item: strict-modes-failed
promise: Each frame verb (send, wait, last, answer, pause, release) refuses a widened instance before any log, exiting 21 with detail strict-modes-failed, no frame and no line.
@row_add n=64 row=decision/home-another-user-may-read-refused area=test-strategy rec=21
name: Readable home refused
rule: A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
@row_add n=65 row=decision/single-writer-state-files area=test-strategy rec=21
name: Single writer of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
@row_add n=66 row=decision/child-spawn-test-layers area=test-strategy rec=22
name: Child spawn test layers
rule: Child spawn is tested at integration and E2E.
@row_add n=67 row=decision/supply-chain-ci-gates area=test-strategy rec=23
name: Supply chain held by CI gates
rule: The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
@row_add n=68 row=decision/property-trigger-layer area=test-strategy rec=24
name: Property-test trigger layer
rule: The property-test trigger is met by proptest and the fuzz job.
@row_add n=69 row=decision/chaos-trigger-layer area=test-strategy rec=25
name: Chaos-test trigger layer
rule: The chaos-test trigger is met by a chaos suite.
@row_add n=70 row=decision/performance-trigger-layer area=test-strategy rec=26
name: Performance-budget trigger layer
rule: The performance-budget trigger is met by perf budget tests.
@row_add n=71 row=decision/cross-surface-trigger-layer area=test-strategy rec=27
name: Cross-surface trigger layer
rule: Cross-surface coordination is tested at E2E, with cross-surface parity checks.
@row_add n=72 row=decision/single-os-ci-leg area=test-strategy rec=28
name: One CI leg on Linux
rule: The CI matrix builds on one leg, ubuntu-latest.
@row_add n=73 row=decision/os-gated-tests-registered area=test-strategy rec=28
name: OS-gated tests registered with a reason
rule: Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=74 row=decision/multi-version-trigger-layers area=test-strategy rec=29
name: Multi-version compatibility layers
rule: Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
@row_add n=75 row=decision/sandbox-contract-trigger-layer area=test-strategy rec=30
name: Sandbox contract trigger layer
rule: The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
@row_add n=76 row=decision/error-bodies-hold-no-path area=test-strategy rec=31
name: Error bodies hold no path or upstream text
rule: CLI error documents, channel error bodies and MCP tool error results hold no path or upstream text, checked on each surface and by an enumeration of error documents with a census of writers.
guard: cli_error_documents_hold_no_path_or_upstream_text, contract_error_bodies
@row_add n=77 row=decision/mutation-trigger-layer area=test-strategy rec=32
name: Mutation trigger layer
rule: Mutation testing is met by the mutation run.
@row_add n=78 row=decision/discipline-at-every-layer area=test-strategy rec=33
name: Discipline held at every layer
rule: No screen parsing, no blanket approval and no flaky tests hold at every test layer.
@row_add n=79 row=contract/code-word/format area=unit-test-strategy rec=34
boundary: code words
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_add n=80 row=contract/unreachable-cause/socket-dir-refused area=unit-test-strategy rec=34
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
@row_add n=81 row=contract/json-rpc-errors/-32700 area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: -32700
promise: Parse error is reported as -32700.
@row_add n=82 row=contract/json-rpc-errors/-32600 area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: -32600
promise: Invalid request is reported as -32600.
@row_add n=83 row=contract/json-rpc-errors/-32601 area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: -32601
promise: Unknown method is reported as -32601.
@row_add n=84 row=contract/json-rpc-errors/-32602 area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: -32602
promise: Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart.
@row_add n=85 row=contract/json-rpc-errors/-32603 area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: -32603
promise: Internal error is reported as -32603 with the exact message "internal error" and data null.
@row_add n=86 row=contract/json-rpc-errors/wrapper-fault area=unit-test-strategy rec=35
boundary: wrapper channel JSON-RPC errors
item: a code outside the five
promise: A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code.
@row_add n=87 row=contract/channel-endpoint/socket-path area=unit-test-strategy rec=35
boundary: the wrapper channel endpoint
item: socket path
promise: The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
@row_add n=88 row=decision/peer-euid-admission area=unit-test-strategy rec=35
name: Only the owning user's peer is admitted
rule: The wrapper admits a channel peer only when its observed euid equals its own; a peer whose euid cannot be read is refused, and a refused peer is dropped before any frame, with no line written.
guard: serve_drops_a_refused_peer_before_any_frame
@row_add n=89 row=decision/defensive-log-reader area=unit-test-strategy rec=36
name: Event log reader counts and never panics
rule: The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned.
guard: three_counts
@row_add n=90 row=decision/server-identity-check area=unit-test-strategy rec=36
name: Server identity check
rule: A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint.
guard: server_verdict_rows
@row_add n=91 row=decision/strict-modes-two-rules area=unit-test-strategy rec=36
name: Strict modes write and read rules
rule: Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule.
guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
@row_add n=92 row=decision/socket-directory-check area=unit-test-strategy rec=36
name: Socket directory check
rule: The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes.
guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
@row_add n=93 row=decision/stamps-refuse-widened-ledger area=unit-test-strategy rec=36
name: Stamps refuse a widened ledger
rule: Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing.
guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
@row_add n=94 row=decision/private-placement-mode-first area=unit-test-strategy rec=36
name: Private copies placed mode-first
rule: A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name.
guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
@row_add n=95 row=decision/bounded-compare area=unit-test-strategy rec=36
name: Bounded compare of a shared target
rule: Comparing a target to known bytes reads no more than one byte past their length, never the whole file.
guard: replace_private_shared_reads_no_more_than_one_byte_past_the_bytes
@row_add n=96 row=decision/hook-unknown-fields-tolerated area=unit-test-strategy rec=37
name: Unknown hook payload fields tolerated
rule: Unknown fields in a hook payload are counted, never fatal.
@row_add n=97 row=decision/dialog-answer-mapping area=unit-test-strategy rec=37
name: Dialog answer mapping
rule: Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny.
@row_add n=98 row=decision/claude-env-strip area=unit-test-strategy rec=37
name: Agent identity environment stripped
rule: Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names.
guard: an R8 test against a literal copy of the floor, never imported from the product
@row_add n=99 row=contract/dialog-data/null-details area=unit-test-strategy rec=37
boundary: dialog and Stop event data
item: description, preview and background_tasks
promise: A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array.
@row_add n=100 row=decision/version-gate-needs-all-relied-rows area=unit-test-strategy rec=37
name: Version gate needs every relied row
rule: A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified.
guard: stamps_verdict_needs_the_dialog_rows
@row_add n=101 row=decision/agents-parser-fails-to-unknown area=unit-test-strategy rec=37
name: Agents list parse failure reads unknown
rule: An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
@row_add n=102 row=contract/prompt-submitted/origin-harness area=unit-test-strategy rec=37
boundary: the prompt-submitted event
item: origin harness
promise: A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human.
@row_add n=103 row=decision/screen-signatures-only area=unit-test-strategy rec=37
name: Screen read only for compiled signatures
rule: The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
@row_add n=104 row=contract/readiness-gate/gate-max-wait area=unit-test-strategy rec=37
boundary: the readiness gate
item: GATE_MAX_WAIT
promise: On a verified CLI a quiet screen with no input-box literal waits up to 8 500 ms and reads input-not-ready from that bound on.
@row_add n=105 row=decision/vt100-feed-panic-safe area=unit-test-strategy rec=37
name: Screen feed panic-safe
rule: The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready.
@row_add n=106 row=decision/mcp-stdin-line-cap area=unit-test-strategy rec=38
name: MCP stdin line cap
rule: The MCP server's stdin yields whole lines of at most MAX_FRAME bytes; a longer line releases no byte, sets the cut flag and reads as end of stream, ending a session before initialize as the initialize error.
guard: capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more
@row_add n=107 row=contract/mcp-reply/screened-words area=unit-test-strategy rec=38
boundary: MCP tool replies
item: refusal and detail words
promise: A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.