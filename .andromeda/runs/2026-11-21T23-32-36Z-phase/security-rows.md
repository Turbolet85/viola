# security-plan · the rows that bind 2026-11-21-key-after-a-resize — picked 104 of 630 (58 by every draw) · rule any · pin 3781

## threat-model-summary · ## Threat Model Summary
decision/bridge-send-one-bracketed-paste · Bridge send is one bracketed paste — A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/delivery-confirmation-trailing-tolerance · Delivery confirmed with a trailing whitespace tolerance — Delivery confirmation takes the submitted prompt text as the driver's when it equals the typed text exactly, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; no other difference, leading trimming included, is tolerated. · why: The claude CLI trims whitespace at submit, so a driver's send ending in that whitespace otherwise read back as not its own. · guard: submitted_form_takes_a_trailing_run_the_cli_trims
decision/human-typing-takes-wheel · Human typing takes the wheel — A byte on viola run's own stdin that is typing takes the wheel from the driver. · except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/send-text-is-one-bracketed-paste · Send text typed as one bracketed paste — Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
decision/stdin-closed-non-editing-list · Stdin classifier's closed non-editing list — Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess. · why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
decision/verify-never-answers-dialogs · Verify never answers dialogs — viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key; a browser-tools prompt standing at start is left for the human to answer once in claude, as verify says before its summary.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signatures: the literals, and the compiled shapes over them, and their row text leaves the process only as signature-only record fixtures. · except: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signatures: the literals, and the compiled shapes over them, their row text leaving the process only as signature-only record fixtures. · except: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
decision/verify-writes-no-dialog-byte · Verify never types into a dialog — No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output. · why: ratified
contract/mcp-tools/send · MCP tools · send — A tool called by the driver model, input checked by its schema and the ViolaName check.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wheel-event/human-input · wheel event · cause human-input — The wheel moved to holder human because a byte on viola run's stdin counted as typing.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session; a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation.
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/non-editing-list-measured-only · Non-editing list holds measured replies only — A terminal reply is added to the stdin classifier's non-editing list only once a terminal has been measured to write it, never on a guess.
decision/pinned-artefacts-never-deleted · Pinned binary and plugin folders kept — The pinned binary and the plugin folders are never deleted in v1.
decision/pinned-binary-placed-once · Pinned binary placed once — The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears. · why: an exec of a copy another start still holds open for write fails with ETXTBSY
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals; Run B's settings take a compiled accept literal, and the sender's prompt holds compiled literals and the composed uds address alone. · why: Ratified live rulings, each given after the widening was shown.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.

## authentication-authorization · ## Authentication & Authorization
decision/live-round-rig-keys-own-pty · Live-round rig keys only into its own pty — The founder's word anew per chunk binds only the live rig's closed argv grant: at most two down-arrows and Enter only on a read trust row, one x and one DEL in a ready box. Its stop, Ctrl-C into its own pty or a no-key group kill, needs no word; other keys write nothing; viola types into no dialog. · why: A boundary widening bounded as the founder bounded it; a rig is no viola, so the product's own pty seam stays unchanged.
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version on which every relied ledger row counts: it reads pass, or absent on the browser-tools-prompt row alone; absent on any other row leaves the version unverified. · except: StopFailure is registered with no capability ledger row and no verify probe.
decision/explicit-mode-before-write · Mode set explicitly before any byte — A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded. · except: The trust record writer replaces Claude's config file keeping that file's own mode.
decision/live-rig-hb-arm-keeps-attributes-only · Live rig's HB arm keeps cell attributes and counts only — The live rig's HB arm types one x and one DEL into a fresh box in its own unwrapped pty before any send, and records only cell attributes and counts, never screen text.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/cli-input-checked-before-frame · CLI input checked before any frame — answer and send input is read under MAX_FRAME and parsed into closed enums or through the paste text validator before any frame is sent.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/input-box-readiness-from-literals · Input box readiness read from compiled literals — The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a standing modal is up; one predicate decides for verify and for viola run's pre-send gate. · why: A boundary widening that was ratified. · except: A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.
decision/paste-validation-on-both-sides · Paste validation on both sides — Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
decision/refused-text-never-stripped · Refused text is refused whole — A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
decision/reject-dont-strip-paste-text · Reject, don't strip — A sent text that holds a refused character is refused whole; no refused character is ever removed and the rest typed.
decision/typed-text-line-ending-normalisation · Typed text line-ending normalisation — After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste. · why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signatures: the literals, and the compiled shapes over them.
decision/verify-reads-only-signatures · Verify reads PTY output by signatures — verify's PTY output is read only against the compiled signatures: the literals, and the compiled shapes over them.
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/wrapper-checks-authoritatively · Wrapper re-checks text authoritatively — Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
contract/send-refusal/control-character · send refusal · control-character — A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
decision/empty-typed-text-refused · Empty typed text refused — A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed. · why: Ratified.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/paste-text-character-set · Paste text character set — Paste text allows LF, CR and TAB and refuses every other C0 control, DEL and every C1 character, checked over decoded characters.
decision/paste-text-control-character-set · Paste text control-character set — Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes. · why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/send-refusal/empty-text · send refusal · empty-text — A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.

## data-protection · ## Data Protection
decision/live-session-screens-kept-local · Live session screens kept local — A live session's screen frames are kept only under a gitignored local path, never in a work dir, a run dir or a commit; what lands is a closed-word summary per frame. · why: A real session's screen holds the user's own status-line output and the repository is public.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design

## secret-management · ## Secret Management
decision/live-rig-credentials-copy · Live rig credentials copy transient and unrecorded — The live rig copies the claude OAuth credentials file owner-only into its private scratch, reads only its expiry, refuses a start with under 20 minutes of token, removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary. · guard: rig_leaves_no_credentials_copy_on_any_exit_path; rig_keeps_no_credential_or_host_path_in_any_record · except: The VH arm, run under the real home, makes no copy and reads no credentials file.
decision/live-rig-isolated-child · Live rig child isolated from the user's config — The live rig runs its child under a private CLAUDE_CONFIG_DIR with an environment from a closed kept list of host names plus its own, and reads the user's ~/.claude.json only after a moved stamp, searching it for rig-made needles, dropping the bytes and keeping one closed word. · guard: rig_reads_the_user_config_before_and_after · except: The VH arm runs under the real home and config, reads no credentials, keeps only key names and one needle word, prints no value, and needs an explicit home, Claude dir and user config.
decision/live-rig-seed-from-bundle-logic · Live rig's child config seeded from bundle logic only — The live rig seeds its child's private config with no value read from the user's config: its used-config markers and, on every arm but VP, the browser-tools prompt turned off come from the CLI bundle's own logic; VP leaves the browser-tools key out.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.

## error-handling · ## Error Handling
contract/channel-refusal/control-character · channel result.refusal · not-delivered/control-character — A text holding a refused control character is answered with result.refusal not-delivered/control-character.
contract/instance-unreachable-cause/call-deadline · instance-unreachable detail causes · call-deadline — A closed cause of instance-unreachable, written by the MCP frame tools alone when a call reaches its call deadline; the CLI exit-21 cause set stays six and never carries it.
contract/mcp-iserror/instance-unreachable · MCP isError structuredContent · error instance-unreachable — An unreachable instance, or a frame tool call reaching its call deadline, reads error instance-unreachable with detail one of its closed causes, call-deadline among them on MCP alone.
contract/mcp-iserror/text · MCP isError text · refused text — Every isError text reads refused: <code>.
decision/internal-error-held-at-unit-layer · Internal errors tested at the unit layer — No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path. · guard: answer_a_panicking_dispatch_is_internal_error_and_closes
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/unusable-response-line-on-stderr · Unusable answer or send text goes to stderr — The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document. · why: A named exception to the --json error document.
contract/mcp-errors/-32603 · MCP JSON-RPC errors · -32603 — Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.
contract/mcp-frame-tool-result/unreachable · viola mcp frame tool results · instance-unreachable — A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command names come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text; a typed text is a local command only by its compiled name, alone or followed by one space and arguments.
decision/never-refuse-line-breaks · Line breaks and tabs are allowed — LF, CR and TAB are never refused, and a text with content is never refused for holding them. · why: Multi-line relays, plans and answers are normal.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/refuse-control-characters · Refuse control characters, never strip them — A send text or answer free text holding any C0 control other than LF, CR and TAB, DEL or any C1 control is rejected with control-character and never typed into the PTY nor silently stripped. · except: send typing CR LF pairs and lone CRs as LF and dropping trailing CR and LF after validation is not stripping.
decision/vt100-inside-catch-unwind · vt100 only inside catch_unwind — vt100 is never fed outside catch_unwind. · why: a parser panic must not take down the human's terminal path
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row counts: pass, or absent on the browser-tools-prompt row alone. · why: a stamp is what switches on dialog answering
decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer refused unverified-cli: no verify stamp for the child's CLI version on which every relied ledger row counts (pass, or absent on the browser-tools-prompt row alone).

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
