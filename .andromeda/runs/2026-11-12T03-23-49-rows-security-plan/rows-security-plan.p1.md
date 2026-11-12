@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=0
what: an editing note on when the document may be changed
@row_add n=1 row=overview/security-plan-viola area=security-plan-viola rec=3
text: viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract, with no public network surface. Its IPC, filesystem and PTY controls stand in for the auth, database and API concerns of a web stack.
@row_none n=3 rec=4 type=S
@row_none n=4 rec=5 type=S
@row_add n=2 row=decision/security-tier area=threat-model-summary rec=6
name: Security tier
rule: The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home.
why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
@row_none n=5 rec=7 type=S
@row_add n=3 row=decision/data-classes area=threat-model-summary rec=8
name: Data classes
rule: viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
@row_add n=4 row=decision/cwd-only-in-snapshot area=threat-model-summary rec=9
name: Recorded cwd kept to the snapshot
rule: The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
@row_add n=5 row=absence/regulated-personal-data area=threat-model-summary rec=10,33,34,35,36
capability: Payment, health, children's or multi-user personal data of any kind
@row_none n=6 rec=11 type=S
@row_add n=6 row=decision/os-user-ambient-trust area=threat-model-summary rec=12,24,42,27
name: OS-user ambient trust
rule: The only principal is the local OS user: the IPC endpoints and the viola home trust every process of the same OS user and no other, with no per-connection authentication.
why: viola is a single-user local tool with no accounts or hosting, driven by agents with no interactive login, which matches an ambient OS-user trust model rather than a credential flow.
@row_add n=7 row=decision/from-field-authenticates-nothing area=threat-model-summary rec=12
name: Self-reported from authenticates nothing
rule: The self-reported from field on the wrapper channel authenticates nothing and is never treated as an identity.
@row_none n=7 rec=13 type=4
relied: The send text is written into the PTY as one bracketed paste, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes.
@row_add n=8 row=decision/human-typing-takes-wheel area=threat-model-summary rec=14
name: Human typing takes the wheel
rule: A byte on viola run's own stdin that is typing takes the wheel from the driver.
except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
@row_add n=9 row=decision/non-editing-list-measured-only area=threat-model-summary rec=14
name: Non-editing list holds measured replies only
rule: A terminal reply is added to the stdin classifier's non-editing list only once a terminal has been measured to write it, never on a guess.
@row_add n=10 row=decision/hook-payload-is-content area=threat-model-summary rec=15
name: Hook payload is content
rule: Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
@row_add n=11 row=decision/hook-fails-open area=threat-model-summary rec=15
name: Hook fails open to the human
rule: Any failure in a viola hook fails open to the human.
@row_add n=12 row=decision/statusline-command-integrity area=threat-model-summary rec=15
name: Statusline command integrity
rule: The hook statusline runs statusline_command through a shell, its integrity resting on filesystem permissions of the viola home.
@row_add n=13 row=decision/mcp-input-checked area=threat-model-summary rec=16
name: MCP input checked by schema
rule: MCP tool input is checked by the tool input schemas and the ViolaName newtype.
@row_add n=14 row=decision/mcp-frame-cap area=threat-model-summary rec=16
name: MCP stdin line cap
rule: Each MCP stdin line is capped at MAX_FRAME before rmcp reads it; an over-long line is refused whole and ends the session, never dispatched in part.
@row_add n=15 row=decision/driver-permission-policy-outside-viola area=threat-model-summary rec=16,53
name: Driver prompt injection accepted
rule: The driver model reads untrusted driven-session output and may be prompt-injected into answering a PermissionRequest; what to answer rests with the driver, outside viola.
why: What to answer is policy outside viola; an accepted risk.
@row_add n=16 row=decision/mcp-tool-list-affordance area=threat-model-summary rec=16
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_add n=17 row=contract/cli-state-root/home-flag area=threat-model-summary rec=17
boundary: CLI state root selection
item: --home
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_add n=18 row=contract/cli-state-root/viola-dir area=threat-model-summary rec=17
boundary: CLI state root selection
item: VIOLA_DIR
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_add n=19 row=decision/cli-input-trust-boundary area=threat-model-summary rec=17
name: CLI input trust boundary
rule: CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
@row_add n=20 row=decision/readers-tolerate-torn-lines area=threat-model-summary rec=18
name: Readers tolerate torn lines
rule: Readers of state under the viola home tolerate torn lines.
@row_add n=21 row=decision/pinned-artefacts-never-deleted area=threat-model-summary rec=18
name: Pinned artefacts never deleted
rule: The pinned binary and the plugin folders are never deleted in v1.
@row_add n=22 row=decision/strict-modes-at-start area=threat-model-summary rec=18
name: Strict-modes check at start
rule: Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
@row_add n=23 row=decision/child-path-trust area=threat-model-summary rec=19
name: Child processes trust PATH
rule: Spawned child processes are whatever the calling process's PATH resolves to.
@row_add n=24 row=decision/child-output-tolerant area=threat-model-summary rec=19
name: Child output parsed tolerantly
rule: Output of spawned children is parsed tolerantly and a failure becomes unknown.
@row_add n=25 row=decision/verify-signature-only area=threat-model-summary rec=19
name: Verify reads signatures only
rule: viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
@row_add n=26 row=decision/verify-never-answers-dialogs area=threat-model-summary rec=20
name: Verify never answers dialogs
rule: viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
@row_add n=27 row=decision/agents-probe-log-redaction area=threat-model-summary rec=20
name: Agents-probe logging redacted
rule: Each agents-probe spawn pair is logged with no row text, id, cwd or path.
@row_add n=28 row=decision/cargo-deny-gate area=threat-model-summary rec=21
name: Dependency audit gate
rule: Cargo dependencies are held to the cargo deny check.
guard: cargo deny check
@row_add n=29 row=decision/sync-crate-tokio-ban area=threat-model-summary rec=21
name: No tokio in sync crates
rule: tokio is banned from the sync crates.
@row_add n=30 row=decision/actions-sha-pinned area=threat-model-summary rec=21
name: SHA-pinned third-party actions
rule: Third-party GitHub Actions are pinned by SHA.
@row_add n=31 row=decision/no-publish area=threat-model-summary rec=21
name: Crates not publishable
rule: Crates carry publish = false and builds rely on the committed Cargo.lock.
@row_add n=32 row=decision/fuzz-lockfile-own-audit area=threat-model-summary rec=21
name: Fuzz lockfile audited on its own
rule: The test-only fuzz lockfile is audited on its own, apart from the main dependency audit.
why: A ratified exemption.
@row_add n=33 row=absence/release-and-deploy area=threat-model-summary rec=21,32
capability: A release, signing or deploy stage with release artefacts
@row_add n=34 row=absence/public-network-listener area=threat-model-summary rec=22,3
capability: A public network listener or public internet API
@row_add n=35 row=absence/websocket area=threat-model-summary rec=22
capability: WebSocket or polling endpoints
@row_add n=36 row=absence/hosting area=threat-model-summary rec=22,29
capability: Hosting or containerised deployment of any kind
@row_none n=8 rec=23 type=S
@row_add n=37 row=decision/auth-self-implemented area=threat-model-summary rec=25
name: No auth framework
rule: Authentication is self-implemented on OS mechanisms, with no auth framework or identity SDK.
@row_add n=38 row=absence/remote-access area=threat-model-summary rec=26
capability: Remote or phone access to viola
@row_none n=9 rec=28 type=S
@row_add n=39 row=decision/local-cargo-install area=threat-model-summary rec=29
name: Local install from source
rule: viola is installed locally with cargo install --path . and runs only on the local machine.
@row_add n=40 row=decision/flat-file-state-store area=threat-model-summary rec=30
name: Flat-file state store
rule: State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
@row_add n=41 row=decision/plaintext-state area=threat-model-summary rec=30
name: Plaintext state
rule: Nothing in the state store is encrypted; all of it is plaintext.
why: Readability, by design.
@row_add n=42 row=decision/socket-in-per-user-dir area=threat-model-summary rec=31
name: Sockets in a per-user directory
rule: Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
@row_add n=43 row=decision/no-abstract-namespace area=threat-model-summary rec=31,48
name: No Linux abstract namespace
rule: IPC sockets never use the Linux abstract namespace.
@row_add n=44 row=decision/mcp-over-stdio area=threat-model-summary rec=31
name: MCP over stdio
rule: The MCP server talks over stdio only.
@row_add n=45 row=absence/outbound-network-calls area=threat-model-summary rec=31
capability: Outbound network calls made by viola itself
@row_add n=46 row=decision/ci-no-secrets-no-real-claude area=threat-model-summary rec=32
name: CI holds no secrets
rule: CI holds no secrets and never runs the real claude CLI, which runs only locally.
@row_add n=47 row=absence/third-party-personal-data area=threat-model-summary rec=37
capability: Collection of personal data from third parties
@row_none n=10 rec=38 type=0
what: an assessment note that a later public distribution adds no compliance trigger
@row_none n=11 rec=39 type=0
what: a note on the tier's role for downstream skills
@row_none n=12 rec=40 type=S
@row_none n=13 rec=41 type=S
@row_none n=14 rec=43 type=S
@row_add n=48 row=decision/interprocess-peer-creds area=authentication-authorization rec=44
name: interprocess peer credentials
rule: interprocess's peer credentials over SO_PEERCRED are what read the peer's pid and euid on both sides of the IPC channel.
@row_add n=49 row=decision/sysinfo-process-identity area=authentication-authorization rec=44
name: sysinfo for process identity
rule: sysinfo is what reads a process's pid and start time for server verification.
@row_add n=50 row=decision/no-listener-mode-option area=authentication-authorization rec=44
name: Socket mode set by chmod
rule: The bound Unix socket's mode is set with std chmod, and interprocess's listener mode option is not used.
@row_none n=15 rec=45 type=0
what: an adapter note on the OS-principal model
@row_add n=51 row=absence/user-accounts-passwords area=authentication-authorization rec=46
capability: User accounts with passwords
@row_add n=52 row=decision/all-or-nothing-access area=authentication-authorization rec=47
name: All-or-nothing access
rule: Any caller that passes the OS check gets the full surface, with no roles.
@row_add n=53 row=decision/socket-dir-checked area=authentication-authorization rec=48
name: Socket directory checked
rule: The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
@row_add n=54 row=decision/socket-chmod-after-bind area=authentication-authorization rec=48
name: Socket owner-only after bind
rule: The Unix socket is set to mode 0600 right after the bind.
@row_add n=55 row=decision/no-socket-overwrite area=authentication-authorization rec=48
name: No socket overwrite
rule: Binding never overwrites an existing socket; try_overwrite stays off.
@row_add n=56 row=decision/server-peer-euid-check area=authentication-authorization rec=49
name: Server-side peer euid check
rule: On every accept, a peer whose euid differs from the server's is dropped before a frame is read.
@row_add n=57 row=decision/client-strict-modes-first area=authentication-authorization rec=50
name: Client strict-modes check first
rule: Before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line.
@row_add n=58 row=decision/client-verifies-server area=authentication-authorization rec=50
name: Client verifies the server
rule: The client verifies the connected peer's pid and that process's start time against its one snapshot read; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect.
@row_add n=59 row=decision/client-peer-euid-check area=authentication-authorization rec=50
name: Client-side peer euid check
rule: Past the connect, the client admits only a server of its own euid and refuses an unreadable peer, sending no frame.
@row_add n=60 row=contract/ipc-client-refusals/socket-dir-refused area=authentication-authorization rec=50
boundary: IPC client refusals
item: socket-dir-refused
promise: Reported when the recorded endpoint's socket directory fails its check before the connect; no frame is sent.
@row_add n=61 row=contract/ipc-client-refusals/server-verify-failed area=authentication-authorization rec=50
boundary: IPC client refusals
item: server-verify-failed
promise: Reported when the connected peer is not of the client's euid or cannot be read; no frame is sent.
@row_add n=62 row=decision/driver-release-refused area=authentication-authorization rec=51
name: Driver-originated release refused
rule: The wrapper refuses a release whose params carry a string from, before any budget is acted on.
why: An affordance-level guard against drivers handing the wheel back to automation, not enforcement, since a same-user process can leave out from.
@row_add n=63 row=contract/release-errors/release-from-driver area=authentication-authorization rec=51
boundary: The release method's errors
item: release-from-driver
promise: A release carrying a string from is refused with code -32602 and reason release-from-driver.
@row_add n=64 row=decision/home-owner-only area=authentication-authorization rec=52
name: Owner-only viola home
rule: The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
@row_add n=65 row=decision/strict-modes-before-read area=authentication-authorization rec=52
name: Strict-modes check before trusted reads
rule: Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
@row_add n=66 row=decision/answer-behavior-closed-enum area=authentication-authorization rec=53
name: Closed answer behaviour
rule: An answer's behavior is a closed enum.
@row_add n=67 row=decision/unverified-cli-gate area=authentication-authorization rec=53
name: Unverified CLI gate
rule: No non-null answer decision is sent without a viola verify stamp for the child's CLI version.
@row_none n=16 rec=54 type=S
@row_none n=17 rec=55 type=S
