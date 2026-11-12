@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=5
@row_add n=1 row=decision/run-archive-never-uploaded area=occupied-resources rec=9
name: Per-run archive never uploaded
rule: The per-run archive of JUnit files and mutation outcomes stays local, outside the agent-run artifacts, and is never uploaded.
why: The mutation outcomes file carries absolute argv paths.
@row_none n=9 rec=10 type=5
@row_none n=10 rec=11 type=5
@row_none n=11 rec=12 type=S
@row_add n=2 row=decision/config-precedence area=cross-cutting-patterns rec=13
name: Configuration precedence
rule: CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
@row_add n=3 row=decision/viola-home-resolution area=cross-cutting-patterns rec=13
name: Viola home resolution order
rule: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home.
@row_add n=4 row=decision/no-env-config-channel area=cross-cutting-patterns rec=13
name: Environment variables are no configuration channel
rule: Environment variables are not a configuration channel; a dependency or feature that reads one breaks this rule like a direct read does.
@row_add n=5 row=absence/secrets area=cross-cutting-patterns rec=13
capability: Secrets of any kind in v1 configuration
@row_add n=6 row=decision/agent-driven-development area=cross-cutting-patterns rec=14,42
name: Agent-driven development style
rule: The project's development style is agent-driven.
@row_add n=7 row=decision/identity-by-instance area=cross-cutting-patterns rec=15
name: Identity by instance, never by location
rule: Every hook, MCP call and CLI verb finds its instance through VIOLA_NAME or an explicit ViolaName argument, never through the working directory, so two sessions in one folder never share records.
@row_add n=8 row=decision/hook-fails-open area=cross-cutting-patterns rec=16,36
name: Hook fails open toward the human
rule: Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
@row_add n=9 row=decision/unconfirmed-delivery-not-delivered area=cross-cutting-patterns rec=16
name: Unconfirmed delivery reported as not-delivered
rule: Any failure to confirm delivery is reported as not-delivered and never retried blindly.
except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.
@row_add n=10 row=decision/mixed-version-tolerance area=cross-cutting-patterns rec=17,37
name: Mixed-version tolerance
rule: Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
@row_add n=11 row=contract/channel-params/v area=cross-cutting-patterns rec=17,34
boundary: Channel params
item: v
promise: Every channel params object carries the format version v.
@row_add n=12 row=contract/channel-params/sender-version area=cross-cutting-patterns rec=17,34
boundary: Channel params
item: sender version
promise: Every channel params object carries the sender's CARGO_PKG_VERSION.
@row_add n=13 row=contract/channel-errors/32602-newer-request area=cross-cutting-patterns rec=17
boundary: Channel errors
item: -32602 for a newer request
promise: A request newer than the wrapper supports gets a -32602 error naming the wrapper's version.
@row_add n=14 row=decision/capability-ledger-gate area=cross-cutting-patterns rec=18,29,39
name: Capability ledger as the single gate for CLI behaviour
rule: Any dependence on an undocumented claude behaviour is a capability-ledger row in viola-agent-claude with a viola verify probe that checks a post-condition, never an unconditional hard-coded assumption.
except: A CLI shape no send relies on needs no probe.
why: A ruled limit sets the exception.
@row_add n=15 row=decision/tokio-containment area=cross-cutting-patterns rec=19,32
name: Tokio containment
rule: Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
@row_add n=16 row=decision/crash-safe-disk-writes area=cross-cutting-patterns rec=20
name: Crash-safe disk writes
rule: Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
@row_add n=17 row=contract/hook-output/stdout area=cross-cutting-patterns rec=21
boundary: Hook output
item: stdout and stderr
promise: viola hook writes only its decision body to stdout and nothing to stderr.
@row_add n=18 row=contract/hook-statusline-output/stdout area=cross-cutting-patterns rec=21
boundary: Hook statusline output
item: stdout
promise: The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.
@row_add n=19 row=decision/run-terminal-child-only area=cross-cutting-patterns rec=21
name: Run writes only the child's output to the terminal
rule: While the child runs, viola run writes nothing to the terminal except the child's own output.
why: The terminal carries the child's screen.
@row_add n=20 row=decision/codes-only-process-logs area=cross-cutting-patterns rec=21
name: Codes-only process logs per role
rule: Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
@row_add n=21 row=decision/content-detail-per-instance area=cross-cutting-patterns rec=21
name: Content-bearing detail only in the instance
rule: Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
@row_add n=22 row=contract/mcp-output/stdout area=cross-cutting-patterns rec=21
boundary: MCP server output
item: stdout and stderr
promise: viola mcp writes only MCP frames to stdout and logs to its diagnostics file; it writes JSON to stderr only when that file cannot be opened.
@row_add n=23 row=decision/cli-verbs-human-stderr area=cross-cutting-patterns rec=21
name: Short-lived CLI verbs may use stderr
rule: Short-lived CLI verbs may use stderr for their human output.
@row_add n=24 row=decision/plugin-paths-forward-slashes area=cross-cutting-patterns rec=22
name: Forward slashes in plugin paths
rule: Every path written into plugin files uses forward slashes.
@row_add n=25 row=decision/no-shell-out area=cross-cutting-patterns rec=22
name: No shelling out
rule: Nothing shells out through sh or bash, and child processes are spawned directly.
except: hook statusline runs the user's own statusline command.
@row_add n=26 row=decision/hashed-short-endpoint-names area=cross-cutting-patterns rec=22
name: Hashed short endpoint names
rule: Endpoint names are hashed and short.
@row_add n=27 row=decision/linux-only area=cross-cutting-patterns rec=22,31
name: Linux only
rule: The product builds, is tested and ships on Linux only in v1; the Windows and macOS code is not carried.
why: Ratified ruling; the removed code stays recoverable at a tag.
@row_add n=28 row=decision/os-gated-tests-name-reason area=cross-cutting-patterns rec=22
name: OS-gated tests name their reason
rule: Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=29 row=decision/ci-linux-fake-agent area=cross-cutting-patterns rec=22,41
name: CI on Linux against the fake agent
rule: CI runs on GitHub Actions ubuntu-latest against the fake agent.
@row_add n=30 row=decision/untrusted-upstream-text area=cross-cutting-patterns rec=23
name: Upstream text is untrusted
rule: A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
@row_add n=31 row=decision/bounded-readers area=cross-cutting-patterns rec=24
name: Bounded readers
rule: Every product reader of external bytes is bounded in its own statement or named as an exception.
except: The own-executable reads.
guard: contract_bounded_readers
@row_add n=32 row=decision/same-user-endpoint-trust area=cross-cutting-patterns rec=25
name: Local endpoint trusts the same OS user only
rule: v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
@row_add n=33 row=overview/project-intent area=project-intent rec=26,27,28
text: viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary, a Claude Code plugin of hooks and a stdio MCP server that calls that binary, and a keyboard-driven terminal fleet console. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.
@row_add n=34 row=absence/web-or-graphical-frontend area=project-intent rec=26
capability: A web or graphical frontend
@row_add n=35 row=decision/wrap-unmodified-claude area=project-intent rec=27
name: Wrap the unmodified claude CLI
rule: viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.
@row_add n=36 row=decision/modular-monolith-no-daemon area=project-intent rec=28
name: Modular monolith, no daemon
rule: The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
@row_add n=37 row=decision/new-capabilities-as-verbs area=project-intent rec=29
name: New capabilities as subcommands or channel verbs
rule: New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.
@row_add n=38 row=decision/second-agent-sibling-crate area=project-intent rec=29
name: A second driven agent as a sibling crate
rule: A second driven agent arrives as a sibling viola-agent crate, at which point the adapter trait is extracted into viola-core.
@row_none n=12 rec=30 type=0
what: a statement that the repository is a single product with no template patterns
@row_add n=39 row=decision/rust-toolchain-pinned area=inherited-defaults rec=32
name: Pinned Rust toolchain
rule: The Rust toolchain is pinned by rust-toolchain.toml, with a declared MSRV and edition 2024.
@row_add n=40 row=decision/rmcp-for-mcp area=inherited-defaults rec=32
name: rmcp serves MCP
rule: rmcp is what serves MCP.
@row_add n=41 row=decision/clap-for-cli area=inherited-defaults rec=32
name: clap parses the CLI
rule: clap is what parses the CLI.
@row_add n=42 row=absence/database area=inherited-defaults rec=33
capability: A database
@row_add n=43 row=decision/ndjson-logs-and-json-snapshots area=inherited-defaults rec=33
name: State in ndjson logs and JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
@row_add n=44 row=decision/one-shared-persist-helper area=inherited-defaults rec=33
name: Snapshots persisted through one shared helper
rule: Atomic JSON snapshots are written with tempfile persist through one shared helper.
@row_add n=45 row=decision/interprocess-local-sockets area=inherited-defaults rec=34
name: interprocess local sockets for IPC
rule: IPC uses interprocess local sockets, one endpoint per viola run.
@row_add n=46 row=contract/channel/framing area=inherited-defaults rec=34
boundary: Channel
item: framing
promise: The channel speaks JSON-RPC 2.0 over ndjson.
@row_add n=47 row=contract/channel-result/refusal area=inherited-defaults rec=35
boundary: Channel result
item: refusal
promise: A refusal is a typed RefusalReason in result.refusal.
@row_add n=48 row=decision/thiserror-one-enum-per-crate area=inherited-defaults rec=36
name: thiserror, one error enum per crate
rule: Each crate has one thiserror error enum.
except: viola-pty's hand-written PtyError; viola-mcp's ServeError beside McpError; viola-state's StampsError beside StateError, PinError, StampsReadError and Refused.
why: The exceptions are ratified.
@row_add n=49 row=decision/anyhow-only-root-bin area=inherited-defaults rec=36
name: anyhow only in the root binary
rule: anyhow is used only in the root binary; CLI exit codes are typed.
@row_add n=50 row=decision/tolerant-serde-external area=inherited-defaults rec=37
name: Tolerant serde for external payloads
rule: External payloads are deserialised tolerantly.
@row_add n=51 row=decision/nutype-newtypes area=inherited-defaults rec=37
name: nutype newtypes for validation
rule: Validated values are nutype newtypes.
@row_add n=52 row=contract/timestamps/format area=inherited-defaults rec=38
boundary: Timestamps
item: format
promise: Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
@row_add n=53 row=decision/claude-shapes-isolated area=inherited-defaults rec=39
name: Claude-specific shapes isolated
rule: Claude-specific shapes live only in viola-agent-claude.
@row_add n=54 row=decision/local-cargo-install area=inherited-defaults rec=40
name: Local install with embedded plugin
rule: viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.
@row_add n=55 row=absence/containers-and-hosting area=inherited-defaults rec=40
capability: Containers or hosting of any kind
@row_add n=56 row=decision/no-crates-io-publish area=inherited-defaults rec=43
name: Nothing published to crates.io
rule: Every workspace crate, the root bin included, sets publish false; nothing is published to crates.io in v1.
@row_add n=57 row=decision/dual-mit-apache-license area=inherited-defaults rec=43
name: MIT OR Apache-2.0 license
rule: Every workspace crate inherits the license MIT OR Apache-2.0 from the workspace package.
@row_none n=13 rec=44 type=0
what: a note that the project had no existing scopes
@row_add n=58 row=contract/refusal-reason/human-typing area=conventions rec=45
boundary: RefusalReason
item: human-typing
promise: Serialised kebab-case; its detail is null or manual-pause.
@row_add n=59 row=contract/refusal-reason/budget-paused area=conventions rec=45
boundary: RefusalReason
item: budget-paused
promise: Serialised kebab-case; its detail is five-hour or seven-day.
@row_add n=60 row=contract/refusal-reason/unverified-cli area=conventions rec=45
boundary: RefusalReason
item: unverified-cli
promise: Serialised kebab-case; its detail is null.
@row_add n=61 row=contract/refusal-reason/not-delivered area=conventions rec=45
boundary: RefusalReason
item: not-delivered
promise: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_add n=62 row=contract/refusal-reason/unknown-catch-all area=conventions rec=45
boundary: RefusalReason
item: unknown catch-all
promise: The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
@row_add n=63 row=contract/refusal-reason/detail area=conventions rec=45
boundary: RefusalReason
item: detail
promise: A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
@row_add n=64 row=contract/refusal-order/first-only area=conventions rec=45
boundary: Refusal order
item: several refusals
promise: When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
@row_add n=65 row=contract/refusal-order/send area=conventions rec=45
boundary: Refusal order
item: send
promise: control-character, empty-text, human-typing, budget-paused, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
@row_add n=66 row=contract/refusal-order/answer area=conventions rec=45
boundary: Refusal order
item: answer
promise: control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.
@row_add n=67 row=contract/paste-text/control-characters area=conventions rec=45
boundary: Paste text
item: control characters
promise: LF, CR and TAB pass; every other C0, DEL and C1 is refused as not-delivered control-character, never stripped; the CLI checks before any frame and the wrapper checks again.
@row_add n=68 row=contract/paste-text/empty area=conventions rec=45
boundary: Paste text
item: empty text
promise: A text empty after CR LF and lone CR become LF and trailing CR and LF are dropped is refused as not-delivered empty-text, by the CLI and again by the wrapper; one also holding a refused character is control-character.
@row_add n=69 row=contract/send/paste-failure-after-issue area=conventions rec=45
boundary: send refusals
item: paste failing after send-issued
promise: A paste that fails after send-issued is refused as input-not-ready with that send's cursor.
@row_add n=70 row=contract/ok-payload/unconfirmable area=conventions rec=45
boundary: ok payload
item: unconfirmable
promise: unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured.
@row_add n=71 row=contract/code-word/format area=conventions rec=46
boundary: Code word
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_add n=72 row=contract/peer-reply/refusal area=conventions rec=47
boundary: A peer's reply
item: refusal
promise: A refusal that is no code word is read as unknown.
@row_add n=73 row=contract/peer-reply/detail area=conventions rec=47
boundary: A peer's reply
item: detail
promise: A string detail that is no code word is read as null; a non-string detail makes the reply no known reply.
@row_add n=74 row=contract/unreachable-cause/cause-set area=conventions rec=48
boundary: Unreachable cause
item: cause set
promise: Exit 21 has a closed cause set (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused) written by both clients, with no serde path; it is no RefusalReason.
@row_add n=75 row=contract/cli-exit-codes/0 area=conventions rec=49
boundary: CLI exit codes
item: 0
promise: ok.
@row_add n=76 row=contract/cli-exit-codes/1 area=conventions rec=49,51,52,53
boundary: CLI exit codes
item: 1
promise: Internal error, and the start or preflight refusals of run, revive and verify, each an unable and hint pair on stderr with no path, or under --json one refusal document; verify's failing ledger row and interruption too.
@row_add n=77 row=contract/cli-exit-codes/2 area=conventions rec=49
boundary: CLI exit codes
item: 2
promise: Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
@row_add n=78 row=contract/cli-exit-codes/10 area=conventions rec=49
boundary: CLI exit codes
item: 10
promise: human-typing refusal.
@row_add n=79 row=contract/cli-exit-codes/11 area=conventions rec=49
boundary: CLI exit codes
item: 11
promise: budget-paused refusal.
@row_add n=80 row=contract/cli-exit-codes/12 area=conventions rec=49
boundary: CLI exit codes
item: 12
promise: unverified-cli refusal.
@row_add n=81 row=contract/cli-exit-codes/13 area=conventions rec=49
boundary: CLI exit codes
item: 13
promise: not-delivered refusal.
@row_add n=82 row=contract/cli-exit-codes/14 area=conventions rec=49
boundary: CLI exit codes
item: 14
promise: unknown refusal.
@row_add n=83 row=contract/cli-exit-codes/20 area=conventions rec=49
boundary: CLI exit codes
item: 20
promise: Protocol fault from the wrapper, a -32602 from link or unlink included; the link self-pair is refused by the wrapper only, never client-side.
@row_add n=84 row=contract/cli-exit-codes/21 area=conventions rec=49,55
boundary: CLI exit codes
item: 21
promise: Instance unreachable: no live endpoint, a server that is not the recorded wrapper, or a refused strict-modes check of the board, the plugin install tree or the target instance; under --json the instance-unreachable document with its cause as detail.
@row_add n=85 row=contract/cli-json-error-document/shape area=conventions rec=49
boundary: CLI --json error document
item: shape
promise: One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
@row_add n=86 row=contract/cli-internal-error/output area=conventions rec=49
boundary: CLI internal error output
item: internal error line
promise: A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
@row_none n=14 rec=50 type=4
relied: run, revive's start, mcp and hook stay silent at the internal-error catch site.
@row_add n=87 row=contract/run-start-refusals/socket-dir-refused area=conventions rec=51
boundary: viola run and revive start refusals
item: socket-dir-refused
promise: A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
@row_add n=88 row=contract/run-start-refusals/home-tree area=conventions rec=52
boundary: viola run and revive start refusals
item: home tree
promise: A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
@row_add n=89 row=contract/verify-refusals/home-tree area=conventions rec=53
boundary: viola verify refusals
item: home tree
promise: A refused home tree, or a strict-modes refusal while measuring, exits 1 with the home-state unable and hint pair on stderr, and no step line, summary or log line.
@row_add n=90 row=contract/verify-process-exit/program-not-found area=conventions rec=54
boundary: viola verify process-exit detail
item: program-not-found
promise: The code verify's process-exit carries when the claude CLI is not found.
@row_add n=91 row=contract/verify-process-exit/pinned-hash-mismatch area=conventions rec=54
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: The code verify's process-exit carries when the pinned copy fails its re-hash.
@row_add n=92 row=contract/verify-process-exit/version-unreadable area=conventions rec=54
boundary: viola verify process-exit detail
item: version-unreadable
promise: The code verify's process-exit carries when the CLI version cannot be read.
@row_add n=93 row=contract/verify-process-exit/strict-modes-failed area=conventions rec=54
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: The code verify's process-exit carries on a strict-modes refusal.
@row_add n=94 row=contract/verify-process-exit/unclean-fixture area=conventions rec=54
boundary: viola verify process-exit detail
item: unclean-fixture
promise: The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
@row_add n=95 row=contract/verify-process-exit/interrupted area=conventions rec=54
boundary: viola verify process-exit detail
item: interrupted
promise: The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
@row_add n=96 row=contract/verify/no-json area=conventions rec=54
boundary: viola verify
item: --json
promise: viola verify takes no --json.
@row_add n=97 row=contract/client-server-verify/server-verify-failed area=conventions rec=55
boundary: Client server verification
item: server-verify-failed
promise: A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
@row_add n=98 row=contract/client-server-verify/check-order area=conventions rec=55
boundary: Client server verification
item: check order
promise: The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
