# security-plan · the rows that bind 2026-11-14-budget-gate-removed — picked 187 of 618 (110 by every draw) · rule any · pin 1142

## threat-model-summary · ## Threat Model Summary
decision/cli-input-trust-boundary · CLI input trust boundary — CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/from-field-authenticates-nothing · Self-reported from authenticates nothing — The self-reported from field on the wrapper channel authenticates nothing and is never treated as an identity.
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/hook-payload-is-content · Hook payload is content — Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
decision/hook-strict-check-first · Hook strict-modes check before any read — Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
decision/list-home-read-rule · Board scan refuses a readable home — viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
decision/mcp-frame-cap · MCP stdin line cap — Each MCP stdin line is capped at MAX_FRAME before rmcp reads it; an over-long line is refused whole and ends the session, never dispatched in part.
decision/mcp-tool-list-affordance · MCP tool list is an affordance — Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
decision/mcp-tool-list-not-enforcement · MCP tool list is an affordance — Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/replayed-snapshot-yields-nothing · Replayed snapshot runs nothing and frames nothing — A snapshot replayed from the log carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command.
decision/send-text-is-one-bracketed-paste · Send text typed as one bracketed paste — Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
decision/tolerant-hook-parsing · Tolerant hook payload parsing — Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/cli-exit-codes/21 · CLI exit codes · 21 — A frame verb refused because the instance failed the strict-modes check.
contract/mcp-tools/list · MCP tools · list — A tool called by the driver model that takes no input, reading the board.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wrapper-channel-methods/release · wrapper channel methods · release — A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session.
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent. · why: an absolute host path can carry the OS username
decision/statusline-command-integrity · Statusline command integrity — The hook statusline runs statusline_command through a shell, its integrity resting on filesystem permissions of the viola home.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
contract/wrapper-channel-methods/pause · wrapper channel methods · pause — A method of the wrapper channel, JSON-RPC 2.0 over ndjson.

## authentication-authorization · ## Authentication & Authorization
decision/board-home-read-rule · The board holds the home to the read rule — viola list refuses a home another user may read, with or without an instance.
decision/budget-gate-fails-open · Budget gate reads an untrusted file, failing open — The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file; a refused or unreadable file is no reading and the gate does not pause. · why: a ratified boundary widening: a file a hook arm writes decides a wrapper's refusal
decision/driver-release-refused · Driver-originated release refused — The wrapper refuses a release whose params carry a string from, before any budget is acted on. · why: An affordance-level guard against drivers handing the wheel back to automation, not enforcement, since a same-user process can leave out from.
decision/home-write-rule-checkers · Budget read, mcp start and plugin install check the home — The budget read, the start of viola mcp and viola plugin install hold the home to the write rule.
decision/release-from-driver-guard · Driver-originated release is refused — The wrapper refuses a release whose params carry a string from, before budget is acted on. · why: an affordance-level guard against drivers handing the wheel back to automation, not enforcement: a same-user process can leave out from · except: pause, link and unlink from drivers are allowed
decision/start-checks-home-tree-before-log-opens · Start checks the home tree before its log opens — Run and revive's start check the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child. · why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/whole-board-refusal · viola list refuses the whole board — viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board. · why: ratified
contract/channel-release/non-string-from · channel release method · from of another type — Refused with -32602 and data null.
contract/channel-release/params · channel release method · params — release params are {budget?, from?}; a release on a wheel the driver already holds changes nothing.
contract/channel-release/string-from · channel release method · string from — Refused with -32602 "invalid params" and data {"reason":"release-from-driver"}, one release-from-driver line logged (from_trust:"self-reported"); a budget pause is not lifted and the snapshot's budget fields stay as they were.
contract/channel-response-line/release-from-driver · channel-response log line · release-from-driver — The server's and client's line is WARN with detail release-from-driver and no param name or value; a from of another type reads invalid-params; the line adds nothing to the wire body.
contract/cli-release/from · CLI viola release · from — from is sent from VIOLA_NAME, so viola release run from a driver session exits 20.
contract/release-errors/release-from-driver · The release method's errors · release-from-driver — A release carrying a string from is refused with code -32602 and reason release-from-driver.
contract/strict-modes-refusal/hook · strict-modes refusal · viola hook — Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
contract/strict-modes-refusal/list-human · strict-modes refusal · viola list in human form — Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
contract/strict-modes-refusal/list-json · strict-modes refusal · viola list under --json — Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
contract/strict-modes-refusal/mcp-start · strict-modes refusal · viola mcp start — Exit 1 with nothing on stdout or stderr and no role file.
contract/strict-modes-refusal/plugin-install · strict-modes refusal · viola plugin install — Exit 21 (instance-unreachable under --json) with nothing created and no log opened.
contract/strict-modes-refusal/run-revive · strict-modes refusal · run and revive — Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.
contract/strict-refusal-message/readable · strict-modes refusal message · Readable — The read rule's refusal reads exactly "a trusted state path is readable by another user".
decision/all-or-nothing-channel-access · Capability-based, all-or-nothing channel access — Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles. · why: the only principal is the local OS user
decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
contract/server-verify-refusal/hook · server-verification refusal · viola hook — Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
contract/socket-dir-refusal/hook · socket-directory refusal · viola hook — Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
contract/strict-refusal-message/symlink · strict-modes refusal message · Symlink — A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/budget-file-single-writer · hook statusline is the only budget writer — budget.json is written by hook statusline alone.
decision/budget-json-single-writer · budget.json has a single writer — budget.json is written only by the statusline hook arm, and only when the payload holds a rate_limits object.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/config-bad-value-handling · Bad config values — Configuration is parsed tolerantly under MAX_FRAME with its version checked; a bad value takes its key's default or is rejected whole, and is reported as parse-rejected.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value switches the budget gate off or makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/config-tolerant-parse · Tolerant config parse — config.json is parsed tolerantly, unknown keys counted, its v checked and its read capped at MAX_FRAME; a bad value falls back to its default and is reported as parse-rejected.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/failed-budget-read-no-reading · Failed budget read is no reading — Every failed read of the budget file counts as no reading.
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/hook-instance-check-first · Hook instance check before anything — Hook event, dialog and statusline paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
decision/hook-security-fields-closed · Hook security fields in closed enums — Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
decision/hook-stdin-never-truncated · Hook stdin over-limit detected — Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
decision/list-scan-checks-before-open · viola list checks before opening — viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/no-statusline-command-in-logs · Statusline command never logged — No log line or span carries the statusline command.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/paste-validation-on-both-sides · Paste validation on both sides — Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
decision/pause-release-from-viola-name · pause and release take from from VIOLA_NAME — The CLI pause and release verbs take their from only from VIOLA_NAME.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/replayed-snapshot-fallbacks · Replayed snapshot fallbacks — Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, statusline prints nothing, and the collision check judges liveness by the heartbeat alone.
decision/replayed-snapshot-inert · Replayed snapshot is inert — A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid, start time or statusline command, so no frame is written and no command runs from it.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/statusline-payload-fails-silent · Statusline payload fails silent — The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr, nothing printed and no budget file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/budget-json/resets-at · budget.json · resets_at — Read as epoch seconds or RFC 3339 and written as RFC 3339 UTC with milliseconds; a malformed value becomes "unknown".
contract/budget-json/used-percentage · budget.json · used_percentage — Kept only from 0 to 100, else "unknown".
contract/budget-json/window · budget.json · window — A window that is absent or not an object is the word "unknown".
contract/channel-errors/32602 · channel error codes · -32602 — A method whose params fail the wrapper's check is answered -32602.
contract/channel-frames/release-params · channel frames · release params — budget is absent or a bool, else -32602 with data null; from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null.
contract/config-json/five-hour · config.json · five_hour — A JSON whole number 0 to 100, default 90; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
contract/config-json/seven-day · config.json · seven_day — A JSON whole number 0 to 100, default 85; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/channel-frame-cap-before-read · Channel frame cap before reading — A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/claude-agents-resolved-from-path · claude agents resolved from the caller's PATH — list, revive's preflight and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME. · why: Same-user trust, an accepted risk; the MCP widening ratified.
decision/empty-typed-text-refused · Empty typed text refused — A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed. · why: Ratified.
decision/frame-verbs-instance-check-first · Frame verbs check the instance first — CLI frame verbs and link/unlink run the instance strict-modes check first; a refusal is exit 21 with the instance pair, nothing opened under the home and no line.
decision/mcp-handlers-check-raw-args · MCP handlers check raw arguments — Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only. · why: rmcp does not enforce the schemars schemas
decision/mcp-handlers-validate-raw-arguments · MCP handlers validate raw arguments — MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation. · why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
decision/mcp-stdin-line-cap · MCP stdin lines capped at the frame limit — Each MCP stdin line is capped at MAX_FRAME, its newline counted; an over-long line is refused whole, never truncated or skipped, and ends the session as a stdin close does, its warning holding no byte of it. · why: rmcp reads stdin lines unbounded, so viola hands it a capped reader
decision/mcp-stdin-line-cap-on-reader · MCP stdin line cap on the reader — Each MCP stdin line is capped at MAX_FRAME, newline counted, by the reader handed to the SDK; a longer line is refused whole, never truncated or skipped, and ends the session as a stdin close does. · why: rmcp's stdio transport reads each line unbounded and its codec length bound covers only the writer.
decision/revive-strict-modes-before-use · revive checks strict modes before use — Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
decision/single-shell-out · statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.
decision/statusline-command-trust · Statusline command trusted via strict modes — statusline_command is trusted only because of the home strict-modes check. · why: Write access to it equals code execution.
decision/statusline-deadline · Statusline command deadline — The wait for the statusline command is bounded at 5 s (detail deadline); a failing command prints nothing. · provisional
decision/statusline-shell-out · Statusline shell-out — The recorded statusline command, the only shell-out, runs only in hook statusline as /bin/sh -c with the command as one argument; stderr is discarded, stdout capped and printed byte for byte, never parsed, and no log line carries the command. · why: A ratified boundary widening.
decision/statusline-source-read-only · Statusline source read only — A statusline command is taken only from a non-empty statusLine of type command with no NUL, read once and capped; viola writes neither source.
decision/unwrapped-session-name-not-target · Unwrapped session name never a target — An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
decision/wrapper-checks-authoritatively · Wrapper re-checks text authoritatively — Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
contract/channel-errors/32600 · channel error codes · -32600 — A frame over the MAX_FRAME cap is answered -32600 and the connection is closed.
contract/channel-response-log/detail · channel-response log line · detail — The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
contract/hook-stdin/not-one-object · hook stdin · malformed-json — A payload that is not one JSON object fails open with malformed-json.
contract/hook-stdin/oversize · hook stdin · oversize payload — A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
contract/send-refusal/control-character · send refusal · control-character — A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
contract/send-refusal/empty-text · send refusal · empty-text — A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.

## data-protection · ## Data Protection
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives. · except: A workspace.repo naming this repository's own remote keeps its owner handle, neither rewritten nor refused.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.

## dependency-security · ## Dependency Security
decision/g2-single-panic-exemption · G2 zero-panics gate has one exemption — CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry. · guard: G2 --probe run before every G2 check
decision/g2-zero-panics-single-exemption · Zero-panics gate with one exemption — CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry. · guard: G2 --probe · except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arms never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/reply-code-words-validated · Reply code words validated — A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply. · except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
contract/channel-errors/-32602 · channel JSON-RPC errors · -32602 — A release call carrying from gets the JSON-RPC error -32602.
contract/channel-response-detail/invalid-params · channel-response detail · invalid-params — The closed detail word for the invalid-params -32602 fault, written on the server's and the client's line.
contract/channel-response-detail/release-from-driver · channel-response detail · release-from-driver — The closed detail word for the release-from-driver -32602 fault, written on the server's and the client's line.
contract/hook-decision-detail/malformed-json · hook-decision detail · malformed-json — hook fails open with this detail when stdin is not one JSON object.
contract/hook-decision-detail/oversize-stdin · hook-decision detail · oversize-stdin — hook fails open with this detail when stdin exceeds its cap.
contract/mcp-iserror/refusal · MCP isError structuredContent · refusal — A refusal that is no code word reads unknown, alike on MCP and CLI --json.
decision/channel-faults-sanitised · Channel faults sanitised by the clients — Clients read a wrapper fault only with an integer code; each known code reads its fixed message with data null. · except: Two -32602 bodies keep data: the unsupported-version body with an integer data.supported and a 1-64 character data.wrapper, and the body whose data.reason is release-from-driver.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The two capture arms run ahead of that check.
contract/channel-fault/unknown-code · wrapper faults as read by clients · a code outside the five — Reads {"code": <code>, "message": "wrapper fault", "data": null}; `wrapper fault` is a fixed word on --json and MCP, and the human line reads `error: wrapper fault  <code>`.
contract/channel-refusal/control-character · channel result.refusal · not-delivered/control-character — A text holding a refused control character is answered with result.refusal not-delivered/control-character.
contract/channel-refusal/empty-text · channel result.refusal · not-delivered/empty-text — A text whose typed text is empty is answered with result.refusal not-delivered/empty-text.
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
contract/mcp-iserror/text · MCP isError text · refused text — Every isError text reads refused: <code>.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/channel-from-not-identity · Channel from is not an identity — The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/request-size-limits · Request size limits — MAX_FRAME is enforced on the channel and on each viola mcp stdin line; request size limits are never skipped.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes. · why: a stamp is what switches on dialog answering
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames. · except: A workspace.repo naming this repository's own remote is kept.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/statusline-only-from-strict-home · statusline command only from a strict home — statusline_command is never run from a snapshot whose home or instance directory fails the strict-modes check. · why: Write access there equals code execution.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.

# security-plan · its absences, names and overviews, whole

## security-plan-viola · (above the first heading)
overview/security-plan-viola — viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract, with no public network surface. Its IPC, filesystem and PTY controls stand in for the auth, database and API concerns of a web stack.

## threat-model-summary · ## Threat Model Summary
absence/hosting — Hosting or containerised deployment of any kind
absence/model-api-calls — Model API calls of its own; the subscription credentials belong to the unmodified claude CLI
absence/outbound-network-calls — Outbound network calls made by viola itself
absence/public-network-listener — A public network listener or public internet API
absence/regulated-personal-data — Payment, health, children's or multi-user personal data of any kind
absence/release-and-deploy — A release, signing or deploy stage with release artefacts
absence/release-stage — A release, signing or deploy stage in v1
absence/remote-access — Remote or phone access to viola
absence/secrets — Secrets of any kind in v1
absence/third-party-personal-data — Collection of personal data from third parties
absence/websocket — WebSocket or polling endpoints

## authentication-authorization · ## Authentication & Authorization
absence/accounts-and-passwords — User accounts and passwords
absence/user-accounts-passwords — User accounts with passwords

## input-validation · ## Input Validation
absence/database — A database or query language
absence/file-upload — A file upload surface
absence/webhook — A webhook surface
overview/input-validation — Viola validates untrusted data at every point where it enters. Two validation regimes apply: tolerant parsing for external payloads and versioned-tolerant parsing for viola's own formats, with the security floors holding in both. Shared validators live in one core crate, so the CLI, the MCP server and the wrapper all call the same function.

## data-protection · ## Data Protection
absence/key-management — Keys at rest and their management

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
absence/stored-secrets — Stored secrets in v1

## secret-management · ## Secret Management
absence/secret-manager — A secret manager, and so any secret access auditing

## error-handling · ## Error Handling
absence/external-error-reporting — An external error reporting integration

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
absence/user-accounts — User accounts or login endpoints
