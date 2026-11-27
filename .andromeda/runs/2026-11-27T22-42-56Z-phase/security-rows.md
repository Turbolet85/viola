# security-plan · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 101 of 667 (60 by every draw) · rule any · pin 5179

## threat-model-summary · ## Threat Model Summary
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot, as the key of the start's trust record in the Claude config, and as the working directory on viola up's terminal argv, never on a log line, error body or stdout.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/run-pins-own-folder-first-on-path · Pinned folder first on the child's PATH — viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
decision/socket-in-owner-only-dir · Socket in a per-user owner-only directory — The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
decision/socket-in-per-user-dir · Sockets in a per-user directory — Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, as the key of its trust record in the Claude config, and as the working directory on viola up's terminal argv, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start, adopt's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
contract/cli-exit-codes/21 · CLI exit codes · 21 — A frame verb, or viola focus, refused because the instance failed the strict-modes check, or viola up refused by its board scan's strict-modes check.
decision/child-path-trust · Child processes trust PATH — Spawned child processes are whatever the calling process's PATH resolves to.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/switch-method-behind-ipc-controls · Switch method mapped to the IPC controls — The wrapper's switch method, which ends and resumes the driven child, is served only behind the IPC controls (socket-directory, peer euid and server checks), re-checks both halves in the wrapper and reads no from. · why: A new channel method needs its own entry mapping it to the IPC controls.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/wrapper-channel-methods/switch · wrapper channel methods · switch — Switches the driven session to a given model and effort: params {v, sender, model?, effort?}; replies ok with model and effort, or a refusal: human-typing, not-delivered turn-running, dialog-pending, no-session, child-not-ended or resume-failed, or unverified-cli; from is not read.

## authentication-authorization · ## Authentication & Authorization
decision/switch-ends-only-its-own-child · Switch ends only the wrapper's own child — A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.
decision/up-checks-before-log-and-spawn · viola up checks before its log and before a spawn — viola up runs the strict board scan before its log opens, and holds the home tree to the strict-modes check and re-hashes the pinned copy before its first spawn.
decision/window-step-moves-own-terminal-window · Wrapper's window step moves only its own terminal's window — The wrapper's window step moves only the window of its own terminal, found by its parent pid within a 5 s wait, at most once, and never a window it did not open; it switches back once to the workspace active before it only when the move changed it; a failed read or restore changes no placement. · guard: window_step_moves_the_window_of_its_terminal_to_the_place
contract/strict-modes-refusal/up · strict-modes refusal · viola up — The board scan's refusal exits 21 with no line and nothing spawned, under --json the instance-unreachable document with detail strict-modes-failed.
decision/adopt-signals-no-foreign-process · Adopt acts on no process it did not start — viola adopt resumes only a session no live claude holds, spawns its own child in its own terminal, and never ends or signals a process viola did not start. · why: A plain session still running elsewhere is refused rather than taken over.
decision/socket-dir-checked · Socket directory checked — The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
decision/start-checks-home-tree-before-log-opens · Start checks the home tree before its log opens — The start of run, revive and adopt checks the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child. · why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
decision/start-checks-socket-dir-before-bind · Socket directory checked before any bind, snapshot or child — A start, run, revive or adopt alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict. · why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-before-snapshot-read · Strict-modes check before the snapshot is read — Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification. · why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/switch-refusal-order · Switch refusal order — A switch is refused in a fixed order, the first rung winning: human typing, a running turn, a pending dialog, an unverified CLI, then no session; no refusal appends a line or ends the child.
contract/socket-dir-refusal/run-and-revive-start · socket-directory refusal · run and the start of revive — A failed create and a refused check read one code: exit 1 with socket-dir-refused and one process-exit line of that detail; a Server::bind failure is outside this code.
contract/strict-modes-refusal/adopt · strict-modes refusal · viola adopt — Refused before any log with no child started and the home or instance mode left as found: the instance-state unable:/hint: pair on stderr, or under --json the strict-modes-failed refusal code.
contract/strict-modes-refusal/run-revive · strict-modes refusal · run and revive — Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/launch-pair-argv-crossing · Launch pair handed on the claude child's argv — The launch pair crosses onto the claude child's command line only as four separate argv elements, --model and its word then --effort and its word, of a direct spawn, never through a shell, after the plugin-dir flag and before the caller's words. · why: A ratified crossing; revive handing the recorded pair onto the resumed child's argv is a second ratified crossing.
decision/open-terminal-direct-spawn · viola open spawns its terminal directly — viola open and viola up spawn the terminal program resolved on the calling process's PATH directly, never through a shell, with a closed argv of one element per value; open starts in it only the pinned viola run with claude, up only the pinned viola revive <name> --origin in the recorded cwd. · guard: open_argv_is_the_closed_argv_one_element_each; open_spawns_the_terminal_with_the_closed_argv
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/revive-cwd-spawn-only · Recorded cwd used only to spawn — The recorded working directory of a revive is read after the strict-modes check and used only as the spawn directory and as the key of the start's trust record.
decision/revive-recorded-cwd-spawn-dir · revive spawns in the recorded cwd — viola revive spawns claude in the snapshot's recorded cwd, read only after the strict-modes check, required to exist, and used only as the spawn directory and as the key of the start's trust record; program resolution and the version gate never read it. · why: A ratified boundary widening.
decision/run-window-flags-checked · viola run's hidden window flags checked — viola run's hidden --origin takes only the closed words cli or agent and --place only an integer, and --place without --origin is a usage error; viola revive's hidden --origin takes the same two words, conflicts with --list and asks a window only at the workspace a real snapshot recorded. · why: A boundary widening of the started run's argv. · guard: run_args_refuse_a_place_without_an_origin; run_args_take_a_hidden_origin_and_place
decision/session-id-argv-closed-shape · Session id on child argv — A session id placed on a child's command line has the closed shape (36 characters, hyphens at 8, 13, 18, 23, hex elsewhere) and is one argv element of a direct spawn, never a shell, built only by one resume-args builder. · why: A ratified crossing of a logged value onto a child's command line. · except: viola adopt places an operator-typed id there too, not a logged one, in the same closed shape.
decision/session-id-argv-no-shell · Session id as one argv element — The session id after --resume has the closed session-id shape and is one argv element of a direct spawn, never passed through a shell.
decision/trust-record-closed-words · Trust record outcomes are closed code words — A trust record write or removal ends in one of 14 closed code words, a start's trust step in one of 16 start words; verify's word alone is stamped as the trust-record row's value when nothing was written, a start's is its trust-record line's detail, never a path, key or value.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/revive-strict-modes-before-use · revive checks strict modes before use — Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
decision/run-resolves-absolute-program · run resolves its child program itself — viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
contract/revive-json/internal-error · viola revive --json codes · internal-error — A closed exit-1 code, printed for an internal error of the start path, which is still logged with its chain; a child-pump error prints no document.

## data-protection · ## Data Protection
decision/launch-pair-kept-to-snapshot-and-event-log · Launch pair kept to the snapshot and the event log — An instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings; log-only with no span, process-log line or terminal byte, it leaves only through list --json and the MCP list item.
decision/no-handle-inheritance · No channel handle inheritance — Channel handles never leak into the claude child: every inherited descriptor is closed in the child.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/start-trust-skips-trusted-folders · Start skips the home folder and trusted folders — The start writes no trust record for the user home folder, nor for a folder the claude CLI's own walk reads as trusted by its own key or by a parent up to the git root.
decision/start-writes-trust-record-flag-only · Start writes the trust record, flag only — On a verified CLI only, the start of run, revive and adopt writes the claude CLI's trust record for an untrusted spawn folder into the config its child reads, setting hasTrustDialogAccepted alone with every other key kept, or a one-key entry where none stands; unlike verify's, the record persists. · why: So no trust dialog stands and no key is typed. Ratified with its price: a human's earlier no for the folder is overridden, and another session's write lost in the lock window is accepted.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.
decision/window-record-kept-to-snapshot-and-event-log · Window record kept to the snapshot and the event log — An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item; no address, pid or workspace enters a process-log line.
decision/hyprland-reply-text-dropped · Hyprland reply text dropped at the parse — Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error. · guard: contract_hyprland_canary_reaches_no_line_and_no_error
decision/open-ancestor-walk-in-memory · Open's ancestor walk held in memory — viola open's parent-window lookup walks at most 8 ancestor pids, held in memory only and never recorded. · why: A boundary widening of what viola open reads.
decision/trust-record-writer-guarded · Trust record writer guarded — The one trust record writer, verify's and the start's, never creates an absent config, refuses a link, a non-regular file, another owner's file, an oversize or malformed document and, for verify, a present key, locks only when free, tries three times, keeps the mode and touches no other key.

## dependency-security · ## Dependency Security
decision/g2-single-panic-exemption · G2 zero-panics gate has one exemption — CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry. · guard: G2 --probe run before every G2 check
decision/g2-zero-panics-single-exemption · Zero-panics gate with one exemption — CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry. · guard: G2 --probe · except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/ci-cargo-deny-gate · cargo-deny gate in CI — CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory not on its reasoned ignore list, any yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/dependency-audit-tools · cargo-deny and zizmor as audit tools — Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.

## secret-management · ## Secret Management
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/credential-strip-by-name-only · Credential strip reads names only — The environment strip handles variable names only and never reads a variable's value. · except: viola verify and the start of run, revive and adopt read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/messaging-credentials-are-secret · Messaging token and socket are secrets — The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release, list's colour facts, XDG_RUNTIME_DIR for the socket directory, and CLAUDE_CONFIG_DIR's value for verify and the start of run, revive and adopt when kept
decision/r8-claude-env-strip · R8 CLAUDE environment strip — Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values. · why: Ratified. · except: verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps it, to choose the Claude config of the trust record; else the user HOME's .claude.json.
contract/claude-child-process-start/env-kept · process-start claude-child line · env_kept — Names the kept environment names, never a value.
contract/claude-child-process-start/env-stripped-known · process-start claude-child line · env_stripped_known — Names the stripped environment names, never a value.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/non-identity-hook-names · Four hook names are not identity names — CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA, names the CLI adds to a hook, are known and not identity names: they stand off the identity floor and are read only by verify's env reading. · why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.

## error-handling · ## Error Handling
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start, adopt's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/run-refuses-socket-dir-before-start · run refuses a bad socket directory first — viola run, revive's start and adopt's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/vt100-inside-catch-unwind · vt100 only inside catch_unwind — vt100 is never fed outside catch_unwind. · why: a parser panic must not take down the human's terminal path
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
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
