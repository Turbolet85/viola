# security-plan · the rows that bind 2026-11-14-hyprland-seam — picked 113 of 615 (74 by every draw) · rule any · pin 1006

## threat-model-summary · ## Threat Model Summary
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly and a failure becomes unknown.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown.
decision/child-path-trust · Child processes trust PATH — Spawned child processes are whatever the calling process's PATH resolves to.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/cli-input-trust-boundary · CLI input trust boundary — CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
decision/crates-unpublished · Crates not published — Every crate is publish = false.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/no-abstract-namespace · No Linux abstract namespace — The Linux abstract namespace is never used for IPC sockets.
decision/no-publish · Crates not publishable — Crates carry publish = false and builds rely on the committed Cargo.lock.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/socket-in-owner-only-dir · Socket in a per-user owner-only directory — The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
decision/socket-in-per-user-dir · Sockets in a per-user directory — Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
decision/statusline-only-shell-out · Statusline command is the only shell-out — The user-written statusline command run through a shell is the only shell-out in viola, and the per-session settings override holds the pinned hook command, never the user's command.
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/run-pins-own-folder-first-on-path · Pinned folder first on the child's PATH — viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.

## authentication-authorization · ## Authentication & Authorization
decision/client-seam-check-order · Check order at every client seam — On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
decision/one-socket-dir-check-for-bind-and-clients · One socket-directory check for bind and clients — The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools. · why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
decision/socket-dir-check-never-chmods · Socket-directory check never chmods — The socket-directory check never changes a directory's mode: a refused directory stays as found. · guard: security_negatives_socket_dir
decision/socket-dir-checked · Socket directory checked — The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
decision/no-seam-for-foreign-peer · No seam to drive a foreign peer — A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them. · why: one account can make no foreign peer
decision/per-user-socket-directory · Socket in a per-user 0700 directory — The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/channel-frame-cap-before-read · Channel frame cap before reading — A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
decision/channel-json-default-depth-limit · Channel JSON default depth limit — Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/json-default-depth-limit · Default JSON depth limit — Channel frames are parsed with serde_json's default depth limit, never with unbounded depth.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/run-resolves-absolute-program · run resolves its child program itself — viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
decision/single-shell-out · statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.
decision/claude-agents-read-fail-open · claude agents read bounded, fail open — The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
decision/claude-agents-resolved-from-path · claude agents resolved from the caller's PATH — list, revive's preflight and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME. · why: Same-user trust, an accepted risk; the MCP widening ratified.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value switches the budget gate off or makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/frame-connect-check-order · Checks before a frame is sent — After the instance check, a CLI frame verb or MCP frame tool runs the liveness pre-check, the endpoint directory check (socket-dir-refused), the connect, the peer euid decision (server-verify-failed) and the server check, all before any frame.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser and the vt100 feed are in scope for fuzz and property coverage.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/channel-frames/oversize-frame · channel frames · over-limit frame — A frame is read under a MAX_FRAME cap; an over-limit frame gets -32600 and the connection is closed.
contract/hook-stdin/oversize · hook stdin · oversize payload — A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.

## data-protection · ## Data Protection
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives. · except: A workspace.repo naming this repository's own remote keeps its owner handle, neither rewritten nor refused.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/no-tls-local-transport · No TLS on local transports — No TLS is used: the IPC channel is local Unix sockets and MCP is stdio, so confidentiality in transit rests on the socket-directory and peer checks.
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
contract/verify-refusal-codes/unclean-fixture · verify's refusal codes · unclean-fixture — One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.

## dependency-security · ## Dependency Security
decision/c-building-crates-banned · C-building crates banned — Crates that build C are banned from the dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/ci-cargo-deny-gate · cargo-deny gate in CI — CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory not on its reasoned ignore list, any yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
decision/crates-io-only · crates.io as the only dependency source — cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only. · why: keeps CVE-2026-5222's sparse-registry class out of reach
decision/exact-dependency-pins · Exact pins for third-party crates — Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly. · except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
decision/exact-third-party-pins · Exact third-party version pins — The workspace pins every third-party dependency version exactly.
decision/licence-allow-list · Closed licence allow list — The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0; 0BSD is never added to the allow list. · why: keeping 0BSD out of the allow list makes any other 0BSD crate still fail the gate; ratified · guard: cargo deny check licenses · except: 0BSD admitted per crate for doctest-file, pulled in by interprocess; each new exception needs its own Decisions Log entry
decision/no-c-building-crates · No C-building crates — Crates that build C code are banned in the root dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth
contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.
decision/cargo-deny-audit · cargo-deny as audit tool — cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.
decision/g2-single-panic-exemption · G2 zero-panics gate has one exemption — CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry. · guard: G2 --probe run before every G2 check
decision/g2-zero-panics-single-exemption · Zero-panics gate with one exemption — CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry. · guard: G2 --probe · except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
decision/no-stale-license-exceptions · No license exception for an absent crate — A license exception naming a crate absent from the graph is dropped, never kept. · why: Such an exception narrows nothing and widens nothing.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/local-boundary-no-auth-library · Local boundary instead of an auth library — Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/dependency-audit-tools · cargo-deny and zizmor as audit tools — Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/socket-owner-only-hardening · Owner-only channel socket — The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.

## error-handling · ## Error Handling
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/socket-dir-refusal-names-only-name · Socket-directory refusal discloses no path — The socket-directory refusal names only the parsed ViolaName, and its lines hold no path, pid or uid.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace sockets are never used. · why: They have no filesystem permissions.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/no-serde-unbounded-depth · No unbounded JSON depth — serde_json's unbounded_depth feature is never enabled. · guard: deny.toml ban
decision/no-socket-in-tmp-root · No Unix socket at the temp root — A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory. · why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/new-listener-needs-log-entry · New listener or channel method needs a log entry — No listener (TCP port, WebSocket, http hooks, MCP HTTP transport) or new channel method is added without a Security Decisions Log entry mapping it to the security plan's IPC controls.
decision/socket-mode-not-sole-control · Socket file mode is not the only control — The socket file's 0600 mode is never the only Unix access control. · why: unix(7) says socket permissions may be ignored.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.

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
