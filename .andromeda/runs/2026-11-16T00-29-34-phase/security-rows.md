# security-plan · the rows that bind 2026-11-16-wait-wake-condition — picked 182 of 599 (117 by every draw) · rule any · pin 1668

## threat-model-summary · ## Threat Model Summary
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/hook-payload-is-content · Hook payload is content — Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
decision/mcp-input-checked · MCP input checked by schema — MCP tool input is checked by the tool input schemas and the ViolaName newtype.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/tolerant-hook-parsing · Tolerant hook payload parsing — Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/mcp-tools/wait · MCP tools · wait — A tool called by the driver model that reads the driven session's output.
contract/wrapper-channel-methods/hook-event · wrapper channel methods · hook.event — A notification from the hook to the wrapper.
contract/wrapper-channel-methods/wait · wrapper channel methods · wait — Reads the driven session's conversation.
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/driver-permission-policy-outside-viola · Driver prompt injection accepted — The driver model reads untrusted driven-session output and may be prompt-injected into answering a PermissionRequest; what to answer rests with the driver, outside viola. · why: What to answer is policy outside viola; an accepted risk.
decision/flat-file-state-store · Flat-file state store — State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
contract/internal-error-output/json · catch-site internal error output · --json — Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
contract/mcp-tools/last · MCP tools · last — A tool called by the driver model that reads the driven session's latest reply.
contract/wrapper-channel-methods/last · wrapper channel methods · last — Reads the driven session's latest reply.

## authentication-authorization · ## Authentication & Authorization
decision/explicit-mode-before-write · Mode set explicitly before any byte — A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded.
decision/home-owner-only · Owner-only viola home — The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
decision/private-file-modes · Owner-only modes for the viola home — The home (and any --home) is created 0700 and every file 0600, never group- or world-writable. · except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/all-or-nothing-channel-access · Capability-based, all-or-nothing channel access — Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles. · why: the only principal is the local OS user
decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
contract/server-verify-refusal/hook · server-verification refusal · viola hook — Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
contract/socket-dir-refusal/hook · socket-directory refusal · viola hook — Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
contract/strict-modes-refusal/hook · strict-modes refusal · viola hook — Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
contract/strict-modes-refusal/mcp-tool · strict-modes refusal · viola mcp tool — Returns instance-unreachable with detail strict-modes-failed, no frame and no line.
contract/strict-modes-refusal/verify · strict-modes refusal · viola verify — Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/cli-names-through-name-check · CLI names through the viola name check — Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/hook-security-fields-closed · Hook security fields in closed enums — Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
decision/hook-security-fields-closed-enums · Hook security fields in closed enums — Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
decision/hook-stdin-never-truncated · Hook stdin over-limit detected — Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
decision/hook-unknown-event-silent-exit · Hook unknown event exits silently — The hook event argument maps into a closed set of ten events and no other word; an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing read or written anywhere.
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/mcp-handlers-check-raw-args · MCP handlers check raw arguments — Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only. · why: rmcp does not enforce the schemars schemas
decision/mcp-handlers-validate-raw-arguments · MCP handlers validate raw arguments — MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation. · why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
decision/question-option-text-crosses-boundary · Question option text crosses into events — A question option's description and preview are taken only as strings, else null, never refusing the dialog, and cross into the events log, hook.dialog, CLI wait --json and the MCP wait. · why: A ratified boundary widening.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/stop-background-tasks-count-only · Stop background tasks carried as a count only — Stop's background_tasks is carried only as its length, no element read; null when absent, null or not an array, and a non-array also writes a drift entry. · why: Ratified with the option-text widening.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/channel-errors/32602 · channel error codes · -32602 — A method whose params fail the wrapper's check is answered -32602.
contract/channel-frames/hook-event-revalidation · channel frames · hook.event — kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; a turn-ended cause, when present, is a string in the closed turn-end cause set (api-error alone); an invalid event is not appended.
contract/channel-frames/wait-last-params · channel frames · wait and last params — after and timeout_ms are each absent or a non-negative integer that fits u64, else -32602 invalid params with data null; from follows the send rule; unknown fields are ignored.
contract/channel-response-log/detail · channel-response log line · detail — The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
contract/hook-stdin/not-one-object · hook stdin · malformed-json — A payload that is not one JSON object fails open with malformed-json.
contract/hook-stdin/oversize · hook stdin · oversize payload — A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
contract/mcp-errors/invalid-params · MCP tool errors · invalid-params — A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
contract/mcp-tool-inputs/invalid-params · MCP tool inputs · invalid-params — A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.
contract/mcp-tool-inputs/timeout-ms · MCP tool inputs · timeout_ms — wait's timeout_ms is absent, null or a u64, clamped to 50 000 ms, which is also its default.
decision/channel-frame-cap-before-read · Channel frame cap before reading — A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
decision/channel-json-default-depth-limit · Channel JSON default depth limit — Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
decision/colour-env-only-compared · Terminal colour variables only compared — NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/hook-instance-check-first · Hook instance check before anything — Hook event and dialog paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
decision/invalid-hook-frames-dropped · Invalid hook frames dropped — An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/verify-home-tree-check · verify checks the home tree first — viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
contract/channel-frames/hook-dialog-params · channel frames · hook.dialog params — kind is a closed dialog kind, data an object, hook_event closed, and tool, input and continuation closed or absent; an invalid frame is not registered.

## data-protection · ## Data Protection
decision/api-error-turn-end-text-dropped · API-error turn end carries no error text — The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and the closed cause word api-error, which crosses into events.ndjson, wait --json and MCP wait. · why: A boundary widening held to one closed word, so the upstream error text crosses into nothing. · guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot · provisional
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
decision/api-error-classes-adapter-only · API-error classes kept inside the Claude adapter — The CLI's StopFailure error word is classified into nine closed API-error classes inside the Claude adapter crate only; no class is written to event data, an event line, the snapshot or a log line. · provisional
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file.
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
contract/verify-refusal-codes/strict-modes-failed · verify's refusal codes · strict-modes-failed — One of the closed codes verify writes on its process-exit when refused.

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/r8-claude-env-strip · R8 CLAUDE environment strip — Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values. · why: Ratified.
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.

## error-handling · ## Error Handling
decision/channel-faults-sanitised · Channel faults sanitised by the clients — Clients read a wrapper fault only with an integer code; each known code reads its fixed message with data null. · except: Two -32602 bodies keep data: the unsupported-version body with an integer data.supported and a 1-64 character data.wrapper, and the body whose data.reason is release-from-driver.
decision/cli-frame-verbs-strict-modes-first · CLI frame verbs check the instance first — CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/mcp-frame-tool-refusals · Refused MCP frame tools send nothing — A viola mcp frame tool refused by the strict-modes, liveness, socket-directory or server check sends no frame, writes no log line and returns the instance-unreachable result.
decision/reply-code-words-validated · Reply code words validated — A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply. · except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
contract/channel-fault/unknown-code · wrapper faults as read by clients · a code outside the five — Reads {"code": <code>, "message": "wrapper fault", "data": null}; `wrapper fault` is a fixed word on --json and MCP, and the human line reads `error: wrapper fault  <code>`.
contract/channel-response-detail/invalid-params · channel-response detail · invalid-params — The closed detail word for the invalid-params -32602 fault, written on the server's and the client's line.
contract/cli-instance-unreachable/json-document · CLI exit 21 · --json document — Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
contract/cli-json-error/internal-error · CLI --json error documents · internal-error — Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
contract/instance-unreachable-cause/endpoint-lost · instance-unreachable detail causes · endpoint-lost — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/instance-dead · instance-unreachable detail causes · instance-dead — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/server-verify-failed · instance-unreachable detail causes · server-verify-failed — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/socket-dir-refused · instance-unreachable detail causes · socket-dir-refused — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable detail causes · strict-modes-failed — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/unwrapped-name · instance-unreachable detail causes · unwrapped-name — A closed cause of instance-unreachable.
contract/instance-unreachable-detail/endpoint-lost · instance-unreachable detail · endpoint-lost — A cause of exit 21 in the CLI --json document and MCP's isError result: the wrapper was lost with the call open.
contract/instance-unreachable-detail/server-verify-failed · instance-unreachable detail · server-verify-failed — A cause of exit 21 in the CLI --json document and MCP's isError result: the peer euid decision refused past the connect, and no frame was sent; a peer euid refusal has no word of its own and reads server-verify-failed.
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — A cause of exit 21 in the CLI --json document and MCP's isError result: the socket directory of the recorded endpoint was refused, and no frame was sent; on a CLI frame verb it is during connect, one process-exit at WARN and no channel-request.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — A cause of exit 21 in the CLI --json document and MCP's isError result: the instance's strict-modes check refused, and no frame was sent.
contract/mcp-errors/-32603 · MCP JSON-RPC errors · -32603 — Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.
contract/mcp-iserror/detail · MCP isError structuredContent · detail — A string detail that is no known detail reads null, alike on MCP and CLI --json.
contract/mcp-iserror/instance-unreachable · MCP isError structuredContent · error instance-unreachable — An unreachable instance reads error instance-unreachable with detail one of its closed causes.
contract/mcp-iserror/invalid-params · MCP isError structuredContent · error invalid-params — Invalid parameters read error invalid-params with a closed field word.
contract/mcp-iserror/text · MCP isError text · refused text — Every isError text reads refused: <code>.
contract/mcp-iserror/wrapper-fault · MCP isError structuredContent · error wrapper-fault — A wrapper fault reads error wrapper-fault with a sanitised code, message and data, never the peer's own.
decision/hook-refusals-write-no-frame · A refused hook writes no frame — A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The capture arm runs ahead of that check.
decision/socket-dir-refusal-names-only-name · Socket-directory refusal discloses no path — The socket-directory refusal names only the parsed ViolaName, and its lines hold no path, pid or uid.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
contract/cli-stderr-hints/unwrapped-name-endpoint-lost · CLI stderr hints · unwrapped-name and endpoint-lost hints — The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
contract/hook-decision-detail/malformed-json · hook-decision detail · malformed-json — hook fails open with this detail when stdin is not one JSON object.
contract/hook-decision-detail/oversize-stdin · hook-decision detail · oversize-stdin — hook fails open with this detail when stdin exceeds its cap.
contract/mcp-frame-tool-result/unreachable · viola mcp frame tool results · instance-unreachable — A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.
contract/mcp-iserror/refusal · MCP isError structuredContent · refusal — A refusal that is no code word reads unknown, alike on MCP and CLI --json.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/control-check-on-decoded-chars · Control-character check on decoded characters — The control-character check runs over decoded characters, never raw bytes. · why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/harness-prefix-literals · Harness prefixes as four raw-start literals — A prompt is classified harness only by four compiled literals matched on its raw start with no trim: <agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message. · why: ratified side effect: the escaped cross-session form is the one escaped tag that classifies harness, so a human who types it at a prompt's start is filed harness
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/verify-server-before-frame · Verify the server before any frame — No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
decision/server-verified-before-any-frame · Server verified before any channel frame — No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
decision/stamps-strict-check-first · Stamps touched only after strict-modes check — viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.

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
