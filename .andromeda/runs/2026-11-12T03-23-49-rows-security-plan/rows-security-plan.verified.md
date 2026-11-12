@row_add n=1 row=overview/security-plan-viola area=security-plan-viola rec=fct-odkkhtym@1
text: viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract, with no public network surface. Its IPC, filesystem and PTY controls stand in for the auth, database and API concerns of a web stack.
@row_check n=1 row=overview/security-plan-viola sha=fdfb440eaf82 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:100
quote: } else if arg == "mcp" {
at: src/main.rs:98
quote: return if arg == "hook" {
at: Cargo.toml:207
quote: interprocess = "=2.4.4"
searched: TcpListener|TcpStream|UdpSocket · in src/
@row_add n=2 row=decision/security-tier area=threat-model-summary rec=fct-55ydrrgu@2
name: Security tier
rule: The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home.
why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
@row_check n=2 row=decision/security-tier sha=81f47af3f36e v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=3 row=decision/data-classes area=threat-model-summary rec=fct-xhdfgari@1
name: Data classes
rule: viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
@row_check n=3 row=decision/data-classes sha=d14d749fa8b4 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=4 row=decision/cwd-only-in-snapshot area=threat-model-summary rec=fct-l2iemwjy@1
name: Recorded cwd kept to the snapshot
rule: The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
@row_check n=4 row=decision/cwd-only-in-snapshot sha=041272242937 v=holds judge=sonnet head=8834195b0683
searched: cwd = · in src/**/*.rs and crates/**/*.rs (no obs_event/println/eprintln carries cwd)
searched: (println|eprintln|obs_event).*cwd · in src/**/*.rs and crates/**/*.rs
@row_add n=5 row=absence/regulated-personal-data area=threat-model-summary rec=fct-fqtuwzzv@1,fct-2xvd2675@1,fct-37wlu7mu@1,fct-f4oo2w3w@1,fct-ruqn4gwy@1
capability: Payment, health, children's or multi-user personal data of any kind
@row_check n=5 row=absence/regulated-personal-data sha=18272427eaa6 v=holds judge=sonnet head=8834195b0683
searched: stripe|payment|health · in Cargo.toml (case-insensitive)
@row_add n=6 row=decision/os-user-ambient-trust area=threat-model-summary rec=fct-qtqta3hw@1,fct-s6dcdjs6@2,fct-qkuk7pih@2,fct-34c4k36s@1
name: OS-user ambient trust
rule: The only principal is the local OS user: the IPC endpoints and the viola home trust every process of the same OS user and no other, with no per-connection authentication.
why: viola is a single-user local tool with no accounts or hosting, driven by agents with no interactive login, which matches an ambient OS-user trust model rather than a credential flow.
@row_check n=6 row=decision/os-user-ambient-trust sha=18ae9cb7c9c8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:144
quote: pub fn peer_admitted(observed: Option<u32>) -> bool {
@row_add n=7 row=decision/from-field-authenticates-nothing area=threat-model-summary rec=fct-qtqta3hw@1
name: Self-reported from authenticates nothing
rule: The self-reported from field on the wrapper channel authenticates nothing and is never treated as an identity.
@row_check n=7 row=decision/from-field-authenticates-nothing sha=01d547e696b2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
@row_add n=8 row=decision/human-typing-takes-wheel area=threat-model-summary rec=fct-hscavt6w@1
name: Human typing takes the wheel
rule: A byte on viola run's own stdin that is typing takes the wheel from the driver.
except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
@row_check n=8 row=decision/human-typing-takes-wheel sha=fcedcf937797 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:405
quote: true
at: src/run/wheel.rs:350
quote: /// Which bytes on stdin are typing. Every byte is, except the sequences on F-W2's closed list:
@row_add n=9 row=decision/non-editing-list-measured-only area=threat-model-summary rec=fct-hscavt6w@1
name: Non-editing list holds measured replies only
rule: A terminal reply is added to the stdin classifier's non-editing list only once a terminal has been measured to write it, never on a guess.
@row_check n=9 row=decision/non-editing-list-measured-only sha=cfb12f67a6c0 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=10 row=decision/hook-payload-is-content area=threat-model-summary rec=fct-3ajwacjp@1
name: Hook payload is content
rule: Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
@row_check n=10 row=decision/hook-payload-is-content sha=57428e015940 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:95
quote: #[serde(default)]
searched: deny_unknown_fields · in crates/viola-agent-claude/src
@row_add n=11 row=decision/statusline-command-integrity area=threat-model-summary rec=fct-3ajwacjp@1
name: Statusline command integrity
rule: The hook statusline runs statusline_command through a shell, its integrity resting on filesystem permissions of the viola home.
@row_check n=11 row=decision/statusline-command-integrity sha=ad0db2d3b011 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
@row_add n=12 row=decision/mcp-input-checked area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP input checked by schema
rule: MCP tool input is checked by the tool input schemas and the ViolaName newtype.
@row_check n=12 row=decision/mcp-input-checked sha=e5b713ed8542 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
@row_add n=13 row=decision/mcp-frame-cap area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP stdin line cap
rule: Each MCP stdin line is capped at MAX_FRAME before rmcp reads it; an over-long line is refused whole and ends the session, never dispatched in part.
@row_check n=13 row=decision/mcp-frame-cap sha=16ec097232a5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:60-63
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed);
@row_add n=14 row=decision/driver-permission-policy-outside-viola area=threat-model-summary rec=fct-kfezk4o5@1,fct-ty6c2nnj@1
name: Driver prompt injection accepted
rule: The driver model reads untrusted driven-session output and may be prompt-injected into answering a PermissionRequest; what to answer rests with the driver, outside viola.
why: What to answer is policy outside viola; an accepted risk.
@row_check n=14 row=decision/driver-permission-policy-outside-viola sha=282116441198 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=15 row=decision/mcp-tool-list-affordance area=threat-model-summary rec=fct-kfezk4o5@1
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_check n=15 row=decision/mcp-tool-list-affordance sha=800bbd7e475e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=16 row=contract/cli-state-root/home-flag area=threat-model-summary rec=fct-zxlwdggu@1
boundary: CLI state root selection
item: --home
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_check n=16 row=contract/cli-state-root/home-flag sha=51d1326da7a5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294-296
quote: if let Some(home) = flag {         return Ok(home);     }
@row_add n=17 row=contract/cli-state-root/viola-dir area=threat-model-summary rec=fct-zxlwdggu@1
boundary: CLI state root selection
item: VIOLA_DIR
promise: Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
@row_check n=17 row=contract/cli-state-root/viola-dir sha=1b5629c1501f v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:297-299
quote: if let Some(instance) = instance_of(name, dir) {         return Ok(instance.home);     }
@row_add n=18 row=decision/cli-input-trust-boundary area=threat-model-summary rec=fct-zxlwdggu@1
name: CLI input trust boundary
rule: CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
@row_check n=18 row=decision/cli-input-trust-boundary sha=ddcb152d0e7b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:29
quote: #[arg(value_parser = parse_name)]
at: src/cmd/send.rs:44
quote: ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
at: src/obs.rs:184
quote: match File::open(home.join("config.json")) {
@row_add n=19 row=decision/strict-modes-at-start area=threat-model-summary rec=fct-2i3lkhzs@2
name: Strict-modes check at start
rule: Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
@row_check n=19 row=decision/strict-modes-at-start sha=fecf152b1390 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: crates/viola-state/src/strict.rs:104
quote: (bin.join("viola"), Rule::Write),
at: crates/viola-state/src/strict.rs:106
quote: (plugin.join("hooks"), Rule::Write),
@row_add n=20 row=decision/child-path-trust area=threat-model-summary rec=fct-m3uflr5q@1
name: Child processes trust PATH
rule: Spawned child processes are whatever the calling process's PATH resolves to.
@row_check n=20 row=decision/child-path-trust sha=24d510846519 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:143
quote: let path = std::env::var_os("PATH");
@row_add n=21 row=decision/child-output-tolerant area=threat-model-summary rec=fct-m3uflr5q@1
name: Child output parsed tolerantly
rule: Output of spawned children is parsed tolerantly and a failure becomes unknown.
@row_check n=21 row=decision/child-output-tolerant sha=2611bd186ebb v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:93-94
quote: let Ok(Value::Array(elements)) = serde_json::from_slice::<Value>(bytes) else { return Err(AgentsFailure::Malformed);
at: src/run/agents.rs:69-70
quote: let Ok(ran) = ran else { return unread(false);
at: crates/viola-state/src/board.rs:236
quote: status: SessionStatus::Unknown,
@row_add n=22 row=decision/verify-signature-only area=threat-model-summary rec=fct-m3uflr5q@1
name: Verify reads signatures only
rule: viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
@row_check n=22 row=decision/verify-signature-only sha=4132cd323aeb v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:41-42
quote: input_box: &["for agents", "(shift+tab to cycle)"], modals: &["Yes, I trust this folder", "Yes, allow external imports"],
at: src/cmd/verify/typed/pty.rs:147
quote: .is_some_and(|rows| rows.iter().any(|r| SIGNATURES.holds_any(r)));
@row_add n=23 row=decision/verify-never-answers-dialogs area=threat-model-summary rec=fct-uhjzl3zt@1
name: Verify never answers dialogs
rule: viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
@row_check n=23 row=decision/verify-never-answers-dialogs sha=e155c3309d20 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/run.rs:180-184
quote: /// Ends the child with no key. pub(super) fn end_by_kill(self) { self.kill(); self.finish(); }
at: src/cmd/verify/typed/turns.rs:76
quote: holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
@row_add n=24 row=decision/agents-probe-log-redaction area=threat-model-summary rec=fct-uhjzl3zt@1
name: Agents-probe logging redacted
rule: Each agents-probe spawn pair is logged with no row text, id, cwd or path.
@row_check n=24 row=decision/agents-probe-log-redaction sha=57b789d5df78 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:62-67
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = "agents-probe", child_exit_status = status, duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
@row_add n=25 row=decision/sync-crate-tokio-ban area=threat-model-summary rec=fct-tm2e2x5g@1
name: No tokio in sync crates
rule: tokio is banned from the sync crates.
@row_check n=25 row=decision/sync-crate-tokio-ban sha=8ab7da002388 v=holds judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
@row_add n=26 row=decision/no-publish area=threat-model-summary rec=fct-tm2e2x5g@1
name: Crates not publishable
rule: Crates carry publish = false and builds rely on the committed Cargo.lock.
@row_check n=26 row=decision/no-publish sha=5808fca1af88 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:190
quote: publish = false
at: crates/viola-state/Cargo.toml:7
quote: publish.workspace = true
@row_add n=27 row=decision/fuzz-lockfile-own-audit area=threat-model-summary rec=fct-tm2e2x5g@1
name: Fuzz lockfile audited on its own
rule: The test-only fuzz lockfile is audited on its own, apart from the main dependency audit.
why: A ratified exemption.
@row_check n=27 row=decision/fuzz-lockfile-own-audit sha=940c02cc38dd v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
@row_add n=28 row=absence/release-and-deploy area=threat-model-summary rec=fct-tm2e2x5g@1,fct-ju2d5wlo@2
capability: A release, signing or deploy stage with release artefacts
@row_check n=28 row=absence/release-and-deploy sha=f40ba4403c00 v=holds judge=sonnet head=8834195b0683
searched: gh release|cosign|sigstore|gpg|softprops|deploy:|cargo publish|action-gh-release|attest · in .github/
searched: publish = false · in Cargo.toml (workspace package)
@row_add n=29 row=absence/public-network-listener area=threat-model-summary rec=fct-k62evpgk@1,fct-odkkhtym@1
capability: A public network listener or public internet API
@row_check n=29 row=absence/public-network-listener sha=8c4cdd25248c v=holds judge=sonnet head=8834195b0683
searched: TcpListener|TcpStream|SocketAddr|0.0.0.0 · in src/**/*.rs, crates/*/src/**/*.rs
searched: axum|hyper|reqwest|tiny_http · in Cargo.toml, crates/*/Cargo.toml
@row_add n=30 row=absence/websocket area=threat-model-summary rec=fct-k62evpgk@1
capability: WebSocket or polling endpoints
@row_check n=30 row=absence/websocket sha=6be7a43724c9 v=holds judge=sonnet head=8834195b0683
searched: WebSocket|tungstenite · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml
@row_add n=31 row=absence/hosting area=threat-model-summary rec=fct-k62evpgk@1,fct-nj4c3hrg@1
capability: Hosting or containerised deployment of any kind
@row_check n=31 row=absence/hosting sha=d61f2a656642 v=holds judge=sonnet head=8834195b0683
searched: Dockerfile|docker · in src/**/*.rs, crates/*/src/**/*.rs, scripts/*, .github/**
searched: Dockerfile* · in repository root glob
@row_add n=32 row=decision/auth-self-implemented area=threat-model-summary rec=fct-gck5qvs6@1
name: No auth framework
rule: Authentication is self-implemented on OS mechanisms, with no auth framework or identity SDK.
@row_check n=32 row=decision/auth-self-implemented sha=3986c5336b07 v=holds judge=sonnet head=8834195b0683
at: deny.toml:45
quote: deny = ["transport-streamable-http-server", "auth"]
@row_add n=33 row=absence/remote-access area=threat-model-summary rec=fct-c4azjngh@1
capability: Remote or phone access to viola
@row_check n=33 row=absence/remote-access sha=145fec7bc9a2 v=holds judge=sonnet head=8834195b0683
searched: TcpListener|0.0.0.0|tungstenite|axum|hyper · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml
@row_add n=34 row=decision/local-cargo-install area=threat-model-summary rec=fct-nj4c3hrg@1
name: Local install from source
rule: viola is installed locally with cargo install --path . and runs only on the local machine.
@row_check n=34 row=decision/local-cargo-install sha=c59bfc784d68 v=unverifiable judge=sonnet head=8834195b0683
searched: cargo install --path|install --path · in src/**/*.rs, scripts/*, .github/**, Cargo.toml
why: no evidence found
@row_add n=35 row=decision/flat-file-state-store area=threat-model-summary rec=fct-y7373awj@1
name: Flat-file state store
rule: State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
@row_check n=35 row=decision/flat-file-state-store sha=094537b3cbca v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:19
quote: const EVENTS_LOCK: &str = "events.ndjson.lock";
at: crates/viola-state/src/fs.rs:77
quote: pub fn replace_private(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> {
at: crates/viola-state/src/snapshot.rs:18
quote: const SNAPSHOT_LOCK: &str = "snapshot.json.lock";
@row_add n=36 row=decision/plaintext-state area=threat-model-summary rec=fct-y7373awj@1
name: Plaintext state
rule: Nothing in the state store is encrypted; all of it is plaintext.
why: Readability, by design.
@row_check n=36 row=decision/plaintext-state sha=353315ec0772 v=holds judge=sonnet head=8834195b0683
searched: encrypt|cipher|aes|chacha|keyring|crypto (case-insensitive) · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=37 row=decision/socket-in-per-user-dir area=threat-model-summary rec=fct-2jp5ovui@2
name: Sockets in a per-user directory
rule: Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
@row_check n=37 row=decision/socket-in-per-user-dir sha=19774cbb843f v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-channel/src/endpoint.rs:39
quote: Some(xdg) => Path::new(xdg).join("viola"),
at: crates/viola-channel/src/endpoint.rs:40
quote: None => PathBuf::from(format!("{host path}{uid}")),
@row_add n=38 row=decision/mcp-over-stdio area=threat-model-summary rec=fct-2jp5ovui@2
name: MCP over stdio
rule: The MCP server talks over stdio only.
@row_check n=38 row=decision/mcp-over-stdio sha=620afddd9579 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
@row_add n=39 row=decision/ci-no-secrets-no-real-claude area=threat-model-summary rec=fct-ju2d5wlo@2
name: CI holds no secrets
rule: CI holds no secrets and never runs the real claude CLI, which runs only locally.
@row_check n=39 row=decision/ci-no-secrets-no-real-claude sha=79da2db08d0c v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
searched: secrets|GITHUB_TOKEN|token (case-insensitive) · in .github/workflows
searched: claude (case-insensitive word) · in .github/workflows
@row_add n=40 row=absence/third-party-personal-data area=threat-model-summary rec=fct-wmp6i3hm@1
capability: Collection of personal data from third parties
@row_check n=40 row=absence/third-party-personal-data sha=40d7f236754d v=holds judge=sonnet head=8834195b0683
searched: TcpListener|TcpStream|reqwest|ureq|api.anthropic|UdpSocket · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml
@row_add n=41 row=decision/interprocess-peer-creds area=authentication-authorization rec=fct-7nb5glkx@2
name: interprocess peer credentials
rule: interprocess's peer credentials over SO_PEERCRED are what read the peer's pid and euid on both sides of the IPC channel.
@row_check n=41 row=decision/interprocess-peer-creds sha=26b9332bed31 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:133
quote: let pid = stream.peer_creds().ok()?.pid()?;
at: crates/viola-channel/src/client.rs:156
quote: stream.peer_creds().ok()?.euid()
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=42 row=decision/sysinfo-process-identity area=authentication-authorization rec=fct-7nb5glkx@2
name: sysinfo for process identity
rule: sysinfo is what reads a process's pid and start time for server verification.
@row_check n=42 row=decision/sysinfo-process-identity sha=56e43a5a026d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:56
quote: .map(sysinfo::Process::start_time)
at: crates/viola-state/src/liveness.rs:157
quote: let observed_start = observed_pid.and_then(process_start_time);
@row_add n=43 row=decision/no-listener-mode-option area=authentication-authorization rec=fct-7nb5glkx@2
name: Socket mode set by chmod
rule: The bound Unix socket's mode is set with std chmod, and interprocess's listener mode option is not used.
@row_check n=43 row=decision/no-listener-mode-option sha=c0fbdb8c9cdd v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-channel/src/server.rs:132
quote: // After the bind, not `ListenerOptionsExt::mode`: the 0700 directory already closes the umask
@row_add n=44 row=absence/user-accounts-passwords area=authentication-authorization rec=fct-uy3rjgvj@1
capability: User accounts with passwords
@row_check n=44 row=absence/user-accounts-passwords sha=a558d817c865 v=holds judge=sonnet head=8834195b0683
searched: password|passwd|login|user_account|bcrypt|argon2 (case-insensitive) · in src/**/*.rs and crates/**/*.rs (only hits: the e2e harness reading the OS passwd entry for the home dir, no accounts)
@row_add n=45 row=decision/all-or-nothing-access area=authentication-authorization rec=fct-j6fyndcf@1
name: All-or-nothing access
rule: Any caller that passes the OS check gets the full surface, with no roles.
@row_check n=45 row=decision/all-or-nothing-access sha=4097276614fc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=46 row=decision/socket-dir-checked area=authentication-authorization rec=fct-dthgjfbs@1
name: Socket directory checked
rule: The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
@row_check n=46 row=decision/socket-dir-checked sha=e79526b93d2b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:168
quote: if is_symlink {
at: crates/viola-state/src/strict.rs:174
quote: unix_read_verdict(mode, uid, euid)
at: src/cmd/run.rs:511
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
@row_add n=47 row=decision/socket-chmod-after-bind area=authentication-authorization rec=fct-dthgjfbs@1
name: Socket owner-only after bind
rule: The Unix socket is set to mode 0600 right after the bind.
@row_check n=47 row=decision/socket-chmod-after-bind sha=c358ce8ada44 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
@row_add n=48 row=decision/no-socket-overwrite area=authentication-authorization rec=fct-dthgjfbs@1
name: No socket overwrite
rule: Binding never overwrites an existing socket; try_overwrite stays off.
@row_check n=48 row=decision/no-socket-overwrite sha=e23c68e06d87 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:123-127
quote: let listener = ListenerOptions::new() .name(name) .reclaim_name(false) .create_sync() .map_err(ChannelError::Bind)?;
@row_add n=49 row=decision/server-peer-euid-check area=authentication-authorization rec=fct-unwy7exn@1
name: Server-side peer euid check
rule: On every accept, a peer whose euid differs from the server's is dropped before a frame is read.
@row_check n=49 row=decision/server-peer-euid-check sha=c4f709e151fb v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=50 row=decision/client-strict-modes-first area=authentication-authorization rec=fct-phspeqis@2
name: Client strict-modes check first
rule: Before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line.
@row_check n=50 row=decision/client-strict-modes-first sha=956e55edc8ae v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:85
quote: check_instance(&instance.home, &instance.dir).is_ok()
@row_add n=51 row=decision/client-verifies-server area=authentication-authorization rec=fct-phspeqis@2
name: Client verifies the server
rule: The client verifies the connected peer's pid and that process's start time against its one snapshot read; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect.
@row_check n=51 row=decision/client-verifies-server sha=93c2bf9451f1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: crates/viola-state/src/liveness.rs:141
quote: if observed_pid != recorded_pid {
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
@row_add n=52 row=decision/client-peer-euid-check area=authentication-authorization rec=fct-phspeqis@2
name: Client-side peer euid check
rule: Past the connect, the client admits only a server of its own euid and refuses an unreadable peer, sending no frame.
@row_check n=52 row=decision/client-peer-euid-check sha=97b14eadb2b5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
@row_add n=53 row=contract/ipc-client-refusals/socket-dir-refused area=authentication-authorization rec=fct-phspeqis@2
boundary: IPC client refusals
item: socket-dir-refused
promise: Reported when the recorded endpoint's socket directory fails its check before the connect; no frame is sent.
@row_check n=53 row=contract/ipc-client-refusals/socket-dir-refused sha=5f45b3153824 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() {         return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:176
quote: let client = Client::connect(&live.endpoint, "cli")?;
@row_add n=54 row=contract/ipc-client-refusals/server-verify-failed area=authentication-authorization rec=fct-phspeqis@2
boundary: IPC client refusals
item: server-verify-failed
promise: Reported when the connected peer is not of the client's euid or cannot be read; no frame is sent.
@row_check n=54 row=contract/ipc-client-refusals/server-verify-failed sha=612eca0c3a17 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) {         return Err(CallError::ServerUnverified);
at: crates/viola-channel/src/client.rs:144-145
quote: pub fn peer_admitted(observed: Option<u32>) -> bool {     peer_euid_admitted(observed, own_euid())
@row_add n=55 row=decision/driver-release-refused area=authentication-authorization rec=fct-cwpea3sr@1
name: Driver-originated release refused
rule: The wrapper refuses a release whose params carry a string from, before any budget is acted on.
why: An affordance-level guard against drivers handing the wheel back to automation, not enforcement, since a same-user process can leave out from.
@row_check n=55 row=decision/driver-release-refused sha=c78960e23400 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
@row_add n=56 row=contract/release-errors/release-from-driver area=authentication-authorization rec=fct-cwpea3sr@1
boundary: The release method's errors
item: release-from-driver
promise: A release carrying a string from is refused with code -32602 and reason release-from-driver.
@row_check n=56 row=contract/release-errors/release-from-driver sha=8f067f85a55f v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") {             log_release_from_driver(call, from);             return Err(ProtocolError::ReleaseFromDriver);
@row_add n=57 row=decision/home-owner-only area=authentication-authorization rec=fct-jya3vgt6@2
name: Owner-only viola home
rule: The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
@row_check n=57 row=decision/home-owner-only sha=ede036095b20 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:80
quote: restrict(tmp.path(), mode)?;
at: crates/viola-state/src/fs.rs:10-11
quote: pub const DIR_MODE: u32 = 0o700; pub const FILE_MODE: u32 = 0o600;
@row_add n=58 row=decision/strict-modes-before-read area=authentication-authorization rec=fct-jya3vgt6@2
name: Strict-modes check before trusted reads
rule: Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
@row_check n=58 row=decision/strict-modes-before-read sha=21d20fffe976 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:122
quote: if meta.file_type().is_symlink() {
at: crates/viola-state/src/strict.rs:229
quote: if mode & 0o077 != 0 {
at: crates/viola-state/src/strict.rs:219
quote: if mode & 0o022 != 0 {
@row_add n=59 row=decision/answer-behavior-closed-enum area=authentication-authorization rec=fct-ty6c2nnj@1
name: Closed answer behaviour
rule: An answer's behavior is a closed enum.
@row_check n=59 row=decision/answer-behavior-closed-enum sha=a3d48910310d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:168-171
quote: pub enum Permit { Allow, Deny, }
at: crates/viola-agent-claude/src/dialog.rs:175-178
quote: pub enum Verdict { Approve, Revise, }
@row_add n=60 row=decision/unverified-cli-gate area=authentication-authorization rec=fct-ty6c2nnj@1
name: Unverified CLI gate
rule: No non-null answer decision is sent without a viola verify stamp for the child's CLI version.
@row_check n=60 row=decision/unverified-cli-gate sha=e62ac59840c2 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=61 row=overview/input-validation area=input-validation rec=fct-ambk6d46@1
text: Viola validates untrusted data at every point where it enters. Two validation regimes apply: tolerant parsing for external payloads and versioned-tolerant parsing for viola's own formats, with the security floors holding in both. Shared validators live in one core crate, so the CLI, the MCP server and the wrapper all call the same function.
@row_check n=61 row=overview/input-validation sha=26a707e40f33 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:356
quote: pub fn validate_paste_text(text: &str) -> Result<(), NotDelivered> {
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=62 row=decision/shared-validators-in-core area=input-validation rec=fct-ambk6d46@1
name: Shared validators in the core crate
rule: Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one.
why: so the CLI, MCP and the wrapper apply one identical check
@row_check n=62 row=decision/shared-validators-in-core sha=4547309223cc v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:356
quote: pub fn validate_paste_text(text: &str) -> Result<(), NotDelivered> {
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=63 row=decision/paste-text-character-set area=input-validation rec=fct-bvljrxyp@1
name: Paste text character set
rule: Paste text allows LF, CR and TAB and refuses every other C0 control, DEL and every C1 character, checked over decoded characters.
@row_check n=63 row=decision/paste-text-character-set sha=3cdad33367b8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:366-367
quote: '\n' | '\r' | '\t' => false, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
at: crates/viola-core/src/lib.rs:357
quote: if text.chars().any(is_refused_control) {
@row_add n=64 row=decision/refused-text-never-stripped area=input-validation rec=fct-bvljrxyp@1
name: Refused text is refused whole
rule: A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
@row_check n=64 row=decision/refused-text-never-stripped sha=bf062fd1edf4 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88-89
quote: if let Err(detail) = viola_core::validate_paste_text(text) { return ctx.not_delivered(None, detail);
at: src/run/send.rs:91-92
quote: let typed = typed_text(text); let text: &str = &typed;
@row_add n=65 row=decision/wrapper-checks-authoritatively area=input-validation rec=fct-bvljrxyp@1,fct-2hvuwr4u@1
name: Wrapper re-checks text authoritatively
rule: Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
@row_check n=65 row=decision/wrapper-checks-authoritatively sha=f591a51ea26e v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=66 row=decision/dialog-free-text-character-set area=input-validation rec=fct-2hvuwr4u@1,fct-bvljrxyp@1
name: Dialog free text uses the paste text set
rule: Dialog free text (permission and plan messages, question answer values) is held to the same exact character set as paste text.
@row_check n=66 row=decision/dialog-free-text-character-set sha=91a91c3059c6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=67 row=contract/answer-refusal/control-character area=input-validation rec=fct-2hvuwr4u@1
boundary: answer refusal words
item: control-character
promise: Dialog free text holding a refused character is refused with the word control-character before any frame is sent.
@row_check n=67 row=contract/answer-refusal/control-character sha=8e77425a2659 v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/cmd/answer.rs:112
quote: out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()));
at: crates/viola-core/src/lib.rs:358
quote: Err(NotDelivered::ControlCharacter)
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=68 row=decision/channel-frame-cap-before-read area=input-validation rec=fct-vhivn34r@1
name: Channel frame cap before reading
rule: A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
@row_check n=68 row=decision/channel-frame-cap-before-read sha=d72a2950ea6b v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:24-25
quote: io::Read::take(&mut *reader, cap) .read_until(b'\n', &mut line)
at: crates/viola-channel/src/server.rs:269
quote: close: true,
@row_add n=69 row=decision/json-default-depth-limit area=input-validation rec=fct-vhivn34r@1
name: Default JSON depth limit
rule: Channel frames are parsed with serde_json's default depth limit, never with unbounded depth.
@row_check n=69 row=decision/json-default-depth-limit sha=26d0280f9d7f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:79
quote: let value: Value = serde_json::from_slice(line).map_err(|_| (None, ProtocolError::Parse))?;
at: deny.toml:55
quote: deny = ["unbounded_depth"]
@row_add n=70 row=contract/channel-errors/32600 area=input-validation rec=fct-vhivn34r@1
boundary: channel error codes
item: -32600
promise: A frame over the MAX_FRAME cap is answered -32600 and the connection is closed.
@row_check n=70 row=contract/channel-errors/32600 sha=cc77fab7f2ed v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:268-269
quote: frame: error_response(None, ProtocolError::InvalidRequest), close: true,
at: crates/viola-channel/src/lib.rs:72
quote: Self::InvalidRequest => -32600,
@row_add n=71 row=contract/channel-errors/32602 area=input-validation rec=fct-vhivn34r@1
boundary: channel error codes
item: -32602
promise: A method whose params fail the wrapper's check is answered -32602.
@row_check n=71 row=contract/channel-errors/32602 sha=61ef258383e3 v=holds judge=sonnet head=8834195b0683
at: src/run/send/params.rs:7-9
quote: let text = params["text"] .as_str() .ok_or(ProtocolError::InvalidParams)?;
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
@row_add n=72 row=decision/invalid-hook-frames-dropped area=input-validation rec=fct-vhivn34r@1
name: Invalid hook frames dropped
rule: An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
@row_check n=72 row=decision/invalid-hook-frames-dropped sha=c606117f5582 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:102
quote: let data = event.get("data").filter(|d| d.is_object())?;
at: src/cmd/run.rs:124
quote: if let Some(line) = self.hook_event_line(params) {
at: src/run/dialog.rs:241
quote: let raise = Raise::parse(params)?;
@row_add n=73 row=decision/hook-stdin-never-truncated area=input-validation rec=fct-u7v2p5mv@1
name: Hook stdin over-limit detected
rule: Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
@row_check n=73 row=decision/hook-stdin-never-truncated sha=c7826b6baf6e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:178-181
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?; if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { reject_stdin("oversize"); return Ok(Some("oversize-stdin"));
@row_add n=74 row=decision/hook-security-fields-closed area=input-validation rec=fct-u7v2p5mv@1
name: Hook security fields in closed enums
rule: Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
@row_check n=74 row=decision/hook-security-fields-closed sha=d9b374788c92 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:121-122
quote: let mut payload: Value = serde_json::from_slice(bytes).map_err(|_| AgentError::Malformed)?; if !payload.is_object() {
at: crates/viola-agent-claude/src/hook.rs:120
quote: let kind = event.kind().ok_or(AgentError::NotAnEvent)?;
at: src/cmd/run.rs:103-104
quote: let prompt_ok = data["text"].is_string() && matches!(data["origin"].as_str(), Some("harness" | "human"));
@row_add n=75 row=decision/hooks-fail-open area=input-validation rec=fct-u7v2p5mv@1
name: Hooks fail open
rule: Every hook failure fails open: exit 0 with no body.
@row_check n=75 row=decision/hooks-fail-open sha=a2efe67dfbb4 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
@row_add n=76 row=decision/capture-arms-isolated area=input-validation rec=fct-u7v2p5mv@1
name: Hidden capture arms isolated
rule: The hidden capture arms read no VIOLA_* variable, open no channel and always exit 0.
@row_check n=76 row=decision/capture-arms-isolated sha=287b95de5a01 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:91
quote: let Some(dir) = &args.capture else {
at: src/cmd/hook.rs:95
quote: capture_statusline(dir, shell, &mut std::io::stdin().lock());
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
at: src/cmd/hook.rs:102
quote: let filed = capture(event, dir, &mut std::io::stdin().lock());
at: src/cmd/hook.rs:109
quote: std::env::var_os("VIOLA_NAME"),
@row_add n=77 row=decision/mcp-handlers-check-raw-args area=input-validation rec=fct-gtu3y5ba@1
name: MCP handlers check raw arguments
rule: Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only.
why: rmcp does not enforce the schemars schemas
@row_check n=77 row=decision/mcp-handlers-check-raw-args sha=cecb207d43f3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:262
quote: fn target_of(args: &JsonObject) -> Checked<ViolaName> {
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: crates/viola-mcp/src/tools.rs:277
quote: Some(value) => value.as_u64().map(Some).ok_or(field),
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=78 row=contract/mcp-errors/invalid-params area=input-validation rec=fct-gtu3y5ba@1
boundary: MCP tool errors
item: invalid-params
promise: A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
@row_check n=78 row=contract/mcp-errors/invalid-params sha=3696e4bc57bc v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
at: crates/viola-mcp/src/tools.rs:372
quote: (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
@row_add n=79 row=contract/mcp-stdin/parse-rejected area=input-validation rec=fct-bngndo2w@1
boundary: MCP stdin refusal log
item: parse-rejected
promise: An over-long MCP stdin line is recorded as one parse-rejected line holding no byte of it.
@row_check n=79 row=contract/mcp-stdin/parse-rejected sha=c1aa72b5f067 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/lib.rs:86-87
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line();
at: crates/viola-mcp/src/lib.rs:97-100
quote: ObsEvent::ParseRejected, parser = "mcp-stdin", detail = "oversize", count = 1u64,
@row_add n=80 row=decision/names-through-validated-type area=input-validation rec=fct-nad3igah@1
name: Names through the validated name type
rule: Every instance name taken from CLI arguments passes the validated name constructor.
@row_check n=80 row=decision/names-through-validated-type sha=0d4d178a0b19 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:44
quote: ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
at: src/cmd/wait.rs:22
quote: #[arg(value_parser = super::send::parse_name)]
at: src/cmd/run.rs:51
quote: ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
@row_add n=81 row=decision/home-strict-modes-first area=input-validation rec=fct-nad3igah@1,fct-brqp4paf@1
name: Home strict-modes check first
rule: The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
@row_check n=81 row=decision/home-strict-modes-first sha=d106fd28dad9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:308
quote: std::fs::canonicalize(home)
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: crates/viola-state/src/stamps.rs:61
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
at: crates/viola-state/src/stamps.rs:66
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
@row_add n=82 row=decision/stamps-checked-under-lock area=input-validation rec=fct-nad3igah@1
name: Stamps checked before and under lock
rule: The ledger stamps file is checked before the ledger directory is created and again under its lock.
@row_check n=82 row=decision/stamps-checked-under-lock sha=10386e3902aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:61
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
at: crates/viola-state/src/stamps.rs:63
quote: create_private_dir(&dir)?;
at: crates/viola-state/src/stamps.rs:66
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
@row_add n=83 row=decision/cli-input-checked-before-frame area=input-validation rec=fct-nad3igah@1
name: CLI input checked before any frame
rule: answer and send input is read under MAX_FRAME and parsed into closed enums or through the paste text validator before any frame is sent.
@row_check n=83 row=decision/cli-input-checked-before-frame sha=b3723909f579 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:356
quote: pub fn validate_paste_text(text: &str) -> Result<(), NotDelivered> {
at: src/cmd/send.rs:57
quote: reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/send.rs:198
quote: let Some(live) = live_endpoint(&instance_dir) else {
at: src/cmd/answer.rs:49
quote: reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: src/cmd/answer.rs:55
quote: Response::parse(&value).ok()
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
@row_add n=84 row=contract/revive-json/output-fields area=input-validation rec=fct-nad3igah@1
boundary: revive --json output
item: output fields
promise: revive --json prints codes, closed words and session-shaped ids only, never a path or pid.
@row_check n=84 row=contract/revive-json/output-fields sha=c9f962c89efa v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:290
quote: .map(|row| json!({"ts": row.ts, "cause": row.cause, "id": row.id}))
at: src/cmd/revive.rs:321
quote: .filter(|link| is_session_id(&link.agent_session_id))
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
@row_add n=85 row=decision/config-bad-value-handling area=input-validation rec=fct-wsshf6pn@1
name: Bad config values
rule: Configuration is parsed tolerantly under MAX_FRAME with its version checked; a bad value takes its key's default or is rejected whole, and is reported as parse-rejected.
@row_check n=85 row=decision/config-bad-value-handling sha=1890503b2dfc v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:187
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/obs.rs:198
quote: if map.get("v").and_then(Value::as_u64) != Some(1) {
at: src/run/budget.rs:81
quote: compiled(default)
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: src/run/env.rs:55
quote: None => (Vec::new(), Some(ConfigRejection::Malformed)),
at: src/obs.rs:223
quote: ObsEvent::ParseRejected,
@row_add n=86 row=decision/config-cannot-disable-controls area=input-validation rec=fct-wsshf6pn@1
name: Configuration cannot disable controls
rule: No configuration value switches the budget gate off or makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
@row_check n=86 row=decision/config-cannot-disable-controls sha=32c5f8e072fe v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:92-93
quote: let whole = u8::try_from(value.as_u64()?).ok()?; Percent::try_new(whole).ok()
at: src/run/dialog.rs:185-186
quote: let whole = u16::try_from(value.as_u64()?).ok()?; DialogHold::try_new(whole).ok()
at: crates/viola-agent-claude/src/lib.rs:72-73
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) {
@row_add n=87 row=decision/test-seams-compiled-out area=input-validation rec=fct-ykdescly@1,fct-bbnvgu6s@1
name: Test seams compiled out of release
rule: Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
@row_check n=87 row=decision/test-seams-compiled-out sha=15ae3fdbf9f9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61-62
quote: #[cfg(feature = "fake-agent")] fn hold_pump_start() {
at: src/cmd/hook/seam.rs:8-9
quote: #[cfg(feature = "fake-agent")] pub(super) fn panic_if_asked() {
at: src/cmd/hook/seam.rs:18
quote: if value.as_deref() == Some(OsStr::new("1")) {
at: Cargo.toml:10
quote: fake-agent = []
@row_add n=88 row=decision/colour-env-only-compared area=input-validation rec=fct-hqz7q7dg@1
name: Terminal colour variables only compared
rule: NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
@row_check n=88 row=decision/colour-env-only-compared sha=d0fac8294fb8 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:96
quote: term: text("TERM"),
at: src/human/style.rs:97
quote: colorterm: text("COLORTERM"),
searched: "NO_COLOR" · in src/**/*.rs crates/**/*.rs (only style.rs reads or names it; no set_var)
@row_add n=89 row=decision/state-readers-never-write area=input-validation rec=fct-brqp4paf@1
name: State file readers never write
rule: A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
@row_check n=89 row=decision/state-readers-never-write sha=dcacee57525e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:118
quote: const LINE_CAP: u64 = MAX_FRAME + 1;
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:257-258
quote: let Ok(Value::Object(object)) = serde_json::from_slice::<Value>(&line) else { self.skipped.torn_lines += 1;
@row_add n=90 row=decision/replayed-snapshot-inert area=input-validation rec=fct-brqp4paf@1
name: Replayed snapshot is inert
rule: A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid, start time or statusline command, so no frame is written and no command runs from it.
@row_check n=90 row=decision/replayed-snapshot-inert sha=a071a6a2dcbf v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:19
quote: pub wheel: Option<Wheel>,
at: crates/viola-state/src/replay.rs:23
quote: pub links: Vec<Value>,
at: crates/viola-state/src/replay.rs:24
quote: pub dialog_pending: bool,
at: crates/viola-state/src/replay.rs:155
quote: SnapshotRead::Unsupported { v_seen } => ReplayCause::Unsupported { v_seen },
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook/statusline.rs:130
quote: Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command,
@row_add n=91 row=decision/failed-budget-read-no-reading area=input-validation rec=fct-brqp4paf@1
name: Failed budget read is no reading
rule: Every failed read of the budget file counts as no reading.
@row_check n=91 row=decision/failed-budget-read-no-reading sha=01df3e00f958 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:94-95
quote: check_path(home).ok()?; check_path(&path).ok()?;
at: crates/viola-state/src/budget.rs:107
quote: let stored: Stored = serde_json::from_slice(&bytes).ok()?;
at: crates/viola-state/src/budget.rs:112
quote: (stored.v == BUDGET_V).then_some(StampedReading {
@row_add n=92 row=decision/child-output-bounded area=input-validation rec=fct-q7h4j7ft@1
name: Child output bounded and tolerant
rule: Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
@row_check n=92 row=decision/child-output-bounded sha=3fb7c3e44c18 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
at: src/run/agents.rs:69-70
quote: let Ok(ran) = ran else { return unread(false);
at: src/run/version_gate.rs:49
quote: run_capped(program, args, cwd, strip, deadline, stopped, MAX_FRAME)
at: src/run/agents.rs:78
quote: let list = agents::parse(&ran.stdout).map_err(|failure| {
@row_add n=93 row=decision/verify-reads-only-signatures area=input-validation rec=fct-q7h4j7ft@1
name: Verify reads PTY output by signatures
rule: verify's PTY output is read only against the compiled signature literals.
@row_check n=93 row=decision/verify-reads-only-signatures sha=84783f8797cd v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/pty.rs:147
quote: .is_some_and(|rows| rows.iter().any(|r| SIGNATURES.holds_any(r)));
at: src/cmd/verify/typed/pty.rs:179
quote: .any(|r| SIGNATURES.holds_any(r))
at: src/cmd/verify/typed/turns.rs:76
quote: holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
@row_add n=94 row=decision/unwrapped-name-never-target area=input-validation rec=fct-q7h4j7ft@1
name: Unwrapped agent names are no targets
rule: An unwrapped agent row's name is never a validated viola name and never a valid target.
@row_check n=94 row=decision/unwrapped-name-never-target sha=dce98579b7ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:168
quote: pub name: String,
at: crates/viola-state/src/board.rs:193
quote: name: observation.session_id,
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
@row_add n=95 row=decision/vt100-feed-isolated area=input-validation rec=fct-juszneez@1
name: Screen model feed isolated from passthrough
rule: PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
@row_check n=95 row=decision/vt100-feed-isolated sha=8c040be7dbcf v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:134-135
quote: let written = self.output.write(bytes)?; self.feed.offer(Feed::Bytes(bytes[..written].to_vec()));
at: src/run/gate.rs:109
quote: if let Err(TrySendError::Full(_)) = self.tx.try_send(message) {
at: src/run/gate.rs:46-47
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() { self.screen.poison();
at: src/run/gate.rs:69
quote: self.screen.poison();
@row_add n=96 row=decision/claude-resolved-from-path area=input-validation rec=fct-sgmmo3ki@2
name: claude resolved from PATH
rule: The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded.
why: a same-user trust and accepted risk
@row_check n=96 row=decision/claude-resolved-from-path sha=b5fa60a8a753 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:143
quote: let path = std::env::var_os("PATH");
at: src/run/agents.rs:110
quote: let Ok(program) = super::resolve_program(OsStr::new(PROGRAM), cwd) else {
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
@row_add n=97 row=decision/revive-cwd-spawn-only area=input-validation rec=fct-sgmmo3ki@2
name: Recorded cwd used only to spawn
rule: The recorded working directory of a revive is read after the strict-modes check and used only as the spawn directory.
@row_check n=97 row=decision/revive-cwd-spawn-only sha=14f712f28e33 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:137
quote: if !trusted(home, &args.name)? {
at: src/cmd/revive.rs:263
quote: .filter(|dir| dir.is_dir()),
at: src/cmd/revive.rs:236
quote: spawn_dir,
@row_add n=98 row=decision/session-id-argv-no-shell area=input-validation rec=fct-pt33yw4f@1
name: Session id as one argv element
rule: The session id after --resume has the closed session-id shape and is one argv element of a direct spawn, never passed through a shell.
@row_check n=98 row=decision/session-id-argv-no-shell sha=fdca4bebfad2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:110
quote: let mut args = vec![OsString::from(RESUME_FLAG), OsString::from(id)];
at: src/cmd/revive.rs:247
quote: .filter(|id| is_session_id(id));
@row_add n=99 row=contract/revive-refusal/no-session area=input-validation rec=fct-pt33yw4f@1
boundary: revive refusal words
item: no-session
promise: An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.
@row_check n=99 row=contract/revive-refusal/no-session sha=1424476896c9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:249
quote: Some(asked) => ids.find(|id| *id == asked).ok_or(Refusal::UnknownId),
at: src/cmd/revive.rs:86-89
quote: Self::UnknownId => ( format!("{name} has no logged session with that id"), format!("viola revive {name} --list"), ),
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=100 row=decision/statusline-source-read-only area=input-validation rec=fct-h4ojdq3v@1
name: Statusline source read only
rule: A statusline command is taken only from a non-empty statusLine of type command with no NUL, read once and capped; viola writes neither source.
@row_check n=100 row=decision/statusline-source-read-only sha=2858cacbc9e9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:554
quote: .take(MAX_FRAME)
at: crates/viola-agent-claude/src/statusline.rs:87
quote: (!command.is_empty() && !command.contains('\0')).then(|| command.to_owned())
@row_add n=101 row=decision/no-statusline-command-in-logs area=input-validation rec=fct-2fgozo5r@2
name: Statusline command never logged
rule: No log line or span carries the statusline command.
@row_check n=101 row=decision/no-statusline-command-in-logs sha=84966f2406f8 v=holds judge=sonnet head=8834195b0683
searched: command = · in src/cmd/hook/statusline.rs, src/cmd/run.rs, src/obs.rs, src/cmd/hook.rs
searched: ObsEvent::\w+,[^)]*\bcommand · in **/*.rs
searched: statusline_command · in src/** (only snapshot field plumbing, no obs_event field)
@row_add n=102 row=decision/parser-fuzz-scope area=input-validation rec=fct-h4bkgare@1
name: Parser surfaces in fuzz scope
rule: Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser and the vt100 feed are in scope for fuzz and property coverage.
@row_check n=102 row=decision/parser-fuzz-scope sha=1277c82fa56b v=holds judge=sonnet head=8834195b0683
at: fuzz/fuzz_targets/paste_text.rs:22
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/channel_frame.rs:13
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/hook_stdin.rs:11
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/claude_agents_json.rs:9
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/vt100_feed.rs:33
quote: if catch_unwind(AssertUnwindSafe(|| screen.feed(chunk, base))).is_err() {
@row_add n=103 row=decision/max-frame-cap area=input-validation rec=fct-ydf4aeiw@1
name: One external read cap
rule: MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read.
except: viola's reads of its own executable are no external input
@row_check n=103 row=decision/max-frame-cap sha=92729b20938e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:11
quote: pub const MAX_FRAME: u64 = 16 * 1024 * 1024;
@row_add n=104 row=decision/max-frame-revisit-signal area=input-validation rec=fct-ydf4aeiw@1
name: Frame cap revisit signal
rule: A stamped largest hook payload within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_check n=104 row=decision/max-frame-revisit-signal sha=54a06f99c70d v=unverifiable judge=sonnet head=8834195b0683
searched: MAX_FRAME\s*/\s*4 · in src, crates, tests, scripts
searched: revisit · in src, crates, tests, scripts
why: a revisit signal is a process rule; the code stamps largest_hook_payload but no code compares it to MAX_FRAME by a factor of 4
@row_add n=105 row=decision/data-protection-minimal-tier area=data-protection rec=fct-cm4lvc5u@1
name: Data protection at Minimal tier, adapted
rule: Data protection follows the Minimal tier with file-permission controls kept in scope.
why: viola stores sensitive user content; HTTPS enforcement does not apply
@row_check n=105 row=decision/data-protection-minimal-tier sha=1c1b70aca76e v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=106 row=decision/no-tls-local-transport area=data-protection rec=fct-6ikkebgm@2
name: No TLS on local transports
rule: No TLS is used: the IPC channel is local Unix sockets and MCP is stdio, so confidentiality in transit rests on the socket-directory and peer checks.
@row_check n=106 row=decision/no-tls-local-transport sha=1efb3b68400c v=holds judge=sonnet head=8834195b0683
searched: rustls|native-tls|openssl|\btls\b · in **/Cargo.toml (no TLS dependency; only portable-pty at Cargo.toml:217)
searched: Tls|TLS|rustls · in src/**/*.rs and crates/**/src (no hit)
@row_add n=107 row=decision/no-handle-inheritance area=data-protection rec=fct-p7rzex2i@3
name: No channel handle inheritance
rule: Channel handles never leak into the claude child: every inherited descriptor is closed in the child.
@row_check n=107 row=decision/no-handle-inheritance sha=0b29aabf5994 v=unverifiable judge=sonnet head=8834195b0683
searched: CLOEXEC|close_range|FD_CLOEXEC · in src/**/*.rs and crates/**/*.rs
searched: /proc/self/fd|descriptor · in src/**, crates/**, tests/**
why: no evidence found; the closing is portable-pty's, not in the tree, and only a test (tests/tui_channel_fds.rs) reads /proc/<pid>/fd
@row_add n=108 row=decision/state-files-os-permissions area=data-protection rec=fct-7aw7emh6@2
name: State files protected by OS permissions
rule: State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
@row_check n=108 row=decision/state-files-os-permissions sha=2574b1c9f255 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
@row_add n=109 row=decision/settings-override-plain-path area=data-protection rec=fct-7aw7emh6@2
name: Settings override only for plain paths
rule: The settings override is written only for a pinned path of plain characters.
why: a shell runs the override unquoted
@row_check n=109 row=decision/settings-override-plain-path sha=0d243b20243e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
@row_add n=110 row=decision/plugin-files-rewritten-each-start area=data-protection rec=fct-7aw7emh6@2
name: Plugin files rewritten every start
rule: The plugin files and the settings override are rewritten at every start.
@row_check n=110 row=decision/plugin-files-rewritten-each-start sha=96c620e7430e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:475-478
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) { let path = plugin_dir.join(rel); create_private_dir(path.parent().unwrap_or(&plugin_dir))?; replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
@row_add n=111 row=decision/pinned-binary-rehashed area=data-protection rec=fct-7aw7emh6@2
name: Pinned binary re-hashed before use
rule: The pinned viola binary is re-hashed before use.
@row_check n=111 row=decision/pinned-binary-rehashed sha=c41a3ebb6e76 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:88-90
quote: match file_key(&path) { Ok(found) if found == hash => {} Ok(_) => return Err(PinError::HashMismatch),
@row_add n=112 row=decision/probe-artifacts-cleanup area=data-protection rec=fct-kgbhs4lx@1
name: Probe artifacts owner-only and removed
rule: Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing.
except: a kill or a crash leaves them to be removed by hand
@row_check n=112 row=decision/probe-artifacts-cleanup sha=fcd08f36f3bf v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:337
quote: create_private_dir(&statusline)?;
at: src/cmd/verify.rs:389-391
quote: impl Drop for ProbeDir { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.dir);
@row_add n=113 row=decision/logs-owner-only area=data-protection rec=fct-2vvustrg@1
name: Logs owner-only
rule: The event log and diagnostics files are 0600 inside 0700 directories.
@row_check n=113 row=decision/logs-owner-only sha=675b15173eba v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:330
quote: let mut file = open_private_append(&instance_dir.join(EVENTS))?;
at: src/obs.rs:331
quote: if let Ok(mut file) = open_private_append(&path) {
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
@row_add n=114 row=decision/drift-reports-diagnostics-only area=data-protection rec=fct-2vvustrg@1
name: Drift reports only in diagnostics
rule: Deserialisation drift reports are written only to the diagnostics directory.
why: drift reports can quote tool input
@row_check n=114 row=decision/drift-reports-diagnostics-only sha=cd4fccaa03ed v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
@row_add n=115 row=decision/fixture-recording-scrubbed area=data-protection rec=fct-dg66rgro@1
name: Fixture recordings scrubbed
rule: verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
@row_check n=115 row=decision/fixture-recording-scrubbed sha=d1f6a03c43c7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:585-587
quote: let scrubbed = ledger::scrub(&payload, home, user); if let Some(why) = ledger::unclean(&scrubbed, user) { return Err(format!("{name} {}", why.code()));
at: crates/viola-agent-claude/src/ledger.rs:1433
quote: text_fault(text, user).or_else(|| has_email(text).then_some(Unclean::Email))
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=116 row=decision/fixtures-reviewed-before-commit area=data-protection rec=fct-dg66rgro@1
name: Fixtures reviewed before commit
rule: Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
@row_check n=116 row=decision/fixtures-reviewed-before-commit sha=981a7f48ce05 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=117 row=decision/no-keys-at-rest area=data-protection rec=fct-7roiu3gz@1
name: No keys at rest
rule: Key management is omitted for the Minimal tier: there are no keys at rest.
@row_check n=117 row=decision/no-keys-at-rest sha=5230f13a1fc2 v=holds judge=sonnet head=8834195b0683
searched: encrypt|cipher|aes|chacha|keyring|argon · in **/Cargo.toml (no key-handling dependency; sha2 only for hashing)
searched: Sha256 · in crates/viola-state/src (hashing only, pin.rs)
@row_add n=118 row=decision/log-rotation-keeps-permissions area=data-protection rec=fct-7roiu3gz@1
name: Log rotation keeps permissions
rule: Any rotation scheme for the event log must keep 0600/0700 permissions and valid offsets.
@row_check n=118 row=decision/log-rotation-keeps-permissions sha=9fd97f5e312a v=unverifiable judge=sonnet head=8834195b0683
searched: fn prune|retention|rotate|rotation · in crates/viola-state/src
why: no rotation scheme exists to check; a requirement on future code
@row_add n=119 row=decision/verify-trusted-runs-risk area=data-protection rec=fct-j3zmg7hp@1
name: Verify trusted runs accepted risk
rule: verify's trusted runs B, C and D are real interactive turns whose transcripts the CLI persists and for which the user's global hooks run.
why: accepted risk
@row_check n=119 row=decision/verify-trusted-runs-risk sha=8dfc674b1ad5 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim; the CLI's transcript persistence and the user's hooks are outside the tree
@row_add n=120 row=decision/viola-writes-nothing-in-claude-dir area=data-protection rec=fct-j3zmg7hp@1,fct-vaineopj@1
name: viola writes nothing in Claude's config
rule: viola itself writes nothing under the user's Claude Code config directory or its global config file.
@row_check n=120 row=decision/viola-writes-nothing-in-claude-dir sha=32cc3c821fdc v=holds judge=sonnet head=8834195b0683
searched: \.claude\.json · in src/**/*.rs and crates/**/src (no hit)
searched: \.claude|home_dir · in src/**/*.rs (non-test hits only read ~/.claude/settings.json via USER_SETTINGS, statusline.rs:18, and src/cmd/plugin.rs:3 states it writes nothing under ~/.claude)
@row_add n=121 row=decision/scratch-footer-reading-residual area=data-protection rec=fct-f5gbcl67@1
name: Scratch footer reading is no standing permission
rule: A scratch footer reading of a newer CLI is one tmpfs-home run start, no key typed, only scrubbed footer rows kept; it is no product path, and another such reading needs its own answer.
why: accepted residual
@row_check n=121 row=decision/scratch-footer-reading-residual sha=7f52b2a93d66 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=122 row=decision/live-plugin-install-round area=data-protection rec=fct-vaineopj@1
name: Live plugin install round
rule: A live plugin install round runs the CLI's four plugin commands at local scope outside the work tree, passing folder trust by keys into its own pty only, never a desktop key.
why: accepted residual
@row_check n=122 row=decision/live-plugin-install-round sha=168833a5675d v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=123 row=decision/verify-never-answers-import area=data-protection rec=fct-tjw7g733@1
name: Verify needs a pre-trusted folder
rule: verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
@row_check n=123 row=decision/verify-never-answers-import sha=655a4ae91ecf v=holds judge=sonnet head=8834195b0683
searched: Yes, allow external imports · in src/cmd/verify (only hit src/cmd/verify/typed/turns.rs:251, a test asserting the input box is not up behind that modal; no key is sent to answer it)
searched: external import · in src/cmd/verify/**, -i (no code answering the import dialog)
@row_add n=124 row=decision/event-log-growth-risk area=data-protection rec=fct-q6dicbwj@1
name: Event log growth accepted
rule: Unbounded growth of the event log may exhaust local disk until retention is resolved.
why: accepted risk: availability impact only, no confidentiality or integrity impact
@row_check n=124 row=decision/event-log-growth-risk sha=1bedb89abbe0 v=unverifiable judge=sonnet head=8834195b0683
searched: retention · in src
why: not a code claim (accepted risk; a search of src for retention found no code either way)
@row_add n=125 row=decision/rmcp-feature-set area=api-security rec=fct-kjrxk2yb@1
name: rmcp feature set
rule: The release graph enables rmcp features exactly server and transport-io; the test-only e2e crate takes client only, never transport-child-process.
why: all five 2026 rmcp advisories sit in unused features
guard: scripts/rmcp-features.sh in CI's lint job
@row_check n=125 row=decision/rmcp-feature-set sha=0e6f81cdab6c v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: crates/viola-e2e/Cargo.toml:44
quote: rmcp = { workspace = true, features = ["client"] }
guard_at: scripts/rmcp-features.sh:37
@row_add n=126 row=decision/rmcp-banned-features area=api-security rec=fct-kjrxk2yb@1
name: rmcp HTTP and auth features banned
rule: rmcp's transport-streamable-http-server and auth features are banned.
why: all five 2026 rmcp advisories sit in unused features
guard: cargo-deny bans.features
@row_check n=126 row=decision/rmcp-banned-features sha=e5de1b496df7 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:45
quote: deny = ["transport-streamable-http-server", "auth"]
guard_at: deny.toml:45
@row_add n=127 row=decision/cargo-deny-audit area=dependency-security rec=fct-bzbrgpfq@1,fct-cmrmoq4t@1
name: cargo-deny as audit tool
rule: cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.
@row_check n=127 row=decision/cargo-deny-audit sha=931c68f04556 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:354
quote: tool: cargo-deny@0.20.2
@row_add n=128 row=decision/zizmor-workflow-audit area=dependency-security rec=fct-5ktlwg6z@1,fct-cmrmoq4t@1
name: zizmor for workflows
rule: zizmor 1.30.1 or newer audits the GitHub Actions workflows.
@row_check n=128 row=decision/zizmor-workflow-audit sha=07515ae1e7a9 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:356
quote: run: cargo install --locked zizmor@1.30.1
@row_add n=129 row=decision/cargo-audit-not-gated area=dependency-security rec=fct-ioqfwukn@1
name: cargo-audit not gated
rule: cargo-audit is not gated; it is reserved for binary scans of cargo-auditable release binaries.
why: it would duplicate cargo-deny advisories on the same lockfile
@row_check n=129 row=decision/cargo-audit-not-gated sha=10efa8bdb3f4 v=holds judge=sonnet head=8834195b0683
searched: cargo-audit|cargo audit · in .github/**, scripts/**, Cargo.toml, crates/*/Cargo.toml, deny*.toml
@row_add n=130 row=decision/no-ignored-advisories area=dependency-security rec=fct-serp72ds@2
name: No ignored advisories
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores no advisory; a new ignore follows the Critical CVE response SLA, its reason naming the disabled feature.
@row_check n=130 row=decision/no-ignored-advisories sha=8a4ff1a64ca7 v=holds judge=sonnet head=8834195b0683
at: deny.toml:8-11
quote: unmaintained = "all" unsound = "all" yanked = "deny" ignore = []
@row_add n=131 row=decision/crates-io-only area=dependency-security rec=fct-l55i2g5k@1
name: crates.io as the only dependency source
rule: cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only.
why: keeps CVE-2026-5222's sparse-registry class out of reach
@row_check n=131 row=decision/crates-io-only sha=fa28f76e7ce4 v=holds judge=sonnet head=8834195b0683
at: deny.toml:24-25
quote: unknown-registry = "deny" unknown-git = "deny"
@row_add n=132 row=decision/licence-allow-list area=dependency-security rec=fct-32d4ty7r@2
name: Closed licence allow list
rule: The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0; 0BSD is never added to the allow list.
why: keeping 0BSD out of the allow list makes any other 0BSD crate still fail the gate; ratified
guard: cargo deny check licenses
except: 0BSD admitted per crate for doctest-file, pulled in by interprocess; each new exception needs its own Decisions Log entry
@row_check n=132 row=decision/licence-allow-list sha=f204fe194a5a v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:14
quote: allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]
guard_at: .github/workflows/ci.yml:367
@row_add n=133 row=decision/rmcp-http-server-and-auth-banned area=dependency-security rec=fct-mrnvctku@1
name: rmcp HTTP server and auth features banned
rule: cargo-deny denies the rmcp features transport-streamable-http-server and auth.
@row_check n=133 row=decision/rmcp-http-server-and-auth-banned sha=20ecd144e2be v=holds judge=sonnet head=8834195b0683
at: deny.toml:44-45
quote: crate = "rmcp" deny = ["transport-streamable-http-server", "auth"]
@row_add n=134 row=decision/serde-json-bounded-depth area=dependency-security rec=fct-mrnvctku@1
name: serde_json recursion limit kept
rule: cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit.
guard: scripts/deny-probes.sh
@row_check n=134 row=decision/serde-json-bounded-depth sha=e7dab1a8b3ff v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:54-55
quote: crate = "serde_json" deny = ["unbounded_depth"]
guard_at: scripts/deny-probes.sh:32
@row_add n=135 row=decision/no-c-building-crates area=dependency-security rec=fct-cavsczz7@1
name: No C-building crates
rule: Crates that build C code are banned in the root dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_check n=135 row=decision/no-c-building-crates sha=1d129c8f86ed v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
guard_at: scripts/deny-probes.sh:21
@row_add n=136 row=decision/fuzz-workspace-exemption area=dependency-security rec=fct-blte3oxa@1
name: Fuzz workspace outside the deny graph
rule: The fuzz crate is a separate test-only cargo workspace with its own committed lockfile, outside cargo-deny's root graph; its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it.
why: a ratified test-only exemption, holding only while the fuzz crate is never linked into a shipped binary and never joins the root workspace
@row_check n=136 row=decision/fuzz-workspace-exemption sha=f76abbc90727 v=holds judge=sonnet head=8834195b0683
at: fuzz/Cargo.toml:20-21
quote: [workspace] members = ["."]
at: Cargo.toml:182
quote: exclude = ["fuzz"]
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
@row_add n=137 row=decision/exact-dependency-pins area=dependency-security rec=fct-uydjlfhj@2
name: Exact pins for third-party crates
rule: Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly.
except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
@row_check n=137 row=decision/exact-dependency-pins sha=192e518376ca v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: fuzz/Cargo.toml:12-13
quote: arbitrary = "=1.4.2" libfuzzer-sys = "=0.4.13"
at: Cargo.lock:1170-1171
quote: name = "rmcp" version = "3.4.1"
@row_add n=138 row=decision/bytes-advisory-floor area=dependency-security rec=fct-q5ipmlaa@1
name: bytes version floor
rule: The lockfile must resolve bytes >=1.11.1.
why: RUSTSEC-2026-0007
@row_check n=138 row=decision/bytes-advisory-floor sha=b8a3f55a2bb2 v=holds judge=sonnet head=8834195b0683
at: Cargo.lock:122-123
quote: name = "bytes" version = "1.12.1"
@row_add n=139 row=decision/tracing-subscriber-floor area=dependency-security rec=fct-q5ipmlaa@1,fct-auxwpgtg@1
name: tracing-subscriber version floor
rule: If obs selects tracing-subscriber, it must be >=0.3.20.
why: RUSTSEC-2025-0055
@row_check n=139 row=decision/tracing-subscriber-floor sha=1865d142c5df v=holds judge=sonnet head=8834195b0683
at: Cargo.lock:1666-1667
quote: name = "tracing-subscriber" version = "0.3.23"
@row_add n=140 row=decision/product-toolchain-pin area=dependency-security rec=fct-lagqorre@1
name: Product toolchain pinned exactly
rule: The product toolchain is pinned exactly to Rust 1.98.1 with rustfmt and clippy, installed through rustup on the dev host and in CI.
@row_check n=140 row=decision/product-toolchain-pin sha=b82da41feb95 v=holds judge=sonnet head=8834195b0683
at: rust-toolchain.toml:2-3
quote: channel = "1.98.1" components = ["rustfmt", "clippy"]
@row_add n=141 row=decision/msrv-floor area=dependency-security rec=fct-lagqorre@1
name: Minimum supported Rust version
rule: The workspace rust-version floor is 1.96, checked in CI with the 1.96 toolchain.
guard: CI msrv job
@row_check n=141 row=decision/msrv-floor sha=2c8e3bf51aaa v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:188
quote: rust-version = "1.96"
at: .github/workflows/ci.yml:203
quote: run: rustup toolchain install 1.96 --profile minimal
guard_at: .github/workflows/ci.yml:191
@row_add n=142 row=decision/fuzz-nightly-pin area=dependency-security rec=fct-lagqorre@1
name: Fuzz nightly toolchain pinned
rule: The fuzz workspace pins its own exact nightly toolchain, separate from the product toolchain, installed through rustup.
@row_check n=142 row=decision/fuzz-nightly-pin sha=7f03148ef50c v=holds judge=sonnet head=8834195b0683
at: fuzz/rust-toolchain.toml:2
quote: channel = "nightly-2026-09-20"
at: .github/workflows/ci.yml:244
quote: run: rustup toolchain install
@row_add n=143 row=decision/pre-push-checks-pins-installs-nothing area=dependency-security rec=fct-2c53xvmz@1
name: pre-push checks tool pins, installs nothing
rule: viola-harness pre-push is never a new version source: it checks the host's cc, toolchain channel, cargo-nextest, cargo-mutants and cargo-llvm-cov against CI's own pins and installs nothing.
@row_check n=143 row=decision/pre-push-checks-pins-installs-nothing sha=3f9625a6d853 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:106
quote: return Err(Stop::new("tool-missing", "cc"));
at: crates/viola-e2e/src/harness/pre_push/linux.rs:109-115
quote: let channel = fs::read_to_string(ws.root.join("rust-toolchain.toml")) .ok() .and_then(|toml| fuzz_channel(&toml)) .ok_or_else(unreadable)?; let yml = fs::read_to_string(ws.root.join(".github").join("workflows").join("ci.yml")) .map_err(|_| unreadable())?; let pins = ci_pins(&yml)
at: crates/viola-e2e/src/harness/pre_push/linux.rs:132
quote: return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=144 row=contract/pre-push-refusals/tool-pin-mismatch area=dependency-security rec=fct-2c53xvmz@1
boundary: viola-harness pre-push refusals
item: tool-pin-mismatch
promise: Refused when an installed tool's version differs from its CI pin line.
@row_check n=144 row=contract/pre-push-refusals/tool-pin-mismatch sha=d35cee179205 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:132
quote: return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=145 row=contract/pre-push-refusals/tool-missing-cc area=dependency-security rec=fct-2c53xvmz@1
boundary: viola-harness pre-push refusals
item: tool-missing: cc
promise: Refused when the host has no C toolchain.
@row_check n=145 row=contract/pre-push-refusals/tool-missing-cc sha=420076a7dc9b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:106
quote: return Err(Stop::new("tool-missing", "cc"));
@row_add n=146 row=decision/nothing-runs-as-root area=dependency-security rec=fct-2c53xvmz@1
name: Nothing launched as root
rule: No harness command, gate or pre-push stage launches anything as uid 0, and no provisioning script exists.
@row_check n=146 row=decision/nothing-runs-as-root sha=3050b9e8a3ce v=holds judge=sonnet head=8834195b0683
searched: sudo|doas|geteuid|uid 0 · in **/*.{rs,sh,py,yml} (only geteuid reads in strict.rs/channel, none launching anything)
searched: Command::new("sudo") · in crates/viola-e2e/src/harness/pre_push
searched: provisioning script · in scripts/* (none; scripts hold only gate/probe scripts)
@row_add n=147 row=decision/manual-dependency-updates area=dependency-security rec=fct-27vpn33i@1
name: Manual dependency update review
rule: Dependency updates are reviewed manually, triggered by the scheduled advisory run; no automated update bot is used.
why: no automated update bot was researched; the rmcp minor range makes the scheduled run the main signal for that crate
@row_check n=147 row=decision/manual-dependency-updates sha=edf99751d358 v=holds judge=sonnet head=8834195b0683
searched: dependabot.yml|renovate.json|.renovaterc · in repository root and .github/ (none; .github holds only workflows/ci.yml and nightly.yml)
@row_add n=148 row=decision/ci-cargo-deny-gate area=dependency-security rec=fct-mi73d5qc@1
name: cargo-deny gate in CI
rule: CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory, yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
@row_check n=148 row=decision/ci-cargo-deny-gate sha=e84fc3e6f037 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: deny.toml:10
quote: yanked = "deny"
@row_add n=149 row=decision/scheduled-advisory-run area=dependency-security rec=fct-eugkq5vm@1
name: Weekly scheduled advisory run
rule: A separate scheduled workflow runs cargo deny advisory checks weekly, without cache, on both the root and the fuzz manifests.
why: the advisory database changes without code changes
@row_check n=149 row=decision/scheduled-advisory-run sha=39071e5fb863 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:5
quote: - cron: "17 4 * * 1"
at: .github/workflows/nightly.yml:29
quote: cargo deny check advisories
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=150 row=decision/scheduled-fuzzing area=dependency-security rec=fct-eugkq5vm@1
name: Scheduled fuzzing
rule: The scheduled workflow runs each fuzz target for 120 seconds on the pinned fuzz nightly and uploads the fuzz artifacts on failure.
@row_check n=150 row=decision/scheduled-fuzzing sha=d22b3240bb11 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:55
quote: cargo "+$channel" fuzz run --fuzz-dir fuzz "$t" "fuzz/corpus/$t" -- -max_total_time=120
at: .github/workflows/nightly.yml:59-63
quote: if: failure() uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1 with: name: fuzz-artifacts path: fuzz/artifacts/
@row_add n=151 row=decision/no-workflow-concurrency area=dependency-security rec=fct-pxt7iprv@2
name: No concurrency block in workflows
rule: The CI workflows carry no concurrency block; zizmor's pedantic concurrency-limits finding is declined and stays visible.
why: a concurrency group cancels pending runs even with cancel-in-progress false, which would drop a cancelled run's always() gate and upload chain
@row_check n=151 row=decision/no-workflow-concurrency sha=67ae28809d42 v=holds judge=sonnet head=8834195b0683
searched: concurrency · in .github/workflows/*.yml (no match)
@row_add n=152 row=decision/release-build-without-test-features area=dependency-security rec=fct-h3byb6pq@2
name: Release build free of test-only features
rule: The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent).
guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
@row_check n=152 row=decision/release-build-without-test-features sha=95eb179f50f5 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/release-check.sh:19
quote: | .target.name as $t | (.features // [])[] | select(. == "test-support" or . == "fake-agent")
at: scripts/release-check.sh:31-32
quote: if [ "$name" != "viola" ]; then echo "release-check: FAILED — test-only binary $name"
guard_at: scripts/release-check.sh:60
@row_add n=153 row=contract/release-check-output/test-only-feature area=dependency-security rec=fct-h3byb6pq@2
boundary: release-check output
item: release-check: FAILED — test-only feature {feature} in {target}
promise: Printed when a release artifact's features include a test-only feature.
@row_check n=153 row=contract/release-check-output/test-only-feature sha=af8bd6960249 v=holds judge=sonnet head=8834195b0683
at: scripts/release-check.sh:22
quote: echo "release-check: FAILED — test-only feature $(head -n 1 <<< "$featured")"
@row_add n=154 row=decision/test-support-dev-only area=dependency-security rec=fct-h3byb6pq@2
name: test-support enabled only for development
rule: viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
@row_check n=154 row=decision/test-support-dev-only sha=89e2665d4970 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:175
quote: viola-channel = { path = "crates/viola-channel", features = ["test-support"] }
at: Cargo.toml:163
quote: [dev-dependencies]
searched: test-support · in **/Cargo.toml (only Cargo.toml:175 enables it; crates/viola-channel/Cargo.toml:14 defines it)
@row_add n=155 row=decision/zizmor-workflow-gate area=dependency-security rec=fct-ivw6qaje@1
name: zizmor workflow audit gate
rule: zizmor audits the workflows in CI and fails on unpinned actions, excessive permissions, template injection and cache poisoning.
@row_check n=155 row=decision/zizmor-workflow-gate sha=71b6e15b1e7b v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
@row_add n=156 row=decision/perf-uploads-scan-gated area=dependency-security rec=fct-zhdn6mmq@2
name: perf job uploads gated by secret scan
rule: The perf job's hyperfine exports and diagnostics are uploaded only when its own secret scan succeeds, the scan output only when it fails; every perf input is synthetic.
@row_check n=156 row=decision/perf-uploads-scan-gated sha=902fd36ae1c2 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:170
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:178
quote: if: always() && steps.secret-scan.outcome == 'failure'
@row_add n=157 row=decision/g2-single-panic-exemption area=dependency-security rec=fct-wsxv34pw@1
name: G2 zero-panics gate has one exemption
rule: CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry.
guard: G2 --probe run before every G2 check
@row_check n=157 row=decision/g2-single-panic-exemption sha=30e247f076ff v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
guard_at: scripts/g2-zero-panics.sh:62
@row_add n=158 row=decision/toolchains-via-rustup area=dependency-security rec=fct-ol2hzo3z@1
name: CI toolchains installed through rustup
rule: CI toolchains come from rustup toolchain install steps; no toolchain action is used.
@row_check n=158 row=decision/toolchains-via-rustup sha=f0356af5d80f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:203
quote: run: rustup toolchain install 1.96 --profile minimal
at: .github/workflows/ci.yml:244
quote: run: rustup toolchain install
searched: dtolnay/rust-toolchain|actions-rs/toolchain|setup-rust · in .github/workflows/*.yml (none; uses: lines are checkout, rust-cache, install-action, upload-artifact)
@row_add n=159 row=decision/event-values-through-env area=dependency-security rec=fct-npplilir@2
name: Event payload values only through env
rule: A value from the GitHub event payload reaches a workflow step only through env, never through an expression inside run.
@row_check n=159 row=decision/event-values-through-env sha=5780897b193f v=holds judge=sonnet head=8834195b0683
searched: github.event · in .github/workflows/*.yml (no match)
searched: ${{ github. · in .github/workflows/*.yml (no match; only matrix.os and steps.*.outcome expressions)
@row_add n=160 row=decision/least-workflow-permissions area=dependency-security rec=fct-lcepm2kd@1
name: Least workflow permissions
rule: Workflows set empty permissions at workflow level and contents read at job level.
@row_check n=160 row=decision/least-workflow-permissions sha=5379dd62fc07 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
at: .github/workflows/ci.yml:13-14
quote: permissions:       contents: read
at: .github/workflows/nightly.yml:8
quote: permissions: {}
at: .github/workflows/nightly.yml:14-15
quote: permissions:       contents: read
@row_add n=161 row=decision/cve-sla-enabled-feature area=dependency-security rec=fct-4r3p2tgk@2,fct-x6oor32x@1
name: Critical CVE response for an enabled feature
rule: Within 7 days of a RustSec or GHSA advisory against a crate and feature viola enables, the crate is bumped.
@row_check n=161 row=decision/cve-sla-enabled-feature sha=9a6b1971cd6e v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=162 row=decision/cve-sla-disabled-feature area=dependency-security rec=fct-oj56jp4z@1,fct-x6oor32x@1
name: Critical CVE response for a disabled feature
rule: Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.
@row_check n=162 row=decision/cve-sla-disabled-feature sha=2a81ea90cafb v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=163 row=decision/release-artifact-attestations area=dependency-security rec=fct-27c7vqqx@2,fct-qhcowchx@1
name: Release built with dist and attestations
rule: A v1.x release requires dist >=0.33.0 with GitHub Artifact Attestations.
@row_check n=163 row=decision/release-artifact-attestations sha=24ec048ea00a v=unverifiable judge=sonnet head=8834195b0683
searched: attest · in .github/**, Cargo.toml, scripts/*
searched: dist · in Cargo.toml, dist*.toml, .github/**
why: no evidence found
@row_add n=164 row=decision/release-binary-audit area=dependency-security rec=fct-u57emdou@1,fct-qhcowchx@1
name: Auditable release binaries
rule: A v1.x release requires cargo-auditable >=0.7.6, with cargo audit bin (cargo-audit >=0.22.2) run on the release binaries.
@row_check n=164 row=decision/release-binary-audit sha=b0102124b530 v=unverifiable judge=sonnet head=8834195b0683
searched: auditable · in .github/**, Cargo.toml, scripts/*
searched: cargo audit · in .github/**, scripts/*
why: no evidence found
@row_add n=165 row=decision/release-cache-poisoning-addressed area=dependency-security rec=fct-but52eow@1,fct-qhcowchx@1
name: Cache-poisoning findings addressed before release
rule: A v1.x release requires zizmor's cache-poisoning findings to be addressed.
@row_check n=165 row=decision/release-cache-poisoning-addressed sha=6567fe19b78e v=unverifiable judge=sonnet head=8834195b0683
searched: cache-poisoning · in .github/**, scripts/*, deny*.toml
why: no evidence found
@row_add n=166 row=decision/local-boundary-no-auth-library area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Local boundary instead of an auth library
rule: Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
@row_check n=166 row=decision/local-boundary-no-auth-library sha=3fdd511eac55 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-state/src/strict.rs:137
quote: pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> {
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
at: crates/viola-state/src/liveness.rs:152
quote: pub fn verify_server(
@row_add n=167 row=decision/owner-only-home-and-pinned-binary area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Owner-only home and pinned binary re-hash
rule: The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
@row_check n=167 row=decision/owner-only-home-and-pinned-binary sha=f867dd441e4d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
at: crates/viola-state/src/strict.rs:84
quote: pub fn check_home_tree(
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
@row_add n=168 row=decision/code-files-rewritten-at-start area=bootstrap-phases-derive-for-route-setup-project rec=fct-pz6zubwd@2
name: Code-bearing files rewritten at start
rule: Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
@row_check n=168 row=decision/code-files-rewritten-at-start sha=3ec4332ee7fd v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:475-478
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) { let path = plugin_dir.join(rel); create_private_dir(path.parent().unwrap_or(&plugin_dir))?; replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
@row_add n=169 row=decision/bounded-external-readers area=bootstrap-phases-derive-for-route-setup-project rec=fct-pqj3orx7@1
name: Bounded external readers
rule: Every external reader is wrapped by a size bound at a maximum frame size.
@row_check n=169 row=decision/bounded-external-readers sha=7126561fe743 v=holds judge=sonnet head=8834195b0683
at: src/run/env.rs:25
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/cmd/hook.rs:178
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
@row_add n=170 row=decision/dependency-audit-tools area=bootstrap-phases-derive-for-route-setup-project rec=fct-n3fibuep@2
name: cargo-deny and zizmor as audit tools
rule: Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
@row_check n=170 row=decision/dependency-audit-tools sha=9731c467dfdb v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:354
quote: tool: cargo-deny@0.20.2
at: .github/workflows/ci.yml:356
quote: run: cargo install --locked zizmor@1.30.1
@row_add n=171 row=absence/stored-secrets area=bootstrap-phases-derive-for-route-setup-project rec=fct-gh3kajmo@1
capability: Stored secrets in v1
@row_check n=171 row=absence/stored-secrets sha=2a8e02332ced v=holds judge=sonnet head=8834195b0683
searched: keyring|credential|password · in crates/**/*.rs, src/**/*.rs, Cargo.toml, crates/**/Cargo.toml (only client.rs peer credentials, SO_PEERCRED; and persist-credentials: false in workflows)
@row_add n=172 row=decision/scan-gated-ci-uploads area=bootstrap-phases-derive-for-route-setup-project rec=fct-rwpm3oj4@1
name: CI uploads gated by the artifact canary scan
rule: The CI test job runs the artifact canary scan against the NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass.
except: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, leave CI unscanned; a non-synthetic seed must first be dropped
@row_check n=172 row=decision/scan-gated-ci-uploads sha=0b716caeda44 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:98
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=173 row=decision/error-display-no-upstream-text area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
name: Error messages carry no upstream text
rule: The Display output of the product error enums never interpolates upstream text or absolute paths.
@row_check n=173 row=decision/error-display-no-upstream-text sha=432d3c791e64 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:36
quote: #[error("channel endpoint could not be bound")]
at: crates/viola-pty/src/lib.rs:84
quote: Self::Open(_) => "pty open failed",
@row_add n=174 row=contract/mcp-structured-content/codes area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
boundary: MCP structuredContent
item: refusal, detail and error codes
promise: structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
@row_check n=174 row=contract/mcp-structured-content/codes sha=182c16576b79 v=stale judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:16
quote: CallToolResult::structured(payload)
shows: An ok result carries the whole channel payload as structuredContent; only the refusal, unreachable, fault and invalid-params results carry codes alone (refusal/detail or error/detail).
cell: promise
value: structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.
@row_add n=175 row=decision/serde-errors-mapped-to-fixed-message area=bootstrap-phases-derive-for-route-setup-project rec=fct-y26eifc2@1
name: Upstream-quoting errors mapped to fixed messages
rule: An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics.
why: serde errors quote the offending value
@row_check n=175 row=decision/serde-errors-mapped-to-fixed-message sha=24347a151888 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:258-260
quote: if let Some(sink) = sink { let line = chain_detail_line(&timestamp(Utc::now()), sink.process, &sink.instance, error); write_detail(&sink.home, &sink.instance, sink.process, &line);
@row_add n=176 row=decision/never-log-claude-values area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: CLAUDE values never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
@row_check n=176 row=decision/never-log-claude-values sha=479cbbe453cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:76
quote: plan.remove.push(name);
@row_add n=177 row=decision/user-content-only-to-diagnostics area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: User content only in diagnostics
rule: Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data.
except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design
@row_check n=177 row=decision/user-content-only-to-diagnostics sha=b7c9fd3ce9b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
@row_add n=178 row=decision/no-paths-in-error-output area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: No paths or type names in error output
rule: Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data.
why: the OS username is the only PII in scope
@row_check n=178 row=decision/no-paths-in-error-output sha=cb8a20b743f4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:117
quote: pub fn sanitised_fault(error: &Value) -> Option<Value> {
@row_add n=179 row=decision/diagnostics-owner-only area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: Diagnostics files owner-only
rule: Files under the instance diagnostics are mode 0600.
@row_check n=179 row=decision/diagnostics-owner-only sha=c42f80f4b98f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
@row_add n=180 row=decision/human-output-escapes-controls area=bootstrap-phases-derive-for-route-setup-project rec=fct-auxwpgtg@1
name: Human-mode output escapes control characters
rule: Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
@row_check n=180 row=decision/human-output-escapes-controls sha=4982fe3437e3 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
at: src/human/escape.rs:23
quote: if c.is_control() {
@row_add n=181 row=decision/env-not-config-channel area=secret-management rec=fct-ugohyrrh@2
name: Environment variables are not configuration
rule: Environment variables are never a configuration channel for the product.
@row_check n=181 row=decision/env-not-config-channel sha=b9387b08c3eb v=holds judge=sonnet head=8834195b0683
searched: #[arg(env · in src/** and crates/*/src/**
searched: env = " · in src/** and crates/*/src/**
searched: RUST_LOG · in src/** and crates/*/src/** (only a comment at src/obs.rs:180 saying never RUST_LOG)
@row_add n=182 row=decision/claude-credentials-stripped-from-child area=secret-management rec=fct-ugohyrrh@2
name: Inherited CLAUDE credentials stripped from the child
rule: The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
@row_check n=182 row=decision/claude-credentials-stripped-from-child sha=88785c8f7cef v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:72
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str());
at: crates/viola-agent-claude/src/lib.rs:76
quote: plan.remove.push(name);
at: src/cmd/run.rs:644
quote: env_remove: strip.remove.clone(),
@row_add n=183 row=decision/credential-strip-by-name-only area=secret-management rec=fct-ugohyrrh@2
name: Credential strip reads names only
rule: The environment strip handles variable names only and never reads a variable's value.
@row_check n=183 row=decision/credential-strip-by-name-only sha=f11df54e2aa3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:289
quote: let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), persistent);
@row_add n=184 row=decision/hermetic-child-environment area=secret-management rec=fct-z4tolcdo@1
name: Hermetic child environment in the pre-push gate
rule: The local pre-push gate launches every child under env -i with exactly HOME and PATH, so no host environment value crosses into a child.
except: Any assignment whose value comes from the host is a boundary widening that needs its own Decisions Log entry.
@row_check n=184 row=decision/hermetic-child-environment sha=088188ee0102 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39
quote: cmd.arg("-i")
at: crates/viola-e2e/src/harness/pre_push/linux.rs:42
quote: "PATH={home}/.cargo/bin:/usr/local/bin:/usr/bin:/bin",
@row_add n=185 row=decision/no-uid-0-launch area=secret-management rec=fct-z4tolcdo@1
name: No root launch
rule: No child is ever launched as uid 0.
@row_check n=185 row=decision/no-uid-0-launch sha=6c44a8542bcd v=holds judge=sonnet head=8834195b0683
searched: .uid( · in crates/viola-e2e/src/harness/pre_push.rs and pre_push/
searched: setuid · in crates/viola-e2e/src/harness/
searched: Command::new · in crates/viola-e2e/src/harness/pre_push* (only ENV = /usr/bin/env launches in linux.rs)
@row_add n=186 row=decision/no-credential-literals area=secret-management rec=fct-kmhnhsns@1
name: No credential literals in code
rule: No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
@row_check n=186 row=decision/no-credential-literals sha=429c772afbca v=holds judge=sonnet head=8834195b0683
searched: sk-ant · in src/**, crates/*/src/**, plugin/**, .github/**
searched: ghp_ · in src/**, crates/*/src/**, plugin/**, .github/**
searched: BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY · in src/**, crates/*/src/**, plugin/**, .github/**
searched: Bearer [A-Za-z0-9]{10} · in src/**, crates/*/src/**, plugin/**, .github/**
@row_add n=187 row=decision/artifact-canary-scan area=secret-management rec=fct-ab75crtl@1,fct-h2ql6c6w@1
name: CI artifact canary scan
rule: CI always runs the artifact canary secret scan over every file under the agent-run artifact directory, and the test-home uploads run only when it succeeds.
except: The mutants chunk diff is skipped by exact path and excluded from the harness upload, so it is never uploaded.
@row_check n=187 row=decision/artifact-canary-scan sha=d6123628be54 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:73-74
quote: id: secret-scan if: always()
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: crates/viola-e2e/src/harness/secret_scan.rs:79-80
quote: let mutation_diff = roots.agent_run.join("chunk.diff"); capture.retain(|path| *path != mutation_diff);
@row_add n=188 row=contract/artifact-canary-scan-classes/claude-stripped area=secret-management rec=fct-rrs6uojm@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: claude-stripped
promise: The scan fails on any of the twelve stripped CLAUDE* canary needles.
@row_check n=188 row=contract/artifact-canary-scan-classes/claude-stripped sha=a549a2c1c080 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:17
quote: ("claude-stripped", "canary-token-value-7f3a"),
at: crates/viola-e2e/src/harness/secret_scan.rs:124-126
quote: for (class, needle) in CRITICAL { for offset in find_all(&bytes, needle.as_bytes()) { report(offset, class);
@row_add n=189 row=contract/artifact-canary-scan-classes/cookie-header area=secret-management rec=fct-26k2fw6o@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: cookie header
promise: The scan fails on a cookie: header, matched case-insensitively.
@row_check n=189 row=contract/artifact-canary-scan-classes/cookie-header sha=2830ac6ee776 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:129
quote: for offset in find_all(&lower, b"cookie:") {
@row_add n=190 row=contract/artifact-canary-scan-classes/content-canary area=secret-management rec=fct-pt67jwgh@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: content canary
promise: The content canary fails the scan only in home-level role files.
@row_check n=190 row=contract/artifact-canary-scan-classes/content-canary sha=3eb84799881f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:132-133
quote: if role { for (class, needle) in CONTENT {
@row_add n=191 row=contract/artifact-canary-scan-classes/diagnostics-mode area=secret-management rec=fct-umh22lbq@1,fct-ab75crtl@1
boundary: artifact canary scan classes
item: diagnostics file mode
promise: The scan fails on a diagnostics file whose mode is not 0600.
@row_check n=191 row=contract/artifact-canary-scan-classes/diagnostics-mode sha=fa4a41e87d1d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:175
quote: mode.filter(|&m| !owner_only(m))
at: crates/viola-e2e/src/harness/secret_scan.rs:180
quote: mode & 0o777 == 0o600
@row_add n=192 row=decision/canary-scan-never-echoes-matches area=secret-management rec=fct-h2ql6c6w@1
name: Canary scan never echoes matches
rule: The artifact canary scan never prints or writes the matched bytes.
@row_check n=192 row=decision/canary-scan-never-echoes-matches sha=12e9efdffd2b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=193 row=absence/secret-manager area=secret-management rec=fct-rijajpwh@1
capability: A secret manager, and so any secret access auditing
@row_check n=193 row=absence/secret-manager sha=ce66f127615f v=holds judge=sonnet head=8834195b0683
searched: keyring · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/**
searched: secret.?manager · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/**
searched: vault · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/**
@row_add n=194 row=decision/messaging-credentials-are-secret area=secret-management rec=fct-b7clcfac@1,fct-3wufpdib@1
name: Messaging token and socket are secrets
rule: The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
@row_check n=194 row=decision/messaging-credentials-are-secret sha=b0c96089273c v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:23
quote: "CLAUDE_CODE_MESSAGING_SOCKET",
at: crates/viola-agent-claude/src/lib.rs:24
quote: "CLAUDE_CODE_MESSAGING_TOKEN",
@row_add n=195 row=decision/sanitised-external-errors area=error-handling rec=fct-xjdp32rs@1,fct-zhbrnvo2@1
name: Sanitised external error responses
rule: Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
@row_check n=195 row=decision/sanitised-external-errors sha=0e66a9128973 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:118
quote: let code = error["code"].as_i64()?;
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
at: src/cmd/client.rs:70
quote: return sanitised_fault(error).map(Reply::Fault);
@row_add n=196 row=decision/fixed-error-display-messages area=error-handling rec=fct-scytqusw@1,fct-xjdp32rs@1
name: Fixed error display messages
rule: Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
@row_check n=196 row=decision/fixed-error-display-messages sha=062808425470 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:84
quote: Self::Open(_) => "pty open failed",
at: crates/viola-state/src/strict.rs:20
quote: #[error("a trusted state path is not owned by this user")]
at: crates/viola-channel/src/lib.rs:34
quote: #[error("channel endpoint is held by another process")]
at: crates/viola-state/src/stamps.rs:40
quote: #[error("the capability stamps failed the strict-modes check")]
at: crates/viola-state/src/pin.rs:19
quote: #[error("the pinned copy failed its integrity check")]
at: crates/viola-mcp/src/lib.rs:40
quote: #[error("the MCP runtime could not be built")]
@row_add n=197 row=contract/channel-errors/-32603 area=error-handling rec=fct-crmfdsu7@1
boundary: channel JSON-RPC errors
item: -32603
promise: An internal error carries the fixed message "internal error" with data null.
@row_check n=197 row=contract/channel-errors/-32603 sha=c6e5b573221a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:97
quote: _ => Value::Null,
@row_add n=198 row=contract/mcp-iserror/instance-unreachable area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error instance-unreachable
promise: An unreachable instance reads error instance-unreachable with detail one of its closed causes.
@row_check n=198 row=contract/mcp-iserror/instance-unreachable sha=a841d5f9c56a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-core/src/lib.rs:196-201
quote: Self::StrictModes => "strict-modes-failed", Self::InstanceDead => "instance-dead", Self::UnwrappedName => "unwrapped-name", Self::EndpointLost => "endpoint-lost", Self::ServerVerify => "server-verify-failed", Self::SocketDir => "socket-dir-refused",
@row_add n=199 row=contract/mcp-iserror/wrapper-fault area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error wrapper-fault
promise: A wrapper fault reads error wrapper-fault with a sanitised code, message and data, never the peer's own.
@row_check n=199 row=contract/mcp-iserror/wrapper-fault sha=8b93026c42aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:60-61
quote: return sanitised_fault(fault) .map(|detail| error("wrapper-fault", detail))
@row_add n=200 row=contract/mcp-iserror/invalid-params area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: error invalid-params
promise: Invalid parameters read error invalid-params with a closed field word.
@row_check n=200 row=contract/mcp-iserror/invalid-params sha=6adf0515a350 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=201 row=contract/instance-unreachable-cause/strict-modes-failed area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: strict-modes-failed
promise: A closed cause of instance-unreachable.
@row_check n=201 row=contract/instance-unreachable-cause/strict-modes-failed sha=6a166cc275b7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=202 row=contract/instance-unreachable-cause/instance-dead area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: instance-dead
promise: A closed cause of instance-unreachable.
@row_check n=202 row=contract/instance-unreachable-cause/instance-dead sha=065b0d415583 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: crates/viola-mcp/src/result.rs:53
quote: Err(ChannelError::Connect(_)) => return Ok(unreachable(UnreachableCause::InstanceDead)),
@row_add n=203 row=contract/instance-unreachable-cause/unwrapped-name area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: unwrapped-name
promise: A closed cause of instance-unreachable.
@row_check n=203 row=contract/instance-unreachable-cause/unwrapped-name sha=a875abfcd507 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
@row_add n=204 row=contract/instance-unreachable-cause/endpoint-lost area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: endpoint-lost
promise: A closed cause of instance-unreachable.
@row_check n=204 row=contract/instance-unreachable-cause/endpoint-lost sha=f248a45b0dd0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: crates/viola-mcp/src/result.rs:55
quote: return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=205 row=contract/instance-unreachable-cause/server-verify-failed area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: server-verify-failed
promise: A closed cause of instance-unreachable.
@row_check n=205 row=contract/instance-unreachable-cause/server-verify-failed sha=00eec1b01594 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=206 row=contract/instance-unreachable-cause/socket-dir-refused area=error-handling rec=fct-kulo4ws7@1
boundary: instance-unreachable detail causes
item: socket-dir-refused
promise: A closed cause of instance-unreachable.
@row_check n=206 row=contract/instance-unreachable-cause/socket-dir-refused sha=6af55dd3f5ca v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=207 row=contract/mcp-iserror/refusal area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: refusal
promise: A refusal that is no code word reads unknown, alike on MCP and CLI --json.
@row_check n=207 row=contract/mcp-iserror/refusal sha=047df7ab4a46 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:76
quote: RefusalReason::Unknown.as_str()
at: src/cmd/client.rs:80
quote: RefusalReason::Unknown.as_str().to_owned()
@row_add n=208 row=contract/mcp-iserror/detail area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError structuredContent
item: detail
promise: A string detail that is no known detail reads null, alike on MCP and CLI --json.
@row_check n=208 row=contract/mcp-iserror/detail sha=364922483441 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
@row_add n=209 row=contract/mcp-iserror/text area=error-handling rec=fct-kulo4ws7@1
boundary: MCP isError text
item: refused text
promise: Every isError text reads refused: <code>.
@row_check n=209 row=contract/mcp-iserror/text sha=31556b10720d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=210 row=contract/mcp-errors/-32603 area=error-handling rec=fct-kulo4ws7@1
boundary: MCP JSON-RPC errors
item: -32603
promise: Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_check n=210 row=contract/mcp-errors/-32603 sha=0f3791c8f51a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:194
quote: ErrorCode::INTERNAL_ERROR,
at: crates/viola-mcp/src/tools.rs:243
quote: Ok(Err(McpError::Internal)) | Err(_) => Err(internal_fault()),
@row_add n=211 row=decision/full-error-detail-to-diagnostics area=error-handling rec=fct-b26ns4gs@1
name: Full error detail only in diagnostics
rule: Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
@row_check n=211 row=decision/full-error-detail-to-diagnostics sha=25b2563a45f4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:331
quote: if let Ok(mut file) = open_private_append(&path) {
at: src/obs.rs:597
quote: assert_eq!(mode(&path), 0o600);
@row_add n=212 row=decision/stderr-never-log-floor area=error-handling rec=fct-b26ns4gs@1
name: stderr honours the never-log floor
rule: stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
@row_check n=212 row=decision/stderr-never-log-floor sha=151b394e3791 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:54
quote: let _ = write_internal_error(&mut io::stderr().lock());
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=213 row=decision/hook-silent-exit-zero area=error-handling rec=fct-r24nqlwe@1
name: hook is silent and always exits 0
rule: The hook verb and its hidden capture arms never write to stderr and always exit 0; a failing security check ends in exit 0 with no body.
why: So the dialog still renders for the human.
@row_check n=213 row=decision/hook-silent-exit-zero sha=371f78308355 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=214 row=contract/channel-errors/-32600 area=error-handling rec=fct-k74n6zto@1
boundary: channel JSON-RPC errors
item: -32600
promise: A frame over MAX_FRAME gets the JSON-RPC error -32600.
@row_check n=214 row=contract/channel-errors/-32600 sha=64be2c6ee3d1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:268
quote: frame: error_response(None, ProtocolError::InvalidRequest),
at: crates/viola-channel/src/lib.rs:72
quote: Self::InvalidRequest => -32600,
@row_add n=215 row=contract/channel-errors/-32602 area=error-handling rec=fct-k74n6zto@1
boundary: channel JSON-RPC errors
item: -32602
promise: A release call carrying from gets the JSON-RPC error -32602.
@row_check n=215 row=contract/channel-errors/-32602 sha=7ca4cf8ccdb0 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:206
quote: return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
@row_add n=216 row=contract/channel-refusal/control-character area=error-handling rec=fct-k74n6zto@1
boundary: channel result.refusal
item: not-delivered/control-character
promise: A text holding a refused control character is answered with result.refusal not-delivered/control-character.
@row_check n=216 row=contract/channel-refusal/control-character sha=eea43ca5d450 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88-89
quote: if let Err(detail) = viola_core::validate_paste_text(text) { return ctx.not_delivered(None, detail);
at: crates/viola-core/src/lib.rs:358
quote: Err(NotDelivered::ControlCharacter)
@row_add n=217 row=contract/channel-refusal/empty-text area=error-handling rec=fct-k74n6zto@1
boundary: channel result.refusal
item: not-delivered/empty-text
promise: A text whose typed text is empty is answered with result.refusal not-delivered/empty-text.
@row_check n=217 row=contract/channel-refusal/empty-text sha=9e4118393b95 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=218 row=absence/external-error-reporting area=error-handling rec=fct-6tg2dujf@1
capability: An external error reporting integration
@row_check n=218 row=absence/external-error-reporting sha=a30d237a6e7d v=holds judge=sonnet head=8834195b0683
searched: sentry · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: reqwest|ureq|isahc|TcpStream · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=219 row=decision/logging-bans-before-reporting area=error-handling rec=fct-6tg2dujf@1
name: Logging bans apply to any external reporter
rule: If an external error reporter is ever added, the logging bans apply before transmission.
@row_check n=219 row=decision/logging-bans-before-reporting sha=31221465341a v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=220 row=decision/no-socket-in-tmp-root area=security-anti-patterns-never-do-these rec=fct-zshlqykf@1
name: No Unix socket at the temp root
rule: A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory.
why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
@row_check n=220 row=decision/no-socket-in-tmp-root sha=9bd6ecc98468 v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-channel/src/endpoint.rs:40
quote: None => PathBuf::from(format!("{host path}{uid}")),
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
@row_add n=221 row=decision/socket-mode-not-sole-control area=security-anti-patterns-never-do-these rec=fct-zyth7okk@2
name: Socket file mode is not the only control
rule: The socket file's 0600 mode is never the only Unix access control.
why: unix(7) says socket permissions may be ignored.
@row_check n=221 row=decision/socket-mode-not-sole-control sha=ef6458cc4d86 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
@row_add n=222 row=decision/chmod-socket-after-bind area=security-anti-patterns-never-do-these rec=fct-zyth7okk@2
name: Socket mode set by chmod after bind
rule: The socket mode is set by chmod after the bind, never through the listener's mode option.
why: Set while macOS, where that option fails the bind, was a target, and kept.
@row_check n=222 row=decision/chmod-socket-after-bind sha=ef02a4c3fc47 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:132-134
quote: // After the bind, not `ListenerOptionsExt::mode`: the 0700 directory already closes the umask // window (security-plan IPC access control, Unix). fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
@row_add n=223 row=decision/no-abstract-namespace-sockets area=security-anti-patterns-never-do-these rec=fct-2g5b5nut@1
name: No abstract-namespace sockets
rule: Linux abstract-namespace sockets are never used.
why: They have no filesystem permissions.
@row_check n=223 row=decision/no-abstract-namespace-sockets sha=fc9f641988e1 v=holds judge=sonnet head=8834195b0683
searched: GenericNamespaced · in src/**/*.rs and crates/**/*.rs
searched: to_ns_name · in src/**/*.rs and crates/**/*.rs
searched: abstract · in src/**/*.rs and crates/**/*.rs
@row_add n=224 row=decision/verify-server-before-frame area=security-anti-patterns-never-do-these rec=fct-6cauxi6n@2
name: Verify the server before any frame
rule: No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.
@row_check n=224 row=decision/verify-server-before-frame sha=59665446aed4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
@row_add n=225 row=decision/channel-from-not-identity area=security-anti-patterns-never-do-these rec=fct-ezxl4ss7@1
name: Channel from is not an identity
rule: The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
@row_check n=225 row=decision/channel-from-not-identity sha=c8f8b0e0f7dd v=holds judge=sonnet head=8834195b0683
at: src/run/send/params.rs:13
quote: /// `from`: absent, `null` or a valid name, self-reported; anything else is `-32602`.
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
@row_add n=226 row=absence/user-accounts area=security-anti-patterns-never-do-these rec=fct-32h3f5um@1
capability: User accounts or login endpoints
@row_check n=226 row=absence/user-accounts sha=fafe95a0723b v=holds judge=sonnet head=8834195b0683
searched: login|password|oauth · in src/**/*.rs and crates/**/*.rs (comments only, no code)
searched: account · in src/**/*.rs and crates/**/*.rs (only doc comments)
@row_add n=227 row=decision/server-side-revalidation area=security-anti-patterns-never-do-these rec=fct-bz3kcxxz@1
name: Wrapper re-validates every input
rule: MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
@row_check n=227 row=decision/server-side-revalidation sha=85e4e59c1f4b v=holds judge=sonnet head=8834195b0683
at: src/run/send/params.rs:17
quote: Some(Value::String(from)) => ViolaName::try_new(from.clone())
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: crates/viola-mcp/src/tools.rs:290
quote: .and_then(|value| Response::parse(value).ok())
@row_add n=228 row=decision/instance-path-from-validated-name area=security-anti-patterns-never-do-these rec=fct-erbjrvk6@1
name: Instance paths only from validated names
rule: An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
@row_check n=228 row=decision/instance-path-from-validated-name sha=f18b6a8a173a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: src/cmd/link.rs:103
quote: let instance_dir = home.join("instances").join(driven);
@row_add n=229 row=decision/refuse-control-characters area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Refuse control characters, never strip them
rule: A send text or answer free text holding any C0 control other than LF, CR and TAB, DEL or any C1 control is rejected with control-character and never typed into the PTY nor silently stripped.
except: send typing CR LF pairs and lone CRs as LF and dropping trailing CR and LF after validation is not stripping.
@row_check n=229 row=decision/refuse-control-characters sha=995a77067e42 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:357
quote: if text.chars().any(is_refused_control) {
at: crates/viola-core/src/lib.rs:367
quote: '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
@row_add n=230 row=decision/never-refuse-line-breaks area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Line breaks and tabs are allowed
rule: LF, CR and TAB are never refused, and a text with content is never refused for holding them.
why: Multi-line relays, plans and answers are normal.
@row_check n=230 row=decision/never-refuse-line-breaks sha=6d6375b6839b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:366
quote: '\n' | '\r' | '\t' => false,
@row_add n=231 row=decision/empty-text-refused area=security-anti-patterns-never-do-these rec=fct-qtoyvdpi@1
name: Empty typed text is refused
rule: A text whose typed text is empty is refused with empty-text, since there is nothing to type.
@row_check n=231 row=decision/empty-text-refused sha=526c88251769 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:94
quote: return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=232 row=decision/control-check-on-decoded-chars area=security-anti-patterns-never-do-these rec=fct-ghv4dm4o@1
name: Control-character check on decoded characters
rule: The control-character check runs over decoded characters, never raw bytes.
why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
@row_check n=232 row=decision/control-check-on-decoded-chars sha=fc45fea9c576 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:357
quote: if text.chars().any(is_refused_control) {
@row_add n=233 row=decision/reads-capped-at-max-frame area=security-anti-patterns-never-do-these rec=fct-zavgpgbb@1
name: Reads capped at MAX_FRAME
rule: Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
@row_check n=233 row=decision/reads-capped-at-max-frame sha=329bd04b3f5e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:24
quote: io::Read::take(&mut *reader, cap)
at: crates/viola-channel/src/tokio_client.rs:89
quote: .take(MAX_FRAME)
at: src/cmd/hook.rs:178
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
at: src/run/version_gate.rs:75
quote: let _stderr = drain(child.stderr.take(), MAX_FRAME);
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
@row_add n=234 row=decision/over-long-line-ends-session area=security-anti-patterns-never-do-these rec=fct-zavgpgbb@1
name: Over-long line ends the session
rule: An over-long MCP stdin line ends the session; it is never truncated or skipped.
@row_check n=234 row=decision/over-long-line-ends-session sha=0d248cfc7858 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/stdin.rs:60-63
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed);
@row_add n=235 row=decision/no-serde-unbounded-depth area=security-anti-patterns-never-do-these rec=fct-eegmmvld@1
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is never enabled.
guard: deny.toml ban
@row_check n=235 row=decision/no-serde-unbounded-depth sha=abd1cf302c6f v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: deny.toml:55
@row_add n=236 row=decision/closed-types-for-protocol-fields area=security-anti-patterns-never-do-these rec=fct-ncpenfx7@1
name: Closed types for protocol fields
rule: behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
@row_check n=236 row=decision/closed-types-for-protocol-fields sha=d21ea39195a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:17
quote: pub enum DialogKind {
at: crates/viola-state/src/snapshot.rs:23
quote: pub enum Wheel {
at: crates/viola-agent-claude/src/dialog.rs:189
quote: behavior: Permit,
at: crates/viola-state/src/snapshot.rs:78
quote: pub dialog_id: u64,
@row_add n=237 row=decision/agents-row-name-not-a-target area=security-anti-patterns-never-do-these rec=fct-ni56qcoe@1
name: claude agents names are not targets
rule: A claude agents --json row name is never used as a ViolaName or target.
@row_check n=237 row=decision/agents-row-name-not-a-target sha=150f1b46c677 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:3-4
quote: //! Pure functions over bytes: the child is spawned by the root binary. A row's text is content, //! never a command, a path or a `ViolaName`.
searched: ViolaName · in crates/viola-agent-claude/src/agents.rs
searched: "name" · in crates/viola-agent-claude/src/agents.rs
@row_add n=238 row=decision/owner-only-file-modes area=security-anti-patterns-never-do-these rec=fct-fmxdaj54@2
name: Owner-only modes under the viola home
rule: The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask.
except: The pinned versioned viola binary is 0700, and never group- or world-writable.
@row_check n=238 row=decision/owner-only-file-modes sha=7c8abb889ba6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
@row_add n=239 row=decision/statusline-only-from-strict-home area=security-anti-patterns-never-do-these rec=fct-sv5b7ryi@1
name: statusline command only from a strict home
rule: statusline_command is never run from a snapshot whose home or instance directory fails the strict-modes check.
why: Write access there equals code execution.
@row_check n=239 row=decision/statusline-only-from-strict-home sha=d195d2b41916 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:85
quote: check_instance(&instance.home, &instance.dir).is_ok()
@row_add n=240 row=decision/rehash-pinned-binary area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Re-hash a pinned binary before reuse
rule: An existing pinned viola binary is never reused without re-hashing it against the hash in its directory name.
@row_check n=240 row=decision/rehash-pinned-binary sha=1215b28fa4a7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:88-90
quote: match file_key(&path) { Ok(found) if found == hash => {} Ok(_) => return Err(PinError::HashMismatch),
@row_add n=241 row=decision/cryptographic-pin-hash area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Pin hash is cryptographic
rule: The pinned binary's hash is a truncated SHA-256, never a non-cryptographic hash.
@row_check n=241 row=decision/cryptographic-pin-hash sha=6ef443fee511 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:47
quote: hex16(&Sha256::digest(bytes))
@row_add n=242 row=decision/rewrite-plugin-files area=security-anti-patterns-never-do-these rec=fct-gmz2hnmb@2
name: Plugin files always rewritten
rule: Existing plugin files are never reused without rewriting them.
@row_check n=242 row=decision/rewrite-plugin-files sha=bee89fa27e48 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:116
quote: replace_private(&root.join(rel), content.as_bytes(), FILE_MODE)?;
@row_add n=243 row=decision/non-inheritable-channel-handles area=security-anti-patterns-never-do-these rec=fct-iftwwapx@2
name: Channel handles not inheritable
rule: Channel handles are never made inheritable.
@row_check n=243 row=decision/non-inheritable-channel-handles sha=df54c501379d v=holds judge=sonnet head=8834195b0683
searched: set_inheritable|inheritable|HANDLE_FLAG_INHERIT · in **/*.rs, **/*.toml
searched: CLOEXEC · in **/*.rs, **/*.toml
@row_add n=244 row=decision/fixture-hygiene area=security-anti-patterns-never-do-these rec=fct-kj2bdrt2@1
name: Recorded fixture hygiene
rule: Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
except: A workspace.repo naming this repository's own remote is kept.
@row_check n=244 row=decision/fixture-hygiene sha=524f32a7e3f4 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=245 row=decision/no-rmcp-http-or-auth-features area=security-anti-patterns-never-do-these rec=fct-ggmql6i2@1
name: No rmcp HTTP server or auth features
rule: rmcp's transport-streamable-http-server and auth features are never enabled.
@row_check n=245 row=decision/no-rmcp-http-or-auth-features sha=0c1da6f2574d v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
@row_add n=246 row=decision/request-size-limits area=security-anti-patterns-never-do-these rec=fct-kemtlykb@1
name: Request size limits
rule: MAX_FRAME is enforced on the channel and on each viola mcp stdin line; request size limits are never skipped.
@row_check n=246 row=decision/request-size-limits sha=0149e21ca60d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:18
quote: read_frame_within(reader, MAX_FRAME)
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
@row_add n=247 row=decision/no-messaging-secrets-in-outputs area=security-anti-patterns-never-do-these rec=fct-vbcb52sx@1
name: No messaging secrets in any output
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
@row_check n=247 row=decision/no-messaging-secrets-in-outputs sha=38aceca6458f v=holds judge=sonnet head=8834195b0683
searched: CLAUDE_CODE_MESSAGING · in src/**/*.rs and crates/**/*.rs (only the strip list in crates/viola-agent-claude/src/lib.rs and its tests)
@row_add n=248 row=decision/no-secrets-in-git area=security-anti-patterns-never-do-these rec=fct-jo3zjurs@1
name: No secrets or local homes in git
rule: .env files, local --home test directories and signing material are never committed to git.
@row_check n=248 row=decision/no-secrets-in-git sha=c72238744d4a v=holds judge=sonnet head=8834195b0683
searched: .env · in .gitignore (lines 30-31 ignore .env and .env.*)
searched: **/.env, **/.env.*, **/*.pem, **/*.p12, **/*.pfx, **/*.keystore · in the repository (no file)
@row_add n=249 row=decision/no-hardcoded-credentials area=security-anti-patterns-never-do-these rec=fct-rnwhyrn3@1
name: No hardcoded credentials
rule: Credentials are never hardcoded in source or in the include_str! plugin files.
@row_check n=249 row=decision/no-hardcoded-credentials sha=40f1c7548069 v=holds judge=sonnet head=8834195b0683
searched: (password|api_key|secret) = "..." assignment · in src/**/*.rs and crates/**/*.rs (only a test canary in crates/viola-e2e)
searched: include_str! · in src (only ../schemas/config.v1.json)
@row_add n=250 row=decision/no-parser-error-chains-outward area=security-anti-patterns-never-do-these rec=fct-ilzyulw7@1
name: No parser error chains in external output
rule: serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr.
why: their Display quotes input values such as tool input
@row_check n=250 row=decision/no-parser-error-chains-outward sha=33c2b3b0bb2b v=holds judge=sonnet head=8834195b0683
searched: {:#} · in src
searched: serde_path_to_error · in src/**/*.rs and crates/**/*.rs (only crates/viola-agent-claude/src/hook.rs, which keeps path and expected type, never the value)
@row_add n=251 row=decision/hook-silent-and-exit-zero area=security-anti-patterns-never-do-these rec=fct-tnu5bfz7@1
name: viola hook writes no stderr and exits zero
rule: viola hook never writes to stderr and never exits non-zero, including on security-check failures.
@row_check n=251 row=decision/hook-silent-and-exit-zero sha=8bc3061e505e v=holds judge=sonnet head=8834195b0683
searched: eprint|stderr · in src/cmd/hook.rs and src/cmd/hook/ (only comments and a Stdio::null for the child)
searched: (Role::Hook, _) => ExitCode::SUCCESS · in src/main.rs:133
@row_add n=252 row=decision/escape-terminal-controls area=security-anti-patterns-never-do-these rec=fct-llbob32q@1
name: Escape C0/C1 controls in human output
rule: wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls.
why: the same class as RUSTSEC-2025-0055
@row_check n=252 row=decision/escape-terminal-controls sha=5e7c55c69a1f v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:21
quote: out.write_all(escape_message(&line).as_bytes())
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: src/human/escape.rs:23
quote: if c.is_control() {
@row_add n=253 row=decision/tracing-subscriber-version-floor area=security-anti-patterns-never-do-these rec=fct-egib22ly@1
name: tracing-subscriber version floor
rule: tracing-subscriber below 0.3.20 is never used, if obs selects it.
@row_check n=253 row=decision/tracing-subscriber-version-floor sha=d28b3cb10106 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
@row_add n=254 row=decision/private-diagnostics-files area=security-anti-patterns-never-do-these rec=fct-423bwibm@1
name: Private diagnostics files
rule: diagnostics/ files are never created readable by other users.
@row_check n=254 row=decision/private-diagnostics-files sha=1f21af1e915b v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:91
quote: ensure_private_dir(&dir)?;
@row_add n=255 row=decision/no-absolute-paths-in-external-errors area=security-anti-patterns-never-do-these rec=fct-6xxq3ubv@1
name: No absolute paths in external errors
rule: No external error ever exposes an absolute path.
@row_check n=255 row=decision/no-absolute-paths-in-external-errors sha=1dfd53cd062f v=unverifiable judge=sonnet head=8834195b0683
searched: absolute path in external error bodies · in src/**/*.rs, crates/**/*.rs
why: no evidence found
@row_add n=256 row=decision/upstream-text-is-never-a-command area=security-anti-patterns-never-do-these rec=fct-4py7x3cl@1
name: Upstream text is never a command (R1)
rule: Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
@row_check n=256 row=decision/upstream-text-is-never-a-command sha=f7d0a10bbc03 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
@row_add n=257 row=decision/compiled-classification-sources area=security-anti-patterns-never-do-these rec=fct-xmaxdng7@1
name: Classification sources are compiled only
rule: Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
@row_check n=257 row=decision/compiled-classification-sources sha=877c3195a8dd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:110
quote: const HARNESS_PREFIXES: [&str; 4] = [
@row_add n=258 row=decision/harness-prefix-literals area=security-anti-patterns-never-do-these rec=fct-xmaxdng7@1
name: Harness prefixes as four raw-start literals
rule: A prompt is classified harness only by four compiled literals matched on its raw start with no trim: <agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message.
why: ratified side effect: the escaped cross-session form is the one escaped tag that classifies harness, so a human who types it at a prompt's start is filed harness
@row_check n=258 row=decision/harness-prefix-literals sha=57f9762639bd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:110-115
quote: const HARNESS_PREFIXES: [&str; 4] = [ "<agent-message from=", "<task-notification>", "<\\cross-session-message", "<cross-session-message", ];
@row_add n=259 row=decision/no-interprocess-try-overwrite area=security-anti-patterns-never-do-these rec=fct-3nboftz7@1,fct-2pxk6g7w@2
name: interprocess try_overwrite stays off
rule: interprocess try_overwrite is never enabled on the socket bind.
why: TOCTOU deletion of an attacker-swapped file
@row_check n=259 row=decision/no-interprocess-try-overwrite sha=09dc68a28f19 v=holds judge=sonnet head=8834195b0683
searched: try_overwrite(true · in src/**/*.rs, crates/**/*.rs (only a doc comment at crates/viola-channel/src/server.rs:98 names it, "stays off")
searched: try_overwrite · in crates/viola-channel/src/server.rs
@row_add n=260 row=decision/vt100-inside-catch-unwind area=security-anti-patterns-never-do-these rec=fct-iwuq6h35@1
name: vt100 only inside catch_unwind
rule: vt100 is never fed outside catch_unwind.
why: a parser panic must not take down the human's terminal path
@row_check n=260 row=decision/vt100-inside-catch-unwind sha=0110fe4a7b9b v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
@row_add n=261 row=decision/e2e-home-removal-own-dirs-only area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: Test-side removal only of its own dirs
rule: A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
@row_check n=261 row=decision/e2e-home-removal-own-dirs-only sha=a8e19492f28b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:162
quote: if !is_session_dir || parent.parent() != Some(ws.e2e_home().as_path()) {
@row_add n=262 row=decision/e2e-home-keepers-linked-base area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: e2e-home keepers follow only a safe linked base
rule: The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
@row_check n=262 row=decision/e2e-home-keepers-linked-base sha=9c623eb18fd7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:108
quote: if !target.is_absolute() {
at: crates/viola-e2e/src/harness/mod.rs:120
quote: if meta.permissions().mode() & 0o077 != 0 {
@row_add n=263 row=decision/viola-ignores-e2e-home-backing area=security-anti-patterns-never-do-these rec=fct-4fkoyw3x@1
name: viola never touches the e2e-home backing
rule: The viola binary itself never reads or writes the target/e2e-home backing by that name.
@row_check n=263 row=decision/viola-ignores-e2e-home-backing sha=e8ff2ae939c3 v=holds judge=sonnet head=8834195b0683
searched: e2e-home · in src/**/*.rs (only a test doc comment at src/cmd/run.rs:1612)
searched: e2e-home · in crates/viola-*/src (non-e2e crates)
@row_add n=264 row=decision/new-listener-needs-log-entry area=security-anti-patterns-never-do-these rec=fct-yfqtfw75@1
name: New listener or channel method needs a log entry
rule: No listener (TCP port, WebSocket, http hooks, MCP HTTP transport) or new channel method is added without a Security Decisions Log entry mapping it to the security plan's IPC controls.
@row_check n=264 row=decision/new-listener-needs-log-entry sha=a15db857baa3 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=265 row=decision/refusals-never-block-human area=security-anti-patterns-never-do-these rec=fct-ciublye4@1
name: Security refusals never block the human
rule: Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
@row_check n=265 row=decision/refusals-never-block-human sha=48006ce20476 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=266 row=decision/absolute-viola-path-in-commands area=security-anti-patterns-never-do-these rec=fct-enn2bam2@2
name: Hook, MCP and statusline commands use an absolute viola path
rule: Every exec-form command in the embedded hooks.json or .mcp.json, in plugin install's .mcp.json, and the statusline command written into instances/<name>/settings.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH.
why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
@row_check n=266 row=decision/absolute-viola-path-in-commands sha=709374d04ba9 v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: src/cmd/plugin.rs:108
quote: let exe = super::canonical_home(&std::env::current_exe()?);
at: src/cmd/run.rs:564
quote: let Some(document) = statusline::override_document(&pinned.path_fwd) else {
@row_add n=267 row=decision/snapshot-written-only-by-wrapper area=security-anti-patterns-never-do-these rec=fct-pmvbeqyd@1
name: Only the instance's wrapper writes its snapshot
rule: No process other than the instance's own wrapper writes instances/<name>/snapshot.json.
why: endpoint, pinned_bin and statusline_command in it decide where clients connect and what hook statusline executes
@row_check n=267 row=decision/snapshot-written-only-by-wrapper sha=650c6c203874 v=holds judge=sonnet head=8834195b0683
at: src/run/snapshot.rs:31
quote: write_snapshot(&self.instance_dir, &snapshot)?;
@row_add n=268 row=decision/controls-not-switchable area=security-anti-patterns-never-do-these rec=fct-7ackeyph@2
name: No config, env var or flag switches off a control
rule: config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text).
why: environment variables are not a configuration channel
except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
@row_check n=268 row=decision/controls-not-switchable sha=a5d2df84cd04 v=holds judge=sonnet head=8834195b0683
searched: std::env::var|env::var_os|env::var\( · in src/** and crates/*/src/** (only VIOLA_NAME/VIOLA_DIR path reads, PATH, NO_COLOR/TERM colour, FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC test seams; none gates check_endpoint_dir, peer_admitted, verify_server, strict checks or validate_paste_text)
searched: VIOLA_ · in src/cmd/hook.rs, src/cmd/mod.rs, src/cmd/client.rs (name/home resolution only, no control switched off)
@row_add n=269 row=decision/new-env-read-needs-log-entry area=security-anti-patterns-never-do-these rec=fct-7ackeyph@2
name: New seam or environment read needs a log entry
rule: No further test-only seam or environment read is added without its own Decisions Log entry.
@row_check n=269 row=decision/new-env-read-needs-log-entry sha=204a3a478eec v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=270 row=absence/accounts-and-passwords area=authentication-authorization rec=fct-riad4wv3@1
capability: User accounts and passwords
@row_check n=270 row=absence/accounts-and-passwords sha=6dcd82411b8f v=holds judge=sonnet head=8834195b0683
searched: password|passwd|argon|bcrypt|login (case-insensitive) · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml (only the e2e harness reads the OS passwd entry for the home directory; no account or password code)
@row_add n=271 row=decision/all-or-nothing-channel-access area=authentication-authorization rec=fct-4riic6pc@1
name: Capability-based, all-or-nothing channel access
rule: Any caller that passes the OS check gets the full channel surface, each method behind the IPC controls and its own params check; there are no roles.
why: the only principal is the local OS user
@row_check n=271 row=decision/all-or-nothing-channel-access sha=06b840463c18 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:140
quote: "send" => send::send(&self.send, &self.name, &self.instance_dir, call),
at: src/cmd/run.rs:143
quote: "hook.dialog" => self.dialogs.hook_dialog(call.params),
at: src/cmd/run.rs:146
quote: "release" => self.wheel.release(call, &self.budget),
at: src/cmd/run.rs:120
quote: if method != "hook.event" {
@row_add n=272 row=decision/per-user-socket-directory area=authentication-authorization rec=fct-2pxk6g7w@2
name: Socket in a per-user 0700 directory
rule: The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.
@row_check n=272 row=decision/per-user-socket-directory sha=2144b25aa072 v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-channel/src/endpoint.rs:39
quote: Some(xdg) => Path::new(xdg).join("viola"),
at: crates/viola-channel/src/endpoint.rs:40
quote: None => PathBuf::from(format!("{host path}{uid}")),
at: crates/viola-channel/src/endpoint.rs:52
quote: .join(format!("{base}.sock"))
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
@row_add n=273 row=decision/socket-directory-check area=authentication-authorization rec=fct-2pxk6g7w@2,fct-ezmg3kua@2
name: Socket-directory check before bind and connect
rule: Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink.
guard: security_negatives_socket_dir
@row_check n=273 row=decision/socket-directory-check sha=f5a8d9d65a16 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:139
quote: let meta = match std::fs::symlink_metadata(dir) {
at: crates/viola-state/src/strict.rs:174
quote: unix_read_verdict(mode, uid, euid)
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
guard_at: tests/security_negatives_socket_dir.rs:1
@row_add n=274 row=contract/socket-dir-refusal/run-and-revive-start area=authentication-authorization rec=fct-2pxk6g7w@2
boundary: socket-directory refusal
item: run and the start of revive
promise: A failed create and a refused check read one code: exit 1 with socket-dir-refused and one process-exit line of that detail; a Server::bind failure is outside this code.
@row_check n=274 row=contract/socket-dir-refusal/run-and-revive-start sha=449b0400bb58 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:498
quote: return Ok(Bound::SocketDirRefused);
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412
quote: run::log_self_exit(1, Some(detail));
@row_add n=275 row=contract/socket-dir-refusal/cli-json area=authentication-authorization rec=fct-2pxk6g7w@2,fct-ybwsdbo3@1
boundary: socket-directory refusal
item: CLI frame verb under --json
promise: No frame is written and the verb exits 21 with {"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"} on stdout and stderr empty.
@row_check n=275 row=contract/socket-dir-refusal/cli-json sha=8b5d2d290bc2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:174
quote: return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:265
quote: 21
@row_add n=276 row=contract/socket-dir-refusal/cli-human area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: socket-directory refusal
item: CLI frame verb in human mode
promise: No frame is written and the verb exits 21 with the unable <name> instance-unreachable line and the hint of cause socket-dir last on stderr.
@row_check n=276 row=contract/socket-dir-refusal/cli-human sha=230336e27b0e v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:174
quote: return Err(CallError::SocketDirRefused);
at: src/human/hints.rs:59
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
at: src/cmd/client.rs:265
quote: 21
@row_add n=277 row=contract/socket-dir-refusal/mcp area=authentication-authorization rec=fct-2pxk6g7w@2,fct-6ntn52d2@1
boundary: socket-directory refusal
item: viola mcp tool
promise: The instance-unreachable result with detail socket-dir-refused, no frame and no line, isError the only record.
@row_check n=277 row=contract/socket-dir-refusal/mcp sha=0869a08d4e11 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=278 row=contract/socket-dir-refusal/hook area=authentication-authorization rec=fct-2pxk6g7w@2
boundary: socket-directory refusal
item: viola hook
promise: Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
@row_check n=278 row=contract/socket-dir-refusal/hook sha=3c15def682f8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:232
quote: return Err(NotSent::SocketDir);
at: src/cmd/hook.rs:215
quote: Self::SocketDir => UnreachableCause::SocketDir.as_str(),
at: src/cmd/hook.rs:390
quote: WARN,
@row_add n=279 row=decision/socket-file-chmod-after-bind area=authentication-authorization rec=fct-2pxk6g7w@2
name: Socket file chmod 0600 after bind
rule: The socket file is chmod 0600 right after the bind; setting the mode through listener options is not used, chmod after the bind being the one form.
@row_check n=279 row=decision/socket-file-chmod-after-bind sha=cb9fcef18c55 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-channel/src/server.rs:123
quote: let listener = ListenerOptions::new()
@row_add n=280 row=decision/socket-dir-check-never-chmods area=authentication-authorization rec=fct-a2nznb7h@1
name: Socket-directory check never chmods
rule: The socket-directory check never changes a directory's mode: a refused directory stays as found.
guard: security_negatives_socket_dir
@row_check n=280 row=decision/socket-dir-check-never-chmods sha=245db280a41d v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:139
quote: let meta = match std::fs::symlink_metadata(dir) {
at: crates/viola-state/src/strict.rs:144-150
quote: socket_dir_verdict( meta.file_type().is_symlink(), meta.is_dir(), meta.mode(), meta.uid(), unix::euid(), )
guard_at: tests/security_negatives_socket_dir.rs:1
@row_add n=281 row=decision/leftover-socket-file-kept area=authentication-authorization rec=fct-sqeb75n5@1
name: Leftover socket file is not removed
rule: A killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: by choice: a harmless file in a 0700 directory
@row_check n=281 row=decision/leftover-socket-file-kept sha=cb4f739641db v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:118
quote: let _ = fs::remove_file(&socket);
at: crates/viola-channel/src/server.rs:93
quote: let _ = std::fs::remove_file(&self.socket);
@row_add n=282 row=decision/server-side-peer-euid-check area=authentication-authorization rec=fct-ekk5s2bu@1
name: Server drops a foreign-euid peer
rule: On every accept the server reads the peer's credentials and drops the connection before reading a frame if the peer's euid is not its own.
@row_check n=282 row=decision/server-side-peer-euid-check sha=afa50b3130ff v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: crates/viola-channel/src/client.rs:156
quote: stream.peer_creds().ok()?.euid()
@row_add n=283 row=decision/client-verifies-server-before-frame area=authentication-authorization rec=fct-ju3c6nzj@2
name: Client verifies the server before any frame
rule: Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal.
why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
@row_check n=283 row=decision/client-verifies-server-before-frame sha=bf2d831fb3bd v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:570
quote: .notify("hook.event", params)
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
at: crates/viola-state/src/liveness.rs:141
quote: if observed_pid != recorded_pid {
@row_add n=284 row=decision/strict-modes-before-snapshot-read area=authentication-authorization rec=fct-ju3c6nzj@2,fct-65crhg3m@2
name: Strict-modes check before the snapshot is read
rule: Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification.
why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
@row_check n=284 row=decision/strict-modes-before-snapshot-read sha=cd28a8d9e22c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
@row_add n=285 row=contract/server-verify-refusal/cli-exit area=authentication-authorization rec=fct-ju3c6nzj@2,fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb exit and line
promise: No frame is written; the verb exits 21 instance-unreachable and writes one process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"} WARN line.
@row_check n=285 row=contract/server-verify-refusal/cli-exit sha=cbe39b71c21f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:178
quote: return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=286 row=contract/server-verify-refusal/mcp area=authentication-authorization rec=fct-ju3c6nzj@2,fct-6ntn52d2@1
boundary: server-verification refusal
item: viola mcp tool
promise: A refused peer euid or failed server check returns instance-unreachable with detail server-verify-failed, no frame and no line, isError the only record.
@row_check n=286 row=contract/server-verify-refusal/mcp sha=a8077dce79f7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=287 row=contract/server-verify-refusal/hook area=authentication-authorization rec=fct-ju3c6nzj@2
boundary: server-verification refusal
item: viola hook
promise: Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
@row_check n=287 row=contract/server-verify-refusal/hook sha=a962678673ca v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:237
quote: return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
@row_add n=288 row=decision/no-identity-values-in-refusals area=authentication-authorization rec=fct-ju3c6nzj@2
name: No identity values in verification output
rule: No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
@row_check n=288 row=decision/no-identity-values-in-refusals sha=b972998295ce v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=289 row=contract/strict-modes-refusal/cli-frame-verbs area=authentication-authorization rec=fct-ju3c6nzj@2,fct-ybwsdbo3@1
boundary: strict-modes refusal
item: CLI frame verb
promise: Before liveness and server checks: no frame and no line, nothing opened under the home; exit 21 with the instance-state unable:/hint: pair, or under --json the instance-unreachable document with detail strict-modes-failed.
@row_check n=289 row=contract/strict-modes-refusal/cli-frame-verbs sha=d4bbac60cdcb v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
@row_add n=290 row=contract/strict-modes-refusal/hook area=authentication-authorization rec=fct-ju3c6nzj@2,fct-65crhg3m@2
boundary: strict-modes refusal
item: viola hook
promise: Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
@row_check n=290 row=contract/strict-modes-refusal/hook sha=6c16f033a224 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=291 row=contract/strict-modes-refusal/mcp-tool area=authentication-authorization rec=fct-ju3c6nzj@2,fct-6ntn52d2@1
boundary: strict-modes refusal
item: viola mcp tool
promise: Returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_check n=291 row=contract/strict-modes-refusal/mcp-tool sha=c9a51f6a3107 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=292 row=decision/collision-check-needs-no-server-verify area=authentication-authorization rec=fct-ju3c6nzj@2
name: run's collision check is not server-verified
rule: run's collision check judges a recorded wrapper by its pid, OS start time and heartbeat (a replayed snapshot by the beat alone) and opens no connection, so the peer-pid server check does not apply to it.
why: it writes no frame, so there is no serving process to verify
@row_check n=292 row=decision/collision-check-needs-no-server-verify sha=809cd313f52f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:440
quote: Some(Recovered::Snapshot(snapshot)) => Some(classify(age(), same_process(snapshot))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=293 row=decision/client-seam-check-order area=authentication-authorization rec=fct-kbl7frdn@1
name: Check order at every client seam
rule: On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
@row_check n=293 row=decision/client-seam-check-order sha=bebeae3e8f3d v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:176
quote: let client = Client::connect(&live.endpoint, "cli")?;
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/hook.rs:234-235
quote: let client = Client::connect_by(endpoint, "hook", connect_by).map_err(|_| NotSent::Unreachable)?;
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:239
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:345
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: crates/viola-mcp/src/tools.rs:348
quote: let mut client = match TokioClient::connect(&live.endpoint, "mcp").await {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
@row_add n=294 row=decision/release-from-driver-guard area=authentication-authorization rec=fct-sjhqtetq@1
name: Driver-originated release is refused
rule: The wrapper refuses a release whose params carry a string from, before budget is acted on.
why: an affordance-level guard against drivers handing the wheel back to automation, not enforcement: a same-user process can leave out from
except: pause, link and unlink from drivers are allowed
@row_check n=294 row=decision/release-from-driver-guard sha=6aa56d5edfe8 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: src/run/wheel.rs:209
quote: let budget = match params.get("budget") {
@row_add n=295 row=contract/channel-release/string-from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: string from
promise: Refused with -32602 "invalid params" and data {"reason":"release-from-driver"}, one release-from-driver line logged (from_trust:"self-reported"); a budget pause is not lifted and the snapshot's budget fields stay as they were.
@row_check n=295 row=contract/channel-release/string-from sha=45e5dedf44d3 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:96
quote: Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
at: src/run/wheel.rs:244
quote: from_trust = "self-reported",
@row_add n=296 row=contract/channel-release/non-string-from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: from of another type
promise: Refused with -32602 and data null.
@row_check n=296 row=contract/channel-release/non-string-from sha=ca5148c20ae6 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:208
quote: parse_from(params)?;
at: src/run/send/params.rs:20
quote: Some(_) => Err(ProtocolError::InvalidParams),
@row_add n=297 row=contract/channel-release/params area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel release method
item: params
promise: release params are {budget?, from?}; a release on a wheel the driver already holds changes nothing.
@row_check n=297 row=contract/channel-release/params sha=518a8adb618d v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:46
quote: (Input::Human | Input::Release, _, _) => None,
at: src/run/wheel.rs:209
quote: let budget = match params.get("budget") {
at: src/run/wheel.rs:211
quote: Some(budget) => budget.as_bool().ok_or(ProtocolError::InvalidParams)?,
@row_add n=298 row=contract/cli-release/from area=authentication-authorization rec=fct-sjhqtetq@1
boundary: CLI viola release
item: from
promise: from is sent from VIOLA_NAME, so viola release run from a driver session exits 20.
@row_check n=298 row=contract/cli-release/from sha=bae78c6b0d65 v=holds judge=sonnet head=8834195b0683
at: src/cmd/pause.rs:70-71
quote: if let Some(from) = own_name() { params.insert("from".to_owned(), from.as_ref().into());
at: src/cmd/pause.rs:75
quote: Ok(Reply::Fault(detail)) => return Ok(client::fault(json, &detail)),
@row_add n=299 row=contract/channel-response-line/release-from-driver area=authentication-authorization rec=fct-sjhqtetq@1
boundary: channel-response log line
item: release-from-driver
promise: The server's and client's line is WARN with detail release-from-driver and no param name or value; a from of another type reads invalid-params; the line adds nothing to the wire body.
@row_check n=299 row=contract/channel-response-line/release-from-driver sha=0f9505a4ccdf v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/server.rs:496
quote: response_line!(WARN)
@row_add n=300 row=decision/private-file-modes area=authentication-authorization rec=fct-65crhg3m@2
name: Owner-only modes for the viola home
rule: The home (and any --home) is created 0700 and every file 0600, never group- or world-writable.
except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
@row_check n=300 row=decision/private-file-modes sha=9e52ce5ac5ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10-11
quote: pub const DIR_MODE: u32 = 0o700; pub const FILE_MODE: u32 = 0o600;
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
at: crates/viola-state/src/fs.rs:115
quote: restrict(tmp.path(), mode)?;
@row_add n=301 row=decision/explicit-mode-before-write area=authentication-authorization rec=fct-65crhg3m@2
name: Mode set explicitly before any byte
rule: A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded.
@row_check n=301 row=decision/explicit-mode-before-write sha=6140e46c2379 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:80-81
quote: restrict(tmp.path(), mode)?; tmp.write_all(bytes)?;
@row_add n=302 row=decision/one-replace-helper area=authentication-authorization rec=fct-65crhg3m@2
name: One shared helper replaces files
rule: Every replaced file goes through one shared helper that writes a private temp file in the same directory, syncs it and persists it into place.
@row_check n=302 row=decision/one-replace-helper sha=f30d262e8d18 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:79-83
quote: let mut tmp = tempfile::NamedTempFile::new_in(parent)?; restrict(tmp.path(), mode)?; tmp.write_all(bytes)?; tmp.as_file().sync_all()?; tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
@row_add n=303 row=decision/pinned-viola-never-replaced area=authentication-authorization rec=fct-65crhg3m@2
name: Pinned viola is never replaced
rule: The pinned viola is never replaced: an absent copy is written to a 0700 temp file, synced, closed and placed without clobbering, never over a present name.
@row_check n=303 row=decision/pinned-viola-never-replaced sha=cd2469224165 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
at: crates/viola-state/src/fs.rs:117-118
quote: tmp.as_file().sync_all()?; match tmp.into_temp_path().persist_noclobber(path) {
@row_add n=304 row=decision/strict-modes-read-rule area=authentication-authorization rec=fct-65crhg3m@2
name: Strict-modes read rule
rule: The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission.
why: so a pre-existing home cannot expose events.ndjson or diagnostics/
@row_check n=304 row=decision/strict-modes-read-rule sha=0ed34a0bcd7b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:229
quote: if mode & 0o077 != 0 {
at: crates/viola-state/src/strict.rs:63
quote: (home.to_path_buf(), Rule::Read),
at: crates/viola-state/src/strict.rs:65
quote: (instance_dir.to_path_buf(), Rule::Read),
at: crates/viola-state/src/strict.rs:66
quote: (instance_dir.join(DIAGNOSTICS), Rule::Read),
@row_add n=305 row=decision/strict-refusal-leaves-no-trace area=authentication-authorization rec=fct-65crhg3m@2
name: A strict-modes refusal leaves no trace
rule: A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens.
why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
@row_check n=305 row=decision/strict-refusal-leaves-no-trace sha=cfd0a4b07810 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/list.rs:43
quote: let scan = match board::scan(&home)? {
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
@row_add n=306 row=decision/unstattable-home-refused area=authentication-authorization rec=fct-65crhg3m@2
name: An unstattable home is refused
rule: An unstattable home (a path under a regular file) is a strict-modes refusal.
@row_check n=306 row=decision/unstattable-home-refused sha=c4f597c687c9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:184
quote: Err(_) => return Err(Refused::Unreadable),
@row_add n=307 row=decision/whole-board-refusal area=authentication-authorization rec=fct-65crhg3m@2
name: viola list refuses the whole board
rule: viola list runs the strict-modes checks on the home, instances/ and each instance before opening anything under the home, and the first failure refuses the whole board.
why: ratified
@row_check n=307 row=decision/whole-board-refusal sha=1768343fb80c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59
quote: if check_if_present(home, check_read).is_err()
at: crates/viola-state/src/board.rs:97
quote: if check_instance(home, dir).is_err() {
at: crates/viola-state/src/board.rs:98
quote: return Ok(Err(ScanRefused::Instance(name.clone())));
at: src/cmd/list.rs:45-47
quote: Err(refused) => { refuse(args.json, &refused); return Ok(ExitCode::from(21));
@row_add n=308 row=decision/board-home-read-rule area=authentication-authorization rec=fct-pq4mk5dy@3,fct-65crhg3m@2
name: The board holds the home to the read rule
rule: viola list refuses a home another user may read, with or without an instance.
@row_check n=308 row=decision/board-home-read-rule sha=c3855c977c3e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59
quote: if check_if_present(home, check_read).is_err()
@row_add n=309 row=decision/home-write-rule-checkers area=authentication-authorization rec=fct-pq4mk5dy@3
name: Budget read, mcp start and plugin install check the home
rule: The budget read, the start of viola mcp and viola plugin install hold the home to the write rule.
@row_check n=309 row=decision/home-write-rule-checkers sha=10724d6b29f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:94
quote: check_path(home).ok()?;
at: src/cmd/mcp.rs:46
quote: Ok(_) => viola_state::strict::check_path(path).is_ok(),
at: src/cmd/plugin.rs:85
quote: Ok(_) => viola_state::strict::check_path(path).is_ok(),
@row_add n=310 row=contract/strict-modes-refusal/run-revive area=authentication-authorization rec=fct-65crhg3m@2,fct-mhqlyrp7@2
boundary: strict-modes refusal
item: run and revive
promise: Exit 1 with no child, no log and no line, the home or instance unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"strict-modes-failed","detail":null} on stdout with stderr empty.
@row_check n=310 row=contract/strict-modes-refusal/run-revive sha=ef8aa6183747 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:172
quote: refuse_tree(&args.name, refused);
at: src/cmd/run.rs:173
quote: return Ok(ExitCode::from(1));
at: src/cmd/revive.rs:137-139
quote: if !trusted(home, &args.name)? { Refusal::StrictModes.write(&args.name, args.json); return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=311 row=contract/strict-modes-refusal/list-json area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola list under --json
promise: Exit 21 with exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout and stderr empty.
@row_check n=311 row=contract/strict-modes-refusal/list-json sha=281e8cd3de73 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:94
quote: json!({"v": 1, "error": "instance-unreachable", "detail": cause})
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
@row_add n=312 row=contract/strict-modes-refusal/list-human area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola list in human form
promise: Exit 21 with the instance-state or home-state unable:/hint: pair on stderr and stdout empty.
@row_check n=312 row=contract/strict-modes-refusal/list-human sha=a9d478b12537 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:105
quote: let (unable, hint) = human::home_state_pair();
at: src/cmd/list.rs:106
quote: human::refuse(unable, hint);
at: src/cmd/list.rs:109
quote: let (unable, hint) = human::instance_state_pair(name.as_ref());
@row_add n=313 row=contract/strict-modes-refusal/link-unlink area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola link and viola unlink
promise: Exit 21 with the instance-state unable:/hint: pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout; no log, no line, the mode as found.
@row_check n=313 row=contract/strict-modes-refusal/link-unlink sha=7a540480cc23 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
@row_add n=314 row=contract/strict-modes-refusal/mcp-start area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola mcp start
promise: Exit 1 with nothing on stdout or stderr and no role file.
@row_check n=314 row=contract/strict-modes-refusal/mcp-start sha=3c06bbd4b157 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:22-23
quote: if !home_passes(home) { return Ok(ExitCode::from(1));
@row_add n=315 row=contract/strict-modes-refusal/plugin-install area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola plugin install
promise: Exit 21 (instance-unreachable under --json) with nothing created and no log opened.
@row_check n=315 row=contract/strict-modes-refusal/plugin-install sha=a88f4a850fe5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:53
quote: if !tree_passes(&home) {
at: src/cmd/plugin.rs:55
quote: client::unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
@row_add n=316 row=contract/strict-modes-refusal/verify area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola verify
promise: Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.
@row_check n=316 row=contract/strict-modes-refusal/verify sha=f60229c6106b v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-68
quote: if check_home_tree(&home, None, None).is_err() { refuse_home();
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=317 row=contract/strict-modes-refusal/version-gate area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal
item: viola run's version gate
promise: A refused stamps read is cli_verified:false with parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}.
@row_check n=317 row=contract/strict-modes-refusal/version-gate sha=eaaf46b6a75c v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
at: src/run/version_gate.rs:177
quote: parser = "ledger-stamps",
@row_add n=318 row=contract/strict-refusal-message/readable area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal message
item: Readable
promise: The read rule's refusal reads exactly "a trusted state path is readable by another user".
@row_check n=318 row=contract/strict-refusal-message/readable sha=d6aa8cd7d132 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:24
quote: #[error("a trusted state path is readable by another user")]
@row_add n=319 row=contract/strict-refusal-message/symlink area=authentication-authorization rec=fct-65crhg3m@2
boundary: strict-modes refusal message
item: Symlink
promise: A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".
@row_check n=319 row=contract/strict-refusal-message/symlink sha=8f1cfba08b45 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:26
quote: #[error("a trusted state path is a symbolic link")]
@row_add n=320 row=decision/statusline-sources-untrusted-read area=authentication-authorization rec=fct-65crhg3m@2
name: Statusline sources read untrusted
rule: statusline-source.json and the user's ~/.claude/settings.json are not trusted files; their control is a read-only, capped, tolerant read.
why: a boundary widening that was ratified
@row_check n=320 row=decision/statusline-sources-untrusted-read sha=71f0e009de6a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:552-556
quote: File::open(source) .ok()? .take(MAX_FRAME) .read_to_end(&mut bytes) .ok()?;
@row_add n=321 row=decision/budget-gate-fails-open area=authentication-authorization rec=fct-65crhg3m@2
name: Budget gate reads an untrusted file, failing open
rule: The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file; a refused or unreadable file is no reading and the gate does not pause.
why: a ratified boundary widening: a file a hook arm writes decides a wrapper's refusal
@row_check n=321 row=decision/budget-gate-fails-open sha=7b5b508e4e80 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:94-95
quote: check_path(home).ok()?; check_path(&path).ok()?;
at: src/run/budget.rs:179
quote: let paused = reading.and_then(|reading| {
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
@row_add n=322 row=contract/server-verify-refusal/cli-json area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb under --json
promise: No frame; exit 21 with {"v":1,"error":"instance-unreachable","detail":"server-verify-failed"} on stdout and stderr empty; a refused peer euid reads the same.
@row_check n=322 row=contract/server-verify-refusal/cli-json sha=033a785f13e9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
@row_add n=323 row=contract/server-verify-refusal/cli-human area=authentication-authorization rec=fct-ybwsdbo3@1
boundary: server-verification refusal
item: CLI frame verb in human mode
promise: No frame; exit 21 with the unable <name> instance-unreachable line and the hint "the process on <name>'s endpoint is not its recorded wrapper; it was not contacted" last on stderr.
@row_check n=323 row=contract/server-verify-refusal/cli-human sha=b47f0291c72d v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:57
quote: "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted"
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
@row_add n=324 row=decision/no-seam-for-foreign-peer area=authentication-authorization rec=fct-efyzkncd@1
name: No seam to drive a foreign peer
rule: A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them.
why: one account can make no foreign peer
@row_check n=324 row=decision/no-seam-for-foreign-peer sha=d2b974142271 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:139
quote: pub fn peer_euid_admitted(observed: Option<u32>, own: u32) -> bool {
at: crates/viola-state/src/strict.rs:215
quote: pub fn unix_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {
@row_add n=325 row=decision/checks-at-seams-not-channel-client area=authentication-authorization rec=fct-2yxn5okb@1
name: Security checks sit at the seams, not the channel client
rule: No check sits inside the channel crate's client connect; the checks sit at the seams, and a test may write through that client past every check by design.
why: viola-channel has no viola-state edge
@row_check n=325 row=decision/checks-at-seams-not-channel-client sha=362f024589b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:33
quote: Self::connect_by(endpoint, process, busy_deadline(Instant::now()))
at: src/cmd/client.rs:176
quote: let client = Client::connect(&live.endpoint, "cli")?;
@row_add n=326 row=decision/no-auth-library-local-boundary area=bootstrap-phases rec=fct-sawbqqxr@2
name: Authentication narrowed to the local boundary
rule: Authentication is narrowed to the local boundary, and no auth library is taken.
@row_check n=326 row=decision/no-auth-library-local-boundary sha=ee40a12b38e3 v=holds judge=sonnet head=8834195b0683
searched: jsonwebtoken|oauth|openidconnect|argon|bcrypt|password-hash|casbin|tower-sessions|axum-login|keyring (line-start, case-insensitive) · in **/Cargo.toml
@row_add n=327 row=decision/socket-owner-only-hardening area=bootstrap-phases rec=fct-sawbqqxr@2
name: Owner-only channel socket
rule: The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
@row_check n=327 row=decision/socket-owner-only-hardening sha=6a582871734c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=328 row=decision/client-checks-before-frame area=bootstrap-phases rec=fct-sawbqqxr@2,fct-tukgmehf@1
name: Client checks before any frame
rule: Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).
@row_check n=328 row=decision/client-checks-before-frame sha=e460bcab69c8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-mcp/src/tools.rs:342
quote: let Some(live) = live_endpoint(&instance_dir) else {
at: crates/viola-mcp/src/tools.rs:345
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:85
quote: check_instance(&instance.home, &instance.dir).is_ok()
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
@row_add n=329 row=decision/plaintext-state-os-permissions area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Plaintext state protected by OS permissions
rule: Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files.
except: The pinned bin/<version>-<hash>/viola copy is 0700.
@row_check n=329 row=decision/plaintext-state-os-permissions sha=d1623b4973a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
@row_add n=330 row=decision/code-bearing-files-rewritten-each-start area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Code-bearing files rewritten at every start
rule: The plugin files and the instance settings.json override are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.
why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
@row_check n=330 row=decision/code-bearing-files-rewritten-each-start sha=c7ecdccb171b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:478
quote: replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
at: crates/viola-state/src/fs.rs:80-81
quote: restrict(tmp.path(), mode)?; tmp.write_all(bytes)?;
@row_add n=331 row=decision/statusline-override-plain-path-only area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Statusline override only for a plain pinned path
rule: The settings.json statusline override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings flag is passed.
why: A shell runs the command string unquoted.
@row_check n=331 row=decision/statusline-override-plain-path-only sha=0096a447a56d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
at: crates/viola-agent-claude/src/statusline.rs:103
quote: c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':')
at: src/run/mod.rs:132-133
quote: if let Some(settings) = settings { args.push(OsString::from(SETTINGS_FLAG));
@row_add n=332 row=decision/pinned-copy-content-hash area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Pinned copy keyed and checked by SHA-256
rule: The pinned copy's <hash> is the first 16 hex characters of the SHA-256 of the exe bytes; before reuse the copy is re-hashed and a mismatch refuses the start with exit 1.
why: Being cryptographic, the hash catches substitution as well as accidental corruption.
guard: contract_content_hash
@row_check n=332 row=decision/pinned-copy-content-hash sha=7a2a7d4ab946 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:47
quote: hex16(&Sha256::digest(bytes))
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
at: src/cmd/run.rs:282-284
quote: return Ok(refused_with( launch.json, "pinned-hash-mismatch",
guard_at: tests/contract_content_hash.rs:24
@row_add n=333 row=decision/sha2-content-hash area=data-protection rec=fct-zvqsbdzq@1,fct-sawbqqxr@2
name: sha2 is the SHA-256 implementation
rule: The SHA-256 implementation is the sha2 crate with default features off.
why: Pure Rust, no cc on the one target triple, so it passes the cargo-deny C-build ban.
guard: contract_content_hash
@row_check n=333 row=decision/sha2-content-hash sha=ccbdf4b7d017 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:232
quote: sha2 = { version = "=0.11.0", default-features = false }
guard_at: tests/contract_content_hash.rs:24
@row_add n=334 row=decision/no-non-cryptographic-hash-fallback area=data-protection rec=fct-zvqsbdzq@1
name: No non-cryptographic hash fallback
rule: A non-cryptographic fallback for the pinned copy's content hash is banned.
@row_check n=334 row=decision/no-non-cryptographic-hash-fallback sha=ae48595d8725 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:47
quote: hex16(&Sha256::digest(bytes))
searched: DefaultHasher|SipHasher|crc32|adler|xxhash|ahash · in src/**/*.rs, crates/**/*.rs, crates/**/*.toml (no hit)
searched: fnv · in src, crates (only crates/viola-channel/src/endpoint.rs:19, the socket-name hash, not the pinned copy's content hash)
@row_add n=335 row=decision/same-user-tampering-out-of-scope area=data-protection rec=fct-zvqsbdzq@1
name: Same-user tampering outside the boundary
rule: Tampering with the pinned copy by the same OS user stays outside the v1 security boundary.
why: The same user could also rewrite the folder name.
@row_check n=335 row=decision/same-user-tampering-out-of-scope sha=3dcfc60eccbe v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=336 row=decision/marketplace-written-only-by-install area=data-protection rec=fct-zvqsbdzq@1
name: Marketplace tree written only by plugin install
rule: The marketplace tree is written only by viola plugin install, each file rewritten whole at 0600 and each dir 0700, with no hooks in any document and any other entry left as found; viola never reads the tree back.
@row_check n=336 row=decision/marketplace-written-only-by-install sha=065b99c24cc1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:116
quote: replace_private(&root.join(rel), content.as_bytes(), FILE_MODE)?;
at: src/cmd/plugin.rs:110
quote: create_private_dir(dir)?;
at: crates/viola-agent-claude/src/marketplace.rs:24
quote: pub fn marketplace_files(exe_fwd: &str, home_fwd: &str) -> [(&'static str, String); 3] {
searched: marketplace · in src (non-test; only src/cmd/plugin.rs writes it, src/human/verbs.rs is text)
@row_add n=337 row=decision/no-writes-under-claude-home area=data-protection rec=fct-zvqsbdzq@1,fct-uz6mlfv5@2
name: viola writes nothing under the Claude home
rule: viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
@row_check n=337 row=decision/no-writes-under-claude-home sha=a97540adaf5c v=holds judge=sonnet head=8834195b0683
searched: claude\.json · in src/**/*.rs, crates/**/*.rs (no hit)
searched: Command::new\(.claude · in src/**/*.rs, crates/**/*.rs (no hit)
searched: \.join\("\.claude"\) · in src/**/*.rs, crates/**/*.rs (only src/cmd/run.rs:1489, a read of the user's settings path, and tests)
@row_add n=338 row=decision/verify-probe-captures-transient area=data-protection rec=fct-zvqsbdzq@1
name: Verify probe captures are transient
rule: viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
@row_check n=338 row=decision/verify-probe-captures-transient sha=16937f38241b v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:443
quote: replace_private(&path, &bytes, FILE_MODE).ok()?;
at: src/cmd/verify.rs:398
quote: create_private_dir(&captures)?;
at: src/cmd/verify.rs:391
quote: let _ = std::fs::remove_dir_all(&self.dir);
at: src/cmd/verify.rs:300-301
quote: match &args.record { Some(dir) => record(dir, &version, &probes),
@row_add n=339 row=decision/verify-probe-dirs-cleanup area=data-protection rec=fct-zvqsbdzq@1
name: Verify probe directories removed by drop guards
rule: Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists.
except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
@row_check n=339 row=decision/verify-probe-dirs-cleanup sha=cc48ab46545d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/runs.rs:219
quote: create_private_dir(&guard.0)?;
at: src/cmd/verify/typed/runs.rs:226
quote: let _ = std::fs::remove_dir_all(&self.0);
at: src/cmd/verify.rs:391
quote: let _ = std::fs::remove_dir_all(&self.dir);
at: src/cmd/verify/stop.rs:12
quote: for signal in [libc::SIGINT, libc::SIGTERM] {
@row_add n=340 row=decision/verify-probes-confined-to-own-dir area=data-protection rec=fct-zvqsbdzq@1,fct-uz6mlfv5@2
name: Verify probe effects stay in their own directory
rule: Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
@row_check n=340 row=decision/verify-probes-confined-to-own-dir sha=d741437515c8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:238
quote: r#"{"permissions":{"ask":["Bash(touch viola-probe-permission)"]}}"#;
at: crates/viola-agent-claude/src/ledger.rs:242
quote: json!({ "plansDirectory": plans_dir }).to_string()
at: src/cmd/verify/typed/runs.rs:154
quote: let plans = dir.0.join("plans");
at: src/cmd/verify/typed/runs.rs:162
quote: let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(KeyPipe::new(pipe)))?;
@row_add n=341 row=decision/pinned-copy-no-clobber-placement area=data-protection rec=fct-l66ara67@2,fct-lzwf4bbi@1,fct-zvqsbdzq@1
name: No-clobber placement of the pinned copy
rule: An absent pinned copy is placed by a no-clobber placement of a temp file whose mode is set before any byte, written, synced and closed first; a present name is never replaced, the target left untouched and the temp file removed.
why: The name never appears while a handle holds it open for write.
@row_check n=341 row=decision/pinned-copy-no-clobber-placement sha=e1e804d1ab9d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:114
quote: let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
at: crates/viola-state/src/fs.rs:115
quote: restrict(tmp.path(), mode)?;
at: crates/viola-state/src/fs.rs:116
quote: tmp.write_all(bytes)?;
at: crates/viola-state/src/fs.rs:117
quote: tmp.as_file().sync_all()?;
at: crates/viola-state/src/fs.rs:118
quote: match tmp.into_temp_path().persist_noclobber(path) {
at: crates/viola-state/src/fs.rs:123
quote: Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
@row_add n=342 row=decision/lost-placement-rehash area=data-protection rec=fct-i5etdjvh@1,fct-zvqsbdzq@1
name: A lost placement re-hashes the standing copy
rule: When a concurrent start placed the pinned copy first, the standing copy is re-hashed: equal bytes are reused, different bytes are a hash mismatch with the file never overwritten or deleted.
@row_check n=342 row=decision/lost-placement-rehash sha=00830d40f1c0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:107
quote: Placed::Present if file_key(path)? == hash => Ok(()),
at: crates/viola-state/src/pin.rs:108
quote: Placed::Present => Err(PinError::HashMismatch),
@row_add n=343 row=decision/two-keys-per-tree-accepted area=data-protection rec=fct-uxsbnoo5@2
name: One tree may key two pinned folders
rule: Clean release builds of one tree may key two bin and plugin folders; this is accepted with no action, the pinned binary's re-hash staying the guard.
@row_check n=343 row=decision/two-keys-per-tree-accepted sha=f879bf2a69f3 v=unverifiable judge=sonnet head=8834195b0683
searched: two keyed folders accepted · in crates/viola-state/src/pin.rs (the re-hash at pin.rs:90 exists, but the acceptance of two keys is a recorded decision)
why: not a code claim
@row_add n=344 row=contract/events-link-lines/data area=data-protection rec=fct-cettrvwp@1
boundary: events.ndjson link lines
item: data
promise: A link or unlink line carries data {driver, driven}: the two instance names only, no user content.
@row_check n=344 row=contract/events-link-lines/data sha=8cb0b9556c47 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:123
quote: let data = json!({"driver": driver, "driven": driven});
@row_add n=345 row=contract/events-link-lines/no-op-pairs area=data-protection rec=fct-cettrvwp@1
boundary: events.ndjson link lines
item: a repeated link or an absent pair's unlink
promise: A held pair's second link and an absent pair's unlink append nothing.
@row_check n=345 row=contract/events-link-lines/no-op-pairs sha=d8b6d25f9df8 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-64
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true)); }
at: src/run/links.rs:105-107
quote: if self.since(&set, driver).is_none() { return Ok(false); }
@row_add n=346 row=contract/channel-link-method/reply area=data-protection rec=fct-cettrvwp@1,fct-lce5n72k@1
boundary: the wrapper's link method
item: reply
promise: A successful link replies only {since, already_linked}, since a string and already_linked a boolean.
@row_check n=346 row=contract/channel-link-method/reply sha=9c1aebd9e8d0 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
@row_add n=347 row=contract/channel-unlink-method/reply area=data-protection rec=fct-cettrvwp@1,fct-lce5n72k@1
boundary: the wrapper's unlink method
item: reply
promise: A successful unlink replies only an empty object {}.
@row_check n=347 row=contract/channel-unlink-method/reply sha=c46de3a1a0ff v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
@row_add n=348 row=decision/link-names-kept-out-of-process-log area=data-protection rec=fct-jbwneego@1
name: Link names kept out of the process log
rule: The wrapper's link and unlink methods write no process-log line of their own and their request and response lines carry no name; viola link and unlink print only their own parsed arguments, never a name read back from a reply or log.
@row_check n=348 row=decision/link-names-kept-out-of-process-log sha=2e4d894332d0 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:72-81
quote: pub(crate) fn link(&self, params: &Value) -> Result<Value, ProtocolError> { let driver = self.driver_of(params)?; let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?; Ok(json!({"ok": {"since": since, "already_linked"…
at: crates/viola-channel/src/server.rs:371-373
quote: method = method, sender = params.sender.as_deref().and_then(version_label), v = params.v,
at: src/cmd/link.rs:93
quote: let (driver, driven): (&str, &str) = (args.driver.as_ref(), args.driven.as_ref());
@row_add n=349 row=decision/diagnostics-open-fails-to-sink area=data-protection rec=fct-gfsndi6o@2
name: A refused diagnostics open writes nowhere
rule: Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
@row_check n=349 row=decision/diagnostics-open-fails-to-sink sha=664ca07c5880 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:89
quote: check_diagnostics(&dir, &path)?;
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
at: src/obs.rs:74
quote: .try_for_each(viola_state::strict::check_diagnostics)
@row_add n=350 row=decision/probe-prompts-synthetic area=data-protection rec=fct-cnkeyzag@2
name: Probe prompts are synthetic
rule: Probe prompts used to record committed fixtures must be synthetic.
@row_check n=350 row=decision/probe-prompts-synthetic sha=dabcb27db5f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:219
quote: pub const DIALOG_PROMPT_QUESTIONS: &str = "viola verify probe: call the AskUserQuestion tool exactly \
at: crates/viola-agent-claude/src/ledger.rs:232
quote: pub const PLAN_PROMPT: &str = "viola verify probe: make a plan only, do not run anything. The plan \
@row_add n=351 row=decision/screen-fixture-signature-rows-only area=data-protection rec=fct-cnkeyzag@2
name: Screen fixtures keep only signature rows
rule: A screen fixture keeps only the rows holding a compiled signature literal, the rest blanked; a kept row the scrub would rewrite, or a username split across a row seam, refuses the recording.
@row_check n=351 row=decision/screen-fixture-signature-rows-only sha=067c7a88097a v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1377
quote: if SIGNATURES.holds_any(r) {
at: crates/viola-agent-claude/src/ledger.rs:1430
quote: if scrub_text(text, home, "") != text {
at: crates/viola-agent-claude/src/ledger.rs:1410
quote: texts.push((true, seam(above, row)));
at: src/cmd/verify.rs:601
quote: if let Some(fault) = ledger::screen_fault(rows, &kept, home, user) {
@row_add n=352 row=contract/fixture-refusal-line/form area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal
item: line form
promise: The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
@row_check n=352 row=contract/fixture-refusal-line/form sha=4b11ca37fdbd v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:626
quote: &format!("a recorded fixture is not clean: {named}"),
at: src/cmd/verify.rs:603
quote: return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
@row_add n=353 row=contract/fixture-refusal-codes/home-path area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: home-path
promise: A closed check code of the fixture refusal, for a surviving home path.
@row_check n=353 row=contract/fixture-refusal-codes/home-path sha=3ecd9a3d1df9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1306
quote: Self::HomePath => "home-path",
@row_add n=354 row=contract/fixture-refusal-codes/absolute-path area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: absolute-path
promise: A closed check code of the fixture refusal, for a surviving absolute path.
@row_check n=354 row=contract/fixture-refusal-codes/absolute-path sha=d929a927e513 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1307
quote: Self::AbsolutePath => "absolute-path",
@row_add n=355 row=contract/fixture-refusal-codes/username area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: username
promise: A closed check code of the fixture refusal, for a surviving user word.
@row_check n=355 row=contract/fixture-refusal-codes/username sha=f3bcfe6776b8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1308
quote: Self::Username => "username",
@row_add n=356 row=contract/fixture-refusal-codes/email area=data-protection rec=fct-cnkeyzag@2
boundary: verify's unclean-fixture refusal codes
item: email
promise: A closed check code of the fixture refusal, for a surviving email-shaped token.
@row_check n=356 row=contract/fixture-refusal-codes/email sha=0f7827646dfd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1309
quote: Self::Email => "email",
@row_add n=357 row=decision/mcp-fixtures-via-test-only-tap area=data-protection rec=fct-cnkeyzag@2
name: MCP fixtures recorded through a test-only tap
rule: MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes.
guard: schemas/claude-mcp-fixture.v1.json
@row_check n=357 row=decision/mcp-fixtures-via-test-only-tap sha=a15866120965 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:83
quote: let spawned = Command::new(viola)
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:60
quote: let read = reader.by_ref().take(MAX_FRAME).read_until(b'\n', &mut line);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:52
quote: options.mode(0o600);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:212
quote: let scrubbed = ledger::scrub(&doc, &home, &user);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:187
quote: ledger::screen_fault(&row, &row, home, user).map(|f| f.why.code())
guard_at: schemas/claude-mcp-fixture.v1.json:1
@row_add n=358 row=contract/verify-refusal-codes/program-not-found area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: program-not-found
promise: One of the closed codes verify writes on its process-exit when refused.
@row_check n=358 row=contract/verify-refusal-codes/program-not-found sha=dfb98c24042f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
@row_add n=359 row=contract/verify-refusal-codes/pinned-hash-mismatch area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: pinned-hash-mismatch
promise: One of the closed codes verify writes on its process-exit when refused.
@row_check n=359 row=contract/verify-refusal-codes/pinned-hash-mismatch sha=5a6ea4414253 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
@row_add n=360 row=contract/verify-refusal-codes/version-unreadable area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: version-unreadable
promise: One of the closed codes verify writes on its process-exit when refused.
@row_check n=360 row=contract/verify-refusal-codes/version-unreadable sha=c5659e6b28a4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
@row_add n=361 row=contract/verify-refusal-codes/strict-modes-failed area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: strict-modes-failed
promise: One of the closed codes verify writes on its process-exit when refused.
@row_check n=361 row=contract/verify-refusal-codes/strict-modes-failed sha=8cf0a9dbbafe v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=362 row=contract/verify-refusal-codes/unclean-fixture area=data-protection rec=fct-zaeoull5@2
boundary: verify's refusal codes
item: unclean-fixture
promise: One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
@row_check n=362 row=contract/verify-refusal-codes/unclean-fixture sha=972ed325f8f8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=363 row=absence/key-management area=data-protection rec=fct-uz6mlfv5@2
capability: Keys at rest and their management
@row_check n=363 row=absence/key-management sha=a5d4ce5911f0 v=holds judge=sonnet head=8834195b0683
searched: keyring|encrypt|chacha|ed25519|hmac|secret_key|private_key|api_key|rustls · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=364 row=decision/events-rotation-keeps-modes-and-offsets area=data-protection rec=fct-uz6mlfv5@2
name: Event log rotation keeps modes and offsets
rule: Any rotation scheme for events.ndjson keeps 0600/0700 permissions and keeps offsets valid.
@row_check n=364 row=decision/events-rotation-keeps-modes-and-offsets sha=8bcf2b585dfb v=unverifiable judge=sonnet head=8834195b0683
searched: rotat|retention|set_len · in src/**/*.rs, crates/*/src/**/*.rs (no rotation scheme exists; only test truncations)
why: no evidence found
@row_add n=365 row=decision/verify-real-cli-transcripts-accepted area=data-protection rec=fct-uz6mlfv5@2
name: Verify's real-CLI transcripts accepted
rule: verify's real interactive runs under a trusted parent leave the CLI's own synthetic-prompt session transcripts and run the user's global hooks; this residual is an accepted risk class.
@row_check n=365 row=decision/verify-real-cli-transcripts-accepted sha=276b7c271786 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=366 row=decision/plugin-install-live-round-rig area=data-protection rec=fct-uz6mlfv5@2
name: Live plugin-install rounds isolated
rule: A live round of viola plugin install runs the CLI's commands at local scope from a project that is its own git repo outside the work tree, passing the trust dialog only by keys into its own pty, never a desktop key.
@row_check n=366 row=decision/plugin-install-live-round-rig sha=0aeead637fff v=unverifiable judge=sonnet head=8834195b0683
searched: --scope · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
searched: marketplace add · in crates/viola-e2e
why: no evidence found
@row_add n=367 row=decision/verify-never-answers-import-dialog area=data-protection rec=fct-uz6mlfv5@2
name: Real-CLI verify needs a prepared trusted folder
rule: viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
@row_check n=367 row=decision/verify-never-answers-import-dialog sha=8436d2ea7416 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/turns.rs:76
quote: holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
at: crates/viola-agent-claude/src/screen.rs:42
quote: modals: &["Yes, I trust this folder", "Yes, allow external imports"],
@row_add n=368 row=decision/event-log-growth-availability-risk area=data-protection rec=fct-uz6mlfv5@2
name: Event log growth accepted as availability risk
rule: Unbounded growth of events.ndjson exhausting local disk is an accepted risk until retention is resolved.
why: The impact is on availability only, with no confidentiality or integrity impact.
@row_check n=368 row=decision/event-log-growth-availability-risk sha=adc071319915 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=369 row=decision/serde-json-unbounded-depth-banned area=dependency-security rec=fct-pgy5bb7v@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned in cargo-deny.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth
@row_check n=369 row=decision/serde-json-unbounded-depth-banned sha=8bc3cf503495 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: tests/contract_bounded_readers.rs:764
@row_add n=370 row=decision/no-stale-license-exceptions area=dependency-security rec=fct-qfxjrvyr@1
name: No license exception for an absent crate
rule: A license exception naming a crate absent from the graph is dropped, never kept.
why: Such an exception narrows nothing and widens nothing.
@row_check n=370 row=decision/no-stale-license-exceptions sha=62fe4f4f6cf4 v=holds judge=sonnet head=8834195b0683
searched: name = "doctest-file" · in Cargo.lock (the only [[licenses.exceptions]] crate in deny.toml is present in the graph)
searched: [[licenses.exceptions]] · in deny.toml, deny-sync.toml (one entry, doctest-file)
@row_add n=371 row=decision/c-building-crates-banned area=dependency-security rec=fct-o63eny7e@2
name: C-building crates banned
rule: Crates that build C are banned from the dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_check n=371 row=decision/c-building-crates-banned sha=b3698b5b8914 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
guard_at: scripts/deny-probes.sh:21
@row_add n=372 row=decision/cargo-deny-gate-clean area=dependency-security rec=fct-6nwwowx3@2
name: cargo-deny gate with no advisory ignores
rule: CI's supply-chain gate runs cargo deny over advisories, bans, licenses and sources with the advisory ignore list empty.
@row_check n=372 row=decision/cargo-deny-gate-clean sha=5c4ace69c147 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: deny.toml:11
quote: ignore = []
@row_add n=373 row=decision/fuzz-workspace-exempt-from-root-deny area=dependency-security rec=fct-i5izpzxc@1
name: Fuzz workspace outside the root deny graph
rule: fuzz is a separate cargo workspace with its own lockfile, outside cargo-deny's root graph, so the C-build ban does not apply to it.
why: Ratified as a test-only exemption.
except: Holds only while the fuzz crate is never linked into a shipped binary and never joins the root workspace.
@row_check n=373 row=decision/fuzz-workspace-exempt-from-root-deny sha=6f5316cf26eb v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:182
quote: exclude = ["fuzz"]
at: fuzz/Cargo.toml:20-21
quote: [workspace] members = ["."]
@row_add n=374 row=decision/fuzz-lockfile-audited-separately area=dependency-security rec=fct-i5izpzxc@1
name: Fuzz lockfile audited separately
rule: The fuzz lockfile is audited separately for advisories and sources in CI and for advisories weekly.
@row_check n=374 row=decision/fuzz-lockfile-audited-separately sha=3322bd9ebd59 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=375 row=decision/g2-zero-panics-single-exemption area=dependency-security rec=fct-fguujmtu@1
name: Zero-panics gate with one exemption
rule: CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry.
except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
guard: G2 --probe
@row_check n=375 row=decision/g2-zero-panics-single-exemption sha=93027fceca47 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
guard_at: scripts/g2-zero-panics.sh:101
@row_add n=376 row=decision/checkout-no-persisted-credentials area=dependency-security rec=fct-dj5p5k3a@1
name: Checkout without persisted credentials
rule: The CI checkout action runs with persist-credentials false.
@row_check n=376 row=decision/checkout-no-persisted-credentials sha=09ff144e13f7 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:24
quote: persist-credentials: false
@row_add n=377 row=decision/ci-artifact-canary-scan area=bootstrap-phases rec=fct-f326yvjk@1
name: CI artifact canary scan gates uploads
rule: Every scan-gated CI upload waits for the artifact canary scan over test diagnostics, harness capture and test reports against the NEVER-log floor, which never prints or writes the matched bytes.
except: The harness capture's chunk.diff, repository source text by construction, is skipped by exact path and never uploaded.
@row_check n=377 row=decision/ci-artifact-canary-scan sha=7e68864c89c0 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:94
quote: !target/agent-run/chunk.diff
at: crates/viola-e2e/src/harness/secret_scan.rs:80
quote: capture.retain(|path| *path != mutation_diff);
@row_add n=378 row=decision/no-mutants-out-upload area=bootstrap-phases rec=fct-f326yvjk@1
name: No mutants.out upload
rule: CI does not upload mutants.out.
why: It held absolute argv paths and test output.
@row_check n=378 row=decision/no-mutants-out-upload sha=2c1fa2e2b4f5 v=holds judge=sonnet head=8834195b0683
searched: mutants.out · in .github/
searched: upload-artifact (every path: value read: target/e2e-home, target/agent-run, target/nextest, target/secret-scan, target/supply-chain, fuzz/artifacts) · in .github/workflows/
@row_add n=379 row=decision/fuzz-artifacts-upload-unscanned area=bootstrap-phases rec=fct-f326yvjk@1
name: Fuzz crash artifacts the one unscanned upload
rule: The nightly fuzz job's crash-input upload is the one CI upload left unscanned; a non-synthetic corpus seed must first drop that upload.
why: Ratified as admissible by content: the inputs are mutated from the committed synthetic corpus.
@row_check n=379 row=decision/fuzz-artifacts-upload-unscanned sha=f9a6a8693076 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:63
quote: path: fuzz/artifacts/
searched: secret-scan · in .github/workflows/nightly.yml
@row_add n=380 row=decision/test-homes-gitignored area=bootstrap-phases rec=fct-f326yvjk@1
name: Test homes kept out of git
rule: Local --home test directories are git-ignored so recorded state never lands in git.
@row_check n=380 row=decision/test-homes-gitignored sha=e994fb9aca56 v=holds judge=sonnet head=8834195b0683
at: .gitignore:16
quote: target/
@row_add n=381 row=decision/hook-strict-modes-first area=error-handling rec=fct-io4zn2wb@2
name: hook checks the instance before anything
rule: The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
except: The two capture arms run ahead of that check.
@row_check n=381 row=decision/hook-strict-modes-first sha=4dafd82eb493 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:101-107
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS); }
@row_add n=382 row=contract/hook-decision-detail/oversize-stdin area=error-handling rec=fct-io4zn2wb@2
boundary: hook-decision detail
item: oversize-stdin
promise: hook fails open with this detail when stdin exceeds its cap.
@row_check n=382 row=contract/hook-decision-detail/oversize-stdin sha=7d6acec46052 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:181
quote: return Ok(Some("oversize-stdin"));
@row_add n=383 row=contract/hook-decision-detail/malformed-json area=error-handling rec=fct-io4zn2wb@2
boundary: hook-decision detail
item: malformed-json
promise: hook fails open with this detail when stdin is not one JSON object.
@row_check n=383 row=contract/hook-decision-detail/malformed-json sha=16c381afae82 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:185
quote: return Ok(Some("malformed-json"));
@row_add n=384 row=contract/hook-decision-detail/channel-unreachable area=error-handling rec=fct-io4zn2wb@2,fct-bp6aacay@1
boundary: hook-decision detail
item: channel-unreachable
promise: hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
@row_check n=384 row=contract/hook-decision-detail/channel-unreachable sha=34a335f9a216 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:213
quote: Self::Unreachable => "channel-unreachable",
@row_add n=385 row=contract/hook-decision-detail/server-verify-failed area=error-handling rec=fct-io4zn2wb@2,fct-5pxlrd5y@1
boundary: hook-decision detail
item: server-verify-failed
promise: hook fails open with this detail when the peer euid or server verification fails; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_check n=385 row=contract/hook-decision-detail/server-verify-failed sha=4fd9b3a40f43 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
@row_add n=386 row=contract/hook-decision-detail/socket-dir-refused area=error-handling rec=fct-io4zn2wb@2,fct-33fbgdc3@1
boundary: hook-decision detail
item: socket-dir-refused
promise: hook fails open with this detail, in one line at WARN, when the socket directory is refused; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_check n=386 row=contract/hook-decision-detail/socket-dir-refused sha=8819cd32933c v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:215
quote: Self::SocketDir => UnreachableCause::SocketDir.as_str(),
at: src/cmd/hook.rs:390
quote: WARN,
@row_add n=387 row=decision/hook-refusals-write-no-frame area=error-handling rec=fct-5pxlrd5y@1,fct-33fbgdc3@1,fct-bp6aacay@1
name: A refused hook writes no frame
rule: A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
@row_check n=387 row=decision/hook-refusals-write-no-frame sha=2a3a0ec75e17 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:231-233
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir); }
at: src/cmd/hook.rs:227-228
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else { return Err(NotSent::Unreachable);
at: src/cmd/hook.rs:236-237
quote: if !peer_admitted(client.server_euid()) { return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:193-194
quote: if event == HookEvent::SessionEnd { append_session_end(instance, normalised);
@row_add n=388 row=decision/mcp-frame-tool-refusals area=error-handling rec=fct-2qzjtoe6@2,fct-tukgmehf@1
name: Refused MCP frame tools send nothing
rule: A viola mcp frame tool refused by the strict-modes, liveness, socket-directory or server check sends no frame, writes no log line and returns the instance-unreachable result.
@row_check n=388 row=decision/mcp-frame-tool-refusals sha=c7079201a675 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
at: crates/viola-mcp/src/tools.rs:342-343
quote: let Some(live) = live_endpoint(&instance_dir) else { return Ok(result::unreachable(without_endpoint(&instance_dir)));
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=389 row=contract/mcp-frame-tool-result/unreachable area=error-handling rec=fct-2qzjtoe6@2,fct-tukgmehf@1,fct-au76dwrt@1
boundary: viola mcp frame tool results
item: instance-unreachable
promise: A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.
@row_check n=389 row=contract/mcp-frame-tool-result/unreachable sha=39ad039dddec v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=390 row=contract/channel-fault/-32603 area=error-handling rec=fct-lce5n72k@1
boundary: wrapper channel faults
item: -32603
promise: A failed link or unlink record answers -32603 with the fixed message `internal error` and data null.
@row_check n=390 row=contract/channel-fault/-32603 sha=cab060aea29e v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:74
quote: let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?;
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
@row_add n=391 row=contract/link-verbs-exit-codes/20 area=error-handling rec=fct-lce5n72k@1
boundary: viola link / unlink exit codes
item: 20
promise: A wrapper error reply, a -32602 included, is a wrapper fault, exit 20.
@row_check n=391 row=contract/link-verbs-exit-codes/20 sha=711dfca5d046 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
@row_add n=392 row=contract/link-verbs-exit-codes/1 area=error-handling rec=fct-lce5n72k@1
boundary: viola link / unlink exit codes
item: 1
promise: A Refused reply or an ok of a shape other than the verb's own is an internal error, exit 1.
@row_check n=392 row=contract/link-verbs-exit-codes/1 sha=8c4b80e52bf2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:61
quote: Reply::Refused { .. } => anyhow::bail!("a link request was refused"),
at: src/cmd/link.rs:51
quote: anyhow::ensure!(known, "malformed reply");
@row_add n=393 row=decision/channel-faults-sanitised area=error-handling rec=fct-2c6j473b@1,fct-mvo56ebm@1
name: Channel faults sanitised by the clients
rule: Clients read a wrapper fault only with an integer code; each known code reads its fixed message with data null.
except: Two -32602 bodies keep data: the unsupported-version body with an integer data.supported and a 1-64 character data.wrapper, and the body whose data.reason is release-from-driver.
@row_check n=393 row=decision/channel-faults-sanitised sha=4e1707aae989 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:118
quote: let code = error["code"].as_i64()?;
at: crates/viola-channel/src/lib.rs:128
quote: -32602 => match (data["supported"].as_i64(), wrapper) {
at: crates/viola-channel/src/lib.rs:133
quote: _ if data["reason"] == "release-from-driver" => (
at: crates/viola-channel/src/lib.rs:146
quote: (1..=64).contains(&text.len())
@row_add n=394 row=contract/channel-fault/unknown-code area=error-handling rec=fct-yrpufic3@1
boundary: wrapper faults as read by clients
item: a code outside the five
promise: Reads {"code": <code>, "message": "wrapper fault", "data": null}; `wrapper fault` is a fixed word on --json and MCP, and the human line reads `error: wrapper fault  <code>`.
@row_check n=394 row=contract/channel-fault/unknown-code sha=bf8f682ba161 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=395 row=decision/reply-code-words-validated area=error-handling rec=fct-tr2s4okr@1,fct-lure7uoj@1
name: Reply code words validated
rule: A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply.
except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
@row_check n=395 row=decision/reply-code-words-validated sha=980b778c7a55 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:377-381
quote: bytes.len() <= 64 && bytes.first().is_some_and(u8::is_ascii_lowercase) && bytes .iter() .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
at: src/cmd/client.rs:77
quote: let reason = if is_code_word(reason) {
at: src/cmd/client.rs:84-85
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()), Some(_) => return None,
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=396 row=contract/channel-response-detail/unsupported-version area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: unsupported-version
promise: The closed detail word for the unsupported-version -32602 fault, written on the server's and the client's line.
@row_check n=396 row=contract/channel-response-detail/unsupported-version sha=97aaaf75b8ad v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105
quote: Self::UnsupportedVersion => Some("unsupported-version"),
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
@row_add n=397 row=contract/channel-response-detail/invalid-params area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: invalid-params
promise: The closed detail word for the invalid-params -32602 fault, written on the server's and the client's line.
@row_check n=397 row=contract/channel-response-detail/invalid-params sha=6dec1de440c2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
@row_add n=398 row=contract/channel-response-detail/release-from-driver area=error-handling rec=fct-sbwauatb@1
boundary: channel-response detail
item: release-from-driver
promise: The closed detail word for the release-from-driver -32602 fault, written on the server's and the client's line.
@row_check n=398 row=contract/channel-response-detail/release-from-driver sha=69ae7cbae5f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
@row_add n=399 row=decision/refused-peer-dropped-before-frame area=error-handling rec=fct-3t6spjp3@1,fct-sawbqqxr@2
name: A refused peer is dropped before any frame
rule: A connection whose peer euid is refused is dropped on the accept thread before any worker or frame: the peer reads end-of-file and the wrapper writes no reply, no line and nothing on the terminal.
@row_check n=399 row=decision/refused-peer-dropped-before-frame sha=e901addd79b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: crates/viola-channel/src/server.rs:149-150
quote: .name("channel-conn".to_owned()) .spawn(move || serve_conn(&stream, &format!("srv-{n}"), dispatch.as_ref()));
@row_add n=400 row=decision/internal-error-held-at-unit-layer area=error-handling rec=fct-iwcvxbkd@1
name: Internal errors tested at the unit layer
rule: No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_check n=400 row=decision/internal-error-held-at-unit-layer sha=a1aac0c7a99e v=holds guard=true judge=sonnet head=8834195b0683
searched: failpoint · in tests/**/*.rs and crates/viola-e2e/**
searched: -32603 · in tests/**/*.rs (only the literal-body table tests/contract_error_bodies.rs and the doc line tests/channel_error_bodies.rs:5 saying it stays at the unit layer; no scanned-home driver) and crates/viola-e2e/**
searched: counted panic · in crates/viola-e2e/** and tests/channel_error_bodies.rs
guard_at: crates/viola-channel/src/server.rs:677
@row_add n=401 row=decision/cli-frame-verbs-strict-modes-first area=error-handling rec=fct-uv5qlev7@1
name: CLI frame verbs check the instance first
rule: CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
@row_check n=401 row=decision/cli-frame-verbs-strict-modes-first sha=eb1f24b74e2e v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/last.rs:30
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/answer.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/pause.rs:32
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/unlink.rs:12
quote: use super::link::{LinkArgs, Verb, link_call};
@row_add n=402 row=contract/cli-instance-unreachable/json-document area=error-handling rec=fct-au76dwrt@1,fct-uv5qlev7@1
boundary: CLI exit 21
item: --json document
promise: Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
@row_check n=402 row=contract/cli-instance-unreachable/json-document sha=3674ba78b4fa v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: crates/viola-core/src/lib.rs:185-190
quote: StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir,
@row_add n=403 row=contract/cli-stderr-hints/unwrapped-name-endpoint-lost area=error-handling rec=fct-t5nvscu6@1
boundary: CLI stderr hints
item: unwrapped-name and endpoint-lost hints
promise: The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
@row_check n=403 row=contract/cli-stderr-hints/unwrapped-name-endpoint-lost sha=fdfc270c40da v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:50-52
quote: "unwrapped-name" => format!(             "{name} is not a wrapped session; only sessions started with viola run take commands"         ),
at: src/human/hints.rs:53-55
quote: "endpoint-lost" => format!(             "{name}'s wrapper went away during the call; viola list shows whether it is still running"         ),
@row_add n=404 row=decision/socket-dir-refusal-names-only-name area=error-handling rec=fct-2iushqjk@1
name: Socket-directory refusal discloses no path
rule: The socket-directory refusal names only the parsed ViolaName, and its lines hold no path, pid or uid.
@row_check n=404 row=decision/socket-dir-refusal-names-only-name sha=287e2004b3e0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:738
quote: &format!("the socket directory for {name} cannot be trusted"),
at: src/human/hints.rs:59
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
@row_add n=405 row=decision/unusable-response-line-on-stderr area=error-handling rec=fct-6qpqztvz@1
name: Unusable answer or send text goes to stderr
rule: The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document.
why: A named exception to the --json error document.
@row_check n=405 row=decision/unusable-response-line-on-stderr sha=702d401ff9ff v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
@row_add n=406 row=decision/home-tree-strict-modes-before-log area=error-handling rec=fct-j463drna@2
name: Home tree checked before any log opens
rule: viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_check n=406 row=decision/home-tree-strict-modes-before-log sha=b521fc3da19e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/revive.rs:141
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
@row_add n=407 row=decision/run-refuses-socket-dir-before-start area=error-handling rec=fct-oolatrnj@1
name: run refuses a bad socket directory first
rule: viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.
@row_check n=407 row=decision/run-refuses-socket-dir-before-start sha=a31922c8f808 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=408 row=contract/cli-json-error/internal-error area=error-handling rec=fct-vr46nktj@1
boundary: CLI --json error documents
item: internal-error
promise: Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
@row_check n=408 row=contract/cli-json-error/internal-error sha=0f0951bd4c9a v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-68
quote: if json_asked(std::env::args_os()) {             cmd::internal_error_document();
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=409 row=contract/collision-check/stale-heartbeat area=error-handling rec=fct-fmkuqvun@1
boundary: start collision check
item: stale-heartbeat
promise: Printed and logged for a stale holder of the name; stale heartbeat is exit 1's cause alone.
@row_check n=409 row=contract/collision-check/stale-heartbeat sha=8cd02db8278f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=410 row=contract/collision-check/already-live area=error-handling rec=fct-fmkuqvun@1
boundary: start collision check
item: already-live
promise: Printed and logged for a live holder and for a fresh replayed beat.
@row_check n=410 row=contract/collision-check/already-live sha=a8fda8bbd75c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age()             .is_some_and(|age| age <= STALE_AFTER)             .then_some(Liveness::Live),
@row_add n=411 row=decision/error-messages-interpolate-nothing area=error-handling rec=fct-g4i32qdx@2,fct-ttxl4mzv@1,fct-syp3b7yx@1
name: Error messages interpolate nothing
rule: Product error types carry fixed Display messages with no path or payload interpolated.
guard: thiserror_messages_interpolate_nothing
@row_check n=411 row=decision/error-messages-interpolate-nothing sha=f8f64b359017 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:40
quote: #[error("the capability stamps failed the strict-modes check")]
guard_at: tests/contract_error_bodies.rs:794
@row_add n=412 row=decision/error-enum-convention-exceptions area=error-handling rec=fct-gnmmvxa5@2
name: Named exceptions to the error-enum convention
rule: ServeError, StampsError, PinError, StampsReadError and Refused diverge from the error-enum convention as named exceptions.
why: Ratified as named exceptions.
@row_check n=412 row=decision/error-enum-convention-exceptions sha=980117f69519 v=unverifiable judge=sonnet head=8834195b0683
searched: error convention|error-enum|error enum · in src crates tests *.rs
why: no evidence found; the enums exist but no code or doc in the tree states the error-enum convention they diverge from
@row_add n=413 row=decision/every-error-writer-named area=error-handling rec=fct-syp3b7yx@1
name: Every product error writer named
rule: Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census.
guard: product_error_writers_are_each_named
@row_check n=413 row=decision/every-error-writer-named sha=100657874fdd v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:786-790
quote: assert!(read_lines > 0, "the census read no writer"); assert!( unnamed.is_empty(), "error writers no row names: {unnamed:#?}" );
guard_at: tests/contract_error_bodies.rs:771
@row_add n=414 row=decision/diagnostics-strict-modes-fail-open area=error-handling rec=fct-wyrvqwn5@2
name: Diagnostics files failing strict modes fail open
rule: A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
@row_check n=414 row=decision/diagnostics-strict-modes-fail-open sha=23e5b014f99e v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:89
quote: check_diagnostics(&dir, &path)?;
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
@row_add n=415 row=decision/paste-text-control-character-set area=input-validation rec=fct-ub5klpnc@1
name: Paste text control-character set
rule: Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes.
why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
@row_check n=415 row=decision/paste-text-control-character-set sha=3e08d28e478e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:366-367
quote: '\n' | '\r' | '\t' => false, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
at: crates/viola-core/src/lib.rs:357
quote: if text.chars().any(is_refused_control) {
@row_add n=416 row=decision/reject-dont-strip-paste-text area=input-validation rec=fct-ub5klpnc@1
name: Reject, don't strip
rule: A sent text that holds a refused character is refused whole; no refused character is ever removed and the rest typed.
@row_check n=416 row=decision/reject-dont-strip-paste-text sha=279c3aa82794 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:357-358
quote: if text.chars().any(is_refused_control) { Err(NotDelivered::ControlCharacter)
at: src/run/send.rs:88-89
quote: if let Err(detail) = viola_core::validate_paste_text(text) { return ctx.not_delivered(None, detail);
@row_add n=417 row=decision/typed-text-line-ending-normalisation area=input-validation rec=fct-ub5klpnc@1
name: Typed text line-ending normalisation
rule: After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste.
why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged and the delivery-confirmation match stays an exact equality.
@row_check n=417 row=decision/typed-text-line-ending-normalisation sha=288ac2b0f639 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send.rs:91-92
quote: let typed = typed_text(text); let text: &str = &typed;
@row_add n=418 row=decision/empty-typed-text-refused area=input-validation rec=fct-ub5klpnc@1
name: Empty typed text refused
rule: A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed.
why: Ratified.
@row_check n=418 row=decision/empty-typed-text-refused sha=92ed47ceb8de v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:185-187
quote: typed_text(text) .is_empty() .then_some(NotDelivered::EmptyText)
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=419 row=decision/paste-validation-on-both-sides area=input-validation rec=fct-ub5klpnc@1
name: Paste validation on both sides
rule: Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
@row_check n=419 row=decision/paste-validation-on-both-sides sha=b9e624e2180e v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:191
quote: if let Some(detail) = refusal_of(text) {
at: crates/viola-mcp/src/tools.rs:374
quote: if let Some(detail) = refusal_of(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=420 row=contract/send-refusal/control-character area=input-validation rec=fct-ub5klpnc@1
boundary: send refusal
item: control-character
promise: A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
@row_check n=420 row=contract/send-refusal/control-character sha=db37e697c8f2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88-89
quote: if let Err(detail) = viola_core::validate_paste_text(text) {         return ctx.not_delivered(None, detail);
at: src/cmd/send.rs:194
quote: run::log_self_exit(13, None);
@row_add n=421 row=decision/dialog-free-text-control-character-set area=input-validation rec=fct-fn2six5m@1
name: Dialog free text control-character set
rule: Dialog free text (an answer's message and question answer values) follows the paste-text set, refused control-character first, in viola answer before any frame (exit 13) and again, authoritatively, in the wrapper's answer handler.
@row_check n=421 row=decision/dialog-free-text-control-character-set sha=38ffeb49f6b8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=422 row=contract/channel-frames/oversize-frame area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: over-limit frame
promise: A frame is read under a MAX_FRAME cap; an over-limit frame gets -32600 and the connection is closed.
@row_check n=422 row=contract/channel-frames/oversize-frame sha=4fddca87409b v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:24
quote: io::Read::take(&mut *reader, cap)
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:267-270
quote: Reply { frame: error_response(None, ProtocolError::InvalidRequest), close: true, }
@row_add n=423 row=decision/channel-json-default-depth-limit area=input-validation rec=fct-wrh45y73@1
name: Channel JSON default depth limit
rule: Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
@row_check n=423 row=decision/channel-json-default-depth-limit sha=9787af3482c0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:79
quote: let value: Value = serde_json::from_slice(line).map_err(|_| (None, ProtocolError::Parse))?;
at: deny.toml:55
quote: deny = ["unbounded_depth"]
@row_add n=424 row=contract/channel-frames/version area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: v
promise: A frame whose v fails the version check gets -32602.
@row_check n=424 row=contract/channel-frames/version sha=19ace7535ac5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238-239
quote: if params.v > PROTOCOL_V { return Err(ProtocolError::UnsupportedVersion);
@row_add n=425 row=contract/channel-frames/send-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: send params
promise: text is a string and from is absent, null or a valid viola name; anything else is -32602 invalid params with data null, before paste validation runs; the envelope types no method param, so a mistyped from never gets -32600.
@row_check n=425 row=contract/channel-frames/send-params sha=f7dd9ec8c9b9 v=holds judge=sonnet head=8834195b0683
at: src/run/send/params.rs:7-9
quote: let text = params["text"] .as_str() .ok_or(ProtocolError::InvalidParams)?;
at: src/run/send/params.rs:16
quote: None | Some(Value::Null) => Ok(None),
at: src/run/send/params.rs:20
quote: Some(_) => Err(ProtocolError::InvalidParams),
at: crates/viola-channel/src/frame.rs:67-74
quote: pub struct Params { #[serde(default)] pub v: u64, #[serde(default)] pub sender: Option<String>, #[serde(default)] pub conn: Option<String>, }
@row_add n=426 row=contract/channel-frames/wait-last-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: wait and last params
promise: after and timeout_ms are each absent or a non-negative integer that fits u64, else -32602 invalid params with data null; from follows the send rule; unknown fields are ignored.
@row_check n=426 row=contract/channel-frames/wait-last-params sha=fc36d4cf5edc v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:185
quote: Some(value) => value.as_u64().map(Some).ok_or(ProtocolError::InvalidParams),
at: src/run/wait.rs:136-138
quote: let after = count(params, "after")?; let timeout_ms = count(params, "timeout_ms")?; parse_from(params)?;
at: src/run/wait.rs:174
quote: parse_from(params)?;
@row_add n=427 row=contract/channel-frames/hook-dialog-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: hook.dialog params
promise: kind is a closed dialog kind, data an object, hook_event closed, and tool, input and continuation closed or absent; an invalid frame is not registered.
@row_check n=427 row=contract/channel-frames/hook-dialog-params sha=5d2ebf4aa9b4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:101-104
quote: let kind = params["kind"] .as_str() .and_then(DialogKind::from_name) .ok_or(ProtocolError::InvalidParams)?;
at: src/run/dialog.rs:105-108
quote: let data = params["data"] .as_object() .cloned() .ok_or(ProtocolError::InvalidParams)?;
at: src/run/dialog.rs:109-113
quote: let hook = params["hook_event"] .as_str() .and_then(HookEvent::from_arg) .filter(|h| h.is_dialog()) .ok_or(ProtocolError::InvalidParams)?;
at: src/run/dialog.rs:241
quote: let raise = Raise::parse(params)?;
@row_add n=428 row=contract/channel-frames/answer-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: answer params
promise: dialog_id is u64, from follows the send rule, response maps into the closed per-kind enums, then paste validation runs on its free text.
@row_check n=428 row=contract/channel-frames/answer-params sha=980d838a0201 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:416-421
quote: let dialog_id = params["dialog_id"] .as_u64() .ok_or(ProtocolError::InvalidParams)?; let from = parse_from(params)?; let response = Response::parse(&params["response"]).map_err(|_| ProtocolError::InvalidParams)?;
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=429 row=contract/channel-frames/pause-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: pause params
promise: pause's from follows the send rule.
@row_check n=429 row=contract/channel-frames/pause-params sha=d40077c72fe8 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:189
quote: parse_from(call.params)?;
@row_add n=430 row=contract/channel-frames/release-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: release params
promise: budget is absent or a bool, else -32602 with data null; from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null.
@row_check n=430 row=contract/channel-frames/release-params sha=1aab29ec2255 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: src/run/wheel.rs:208-211
quote: parse_from(params)?; let budget = match params.get("budget") { None => false, Some(budget) => budget.as_bool().ok_or(ProtocolError::InvalidParams)?,
@row_add n=431 row=contract/channel-frames/link-unlink-params area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: link and unlink params
promise: driver and driven are valid viola name strings; a missing, non-string or invalid name, a driven other than the serving wrapper's name, or a driver equal to driven is -32602 with data null and no line; from is never read.
@row_check n=431 row=contract/channel-frames/link-unlink-params sha=32afeb1b302a v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:89-92
quote: params[key] .as_str() .and_then(|name| ViolaName::try_new(name.to_owned()).ok()) .ok_or(ProtocolError::InvalidParams)
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=432 row=contract/channel-frames/hook-event-revalidation area=input-validation rec=fct-wrh45y73@1
boundary: channel frames
item: hook.event
promise: kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; an invalid event is not appended.
@row_check n=432 row=contract/channel-frames/hook-event-revalidation sha=efaf1daf0cff v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:86
quote: const HOOK_KINDS: [EventKind; 5] = [
at: src/cmd/run.rs:102-107
quote: let data = event.get("data").filter(|d| d.is_object())?; let prompt_ok = data["text"].is_string() && matches!(data["origin"].as_str(), Some("harness" | "human")); if kind == EventKind::PromptSubmitted && !prompt_ok { return None;
@row_add n=433 row=decision/client-reply-code-words-only area=input-validation rec=fct-wrh45y73@1
name: Client reads reply refusals as code words only
rule: A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
@row_check n=433 row=decision/client-reply-code-words-only sha=a9b19b01ac36 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
at: src/cmd/client.rs:70
quote: return sanitised_fault(error).map(Reply::Fault);
at: crates/viola-mcp/src/result.rs:60
quote: return sanitised_fault(fault)
@row_add n=434 row=contract/channel-response-log/detail area=input-validation rec=fct-wrh45y73@1
boundary: channel-response log line
item: detail
promise: The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
@row_check n=434 row=contract/channel-response-log/detail sha=127c2e7abd5b v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105-108
quote: Self::UnsupportedVersion => Some("unsupported-version"), Self::InvalidParams => Some("invalid-params"), Self::ReleaseFromDriver => Some("release-from-driver"), Self::Parse | Self::InvalidRequest | Self::MethodNotFound | Self::Internal => None,
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/server.rs:490-496
quote: Some( ProtocolError::InvalidRequest | ProtocolError::UnsupportedVersion | ProtocolError::InvalidParams | ProtocolError::ReleaseFromDriver, ) => { response_line!(WARN)
@row_add n=435 row=contract/hook-stdin/oversize area=input-validation rec=fct-p4cpzbge@1
boundary: hook stdin
item: oversize payload
promise: A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
@row_check n=435 row=contract/hook-stdin/oversize sha=6749de633aa6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:178-181
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?; if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { reject_stdin("oversize"); return Ok(Some("oversize-stdin"));
@row_add n=436 row=decision/hook-unknown-event-silent-exit area=input-validation rec=fct-p4cpzbge@1
name: Hook unknown event exits silently
rule: The hook event argument maps into a closed set of nine events (statusline taken first, apart); an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing written anywhere.
@row_check n=436 row=decision/hook-unknown-event-silent-exit sha=29b34ee93f08 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:63
quote: Self::ALL.into_iter().find(|event| event.as_str() == arg)
at: src/cmd/hook.rs:98-99
quote: let Some(event) = HookEvent::from_arg(&args.event) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
@row_add n=437 row=decision/hook-instance-check-first area=input-validation rec=fct-p4cpzbge@1,fct-l7hy7pzk@2
name: Hook instance check before anything
rule: Hook event, dialog and statusline paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
@row_check n=437 row=decision/hook-instance-check-first sha=106127d9a257 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:122
quote: let _ = obs::viola_obs_init(
at: src/cmd/hook/statusline.rs:42
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook/statusline.rs:46
quote: let _ = obs::viola_obs_init(
@row_add n=438 row=contract/hook-stdin/not-one-object area=input-validation rec=fct-p4cpzbge@1
boundary: hook stdin
item: malformed-json
promise: A payload that is not one JSON object fails open with malformed-json.
@row_check n=438 row=contract/hook-stdin/not-one-object sha=d2fe9aaa4647 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:122-123
quote: if !payload.is_object() { return Err(AgentError::Malformed);
at: src/cmd/hook.rs:183-185
quote: let Ok(normalised) = viola_agent_claude::hook::normalise(event, &bytes) else { reject_stdin("malformed"); return Ok(Some("malformed-json"));
@row_add n=439 row=decision/hook-security-fields-closed-enums area=input-validation rec=fct-p4cpzbge@1
name: Hook security fields in closed enums
rule: Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
@row_check n=439 row=decision/hook-security-fields-closed-enums sha=4d3a2aa043e2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:100
quote: let tool = DialogTool::from_name(name);
at: src/cmd/hook.rs:277
quote: classify(event, &bytes).map_err(|_| ("malformed", "malformed-json"))
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
@row_add n=440 row=decision/hook-dialog-reply-read-bound area=input-validation rec=fct-p4cpzbge@1
name: Hook dialog reply read bound
rule: The hook's read of a dialog reply is bounded at the maximum dialog hold plus 5 s (3605 s), the wrapper replying null at its own configured hold's end.
@row_check n=440 row=decision/hook-dialog-reply-read-bound sha=e5958264b394 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:72
quote: started + DIALOG_HOLD_MAX + REPLY_GRACE
at: crates/viola-core/src/lib.rs:27
quote: pub const DIALOG_HOLD_MAX: Duration = Duration::from_secs(3600);
@row_add n=441 row=decision/question-option-text-crosses-boundary area=input-validation rec=fct-p4cpzbge@1
name: Question option text crosses into events
rule: A question option's description and preview are taken only as strings, else null, never refusing the dialog, and cross into the events log, hook.dialog, CLI wait --json and the MCP wait.
why: A ratified boundary widening.
@row_check n=441 row=decision/question-option-text-crosses-boundary sha=a521438f3f91 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:154
quote: |o| json!({"description": o["description"].as_str(), "preview": o["preview"].as_str()}),
at: src/cmd/hook.rs:323
quote: params.insert("data".to_owned(), dialog.data.clone());
at: src/run/dialog.rs:287
quote: let mut data = raise.data.clone();
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
@row_add n=442 row=decision/stop-background-tasks-count-only area=input-validation rec=fct-p4cpzbge@1
name: Stop background tasks carried as a count only
rule: Stop's background_tasks is carried only as its length, no element read; null when absent, null or not an array, and a non-array also writes a drift entry.
why: Ratified with the option-text widening.
@row_check n=442 row=decision/stop-background-tasks-count-only sha=2fbb284d3681 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:104
quote: background_tasks: Option<Vec<IgnoredAny>>,
at: crates/viola-agent-claude/src/hook.rs:172
quote: "background_tasks": known.background_tasks.map(|tasks| tasks.len()),
at: crates/viola-agent-claude/src/hook.rs:147-152
quote: let expected = if path == "background_tasks" { "array" } else { "string" }; drift.push(json!({"path": path, "expected": expected}));
@row_add n=443 row=decision/statusline-payload-fails-silent area=input-validation rec=fct-p4cpzbge@1
name: Statusline payload fails silent
rule: The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr, nothing printed and no budget file.
@row_check n=443 row=decision/statusline-payload-fails-silent sha=cc78e97ff20d v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:116-123
quote: io.stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?; if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { reject_stdin("oversize"); return Ok(Outcome::failed("oversize-stdin")); } let Ok(reading) = statusline::reading(&bytes) else { reject_stdin("malformed"); return Ok(Outcome::failed("malformed-json"));
@row_add n=444 row=decision/budget-json-single-writer area=input-validation rec=fct-p4cpzbge@1
name: budget.json has a single writer
rule: budget.json is written only by the statusline hook arm, and only when the payload holds a rate_limits object.
@row_check n=444 row=decision/budget-json-single-writer sha=51af06e5b0e9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: crates/viola-agent-claude/src/statusline.rs:32-33
quote: let Some(Value::Object(limits)) = payload.get("rate_limits") else { return Ok(None);
@row_add n=445 row=contract/budget-json/window area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: window
promise: A window that is absent or not an object is the word "unknown".
@row_check n=445 row=contract/budget-json/window sha=40a2bb388230 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:42-43
quote: let Some(Value::Object(window)) = value else { return Reading::UNKNOWN;
@row_add n=446 row=contract/budget-json/used-percentage area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: used_percentage
promise: Kept only from 0 to 100, else "unknown".
@row_check n=446 row=contract/budget-json/used-percentage sha=a420150a80f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:53-54
quote: Some(used) if (0.0..=100.0).contains(&used) => Reading::Known(used), _ => Reading::UNKNOWN,
@row_add n=447 row=contract/budget-json/resets-at area=input-validation rec=fct-p4cpzbge@1
boundary: budget.json
item: resets_at
promise: Read as epoch seconds or RFC 3339 and written as RFC 3339 UTC with milliseconds; a malformed value becomes "unknown".
@row_check n=447 row=contract/budget-json/resets-at sha=a3e99a8f666e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:62-69
quote: Some(Value::Number(seconds)) => seconds .as_i64() .and_then(|seconds| DateTime::from_timestamp(seconds, 0)), Some(Value::String(text)) => DateTime::parse_from_rfc3339(text) .ok() .map(|at| at.with_timezone(&Utc)), _ => None, };
at: crates/viola-agent-claude/src/statusline.rs:71-74
quote: Some(at) if (0..=9999).contains(&at.year()) => { Reading::Known(at.to_rfc3339_opts(SecondsFormat::Millis, true)) } _ => Reading::UNKNOWN,
@row_add n=448 row=decision/hook-capture-arm area=input-validation rec=fct-p4cpzbge@1
name: Hook capture arm
rule: The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0.
why: A ratified boundary widening.
@row_check n=448 row=decision/hook-capture-arm sha=16a95e2f1548 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:421
quote: if !dir.is_absolute() || !dir.is_dir() {
at: src/cmd/hook.rs:437
quote: match create_private_new(&path) {
at: crates/viola-state/src/fs.rs:58
quote: options.write(true).create_new(true);
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: src/cmd/hook.rs:101-107
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS); }
@row_add n=449 row=decision/statusline-capture-arm area=input-validation rec=fct-p4cpzbge@1
name: Statusline capture arm
rule: The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte, else other, the word itself written nowhere; it prints nothing, exits 0 and opens no channel.
why: Ratified.
@row_check n=449 row=decision/statusline-capture-arm sha=7928688a7aca v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:460
quote: let verdict = if shell == Some(OsStr::new(BIN_SH)) {
at: crates/viola-agent-claude/src/statusline.rs:21
quote: pub const BIN_SH: &str = "/bin/sh";
at: crates/viola-agent-claude/src/ledger.rs:621
quote: Self::BinSh => "bin-sh",
at: src/cmd/hook.rs:475-476
quote: let shell_path = dir.join(statusline_shell_file_name(k)); replace_private(&shell_path, verdict.word().as_bytes(), FILE_MODE).ok()?;
@row_add n=450 row=decision/mcp-handlers-validate-raw-arguments area=input-validation rec=fct-dodlc2ml@1
name: MCP handlers validate raw arguments
rule: MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation.
why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
@row_check n=450 row=decision/mcp-handlers-validate-raw-arguments sha=ec8afb4e311b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:219
quote: self.call(&request.name, request.arguments.unwrap_or_default())
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: crates/viola-mcp/src/tools.rs:290
quote: .and_then(|value| Response::parse(value).ok())
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=451 row=contract/mcp-tool-inputs/invalid-params area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: invalid-params
promise: A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.
@row_check n=451 row=contract/mcp-tool-inputs/invalid-params sha=05eb5f2703f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
at: crates/viola-mcp/src/tools.rs:266
quote: .ok_or("target")
at: crates/viola-mcp/src/tools.rs:270
quote: args.get("text").and_then(Value::as_str).ok_or("text")
at: crates/viola-mcp/src/tools.rs:277
quote: Some(value) => value.as_u64().map(Some).ok_or(field),
at: crates/viola-mcp/src/tools.rs:284
quote: .ok_or("dialog_id")
at: crates/viola-mcp/src/tools.rs:291
quote: .ok_or("response")
@row_add n=452 row=contract/mcp-tool-inputs/unknown-tool area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: unknown tool name
promise: An unknown tool name is JSON-RPC -32602.
@row_check n=452 row=contract/mcp-tool-inputs/unknown-tool sha=30a133f843b7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:233-234
quote: let Some(tool) = ToolName::of(name) else { return Err(ErrorData::invalid_params(
@row_add n=453 row=contract/mcp-tool-inputs/timeout-ms area=input-validation rec=fct-dodlc2ml@1
boundary: MCP tool inputs
item: timeout_ms
promise: wait's timeout_ms is absent, null or a u64, clamped to 50 000 ms, which is also its default.
@row_check n=453 row=contract/mcp-tool-inputs/timeout-ms sha=1e5a6987bdb0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:277
quote: Some(value) => value.as_u64().map(Some).ok_or(field),
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
@row_add n=454 row=decision/mcp-stdin-line-cap-on-reader area=input-validation rec=fct-5zwnjssk@1
name: MCP stdin line cap on the reader
rule: Each MCP stdin line is capped at MAX_FRAME, newline counted, by the reader handed to the SDK; a longer line is refused whole, never truncated or skipped, and ends the session as a stdin close does.
why: rmcp's stdio transport reads each line unbounded and its codec length bound covers only the writer.
@row_check n=454 row=decision/mcp-stdin-line-cap-on-reader sha=63da3b2b73e6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:60-64
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed); return;
@row_add n=455 row=contract/mcp-stdin/oversize-line area=input-validation rec=fct-5zwnjssk@1
boundary: viola mcp stdin
item: oversize line
promise: After the session ends one WARN parse-rejected parser mcp-stdin detail oversize count 1, no byte of the line; exit 0 for an initialized session, exit 1 with both streams empty when cut before initialize completes.
@row_check n=455 row=contract/mcp-stdin/oversize-line sha=b80ac9cbffa7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/lib.rs:80
quote: Err(_) => Err(ServeError::Initialize),
at: crates/viola-mcp/src/lib.rs:86-88
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line(); }
at: crates/viola-mcp/src/lib.rs:98
quote: parser = "mcp-stdin",
at: crates/viola-mcp/src/lib.rs:99
quote: detail = "oversize",
at: src/main.rs:137-138
quote: Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=456 row=decision/home-strict-modes-before-read area=input-validation rec=fct-l7hy7pzk@2
name: Home strict-modes check before any read
rule: --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
@row_check n=456 row=decision/home-strict-modes-before-read sha=f4ac4319193f v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:308
quote: std::fs::canonicalize(home)
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
@row_add n=457 row=decision/verify-home-tree-check area=input-validation rec=fct-l7hy7pzk@2
name: verify checks the home tree first
rule: viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
@row_check n=457 row=decision/verify-home-tree-check sha=a61d8694741d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
at: crates/viola-state/src/strict.rs:95-97
quote: (ledger_dir(home), Rule::Write), (home.join("bin"), Rule::Write), (home.join("plugin"), Rule::Write),
at: crates/viola-state/src/stamps.rs:61-63
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?; let dir = ledger_dir(home); create_private_dir(&dir)?;
@row_add n=458 row=contract/cli-send-text/input area=input-validation rec=fct-l7hy7pzk@2
boundary: viola send text
item: text input
promise: The text comes from stdin or --file only, capped at MAX_FRAME; over the cap or not UTF-8 is exit 2 with one fixed line; a second positional argument is a usage error.
@row_check n=458 row=contract/cli-send-text/input sha=33b5e9b0308c v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:57
quote: reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: src/cmd/send.rs:164-165
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes()); return Ok(ExitCode::from(2));
at: src/cmd/send.rs:39
quote: #[arg(hide = true, value_parser = refuse_text_argument)]
@row_add n=459 row=decision/frame-verbs-instance-check-first area=input-validation rec=fct-l7hy7pzk@2
name: Frame verbs check the instance first
rule: CLI frame verbs and link/unlink run the instance strict-modes check first; a refusal is exit 21 with the instance pair, nothing opened under the home and no line.
@row_check n=459 row=decision/frame-verbs-instance-check-first sha=cfa0e7836969 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
@row_add n=460 row=contract/cli-json-error/strict-modes-failed area=input-validation rec=fct-l7hy7pzk@2
boundary: CLI frame verb --json error
item: strict-modes-failed
promise: A refused instance check under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21.
@row_check n=460 row=contract/cli-json-error/strict-modes-failed sha=9dd94bc0fd5c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=461 row=decision/frame-connect-check-order area=input-validation rec=fct-l7hy7pzk@2
name: Checks before a frame is sent
rule: After the instance check, a CLI frame verb or MCP frame tool runs the liveness pre-check, the endpoint directory check (socket-dir-refused), the connect, the peer euid decision (server-verify-failed) and the server check, all before any frame.
@row_check n=461 row=decision/frame-connect-check-order sha=b675e85b1c78 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:198
quote: let Some(live) = live_endpoint(&instance_dir) else {
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-mcp/src/tools.rs:345
quote: if check_endpoint_dir(&live.endpoint).is_err() {
@row_add n=462 row=decision/mcp-frame-tools-refuse-quietly area=input-validation rec=fct-l7hy7pzk@2
name: MCP frame tool refusals
rule: An MCP frame tool refused by the instance, endpoint directory or peer check returns instance-unreachable, with no frame and no line.
@row_check n=462 row=decision/mcp-frame-tools-refuse-quietly sha=c52465407b22 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=463 row=decision/pause-release-from-viola-name area=input-validation rec=fct-l7hy7pzk@2
name: pause and release take from from VIOLA_NAME
rule: The CLI pause and release verbs take their from only from VIOLA_NAME.
@row_check n=463 row=decision/pause-release-from-viola-name sha=ec2d7fb27a12 v=holds judge=sonnet head=8834195b0683
at: src/cmd/pause.rs:70
quote: if let Some(from) = own_name() {
at: src/cmd/release.rs:39
quote: let code = wheel_call(
at: src/cmd/client.rs:210
quote: let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
@row_add n=464 row=contract/revive-arguments/id area=input-validation rec=fct-l7hy7pzk@2,fct-22lqecbw@1
boundary: viola revive arguments
item: --id
promise: --id accepts only the session-id shape, anything else a usage error with exit 2; --list conflicts with --id, --fork and the trailing arguments.
@row_check n=464 row=contract/revive-arguments/id sha=97a19244b9df v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:30
quote: #[arg(long, value_name = "ID", value_parser = parse_session_id)]
at: src/cmd/revive.rs:47
quote: if is_session_id(raw) {
at: src/cmd/revive.rs:36
quote: #[arg(long, conflicts_with_all = ["id", "fork", "extra"])]
@row_add n=465 row=decision/revive-trailing-words-as-argv area=input-validation rec=fct-l7hy7pzk@2
name: revive trailing words as argv
rule: The words after -- in viola revive go to the child as argv elements of a direct spawn.
@row_check n=465 row=decision/revive-trailing-words-as-argv sha=979165cb84a4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:42-43
quote: #[arg(last = true)] extra: Vec<OsString>,
at: src/cmd/revive.rs:231
quote: child_args.extend(args.extra);
at: src/cmd/run.rs:641
quote: args: launch.args,
@row_add n=466 row=contract/revive-json/error-document area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json
item: error document
promise: A refusal prints exactly {"v":1,"error":"<code>","detail":null} on stdout, stderr empty, exit 1, the code one of the closed revive codes.
@row_check n=466 row=contract/revive-json/error-document sha=b55feaf033e1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
@row_add n=467 row=contract/revive-json/strict-modes-failed area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: strict-modes-failed
promise: A closed exit-1 code of viola revive --json.
@row_check n=467 row=contract/revive-json/strict-modes-failed sha=701514f4295f v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=468 row=contract/revive-json/already-live area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: already-live
promise: A closed exit-1 code of viola revive --json.
@row_check n=468 row=contract/revive-json/already-live sha=78ef5356f785 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=469 row=contract/revive-json/no-session area=input-validation rec=fct-l7hy7pzk@2,fct-22lqecbw@1
boundary: viola revive --json codes
item: no-session
promise: A closed exit-1 code: an --id the log does not hold, or a log with no session, exit 1 before any spawn.
@row_check n=469 row=contract/revive-json/no-session sha=778c1ef4423a v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:250
quote: None => ids.next_back().ok_or(Refusal::NoSession),
at: src/cmd/revive.rs:249
quote: Some(asked) => ids.find(|id| *id == asked).ok_or(Refusal::UnknownId),
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=470 row=contract/revive-json/session-live area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: session-live
promise: A closed exit-1 code of viola revive --json.
@row_check n=470 row=contract/revive-json/session-live sha=b025e777c01e v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=471 row=contract/revive-json/cwd-missing area=input-validation rec=fct-l7hy7pzk@2,fct-q67nxnld@2
boundary: viola revive --json codes
item: cwd-missing
promise: A closed exit-1 code: the recorded cwd is absent or not an existing directory, no child started.
@row_check n=471 row=contract/revive-json/cwd-missing sha=8d7f9c39be9b v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
at: src/cmd/revive.rs:263
quote: .filter(|dir| dir.is_dir()),
@row_add n=472 row=contract/revive-json/stale-heartbeat area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: stale-heartbeat
promise: A closed exit-1 code of viola revive --json.
@row_check n=472 row=contract/revive-json/stale-heartbeat sha=01a86e70416a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=473 row=contract/revive-json/pinned-hash-mismatch area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: pinned-hash-mismatch
promise: A closed exit-1 code of viola revive --json.
@row_check n=473 row=contract/revive-json/pinned-hash-mismatch sha=69eef582b43d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=474 row=contract/revive-json/squatted-name area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: squatted-name
promise: A closed exit-1 code of viola revive --json.
@row_check n=474 row=contract/revive-json/squatted-name sha=ed005a5e68ae v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=475 row=contract/revive-json/socket-dir-refused area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: socket-dir-refused
promise: A closed exit-1 code of viola revive --json.
@row_check n=475 row=contract/revive-json/socket-dir-refused sha=b5f9f6d23119 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=476 row=contract/revive-json/internal-error area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --json codes
item: internal-error
promise: A closed exit-1 code, printed for an internal error of the start path, which is still logged with its chain; a child-pump error prints no document.
@row_check n=476 row=contract/revive-json/internal-error sha=4b48e08b47db v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-124
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error");
@row_add n=477 row=contract/revive-list-json/document area=input-validation rec=fct-l7hy7pzk@2
boundary: viola revive --list --json
item: sessions document
promise: Prints {"v":1,"ok":{"sessions":[{"ts","cause","id"}]}} in log order: codes, closed words and session-shaped ids only, never a path or pid.
@row_check n=477 row=contract/revive-list-json/document sha=09875493caaa v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:290
quote: .map(|row| json!({"ts": row.ts, "cause": row.cause, "id": row.id}))
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
@row_add n=478 row=decision/revive-strict-modes-before-use area=input-validation rec=fct-l7hy7pzk@2
name: revive checks strict modes before use
rule: Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
@row_check n=478 row=decision/revive-strict-modes-before-use sha=d006a6161321 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:137
quote: if !trusted(home, &args.name)? {
at: src/cmd/revive.rs:141
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/revive.rs:272
quote: if !trusted(home, name)? {
@row_add n=479 row=decision/link-unlink-driven-instance area=input-validation rec=fct-l7hy7pzk@2
name: link and unlink act on the driven instance
rule: link and unlink act on the driven instance, joined only from its parsed name and checked before any snapshot read; a driver equal to driven is refused by the wrapper, not the client.
@row_check n=479 row=decision/link-unlink-driven-instance sha=a78254aefdfb v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/link.rs:103
quote: let instance_dir = home.join("instances").join(driven);
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
@row_add n=480 row=decision/cli-home-resolution-order area=input-validation rec=fct-l7hy7pzk@2
name: CLI home resolution order
rule: Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created.
why: A ratified widening of what chooses a CLI verb's home.
except: hook keeps its own shape check.
@row_check n=480 row=decision/cli-home-resolution-order sha=371ecc6e6e6c v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294-302
quote: if let Some(home) = flag { return Ok(home); } if let Some(instance) = instance_of(name, dir) { return Ok(instance.home); } user_home .map(|user_home| default_home(&user_home)) .context("no user home directory")
at: src/cmd/mod.rs:308-310
quote: std::fs::canonicalize(home) .or_else(|_| std::path::absolute(home)) .unwrap_or_else(|_| home.to_path_buf())
at: src/cmd/mod.rs:207
quote: let home = resolved_home(cli.home)?;
@row_add n=481 row=decision/list-scan-checks-before-open area=input-validation rec=fct-l7hy7pzk@2
name: viola list checks before opening
rule: viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
@row_check n=481 row=decision/list-scan-checks-before-open sha=26b3f9001e53 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59-60
quote: if check_if_present(home, check_read).is_err() || check_if_present(&instances_dir, check_path).is_err()
at: crates/viola-state/src/board.rs:97
quote: if check_instance(home, dir).is_err() {
at: crates/viola-state/src/board.rs:77
quote: if kind.is_symlink() {
@row_add n=482 row=decision/config-tolerant-parse area=input-validation rec=fct-nuclnamd@2
name: Tolerant config parse
rule: config.json is parsed tolerantly, unknown keys counted, its v checked and its read capped at MAX_FRAME; a bad value falls back to its default and is reported as parse-rejected.
@row_check n=482 row=decision/config-tolerant-parse sha=118c6bd45d64 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:187
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/obs.rs:198
quote: if map.get("v").and_then(Value::as_u64) != Some(1) {
at: src/obs.rs:206-209
quote: let unknown = map .keys() .filter(|k| !CONFIG_KEYS.contains(&k.as_str())) .count();
at: src/obs.rs:224
quote: parser = "config-json",
@row_add n=483 row=contract/config-json/five-hour area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: five_hour
promise: A JSON whole number 0 to 100, default 90; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_check n=483 row=contract/config-json/five-hour sha=bd62688bd673 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:85
quote: five_hour: threshold(FIVE_HOUR_KEY, FIVE_HOUR_DEFAULT),
at: src/run/budget.rs:92-93
quote: let whole = u8::try_from(value.as_u64()?).ok()?; Percent::try_new(whole).ok()
at: src/run/budget.rs:79-81
quote: Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default)
@row_add n=484 row=contract/config-json/seven-day area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: seven_day
promise: A JSON whole number 0 to 100, default 85; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_check n=484 row=contract/config-json/seven-day sha=be7f9192b2de v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:86
quote: seven_day: threshold(SEVEN_DAY_KEY, SEVEN_DAY_DEFAULT),
at: src/run/budget.rs:31
quote: const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/run/budget.rs:92-93
quote: let whole = u8::try_from(value.as_u64()?).ok()?; Percent::try_new(whole).ok()
@row_add n=485 row=contract/config-json/dialog-hold-s area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: dialog_hold_s
promise: A JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected line; no value makes the hold zero or unbounded.
@row_check n=485 row=contract/config-json/dialog-hold-s sha=3bd9c1a46ce3 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:185-186
quote: let whole = u16::try_from(value.as_u64()?).ok()?; DialogHold::try_new(whole).ok()
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: crates/viola-core/src/lib.rs:277
quote: validate(greater_or_equal = 60, less_or_equal = 3600),
@row_add n=486 row=decision/dialog-hold-config-only area=input-validation rec=fct-nuclnamd@2
name: Dialog hold set only in config
rule: The dialog hold is read once at the wrapper's start from config.json only; no env var or flag sets it.
why: A ratified boundary widening.
@row_check n=486 row=decision/dialog-hold-config-only sha=d5c76ea5b117 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:311
quote: let (hold, rejections) = run::dialog::read_hold(&home);
at: src/run/dialog.rs:161
quote: match File::open(home.join("config.json")) {
@row_add n=487 row=contract/config-json/diagnostics-level area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: diagnostics_level
promise: The closed set info or debug; any other value falls back to info and is reported as parse-rejected.
@row_check n=487 row=contract/config-json/diagnostics-level sha=e7cb8b485a57 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:202-204
quote: None | Some(Some("info")) => (Level::INFO, false), Some(Some("debug")) => (Level::DEBUG, false), Some(_) => (Level::INFO, true),
at: src/obs.rs:211
quote: Some(ConfigRejection::Malformed)
@row_add n=488 row=contract/config-json/claude-env-keep area=input-validation rec=fct-nuclnamd@2
boundary: config.json
item: claude_env_keep
promise: An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.
@row_check n=488 row=contract/config-json/claude-env-keep sha=e41ebacb186e v=holds judge=sonnet head=8834195b0683
at: src/run/env.rs:44
quote: (items.len() <= KEEP_MAX)
at: src/run/env.rs:48
quote: .map(|v| v.as_str().filter(|s| is_claude_name(s)).map(str::to_owned))
at: src/run/env.rs:55
quote: None => (Vec::new(), Some(ConfigRejection::Malformed)),
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
@row_add n=489 row=decision/fake-agent-pump-delay-seam area=input-validation rec=fct-jd7jb3il@1
name: Pump delay test seam
rule: FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction.
why: Lets a test land a host resize in the spawn-to-pump window; ratified.
@row_check n=489 row=decision/fake-agent-pump-delay-seam sha=f37aa08072d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61
quote: #[cfg(feature = "fake-agent")]
at: src/cmd/run.rs:63-65
quote: let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS") .ok() .and_then(|v| v.parse::<u64>().ok());
at: src/cmd/run.rs:67
quote: std::thread::sleep(std::time::Duration::from_millis(ms.min(5_000)));
@row_add n=490 row=decision/fake-agent-hook-panic-seam area=input-validation rec=fct-qyyrikqm@1
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction.
why: Ratified.
@row_check n=490 row=decision/fake-agent-hook-panic-seam sha=f98cae8cd7fc v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/seam.rs:10
quote: panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
at: src/cmd/hook/seam.rs:18
quote: if value.as_deref() == Some(OsStr::new("1")) {
at: src/cmd/hook.rs:128-129
quote: #[cfg(feature = "fake-agent")] seam::panic_if_asked();
@row_add n=491 row=decision/terminal-colour-facts area=input-validation rec=fct-lj2i7oks@2
name: Terminal colour facts read for list only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode, only compared, never echoed, logged or set, and decide only whether and how deep SGR is written around already escaped rows.
why: Ratified.
@row_check n=491 row=decision/terminal-colour-facts sha=fc904785b212 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:96
quote: term: text("TERM"),
at: src/human/style.rs:97
quote: colorterm: text("COLORTERM"),
at: src/cmd/list.rs:64
quote: let colour = human::read_colour(false, false);
@row_add n=492 row=decision/no-width-or-colour-override-env area=input-validation rec=fct-lj2i7oks@2
name: No width or colour override env
rule: COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read; the board's width comes from the host terminal size, no env read.
@row_check n=492 row=decision/no-width-or-colour-override-env sha=688f157a6bea v=holds judge=sonnet head=8834195b0683
searched: "COLUMNS" · in src/**/*.rs and crates/**/*.rs (only a comment, src/cmd/list.rs:61)
searched: "CLICOLOR" · in src/**/*.rs and crates/**/*.rs (none)
searched: "CLICOLOR_FORCE" · in src/**/*.rs and crates/**/*.rs (none)
searched: "CI" · in src/**/*.rs (none; only crates/viola-e2e test harness reads it)
searched: viola_pty::host_size · in src/cmd/list.rs (width at line 63)
@row_add n=493 row=decision/events-reader-bounded-lines area=input-validation rec=fct-wkbf5hxn@1
name: Event log reader bounded lines
rule: The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
@row_check n=493 row=decision/events-reader-bounded-lines sha=0d633b088518 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:118
quote: const LINE_CAP: u64 = MAX_FRAME + 1;
at: crates/viola-state/src/events.rs:236
quote: std::io::Read::take(&mut *reader, self.cap).read_until(b'\n', &mut line)?;
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
@row_add n=494 row=decision/link-restore-trusts-written-names area=input-validation rec=fct-wkbf5hxn@1
name: Link restore trusts names it wrote
rule: Link-set restore uses a link or unlink line's driver and driven only when both are strings and does not parse them as names again; it appends and logs nothing.
why: The wrapper wrote them, each having passed the viola name check.
@row_check n=494 row=decision/link-restore-trusts-written-names sha=d698cb3794ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:82
quote: let (Some(driver), Some(driven)) = (data["driver"].as_str(), data["driven"].as_str()) else {
at: src/run/links.rs:45-46
quote: for line in read_from(&self.instance_dir, 0)? { apply_link_line(&mut restored, &line?.value);
@row_add n=495 row=decision/torn-last-line-healed-on-append area=input-validation rec=fct-wkbf5hxn@1
name: Torn last line healed on append
rule: The next append heals a torn last line by writing one LF ahead of its line in the same single write under the append lock, and logs one state-recovered line with a detail code, basename and offset, no content.
@row_check n=495 row=decision/torn-last-line-healed-on-append sha=2a66b6cfd0ee v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:338-341
quote: ObsEvent::StateRecovered, detail = "torn-line-healed", file = EVENTS, offset = tail.len,
@row_add n=496 row=decision/state-reads-after-strict-modes area=input-validation rec=fct-wkbf5hxn@1
name: State files read after strict modes
rule: Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
@row_check n=496 row=decision/state-reads-after-strict-modes sha=38c3c13de9e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:85
quote: check_instance(&instance.home, &instance.dir).is_ok()
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-state/src/board.rs:97
quote: if check_instance(home, dir).is_err() {
@row_add n=497 row=decision/replayed-snapshot-fallbacks area=input-validation rec=fct-wkbf5hxn@1
name: Replayed snapshot fallbacks
rule: Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, statusline prints nothing, and the collision check judges liveness by the heartbeat alone.
@row_check n=497 row=decision/replayed-snapshot-fallbacks sha=13438efb89ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook.rs:213
quote: Self::Unreachable => "channel-unreachable",
at: src/cmd/hook/statusline.rs:129-131
quote: let command = match read_snapshot_or_replay(&instance.dir) { Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command, _ => None,
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=498 row=decision/classified-snapshot-read-only area=input-validation rec=fct-wkbf5hxn@1
name: Classified snapshot read only
rule: No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
@row_check n=498 row=decision/classified-snapshot-read-only sha=9fd8781bfccb v=holds judge=sonnet head=8834195b0683
searched: read_snapshot( · in src/**/*.rs outside test modules (every hit sits after its file's #[cfg(test)]: run.rs:1259, dialog.rs:646, links.rs:188, budget.rs:726, wheel.rs, snapshot.rs, send/slot.rs:458)
searched: read_snapshot( · in crates/**/*.rs outside test modules (only the definition crates/viola-state/src/snapshot.rs:150 and test uses)
@row_add n=499 row=decision/claude-agents-read-fail-open area=input-validation rec=fct-cysd5aat@1
name: claude agents read bounded, fail open
rule: The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
@row_check n=499 row=decision/claude-agents-read-fail-open sha=2028c3fe907c v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
at: src/run/agents.rs:53
quote: VERSION_DEADLINE,
at: src/run/version_gate.rs:22
quote: pub(crate) const VERSION_DEADLINE: Duration = Duration::from_secs(5);
at: src/run/agents.rs:114
quote: let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
at: crates/viola-agent-claude/src/agents.rs:90-94
quote: if bytes.len() as u64 > MAX_FRAME { return Err(AgentsFailure::Oversize); } let Ok(Value::Array(elements)) = serde_json::from_slice::<Value>(bytes) else { return Err(AgentsFailure::Malformed);
at: src/cmd/list.rs:121-127
quote: if let Ok(list) = std::env::current_dir() .ok() .ok_or(()) .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ())) { board::join(&mut board, observations(list)); }
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=500 row=decision/unwrapped-session-name-not-target area=input-validation rec=fct-cysd5aat@1,fct-q67nxnld@2
name: Unwrapped session name never a target
rule: An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
@row_check n=500 row=decision/unwrapped-session-name-not-target sha=59283a39386a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:193
quote: name: observation.session_id,
at: crates/viola-state/src/board.rs:194
quote: wrapped: false,
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: src/cmd/list.rs:84
quote: let (board, budget, _) = read(home, &scan, true);
@row_add n=501 row=decision/verify-pty-output-caught area=input-validation rec=fct-cysd5aat@1
name: verify PTY output bounded and caught
rule: verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
@row_check n=501 row=decision/verify-pty-output-caught sha=72ac49b7f7d8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/pty.rs:100
quote: if self.total > MAX_FRAME {
at: src/cmd/verify/typed/pty.rs:106
quote: if catch_unwind(AssertUnwindSafe(|| self.screen.feed(bytes, now))).is_err() {
at: src/run/gate.rs:91
quote: sigs: Option<&'static Signatures>,
@row_add n=502 row=decision/vt100-feed-under-panic-catch area=input-validation rec=fct-g433mics@2
name: Screen feed under a panic catch
rule: The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues.
why: vt100 panics are reachable at real small sizes.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_check n=502 row=decision/vt100-feed-under-panic-catch sha=de5fe964585a v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/gate.rs:46-49
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() { self.screen.poison(); self.panicked = true; parse_rejected("panicked");
at: src/run/gate.rs:57-58
quote: self.panicked = false; self.overflowed = false;
at: crates/viola-agent-claude/src/screen.rs:139-140
quote: let Some(parser) = &self.parser else { return GateStep::Done(Readiness::InputNotReady);
at: src/run/gate.rs:135
quote: self.feed.offer(Feed::Bytes(bytes[..written].to_vec()));
guard_at: tests/chaos_feed_panic.rs:44
@row_add n=503 row=decision/vt100-feed-bounded-queue area=input-validation rec=fct-g433mics@2
name: Screen feed bounded queue
rule: Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
@row_check n=503 row=decision/vt100-feed-bounded-queue sha=35d1432a368b v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:23
quote: pub(crate) const FEED_CAPACITY: usize = 256;
at: src/run/gate.rs:109
quote: if let Err(TrySendError::Full(_)) = self.tx.try_send(message) {
at: src/run/gate.rs:134-135
quote: let written = self.output.write(bytes)?; self.feed.offer(Feed::Bytes(bytes[..written].to_vec()));
at: src/run/gate.rs:69
quote: self.screen.poison();
@row_add n=504 row=decision/claude-agents-resolved-from-path area=input-validation rec=fct-q67nxnld@2
name: claude agents resolved from the caller's PATH
rule: list, revive's preflight and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME.
why: Same-user trust, an accepted risk; the MCP widening ratified.
@row_check n=504 row=decision/claude-agents-resolved-from-path sha=6fef78ba27ae v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:110
quote: let Ok(program) = super::resolve_program(OsStr::new(PROGRAM), cwd) else {
at: src/run/mod.rs:143
quote: let path = std::env::var_os("PATH");
at: src/cmd/list.rs:121-124
quote: if let Ok(list) = std::env::current_dir() .ok() .ok_or(()) .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
at: src/cmd/revive.rs:207
quote: let cwd = std::env::current_dir()?;
at: src/cmd/list.rs:84
quote: let (board, budget, _) = read(home, &scan, true);
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
@row_add n=505 row=decision/run-resolves-absolute-program area=input-validation rec=fct-q67nxnld@2
name: run resolves its child program itself
rule: viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
@row_check n=505 row=decision/run-resolves-absolute-program sha=1626351b5507 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:192-195
quote: if given.is_absolute() || given.components().count() > 1 { Ok(cwd.join(given)) } else { find_on_path(program, search, is_file).ok_or(AgentError::ProgramNotFound)
at: src/run/mod.rs:147
quote: viola_agent_claude::resolve_program(program, cwd, search, &|p: &Path| p.is_file())
@row_add n=506 row=decision/revive-recorded-cwd-spawn-dir area=input-validation rec=fct-q67nxnld@2,fct-wkbf5hxn@1
name: revive spawns in the recorded cwd
rule: viola revive spawns claude in the snapshot's recorded cwd, read only after the strict-modes check, used as the spawn directory and for nothing else, and required to exist; program resolution and the version gate never read it.
why: A ratified boundary widening.
@row_check n=506 row=decision/revive-recorded-cwd-spawn-dir sha=a59541e53877 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:259-263
quote: Recovered::Snapshot(snapshot) => snapshot .cwd .as_deref() .map(PathBuf::from) .filter(|dir| dir.is_dir()),
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
at: src/cmd/revive.rs:234
quote: program: OsString::from(PROGRAM),
at: src/cmd/revive.rs:236
quote: spawn_dir,
@row_add n=507 row=decision/session-id-argv-closed-shape area=input-validation rec=fct-22lqecbw@1
name: Session id on child argv
rule: A session id placed on a child's command line has the closed shape (36 characters, hyphens at 8, 13, 18, 23, hex elsewhere) and is one argv element of a direct spawn, never a shell, built only by one resume-args builder.
why: A ratified crossing of a logged value onto a child's command line.
@row_check n=507 row=decision/session-id-argv-closed-shape sha=e70048f4621e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:97-104
quote: id.len() == 36 && id.bytes().enumerate().all(|(at, byte)| { if matches!(at, 8 | 13 | 18 | 23) { byte == b'-' } else { byte.is_ascii_hexdigit() } })
at: crates/viola-agent-claude/src/lib.rs:110
quote: let mut args = vec![OsString::from(RESUME_FLAG), OsString::from(id)];
at: src/cmd/revive.rs:230
quote: let mut child_args = resume_args(&id, args.fork);
at: src/cmd/verify/typed/runs.rs:194
quote: args.extend(viola_agent_claude::resume_args(id, false));
@row_add n=508 row=decision/verify-run-e-resume-target area=input-validation rec=fct-22lqecbw@1
name: verify Run E resume target
rule: verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails.
why: Ratified.
@row_check n=508 row=decision/verify-run-e-resume-target sha=d4842a3fcc84 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:884-889
quote: pub fn resume_target(trusted: &[Capture]) -> Option<&str> { trusted .iter() .find(|c| c.event == HookEvent::SessionStart) .and_then(|c| c.field("session_id")) .filter(|id| crate::is_session_id(id))
at: src/cmd/verify/typed/runs.rs:187-189
quote: let Some(id) = ledger::resume_target(&trusted) else { return Ok(()); };
at: src/cmd/verify/typed/runs.rs:194
quote: args.extend(viola_agent_claude::resume_args(id, false));
at: crates/viola-agent-claude/src/ledger.rs:899-900
quote: let Some(target) = resume_target(trusted) else { return false;
@row_add n=509 row=decision/refusals-hold-no-paths-pids-ids area=input-validation rec=fct-22lqecbw@1
name: Refusals hold no paths, pids or ids
rule: No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
@row_check n=509 row=decision/refusals-hold-no-paths-pids-ids sha=f1f5b9755242 v=holds judge=sonnet head=8834195b0683
searched: Refusal::pair texts (cwd, pid, id interpolation) · in src/cmd/revive.rs:79-102 (only the instance name, PROGRAM, RESUME_FLAG and the literal placeholder <id> are interpolated)
searched: is_session_id · in src/cmd/revive.rs:321 (the --list rows are filtered to the session-id shape: .filter(|link| is_session_id(&link.agent_session_id)))
@row_add n=510 row=decision/statusline-source-selection area=input-validation rec=fct-xqkexewr@1
name: Statusline source selection
rule: viola run reads the statusline command from the home's statusline-source.json when it exists, else only for the default home the user's Claude settings; a command only from a non-empty command-type statusLine with no NUL; viola writes neither source.
why: Ratified, the file in the viola home chosen.
@row_check n=510 row=decision/statusline-source-selection sha=57a355c91f25 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:533-535
quote: let own = home.join(STATUSLINE_SOURCE); if own.exists() { return Some(own);
at: src/cmd/run.rs:538-539
quote: let default = super::canonical_home(&super::default_home(user_home)); (default == home).then(|| {
at: crates/viola-agent-claude/src/statusline.rs:83
quote: if line.get("type")?.as_str()? != "command" {
at: crates/viola-agent-claude/src/statusline.rs:87
quote: (!command.is_empty() && !command.contains('\0')).then(|| command.to_owned())
at: src/cmd/run.rs:552-554
quote: File::open(source) .ok()? .take(MAX_FRAME)
@row_add n=511 row=decision/statusline-shell-out area=input-validation rec=fct-m6zdbeoz@3
name: Statusline shell-out
rule: The recorded statusline command, the only shell-out, runs only in hook statusline as /bin/sh -c with the command as one argument; stderr is discarded, stdout capped and printed byte for byte, never parsed, and no log line carries the command.
why: A ratified boundary widening.
@row_check n=511 row=decision/statusline-shell-out sha=01f32f8e06d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:124
quote: [BIN_SH, "-c", command]
at: crates/viola-agent-claude/src/statusline.rs:21
quote: pub const BIN_SH: &str = "/bin/sh";
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
at: src/cmd/hook/statusline.rs:228
quote: .stderr(Stdio::null())
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:200
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
@row_add n=512 row=decision/statusline-deadline area=input-validation rec=fct-m6zdbeoz@3
name: Statusline command deadline
rule: The wait for the statusline command is bounded at 5 s (detail deadline); a failing command prints nothing.
state: provisional
@row_check n=512 row=decision/statusline-deadline sha=262899a1aecf v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
at: src/cmd/hook/statusline.rs:147
quote: Some(Shell::Exited { .. } | Shell::NotStarted) | None => None,
@row_add n=513 row=decision/max-frame-shared-cap area=input-validation rec=fct-jy3jkgu2@1
name: One shared MAX_FRAME cap
rule: One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps.
why: Sized above the largest expected permission input (Write-tool file bodies).
@row_check n=513 row=decision/max-frame-shared-cap sha=3e34d1948c98 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:11
quote: pub const MAX_FRAME: u64 = 16 * 1024 * 1024;
at: crates/viola-channel/src/frame.rs:18
quote: read_frame_within(reader, MAX_FRAME)
at: src/cmd/hook.rs:178
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
at: src/cmd/hook/statusline.rs:116
quote: io.stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
at: src/cmd/run.rs:554
quote: .take(MAX_FRAME)
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/cmd/answer.rs:49
quote: reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: src/cmd/send.rs:57
quote: reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: src/run/version_gate.rs:220
quote: let cap = MAX_FRAME + 1;
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
at: crates/viola-state/src/events.rs:118
quote: const LINE_CAP: u64 = MAX_FRAME + 1;
at: src/obs.rs:187
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: crates/viola-state/src/stamps.rs:32
quote: file.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
@row_add n=514 row=decision/max-frame-checked-against-ledger area=input-validation rec=fct-jy3jkgu2@1
name: MAX_FRAME checked against data
rule: verify records the largest hook payload per event kind, stamped per CLI version; a stamped maximum within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_check n=514 row=decision/max-frame-checked-against-ledger sha=c07f95b54712 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1142
quote: "largest_hook_payload": measured,
at: src/cmd/verify.rs:272
quote: let largest = ledger::largest(&probes.print.captures);
@row_add n=515 row=decision/bounded-readers-census area=input-validation rec=fct-jy3jkgu2@1
name: Every product reader bounded or named
rule: Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness.
guard: product_readers_are_each_bounded_or_named
except: Reads of viola's own executable for the pinned copy are not external input.
@row_check n=515 row=decision/bounded-readers-census sha=97f227872f50 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_bounded_readers.rs:690
quote: fn product_readers_are_each_bounded_or_named() {
at: tests/contract_bounded_readers.rs:29
quote: const BOUNDARIES: &[Boundary] = &[
at: tests/contract_bounded_readers.rs:394
quote: const EXCEPTIONS: &[(&str, &str, &str)] = &[
guard_at: tests/contract_bounded_readers.rs:690
@row_add n=516 row=decision/inherited-credentials-never-serialised area=secret-management rec=fct-fdqq3i6o@2
name: Inherited credentials never serialised
rule: Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
@row_check n=516 row=decision/inherited-credentials-never-serialised sha=7420dc89a76a v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:44
quote: /// The removed names, sorted and comma-joined; names only, never values.
at: src/cmd/run.rs:644
quote: env_remove: strip.remove.clone(),
@row_add n=517 row=decision/r8-claude-env-strip area=secret-management rec=fct-fdqq3i6o@2
name: R8 CLAUDE environment strip
rule: Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values.
why: Ratified.
@row_check n=517 row=decision/r8-claude-env-strip sha=b3076632ebb6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:69-77
quote: if !text.starts_with("CLAUDE") { continue; } let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) { plan.kept.push(text); } else { plan.remove.push(name); }
@row_add n=518 row=decision/identity-floor-eleven-names area=secret-management rec=fct-fdqq3i6o@2
name: Identity floor stays eleven names
rule: The identity floor stays the eleven names; the plugin, project and env-file names are not identity names.
why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
@row_check n=518 row=decision/identity-floor-eleven-names sha=be2b31120d82 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
@row_add n=519 row=contract/claude-child-process-start/env-stripped-known area=secret-management rec=fct-fdqq3i6o@2
boundary: process-start claude-child line
item: env_stripped_known
promise: Names the stripped environment names, never a value.
@row_check n=519 row=contract/claude-child-process-start/env-stripped-known sha=6dee394cc5bb v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:46
quote: joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
@row_add n=520 row=contract/claude-child-process-start/env-kept area=secret-management rec=fct-fdqq3i6o@2
boundary: process-start claude-child line
item: env_kept
promise: Names the kept environment names, never a value.
@row_check n=520 row=contract/claude-child-process-start/env-kept sha=8d073f2bb2e6 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:59
quote: env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:50
quote: joined(self.kept.iter().cloned())
@row_add n=521 row=decision/pre-push-linux-only area=secret-management rec=fct-gecdpdpp@1
name: Pre-push gate Linux only
rule: The local pre-push gate runs only on a Linux host; elsewhere it exits 2 with pre-push-linux-only.
@row_check n=521 row=decision/pre-push-linux-only sha=6f6238def61a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
@row_add n=522 row=decision/pre-push-empty-environment-launch area=secret-management rec=fct-gecdpdpp@1
name: Pre-push children launched with an empty environment
rule: The pre-push gate launches every child natively with an emptied environment carrying exactly HOME (from the passwd entry, never its own $HOME) and PATH of constant dirs, so no host value crosses; it has no other launcher and no uid-0 launch.
why: Keeps inherited CLAUDE* credentials out of every child.
except: A host-valued assignment is a boundary widening needing its own Decisions Log entry.
@row_check n=522 row=decision/pre-push-empty-environment-launch sha=72ab26defb72 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39
quote: cmd.arg("-i")
at: crates/viola-e2e/src/harness/pre_push/linux.rs:92
quote: let entry = out(runner, probe(&["getent", "passwd", uid])).ok_or_else(missing)?;
@row_add n=523 row=decision/server-verified-before-any-frame area=security-anti-patterns rec=fct-izf47ccx@2
name: Server verified before any channel frame
rule: No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
@row_check n=523 row=decision/server-verified-before-any-frame sha=ba4b927fe944 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180-183
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid()); match verdict { ServerVerdict::Matches => Ok(client), ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
@row_add n=524 row=decision/snapshot-strict-checked-before-trust area=security-anti-patterns rec=fct-izf47ccx@2
name: Snapshot reference values only after strict-modes check
rule: The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
@row_check n=524 row=decision/snapshot-strict-checked-before-trust sha=ca4312482358 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/send.rs:198
quote: let Some(live) = live_endpoint(&instance_dir) else {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: crates/viola-mcp/src/tools.rs:339-342
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes)); } let Some(live) = live_endpoint(&instance_dir) else {
@row_add n=525 row=decision/test-removal-own-dirs-only area=security-anti-patterns rec=fct-y573epy2@1
name: Test-side removal only of test-created dirs
rule: A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
@row_check n=525 row=decision/test-removal-own-dirs-only sha=b40f19c317e8 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:72-75
quote: if process_start(pid) == Some(started_at) { continue; } if remove_owned(&dir) {
at: tests/support/home.rs:198-202
quote: let dir = tempfile::Builder::new() .prefix("viola-test-") .tempdir_in(base) .expect("tempdir"); write_owner(dir.path());
at: crates/viola-e2e/src/harness/cleanup.rs:161
quote: .is_some_and(|n| n.starts_with("viola-session-"));
@row_add n=526 row=decision/e2e-home-link-guard area=security-anti-patterns rec=fct-y573epy2@1
name: E2E home linked base guard
rule: The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing.
why: a ratified boundary widening: on the dev host the path links outside the working directory
@row_check n=526 row=decision/e2e-home-link-guard sha=e0d1ff580e0d v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:128
quote: let _ = fs::DirBuilder::new().mode(0o700).create(&target);
at: tests/support/home.rs:131
quote: meta.is_dir(),
at: tests/support/home.rs:135
quote: meta.permissions().mode() & 0o077 == 0,
at: tests/support/home.rs:138
quote: if let Some(refused) = headroom_refusal(free_bytes(&target), HEADROOM_FLOOR_MIB) {
at: crates/viola-e2e/src/harness/mod.rs:108
quote: if !target.is_absolute() {
at: crates/viola-e2e/src/harness/mod.rs:113
quote: let _ = fs::DirBuilder::new().mode(0o700).create(&target);
at: crates/viola-e2e/src/harness/mod.rs:115
quote: if !meta.is_dir() {
at: crates/viola-e2e/src/harness/mod.rs:120
quote: if meta.permissions().mode() & 0o077 != 0 {
at: crates/viola-e2e/src/harness/mod.rs:125
quote: if let Some(low) = headroom_refusal(free(&target)?, HEADROOM_FLOOR_MIB) {
@row_add n=527 row=contract/cli-exit-codes/12 area=security-anti-patterns rec=fct-zvkglgpg@2
boundary: CLI exit codes
item: 12
promise: answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.
@row_check n=527 row=contract/cli-exit-codes/12 sha=d3a31336c70f v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/run/version_gate.rs:171-172
quote: let cli_verified = cli_version.as_deref().is_some_and(|version| { let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
@row_add n=528 row=decision/stamps-strict-check-first area=security-anti-patterns rec=fct-b7o2ag2e@1
name: Stamps touched only after strict-modes check
rule: viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
@row_check n=528 row=decision/stamps-strict-check-first sha=e0ecf20e1f9b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:61-63
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?; let dir = ledger_dir(home); create_private_dir(&dir)?;
at: crates/viola-state/src/stamps.rs:66-68
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?; let path = dir.join(STAMPS); let current = read_capped(&path)?.filter(|(_, over)| !over);
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
@row_add n=529 row=decision/strict-check-before-narrowing area=security-anti-patterns rec=fct-b7o2ag2e@1
name: Strict-modes check before any narrowing
rule: The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks.
why: a widened directory narrowed to 0700 first would pass the check
@row_check n=529 row=decision/strict-check-before-narrowing sha=18ee50719f9d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-73
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1)); } if let Some(name) = instance { let (level, rejection) = obs::read_diagnostics_level(&home); obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
at: crates/viola-state/src/stamps.rs:61-64
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?; let dir = ledger_dir(home); create_private_dir(&dir)?; let lock = open_private_lock(&dir.join(STAMPS_LOCK))?;
at: src/cmd/hook.rs:116-122
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS); }; let (level, _) = obs::read_diagnostics_level(&instance.home); // A role file that will not open leaves no subscriber, so every line goes nowhere and the hook
@row_add n=530 row=decision/no-control-switch-off area=security-anti-patterns rec=fct-bg5w767t@2
name: No switch turns off a control
rule: No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation.
why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence
except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
@row_check n=530 row=decision/no-control-switch-off sha=f7bebafb32c3 v=holds judge=sonnet head=8834195b0683
searched: FAKE_AGENT_ · in src/**/*.rs (only src/cmd/run.rs:63 PUMP_DELAY_MS under cfg(feature = "fake-agent"), src/cmd/hook/seam.rs:10 HOOK_PANIC under cfg(feature = "fake-agent"); neither switches off a control)
searched: env::var · in src/**/*.rs and crates/*/src/**/*.rs (only VIOLA_NAME, VIOLA_DIR (choose home/instance), PATH, NO_COLOR/TERM/COLORTERM in src/human/style.rs:95, XDG_RUNTIME_DIR, harness AGENT_RUN_* keys; none disables the socket-dir check, peer/server verification, strict-modes check or validate_paste_text)
searched: peer_admitted · in src/**/*.rs and crates/*/src/**/*.rs (calls at src/cmd/client.rs:177, src/cmd/hook.rs:236, crates/viola-mcp/src/tools.rs:352 carry no condition on config, env or flag)
@row_add n=531 row=decision/test-seams-fake-agent-only area=security-anti-patterns rec=fct-bg5w767t@2
name: Test seams only under the fake-agent feature
rule: An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
@row_check n=531 row=decision/test-seams-fake-agent-only sha=eb7ded47b60c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61-63
quote: #[cfg(feature = "fake-agent")] fn hold_pump_start() { let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS")
at: src/cmd/hook/seam.rs:8-10
quote: #[cfg(feature = "fake-agent")] pub(super) fn panic_if_asked() { panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
at: src/cmd/hook.rs:128-129
quote: #[cfg(feature = "fake-agent")] seam::panic_if_asked();
@row_add n=532 row=decision/colour-facts-choose-sgr-only area=security-anti-patterns rec=fct-bg5w767t@2
name: Terminal colour facts choose SGR only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode to choose whether and how deep SGR bytes are written; they change no control, widen no redaction and reach no log line, error or refusal.
why: ratified
@row_check n=532 row=decision/colour-facts-choose-sgr-only sha=8920116e7b51 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:96-97
quote: term: text("TERM"), colorterm: text("COLORTERM"),
at: src/cmd/list.rs:64
quote: let colour = human::read_colour(false, false);
@row_add n=533 row=decision/no-seam-forcing-peer-refusal area=security-anti-patterns rec=fct-lvhmwli7@2
name: No seam to force a peer-euid refusal
rule: No env, flag or feature seam forces a refusing peer-euid decision; the decisions are proven over injected values and each client seam's wiring stays the named weakest proof.
why: one account can make no foreign peer
@row_check n=533 row=decision/no-seam-forcing-peer-refusal sha=613e27700723 v=holds judge=sonnet head=8834195b0683
searched: peer_admitted · in src/**/*.rs and crates/*/src/**/*.rs (client.rs:144 reads own_euid() only; the three seams call it unconditionally)
searched: serve_admitting · in crates/viola-channel/src/server.rs (private fn, line 72; public path at line 69 passes peer_admitted, a second use only in the unit-test at line 1043 over an injected Admission)
searched: VIOLA_.*EUID|FORCE_REFUS|PEER_EUID · in src/**/*.rs and crates/*/src/**/*.rs (no env, flag or feature reads a forced peer euid; the only env reads are listed under row 7)
@row_add n=534 row=decision/statusline-only-shell-out area=threat-model-summary rec=fct-33o4e7zi@2
name: Statusline command is the only shell-out
rule: The user-written statusline command run through a shell is the only shell-out in viola, and the per-session settings override holds the pinned hook command, never the user's command.
@row_check n=534 row=decision/statusline-only-shell-out sha=e2aa6eaa6d75 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:124
quote: [BIN_SH, "-c", command]
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
at: crates/viola-agent-claude/src/statusline.rs:98
quote: let command = format!("{pinned_bin_fwd} hook statusline");
@row_add n=535 row=decision/parent-identity-env-stripped area=threat-model-summary rec=fct-jy2pv6x7@1
name: Parent identity variables stripped from the child
rule: The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
@row_check n=535 row=decision/parent-identity-env-stripped sha=f1d1f9e592f0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:69
quote: if !text.starts_with("CLAUDE") {
at: crates/viola-agent-claude/src/lib.rs:23-24
quote: "CLAUDE_CODE_MESSAGING_SOCKET", "CLAUDE_CODE_MESSAGING_TOKEN",
at: src/run/version_gate.rs:70-71
quote: for name in &strip.remove { command.env_remove(name);
@row_add n=536 row=absence/model-api-calls area=threat-model-summary rec=fct-jy2pv6x7@1
capability: Model API calls of its own; the subscription credentials belong to the unmodified claude CLI
@row_check n=536 row=absence/model-api-calls sha=c29d0887b262 v=holds judge=sonnet head=8834195b0683
searched: reqwest|ureq|TcpStream|api.anthropic · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=537 row=absence/secrets area=threat-model-summary rec=fct-jy2pv6x7@1,fct-v2lh3e2i@3
capability: Secrets of any kind in v1
@row_check n=537 row=absence/secrets sha=8006db341ada v=holds judge=sonnet head=8834195b0683
searched: secrets|GITHUB_TOKEN|token (case-insensitive) · in .github/workflows
searched: encrypt|keyring|crypto (case-insensitive) · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=538 row=decision/spawn-cwd-snapshot-only area=threat-model-summary rec=fct-qgluq6ji@1
name: Spawn directory kept in the snapshot only
rule: The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent.
why: an absolute host path can carry the OS username
@row_check n=538 row=decision/spawn-cwd-snapshot-only sha=b072274a8f56 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:610
quote: cwd: recorded.spawn_dir.to_str().map(str::to_owned),
at: crates/viola-state/src/snapshot.rs:64
quote: pub cwd: Option<String>,
@row_add n=539 row=decision/same-user-trust-boundary area=threat-model-summary rec=fct-t7qeqkiu@2
name: Same OS user trust boundary
rule: The wrapper channel trusts every process of the same OS user and no other; there is no per-connection authentication and the caller's self-reported from is not authenticated.
@row_check n=539 row=decision/same-user-trust-boundary sha=c896d7a410cb v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:144-145
quote: pub fn peer_admitted(observed: Option<u32>) -> bool { peer_euid_admitted(observed, own_euid())
at: crates/viola-channel/src/server.rs:69
quote: self.serve_admitting(dispatch, peer_admitted)
@row_add n=540 row=decision/socket-in-owner-only-dir area=threat-model-summary rec=fct-t7qeqkiu@2
name: Socket in a per-user owner-only directory
rule: The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
@row_check n=540 row=decision/socket-in-owner-only-dir sha=5a799e36b979 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:137
quote: pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> {
at: src/cmd/run.rs:511
quote: if check_endpoint_dir(endpoint).is_err() {
@row_add n=541 row=decision/exclusive-endpoint-bind area=threat-model-summary rec=fct-t7qeqkiu@2
name: Exclusive endpoint bind
rule: viola run binds its endpoint exclusively, so a squatted name blocks startup with exit 1.
@row_check n=541 row=decision/exclusive-endpoint-bind sha=bbd6f56f9083 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:112-114
quote: match lock.try_lock() { Ok(()) => {} Err(TryLockError::WouldBlock) => return Err(ChannelError::BindTaken),
@row_add n=542 row=contract/wrapper-channel-methods/send area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: send
promise: Types a prompt into the driven Claude session.
@row_check n=542 row=contract/wrapper-channel-methods/send sha=dd4a0c056d9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:140
quote: "send" => send::send(&self.send, &self.name, &self.instance_dir, call),
at: src/run/send.rs:148
quote: if paste(text).is_err() {
@row_add n=543 row=contract/wrapper-channel-methods/wait area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: wait
promise: Reads the driven session's conversation.
@row_check n=543 row=contract/wrapper-channel-methods/wait sha=6bcae06def3c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:141
quote: "wait" => self.wait.wait(&self.instance_dir, call.params),
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
@row_add n=544 row=contract/wrapper-channel-methods/last area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: last
promise: Reads the driven session's latest reply.
@row_check n=544 row=contract/wrapper-channel-methods/last sha=16a457b581af v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:142
quote: "last" => self.wait.last(call.params),
at: src/run/wait.rs:177
quote: Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
@row_add n=545 row=contract/wrapper-channel-methods/answer area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: answer
promise: Answers a dialog, including allowing a PermissionRequest with behavior allow.
@row_check n=545 row=contract/wrapper-channel-methods/answer sha=3d8f97971c2f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:144
quote: "answer" => self.dialogs.answer(call),
at: crates/viola-agent-claude/src/dialog.rs:222-223
quote: Some("allow") => Ok(Self::Permission { behavior: Permit::Allow,
@row_add n=546 row=contract/wrapper-channel-methods/pause area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: pause
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_check n=546 row=contract/wrapper-channel-methods/pause sha=9dbcaa0c4682 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:145
quote: "pause" => self.wheel.pause(call),
@row_add n=547 row=contract/wrapper-channel-methods/release area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: release
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_check n=547 row=contract/wrapper-channel-methods/release sha=b8e9c1efeb43 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:146
quote: "release" => self.wheel.release(call, &self.budget),
@row_add n=548 row=contract/wrapper-channel-methods/link area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: link
promise: Adds a driver link, whose effect is a link line in the event log and the snapshot's links only; the self-reported from is not read.
@row_check n=548 row=contract/wrapper-channel-methods/link sha=e33d82832d40 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:147
quote: "link" => self.links.link(call.params),
at: src/run/links.rs:85-86
quote: /// A method's `driver`, once both names are valid, `driven` is this instance and `driver` is /// another one; `from` is never read.
@row_add n=549 row=contract/wrapper-channel-methods/unlink area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: unlink
promise: Removes a driver link, whose effect is an unlink line in the event log and the snapshot's links only; the self-reported from is not read.
@row_check n=549 row=contract/wrapper-channel-methods/unlink sha=1888ecefcab0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:148
quote: "unlink" => self.links.unlink(call.params),
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
@row_add n=550 row=contract/wrapper-channel-methods/hook-dialog area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: hook.dialog
promise: Carries a dialog from the hook to the wrapper and its decision back.
@row_check n=550 row=contract/wrapper-channel-methods/hook-dialog sha=c83837fedb61 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:143
quote: "hook.dialog" => self.dialogs.hook_dialog(call.params),
at: src/cmd/hook.rs:328
quote: match client.request_until("hook.dialog", params, reply_by) {
@row_add n=551 row=contract/wrapper-channel-methods/hook-event area=threat-model-summary rec=fct-t7qeqkiu@2
boundary: wrapper channel methods
item: hook.event
promise: A notification from the hook to the wrapper.
@row_check n=551 row=contract/wrapper-channel-methods/hook-event sha=cc7e965c7a5f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:120
quote: if method != "hook.event" {
at: src/cmd/hook.rs:570
quote: .notify("hook.event", params)
@row_add n=552 row=decision/client-reads-closed-values-only area=threat-model-summary rec=fct-t7qeqkiu@2
name: Client screens the server's answer to closed values
rule: A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result.
except: a code word outside the closed list stays visible by design
@row_check n=552 row=decision/client-reads-closed-values-only sha=cbd01704a594 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:70
quote: return sanitised_fault(error).map(Reply::Fault);
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
at: crates/viola-mcp/src/result.rs:60
quote: return sanitised_fault(fault)
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
@row_add n=553 row=decision/bridge-send-one-bracketed-paste area=threat-model-summary rec=fct-ql7nqspi@1
name: Bridge send is one bracketed paste
rule: A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
@row_check n=553 row=decision/bridge-send-one-bracketed-paste sha=67fadf497d07 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:79-81
quote: bytes.extend_from_slice(b"\x1b[200~"); bytes.extend_from_slice(text.as_bytes()); bytes.extend_from_slice(b"\x1b[201~\r");
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send.rs:91
quote: let typed = typed_text(text);
@row_add n=554 row=decision/delivery-confirmation-exact-match area=threat-model-summary rec=fct-ql7nqspi@1
name: Delivery confirmed by exact equality
rule: Delivery confirmation compares the submitted prompt text with the typed text by exact equality, with no tolerance on the submitted side.
@row_check n=554 row=decision/delivery-confirmation-exact-match sha=59eb5f67bbd2 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:91
quote: && f.text == text =>
@row_add n=555 row=decision/stdin-closed-non-editing-list area=threat-model-summary rec=fct-zyxi3lxw@1
name: Stdin classifier's closed non-editing list
rule: Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess.
why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
@row_check n=555 row=decision/stdin-closed-non-editing-list sha=50989bf723e4 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:520
quote: (Some(b'?'), [], b'n') => numbers == b"997;1" || numbers == b"997;2",
at: src/run/wheel.rs:524
quote: (Some(b'>'), [], b'm') => fields == 2 && numeric && first == b"4",
at: src/run/wheel.rs:525
quote: _ => false,
@row_add n=556 row=contract/send-refusal/human-typing area=threat-model-summary rec=fct-zyxi3lxw@1
boundary: send refusal
item: human-typing
promise: A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
@row_check n=556 row=contract/send-refusal/human-typing sha=145903b6a0d9 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
@row_add n=557 row=contract/wheel-event/human-input area=threat-model-summary rec=fct-zyxi3lxw@1
boundary: wheel event
item: cause human-input
promise: The wheel moved to holder human because a byte on viola run's stdin counted as typing.
@row_check n=557 row=contract/wheel-event/human-input sha=107514d71f2b v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/wheel.rs:322-323
quote: if self.keys.feed(&buf[..n]) { self.wheel.human_input();
@row_add n=558 row=decision/tolerant-hook-parsing area=threat-model-summary rec=fct-6fvyxh3k@1
name: Tolerant hook payload parsing
rule: Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
@row_check n=558 row=decision/tolerant-hook-parsing sha=527cc4eef1f7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:138
quote: let error = match serde_path_to_error::deserialize::<_, Known>(&*payload) {
searched: deny_unknown_fields · in **/*.rs and **/*.toml (no match)
@row_add n=559 row=decision/upstream-text-is-content area=threat-model-summary rec=fct-6fvyxh3k@1,fct-bur5z45t@1
name: Upstream text is content, never commands
rule: Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola.
why: mechanism, not policy
@row_check n=559 row=decision/upstream-text-is-content sha=e67823a3d6c7 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=560 row=decision/hook-strict-check-first area=threat-model-summary rec=fct-6fvyxh3k@1
name: Hook strict-modes check before any read
rule: Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
@row_check n=560 row=decision/hook-strict-check-first sha=60e93492ec01 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:117
quote: return Ok(ExitCode::SUCCESS);
@row_add n=561 row=decision/replayed-snapshot-yields-nothing area=threat-model-summary rec=fct-6fvyxh3k@1,fct-feql22h5@2
name: Replayed snapshot runs nothing and frames nothing
rule: A snapshot replayed from the log carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command.
@row_check n=561 row=decision/replayed-snapshot-yields-nothing sha=cc1404c60c86 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:18-27
quote: pub struct Replayed { pub wheel: Option<Wheel>, pub budget_paused: Option<bool>, pub budget_override_until: Option<String>, pub agent_session_id: Option<String>, pub links: Vec<Value>, pub dialog_pending: bool, pub skipped: Skipped, }
@row_add n=562 row=contract/mcp-tools/send area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: send
promise: A tool called by the driver model, input checked by its schema and the ViolaName check.
@row_check n=562 row=contract/mcp-tools/send sha=fff4445a32fb v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: crates/viola-mcp/src/tools.rs:379
quote: frame(ctx, &target, "send", params, true).await
@row_add n=563 row=contract/mcp-tools/wait area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: wait
promise: A tool called by the driver model that reads the driven session's output.
@row_check n=563 row=contract/mcp-tools/wait sha=02e60113de80 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:402
quote: frame(ctx, &target, "wait", params, false).await
@row_add n=564 row=contract/mcp-tools/last area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: last
promise: A tool called by the driver model that reads the driven session's latest reply.
@row_check n=564 row=contract/mcp-tools/last sha=b4469d6eebe3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:410
quote: frame(ctx, &target, "last", Map::new(), false).await
@row_add n=565 row=contract/mcp-tools/answer area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: answer
promise: A tool called by the driver model that answers the driven session's dialog, including a PermissionRequest.
@row_check n=565 row=contract/mcp-tools/answer sha=9852656a6e2b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:433
quote: frame(ctx, &target, "answer", params, true).await
@row_add n=566 row=contract/mcp-tools/list area=threat-model-summary rec=fct-bur5z45t@1
boundary: MCP tools
item: list
promise: A tool called by the driver model that takes no input, reading the board.
@row_check n=566 row=contract/mcp-tools/list sha=68057ad0cfa2 v=stale judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:255
quote: ToolName::List => list(ctx).await,
shows: The list handler receives no arguments at all and checks no input; its empty schema is only advertised, "never trusted" (tools.rs:1-2).
cell: promise
value: A tool called by the driver model that takes no input, reading the board.
@row_add n=567 row=decision/mcp-tool-list-not-enforcement area=threat-model-summary rec=fct-bur5z45t@1
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_check n=567 row=decision/mcp-tool-list-not-enforcement sha=84287c52f785 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=568 row=decision/state-tree-strict-modes-before-use area=threat-model-summary rec=fct-qxr7dgpn@2,fct-feql22h5@2
name: State tree strict-modes check before first use
rule: run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
@row_check n=568 row=decision/state-tree-strict-modes-before-use sha=1897137102e2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/revive.rs:141
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=569 row=contract/cli-exit-codes/21 area=threat-model-summary rec=fct-qxr7dgpn@2
boundary: CLI exit codes
item: 21
promise: A frame verb refused because the instance failed the strict-modes check.
@row_check n=569 row=contract/cli-exit-codes/21 sha=f2543c7eaffe v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31-33
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() { return Ok(None); }
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=570 row=decision/list-home-read-rule area=threat-model-summary rec=fct-qxr7dgpn@2
name: Board scan refuses a readable home
rule: viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
@row_check n=570 row=decision/list-home-read-rule sha=3e76f25fbfdb v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59
quote: if check_if_present(home, check_read).is_err()
at: crates/viola-state/src/board.rs:62
quote: return Ok(Err(ScanRefused::Home));
@row_add n=571 row=contract/internal-error-output/json area=threat-model-summary rec=fct-qxr7dgpn@2
boundary: catch-site internal error output
item: --json
promise: Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
@row_check n=571 row=contract/internal-error-output/json sha=e0dbc07d8a6c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67
quote: if json_asked(std::env::args_os()) {
at: src/main.rs:154
quote: .any(|arg| arg == "--json")
at: src/cmd/mod.rs:101
quote: client::refused_document("internal-error");
@row_add n=572 row=decision/pinned-binary-placed-once area=threat-model-summary rec=fct-a3npzmco@1
name: Pinned binary placed once
rule: The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears.
why: an exec of a copy another start still holds open for write fails with ETXTBSY
@row_check n=572 row=decision/pinned-binary-placed-once sha=e8d7e5ffe19f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:118
quote: match tmp.into_temp_path().persist_noclobber(path) {
at: crates/viola-state/src/fs.rs:123
quote: Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
@row_add n=573 row=decision/run-pins-own-folder-first-on-path area=threat-model-summary rec=fct-uemrwfo5@2
name: Pinned folder first on the child's PATH
rule: viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
@row_check n=573 row=decision/run-pins-own-folder-first-on-path sha=23ffdb92fb6f v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:117
quote: let dirs = std::iter::once(pinned_dir).chain(std::env::split_paths(&inherited));
at: src/run/mod.rs:143
quote: let path = std::env::var_os("PATH");
@row_add n=574 row=decision/verify-writes-no-dialog-byte area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify never types into a dialog
rule: No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output.
why: ratified
@row_check n=574 row=decision/verify-writes-no-dialog-byte sha=931e503ca2a8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/turns.rs:76
quote: holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
@row_add n=575 row=decision/nothing-written-under-claude-home area=threat-model-summary rec=fct-uemrwfo5@2
name: Nothing written under the Claude home
rule: viola writes nothing under ~/.claude.
@row_check n=575 row=decision/nothing-written-under-claude-home sha=135dd945f8a9 v=holds judge=sonnet head=8834195b0683
searched: USER_SETTINGS · in src/**/*.rs and crates/**/*.rs (one use, src/cmd/run.rs:540, read only through File::open at run.rs:552)
searched: ".claude" · in src/**/*.rs and crates/**/*.rs (no write to it; remaining hits are ".claude-plugin" folders under the viola home and test fixtures)
@row_add n=576 row=decision/agents-probe-log-content-free area=threat-model-summary rec=fct-uemrwfo5@2
name: Agents probe log holds no content
rule: An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
@row_check n=576 row=decision/agents-probe-log-content-free sha=63851e04399a v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:65
quote: subject = "agents-probe",
at: src/run/agents.rs:66
quote: child_exit_status = status,
@row_add n=577 row=decision/verify-screen-read-against-literals area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify screen read only against compiled literals
rule: verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
@row_check n=577 row=decision/verify-screen-read-against-literals sha=d1b946a0b622 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/turns.rs:75
quote: let holds = |list: &[&str]| rows.iter().any(|r| list.iter().any(|l| r.contains(l)));
@row_add n=578 row=decision/child-output-tolerant-unknown area=threat-model-summary rec=fct-uemrwfo5@2
name: Child output parsing fails to unknown
rule: Child output parsing is tolerant, and a failure becomes unknown.
@row_check n=578 row=decision/child-output-tolerant-unknown sha=c5770ad130f0 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:87-88
quote: AgentsFailure::Oversize => ReadFailure::Oversize, AgentsFailure::Malformed => ReadFailure::Malformed,
at: src/cmd/list.rs:146
quote: AgentStatus::Unknown => SessionStatus::Unknown,
@row_add n=579 row=decision/sync-crates-tokio-free area=threat-model-summary rec=fct-dvr22577@1
name: Sync crates free of tokio
rule: A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature.
guard: deny-sync.toml; scripts/channel-tokio-free.sh
@row_check n=579 row=decision/sync-crates-tokio-free sha=5860b80f0218 v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
guard_at: deny-sync.toml:12
@row_add n=580 row=decision/toolchains-by-rustup area=threat-model-summary rec=fct-dvr22577@1
name: Toolchains installed by rustup
rule: Every toolchain is installed by rustup, with no toolchain action.
@row_check n=580 row=decision/toolchains-by-rustup sha=e77bc5d865b6 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:26
quote: run: rustup toolchain install
at: rust-toolchain.toml:2
quote: channel = "1.98.1"
at: fuzz/rust-toolchain.toml:2
quote: channel = "nightly-2026-09-20"
@row_add n=581 row=decision/crates-unpublished area=threat-model-summary rec=fct-dvr22577@1
name: Crates not published
rule: Every crate is publish = false.
@row_check n=581 row=decision/crates-unpublished sha=da1e761184e4 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:190
quote: publish = false
at: crates/viola-core/Cargo.toml:7
quote: publish.workspace = true
at: fuzz/Cargo.toml:6
quote: publish = false
@row_add n=582 row=absence/release-stage area=threat-model-summary rec=fct-dvr22577@1,fct-v2lh3e2i@3
capability: A release, signing or deploy stage in v1
@row_check n=582 row=absence/release-stage sha=2771a0ec0dea v=holds judge=sonnet head=8834195b0683
searched: cosign · in .github/workflows
searched: gh release · in .github/workflows
searched: deploy · in .github/workflows
searched: dist-workspace.toml · in repository root (the only workflows are ci.yml and nightly.yml; the `release` job only builds and runs scripts/release-check.sh)
@row_add n=583 row=decision/rmcp-features-exact area=threat-model-summary rec=fct-v2lh3e2i@3
name: rmcp features pinned in the release graph
rule: rmcp is present in viola's release graph with exactly the features server and transport-io.
guard: scripts/rmcp-features.sh
@row_check n=583 row=decision/rmcp-features-exact sha=3a4ae9072492 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
guard_at: scripts/rmcp-features.sh:33
@row_add n=584 row=decision/release-build-no-test-binaries area=threat-model-summary rec=fct-v2lh3e2i@3
name: Release build refuses test-only binaries
rule: The CI release build refuses any test-only binary.
guard: scripts/release-check.sh
@row_check n=584 row=decision/release-build-no-test-binaries sha=c0821cb85a31 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/release-check.sh:32
quote: echo "release-check: FAILED — test-only binary $name"
guard_at: scripts/release-check.sh:88
@row_add n=585 row=decision/mutation-testing-at-audit area=threat-model-summary rec=fct-v2lh3e2i@3
name: Mutation testing at the epoch audit
rule: No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
@row_check n=585 row=decision/mutation-testing-at-audit sha=e1d21512f2a9 v=holds judge=sonnet head=8834195b0683
searched: cargo mutants · in .github/workflows/ci.yml
searched: agent-run.sh run --mutants · in .github/workflows/*.yml
searched: mutants · in .github/workflows (only tool installs and comments at ci.yml:32,33,91,209; no job step runs mutation)
@row_add n=586 row=decision/real-cli-local-only area=threat-model-summary rec=fct-v2lh3e2i@3
name: Real claude CLI only locally
rule: The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
@row_check n=586 row=decision/real-cli-local-only sha=528c354ca2f9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:269
quote: .args(["verify", "--"])
at: crates/viola-e2e/src/harness/boot.rs:235
quote: let program = forward(&exe(bin_dir, "viola-fake-agent"))?;
@row_add n=587 row=decision/hook-fails-open area=threat-model-summary rec=fct-3ajwacjp@1,fct-io4zn2wb@2,fct-6fvyxh3k@1
name: Hook fails open to the human
rule: Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr.
why: So the dialog renders for the human.
@row_check n=587 row=decision/hook-fails-open sha=da9c3110d9b0 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
@row_add n=588 row=decision/readers-tolerate-torn-lines area=threat-model-summary rec=fct-2i3lkhzs@2,fct-feql22h5@2
name: Readers tolerate torn lines
rule: Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
@row_check n=588 row=decision/readers-tolerate-torn-lines sha=d0cdef07e426 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
@row_add n=589 row=decision/pinned-artefacts-never-deleted area=threat-model-summary rec=fct-2i3lkhzs@2,fct-feql22h5@2
name: Pinned binary and plugin folders kept
rule: The pinned binary and the plugin folders are never deleted in v1.
@row_check n=589 row=decision/pinned-artefacts-never-deleted sha=09d15d82127b v=holds judge=sonnet head=8834195b0683
searched: remove_dir_all|remove_file|remove_dir · in src/**/*.rs and crates/**/*.rs (product hits touch only instance dirs, sockets, verify scratch dirs and pty reports; the only pin deletion, crates/viola-state/src/pin.rs:173, is inside a test)
@row_add n=590 row=decision/cargo-deny-gate area=threat-model-summary rec=fct-tm2e2x5g@1,fct-dvr22577@1
name: Cargo deny gate on the root graph
rule: Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans.
guard: cargo deny check
except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
@row_check n=590 row=decision/cargo-deny-gate sha=143e915a9b1c v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:24
quote: unknown-registry = "deny"
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
guard_at: .github/workflows/ci.yml:367
@row_add n=591 row=decision/actions-sha-pinned area=threat-model-summary rec=fct-tm2e2x5g@1,fct-dvr22577@1
name: Third-party actions SHA-pinned
rule: Third-party GitHub Actions are SHA-pinned.
@row_check n=591 row=decision/actions-sha-pinned sha=e543ebba976b v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/ci.yml:29
quote: - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2
at: .github/workflows/ci.yml:30
quote: - uses: taiki-e/install-action@7623a79cdfecb99d681017af368ca353d9f49bb5 # v2.87.19
at: .github/workflows/ci.yml:80
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
@row_add n=592 row=decision/no-abstract-namespace area=threat-model-summary rec=fct-2jp5ovui@2,fct-dthgjfbs@1,fct-2pxk6g7w@2
name: No Linux abstract namespace
rule: The Linux abstract namespace is never used for IPC sockets.
@row_check n=592 row=decision/no-abstract-namespace sha=30debd32a233 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:121
quote: .to_fs_name::<GenericFilePath>()
searched: GenericNamespaced|ToNsName|NamespacedNameType|to_ns_name · in src/**/*.rs and crates/**/*.rs
@row_add n=593 row=absence/outbound-network-calls area=threat-model-summary rec=fct-2jp5ovui@2,fct-6tg2dujf@1
capability: Outbound network calls made by viola itself
@row_check n=593 row=absence/outbound-network-calls sha=6fb586266f0e v=holds judge=sonnet head=8834195b0683
searched: TcpStream|TcpListener|UdpSocket|reqwest|hyper|ureq|isahc · in src/**/*.rs and crates/**/*.rs
searched: reqwest|hyper|ureq|isahc|curl|native-tls|rustls|self_update · in **/Cargo.toml
@row_add n=594 row=contract/send-refusal/empty-text area=input-validation rec=fct-bvljrxyp@1,fct-ub5klpnc@1,fct-ql7nqspi@1
boundary: send refusal
item: empty-text
promise: A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.
@row_check n=594 row=contract/send-refusal/empty-text sha=8b56008e82b9 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() {         return ctx.not_delivered(None, NotDelivered::EmptyText);
at: src/cmd/send.rs:185-187
quote: typed_text(text)         .is_empty()         .then_some(NotDelivered::EmptyText)
@row_add n=595 row=decision/budget-file-single-writer area=input-validation rec=fct-u7v2p5mv@1,fct-65crhg3m@2
name: hook statusline is the only budget writer
rule: budget.json is written by hook statusline alone.
@row_check n=595 row=decision/budget-file-single-writer sha=936c81bdffa5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
searched: write_budget( · in src/**/*.rs and crates/**/*.rs (non-test product code; the only non-test caller is src/cmd/hook/statusline.rs)
@row_add n=596 row=decision/mcp-stdin-line-cap area=input-validation rec=fct-bngndo2w@1,fct-yxw7xtk5@1
name: MCP stdin lines capped at the frame limit
rule: Each MCP stdin line is capped at MAX_FRAME, its newline counted; an over-long line is refused whole, never truncated or skipped, and ends the session as a stdin close does, its warning holding no byte of it.
why: rmcp reads stdin lines unbounded, so viola hands it a capped reader
@row_check n=596 row=decision/mcp-stdin-line-cap sha=693298bd0661 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:57
quote: Some(at) => (&bytes[..=at], &bytes[at + 1..], true),
at: crates/viola-mcp/src/stdin.rs:60-63
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed);
at: crates/viola-mcp/src/lib.rs:98-100
quote: parser = "mcp-stdin", detail = "oversize", count = 1u64,
@row_add n=597 row=decision/statusline-command-trust area=input-validation rec=fct-brqp4paf@1,fct-wkbf5hxn@1
name: Statusline command trusted via strict modes
rule: statusline_command is trusted only because of the home strict-modes check.
why: Write access to it equals code execution.
@row_check n=597 row=decision/statusline-command-trust sha=ffa3ea40d33f v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook/statusline.rs:130
quote: Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command,
@row_add n=598 row=decision/single-shell-out area=input-validation rec=fct-2fgozo5r@2,fct-4py7x3cl@1
name: statusline_command is the only shell-out
rule: The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.
@row_check n=598 row=decision/single-shell-out sha=8b364417c810 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
at: crates/viola-agent-claude/src/statusline.rs:124
quote: [BIN_SH, "-c", command]
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/cmd/hook/statusline.rs:278
quote: let bytes = stdout.recv_timeout(bound).ok()?;
@row_add n=599 row=absence/file-upload area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1
capability: A file upload surface
@row_check n=599 row=absence/file-upload sha=4d80954fb503 v=holds judge=sonnet head=8834195b0683
searched: upload|multipart · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs (hits only in viola-e2e harness comments)
@row_add n=600 row=absence/webhook area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1
capability: A webhook surface
@row_check n=600 row=absence/webhook sha=03fe3cb1d764 v=holds judge=sonnet head=8834195b0683
searched: webhook|axum|hyper|warp · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=601 row=absence/database area=input-validation rec=fct-h4t4qrfe@1,fct-5v55psto@1,fct-cy2wvz77@1,fct-c3b3fd57@1
capability: A database or query language
@row_check n=601 row=absence/database sha=043bb915f2ff v=holds judge=sonnet head=8834195b0683
searched: rusqlite|sqlx|diesel|sqlite|postgres · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=602 row=decision/no-tokio-in-sync-crates area=dependency-security rec=fct-cavsczz7@1,fct-o63eny7e@2
name: No tokio in the sync crates
rule: tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root.
except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh
@row_check n=602 row=decision/no-tokio-in-sync-crates sha=60e55f3785c9 v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: scripts/channel-tokio-free.sh:15
quote: out=$(cargo tree -p viola-channel -e normal --target all -i tokio --prefix none "$@" 2>&1) || rc=$?
guard_at: scripts/deny-probes.sh:20
@row_add n=603 row=decision/actions-pinned-by-sha area=dependency-security rec=fct-dc3z7vyg@1,fct-vtfmovh3@1,fct-dj5p5k3a@1
name: GitHub Actions pinned by full commit SHA
rule: Every GitHub Action is referenced by a full commit SHA with a version comment, never by @stable, @v2 or any mutable ref, and actions/checkout runs with persist-credentials: false.
@row_check n=603 row=decision/actions-pinned-by-sha sha=ab0540365f9b v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/ci.yml:24
quote: persist-credentials: false
@row_add n=604 row=decision/signed-self-update area=dependency-security rec=fct-w6kcw2wp@1,fct-qhcowchx@1,fct-heudg7au@1
name: self_update only with signatures
rule: self_update never ships without its signatures feature (zipsign); a v1.x release requires self_update 1.3.0 built with signatures.
why: a checksum from the same release proves nothing
@row_check n=604 row=decision/signed-self-update sha=2827b6d4b069 v=holds judge=sonnet head=8834195b0683
searched: self_update · in the tracked tree outside .claude/ .andromeda/ viola-0.1.0/ viola-0.2.0/ (no match: nothing ships self_update, so nothing ships it unsigned)
@row_add n=605 row=decision/no-repo-secret-scanner area=bootstrap-phases-derive-for-route-setup-project rec=fct-rwpm3oj4@1,fct-ypiftj4r@1,fct-f326yvjk@1
name: No repository secret scanner in v1
rule: v1 runs no repository secret scanner; revisit when the v1.x release workflow adds signing credentials.
why: there are no secrets in the repo or CI
@row_check n=605 row=decision/no-repo-secret-scanner sha=62a1c5c00e57 v=holds judge=sonnet head=8834195b0683
searched: gitleaks|trufflehog|detect-secrets|secretlint|git-secrets · in .github/**, scripts/**, Cargo.toml, deny.toml, .pre-commit-config.yaml, crates/**/*.rs, src/**/*.rs
@row_add n=606 row=decision/tests-use-own-home area=secret-management rec=fct-z4tolcdo@1,fct-gecdpdpp@1
name: Tests use their own home
rule: Development handles secrets as production does, and tests use their own --home directory.
@row_check n=606 row=decision/tests-use-own-home sha=9a32c7835aaa v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:620
quote: let mut args: Vec<OsString> = vec!["--home".into(), home.clone().into()];
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39-40
quote: cmd.arg("-i") .arg(format!("HOME={}", self.home))
@row_add n=607 row=decision/dialog-answers-need-verify-stamp area=security-anti-patterns-never-do-these rec=fct-syrndlhx@1,fct-zvkglgpg@2
name: Dialog answers need a verify stamp
rule: A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes.
why: a stamp is what switches on dialog answering
@row_check n=607 row=decision/dialog-answers-need-verify-stamp sha=5fb43ec5c1bd v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251
quote: let hold = self.cli_verified
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=608 row=decision/stamps-written-only-by-verify area=security-anti-patterns-never-do-these rec=fct-syrndlhx@1,fct-zvkglgpg@2,fct-zvqsbdzq@1,fct-sawbqqxr@2
name: Only viola verify writes the stamps file
rule: No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600.
why: a stamp is what switches on dialog answering, PermissionRequest allow included
@row_check n=608 row=decision/stamps-written-only-by-verify sha=49b80bfbfb6a v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
at: crates/viola-state/src/stamps.rs:65
quote: lock.lock()?;
at: crates/viola-state/src/stamps.rs:70
quote: Ok(replace_private(&path, &bytes, FILE_MODE)?)
@row_add n=609 row=decision/version-gate-strict-stamps-read area=security-anti-patterns-never-do-these rec=fct-7izg6s4l@1,fct-wkbf5hxn@1
name: Version gate reads stamps through the strict-modes check
rule: A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
@row_check n=609 row=decision/version-gate-strict-stamps-read sha=0c2616758bea v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:172
quote: let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
at: crates/viola-state/src/stamps.rs:97
quote: strict::check_stamps(home).map_err(StampsReadError::StrictModes)?;
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
@row_add n=610 row=decision/marketplace-off-integrity-set area=authentication-authorization rec=fct-65crhg3m@2,fct-feql22h5@2
name: marketplace/ off the integrity set
rule: marketplace/ is no trusted file and is off the integrity set: viola never reads it back, and the installed command gets no re-hash.
@row_check n=610 row=decision/marketplace-off-integrity-set sha=9baec6758c8a v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:20
quote: /// The marketplace dir under the home: written only by this verb.
at: crates/viola-state/src/strict.rs:95-97
quote: (ledger_dir(home), Rule::Write), (home.join("bin"), Rule::Write), (home.join("plugin"), Rule::Write),
@row_add n=611 row=contract/instance-unreachable-detail/instance-dead area=authentication-authorization rec=fct-6lhjfvvj@1,fct-6ntn52d2@1,fct-au76dwrt@1,fct-fmkuqvun@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: instance-dead
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with the instance directory standing (a stale wrapper included), or a failed connect; the reading opens nothing and writes no line.
@row_check n=611 row=contract/instance-unreachable-detail/instance-dead sha=35dc1d6a2552 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:112
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
at: src/cmd/client.rs:197
quote: return Ok(Err(Unreachable::Dead));
@row_add n=612 row=contract/instance-unreachable-detail/unwrapped-name area=authentication-authorization rec=fct-6lhjfvvj@1,fct-6ntn52d2@1,fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: unwrapped-name
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent, a regular file or a symlink); the reading opens nothing and writes no line.
@row_check n=612 row=contract/instance-unreachable-detail/unwrapped-name sha=c678b21f36d3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
@row_add n=613 row=contract/instance-unreachable-detail/strict-modes-failed area=error-handling rec=fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: strict-modes-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the instance's strict-modes check refused, and no frame was sent.
@row_check n=613 row=contract/instance-unreachable-detail/strict-modes-failed sha=f0cff93f5a4e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
@row_add n=614 row=contract/instance-unreachable-detail/endpoint-lost area=error-handling rec=fct-au76dwrt@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: endpoint-lost
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the wrapper was lost with the call open.
@row_check n=614 row=contract/instance-unreachable-detail/endpoint-lost sha=63faadde1bdd v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:200
quote: return Ok(Err(Unreachable::Lost));
at: crates/viola-mcp/src/result.rs:55
quote: return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=615 row=contract/instance-unreachable-detail/server-verify-failed area=error-handling rec=fct-au76dwrt@1,fct-pfehx2bz@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: server-verify-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the peer euid decision refused past the connect, and no frame was sent; a peer euid refusal has no word of its own and reads server-verify-failed.
@row_check n=615 row=contract/instance-unreachable-detail/server-verify-failed sha=03b6ae96051f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:178
quote: return Err(CallError::ServerUnverified);
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=616 row=contract/instance-unreachable-detail/socket-dir-refused area=error-handling rec=fct-au76dwrt@1,fct-iaqygteq@1,fct-t7qeqkiu@2
boundary: instance-unreachable detail
item: socket-dir-refused
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the socket directory of the recorded endpoint was refused, and no frame was sent; on a CLI frame verb it is during connect, one process-exit at WARN and no channel-request.
@row_check n=616 row=contract/instance-unreachable-detail/socket-dir-refused sha=a5f7565f3d84 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:174
quote: return Err(CallError::SocketDirRefused);
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=617 row=decision/channel-closed-types area=input-validation rec=fct-wrh45y73@1
name: Channel closed types
rule: Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
@row_check n=617 row=decision/channel-closed-types sha=808f4871ca71 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:168-171
quote: pub enum Permit { Allow, Deny, }
at: crates/viola-agent-claude/src/dialog.rs:175-178
quote: pub enum Verdict { Approve, Revise, }
at: src/run/dialog.rs:60
quote: dialog_id: u64,
at: src/run/links.rs:91
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: src/run/links.rs:94
quote: let (driver, driven) = (named("driver")?, named("driven")?);
@row_add n=618 row=decision/capture-answers-closed-probe-ids area=input-validation rec=fct-p4cpzbge@1
name: Capture answers from closed probe ids
rule: --answers is read only with --capture; each answer file is read through take(64) and matched exactly against the closed ProbeAnswer ids, the body built only through dialog::decision_body.
why: Ratified: the hook answers, never a key.
@row_check n=618 row=decision/capture-answers-closed-probe-ids sha=e7c80bdc9fff v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:101-103
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) {
at: src/cmd/hook.rs:481
quote: const ANSWER_CAP: u64 = 64;
at: src/cmd/hook.rs:499
quote: if file.take(ANSWER_CAP).read_to_string(&mut id).is_err() {
at: src/cmd/hook.rs:503
quote: ProbeAnswer::from_id(&id).and_then(|answer| probe_body(event, &filed.bytes, answer))
at: crates/viola-agent-claude/src/ledger.rs:377
quote: decision_body(event, input, &response)
@row_add n=619 row=decision/verify-children-direct-spawn area=threat-model-summary rec=fct-uemrwfo5@2
name: Verify children spawned directly
rule: viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals.
why: Ratified live rulings, each given after the widening was shown.
@row_check n=619 row=decision/verify-children-direct-spawn sha=4e9e0c822aac v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/run.rs:80-87
quote: let spec = SpawnSpec { program: inputs.program.to_path_buf(), args: args.to_vec(), cwd: cwd.to_path_buf(), env_set: Vec::new(), env_remove: inputs.strip.remove.clone(), size: Size::DEFAULT, };
at: crates/viola-pty/src/lib.rs:134
quote: let mut cmd = CommandBuilder::new(&spec.program);
at: crates/viola-pty/src/lib.rs:30
quote: pub const DEFAULT: Self = Self { cols: 80, rows: 24 };
at: src/cmd/verify/typed/turns.rs:134
quote: for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE] {
@row_none n=1 rec=fct-2nehsyko@1 type=S
@row_none n=2 rec=fct-y7k647gj@1 type=0
what: an editing note on when the document may be changed
@row_none n=3 rec=fct-hj3kz4j6@1 type=S
@row_none n=4 rec=fct-huufiwqz@1 type=S
@row_none n=5 rec=fct-zfyhqz3p@1 type=S
@row_none n=6 rec=fct-odek45td@1 type=S
@row_none n=7 rec=fct-cw3l7aky@1 type=4
relied: The send text is written into the PTY as one bracketed paste, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes.
@row_none n=8 rec=fct-c4sr2jmm@1 type=S
@row_none n=9 rec=fct-4trrznr3@1 type=S
@row_none n=10 rec=fct-cajceia6@1 type=0
what: an assessment note that a later public distribution adds no compliance trigger
@row_none n=11 rec=fct-hc6nagha@1 type=0
what: a note on the tier's role for downstream skills
@row_none n=12 rec=fct-dewavww2@1 type=S
@row_none n=13 rec=fct-zup45xyd@1 type=S
@row_none n=14 rec=fct-6zsj4czx@1 type=S
@row_none n=15 rec=fct-hv64gfd5@1 type=0
what: an adapter note on the OS-principal model
@row_none n=16 rec=fct-bdsvaehh@1 type=S
@row_none n=17 rec=fct-2oemhsry@1 type=S
@row_none n=18 rec=fct-bcbs2cpv@1 type=S
@row_none n=19 rec=fct-nllpai4s@1 type=S
@row_none n=20 rec=fct-rn7l7pww@1 type=S
@row_none n=21 rec=fct-zo3abxl7@1 type=0
what: a not-applicable control line for HSTS
@row_none n=22 rec=fct-rgyiwtx7@1 type=0
what: a not-applicable control line for certificate pinning
@row_none n=23 rec=fct-rwwvtlhb@1 type=S
@row_none n=24 rec=fct-kvxhlbnb@1 type=S
@row_none n=25 rec=fct-ceqkpcl5@1 type=S
@row_none n=26 rec=fct-6g2bqf3c@1 type=S
@row_none n=27 rec=fct-wyr7i6wd@1 type=S
@row_none n=28 rec=fct-ynklxelz@1 type=S
@row_none n=29 rec=fct-ysyuym2x@1 type=S
@row_none n=30 rec=fct-ujqwqmh6@1 type=S
@row_none n=31 rec=fct-q56htl73@1 type=S
@row_none n=32 rec=fct-ylpntjib@1 type=S
@row_none n=33 rec=fct-4ggr436n@2 type=S
@row_none n=34 rec=fct-dr3u7j76@1 type=9
@row_none n=35 rec=fct-4f542nxn@1 type=S
@row_none n=36 rec=fct-cutkxcmw@1 type=S
@row_none n=37 rec=fct-pc7jhvfx@1 type=S
@row_none n=38 rec=fct-pah7hshr@1 type=S
@row_none n=39 rec=fct-reffq76w@2 type=9
@row_none n=40 rec=fct-amjegsv7@1 type=9
@row_none n=41 rec=fct-6scuvr36@1 type=0
what: instructions to downstream skills on how to order and materialize the bootstrap phases
@row_none n=42 rec=fct-g62enihb@1 type=S
@row_none n=43 rec=fct-gwcpziil@1 type=S
@row_none n=44 rec=fct-fsiwgpe3@1 type=S
@row_none n=45 rec=fct-mkzs6jmr@1 type=S
@row_none n=46 rec=fct-5dopdsx2@1 type=0
what: a not-applicable placeholder line for API key and database password rotation
@row_none n=47 rec=fct-v2lsyubr@1 type=0
what: a not-applicable placeholder line for JWT signing key rotation
@row_none n=48 rec=fct-u7x6vssz@1 type=0
what: a not-applicable placeholder line for TLS certificate rotation
@row_none n=49 rec=fct-7qkvkkiz@1 type=S
@row_none n=50 rec=fct-ooy2axz4@1 type=S
@row_none n=51 rec=fct-jizwmwx6@1 type=S
@row_none n=52 rec=fct-kzddf7bk@1 type=S
@row_none n=53 rec=fct-4bkqvytp@1 type=S
@row_none n=54 rec=fct-mvfk3yxj@1 type=S
@row_none n=55 rec=fct-ltd2ftfx@1 type=S
@row_none n=56 rec=fct-nl2vclvy@1 type=S
@row_none n=57 rec=fct-jjw3aoc5@1 type=S
@row_none n=58 rec=fct-lnwynz4l@1 type=S
@row_none n=59 rec=fct-a6h3q7dw@2 type=4
relied: An absent socket directory passes the check, so a start can create it, while an unstattable one is refused.
@row_none n=60 rec=fct-ohbg3pqy@1 type=4
relied: The bind and the three client seams share one check on a recorded endpoint's parent directory.
@row_none n=61 rec=fct-6oinutja@1 type=4
relied: The socket-directory check runs before any snapshot, child or bind at run and at the start of revive, a race's loser reading the same.
@row_none n=62 rec=fct-pf2l572y@1 type=4
relied: A dropped peer reads end-of-file, and no line and nothing on the terminal is written.
@row_none n=63 rec=fct-gfc5nchi@2 type=4
relied: Reading the connected stream's peer pid writes and logs nothing.
@row_none n=64 rec=fct-zpaxdj2d@2 type=4
relied: A peer whose euid cannot be read is refused, and reading it writes and logs nothing.
@row_none n=65 rec=fct-5ryfhirx@2 type=4
relied: Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a frame be written.
@row_none n=66 rec=fct-p7bllbag@2 type=4
relied: link and unlink take the same order as every CLI frame verb: strict-modes check, liveness pre-check, then the socket-directory, euid and server checks before their one frame.
@row_none n=67 rec=fct-h63tjgiv@2 type=5
@row_none n=68 rec=fct-wandj5yh@1 type=4
@row_none n=69 rec=fct-excpcefl@1 type=4
@row_none n=70 rec=fct-hzzqncjb@2 type=5
@row_none n=71 rec=fct-zu3fn6nz@2 type=4
@row_none n=72 rec=fct-ticjysfg@1 type=5
@row_none n=73 rec=fct-msfbrx4e@1 type=5
@row_none n=74 rec=fct-4kx565zx@1 type=7
@row_none n=75 rec=fct-qcqqc5fi@1 type=5
@row_none n=76 rec=fct-zfobg22w@1 type=5
