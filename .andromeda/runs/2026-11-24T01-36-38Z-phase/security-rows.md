# security-plan · the rows that bind 2026-11-24-viola-focus — picked 134 of 653 (92 by every draw) · rule any · pin 4392

## threat-model-summary · ## Threat Model Summary
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/child-path-trust · Child processes trust PATH — Spawned child processes are whatever the calling process's PATH resolves to.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cli-input-trust-boundary · CLI input trust boundary — CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype, the closed model-word and effort-word checks and tolerant config.json parsing.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and, as the key of the start's trust record, in the Claude config, never on a log line, error body or stdout.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot and, as the key of its trust record, in the Claude config, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/internal-error-output/json · catch-site internal error output · --json — Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/replayed-snapshot-yields-nothing · Replayed snapshot runs nothing and frames nothing — A snapshot replayed from the log carries no endpoint, pid or start time, so it yields no frame.
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
contract/cli-exit-codes/21 · CLI exit codes · 21 — A frame verb refused because the instance failed the strict-modes check.

## authentication-authorization · ## Authentication & Authorization
decision/hyprland-recorder-moves-own-window · Hyprland recorder moves only its own terminal's window — The test-only Hyprland recorder moves only the window of a terminal it started itself, then ends and reaps that terminal and waits until its window is gone; it never moves a window it did not open. · guard: hypr_tap_record_move_moves_the_window_of_its_own_terminal_and_ends_it · provisional
decision/launch-pair-authenticates-nothing · Recorded launch pair authenticates nothing — The recorded model and effort are self-reported words and authenticate nothing. · provisional
decision/lineage-parent-authenticates-nothing · Self-reported parent authenticates nothing — An instance's parent is read as the parent only — never the instance, never an identity, never joined into a path — taken from --parent or else the inherited VIOLA_NAME and dropped when it is the instance's own name; it authenticates nothing. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-before-snapshot-read · Strict-modes check before the snapshot is read — Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification. · why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/sysinfo-process-identity · sysinfo for process identity — sysinfo is what reads a process's pid and start time for server verification.
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
decision/window-origin-authenticates-nothing · Recorded window origin authenticates nothing — The recorded window origin, cli or agent, is self-reported by the caller and authenticates nothing. · provisional
decision/window-step-moves-own-terminal-window · Wrapper's window step moves only its own terminal's window — The wrapper's window step moves only the window of its own terminal, found by its parent pid within a 5 s wait, at most once, and never a window it did not open. · guard: window_step_moves_the_window_of_its_terminal_to_the_place · provisional
contract/strict-refusal-message/readable · strict-modes refusal message · Readable — The read rule's refusal reads exactly "a trusted state path is readable by another user".
contract/strict-refusal-message/symlink · strict-modes refusal message · Symlink — A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".
decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
decision/no-seam-for-foreign-peer · No seam to drive a foreign peer — A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them. · why: one account can make no foreign peer
contract/strict-modes-refusal/cli-frame-verbs · strict-modes refusal · CLI frame verb — Before liveness and server checks: no frame and no line, nothing opened under the home; exit 21 with the instance-state unable:/hint: pair, or under --json the instance-unreachable document with detail strict-modes-failed.
contract/strict-modes-refusal/link-unlink · strict-modes refusal · viola link and viola unlink — Exit 21 with the instance-state unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout; no log, no line, the mode as found.
contract/strict-modes-refusal/list-human · strict-modes refusal · viola list in human form — Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
contract/strict-modes-refusal/list-json · strict-modes refusal · viola list under --json — Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
contract/strict-modes-refusal/plugin-install · strict-modes refusal · viola plugin install — Exit 21 (instance-unreachable under --json) with nothing created and no log opened.
contract/strict-modes-refusal/run-revive · strict-modes refusal · run and revive — Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.

## input-validation · ## Input Validation
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/cli-names-through-name-check · CLI names through the viola name check — Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
decision/frame-verbs-instance-check-first · Frame verbs check the instance first — CLI frame verbs and link/unlink run the instance strict-modes check first; a refusal is exit 21 with the instance pair, nothing opened under the home and no line.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/hyprctl-read-deadline · hyprctl child bounded by a deadline — The hyprctl child is resolved on the calling process's PATH and runs with null stdin and the named environment variables removed, the R8 strip's names on the wrapper's read, under a 5 s deadline at which it is killed and reaped; a deadline kill or a non-zero exit reads as unreachable. · guard: read_past_the_deadline_kills_and_reaps_its_child
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/names-through-validated-type · Names through the validated name type — Every instance name taken from CLI arguments passes the validated name constructor.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate, and the Claude transcript path shape check in the Claude adapter crate, which the hook and the wrapper both call.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/unwrapped-name-never-target · Unwrapped agent names are no targets — An unwrapped agent row's name is never a validated viola name and never a valid target.
decision/unwrapped-session-name-not-target · Unwrapped session name never a target — An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
contract/cli-json-error/strict-modes-failed · CLI frame verb --json error · strict-modes-failed — A refused instance check under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21.
decision/board-window-and-origin-re-parsed · Board window and origin shown only when they re-parse — viola list --json and the MCP list item carry a recorded window only when its address re-parses through the one window address validator, and an origin only when it is a closed origin word. · why: A boundary widening of the list item. · guard: board_read_drops_a_window_address_that_does_not_parse · provisional
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/channel-frame-cap-before-read · Channel frame cap before reading — A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value makes the dialog hold zero or unbounded or lifts the unverified CLI gate, and an identity-floor name in claude_env_keep has no effect.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
contract/channel-frames/oversize-frame · channel frames · over-limit frame — A frame is read under a MAX_FRAME cap; an over-limit frame gets -32600 and the connection is closed.

## data-protection · ## Data Protection
decision/diagnostics-open-fails-to-sink · A refused diagnostics open writes nowhere — Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
decision/hyprland-recorder-string-allowlist · Hyprland recorder keeps strings by a closed allowlist — viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag; none of monitors); any other string becomes the placeholder, a non-JSON reply (the move's) only ok; the shared scrub refuses an unclean set whole. · why: Hyprland 0.56.2 carries titles and user text under keys no deny-list foresees (lastwindowtitle, nested workspace.name, tags). · guard: record_replaces_every_string_off_the_allowlist; record_keeps_no_monitors_string
decision/hyprland-reply-text-dropped · Hyprland reply text dropped at the parse — Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error. · guard: contract_hyprland_canary_reaches_no_line_and_no_error
decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
decision/open-ancestor-walk-in-memory · Open's ancestor walk held in memory — viola open's parent-window lookup walks at most 8 ancestor pids, held in memory only and never recorded. · why: A boundary widening of what viola open reads. · provisional
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/window-record-kept-to-snapshot-and-event-log · Window record kept to the snapshot and the event log — An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item; no address, pid or workspace enters a process-log line. · provisional
decision/data-protection-minimal-tier · Data protection at Minimal tier, adapted — Data protection follows the Minimal tier with file-permission controls kept in scope. · why: viola stores sensitive user content; HTTPS enforcement does not apply
decision/launch-pair-kept-to-snapshot-and-event-log · Launch pair kept to the snapshot and the event log — An instance's launch pair is kept only in the snapshot's launch field and the events log's launch line, log-only with no span, process-log line or terminal byte, and leaves only through list --json and the MCP list item. · provisional
decision/lineage-kept-to-snapshot-and-event-log · Lineage kept to the snapshot and the event log — An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.

## dependency-security · ## Dependency Security
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/human-output-escapes-controls · Human-mode output escapes control characters — Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.

## error-handling · ## Error Handling
decision/error-enum-convention-exceptions · Named exceptions to the error-enum convention — ServeError, StampsError, PinError, StampsReadError and Refused diverge from the error-enum convention as named exceptions. · why: Ratified as named exceptions.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/internal-error-held-at-unit-layer · Internal errors tested at the unit layer — No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path. · guard: answer_a_panicking_dispatch_is_internal_error_and_closes
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
contract/cli-json-error/internal-error · CLI --json error documents · internal-error — Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
contract/cli-stderr-hints/unwrapped-name-endpoint-lost · CLI stderr hints · unwrapped-name and endpoint-lost hints — The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
contract/mcp-iserror/refusal · MCP isError structuredContent · refusal — A refusal that is no code word reads unknown, alike on MCP and CLI --json.
decision/cli-frame-verbs-strict-modes-first · CLI frame verbs check the instance first — CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
contract/cli-instance-unreachable/json-document · CLI exit 21 · --json document — Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
contract/instance-unreachable-cause/strict-modes-failed · instance-unreachable detail causes · strict-modes-failed — A closed cause of instance-unreachable.
contract/instance-unreachable-cause/unwrapped-name · instance-unreachable detail causes · unwrapped-name — A closed cause of instance-unreachable.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — A cause of exit 21 in the CLI --json document and MCP's isError result: the instance's strict-modes check refused, and no frame was sent.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/agents-row-name-not-a-target · claude agents names are not targets — A claude agents --json row name is never used as a ViolaName or target.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/private-diagnostics-files · Private diagnostics files — diagnostics/ files are never created readable by other users.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/control-check-on-decoded-chars · Control-character check on decoded characters — The control-character check runs over decoded characters, never raw bytes. · why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included · except: The test-only live rig seeds a stamp kept from its own verify scratch home into its own CC or R3W scratch home only.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
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
