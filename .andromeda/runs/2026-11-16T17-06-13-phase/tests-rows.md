# test-plan · the rows that bind 2026-11-16-granular-api-error-cause — picked 224 of 1331 (131 by every draw) · rule any · pin 2030

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/hooks-fail-open · Hooks fail open — A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/last/result · last · result — last returns {last_assistant_message, ts} from the newest turn-ended event.
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only.
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> always exits 0.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
contract/wait/already-logged · wait · after cursor — wait returns at once when a waking event is already logged at or after the after cursor.
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr.
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?}.
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result.
contract/wait/timeout · wait · timeout — wait times out with {timed_out:true}.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/mcp-tool-result/ok · MCP tool result · ok — isError false with structuredContent equal to the ok payload.
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
contract/hook/impostor-server · Hook · impostor server — A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
contract/mcp-reply-screen/detail · MCP reply screen · detail — A string detail that is no code word reads null, and a non-string detail is no known reply.
contract/mcp-reply-screen/refusal · MCP reply screen · refusal — A refusal that is no code word reads unknown.
contract/mcp-server/internal-fault · MCP server · internal handler fault — Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/dialog-bodies-pinned-by-insta · Dialog decision bodies pinned by insta — insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null.
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault.
contract/reply-screen/refusal · the reply screen · refusal — A refusal that is no code word reads unknown.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/hook-unknown-fields-tolerated · Unknown hook payload fields tolerated — Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
decision/list-row-control-escaping · list rows show every control character as hex — In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text. · guard: escape_row_shows_every_control_character_as_hex_text
contract/channel-wait/wake-kinds · the channel wait method · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/mcp-reply/screened-words · MCP tool replies · refusal and detail words — A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
contract/channel-wait/timed-out · the channel wait method · timed_out — wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.
decision/mcp-handlers-use-tokio-client · MCP handlers use the Tokio client — MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only and absent on a Stop.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
decision/hook-line-cases-in-made-instance-dir · Hook line cases run in a viola-made instance dir — Hook test cases that expect lines run in a viola-made instance directory. · why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line and the hook diagnostics trace.
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook is driven by std::process::Command with an Instant deadline over recorded fixture stdin.
decision/hook-fails-open · Hook fails open silently — The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper. · guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
contract/viola-wait/wake · viola wait · wake on turn-ended — wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
decision/list-names-escaped · List names escaped — Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell. · guard: cross_list_rows
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
contract/viola-last/newest-message · viola last · newest turn message — last returns the newest turn's message, and the MCP result is equal to the CLI's.
contract/viola-wait/timed-out · viola wait · timed_out — A wait past the log end times out with {timed_out:true}.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/list-rows-escape-control-chars · list rows escape control characters — viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode. · guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/hook-process/exit · dialog hook process · exit and stderr — The hook process exits 0 with empty stderr.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v.
contract/mcp-wait/timeout-ms · MCP wait · timeout_ms — The bound sent is clamped to 50 000 ms, which is also its default.
contract/viola-last/document · viola last · last document — Gives {last_assistant_message, ts} of the last turn.
contract/viola-wait/turn-ended · viola wait · turn-ended wake — Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/hook-panic-payload-to-detail-file · Hook panic payload kept to the detail file — A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only. · guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
decision/mcp-tests-own-server-child · MCP tests own their server child — MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport. · why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/caught-panic-exit-code/hook · exit code on a caught panic · hook — hook exits 0 on a caught panic.
contract/channel-error/-32603 · channel error · -32603 — Every -32603 is exactly "internal error" with data null.
contract/channel-error/fault-bodies · channel error · fault bodies — The channel's faults are seven literal bodies.
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
contract/cli-json/internal-error · CLI --json document · internal-error — Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
contract/list-human/columns · viola list human board · columns — The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
contract/turn-ended/background-tasks · turn-ended event data · background_tasks — The Stop array's length, or null.
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.
contract/viola-wait/already-past · viola wait · event already past the cursor — A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
contract/viola-wait/timeout · viola wait · timeout — A wait that times out gives {timed_out:true} with exit 0.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/hygiene-reports-class-only · hygiene checker reports class only — The hygiene checker reports a violation class only, never the offending content.
decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse, Stop, Notification and UserPromptSubmit steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume, a compact replay or the launch SessionStart under --transcript sets.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the ten registered hook events, validated by the fake-script v1 schema.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/rstest-fixtures · rstest for fixtures — rstest is what provides test fixtures and cases.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-hooks/command-spawn · viola-fake-agent hook execution · hook command — The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume, its transcript_path under --transcript, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them. · except: StopFailure is registered with no recorded payload, no ledger row and no verify probe; its tests use a synthetic StopFailure payload.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/mcp-wait-tool/timeout-bound · MCP wait tool · wait bound — The wait bound is the given timeout capped at 50 000 ms, min(given, 50 000).
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/fake-agent-copies-planted-first · Fake-agent copies planted before any child — A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child. · why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it. · why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/fail-open-spine-bound-at-unit-speed · Fail-open spine bound asserted at unit speed — Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed. · guard: hook_fails_open_silently_within_the_spine_bound
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/agent-run-logs-stream/torn-line · agent-run logs ndjson stream · torn line — A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/g2-zero-panics · G2 zero panics — CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs. · except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/hook-trace-write-failure-swallowed · Hook trace write failures swallowed — A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null.

## test-data-bootstrap · key of ## 3. Test Harness Contract
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
