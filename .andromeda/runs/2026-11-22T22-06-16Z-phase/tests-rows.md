# test-plan · the rows that bind 2026-11-22-lineage-registry — picked 300 of 1373 (153 by every draw) · rule any · pin 4060

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes and oversize answer JSON are each covered by negative tests that show them refused.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and the standing live rig's live starts, never in CI.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?}.
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/leading-slash-never-a-prompt · Leading-slash argument never a prompt — A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered. · guard: send_leading_slash_argument_is_a_usage_error
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case byte-searches the outer terminal; the test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text.
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/cli-exit-codes/14 · CLI exit codes · 14 — A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
contract/instance-liveness/live · instance liveness · live — An instance whose process runs and whose heartbeat is at most 5 s old is live.
contract/instance-liveness/stale · instance liveness · stale — An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/run-start-verify-and-trust-after-bind · viola run's verify and trust steps after the bind — Between its exclusive endpoint bind and its first snapshot, a viola run start runs any automatic verify and then, on a verified final verdict, writes an untrusted spawn folder's trust record.
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id and wheel with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
decision/torn-tail-healed-by-append · Torn log tail healed by the next append — A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal. · guard: torn_tail
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then the agent's session-start with source hook.
contract/wrapper-channel/link · Wrapper channel · link — Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603.
contract/wrapper-channel/unlink · Wrapper channel · unlink — Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603.
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough · except: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
decision/self-link-refused-by-wrapper · Self-link refused by the wrapper — A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20.
contract/channel-fault-screen/-32602 · Channel fault screen · -32602 — Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null.
contract/channel-response-line/detail · channel-response log line · detail — On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param.
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.
contract/cli-send-json/empty-text · CLI send --json · not-delivered / empty-text — A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
contract/cli-send-json/exit-0 · CLI send --json · exit 0 — A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
contract/cli-send-json/human-typing · CLI send --json · exit 10 — A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
contract/mcp-send/refused-human-typing · MCP send · refused: human-typing — An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
contract/mcp-send/success · MCP send · success result — A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
contract/revive-exit-codes/0 · revive exit codes · 0 — Success.
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
contract/revive-exit-codes/2 · revive exit codes · 2 — A clap usage error, a malformed --id included.
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
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written; a paste cut at its bound writes no Enter, and the send is refused paste-cut with that cursor.
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/home-another-user-may-read-refused · Readable home refused — A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
contract/channel-links/invalid-pair · the wrapper channel · link and unlink with a bad name or pair — A raw link or unlink frame with a bad name or pair gets -32602 and no line is written.
decision/chaos-fault-sources · Chaos fault sources — Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
decision/chaos-trigger-layer · Chaos-test trigger layer — The chaos-test trigger is met by a chaos suite.
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/answer-refusal-order · answer refusal order — answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog.
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character
decision/list-row-control-escaping · list rows show every control character as hex — In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text. · guard: escape_row_shows_every_control_character_as_hex_text
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready; once the gate is ready, human-typing and turn-running again and a standing cut mark's paste-cut, before anything is issued or typed; then a cut paste's paste-cut, then no-prompt-submitted.
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
contract/link-method/bad-pair · the served link methods · bad pair — A bad pair is refused with invalid_params.
contract/cli-arguments/json-flag · CLI arguments · --json — The --json flag is read from the arguments only before a `--` separator.
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/json-rpc-errors/-32602 · wrapper channel JSON-RPC errors · -32602 — Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart.
contract/link-method/append-failure · the served link methods · failed append — An append that fails answers internal with the link set unchanged.
contract/link-method/from · the served link methods · from — A from on a link or unlink changes nothing.
contract/link-method/link-twice · the served link methods · link of a linked pair — Linking a pair twice writes one line and answers the same since.
contract/link-method/unlink-absent-pair · the served link methods · unlink of an absent pair — Unlinking an absent pair writes nothing.
contract/link-method/unlink-then-link · the served link methods · unlink then link — An unlink then a link takes a later since.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/board-on-the-read-rule · Board held to the read rule — viola list refuses a home another user may read, with and without an instance. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
contract/cli-argv/leading-slash · CLI argv · leading slash — A leading-slash argument is never delivered as a prompt.
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
contract/event-line/ts · the event line · ts — ts is RFC 3339 UTC with ms and Z.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.
contract/wrapper-channel/control-character · the wrapper channel · control-character — In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run
decision/link-since-kept · Link keeps its first since — A linked pair keeps its first since until an unlink, and takes a later one after it.
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/event-data/link · Event data keys · link — data holds the keys driver and driven.
contract/event-data/unlink · Event data keys · unlink — data holds the keys driver and driven.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21.
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
contract/strict-modes-refusal/run · Strict-modes refusal · viola run — Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
decision/channel-client-no-dir-check · Channel client carries no directory check — The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check. · why: viola-channel has no viola-state edge. · guard: channel_link_names_are_validated_by_the_wrapper
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
contract/channel-response-detail/invalid-params · channel-response line detail · invalid-params — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/mcp-unreachable-detail/strict-modes-failed · MCP frame tools instance-unreachable detail · strict-modes-failed — A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/link-name-validation · Raw-channel link name validation — link and unlink on the raw channel validate their names. · guard: channel_links
decision/list-board-refusal-opens-no-log · Board refusal opens no log — viola list's whole-board strict-modes refusal exits 21 and opens no log. · guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
decision/list-names-escaped · List names escaped — Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell. · guard: cross_list_rows
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json, the verify a keyed start runs included; every other writer-capable entry point leaves both byte-identical.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
decision/start-lines-read-from-start-offset · A start's lines read from its own offset — A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log. · why: Cursors are byte offsets and the event log is never rewritten.
decision/torn-append-healed · Torn append healed — A torn log append is counted and the next record starts on a fresh line.
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
decision/umask-independent-modes · Umask-independent modes — Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private. · guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
decision/unlink-survives-restart · Unlink survives a wrapper restart — An unlink holds across a wrapper restart. · guard: links_an_unlink_survives_a_wrapper_restart
decision/unwrapped-name-never-a-target · Unwrapped row name is never a target — A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target. · guard: e1_send_naming_an_unwrapped_row_is_unreachable
contract/list-json/name · list --json · name — list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
decision/injected-turn-keeps-wheel · Harness-injected turn keeps the wheel — A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
decision/link-is-idempotent · Link is idempotent — Re-linking a driver to the same driven instance adds no second link record.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/standing-live-rig · One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand under the founder's start cap for each chunk, each ledgered with its clock time first, and its live arm refuses under CI. · why: Ratified as the one rig every later live round reuses. · guard: rig_rehearsal · except: The SF arm's rehearsals run unwrapped in CI; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it.
contract/event-log/unlink · the event log · unlink record — viola unlink logs an unlink record.
contract/exit-causes/socket-dir-refused · exit causes · socket-dir-refused — A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
contract/viola-answer/unknown-dialog · viola answer · 13 unknown-dialog — An answer to an unknown dialog id exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
contract/viola-answer/unverified-cli · viola answer · 12 unverified-cli — An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
contract/viola-run-exit-codes/1 · viola run exit codes · 1 — A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
contract/viola-run/stale-heartbeat · viola run · stale-heartbeat — A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
contract/viola-send/exit-10 · viola send · 10 — A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
contract/viola-send/exit-11 · viola send · 11 — Retired (was budget-paused) and never renumbered or reused: no refusal maps to 11, and an older wrapper's budget-paused word reads unknown, exit 14.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/link-driver-self-reported · Link driver is a self-reported name — The driver of a link is a self-reported name and need not be a booted instance.
decision/links-survive-wrapper-restart · Links survive a wrapper restart — A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line. · guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/list-rows-escape-control-chars · list rows escape control characters — viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode. · guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/revive-preflight-order · revive preflight order — revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/stale-row-dim · Stale row shown dim — On a terminal a stale row is dim from first byte to last with its NAME dim and bold, shows stale, and no SGR touches the caption or separator. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/torn-append-by-truncation · Torn append simulated by truncation — The torn-append chaos case stops the wrapper and shortens its log in place rather than killing it mid-append. · why: The file on disk is the same, and a killed instrumented process can leave a short coverage profile.
decision/torn-append-heals · Torn append heals on the next start — A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged. · guard: chaos_torn_append
decision/tty-styling-asserted-on-sgr · Terminal styling asserted on SGR sequences — Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
contract/channel-link-names/invalid-name · channel link and unlink methods · invalid name — A driver or driven that is not a valid name, a driven other than the serving wrapper's own name, or a driver equal to driven gets -32602 with data null and appends no record.
contract/channel-link/already-linked · wrapper link method · already_linked — A second link of an already linked pair appends no record, keeps the first since, and answers already_linked: true.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
contract/list-human/columns · viola list human board · columns — The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
contract/list-human/unwrapped-separator · viola list human board · unwrapped separator — Unwrapped sessions follow the separator line "-- UNWRAPPED - READ-ONLY --".
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/start-exit-1/strict-modes-failed · run and revive exit 1 causes · strict-modes-failed — The home or instance state could be written, or read, by another user; no log opened, no line, no child.
contract/turn-ended/scheduled-wakes · turn-ended event data · scheduled_wakes — The Stop payload's session_crons array length, or null when the key is absent, null or not an array and on an API-error end; it reaches events.ndjson, CLI wait --json and MCP wait only, never last, list, the snapshot or the replay.
contract/viola-answer/human-wheel · viola answer · wheel held by human — answer exits 10 and the hook stdout stays empty.
contract/viola-list-json/live-instance · viola list --json · running wrapped instance — A running wrapped instance's item reads wrapped:true and liveness "live".
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
contract/viola-run-start/home-tree-check · viola run start · home-tree strict-modes check — The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
contract/viola-send/human-key-during-gate · viola send · human key during the gate wait — The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
contract/viola-send/human-typing · viola send · human-typing refusal — While the human holds the wheel, send exits 10 human-typing with detail null.
contract/viola-send/manual-pause · viola send · manual-pause refusal — After pause, send exits 10 with detail "manual-pause".
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.
decision/channel-e2e-driver · Channel tested with the sync client — The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
decision/channel-endpoint-from-snapshot · Channel endpoint read from the snapshot — Channel tests read the wrapper endpoint only from the instance snapshot.
decision/collision-check-over-replay-reads-beat · Collision check over a replay reads the beat — Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free. · guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/home-readable-by-other-refused · Home another user may read is refused — A pre-existing home another user may read is refused at run with exit 1. · guard: a_home_another_user_may_read_refuses_run
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/session-live-read-fails-open · session-live reading fails open — revive --fork skips the session-live reading, and a failed claude agents read passes it. · guard: revive_passes_the_session_live_reading, revive_whose_agents_read_fails_starts
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/channel-error/fault-bodies · channel error · fault bodies — The channel's faults are seven literal bodies.
contract/channel-unlink/unlink-record · wrapper unlink method · unlink record — unlink appends one unlink record (source: wrapper, {driver, driven}) and empties the snapshot's links; a second unlink appends nothing, and a later link is a new pair with a later since.
contract/cli-answer/unverified-cli · viola answer · unverified-cli — answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/cli-send/input-not-ready · viola send · input-not-ready — send exits 13 with input-not-ready when the screen model is not ready.
contract/exit-21-detail/strict-modes-failed · exit 21 instance-unreachable detail · strict-modes-failed — The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — The instance snapshot holds the wrapper channel endpoint.
contract/instance-snapshot/pinned-bin · instance snapshot · pinned_bin — The instance snapshot holds pinned_bin.
contract/instance-snapshot/started-at · instance snapshot · started_at — The instance snapshot holds started_at.
contract/list-json/liveness-stale · list --json · liveness stale — A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
contract/revive-exit-1/cwd-missing · revive exit 1 causes · cwd-missing — The recorded directory is gone, or the snapshot holds no cwd.
contract/revive-exit-1/no-session · revive exit 1 causes · no-session — No logged session for the name, or an --id the log does not hold.
contract/revive-exit-1/session-live · revive exit 1 causes · session-live — One claude agents --json read lists the logged session live in another claude; no child starts.
contract/revive-json/refusal-document · revive --json · refusal document — An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
contract/start-exit-1/already-live · run and revive exit 1 causes · already-live — The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/start-exit-1/squatted-name · run and revive exit 1 causes · squatted-name — The endpoint name is squatted.
contract/start-exit-1/stale-heartbeat · run and revive exit 1 causes · stale-heartbeat — The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
contract/viola-wait/already-past · viola wait · event already past the cursor — A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
contract/viola-wait/non-waking-records · viola wait · activity and wheel records — Activity and wheel records, and a harness-origin prompt-submitted, never wake a parked wait.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
contract/fake-agent-events/session-start · viola-fake-agent fired events · SessionStart — After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire; under --screens with an untrusted cwd only after Enter on the trust row, which draws the ready screen.
contract/fake-agent-receipt/env · viola-fake-agent receipt · env — An env receipt holds variable names, sorted, names only, never a value.
contract/fake-agent-receipt/kind · viola-fake-agent receipt file · kind — Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, step.
contract/fake-agent-receipt/line · viola-fake-agent receipt file · receipt line — The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/link-since-asserted-by-shape · Link time asserted by shape — The link line's since time is asserted by its shape only. · why: It is stamped outside the clock seam.
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it. · why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
contract/fake-agent-hooks/session-start-env · viola-fake-agent hook execution · SessionStart env — The fake agent gives every SessionStart hook child each name of its set's recorded env names, always with the fixed value fake-agent.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt. · except: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
contract/boot-readiness/events · boot readiness · events — Event lines 1 and 2 are a wheel line with cause start, then session-start with source hook; missing reads events.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/session-home-retention · Session home retention — Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
contract/secret-scan-class/mode · secret-scan hit class · mode — Unix only: a diagnostics file that is not 0600.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/status-shape-ownership · Ownership of the status shape — The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.

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
