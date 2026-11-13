# test-plan · the rows that bind 2026-11-13-epoch-8-cleanup — picked 493 of 1375 (313 by every draw) · rule any · pin 885

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
decision/hooks-fail-open · Hooks fail open — A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains the Cookie header or a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/pinned-exe-hash-checked · Pinned executable hash checked — A pinned executable whose hash does not match makes the command exit 1.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/release-build-viola-only · Release build carries viola only — The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step. · guard: release-check.sh and its probe
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/statusline-command-needs-strict-home · Statusline command gated on strict home — The configured statusline_command never runs when the home fails strict-modes.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/unwrapped-hooks-silent-noop · Unwrapped session hooks are a no-op — A viola hook fired in a session viola does not wrap is a silent no-op.
contract/channel-errors/32600 · channel errors · -32600 — A frame over MAX_FRAME is answered with -32600 and the connection is closed.
contract/hook-decision-body/permission · hook decision body · permission — A permission is answered with PermissionRequest allow or deny.
contract/hook-decision-body/plan · hook decision body · plan — A plan is approved through PreToolUse only and revised through PermissionRequest deny plus message.
contract/hook-decision-body/question · hook decision body · question — A question is answered with PreToolUse allow plus updatedInput.answers, annotations included.
contract/hook-dialog/unverified-null · channel hook.dialog · unverified CLI — On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty.
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it.
contract/viola-hook-statusline/passthrough · viola hook statusline · passthrough — The user's statusline is passed through unchanged, after budget.json is written.
contract/viola-hook/deadline · viola hook exec-form entry points · deadline — Spine hooks finish within the spine deadline, and SessionEnd within about 1 s.
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> and viola hook statusline always exit 0.
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr.
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body; viola hook statusline prints the user's status-line command output unchanged.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/ipc-vector-negative-tests · IPC security vector negative tests — The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
decision/mcp-negative-inputs-violate-schema · MCP negative inputs violate the schema — Negative MCP test inputs must violate the advertised tool schema. · why: The advertised schema is advisory.
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
contract/channel-errors/32602 · channel errors · -32602 — A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper.
contract/channel-errors/32603 · channel errors · -32603 — The message is exactly "internal error" and data is null.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/answer-send-exit-2 · CLI --json output · exit 2 of answer and send — On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/dialog/event-logged-once · hook dialogs · dialog event — A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
contract/dialog/one-pending · hook dialogs · one pending dialog — Exactly one dialog is pending per instance; a second concurrent dialog gets a null response.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
decision/mcp-frame-tools-check-instance-first · MCP frame tools check the instance first — The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line. · guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
decision/statusline-passthrough · Statusline passthrough — The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0.
decision/statusline-shell-out-frame-cap · Statusline shell-out frame cap — The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command. · guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
contract/hook/impostor-server · Hook · impostor server — A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable causes · strict-modes-failed — A closed cause word of instance-unreachable's detail.
contract/mcp-stdin/oversize-before-initialize · viola mcp stdin · line past MAX_FRAME before initialize — viola mcp exits 1 with both streams empty and one parse-rejected mcp-stdin oversize line.
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
contract/revive-refusal-code/strict-modes-failed · revive refusal codes · strict-modes-failed — A closed code of the revive --json refusal document.
contract/wrapper-channel/link · Wrapper channel · link — Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603.
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB.
contract/wrapper-channel/unlink · Wrapper channel · unlink — Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603.
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
decision/multi-line-send-one-prompt · Multi-line text is one prompt — A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
contract/revive-refusal-code/pinned-hash-mismatch · revive refusal codes · pinned-hash-mismatch — A closed code of the revive --json refusal document.
contract/wrapper-channel/-32603 · Wrapper channel · -32603 — internal error with data null.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
decision/insta-check-mode-only · Snapshot tests in check mode only — insta runs only in check mode, never updating snapshots.
decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
decision/machine-parseable-test-output · Machine-parseable test output — Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
decision/mutation-run-failure-conditions · Mutation run failure conditions — A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/dialog-bodies-pinned-by-insta · Dialog decision bodies pinned by insta — insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/home-another-user-may-read-refused · Readable home refused — A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/single-os-ci-leg · One CI leg on Linux — The CI matrix builds on one leg, ubuntu-latest.
decision/chaos-fault-sources · Chaos fault sources — Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/mutation-run-verdict · Mutation run verdict — A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones. · except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
contract/channel-links/invalid-pair · the wrapper channel · link and unlink with a bad name or pair — A raw link or unlink frame with a bad name or pair gets -32602 and no line is written.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/harness-logic-in-rust · Harness logic once, in Rust — One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust.
contract/harness-exit-codes/0 · harness exit codes · 0 — The harness command succeeded.
contract/harness-exit-codes/1 · harness exit codes · 1 — The harness command failed.
contract/harness-exit-codes/2 · harness exit codes · 2 — The harness command was given a usage error, and its document carries reason usage and a detail.
contract/harness-stdout/document · harness stdout · document — Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/insta-snapshots-for-decision-bodies · Insta snapshots for decision bodies — Decision bodies are pinned by insta snapshots of the body text.
contract/mcp-stdin/line-cap · MCP stdin · line cap — A line of exactly MAX_FRAME bytes, its newline counted, passes whole; one byte more passes no byte, reads as end of stream, and serve ends the session.
contract/viola-mcp-exit/oversize-before-initialize · viola mcp exit · oversize line before initialize — A stdin line past MAX_FRAME before initialize completes makes viola mcp exit 1 with both streams empty and one parse-rejected mcp-stdin / oversize line of count 1.
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/dialog-answer-mapping · Dialog answer mapping — Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny.
decision/dialog-timeouts-outlast-the-hold · Dialog timeouts outlast the longest hold — The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold. · guard: hooks_json_dialog_timeouts_outlast_the_longest_hold
decision/mcp-stdin-line-cap · MCP stdin line cap — The MCP server's stdin yields whole lines of at most MAX_FRAME bytes; a longer line releases no byte, sets the cut flag and reads as end of stream, ending a session before initialize as the initialize error. · guard: capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more
decision/private-placement-mode-first · Private copies placed mode-first — A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name. · guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
contract/e2e-home-refusal/message · the e2e-home headroom refusal · message — e2e-home backing: {free} MiB free, below the {floor} MiB floor
contract/link-method/link-twice · the served link methods · link of a linked pair — Linking a pair twice writes one line and answers the same since.
contract/link-method/unlink-absent-pair · the served link methods · unlink of an absent pair — Unlinking an absent pair writes nothing.
contract/link-method/unlink-then-link · the served link methods · unlink then link — An unlink then a link takes a later since.
decision/diagnostics-checked-before-narrowing · Diagnostics path checked before narrowing — A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/dialog-hold-expiry · Dialog hold expiry — An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line. · guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line
decision/dialog-hold-read-once · Dialog hold read once at start — The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection. · guard: read_hold_falls_back_to_sixty_with_one_rejection
decision/hook-dialog-no-frame-to-impostor · Hook writes no frame to an impostor server — The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper. · guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
decision/one-global-tracing-install · One global tracing install — The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process.
decision/pending-dialog-before-its-line · Pending dialog on the snapshot before its line — A held dialog is on the snapshot before its line is logged, and is cleared when the append fails. · guard: hook_dialog_append_that_failed_clears_the_pending_dialog
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
contract/link-method/append-failure · the served link methods · failed append — An append that fails answers internal with the link set unchanged.
contract/link-method/bad-pair · the served link methods · bad pair — A bad pair is refused with invalid_params.
contract/link-method/from · the served link methods · from — A from on a link or unlink changes nothing.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/strict-modes-refusal/hook · strict-modes refusal · hook — A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
contract/strict-modes-refusal/mcp-frame-tools · strict-modes refusal · MCP frame tools — A strict-modes refusal in the MCP frame tools gives instance-unreachable.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
contract/viola-hook/no-viola-name · viola hook · no VIOLA_NAME — With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.
contract/viola-hook/viola-dir · viola hook · VIOLA_DIR — A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
decision/board-on-the-read-rule · Board held to the read rule — viola list refuses a home another user may read, with and without an instance. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted.
decision/mcp-handlers-use-tokio-client · MCP handlers use the Tokio client — MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/link-since-kept · Link keeps its first since — A linked pair keeps its first since until an unlink, and takes a later one after it.
decision/placed-copy-execs-at-once · Placed copy closed before it is persisted — A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement. · why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY). · guard: place_private_new_placed_copy_execs_at_once
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
contract/channel-log/link-existing-pair · Wrapper channel log · link of an existing pair — Linking an already linked pair adds no log line.
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/event-data/link · Event data keys · link — data holds the keys driver and driven.
contract/event-data/unlink · Event data keys · unlink — data holds the keys driver and driven.
contract/mcp-unreachable-detail/strict-modes-failed · MCP frame tools instance-unreachable detail · strict-modes-failed — A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.
contract/strict-modes-refusal/mcp · Strict-modes refusal · viola mcp — Exit 1.
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21.
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
contract/strict-modes-refusal/run · Strict-modes refusal · viola run — Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
contract/strict-modes-refusal/verify · Strict-modes refusal · viola verify — Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail.
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/home-resolution-order · Home resolution order — The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists. · why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening. · guard: home_resolution_takes_the_flag_then_the_session_then_the_default
decision/hook-line-cases-in-made-instance-dir · Hook line cases run in a viola-made instance dir — Hook test cases that expect lines run in a viola-made instance directory. · why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json.
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/server-identity-verified · Server identity verified before any frame — A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed. · guard: security_negatives_server_identity
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
decision/verify-takes-no-json · viola verify takes no --json — viola verify takes no --json flag.
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/child-env-strip · Child environment strip — No stripped environment name reaches the agent child. · guard: the secret scan of the canary values · except: a name listed in claude_env_keep survives
decision/decision-body-gate · Decision body gate — No dialog decision body is ever emitted unless the wheel is driver and the CLI version is stamped.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/frame-verbs-refuse-widened-instance · Frame verbs refuse a widened instance — The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line. · guard: security_negatives_strict_modes
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook and hook statusline are driven by std::process::Command with an Instant deadline over recorded fixture stdin.
decision/hook-fails-open · Hook fails open silently — The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper. · guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
decision/link-is-idempotent · Link is idempotent — Re-linking a driver to the same driven instance adds no second link record.
decision/list-board-refusal-opens-no-log · Board refusal opens no log — viola list's whole-board strict-modes refusal exits 21 and opens no log. · guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/pinned-exe-hash-vectors · Pinned-exe hash pinned by vectors — The pinned-exe content hash is pinned to its library by literal test vectors.
decision/plugin-install-refusal-creates-nothing · Plugin install refusal creates nothing — viola plugin install refused by strict modes exits 21 and creates nothing. · guard: strict_modes_on_plugin_install_is_exit_21_and_creates_nothing
decision/scan-hits-never-echo-bytes · Scan hits never echo bytes — A secret-scan hit is reported by place and class, never by the matched bytes.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_what_the_wrapper_wrote
decision/start-collision-by-heartbeat · Start collision read from the heartbeat — Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not. · guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
decision/statusline-command-after-strict-modes · Status-line command gated by strict modes — The configured statusline_command never runs when the strict-modes check fails.
decision/umask-independent-modes · Umask-independent modes — Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private. · guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
decision/unlink-survives-restart · Unlink survives a wrapper restart — An unlink holds across a wrapper restart. · guard: links_an_unlink_survives_a_wrapper_restart
decision/unverified-cli-send-only · Unverified CLI degrades to send only — On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
contract/event-log/unlink · the event log · unlink record — viola unlink logs an unlink record.
contract/hook-statusline/passthrough · viola hook statusline · statusline output — The statusline output passes through byte for byte, and budget.json holds the fed reading.
contract/hook-stdout/concurrent-dialog · hook stdout · second concurrent dialog — A second concurrent dialog gets an empty hook stdout.
contract/hook-stdout/decision-body · hook stdout · decision body — The hook prints the exact decision body per dialog kind, pinned by insta snapshots.
contract/viola-run-exit-codes/1 · viola run exit codes · 1 — A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
decision/cross-user-peers-untestable · Cross-OS-user peers skipped as untestable — Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
decision/diagnostics-files-owner-only · Diagnostics files owner-only — Diagnostics files are mode 0600 on Unix. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
decision/endpoint-read-from-snapshot · Endpoint read only from the snapshot — Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/implicit-link-on-send · Implicit link on send or answer — A send or answer from another instance records an implicit link.
decision/link-name-validation · Raw-channel link name validation — link and unlink on the raw channel validate their names. · guard: channel_links
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/panic-seam-test-only · Panic seam is test-only — The forced-panic seam fires only under the fake-agent feature and never reaches the release build.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
contract/process-start/claude-child · the event log · process-start{subject:"claude-child"} — The record carries the strip counts and the stripped names only, never their values.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
contract/viola-answer/unverified-cli · viola answer · 12 unverified-cli — An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
contract/viola-send/confirmed · viola send · confirmed send — A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/collision-check-over-replay-reads-beat · Collision check over a replay reads the beat — Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free. · guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
decision/error-bodies-held-by-surface-witnesses · Error bodies held by surface witnesses — No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fake-agent-panic-seam-test-only · Forced-panic seam is test-only — The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it. · why: Ratified in the security-plan Decisions Log. · guard: cli_controls_not_disableable
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/home-readable-by-other-refused · Home another user may read is refused — A pre-existing home another user may read is refused at run with exit 1. · guard: a_home_another_user_may_read_refuses_run
decision/hook-instance-check-before-config · Hook instance check before any config read — The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing · except: The two --capture arms stay ahead of the check.
decision/hook-panic-payload-to-detail-file · Hook panic payload kept to the detail file — A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only. · guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
decision/link-driver-self-reported · Link driver is a self-reported name — The driver of a link is a self-reported name and need not be a booted instance.
decision/links-survive-wrapper-restart · Links survive a wrapper restart — A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line. · guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/no-decision-body-without-driver-and-stamp · Decision bodies only for a driver on a stamped CLI — No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/pinned-exe-hash-sha2 · sha2 for the pinned-exe content hash — The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex. · guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
decision/refused-or-own-send-records-no-link · Refused or own-name send records no link — A refused send and a send to an instance's own name record no link. · guard: links_a_refused_send_and_an_own_name_send_record_none
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value and no Cookie header appears in any log or diagnostics file.
decision/send-from-another-instance-links-once · Send from another instance links once — An issued send from another instance appends one link{driver, driven} record with source wrapper before its send-issued, and a later send of the same pair adds none. · guard: links_a_send_from_another_instance_links_once
decision/statusline-command-never-runs-unsafe · Statusline command never runs over unsafe state — The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing. · guard: statusline_over_a_replayed_snapshot_runs_nothing
decision/statusline-command-shell-and-bound · Statusline command run through sh under a bound — The wrapped statusline arm runs the recorded user command as /bin/sh -c <command> under a 5 s bound. · provisional
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
contract/caught-panic-exit-code/hook · exit code on a caught panic · hook — hook exits 0 on a caught panic.
contract/channel-link/already-linked · wrapper link method · already_linked — A second link of an already linked pair appends no record, keeps the first since, and answers already_linked: true.
contract/channel-unlink/unlink-record · wrapper unlink method · unlink record — unlink appends one unlink record (source: wrapper, {driver, driven}) and empties the snapshot's links; a second unlink appends nothing, and a later link is a new pair with a later since.
contract/exit-21-detail/strict-modes-failed · exit 21 instance-unreachable detail · strict-modes-failed — The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
contract/hook-decision/concurrent-dialog · dialog hook decision body · second concurrent dialog — A second dialog raised while one is pending gets an empty hook stdout.
contract/hook-decision/permission · dialog hook decision body · permission answer — PermissionRequest decision.behavior "allow", or "deny" with message.
contract/hook-decision/plan-approve · dialog hook decision body · plan approve — PreToolUse allow with updatedInput as the tool's own input unchanged, and no PermissionRequest body.
contract/hook-decision/plan-revise · dialog hook decision body · plan revise — PermissionRequest deny with message.
contract/hook-decision/question · dialog hook decision body · question answer — PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included.
contract/hook-decision/server-verify-failed · hook-decision record · server-verify-failed — A hook facing a server that is not the recorded wrapper logs one hook-decision line with detail "server-verify-failed", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
contract/hook-decision/socket-dir-refused · hook-decision record · socket-dir-refused — A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
contract/hook-process/exit · dialog hook process · exit and stderr — The hook process exits 0 with empty stderr.
contract/revive-json/refusal-document · revive --json · refusal document — An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/start-exit-1/strict-modes-failed · run and revive exit 1 causes · strict-modes-failed — The home or instance state could be written, or read, by another user; no log opened, no line, no child.
contract/statusline-hook/stdout · viola hook statusline · stdout — The wrapped statusline prints the user's command output byte for byte, with exit 0 and empty stderr.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-answer/allow-suggestion-fields · viola answer · permission allow — An allow's suggestion fields never reach the agent CLI.
contract/viola-answer/human-wheel · viola answer · wheel held by human — answer exits 10 and the hook stdout stays empty.
contract/viola-run-start/home-tree-check · viola run start · home-tree strict-modes check — The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
contract/viola-run-start/tampered-pinned-exe · viola run start · tampered pinned exe — A run whose pinned executable was altered exits 1.
decision/ci-scan-before-upload · CI secret scan before any upload — The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload. · except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/mcp-tests-own-server-child · MCP tests own their server child — MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport. · why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
decision/revive-preflight-order · revive preflight order — revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/channel-error/-32603 · channel error · -32603 — Every -32603 is exactly "internal error" with data null.
contract/channel-error/fault-bodies · channel error · fault bodies — The channel's faults are seven literal bodies.
contract/channel-link-names/invalid-name · channel link and unlink methods · invalid name — A driver or driven that is not a valid name, a driven other than the serving wrapper's own name, or a driver equal to driven gets -32602 with data null and appends no record.
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/cli-json/internal-error · CLI --json document · internal-error — Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
contract/dialog-events/logged-once · dialog events · question, permission, plan — Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — The instance snapshot holds the wrapper channel endpoint.
contract/instance-snapshot/pinned-bin · instance snapshot · pinned_bin — The instance snapshot holds pinned_bin.
contract/instance-snapshot/started-at · instance snapshot · started_at — The instance snapshot holds started_at.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/statusline-settings-source/source-file · statusline settings source · source-file — viola reads the user statusline command from <home>/statusline-source.json (the Claude settings shape) when it exists, else in the default home from the user's .claude/settings.json, and in any other home from none; viola only reads it.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/decision-bodies-snapshotted · Decision bodies snapshotted — Dialog decision bodies are held as insta snapshots, regenerated only by the local fixture refresh.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
decision/fake-agent-children-own-process-group · fake agent children in their own process group — On Unix every command the fake agent spawns, each hook and the status line command, runs in a process group of its own. · why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/mcp-exchange-recorded-by-tap · MCP exchange recorded by a test-only tap — The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
decision/mcp-tap-refuses-dirty-exchange · MCP tap refuses a dirty exchange whole — The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
decision/record-only-at-zero-failing-rows · fixtures recorded only when nothing fails — The viola verify recorder writes fixtures only at 0 failing rows.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub. · except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
decision/screen-fixture-scrub-as-detector · screen fixtures checked scrub-as-detector — A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-cli/unknown-arguments · viola-fake-agent command line · unknown arguments — Unknown arguments are ignored.
contract/fake-agent-events/session-start · viola-fake-agent fired events · SessionStart — After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.
contract/fake-agent-hooks/command-spawn · viola-fake-agent hook execution · hook command — The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
contract/fake-agent-hooks/status-line-command · viola-fake-agent hook execution · status line command — The fake agent runs a status line command only when the command is absolute.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-agent-input/bracketed-paste · viola-fake-agent input · bracketed paste — Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.
contract/fake-agent-input/clear · viola-fake-agent input · /clear — Under --framing the probed /clear fires the recorded clear-1 SessionEnd then SessionStart and no UserPromptSubmit.
contract/fake-agent-input/compact · viola-fake-agent input · /compact — Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
contract/fake-agent-input/framing-paste · viola-fake-agent input · --framing paste — Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.
contract/fake-agent-input/keystroke · viola-fake-agent input · keystroke — Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
contract/fake-agent-receipt/env · viola-fake-agent receipt · env — An env receipt holds variable names, sorted, names only, never a value.
contract/fake-agent-receipt/kind · viola-fake-agent receipt file · kind — Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, statusline, step.
contract/fake-agent-receipt/line · viola-fake-agent receipt file · receipt line — The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.
contract/fake-script/unreadable-exit · fake agent turn script · unreadable script — An unreadable script makes the fake agent exit 2.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
decision/fake-agent-exit-ignores-sighup · fake agent exit ignores SIGHUP — On Unix the fake agent ignores SIGHUP from the first step of its exit (\x03 or stdin's end), while a hangup landing before the exit begins still ends it. · why: The agent is its PTY's session leader and cannot leave that group, so a hangup during exit would cut its own exit-time coverage write.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/fixtures-per-cli-version · Recorded fixtures per CLI version — Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/insta-snapshots · insta for snapshots — insta is what holds snapshot expectations.
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint, log or budget.json is shared between tests.
decision/planted-hygiene-violations · planted inputs prove hygiene classes red — Every hygiene violation class is proven red by a planted input.
decision/statusline-source-read-only · Statusline source is read-only to viola — The statusline source is written by the test, never by viola, which only reads it.
decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/decision-bodies-as-insta-snapshots · Decision bodies as insta snapshots — Decision bodies are checked against insta snapshots, regenerated only by the local fixture refresh.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/link-since-asserted-by-shape · Link time asserted by shape — The link line's since time is asserted by its shape only. · why: It is stamped outside the clock seam.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/cwd · Fake-agent receipt · cwd — Carries cwd.
contract/fake-agent-receipt/fds · Fake-agent receipt · fds — Carries fds.
contract/fake-agent-receipt/hold · Fake-agent receipt · hold — Carries pid, written by the --exit-no-eof holder at its start.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/fake-agent-receipt/size · Fake-agent receipt · size — Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/fake-agent-receipt/statusline · Fake-agent receipt · statusline — Carries the hook kind's fields without event.
contract/fake-agent-receipt/step · Fake-agent receipt · step — Carries index and event.
contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/home-path · MCP fixture scrub refusal codes · home-path — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/mcp-fixture-scrub-refusal/username · MCP fixture scrub refusal codes · username — A closed refusal code of the MCP fixture scrub, naming the file only.
contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/statusline-source-written-by-test · Statusline source planted by the test — The statusline source is written by the test, never by viola, which only reads it.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision such as the budget gate's takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans
decision/artifacts-behind-secret-scan · Artifacts leave CI behind a secret scan — Every test or perf artifact leaves CI only after its job's secret scan passed; when the scan fails only the scan's hit report is uploaded.
decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.
decision/linux-only-ci-matrix · Linux-only CI matrix — CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
contract/ci-artifacts/secret-scan · CI artifacts · secret-scan-${{ matrix.os }} — Holds the test job's secret-scan hit report, uploaded only when the scan failed.
contract/ci-artifacts/secret-scan-perf · CI artifacts · secret-scan-perf-${{ matrix.os }} — Holds the perf job's secret-scan hit report, uploaded only when that scan failed.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-single-ubuntu-leg · CI runs on one ubuntu-latest leg — CI jobs run on a single ubuntu-latest leg.
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/coverage-exclusions · Coverage exclusions — The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness. · why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
decision/fail-open-spine-bound-at-unit-speed · Fail-open spine bound asserted at unit speed — Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed. · guard: hook_fails_open_silently_within_the_spine_bound
decision/hook-connect-deadline · Hook connect deadline below the spine deadline — The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces. · guard: connect_deadline_is_below_the_spine_deadline · provisional
decision/hook-deadlines-asserted-at-unit-speed · Hook deadlines asserted at unit speed — The hook deadlines are also asserted at unit speed inside the hook subprocess tests, with a monotonic `std::time::Instant` bound.
decision/host-unreachable-mutants-excluded · Host-unreachable mutants recorded as host-excluded — A missed mutant the host never compiled is recorded host-excluded: kept out of the survivors and never counted as caught.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/no-test-binary-in-release · No test-only binary in a release build — A release build carrying a test-only binary is a build failure condition.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
decision/host-unreachable-mutants · Host-unreachable mutants recorded, not caught — A mutant this host cannot compile or reach is recorded not measured here, owed to the route entry that measures it, never equivalent and never caught; one wholly under a cfg proved false for the host is left out of the count with its predicate.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/doctest-only-catch-is-missed · Doctest-only catch counts as missed — A mutant caught only by a doctest counts as missed, and the missing unit test must be added.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/coverage-ignore-regex-not-widened · Coverage ignore regex is never widened — The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first. · why: Widening it is a back door to lowering the coverage thresholds. · except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-test-passing-unstamped-decision-body · No test passing an unguarded decision body — No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/harness-output-json-and-exit-code · Harness commands always give JSON and exit code — Every harness command prints its JSON document and exit code, never a human-only message alone.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/ci-keeps-test-homes · CI keeps every test home until the gates ran — In CI, AGENT_RUN_KEEP_HOMES=1 is set job-wide in the test and perf jobs (only run --mutants overrides it to 0), so every rstest home and every harness session home of those jobs stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run. · why: obs-plan's gate steps and uploads read the homes; the ephemeral runner then discards them with the job.
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on tmpfs — On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged.
decision/e2e-home-deletion-through-link · Test-home deletion through the dev-host link — On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created. · why: A boundary widening, ratified.
decision/e2e-home-link-base · E2E home base may be a link — A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/host-excluded-mutants · Host-excluded missed mutants — A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/run-document/host-excluded · the harness run document · host_excluded — Present only when a mutant was left out: an array of name and cfg as written in source, repo-relative.
contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.
contract/secret-scan-class/cookie · secret-scan hit class · cookie — A hit of the cookie: header, case-insensitive.
contract/secret-scan-class/mode · secret-scan hit class · mode — Unix only: a diagnostics file that is not 0600.
contract/secret-scan-class/unreadable · secret-scan hit class · unreadable — A file in scope that could not be read.
contract/secret-scan-exit-codes/1 · secret-scan exit codes · 1 — Exit 1 on any hit.
contract/secret-scan-exit-codes/2 · secret-scan exit codes · 2 — Exit 2 on a usage error.
contract/secret-scan-hits-file/hits-json · secret-scan hits file · target/secret-scan/hits.json — Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run.
contract/secret-scan-output/hits · secret-scan output · hits — Each hit is {file, line, offset, class}; the matched bytes are never printed.
contract/secret-scan-output/report · secret-scan output · report — "report":"unwritten" is added when the hit file cannot be written.
contract/secret-scan-reason/empty-scope · secret-scan reason · empty-scope — Nothing in scope was found.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/no-nested-harness-in-nextest · No nested cargo run inside nextest — No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
decision/pre-push-local-linux-gate · pre-push local Linux gate — The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
decision/workspace-root-absolute · Workspace root passed absolute — The harness resolves the workspace root and passes it as an absolute path. · why: nextest runs the e2e crate's tests with that crate as the cwd
contract/boot-readiness-missing/snapshot · boot readiness missing code · <name>:snapshot — The snapshot of an instance is missing.
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
contract/pre-push-reason/pre-push-linux-only · pre-push reason · pre-push-linux-only — Exit 2.
contract/pre-push-stage/host · pre-push stage · host — Off Linux, exit 2 with pre-push-linux-only.
contract/run-reason/fuzz-linux-only · run reason · fuzz-linux-only — Exit 2.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/syn-cfg-reader-for-host-exclusion · syn as the cfg reader — syn and proc-macro2 are ordinary harness dependencies that read #[cfg] attributes for run --mutants' host exclusion.
decision/verify-window-60s-kill · 60 s kill for the verify_window_ class — Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first. · why: Their cases wait out the gate's 8.5 s maximum several times by design. · guard: mutants_verify_window_override_stands_before_the_harness_package_override
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.

## log-format · key of ## 3. Test Harness Contract
decision/hook-trace-write-failure-swallowed · Hook trace write failures swallowed — A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null.
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
contract/log-line/hook-panic · process log line · hook panic — A panic in a hook is caught and logged as one level ERROR, event panic line.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
decision/e2e-home-headroom-floor · Headroom floor of the e2e home base — A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom. · why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. · guard: harness::tests::e2e_home_*
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
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
