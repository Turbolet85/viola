# security-plan · the rows that bind 2026-11-22-lineage-registry — picked 148 of 633 (106 by every draw) · rule any · pin 4060

## threat-model-summary · ## Threat Model Summary
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cli-input-trust-boundary · CLI input trust boundary — CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/flat-file-state-store · Flat-file state store — State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
decision/from-field-authenticates-nothing · Self-reported from authenticates nothing — The self-reported from field on the wrapper channel authenticates nothing and is never treated as an identity.
decision/human-typing-takes-wheel · Human typing takes the wheel — A byte on viola run's own stdin that is typing takes the wheel from the driver. · except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
decision/list-home-read-rule · Board scan refuses a readable home — viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/os-user-ambient-trust · OS-user ambient trust — The only principal is the local OS user: the IPC endpoints and the viola home trust every process of the same OS user and no other, with no per-connection authentication. · why: viola is a single-user local tool with no accounts or hosting, driven by agents with no interactive login, which matches an ambient OS-user trust model rather than a credential flow.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
decision/plaintext-state · Plaintext state — Nothing in the state store is encrypted; all of it is plaintext. · why: Readability, by design.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/replayed-snapshot-yields-nothing · Replayed snapshot runs nothing and frames nothing — A snapshot replayed from the log carries no endpoint, pid or start time, so it yields no frame.
decision/same-user-trust-boundary · Same OS user trust boundary — The wrapper channel trusts every process of the same OS user and no other; there is no per-connection authentication and the caller's self-reported from is not authenticated.
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wheel-event/human-input · wheel event · cause human-input — The wheel moved to holder human because a byte on viola run's stdin counted as typing.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and, as the key of the start's trust record, in the Claude config, never on a log line, error body or stdout.
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot and, as the key of its trust record, in the Claude config, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username
decision/stdin-closed-non-editing-list · Stdin classifier's closed non-editing list — Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess. · why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
contract/wrapper-channel-methods/link · wrapper channel methods · link — Adds a driver link, whose effect is a link line in the event log and the snapshot's links only; the self-reported from is not read.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session; a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation.
contract/wrapper-channel-methods/unlink · wrapper channel methods · unlink — Removes a driver link, whose effect is an unlink line in the event log and the snapshot's links only; the self-reported from is not read.

## authentication-authorization · ## Authentication & Authorization
decision/all-or-nothing-access · All-or-nothing access — Any caller that passes the OS check gets the full surface, with no roles.
decision/all-or-nothing-channel-access · Capability-based, all-or-nothing channel access — Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles. · why: the only principal is the local OS user
decision/board-home-read-rule · The board holds the home to the read rule — viola list refuses a home another user may read, with or without an instance.
decision/home-owner-only · Owner-only viola home — The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
decision/private-file-modes · Owner-only modes for the viola home — The home (and any --home) is created 0700 and every file 0600, never group- or world-writable. · except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
decision/start-checks-home-tree-before-log-opens · Start checks the home tree before its log opens — Run and revive's start check the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child. · why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
decision/start-checks-socket-dir-before-bind · Socket directory checked before any bind, snapshot or child — A start, run or revive alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict. · why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-before-snapshot-read · Strict-modes check before the snapshot is read — Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification. · why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/whole-board-refusal · viola list refuses the whole board — viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board. · why: ratified
contract/socket-dir-refusal/run-and-revive-start · socket-directory refusal · run and the start of revive — A failed create and a refused check read one code: exit 1 with socket-dir-refused and one process-exit line of that detail; a Server::bind failure is outside this code.
contract/strict-modes-refusal/list-human · strict-modes refusal · viola list in human form — Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
contract/strict-modes-refusal/list-json · strict-modes refusal · viola list under --json — Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
contract/strict-modes-refusal/run-revive · strict-modes refusal · run and revive — Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.
decision/explicit-mode-before-write · Mode set explicitly before any byte — A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded. · except: The trust record writer replaces Claude's config file keeping that file's own mode.
decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
decision/only-matches-verdict-lets-a-frame-out · Only a Matches server verdict lets a frame out — Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a client write a frame; a mismatched pid or start time, or an unverifiable server, is refused before any frame. · why: The CLI verbs, the hooks and the MCP frame tools rely on it to reach only the recorded wrapper; server-verify-failed names the refusal.
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with the instance directory standing (a stale wrapper included), or a failed connect; the reading opens nothing and writes no line.
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent or a regular file); the reading opens nothing and writes no line.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/cli-names-through-name-check · CLI names through the viola name check — Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
decision/colour-env-only-compared · Terminal colour variables only compared — NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/link-restore-trusts-written-names · Link restore trusts names it wrote — Link-set restore uses a link or unlink line's driver and driven only when both are strings and does not parse them as names again; it appends and logs nothing. · why: The wrapper wrote them, each having passed the viola name check.
decision/list-scan-checks-before-open · viola list checks before opening — viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/names-through-validated-type · Names through the validated name type — Every instance name taken from CLI arguments passes the validated name constructor.
decision/no-width-or-colour-override-env · No width or colour override env — COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read; the board's width comes from the host terminal size, no env read.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/replayed-snapshot-inert · Replayed snapshot is inert — A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid or start time, so no frame is written from it.
decision/revive-strict-modes-before-use · revive checks strict modes before use — Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate, and the Claude transcript path shape check in the Claude adapter crate, which the hook and the wrapper both call.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/terminal-colour-facts · Terminal colour facts read for list only — NO_COLOR, TERM and COLORTERM are read only by viola list's human mode, only compared, never echoed, logged or set, and decide only whether and how deep SGR is written around already escaped rows. · why: Ratified.
decision/torn-last-line-healed-on-append · Torn last line healed on append — The next append heals a torn last line by writing one LF ahead of its line in the same single write under the append lock, and logs one state-recovered line with a detail code, basename and offset, no content.
decision/unwrapped-name-never-target · Unwrapped agent names are no targets — An unwrapped agent row's name is never a validated viola name and never a valid target.
decision/unwrapped-session-name-not-target · Unwrapped session name never a target — An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
contract/config-json/claude-env-keep · config.json · claude_env_keep — An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
contract/channel-frames/send-params · channel frames · send params — text is a string and from is absent, null or a valid viola name; anything else is -32602 invalid params with data null, before paste validation runs; the envelope types no method param, so a mistyped from never gets -32600.
contract/revive-json/internal-error · viola revive --json codes · internal-error — A closed exit-1 code, printed for an internal error of the start path, which is still logged with its chain; a child-pump error prints no document.
contract/revive-json/output-fields · revive --json output · output fields — revive --json prints codes, closed words and session-shaped ids only, never a path or pid.
contract/revive-json/strict-modes-failed · viola revive --json codes · strict-modes-failed — A closed exit-1 code of viola revive --json.
contract/revive-refusal/no-session · revive refusal words · no-session — An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.

## data-protection · ## Data Protection
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
decision/link-names-kept-out-of-process-log · Link names kept out of the process log — The wrapper's link and unlink methods write no process-log line of their own and their request and response lines carry no name; viola link and unlink print only their own parsed arguments, never a name read back from a reply or log.
decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/data-protection-minimal-tier · Data protection at Minimal tier, adapted — Data protection follows the Minimal tier with file-permission controls kept in scope. · why: viola stores sensitive user content; HTTPS enforcement does not apply
decision/same-user-tampering-out-of-scope · Same-user tampering outside the boundary — Tampering with the pinned copy by the same OS user stays outside the v1 security boundary. · why: The same user could also rewrite the folder name.
decision/transcript-path-redacted · Transcript path redacted — The Claude transcript path is a redacted value, carried only as the optional transcript param beside hook.event, and appears in no event data, events log, log line, span, reply or verify record. · guard: session_start_transcript_path_stays_out_of_data
contract/events-link-lines/data · events.ndjson link lines · data — A link or unlink line carries data {driver, driven}: the two instance names only, no user content.
contract/events-link-lines/no-op-pairs · events.ndjson link lines · a repeated link or an absent pair's unlink — A held pair's second link and an absent pair's unlink append nothing.

## dependency-security · ## Dependency Security
decision/scheduled-fuzzing · Scheduled fuzzing — The scheduled workflow runs each fuzz target for 120 seconds on the pinned fuzz nightly and uploads the fuzz artifacts on failure.
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/credential-strip-by-name-only · Credential strip reads names only — The environment strip handles variable names only and never reads a variable's value. · except: viola verify and the start of run and revive read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/messaging-credentials-are-secret · Messaging token and socket are secrets — The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
decision/r8-claude-env-strip · R8 CLAUDE environment strip — Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values. · why: Ratified. · except: verify and run and revive's start read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps the name, to choose the Claude config holding the trust record; else the user HOME's .claude.json.
decision/non-identity-hook-names · Four hook names are not identity names — CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA, names the CLI adds to a hook, are known and not identity names: they stand off the identity floor and are read only by verify's env reading. · why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
contract/claude-child-process-start/env-stripped-known · process-start claude-child line · env_stripped_known — Names the stripped environment names, never a value.

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/run-refuses-socket-dir-before-start · run refuses a bad socket directory first — viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
contract/cli-json-error/internal-error · CLI --json error documents · internal-error — Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
contract/cli-stderr-hints/unwrapped-name-endpoint-lost · CLI stderr hints · unwrapped-name and endpoint-lost hints — The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
contract/collision-check/already-live · start collision check · already-live — Printed and logged for a live holder and for a fresh replayed beat.
contract/collision-check/stale-heartbeat · start collision check · stale-heartbeat — Printed and logged for a stale holder of the name; stale heartbeat is exit 1's cause alone.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/private-diagnostics-files · Private diagnostics files — diagnostics/ files are never created readable by other users.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect
decision/agents-row-name-not-a-target · claude agents names are not targets — A claude agents --json row name is never used as a ViolaName or target.
decision/channel-from-not-identity · Channel from is not an identity — The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
decision/control-check-on-decoded-chars · Control-character check on decoded characters — The control-character check runs over decoded characters, never raw bytes. · why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/colour-facts-choose-sgr-only · Terminal colour facts choose SGR only — NO_COLOR, TERM and COLORTERM are read only by viola list's human mode to choose whether and how deep SGR bytes are written; they change no control, widen no redaction and reach no log line, error or refusal. · why: ratified
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.

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
