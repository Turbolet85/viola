# test-plan · the rows 2026-11-14-hyprland-exact-frame-bound-pinned touches — picked 6 of 1375 · rule any · pin 1141

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/acyclic-boundary-review · On-demand acyclic boundary review — Module boundaries are reviewed for cycles on demand rather than gated on every run. · r1
decision/agents-json-fallback-unknown · Unreadable agents listing reads unknown — Oversize or malformed claude agents --json output is read as unknown. · r1
decision/budget-pause-thresholds · Budget pause thresholds — Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it. · r1
decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny · r1
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target · r1
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier. · r1
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction. · r1
decision/child-exit-read-on-the-process-handle · Child exit is read on the process handle, never on EOF — The pump detects the child's exit by polling the process handle and never by output EOF, then drains the output still in flight for a bounded time. · why: The wrapper's exit code relies on it, and the pump tests use a reader that never ends so only the handle can end the pump. · r1
decision/cli-env-vector-negative-tests · CLI and env vector negative tests — Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused. · r1
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result. · r1
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout. · r1
decision/clippy-deny-warnings · Clippy warnings are errors — Clippy runs with warnings denied. · guard: clippy -D warnings · r1
decision/compat-tests-for-cli-versions · Compatibility tests across CLI versions — Multi-version Claude CLI fixtures are covered by compatibility tests. · why: A driver of the Comprehensive tier. · r1
decision/comprehensive-test-tier · Comprehensive test tier — The project is tested at the Comprehensive tier (2). · why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up. · r1
decision/cross-user-tested-at-peer-check · Cross-user attacks tested at the peer-credential check — Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test. · why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test. · r1
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input. · r1
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction. · r1
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI · r1
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion. · r1
decision/hooks-fail-open · Hooks fail open — A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body. · r1
decision/human-never-blocked · The human is never blocked — The human is never blocked: human typing still reaches the agent while automation is refused. · r1
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters. · r1
decision/human-takes-wheel-on-edit · Human editing takes the wheel — A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel. · r1
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor. · r1
decision/ipc-vector-negative-tests · IPC security vector negative tests — The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles. · r1
decision/leading-slash-never-a-prompt · Leading-slash argument never a prompt — A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered. · guard: send_leading_slash_argument_is_a_usage_error · r1
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line. · r1
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests · r1
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan · r2
decision/mcp-negative-inputs-violate-schema · MCP negative inputs violate the schema — Negative MCP test inputs must violate the advertised tool schema. · why: The advertised schema is advisory. · r1
decision/mcp-schema-violations-rejected · MCP schema-violating tool calls rejected — MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler. · r1
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run. · r1
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier. · r1
decision/no-advisory-ignored · No dependency advisory ignored — CI gates on cargo deny check with no advisory ignored. · except: An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5. · r2
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test. · r1
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal. · r1
decision/no-compliance-verification · No compliance verification — Compliance verification, the one Comprehensive-tier component, is not applied. · why: The security excerpt names no compliance triggers. · r1
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content. · r1
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test · r2
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control. · r1
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation. · r2
decision/paste-control-characters-rejected · Control characters in pasted text rejected — ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper. · except: LF, CR, TAB and multi-byte UTF-8 are accepted. · r1
decision/pause-forces-human-wheel · Pause forces the human wheel — pause moves the wheel to the human manually. · r1
decision/peer-euid-checked · Peer of another euid dropped — A peer of another euid is dropped by the wrapper before any frame, and a client reads server-verify-failed. · r1
decision/performance-budget-tests-for-hook-deadlines · Performance-budget tests for hook deadlines — Hook deadlines are covered by performance-budget tests. · why: A driver of the Comprehensive tier. · r1
decision/pinned-exe-hash-checked · Pinned executable hash checked — A pinned executable whose hash does not match makes the command exit 1. · r1
decision/plugin-rewritten-on-start · Plugin and settings rewritten on start — The plugin directory and the instance's settings.json are rewritten on every start. · r1
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused. · r1
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces. · r1
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor. · except: Persistent-environment names listed in config.json claude_env_keep are kept. · r1
decision/readiness-gate-tested-by-refusals · Readiness gate tested by its refusals — The readiness gate of viola run is tested through its refusal outputs. · r1
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI. · r1
decision/release-build-viola-only · Release build carries viola only — The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step. · guard: release-check.sh and its probe · r1
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated. · r1
decision/row-name-never-a-target · Agent row name never a target — An agent row name is never usable as a target. · r1
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames. · r1
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded. · r1
decision/sha-pinned-actions · SHA-pinned Actions — Every Action a workflow uses is SHA-pinned, never referenced by a mutable tag. · guard: zizmor unpinned-uses · r1
decision/single-test-only-crate · One test-only workspace member — The workspace has exactly one test-only member, crates/viola-e2e, never published. · r1
decision/statusline-command-needs-strict-home · Statusline command gated on strict home — The configured statusline_command never runs when the home fails strict-modes. · r1
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction. · r1
decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban · r1
decision/tailing-ignores-symlinks · Tailing ignores symlinks and invalid names — Tailing ignores symlinks and invalid names, and an over-long ndjson line is counted in skipped.torn_lines. · r1
decision/test-tier-single-source · Test tier is the single source of rigor — The test tier is the single source of truth for how much test rigor downstream skills apply. · r1
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg. · r1
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing. · r1
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp. · r1
decision/unwrapped-hooks-silent-noop · Unwrapped session hooks are a no-op — A viola hook fired in a session viola does not wrap is a silent no-op. · r1
decision/vt100-panic-contained · vt100 panic contained — A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it. · r1
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps. · r1
decision/wheel-returns-only-via-cli-release · Wheel returns only through viola release — The wheel returns from the human only through the CLI viola release, never through a release carrying from. · r1
decision/widened-install-dirs-refused · Widened installed directories refused — A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step. · r1
decision/workflows-audited-and-sha-pinned · Workflows audited and Actions SHA-pinned — CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code. · r1
contract/channel-errors/32600 · channel errors · -32600 — A frame over MAX_FRAME is answered with -32600 and the connection is closed. · r1
contract/channel-errors/32602 · channel errors · -32602 — A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper. · r1
contract/channel-errors/32603 · channel errors · -32603 — The message is exactly "internal error" and data is null. · r1
contract/cli-exit-codes/10-14 · CLI exit codes · 10-14 — A refused send shows the refused marker and exits with a code from 10 to 14. · r1
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer on an unverified CLI version exits 12. · r1
contract/cli-exit-codes/13 · CLI exit codes · 13 — An unknown dialog_id given to answer exits 13 unknown-dialog. · r1
contract/cli-exit-codes/14 · CLI exit codes · 14 — A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20. · r1
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb. · r1
contract/cli-json-output/answer-send-exit-2 · CLI --json output · exit 2 of answer and send — On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document. · r1
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout. · r1
contract/dialog/event-logged-once · hook dialogs · dialog event — A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once. · r1
contract/dialog/one-pending · hook dialogs · one pending dialog — Exactly one dialog is pending per instance; a second concurrent dialog gets a null response. · r1
contract/driver-send/confirmation · driver send · confirmation — A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted. · r1
contract/driver-send/empty-text · driver send · empty-text — An empty typed text is refused empty-text. · r1
contract/driver-send/human-typing · driver send · human-typing — A driver send while the human holds the wheel is refused human-typing. · r1
contract/driver-send/local-command · driver send · ledger-listed local command — A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered. · r1
contract/driver-send/send-issued · driver send · send-issued — A driver send logs a send-issued event. · r1
contract/driver-send/typed-text · driver send · typed text — The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped. · r1
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}. · r1
contract/event-line/source · the event line · source — source is one of hook or wrapper. · r1
contract/heartbeat-file/interval · instance heartbeat file · touch interval — The instance heartbeat file is touched every 1 s. · r1
contract/hook-decision-body/permission · hook decision body · permission — A permission is answered with PermissionRequest allow or deny. · r1
contract/hook-decision-body/plan · hook decision body · plan — A plan is approved through PreToolUse only and revised through PermissionRequest deny plus message. · r1
contract/hook-decision-body/question · hook decision body · question — A question is answered with PreToolUse allow plus updatedInput.answers, annotations included. · r1
contract/hook-dialog/unverified-null · channel hook.dialog · unverified CLI — On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty. · r1
contract/instance-liveness/gone · instance liveness · gone — An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age. · r1
contract/instance-liveness/live · instance liveness · live — An instance whose process runs and whose heartbeat is at most 5 s old is live. · r1
contract/instance-liveness/stale · instance liveness · stale — An instance whose process runs but whose heartbeat is older than 5 s or absent is stale. · r1
contract/instance-snapshot/pid · instance snapshot.json · pid — The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file. · r1
contract/last/result · last · result — last returns {last_assistant_message, ts} from the newest turn-ended event. · r1
contract/mcp-answer/unverified-cli · MCP answer tool · unverified-cli — MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli". · r1
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only. · r1
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it. · r1
contract/mcp-tools-list/tool-set · MCP tools/list · tool set — tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink. · r1
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness. · r1
contract/test-harness/cleanup · test harness commands · cleanup — cleanup stops every wrapper and deletes the home, idempotently. · r1
contract/test-harness/logs · test harness commands · logs — logs prints the per-process logs as JSON lines. · r1
contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives. · r1
contract/test-harness/status · test harness commands · status — status queries viola list --json. · r1
contract/unix-socket-permissions/directory · Unix socket permissions · directory — The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found. · r1
contract/unix-socket-permissions/file · Unix socket permissions · file — The Unix socket file is mode 0600. · r1
contract/viola-hook-statusline/budget-json · viola hook statusline · budget.json — rate_limits is written to budget.json only when the payload holds a rate_limits object. · r1
contract/viola-hook-statusline/passthrough · viola hook statusline · passthrough — The user's statusline is passed through unchanged, after budget.json is written. · r1
contract/viola-hook/deadline · viola hook exec-form entry points · deadline — Spine hooks finish within the spine deadline, and SessionEnd within about 1 s. · r1
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> and viola hook statusline always exit 0. · r1
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr. · r1
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body; viola hook statusline prints the user's status-line command output unchanged. · r1
contract/viola-list-json/budget · viola list --json · budget — The ok payload's budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or "unknown". · r1
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, budget_paused?, dialog_pending, cli_version?, cli_verified?}. · r1
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result. · r1
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines. · r1
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn. · r1
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail". · r1
contract/wait/already-logged · wait · after cursor — wait returns at once when a waking event is already logged at or after the after cursor. · r1
contract/wait/instance-unreachable · wait · unreachable wrapper — A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable. · r1
contract/wait/timeout · wait · timeout — wait times out with {timed_out:true}. · r1
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it. · r1

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/channel-error-bodies-literal · Channel error bodies are literals — The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text. · guard: channel_error_bodies_hold_no_path_or_upstream_text · r1
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document. · r1
decision/cross-user-at-peer-check-boundary · Cross-user rejection asserted at the boundary — Rejection of a peer with a different euid is asserted at the peer-credential check boundary, over injected euids and an injected decision, with no seam added to the client wiring. · why: Hosted CI runners have no second OS account, so a foreign peer cannot be driven. · guard: peer_euid_admitted_takes_only_the_owning_user · r1
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home. · r1
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift · r1
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern. · r1
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap. · r2
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line. · r1
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists. · r1
decision/harness-turn-keeps-the-wheel · Harness-injected turns keep the wheel — A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel. · r1
decision/human-input-takes-the-wheel · Human input takes the wheel — A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release. · r1
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal · r1
decision/local-command-exact-match · Local commands matched by exact equality — Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test. · r1
decision/local-command-post-condition · Local commands confirmed by post-condition only — On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered. · r1
decision/mcp-frame-tools-check-instance-first · MCP frame tools check the instance first — The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line. · guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame · r1
decision/mcp-negatives-violate-schema · MCP negatives violate the advertised schema — MCP tests send negative inputs that violate the advertised tool schemas. · why: The schemas are advisory. · r1
decision/mcp-no-control-tools · No control tools over MCP — The MCP server offers no release, pause, link or unlink tool. · r1
decision/multi-line-send-one-prompt · Multi-line text is one prompt — A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact. · r1
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red. · r1
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough · r1
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb. · r1
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state. · r1 · contradict
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent. · r1
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log · r1
decision/self-link-refused-by-wrapper · Self-link refused by the wrapper — A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20. · r1
decision/sent-text-typed-form · Sent text is typed in normalised form — A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched. · guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf · r1
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read. · r1
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay · r1
decision/statusline-passthrough · Statusline passthrough — The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0. · r1
decision/statusline-shell-out-frame-cap · Statusline shell-out frame cap — The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command. · guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap · r1
decision/torn-tail-healed-by-append · Torn log tail healed by the next append — A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal. · guard: torn_tail · r1
contract/budget-json/rate-limits · budget.json · rate_limits — The statusline hook writes rate_limits to budget.json only when the payload holds a rate_limits object; a malformed resets_at becomes "unknown". · r1
contract/channel-fault-screen/-32602 · Channel fault screen · -32602 — Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null. · r1
contract/channel-fault-screen/known-code · Channel fault screen · known code — Each of the five known codes takes its fixed message, with data null save the kept -32602 data. · r1
contract/channel-fault-screen/no-integer-code · Channel fault screen · error without an integer code — Reads nothing. · r1
contract/channel-fault-screen/unknown-code · Channel fault screen · code outside the five — Reads wrapper fault with data null. · r1
contract/channel-response-line/detail · channel-response log line · detail — On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param. · r1
contract/cli-frame-verbs/server-verify-failed · CLI frame verbs · server-verify-failed — A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json. · r1
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written. · r1
contract/cli-human-lines/read-back · CLI human lines · read back — [RB] read back … cursor <n> · r1
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail · r1
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:. · r1
contract/cli-release/budget · CLI release --budget · budget_paused false — Release --budget gives budget_paused false, with override_until when a pause stood. · r1
contract/cli-release/release-from-driver · CLI release · release-from-driver — A release carrying from is rejected with -32602 release-from-driver, exit 20. · r1
contract/cli-release/wheel-driver · CLI release · wheel driver — Release gives wheel driver, and the next send is accepted. · r1
contract/cli-send-json/budget-paused · CLI send --json · exit 11 — A send refused by the budget gate exits 11 with detail five-hour or seven-day. · r1
contract/cli-send-json/empty-text · CLI send --json · not-delivered / empty-text — A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper. · r1
contract/cli-send-json/exit-0 · CLI send --json · exit 0 — A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste. · r1
contract/cli-send-json/human-typing · CLI send --json · exit 10 — A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause. · r1
contract/cli-send-json/no-prompt-submitted · CLI send --json · not-delivered / no-prompt-submitted — A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged. · r1
contract/cli-wait/endpoint-lost · CLI wait · endpoint-lost — An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost. · r1
contract/event-log/budget-gate · the event log · budget-gate — A budget pause logs budget-gate with paused true and its window. · r1
contract/event-log/wheel-human-input · the event log · wheel — A human taking the wheel logs wheel with holder human and cause human-input. · r1
contract/harness-boot/readiness-failure · Harness boot command · readiness failure — Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text. · r1
contract/harness-cleanup/endpoint-gone · Harness cleanup command · endpoint check — Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file. · r1
contract/harness-cleanup/idempotent · Harness cleanup command · second run — A second cleanup exits 0 and reports nothing to clean. · r1
contract/harness-run/exit · Harness run command · exit — Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived. · r1
contract/hook/impostor-server · Hook · impostor server — A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body. · r1
contract/instance-settings/statusline-override · Instance settings file · statusline override — Written 0600 with the absolute pinned path, only for a pinned path of plain characters. · r1
contract/instance-unreachable-cause/endpoint-lost · instance-unreachable causes · endpoint-lost — A closed cause word of instance-unreachable's detail. · r1
contract/instance-unreachable-cause/instance-dead · instance-unreachable causes · instance-dead — A closed cause word of instance-unreachable's detail. · r1
contract/instance-unreachable-cause/server-verify-failed · instance-unreachable causes · server-verify-failed — A closed cause word of instance-unreachable's detail. · r1
contract/instance-unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A closed cause word of instance-unreachable's detail. · r1
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable causes · strict-modes-failed — A closed cause word of instance-unreachable's detail. · r1
contract/instance-unreachable-cause/unwrapped-name · instance-unreachable causes · unwrapped-name — A closed cause word of instance-unreachable's detail. · r1
contract/mcp-invalid-params-field/after · MCP invalid-params fields · after — A closed field word of invalid-params' detail. · r1
contract/mcp-invalid-params-field/dialog-id · MCP invalid-params fields · dialog_id — A closed field word of invalid-params' detail. · r1
contract/mcp-invalid-params-field/response · MCP invalid-params fields · response — A closed field word of invalid-params' detail. · r1
contract/mcp-invalid-params-field/target · MCP invalid-params fields · target — A closed field word of invalid-params' detail. · r1
contract/mcp-invalid-params-field/text · MCP invalid-params fields · text — A closed field word of invalid-params' detail. · r1
contract/mcp-invalid-params-field/timeout-ms · MCP invalid-params fields · timeout_ms — A closed field word of invalid-params' detail. · r1
contract/mcp-reply-screen/detail · MCP reply screen · detail — A string detail that is no code word reads null, and a non-string detail is no known reply. · r1
contract/mcp-reply-screen/refusal · MCP reply screen · refusal — A refusal that is no code word reads unknown. · r1
contract/mcp-send/refused-human-typing · MCP send · refused: human-typing — An MCP send while the human holds the wheel gives isError true with the text refused: human-typing. · r1
contract/mcp-send/success · MCP send · success result — A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result. · r1
contract/mcp-server/internal-fault · MCP server · internal handler fault — Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null. · r1
contract/mcp-server/tools-list · MCP server · tools/list — Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private. · r1
contract/mcp-server/unknown-tool · MCP server · unknown tool — The JSON-RPC error -32602. · r1
contract/mcp-stdin/oversize-before-initialize · viola mcp stdin · line past MAX_FRAME before initialize — viola mcp exits 1 with both streams empty and one parse-rejected mcp-stdin oversize line. · r1
contract/mcp-tool-result/instance-unreachable · MCP tool result · instance-unreachable — isError true with {"error":"instance-unreachable","detail":"<cause>"}, the CLI exit-21 document less v, text refused: <code>. · r1
contract/mcp-tool-result/invalid-params · MCP tool result · invalid-params — isError true with {"error":"invalid-params","detail":"<field>"}, no frame written. · r1
contract/mcp-tool-result/ok · MCP tool result · ok — isError false with structuredContent equal to the ok payload. · r1
contract/mcp-tool-result/refusal · MCP tool result · refusal — isError true with {refusal, detail} and text "refused: <reason>". · r1
contract/mcp-tool-result/wrapper-fault · MCP tool result · wrapper-fault — isError true with {"error":"wrapper-fault","detail":{code,message,data}}, the detail being the channel fault screen's. · r1
contract/revive-exit-codes/0 · revive exit codes · 0 — Success. · r1
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start. · r1
contract/revive-exit-codes/2 · revive exit codes · 2 — A clap usage error, a malformed --id included. · r1
contract/revive-human/list-row · revive human output · --list row — One <ts>  <cause>  <id> row per logged session on stdout. · r1
contract/revive-human/refusal · revive human output · refusal — Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty. · r1
contract/revive-json/list-document · revive --json · --list document — revive --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order. · r1
contract/revive-json/pump-child-error · revive --json · child pump error — An error while pumping the child prints no document, the child having held the terminal. · r1
contract/revive-refusal-code/already-live · revive refusal codes · already-live — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/cwd-missing · revive refusal codes · cwd-missing — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/internal-error · revive refusal codes · internal-error — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/no-session · revive refusal codes · no-session — A closed code of the revive --json refusal document; also the document of an empty --list chain. · r1
contract/revive-refusal-code/pinned-hash-mismatch · revive refusal codes · pinned-hash-mismatch — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/session-live · revive refusal codes · session-live — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/socket-dir-refused · revive refusal codes · socket-dir-refused — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/squatted-name · revive refusal codes · squatted-name — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/stale-heartbeat · revive refusal codes · stale-heartbeat — A closed code of the revive --json refusal document. · r1
contract/revive-refusal-code/strict-modes-failed · revive refusal codes · strict-modes-failed — A closed code of the revive --json refusal document. · r1
contract/run-refusal/already-live · run and revive start refusals · already-live — A live holder or a fresh replayed beat refuses the start with exit 1. · r1
contract/run-refusal/socket-dir-refused · run and revive start refusals · socket-dir-refused — A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused. · r1
contract/run-refusal/stale-heartbeat · run and revive start refusals · stale-heartbeat — A stale holder refuses the start with exit 1. · r1
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then budget-gate, then the agent's session-start with source hook. · r1
contract/send-confirmation/prompt-submitted-driver · send confirmation · prompt-submitted with origin driver — A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window. · r1
contract/send-event-log/send-issued · the send event log · send-issued — An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written. · r1
contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window. · r1
contract/wrapper-channel/-32603 · Wrapper channel · -32603 — internal error with data null. · r1
contract/wrapper-channel/link · Wrapper channel · link — Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603. · r1
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB. · r1
contract/wrapper-channel/protocol-fault · Wrapper channel · protocol fault — {"error":{code,message,data}}. · r1
contract/wrapper-channel/refusal · Wrapper channel · refusal — {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order. · r1
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}} · r1
contract/wrapper-channel/unlink · Wrapper channel · unlink — Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603. · r1
absence/browser-suite — A browser test suite · r1
absence/windows-and-macos-support — Windows and macOS support: code, tests, fixtures and CI jobs · r1

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/agent-driven-harness · Agent-driven harness development — Development is agent-driven through a test harness whose contract the test plan specifies. · r1
decision/harness-logic-in-rust · Harness logic once, in Rust — One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust. · r1
decision/harness-records-tolerate-unknown-fields · Harness records tolerate missing and unknown fields — The harness crate never denies unknown fields, so an older session record lacking a newer field still parses. · r1
decision/unbuilt-surface-is-usage-error · Unbuilt harness surface is a usage error — A harness flag or selector whose surface is not built yet is a usage error, never a vacuous pass. · r1
contract/harness-cli/session · harness CLI · --session — --session with an id selects a session, allowing parallel sessions. · r1
contract/harness-exit-codes/0 · harness exit codes · 0 — The harness command succeeded. · r1
contract/harness-exit-codes/1 · harness exit codes · 1 — The harness command failed. · r1
contract/harness-exit-codes/2 · harness exit codes · 2 — The harness command was given a usage error, and its document carries reason usage and a detail. · r1
contract/harness-session-record/agents-mode · harness session record · agents_mode — agents_mode is one of recorded, oversize or malformed, defaults to recorded, is written by boot and stands in supervise.json too. · r1
contract/harness-session-record/home · harness session record · home — The session record carries the session's home. · r1
contract/harness-session-record/instances · harness session record · instances — The session record lists instances, each with name, wrapper_pid, started_at and fake_args. · r1
contract/harness-session-record/session · harness session record · session — The session record carries the session id; the default session is default. · r1
contract/harness-session-record/supervisor-pid · harness session record · supervisor_pid — The session record carries the supervisor's pid. · r1
contract/harness-session-record/supervisor-started-at · harness session record · supervisor_started_at — The session record carries the supervisor's start time. · r1
contract/harness-session-record/v · harness session record · v — The session record carries its format version v. · r1
contract/harness-stdout/document · harness stdout · document — Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing. · r1
contract/harness-usage-detail/arguments · harness usage error detail · arguments — The arguments do not parse, including a flag or selector whose surface is not built yet. · r1
contract/harness-usage-detail/invalid-instance · harness usage error detail · invalid-instance — The instance named is not valid. · r1
contract/harness-usage-detail/invalid-session-id · harness usage error detail · invalid-session-id — The session id given is not valid. · r1
contract/harness-usage-detail/no-supervise-spec · harness usage error detail · no-supervise-spec — No supervise spec exists for the request. · r1
contract/harness-usage-detail/session-exists · harness usage error detail · session-exists — The session named already exists. · r1
contract/harness-usage-detail/unknown-suite · harness usage error detail · unknown-suite — A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay. · r1

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/boot-readiness-file-probe · Boot readiness is a file-state probe — boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation. · r1
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs · r1
decision/chunk-diff-pins-prefixes · Chunk diff pins its prefixes — The chunk diff covers the working tree and untracked files and always pins the a/ and b/ prefixes. · why: a host's diff prefix config would write headers the classification cannot read · guard: chunk_diff_pins_its_prefixes_under_a_hostile_config · r1
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step. · r1
decision/ci-keeps-test-homes · CI keeps every test home until the gates ran — In CI, AGENT_RUN_KEEP_HOMES=1 is set job-wide in the test and perf jobs (only run --mutants overrides it to 0), so every rstest home and every harness session home of those jobs stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run. · why: obs-plan's gate steps and uploads read the homes; the ephemeral runner then discards them with the job. · r1
decision/ci-uploads-only-after-secret-scan · CI uploads only after the secret scan — On CI failure the home's diagnostics, events and harness artifacts leave CI only in uploads gated on a passed secret scan; the only ungated uploads (the supply-chain reports among them) carry no home content. · r1
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed. · r1
decision/cleanup-is-idempotent · Idempotent cleanup — Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0. · r1
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry. · r1
decision/coverage-floors · Coverage floors — CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored. · r1
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on tmpfs — On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged. · r1
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4. · r1
decision/doc-examples-never-ignore · Doc examples never use ignore — A doc example that must not run uses no_run or a text fence, never ignore. · why: ignored doctests count as skipped · r1
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain. · r1
decision/e2e-home-deletion-through-link · Test-home deletion through the dev-host link — On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created. · why: A boundary widening, ratified. · r1
decision/e2e-home-link-base · E2E home base may be a link — A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed. · r1
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read. · r1
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree · r1
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e · r1
decision/fuzz-corpus-never-vacuous · Fuzz replay never passes vacuously — A missing or empty corpus, or no target at all, fails corpus-empty, never a vacuous pass. · r1
decision/fuzz-replay-linux-only · Fuzz replay on Linux only — Fuzz replay runs on Linux only; elsewhere it exits 2 fuzz-linux-only. · r1
decision/fuzz-reproducers-committed · Fuzz corpus committed with reproducers — The replay corpus is committed, seeded from recorded payloads or synthetic seeds, and every reproducer is committed with its fix; nothing is cached between CI runs. · r1
decision/fuzz-toolchain-single-source · Fuzz toolchain from one source — The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root. · r1
decision/harness-builds-in-its-own-target · Harness builds into its own target dir — The harness build always uses its own target dir, never the target dir the shim's cargo run uses. · why: a running harness binary could not be relinked, and a workspace-wide build re-fingerprints it · r1
decision/harness-never-reads-cargo-bin-exe · Harness never reads CARGO_BIN_EXE — The harness takes its binaries from a passed binary dir and never reads CARGO_BIN_EXE_*. · why: Cargo sets those only for the root package's own integration tests · r1
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process. · r1
decision/host-excluded-mutants · Host-excluded missed mutants — A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted. · r1
decision/internal-harness-subcommands-off-agent-surface · Internal harness subcommands — The internal harness subcommands are implemented in viola-harness, forwarded unchanged by the shims, and are not part of the agent's 5-command surface. · r1
decision/junit-copied-after-each-invocation · JUnit report copied after each nextest invocation — The run command deletes the nextest junit report before each invocation and copies it per suite right after; a report missing afterwards is an artifact-missing breach, never a copy of an earlier run's file. · why: nextest overwrites that path on every invocation. · r1
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays · r1
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite. · r1
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI. · r1
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1. · r1
decision/members-declare-fake-agent-feature · Mutated members declare fake-agent — Every workspace member but viola-core declares a fake-agent feature, a no-op where it has none. · why: cargo-mutants passes the feature to package-scoped runs, and a missing feature fails the run · r1
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass. · r1
decision/mutant-build-timeout · Fixed mutant build timeout — Each mutant's build is bounded by a fixed 400 s, five times the largest measured baseline, not a multiplier. · why: a multiplier derives a sub-second bound from a sub-second baseline · r1
decision/mutants-output-live · cargo-mutants output streams live — cargo-mutants' output goes live to the harness's stderr with every outcome line printed, never captured. · why: a stalled or cancelled run names the mutant it stopped at · r1
decision/mutants-survived-count · Mutants survived count — The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes. · r1
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests. · r1
decision/mutation-base-whole-chunk · Mutation base is the whole chunk — The mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending record, so every run of a pass mutates the whole chunk; an unresolved base exits 1 base-missing and no empty diff is written. · why: an unbounded search moves mid-pass, and an empty diff reads as a false zero-mutant pass · r1
decision/mutation-diff-classification · Diffs with no mutable Rust skip cargo-mutants — A diff naming no Rust path passes as no-rust-delta and one whose Rust paths are all test targets passes as test-only-rust-delta, both without reaching cargo-mutants; any other deletes the old outcomes first and is red with none. · why: cargo-mutants exits 0 and writes no outcomes on such diffs · r1
decision/mutation-keeps-no-test-home · Mutation run keeps no test home — A mutation run keeps no test home, overriding the workflow's keep-homes setting. · why: every home carries a pinned copy of the viola binary · r1
decision/mutation-own-target-dir · Mutation run builds in its own target dir — The mutation run pre-builds the root package and builds every mutant in its own target dir, copied into each scratch tree. · why: a copied default target keeps the original tree's binary paths, so tests drove the unmutated binary · r1
decision/mutation-verdict-by-counts · Mutation verdict by counts — A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide. · why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing · r1
decision/nightly-fuzz-fails-on-crash · Nightly fuzz fails on any crash — The nightly time-boxed fuzz run uploads its artifacts and fails the job on any crash. · r1
decision/no-ci-mutants-gate · No CI mutants gate — No CI job gates the mutants suite. · r1
decision/no-nested-harness-in-nextest · No nested cargo run inside nextest — No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest. · r1
decision/panic-counts-read-role-files-only · Panic counts read role files only — Panic counts over the logs stream use the role files only, excluding detail files. · why: A panic line appears both in a role file and in its detail file. · r1
decision/perf-verdict-belongs-to-gate · Perf verdict belongs to the gate — The perf arm runs only when named and reports its suite; the deadline verdict belongs to the gate's perf requirement, never to run. · r1
decision/pre-push-clean-environment · pre-push children run in a clean environment — Every pre-push child runs under env -i with HOME from the passwd entry and a PATH of constants only, inheriting no host environment; the harness's own HOME is never read. · why: No CLAUDE* value crosses. · r1
decision/pre-push-local-linux-gate · pre-push local Linux gate — The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command. · r1
decision/pre-push-runs-no-mutation · pre-push runs no mutation — No pre-push stage runs mutation testing. · why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests. · r1
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout · r1
decision/scoped-mutants-not-the-verdict · Scoped mutation is the inner fix loop — A file-scoped mutation run is the inner fix loop; the counted chunk mutation run stays the verdict. · r1
decision/secret-scan-scope · Secret scan scope — The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload. · why: The run --mutants chunk diff is repository source text that hits the scan's own needles. · except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned. · r1
decision/session-home-retention · Session home retention — Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept. · r1
decision/skipped-counts-real-skips-only · skipped counts real skips only — skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted. · why: so the gate's suite-skipped breach fires only on a real skip · r1
decision/status-bounded-read · Harness status reads a bounded stdout — Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing. · r1
decision/status-strips-bridge-env · Harness status strips the bridge env — Harness status runs viola list with the session PATH and with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed. · why: so the read files no role line under a host bridge's name · r1
decision/supervisor-ordinary-child · Supervisor is an ordinary child — The supervisor is spawned as an ordinary child with no detach flags, outlives boot, and stops only through its stop request. · r1
decision/supervisor-spawned-only-by-boot · Supervisor spawned only by boot — The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears. · r1
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer. · r1
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped. · r1
decision/workspace-root-absolute · Workspace root passed absolute — The harness resolves the workspace root and passes it as an absolute path. · why: nextest runs the e2e crate's tests with that crate as the cwd · r1
contract/agent-run-logs-stream/detail-line · agent-run logs ndjson stream · detail line — Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances. · r1
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}. · r1
contract/agent-run-logs-stream/events-line · agent-run logs ndjson stream · events line — Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}. · r1
contract/agent-run-logs-stream/torn-line · agent-run logs ndjson stream · torn line — A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped. · r1
contract/agent-run-status-exit-codes/0 · agent-run status exit codes · 0 — Exit 0 when ok is true, which is only for state ready. · r1
contract/agent-run-status-exit-codes/1 · agent-run status exit codes · 1 — Exit 1 for degraded or down; the full status document is still printed, so an agent waiting for a stale transition reads exit 1 as a state, not a harness fault. · r1
contract/agent-run-status-exit-codes/2 · agent-run status exit codes · 2 — Exit 2 on a usage error, or when no session.json exists for the id, printing {"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}. · r1
contract/agent-run-status-polled-fields/budget · agent-run status polled fields · budget — An envelope field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/budget-paused · agent-run status polled fields · budget_paused — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/cli-verified · agent-run status polled fields · cli_verified — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/dialog-pending · agent-run status polled fields · dialog_pending — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/liveness · agent-run status polled fields · liveness — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/name · agent-run status polled fields · name — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines. · r1
contract/agent-run-status-polled-fields/status · agent-run status polled fields · status — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/wheel · agent-run status polled fields · wheel — A per-item field of the status document that the agent polls. · r1
contract/agent-run-status-polled-fields/wrapped · agent-run status polled fields · wrapped — A per-item field of the status document that the agent polls. · r1
contract/boot-readiness-missing/child-process-start · boot readiness missing code · <name>:child-process-start — The child process start record of an instance is missing. · r1
contract/boot-readiness-missing/endpoint · boot readiness missing code · <name>:endpoint — The endpoint of an instance is missing. · r1
contract/boot-readiness-missing/events · boot readiness missing code · <name>:events — The events of an instance are missing. · r1
contract/boot-readiness-missing/heartbeat · boot readiness missing code · <name>:heartbeat — The heartbeat of an instance is missing. · r1
contract/boot-readiness-missing/run-process-start · boot readiness missing code · <name>:run-process-start — The run process start record of an instance is missing. · r1
contract/boot-readiness-missing/snapshot · boot readiness missing code · <name>:snapshot — The snapshot of an instance is missing. · r1
contract/boot-readiness/events · boot readiness · events — Event lines 1 to 3 are a wheel line with cause start, then budget-gate, then session-start with source hook; missing reads events. · r1
contract/boot-readiness/heartbeat · boot readiness · heartbeat — The heartbeat mtime is less than 5 s old; missing reads heartbeat. · r1
contract/boot-readiness/process-start · boot readiness · process-start — The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited. · r1
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint. · r1
contract/cleanup-document/endpoint-gone · cleanup document · endpoint_gone — True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it. · r1
contract/cleanup-document/home-removed · cleanup document · home_removed — True when the session home was removed; reports "kept" when homes are kept, which is not a failure. · r1
contract/cleanup-document/killed · cleanup document · killed — Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup. · r1
contract/cleanup-document/processes-gone · cleanup document · processes_gone — True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first. · r1
contract/cleanup-exit-codes/1 · cleanup exit codes · 1 — Any false field in the cleanup document means exit 1. · r1
contract/coverage-failure/llvm-cov-exit · coverage suite failure codes · llvm-cov-exit-N — A red llvm-cov exit over green tests, such as a floor breach. · r1
contract/coverage-failure/llvm-cov-summary-missing · coverage suite failure codes · llvm-cov-summary-missing — The coverage summary was not written. · r1
contract/gate-breach/artifact-missing · gate breach · artifact-missing — A required artifact is missing, each absent perf row its own breach naming that file. · r1
contract/gate-breach/coverage · gate breach · coverage — Coverage below a floor, each floor its own breach. · r1
contract/gate-breach/mutants · gate breach · mutants — The mutants suite has survived > 0. · r1
contract/gate-breach/perf · gate breach · perf — A perf result's max is at or over its gate; detail reads {file} max {max} >= {bound}. · r1
contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0. · r1
contract/gate-breach/suite-missing · gate breach · suite-missing — A required suite is absent from the run summary. · r1
contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0. · r1
contract/gate-exit-codes/0 · gate exit codes · 0 — Exit 0 with no breaches. · r1
contract/gate-exit-codes/1 · gate exit codes · 1 — Exit 1 on any breach. · r1
contract/gate-exit-codes/2 · gate exit codes · 2 — Exit 2 on a usage error. · r1
contract/gate-output/detail · gate output · detail — A fixed code or a count, never file content. · r1
contract/gate-require/suites · gate --require · suites — Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite. · r1
contract/harness-boot-exit/0 · harness boot exit codes · 0 — Every instance is ready within the timeout. · r1
contract/harness-boot-exit/1 · harness boot exit codes · 1 — Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing. · r1
contract/harness-boot-reason/build-failed · harness boot refusal reason · build-failed — One of the four boot refusal reasons. · r1
contract/harness-boot-reason/readiness-timeout · harness boot refusal reason · readiness-timeout — One of the four boot refusal reasons. · r1
contract/harness-boot-reason/run-exited · harness boot refusal reason · run-exited — The wrapper exited; exit_code is the wrapper's exit code verbatim. · r1
contract/harness-boot-reason/verify-failed · harness boot refusal reason · verify-failed — One of the four boot refusal reasons. · r1
contract/harness-boot/agents-mode · harness boot · --agents-mode — --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records. · r1
contract/harness-boot/default-instances · harness boot · --instance — With no --instance flag the instances are overseer and builder whatever other flags are given; one or more --instance flags replace that list. · r1
contract/harness-boot/statusline-echo · harness boot · --statusline-echo — --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var and writes no budget file. · r1
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build. · r1
contract/harness-run-exit/0 · harness run exit codes · 0 — Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught. · r1
contract/harness-run-exit/non-zero · harness run exit codes · non-zero — Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red. · r1
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default. · r1
contract/harness-run/mutants-exit-reason · harness run · mutants-exit-<code> — cargo-mutants exit 1, 4, 5, 6 or 70 fails run --mutants with reason mutants-exit-<code>. · r1
contract/harness-run/mutants-only-flags · harness run · --file and --package — --file and --package each require --mutants. · r1
contract/harness-run/package-refused · harness run · package-refused — A --package that is not viola or a member declaring fake-agent, named in lowercase ASCII, digits, - and _, exits 2 package-refused before any cargo runs. · r1
contract/harness-run/tool-missing · harness run · tool-missing — Exits 1 with a detail naming the missing tool or file, such as cargo-fuzz. · r1
contract/harness-session/cleanup-report · harness session · cleanup report — A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command. · r1
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir. · r1
contract/local-live-failure/build · local-live failure code · build — A closed local-live failure code. · r1
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code. · r1
contract/local-live-failure/verify-exit-n · local-live failure code · verify-exit-<n> — A closed local-live failure code. · r1
contract/local-live-failure/verify-exit-none · local-live failure code · verify-exit-none — A closed local-live failure code. · r1
contract/local-live-suite/failure-codes · the local-live suite · failure codes — A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing. · r1
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that. · r1
contract/mutants-verdict/scoped · run --mutants verdict · scoped — The run was scoped by --file. · r1
contract/pre-push-detail/cargo-llvm-cov · pre-push detail · cargo-llvm-cov — A closed pre-push detail. · r1
contract/pre-push-detail/cargo-mutants · pre-push detail · cargo-mutants — A closed pre-push detail. · r1
contract/pre-push-detail/cargo-nextest · pre-push detail · cargo-nextest — A closed pre-push detail. · r1
contract/pre-push-detail/cc · pre-push detail · cc — A closed pre-push detail. · r1
contract/pre-push-detail/gate · pre-push detail · gate — The gate document was unreadable. · r1
contract/pre-push-detail/passwd-home · pre-push detail · passwd-home — Detail of tool-missing. · r1
contract/pre-push-detail/pins-unreadable · pre-push detail · pins-unreadable — A closed pre-push detail. · r1
contract/pre-push-detail/run · pre-push detail · run — The run document was unreadable. · r1
contract/pre-push-detail/rustc · pre-push detail · rustc — A closed pre-push detail. · r1
contract/pre-push-exit-codes/0 · pre-push exit codes · 0 — Exit 0 when every stage is green. · r1
contract/pre-push-exit-codes/1 · pre-push exit codes · 1 — Exit 1 on a red stage or a stop reason. · r1
contract/pre-push-exit-codes/2 · pre-push exit codes · 2 — Exit 2 on pre-push-linux-only or a usage error. · r1
contract/pre-push-output/paths · pre-push output · paths — Codes, counts and repo-relative names only, never a home or repository path. · r1
contract/pre-push-reason/linux-document-unreadable · pre-push reason · linux-document-unreadable — The detail, run or gate, names the harness command whose document was unreadable. · r1
contract/pre-push-reason/pre-push-linux-only · pre-push reason · pre-push-linux-only — Exit 2. · r1
contract/pre-push-reason/tool-missing · pre-push reason · tool-missing — Reused from run; its detail names the missing tool or passwd-home. · r1
contract/pre-push-reason/tool-pin-mismatch · pre-push reason · tool-pin-mismatch — A closed pre-push reason. · r1
contract/pre-push-stage/host · pre-push stage · host — Off Linux, exit 2 with pre-push-linux-only. · r1
contract/pre-push-stage/linux-tests · pre-push stage · linux-tests — Runs run --coverage then gate --require coverage,doctest; the last stage, ok:true when green. · r1
contract/pre-push-stage/tools · pre-push stage · tools — Checks the passwd home, cc, and rustc, cargo-nextest, cargo-mutants and cargo-llvm-cov at their pinned versions. · r1
contract/run-document/archived · the harness run document · archived — The repo-relative run-archive dir holding this run's JUnit files and read outcomes, the newest 10 kept and never uploaded; an absent source is skipped and named on stderr. · r1
contract/run-document/host-excluded · the harness run document · host_excluded — Present only when a mutant was left out: an array of name and cfg as written in source, repo-relative. · r1
contract/run-document/mutants · the harness run document · mutants — A --mutants run adds a mutants object with tested and a verdict of counted, no-rust-delta, test-only-rust-delta, scoped or package, with the full base sha except on package. · r1
contract/run-document/reason · the harness run document · reason — A document refused by a tool arm carries reason and, where one applies, detail. · r1
contract/run-document/suites · the harness run document · suites — Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names. · r1
contract/run-reason/corpus-empty · run reason · corpus-empty — A closed run reason. · r1
contract/run-reason/fuzz-linux-only · run reason · fuzz-linux-only — Exit 2. · r1
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2. · r1
contract/run-reason/package-refused · run reason · package-refused — Exit 2. · r1
contract/run-reason/tool-missing · run reason · tool-missing — run --perf and --fuzz-replay check their tool is on PATH first; when missing they exit 1 with this reason and the tool name, never a silent pass. · r1
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require. · r1
contract/run-usage-detail/unknown-suite · run usage detail · unknown-suite — The closed usage detail for a suite value outside the closed list. · r1
contract/schema-check-exit-codes/2 · schema-check exit codes · 2 — Exit 2 on a usage error. · r1
contract/schema-check-output/failures · schema-check output · failures — Each failure is {file, line, keyword}, file relative to target/e2e-home; no line content is ever printed. · r1
contract/schema-check-output/skipped-non-file · schema-check output · skipped_non_file — The count of directories at a diagnostics file path, which are never read. · r1
contract/schema-check-output/torn · schema-check output · torn — The count of non-JSON lines. · r1
contract/schema-check-reason/empty-scope · schema-check reason · empty-scope — No diagnostics file was found. · r1
contract/schema-check-reason/schema-unreadable · schema-check reason · schema-unreadable — A schema could not be read. · r1
contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant. · r1
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file. · r1
contract/secret-scan-class/mode · secret-scan hit class · mode — Unix only: a diagnostics file that is not 0600. · r1
contract/secret-scan-class/unreadable · secret-scan hit class · unreadable — A file in scope that could not be read. · r1
contract/secret-scan-exit-codes/1 · secret-scan exit codes · 1 — Exit 1 on any hit. · r1
contract/secret-scan-exit-codes/2 · secret-scan exit codes · 2 — Exit 2 on a usage error. · r1
contract/secret-scan-hits-file/hits-json · secret-scan hits file · target/secret-scan/hits.json — Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run. · r1
contract/secret-scan-output/hits · secret-scan output · hits — Each hit is {file, line, offset, class}; the matched bytes are never printed. · r1
contract/secret-scan-output/report · secret-scan output · report — "report":"unwritten" is added when the hit file cannot be written. · r1
contract/secret-scan-reason/empty-scope · secret-scan reason · empty-scope — Nothing in scope was found. · r1
contract/status-last-error/list-exit-code · status last_error · list-exit-<code> — viola list exited with that code (for example list-exit-21); list-exit-0 when it exited 0 with no JSON object on stdout. · r1
contract/status-last-error/list-exit-none · status last_error · list-exit-none — viola list gave no exit code, could not be spawned, or had no supervise record to run from. · r1
contract/status-last-error/null · status last_error · null — No error. · r1
contract/supervise-exit-codes/0 · supervise exit codes · 0 — Exit 0 when no child needed kill(). · r1
contract/supervise-exit-codes/1 · supervise exit codes · 1 — Exit 1 when any child needed kill(). · r1
contract/supervise-exit-codes/2 · supervise exit codes · 2 — Exit 2 on a usage error. · r1
contract/supervisor-exit-file/exits · supervisor-exit.json · exits — One entry per instance: {"name":<instance>,"exit_code":<int|null>,"killed":<bool>}, inside {"v":1,"cmd":"supervise","ok":<bool>,"exits":[…]}. · r1
contract/supervisor-exit-file/stdout · supervisor-exit.json · stdout — The supervisor's stdout is discarded; the JSON document is the file. · r1

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/agent-run-shim-over-harness · agent-run shim over viola-harness — The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs. · r1
decision/cargo-llvm-cov-coverage · cargo-llvm-cov for coverage — cargo-llvm-cov 0.9.1, installed through taiki-e/install-action, produces coverage as lcov and a JSON summary. · r1
decision/ci-actions-sha-pinned · SHA-pinned actions, no default permissions — Every CI action is SHA-pinned, with permissions: {} at the workflow's top level. · r1
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check. · r1
decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods). · r1
decision/ci-tools-version-pinned · Version-pinned CI tools — Every tool CI installs is version-pinned; binaries the runner image ships, such as jq and python3, run unpinned. · r1
decision/fixture-git-no-auto-maintenance · Fixture git repos without auto maintenance — Fixture git repositories in tests run git with maintenance.auto=false. · why: The detached auto maintenance each fixture commit spawned wrote into .git while the temp dir's drop walked it, leaving half-removed dirs. · r1
decision/g2-zero-panics · G2 zero panics — CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs. · except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>. · r1
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break. · r1
decision/mutants-fail-fast-wait · Mutants profile stops after one failure and waits — The mutants nextest profile stops scheduling at the first failure and lets the tests already running finish (terminate = wait). · why: Running tests then drop their temp dirs and session guards; immediate termination left leftover temp dirs. · r1
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line · r1
decision/ripgrep-verified-install · ripgrep for G1 and G3, checksum-verified — ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256. · why: taiki-e/install-action has no ripgrep manifest. · r1
decision/rmcp-client-only · rmcp client only in the harness — The harness uses rmcp with the client feature only, without transport-child-process. · why: The test owns the viola mcp child process. · r1
decision/runner-provided-jq · Runner-provided jq in CI scripts — CI scripts use the runner-provided jq, presence-checked and never installed, and refuse with tool-missing: jq without it. · r1
decision/send-window-paste-hint-30s-kill · 30 s mutants kill for send-window classes — Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill. · why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint, within a second of the 10 s default kill. · guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill · r1
decision/status-shape-ownership · Ownership of the status shape — The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it. · r1
decision/syn-cfg-reader-for-host-exclusion · syn as the cfg reader — syn and proc-macro2 are ordinary harness dependencies that read #[cfg] attributes for run --mutants' host exclusion. · r1
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail. · r1
decision/verify-window-60s-kill · 60 s kill for the verify_window_ class — Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first. · why: Their cases wait out the gate's 8.5 s maximum several times by design. · guard: mutants_verify_window_override_stands_before_the_harness_package_override · r1
decision/viola-e2e-30s-mutants-kill · 30 s mutants kill for the harness package — Under the mutants profile the viola-e2e package takes a 30 s per-test kill. · why: The harness's own tests wait out its 20 s boot deadline by design. · r1
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill. · r1

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files. · r1
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision. · r1
decision/hook-trace-write-failure-swallowed · Hook trace write failures swallowed — A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr. · r1
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics. · r1
contract/log-event-corr/channel-request · log event corr · channel-request — corr is the JSON-RPC request id; null for an id-less hook.event notification. · r1
contract/log-event-corr/channel-response · log event corr · channel-response — corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600). · r1
contract/log-event-corr/dialog-answered · log event corr · dialog-answered — corr is the dialog_id. · r1
contract/log-event-corr/dialog-raised · log event corr · dialog-raised — corr is the dialog_id. · r1
contract/log-event-corr/hook-decision · log event corr · hook-decision — corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null. · r1
contract/log-event-corr/hook-invoked · log event corr · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known. · r1
contract/log-event-corr/panic · log event corr · panic — corr is null. · r1
contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null. · r1
contract/log-event-corr/process-exit · log event corr · process-exit — corr is null. · r1
contract/log-event-corr/process-start · log event corr · process-start — corr is null. · r1
contract/log-event-corr/release-from-driver · log event corr · release-from-driver — corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line. · r1
contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor. · r1
contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor. · r1
contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null. · r1
contract/log-event-corr/state-recovered · log event corr · state-recovered — corr is null. · r1
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line. · r1
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace. · r1
contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines. · r1
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id). · r1
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds. · r1
contract/log-line/hook-panic · process log line · hook panic — A panic in a hook is caught and logged as one level ERROR, event panic line. · r1
contract/log-line/instance · process log line · instance — A ViolaName or null. · r1
contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR. · r1
contract/log-line/message · process log line · message — fields.message, or the flattened message; required. · r1
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null. · r1
contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file. · r1
contract/log-line/target · process log line · target — A required field. · r1
contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z. · r1
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}. · r1

## pid-file · key of ## 3. Test Harness Contract
decision/no-product-pid-file · No product pid file — The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json. · r1
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed. · r1
contract/session-json/lifecycle · harness session.json · lifecycle — Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup. · r1
contract/session-json/supervisor-pid · harness session.json · supervisor_pid — The supervisor's pid. · r1
contract/session-json/wrapper-pid · harness session.json · wrapper_pid — The wrapper's pid. · r1

## status-endpoint-shape · key of ## 3. Test Harness Contract
contract/status-document/instances · agent-run status document · instances — One entry per booted instance: name, wrapper_pid, and alive by pid and start-time liveness. · r1
contract/status-document/list · agent-run status document · list — The parsed viola list --json document verbatim, the exit-21 error document included; null when list exits 0 with no JSON object on stdout. · r1
contract/status-document/ok · agent-run status document · ok — True only for state ready. · r1
contract/status-document/pid · agent-run status document · pid — Stays null. · r1
contract/status-document/uptime-ms · agent-run status document · uptime_ms — Stays null. · r1
contract/status-state/degraded · agent-run status state · degraded — A booted instance is stale or absent from the list, or the document holds no items. · r1
contract/status-state/down · agent-run status state · down — viola list exits non-zero, gives no exit code, cannot be spawned, has no supervise record to run from, or exits 0 with no JSON object on stdout. · r1
contract/status-state/ready · agent-run status state · ready — viola list exits 0 and every booted instance is an item wrapped:true, liveness:"live"; an unwrapped item is ignored, and a session that booted none reads ready. · r1

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out. · r1 · contradict
decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path. · r1
decision/e2e-home-headroom-floor · Headroom floor of the e2e home base — A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom. · why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. · guard: harness::tests::e2e_home_* · r1
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps. · r1
decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home. · r1
decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir. · r1
decision/proptest-committed-seeds · proptest with committed seeds — Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds. · r1
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper. · r1
decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home. · r1

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/doctests-via-cargo-test · Doctests run through cargo test — Doctests run through cargo test --workspace --doc. · why: nextest cannot run doctests · r1
decision/identity-floor-tested-against-literal-copy · Identity floor tested against a literal copy — The R8 strip is tested against a literal copy of the identity floor, never an imported one. · r1
decision/insta-snapshots-for-decision-bodies · Insta snapshots for decision bodies — Decision bodies are pinned by insta snapshots of the body text. · r1
decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary. · r1
decision/mocks-only-at-product-seams · Mocks only at product seams — Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock. · r1
decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them. · r1
decision/no-assertion-beyond-compiled-literals · No assertion beyond compiled literals — viola-agent-claude unit tests never assert agent content beyond the compiled literals. · r1
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases. · r1
decision/pty-exit-from-wait-not-eof · PTY exit reported from wait, never from reader EOF — viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status. · r1
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails. · r1
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures. · r1
contract/mcp-stdin/line-cap · MCP stdin · line cap — A line of exactly MAX_FRAME bytes, its newline counted, passes whole; one byte more passes no byte, reads as end of stream, and serve ends the session. · r1
contract/mcp-tool-arguments/invalid-argument · MCP tool arguments · invalid argument — A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload. · r1
contract/mcp-tools-list/tools-list · MCP tools/list · tools/list — tools/list is exactly send, wait, last, answer, list. · r1
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null. · r1
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault. · r1
contract/reply-screen/refusal · the reply screen · refusal — A refusal that is no code word reads unknown. · r1
contract/viola-mcp-exit/oversize-before-initialize · viola mcp exit · oversize line before initialize — A stdin line past MAX_FRAME before initialize completes makes viola mcp exit 1 with both streams empty and one parse-rejected mcp-stdin / oversize line of count 1. · r1

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/agents-parser-fails-to-unknown · Agents list parse failure reads unknown — An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name. · r1
decision/answer-refusal-order · answer refusal order — answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog. · r1
decision/bounded-compare · Bounded compare of a shared target — Comparing a target to known bytes reads no more than one byte past their length, never the whole file. · guard: replace_private_shared_reads_no_more_than_one_byte_past_the_bytes · r1
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The status line shell-out, the version gate drain, the hyprctl reader and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers · r2 · stale
decision/budget-decision-takes-the-instant · Budget decision takes the instant as an argument — The budget decision is a pure function that takes the wall-clock instant as an argument, so its thresholds and override expiry are unit-tested with no clock seam and no mocked instant. · guard: gate_decide_* · r1
decision/budget-governor-thresholds · Budget governor thresholds — Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override instant has passed. · guard: gate_decide_an_expired_override_is_dropped · r1
decision/claude-env-strip · Agent identity environment stripped — Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names. · guard: an R8 test against a literal copy of the floor, never imported from the product · r1
decision/clear-confirmed-by-new-session · /clear confirmed by a new session — /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor. · guard: …_clear_is_confirmed_by_a_new_session · r1
decision/compact-confirmed-by-same-session · /compact confirmed by its same session — /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing. · guard: …_compact_is_confirmed_by_its_same_session · r1
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts · r1
decision/diagnostics-checked-before-narrowing · Diagnostics path checked before narrowing — A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped. · guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it · r1
decision/dialog-answer-mapping · Dialog answer mapping — Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny. · r1
decision/dialog-hold-expiry · Dialog hold expiry — An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line. · guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line · r1
decision/dialog-hold-read-once · Dialog hold read once at start — The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection. · guard: read_hold_falls_back_to_sixty_with_one_rejection · r1
decision/dialog-timeouts-outlast-the-hold · Dialog timeouts outlast the longest hold — The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold. · guard: hooks_json_dialog_timeouts_outlast_the_longest_hold · r1
decision/empty-text-refused-before-the-wheel · Empty text refused before the wheel is read — An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read. · guard: send_empty_text_is_refused_first_after_control_character · r1
decision/hook-dialog-no-frame-to-impostor · Hook writes no frame to an impostor server — The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper. · guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper · r1
decision/hook-unknown-fields-tolerated · Unknown hook payload fields tolerated — Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry. · r1
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab. · r1
decision/list-row-control-escaping · list rows show every control character as hex — In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text. · guard: escape_row_shows_every_control_character_as_hex_text · r1
decision/local-command-unconfirmable · Local command without a post-condition is unconfirmable — A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window. · guard: …_without_a_measured_post_condition_is_unconfirmable · r1
decision/mcp-stdin-line-cap · MCP stdin line cap — The MCP server's stdin yields whole lines of at most MAX_FRAME bytes; a longer line releases no byte, sets the cut flag and reads as end of stream, ending a session before initialize as the initialize error. · guard: capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more · r1
decision/one-global-tracing-install · One global tracing install — The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process. · r1
decision/pending-dialog-before-its-line · Pending dialog on the snapshot before its line — A held dialog is on the snapshot before its line is logged, and is cleared when the append fails. · guard: hook_dialog_append_that_failed_clears_the_pending_dialog · r1
decision/private-placement-mode-first · Private copies placed mode-first — A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name. · guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp · r1
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner. · r1
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted. · r1
decision/send-confirmation-exact-match · Send confirmation by exact normalised match — A send is confirmed only when the submitted prompt text equals the typed text, every CR LF and lone CR read as one LF and the trailing CR and LF removed; a submitted prompt that still holds a CR or the trailing newline claims nothing. · guard: send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing · r1
decision/send-confirmation-window-expiry · Send confirmation window expiry — A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted. · r1
decision/send-refusal-order · send refusal order — send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed. · r1
decision/server-checked-before-any-frame · Server checked before any frame — Every client verb and the hook refuse an unverified server inside their own span with no frame written. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span · r1
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows · r1
decision/session-end-direct-append · SessionEnd falls back to a direct append — A SessionEnd hook whose server check fails appends the session end directly. · r1
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span · r1
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit · r1
decision/stamps-refuse-widened-ledger · Stamps refuse a widened ledger — Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing. · guard: update_stamps_refuses_a_widened_ledger_and_leaves_it · r1
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit · r1
decision/unknown-refusal-word-stays-visible · Unknown refusal word stays visible — A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit. · guard: reply_of_reads_the_three_shapes · r1
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows · r1
decision/vt100-feed-panic-safe · Screen feed panic-safe — The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready. · r1
contract/channel-endpoint/socket-path · the wrapper channel endpoint · socket path — The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback. · r1
contract/channel-wait/timed-out · the channel wait method · timed_out — wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process. · r1
contract/channel-wait/wake-kinds · the channel wait method · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`. · r1
contract/cli-arguments/json-flag · CLI arguments · --json — The --json flag is read from the arguments only before a `--` separator. · r1
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter. · r1
contract/dialog-data/null-details · dialog and Stop event data · description, preview and background_tasks — A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array. · r1
contract/e2e-home-refusal/message · the e2e-home headroom refusal · message — e2e-home backing: {free} MiB free, below the {floor} MiB floor · r1
contract/human-hint-cause/not-running · the human hint cause · not-running — The hint cause of an instance-dead unreachable is not-running, a word apart from the log's. · r1
contract/human-hint-cause/socket-dir · the human hint cause · socket-dir — The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused. · r1
contract/json-rpc-errors/-32600 · wrapper channel JSON-RPC errors · -32600 — Invalid request is reported as -32600. · r1
contract/json-rpc-errors/-32601 · wrapper channel JSON-RPC errors · -32601 — Unknown method is reported as -32601. · r1
contract/json-rpc-errors/-32602 · wrapper channel JSON-RPC errors · -32602 — Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart. · r1
contract/json-rpc-errors/-32700 · wrapper channel JSON-RPC errors · -32700 — Parse error is reported as -32700. · r1
contract/json-rpc-errors/wrapper-fault · wrapper channel JSON-RPC errors · a code outside the five — A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code. · r1
contract/link-method/append-failure · the served link methods · failed append — An append that fails answers internal with the link set unchanged. · r1
contract/link-method/bad-pair · the served link methods · bad pair — A bad pair is refused with invalid_params. · r1
contract/link-method/from · the served link methods · from — A from on a link or unlink changes nothing. · r1
contract/link-method/link-twice · the served link methods · link of a linked pair — Linking a pair twice writes one line and answers the same since. · r1
contract/link-method/unlink-absent-pair · the served link methods · unlink of an absent pair — Unlinking an absent pair writes nothing. · r1
contract/link-method/unlink-then-link · the served link methods · unlink then link — An unlink then a link takes a later since. · r1
contract/link-verb-line/since · the link verb line · since — The line's time is the reply's since as UTC clock time; a since that is no RFC 3339 instant prints escaped. · r1
contract/mcp-reply/screened-words · MCP tool replies · refusal and detail words — A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault. · r1
contract/prompt-submitted/origin-harness · the prompt-submitted event · origin harness — A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human. · r1
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on. · r2
contract/unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word. · r1
contract/unreachable-log-cause/endpoint-lost · the unreachable log cause · endpoint-lost — Logged during call. · r1
contract/unreachable-log-cause/instance-dead · the unreachable log cause · instance-dead — Logged during connect. · r1
contract/unreachable-log-cause/server-verify-failed · the unreachable log cause · server-verify-failed — Logged during connect. · r1
contract/unreachable-log-cause/socket-dir-refused · the unreachable log cause · socket-dir-refused — Logged during connect when the socket directory is refused. · r1
contract/unreachable-log-cause/unwrapped-name · the unreachable log cause · unwrapped-name — Logged during connect. · r1

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/board-on-the-read-rule · Board held to the read rule — viola list refuses a home another user may read, with and without an instance. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance · r1
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts. · r1
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table · r1
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir. · r2 · contradict
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans. · r1
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted. · r1
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests · r1
decision/ipc-tested-on-real-endpoint · IPC tested on a real endpoint — IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back. · r1
decision/mcp-handlers-use-tokio-client · MCP handlers use the Tokio client — MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper. · r1
decision/module-integration-direct-calls · Module integration by direct calls — Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures. · r1
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object. · r1
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it. · r2
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem. · r1
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty. · r1
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path. · r1
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly. · r1
contract/cli-argv/leading-slash · CLI argv · leading slash — A leading-slash argument is never delivered as a prompt. · r1
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb. · r1
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line. · r1
contract/event-line/ts · the event line · ts — ts is RFC 3339 UTC with ms and Z. · r1
contract/event-tailing/ignored-entries · event tailing · symlinks and invalid names — Tailing ignores symlinks and invalid names. · r1
contract/mcp-frame-tools/socket-dir-refused · MCP frame tools · socket-dir-refused — A loosened socket directory gives isError with socket-dir-refused, no frame and no line. · r1
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair. · r1
contract/strict-modes-refusal/hook · strict-modes refusal · hook — A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append. · r1
contract/strict-modes-refusal/mcp-frame-tools · strict-modes refusal · MCP frame tools — A strict-modes refusal in the MCP frame tools gives instance-unreachable. · r1
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line. · r1
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1. · r1
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr. · r1
contract/viola-hook/no-viola-name · viola hook · no VIOLA_NAME — With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created. · r1
contract/viola-hook/viola-dir · viola hook · VIOLA_DIR — A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append. · r1
contract/wrapper-channel/-32600 · the wrapper channel · -32600 — An oversize frame is answered with -32600 and the connection closed. · r1
contract/wrapper-channel/-32602 · the wrapper channel · -32602 — A request with a higher v is too new and is answered -32602 with data {supported, wrapper}. · r1
contract/wrapper-channel/control-character · the wrapper channel · control-character — In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal. · r1
contract/wrapper-channel/cursor · the wrapper channel · cursor — A result's cursor equals the events.ndjson length at acceptance. · r1
absence/database — A database · r1
absence/outbound-http — Outbound HTTP · r1

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/bad-config-value-falls-back · Bad config value falls back to its default — A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it. · guard: setting_does_not_disable_the_budget_gate · r1
decision/channel-client-no-dir-check · Channel client carries no directory check — The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check. · why: viola-channel has no viola-state edge. · guard: channel_link_names_are_validated_by_the_wrapper · r1
decision/channel-tests-real-endpoint · Channel tests on a real endpoint — Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas. · r1
decision/concurrent-append-whole-lines · Whole lines under concurrent append — Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · guard: hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side · r1
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home. · r1
decision/help-no-bold-underline · Help carries no bold or underline — viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF. · guard: cli_output_plain · r1
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run · r1
decision/home-resolution-order · Home resolution order — The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists. · why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening. · guard: home_resolution_takes_the_flag_then_the_session_then_the_default · r1
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing · r1
decision/hook-line-cases-in-made-instance-dir · Hook line cases run in a viola-made instance dir — Hook test cases that expect lines run in a viola-made instance directory. · why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing. · r1
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json. · r1
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself. · r1
decision/instance-check-before-role-file · Instance check before any role file or frame — viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found. · why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home. · guard: link_verbs_refuse_a_widened_instance_before_any_log · r1
decision/key-after-resize-waits-for-size · Key after a resize waits for the new size — A test that writes a key after a resize sends it only once the child reports the new size. · guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code · r1
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes · r1
decision/link-since-kept · Link keeps its first since — A linked pair keeps its first since until an unlink, and takes a later one after it. · r1
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest. · r1
decision/no-cli-snapshot-crates · No CLI snapshot crates — No manifest holds assert_cmd, predicates, trycmd or snapbox, and there is no command fixture directory. · r1
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain · r1
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names · r1
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database. · r1
decision/owner-only-socket-modes · Owner-only socket modes — The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind. · r1
decision/paste-validated-at-both-ends · Sent text validated at client and wrapper — Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper. · guard: channel_paste_validation · r1
decision/placed-copy-execs-at-once · Placed copy closed before it is persisted — A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement. · why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY). · guard: place_private_new_placed_copy_execs_at_once · r1
decision/plugin-install-owner-only · Plugin install writes owner-only — viola plugin install writes its tree at 0700 / 0600 whatever the umask, and refuses a widened home with exit 21, leaving the home as found. · guard: plugin_install_writes_owner_only_modes · r1
decision/plugin-marketplace-mcp-only · Plugin marketplace is MCP-only — viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere. · guard: plugin_install_writes_the_mcp_only_marketplace · r1
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check. · r1
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam. · r1
decision/readiness-stall-report · Readiness stall report — When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing. · why: So the report reaches the JUnit. · r1
decision/resize-baseline-is-spawn-size · Pump resize baseline is the spawn size — The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child. · why: A resize can land between the spawn sizing and the pump's first look. · guard: pump_forwards_a_resize_that_lands_before_its_first_look · r1
decision/server-identity-verified · Server identity verified before any frame — A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed. · guard: security_negatives_server_identity · r1
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged. · r1
decision/socket-dir-check · Socket directory check before use — The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods. · guard: security_negatives_socket_dir · r1
decision/squatter-via-server-api · Squatter listener through the server API — A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite. · r1
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path. · r1
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line. · r1
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code. · r1
decision/verify-takes-no-json · viola verify takes no --json — viola verify takes no --json flag. · r1
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log · r1
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Wrappers started in a test are held in a Drop guard that kills the child, then waits for it. · r1
contract/channel-log/link-existing-pair · Wrapper channel log · link of an existing pair — Linking an already linked pair adds no log line. · r1
contract/channel-log/unserved-method · Wrapper channel log · response line of an unserved method — The conn-less response line of an unserved method carries no method key; method names only the closed method list. · r1
contract/channel-response-detail/invalid-params · channel-response line detail · invalid-params — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line. · r1
contract/channel-response-detail/release-from-driver · channel-response line detail · release-from-driver — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line. · r1
contract/channel-response-detail/unsupported-version · channel-response line detail · unsupported-version — One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line. · r1
contract/cli-exit-codes/13-text-refused · CLI exit codes · 13 for refused text — viola send and viola answer with a control character or an empty text exit 13 before any frame is sent. · r1
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21. · r1
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document. · r1
contract/event-data/dialog-expired · Event data keys · dialog-expired — data holds the keys dialog_id and dialog_kind. · r1
contract/event-data/link · Event data keys · link — data holds the keys driver and driven. · r1
contract/event-data/question · Event data keys · question — option_details is a key of each entry in the questions array of a question line's data; the data keys themselves are dialog_id and questions. · r1
contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message and background_tasks. · r1
contract/event-data/unlink · Event data keys · unlink — data holds the keys driver and driven. · r1
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader. · r1
contract/mcp-frame-tools/free-text-refusal · MCP frame tools · refusal of free text — A wrapper refusal of free text reads as unknown, with no detail. · r1
contract/mcp-frame-tools/refusal-result · MCP frame tools · refusal result — A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable. · r1
contract/mcp-unreachable-detail/endpoint-lost · MCP frame tools instance-unreachable detail · endpoint-lost — A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost. · r1
contract/mcp-unreachable-detail/instance-dead · MCP frame tools instance-unreachable detail · instance-dead — A failed connect gives detail instance-dead, with no frame and no log line. · r1
contract/mcp-unreachable-detail/server-verify-failed · MCP frame tools instance-unreachable detail · server-verify-failed — A peer euid the admission refuses, or a failed server verification, gives detail server-verify-failed, with no frame and no log line. · r1
contract/mcp-unreachable-detail/socket-dir-refused · MCP frame tools instance-unreachable detail · socket-dir-refused — A refused socket directory check after the live endpoint is read gives detail socket-dir-refused, with no frame and no log line. · r1
contract/mcp-unreachable-detail/strict-modes-failed · MCP frame tools instance-unreachable detail · strict-modes-failed — A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found. · r1
contract/strict-modes-refusal/mcp · Strict-modes refusal · viola mcp — Exit 1. · r1
contract/strict-modes-refusal/other-verbs · Strict-modes refusal · every other CLI verb — Exit 21. · r1
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found. · r1
contract/strict-modes-refusal/run · Strict-modes refusal · viola run — Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child. · r1
contract/strict-modes-refusal/verify · Strict-modes refusal · viola verify — Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged. · r1
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal. · r1
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered. · r1
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found. · r1
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses. · r1
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code. · r1
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read. · r1
contract/verify-output/step-line · viola verify output · step line — One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count. · r1
contract/verify-output/summary-line · viola verify output · summary line — The last stdout line is stamped <ver>  <n> pass  <m> fail. · r1
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted. · r1
contract/viola-answer/oversize-stdin · viola answer · stdin over MAX_FRAME — Exits 2 before connecting, with the fixed unusable-response line on stderr whatever --json says and no document; the dialog stays pending. · r1
contract/viola-help/verb-table · viola --help · grouped verb table — Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty. · r1
contract/wrapper-channel/accepted-text · Wrapper channel · text with LF, CR, TAB and multi-byte characters — Text with LF, a lone inner CR, TAB and multi-byte characters is accepted and typed as one prompt, its inner CR as one LF. · r1
contract/wrapper-channel/empty-text · Wrapper channel · refusal detail empty-text — An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record. · r1
contract/wrapper-channel/frame-over-max-frame · Wrapper channel · frame over MAX_FRAME — A frame over MAX_FRAME is answered with error -32600 and the connection is closed; a pending dialog stays pending. · r1
contract/wrapper-channel/params-v-higher · Wrapper channel · params.v higher than served — A request whose params.v is higher than the served version is answered with error -32602. · r1

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and Anthropic usage values by generated statusline stdin; tests never reach the real services. · r2 · stale
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test. · r1
decision/mockall-on-seam-traits-only · mockall doubles on seam traits only — Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables. · r1
decision/no-http-fake-servers · no HTTP fake servers — Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used. · r1
decision/no-http-mocking · no HTTP mock library — No HTTP mock library is used. · why: viola makes no outbound HTTP calls. · r1
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E. · r1
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked. · r1

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir. · r1
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list. · r1
decision/hook-unit-impostor-varies-start-time · Hook unit impostor varies the start time — The hook's unit impostor varies the recorded start time, a second early or an unparseable started_at, instead of using a second process. · why: A root-bin unit test has no portable second process. · r1
decision/impostor-holder-real-process · Impostor's held process is real — A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard. · r1
decision/impostor-server-real-listener · Impostor server is a real listener — Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process. · r1
decision/peer-admission-by-injection · Peer admission tested by injection — The peer euid decision takes its euids as arguments and the private serve function takes its admission as an argument, with no env, flag or feature seam. · r1
decision/peer-euid-admission · Peer admitted only on the owning user's euid — A channel peer is admitted only when its observed euid equals the server's own; an unreadable euid is refused, and a refused peer is dropped before any frame, with no line written. · guard: serve_drops_a_refused_peer_before_any_frame, peer_euid_admitted_takes_only_the_owning_user · r1
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier. · r1
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either. · r1
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument. · r1
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision such as the budget gate's takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only. · r1
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper. · r1

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values. · r1
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically. · r1
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF. · r1
decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs. · r1
decision/coverage-ignore-regex-not-widened · Coverage ignore regex is never widened — The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first. · why: Widening it is a back door to lowering the coverage thresholds. · except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged. · r1
decision/coverage-threshold-never-lowered · Coverage threshold never lowered to pass — The coverage threshold is never lowered to pass a build. · why: Raise quality, not the bar. · r1
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid. · r1
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`. · r1
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage. · r1
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal. · r1
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI. · r1
decision/harness-output-json-and-exit-code · Harness commands always give JSON and exit code — Every harness command prints its JSON document and exit code, never a human-only message alone. · r1
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test. · r1
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog. · r1
decision/no-committed-test-secrets · No test dirs, env files or tokens committed — `--home` test directories, `.env` files and tokens are never committed. · r1
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code. · r1
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget. · r1
decision/no-human-reviewed-visual-regression · No human-reviewed visual regression — Percy, Chromatic, Applitools and other visual regression with human review are never used. · r1
decision/no-human-screenshot-review · No human screenshot review — No test step relies on a human reviewing screenshots. · r1
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run. · r1
decision/no-interactive-snapshot-review · No interactive snapshot review as a gate — Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed. · r1
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline. · r1
decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer. · r1
decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration. · r1
decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour. · r1
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads. · r1
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept. · r2
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set · r1
decision/no-real-network-in-tests · No real network calls in tests — Tests never make real network calls; they use mocks or localhost stubs. · r1
decision/no-recorder-authored-tests · No recorder-authored tests — Tests are never authored by Selenium IDE or the Playwright codegen recorder. · r1
decision/no-runtime-monkey-patching · No runtime monkey-patching — Production code is never monkey-patched at runtime; dependency injection is used instead. · r1
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt. · r1
decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state. · r1
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once. · r1
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field. · r1
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case. · r1
decision/no-test-passing-unstamped-decision-body · No test passing an unguarded decision body — No test passes when a decision body is emitted without a verify stamp and the wheel held by driver. · r1
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults. · r1
decision/proptest-seeds-committed · Property tests replay from committed seeds — Property tests replay deterministically: proptest regression seeds persist in committed regression files. · r1
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote. · r1
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule. · r1
decision/rust-cache-only-in-ci · rust-cache is the only cache, CI only — Swatinem/rust-cache is the only cache in `ci.yml`, and no release workflow uses it. · r1
decision/seeded-generators · Seed-controlled generators — Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed. · r1
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes. · r1
decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state. · r1 · stale
decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design. · r1
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits. · r1
