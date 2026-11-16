# security-plan · the rows that bind 2026-11-16-granular-api-error-cause — picked 93 of 603 (57 by every draw) · rule any · pin 2030

## threat-model-summary · ## Threat Model Summary
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/hook-payload-is-content · Hook payload is content — Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/tolerant-hook-parsing · Tolerant hook payload parsing — Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/mcp-tools/wait · MCP tools · wait — A tool called by the driver model that reads the driven session's output.
contract/wrapper-channel-methods/hook-event · wrapper channel methods · hook.event — A notification from the hook to the wrapper; for a SessionStart it may carry the optional transcript param beside the event, which the wrapper re-checks for a session-start line only and drops otherwise.
contract/wrapper-channel-methods/wait · wrapper channel methods · wait — Reads the driven session's conversation.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
decision/hook-strict-check-first · Hook strict-modes check before any read — Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.

## authentication-authorization · ## Authentication & Authorization
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version. · except: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/hook-security-fields-closed · Hook security fields in closed enums — Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
decision/invalid-hook-frames-dropped · Invalid hook frames dropped — An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate, and the Claude transcript path shape check in the Claude adapter crate, which the hook and the wrapper both call.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/channel-frames/hook-event-revalidation · channel frames · hook.event — kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; a turn-ended cause, when present, is a string in the closed turn-end cause set (api-error alone); an invalid event is not appended.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/hook-security-fields-closed-enums · Hook security fields in closed enums — Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/refused-text-never-stripped · Refused text is refused whole — A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
contract/channel-errors/32602 · channel error codes · -32602 — A method whose params fail the wrapper's check is answered -32602.
contract/hook-stdin/not-one-object · hook stdin · malformed-json — A payload that is not one JSON object fails open with malformed-json.
contract/hook-stdin/oversize · hook stdin · oversize payload — A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.

## data-protection · ## Data Protection
decision/api-error-classes-adapter-only · API-error classes kept inside the Claude adapter — The CLI's StopFailure error word is classified into nine closed API-error classes inside the Claude adapter crate only; no class is written to event data, an event line, the snapshot or a log line. · provisional
decision/api-error-turn-end-text-dropped · API-error turn end carries no error text — The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and the closed cause word api-error, which crosses into events.ndjson, wait --json and MCP wait. · why: A boundary widening held to one closed word, so the upstream error text crosses into nothing. · guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot · provisional
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/probe-prompts-synthetic · Probe prompts are synthetic — Probe prompts used to record committed fixtures must be synthetic.
decision/transcript-path-redacted · Transcript path redacted — The Claude transcript path is a redacted value, carried only as the optional transcript param beside hook.event, and appears in no event data, events log, log line, span, reply or verify record. · guard: session_start_transcript_path_stays_out_of_data

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.

## secret-management · ## Secret Management
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The capture arm runs ahead of that check.
decision/internal-error-held-at-unit-layer · Internal errors tested at the unit layer — No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path. · guard: answer_a_panicking_dispatch_is_internal_error_and_closes
decision/reply-code-words-validated · Reply code words validated — A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply. · except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
contract/cli-json-error/internal-error · CLI --json error documents · internal-error — Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
contract/mcp-errors/-32603 · MCP JSON-RPC errors · -32603 — Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command names come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text; a typed text is a local command only by its compiled name, alone or followed by one space and arguments.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing

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
