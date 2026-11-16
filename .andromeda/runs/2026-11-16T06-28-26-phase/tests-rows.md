# test-plan · the rows that bind 2026-11-16-local-command-delivery — picked 278 of 1329 (164 by every draw) · rule any · pin 1879

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/leading-slash-never-a-prompt · Leading-slash argument never a prompt — A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered. · guard: send_leading_slash_argument_is_a_usage_error
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
contract/driver-send/local-command · driver send · ledger-listed local command — A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
contract/driver-send/send-issued · driver send · send-issued — A driver send logs a send-issued event.
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/driver-send/typed-text · driver send · typed text — The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/local-command-exact-match · Local commands matched by exact equality — Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test.
decision/local-command-post-condition · Local commands confirmed by post-condition only — On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/sent-text-typed-form · Sent text is typed in normalised form — A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched. · guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
contract/cli-send-json/exit-0 · CLI send --json · exit 0 — A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
contract/cli-send-json/no-prompt-submitted · CLI send --json · not-delivered / no-prompt-submitted — A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
contract/mcp-reply-screen/detail · MCP reply screen · detail — A string detail that is no code word reads null, and a non-string detail is no known reply.
contract/mcp-send/success · MCP send · success result — A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
contract/mcp-tool-result/ok · MCP tool result · ok — isError false with structuredContent equal to the ok payload.
contract/send-confirmation/prompt-submitted-driver · send confirmation · prompt-submitted with origin driver — A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}}
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/mutation-run-failure-conditions · Mutation run failure conditions — A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/mutation-run-verdict · Mutation run verdict — A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones. · except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/relayed-fixtures-kept-superseded · Relayed fixtures kept as superseded — Fixtures relayed from the prototype are superseded by live recordings but stay committed with their provenance under a Superseded section.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null.
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/clear-confirmed-by-new-session · /clear confirmed by a new session — /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor. · guard: …_clear_is_confirmed_by_a_new_session
decision/compact-confirmed-by-same-session · /compact confirmed by its same session — /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing. · guard: …_compact_is_confirmed_by_its_same_session
decision/local-command-unconfirmable · Local command without a post-condition is unconfirmable — A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window. · guard: …_without_a_measured_post_condition_is_unconfirmable
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
decision/send-confirmation-window-expiry · Send confirmation window expiry — A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/mcp-reply/screened-words · MCP tool replies · refusal and detail words — A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
decision/private-placement-mode-first · Private copies placed mode-first — A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name. · guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
decision/send-confirmation-trailing-trim-match · Send confirmation tolerates a trailing trim — A send is confirmed only when the submitted prompt equals the typed text, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; any other difference, a leading trim included, claims nothing. · why: The CLI trims whitespace from a prompt at submit, so a driver's own send ending in it went unclaimed; the trim set is an assumption, not a live reading. · guard: submitted_form_takes_a_trailing_run_the_cli_trims, submitted_form_refuses_every_other_difference
decision/vt100-feed-panic-safe · Screen feed panic-safe — The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
contract/cli-argv/leading-slash · CLI argv · leading slash — A leading-slash argument is never delivered as a prompt.
decision/mcp-handlers-use-tokio-client · MCP handlers use the Tokio client — MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/wrapper-channel/cursor · the wrapper channel · cursor — A result's cursor equals the events.ndjson length at acceptance.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/wrapper-channel/empty-text · Wrapper channel · refusal detail empty-text — An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
contract/mcp-send/ok-keys · the MCP send tool · ok result keys — The MCP send result carries the same ok key set as the CLI.
contract/viola-send/confirmed · viola send · confirmed send — A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
contract/viola-send/cr-lf-text · viola send · text ending in or holding CR or LF — A text ending in or holding CR or LF is confirmed as its typed text.
contract/viola-send/not-delivered · viola send · not-delivered — A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
contract/viola-send/unconfirmable · viola send · unconfirmable — A listed local command is confirmed only by its post-condition, or else is unconfirmable.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
contract/mcp-cli-parity/refusal · MCP and CLI parity · refusal — For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/compact-confirm-window · Compact confirmation window — `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor. · guard: send_compact_confirm_window_is_its_own_bound · except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send. · provisional
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-send-human-mode/read-back · viola send human mode · confirmed line — A confirmed send prints the stdout line [RB] read back … cursor <n>.
contract/viola-send-human-mode/unconfirmable · viola send human mode · unconfirmable line — A local command with no measured post-condition prints one stdout line [  ] unconfirmable <name> local command, no measured post-condition, exit 0, nothing on stderr.
contract/viola-send/clear-confirmed · viola send · /clear on a verified CLI — Confirmed by its post-condition alone, a session-start with cause clear and a new agent_session_id, ending exit 0 {submitted_at, cursor}.
contract/viola-send/compact-confirmed · viola send · /compact on a verified CLI — Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.
contract/viola-send/no-prompt-submitted · viola send · unconfirmed send — A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
contract/viola-send/off-list-slash-text · viola send · slash text off the compiled list — Exits 13 not-delivered/no-prompt-submitted.
contract/viola-send/ok-document · viola send · ok document — A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
contract/viola-send/records · viola send · send records — The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the submitted text: the typed text, or the typed text less the trailing run the CLI trimmed.
contract/viola-send/unconfirmable-local-command · viola send · local command with no measured post-condition — Exits 0 with ok {confirmed:false, detail:"unconfirmable", cursor}, logs send-issued then send-confirmed{confirmed:false}, no send-refused, no confirmation window.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
contract/mcp-tool-input/empty-or-control-text · MCP tool input · empty or control-character text — A send text that is empty or holds a control character is not-delivered.
contract/viola-send-human-mode/unable · viola send human mode · refusal lines — A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
contract/viola-send/human-key-during-gate · viola send · human key during the gate wait — The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
contract/viola-send/inner-cr · viola send · inner CR — A CR or CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel.
contract/viola-send/trailing-newlines · viola send · trailing newlines — A text ending in LF, CR, CRLF or two CRs is typed without that ending and confirmed.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
decision/time-as-argument · pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-input/bracketed-paste · viola-fake-agent input · bracketed paste — Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text, or under --trim-prompt-end with its trailing run of space, TAB, CR and LF removed.
contract/fake-agent-input/clear · viola-fake-agent input · /clear — Under --framing the probed /clear fires the recorded clear-1 SessionEnd then SessionStart and no UserPromptSubmit.
contract/fake-agent-input/compact · viola-fake-agent input · /compact — Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
contract/fake-agent-cli/unknown-arguments · viola-fake-agent command line · unknown arguments — Unknown arguments are ignored.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-agent-input/framing-paste · viola-fake-agent input · --framing paste — Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the ten registered hook events, validated by the fake-script v1 schema.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it. · why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/test-clocks-behind-clock-seam · Test clocks behind the clock seam — Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.
decision/host-unreachable-mutants-excluded · Host-unreachable mutants recorded as host-excluded — A missed mutant the host never compiled is recorded host-excluded: kept out of the survivors and never counted as caught.
decision/mutants-verdict-counted-by-harness · Mutation verdict counted by the harness — At a `run --mutants` the harness counts missed, timeout, unviable and caught outcomes itself, leaving host-excluded missed mutants out; any other missed mutant, any timed-out mutant, or more unviable than caught mutants is red. · why: The cargo-mutants exit code and summary cannot be trusted alone: exit 3 masks exit 2, and the summary counts a host-excluded mutant as missed.
decision/mutation-gate-thresholds · Mutation gate thresholds — A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out. · why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/doctest-only-catch-is-missed · Doctest-only catch counts as missed — A mutant caught only by a doctest counts as missed, and the missing unit test must be added.
decision/host-unreachable-mutants · Host-unreachable mutants recorded, not caught — A mutant this host cannot compile or reach is recorded not measured here, owed to the route entry that measures it, never equivalent and never caught; one wholly under a cfg proved false for the host is left out of the count with its predicate.
contract/mutants-verdict/counted · run --mutants verdict · counted — A closed mutants.verdict value: any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.
contract/mutants-verdict/unviable-exceeds-caught · Mutation run verdict · unviable-exceeds-caught — The run is red when unviable mutants outnumber caught ones.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/gate-breach/mutants · gate breach · mutants — The mutants suite has survived > 0.
contract/harness-run-exit/0 · harness run exit codes · 0 — Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
contract/harness-run-exit/non-zero · harness run exit codes · non-zero — Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/run-document/mutants · the harness run document · mutants — A --mutants run adds a mutants object with tested and a verdict of counted, no-rust-delta, test-only-rust-delta, scoped or package, with the full base sha except on package.
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/host-excluded-mutants · Host-excluded missed mutants — A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
decision/mutants-survived-count · Mutants survived count — The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/mutation-verdict-by-counts · Mutation verdict by counts — A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide. · why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing
contract/agent-run-logs-stream/detail-line · agent-run logs ndjson stream · detail line — Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/mutants-verdict/scoped · run --mutants verdict · scoped — The run was scoped by --file.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/send-window-paste-hint-30s-kill · 30 s mutants kill for send-window classes — Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill. · why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint. · guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/viola-e2e-30s-mutants-kill · 30 s mutants kill for the harness package — Under the mutants profile the viola-e2e package takes a 30 s per-test kill. · why: The harness's own tests wait out its 20 s boot deadline by design.

## log-format · key of ## 3. Test Harness Contract
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor.
contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor.
contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
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
