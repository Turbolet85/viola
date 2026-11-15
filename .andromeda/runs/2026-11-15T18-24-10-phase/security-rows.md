# security-plan · the rows that bind 2026-11-15-mutation-and-coverage-baselines-green — picked 71 of 597 (35 by every draw) · rule any · pin 1565

## threat-model-summary · ## Threat Model Summary
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/no-abstract-namespace · No Linux abstract namespace — The Linux abstract namespace is never used for IPC sockets.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/socket-in-owner-only-dir · Socket in a per-user owner-only directory — The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
decision/socket-in-per-user-dir · Sockets in a per-user directory — Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/exclusive-endpoint-bind · Exclusive endpoint bind — viola run binds its endpoint exclusively, so a squatted name blocks startup with exit 1.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/wrapper-channel-methods/hook-dialog · wrapper channel methods · hook.dialog — Carries a dialog from the hook to the wrapper and its decision back.
contract/wrapper-channel-methods/hook-event · wrapper channel methods · hook.event — A notification from the hook to the wrapper.

## authentication-authorization · ## Authentication & Authorization
decision/no-socket-overwrite · No socket overwrite — Binding never overwrites an existing socket; try_overwrite stays off.
decision/one-socket-dir-check-for-bind-and-clients · One socket-directory check for bind and clients — The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools. · why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
decision/per-user-socket-directory · Socket in a per-user 0700 directory — The socket lives in a per-user directory with mode 0700, $XDG_RUNTIME_DIR/viola/ with /tmp/viola-<uid>/ only as a fallback, the endpoint being <dir>/viola-<h12>.sock.
decision/socket-dir-checked · Socket directory checked — The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
contract/socket-dir-refusal/hook · socket-directory refusal · viola hook — Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
decision/client-seam-check-order · Check order at every client seam — On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
decision/client-verifies-server · Client verifies the server — The client verifies the connected peer's pid and that process's start time against its one snapshot read; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect.
decision/client-verifies-server-before-frame · Client verifies the server before any frame — Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal. · why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
decision/no-listener-mode-option · Socket mode set by chmod — The bound Unix socket's mode is set with std chmod, and interprocess's listener mode option is not used.
decision/only-matches-verdict-lets-a-frame-out · Only a Matches server verdict lets a frame out — Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a client write a frame; a mismatched pid or start time, or an unverifiable server, is refused before any frame. · why: The CLI verbs, the hooks and the MCP frame tools rely on it to reach only the recorded wrapper; server-verify-failed names the refusal.
decision/socket-chmod-after-bind · Socket owner-only after bind — The Unix socket is set to mode 0600 right after the bind.
decision/socket-file-chmod-after-bind · Socket file chmod 0600 after bind — The socket file is chmod 0600 right after the bind; setting the mode through listener options is not used, chmod after the bind being the one form.
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with the instance directory standing (a stale wrapper included), or a failed connect; the reading opens nothing and writes no line.
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent or a regular file); the reading opens nothing and writes no line.
contract/ipc-client-refusals/socket-dir-refused · IPC client refusals · socket-dir-refused — Reported when the recorded endpoint's socket directory fails its check before the connect; no frame is sent.
contract/server-verify-refusal/hook · server-verification refusal · viola hook — Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.

## input-validation · ## Input Validation
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/verify-run-e-resume-target · verify Run E resume target — verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails. · why: Ratified.
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/replayed-snapshot-fallbacks · Replayed snapshot fallbacks — Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, and the collision check judges liveness by the heartbeat alone.

## data-protection · ## Data Protection
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
decision/hyprland-recorder-string-allowlist · Hyprland recorder keeps strings by a closed allowlist — viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag); every other string becomes the placeholder, a non-JSON reply stays only as ok, then the shared scrub runs and an unclean set is refused whole. · why: Hyprland 0.56.2 carries titles and user text under keys no deny-list foresees (lastwindowtitle, nested workspace.name, tags). · guard: record_replaces_every_string_off_the_allowlist; planted_hyprland_lastwindowtitle_is_rejected
decision/hyprland-reply-text-dropped · Hyprland reply text dropped at the parse — Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error. · guard: contract_hyprland_canary_reaches_no_line_and_no_error
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/verify-refusal-codes/unclean-fixture · verify's refusal codes · unclean-fixture — One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
decision/no-tls-local-transport · No TLS on local transports — No TLS is used: the IPC channel is local Unix sockets and MCP is stdio, so confidentiality in transit rests on the socket-directory and peer checks.
contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/test-support-dev-only · test-support enabled only for development — viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/local-boundary-no-auth-library · Local boundary instead of an auth library — Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/no-mutants-out-upload · No mutants.out upload — CI does not upload mutants.out. · why: It held absolute argv paths and test output.
decision/socket-owner-only-hardening · Owner-only channel socket — The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## error-handling · ## Error Handling
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
decision/hook-refusals-write-no-frame · A refused hook writes no frame — A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/chmod-socket-after-bind · Socket mode set by chmod after bind — The socket mode is set by chmod after the bind, never through the listener's mode option. · why: Set while macOS, where that option fails the bind, was a target, and kept.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace sockets are never used. · why: They have no filesystem permissions.
decision/no-interprocess-try-overwrite · interprocess try_overwrite stays off — interprocess try_overwrite is never enabled on the socket bind. · why: TOCTOU deletion of an attacker-swapped file
decision/no-socket-in-tmp-root · No Unix socket at the temp root — A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory. · why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/verify-server-before-frame · Verify the server before any frame — No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/server-verified-before-any-frame · Server verified before any channel frame — No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
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
