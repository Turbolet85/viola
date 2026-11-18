# test-plan · the rows that bind 2026-11-18-browser-tools-prompt-modal — picked 260 of 1346 (147 by every draw) · rule any · pin 2742

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal; the test-only live rig reads the child's screen by compiled literals into closed words its rehearsal asserts on.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and the standing live rig's live starts, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/human-never-blocked · The human is never blocked — The human is never blocked: human typing still reaches the agent while automation is refused.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/wait/timeout · wait · timeout — wait times out with {timed_out:true}.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough · except: The test-only live rig reads the child's screen by compiled literals only into closed words, which its rehearsal asserts on.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, any listed command on an unverified CLI, or an echo-confirmed command with no remembered transcript or no readable transcript end, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer. · except: The test-only live rig reads the child's screen by compiled literals into closed words, and passes a read trust dialog under its key grant.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/tied-verify-kill-witness · Kill witness for a tied stamping verify — A kill witness for a tied stamping verify waits until verify holds a child inside a viola-verify run dir, asserts an owned dir stood at the kill, and reads the stamp's absence besides processes and dirs. · why: An untied verify orphaned by the kill finishes inside the wait bound and removes its own dirs, so processes and dirs alone cannot tell it from a tied one. · guard: killed_test_leaves_no_stamping_verify
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/paste-injection-test-layers · Paste injection test layers — Paste injection is tested by unit property tests, integration and E2E.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 3-test-harness-contract · ## 3. Test Harness Contract
contract/harness-usage-detail/unknown-suite · harness usage error detail · unknown-suite — A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/no-untied-harness-boot · No viola-e2e test boots a harness session untied — No viola-e2e test boots a harness session untied: a census over viola-e2e's test sources finds every in-test boot tied, and is read red on a planted untied boot first. · guard: viola_e2e_census_finds_no_untied_boot
decision/no-verify-start-outside-test-home · Every test verify start tied to a test-owned home — A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start. · guard: every_test_verify_runs_under_a_test_home
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/live-failing-reading-not-verification · Live reading recorded as read, never as verification — A live reading that fails a ledger row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/standing-live-rig · One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand under the founder's start cap for each chunk, each ledgered with its clock time first, and its live arm refuses under CI. · why: Ratified as the one rig every later live round reuses. · guard: rig_rehearsal
decision/start-lines-read-from-start-offset · A start's lines read from its own offset — A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log. · why: Cursors are byte offsets and the event log is never rewritten.
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
contract/viola-answer/unverified-cli · viola answer · 12 unverified-cli — An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
contract/viola-run-start/settings · viola run start · settings — A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/decision-body-gate · Decision body gate — No dialog decision body is ever emitted unless the wheel is driver and the CLI version is stamped.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.
contract/mcp-tool-result/unverified-cli · MCP tool result · unverified-cli — MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
decision/ci-scan-before-upload · CI secret scan before any upload — The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload. · except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/refused-or-own-send-records-no-link · Refused or own-name send records no link — A refused send and a send to an instance's own name record no link. · guard: links_a_refused_send_and_an_own_name_send_record_none
decision/terminal-reply-reading-live-measured · Terminal reply reading measured live — The classifier's reading of terminal replies on a real terminal is a local live measurement, not a suite. · why: The outer-PTY tier writes no terminal reply of its own.
decision/vt100-panic-keeps-passthrough · vt100 panic keeps human passthrough — After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
contract/viola-send-human-mode/unable · viola send human mode · refusal lines — A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
contract/viola-wait/timeout · viola wait · timeout — A wait that times out gives {timed_out:true} with exit 0.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/record-only-at-zero-failing-rows · fixtures recorded only when nothing fails — The viola verify recorder writes fixtures only at 0 failing rows. · except: A failed recording verify by the live rig alone writes a first-screen file of whole scrubbed rows per run short of the input box (Run A's trust excepted), refused whole when unclean, never committed.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF, and exits 1 on Enter on the trust form's No; before it exits it waits for a hook still running and starts no other.
contract/fake-agent-input/bracketed-paste · viola-fake-agent input · bracketed paste — Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text, or under --trim-prompt-end with its trailing run of space, TAB, CR and LF removed.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke that draws a box rule and a prompt row; a DEL erases the prompt's last character, UTF-8 aware, and a CR submits what was typed.
contract/fake-agent-screens/trust-dialog · viola-fake-agent screens · trust dialog — Under --screens with an untrusted cwd the fake agent draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No, exit directly above the recorded trust row (row 16 with no recorded modal); arrows move the selector, and Enter on No exits 1.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/screen-fixture-box-rows-as-literals · Box rows recorded as their literals alone — A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text. · guard: planted_box_row_beyond_its_literal_is_rejected
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/record-home-names-no-home · Record homes never named home — A home a test gives to viola verify --record comes from the shared record-home helper and fixture, and no component of its path is named home. · guard: record_home_names_no_path_component_home
decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it. · why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/home-path · MCP fixture scrub refusal codes · home-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/username · MCP fixture scrub refusal codes · username — A closed refusal code of the MCP fixture scrub, naming the file only.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/jobs-end-in-harness-gate · CI jobs end in the harness gate — Each CI test job ends in viola-harness gate requiring its suites.
contract/ci-artifacts/diag · CI artifacts · diag-${{ matrix.os }} — Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-jobs-end-in-harness-gate · CI test jobs end in the harness gate — Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
decision/boot-deadline-not-a-budget · Boot deadline is not a performance budget — The 20 s `boot` deadline is a harness readiness timeout, deliberately not a product performance budget in the quality gates.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/no-ignore-parking · No #[ignore] parking — `#[ignore]` is not allowed as a parking place for a failing or flaky test. · why: Agent-driven development cannot tell a flake from a real bug.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.
decision/test-failure-fails-build · Any test failure fails the build — A test failure at any level is a build failure condition.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog. · except: The live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-interactive-snapshot-review · No interactive snapshot review as a gate — Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt. · except: The test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/no-test-passing-unstamped-decision-body · No test passing an unguarded decision body — No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/harness-output-json-and-exit-code · Harness commands always give JSON and exit code — Every harness command prints its JSON document and exit code, never a human-only message alone.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/harness-boot-reason/verify-failed · harness boot refusal reason · verify-failed — One of the four boot refusal reasons.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
decision/skipped-counts-real-skips-only · skipped counts real skips only — skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted. · why: so the gate's suite-skipped breach fires only on a real skip
decision/supervisor-ordinary-child · Supervisor is an ordinary child — The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session ties it to the test thread by parent-death SIGKILL and its stamping viola verify by SIGTERM.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/agent-run-status-polled-fields/cli-verified · agent-run status polled fields · cli_verified — A per-item field of the status document that the agent polls.
contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
contract/gate-breach/suite-missing · gate breach · suite-missing — A required suite is absent from the run summary.
contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0.
contract/gate-require/suites · gate --require · suites — Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.
contract/harness-boot-reason/readiness-timeout · harness boot refusal reason · readiness-timeout — One of the four boot refusal reasons.
contract/harness-boot/agents-mode · harness boot · --agents-mode — --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records.
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build.
contract/local-live-suite/failure-codes · the local-live suite · failure codes — A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/one-verify-stamp-per-build · One verify stamp per build — Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock. · why: Taking the stamp once keeps every stamped-home test under its unchanged per-test kill. · guard: stamped_home_reuses_one_stamp_for_every_home
decision/stamping-verify-passes-transcript · Stamping verifies give the fake agent a transcript — Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing. · why: Without a transcript Run B's echo probe is skipped and the local-command-echo row fails. · except: The framing-free verify helper passes no --transcript.
decision/verify-under-test-owned-user-home · Test verifies run under a test-owned user home — Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file. · why: verify's --record scrub reads HOME, so a test-owned HOME would make a recording's absolute paths unclean. · except: A --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned user home instead.

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
