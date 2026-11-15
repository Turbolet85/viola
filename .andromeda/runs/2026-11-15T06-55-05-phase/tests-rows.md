# test-plan · the rows that bind 2026-11-15-live-round-on-the-trimmed-product — picked 437 of 1325 (255 by every draw) · rule any · pin 1297

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/child-exit-read-on-the-process-handle · Child exit is read on the process handle, never on EOF — The pump detects the child's exit by polling the process handle and never by output EOF, then drains the output still in flight for a bounded time. · why: The wrapper's exit code relies on it, and the pump tests use a reader that never ends so only the handle can end the pump.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
decision/human-never-blocked · The human is never blocked — The human is never blocked: human typing still reaches the agent while automation is refused.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/pinned-exe-hash-checked · Pinned executable hash checked — A pinned executable whose hash does not match makes the command exit 1.
decision/plugin-rewritten-on-start · Plugin rewritten on start — The plugin directory is rewritten on every start.
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor. · except: Persistent-environment names listed in config.json claude_env_keep are kept.
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/unwrapped-hooks-silent-noop · Unwrapped session hooks are a no-op — A viola hook fired in a session viola does not wrap is a silent no-op.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/hook-dialog/unverified-null · channel hook.dialog · unverified CLI — On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty.
contract/instance-liveness/live · instance liveness · live — An instance whose process runs and whose heartbeat is at most 5 s old is live.
contract/instance-liveness/stale · instance liveness · stale — An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
contract/instance-snapshot/pid · instance snapshot.json · pid — The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
contract/mcp-tools-list/tool-set · MCP tools/list · tool set — tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink.
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness.
contract/test-harness/cleanup · test harness commands · cleanup — cleanup stops every wrapper and deletes the home, idempotently.
contract/test-harness/logs · test harness commands · logs — logs prints the per-process logs as JSON lines.
contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
contract/test-harness/status · test harness commands · status — status queries viola list --json.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?}.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/human-takes-wheel-on-edit · Human editing takes the wheel — A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/single-test-only-crate · One test-only workspace member — The workspace has exactly one test-only member, crates/viola-e2e, never published.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/dialog/event-logged-once · hook dialogs · dialog event — A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
contract/heartbeat-file/interval · instance heartbeat file · touch interval — The instance heartbeat file is touched every 1 s.
contract/instance-liveness/gone · instance liveness · gone — An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/human-input-takes-the-wheel · Human input takes the wheel — A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
decision/mcp-no-control-tools · No control tools over MCP — The MCP server offers no release, pause, link or unlink tool.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
contract/event-log/wheel-human-input · the event log · wheel — A human taking the wheel logs wheel with holder human and cause human-input.
contract/harness-boot/readiness-failure · Harness boot command · readiness failure — Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text.
contract/harness-cleanup/endpoint-gone · Harness cleanup command · endpoint check — Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
contract/mcp-server/tools-list · MCP server · tools/list — Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private.
contract/run-refusal/already-live · run and revive start refusals · already-live — A live holder or a fresh replayed beat refuses the start with exit 1.
contract/run-refusal/stale-heartbeat · run and revive start refusals · stale-heartbeat — A stale holder refuses the start with exit 1.
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then the agent's session-start with source hook.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/local-command-post-condition · Local commands confirmed by post-condition only — On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
contract/harness-cleanup/idempotent · Harness cleanup command · second run — A second cleanup exits 0 and reports nothing to clean.
contract/harness-run/exit · Harness run command · exit — Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/heartbeat-staleness-pure-function · Heartbeat staleness checked without a clock — The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/heartbeat-staleness-pure-classifier · Heartbeat staleness by a pure classifier — Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/os-gated-cases-in-binary-root · OS-gated cases stay in a split binary's root file — The OS-gated census walks only files directly under tests/ and crates/*/tests/, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file. · why: A gated case moved into the module directory would leave its gates row reading as no longer standing. · guard: contract_os_gated_tests
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/single-os-ci-leg · One CI leg on Linux — The CI matrix builds on one leg, ubuntu-latest.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/harness-logic-in-rust · Harness logic once, in Rust — One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust.
contract/harness-session-record/supervisor-pid · harness session record · supervisor_pid — The session record carries the supervisor's pid.
contract/harness-session-record/supervisor-started-at · harness session record · supervisor_started_at — The session record carries the supervisor's start time.
contract/harness-exit-codes/0 · harness exit codes · 0 — The harness command succeeded.
contract/harness-exit-codes/1 · harness exit codes · 1 — The harness command failed.
contract/harness-session-record/home · harness session record · home — The session record carries the session's home.
contract/harness-session-record/instances · harness session record · instances — The session record lists instances, each with name, wrapper_pid, started_at and fake_args.
contract/harness-stdout/document · harness stdout · document — Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing.
contract/harness-usage-detail/session-exists · harness usage error detail · session-exists — The session named already exists.
contract/harness-usage-detail/unknown-suite · harness usage error detail · unknown-suite — A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/identity-floor-tested-against-literal-copy · Identity floor tested against a literal copy — The R8 strip is tested against a literal copy of the identity floor, never an imported one.
decision/pty-exit-from-wait-not-eof · PTY exit reported from wait, never from reader EOF — viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status.
contract/mcp-tools-list/tools-list · MCP tools/list · tools/list — tools/list is exactly send, wait, last, answer, list.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The version gate drain, the hyprctl reader and the dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/claude-env-strip · Agent identity environment stripped — Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names. · guard: an R8 test against a literal copy of the floor, never imported from the product
decision/compact-confirmed-by-same-session · /compact confirmed by its same session — /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing. · guard: …_compact_is_confirmed_by_its_same_session
decision/dialog-hold-expiry · Dialog hold expiry — An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line. · guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line
decision/dialog-hold-read-once · Dialog hold read once at start — The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection. · guard: read_hold_falls_back_to_sixty_with_one_rejection
decision/dialog-timeouts-outlast-the-hold · Dialog timeouts outlast the longest hold — The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold. · guard: hooks_json_dialog_timeouts_outlast_the_longest_hold
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/e2e-home-refusal/message · the e2e-home headroom refusal · message — e2e-home backing: {free} MiB free, below the {floor} MiB floor
decision/pending-dialog-before-its-line · Pending dialog on the snapshot before its line — A held dialog is on the snapshot before its line is logged, and is cleared when the append fails. · guard: hook_dialog_append_that_failed_clears_the_pending_dialog
decision/private-placement-mode-first · Private copies placed mode-first — A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name. · guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
contract/viola-hook/no-viola-name · viola hook · no VIOLA_NAME — With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/bad-config-value-falls-back · Bad config value falls back to its default — A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it.
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/home-resolution-order · Home resolution order — The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists. · why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening. · guard: home_resolution_takes_the_flag_then_the_session_then_the_default
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.
contract/event-data/dialog-expired · Event data keys · dialog-expired — data holds the keys dialog_id and dialog_kind.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
decision/plugin-marketplace-mcp-only · Plugin marketplace is MCP-only — viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere. · guard: plugin_install_writes_the_mcp_only_marketplace
decision/readiness-stall-report · Readiness stall report — When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing. · why: So the report reaches the JUnit.
decision/squatter-via-server-api · Squatter listener through the server API — A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
decision/verify-takes-no-json · viola verify takes no --json — viola verify takes no --json flag.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/child-env-names-viola-instance · Child given its viola name, instance dir and binary — The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path).
decision/child-env-strip · Child environment strip — No stripped environment name reaches the agent child. · guard: the secret scan of the canary values · except: a name listed in claude_env_keep survives
decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/start-collision-by-heartbeat · Start collision read from the heartbeat — Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not. · guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
contract/generated-plugin/hook-commands · the generated plugin · hook commands — A restart rewrites plugin/ with absolute hook commands, never a PATH lookup.
contract/process-start/claude-child · the event log · process-start{subject:"claude-child"} — The record carries the strip counts and the stripped names only, never their values.
contract/viola-run-exit-codes/1 · viola run exit codes · 1 — A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
contract/viola-run-start/settings · viola run start · settings — A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.
contract/viola-run/stale-heartbeat · viola run · stale-heartbeat — A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
contract/wheel-record/human-input · the event log · wheel human-input — A human key logs wheel{holder:"human",cause:"human-input"} and the human's prompt still lands.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/scan-hits-never-echo-bytes · Scan hits never echo bytes — A secret-scan hit is reported by place and class, never by the matched bytes.
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
contract/dialog-event/logged-once · the event log · dialog event — Each dialog event is logged exactly once with a dialog_id, and wait returns it at once.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
contract/viola-answer/unverified-cli · viola answer · 12 unverified-cli — An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/claude-child-env-strip · claude child environment strip — The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive. · guard: tui_env_strip
decision/collision-check-over-replay-reads-beat · Collision check over a replay reads the beat — Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free. · guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
decision/exit-detected-on-handle · Child exit detected on the handle — The wrapper detects the child's exit on the process handle (exit_source "handle-wait"), not on output EOF.
decision/fd-test-reads-live-child-table · fd test reads the live child's fd table — The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump. · why: The fake agent's own receipt and /dev/fd handle show as extra fds in its self-dump.
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/mcp-tests-own-server-child · MCP tests own their server child — MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport. · why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/pinned-exe-hash-sha2 · sha2 for the pinned-exe content hash — The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex. · guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record, and the hook's session-start is its second and last start record.
contract/events/human-input-wheel · events.ndjson · human-input — Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/pinned-bin · instance snapshot · pinned_bin — The instance snapshot holds pinned_bin.
contract/instance-snapshot/started-at · instance snapshot · started_at — The instance snapshot holds started_at.
contract/process-start-claude-child/env-kept · process-start{subject:"claude-child"} record · env_kept — Carries the environment names kept for the claude child, names only, never values.
contract/process-start-claude-child/env-stripped-count · process-start{subject:"claude-child"} record · env_stripped_count — Carries the count of environment names stripped from the claude child.
contract/process-start-claude-child/env-stripped-known · process-start{subject:"claude-child"} record · env_stripped_known — Carries the known stripped environment names, names only, never values.
contract/start-exit-1/already-live · run and revive exit 1 causes · already-live — The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
contract/start-exit-1/stale-heartbeat · run and revive exit 1 causes · stale-heartbeat — The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-run-passthrough/ctrl-z · viola run passthrough · Ctrl-Z — Ctrl-Z reaches the child, and later keys still do.
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
contract/viola-run-start/duplicate-run · viola run start · duplicate run — A second run of an instance already running exits 1.
contract/viola-run-start/plugin-hooks · viola run start · plugin hooks.json — Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/home-readable-by-other-refused · Home another user may read is refused — A pre-existing home another user may read is refused at run with exit 1. · guard: a_home_another_user_may_read_refuses_run
decision/no-decision-body-without-driver-and-stamp · Decision bodies only for a driver on a stamped CLI — No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
decision/terminal-replies-not-editing · Terminal replies are not editing — Terminal reply shapes on a closed list are read as not editing and never take the wheel; any other stdin input is typing. · guard: classifier_terminal_reply_is_not_editing, classifier_typing_is_editing
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/dialog-events/logged-once · dialog events · question, permission, plan — Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
contract/hook-process/exit · dialog hook process · exit and stderr — The hook process exits 0 with empty stderr.
contract/list-json/liveness-stale · list --json · liveness stale — A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
contract/mcp-tool-result/unverified-cli · MCP tool result · unverified-cli — MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/viola-run-start/tampered-pinned-exe · viola run start · tampered pinned exe — A run whose pinned executable was altered exits 1.
contract/viola-run/child-on-wrapper-kill · viola run wrapper · wrapper killed — When the wrapper is killed, its child agent is gone within a bounded time.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/cleanup-removes-home-record-and-supervisor · Cleanup removes the home, session record and supervisor — Per session, harness cleanup stops the supervisor and kills it and the recorded processes if needed, removes the session home unless homes are kept, and drops the session record. · why: Boot refuses a session id whose record exists, and cleanup of all sessions finds sessions only by their records.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload, MCP exchange or Hyprland reply set that fails the unclean check refuses the whole recording.
decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/mcp-exchange-recorded-by-tap · MCP exchange recorded by a test-only tap — The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
decision/mcp-tap-refuses-dirty-exchange · MCP tap refuses a dirty exchange whole — The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
decision/record-only-at-zero-failing-rows · fixtures recorded only when nothing fails — The viola verify recorder writes fixtures only at 0 failing rows.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-hooks/command-spawn · viola-fake-agent hook execution · hook command — The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
contract/fake-agent-input/compact · viola-fake-agent input · /compact — Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
decision/time-as-argument · pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.
contract/fake-agent-receipt/env · viola-fake-agent receipt · env — An env receipt holds variable names, sorted, names only, never a value.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/screen-fixture-box-rows-as-literals · Box rows recorded as their literals alone — A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text. · guard: planted_box_row_beyond_its_literal_is_rejected
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/home-path · MCP fixture scrub refusal codes · home-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/username · MCP fixture scrub refusal codes · username — A closed refusal code of the MCP fixture scrub, naming the file only.
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fake-agent-copies-planted-first · Fake-agent copies planted before any child — A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child. · why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/hold · Fake-agent receipt · hold — Carries pid, written by the --exit-no-eof holder at its start.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/agent-reads-ci-through-gh · Agent reads CI through gh — The agent reads CI job conclusions and fetches artifacts with the gh CLI, and locally gets the same files from agent-run run and agent-run logs.
decision/linux-only-ci-matrix · Linux-only CI matrix — CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
decision/single-instrumented-test-run · one instrumented coverage run in CI — CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/harness-gate-fails-on-breach · Harness gate fails on any quality breach — viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
decision/jq-runner-provided · jq taken from the runner — jq is taken from the CI runner, never installed; the scripts that need it refuse with tool-missing: jq when it is absent.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
contract/heartbeat-staleness/age-threshold · Heartbeat staleness classification · heartbeat age — For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/coverage-exclusions · Coverage exclusions — The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness. · why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/no-ignore-parking · No #[ignore] parking — `#[ignore]` is not allowed as a parking place for a failing or flaky test. · why: Agent-driven development cannot tell a flake from a real bug.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
contract/heartbeat-staleness/identity-mismatch · Heartbeat staleness classification · pid or start-time mismatch — A pid or start-time mismatch classifies the instance gone at any age.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-interactive-snapshot-review · No interactive snapshot review as a gate — Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-passing-unstamped-decision-body · No test passing an unguarded decision body — No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on tmpfs — On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/e2e-home-deletion-through-link · Test-home deletion through the dev-host link — On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created. · why: A boundary widening, ratified.
decision/e2e-home-link-base · E2E home base may be a link — A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/harness-builds-in-its-own-target · Harness builds into its own target dir — The harness build always uses its own target dir, never the target dir the shim's cargo run uses. · why: a running harness binary could not be relinked, and a workspace-wide build re-fingerprints it
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/internal-harness-subcommands-off-agent-surface · Internal harness subcommands — The internal harness subcommands are implemented in viola-harness, forwarded unchanged by the shims, and are not part of the agent's 5-command surface.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/no-nested-harness-in-nextest · No nested cargo run inside nextest — No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
decision/workspace-root-absolute · Workspace root passed absolute — The harness resolves the workspace root and passes it as an absolute path. · why: nextest runs the e2e crate's tests with that crate as the cwd
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/agent-run-status-exit-codes/0 · agent-run status exit codes · 0 — Exit 0 when ok is true, which is only for state ready.
contract/agent-run-status-exit-codes/1 · agent-run status exit codes · 1 — Exit 1 for degraded or down; the full status document is still printed, so an agent waiting for a stale transition reads exit 1 as a state, not a harness fault.
contract/agent-run-status-exit-codes/2 · agent-run status exit codes · 2 — Exit 2 on a usage error, or when no session.json exists for the id, printing {"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}.
contract/agent-run-status-polled-fields/cli-verified · agent-run status polled fields · cli_verified — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/dialog-pending · agent-run status polled fields · dialog_pending — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/liveness · agent-run status polled fields · liveness — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/name · agent-run status polled fields · name — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/status · agent-run status polled fields · status — A per-item field of the status document that the agent polls.
contract/agent-run-status-polled-fields/wrapped · agent-run status polled fields · wrapped — A per-item field of the status document that the agent polls.
contract/boot-readiness/events · boot readiness · events — Event lines 1 and 2 are a wheel line with cause start, then session-start with source hook; missing reads events.
contract/boot-readiness/heartbeat · boot readiness · heartbeat — The heartbeat mtime is less than 5 s old; missing reads heartbeat.
contract/boot-readiness/process-start · boot readiness · process-start — The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited.
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
contract/cleanup-document/processes-gone · cleanup document · processes_gone — True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.
contract/status-last-error/list-exit-code · status last_error · list-exit-<code> — viola list exited with that code (for example list-exit-21); list-exit-0 when it exited 0 with no JSON object on stdout.
contract/status-last-error/list-exit-none · status last_error · list-exit-none — viola list gave no exit code, could not be spawned, or had no supervise record to run from.
contract/status-last-error/null · status last_error · null — No error.
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
decision/cleanup-is-idempotent · Idempotent cleanup — Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/harness-never-reads-cargo-bin-exe · Harness never reads CARGO_BIN_EXE — The harness takes its binaries from a passed binary dir and never reads CARGO_BIN_EXE_*. · why: Cargo sets those only for the root package's own integration tests
decision/pre-push-local-linux-gate · pre-push local Linux gate — The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
decision/pre-push-runs-no-mutation · pre-push runs no mutation — No pre-push stage runs mutation testing. · why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests.
decision/skipped-counts-real-skips-only · skipped counts real skips only — skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted. · why: so the gate's suite-skipped breach fires only on a real skip
decision/status-bounded-read · Harness status reads a bounded stdout — Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing.
decision/status-strips-bridge-env · Harness status strips the bridge env — Harness status runs viola list with the session PATH and with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed. · why: so the read files no role line under a host bridge's name
decision/supervisor-spawned-only-by-boot · Supervisor spawned only by boot — The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
contract/agent-run-logs-stream/detail-line · agent-run logs ndjson stream · detail line — Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/agent-run-logs-stream/torn-line · agent-run logs ndjson stream · torn line — A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/agent-run-status-polled-fields/wheel · agent-run status polled fields · wheel — A per-item field of the status document that the agent polls.
contract/boot-readiness-missing/child-process-start · boot readiness missing code · <name>:child-process-start — The child process start record of an instance is missing.
contract/boot-readiness-missing/endpoint · boot readiness missing code · <name>:endpoint — The endpoint of an instance is missing.
contract/boot-readiness-missing/events · boot readiness missing code · <name>:events — The events of an instance are missing.
contract/boot-readiness-missing/heartbeat · boot readiness missing code · <name>:heartbeat — The heartbeat of an instance is missing.
contract/boot-readiness-missing/run-process-start · boot readiness missing code · <name>:run-process-start — The run process start record of an instance is missing.
contract/boot-readiness-missing/snapshot · boot readiness missing code · <name>:snapshot — The snapshot of an instance is missing.
contract/cleanup-document/endpoint-gone · cleanup document · endpoint_gone — True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.
contract/cleanup-document/home-removed · cleanup document · home_removed — True when the session home was removed; reports "kept" when homes are kept, which is not a failure.
contract/cleanup-document/killed · cleanup document · killed — Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup.
contract/cleanup-exit-codes/1 · cleanup exit codes · 1 — Any false field in the cleanup document means exit 1.
contract/harness-boot-exit/0 · harness boot exit codes · 0 — Every instance is ready within the timeout.
contract/harness-boot-exit/1 · harness boot exit codes · 1 — Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
contract/harness-boot-reason/build-failed · harness boot refusal reason · build-failed — One of the four boot refusal reasons.
contract/harness-boot-reason/readiness-timeout · harness boot refusal reason · readiness-timeout — One of the four boot refusal reasons.
contract/harness-boot-reason/run-exited · harness boot refusal reason · run-exited — The wrapper exited; exit_code is the wrapper's exit code verbatim.
contract/harness-boot-reason/verify-failed · harness boot refusal reason · verify-failed — One of the four boot refusal reasons.
contract/harness-boot/default-instances · harness boot · --instance — With no --instance flag the instances are overseer and builder whatever other flags are given; one or more --instance flags replace that list.
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/harness-session/cleanup-report · harness session · cleanup report — A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/local-live-failure/build · local-live failure code · build — A closed local-live failure code.
contract/local-live-failure/verify-exit-n · local-live failure code · verify-exit-<n> — A closed local-live failure code.
contract/local-live-failure/verify-exit-none · local-live failure code · verify-exit-none — A closed local-live failure code.
contract/local-live-suite/failure-codes · the local-live suite · failure codes — A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/agent-run-shim-over-harness · agent-run shim over viola-harness — The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.
decision/rmcp-client-only · rmcp client only in the harness — The harness uses rmcp with the client feature only, without transport-child-process. · why: The test owns the viola mcp child process.
decision/status-shape-ownership · Ownership of the status shape — The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.

## log-format · key of ## 3. Test Harness Contract
contract/log-event-corr/dialog-answered · log event corr · dialog-answered — corr is the dialog_id.
contract/log-event-corr/dialog-raised · log event corr · dialog-raised — corr is the dialog_id.
contract/log-event-corr/hook-decision · log event corr · hook-decision — corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.
contract/log-event-corr/hook-invoked · log event corr · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
contract/log-event-corr/process-exit · log event corr · process-exit — corr is null.
contract/log-event-corr/process-start · log event corr · process-start — corr is null.
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.
contract/session-json/lifecycle · harness session.json · lifecycle — Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup.
contract/session-json/supervisor-pid · harness session.json · supervisor_pid — The supervisor's pid.
contract/session-json/wrapper-pid · harness session.json · wrapper_pid — The wrapper's pid.
decision/no-product-pid-file · No product pid file — The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json.

## status-endpoint-shape · key of ## 3. Test Harness Contract
contract/status-document/instances · agent-run status document · instances — One entry per booted instance: name, wrapper_pid, and alive by pid and start-time liveness.
contract/status-document/list · agent-run status document · list — The parsed viola list --json document verbatim, the exit-21 error document included; null when list exits 0 with no JSON object on stdout.
contract/status-document/ok · agent-run status document · ok — True only for state ready.
contract/status-document/pid · agent-run status document · pid — Stays null.
contract/status-document/uptime-ms · agent-run status document · uptime_ms — Stays null.
contract/status-state/degraded · agent-run status state · degraded — A booted instance is stale or absent from the list, or the document holds no items.
contract/status-state/down · agent-run status state · down — viola list exits non-zero, gives no exit code, cannot be spawned, has no supervise record to run from, or exits 0 with no JSON object on stdout.
contract/status-state/ready · agent-run status state · ready — viola list exits 0 and every booted instance is an item wrapped:true, liveness:"live"; an unwrapped item is ignored, and a session that booted none reads ready.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
decision/e2e-home-headroom-floor · Headroom floor of the e2e home base — A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom. · why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. · guard: harness::tests::e2e_home_*
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home.

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
