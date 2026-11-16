# security-plan · the rows that bind 2026-11-16-send-confirmation — picked 110 of 599 (71 by every draw) · rule any · pin 1768

## threat-model-summary · ## Threat Model Summary
decision/bridge-send-one-bracketed-paste · Bridge send is one bracketed paste — A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/delivery-confirmation-exact-match · Delivery confirmed by exact equality — Delivery confirmation compares the submitted prompt text with the typed text by exact equality, with no tolerance on the submitted side.
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/hook-payload-is-content · Hook payload is content — Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/send-text-is-one-bracketed-paste · Send text typed as one bracketed paste — Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
decision/tolerant-hook-parsing · Tolerant hook payload parsing — Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/mcp-tools/send · MCP tools · send — A tool called by the driver model, input checked by its schema and the ViolaName check.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wheel-event/human-input · wheel event · cause human-input — The wheel moved to holder human because a byte on viola run's stdin counted as typing.
contract/wrapper-channel-methods/hook-event · wrapper channel methods · hook.event — A notification from the hook to the wrapper.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session.
decision/hook-strict-check-first · Hook strict-modes check before any read — Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
decision/human-typing-takes-wheel · Human typing takes the wheel — A byte on viola run's own stdin that is typing takes the wheel from the driver. · except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/stdin-closed-non-editing-list · Stdin classifier's closed non-editing list — Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess. · why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start

## authentication-authorization · ## Authentication & Authorization
decision/all-or-nothing-channel-access · Capability-based, all-or-nothing channel access — Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles. · why: the only principal is the local OS user
decision/checks-at-seams-not-channel-client · Security checks sit at the seams, not the channel client — No check sits inside the channel crate's client connect; the checks sit at the seams, and a test may write through that client past every check by design. · why: viola-channel has no viola-state edge
decision/client-refusals-first · Client's own refusals come first — A CLI frame verb's strict-modes refusal comes first, then its own exit-13 refusals, before its liveness and server checks.
decision/client-seam-check-order · Check order at every client seam — On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
decision/client-strict-modes-first · Client strict-modes check first — Before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line.
decision/client-verifies-server-before-frame · Client verifies the server before any frame — Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal. · why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
decision/live-round-rig-keys-own-pty · Live-round rig keys only into its own pty — On the founder's word for that chunk, a live round's rig types into its own pty at most two down-arrows and one Enter on the folder-trust literal and one x and one DEL in the input box, and reads statusLine.command only by a read-only hash; each round needs his word anew; viola types into no dialog. · why: A boundary widening bounded as the founder bounded it; a rig is no viola, so the product's own pty seam stays unchanged.
decision/socket-dir-check-never-chmods · Socket-directory check never chmods — The socket-directory check never changes a directory's mode: a refused directory stays as found. · guard: security_negatives_socket_dir
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/cli-input-checked-before-frame · CLI input checked before any frame — answer and send input is read under MAX_FRAME and parsed into closed enums or through the paste text validator before any frame is sent.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/empty-typed-text-refused · Empty typed text refused — A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed. · why: Ratified.
decision/hook-security-fields-closed · Hook security fields in closed enums — Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
decision/input-box-readiness-from-literals · Input box readiness read from compiled literals — The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate. · why: A boundary widening that was ratified.
decision/invalid-hook-frames-dropped · Invalid hook frames dropped — An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
decision/mcp-handlers-check-raw-args · MCP handlers check raw arguments — Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only. · why: rmcp does not enforce the schemars schemas
decision/mcp-handlers-validate-raw-arguments · MCP handlers validate raw arguments — MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation. · why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
decision/paste-text-character-set · Paste text character set — Paste text allows LF, CR and TAB and refuses every other C0 control, DEL and every C1 character, checked over decoded characters.
decision/paste-text-control-character-set · Paste text control-character set — Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes. · why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
decision/paste-validation-on-both-sides · Paste validation on both sides — Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/refused-text-never-stripped · Refused text is refused whole — A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
decision/reject-dont-strip-paste-text · Reject, don't strip — A sent text that holds a refused character is refused whole; no refused character is ever removed and the rest typed.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/typed-text-line-ending-normalisation · Typed text line-ending normalisation — After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste. · why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged and the delivery-confirmation match stays an exact equality.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/wrapper-checks-authoritatively · Wrapper re-checks text authoritatively — Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
contract/channel-errors/32602 · channel error codes · -32602 — A method whose params fail the wrapper's check is answered -32602.
contract/channel-frames/hook-event-revalidation · channel frames · hook.event — kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; a turn-ended cause, when present, is a string in the closed turn-end cause set (api-error alone); an invalid event is not appended.
contract/channel-frames/send-params · channel frames · send params — text is a string and from is absent, null or a valid viola name; anything else is -32602 invalid params with data null, before paste validation runs; the envelope types no method param, so a mistyped from never gets -32600.
contract/mcp-errors/invalid-params · MCP tool errors · invalid-params — A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
contract/mcp-tool-inputs/invalid-params · MCP tool inputs · invalid-params — A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.
contract/send-refusal/control-character · send refusal · control-character — A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
contract/send-refusal/empty-text · send refusal · empty-text — A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/hook-security-fields-closed-enums · Hook security fields in closed enums — Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
contract/channel-response-log/detail · channel-response log line · detail — The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.

## data-protection · ## Data Protection
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product.

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/reply-code-words-validated · Reply code words validated — A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply. · except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/unusable-response-line-on-stderr · Unusable answer or send text goes to stderr — The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document. · why: A named exception to the --json error document.
contract/channel-refusal/control-character · channel result.refusal · not-delivered/control-character — A text holding a refused control character is answered with result.refusal not-delivered/control-character.
contract/channel-refusal/empty-text · channel result.refusal · not-delivered/empty-text — A text whose typed text is empty is answered with result.refusal not-delivered/empty-text.
contract/mcp-iserror/refusal · MCP isError structuredContent · refusal — A refusal that is no code word reads unknown, alike on MCP and CLI --json.
contract/mcp-iserror/text · MCP isError text · refused text — Every isError text reads refused: <code>.
decision/cli-frame-verbs-strict-modes-first · CLI frame verbs check the instance first — CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The capture arm runs ahead of that check.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/control-check-on-decoded-chars · Control-character check on decoded characters — The control-character check runs over decoded characters, never raw bytes. · why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
decision/empty-text-refused · Empty typed text is refused — A text whose typed text is empty is refused with empty-text, since there is nothing to type.
decision/harness-prefix-literals · Harness prefixes as four raw-start literals — A prompt is classified harness only by four compiled literals matched on its raw start with no trim: <agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message. · why: ratified side effect: the escaped cross-session form is the one escaped tag that classifies harness, so a human who types it at a prompt's start is filed harness
decision/never-refuse-line-breaks · Line breaks and tabs are allowed — LF, CR and TAB are never refused, and a text with content is never refused for holding them. · why: Multi-line relays, plans and answers are normal.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/refuse-control-characters · Refuse control characters, never strip them — A send text or answer free text holding any C0 control other than LF, CR and TAB, DEL or any C1 control is rejected with control-character and never typed into the PTY nor silently stripped. · except: send typing CR LF pairs and lone CRs as LF and dropping trailing CR and LF after validation is not stripping.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/channel-from-not-identity · Channel from is not an identity — The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/private-diagnostics-files · Private diagnostics files — diagnostics/ files are never created readable by other users.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.

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
