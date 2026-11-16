# test-plan · the rows that bind 2026-11-16-send-confirmation — picked 318 of 1329 (197 by every draw) · rule any · pin 1768

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/human-takes-wheel-on-edit · Human editing takes the wheel — A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
decision/paste-control-characters-rejected · Control characters in pasted text rejected — ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper. · except: LF, CR, TAB and multi-byte UTF-8 are accepted.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
contract/driver-send/empty-text · driver send · empty-text — An empty typed text is refused empty-text.
contract/driver-send/human-typing · driver send · human-typing — A driver send while the human holds the wheel is refused human-typing.
contract/driver-send/local-command · driver send · ledger-listed local command — A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
contract/driver-send/send-issued · driver send · send-issued — A driver send logs a send-issued event.
contract/driver-send/typed-text · driver send · typed text — The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/human-never-blocked · The human is never blocked — The human is never blocked: human typing still reaches the agent while automation is refused.
decision/mcp-schema-violations-rejected · MCP schema-violating tool calls rejected — MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/vt100-panic-contained · vt100 panic contained — A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/wheel-returns-only-via-cli-release · Wheel returns only through viola release — The wheel returns from the human only through the CLI viola release, never through a release carrying from.
contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists.
decision/human-input-takes-the-wheel · Human input takes the wheel — A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
decision/local-command-exact-match · Local commands matched by exact equality — Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test.
decision/local-command-post-condition · Local commands confirmed by post-condition only — On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
decision/multi-line-send-one-prompt · Multi-line text is one prompt — A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/sent-text-typed-form · Sent text is typed in normalised form — A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched. · guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
contract/cli-send-json/empty-text · CLI send --json · not-delivered / empty-text — A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
contract/cli-send-json/exit-0 · CLI send --json · exit 0 — A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
contract/cli-send-json/human-typing · CLI send --json · exit 10 — A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
contract/cli-send-json/no-prompt-submitted · CLI send --json · not-delivered / no-prompt-submitted — A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
contract/event-log/wheel-human-input · the event log · wheel — A human taking the wheel logs wheel with holder human and cause human-input.
contract/mcp-send/success · MCP send · success result — A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
contract/send-confirmation/prompt-submitted-driver · send confirmation · prompt-submitted with origin driver — A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}}
decision/harness-turn-keeps-the-wheel · Harness-injected turns keep the wheel — A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/torn-tail-healed-by-append · Torn log tail healed by the next append — A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal. · guard: torn_tail
contract/cli-human-lines/read-back · CLI human lines · read back — [RB] read back … cursor <n>
contract/mcp-send/refused-human-typing · MCP send · refused: human-typing — An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
contract/mcp-tool-result/ok · MCP tool result · ok — isError false with structuredContent equal to the ok payload.
contract/mcp-tool-result/refusal · MCP tool result · refusal — isError true with {refusal, detail} and text "refused: <reason>".

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/property-and-fuzz-layer · Property tests and fuzzing — proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/dialog-bodies-pinned-by-insta · Dialog decision bodies pinned by insta — insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/property-trigger-layer · Property-test trigger layer — The property-test trigger is met by proptest and the fuzz job.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/fuzz-weekly-and-corpus-gate · Fuzzing weekly with a corpus replay gate — cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/paste-injection-test-layers · Paste injection test layers — Paste injection is tested by unit property tests, integration and E2E.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
contract/mcp-tool-arguments/invalid-argument · MCP tool arguments · invalid argument — A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/clear-confirmed-by-new-session · /clear confirmed by a new session — /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor. · guard: …_clear_is_confirmed_by_a_new_session
decision/compact-confirmed-by-same-session · /compact confirmed by its same session — /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing. · guard: …_compact_is_confirmed_by_its_same_session
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character
decision/local-command-unconfirmable · Local command without a post-condition is unconfirmable — A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window. · guard: …_without_a_measured_post_condition_is_unconfirmable
decision/send-confirmation-exact-match · Send confirmation by exact normalised match — A send is confirmed only when the submitted prompt text equals the typed text, every CR LF and lone CR read as one LF and the trailing CR and LF removed; a submitted prompt that still holds a CR or the trailing newline claims nothing. · guard: send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing
decision/send-confirmation-window-expiry · Send confirmation window expiry — A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/prompt-submitted/origin-harness · the prompt-submitted event · origin harness — A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human.
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty.
contract/wrapper-channel/control-character · the wrapper channel · control-character — In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
contract/wrapper-channel/cursor · the wrapper channel · cursor — A result's cursor equals the events.ndjson length at acceptance.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/paste-validated-at-both-ends · Sent text validated at client and wrapper — Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper. · guard: channel_paste_validation
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
contract/cli-exit-codes/13-text-refused · CLI exit codes · 13 for refused text — viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.
contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only and absent on a Stop.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/wrapper-channel/accepted-text · Wrapper channel · text with LF, CR, TAB and multi-byte characters — Text with LF, a lone inner CR, TAB and multi-byte characters is accepted and typed as one prompt, its inner CR as one LF.
contract/wrapper-channel/empty-text · Wrapper channel · refusal detail empty-text — An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/injected-turn-keeps-wheel · Harness-injected turn keeps the wheel — A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
contract/mcp-send/ok-keys · the MCP send tool · ok result keys — The MCP send result carries the same ok key set as the CLI.
contract/viola-send/confirmed · viola send · confirmed send — A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
contract/viola-send/cr-lf-text · viola send · text ending in or holding CR or LF — A text ending in or holding CR or LF is confirmed as its typed text.
contract/viola-send/empty-text · viola send · empty-text — A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
contract/viola-send/exit-10 · viola send · 10 — A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
contract/viola-send/not-delivered · viola send · not-delivered — A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
contract/viola-send/unconfirmable · viola send · unconfirmable — A listed local command is confirmed only by its post-condition, or else is unconfirmable.
contract/wheel-record/human-input · the event log · wheel human-input — A human key logs wheel{holder:"human",cause:"human-input"} and the human's prompt still lands.
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook is driven by std::process::Command with an Instant deadline over recorded fixture stdin.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/torn-append-healed · Torn append healed — A torn log append is counted and the next record starts on a fresh line.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.
contract/events/human-input-wheel · events.ndjson · human-input — Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.
contract/mcp-tool-input/empty-or-control-text · MCP tool input · empty or control-character text — A send text that is empty or holds a control character is not-delivered.
contract/viola-send-human-mode/read-back · viola send human mode · confirmed line — A confirmed send prints the stdout line [RB] read back … cursor <n>.
contract/viola-send-human-mode/unconfirmable · viola send human mode · unconfirmable line — A local command with no measured post-condition prints one stdout line [  ] unconfirmable <name> local command, no measured post-condition, exit 0, nothing on stderr.
contract/viola-send/human-key-during-gate · viola send · human key during the gate wait — The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
contract/viola-send/human-typing · viola send · human-typing refusal — While the human holds the wheel, send exits 10 human-typing with detail null.
contract/viola-send/inner-cr · viola send · inner CR — A CR or CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel.
contract/viola-send/no-prompt-submitted · viola send · unconfirmed send — A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
contract/viola-send/off-list-slash-text · viola send · slash text off the compiled list — Exits 13 not-delivered/no-prompt-submitted.
contract/viola-send/ok-document · viola send · ok document — A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
contract/viola-send/paste-framing · viola send · typed framing — A sent text is typed as one bracketed paste followed by one CR, its newlines kept inside.
contract/viola-send/records · viola send · send records — The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the typed text.
contract/viola-send/trailing-newlines · viola send · trailing newlines — A text ending in LF, CR, CRLF or two CRs is typed without that ending and confirmed.
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.
contract/viola-send/unconfirmable-local-command · viola send · local command with no measured post-condition — Exits 0 with ok {confirmed:false, detail:"unconfirmable", cursor}, logs send-issued then send-confirmed{confirmed:false}, no send-refused, no confirmation window.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/compact-confirm-window · Compact confirmation window — `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor. · guard: send_compact_confirm_window_is_its_own_bound · except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send. · provisional
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/refused-or-own-send-records-no-link · Refused or own-name send records no link — A refused send and a send to an instance's own name record no link. · guard: links_a_refused_send_and_an_own_name_send_record_none
decision/send-from-another-instance-links-once · Send from another instance links once — An issued send from another instance appends one link{driver, driven} record with source wrapper before its send-issued, and a later send of the same pair adds none. · guard: links_a_send_from_another_instance_links_once
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/terminal-replies-not-editing · Terminal replies are not editing — Terminal reply shapes on a closed list are read as not editing and never take the wheel; any other stdin input is typing. · guard: classifier_terminal_reply_is_not_editing, classifier_typing_is_editing
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
decision/vt100-panic-keeps-passthrough · vt100 panic keeps human passthrough — After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/events/harness-prompt · events.ndjson · harness-injected turn — A harness-injected turn logs `prompt-submitted{origin:"harness"}` and no `wheel` record.
contract/mcp-send/human-typing · MCP send · human-typing refusal — is_error:true with text refused: human-typing.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v.
contract/viola-release/running-turn · viola release · turn left running — Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
contract/viola-send-human-mode/unable · viola send human mode · refusal lines — A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
contract/viola-send/compact-confirmed · viola send · /compact on a verified CLI — Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.
contract/viola-send/manual-pause · viola send · manual-pause refusal — After pause, send exits 10 with detail "manual-pause".
contract/viola-wait/non-waking-records · viola wait · activity and wheel records — Activity and wheel records, and a harness-origin prompt-submitted, never wake a parked wait.
contract/viola-wait/turn-ended · viola wait · turn-ended wake — Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/proptest-persisted-seeds · proptest with persisted seeds — Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.
decision/proptest-strategies · proptest for generated data — proptest strategies are what generate property-test data.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse, Stop, Notification and UserPromptSubmit steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-agent-input/bracketed-paste · viola-fake-agent input · bracketed paste — Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
contract/fake-agent-receipt/kind · viola-fake-agent receipt file · kind — Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, step.
contract/fake-agent-receipt/line · viola-fake-agent receipt file · receipt line — The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the ten registered hook events, validated by the fake-script v1 schema.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/rstest-fixtures · rstest for fixtures — rstest is what provides test fixtures and cases.
decision/time-as-argument · pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/proptest-for-randomized-data · proptest for randomized data — Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it. · why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/fake-agent-receipt/step · Fake-agent receipt · step — Carries index and event.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them. · except: StopFailure, provisionally, is registered with no recorded payload, no ledger row and no verify probe; its tests use a synthetic StopFailure payload.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/screen-fixture-box-rows-as-literals · Box rows recorded as their literals alone — A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text. · guard: planted_box_row_beyond_its_literal_is_rejected

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/proptest-seeds-committed · Property tests replay from committed seeds — Property tests replay deterministically: proptest regression seeds persist in committed regression files.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/seeded-generators · Seed-controlled generators — Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
contract/cleanup-document/endpoint-gone · cleanup document · endpoint_gone — True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.
contract/cleanup-document/home-removed · cleanup document · home_removed — True when the session home was removed; reports "kept" when homes are kept, which is not a failure.
contract/cleanup-document/killed · cleanup document · killed — Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup.
contract/cleanup-document/processes-gone · cleanup document · processes_gone — True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/local-live-failure/build · local-live failure code · build — A closed local-live failure code.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/local-live-failure/verify-exit-n · local-live failure code · verify-exit-<n> — A closed local-live failure code.
contract/local-live-failure/verify-exit-none · local-live failure code · verify-exit-none — A closed local-live failure code.
contract/local-live-suite/failure-codes · the local-live suite · failure codes — A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
decision/send-window-paste-hint-30s-kill · 30 s mutants kill for send-window classes — Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill. · why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint. · guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.

## log-format · key of ## 3. Test Harness Contract
contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor.
contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor.
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/proptest-committed-seeds · proptest with committed seeds — Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.
decision/one-verify-stamp-per-build · One verify stamp per build — Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock. · why: Taking the stamp once keeps every stamped-home test under its unchanged per-test kill. · guard: stamped_home_reuses_one_stamp_for_every_home
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
