@row_none n=1 rec=fct-2nehsyko@1 type=S
@row_none n=2 rec=fct-y7k647gj@1 type=0
what: an editing note on when the document may be changed
@row_add n=1 row=overview/security-plan-viola area=security-plan-viola rec=fct-odkkhtym@1
text: viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract, with no public network surface. Its IPC, filesystem and PTY controls stand in for the auth, database and API concerns of a web stack.
@row_none n=3 rec=fct-hj3kz4j6@1 type=S
@row_none n=4 rec=fct-huufiwqz@1 type=S
@row_add n=2 row=decision/security-tier area=threat-model-summary rec=fct-55ydrrgu@2
name: Security tier
rule: The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home.
why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
@row_none n=5 rec=fct-zfyhqz3p@1 type=S
@row_add n=3 row=decision/data-classes area=threat-model-summary rec=fct-xhdfgari@1
name: Data classes
rule: viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
@row_add n=4 row=decision/cwd-only-in-snapshot area=threat-model-summary rec=fct-l2iemwjy@1
name: Recorded cwd kept to the snapshot
rule: The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
@row_add n=5 row=absence/regulated-personal-data area=threat-model-summary rec=fct-fqtuwzzv@1,fct-2xvd2675@1,fct-37wlu7mu@1,fct-f4oo2w3w@1,fct-ruqn4gwy@1
capability: Payment, health, children's or multi-user personal data of any kind
@row_none n=6 rec=fct-odek45td@1 type=S
@row_add n=6 row=decision/os-user-ambient-trust area=threat-model-summary rec=fct-qtqta3hw@1,fct-s6dcdjs6@2,fct-qkuk7pih@2,fct-34c4k36s@1
name: OS-user ambient trust
rule: The only principal is the local OS user: the IPC endpoints and the viola home trust every process of the same OS user and no other, with no per-connection authentication.
why: viola is a single-user local tool with no accounts or hosting, driven by agents with no interactive login, which matches an ambient OS-user trust model rather than a credential flow.
@row_add n=7 row=decision/from-field-authenticates-nothing area=threat-model-summary rec=fct-qtqta3hw@1
name: Self-reported from authenticates nothing
rule: The self-reported from field on the wrapper channel authenticates nothing and is never treated as an identity.
@row_none n=7 rec=fct-cw3l7aky@1 type=4
relied: The send text is written into the PTY as one bracketed paste, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes.
@row_add n=8 row=decision/human-typing-takes-wheel area=threat-model-summary rec=fct-hscavt6w@1
name: Human typing takes the wheel
rule: A byte on viola run's own stdin that is typing takes the wheel from the driver.
except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
@row_add n=9 row=decision/non-editing-list-measured-only area=threat-model-summary rec=fct-hscavt6w@1
name: Non-editing list holds measured replies only
rule: A terminal reply is added to the stdin classifier's non-editing list only once a terminal has been measured to write it, never on a guess.
@row_add n=10 row=decision/hook-payload-is-content area=threat-model-summary rec=fct-3ajwacjp@1
name: Hook payload is content
rule: Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
@row_add n=11 row=decision/statusline-command-integrity area=threat-model-summary rec=fct-3ajwacjp@1
name: Statusline command integrity
rule: The hook statusline runs statusline_command through a shell, its integrity resting on filesystem permissions of the viola home.
@row_add n=12 row=decision/mcp-input-checked area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP input checked by schema
rule: MCP tool input is checked by the tool input schemas and the ViolaName newtype.
@row_add n=13 row=decision/mcp-frame-cap area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP stdin line cap
rule: Each MCP stdin line is capped at MAX_FRAME before rmcp reads it; an over-long line is refused whole and ends the session, never dispatched in part.
@row_add n=14 row=decision/driver-permission-policy-outside-viola area=threat-model-summary rec=fct-kfezk4o5@1,fct-ty6c2nnj@1
name: Driver prompt injection accepted
rule: The driver model reads untrusted driven-session output and may be prompt-injected into answering a PermissionRequest; what to answer rests with the driver, outside viola.
why: What to answer is policy outside viola; an accepted risk.
@row_add n=15 row=decision/mcp-tool-list-affordance area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_add n=16 row=contract/cli-state-root/home-flag area=threat-model-summary rec=fct-zxlwdggu@1
boundary: CLI state root selection
item: --home
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_add n=17 row=contract/cli-state-root/viola-dir area=threat-model-summary rec=fct-zxlwdggu@1
boundary: CLI state root selection
item: VIOLA_DIR
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_add n=18 row=decision/cli-input-trust-boundary area=threat-model-summary rec=fct-zxlwdggu@1
name: CLI input trust boundary
rule: CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
@row_add n=19 row=decision/strict-modes-at-start area=threat-model-summary rec=fct-2i3lkhzs@2
name: Strict-modes check at start
rule: Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
@row_add n=20 row=decision/child-path-trust area=threat-model-summary rec=fct-m3uflr5q@1
name: Child processes trust PATH
rule: Spawned child processes are whatever the calling process's PATH resolves to.
@row_add n=21 row=decision/child-output-tolerant area=threat-model-summary rec=fct-m3uflr5q@1
name: Child output parsed tolerantly
rule: Output of spawned children is parsed tolerantly and a failure becomes unknown.
@row_add n=22 row=decision/verify-signature-only area=threat-model-summary rec=fct-m3uflr5q@1
name: Verify reads signatures only
rule: viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
@row_add n=23 row=decision/verify-never-answers-dialogs area=threat-model-summary rec=fct-uhjzl3zt@1
name: Verify never answers dialogs
rule: viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
@row_add n=24 row=decision/agents-probe-log-redaction area=threat-model-summary rec=fct-uhjzl3zt@1
name: Agents-probe logging redacted
rule: Each agents-probe spawn pair is logged with no row text, id, cwd or path.
@row_add n=25 row=decision/sync-crate-tokio-ban area=threat-model-summary rec=fct-tm2e2x5g@1
name: No tokio in sync crates
rule: tokio is banned from the sync crates.
@row_add n=26 row=decision/no-publish area=threat-model-summary rec=fct-tm2e2x5g@1
name: Crates not publishable
rule: Crates carry publish = false and builds rely on the committed Cargo.lock.
@row_add n=27 row=decision/fuzz-lockfile-own-audit area=threat-model-summary rec=fct-tm2e2x5g@1
name: Fuzz lockfile audited on its own
rule: The test-only fuzz lockfile is audited on its own, apart from the main dependency audit.
why: A ratified exemption.
@row_add n=28 row=absence/release-and-deploy area=threat-model-summary rec=fct-tm2e2x5g@1,fct-ju2d5wlo@2
capability: A release, signing or deploy stage with release artefacts
@row_add n=29 row=absence/public-network-listener area=threat-model-summary rec=fct-k62evpgk@1,fct-odkkhtym@1
capability: A public network listener or public internet API
@row_add n=30 row=absence/websocket area=threat-model-summary rec=fct-k62evpgk@1
capability: WebSocket or polling endpoints
@row_add n=31 row=absence/hosting area=threat-model-summary rec=fct-k62evpgk@1,fct-nj4c3hrg@1
capability: Hosting or containerised deployment of any kind
@row_none n=8 rec=fct-c4sr2jmm@1 type=S
@row_add n=32 row=decision/auth-self-implemented area=threat-model-summary rec=fct-gck5qvs6@1
name: No auth framework
rule: Authentication is self-implemented on OS mechanisms, with no auth framework or identity SDK.
@row_add n=33 row=absence/remote-access area=threat-model-summary rec=fct-c4azjngh@1
capability: Remote or phone access to viola
@row_none n=9 rec=fct-4trrznr3@1 type=S
@row_add n=34 row=decision/local-cargo-install area=threat-model-summary rec=fct-nj4c3hrg@1
name: Local install from source
rule: viola is installed locally with cargo install --path . and runs only on the local machine.
@row_add n=35 row=decision/flat-file-state-store area=threat-model-summary rec=fct-y7373awj@1
name: Flat-file state store
rule: State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
@row_add n=36 row=decision/plaintext-state area=threat-model-summary rec=fct-y7373awj@1
name: Plaintext state
rule: Nothing in the state store is encrypted; all of it is plaintext.
why: Readability, by design.
@row_add n=37 row=decision/socket-in-per-user-dir area=threat-model-summary rec=fct-2jp5ovui@2
name: Sockets in a per-user directory
rule: Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
@row_add n=38 row=decision/mcp-over-stdio area=threat-model-summary rec=fct-2jp5ovui@2
name: MCP over stdio
rule: The MCP server talks over stdio only.
@row_add n=39 row=decision/ci-no-secrets-no-real-claude area=threat-model-summary rec=fct-ju2d5wlo@2
name: CI holds no secrets
rule: CI holds no secrets and never runs the real claude CLI, which runs only locally.
@row_add n=40 row=absence/third-party-personal-data area=threat-model-summary rec=fct-wmp6i3hm@1
capability: Collection of personal data from third parties
@row_none n=10 rec=fct-cajceia6@1 type=0
what: an assessment note that a later public distribution adds no compliance trigger
@row_none n=11 rec=fct-hc6nagha@1 type=0
what: a note on the tier's role for downstream skills
@row_none n=12 rec=fct-dewavww2@1 type=S
@row_none n=13 rec=fct-zup45xyd@1 type=S
@row_none n=14 rec=fct-6zsj4czx@1 type=S
@row_add n=41 row=decision/interprocess-peer-creds area=authentication-authorization rec=fct-7nb5glkx@2
name: interprocess peer credentials
rule: interprocess's peer credentials over SO_PEERCRED are what read the peer's pid and euid on both sides of the IPC channel.
@row_add n=42 row=decision/sysinfo-process-identity area=authentication-authorization rec=fct-7nb5glkx@2
name: sysinfo for process identity
rule: sysinfo is what reads a process's pid and start time for server verification.
@row_add n=43 row=decision/no-listener-mode-option area=authentication-authorization rec=fct-7nb5glkx@2
name: Socket mode set by chmod
rule: The bound Unix socket's mode is set with std chmod, and interprocess's listener mode option is not used.
@row_none n=15 rec=fct-hv64gfd5@1 type=0
what: an adapter note on the OS-principal model
@row_add n=44 row=absence/user-accounts-passwords area=authentication-authorization rec=fct-uy3rjgvj@1
capability: User accounts with passwords
@row_add n=45 row=decision/all-or-nothing-access area=authentication-authorization rec=fct-j6fyndcf@1
name: All-or-nothing access
rule: Any caller that passes the OS check gets the full surface, with no roles.
@row_add n=46 row=decision/socket-dir-checked area=authentication-authorization rec=fct-dthgjfbs@1
name: Socket directory checked
rule: The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
@row_add n=47 row=decision/socket-chmod-after-bind area=authentication-authorization rec=fct-dthgjfbs@1
name: Socket owner-only after bind
rule: The Unix socket is set to mode 0600 right after the bind.
@row_add n=48 row=decision/no-socket-overwrite area=authentication-authorization rec=fct-dthgjfbs@1
name: No socket overwrite
rule: Binding never overwrites an existing socket; try_overwrite stays off.
@row_add n=49 row=decision/server-peer-euid-check area=authentication-authorization rec=fct-unwy7exn@1
name: Server-side peer euid check
rule: On every accept, a peer whose euid differs from the server's is dropped before a frame is read.
@row_add n=50 row=decision/client-strict-modes-first area=authentication-authorization rec=fct-phspeqis@2
name: Client strict-modes check first
rule: Before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line.
@row_add n=51 row=decision/client-verifies-server area=authentication-authorization rec=fct-phspeqis@2
name: Client verifies the server
rule: The client verifies the connected peer's pid and that process's start time against its one snapshot read; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect.
@row_add n=52 row=decision/client-peer-euid-check area=authentication-authorization rec=fct-phspeqis@2
name: Client-side peer euid check
rule: Past the connect, the client admits only a server of its own euid and refuses an unreadable peer, sending no frame.
@row_add n=53 row=contract/ipc-client-refusals/socket-dir-refused area=authentication-authorization rec=fct-phspeqis@2
boundary: IPC client refusals
item: socket-dir-refused
promise: Reported when the recorded endpoint's socket directory fails its check before the connect; no frame is sent.
@row_add n=54 row=contract/ipc-client-refusals/server-verify-failed area=authentication-authorization rec=fct-phspeqis@2
boundary: IPC client refusals
item: server-verify-failed
promise: Reported when the connected peer is not of the client's euid or cannot be read; no frame is sent.
@row_add n=55 row=decision/driver-release-refused area=authentication-authorization rec=fct-cwpea3sr@1
name: Driver-originated release refused
rule: The wrapper refuses a release whose params carry a string from, before any budget is acted on.
why: An affordance-level guard against drivers handing the wheel back to automation, not enforcement, since a same-user process can leave out from.
@row_add n=56 row=contract/release-errors/release-from-driver area=authentication-authorization rec=fct-cwpea3sr@1
boundary: The release method's errors
item: release-from-driver
promise: A release carrying a string from is refused with code -32602 and reason release-from-driver.
@row_add n=57 row=decision/home-owner-only area=authentication-authorization rec=fct-jya3vgt6@2
name: Owner-only viola home
rule: The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
@row_add n=58 row=decision/strict-modes-before-read area=authentication-authorization rec=fct-jya3vgt6@2
name: Strict-modes check before trusted reads
rule: Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
@row_add n=59 row=decision/answer-behavior-closed-enum area=authentication-authorization rec=fct-ty6c2nnj@1
name: Closed answer behaviour
rule: An answer's behavior is a closed enum.
@row_add n=60 row=decision/unverified-cli-gate area=authentication-authorization rec=fct-ty6c2nnj@1
name: Unverified CLI gate
rule: No non-null answer decision is sent without a viola verify stamp for the child's CLI version.
@row_none n=16 rec=fct-bdsvaehh@1 type=S
@row_none n=17 rec=fct-2oemhsry@1 type=S
@row_add n=61 row=overview/input-validation area=input-validation rec=fct-ambk6d46@1
text: Viola validates untrusted data at every point where it enters. Two validation regimes apply: tolerant parsing for external payloads and versioned-tolerant parsing for viola's own formats, with the security floors holding in both. Shared validators live in one core crate, so the CLI, the MCP server and the wrapper all call the same function.
@row_add n=62 row=decision/shared-validators-in-core area=input-validation rec=fct-ambk6d46@1
name: Shared validators in the core crate
rule: Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one.
why: so the CLI, MCP and the wrapper apply one identical check
@row_add n=63 row=decision/paste-text-character-set area=input-validation rec=fct-bvljrxyp@1
name: Paste text character set
rule: Paste text allows LF, CR and TAB and refuses every other C0 control, DEL and every C1 character, checked over decoded characters.
@row_add n=64 row=decision/refused-text-never-stripped area=input-validation rec=fct-bvljrxyp@1
name: Refused text is refused whole
rule: A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
@row_add n=65 row=decision/wrapper-checks-authoritatively area=input-validation rec=fct-bvljrxyp@1,fct-2hvuwr4u@1
name: Wrapper re-checks text authoritatively
rule: Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
@row_add n=66 row=decision/dialog-free-text-character-set area=input-validation rec=fct-2hvuwr4u@1,fct-bvljrxyp@1
name: Dialog free text uses the paste text set
rule: Dialog free text (permission and plan messages, question answer values) is held to the same exact character set as paste text.
@row_add n=67 row=contract/answer-refusal/control-character area=input-validation rec=fct-2hvuwr4u@1
boundary: answer refusal words
item: control-character
promise: Dialog free text holding a refused character is refused with the word control-character before any frame is sent.
@row_add n=68 row=decision/channel-frame-cap-before-read area=input-validation rec=fct-vhivn34r@1
name: Channel frame cap before reading
rule: A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
@row_add n=69 row=decision/json-default-depth-limit area=input-validation rec=fct-vhivn34r@1
name: Default JSON depth limit
rule: Channel frames are parsed with serde_json's default depth limit, never with unbounded depth.
@row_add n=70 row=contract/channel-errors/32600 area=input-validation rec=fct-vhivn34r@1
boundary: channel error codes
item: -32600
promise: A frame over the MAX_FRAME cap is answered -32600 and the connection is closed.
@row_add n=71 row=contract/channel-errors/32602 area=input-validation rec=fct-vhivn34r@1
boundary: channel error codes
item: -32602
promise: A method whose params fail the wrapper's check is answered -32602.
@row_add n=72 row=decision/invalid-hook-frames-dropped area=input-validation rec=fct-vhivn34r@1
name: Invalid hook frames dropped
rule: An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
@row_add n=73 row=decision/hook-stdin-never-truncated area=input-validation rec=fct-u7v2p5mv@1
name: Hook stdin over-limit detected
rule: Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
@row_add n=74 row=decision/hook-security-fields-closed area=input-validation rec=fct-u7v2p5mv@1
name: Hook security fields in closed enums
rule: Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
@row_add n=75 row=decision/hooks-fail-open area=input-validation rec=fct-u7v2p5mv@1
name: Hooks fail open
rule: Every hook failure fails open: exit 0 with no body.
@row_add n=76 row=decision/capture-arms-isolated area=input-validation rec=fct-u7v2p5mv@1
name: Hidden capture arms isolated
rule: The hidden capture arms read no VIOLA_* variable, open no channel and always exit 0.
@row_add n=77 row=decision/mcp-handlers-check-raw-args area=input-validation rec=fct-gtu3y5ba@1
name: MCP handlers check raw arguments
rule: Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only.
why: rmcp does not enforce the schemars schemas
@row_add n=78 row=contract/mcp-errors/invalid-params area=input-validation rec=fct-gtu3y5ba@1
boundary: MCP tool errors
item: invalid-params
promise: A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
@row_add n=79 row=contract/mcp-stdin/parse-rejected area=input-validation rec=fct-bngndo2w@1
boundary: MCP stdin refusal log
item: parse-rejected
promise: An over-long MCP stdin line is recorded as one parse-rejected line holding no byte of it.
@row_add n=80 row=decision/names-through-validated-type area=input-validation rec=fct-nad3igah@1
name: Names through the validated name type
rule: Every instance name taken from CLI arguments passes the validated name constructor.
@row_add n=81 row=decision/home-strict-modes-first area=input-validation rec=fct-nad3igah@1,fct-brqp4paf@1
name: Home strict-modes check first
rule: The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
@row_add n=82 row=decision/stamps-checked-under-lock area=input-validation rec=fct-nad3igah@1
name: Stamps checked before and under lock
rule: The ledger stamps file is checked before the ledger directory is created and again under its lock.
@row_add n=83 row=decision/cli-input-checked-before-frame area=input-validation rec=fct-nad3igah@1
name: CLI input checked before any frame
rule: answer and send input is read under MAX_FRAME and parsed into closed enums or through the paste text validator before any frame is sent.
@row_add n=84 row=contract/revive-json/output-fields area=input-validation rec=fct-nad3igah@1
boundary: revive --json output
item: output fields
promise: revive --json prints codes, closed words and session-shaped ids only, never a path or pid.
@row_add n=85 row=decision/config-bad-value-handling area=input-validation rec=fct-wsshf6pn@1
name: Bad config values
rule: Configuration is parsed tolerantly under MAX_FRAME with its version checked; a bad value takes its key's default or is rejected whole, and is reported as parse-rejected.
@row_add n=86 row=decision/config-cannot-disable-controls area=input-validation rec=fct-wsshf6pn@1
name: Configuration cannot disable controls
rule: No configuration value switches the budget gate off or makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
@row_add n=87 row=decision/test-seams-compiled-out area=input-validation rec=fct-ykdescly@1,fct-bbnvgu6s@1
name: Test seams compiled out of release
rule: Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
@row_add n=88 row=decision/colour-env-only-compared area=input-validation rec=fct-hqz7q7dg@1
name: Terminal colour variables only compared
rule: NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
@row_add n=89 row=decision/state-readers-never-write area=input-validation rec=fct-brqp4paf@1
name: State file readers never write
rule: A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
@row_add n=90 row=decision/replayed-snapshot-inert area=input-validation rec=fct-brqp4paf@1
name: Replayed snapshot is inert
rule: A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid, start time or statusline command, so no frame is written and no command runs from it.
@row_add n=91 row=decision/failed-budget-read-no-reading area=input-validation rec=fct-brqp4paf@1
name: Failed budget read is no reading
rule: Every failed read of the budget file counts as no reading.
@row_add n=92 row=decision/tail-only-valid-instance-logs area=input-validation rec=fct-hdcm5acf@1
name: Tail only valid instance logs
rule: The notifier tails only an instance event log whose name passes the validated name check, under the canonical home, ignoring symlinks and names that fail parsing.
@row_add n=93 row=decision/child-output-bounded area=input-validation rec=fct-q7h4j7ft@1
name: Child output bounded and tolerant
rule: Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
@row_add n=94 row=decision/verify-reads-only-signatures area=input-validation rec=fct-q7h4j7ft@1
name: Verify reads PTY output by signatures
rule: verify's PTY output is read only against the compiled signature literals.
@row_add n=95 row=decision/unwrapped-name-never-target area=input-validation rec=fct-q7h4j7ft@1
name: Unwrapped agent names are no targets
rule: An unwrapped agent row's name is never a validated viola name and never a valid target.
@row_add n=96 row=decision/vt100-feed-isolated area=input-validation rec=fct-juszneez@1
name: Screen model feed isolated from passthrough
rule: PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
@row_add n=97 row=decision/claude-resolved-from-path area=input-validation rec=fct-sgmmo3ki@2
name: claude resolved from PATH
rule: The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded.
why: a same-user trust and accepted risk
@row_add n=98 row=decision/revive-cwd-spawn-only area=input-validation rec=fct-sgmmo3ki@2
name: Recorded cwd used only to spawn
rule: The recorded working directory of a revive is read after the strict-modes check and used only as the spawn directory.
@row_add n=99 row=decision/session-id-argv-no-shell area=input-validation rec=fct-pt33yw4f@1
name: Session id as one argv element
rule: The session id after --resume has the closed session-id shape and is one argv element of a direct spawn, never passed through a shell.
@row_add n=100 row=contract/revive-refusal/no-session area=input-validation rec=fct-pt33yw4f@1
boundary: revive refusal words
item: no-session
promise: An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.
@row_add n=101 row=decision/statusline-source-read-only area=input-validation rec=fct-h4ojdq3v@1
name: Statusline source read only
rule: A statusline command is taken only from a non-empty statusLine of type command with no NUL, read once and capped; viola writes neither source.
@row_add n=102 row=decision/no-statusline-command-in-logs area=input-validation rec=fct-2fgozo5r@2
name: Statusline command never logged
rule: No log line or span carries the statusline command.
@row_add n=103 row=decision/parser-fuzz-scope area=input-validation rec=fct-h4bkgare@1
name: Parser surfaces in fuzz scope
rule: Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser and the vt100 feed are in scope for fuzz and property coverage.
@row_add n=104 row=decision/max-frame-cap area=input-validation rec=fct-ydf4aeiw@1
name: One external read cap
rule: MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read.
except: viola's reads of its own executable are no external input
@row_add n=105 row=decision/max-frame-revisit-signal area=input-validation rec=fct-ydf4aeiw@1
name: Frame cap revisit signal
rule: A stamped largest hook payload within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_none n=18 rec=fct-bcbs2cpv@1 type=S
@row_none n=19 rec=fct-nllpai4s@1 type=S
@row_add n=106 row=decision/data-protection-minimal-tier area=data-protection rec=fct-cm4lvc5u@1
name: Data protection at Minimal tier, adapted
rule: Data protection follows the Minimal tier with file-permission controls kept in scope.
why: viola stores sensitive user content; HTTPS enforcement does not apply
@row_none n=20 rec=fct-rn7l7pww@1 type=S
@row_add n=107 row=decision/no-tls-local-transport area=data-protection rec=fct-6ikkebgm@2
name: No TLS on local transports
rule: No TLS is used: the IPC channel is local Unix sockets and MCP is stdio, so confidentiality in transit rests on the socket-directory and peer checks.
@row_none n=21 rec=fct-zo3abxl7@1 type=0
what: a not-applicable control line for HSTS
@row_none n=22 rec=fct-rgyiwtx7@1 type=0
what: a not-applicable control line for certificate pinning
@row_add n=108 row=decision/no-handle-inheritance area=data-protection rec=fct-p7rzex2i@3
name: No channel handle inheritance
rule: Channel handles never leak into the claude child: every inherited descriptor is closed in the child.
@row_none n=23 rec=fct-rwwvtlhb@1 type=S
@row_add n=109 row=decision/state-files-os-permissions area=data-protection rec=fct-7aw7emh6@2
name: State files protected by OS permissions
rule: State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
@row_add n=110 row=decision/settings-override-plain-path area=data-protection rec=fct-7aw7emh6@2
name: Settings override only for plain paths
rule: The settings override is written only for a pinned path of plain characters.
why: a shell runs the override unquoted
@row_add n=111 row=decision/plugin-files-rewritten-each-start area=data-protection rec=fct-7aw7emh6@2
name: Plugin files rewritten every start
rule: The plugin files and the settings override are rewritten at every start.
@row_add n=112 row=decision/pinned-binary-rehashed area=data-protection rec=fct-7aw7emh6@2
name: Pinned binary re-hashed before use
rule: The pinned viola binary is re-hashed before use.
@row_add n=113 row=decision/probe-artifacts-cleanup area=data-protection rec=fct-kgbhs4lx@1
name: Probe artifacts owner-only and removed
rule: Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing.
except: a kill or a crash leaves them to be removed by hand
@row_add n=114 row=decision/logs-owner-only area=data-protection rec=fct-2vvustrg@1
name: Logs owner-only
rule: The event log and diagnostics files are 0600 inside 0700 directories.
@row_add n=115 row=decision/drift-reports-diagnostics-only area=data-protection rec=fct-2vvustrg@1
name: Drift reports only in diagnostics
rule: Deserialisation drift reports are written only to the diagnostics directory.
why: drift reports can quote tool input
@row_add n=116 row=decision/fixture-recording-scrubbed area=data-protection rec=fct-dg66rgro@1
name: Fixture recordings scrubbed
rule: verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
@row_add n=117 row=decision/fixtures-reviewed-before-commit area=data-protection rec=fct-dg66rgro@1
name: Fixtures reviewed before commit
rule: Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
@row_add n=118 row=decision/no-keys-at-rest area=data-protection rec=fct-7roiu3gz@1
name: No keys at rest
rule: Key management is omitted for the Minimal tier: there are no keys at rest.
@row_add n=119 row=decision/log-rotation-keeps-permissions area=data-protection rec=fct-7roiu3gz@1
name: Log rotation keeps permissions
rule: Any rotation scheme for the event log must keep 0600/0700 permissions and valid offsets.
@row_add n=120 row=decision/verify-trusted-runs-risk area=data-protection rec=fct-j3zmg7hp@1
name: Verify trusted runs accepted risk
rule: verify's trusted runs B, C and D are real interactive turns whose transcripts the CLI persists and for which the user's global hooks run.
why: accepted risk
@row_add n=121 row=decision/viola-writes-nothing-in-claude-dir area=data-protection rec=fct-j3zmg7hp@1,fct-vaineopj@1
name: viola writes nothing in Claude's config
rule: viola itself writes nothing under the user's Claude Code config directory or its global config file.
@row_add n=122 row=decision/scratch-footer-reading-residual area=data-protection rec=fct-f5gbcl67@1
name: Scratch footer reading is no standing permission
rule: A scratch footer reading of a newer CLI is one tmpfs-home run start, no key typed, only scrubbed footer rows kept; it is no product path, and another such reading needs its own answer.
why: accepted residual
@row_add n=123 row=decision/live-plugin-install-round area=data-protection rec=fct-vaineopj@1
name: Live plugin install round
rule: A live plugin install round runs the CLI's four plugin commands at local scope outside the work tree, passing folder trust by keys into its own pty only, never a desktop key.
why: accepted residual
@row_add n=124 row=decision/verify-never-answers-import area=data-protection rec=fct-tjw7g733@1
name: Verify needs a pre-trusted folder
rule: verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
@row_add n=125 row=decision/event-log-growth-risk area=data-protection rec=fct-q6dicbwj@1
name: Event log growth accepted
rule: Unbounded growth of the event log may exhaust local disk until retention is resolved.
why: accepted risk: availability impact only, no confidentiality or integrity impact
@row_none n=24 rec=fct-kvxhlbnb@1 type=S
@row_none n=25 rec=fct-ceqkpcl5@1 type=S
@row_none n=26 rec=fct-6g2bqf3c@1 type=S
@row_none n=27 rec=fct-wyr7i6wd@1 type=S
@row_add n=126 row=decision/rmcp-feature-set area=api-security rec=fct-kjrxk2yb@1
name: rmcp feature set
rule: The release graph enables rmcp features exactly server and transport-io; the test-only e2e crate takes client only, never transport-child-process.
why: all five 2026 rmcp advisories sit in unused features
guard: scripts/rmcp-features.sh in CI's lint job
@row_add n=127 row=decision/rmcp-banned-features area=api-security rec=fct-kjrxk2yb@1
name: rmcp HTTP and auth features banned
rule: rmcp's transport-streamable-http-server and auth features are banned.
why: all five 2026 rmcp advisories sit in unused features
guard: cargo-deny bans.features
@row_none n=28 rec=fct-ynklxelz@1 type=S
@row_none n=29 rec=fct-ysyuym2x@1 type=S
@row_none n=30 rec=fct-ujqwqmh6@1 type=S
@row_add n=128 row=decision/cargo-deny-audit area=dependency-security rec=fct-bzbrgpfq@1,fct-cmrmoq4t@1
name: cargo-deny as audit tool
rule: cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.
@row_add n=129 row=decision/zizmor-workflow-audit area=dependency-security rec=fct-5ktlwg6z@1,fct-cmrmoq4t@1
name: zizmor for workflows
rule: zizmor 1.30.1 or newer audits the GitHub Actions workflows.
@row_add n=130 row=decision/cargo-audit-not-gated area=dependency-security rec=fct-ioqfwukn@1
name: cargo-audit not gated
rule: cargo-audit is not gated; it is reserved for binary scans of cargo-auditable release binaries.
why: it would duplicate cargo-deny advisories on the same lockfile
@row_none n=31 rec=fct-q56htl73@1 type=S
@row_add n=131 row=decision/no-ignored-advisories area=dependency-security rec=fct-serp72ds@2
name: No ignored advisories
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores no advisory; a new ignore follows the Critical CVE response SLA, its reason naming the disabled feature.
@row_add n=132 row=decision/crates-io-only area=dependency-security rec=fct-l55i2g5k@1
name: crates.io as the only dependency source
rule: cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only.
why: keeps CVE-2026-5222's sparse-registry class out of reach
@row_add n=133 row=decision/licence-allow-list area=dependency-security rec=fct-32d4ty7r@2
name: Closed licence allow list
rule: The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0; 0BSD is never added to the allow list.
why: keeping 0BSD out of the allow list makes any other 0BSD crate still fail the gate; ratified
guard: cargo deny check licenses
except: 0BSD admitted per crate for doctest-file, pulled in by interprocess; each new exception needs its own Decisions Log entry
@row_add n=134 row=decision/rmcp-http-server-and-auth-banned area=dependency-security rec=fct-mrnvctku@1
name: rmcp HTTP server and auth features banned
rule: cargo-deny denies the rmcp features transport-streamable-http-server and auth.
@row_add n=135 row=decision/serde-json-bounded-depth area=dependency-security rec=fct-mrnvctku@1
name: serde_json recursion limit kept
rule: cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit.
guard: scripts/deny-probes.sh
@row_add n=136 row=decision/no-c-building-crates area=dependency-security rec=fct-cavsczz7@1
name: No C-building crates
rule: Crates that build C code are banned in the root dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_add n=137 row=decision/fuzz-workspace-exemption area=dependency-security rec=fct-blte3oxa@1
name: Fuzz workspace outside the deny graph
rule: The fuzz crate is a separate test-only cargo workspace with its own committed lockfile, outside cargo-deny's root graph; its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it.
why: a ratified test-only exemption, holding only while the fuzz crate is never linked into a shipped binary and never joins the root workspace
@row_none n=32 rec=fct-ylpntjib@1 type=S
@row_add n=138 row=decision/exact-dependency-pins area=dependency-security rec=fct-uydjlfhj@2
name: Exact pins for third-party crates
rule: Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly.
except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
@row_add n=139 row=decision/bytes-advisory-floor area=dependency-security rec=fct-q5ipmlaa@1
name: bytes version floor
rule: The lockfile must resolve bytes >=1.11.1.
why: RUSTSEC-2026-0007
@row_add n=140 row=decision/tracing-subscriber-floor area=dependency-security rec=fct-q5ipmlaa@1,fct-auxwpgtg@1
name: tracing-subscriber version floor
rule: If obs selects tracing-subscriber, it must be >=0.3.20.
why: RUSTSEC-2025-0055
@row_add n=141 row=decision/notify-stays-on-8-2 area=dependency-security rec=fct-efj5qrjv@1
name: notify held on 8.2.0
rule: notify stays on 8.2.0; no 9.0 pre-release is taken.
@row_add n=142 row=decision/product-toolchain-pin area=dependency-security rec=fct-lagqorre@1
name: Product toolchain pinned exactly
rule: The product toolchain is pinned exactly to Rust 1.98.1 with rustfmt and clippy, installed through rustup on the dev host and in CI.
@row_add n=143 row=decision/msrv-floor area=dependency-security rec=fct-lagqorre@1
name: Minimum supported Rust version
rule: The workspace rust-version floor is 1.96, checked in CI with the 1.96 toolchain.
guard: CI msrv job
@row_add n=144 row=decision/fuzz-nightly-pin area=dependency-security rec=fct-lagqorre@1
name: Fuzz nightly toolchain pinned
rule: The fuzz workspace pins its own exact nightly toolchain, separate from the product toolchain, installed through rustup.
@row_add n=145 row=decision/cli-tool-version-floors area=dependency-security rec=fct-fawm7y7e@1,fct-jkafue4u@1
name: Minimum floors for external CLI tools
rule: External CLI and audit tools are expressed as minimum version floors, not exact pins; exact pins are only for lock-pinnable library dependencies.
why: an exact pin triggers a false drift escalation when a patch-behind build is installed
@row_add n=146 row=decision/pre-push-checks-pins-installs-nothing area=dependency-security rec=fct-2c53xvmz@1
name: pre-push checks tool pins, installs nothing
rule: viola-harness pre-push is never a new version source: it checks the host's cc, toolchain channel, cargo-nextest, cargo-mutants and cargo-llvm-cov against CI's own pins and installs nothing.
@row_add n=147 row=contract/pre-push-refusals/tool-pin-mismatch area=dependency-security rec=fct-2c53xvmz@1
boundary: viola-harness pre-push refusals
item: tool-pin-mismatch
promise: Refused when an installed tool's version differs from its CI pin line.
@row_add n=148 row=contract/pre-push-refusals/tool-missing-cc area=dependency-security rec=fct-2c53xvmz@1
boundary: viola-harness pre-push refusals
item: tool-missing: cc
promise: Refused when the host has no C toolchain.
@row_add n=149 row=decision/nothing-runs-as-root area=dependency-security rec=fct-2c53xvmz@1
name: Nothing launched as root
rule: No harness command, gate or pre-push stage launches anything as uid 0, and no provisioning script exists.
@row_add n=150 row=decision/manual-dependency-updates area=dependency-security rec=fct-27vpn33i@1
name: Manual dependency update review
rule: Dependency updates are reviewed manually, triggered by the scheduled advisory run; no automated update bot is used.
why: no automated update bot was researched; the rmcp minor range makes the scheduled run the main signal for that crate
@row_none n=33 rec=fct-4ggr436n@2 type=S
@row_add n=151 row=decision/ci-cargo-deny-gate area=dependency-security rec=fct-mi73d5qc@1
name: cargo-deny gate in CI
rule: CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory, yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
@row_add n=152 row=decision/scheduled-advisory-run area=dependency-security rec=fct-eugkq5vm@1
name: Weekly scheduled advisory run
rule: A separate scheduled workflow runs cargo deny advisory checks weekly, without cache, on both the root and the fuzz manifests.
why: the advisory database changes without code changes
@row_add n=153 row=decision/scheduled-fuzzing area=dependency-security rec=fct-eugkq5vm@1
name: Scheduled fuzzing
rule: The scheduled workflow runs each fuzz target for 120 seconds on the pinned fuzz nightly and uploads the fuzz artifacts on failure.
@row_add n=154 row=decision/no-workflow-concurrency area=dependency-security rec=fct-pxt7iprv@2
name: No concurrency block in workflows
rule: The CI workflows carry no concurrency block; zizmor's pedantic concurrency-limits finding is declined and stays visible.
why: a concurrency group cancels pending runs even with cancel-in-progress false, which would drop a cancelled run's always() gate and upload chain
@row_add n=155 row=decision/release-build-without-test-features area=dependency-security rec=fct-h3byb6pq@2
name: Release build free of test-only features
rule: The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent).
guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
@row_add n=156 row=contract/release-check-output/test-only-feature area=dependency-security rec=fct-h3byb6pq@2
boundary: release-check output
item: release-check: FAILED — test-only feature {feature} in {target}
promise: Printed when a release artifact's features include a test-only feature.
@row_add n=157 row=decision/test-support-dev-only area=dependency-security rec=fct-h3byb6pq@2
name: test-support enabled only for development
rule: viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
@row_add n=158 row=decision/zizmor-workflow-gate area=dependency-security rec=fct-ivw6qaje@1
name: zizmor workflow audit gate
rule: zizmor audits the workflows in CI and fails on unpinned actions, excessive permissions, template injection and cache poisoning.
@row_add n=159 row=decision/perf-uploads-scan-gated area=dependency-security rec=fct-zhdn6mmq@2
name: perf job uploads gated by secret scan
rule: The perf job's hyperfine exports and diagnostics are uploaded only when its own secret scan succeeds, the scan output only when it fails; every perf input is synthetic.
@row_add n=160 row=decision/g2-single-panic-exemption area=dependency-security rec=fct-wsxv34pw@1
name: G2 zero-panics gate has one exemption
rule: CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry.
guard: G2 --probe run before every G2 check
@row_add n=161 row=decision/toolchains-via-rustup area=dependency-security rec=fct-ol2hzo3z@1
name: CI toolchains installed through rustup
rule: CI toolchains come from rustup toolchain install steps; no toolchain action is used.
@row_add n=162 row=decision/event-values-through-env area=dependency-security rec=fct-npplilir@2
name: Event payload values only through env
rule: A value from the GitHub event payload reaches a workflow step only through env, never through an expression inside run.
@row_add n=163 row=decision/least-workflow-permissions area=dependency-security rec=fct-lcepm2kd@1
name: Least workflow permissions
rule: Workflows set empty permissions at workflow level and contents read at job level.
@row_none n=34 rec=fct-dr3u7j76@1 type=9
@row_add n=164 row=decision/cve-sla-enabled-feature area=dependency-security rec=fct-4r3p2tgk@2,fct-x6oor32x@1
name: Critical CVE response for an enabled feature
rule: Within 7 days of a RustSec or GHSA advisory against a crate and feature viola enables, the crate is bumped.
@row_add n=165 row=decision/cve-sla-disabled-feature area=dependency-security rec=fct-oj56jp4z@1,fct-x6oor32x@1
name: Critical CVE response for a disabled feature
rule: Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.
@row_add n=166 row=decision/release-artifact-attestations area=dependency-security rec=fct-27c7vqqx@2,fct-qhcowchx@1
name: Release built with dist and attestations
rule: A v1.x release requires dist >=0.33.0 with GitHub Artifact Attestations.
@row_add n=167 row=decision/release-binary-audit area=dependency-security rec=fct-u57emdou@1,fct-qhcowchx@1
name: Auditable release binaries
rule: A v1.x release requires cargo-auditable >=0.7.6, with cargo audit bin (cargo-audit >=0.22.2) run on the release binaries.
@row_add n=168 row=decision/release-cache-poisoning-addressed area=dependency-security rec=fct-but52eow@1,fct-qhcowchx@1
name: Cache-poisoning findings addressed before release
rule: A v1.x release requires zizmor's cache-poisoning findings to be addressed.
@row_none n=35 rec=fct-4f542nxn@1 type=S
@row_none n=36 rec=fct-cutkxcmw@1 type=S
@row_none n=37 rec=fct-pc7jhvfx@1 type=S
@row_none n=38 rec=fct-pah7hshr@1 type=S
@row_add n=169 row=decision/local-boundary-no-auth-library area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Local boundary instead of an auth library
rule: Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
@row_add n=170 row=decision/owner-only-home-and-pinned-binary area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Owner-only home and pinned binary re-hash
rule: The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
@row_add n=171 row=decision/code-files-rewritten-at-start area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Code-bearing files rewritten at start
rule: Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
@row_add n=172 row=decision/nutype-input-validation area=bootstrap-phases-derive-for-route-setup-project rec=fct-pqj3orx7@1
name: nutype for input validation
rule: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers, channel link and unlink params and the notify path filter.
why: arch rejected garde and validator
@row_add n=173 row=decision/bounded-external-readers area=bootstrap-phases-derive-for-route-setup-project rec=fct-pqj3orx7@1
name: Bounded external readers
rule: Every external reader is wrapped by a size bound at a maximum frame size.
@row_add n=174 row=decision/dependency-audit-tools area=bootstrap-phases-derive-for-route-setup-project rec=fct-n3fibuep@2
name: cargo-deny and zizmor as audit tools
rule: Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
@row_add n=175 row=absence/stored-secrets area=bootstrap-phases-derive-for-route-setup-project rec=fct-gh3kajmo@1
capability: Stored secrets in v1
@row_add n=176 row=decision/scan-gated-ci-uploads area=bootstrap-phases-derive-for-route-setup-project rec=fct-rwpm3oj4@1
name: CI uploads gated by the artifact canary scan
rule: The CI test job runs the artifact canary scan against the NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass.
except: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, leave CI unscanned; a non-synthetic seed must first be dropped
@row_add n=177 row=decision/error-display-no-upstream-text area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
name: Error messages carry no upstream text
rule: The Display output of the product error enums never interpolates upstream text or absolute paths.
@row_add n=178 row=contract/mcp-structured-content/codes area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
boundary: MCP structuredContent
item: refusal, detail and error codes
promise: structuredContent carries only refusal, detail and error codes.
@row_add n=179 row=decision/serde-errors-mapped-to-fixed-message area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
name: Upstream-quoting errors mapped to fixed messages
rule: An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics.
why: serde errors quote the offending value
@row_add n=180 row=decision/never-log-claude-values area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: CLAUDE values never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
@row_add n=181 row=decision/user-content-only-to-diagnostics area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: User content only in diagnostics
rule: Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data.
except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design
@row_add n=182 row=decision/no-paths-in-error-output area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: No paths or type names in error output
rule: Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data.
why: the OS username is the only PII in scope
@row_add n=183 row=decision/diagnostics-owner-only area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: Diagnostics files owner-only
rule: Files under the instance diagnostics are mode 0600.
@row_add n=184 row=decision/human-output-escapes-controls area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: Human-mode output escapes control characters
rule: Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
@row_none n=39 rec=fct-reffq76w@2 type=9
@row_none n=40 rec=fct-amjegsv7@1 type=9
@row_none n=41 rec=fct-6scuvr36@1 type=0
what: instructions to downstream skills on how to order and materialize the bootstrap phases
@row_none n=42 rec=fct-g62enihb@1 type=S
@row_none n=43 rec=fct-gwcpziil@1 type=S
@row_none n=44 rec=fct-fsiwgpe3@1 type=S
@row_add n=185 row=decision/env-not-config-channel area=secret-management rec=fct-ugohyrrh@2
name: Environment variables are not configuration
rule: Environment variables are never a configuration channel for the product.
@row_add n=186 row=decision/claude-credentials-stripped-from-child area=secret-management rec=fct-ugohyrrh@2
name: Inherited CLAUDE credentials stripped from the child
rule: The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
@row_add n=187 row=decision/credential-strip-by-name-only area=secret-management rec=fct-ugohyrrh@2
name: Credential strip reads names only
rule: The environment strip handles variable names only and never reads a variable's value.
@row_add n=188 row=decision/hermetic-child-environment area=secret-management rec=fct-z4tolcdo@1
name: Hermetic child environment in the pre-push gate
rule: The local pre-push gate launches every child under env -i with exactly HOME and PATH, so no host environment value crosses into a child.
except: Any assignment whose value comes from the host is a boundary widening that needs its own Decisions Log entry.
@row_add n=189 row=decision/no-uid-0-launch area=secret-management rec=fct-z4tolcdo@1
name: No root launch
rule: No child is ever launched as uid 0.
@row_add n=190 row=decision/no-credential-literals area=secret-management rec=fct-kmhnhsns@1
name: No credential literals in code
rule: No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
@row_add n=191 row=decision/artifact-canary-scan area=secret-management rec=fct-ab75crtl@1,fct-h2ql6c6w@1
name: CI artifact canary scan
rule: CI always runs the artifact canary secret scan over every file under the agent-run artifact directory, and the test-home uploads run only when it succeeds.
except: The mutants chunk diff is skipped by exact path and excluded from the harness upload, so it is never uploaded.
@row_add n=192 row=contract/artifact-canary-scan-classes/claude-stripped area=secret-management rec=fct-rrs6uojm@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: claude-stripped
promise: The scan fails on any of the twelve stripped CLAUDE* canary needles.
@row_add n=193 row=contract/artifact-canary-scan-classes/cookie-header area=secret-management rec=fct-26k2fw6o@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: cookie header
promise: The scan fails on a cookie: header, matched case-insensitively.
@row_add n=194 row=contract/artifact-canary-scan-classes/content-canary area=secret-management rec=fct-pt67jwgh@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: content canary
promise: The content canary fails the scan only in home-level role files.
@row_add n=195 row=contract/artifact-canary-scan-classes/diagnostics-mode area=secret-management rec=fct-umh22lbq@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: diagnostics file mode
promise: The scan fails on a diagnostics file whose mode is not 0600.
@row_add n=196 row=decision/canary-scan-never-echoes-matches area=secret-management rec=fct-h2ql6c6w@1
name: Canary scan never echoes matches
rule: The artifact canary scan never prints or writes the matched bytes.
@row_none n=45 rec=fct-mkzs6jmr@1 type=S
@row_none n=46 rec=fct-5dopdsx2@1 type=0
what: a not-applicable placeholder line for API key and database password rotation
@row_none n=47 rec=fct-v2lsyubr@1 type=0
what: a not-applicable placeholder line for JWT signing key rotation
@row_none n=48 rec=fct-u7x6vssz@1 type=0
what: a not-applicable placeholder line for TLS certificate rotation
@row_add n=197 row=absence/secret-manager area=secret-management rec=fct-rijajpwh@1
capability: A secret manager, and so any secret access auditing
@row_add n=198 row=decision/messaging-credentials-are-secret area=secret-management rec=fct-b7clcfac@1,fct-3wufpdib@1
name: Messaging token and socket are secrets
rule: The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
@row_add n=199 row=decision/zipsign-key-ci-secret-only area=secret-management rec=fct-6mieayng@2,fct-3wufpdib@1
name: Release signing key held only in CI
rule: The zipsign private key for self_update signatures counts as a secret and is held only as a CI secret in the release workflow.
@row_none n=49 rec=fct-7qkvkkiz@1 type=S
@row_none n=50 rec=fct-ooy2axz4@1 type=S
@row_none n=51 rec=fct-jizwmwx6@1 type=S
@row_add n=200 row=decision/sanitised-external-errors area=error-handling rec=fct-xjdp32rs@1,fct-zhbrnvo2@1
name: Sanitised external error responses
rule: Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
@row_add n=201 row=decision/fixed-error-display-messages area=error-handling rec=fct-scytqusw@1,fct-xjdp32rs@1
name: Fixed error display messages
rule: Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
@row_add n=202 row=contract/channel-errors/-32603 area=error-handling rec=fct-crmfdsu7@1
boundary: channel JSON-RPC errors
item: -32603
promise: An internal error carries the fixed message "internal error" with data null.
@row_add n=203 row=contract/mcp-iserror/instance-unreachable area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error instance-unreachable
promise: An unreachable instance reads error instance-unreachable with detail one of its closed causes.
@row_add n=204 row=contract/mcp-iserror/wrapper-fault area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error wrapper-fault
promise: A wrapper fault reads error wrapper-fault with a sanitised code, message and data, never the peer's own.
@row_add n=205 row=contract/mcp-iserror/invalid-params area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error invalid-params
promise: Invalid parameters read error invalid-params with a closed field word.
@row_add n=206 row=contract/instance-unreachable-cause/strict-modes-failed area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: strict-modes-failed
promise: A closed cause of instance-unreachable.
@row_add n=207 row=contract/instance-unreachable-cause/instance-dead area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: instance-dead
promise: A closed cause of instance-unreachable.
@row_add n=208 row=contract/instance-unreachable-cause/unwrapped-name area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: unwrapped-name
promise: A closed cause of instance-unreachable.
@row_add n=209 row=contract/instance-unreachable-cause/endpoint-lost area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: endpoint-lost
promise: A closed cause of instance-unreachable.
@row_add n=210 row=contract/instance-unreachable-cause/server-verify-failed area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: server-verify-failed
promise: A closed cause of instance-unreachable.
@row_add n=211 row=contract/instance-unreachable-cause/socket-dir-refused area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: socket-dir-refused
promise: A closed cause of instance-unreachable.
@row_add n=212 row=contract/mcp-iserror/refusal area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: refusal
promise: A refusal that is no code word reads unknown, alike on MCP and CLI --json.
@row_add n=213 row=contract/mcp-iserror/detail area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: detail
promise: A string detail that is no known detail reads null, alike on MCP and CLI --json.
@row_add n=214 row=contract/mcp-iserror/text area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError text
item: refused text
promise: Every isError text reads refused: <code>.
@row_add n=215 row=contract/mcp-errors/-32603 area=error-handling rec=fct-kulo4ws7@1
boundary: MCP JSON-RPC errors
item: -32603
promise: Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_none n=52 rec=fct-kzddf7bk@1 type=S
@row_add n=216 row=decision/full-error-detail-to-diagnostics area=error-handling rec=fct-b26ns4gs@1
name: Full error detail only in diagnostics
rule: Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
@row_add n=217 row=decision/stderr-never-log-floor area=error-handling rec=fct-b26ns4gs@1
name: stderr honours the never-log floor
rule: stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
@row_add n=218 row=decision/hook-silent-exit-zero area=error-handling rec=fct-r24nqlwe@1
name: hook is silent and always exits 0
rule: The hook verb and its hidden capture arms never write to stderr and always exit 0; a failing security check ends in exit 0 with no body.
why: So the dialog still renders for the human.
@row_none n=53 rec=fct-4bkqvytp@1 type=S
@row_add n=219 row=contract/channel-errors/-32600 area=error-handling rec=fct-k74n6zto@1
boundary: channel JSON-RPC errors
item: -32600
promise: A frame over MAX_FRAME gets the JSON-RPC error -32600.
@row_add n=220 row=contract/channel-errors/-32602 area=error-handling rec=fct-k74n6zto@1
boundary: channel JSON-RPC errors
item: -32602
promise: A release call carrying from gets the JSON-RPC error -32602.
@row_add n=221 row=contract/channel-refusal/control-character area=error-handling rec=fct-k74n6zto@1
boundary: channel result.refusal
item: not-delivered/control-character
promise: A text holding a refused control character is answered with result.refusal not-delivered/control-character.
@row_add n=222 row=contract/channel-refusal/empty-text area=error-handling rec=fct-k74n6zto@1
boundary: channel result.refusal
item: not-delivered/empty-text
promise: A text whose typed text is empty is answered with result.refusal not-delivered/empty-text.
@row_add n=223 row=absence/external-error-reporting area=error-handling rec=fct-6tg2dujf@1
capability: An external error reporting integration
@row_add n=224 row=decision/logging-bans-before-reporting area=error-handling rec=fct-6tg2dujf@1
name: Logging bans apply to any external reporter
rule: If an external error reporter is ever added, the logging bans apply before transmission.
@row_none n=54 rec=fct-mvfk3yxj@1 type=S
@row_none n=55 rec=fct-ltd2ftfx@1 type=S
@row_none n=56 rec=fct-nl2vclvy@1 type=S
@row_add n=225 row=decision/no-socket-in-tmp-root area=security-anti-patterns-never-do-these rec=fct-zshlqykf@1
name: No Unix socket at the temp root
rule: A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory.
why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
@row_add n=226 row=decision/socket-mode-not-sole-control area=security-anti-patterns-never-do-these rec=fct-zyth7okk@2
name: Socket file mode is not the only control
rule: The socket file's 0600 mode is never the only Unix access control.
why: unix(7) says socket permissions may be ignored.
@row_add n=227 row=decision/chmod-socket-after-bind area=security-anti-patterns-never-do-these rec=fct-zyth7okk@2
name: Socket mode set by chmod after bind
rule: The socket mode is set by chmod after the bind, never through the listener's mode option.
why: Set while macOS, where that option fails the bind, was a target, and kept.
@row_add n=228 row=decision/no-abstract-namespace-sockets area=security-anti-patterns-never-do-these rec=fct-2g5b5nut@1
name: No abstract-namespace sockets
rule: Linux abstract-namespace sockets are never used.
why: They have no filesystem permissions.
@row_add n=229 row=decision/verify-server-before-frame area=security-anti-patterns-never-do-these rec=fct-6cauxi6n@2
name: Verify the server before any frame
rule: No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.
@row_add n=230 row=decision/channel-from-not-identity area=security-anti-patterns-never-do-these rec=fct-ezxl4ss7@1
name: Channel from is not an identity
rule: The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
@row_add n=231 row=absence/user-accounts area=security-anti-patterns-never-do-these rec=fct-32h3f5um@1
capability: User accounts or login endpoints
@row_add n=232 row=decision/server-side-revalidation area=security-anti-patterns-never-do-these rec=fct-bz3kcxxz@1
name: Wrapper re-validates every input
rule: MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
@row_add n=233 row=decision/instance-path-from-validated-name area=security-anti-patterns-never-do-these rec=fct-erbjrvk6@1
name: Instance paths only from validated names
rule: An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
@row_add n=234 row=decision/refuse-control-characters area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Refuse control characters, never strip them
rule: A send text or answer free text holding any C0 control other than LF, CR and TAB, DEL or any C1 control is rejected with control-character and never typed into the PTY nor silently stripped.
except: send typing CR LF pairs and lone CRs as LF and dropping trailing CR and LF after validation is not stripping.
@row_add n=235 row=decision/never-refuse-line-breaks area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Line breaks and tabs are allowed
rule: LF, CR and TAB are never refused, and a text with content is never refused for holding them.
why: Multi-line relays, plans and answers are normal.
@row_add n=236 row=decision/empty-text-refused area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Empty typed text is refused
rule: A text whose typed text is empty is refused with empty-text, since there is nothing to type.
@row_add n=237 row=decision/control-check-on-decoded-chars area=security-anti-patterns-never-do-these rec=fct-ghv4dm4o@1
name: Control-character check on decoded characters
rule: The control-character check runs over decoded characters, never raw bytes.
why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
@row_add n=238 row=decision/reads-capped-at-max-frame area=security-anti-patterns-never-do-these rec=fct-zavgpgbb@1
name: Reads capped at MAX_FRAME
rule: Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
@row_add n=239 row=decision/over-long-line-ends-session area=security-anti-patterns-never-do-these rec=fct-zavgpgbb@1
name: Over-long line ends the session
rule: An over-long MCP stdin line ends the session; it is never truncated or skipped.
@row_add n=240 row=decision/no-serde-unbounded-depth area=security-anti-patterns-never-do-these rec=fct-eegmmvld@1
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is never enabled.
guard: deny.toml ban
@row_add n=241 row=decision/closed-types-for-protocol-fields area=security-anti-patterns-never-do-these rec=fct-ncpenfx7@1
name: Closed types for protocol fields
rule: behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
@row_add n=242 row=decision/agents-row-name-not-a-target area=security-anti-patterns-never-do-these rec=fct-ni56qcoe@1
name: claude agents names are not targets
rule: A claude agents --json row name is never used as a ViolaName or target.
@row_add n=243 row=decision/owner-only-file-modes area=security-anti-patterns-never-do-these rec=fct-fmxdaj54@2
name: Owner-only modes under the viola home
rule: The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask.
except: The pinned versioned viola binary is 0700, and never group- or world-writable.
@row_add n=244 row=decision/statusline-only-from-strict-home area=security-anti-patterns-never-do-these rec=fct-sv5b7ryi@1
name: statusline command only from a strict home
rule: statusline_command is never run from a snapshot whose home or instance directory fails the strict-modes check.
why: Write access there equals code execution.
@row_add n=245 row=decision/rehash-pinned-binary area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Re-hash a pinned binary before reuse
rule: An existing pinned viola binary is never reused without re-hashing it against the hash in its directory name.
@row_add n=246 row=decision/cryptographic-pin-hash area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Pin hash is cryptographic
rule: The pinned binary's hash is a truncated SHA-256, never a non-cryptographic hash.
@row_add n=247 row=decision/rewrite-plugin-files area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Plugin files always rewritten
rule: Existing plugin files are never reused without rewriting them.
@row_add n=248 row=decision/non-inheritable-channel-handles area=security-anti-patterns-never-do-these rec=fct-iftwwapx@2
name: Channel handles not inheritable
rule: Channel handles are never made inheritable.
@row_add n=249 row=decision/fixture-hygiene area=security-anti-patterns-never-do-these rec=fct-kj2bdrt2@1
name: Recorded fixture hygiene
rule: Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
except: A workspace.repo naming this repository's own remote is kept.
@row_add n=250 row=decision/no-rmcp-http-or-auth-features area=security-anti-patterns-never-do-these rec=fct-ggmql6i2@1
name: No rmcp HTTP server or auth features
rule: rmcp's transport-streamable-http-server and auth features are never enabled.
@row_add n=251 row=decision/request-size-limits area=security-anti-patterns-never-do-these rec=fct-kemtlykb@1
name: Request size limits
rule: MAX_FRAME is enforced on the channel and on each viola mcp stdin line; request size limits are never skipped.
@row_add n=252 row=decision/no-messaging-secrets-in-outputs area=security-anti-patterns-never-do-these rec=fct-vbcb52sx@1
name: No messaging secrets in any output
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
@row_add n=253 row=decision/no-secrets-in-git area=security-anti-patterns-never-do-these rec=fct-jo3zjurs@1
name: No secrets or local homes in git
rule: .env files, local --home test directories and signing material are never committed to git.
@row_add n=254 row=decision/no-hardcoded-credentials area=security-anti-patterns-never-do-these rec=fct-rnwhyrn3@1
name: No hardcoded credentials
rule: Credentials are never hardcoded in source or in the include_str! plugin files.
@row_add n=255 row=decision/no-parser-error-chains-outward area=security-anti-patterns-never-do-these rec=fct-ilzyulw7@1
name: No parser error chains in external output
rule: serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr.
why: their Display quotes input values such as tool input
@row_add n=256 row=decision/hook-silent-and-exit-zero area=security-anti-patterns-never-do-these rec=fct-tnu5bfz7@1
name: viola hook writes no stderr and exits zero
rule: viola hook never writes to stderr and never exits non-zero, including on security-check failures.
@row_add n=257 row=decision/escape-terminal-controls area=security-anti-patterns-never-do-these rec=fct-llbob32q@1
name: Escape C0/C1 controls in human output
rule: wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls.
why: the same class as RUSTSEC-2025-0055
@row_add n=258 row=decision/tracing-subscriber-version-floor area=security-anti-patterns-never-do-these rec=fct-egib22ly@1
name: tracing-subscriber version floor
rule: tracing-subscriber below 0.3.20 is never used, if obs selects it.
@row_add n=259 row=decision/private-diagnostics-files area=security-anti-patterns-never-do-these rec=fct-423bwibm@1
name: Private diagnostics files
rule: diagnostics/ files are never created readable by other users.
@row_add n=260 row=decision/no-absolute-paths-in-external-errors area=security-anti-patterns-never-do-these rec=fct-6xxq3ubv@1
name: No absolute paths in external errors
rule: No external error ever exposes an absolute path.
@row_add n=261 row=decision/upstream-text-is-never-a-command area=security-anti-patterns-never-do-these rec=fct-4py7x3cl@1
name: Upstream text is never a command (R1)
rule: Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
@row_add n=262 row=decision/compiled-classification-sources area=security-anti-patterns-never-do-these rec=fct-xmaxdng7@1
name: Classification sources are compiled only
rule: Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
@row_add n=263 row=decision/harness-prefix-literals area=security-anti-patterns-never-do-these rec=fct-xmaxdng7@1
name: Harness prefixes as four raw-start literals
rule: A prompt is classified harness only by four compiled literals matched on its raw start with no trim: <agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message.
why: ratified side effect: the escaped cross-session form is the one escaped tag that classifies harness, so a human who types it at a prompt's start is filed harness
@row_add n=264 row=decision/no-interprocess-try-overwrite area=security-anti-patterns-never-do-these rec=fct-3nboftz7@1,fct-2pxk6g7w@2
name: interprocess try_overwrite stays off
rule: interprocess try_overwrite is never enabled on the socket bind.
why: TOCTOU deletion of an attacker-swapped file
@row_add n=265 row=decision/vt100-inside-catch-unwind area=security-anti-patterns-never-do-these rec=fct-iwuq6h35@1
name: vt100 only inside catch_unwind
rule: vt100 is never fed outside catch_unwind.
why: a parser panic must not take down the human's terminal path
@row_add n=266 row=decision/e2e-home-removal-own-dirs-only area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: Test-side removal only of its own dirs
rule: A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
@row_add n=267 row=decision/e2e-home-keepers-linked-base area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: e2e-home keepers follow only a safe linked base
rule: The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
@row_add n=268 row=decision/viola-ignores-e2e-home-backing area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: viola never touches the e2e-home backing
rule: The viola binary itself never reads or writes the target/e2e-home backing by that name.
@row_add n=269 row=decision/new-listener-needs-log-entry area=security-anti-patterns-never-do-these rec=fct-yfqtfw75@1
name: New listener or channel method needs a log entry
rule: No listener (TCP port, WebSocket, http hooks, MCP HTTP transport) or new channel method is added without a Security Decisions Log entry mapping it to the security plan's IPC controls.
@row_add n=270 row=decision/refusals-never-block-human area=security-anti-patterns-never-do-these rec=fct-ciublye4@1
name: Security refusals never block the human
rule: Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
@row_add n=271 row=decision/absolute-viola-path-in-commands area=security-anti-patterns-never-do-these rec=fct-enn2bam2@2
name: Hook, MCP and statusline commands use an absolute viola path
rule: Every exec-form command in the embedded hooks.json or .mcp.json, in plugin install's .mcp.json, and the statusline command written into instances/<name>/settings.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH.
why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
@row_add n=272 row=decision/snapshot-written-only-by-wrapper area=security-anti-patterns-never-do-these rec=fct-pmvbeqyd@1
name: Only the instance's wrapper writes its snapshot
rule: No process other than the instance's own wrapper writes instances/<name>/snapshot.json.
why: endpoint, pinned_bin and statusline_command in it decide where clients connect and what hook statusline executes
@row_add n=273 row=decision/controls-not-switchable area=security-anti-patterns-never-do-these rec=fct-7ackeyph@2
name: No config, env var or flag switches off a control
rule: config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text).
why: environment variables are not a configuration channel
except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
@row_add n=274 row=decision/new-env-read-needs-log-entry area=security-anti-patterns-never-do-these rec=fct-7ackeyph@2
name: New seam or environment read needs a log entry
rule: No further test-only seam or environment read is added without its own Decisions Log entry.
@row_none n=57 rec=fct-jjw3aoc5@1 type=S
@row_none n=58 rec=fct-lnwynz4l@1 type=S
@row_add n=275 row=absence/accounts-and-passwords area=authentication-authorization rec=fct-riad4wv3@1
capability: User accounts and passwords
@row_add n=276 row=decision/all-or-nothing-channel-access area=authentication-authorization rec=fct-4riic6pc@1
name: Capability-based, all-or-nothing channel access
rule: Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles.
why: the only principal is the local OS user
@row_add n=277 row=decision/per-user-socket-directory area=authentication-authorization rec=fct-2pxk6g7w@2
name: Socket in a per-user 0700 directory
rule: The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.
@row_add n=278 row=decision/socket-directory-check area=authentication-authorization rec=fct-2pxk6g7w@2,fct-ezmg3kua@2
name: Socket-directory check before bind and connect
rule: Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink.
guard: security_negatives_socket_dir
@row_add n=279 row=contract/socket-dir-refusal/run-and-revive-start area=authentication-authorization rec=fct-2pxk6g7w@2
boundary: socket-directory refusal
item: run and the start of revive
promise: A failed create and a refused check read one code: exit 1 with socket-dir-refused and one process-exit line of that detail; a Server::bind failure is outside this code.
@row_add n=280 row=contract/socket-dir-refusal/cli-json area=authentication-authorization rec=fct-2pxk6g7w@2,fct-ybwsdbo3@1
boundary: socket-directory refusal
item: CLI frame verb under --json
promise: No frame is written and the verb exits 21 with {"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"} on stdout and stderr empty.
@row_add n=281 row=contract/socket-dir-refusal/cli-human area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: socket-directory refusal
item: CLI frame verb in human mode
promise: No frame is written and the verb exits 21 with the unable <name> instance-unreachable line and the hint of cause socket-dir last on stderr.
@row_add n=282 row=contract/socket-dir-refusal/mcp area=authentication-authorization rec=fct-2pxk6g7w@2,fct-6ntn52d2@1
boundary: socket-directory refusal
item: viola mcp tool
promise: The instance-unreachable result with detail socket-dir-refused, no frame and no line, isError the only record.
@row_add n=283 row=contract/socket-dir-refusal/hook area=authentication-authorization rec=fct-2pxk6g7w@2
boundary: socket-directory refusal
item: viola hook
promise: Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
@row_add n=284 row=decision/socket-file-chmod-after-bind area=authentication-authorization rec=fct-2pxk6g7w@2
name: Socket file chmod 0600 after bind
rule: The socket file is chmod 0600 right after the bind; setting the mode through listener options is not used, chmod after the bind being the one form.
@row_none n=59 rec=fct-a6h3q7dw@2 type=4
relied: An absent socket directory passes the check, so a start can create it, while an unstattable one is refused.
@row_add n=285 row=decision/socket-dir-check-never-chmods area=authentication-authorization rec=fct-a2nznb7h@1
name: Socket-directory check never chmods
rule: The socket-directory check never changes a directory's mode: a refused directory stays as found.
guard: security_negatives_socket_dir
@row_none n=60 rec=fct-ohbg3pqy@1 type=4
relied: The bind and the three client seams share one check on a recorded endpoint's parent directory.
@row_none n=61 rec=fct-6oinutja@1 type=4
relied: The socket-directory check runs before any snapshot, child or bind at run and at the start of revive, a race's loser reading the same.
@row_add n=286 row=decision/leftover-socket-file-kept area=authentication-authorization rec=fct-sqeb75n5@1
name: Leftover socket file is not removed
rule: A killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: by choice: a harmless file in a 0700 directory
@row_add n=287 row=decision/server-side-peer-euid-check area=authentication-authorization rec=fct-ekk5s2bu@1
name: Server drops a foreign-euid peer
rule: On every accept the server reads the peer's credentials and drops the connection before reading a frame if the peer's euid is not its own.
@row_none n=62 rec=fct-pf2l572y@1 type=4
relied: A dropped peer reads end-of-file, and no line and nothing on the terminal is written.
@row_add n=288 row=decision/client-verifies-server-before-frame area=authentication-authorization rec=fct-ju3c6nzj@2
name: Client verifies the server before any frame
rule: Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal.
why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
@row_add n=289 row=decision/strict-modes-before-snapshot-read area=authentication-authorization rec=fct-ju3c6nzj@2,fct-65crhg3m@2
name: Strict-modes check before the snapshot is read
rule: Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification.
why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
@row_add n=290 row=contract/server-verify-refusal/cli-exit area=authentication-authorization rec=fct-ju3c6nzj@2,fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb exit and line
promise: No frame is written; the verb exits 21 instance-unreachable and writes one process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"} WARN line.
@row_add n=291 row=contract/server-verify-refusal/mcp area=authentication-authorization rec=fct-ju3c6nzj@2,fct-6ntn52d2@1
boundary: server-verification refusal
item: viola mcp tool
promise: A refused peer euid or failed server check returns instance-unreachable with detail server-verify-failed, no frame and no line, isError the only record.
@row_add n=292 row=contract/server-verify-refusal/hook area=authentication-authorization rec=fct-ju3c6nzj@2
boundary: server-verification refusal
item: viola hook
promise: Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
@row_add n=293 row=decision/no-identity-values-in-refusals area=authentication-authorization rec=fct-ju3c6nzj@2
name: No identity values in verification output
rule: No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
@row_add n=294 row=contract/strict-modes-refusal/cli-frame-verbs area=authentication-authorization rec=fct-ju3c6nzj@2,fct-ybwsdbo3@1
boundary: strict-modes refusal
item: CLI frame verb
promise: Before liveness and server checks: no frame and no line, nothing opened under the home; exit 21 with the instance-state unable:/hint: pair, or under --json the instance-unreachable document with detail strict-modes-failed.
@row_add n=295 row=contract/strict-modes-refusal/hook area=authentication-authorization rec=fct-ju3c6nzj@2,fct-65crhg3m@2
boundary: strict-modes refusal
item: viola hook
promise: Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
@row_add n=296 row=contract/strict-modes-refusal/mcp-tool area=authentication-authorization rec=fct-ju3c6nzj@2,fct-6ntn52d2@1
boundary: strict-modes refusal
item: viola mcp tool
promise: Returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_add n=297 row=decision/collision-check-needs-no-server-verify area=authentication-authorization rec=fct-ju3c6nzj@2
name: run's collision check is not server-verified
rule: run's collision check judges a recorded wrapper by its pid, OS start time and heartbeat (a replayed snapshot by the beat alone) and opens no connection, so the peer-pid server check does not apply to it.
why: it writes no frame, so there is no serving process to verify
@row_add n=298 row=decision/client-seam-check-order area=authentication-authorization rec=fct-kbl7frdn@1
name: Check order at every client seam
rule: On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
@row_add n=299 row=decision/release-from-driver-guard area=authentication-authorization rec=fct-sjhqtetq@1
name: Driver-originated release is refused
rule: The wrapper refuses a release whose params carry a string from, before budget is acted on.
why: an affordance-level guard against drivers handing the wheel back to automation, not enforcement: a same-user process can leave out from
except: pause, link and unlink from drivers are allowed
@row_add n=300 row=contract/channel-release/string-from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: string from
promise: Refused with -32602 "invalid params" and data {"reason":"release-from-driver"}, one release-from-driver line logged (from_trust:"self-reported"); a budget pause is not lifted and the snapshot's budget fields stay as they were.
@row_add n=301 row=contract/channel-release/non-string-from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: from of another type
promise: Refused with -32602 and data null.
@row_add n=302 row=contract/channel-release/params area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: params
promise: release params are {budget?, from?}; a release on a wheel the driver already holds changes nothing.
@row_add n=303 row=contract/cli-release/from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: CLI viola release
item: from
promise: from is sent from VIOLA_NAME, so viola release run from a driver session exits 20.
@row_add n=304 row=contract/channel-response-line/release-from-driver area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel-response log line
item: release-from-driver
promise: The server's and client's line is WARN with detail release-from-driver and no param name or value; a from of another type reads invalid-params; the line adds nothing to the wire body.
@row_add n=305 row=decision/private-file-modes area=authentication-authorization rec=fct-65crhg3m@2
name: Owner-only modes for the viola home
rule: The home (and any --home) is created 0700 and every file 0600, never group- or world-writable.
except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
@row_add n=306 row=decision/explicit-mode-before-write area=authentication-authorization rec=fct-65crhg3m@2
name: Mode set explicitly before any byte
rule: A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded.
@row_add n=307 row=decision/one-replace-helper area=authentication-authorization rec=fct-65crhg3m@2
name: One shared helper replaces files
rule: Every replaced file goes through one shared helper that writes a private temp file in the same directory, syncs it and persists it into place.
@row_add n=308 row=decision/pinned-viola-never-replaced area=authentication-authorization rec=fct-65crhg3m@2
name: Pinned viola is never replaced
rule: The pinned viola is never replaced: an absent copy is written to a 0700 temp file, synced, closed and placed without clobbering, never over a present name.
@row_add n=309 row=decision/strict-modes-read-rule area=authentication-authorization rec=fct-65crhg3m@2
name: Strict-modes read rule
rule: The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission.
why: so a pre-existing home cannot expose events.ndjson or diagnostics/
@row_add n=310 row=decision/strict-refusal-leaves-no-trace area=authentication-authorization rec=fct-65crhg3m@2
name: A strict-modes refusal leaves no trace
rule: A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens.
why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
@row_add n=311 row=decision/unstattable-home-refused area=authentication-authorization rec=fct-65crhg3m@2
name: An unstattable home is refused
rule: An unstattable home (a path under a regular file) is a strict-modes refusal.
@row_add n=312 row=decision/whole-board-refusal area=authentication-authorization rec=fct-65crhg3m@2
name: viola list refuses the whole board
rule: viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board.
why: ratified
@row_add n=313 row=decision/board-home-read-rule area=authentication-authorization rec=fct-pq4mk5dy@3,fct-65crhg3m@2
name: The board holds the home to the read rule
rule: viola list refuses a home another user may read, with or without an instance.
@row_add n=314 row=decision/home-write-rule-checkers area=authentication-authorization rec=fct-pq4mk5dy@3
name: Budget read, mcp start and plugin install check the home
rule: The budget read, the start of viola mcp and viola plugin install hold the home to the write rule.
@row_add n=315 row=contract/strict-modes-refusal/run-revive area=authentication-authorization rec=fct-65crhg3m@2,fct-mhqlyrp7@2
boundary: strict-modes refusal
item: run and revive
promise: Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.
@row_add n=316 row=contract/strict-modes-refusal/list-json area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola list under --json
promise: Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
@row_add n=317 row=contract/strict-modes-refusal/list-human area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola list in human form
promise: Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
@row_add n=318 row=contract/strict-modes-refusal/link-unlink area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola link and viola unlink
promise: Exit 21 with the instance-state unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout; no log, no line, the mode as found.
@row_add n=319 row=contract/strict-modes-refusal/mcp-start area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola mcp start
promise: Exit 1 with nothing on stdout or stderr and no role file.
@row_add n=320 row=contract/strict-modes-refusal/plugin-install area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola plugin install
promise: Exit 21 (instance-unreachable under --json) with nothing created and no log opened.
@row_add n=321 row=contract/strict-modes-refusal/verify area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola verify
promise: Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.
@row_add n=322 row=contract/strict-modes-refusal/version-gate area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola run's version gate
promise: A refused stamps read is cli_verified:false with parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}.
@row_add n=323 row=contract/strict-refusal-message/readable area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal message
item: Readable
promise: The read rule's refusal reads exactly "a trusted state path is readable by another user".
@row_add n=324 row=contract/strict-refusal-message/symlink area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal message
item: Symlink
promise: A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".
@row_add n=325 row=decision/statusline-sources-untrusted-read area=authentication-authorization rec=fct-65crhg3m@2
name: Statusline sources read untrusted
rule: statusline-source.json and the user's ~/.claude/settings.json are not trusted files; their control is a read-only, capped, tolerant read.
why: a boundary widening that was ratified
@row_add n=326 row=decision/budget-gate-fails-open area=authentication-authorization rec=fct-65crhg3m@2
name: Budget gate reads an untrusted file, failing open
rule: The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file; a refused or unreadable file is no reading and the gate does not pause.
why: a ratified boundary widening: a file a hook arm writes decides a wrapper's refusal
@row_none n=63 rec=fct-gfc5nchi@2 type=4
relied: Reading the connected stream's peer pid writes and logs nothing.
@row_none n=64 rec=fct-zpaxdj2d@2 type=4
relied: A peer whose euid cannot be read is refused, and reading it writes and logs nothing.
@row_none n=65 rec=fct-5ryfhirx@2 type=4
relied: Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a frame be written.
@row_none n=66 rec=fct-p7bllbag@2 type=4
relied: link and unlink take the same order as every CLI frame verb: strict-modes check, liveness pre-check, then the socket-directory, euid and server checks before their one frame.
@row_add n=327 row=contract/server-verify-refusal/cli-json area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb under --json
promise: No frame; exit 21 with {"v":1,"error":"instance-unreachable","detail":"server-verify-failed"} on stdout and stderr empty; a refused peer euid reads the same.
@row_add n=328 row=contract/server-verify-refusal/cli-human area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb in human mode
promise: No frame; exit 21 with the unable <name> instance-unreachable line and the hint "the process on <name>'s endpoint is not its recorded wrapper; it was not contacted" last on stderr.
@row_add n=329 row=decision/client-exit-13-first area=authentication-authorization rec=fct-ybwsdbo3@1
name: Client's own refusals come first
rule: A CLI frame verb's own exit-13 refusals come first, then its strict-modes refusal, before its liveness and server checks.
@row_add n=330 row=decision/no-seam-for-foreign-peer area=authentication-authorization rec=fct-efyzkncd@1
name: No seam to drive a foreign peer
rule: A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them.
why: one account can make no foreign peer
@row_none n=67 rec=fct-h63tjgiv@2 type=5
@row_add n=331 row=decision/checks-at-seams-not-channel-client area=authentication-authorization rec=fct-2yxn5okb@1
name: Security checks sit at the seams, not the channel client
rule: No check sits inside the channel crate's client connect; the checks sit at the seams, and a test may write through that client past every check by design.
why: viola-channel has no viola-state edge
@row_add n=332 row=decision/no-auth-library-local-boundary area=bootstrap-phases rec=fct-sawbqqxr@2
name: Authentication narrowed to the local boundary
rule: Authentication is narrowed to the local boundary, and no auth library is taken.
@row_add n=333 row=decision/socket-owner-only-hardening area=bootstrap-phases rec=fct-sawbqqxr@2
name: Owner-only channel socket
rule: The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
@row_add n=334 row=decision/client-checks-before-frame area=bootstrap-phases rec=fct-sawbqqxr@2,fct-tukgmehf@1
name: Client checks before any frame
rule: Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).
@row_add n=335 row=decision/plaintext-state-os-permissions area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Plaintext state protected by OS permissions
rule: Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files.
except: The pinned bin/<version>-<hash>/viola copy is 0700.
@row_add n=336 row=decision/code-bearing-files-rewritten-each-start area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Code-bearing files rewritten at every start
rule: The plugin files and the instance settings.json override are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.
why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
@row_add n=337 row=decision/statusline-override-plain-path-only area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Statusline override only for a plain pinned path
rule: The settings.json statusline override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings flag is passed.
why: A shell runs the command string unquoted.
@row_add n=338 row=decision/exec-commands-absolute-pinned-path area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Exec commands use the absolute pinned path
rule: Every exec-form command viola writes for the CLI is the absolute pinned bin/<version>-<hash>/viola path, never a PATH lookup.
@row_add n=339 row=decision/pinned-copy-content-hash area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Pinned copy keyed and checked by SHA-256
rule: The pinned copy's <hash> is the first 16 hex characters of the SHA-256 of the exe bytes; before reuse the copy is re-hashed and a mismatch refuses the start with exit 1.
why: Being cryptographic, the hash catches substitution as well as accidental corruption.
guard: contract_content_hash
@row_add n=340 row=decision/sha2-content-hash area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: sha2 is the SHA-256 implementation
rule: The SHA-256 implementation is the sha2 crate with default features off.
why: Pure Rust, no cc on the one target triple, so it passes the cargo-deny C-build ban.
guard: contract_content_hash
@row_add n=341 row=decision/no-non-cryptographic-hash-fallback area=data-protection rec=fct-zvqsbdzq@1
name: No non-cryptographic hash fallback
rule: A non-cryptographic fallback for the pinned copy's content hash is banned.
@row_add n=342 row=decision/same-user-tampering-out-of-scope area=data-protection rec=fct-zvqsbdzq@1
name: Same-user tampering outside the boundary
rule: Tampering with the pinned copy by the same OS user stays outside the v1 security boundary.
why: The same user could also rewrite the folder name.
@row_add n=343 row=decision/marketplace-written-only-by-install area=data-protection rec=fct-zvqsbdzq@1
name: Marketplace tree written only by plugin install
rule: The marketplace tree is written only by viola plugin install, each file rewritten whole at 0600 and each dir 0700, with no hooks in any document and any other entry left as found; viola never reads the tree back.
@row_add n=344 row=decision/no-writes-under-claude-home area=data-protection rec=fct-zvqsbdzq@1,fct-uz6mlfv5@2
name: viola writes nothing under the Claude home
rule: viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
@row_add n=345 row=decision/verify-probe-captures-transient area=data-protection rec=fct-zvqsbdzq@1
name: Verify probe captures are transient
rule: viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
@row_add n=346 row=decision/verify-probe-dirs-cleanup area=data-protection rec=fct-zvqsbdzq@1
name: Verify probe directories removed by drop guards
rule: Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists.
except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
@row_add n=347 row=decision/verify-probes-confined-to-own-dir area=data-protection rec=fct-zvqsbdzq@1,fct-uz6mlfv5@2
name: Verify probe effects stay in their own directory
rule: Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
@row_add n=348 row=decision/pinned-copy-no-clobber-placement area=data-protection rec=fct-l66ara67@2,fct-lzwf4bbi@1,fct-zvqsbdzq@1
name: No-clobber placement of the pinned copy
rule: An absent pinned copy is placed by a no-clobber placement of a temp file whose mode is set before any byte, written, synced and closed first; a present name is never replaced, the target left untouched and the temp file removed.
why: The name never appears while a handle holds it open for write.
@row_add n=349 row=decision/lost-placement-rehash area=data-protection rec=fct-i5etdjvh@1,fct-zvqsbdzq@1
name: A lost placement re-hashes the standing copy
rule: When a concurrent start placed the pinned copy first, the standing copy is re-hashed: equal bytes are reused, different bytes are a hash mismatch with the file never overwritten or deleted.
@row_add n=350 row=decision/two-keys-per-tree-accepted area=data-protection rec=fct-uxsbnoo5@2
name: One tree may key two pinned folders
rule: Clean release builds of one tree may key two bin and plugin folders; this is accepted with no action, the pinned binary's re-hash staying the guard.
@row_add n=351 row=contract/events-link-lines/data area=data-protection rec=fct-cettrvwp@1
boundary: events.ndjson link lines
item: data
promise: A link or unlink line carries data {driver, driven}: the two instance names only, no user content.
@row_add n=352 row=contract/events-link-lines/no-op-pairs area=data-protection rec=fct-cettrvwp@1
boundary: events.ndjson link lines
item: a repeated link or an absent pair's unlink
promise: A held pair's second link and an absent pair's unlink append nothing.
@row_add n=353 row=contract/channel-link-method/reply area=data-protection rec=fct-cettrvwp@1,fct-lce5n72k@1
boundary: the wrapper's link method
item: reply
promise: A successful link replies only {since, already_linked}, since a string and already_linked a boolean.
@row_add n=354 row=contract/channel-unlink-method/reply area=data-protection rec=fct-cettrvwp@1,fct-lce5n72k@1
boundary: the wrapper's unlink method
item: reply
promise: A successful unlink replies only an empty object {}.
@row_add n=355 row=decision/link-names-kept-out-of-process-log area=data-protection rec=fct-jbwneego@1
name: Link names kept out of the process log
rule: The wrapper's link and unlink methods write no process-log line of their own and their request and response lines carry no name; viola link and unlink print only their own parsed arguments, never a name read back from a reply or log.
@row_add n=356 row=decision/diagnostics-open-fails-to-sink area=data-protection rec=fct-gfsndi6o@2
name: A refused diagnostics open writes nowhere
rule: Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
@row_add n=357 row=decision/probe-prompts-synthetic area=data-protection rec=fct-cnkeyzag@2
name: Probe prompts are synthetic
rule: Probe prompts used to record committed fixtures must be synthetic.
@row_add n=358 row=decision/fixture-scrub-and-refusal area=data-protection rec=fct-cnkeyzag@2
name: Recorded fixtures scrubbed or refused whole
rule: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
except: A workspace.repo naming this repository's own remote keeps its owner handle, neither rewritten nor refused.
@row_add n=359 row=decision/screen-fixture-signature-rows-only area=data-protection rec=fct-cnkeyzag@2
name: Screen fixtures keep only signature rows
rule: A screen fixture keeps only the rows holding a compiled signature literal, the rest blanked; a kept row the scrub would rewrite, or a username split across a row seam, refuses the recording.
@row_add n=360 row=contract/fixture-refusal-line/form area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal
item: line form
promise: The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
@row_add n=361 row=contract/fixture-refusal-codes/home-path area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: home-path
promise: A closed check code of the fixture refusal, for a surviving home path.
@row_add n=362 row=contract/fixture-refusal-codes/absolute-path area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: absolute-path
promise: A closed check code of the fixture refusal, for a surviving absolute path.
@row_add n=363 row=contract/fixture-refusal-codes/username area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: username
promise: A closed check code of the fixture refusal, for a surviving user word.
@row_add n=364 row=contract/fixture-refusal-codes/email area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: email
promise: A closed check code of the fixture refusal, for a surviving email-shaped token.
@row_add n=365 row=decision/mcp-fixtures-via-test-only-tap area=data-protection rec=fct-cnkeyzag@2
name: MCP fixtures recorded through a test-only tap
rule: MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes.
guard: schemas/claude-mcp-fixture.v1.json
@row_add n=366 row=contract/verify-refusal-codes/program-not-found area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: program-not-found
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=367 row=contract/verify-refusal-codes/pinned-hash-mismatch area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: pinned-hash-mismatch
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=368 row=contract/verify-refusal-codes/version-unreadable area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: version-unreadable
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=369 row=contract/verify-refusal-codes/strict-modes-failed area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: strict-modes-failed
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=370 row=contract/verify-refusal-codes/unclean-fixture area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: unclean-fixture
promise: One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
@row_add n=371 row=absence/key-management area=data-protection rec=fct-uz6mlfv5@2
capability: Keys at rest and their management
@row_add n=372 row=decision/events-rotation-keeps-modes-and-offsets area=data-protection rec=fct-uz6mlfv5@2
name: Event log rotation keeps modes and offsets
rule: Any rotation scheme for events.ndjson keeps 0600/0700 permissions and keeps offsets valid.
@row_add n=373 row=decision/verify-real-cli-transcripts-accepted area=data-protection rec=fct-uz6mlfv5@2
name: Verify's real-CLI transcripts accepted
rule: verify's real interactive runs under a trusted parent leave the CLI's own synthetic-prompt session transcripts and run the user's global hooks; this residual is an accepted risk class.
@row_add n=374 row=decision/plugin-install-live-round-rig area=data-protection rec=fct-uz6mlfv5@2
name: Live plugin-install rounds isolated
rule: A live round of viola plugin install runs the CLI's commands at local scope from a project that is its own git repo outside the work tree, passing the trust dialog only by keys into its own pty, never a desktop key.
@row_add n=375 row=decision/verify-never-answers-import-dialog area=data-protection rec=fct-uz6mlfv5@2
name: Real-CLI verify needs a prepared trusted folder
rule: viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
@row_add n=376 row=decision/event-log-growth-availability-risk area=data-protection rec=fct-uz6mlfv5@2
name: Event log growth accepted as availability risk
rule: Unbounded growth of events.ndjson exhausting local disk is an accepted risk until retention is resolved.
why: The impact is on availability only, with no confidentiality or integrity impact.
@row_add n=377 row=decision/serde-json-unbounded-depth-banned area=dependency-security rec=fct-pgy5bb7v@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned in cargo-deny.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth
@row_add n=378 row=decision/no-stale-license-exceptions area=dependency-security rec=fct-qfxjrvyr@1
name: No license exception for an absent crate
rule: A license exception naming a crate absent from the graph is dropped, never kept.
why: Such an exception narrows nothing and widens nothing.
@row_add n=379 row=decision/c-building-crates-banned area=dependency-security rec=fct-o63eny7e@2
name: C-building crates banned
rule: Crates that build C are banned from the dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_add n=380 row=decision/cargo-deny-gate-clean area=dependency-security rec=fct-6nwwowx3@2
name: cargo-deny gate with no advisory ignores
rule: CI's supply-chain gate runs cargo deny over advisories, bans, licenses and sources with the advisory ignore list empty.
@row_add n=381 row=decision/exact-third-party-pins area=dependency-security rec=fct-zj6omeqt@2
name: Exact third-party version pins
rule: The workspace pins every third-party dependency version exactly.
@row_add n=382 row=decision/fuzz-workspace-exempt-from-root-deny area=dependency-security rec=fct-i5izpzxc@1
name: Fuzz workspace outside the root deny graph
rule: fuzz is a separate cargo workspace with its own lockfile, outside cargo-deny's root graph, so the C-build ban does not apply to it.
why: Ratified as a test-only exemption.
except: Holds only while the fuzz crate is never linked into a shipped binary and never joins the root workspace.
@row_add n=383 row=decision/fuzz-lockfile-audited-separately area=dependency-security rec=fct-i5izpzxc@1
name: Fuzz lockfile audited separately
rule: The fuzz lockfile is audited separately for advisories and sources in CI and for advisories weekly.
@row_add n=384 row=decision/g2-zero-panics-single-exemption area=dependency-security rec=fct-fguujmtu@1
name: Zero-panics gate with one exemption
rule: CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry.
except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
guard: G2 --probe
@row_add n=385 row=decision/checkout-no-persisted-credentials area=dependency-security rec=fct-dj5p5k3a@1
name: Checkout without persisted credentials
rule: The CI checkout action runs with persist-credentials false.
@row_add n=386 row=decision/ci-artifact-canary-scan area=bootstrap-phases rec=fct-f326yvjk@1
name: CI artifact canary scan gates uploads
rule: Every scan-gated CI upload waits for the artifact canary scan over test diagnostics, harness capture and test reports against the NEVER-log floor, which never prints or writes the matched bytes.
except: The harness capture's chunk.diff, repository source text by construction, is skipped by exact path and never uploaded.
@row_add n=387 row=decision/no-mutants-out-upload area=bootstrap-phases rec=fct-f326yvjk@1
name: No mutants.out upload
rule: CI does not upload mutants.out.
why: It held absolute argv paths and test output.
@row_add n=388 row=decision/fuzz-artifacts-upload-unscanned area=bootstrap-phases rec=fct-f326yvjk@1
name: Fuzz crash artifacts the one unscanned upload
rule: The nightly fuzz job's crash-input upload is the one CI upload left unscanned; a non-synthetic corpus seed must first drop that upload.
why: Ratified as admissible by content: the inputs are mutated from the committed synthetic corpus.
@row_add n=389 row=decision/test-homes-gitignored area=bootstrap-phases rec=fct-f326yvjk@1
name: Test homes kept out of git
rule: Local --home test directories are git-ignored so recorded state never lands in git.
@row_add n=390 row=decision/hook-strict-modes-first area=error-handling rec=fct-io4zn2wb@2
name: hook checks the instance before anything
rule: The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
except: The two capture arms run ahead of that check.
@row_add n=391 row=contract/hook-decision-detail/oversize-stdin area=error-handling rec=fct-io4zn2wb@2
boundary: hook-decision detail
item: oversize-stdin
promise: hook fails open with this detail when stdin exceeds its cap.
@row_add n=392 row=contract/hook-decision-detail/malformed-json area=error-handling rec=fct-io4zn2wb@2
boundary: hook-decision detail
item: malformed-json
promise: hook fails open with this detail when stdin is not one JSON object.
@row_add n=393 row=contract/hook-decision-detail/channel-unreachable area=error-handling rec=fct-io4zn2wb@2,fct-bp6aacay@1
boundary: hook-decision detail
item: channel-unreachable
promise: hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
@row_add n=394 row=contract/hook-decision-detail/server-verify-failed area=error-handling rec=fct-io4zn2wb@2,fct-5pxlrd5y@1
boundary: hook-decision detail
item: server-verify-failed
promise: hook fails open with this detail when the peer euid or server verification fails; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_add n=395 row=contract/hook-decision-detail/socket-dir-refused area=error-handling rec=fct-io4zn2wb@2,fct-33fbgdc3@1
boundary: hook-decision detail
item: socket-dir-refused
promise: hook fails open with this detail, in one line at WARN, when the socket directory is refused; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_add n=396 row=decision/hook-refusals-write-no-frame area=error-handling rec=fct-5pxlrd5y@1,fct-33fbgdc3@1,fct-bp6aacay@1
name: A refused hook writes no frame
rule: A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
@row_none n=68 rec=fct-wandj5yh@1 type=4
@row_add n=397 row=decision/mcp-frame-tool-refusals area=error-handling rec=fct-2qzjtoe6@2,fct-tukgmehf@1
name: Refused MCP frame tools send nothing
rule: A viola mcp frame tool refused by the strict-modes, liveness, socket-directory or server check sends no frame, writes no log line and returns the instance-unreachable result.
@row_add n=398 row=contract/mcp-frame-tool-result/unreachable area=error-handling rec=fct-2qzjtoe6@2,fct-tukgmehf@1,fct-au76dwrt@1
boundary: viola mcp frame tool results
item: instance-unreachable
promise: A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.
@row_add n=399 row=contract/channel-fault/-32603 area=error-handling rec=fct-lce5n72k@1
boundary: wrapper channel faults
item: -32603
promise: A failed link or unlink record answers -32603 with the fixed message `internal error` and data null.
@row_add n=400 row=contract/link-verbs-exit-codes/20 area=error-handling rec=fct-lce5n72k@1
boundary: viola link / unlink exit codes
item: 20
promise: A wrapper error reply, a -32602 included, is a wrapper fault, exit 20.
@row_add n=401 row=contract/link-verbs-exit-codes/1 area=error-handling rec=fct-lce5n72k@1
boundary: viola link / unlink exit codes
item: 1
promise: A Refused reply or an ok of a shape other than the verb's own is an internal error, exit 1.
@row_add n=402 row=decision/channel-faults-sanitised area=error-handling rec=fct-2c6j473b@1,fct-mvo56ebm@1
name: Channel faults sanitised by the clients
rule: Clients read a wrapper fault only with an integer code; each known code reads its fixed message with data null.
except: Two -32602 bodies keep data: the unsupported-version body with an integer data.supported and a 1-64 character data.wrapper, and the body whose data.reason is release-from-driver.
@row_add n=403 row=contract/channel-fault/unknown-code area=error-handling rec=fct-yrpufic3@1
boundary: wrapper faults as read by clients
item: a code outside the five
promise: Reads {"code": <code>, "message": "wrapper fault", "data": null}; `wrapper fault` is a fixed word on --json and MCP, and the human line reads `error: wrapper fault  <code>`.
@row_add n=404 row=decision/reply-code-words-validated area=error-handling rec=fct-tr2s4okr@1,fct-lure7uoj@1
name: Reply code words validated
rule: A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply.
except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
@row_add n=405 row=contract/channel-response-detail/unsupported-version area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: unsupported-version
promise: The closed detail word for the unsupported-version -32602 fault, written on the server's and the client's line.
@row_add n=406 row=contract/channel-response-detail/invalid-params area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: invalid-params
promise: The closed detail word for the invalid-params -32602 fault, written on the server's and the client's line.
@row_add n=407 row=contract/channel-response-detail/release-from-driver area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: release-from-driver
promise: The closed detail word for the release-from-driver -32602 fault, written on the server's and the client's line.
@row_add n=408 row=decision/refused-peer-dropped-before-frame area=error-handling rec=fct-3t6spjp3@1,fct-sawbqqxr@2
name: A refused peer is dropped before any frame
rule: A connection whose peer euid is refused is dropped on the accept thread before any worker or frame: the peer reads end-of-file and the wrapper writes no reply, no line and nothing on the terminal.
@row_add n=409 row=decision/internal-error-held-at-unit-layer area=error-handling rec=fct-iwcvxbkd@1
name: Internal errors tested at the unit layer
rule: No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_add n=410 row=decision/cli-frame-verbs-strict-modes-first area=error-handling rec=fct-uv5qlev7@1
name: CLI frame verbs check the instance first
rule: CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
@row_add n=411 row=contract/cli-instance-unreachable/json-document area=error-handling rec=fct-au76dwrt@1,fct-uv5qlev7@1
boundary: CLI exit 21
item: --json document
promise: Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
@row_none n=69 rec=fct-excpcefl@1 type=4
@row_add n=412 row=contract/cli-stderr-hints/unwrapped-name-endpoint-lost area=error-handling rec=fct-t5nvscu6@1
boundary: CLI stderr hints
item: unwrapped-name and endpoint-lost hints
promise: The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
@row_add n=413 row=decision/socket-dir-refusal-names-only-name area=error-handling rec=fct-2iushqjk@1
name: Socket-directory refusal discloses no path
rule: The socket-directory refusal names only the parsed ViolaName, and its lines hold no path, pid or uid.
@row_add n=414 row=decision/unusable-response-line-on-stderr area=error-handling rec=fct-6qpqztvz@1
name: Unusable answer or send text goes to stderr
rule: The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document.
why: A named exception to the --json error document.
@row_add n=415 row=decision/home-tree-strict-modes-before-log area=error-handling rec=fct-j463drna@2
name: Home tree checked before any log opens
rule: viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_add n=416 row=decision/run-refuses-socket-dir-before-start area=error-handling rec=fct-oolatrnj@1
name: run refuses a bad socket directory first
rule: viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.
@row_add n=417 row=contract/cli-json-error/internal-error area=error-handling rec=fct-vr46nktj@1
boundary: CLI --json error documents
item: internal-error
promise: Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
@row_add n=418 row=contract/collision-check/stale-heartbeat area=error-handling rec=fct-fmkuqvun@1
boundary: start collision check
item: stale-heartbeat
promise: Printed and logged for a stale holder of the name; stale heartbeat is exit 1's cause alone.
@row_add n=419 row=contract/collision-check/already-live area=error-handling rec=fct-fmkuqvun@1
boundary: start collision check
item: already-live
promise: Printed and logged for a live holder and for a fresh replayed beat.
@row_add n=420 row=decision/error-messages-interpolate-nothing area=error-handling rec=fct-g4i32qdx@2,fct-ttxl4mzv@1,fct-syp3b7yx@1
name: Error messages interpolate nothing
rule: Product error types carry fixed Display messages with no path or payload interpolated.
guard: thiserror_messages_interpolate_nothing
@row_none n=70 rec=fct-hzzqncjb@2 type=5
@row_add n=421 row=decision/error-enum-convention-exceptions area=error-handling rec=fct-gnmmvxa5@2
name: Named exceptions to the error-enum convention
rule: ServeError, StampsError, PinError, StampsReadError and Refused diverge from the error-enum convention as named exceptions.
why: Ratified as named exceptions.
@row_none n=71 rec=fct-zu3fn6nz@2 type=4
@row_add n=422 row=decision/every-error-writer-named area=error-handling rec=fct-syp3b7yx@1
name: Every product error writer named
rule: Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census.
guard: product_error_writers_are_each_named
@row_none n=72 rec=fct-ticjysfg@1 type=5
@row_none n=73 rec=fct-msfbrx4e@1 type=5
@row_none n=74 rec=fct-4kx565zx@1 type=7
@row_add n=423 row=decision/diagnostics-strict-modes-fail-open area=error-handling rec=fct-wyrvqwn5@2
name: Diagnostics files failing strict modes fail open
rule: A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
@row_add n=424 row=decision/paste-text-control-character-set area=input-validation rec=fct-ub5klpnc@1
name: Paste text control-character set
rule: Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes.
why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
@row_add n=425 row=decision/reject-dont-strip-paste-text area=input-validation rec=fct-ub5klpnc@1
name: Reject, don't strip
rule: A sent text that holds a refused character is refused whole; no refused character is ever removed and the rest typed.
@row_add n=426 row=decision/typed-text-line-ending-normalisation area=input-validation rec=fct-ub5klpnc@1
name: Typed text line-ending normalisation
rule: After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste.
why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged and the delivery-confirmation match stays an exact equality.
@row_add n=427 row=decision/empty-typed-text-refused area=input-validation rec=fct-ub5klpnc@1
name: Empty typed text refused
rule: A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed.
why: Ratified.
@row_add n=428 row=decision/paste-validation-on-both-sides area=input-validation rec=fct-ub5klpnc@1
name: Paste validation on both sides
rule: Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
@row_add n=429 row=contract/send-refusal/control-character area=input-validation rec=fct-ub5klpnc@1
boundary: send refusal
item: control-character
promise: A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
@row_add n=430 row=decision/dialog-free-text-control-character-set area=input-validation rec=fct-fn2six5m@1
name: Dialog free text control-character set
rule: Dialog free text (an answer's message and question answer values) follows the paste-text set, refused control-character first, in viola answer before any frame (exit 13) and again, authoritatively, in the wrapper's answer handler.
@row_add n=431 row=contract/channel-frames/oversize-frame area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: over-limit frame
promise: A frame is read under a MAX_FRAME cap; an over-limit frame gets -32600 and the connection is closed.
@row_add n=432 row=decision/channel-json-default-depth-limit area=input-validation rec=fct-wrh45y73@1
name: Channel JSON default depth limit
rule: Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
@row_add n=433 row=contract/channel-frames/version area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: v
promise: A frame whose v fails the version check gets -32602.
@row_add n=434 row=contract/channel-frames/send-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: send params
promise: text is a string and from is absent, null or a valid viola name; anything else is -32602 invalid params with data null, before paste validation runs; the envelope types no method param, so a mistyped from never gets -32600.
@row_add n=435 row=contract/channel-frames/wait-last-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: wait and last params
promise: after and timeout_ms are each absent or a non-negative integer that fits u64, else -32602 invalid params with data null; from follows the send rule; unknown fields are ignored.
@row_add n=436 row=contract/channel-frames/hook-dialog-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: hook.dialog params
promise: kind is a closed dialog kind, data an object, hook_event closed, and tool, input and continuation closed or absent; an invalid frame is not registered.
@row_add n=437 row=contract/channel-frames/answer-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: answer params
promise: dialog_id is u64, from follows the send rule, response maps into the closed per-kind enums, then paste validation runs on its free text.
@row_add n=438 row=contract/channel-frames/pause-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: pause params
promise: pause's from follows the send rule.
@row_add n=439 row=contract/channel-frames/release-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: release params
promise: budget is absent or a bool, else -32602 with data null; from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null.
@row_add n=440 row=contract/channel-frames/link-unlink-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: link and unlink params
promise: driver and driven are valid viola name strings; a missing, non-string or invalid name, a driven other than the serving wrapper's name, or a driver equal to driven is -32602 with data null and no line; from is never read.
@row_add n=441 row=contract/channel-frames/hook-event-revalidation area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: hook.event
promise: kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; an invalid event is not appended.
@row_add n=442 row=decision/client-reply-code-words-only area=input-validation rec=fct-wrh45y73@1
name: Client reads reply refusals as code words only
rule: A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
@row_add n=443 row=contract/channel-response-log/detail area=input-validation rec=fct-wrh45y73@1
boundary: channel-response log line
item: detail
promise: The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
@row_add n=444 row=contract/hook-stdin/oversize area=input-validation rec=fct-p4cpzbge@1
boundary: hook stdin
item: oversize payload
promise: A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
@row_add n=445 row=decision/hook-unknown-event-silent-exit area=input-validation rec=fct-p4cpzbge@1
name: Hook unknown event exits silently
rule: The hook event argument maps into a closed set of nine events (statusline taken first, apart); an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing written anywhere.
@row_add n=446 row=decision/hook-instance-check-first area=input-validation rec=fct-p4cpzbge@1,fct-l7hy7pzk@2
name: Hook instance check before anything
rule: Hook event, dialog and statusline paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
@row_add n=447 row=contract/hook-stdin/not-one-object area=input-validation rec=fct-p4cpzbge@1
boundary: hook stdin
item: malformed-json
promise: A payload that is not one JSON object fails open with malformed-json.
@row_add n=448 row=decision/hook-security-fields-closed-enums area=input-validation rec=fct-p4cpzbge@1
name: Hook security fields in closed enums
rule: Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
@row_add n=449 row=decision/hook-dialog-reply-read-bound area=input-validation rec=fct-p4cpzbge@1
name: Hook dialog reply read bound
rule: The hook's read of a dialog reply is bounded at the maximum dialog hold plus 5 s (3605 s), the wrapper replying null at its own configured hold's end.
@row_add n=450 row=decision/question-option-text-crosses-boundary area=input-validation rec=fct-p4cpzbge@1
name: Question option text crosses into events
rule: A question option's description and preview are taken only as strings, else null, never refusing the dialog, and cross into the events log, hook.dialog, CLI wait --json and the MCP wait.
why: A ratified boundary widening.
@row_add n=451 row=decision/stop-background-tasks-count-only area=input-validation rec=fct-p4cpzbge@1
name: Stop background tasks carried as a count only
rule: Stop's background_tasks is carried only as its length, no element read; null when absent, null or not an array, and a non-array also writes a drift entry.
why: Ratified with the option-text widening.
@row_add n=452 row=decision/statusline-payload-fails-silent area=input-validation rec=fct-p4cpzbge@1
name: Statusline payload fails silent
rule: The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr, nothing printed and no budget file.
@row_add n=453 row=decision/budget-json-single-writer area=input-validation rec=fct-p4cpzbge@1
name: budget.json has a single writer
rule: budget.json is written only by the statusline hook arm, and only when the payload holds a rate_limits object.
@row_add n=454 row=contract/budget-json/window area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: window
promise: A window that is absent or not an object is the word "unknown".
@row_add n=455 row=contract/budget-json/used-percentage area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: used_percentage
promise: Kept only from 0 to 100, else "unknown".
@row_add n=456 row=contract/budget-json/resets-at area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: resets_at
promise: Read as epoch seconds or RFC 3339 and written as RFC 3339 UTC with milliseconds; a malformed value becomes "unknown".
@row_add n=457 row=decision/hook-capture-arm area=input-validation rec=fct-p4cpzbge@1
name: Hook capture arm
rule: The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0.
why: A ratified boundary widening.
@row_add n=458 row=decision/statusline-capture-arm area=input-validation rec=fct-p4cpzbge@1
name: Statusline capture arm
rule: The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte, else other, the word itself written nowhere; it prints nothing, exits 0 and opens no channel.
why: Ratified.
@row_add n=459 row=decision/mcp-handlers-validate-raw-arguments area=input-validation rec=fct-dodlc2ml@1
name: MCP handlers validate raw arguments
rule: MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation.
why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
@row_add n=460 row=contract/mcp-tool-inputs/invalid-params area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: invalid-params
promise: A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.
@row_add n=461 row=contract/mcp-tool-inputs/unknown-tool area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: unknown tool name
promise: An unknown tool name is JSON-RPC -32602.
@row_add n=462 row=contract/mcp-tool-inputs/timeout-ms area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: timeout_ms
promise: wait's timeout_ms is absent, null or a u64, clamped to 50 000 ms, which is also its default.
@row_add n=463 row=decision/mcp-stdin-line-cap-on-reader area=input-validation rec=fct-5zwnjssk@1
name: MCP stdin line cap on the reader
rule: Each MCP stdin line is capped at MAX_FRAME, newline counted, by the reader handed to the SDK; a longer line is refused whole, never truncated or skipped, and ends the session as a stdin close does.
why: rmcp's stdio transport reads each line unbounded and its codec length bound covers only the writer.
@row_add n=464 row=contract/mcp-stdin/oversize-line area=input-validation rec=fct-5zwnjssk@1
boundary: viola mcp stdin
item: oversize line
promise: After the session ends one WARN parse-rejected parser mcp-stdin detail oversize count 1, no byte of the line; exit 0 for an initialized session, exit 1 with both streams empty when cut before initialize completes.
@row_add n=465 row=decision/cli-names-through-name-check area=input-validation rec=fct-l7hy7pzk@2
name: CLI names through the viola name check
rule: Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
@row_add n=466 row=decision/home-strict-modes-before-read area=input-validation rec=fct-l7hy7pzk@2
name: Home strict-modes check before any read
rule: --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
@row_add n=467 row=decision/verify-home-tree-check area=input-validation rec=fct-l7hy7pzk@2
name: verify checks the home tree first
rule: viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
@row_add n=468 row=contract/cli-send-text/input area=input-validation rec=fct-l7hy7pzk@2
boundary: viola send text
item: text input
promise: The text comes from stdin or --file only, capped at MAX_FRAME; over the cap or not UTF-8 is exit 2 with one fixed line; a second positional argument is a usage error.
@row_add n=469 row=decision/frame-verbs-instance-check-first area=input-validation rec=fct-l7hy7pzk@2
name: Frame verbs check the instance first
rule: CLI frame verbs and link/unlink run the instance strict-modes check first; a refusal is exit 21 with the instance pair, nothing opened under the home and no line.
@row_add n=470 row=contract/cli-json-error/strict-modes-failed area=input-validation rec=fct-l7hy7pzk@2
boundary: CLI frame verb --json error
item: strict-modes-failed
promise: A refused instance check under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21.
@row_add n=471 row=decision/frame-connect-check-order area=input-validation rec=fct-l7hy7pzk@2
name: Checks before a frame is sent
rule: After the instance check, a CLI frame verb or MCP frame tool runs the liveness pre-check, the endpoint directory check (socket-dir-refused), the connect, the peer euid decision (server-verify-failed) and the server check, all before any frame.
@row_add n=472 row=decision/mcp-frame-tools-refuse-quietly area=input-validation rec=fct-l7hy7pzk@2
name: MCP frame tool refusals
rule: An MCP frame tool refused by the instance, endpoint directory or peer check returns instance-unreachable, with no frame and no line.
@row_add n=473 row=decision/pause-release-from-viola-name area=input-validation rec=fct-l7hy7pzk@2
name: pause and release take from from VIOLA_NAME
rule: The CLI pause and release verbs take their from only from VIOLA_NAME.
@row_add n=474 row=contract/revive-arguments/id area=input-validation rec=fct-l7hy7pzk@2,fct-22lqecbw@1
boundary: viola revive arguments
item: --id
promise: --id accepts only the session-id shape, anything else a usage error with exit 2; --list conflicts with --id, --fork and the trailing arguments.
@row_add n=475 row=decision/revive-trailing-words-as-argv area=input-validation rec=fct-l7hy7pzk@2
name: revive trailing words as argv
rule: The words after -- in viola revive go to the child as argv elements of a direct spawn.
@row_add n=476 row=contract/revive-json/error-document area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json
item: error document
promise: A refusal prints exactly {"v":1,"error":"<code>","detail":null} on stdout, stderr empty, exit 1, the code one of the closed revive codes.
@row_add n=477 row=contract/revive-json/strict-modes-failed area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: strict-modes-failed
promise: A closed exit-1 code of viola revive --json.
@row_add n=478 row=contract/revive-json/already-live area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: already-live
promise: A closed exit-1 code of viola revive --json.
@row_add n=479 row=contract/revive-json/no-session area=input-validation rec=fct-l7hy7pzk@2,fct-22lqecbw@1
boundary: viola revive --json codes
item: no-session
promise: A closed exit-1 code: an --id the log does not hold, or a log with no session, exit 1 before any spawn.
@row_add n=480 row=contract/revive-json/session-live area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: session-live
promise: A closed exit-1 code of viola revive --json.
@row_add n=481 row=contract/revive-json/cwd-missing area=input-validation rec=fct-l7hy7pzk@2,fct-q67nxnld@2
boundary: viola revive --json codes
item: cwd-missing
promise: A closed exit-1 code: the recorded cwd is absent or not an existing directory, no child started.
@row_add n=482 row=contract/revive-json/stale-heartbeat area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: stale-heartbeat
promise: A closed exit-1 code of viola revive --json.
@row_add n=483 row=contract/revive-json/pinned-hash-mismatch area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: pinned-hash-mismatch
promise: A closed exit-1 code of viola revive --json.
@row_add n=484 row=contract/revive-json/squatted-name area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: squatted-name
promise: A closed exit-1 code of viola revive --json.
@row_add n=485 row=contract/revive-json/socket-dir-refused area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: socket-dir-refused
promise: A closed exit-1 code of viola revive --json.
@row_add n=486 row=contract/revive-json/internal-error area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: internal-error
promise: A closed exit-1 code, printed for an internal error of the start path, which is still logged with its chain; a child-pump error prints no document.
@row_add n=487 row=contract/revive-list-json/document area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --list --json
item: sessions document
promise: Prints {"v":1,"ok":{"sessions":[{"ts","cause","id"}]}} in log order: codes, closed words and session-shaped ids only, never a path or pid.
@row_add n=488 row=decision/revive-strict-modes-before-use area=input-validation rec=fct-l7hy7pzk@2
name: revive checks strict modes before use
rule: Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
@row_add n=489 row=decision/link-unlink-driven-instance area=input-validation rec=fct-l7hy7pzk@2
name: link and unlink act on the driven instance
rule: link and unlink act on the driven instance, joined only from its parsed name and checked before any snapshot read; a driver equal to driven is refused by the wrapper, not the client.
@row_add n=490 row=decision/cli-home-resolution-order area=input-validation rec=fct-l7hy7pzk@2
name: CLI home resolution order
rule: Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created.
why: A ratified widening of what chooses a CLI verb's home.
except: hook keeps its own shape check.
@row_add n=491 row=decision/list-scan-checks-before-open area=input-validation rec=fct-l7hy7pzk@2
name: viola list checks before opening
rule: viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
@row_add n=492 row=decision/config-tolerant-parse area=input-validation rec=fct-nuclnamd@2
name: Tolerant config parse
rule: config.json is parsed tolerantly, unknown keys counted, its v checked and its read capped at MAX_FRAME; a bad value falls back to its default and is reported as parse-rejected.
@row_add n=493 row=contract/config-json/five-hour area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: five_hour
promise: A JSON whole number 0 to 100, default 90; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_add n=494 row=contract/config-json/seven-day area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: seven_day
promise: A JSON whole number 0 to 100, default 85; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_add n=495 row=contract/config-json/dialog-hold-s area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: dialog_hold_s
promise: A JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected line; no value makes the hold zero or unbounded.
@row_add n=496 row=decision/dialog-hold-config-only area=input-validation rec=fct-nuclnamd@2
name: Dialog hold set only in config
rule: The dialog hold is read once at the wrapper's start from config.json only; no env var or flag sets it.
why: A ratified boundary widening.
@row_add n=497 row=contract/config-json/port area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: port
promise: Parsed into a u16.
@row_add n=498 row=contract/config-json/diagnostics-level area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: diagnostics_level
promise: The closed set info or debug; any other value falls back to info and is reported as parse-rejected.
@row_add n=499 row=contract/config-json/claude-env-keep area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: claude_env_keep
promise: An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.
@row_add n=500 row=decision/fake-agent-pump-delay-seam area=input-validation rec=fct-jd7jb3il@1
name: Pump delay test seam
rule: FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction.
why: Lets a test land a host resize in the spawn-to-pump window; ratified.
@row_add n=501 row=decision/fake-agent-hook-panic-seam area=input-validation rec=fct-qyyrikqm@1
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction.
why: Ratified.
@row_add n=502 row=decision/terminal-colour-facts area=input-validation rec=fct-lj2i7oks@2
name: Terminal colour facts read for list only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode, only compared, never echoed, logged or set, and decide only whether and how deep SGR is written around already escaped rows.
why: Ratified.
@row_add n=503 row=decision/no-width-or-colour-override-env area=input-validation rec=fct-lj2i7oks@2
name: No width or colour override env
rule: COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read; the board's width comes from the host terminal size, no env read.
@row_add n=504 row=decision/events-reader-bounded-lines area=input-validation rec=fct-wkbf5hxn@1
name: Event log reader bounded lines
rule: The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
@row_add n=505 row=decision/link-restore-trusts-written-names area=input-validation rec=fct-wkbf5hxn@1
name: Link restore trusts names it wrote
rule: Link-set restore uses a link or unlink line's driver and driven only when both are strings and does not parse them as names again; it appends and logs nothing.
why: The wrapper wrote them, each having passed the viola name check.
@row_add n=506 row=decision/torn-last-line-healed-on-append area=input-validation rec=fct-wkbf5hxn@1
name: Torn last line healed on append
rule: The next append heals a torn last line by writing one LF ahead of its line in the same single write under the append lock, and logs one state-recovered line with a detail code, basename and offset, no content.
@row_add n=507 row=decision/state-reads-after-strict-modes area=input-validation rec=fct-wkbf5hxn@1
name: State files read after strict modes
rule: Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
@row_add n=508 row=decision/replayed-snapshot-fallbacks area=input-validation rec=fct-wkbf5hxn@1
name: Replayed snapshot fallbacks
rule: Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, statusline prints nothing, and the collision check judges liveness by the heartbeat alone.
@row_add n=509 row=decision/classified-snapshot-read-only area=input-validation rec=fct-wkbf5hxn@1
name: Classified snapshot read only
rule: No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
@row_add n=510 row=decision/budget-json-checked-read area=input-validation rec=fct-wkbf5hxn@1
name: budget.json checked read
rule: budget.json is read only after path checks on the home and the file, under a shared lock, capped and tolerant, v 1 only; every failure is no reading and no line, and the read never writes or creates anything.
why: The gate refusing a driver on an untrusted file is ratified.
@row_add n=511 row=decision/tailed-paths-valid-instance-logs area=input-validation rec=fct-njhxmpji@1
name: Tailed paths only valid instance logs
rule: Only instances/<name>/events.ndjson under the canonical home is tailed, where the name passes the viola name check; symlinks and names that fail parsing are ignored.
@row_add n=512 row=decision/claude-agents-read-fail-open area=input-validation rec=fct-cysd5aat@1
name: claude agents read bounded, fail open
rule: The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
@row_add n=513 row=decision/unwrapped-session-name-not-target area=input-validation rec=fct-cysd5aat@1,fct-q67nxnld@2
name: Unwrapped session name never a target
rule: An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
@row_add n=514 row=decision/verify-pty-output-caught area=input-validation rec=fct-cysd5aat@1
name: verify PTY output bounded and caught
rule: verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
@row_add n=515 row=decision/vt100-feed-under-panic-catch area=input-validation rec=fct-g433mics@2
name: Screen feed under a panic catch
rule: The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues.
why: vt100 panics are reachable at real small sizes.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=516 row=decision/vt100-feed-bounded-queue area=input-validation rec=fct-g433mics@2
name: Screen feed bounded queue
rule: Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
@row_add n=517 row=decision/claude-agents-resolved-from-path area=input-validation rec=fct-q67nxnld@2
name: claude agents resolved from the caller's PATH
rule: list, revive's preflight and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME.
why: Same-user trust, an accepted risk; the MCP widening ratified.
@row_add n=518 row=decision/run-resolves-absolute-program area=input-validation rec=fct-q67nxnld@2
name: run resolves its child program itself
rule: viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
@row_add n=519 row=decision/revive-recorded-cwd-spawn-dir area=input-validation rec=fct-q67nxnld@2,fct-wkbf5hxn@1
name: revive spawns in the recorded cwd
rule: viola revive spawns claude in the snapshot's recorded cwd, read only after the strict-modes check, used as the spawn directory and for nothing else, and required to exist; program resolution and the version gate never read it.
why: A ratified boundary widening.
@row_add n=520 row=decision/session-id-argv-closed-shape area=input-validation rec=fct-22lqecbw@1
name: Session id on child argv
rule: A session id placed on a child's command line has the closed shape (36 characters, hyphens at 8, 13, 18, 23, hex elsewhere) and is one argv element of a direct spawn, never a shell, built only by one resume-args builder.
why: A ratified crossing of a logged value onto a child's command line.
@row_add n=521 row=decision/verify-run-e-resume-target area=input-validation rec=fct-22lqecbw@1
name: verify Run E resume target
rule: verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails.
why: Ratified.
@row_add n=522 row=decision/refusals-hold-no-paths-pids-ids area=input-validation rec=fct-22lqecbw@1
name: Refusals hold no paths, pids or ids
rule: No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
@row_add n=523 row=decision/statusline-source-selection area=input-validation rec=fct-xqkexewr@1
name: Statusline source selection
rule: viola run reads the statusline command from the home's statusline-source.json when it exists, else only for the default home the user's Claude settings; a command only from a non-empty command-type statusLine with no NUL; viola writes neither source.
why: Ratified, the file in the viola home chosen.
@row_add n=524 row=decision/statusline-shell-out area=input-validation rec=fct-m6zdbeoz@3
name: Statusline shell-out
rule: The recorded statusline command, the only shell-out, runs only in hook statusline as /bin/sh -c with the command as one argument; stderr is discarded, stdout capped and printed byte for byte, never parsed, and no log line carries the command.
why: A ratified boundary widening.
@row_add n=525 row=decision/statusline-deadline area=input-validation rec=fct-m6zdbeoz@3
name: Statusline command deadline
rule: The wait for the statusline command is bounded at 5 s (detail deadline); a failing command prints nothing.
state: provisional
@row_add n=526 row=decision/max-frame-shared-cap area=input-validation rec=fct-jy3jkgu2@1
name: One shared MAX_FRAME cap
rule: One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps.
why: Sized above the largest expected permission input (Write-tool file bodies).
@row_add n=527 row=decision/max-frame-checked-against-ledger area=input-validation rec=fct-jy3jkgu2@1
name: MAX_FRAME checked against data
rule: verify records the largest hook payload per event kind, stamped per CLI version; a stamped maximum within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_add n=528 row=decision/bounded-readers-census area=input-validation rec=fct-jy3jkgu2@1
name: Every product reader bounded or named
rule: Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness.
guard: product_readers_are_each_bounded_or_named
except: Reads of viola's own executable for the pinned copy are not external input.
@row_add n=529 row=decision/inherited-credentials-never-serialised area=secret-management rec=fct-fdqq3i6o@2
name: Inherited credentials never serialised
rule: Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
@row_add n=530 row=decision/r8-claude-env-strip area=secret-management rec=fct-fdqq3i6o@2
name: R8 CLAUDE environment strip
rule: Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values.
why: Ratified.
@row_add n=531 row=decision/identity-floor-eleven-names area=secret-management rec=fct-fdqq3i6o@2
name: Identity floor stays eleven names
rule: The identity floor stays the eleven names; the plugin, project and env-file names are not identity names.
why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
@row_add n=532 row=contract/claude-child-process-start/env-stripped-known area=secret-management rec=fct-fdqq3i6o@2
boundary: process-start claude-child line
item: env_stripped_known
promise: Names the stripped environment names, never a value.
@row_add n=533 row=contract/claude-child-process-start/env-kept area=secret-management rec=fct-fdqq3i6o@2
boundary: process-start claude-child line
item: env_kept
promise: Names the kept environment names, never a value.
@row_add n=534 row=decision/pre-push-linux-only area=secret-management rec=fct-gecdpdpp@1
name: Pre-push gate Linux only
rule: The local pre-push gate runs only on a Linux host; elsewhere it exits 2 with pre-push-linux-only.
@row_add n=535 row=decision/pre-push-empty-environment-launch area=secret-management rec=fct-gecdpdpp@1
name: Pre-push children launched with an empty environment
rule: The pre-push gate launches every child natively with an emptied environment carrying exactly HOME (from the passwd entry, never its own $HOME) and PATH of constant dirs, so no host value crosses; it has no other launcher and no uid-0 launch.
why: Keeps inherited CLAUDE* credentials out of every child.
except: A host-valued assignment is a boundary widening needing its own Decisions Log entry.
@row_add n=536 row=decision/server-verified-before-any-frame area=security-anti-patterns rec=fct-izf47ccx@2
name: Server verified before any channel frame
rule: No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
@row_add n=537 row=decision/snapshot-strict-checked-before-trust area=security-anti-patterns rec=fct-izf47ccx@2
name: Snapshot reference values only after strict-modes check
rule: The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
@row_add n=538 row=decision/test-removal-own-dirs-only area=security-anti-patterns rec=fct-y573epy2@1
name: Test-side removal only of test-created dirs
rule: A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
@row_add n=539 row=decision/e2e-home-link-guard area=security-anti-patterns rec=fct-y573epy2@1
name: E2E home linked base guard
rule: The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing.
why: a ratified boundary widening: on the dev host the path links outside the working directory
@row_add n=540 row=contract/cli-exit-codes/12 area=security-anti-patterns rec=fct-zvkglgpg@2
boundary: CLI exit codes
item: 12
promise: answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.
@row_add n=541 row=decision/stamps-strict-check-first area=security-anti-patterns rec=fct-b7o2ag2e@1
name: Stamps touched only after strict-modes check
rule: viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
@row_add n=542 row=decision/strict-check-before-narrowing area=security-anti-patterns rec=fct-b7o2ag2e@1
name: Strict-modes check before any narrowing
rule: The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks.
why: a widened directory narrowed to 0700 first would pass the check
@row_add n=543 row=decision/no-control-switch-off area=security-anti-patterns rec=fct-bg5w767t@2
name: No switch turns off a control
rule: No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation.
why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence
except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
@row_add n=544 row=decision/test-seams-fake-agent-only area=security-anti-patterns rec=fct-bg5w767t@2
name: Test seams only under the fake-agent feature
rule: An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
@row_add n=545 row=decision/colour-facts-choose-sgr-only area=security-anti-patterns rec=fct-bg5w767t@2
name: Terminal colour facts choose SGR only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode to choose whether and how deep SGR bytes are written; they change no control, widen no redaction and reach no log line, error or refusal.
why: ratified
@row_add n=546 row=decision/no-seam-forcing-peer-refusal area=security-anti-patterns rec=fct-lvhmwli7@2
name: No seam to force a peer-euid refusal
rule: No env, flag or feature seam forces a refusing peer-euid decision; the decisions are proven over injected values and each client seam's wiring stays the named weakest proof.
why: one account can make no foreign peer
@row_none n=75 rec=fct-qcqqc5fi@1 type=5
@row_none n=76 rec=fct-zfobg22w@1 type=5
@row_add n=547 row=decision/statusline-only-shell-out area=threat-model-summary rec=fct-33o4e7zi@2
name: Statusline command is the only shell-out
rule: The user-written statusline command run through a shell is the only shell-out in viola, and the per-session settings override holds the pinned hook command, never the user's command.
@row_add n=548 row=decision/parent-identity-env-stripped area=threat-model-summary rec=fct-jy2pv6x7@1
name: Parent identity variables stripped from the child
rule: The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
@row_add n=549 row=absence/model-api-calls area=threat-model-summary rec=fct-jy2pv6x7@1
capability: Model API calls of its own; the subscription credentials belong to the unmodified claude CLI
@row_add n=550 row=absence/secrets area=threat-model-summary rec=fct-jy2pv6x7@1,fct-v2lh3e2i@3
capability: Secrets of any kind in v1
@row_add n=551 row=decision/spawn-cwd-snapshot-only area=threat-model-summary rec=fct-qgluq6ji@1
name: Spawn directory kept in the snapshot only
rule: The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent.
why: an absolute host path can carry the OS username
@row_add n=552 row=decision/same-user-trust-boundary area=threat-model-summary rec=fct-t7qeqkiu@2
name: Same OS user trust boundary
rule: The wrapper channel trusts every process of the same OS user and no other; there is no per-connection authentication and the caller's self-reported from is not authenticated.
@row_add n=553 row=decision/socket-in-owner-only-dir area=threat-model-summary rec=fct-t7qeqkiu@2
name: Socket in a per-user owner-only directory
rule: The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
@row_add n=554 row=decision/exclusive-endpoint-bind area=threat-model-summary rec=fct-t7qeqkiu@2
name: Exclusive endpoint bind
rule: viola run binds its endpoint exclusively, so a squatted name blocks startup with exit 1.
@row_add n=555 row=contract/wrapper-channel-methods/send area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: send
promise: Types a prompt into the driven Claude session.
@row_add n=556 row=contract/wrapper-channel-methods/wait area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: wait
promise: Reads the driven session's conversation.
@row_add n=557 row=contract/wrapper-channel-methods/last area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: last
promise: Reads the driven session's latest reply.
@row_add n=558 row=contract/wrapper-channel-methods/answer area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: answer
promise: Answers a dialog, including allowing a PermissionRequest with behavior allow.
@row_add n=559 row=contract/wrapper-channel-methods/pause area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: pause
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_add n=560 row=contract/wrapper-channel-methods/release area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: release
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_add n=561 row=contract/wrapper-channel-methods/link area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: link
promise: Adds a driver link, whose effect is a link line in the event log and the snapshot's links only; the self-reported from is not read.
@row_add n=562 row=contract/wrapper-channel-methods/unlink area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: unlink
promise: Removes a driver link, whose effect is an unlink line in the event log and the snapshot's links only; the self-reported from is not read.
@row_add n=563 row=contract/wrapper-channel-methods/hook-dialog area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: hook.dialog
promise: Carries a dialog from the hook to the wrapper and its decision back.
@row_add n=564 row=contract/wrapper-channel-methods/hook-event area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: hook.event
promise: A notification from the hook to the wrapper.
@row_add n=565 row=decision/client-reads-closed-values-only area=threat-model-summary rec=fct-t7qeqkiu@2
name: Client screens the server's answer to closed values
rule: A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result.
except: a code word outside the closed list stays visible by design
@row_add n=566 row=decision/bridge-send-one-bracketed-paste area=threat-model-summary rec=fct-ql7nqspi@1
name: Bridge send is one bracketed paste
rule: A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
@row_add n=567 row=decision/delivery-confirmation-exact-match area=threat-model-summary rec=fct-ql7nqspi@1
name: Delivery confirmed by exact equality
rule: Delivery confirmation compares the submitted prompt text with the typed text by exact equality, with no tolerance on the submitted side.
@row_add n=568 row=decision/stdin-closed-non-editing-list area=threat-model-summary rec=fct-zyxi3lxw@1
name: Stdin classifier's closed non-editing list
rule: Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess.
why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
@row_add n=569 row=contract/send-refusal/human-typing area=threat-model-summary rec=fct-zyxi3lxw@1
boundary: send refusal
item: human-typing
promise: A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
@row_add n=570 row=contract/wheel-event/human-input area=threat-model-summary rec=fct-zyxi3lxw@1
boundary: wheel event
item: cause human-input
promise: The wheel moved to holder human because a byte on viola run's stdin counted as typing.
@row_add n=571 row=decision/tolerant-hook-parsing area=threat-model-summary rec=fct-6fvyxh3k@1
name: Tolerant hook payload parsing
rule: Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
@row_add n=572 row=decision/upstream-text-is-content area=threat-model-summary rec=fct-6fvyxh3k@1,fct-bur5z45t@1
name: Upstream text is content, never commands
rule: Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola.
why: mechanism, not policy
@row_add n=573 row=decision/hook-strict-check-first area=threat-model-summary rec=fct-6fvyxh3k@1
name: Hook strict-modes check before any read
rule: Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
@row_add n=574 row=decision/replayed-snapshot-yields-nothing area=threat-model-summary rec=fct-6fvyxh3k@1,fct-feql22h5@2
name: Replayed snapshot runs nothing and frames nothing
rule: A snapshot replayed from the log carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command.
@row_add n=575 row=contract/mcp-tools/send area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: send
promise: A tool called by the driver model, input checked by its schema and the ViolaName check.
@row_add n=576 row=contract/mcp-tools/wait area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: wait
promise: A tool called by the driver model that reads the driven session's output.
@row_add n=577 row=contract/mcp-tools/last area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: last
promise: A tool called by the driver model that reads the driven session's latest reply.
@row_add n=578 row=contract/mcp-tools/answer area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: answer
promise: A tool called by the driver model that answers the driven session's dialog, including a PermissionRequest.
@row_add n=579 row=contract/mcp-tools/list area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: list
promise: A tool called by the driver model, input checked by its schema.
@row_add n=580 row=decision/mcp-tool-list-not-enforcement area=threat-model-summary rec=fct-bur5z45t@1
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_add n=581 row=decision/state-tree-strict-modes-before-use area=threat-model-summary rec=fct-qxr7dgpn@2,fct-feql22h5@2
name: State tree strict-modes check before first use
rule: run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
@row_add n=582 row=contract/cli-exit-codes/21 area=threat-model-summary rec=fct-qxr7dgpn@2
boundary: CLI exit codes
item: 21
promise: A frame verb refused because the instance failed the strict-modes check.
@row_add n=583 row=decision/list-home-read-rule area=threat-model-summary rec=fct-qxr7dgpn@2
name: Board scan refuses a readable home
rule: viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
@row_add n=584 row=contract/internal-error-output/json area=threat-model-summary rec=fct-qxr7dgpn@2
boundary: catch-site internal error output
item: --json
promise: Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
@row_add n=585 row=decision/pinned-binary-placed-once area=threat-model-summary rec=fct-a3npzmco@1
name: Pinned binary placed once
rule: The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears.
why: an exec of a copy another start still holds open for write fails with ETXTBSY
@row_add n=586 row=decision/run-pins-own-folder-first-on-path area=threat-model-summary rec=fct-uemrwfo5@2
name: Pinned folder first on the child's PATH
rule: viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
@row_add n=587 row=decision/verify-writes-no-dialog-byte area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify never types into a dialog
rule: No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output.
why: ratified
@row_add n=588 row=decision/nothing-written-under-claude-home area=threat-model-summary rec=fct-uemrwfo5@2
name: Nothing written under the Claude home
rule: viola writes nothing under ~/.claude.
@row_add n=589 row=decision/agents-probe-log-content-free area=threat-model-summary rec=fct-uemrwfo5@2
name: Agents probe log holds no content
rule: An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
@row_add n=590 row=decision/verify-screen-read-against-literals area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify screen read only against compiled literals
rule: verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
@row_add n=591 row=decision/child-output-tolerant-unknown area=threat-model-summary rec=fct-uemrwfo5@2
name: Child output parsing fails to unknown
rule: Child output parsing is tolerant, and a failure becomes unknown.
@row_add n=592 row=decision/sync-crates-tokio-free area=threat-model-summary rec=fct-dvr22577@1
name: Sync crates free of tokio
rule: A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature.
guard: deny-sync.toml; scripts/channel-tokio-free.sh
@row_add n=593 row=decision/toolchains-by-rustup area=threat-model-summary rec=fct-dvr22577@1
name: Toolchains installed by rustup
rule: Every toolchain is installed by rustup, with no toolchain action.
@row_add n=594 row=decision/crates-unpublished area=threat-model-summary rec=fct-dvr22577@1
name: Crates not published
rule: Every crate is publish = false.
@row_add n=595 row=absence/release-stage area=threat-model-summary rec=fct-dvr22577@1,fct-v2lh3e2i@3
capability: A release, signing or deploy stage in v1
@row_add n=596 row=decision/rmcp-features-exact area=threat-model-summary rec=fct-v2lh3e2i@3
name: rmcp features pinned in the release graph
rule: rmcp is present in viola's release graph with exactly the features server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=597 row=decision/release-build-no-test-binaries area=threat-model-summary rec=fct-v2lh3e2i@3
name: Release build refuses test-only binaries
rule: The CI release build refuses any test-only binary.
guard: scripts/release-check.sh
@row_add n=598 row=decision/mutation-testing-at-audit area=threat-model-summary rec=fct-v2lh3e2i@3
name: Mutation testing at the epoch audit
rule: No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
@row_add n=599 row=decision/real-cli-local-only area=threat-model-summary rec=fct-v2lh3e2i@3
name: Real claude CLI only locally
rule: The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
@row_add n=600 row=decision/hook-fails-open area=threat-model-summary rec=fct-3ajwacjp@1,fct-io4zn2wb@2,fct-6fvyxh3k@1
name: Hook fails open to the human
rule: Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr.
why: So the dialog renders for the human.
@row_add n=601 row=decision/readers-tolerate-torn-lines area=threat-model-summary rec=fct-2i3lkhzs@2,fct-feql22h5@2
name: Readers tolerate torn lines
rule: Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
@row_add n=602 row=decision/pinned-artefacts-never-deleted area=threat-model-summary rec=fct-2i3lkhzs@2,fct-feql22h5@2
name: Pinned binary and plugin folders kept
rule: The pinned binary and the plugin folders are never deleted in v1.
@row_add n=603 row=decision/cargo-deny-gate area=threat-model-summary rec=fct-tm2e2x5g@1,fct-dvr22577@1
name: Cargo deny gate on the root graph
rule: Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans.
guard: cargo deny check
except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
@row_add n=604 row=decision/actions-sha-pinned area=threat-model-summary rec=fct-tm2e2x5g@1,fct-dvr22577@1
name: Third-party actions SHA-pinned
rule: Third-party GitHub Actions are SHA-pinned.
@row_add n=605 row=decision/no-abstract-namespace area=threat-model-summary rec=fct-2jp5ovui@2,fct-dthgjfbs@1,fct-2pxk6g7w@2
name: No Linux abstract namespace
rule: The Linux abstract namespace is never used for IPC sockets.
@row_add n=606 row=absence/outbound-network-calls area=threat-model-summary rec=fct-2jp5ovui@2,fct-6tg2dujf@1
capability: Outbound network calls made by viola itself
@row_add n=607 row=contract/send-refusal/empty-text area=input-validation rec=fct-bvljrxyp@1,fct-ub5klpnc@1,fct-ql7nqspi@1
boundary: send refusal
item: empty-text
promise: A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.
@row_add n=608 row=decision/budget-file-single-writer area=input-validation rec=fct-u7v2p5mv@1,fct-65crhg3m@2
name: hook statusline is the only budget writer
rule: budget.json is written by hook statusline alone.
@row_add n=609 row=decision/mcp-stdin-line-cap area=input-validation rec=fct-bngndo2w@1,fct-yxw7xtk5@1
name: MCP stdin lines capped at the frame limit
rule: Each MCP stdin line is capped at MAX_FRAME, its newline counted; an over-long line is refused whole, never truncated or skipped, and ends the session as a stdin close does, its warning holding no byte of it.
why: rmcp reads stdin lines unbounded, so viola hands it a capped reader
@row_add n=610 row=decision/statusline-command-trust area=input-validation rec=fct-brqp4paf@1,fct-wkbf5hxn@1
name: Statusline command trusted via strict modes
rule: statusline_command is trusted only because of the home strict-modes check.
why: Write access to it equals code execution.
@row_add n=611 row=decision/single-shell-out area=input-validation rec=fct-2fgozo5r@2,fct-4py7x3cl@1
name: statusline_command is the only shell-out
rule: The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.
@row_add n=612 row=absence/file-upload area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1
capability: A file upload surface
@row_add n=613 row=absence/webhook area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1
capability: A webhook surface
@row_add n=614 row=absence/database area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1,fct-cy2wvz77@1,fct-c3b3fd57@1
capability: A database or query language
@row_add n=615 row=decision/no-tokio-in-sync-crates area=dependency-security rec=fct-cavsczz7@1,fct-o63eny7e@2
name: No tokio in the sync crates
rule: tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root.
except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh
@row_add n=616 row=decision/actions-pinned-by-sha area=dependency-security rec=fct-dc3z7vyg@1,fct-vtfmovh3@1,fct-dj5p5k3a@1
name: GitHub Actions pinned by full commit SHA
rule: Every GitHub Action is referenced by a full commit SHA with a version comment, never by @stable, @v2 or any mutable ref, and actions/checkout runs with persist-credentials: false.
@row_add n=617 row=decision/signed-self-update area=dependency-security rec=fct-w6kcw2wp@1,fct-qhcowchx@1,fct-heudg7au@1
name: self_update only with signatures
rule: self_update never ships without its signatures feature (zipsign); a v1.x release requires self_update 1.3.0 built with signatures.
why: a checksum from the same release proves nothing
@row_add n=618 row=decision/no-rust-cache-in-release area=dependency-security rec=fct-xp6cvi7s@1,fct-qhcowchx@1,fct-cpjuytxt@1
name: No rust-cache in the release workflow
rule: Swatinem/rust-cache is never restored in the v1.x release workflow.
@row_add n=619 row=decision/no-repo-secret-scanner area=bootstrap-phases-derive-for-route-setup-project rec=fct-rwpm3oj4@1,fct-ypiftj4r@1,fct-f326yvjk@1
name: No repository secret scanner in v1
rule: v1 runs no repository secret scanner; revisit when the v1.x release workflow adds signing credentials.
why: there are no secrets in the repo or CI
@row_add n=620 row=decision/no-stored-secrets area=secret-management rec=fct-ugohyrrh@2,fct-fdqq3i6o@2
name: No stored secrets
rule: There are no stored secrets in v1, and environment variables are not a configuration channel.
except: the test-only seams compiled out of release builds, and the terminal colour facts read by viola list's human mode
@row_add n=621 row=decision/tests-use-own-home area=secret-management rec=fct-z4tolcdo@1,fct-gecdpdpp@1
name: Tests use their own home
rule: Development handles secrets as production does, and tests use their own --home directory.
@row_add n=622 row=decision/dialog-answers-need-verify-stamp area=security-anti-patterns-never-do-these rec=fct-syrndlhx@1,fct-zvkglgpg@2
name: Dialog answers need a verify stamp
rule: A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes.
why: a stamp is what switches on dialog answering
@row_add n=623 row=decision/stamps-written-only-by-verify area=security-anti-patterns-never-do-these rec=fct-syrndlhx@1,fct-zvkglgpg@2,fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Only viola verify writes the stamps file
rule: No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600.
why: a stamp is what switches on dialog answering, PermissionRequest allow included
@row_add n=624 row=decision/version-gate-strict-stamps-read area=security-anti-patterns-never-do-these rec=fct-7izg6s4l@1,fct-wkbf5hxn@1
name: Version gate reads stamps through the strict-modes check
rule: A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
@row_add n=625 row=decision/strict-modes-check area=authentication-authorization rec=fct-65crhg3m@2,fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Strict-modes check on trusted state
rule: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission.
guard: security_negatives_strict_modes, chaos_snapshot_replay
@row_add n=626 row=decision/marketplace-off-integrity-set area=authentication-authorization rec=fct-65crhg3m@2,fct-feql22h5@2
name: marketplace/ off the integrity set
rule: marketplace/ is no trusted file and is off the integrity set: viola never reads it back, and the installed command gets no re-hash.
@row_add n=627 row=contract/instance-unreachable-detail/instance-dead area=authentication-authorization rec=fct-6lhjfvvj@1,fct-6ntn52d2@1,fct-au76dwrt@1,fct-fmkuqvun@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: instance-dead
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with the instance directory standing (a stale wrapper included), or a failed connect; the reading opens nothing and writes no line.
@row_add n=628 row=contract/instance-unreachable-detail/unwrapped-name area=authentication-authorization rec=fct-6lhjfvvj@1,fct-6ntn52d2@1,fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: unwrapped-name
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent, a regular file or a symlink); the reading opens nothing and writes no line.
@row_add n=629 row=contract/instance-unreachable-detail/strict-modes-failed area=error-handling rec=fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: strict-modes-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the instance's strict-modes check refused, and no frame was sent.
@row_add n=630 row=contract/instance-unreachable-detail/endpoint-lost area=error-handling rec=fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: endpoint-lost
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the wrapper was lost with the call open.
@row_add n=631 row=contract/instance-unreachable-detail/server-verify-failed area=error-handling rec=fct-au76dwrt@1,fct-pfehx2bz@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: server-verify-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the peer euid decision refused past the connect, and no frame was sent; a peer euid refusal has no word of its own and reads server-verify-failed.
@row_add n=632 row=contract/instance-unreachable-detail/socket-dir-refused area=error-handling rec=fct-au76dwrt@1,fct-iaqygteq@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: socket-dir-refused
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the socket directory of the recorded endpoint was refused, and no frame was sent; on a CLI frame verb it is during connect, one process-exit at WARN and no channel-request.
@row_add n=633 row=decision/channel-closed-types area=input-validation rec=fct-wrh45y73@1
name: Channel closed types
rule: Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
@row_add n=634 row=decision/capture-answers-closed-probe-ids area=input-validation rec=fct-p4cpzbge@1
name: Capture answers from closed probe ids
rule: --answers is read only with --capture; each answer file is read through take(64) and matched exactly against the closed ProbeAnswer ids, the body built only through dialog::decision_body.
why: Ratified: the hook answers, never a key.
@row_add n=635 row=decision/verify-children-direct-spawn area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify children spawned directly
rule: viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals.
why: Ratified live rulings, each given after the widening was shown.
