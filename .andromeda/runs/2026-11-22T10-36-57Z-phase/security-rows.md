# security-plan · the rows that bind 2026-11-22-paste-write-bounded — picked 81 of 632 (50 by every draw) · rule any · pin 3869

## threat-model-summary · ## Threat Model Summary
decision/bridge-send-one-bracketed-paste · Bridge send is one bracketed paste — A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/delivery-confirmation-trailing-tolerance · Delivery confirmed with a trailing whitespace tolerance — Delivery confirmation takes the submitted prompt text as the driver's when it equals the typed text exactly, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; no other difference, leading trimming included, is tolerated. · why: The claude CLI trims whitespace at submit, so a driver's send ending in that whitespace otherwise read back as not its own. · guard: submitted_form_takes_a_trailing_run_the_cli_trims
decision/human-typing-takes-wheel · Human typing takes the wheel — A byte on viola run's own stdin that is typing takes the wheel from the driver. · except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/send-text-is-one-bracketed-paste · Send text typed as one bracketed paste — Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
decision/stdin-closed-non-editing-list · Stdin classifier's closed non-editing list — Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess. · why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/mcp-tools/send · MCP tools · send — A tool called by the driver model, input checked by its schema and the ViolaName check.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wheel-event/human-input · wheel event · cause human-input — The wheel moved to holder human because a byte on viola run's stdin counted as typing.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session; a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and, as the key of the start's trust record, in the Claude config, never on a log line, error body or stdout.
decision/pinned-binary-placed-once · Pinned binary placed once — The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears. · why: an exec of a copy another start still holds open for write fails with ETXTBSY
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot and, as the key of its trust record, in the Claude config, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username

## authentication-authorization · ## Authentication & Authorization
contract/channel-response-line/release-from-driver · channel-response log line · release-from-driver — The server's and client's line is WARN with detail release-from-driver and no param name or value; a from of another type reads invalid-params; the line adds nothing to the wire body.

## input-validation · ## Input Validation
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/input-box-readiness-from-literals · Input box readiness read from compiled literals — The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a standing modal is up; one predicate decides for verify and for viola run's pre-send gate. · why: A boundary widening that was ratified. · except: A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
contract/channel-response-log/detail · channel-response log line · detail — The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
contract/send-refusal/control-character · send refusal · control-character — A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
contract/send-refusal/empty-text · send refusal · empty-text — A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/paste-text-control-character-set · Paste text control-character set — Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes. · why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/typed-text-line-ending-normalisation · Typed text line-ending normalisation — After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste. · why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
contract/cli-json-error/strict-modes-failed · CLI frame verb --json error · strict-modes-failed — A refused instance check under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21.
contract/mcp-errors/invalid-params · MCP tool errors · invalid-params — A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
contract/mcp-tool-inputs/invalid-params · MCP tool inputs · invalid-params — A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.

## data-protection · ## Data Protection
decision/diagnostics-open-fails-to-sink · A refused diagnostics open writes nowhere — Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.

## dependency-security · ## Dependency Security
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/scan-gated-ci-uploads · CI uploads gated by the artifact canary scan — The CI test job runs the artifact canary scan against the NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass. · except: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, leave CI unscanned; a non-synthetic seed must first be dropped

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/ci-artifact-canary-scan · CI artifact canary scan gates uploads — Every scan-gated CI upload waits for the artifact canary scan over test diagnostics, harness capture and test reports against the NEVER-log floor, which never prints or writes the matched bytes. · except: The harness capture's chunk.diff, repository source text by construction, is skipped by exact path and never uploaded.

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
contract/artifact-canary-scan-classes/diagnostics-mode · artifact canary scan classes · diagnostics file mode — The scan fails on a diagnostics file whose mode is not 0600.

## error-handling · ## Error Handling
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/reply-code-words-validated · Reply code words validated — A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply. · except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
contract/instance-unreachable-cause/call-deadline · instance-unreachable detail causes · call-deadline — A closed cause of instance-unreachable, written by the MCP frame tools alone when a call reaches its call deadline; the CLI exit-21 cause set stays six and never carries it.
contract/mcp-iserror/detail · MCP isError structuredContent · detail — A string detail that is no known detail reads null, alike on MCP and CLI --json.
contract/mcp-iserror/instance-unreachable · MCP isError structuredContent · error instance-unreachable — An unreachable instance, or a frame tool call reaching its call deadline, reads error instance-unreachable with detail one of its closed causes, call-deadline among them on MCP alone.
contract/mcp-iserror/refusal · MCP isError structuredContent · refusal — A refusal that is no code word reads unknown, alike on MCP and CLI --json.
contract/mcp-iserror/text · MCP isError text · refused text — Every isError text reads refused: <code>.
decision/unusable-response-line-on-stderr · Unusable answer or send text goes to stderr — The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document. · why: A named exception to the --json error document.
contract/cli-instance-unreachable/json-document · CLI exit 21 · --json document — Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
contract/mcp-frame-tool-result/unreachable · viola mcp frame tool results · instance-unreachable — A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.
contract/mcp-iserror/wrapper-fault · MCP isError structuredContent · error wrapper-fault — A wrapper fault reads error wrapper-fault with a sanitised code, message and data, never the peer's own.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/private-diagnostics-files · Private diagnostics files — diagnostics/ files are never created readable by other users.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.

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
