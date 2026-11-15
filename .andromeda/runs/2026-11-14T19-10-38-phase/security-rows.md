# security-plan · the rows that bind 2026-11-14-status-line-interception-removed — picked 82 of 610 (48 by every draw) · rule any · pin 1206

## threat-model-summary · ## Threat Model Summary
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude.
decision/replayed-snapshot-yields-nothing · Replayed snapshot runs nothing and frames nothing — A snapshot replayed from the log carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command.
decision/statusline-command-integrity · Statusline command integrity — The hook statusline runs statusline_command through a shell, its integrity resting on filesystem permissions of the viola home.
decision/statusline-only-shell-out · Statusline command is the only shell-out — The user-written statusline command run through a shell is the only shell-out in viola, and the per-session settings override holds the pinned hook command, never the user's command.
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint, statusline_command and ledger stamps are used.
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/no-publish · Crates not publishable — Crates carry publish = false and builds rely on the committed Cargo.lock.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.

## authentication-authorization · ## Authentication & Authorization
decision/statusline-sources-untrusted-read · Statusline sources read untrusted — statusline-source.json and the user's ~/.claude/settings.json are not trusted files; their control is a read-only, capped, tolerant read. · why: a boundary widening that was ratified
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version.
contract/strict-modes-refusal/version-gate · strict-modes refusal · viola run's version gate — A refused stamps read is cli_verified:false with parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
decision/hook-instance-check-first · Hook instance check before anything — Hook event, dialog and statusline paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
decision/hook-unknown-event-silent-exit · Hook unknown event exits silently — The hook event argument maps into a closed set of nine events (statusline taken first, apart); an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing written anywhere.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/no-statusline-command-in-logs · Statusline command never logged — No log line or span carries the statusline command.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/replayed-snapshot-fallbacks · Replayed snapshot fallbacks — Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, statusline prints nothing, and the collision check judges liveness by the heartbeat alone.
decision/replayed-snapshot-inert · Replayed snapshot is inert — A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid, start time or statusline command, so no frame is written and no command runs from it.
decision/single-shell-out · statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.
decision/statusline-capture-arm · Statusline capture arm — The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte, else other, the word itself written nowhere; it prints nothing, exits 0 and opens no channel. · why: Ratified.
decision/statusline-command-trust · Statusline command trusted via strict modes — statusline_command is trusted only because of the home strict-modes check. · why: Write access to it equals code execution.
decision/statusline-deadline · Statusline command deadline — The wait for the statusline command is bounded at 5 s (detail deadline); a failing command prints nothing. · provisional
decision/statusline-payload-fails-silent · Statusline payload fails silent — The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr and nothing printed; the hook reads no rate_limits and writes no file.
decision/statusline-shell-out · Statusline shell-out — The recorded statusline command, the only shell-out, runs only in hook statusline as /bin/sh -c with the command as one argument; stderr is discarded, stdout capped and printed byte for byte, never parsed, and no log line carries the command. · why: A ratified boundary widening.
decision/statusline-source-read-only · Statusline source read only — A statusline command is taken only from a non-empty statusLine of type command with no NUL, read once and capped; viola writes neither source.
decision/statusline-source-selection · Statusline source selection — viola run reads the statusline command from the home's statusline-source.json when it exists, else only for the default home the user's Claude settings; a command only from a non-empty command-type statusLine with no NUL; viola writes neither source. · why: Ratified, the file in the viola home chosen.
decision/verify-reads-only-signatures · Verify reads PTY output by signatures — verify's PTY output is read only against the compiled signature literals.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/stamps-checked-under-lock · Stamps checked before and under lock — The ledger stamps file is checked before the ledger directory is created and again under its lock.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.

## data-protection · ## Data Protection
decision/code-bearing-files-rewritten-each-start · Code-bearing files rewritten at every start — The plugin files and the instance settings.json override are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy. · why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
decision/plugin-files-rewritten-each-start · Plugin files rewritten every start — The plugin files and the settings override are rewritten at every start.
decision/settings-override-plain-path · Settings override only for plain paths — The settings override is written only for a pinned path of plain characters. · why: a shell runs the override unquoted
decision/statusline-override-plain-path-only · Statusline override only for a plain pinned path — The settings.json statusline override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings flag is passed. · why: A shell runs the command string unquoted.
decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file.

## dependency-security · ## Dependency Security
decision/crates-io-only · crates.io as the only dependency source — cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only. · why: keeps CVE-2026-5222's sparse-registry class out of reach
decision/exact-dependency-pins · Exact pins for third-party crates — Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly. · except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
decision/exact-third-party-pins · Exact third-party version pins — The workspace pins every third-party dependency version exactly.
decision/no-stale-license-exceptions · No license exception for an absent crate — A license exception naming a crate absent from the graph is dropped, never kept. · why: Such an exception narrows nothing and widens nothing.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.

## secret-management · ## Secret Management
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory

## error-handling · ## Error Handling
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arms never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The two capture arms run ahead of that check.
decision/run-refuses-socket-dir-before-start · run refuses a bad socket directory first — viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/absolute-viola-path-in-commands · Hook, MCP and statusline commands use an absolute viola path — Every exec-form command in the embedded hooks.json or .mcp.json, in plugin install's .mcp.json, and the statusline command written into instances/<name>/settings.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH. · why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes. · why: a stamp is what switches on dialog answering
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/statusline-only-from-strict-home · statusline command only from a strict home — statusline_command is never run from a snapshot whose home or instance directory fails the strict-modes check. · why: Write access there equals code execution.
decision/version-gate-strict-stamps-read · Version gate reads stamps through the strict-modes check — A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: endpoint, pinned_bin and statusline_command in it decide where clients connect and what hook statusline executes

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing

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
