# security-plan · the rows that bind 2026-11-28-fleet-console-view — picked 113 of 670 (76 by every draw) · rule any · pin 5303

## threat-model-summary · ## Threat Model Summary
decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/child-path-trust · Child processes trust PATH — Spawned child processes are whatever the calling process's PATH resolves to.
decision/list-home-read-rule · Board scan refuses a readable home — viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/run-pins-own-folder-first-on-path · Pinned folder first on the child's PATH — viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/mcp-tools/list · MCP tools · list — A tool called by the driver model that takes no input, reading the board.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot, as the key of the start's trust record in the Claude config, and as the working directory on viola up's terminal argv, never on a log line, error body or stdout.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, as the key of its trust record in the Claude config, and as the working directory on viola up's terminal argv, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start, adopt's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.

## authentication-authorization · ## Authentication & Authorization
decision/board-home-read-rule · The board holds the home to the read rule — viola list refuses a home another user may read, with or without an instance.
decision/console-terminal-and-home-check · viola console checks its terminal and home first — viola console refuses unless stdin and stdout are both terminals, then holds the home to the strict-modes read rule, an absent home passing, before any log opens; either refusal writes no line, leaves the modes as found and exits 1.
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-before-snapshot-read · Strict-modes check before the snapshot is read — Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification. · why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
decision/whole-board-refusal · viola list refuses the whole board — viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board. · why: ratified
contract/strict-modes-refusal/console · strict-modes refusal · viola console — Exit 1 with the fixed pair unable: viola console could not use its home, then hint: make the viola home readable by you alone, then run viola console again, on stderr, with no line written and the mode left as found.
decision/lineage-parent-authenticates-nothing · Self-reported parent authenticates nothing — An instance's parent is read as the parent only — never the instance, never an identity, never joined into a path — taken from --parent or else the inherited VIOLA_NAME and dropped when it is the instance's own name; it authenticates nothing. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
contract/strict-modes-refusal/list-human · strict-modes refusal · viola list in human form — Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
contract/strict-modes-refusal/list-json · strict-modes refusal · viola list under --json — Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.

## input-validation · ## Input Validation
decision/board-launch-pair-re-parsed · Board model and effort shown only when they re-parse — The board shows a recorded model only when it passes the model-word check and a recorded effort only when it passes the code-word check.
decision/board-lineage-and-links-name-checked · Board lineage and links shown only when they re-parse — The board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse; a gone instance gives no links.
decision/board-window-and-origin-re-parsed · Board window and origin shown only when they re-parse — viola list --json and the MCP list item carry a recorded window only when its address re-parses through the one window address validator, and an origin only when it is a closed origin word. · why: A boundary widening of the list item. · guard: board_read_drops_a_window_address_that_does_not_parse
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/claude-agents-read-fail-open · claude agents read bounded, fail open — The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
decision/claude-agents-resolved-from-path · claude agents resolved from the caller's PATH — list, revive's preflight, adopt's live-elsewhere check and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME. · why: Same-user trust, an accepted risk; the MCP widening ratified.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/console-key-reader-bounded · Console key reader bounded and closed — viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded to one closed class: 0x03 anywhere is interrupt, exactly q is quit, anything else is unbound; no key byte reaches any log line or output. · guard: console_logs_an_unbound_key_by_class_alone · provisional
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/list-scan-checks-before-open · viola list checks before opening — viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/no-width-or-colour-override-env · No width or colour override env — COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read; the board's width comes from the host terminal size, no env read.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/torn-last-line-healed-on-append · Torn last line healed on append — The next append heals a torn last line by writing one LF ahead of its line in the same single write under the append lock, and logs one state-recovered line with a detail code, basename and offset, no content.
decision/unwrapped-name-never-target · Unwrapped agent names are no targets — An unwrapped agent row's name is never a validated viola name and never a valid target.
decision/unwrapped-session-name-not-target · Unwrapped session name never a target — An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/colour-env-only-compared · Terminal colour variables only compared — NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
decision/replayed-snapshot-inert · Replayed snapshot is inert — A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid or start time, so no frame is written from it.
decision/terminal-colour-facts · Terminal colour facts read for list only — NO_COLOR, TERM and COLORTERM are read only by viola list's human mode, only compared, never echoed, logged or set, and decide only whether and how deep SGR is written around already escaped rows. · why: Ratified.

## data-protection · ## Data Protection
decision/lineage-kept-to-snapshot-and-event-log · Lineage kept to the snapshot and the event log — An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/window-record-kept-to-snapshot-and-event-log · Window record kept to the snapshot and the event log — An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item; no address, pid or workspace enters a process-log line.
decision/exec-commands-absolute-pinned-path · Exec commands use an absolute path — Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive, adopt and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
decision/launch-pair-kept-to-snapshot-and-event-log · Launch pair kept to the snapshot and the event log — An instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings; log-only with no span, process-log line or terminal byte, it leaves only through list --json and the MCP list item.
decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
contract/events-link-lines/data · events.ndjson link lines · data — A link or unlink line carries data {driver, driven}: the two instance names only, no user content.

## dependency-security · ## Dependency Security
decision/g2-single-panic-exemption · G2 zero-panics gate has one exemption — CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry. · guard: G2 --probe run before every G2 check
decision/g2-zero-panics-single-exemption · Zero-panics gate with one exemption — CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry. · guard: G2 --probe · except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and the start of run, revive and adopt when kept
contract/artifact-canary-scan-classes/content-canary · artifact canary scan classes · content canary — The content canary fails the scan only in home-level role files.

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start, adopt's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/agents-row-name-not-a-target · claude agents names are not targets — A claude agents --json row name is never used as a ViolaName or target.
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/control-check-on-decoded-chars · Control-character check on decoded characters — The control-character check runs over decoded characters, never raw bytes. · why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included · except: The test-only live rig seeds a stamp kept from its own verify scratch home into its own CC or R3W scratch home only.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/colour-facts-choose-sgr-only · Terminal colour facts choose SGR only — NO_COLOR, TERM and COLORTERM are read only by viola list's human mode to choose whether and how deep SGR bytes are written; they change no control, widen no redaction and reach no log line, error or refusal. · why: ratified
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.

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
