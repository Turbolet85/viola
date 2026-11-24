# test-plan · the rows that bind 2026-11-24-switching-a-session-s-model-and-effort — picked 385 of 1385 (246 by every draw) · rule any · pin 4716

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes and oversize answer JSON are each covered by negative tests that show them refused.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/ipc-vector-negative-tests · IPC security vector negative tests — The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/peer-euid-checked · Peer of another euid dropped — A peer of another euid is dropped by the wrapper before any frame, and a client reads server-verify-failed.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including every identity-floor name. · except: Persistent-environment names listed in config.json claude_env_keep are kept.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI.
decision/row-name-never-a-target · Agent row name never a target — An agent row name is never usable as a target.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/instance-liveness/gone · instance liveness · gone — An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
contract/instance-liveness/live · instance liveness · live — An instance whose process runs and whose heartbeat is at most 5 s old is live.
contract/instance-liveness/stale · instance liveness · stale — An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
contract/instance-snapshot/pid · instance snapshot.json · pid — The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
contract/unix-socket-permissions/directory · Unix socket permissions · directory — The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
contract/unix-socket-permissions/file · Unix socket permissions · file — The Unix socket file is mode 0600.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
decision/agents-json-fallback-unknown · Unreadable agents listing reads unknown — Oversize or malformed claude agents --json output is read as unknown.
decision/cross-user-tested-at-peer-check · Cross-user attacks tested at the peer-credential check — Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test. · why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
contract/channel-errors/32600 · channel errors · -32600 — A frame over MAX_FRAME is answered with -32600 and the connection is closed.
contract/channel-errors/32602 · channel errors · -32602 — A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper.
contract/channel-errors/32603 · channel errors · -32603 — The message is exactly "internal error" and data is null.
contract/cli-exit-codes/14 · CLI exit codes · 14 — A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window? {address, workspace}, origin? cli|agent}.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
contract/wait/instance-unreachable · wait · unreachable wrapper — A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt or unsupported-v snapshot is replayed from the event log, recovering only links, agent_session_id, wheel, lineage, launch and window with dialog_pending false, the last such line winning (a later lineage line with no parent clears it); the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
contract/cli-frame-verbs/server-verify-failed · CLI frame verbs · server-verify-failed — A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.
contract/instance-unreachable-cause/endpoint-lost · instance-unreachable causes · endpoint-lost — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/instance-dead · instance-unreachable causes · instance-dead — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/server-verify-failed · instance-unreachable causes · server-verify-failed — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable causes · strict-modes-failed — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/unwrapped-name · instance-unreachable causes · unwrapped-name — A closed cause word of instance-unreachable's detail.
contract/revive-refusal-code/already-live · revive refusal codes · already-live — A closed code of the revive --json refusal document.
contract/revive-refusal-code/cwd-missing · revive refusal codes · cwd-missing — A closed code of the revive --json refusal document.
contract/revive-refusal-code/internal-error · revive refusal codes · internal-error — A closed code of the revive --json refusal document.
contract/revive-refusal-code/no-session · revive refusal codes · no-session — A closed code of the revive --json refusal document; also the document of an empty --list chain.
contract/revive-refusal-code/pinned-hash-mismatch · revive refusal codes · pinned-hash-mismatch — A closed code of the revive --json refusal document.
contract/revive-refusal-code/session-live · revive refusal codes · session-live — A closed code of the revive --json refusal document.
contract/revive-refusal-code/socket-dir-refused · revive refusal codes · socket-dir-refused — A closed code of the revive --json refusal document.
contract/revive-refusal-code/squatted-name · revive refusal codes · squatted-name — A closed code of the revive --json refusal document.
contract/revive-refusal-code/stale-heartbeat · revive refusal codes · stale-heartbeat — A closed code of the revive --json refusal document.
contract/revive-refusal-code/strict-modes-failed · revive refusal codes · strict-modes-failed — A closed code of the revive --json refusal document.
contract/run-refusal/already-live · run and revive start refusals · already-live — A live holder or a fresh replayed beat refuses the start with exit 1.
contract/run-refusal/socket-dir-refused · run and revive start refusals · socket-dir-refused — A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
contract/run-refusal/stale-heartbeat · run and revive start refusals · stale-heartbeat — A stale holder refuses the start with exit 1.
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then, for a start under --origin, one window line with source wrapper, then the agent's session-start with source hook.
contract/wrapper-channel/-32603 · Wrapper channel · -32603 — internal error with data null.
contract/wrapper-channel/protocol-fault · Wrapper channel · protocol fault — {"error":{code,message,data}}.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}}
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
decision/run-start-verify-and-trust-after-bind · viola run's verify and trust steps after the bind — Between its exclusive endpoint bind and its first snapshot, a viola run start runs any automatic verify and then, on a verified final verdict, writes an untrusted spawn folder's trust record.
decision/torn-tail-healed-by-append · Torn log tail healed by the next append — A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal. · guard: torn_tail
contract/revive-exit-codes/0 · revive exit codes · 0 — Success.
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
contract/revive-exit-codes/2 · revive exit codes · 2 — A clap usage error, a malformed --id included.
contract/revive-human/refusal · revive human output · refusal — Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty.
contract/revive-json/pump-child-error · revive --json · child pump error — An error while pumping the child prints no document, the child having held the terminal.
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/heartbeat-staleness-pure-function · Heartbeat staleness checked without a clock — The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/heartbeat-staleness-pure-classifier · Heartbeat staleness by a pure classifier — Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
decision/ipc-vector-test-layers · IPC vector test layers — The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/strip-control-oracle-keeps-guarded-names · Strip control oracle keeps the guarded names — A remove-the-guard control over the CLAUDE env strip uses an oracle that keeps the guarded identity-floor names, so removing the floor guard is seen. · why: The strip removes any unkept CLAUDE name anyway, so an oracle keeping none cannot see the floor. · guard: tui_env_strip
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/home-another-user-may-read-refused · Readable home refused — A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/identity-floor-tested-against-literal-copy · Identity floor tested against a literal copy — The R8 strip is tested against a literal copy of the identity floor, never an imported one.
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null.
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault.
contract/reply-screen/refusal · the reply screen · refusal — A refusal that is no code word reads unknown.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/agents-parser-fails-to-unknown · Agents list parse failure reads unknown — An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
decision/claude-env-strip · Agent identity environment stripped — Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and every identity-floor name is removed whatever the keep set says. · guard: an R8 test against a literal copy of the floor, never imported from the product
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
decision/one-global-tracing-install · One global tracing install — The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process.
decision/server-checked-before-any-frame · Server checked before any frame — Every client verb and the hook refuse an unverified server inside their own span with no frame written. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
contract/cli-arguments/json-flag · CLI arguments · --json — The --json flag is read from the arguments only before a `--` separator.
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/human-hint-cause/not-running · the human hint cause · not-running — The hint cause of an instance-dead unreachable is not-running, a word apart from the log's.
contract/human-hint-cause/socket-dir · the human hint cause · socket-dir — The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused.
contract/unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
contract/unreachable-log-cause/endpoint-lost · the unreachable log cause · endpoint-lost — Logged during call.
contract/unreachable-log-cause/instance-dead · the unreachable log cause · instance-dead — Logged during connect.
contract/unreachable-log-cause/server-verify-failed · the unreachable log cause · server-verify-failed — Logged during connect.
contract/unreachable-log-cause/socket-dir-refused · the unreachable log cause · socket-dir-refused — Logged during connect when the socket directory is refused.
contract/unreachable-log-cause/unwrapped-name · the unreachable log cause · unwrapped-name — Logged during connect.
decision/diagnostics-checked-before-narrowing · Diagnostics path checked before narrowing — A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
decision/unknown-refusal-word-stays-visible · Unknown refusal word stays visible — A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit. · guard: reply_of_reads_the_three_shapes
contract/channel-endpoint/socket-path · the wrapper channel endpoint · socket path — The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
contract/json-rpc-errors/wrapper-fault · wrapper channel JSON-RPC errors · a code outside the five — A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
decision/ipc-tested-on-real-endpoint · IPC tested on a real endpoint — IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
contract/event-line/ts · the event line · ts — ts is RFC 3339 UTC with ms and Z.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, as hyprctl in a test-scoped bin dir, and as foot, the terminal stand-in.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/module-integration-direct-calls · Module integration by direct calls — Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/help-no-bold-underline · Help carries no bold or underline — viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF. · guard: cli_output_plain
decision/home-resolution-order · Home resolution order — The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists. · why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening. · guard: home_resolution_takes_the_flag_then_the_session_then_the_default
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/instance-check-before-role-file · Instance check before any role file or frame — viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found. · why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home. · guard: link_verbs_refuse_a_widened_instance_before_any_log
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/server-identity-verified · Server identity verified before any frame — A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed. · guard: security_negatives_server_identity
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/socket-dir-check · Socket directory check before use — The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods. · guard: security_negatives_socket_dir
decision/squatter-via-server-api · Squatter listener through the server API — A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/mcp-unreachable-detail/endpoint-lost · MCP frame tools instance-unreachable detail · endpoint-lost — A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost.
contract/mcp-unreachable-detail/instance-dead · MCP frame tools instance-unreachable detail · instance-dead — A failed connect gives detail instance-dead, with no frame and no log line.
contract/mcp-unreachable-detail/server-verify-failed · MCP frame tools instance-unreachable detail · server-verify-failed — A peer euid the admission refuses, or a failed server verification, gives detail server-verify-failed, with no frame and no log line.
contract/mcp-unreachable-detail/socket-dir-refused · MCP frame tools instance-unreachable detail · socket-dir-refused — A refused socket directory check after the live endpoint is read gives detail socket-dir-refused, with no frame and no log line.
contract/mcp-unreachable-detail/strict-modes-failed · MCP frame tools instance-unreachable detail · strict-modes-failed — A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21.
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
contract/viola-help/verb-table · viola --help · grouped verb table — Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty.
decision/channel-client-no-dir-check · Channel client carries no directory check — The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check. · why: viola-channel has no viola-state edge. · guard: channel_link_names_are_validated_by_the_wrapper
decision/channel-tests-real-endpoint · Channel tests on a real endpoint — Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/owner-only-socket-modes · Owner-only socket modes — The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
contract/channel-response-detail/invalid-params · channel-response line detail · invalid-params — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
contract/mcp-unreachable-detail/call-deadline · MCP frame tools instance-unreachable detail · call-deadline — Each of send, wait, last and answer ends with detail call-deadline at the MCP call deadline, the frame dropped; a reply arriving before the deadline is returned as the reply.
contract/wrapper-channel/frame-over-max-frame · Wrapper channel · frame over MAX_FRAME — A frame over MAX_FRAME is answered with error -32600 and the connection is closed; a pending dialog stays pending.
contract/wrapper-channel/params-v-higher · Wrapper channel · params.v higher than served — A request whose params.v is higher than the served version is answered with error -32602.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/child-env-strip · Child environment strip — No stripped environment name reaches the agent child. · guard: the secret scan of the canary values · except: a name listed in claude_env_keep survives
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/endpoint-read-from-snapshot · Endpoint read only from the snapshot — Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/frame-verbs-reach-only-the-recorded-wrapper · Frame verbs reach only the recorded wrapper — A CLI frame verb writes no frame to a server that is not the recorded wrapper. · guard: security_negatives_server_identity
decision/frame-verbs-refuse-widened-instance · Frame verbs refuse a widened instance — The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line. · guard: security_negatives_strict_modes
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint · except: boot-time-unread of viola up is held by its unit pair alone, since a host's boot-time source cannot be hidden from a case.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json, the verify a keyed start runs included; every other writer-capable entry point leaves both byte-identical.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel, the agent session id and the launch pair included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/start-collision-by-heartbeat · Start collision read from the heartbeat — Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not. · guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
decision/start-lines-read-from-start-offset · A start's lines read from its own offset — A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log. · why: Cursors are byte offsets and the event log is never rewritten.
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
decision/unwrapped-name-never-a-target · Unwrapped row name is never a target — A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target. · guard: e1_send_naming_an_unwrapped_row_is_unreachable
contract/exit-causes/socket-dir-refused · exit causes · socket-dir-refused — A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
contract/process-start/claude-child · the event log · process-start{subject:"claude-child"} — The record carries the strip counts and the stripped names only, never their values.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
contract/viola-run-start/settings · viola run start · settings — A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.
contract/viola-run/stale-heartbeat · viola run · stale-heartbeat — A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
contract/wait-and-last/killed-wrapper · viola wait and last · killed wrapper — A killed wrapper gives CLI exit 21 and MCP instance-unreachable.
decision/cross-user-peers-untestable · Cross-OS-user peers skipped as untestable — Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/jsonschema-validator · jsonschema validates diagnostics — jsonschema is what validates diagnostics lines against their schemas.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
contract/json-rpc-errors/-32603 · JSON-RPC errors · -32603 — Every -32603 carries exactly the message "internal error" and data null.
contract/viola-send/exit-11 · viola send · 11 — Retired (was budget-paused) and never renumbered or reused: no refusal maps to 11, and an older wrapper's budget-paused word reads unknown, exit 14.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/channel-endpoint-from-snapshot · Channel endpoint read from the snapshot — Channel tests read the wrapper endpoint only from the instance snapshot.
decision/claude-child-env-strip · claude child environment strip — The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive. · guard: tui_env_strip
decision/claude-child-holds-nothing-of-viola · claude child holds nothing of viola's — The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY. · guard: tui_channel_fds
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/client-verifies-server-identity · Client verifies the server before any frame — A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output. · guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
decision/collision-check-over-replay-reads-beat · Collision check over a replay reads the beat — Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free. · guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fd-test-reads-live-child-table · fd test reads the live child's fd table — The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump. · why: The fake agent's own receipt and /dev/fd handle show as extra fds in its self-dump.
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/revive-preflight-order · revive preflight order — revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/session-live-read-fails-open · session-live reading fails open — revive --fork skips the session-live reading, and a failed claude agents read passes it, on revive and adopt alike. · guard: revive_passes_the_session_live_reading, revive_whose_agents_read_fails_starts
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/socket-dir-check-never-chmods · Socket directory refused, never repaired — The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it. · guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs, starts, viola focus and viola up's board scan take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, adopt's start, mcp and hook stay silent there.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/cli-json/instance-unreachable · CLI --json document · instance-unreachable — Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
contract/cli-json/internal-error · CLI --json document · internal-error — Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
contract/exit-21-detail/endpoint-lost · exit 21 instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open; its own hint; during call.
contract/exit-21-detail/instance-dead · exit 21 instance-unreachable detail · instance-dead — No instance is running for the name with its directory standing, a stale wrapper included; during connect.
contract/exit-21-detail/server-verify-failed · exit 21 instance-unreachable detail · server-verify-failed — The listening server is not the recorded wrapper (pid or start time mismatch, or another euid); its own hint; logged as process-exit at WARN; during connect.
contract/exit-21-detail/socket-dir-refused · exit 21 instance-unreachable detail · socket-dir-refused — The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
contract/exit-21-detail/strict-modes-failed · exit 21 instance-unreachable detail · strict-modes-failed — The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
contract/exit-21-detail/unwrapped-name · exit 21 instance-unreachable detail · unwrapped-name — The name holds no instance directory (an unwrapped session or a name never started); its own hint; during connect.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — The instance snapshot holds the wrapper channel endpoint.
contract/process-start-claude-child/env-kept · process-start{subject:"claude-child"} record · env_kept — Carries the environment names kept for the claude child, names only, never values.
contract/process-start-claude-child/env-stripped-count · process-start{subject:"claude-child"} record · env_stripped_count — Carries the count of environment names stripped from the claude child.
contract/process-start-claude-child/env-stripped-known · process-start{subject:"claude-child"} record · env_stripped_known — Carries the known stripped environment names, names only, never values.
contract/revive-exit-1/cwd-missing · revive exit 1 causes · cwd-missing — The recorded directory is gone, or the snapshot holds no cwd.
contract/revive-exit-1/no-session · revive exit 1 causes · no-session — No logged session for the name, or an --id the log does not hold.
contract/revive-exit-1/session-live · revive exit 1 causes · session-live — One claude agents --json read lists the logged session live in another claude; no child starts.
contract/revive-json/refusal-document · revive --json · refusal document — An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
contract/start-exit-1/already-live · run and revive exit 1 causes · already-live — The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/start-exit-1/squatted-name · run and revive exit 1 causes · squatted-name — The endpoint name is squatted.
contract/start-exit-1/stale-heartbeat · run and revive exit 1 causes · stale-heartbeat — The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
contract/start-exit-1/strict-modes-failed · run and revive exit 1 causes · strict-modes-failed — The home or instance state could be written, or read, by another user; no log opened, no line, no child.
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
contract/viola-run-start/duplicate-run · viola run start · duplicate run — A second run of an instance already running exits 1.
contract/viola-run-start/home-tree-check · viola run start · home-tree strict-modes check — The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
contract/viola-run-start/squatted-endpoint · viola run start · squatted endpoint — A run whose channel endpoint is already held by another listener exits 1.
contract/viola-run-start/tampered-pinned-exe · viola run start · tampered pinned exe — A run whose pinned executable was altered exits 1.
decision/channel-e2e-driver · Channel tested with the sync client — The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
decision/child-fd-table-read-after-launch-settles · Child fd guard reads the table once the launch settles — The child fd guard reads the child's fd table again, inside its existing deadline, while a whole table holds a non-fixture path under the home, and asserts the last whole table unchanged. · why: The case read the fake agent's launch-time open of the plugin hooks file as a hold. · guard: tui_channel_fds_the_child_holds_nothing_of_violas · provisional
decision/internal-error-held-at-unit-layer · -32603 held at the unit layer — No -32603 is driven in a scanned home and no failpoint or counted panic stands for one; the unit layer holds it. · guard: answer_a_panicking_dispatch_is_internal_error_and_closes
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/links-survive-wrapper-restart · Links survive a wrapper restart — A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line. · guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/no-fd-count-assertion · No fd-count assertion on the child — The child fd test asserts what the child holds, never a count of its descriptors. · why: portable-pty closes every inherited descriptor above 2 at spawn, so a planted leak cannot turn a count red, and the fake agent's own files add fds.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/torn-append-heals · Torn append heals on the next start — A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged. · guard: chaos_torn_append
decision/tty-styling-asserted-on-sgr · Terminal styling asserted on SGR sequences — Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
contract/channel-error/-32603 · channel error · -32603 — Every -32603 is exactly "internal error" with data null.
contract/channel-error/fault-bodies · channel error · fault bodies — The channel's faults are seven literal bodies.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/pinned-bin · instance snapshot · pinned_bin — The instance snapshot holds pinned_bin.
contract/instance-snapshot/started-at · instance snapshot · started_at — The instance snapshot holds started_at.
contract/list-json/liveness-stale · list --json · liveness stale — A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/viola-list-json/live-instance · viola list --json · running wrapped instance — A running wrapped instance's item reads wrapped:true and liveness "live".
contract/viola-run-start/plugin-hooks · viola run start · plugin hooks.json — Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/time-as-argument · pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.
contract/fake-agent-cli/unknown-arguments · viola-fake-agent command line · unknown arguments — Unknown arguments are ignored.
contract/fake-agent-receipt/env · viola-fake-agent receipt · env — An env receipt holds variable names, sorted, names only, never a value.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/child-argv-read-from-proc · Child argv read from the live child's cmdline — A test claim about the child's argv is read from the live child's /proc cmdline, never from the fake agent's receipt. · why: The fake agent's start receipt records no argv.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/foreign-owner-by-injected-uid · Foreign socket-directory owner by injected uid — A socket directory with a foreign owner is not driven in tests; its verdict is taken by an injected uid.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/socket-dir-check-leaves-dir-as-found · Socket-directory check never repairs — The socket-directory check never chmods the directory it checks; a refused socket directory is left as found.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe; a fake-args file beside its exe goes ahead of argv, argv winning.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version, pid, the model and effort it was handed on argv, and the print-mode prompt; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and the real foot terminal by the same binary as foot; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/no-http-fake-servers · no HTTP fake servers — Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/impostor-holder-real-process · Impostor's held process is real — A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard.
decision/impostor-server-real-listener · Impostor server is a real listener — Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
contract/heartbeat-staleness/age-threshold · Heartbeat staleness classification · heartbeat age — For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
contract/heartbeat-staleness/identity-mismatch · Heartbeat staleness classification · pid or start-time mismatch — A pid or start-time mismatch classifies the instance gone at any age.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/echo-oracle-skips-allowed-word · Echo oracle skips the one allowed word — An oracle asserting that a refusal echoes none of the given words skips the one allowed word when the refusal's fixed line itself names it. · why: Otherwise the fixed line's own mention of the allowed word reads as an echo.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/session-home-retention · Session home retention — Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/boot-readiness/events · boot readiness · events — Event lines 1, 2, 3 and 4 are a wheel line with cause start, a lineage line with source wrapper, a launch line with source wrapper, then session-start with source hook; missing reads events.
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/log-event-corr/process-exit · log event corr · process-exit — corr is null.
contract/log-event-corr/process-start · log event corr · process-start — corr is null.
contract/log-event-corr/state-recovered · log event corr · state-recovered — corr is null.
contract/log-line/instance · process log line · instance — A ViolaName or null.
contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
contract/log-line/target · process log line · target — A required field.
contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME. · why: The start writes into the user's Claude config.
decision/harness-wrappers-under-session-user-home · Harness wrappers under a session user home — The harness's supervised wrappers of a stamped session run under a session user home seeded with an empty config. · why: The start writes into the user's Claude config.
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.

# test-plan · its absences, names and overviews, whole

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
absence/browser-suite — A browser test suite
absence/windows-and-macos-support — Windows and macOS support: code, tests, fixtures and CI jobs

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
absence/database — A database
absence/outbound-http — Outbound HTTP

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
Scenario: E4 — none in 0.1.0 › N/A — viola has no web or graphical frontend.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
absence/deploy-stage — a deploy stage
