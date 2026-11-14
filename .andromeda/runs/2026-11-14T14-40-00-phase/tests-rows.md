# test-plan · the rows that bind 2026-11-14-budget-gate-removed — picked 321 of 1375 (184 by every draw) · rule any · pin 1142

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/budget-pause-thresholds · Budget pause thresholds — Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/wheel-returns-only-via-cli-release · Wheel returns only through viola release — The wheel returns from the human only through the CLI viola release, never through a release carrying from.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/driver-send/empty-text · driver send · empty-text — An empty typed text is refused empty-text.
contract/driver-send/human-typing · driver send · human-typing — A driver send while the human holds the wheel is refused human-typing.
contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
contract/viola-hook-statusline/budget-json · viola hook statusline · budget.json — rate_limits is written to budget.json only when the payload holds a rate_limits object.
contract/viola-hook-statusline/passthrough · viola hook statusline · passthrough — The user's statusline is passed through unchanged, after budget.json is written.
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body; viola hook statusline prints the user's status-line command output unchanged.
contract/viola-list-json/budget · viola list --json · budget — The ok payload's budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or "unknown".
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, budget_paused?, dialog_pending, cli_version?, cli_verified?}.
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
decision/paste-control-characters-rejected · Control characters in pasted text rejected — ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper. · except: LF, CR, TAB and multi-byte UTF-8 are accepted.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/vt100-panic-contained · vt100 panic contained — A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer on an unverified CLI version exits 12.
contract/cli-exit-codes/13 · CLI exit codes · 13 — An unknown dialog_id given to answer exits 13 unknown-dialog.
contract/cli-exit-codes/14 · CLI exit codes · 14 — A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only.
contract/mcp-tools-list/tool-set · MCP tools/list · tool set — tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink.
contract/test-harness/status · test harness commands · status — status queries viola list --json.
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> and viola hook statusline always exit 0.
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
decision/statusline-passthrough · Statusline passthrough — The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0.
contract/budget-json/rate-limits · budget.json · rate_limits — The statusline hook writes rate_limits to budget.json only when the payload holds a rate_limits object; a malformed resets_at becomes "unknown".
contract/channel-fault-screen/-32602 · Channel fault screen · -32602 — Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null.
contract/channel-response-line/detail · channel-response log line · detail — On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param.
contract/cli-release/budget · CLI release --budget · budget_paused false — Release --budget gives budget_paused false, with override_until when a pause stood.
contract/cli-release/release-from-driver · CLI release · release-from-driver — A release carrying from is rejected with -32602 release-from-driver, exit 20.
contract/cli-release/wheel-driver · CLI release · wheel driver — Release gives wheel driver, and the next send is accepted.
contract/cli-send-json/budget-paused · CLI send --json · exit 11 — A send refused by the budget gate exits 11 with detail five-hour or seven-day.
contract/cli-send-json/empty-text · CLI send --json · not-delivered / empty-text — A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
contract/cli-send-json/human-typing · CLI send --json · exit 10 — A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
contract/event-log/budget-gate · the event log · budget-gate — A budget pause logs budget-gate with paused true and its window.
contract/mcp-reply-screen/refusal · MCP reply screen · refusal — A refusal that is no code word reads unknown.
contract/mcp-send/refused-human-typing · MCP send · refused: human-typing — An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
contract/mcp-tool-result/refusal · MCP tool result · refusal — isError true with {refusal, detail} and text "refused: <reason>".
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then budget-gate, then the agent's session-start with source hook.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/human-input-takes-the-wheel · Human input takes the wheel — A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/mcp-no-control-tools · No control tools over MCP — The MCP server offers no release, pause, link or unlink tool.
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
decision/statusline-shell-out-frame-cap · Statusline shell-out frame cap — The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command. · guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
contract/harness-run/exit · Harness run command · exit — Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
contract/mcp-reply-screen/detail · MCP reply screen · detail — A string detail that is no code word reads null, and a non-string detail is no known reply.
contract/mcp-server/tools-list · MCP server · tools/list — Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private.
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/budget-json-from-statusline · Budget file only from statusline stdin — In every booted-wrapper test, budget.json comes only from viola hook statusline stdin. · except: a reader's own unit cases may plant a malformed one only by altering the writer's own bytes
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/e2e-for-critical-paths · E2E for every critical path — All seven critical paths and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/property-and-fuzz-layer · Property tests and fuzzing — proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/dialog-bodies-pinned-by-insta · Dialog decision bodies pinned by insta — insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
decision/fuzz-weekly-and-corpus-gate · Fuzzing weekly with a corpus replay gate — cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The status line shell-out, the version gate drain, the hyprctl reader and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/budget-decision-takes-the-instant · Budget decision takes the instant as an argument — The budget decision is a pure function that takes the wall-clock instant as an argument, so its thresholds and override expiry are unit-tested with no clock seam and no mocked instant. · guard: gate_decide_*
decision/budget-governor-thresholds · Budget governor thresholds — Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override instant has passed. · guard: gate_decide_an_expired_override_is_dropped
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/dialog-hold-read-once · Dialog hold read once at start — The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection. · guard: read_hold_falls_back_to_sixty_with_one_rejection
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character
decision/hook-unknown-fields-tolerated · Unknown hook payload fields tolerated — Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/unknown-refusal-word-stays-visible · Unknown refusal word stays visible — A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit. · guard: reply_of_reads_the_three_shapes
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/mcp-reply/screened-words · MCP tool replies · refusal and detail words — A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
contract/json-rpc-errors/-32602 · wrapper channel JSON-RPC errors · -32602 — Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart.
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/wrapper-channel/control-character · the wrapper channel · control-character — In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/bad-config-value-falls-back · Bad config value falls back to its default — A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it. · guard: setting_does_not_disable_the_budget_gate
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
contract/cli-exit-codes/13-text-refused · CLI exit codes · 13 for refused text — viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
contract/wrapper-channel/empty-text · Wrapper channel · refusal detail empty-text — An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.
decision/help-no-bold-underline · Help carries no bold or underline — viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF. · guard: cli_output_plain
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/paste-validated-at-both-ends · Sent text validated at client and wrapper — Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper. · guard: channel_paste_validation
contract/channel-response-detail/invalid-params · channel-response line detail · invalid-params — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
contract/channel-response-detail/release-from-driver · channel-response line detail · release-from-driver — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/mcp-frame-tools/free-text-refusal · MCP frame tools · refusal of free text — A wrapper refusal of free text reads as unknown, with no detail.
contract/mcp-frame-tools/refusal-result · MCP frame tools · refusal result — A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/budget-pause-threshold · Budget pause threshold — A budget reading of exactly 90 pauses the driver and 89 does not.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/injected-turn-keeps-wheel · Harness-injected turn keeps the wheel — A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
decision/list-names-escaped · List names escaped — Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell. · guard: cross_list_rows
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ cases of cli_budget and cli_links
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
contract/hook-statusline/passthrough · viola hook statusline · statusline output — The statusline output passes through byte for byte, and budget.json holds the fed reading.
contract/mcp-cli-parity/refusal · MCP and CLI parity · refusal — For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.
contract/run-start-records/first-three · the event log at viola run start · first three records — The first three records are wheel{cause:"start"}, budget-gate and the hook's session-start, and the child is spawned after the first two of them; the session-start is the hook of the spawned child.
contract/viola-release/budget · viola release · --budget — release --budget lifts the budget pause for its own instance only.
contract/viola-send/empty-text · viola send · empty-text — A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
contract/viola-send/exit-10 · viola send · 10 — A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
contract/viola-send/exit-11 · viola send · 11 — A budget crossing logs budget-gate{paused:true} and send is refused exit 11 with its window word.
decision/frame-verbs-refuse-widened-instance · Frame verbs refuse a widened instance — The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line. · guard: security_negatives_strict_modes
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook and hook statusline are driven by std::process::Command with an Instant deadline over recorded fixture stdin.
decision/jsonschema-validator · jsonschema validates diagnostics — jsonschema is what validates diagnostics lines against their schemas.
decision/list-board-refusal-opens-no-log · Board refusal opens no log — viola list's whole-board strict-modes refusal exits 21 and opens no log. · guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/statusline-command-after-strict-modes · Status-line command gated by strict modes — The configured statusline_command never runs when the strict-modes check fails.
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
decision/unwrapped-name-never-a-target · Unwrapped row name is never a target — A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target. · guard: e1_send_naming_an_unwrapped_row_is_unreachable
contract/exit-causes/socket-dir-refused · exit causes · socket-dir-refused — A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
contract/list-json/name · list --json · name — list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
contract/viola-release/exit-20 · viola release · 20 — release returns the wheel; release with from exits 20.
contract/viola-send/not-delivered · viola send · not-delivered — A suppressed prompt is refused not-delivered with detail no-prompt-submitted.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/budget-gate-covers-send-only · Budget gate covers send only — The budget gate refuses automated send alone; answer, wait, last and pause behave under a closed gate as under an open one. · guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
decision/budget-override-ends-with-wrapper · Budget override ends with its wrapper — A budget override does not outlive its wrapper, so a restart over the same reading pauses again. · guard: path6_a_restart_pauses_again
decision/budget-override-per-instance · Budget override is per instance — release --budget lifts the pause for the named instance alone; other instances under the same reading stay paused and are refused. · guard: path6_two_instance_release_over_a_harness_session, gate_release_on_one_instance_leaves_another_paused
decision/driver-hints-never-name-release · Driver-facing hints never name release — A driver-facing refusal hint (human-typing, budget-paused) never contains release. · why: Drivers are refused release. · guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/five-hour-pause-threshold · Five-hour pause threshold at 90 — A `five_hour` reading of exactly 90 pauses `send` (exit 11, `detail:"five-hour"`), and 89 does not. · guard: path6_the_boundaries_and_the_window_word
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/override-expiry-tested-at-unit-level · Budget override expiry tested at unit level — Budget override expiry is asserted only in unit cases of the pure gate decision, which takes the instant as an argument, never end to end. · why: No in-process clock reaches the separately spawned wrapper, and waiting in real time would break the sleep(N) ban. · guard: gate_decide_an_expired_override_is_dropped, gate_release_override_ends_at_its_instant
decision/release-without-pause-records-nothing · Release with no pause records nothing — release --budget with no pause standing records nothing. · guard: path6_release_budget_lifts_the_pause
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/statusline-proptest-whole-seconds · Statusline property draws whole seconds — The statusline reader's property strategy draws whole seconds across the date range. · why: As first written the property stayed green on a stub.
decision/unknown-reset-does-not-pause · Unknown or passed reset does not pause — A budget window whose resets_at is "unknown", unparseable or already passed does not pause. · guard: path6_an_unknown_or_passed_reset_does_not_pause
contract/budget-gate-record/paused-false · budget-gate record · paused false — A per-instance release logs one budget-gate{paused:false, override_until} record for that instance alone, its override_until later than its ts and equal to the instance snapshot's budget_override_until.
contract/budget-gate-record/paused-true · budget-gate record · paused true — A budget pause logs a budget-gate{paused:true, window} record.
contract/budget-json/write-condition · budget.json · write condition — budget.json is written only when the statusline payload holds a rate_limits object.
contract/budget-paused-refusal/send-exit-11 · budget-paused refusal · send exit 11 — A send while the budget is paused exits 11 with detail naming the window, "five-hour" or "seven-day".
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/cli-json/instance-unreachable · CLI --json document · instance-unreachable — Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record.
contract/list-human/wheel-budget-paused · viola list human board · WHEEL cell — A budget-paused instance's WHEEL cell reads "driver - budget-paused" on that instance's row alone, with no ESC byte.
contract/list-json/budget-paused · list --json · budget_paused — The top-level .ok.budget.paused reports the budget reading's pause, while each item's budget_paused reports that instance's own gate, false under a per-instance override.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/mcp-send/human-typing · MCP send · human-typing refusal — is_error:true with text refused: human-typing.
contract/mcp-tool-input/empty-or-control-text · MCP tool input · empty or control-character text — A send text that is empty or holds a control character is not-delivered.
contract/mcp-tool-result/budget-paused · MCP tool result · budget-paused — A budget-paused send is isError:true with {"refusal":"budget-paused","detail":"five-hour"} and no cursor, equal to the CLI's document less v.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v.
contract/release-budget-json/document · release --budget --json document · document — release --budget gives {budget_paused:false, override_until} when a pause stood and {budget_paused:false} alone when none did; the next send exits 0.
contract/release-verb/driver-caller · release verb · driver caller — A driver calling release is refused with -32602, exit 20.
contract/statusline-payload/resets-at · statusline payload · resets_at — The statusline reader takes resets_at as epoch seconds or RFC 3339; a malformed value is recorded as "unknown" in budget.json.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-release/document · viola release · release document — Gives {wheel:"driver", budget_paused:false}, after which a send exits 0.
contract/viola-release/from-driver · viola release · release from a driver — Exits 20 {"error":"wrapper-fault","detail":{"code":-32602,…}} carrying release-from-driver.
contract/viola-release/running-turn · viola release · turn left running — Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
contract/viola-run-start/event-order · viola run start · first three events — The first three events.ndjson records are wheel{cause:"start"}, budget-gate, then session-start{source:"hook"} with data.cause "startup", in that order.
contract/viola-send/human-key-during-gate · viola send · human key during the gate wait — The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
contract/viola-send/human-typing · viola send · human-typing refusal — While the human holds the wheel, send exits 10 human-typing with detail null.
contract/viola-send/manual-pause · viola send · manual-pause refusal — After pause, send exits 10 with detail "manual-pause".
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/fake-agent-panic-seam-test-only · Forced-panic seam is test-only — The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it. · why: Ratified in the security-plan Decisions Log. · guard: cli_controls_not_disableable
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/list-rows-escape-control-chars · list rows escape control characters — viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode. · guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/stale-row-dim · Stale row shown dim — On a terminal a stale row is dim from first byte to last with its NAME dim and bold, shows stale, and no SGR touches the caption or separator. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/statusline-command-never-runs-unsafe · Statusline command never runs over unsafe state — The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing. · guard: statusline_over_a_replayed_snapshot_runs_nothing
decision/statusline-command-shell-and-bound · Statusline command run through sh under a bound — The wrapped statusline arm runs the recorded user command as /bin/sh -c <command> under a 5 s bound. · provisional
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
decision/vt100-panic-keeps-passthrough · vt100 panic keeps human passthrough — After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/list-human/columns · viola list human board · columns — The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
contract/mcp-tool-result/unverified-cli · MCP tool result · unverified-cli — MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
contract/mcp-tools/instance-unreachable · MCP tools · instance-unreachable — is_error:true with the CLI document less v, text refused: instance-unreachable.
contract/statusline-hook/stdout · viola hook statusline · stdout — The wrapped statusline prints the user's command output byte for byte, with exit 0 and empty stderr.
contract/statusline-settings-source/source-file · statusline settings source · source-file — viola reads the user statusline command from <home>/statusline-source.json (the Claude settings shape) when it exists, else in the default home from the user's .claude/settings.json, and in any other home from none; viola only reads it.
contract/viola-answer/human-wheel · viola answer · wheel held by human — answer exits 10 and the hook stdout stays empty.
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
contract/viola-send-human-mode/unable · viola send human mode · refusal lines — A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
contract/viola-send/no-prompt-submitted · viola send · unconfirmed send — A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
contract/viola-send/off-list-slash-text · viola send · slash text off the compiled list — Exits 13 not-delivered/no-prompt-submitted.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/budget-file-from-statusline-hook · Budget file from the statusline hook — budget.json comes from generated statusline stdin piped into viola hook statusline; the reader's own unit cases alter only bytes the writer wrote.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs · except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.
decision/mcp-exchange-recorded-by-tap · MCP exchange recorded by a test-only tap — The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
decision/mcp-tap-refuses-dirty-exchange · MCP tap refuses a dirty exchange whole — The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload, MCP exchange or Hyprland reply set that fails the unclean check refuses the whole recording.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub. · except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-agent-input/framing-paste · viola-fake-agent input · --framing paste — Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/budget-seeded-through-statusline-hook · Budget seeded through the statusline hook — A test's budget file is seeded by piping generated statusline stdin into viola hook statusline. · except: The budget reader's own unit cases pad, replace or re-version bytes the writer wrote, because no writer produces an over-cap, unparseable or v 2 file.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and Anthropic usage values by generated statusline stdin; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/jobs-end-in-harness-gate · CI jobs end in the harness gate — Each CI test job ends in viola-harness gate requiring its suites.
decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans
decision/build-fails-on-fuzz-crash · Build fails on a fuzz replay crash — The build fails when the fuzz corpus replay crashes.
decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/quality-gate-command · One quality gate over the test outputs — The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-jobs-end-in-harness-gate · CI test jobs end in the harness gate — Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/profraw-census-dev-host-only · Profraw census is a dev-host tool — The profraw census script is a dev-host tool: no CI job, pre-push stage or harness command runs it.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-interactive-snapshot-review · No interactive snapshot review as a gate — Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-test-passing-unstamped-decision-body · No test passing an unguarded decision body — No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-status-polled-fields/budget · agent-run status polled fields · budget — An envelope field of the status document that the agent polls.
contract/agent-run-status-polled-fields/budget-paused · agent-run status polled fields · budget_paused — A per-item field of the status document that the agent polls.
contract/boot-readiness/events · boot readiness · events — Event lines 1 to 3 are a wheel line with cause start, then budget-gate, then session-start with source hook; missing reads events.
contract/harness-boot/statusline-echo · harness boot · --statusline-echo — --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var and writes no budget file.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/gate-require/suites · gate --require · suites — Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/status-shape-ownership · Ownership of the status shape — The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.

## log-format · key of ## 3. Test Harness Contract
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
contract/log-event-corr/channel-response · log event corr · channel-response — corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).
contract/log-event-corr/release-from-driver · log event corr · release-from-driver — corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.

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
