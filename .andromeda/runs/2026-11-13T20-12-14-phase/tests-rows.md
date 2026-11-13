# test-plan · the rows that bind 2026-11-13-rows-proposal-rule-work — picked 343 of 1375 (187 by every draw) · rule any · pin 950

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/leading-slash-never-a-prompt · Leading-slash argument never a prompt — A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered. · guard: send_leading_slash_argument_is_a_usage_error
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/paste-control-characters-rejected · Control characters in pasted text rejected — ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper. · except: LF, CR, TAB and multi-byte UTF-8 are accepted.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/row-name-never-a-target · Agent row name never a target — An agent row name is never usable as a target.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/vt100-panic-contained · vt100 panic contained — A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/answer-send-exit-2 · CLI --json output · exit 2 of answer and send — On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document.
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
contract/driver-send/empty-text · driver send · empty-text — An empty typed text is refused empty-text.
contract/driver-send/human-typing · driver send · human-typing — A driver send while the human holds the wheel is refused human-typing.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
decision/budget-pause-thresholds · Budget pause thresholds — Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/mcp-schema-violations-rejected · MCP schema-violating tool calls rejected — MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/wheel-returns-only-via-cli-release · Wheel returns only through viola release — The wheel returns from the human only through the CLI viola release, never through a release carrying from.
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer on an unverified CLI version exits 12.
contract/cli-exit-codes/13 · CLI exit codes · 13 — An unknown dialog_id given to answer exits 13 unknown-dialog.
contract/cli-exit-codes/14 · CLI exit codes · 14 — A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it.
contract/wait/instance-unreachable · wait · unreachable wrapper — A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
decision/mcp-frame-tools-check-instance-first · MCP frame tools check the instance first — The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line. · guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.
contract/cli-send-json/budget-paused · CLI send --json · exit 11 — A send refused by the budget gate exits 11 with detail five-hour or seven-day.
contract/cli-send-json/empty-text · CLI send --json · not-delivered / empty-text — A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
contract/cli-send-json/human-typing · CLI send --json · exit 10 — A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
contract/cli-send-json/no-prompt-submitted · CLI send --json · not-delivered / no-prompt-submitted — A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
contract/instance-unreachable-cause/endpoint-lost · instance-unreachable causes · endpoint-lost — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/instance-dead · instance-unreachable causes · instance-dead — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/server-verify-failed · instance-unreachable causes · server-verify-failed — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable causes · strict-modes-failed — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/unwrapped-name · instance-unreachable causes · unwrapped-name — A closed cause word of instance-unreachable's detail.
contract/revive-human/refusal · revive human output · refusal — Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty.
contract/run-refusal/socket-dir-refused · run and revive start refusals · socket-dir-refused — A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/self-link-refused-by-wrapper · Self-link refused by the wrapper — A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20.
contract/cli-frame-verbs/server-verify-failed · CLI frame verbs · server-verify-failed — A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
contract/cli-release/release-from-driver · CLI release · release-from-driver — A release carrying from is rejected with -32602 release-from-driver, exit 20.
contract/cli-wait/endpoint-lost · CLI wait · endpoint-lost — An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost.
contract/mcp-tool-result/refusal · MCP tool result · refusal — isError true with {refusal, detail} and text "refused: <reason>".
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
contract/revive-refusal-code/strict-modes-failed · revive refusal codes · strict-modes-failed — A closed code of the revive --json refusal document.
contract/run-refusal/already-live · run and revive start refusals · already-live — A live holder or a fresh replayed beat refuses the start with exit 1.
contract/run-refusal/stale-heartbeat · run and revive start refusals · stale-heartbeat — A stale holder refuses the start with exit 1.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/home-another-user-may-read-refused · Readable home refused — A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/os-gated-cases-in-binary-root · OS-gated cases stay in a split binary's root file — The OS-gated census walks only files directly under tests/ and crates/*/tests/, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file. · why: A gated case moved into the module directory would leave its gates row reading as no longer standing. · guard: contract_os_gated_tests
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/single-os-ci-leg · One CI leg on Linux — The CI matrix builds on one leg, ubuntu-latest.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.

## 3-test-harness-contract · ## 3. Test Harness Contract
contract/harness-usage-detail/unknown-suite · harness usage error detail · unknown-suite — A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
contract/mcp-tool-arguments/invalid-argument · MCP tool arguments · invalid argument — A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null.
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault.
contract/reply-screen/refusal · the reply screen · refusal — A refusal that is no code word reads unknown.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/diagnostics-checked-before-narrowing · Diagnostics path checked before narrowing — A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/private-placement-mode-first · Private copies placed mode-first — A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name. · guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
decision/unknown-refusal-word-stays-visible · Unknown refusal word stays visible — A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit. · guard: reply_of_reads_the_three_shapes
contract/human-hint-cause/not-running · the human hint cause · not-running — The hint cause of an instance-dead unreachable is not-running, a word apart from the log's.
contract/human-hint-cause/socket-dir · the human hint cause · socket-dir — The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused.
contract/unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
decision/agents-parser-fails-to-unknown · Agents list parse failure reads unknown — An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
decision/answer-refusal-order · answer refusal order — answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog.
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
contract/json-rpc-errors/wrapper-fault · wrapper channel JSON-RPC errors · a code outside the five — A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code.
contract/unreachable-log-cause/unwrapped-name · the unreachable log cause · unwrapped-name — Logged during connect.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/board-on-the-read-rule · Board held to the read rule — viola list refuses a home another user may read, with and without an instance. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size.
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/strict-modes-refusal/hook · strict-modes refusal · hook — A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
contract/strict-modes-refusal/mcp-frame-tools · strict-modes refusal · MCP frame tools — A strict-modes refusal in the MCP frame tools gives instance-unreachable.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.
contract/viola-hook/viola-dir · viola hook · VIOLA_DIR — A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
contract/wrapper-channel/control-character · the wrapper channel · control-character — In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/instance-check-before-role-file · Instance check before any role file or frame — viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found. · why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home. · guard: link_verbs_refuse_a_widened_instance_before_any_log
decision/key-after-resize-waits-for-size · Key after a resize waits for the new size — A test that writes a key after a resize sends it only once the child reports the new size. · guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
decision/resize-baseline-is-spawn-size · Pump resize baseline is the spawn size — The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child. · why: A resize can land between the spawn sizing and the pump's first look. · guard: pump_forwards_a_resize_that_lands_before_its_first_look
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/cli-exit-codes/13-text-refused · CLI exit codes · 13 for refused text — viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/mcp-unreachable-detail/strict-modes-failed · MCP frame tools instance-unreachable detail · strict-modes-failed — A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.
contract/strict-modes-refusal/mcp · Strict-modes refusal · viola mcp — Exit 1.
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21.
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
contract/strict-modes-refusal/run · Strict-modes refusal · viola run — Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
contract/strict-modes-refusal/verify · Strict-modes refusal · viola verify — Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
decision/plugin-install-owner-only · Plugin install writes owner-only — viola plugin install writes its tree at 0700 / 0600 whatever the umask, and refuses a widened home with exit 21, leaving the home as found. · guard: plugin_install_writes_owner_only_modes
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/channel-response-detail/release-from-driver · channel-response line detail · release-from-driver — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/event-data/link · Event data keys · link — data holds the keys driver and driven.
contract/mcp-frame-tools/refusal-result · MCP frame tools · refusal result — A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
contract/viola-answer/oversize-stdin · viola answer · stdin over MAX_FRAME — Exits 2 before connecting, with the fixed unusable-response line on stderr whatever --json says and no document; the dialog stays pending.
contract/wrapper-channel/empty-text · Wrapper channel · refusal detail empty-text — An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/diagnostics-files-owner-only · Diagnostics files owner-only — Diagnostics files are mode 0600 on Unix. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/frame-verbs-refuse-widened-instance · Frame verbs refuse a widened instance — The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line. · guard: security_negatives_strict_modes
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/hook-fails-open · Hook fails open silently — The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper. · guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
decision/list-board-refusal-opens-no-log · Board refusal opens no log — viola list's whole-board strict-modes refusal exits 21 and opens no log. · guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/plugin-install-refusal-creates-nothing · Plugin install refusal creates nothing — viola plugin install refused by strict modes exits 21 and creates nothing. · guard: strict_modes_on_plugin_install_is_exit_21_and_creates_nothing
decision/scan-hits-never-echo-bytes · Scan hits never echo bytes — A secret-scan hit is reported by place and class, never by the matched bytes.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_what_the_wrapper_wrote
decision/statusline-command-after-strict-modes · Status-line command gated by strict modes — The configured statusline_command never runs when the strict-modes check fails.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
decision/umask-independent-modes · Umask-independent modes — Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private. · guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
decision/unwrapped-name-never-a-target · Unwrapped row name is never a target — A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target. · guard: e1_send_naming_an_unwrapped_row_is_unreachable
contract/viola-answer/unknown-dialog · viola answer · 13 unknown-dialog — An answer to an unknown dialog id exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
contract/viola-answer/unverified-cli · viola answer · 12 unverified-cli — An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
contract/viola-send/empty-text · viola send · empty-text — A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
contract/viola-send/exit-10 · viola send · 10 — A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
contract/viola-send/exit-11 · viola send · 11 — A budget crossing logs budget-gate{paused:true} and send is refused exit 11 with its window word.
contract/viola-send/not-delivered · viola send · not-delivered — A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
contract/viola-send/unconfirmable · viola send · unconfirmable — A listed local command is confirmed only by its post-condition, or else is unconfirmable.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/implicit-link-on-send · Implicit link on send or answer — A send or answer from another instance records an implicit link.
decision/injected-turn-keeps-wheel · Harness-injected turn keeps the wheel — A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/start-collision-by-heartbeat · Start collision read from the heartbeat — Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not. · guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
contract/exit-causes/socket-dir-refused · exit causes · socket-dir-refused — A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
contract/mcp-cli-parity/exit-21 · MCP and CLI parity · exit 21 causes — For each cause of exit 21 the MCP isError body {error, detail} equals the CLI --json document less v, its text refused: instance-unreachable.
contract/mcp-cli-parity/refusal · MCP and CLI parity · refusal — For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.
contract/viola-run-exit-codes/1 · viola run exit codes · 1 — A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
contract/viola-run/stale-heartbeat · viola run · stale-heartbeat — A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/claude-child-holds-nothing-of-viola · claude child holds nothing of viola's — The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY. · guard: tui_channel_fds
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/diagnostics-files-0600 · Diagnostics files are owner-only — diagnostics/ files are mode 0600.
decision/driver-hints-never-name-release · Driver-facing hints never name release — A driver-facing refusal hint (human-typing, budget-paused) never contains release. · why: Drivers are refused release. · guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/fd-test-reads-live-child-table · fd test reads the live child's fd table — The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump. · why: The fake agent's own receipt and /dev/fd handle show as extra fds in its self-dump.
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/hook-instance-check-before-config · Hook instance check before any config read — The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing · except: The two --capture arms stay ahead of the check.
decision/no-fd-count-assertion · No fd-count assertion on the child — The child fd test asserts what the child holds, never a count of its descriptors. · why: portable-pty closes every inherited descriptor above 2 at spawn, so a planted leak cannot turn a count red, and the fake agent's own files add fds.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/budget-paused-refusal/send-exit-11 · budget-paused refusal · send exit 11 — A send while the budget is paused exits 11 with detail naming the window, "five-hour" or "seven-day".
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.
contract/exit-21-detail/endpoint-lost · exit 21 instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open; its own hint; during call.
contract/exit-21-detail/instance-dead · exit 21 instance-unreachable detail · instance-dead — No instance is running for the name with its directory standing, a stale wrapper included; during connect.
contract/exit-21-detail/server-verify-failed · exit 21 instance-unreachable detail · server-verify-failed — The listening server is not the recorded wrapper (pid or start time mismatch, or another euid); its own hint; logged as process-exit at WARN; during connect.
contract/exit-21-detail/socket-dir-refused · exit 21 instance-unreachable detail · socket-dir-refused — The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
contract/exit-21-detail/strict-modes-failed · exit 21 instance-unreachable detail · strict-modes-failed — The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
contract/exit-21-detail/unwrapped-name · exit 21 instance-unreachable detail · unwrapped-name — The name holds no instance directory (an unwrapped session or a name never started); its own hint; during connect.
contract/revive-json/refusal-document · revive --json · refusal document — An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
contract/start-exit-1/already-live · run and revive exit 1 causes · already-live — The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/start-exit-1/squatted-name · run and revive exit 1 causes · squatted-name — The endpoint name is squatted.
contract/start-exit-1/stale-heartbeat · run and revive exit 1 causes · stale-heartbeat — The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
contract/start-exit-1/strict-modes-failed · run and revive exit 1 causes · strict-modes-failed — The home or instance state could be written, or read, by another user; no log opened, no line, no child.
contract/viola-answer/human-wheel · viola answer · wheel held by human — answer exits 10 and the hook stdout stays empty.
contract/viola-run-start/home-tree-check · viola run start · home-tree strict-modes check — The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
contract/viola-send-human-mode/unable · viola send human mode · refusal lines — A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
decision/client-verifies-server-identity · Client verifies the server before any frame — A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output. · guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
decision/collision-check-over-replay-reads-beat · Collision check over a replay reads the beat — Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free. · guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/focus-mouse-resize-keep-wheel · Focus, mouse and resize never take the wheel — Focus, mouse and resize input never takes the wheel from the driver. · guard: tui_focus_mouse_and_resize_never_take_the_wheel
decision/home-readable-by-other-refused · Home another user may read is refused — A pre-existing home another user may read is refused at run with exit 1. · guard: a_home_another_user_may_read_refuses_run
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/no-decision-body-without-driver-and-stamp · Decision bodies only for a driver on a stamped CLI — No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/socket-dir-check-never-chmods · Socket directory refused, never repaired — The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it. · guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
decision/statusline-command-never-runs-unsafe · Statusline command never runs over unsafe state — The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing. · guard: statusline_over_a_replayed_snapshot_runs_nothing
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/cli-json/instance-unreachable · CLI --json document · instance-unreachable — Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
contract/viola-send/human-key-during-gate · viola send · human key during the gate wait — The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
contract/viola-send/human-typing · viola send · human-typing refusal — While the human holds the wheel, send exits 10 human-typing with detail null.
contract/viola-send/manual-pause · viola send · manual-pause refusal — After pause, send exits 10 with detail "manual-pause".
contract/viola-send/no-prompt-submitted · viola send · unconfirmed send — A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
contract/viola-send/off-list-slash-text · viola send · slash text off the compiled list — Exits 13 not-delivered/no-prompt-submitted.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs · except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/hygiene-reports-class-only · hygiene checker reports class only — The hygiene checker reports a violation class only (absolute-path, username, schema), never the offending content.
decision/mcp-exchange-recorded-by-tap · MCP exchange recorded by a test-only tap — The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
decision/mcp-tap-refuses-dirty-exchange · MCP tap refuses a dirty exchange whole — The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
decision/planted-hygiene-violations · planted inputs prove hygiene classes red — Every hygiene violation class is proven red by a planted input.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
decision/record-only-at-zero-failing-rows · fixtures recorded only when nothing fails — The viola verify recorder writes fixtures only at 0 failing rows.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub. · except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
contract/fake-agent-receipt/kind · viola-fake-agent receipt file · kind — Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, statusline, step.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/size · Fake-agent receipt · size — Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/home-path · MCP fixture scrub refusal codes · home-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/username · MCP fixture scrub refusal codes · username — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/link-since-asserted-by-shape · Link time asserted by shape — The link line's since time is asserted by its shape only. · why: It is stamped outside the clock seam.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
contract/fake-agent-receipt/fds · Fake-agent receipt · fds — Carries fds.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/linux-only-ci-matrix · Linux-only CI matrix — CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
decision/quality-gate-command · One quality gate over the test outputs — The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/harness-gate-fails-on-breach · Harness gate fails on any quality breach — viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/harness-output-json-and-exit-code · Harness commands always give JSON and exit code — Every harness command prints its JSON document and exit code, never a human-only message alone.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
contract/gate-exit-codes/0 · gate exit codes · 0 — Exit 0 with no breaches.
contract/gate-exit-codes/1 · gate exit codes · 1 — Exit 1 on any breach.
contract/gate-exit-codes/2 · gate exit codes · 2 — Exit 2 on a usage error.
contract/gate-require/suites · gate --require · suites — Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.
contract/run-document/suites · the harness run document · suites — Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names.
contract/secret-scan-class/mode · secret-scan hit class · mode — Unix only: a diagnostics file that is not 0600.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.
decision/ripgrep-verified-install · ripgrep for G1 and G3, checksum-verified — ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256. · why: taiki-e/install-action has no ripgrep manifest.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.

## log-format · key of ## 3. Test Harness Contract
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
decision/hook-trace-write-failure-swallowed · Hook trace write failures swallowed — A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.

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
