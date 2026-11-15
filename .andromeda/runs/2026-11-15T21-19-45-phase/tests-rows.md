# test-plan · the rows that bind 2026-11-15-api-error-turn-end — picked 355 of 1327 (218 by every draw) · rule any · pin 1610

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
decision/hooks-fail-open · Hooks fail open — A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
decision/human-takes-wheel-on-edit · Human editing takes the wheel — A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/ipc-vector-negative-tests · IPC security vector negative tests — The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
decision/performance-budget-tests-for-hook-deadlines · Performance-budget tests for hook deadlines — Hook deadlines are covered by performance-budget tests. · why: A driver of the Comprehensive tier.
decision/plugin-rewritten-on-start · Plugin rewritten on start — The plugin directory is rewritten on every start.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/unwrapped-hooks-silent-noop · Unwrapped session hooks are a no-op — A viola hook fired in a session viola does not wrap is a silent no-op.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/wheel-returns-only-via-cli-release · Wheel returns only through viola release — The wheel returns from the human only through the CLI viola release, never through a release carrying from.
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/last/result · last · result — last returns {last_assistant_message, ts} from the newest turn-ended event.
contract/viola-hook/deadline · viola hook exec-form entry points · deadline — Spine hooks finish within the spine deadline, and SessionEnd within about 1 s.
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> always exits 0.
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr.
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?}.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
contract/wait/already-logged · wait · after cursor — wait returns at once when a waking event is already logged at or after the after cursor.
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
decision/comprehensive-test-tier · Comprehensive test tier — The project is tested at the Comprehensive tier (2). · why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/driver-send/human-typing · driver send · human-typing — A driver send while the human holds the wheel is refused human-typing.
contract/driver-send/send-issued · driver send · send-issued — A driver send logs a send-issued event.
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/cli-send-json/exit-0 · CLI send --json · exit 0 — A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
contract/hook/impostor-server · Hook · impostor server — A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
contract/send-confirmation/prompt-submitted-driver · send confirmation · prompt-submitted with origin driver — A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
decision/harness-turn-keeps-the-wheel · Harness-injected turns keep the wheel — A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel.
decision/human-input-takes-the-wheel · Human input takes the wheel — A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
contract/cli-release/wheel-driver · CLI release · wheel driver — Release gives wheel driver, and the next send is accepted.
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}}

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/hook-deadlines-gated-by-hyperfine · Hook deadlines gated by hyperfine — hyperfine gates the hook deadlines on max.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/property-and-fuzz-layer · Property tests and fuzzing — proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/ipc-vector-test-layers · IPC vector test layers — The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/performance-trigger-layer · Performance-budget trigger layer — The performance-budget trigger is met by perf budget tests.
decision/property-trigger-layer · Property-test trigger layer — The property-test trigger is met by proptest and the fuzz job.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/fuzz-weekly-and-corpus-gate · Fuzzing weekly with a corpus replay gate — cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/unbuilt-surface-is-usage-error · Unbuilt harness surface is a usage error — A harness flag or selector whose surface is not built yet is a usage error, never a vacuous pass.
contract/harness-exit-codes/0 · harness exit codes · 0 — The harness command succeeded.
contract/harness-exit-codes/1 · harness exit codes · 1 — The harness command failed.
contract/harness-exit-codes/2 · harness exit codes · 2 — The harness command was given a usage error, and its document carries reason usage and a detail.
contract/harness-stdout/document · harness stdout · document — Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing.
contract/harness-usage-detail/arguments · harness usage error detail · arguments — The arguments do not parse, including a flag or selector whose surface is not built yet.
contract/harness-usage-detail/unknown-suite · harness usage error detail · unknown-suite — A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The version gate drain, the hyprctl reader and the dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/hook-dialog-no-frame-to-impostor · Hook writes no frame to an impostor server — The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper. · guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
decision/hook-unknown-fields-tolerated · Unknown hook payload fields tolerated — Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
decision/server-checked-before-any-frame · Server checked before any frame — Every client verb and the hook refuse an unverified server inside their own span with no frame written. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/channel-wait/wake-kinds · the channel wait method · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/dialog-data/null-details · dialog and Stop event data · description, preview and background_tasks — A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array.
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows
contract/channel-endpoint/socket-path · the wrapper channel endpoint · socket path — The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
contract/unreachable-log-cause/unwrapped-name · the unreachable log cause · unwrapped-name — Logged during connect.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.
decision/ipc-tested-on-real-endpoint · IPC tested on a real endpoint — IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
contract/event-line/ts · the event line · ts — ts is RFC 3339 UTC with ms and Z.
contract/strict-modes-refusal/hook · strict-modes refusal · hook — A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
contract/viola-hook/no-viola-name · viola hook · no VIOLA_NAME — With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.
contract/viola-hook/viola-dir · viola hook · VIOLA_DIR — A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
contract/wrapper-channel/cursor · the wrapper channel · cursor — A result's cursor equals the events.ndjson length at acceptance.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
decision/hook-line-cases-in-made-instance-dir · Hook line cases run in a viola-made instance dir — Hook test cases that expect lines run in a viola-made instance directory. · why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line and the hook diagnostics trace.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/server-identity-verified · Server identity verified before any frame — A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed. · guard: security_negatives_server_identity
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message and background_tasks.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
decision/channel-tests-real-endpoint · Channel tests on a real endpoint — Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/plugin-marketplace-mcp-only · Plugin marketplace is MCP-only — viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere. · guard: plugin_install_writes_the_mcp_only_marketplace
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
decision/squatter-via-server-api · Squatter listener through the server API — A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
contract/viola-help/verb-table · viola --help · grouped verb table — Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/frame-verbs-reach-only-the-recorded-wrapper · Frame verbs reach only the recorded wrapper — A CLI frame verb writes no frame to a server that is not the recorded wrapper. · guard: security_negatives_server_identity
decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook is driven by std::process::Command with an Instant deadline over recorded fixture stdin.
decision/hook-fails-open · Hook fails open silently — The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper. · guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
decision/injected-turn-keeps-wheel · Harness-injected turn keeps the wheel — A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/no-unbounded-json-depth · No unbounded JSON depth — serde_json's unbounded_depth feature is banned. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/schema-failure-no-content · Schema failure report without content — A schema failure prints file, line number and the failing schema keyword, never the line's content.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
contract/generated-plugin/hook-commands · the generated plugin · hook commands — A restart rewrites plugin/ with absolute hook commands, never a PATH lookup.
contract/viola-send/confirmed · viola send · confirmed send — A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
contract/viola-wait/wake · viola wait · wake on turn-ended — wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
decision/child-env-names-viola-instance · Child given its viola name, instance dir and binary — The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path).
decision/diagnostics-files-owner-only · Diagnostics files owner-only — Diagnostics files are mode 0600 on Unix. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/endpoint-read-from-snapshot · Endpoint read only from the snapshot — Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
decision/hook-suites-in-integration-layer · Non-path suites layer — Non-path trigger-coverage suites run in the E2E layer. · except: root hook_* files run in the integration layer
decision/jsonschema-validator · jsonschema validates diagnostics — jsonschema is what validates diagnostics lines against their schemas.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
contract/viola-last/newest-message · viola last · newest turn message — last returns the newest turn's message, and the MCP result is equal to the CLI's.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/channel-e2e-driver · Channel tested with the sync client — The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
decision/channel-endpoint-from-snapshot · Channel endpoint read from the snapshot — Channel tests read the wrapper endpoint only from the instance snapshot.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
decision/hook-instance-check-before-config · Hook instance check before any config read — The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing · except: The hook <event> --capture arm stays ahead of the check.
decision/no-serde-json-unbounded-depth · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in deny.toml. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
contract/caught-panic-exit-code/hook · exit code on a caught panic · hook — hook exits 0 on a caught panic.
contract/hook-process/exit · dialog hook process · exit and stderr — The hook process exits 0 with empty stderr.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/turn-ended/background-tasks · turn-ended event data · background_tasks — The Stop array's length, or null.
contract/viola-send/ok-document · viola send · ok document — A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
contract/viola-send/records · viola send · send records — The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the typed text.
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
contract/viola-wait/already-past · viola wait · event already past the cursor — A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
contract/viola-wait/turn-ended · viola wait · turn-ended wake — Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
decision/ci-scan-before-upload · CI secret scan before any upload — The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload. · except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
decision/client-verifies-server-identity · Client verifies the server before any frame — A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output. · guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/hook-panic-payload-to-detail-file · Hook panic payload kept to the detail file — A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only. · guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/security-sweep-under-nextest · Security sweep runs under nextest alone — The security sweep runs under nextest alone.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
contract/hook-decision/server-verify-failed · hook-decision record · server-verify-failed — A hook facing a server that is not the recorded wrapper logs one hook-decision line with detail "server-verify-failed", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
contract/list-human/columns · viola list human board · columns — The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-list-json/live-instance · viola list --json · running wrapped instance — A running wrapped instance's item reads wrapped:true and liveness "live".
contract/viola-release/running-turn · viola release · turn left running — Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
contract/viola-run-start/plugin-hooks · viola run start · plugin hooks.json — Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/fake-agent-children-own-process-group · fake agent children in their own process group — On Unix every command the fake agent spawns, each hook, runs in a process group of its own. · why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
decision/proptest-persisted-seeds · proptest with persisted seeds — Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.
decision/proptest-strategies · proptest for generated data — proptest strategies are what generate property-test data.
decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-hooks/command-spawn · viola-fake-agent hook execution · hook command — The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-events/session-start · viola-fake-agent fired events · SessionStart — After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.
contract/fake-agent-receipt/kind · viola-fake-agent receipt file · kind — Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, step.
contract/fake-agent-receipt/line · viola-fake-agent receipt file · receipt line — The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/proptest-for-randomized-data · proptest for randomized data — Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
decision/record-home-names-no-home · Record homes never named home — A home a test gives to viola verify --record comes from the shared record-home helper and fixture, and no component of its path is named home. · guard: record_home_names_no_path_component_home
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/step · Fake-agent receipt · step — Carries index and event.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/hook-unit-impostor-varies-start-time · Hook unit impostor varies the start time — The hook's unit impostor varies the recorded start time, a second early or an unparseable started_at, instead of using a second process. · why: A root-bin unit test has no portable second process.
decision/impostor-server-real-listener · Impostor server is a real listener — Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.
decision/impostor-holder-real-process · Impostor's held process is real — A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/build-fails-on-perf-breach · Build fails on a performance budget breach — The build fails on any hyperfine performance budget breach.
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned from the dependency graph. · guard: deny probe serde_json-unbounded-depth

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/fail-open-spine-bound-at-unit-speed · Fail-open spine bound asserted at unit speed — Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed. · guard: hook_fails_open_silently_within_the_spine_bound
decision/hook-connect-deadline · Hook connect deadline below the spine deadline — The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces. · guard: connect_deadline_is_below_the_spine_deadline · provisional
decision/hook-deadlines-asserted-at-unit-speed · Hook deadlines asserted at unit speed — The hook deadlines are also asserted at unit speed inside the hook subprocess tests, with a monotonic `std::time::Instant` bound.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/hyperfine-max-perf-gate · hyperfine max is the performance gate — Performance budgets, hook deadlines among them, are gated by hyperfine on `max`, never on `mean`, and a budget regression fails the build; criterion 0.8.2 is not a gate. · why: criterion 0.8.2 does not exit non-zero on a regression.
decision/perf-gate-on-max · Performance gate on the max sample — The performance gate judges the max sample; mean, median and stddev are reported for trend only, and no percentile is computed or gated. · why: Claude Code hook deadlines are hard cutoffs.
decision/perf-real-hook-path · Perf times hooks through the real path — The perf run boots one unstamped session with a builder instance and times each hook through the real logging and channel path on synthetic payloads.
decision/spine-deadline-single-source · Spine deadline from the product constant — The perf gate reads the spine deadline from the product's own constant (1.0 s), never from a literal edited into CI or the test plan.
contract/perf-budgets/spine-hooks · Performance budgets · spine hooks session-start, user-prompt-submit, stop — Timed the same way with VIOLA_NAME and VIOLA_DIR exported to hyperfine; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/perf-gate-spine-deadline · Hook perf gate at the spine deadline — The five hook perf rows are required by name, and every timed hook row's hyperfine max must be below the product's SPINE_DEADLINE (1.0 s); a missing row is a breach, never a pass.
decision/perf-hooks-time-the-real-path · Perf hooks time the real path — Every timed hook runs with an instance name and directory set, so it takes the real logging and channel path, never the no-instance no-op, with its instance, server, socket-directory and peer checks inside the timed path.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/proptest-seeds-committed · Property tests replay from committed seeds — Property tests replay deterministically: proptest regression seeds persist in committed regression files.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/seeded-generators · Seed-controlled generators — Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/no-ci-mutants-gate · No CI mutants gate — No CI job gates the mutants suite.
decision/perf-verdict-belongs-to-gate · Perf verdict belongs to the gate — The perf arm runs only when named and reports its suite; the deadline verdict belongs to the gate's perf requirement, never to run.
decision/pre-push-runs-no-mutation · pre-push runs no mutation — No pre-push stage runs mutation testing. · why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests.
decision/secret-scan-scope · Secret scan scope — The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload. · why: The run --mutants chunk diff is repository source text that hits the scan's own needles. · except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/agent-run-status-polled-fields/status · agent-run status polled fields · status — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/wheel · agent-run status polled fields · wheel — A per-item field of the status document that the agent polls.
contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
contract/gate-exit-codes/1 · gate exit codes · 1 — Exit 1 on any breach.
contract/gate-require/suites · gate --require · suites — Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/local-live-suite/failure-codes · the local-live suite · failure codes — A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
decision/hook-trace-write-failure-swallowed · Hook trace write failures swallowed — A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor.
contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/log-event-corr/channel-request · log event corr · channel-request — corr is the JSON-RPC request id; null for an id-less hook.event notification.
contract/log-event-corr/hook-invoked · log event corr · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
contract/log-event-corr/panic · log event corr · panic — corr is null.
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
contract/log-line/hook-panic · process log line · hook panic — A panic in a hook is caught and logged as one level ERROR, event panic line.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/proptest-committed-seeds · proptest with committed seeds — Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.

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
