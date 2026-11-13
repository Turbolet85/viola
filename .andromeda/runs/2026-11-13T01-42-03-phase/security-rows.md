# security-plan · the rows that bind 2026-11-13-verify-readiness-on-the-input-box — picked 126 of 614 (87 by every draw) · rule any · pin 803

## threat-model-summary · ## Threat Model Summary
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly and a failure becomes unknown.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/flat-file-state-store · Flat-file state store — State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/rmcp-features-exact · rmcp features pinned in the release graph — rmcp is present in viola's release graph with exactly the features server and transport-io. · guard: scripts/rmcp-features.sh
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
decision/verify-never-answers-dialogs · Verify never answers dialogs — viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
decision/verify-writes-no-dialog-byte · Verify never types into a dialog — No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output. · why: ratified
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/send-text-is-one-bracketed-paste · Send text typed as one bracketed paste — Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy

## authentication-authorization · ## Authentication & Authorization
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
contract/strict-modes-refusal/verify · strict-modes refusal · viola verify — Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version.
contract/strict-refusal-message/readable · strict-modes refusal message · Readable — The read rule's refusal reads exactly "a trusted state path is readable by another user".
contract/strict-refusal-message/symlink · strict-modes refusal message · Symlink — A symbolic link's refusal reads exactly "a trusted state path is a symbolic link".

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-checked-against-ledger · MAX_FRAME checked against data — verify records the largest hook payload per event kind, stamped per CLI version; a stamped maximum within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/stamps-checked-under-lock · Stamps checked before and under lock — The ledger stamps file is checked before the ledger directory is created and again under its lock.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/verify-home-tree-check · verify checks the home tree first — viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
decision/verify-reads-only-signatures · Verify reads PTY output by signatures — verify's PTY output is read only against the compiled signature literals.
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/hook-capture-arm · Hook capture arm — The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0. · why: A ratified boundary widening.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/single-shell-out · statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.

## data-protection · ## Data Protection
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives. · except: A workspace.repo naming this repository's own remote keeps its owner handle, neither rewritten nor refused.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them to be removed by hand
decision/probe-prompts-synthetic · Probe prompts are synthetic — Probe prompts used to record committed fixtures must be synthetic.
decision/scratch-footer-reading-residual · Scratch footer reading is no standing permission — A scratch footer reading of a newer CLI is one tmpfs-home run start, no key typed, only scrubbed footer rows kept; it is no product path, and another such reading needs its own answer. · why: accepted residual
decision/screen-fixture-signature-rows-only · Screen fixtures keep only signature rows — A screen fixture keeps only the rows holding a compiled signature literal, the rest blanked; a kept row the scrub would rewrite, or a username split across a row seam, refuses the recording.
decision/verify-never-answers-import · Verify needs a pre-trusted folder — verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
decision/verify-never-answers-import-dialog · Real-CLI verify needs a prepared trusted folder — viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
decision/verify-probe-captures-transient · Verify probe captures are transient — viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
decision/verify-real-cli-transcripts-accepted · Verify's real-CLI transcripts accepted — verify's real interactive runs under a trusted parent leave the CLI's own synthetic-prompt session transcripts and run the user's global hooks; this residual is an accepted risk class.
decision/verify-trusted-runs-risk · Verify trusted runs accepted risk — verify's trusted runs B, C and D are real interactive turns whose transcripts the CLI persists and for which the user's global hooks run. · why: accepted risk
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
contract/verify-refusal-codes/strict-modes-failed · verify's refusal codes · strict-modes-failed — One of the closed codes verify writes on its process-exit when refused.
contract/verify-refusal-codes/unclean-fixture · verify's refusal codes · unclean-fixture — One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
decision/exec-commands-absolute-pinned-path · Exec commands use an absolute path — Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/verify-probes-confined-to-own-dir · Verify probe effects stay in their own directory — Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
contract/verify-refusal-codes/program-not-found · verify's refusal codes · program-not-found — One of the closed codes verify writes on its process-exit when refused.
contract/verify-refusal-codes/version-unreadable · verify's refusal codes · version-unreadable — One of the closed codes verify writes on its process-exit when refused.

## api-security · ## API Security
decision/rmcp-banned-features · rmcp HTTP and auth features banned — rmcp's transport-streamable-http-server and auth features are banned. · why: all five 2026 rmcp advisories sit in unused features · guard: cargo-deny bans.features
decision/rmcp-feature-set · rmcp feature set — The release graph enables rmcp features exactly server and transport-io; the test-only e2e crate takes client only, never transport-child-process. · why: all five 2026 rmcp advisories sit in unused features · guard: scripts/rmcp-features.sh in CI's lint job

## dependency-security · ## Dependency Security
decision/c-building-crates-banned · C-building crates banned — Crates that build C are banned from the dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/cargo-deny-gate-clean · cargo-deny gate with no advisory ignores — CI's supply-chain gate runs cargo deny over advisories, bans, licenses and sources with the advisory ignore list empty.
decision/ci-cargo-deny-gate · cargo-deny gate in CI — CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory, yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
decision/crates-io-only · crates.io as the only dependency source — cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only. · why: keeps CVE-2026-5222's sparse-registry class out of reach
decision/cve-sla-disabled-feature · Critical CVE response for a disabled feature — Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.
decision/cve-sla-enabled-feature · Critical CVE response for an enabled feature — Within 7 days of a RustSec or GHSA advisory against a crate and feature viola enables, the crate is bumped.
decision/exact-dependency-pins · Exact pins for third-party crates — Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly. · except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
decision/exact-third-party-pins · Exact third-party version pins — The workspace pins every third-party dependency version exactly.
decision/licence-allow-list · Closed licence allow list — The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0; 0BSD is never added to the allow list. · why: keeping 0BSD out of the allow list makes any other 0BSD crate still fail the gate; ratified · guard: cargo deny check licenses · except: 0BSD admitted per crate for doctest-file, pulled in by interprocess; each new exception needs its own Decisions Log entry
decision/manual-dependency-updates · Manual dependency update review — Dependency updates are reviewed manually, triggered by the scheduled advisory run; no automated update bot is used. · why: no automated update bot was researched; the rmcp minor range makes the scheduled run the main signal for that crate
decision/no-c-building-crates · No C-building crates — Crates that build C code are banned in the root dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/no-ignored-advisories · No ignored advisories — cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores no advisory; a new ignore follows the Critical CVE response SLA, its reason naming the disabled feature.
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/rmcp-http-server-and-auth-banned · rmcp HTTP server and auth features banned — cargo-deny denies the rmcp features transport-streamable-http-server and auth.
decision/scheduled-advisory-run · Weekly scheduled advisory run — A separate scheduled workflow runs cargo deny advisory checks weekly, without cache, on both the root and the fuzz manifests. · why: the advisory database changes without code changes
decision/cargo-deny-audit · cargo-deny as audit tool — cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/dependency-audit-tools · cargo-deny and zizmor as audit tools — Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/local-boundary-no-auth-library · Local boundary instead of an auth library — Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/no-auth-library-local-boundary · Authentication narrowed to the local boundary — Authentication is narrowed to the local boundary, and no auth library is taken.

## secret-management · ## Secret Management
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.

## error-handling · ## Error Handling
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes. · why: a stamp is what switches on dialog answering
decision/never-refuse-line-breaks · Line breaks and tabs are allowed — LF, CR and TAB are never refused, and a text with content is never refused for holding them. · why: Multi-line relays, plans and answers are normal.
decision/no-rmcp-http-or-auth-features · No rmcp HTTP server or auth features — rmcp's transport-streamable-http-server and auth features are never enabled.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included
decision/vt100-inside-catch-unwind · vt100 only inside catch_unwind — vt100 is never fed outside catch_unwind. · why: a parser panic must not take down the human's terminal path
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames. · except: A workspace.repo naming this repository's own remote is kept.
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/version-gate-strict-stamps-read · Version gate reads stamps through the strict-modes check — A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/stamps-strict-check-first · Stamps touched only after strict-modes check — viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.

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
