@row_add n=1 row=decision/no-rmcp-http-or-auth-features area=security-anti-patterns-never-do-these rec=1
name: No rmcp HTTP server or auth features
rule: rmcp's transport-streamable-http-server and auth features are never enabled.
@row_add n=2 row=decision/request-size-limits area=security-anti-patterns-never-do-these rec=2
name: Request size limits
rule: MAX_FRAME is enforced on the channel and on each viola mcp stdin line; request size limits are never skipped.
@row_add n=3 row=decision/no-messaging-secrets-in-outputs area=security-anti-patterns-never-do-these rec=3
name: No messaging secrets in any output
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
@row_add n=4 row=decision/no-secrets-in-git area=security-anti-patterns-never-do-these rec=4
name: No secrets or local homes in git
rule: .env files, local --home test directories and signing material are never committed to git.
@row_add n=5 row=decision/no-hardcoded-credentials area=security-anti-patterns-never-do-these rec=5
name: No hardcoded credentials
rule: Credentials are never hardcoded in source or in the include_str! plugin files.
@row_add n=6 row=decision/no-parser-error-chains-outward area=security-anti-patterns-never-do-these rec=6
name: No parser error chains in external output
rule: serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr.
why: their Display quotes input values such as tool input
@row_add n=7 row=decision/hook-silent-and-exit-zero area=security-anti-patterns-never-do-these rec=7
name: viola hook writes no stderr and exits zero
rule: viola hook never writes to stderr and never exits non-zero, including on security-check failures.
@row_add n=8 row=decision/escape-terminal-controls area=security-anti-patterns-never-do-these rec=8
name: Escape C0/C1 controls in human output
rule: wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls.
why: the same class as RUSTSEC-2025-0055
@row_add n=9 row=decision/tracing-subscriber-version-floor area=security-anti-patterns-never-do-these rec=9
name: tracing-subscriber version floor
rule: tracing-subscriber below 0.3.20 is never used, if obs selects it.
@row_add n=10 row=decision/private-diagnostics-files area=security-anti-patterns-never-do-these rec=10
name: Private diagnostics files
rule: diagnostics/ files are never created readable by other users.
@row_add n=11 row=decision/no-absolute-paths-in-external-errors area=security-anti-patterns-never-do-these rec=11
name: No absolute paths in external errors
rule: No external error ever exposes an absolute path.
@row_add n=12 row=decision/upstream-text-is-never-a-command area=security-anti-patterns-never-do-these rec=12
name: Upstream text is never a command (R1)
rule: Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
@row_add n=13 row=decision/single-shell-out area=security-anti-patterns-never-do-these rec=12
name: statusline_command is the only shell-out
rule: statusline_command is the only shell-out, and no second one is ever added.
@row_add n=14 row=decision/compiled-classification-sources area=security-anti-patterns-never-do-these rec=13
name: Classification sources are compiled only
rule: Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
@row_add n=15 row=decision/harness-prefix-literals area=security-anti-patterns-never-do-these rec=13
name: Harness prefixes as four raw-start literals
rule: A prompt is classified harness only by four compiled literals matched on its raw start with no trim: <agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message.
why: ratified side effect: the escaped cross-session form is the one escaped tag that classifies harness, so a human who types it at a prompt's start is filed harness
@row_add n=16 row=decision/no-interprocess-try-overwrite area=security-anti-patterns-never-do-these rec=14,31
name: interprocess try_overwrite stays off
rule: interprocess try_overwrite is never enabled on the socket bind.
why: TOCTOU deletion of an attacker-swapped file
@row_add n=17 row=decision/vt100-inside-catch-unwind area=security-anti-patterns-never-do-these rec=15
name: vt100 only inside catch_unwind
rule: vt100 is never fed outside catch_unwind.
why: a parser panic must not take down the human's terminal path
@row_add n=18 row=decision/actions-pinned-by-sha area=security-anti-patterns-never-do-these rec=16
name: GitHub Actions pinned by full SHA
rule: A GitHub Action is referenced by a full SHA with a version comment, never by @stable, @v2 or any mutable ref.
@row_add n=19 row=decision/no-rust-cache-in-release area=security-anti-patterns-never-do-these rec=17
name: No rust-cache restore in release workflow
rule: Swatinem/rust-cache is never restored in the v1.x release workflow.
@row_add n=20 row=decision/signed-self-update area=security-anti-patterns-never-do-these rec=18
name: self_update only with signatures
rule: self_update never ships without its signatures feature (zipsign).
why: a checksum from the same release proves nothing
@row_add n=21 row=decision/e2e-home-removal-own-dirs-only area=security-anti-patterns-never-do-these rec=19
name: Test-side removal only of its own dirs
rule: A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
@row_add n=22 row=decision/e2e-home-keepers-linked-base area=security-anti-patterns-never-do-these rec=19
name: e2e-home keepers follow only a safe linked base
rule: The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
@row_add n=23 row=decision/viola-ignores-e2e-home-backing area=security-anti-patterns-never-do-these rec=19
name: viola never touches the e2e-home backing
rule: The viola binary itself never reads or writes the target/e2e-home backing by that name.
@row_add n=24 row=decision/new-listener-needs-log-entry area=security-anti-patterns-never-do-these rec=20
name: New listener or channel method needs a log entry
rule: No listener (TCP port, WebSocket, http hooks, MCP HTTP transport) or new channel method is added without a Security Decisions Log entry mapping it to the security plan's IPC controls.
@row_add n=25 row=decision/refusals-never-block-human area=security-anti-patterns-never-do-these rec=21
name: Security refusals never block the human
rule: Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
@row_add n=26 row=decision/absolute-viola-path-in-commands area=security-anti-patterns-never-do-these rec=22
name: Hook, MCP and statusline commands use an absolute viola path
rule: Every exec-form command in the embedded hooks.json or .mcp.json, in plugin install's .mcp.json, and the statusline command written into instances/<name>/settings.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH.
why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
@row_add n=27 row=decision/dialog-answers-need-verify-stamp area=security-anti-patterns-never-do-these rec=23
name: Dialog answers need a verify stamp
rule: A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes.
why: a stamp is what switches on dialog answering
@row_add n=28 row=decision/stamps-written-only-by-verify area=security-anti-patterns-never-do-these rec=23
name: Only viola verify writes the stamps file
rule: No process other than viola verify ever writes ledger/stamps.json.
why: a stamp is what switches on dialog answering, PermissionRequest allow included
@row_add n=29 row=decision/version-gate-strict-stamps-read area=security-anti-patterns-never-do-these rec=24
name: Version gate reads stamps behind the strict-modes check
rule: A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check; a refused read is cli_verified:false.
@row_add n=30 row=decision/snapshot-written-only-by-wrapper area=security-anti-patterns-never-do-these rec=25
name: Only the instance's wrapper writes its snapshot
rule: No process other than the instance's own wrapper writes instances/<name>/snapshot.json.
why: endpoint, pinned_bin and statusline_command in it decide where clients connect and what hook statusline executes
@row_add n=31 row=decision/controls-not-switchable area=security-anti-patterns-never-do-these rec=26
name: No config, env var or flag switches off a control
rule: config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text).
why: environment variables are not a configuration channel
except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
@row_add n=32 row=decision/new-env-read-needs-log-entry area=security-anti-patterns-never-do-these rec=26
name: New seam or environment read needs a log entry
rule: No further test-only seam or environment read is added without its own Decisions Log entry.
@row_none n=1 rec=27 type=S
@row_none n=2 rec=28 type=S
@row_add n=33 row=absence/accounts-and-passwords area=authentication-authorization rec=29
capability: User accounts and passwords
@row_add n=34 row=decision/all-or-nothing-channel-access area=authentication-authorization rec=30
name: Capability-based, all-or-nothing channel access
rule: Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles.
why: the only principal is the local OS user
@row_add n=35 row=decision/per-user-socket-directory area=authentication-authorization rec=31
name: Socket in a per-user 0700 directory
rule: The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.
@row_add n=36 row=decision/socket-directory-check area=authentication-authorization rec=31,52
name: Socket-directory check before bind and connect
rule: Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink.
guard: security_negatives_socket_dir
@row_add n=37 row=contract/socket-dir-refusal/run-and-revive-start area=authentication-authorization rec=31
boundary: socket-directory refusal
item: run and the start of revive
promise: A failed create and a refused check read one code: exit 1 with socket-dir-refused and one process-exit line of that detail; a Server::bind failure is outside this code.
@row_add n=38 row=contract/socket-dir-refusal/cli-json area=authentication-authorization rec=31,50
boundary: socket-directory refusal
item: CLI frame verb under --json
promise: No frame is written and the verb exits 21 with {"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"} on stdout and stderr empty.
@row_add n=39 row=contract/socket-dir-refusal/cli-human area=authentication-authorization rec=50
boundary: socket-directory refusal
item: CLI frame verb in human mode
promise: No frame is written and the verb exits 21 with the unable <name> instance-unreachable line and the hint of cause socket-dir last on stderr.
@row_add n=40 row=contract/socket-dir-refusal/mcp area=authentication-authorization rec=31,51
boundary: socket-directory refusal
item: viola mcp tool
promise: The instance-unreachable result with detail socket-dir-refused, no frame and no line, isError the only record.
@row_add n=41 row=contract/socket-dir-refusal/hook area=authentication-authorization rec=31
boundary: socket-directory refusal
item: viola hook
promise: Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
@row_add n=42 row=decision/socket-file-chmod-after-bind area=authentication-authorization rec=31
name: Socket file chmod 0600 after bind
rule: The socket file is chmod 0600 right after the bind; setting the mode through listener options is not used, chmod after the bind being the one form.
@row_add n=43 row=decision/no-abstract-namespace area=authentication-authorization rec=31
name: No Linux abstract namespace socket
rule: The Linux abstract socket namespace is never used.
@row_none n=3 rec=32 type=4
relied: An absent socket directory passes the check, so a start can create it, while an unstattable one is refused.
@row_add n=44 row=decision/socket-dir-check-never-chmods area=authentication-authorization rec=33
name: Socket-directory check never chmods
rule: The socket-directory check never changes a directory's mode: a refused directory stays as found.
guard: security_negatives_socket_dir
@row_none n=4 rec=34 type=4
relied: The bind and the three client seams share one check on a recorded endpoint's parent directory.
@row_none n=5 rec=35 type=4
relied: The socket-directory check runs before any snapshot, child or bind at run and at the start of revive, a race's loser reading the same.
@row_add n=45 row=decision/leftover-socket-file-kept area=authentication-authorization rec=36
name: Leftover socket file is not removed
rule: A killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: by choice: a harmless file in a 0700 directory
@row_add n=46 row=decision/server-side-peer-euid-check area=authentication-authorization rec=37
name: Server drops a foreign-euid peer
rule: On every accept the server reads the peer's credentials and drops the connection before reading a frame if the peer's euid is not its own.
@row_none n=6 rec=38 type=4
relied: A dropped peer reads end-of-file, and no line and nothing on the terminal is written.
@row_add n=47 row=decision/client-verifies-server-before-frame area=authentication-authorization rec=39
name: Client verifies the server before any frame
rule: Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal.
why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
@row_add n=48 row=decision/strict-modes-before-snapshot-read area=authentication-authorization rec=39,42
name: Strict-modes check before the snapshot is read
rule: Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification.
why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
@row_add n=49 row=contract/server-verify-refusal/cli-exit area=authentication-authorization rec=39,50
boundary: server-verification refusal
item: CLI frame verb exit and line
promise: No frame is written; the verb exits 21 instance-unreachable and writes one process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"} WARN line.
@row_add n=50 row=contract/server-verify-refusal/mcp area=authentication-authorization rec=39,51
boundary: server-verification refusal
item: viola mcp tool
promise: A refused peer euid or failed server check returns instance-unreachable with detail server-verify-failed, no frame and no line, isError the only record.
@row_add n=51 row=contract/server-verify-refusal/hook area=authentication-authorization rec=39
boundary: server-verification refusal
item: viola hook
promise: Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
@row_add n=52 row=decision/no-identity-values-in-refusals area=authentication-authorization rec=39
name: No identity values in verification output
rule: No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
@row_add n=53 row=contract/strict-modes-refusal/cli-frame-verbs area=authentication-authorization rec=39,50
boundary: strict-modes refusal
item: CLI frame verb
promise: Before liveness and server checks: no frame and no line, nothing opened under the home; exit 21 with the instance-state unable:/hint: pair, or under --json the instance-unreachable document with detail strict-modes-failed.
@row_add n=54 row=contract/strict-modes-refusal/hook area=authentication-authorization rec=39,42
boundary: strict-modes refusal
item: viola hook
promise: Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
@row_add n=55 row=contract/strict-modes-refusal/mcp-tool area=authentication-authorization rec=39,51
boundary: strict-modes refusal
item: viola mcp tool
promise: Returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_add n=56 row=decision/collision-check-needs-no-server-verify area=authentication-authorization rec=39
name: run's collision check is not server-verified
rule: run's collision check judges a recorded wrapper by its pid, OS start time and heartbeat (a replayed snapshot by the beat alone) and opens no connection, so the peer-pid server check does not apply to it.
why: it writes no frame, so there is no serving process to verify
@row_add n=57 row=decision/client-seam-check-order area=authentication-authorization rec=40
name: Check order at every client seam
rule: On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
@row_add n=58 row=decision/release-from-driver-guard area=authentication-authorization rec=41
name: Driver-originated release is refused
rule: The wrapper refuses a release whose params carry a string from, before budget is acted on.
why: an affordance-level guard against drivers handing the wheel back to automation, not enforcement: a same-user process can leave out from
except: pause, link and unlink from drivers are allowed
@row_add n=59 row=contract/channel-release/string-from area=authentication-authorization rec=41
boundary: channel release method
item: string from
promise: Refused with -32602 "invalid params" and data {"reason":"release-from-driver"}, one release-from-driver line logged (from_trust:"self-reported"); a budget pause is not lifted and the snapshot's budget fields stay as they were.
@row_add n=60 row=contract/channel-release/non-string-from area=authentication-authorization rec=41
boundary: channel release method
item: from of another type
promise: Refused with -32602 and data null.
@row_add n=61 row=contract/channel-release/params area=authentication-authorization rec=41
boundary: channel release method
item: params
promise: release params are {budget?, from?}; a release on a wheel the driver already holds changes nothing.
@row_add n=62 row=contract/cli-release/from area=authentication-authorization rec=41
boundary: CLI viola release
item: from
promise: from is sent from VIOLA_NAME, so viola release run from a driver session exits 20.
@row_add n=63 row=contract/channel-response-line/release-from-driver area=authentication-authorization rec=41
boundary: channel-response log line
item: release-from-driver
promise: The server's and client's line is WARN with detail release-from-driver and no param name or value; a from of another type reads invalid-params; the line adds nothing to the wire body.
@row_add n=64 row=decision/private-file-modes area=authentication-authorization rec=42
name: Owner-only modes for the viola home
rule: The home (and any --home) is created 0700 and every file 0600, never group- or world-writable.
except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
@row_add n=65 row=decision/explicit-mode-before-write area=authentication-authorization rec=42
name: Mode set explicitly before any byte
rule: A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded.
@row_add n=66 row=decision/one-replace-helper area=authentication-authorization rec=42
name: One shared helper replaces files
rule: Every replaced file goes through one shared helper that writes a private temp file in the same directory, syncs it and persists it into place.
@row_add n=67 row=decision/pinned-viola-never-replaced area=authentication-authorization rec=42
name: Pinned viola is never replaced
rule: The pinned viola is never replaced: an absent copy is written to a 0700 temp file, synced, closed and placed without clobbering, never over a present name.
@row_add n=68 row=decision/strict-modes-check area=authentication-authorization rec=42
name: Strict-modes check on trusted state
rule: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable or not owned by the effective user (ssh StrictModes style), pre-existing and later-modified homes included.
guard: security_negatives_strict_modes, chaos_snapshot_replay
@row_add n=69 row=decision/strict-modes-read-rule area=authentication-authorization rec=42
name: Strict-modes read rule
rule: The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission.
why: so a pre-existing home cannot expose events.ndjson or diagnostics/
@row_add n=70 row=decision/strict-refusal-leaves-no-trace area=authentication-authorization rec=42
name: A strict-modes refusal leaves no trace
rule: A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens.
why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
@row_add n=71 row=decision/unstattable-home-refused area=authentication-authorization rec=42
name: An unstattable home is refused
rule: An unstattable home (a path under a regular file) is a strict-modes refusal.
@row_add n=72 row=decision/whole-board-refusal area=authentication-authorization rec=42
name: viola list refuses the whole board
rule: viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board.
why: ratified
@row_add n=73 row=decision/board-home-read-rule area=authentication-authorization rec=43,42
name: The board holds the home to the read rule
rule: viola list refuses a home another user may read, with or without an instance.
@row_add n=74 row=decision/home-write-rule-checkers area=authentication-authorization rec=43
name: Budget read, mcp start and plugin install check the home
rule: The budget read, the start of viola mcp and viola plugin install hold the home to the write rule.
@row_add n=75 row=contract/strict-modes-refusal/run-revive area=authentication-authorization rec=42,44
boundary: strict-modes refusal
item: run and revive
promise: Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.
@row_add n=76 row=contract/strict-modes-refusal/list-json area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola list under --json
promise: Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
@row_add n=77 row=contract/strict-modes-refusal/list-human area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola list in human form
promise: Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
@row_add n=78 row=contract/strict-modes-refusal/link-unlink area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola link and viola unlink
promise: Exit 21 with the instance-state unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout; no log, no line, the mode as found.
@row_add n=79 row=contract/strict-modes-refusal/mcp-start area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola mcp start
promise: Exit 1 with nothing on stdout or stderr and no role file.
@row_add n=80 row=contract/strict-modes-refusal/plugin-install area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola plugin install
promise: Exit 21 (instance-unreachable under --json) with nothing created and no log opened.
@row_add n=81 row=contract/strict-modes-refusal/verify area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola verify
promise: Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.
@row_add n=82 row=contract/strict-modes-refusal/version-gate area=authentication-authorization rec=42
boundary: strict-modes refusal
item: viola run's version gate
promise: A refused stamps read is cli_verified:false with parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}.
@row_add n=83 row=contract/strict-refusal-message/readable area=authentication-authorization rec=42
boundary: strict-modes refusal message
item: Readable
promise: The read rule's refusal reads exactly "a trusted state path is readable by another user".
@row_add n=84 row=contract/strict-refusal-message/symlink area=authentication-authorization rec=42
boundary: strict-modes refusal message
item: Symlink
promise: A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".
@row_add n=85 row=decision/marketplace-off-integrity-set area=authentication-authorization rec=42
name: marketplace/ is not trusted state
rule: marketplace/ is no trusted file and is off the integrity set: viola never reads it back, and the installed command gets no re-hash.
@row_add n=86 row=decision/statusline-sources-untrusted-read area=authentication-authorization rec=42
name: Statusline sources read untrusted
rule: statusline-source.json and the user's ~/.claude/settings.json are not trusted files; their control is a read-only, capped, tolerant read.
why: a boundary widening that was ratified
@row_add n=87 row=decision/budget-gate-fails-open area=authentication-authorization rec=42
name: Budget gate reads an untrusted file, failing open
rule: The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file; a refused or unreadable file is no reading and the gate does not pause.
why: a ratified boundary widening: a file a hook arm writes decides a wrapper's refusal
@row_add n=88 row=decision/budget-file-single-writer area=authentication-authorization rec=42
name: hook statusline is the only budget writer
rule: hook statusline is the only writer of budget.json; viola list reads it to show the reading and decides nothing on it.
@row_none n=7 rec=44 type=4
relied: run and revive's start check the whole home tree, home first then instance, ledger, bin and plugin folders, before the wrapper log opens.
@row_none n=8 rec=45 type=4
relied: Reading the connected stream's peer pid writes and logs nothing.
@row_none n=9 rec=46 type=4
relied: A peer whose euid cannot be read is refused, and reading it writes and logs nothing.
@row_none n=10 rec=47 type=4
relied: Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a frame be written.
@row_add n=89 row=contract/instance-unreachable-detail/instance-dead area=authentication-authorization rec=48,51
boundary: instance-unreachable detail without a live endpoint
item: instance-dead
promise: Read when the instance directory is a directory; nothing is opened and no line written.
@row_add n=90 row=contract/instance-unreachable-detail/unwrapped-name area=authentication-authorization rec=48,51
boundary: instance-unreachable detail without a live endpoint
item: unwrapped-name
promise: Read when the instance path is absent, a regular file or a symlink; nothing is opened and no line written.
@row_none n=11 rec=49 type=4
relied: link and unlink take the same order as every CLI frame verb: strict-modes check, liveness pre-check, then the socket-directory, euid and server checks before their one frame.
@row_add n=91 row=contract/server-verify-refusal/cli-json area=authentication-authorization rec=50
boundary: server-verification refusal
item: CLI frame verb under --json
promise: No frame; exit 21 with {"v":1,"error":"instance-unreachable","detail":"server-verify-failed"} on stdout and stderr empty; a refused peer euid reads the same.
@row_add n=92 row=contract/server-verify-refusal/cli-human area=authentication-authorization rec=50
boundary: server-verification refusal
item: CLI frame verb in human mode
promise: No frame; exit 21 with the unable <name> instance-unreachable line and the hint "the process on <name>'s endpoint is not its recorded wrapper; it was not contacted" last on stderr.
@row_add n=93 row=decision/client-exit-13-first area=authentication-authorization rec=50
name: Client's own refusals come first
rule: A CLI frame verb's own exit-13 refusals come first, then its strict-modes refusal, before its liveness and server checks.
@row_none n=12 rec=52 type=5
@row_add n=94 row=decision/no-seam-for-foreign-peer area=authentication-authorization rec=53
name: No seam to drive a foreign peer
rule: A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them.
why: one account can make no foreign peer
@row_none n=13 rec=54 type=5
@row_add n=95 row=decision/checks-at-seams-not-channel-client area=authentication-authorization rec=55
name: Security checks sit at the seams, not the channel client
rule: No check sits inside the channel crate's client connect; the checks sit at the seams, and a test may write through that client past every check by design.
why: viola-channel has no viola-state edge
