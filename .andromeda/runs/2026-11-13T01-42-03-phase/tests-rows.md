# test-plan · the rows that bind 2026-11-13-verify-readiness-on-the-input-box — picked 226 of 1374 (126 by every draw) · rule any · pin 803

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/no-advisory-ignored · No dependency advisory ignored — CI gates on cargo deny check with no advisory ignored.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/driver-send/local-command · driver send · ledger-listed local command — A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/driver-send/typed-text · driver send · typed text — The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
decision/multi-line-send-one-prompt · Multi-line text is one prompt — A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/local-command-post-condition · Local commands confirmed by post-condition only — On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
decision/sent-text-typed-form · Sent text is typed in normalised form — A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched. · guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/tokio-only-in-e2e-crate · Tokio test code confined to the E2E crate — One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients. · why: so the root package and the sync crates carry no tokio dev-dependency

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/paste-injection-test-layers · Paste injection test layers — Paste injection is tested by unit property tests, integration and E2E.
decision/supply-chain-ci-gates · Supply chain held by CI gates — The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them.
decision/pty-exit-from-wait-not-eof · PTY exit reported from wait, never from reader EOF — viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
decision/vt100-feed-panic-safe · Screen feed panic-safe — The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready.
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen with no input-box literal waits up to 8 500 ms and reads input-not-ready from that bound on.
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/local-command-unconfirmable · Local command without a post-condition is unconfirmable — A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window. · guard: …_without_a_measured_post_condition_is_unconfirmable
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
contract/prompt-submitted/origin-harness · the prompt-submitted event · origin harness — A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/readiness-stall-report · Readiness stall report — When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing. · why: So the report reaches the JUnit.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/no-unbounded-json-depth · No unbounded JSON depth — serde_json's unbounded_depth feature is banned. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/no-serde-json-unbounded-depth · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in deny.toml. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/vt100-panic-keeps-passthrough · vt100 panic keeps human passthrough — After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/record-only-at-zero-failing-rows · fixtures recorded only when nothing fails — The viola verify recorder writes fixtures only at 0 failing rows.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-input/bracketed-paste · viola-fake-agent input · bracketed paste — Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.
contract/fake-agent-input/compact · viola-fake-agent input · /compact — Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
contract/fake-agent-input/framing-paste · viola-fake-agent input · --framing paste — Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs · except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/hygiene-reports-class-only · hygiene checker reports class only — The hygiene checker reports a violation class only (absolute-path, username, schema), never the offending content.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub. · except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
decision/time-as-argument · pure time decisions take their instant — A pure time decision, such as the budget gate's override expiry, takes its instant as an argument instead of reading a clock.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/fake-agent-receipt/step · Fake-agent receipt · step — Carries index and event.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/test-clocks-behind-clock-seam · Test clocks behind the clock seam — Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/cargo-deny-gate · cargo-deny supply-chain gate — Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build. · guard: cargo deny; deny-probes, one probe per ban
decision/channel-tokio-free-by-default · viola-channel reaches tokio only by feature — The build fails when viola-channel reaches tokio without its feature enabled. · guard: channel-tokio-free
decision/msrv-check · MSRV check on the security toolchain floor — An ubuntu CI job checks the workspace and runs the unit suite on Rust 1.96, the minimum supported Rust version. · why: Rust 1.96 is the security toolchain floor.
decision/pinned-rust-toolchain · Exactly pinned Rust toolchain — The Rust toolchain is pinned to one exact stable version at or above the 1.96 floor and installed by rustup on both the CI runner and the host.
decision/rmcp-feature-limit · rmcp limited to server and transport-io — The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested. · guard: rmcp-features
decision/supply-chain-fail-closed · fail-closed supply-chain job — One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
decision/weekly-advisories-run · Weekly advisories run — A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/cargo-deny-supply-chain-gate · cargo deny supply-chain gate — The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace. · guard: CI job supply-chain
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/deny-bans-probed · Every deny ban has a negative probe — Every cargo deny ban has a permanent negative probe that must fire, beside a clean control. · guard: deny-probes.sh
decision/msrv-1-96 · Minimum supported Rust 1.96 — The workspace is checked and unit-tested on Rust 1.96, its minimum supported version. · why: 1.96 is the security toolchain floor and the requested workspace rust-version; sysinfo needs 1.95. · guard: CI job msrv, gate --require nextest-unit
decision/no-ignored-advisories · No advisory ignored — cargo deny ignores no advisory. · guard: CI job supply-chain
decision/rmcp-features-server-transport-io · rmcp takes exactly server and transport-io — rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features. · guard: lint step rmcp features (rmcp-features.sh)
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned from the dependency graph. · guard: deny probe serde_json-unbounded-depth
decision/sync-crates-compile-without-features · Sync crates compile alone without features — Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails. · guard: CI job lint, one cargo check per listed crate
decision/tokio-banned-per-sync-crate · tokio banned from each sync crate — tokio is banned from each crate on the sync-crates list, checked with that crate as sole root. · guard: step Tokio ban per sync crate (sole root)
decision/viola-channel-tokio-free · viola-channel normal graph is tokio-free — viola-channel's normal dependency graph holds no tokio; its Tokio client sits behind its tokio feature. · guard: channel-tokio-free.sh and deny probe channel-tokio-edge
decision/weekly-advisory-scan · Weekly advisory scan — Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch. · guard: nightly job advisories
decision/lint-fmt-and-clippy-deny-warnings · Lint denies format drift and clippy warnings — Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on. · guard: CI job lint
decision/workflows-audited-by-zizmor · Workflows audited by zizmor — The GitHub workflows are audited by zizmor in the supply-chain job. · guard: CI job supply-chain

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-interactive-snapshot-review · No interactive snapshot review as a gate — Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/pre-push-stage/tools · pre-push stage · tools — Checks the passwd home, cc, and rustc, cargo-nextest, cargo-mutants and cargo-llvm-cov at their pinned versions.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/rmcp-client-only · rmcp client only in the harness — The harness uses rmcp with the client feature only, without transport-child-process. · why: The test owns the viola mcp child process.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.
decision/verify-window-60s-kill · 60 s kill for the verify_window_ class — Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first. · why: Their cases wait out the gate's 8.5 s maximum several times by design. · guard: mutants_verify_window_override_stands_before_the_harness_package_override
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
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
