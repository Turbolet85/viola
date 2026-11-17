# security-plan · the rows that bind 2026-11-17-supply-chain-green-and-booted-tests-end — picked 64 of 606 (36 by every draw) · rule any · pin 2304

## threat-model-summary · ## Threat Model Summary
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/fuzz-lockfile-own-audit · Fuzz lockfile audited on its own — The test-only fuzz lockfile is audited on its own, apart from the main dependency audit. · why: A ratified exemption.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/socket-in-owner-only-dir · Socket in a per-user owner-only directory — The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
decision/socket-in-per-user-dir · Sockets in a per-user directory — Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/no-abstract-namespace · No Linux abstract namespace — The Linux abstract namespace is never used for IPC sockets.
decision/pinned-artefacts-never-deleted · Pinned binary and plugin folders kept — The pinned binary and the plugin folders are never deleted in v1.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/rmcp-features-exact · rmcp features pinned in the release graph — rmcp is present in viola's release graph with exactly the features server and transport-io. · guard: scripts/rmcp-features.sh
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.

## authentication-authorization · ## Authentication & Authorization
decision/no-seam-for-foreign-peer · No seam to drive a foreign peer — A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them. · why: one account can make no foreign peer
decision/per-user-socket-directory · Socket in a per-user 0700 directory — The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.
decision/socket-dir-checked · Socket directory checked — The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
decision/one-socket-dir-check-for-bind-and-clients · One socket-directory check for bind and clients — The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools. · why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
decision/socket-dir-check-never-chmods · Socket-directory check never chmods — The socket-directory check never changes a directory's mode: a refused directory stays as found. · guard: security_negatives_socket_dir
decision/start-checks-socket-dir-before-bind · Socket directory checked before any bind, snapshot or child — A start, run or revive alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict. · why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
decision/sysinfo-process-identity · sysinfo for process identity — sysinfo is what reads a process's pid and start time for server verification.

## input-validation · ## Input Validation
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/hook-capture-arm · Hook capture arm — The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0. · why: A ratified boundary widening.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/capture-arms-isolated · Hidden capture arms isolated — The hidden capture arm reads no VIOLA_* variable, opens no channel and always exits 0.
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/frame-connect-check-order · Checks before a frame is sent — After the instance check, a CLI frame verb or MCP frame tool runs the liveness pre-check, the endpoint directory check (socket-dir-refused), the connect, the peer euid decision (server-verify-failed) and the server check, all before any frame.
decision/run-resolves-absolute-program · run resolves its child program itself — viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
decision/stamps-checked-under-lock · Stamps checked before and under lock — The ledger stamps file is checked before the ledger directory is created and again under its lock.
decision/verify-home-tree-check · verify checks the home tree first — viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
decision/verify-run-e-resume-target · verify Run E resume target — verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails. · why: Ratified.

## data-protection · ## Data Protection
decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them to be removed by hand
decision/verify-probe-captures-transient · Verify probe captures are transient — viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
decision/verify-probes-confined-to-own-dir · Verify probe effects stay in their own directory — Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.

## api-security · ## API Security
decision/rmcp-banned-features · rmcp HTTP and auth features banned — rmcp's transport-streamable-http-server and auth features are banned. · why: all five 2026 rmcp advisories sit in unused features · guard: cargo-deny bans.features
decision/rmcp-feature-set · rmcp feature set — The release graph enables rmcp features exactly server and transport-io; the test-only e2e crate takes client only, never transport-child-process. · why: all five 2026 rmcp advisories sit in unused features · guard: scripts/rmcp-features.sh in CI's lint job

## dependency-security · ## Dependency Security
decision/ci-cargo-deny-gate · cargo-deny gate in CI — CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory not on its reasoned ignore list, any yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
decision/cve-sla-disabled-feature · Critical CVE response for a disabled feature — Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.
decision/cve-sla-enabled-feature · Critical CVE response for an enabled feature — Within 7 days of a RustSec or GHSA advisory against a crate and feature viola enables, the crate is bumped.
decision/fuzz-lockfile-audited-separately · Fuzz lockfile audited separately — The fuzz lockfile is audited separately for advisories and sources in CI and for advisories weekly.
decision/fuzz-workspace-exempt-from-root-deny · Fuzz workspace outside the root deny graph — fuzz is a separate cargo workspace with its own lockfile, outside cargo-deny's root graph, so the C-build ban does not apply to it. · why: Ratified as a test-only exemption. · except: Holds only while the fuzz crate is never linked into a shipped binary and never joins the root workspace.
decision/fuzz-workspace-exemption · Fuzz workspace outside the deny graph — The fuzz crate is a separate test-only cargo workspace with its own committed lockfile, outside cargo-deny's root graph; its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it. · why: a ratified test-only exemption, holding only while the fuzz crate is never linked into a shipped binary and never joins the root workspace
decision/rmcp-advisory-ignores · Reasoned advisory ignores for disabled rmcp features — cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, its reason naming it, standing until rmcp leaves its below-3.5 range. · why: Ratified, under the Critical CVE response for an advisory confined to a disabled feature.
decision/scheduled-advisory-run · Weekly scheduled advisory run — A separate scheduled workflow runs cargo deny advisory checks weekly, without cache, on both the root and the fuzz manifests. · why: the advisory database changes without code changes
decision/bytes-advisory-floor · bytes version floor — The lockfile must resolve bytes >=1.11.1. · why: RUSTSEC-2026-0007
decision/cargo-deny-audit · cargo-deny as audit tool — cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.
decision/rustix-advisory-floor · rustix version floor — The root lockfile must resolve rustix >=1.1.6, so the advisory against 1.1.5 is cleared by the version and needs no ignore. · why: RUSTSEC-2026-0486

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/local-boundary-no-auth-library · Local boundary instead of an auth library — Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
decision/dependency-audit-tools · cargo-deny and zizmor as audit tools — Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/socket-owner-only-hardening · Owner-only channel socket — The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## error-handling · ## Error Handling
decision/run-refuses-socket-dir-before-start · run refuses a bad socket directory first — viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/e2e-home-keepers-linked-base · e2e-home keepers follow only a safe linked base — The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
decision/no-socket-in-tmp-root · No Unix socket at the temp root — A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory. · why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
decision/viola-ignores-e2e-home-backing · viola never touches the e2e-home backing — The viola binary itself never reads or writes the target/e2e-home backing by that name.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/e2e-home-link-guard · E2E home linked base guard — The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing. · why: a ratified boundary widening: on the dev host the path links outside the working directory
decision/no-seam-forcing-peer-refusal · No seam to force a peer-euid refusal — No env, flag or feature seam forces a refusing peer-euid decision; the decisions are proven over injected values and each client seam's wiring stays the named weakest proof. · why: one account can make no foreign peer
decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
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
