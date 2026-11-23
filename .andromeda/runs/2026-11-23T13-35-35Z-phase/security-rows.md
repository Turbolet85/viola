# security-plan · the rows that bind 2026-11-23-seam-requests-for-viola-open — picked 57 of 644 (36 by every draw) · rule any · pin 4223

## threat-model-summary · ## Threat Model Summary
decision/cargo-deny-gate · Cargo deny gate on the root graph — Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans. · guard: cargo deny check · except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/mcp-frame-cap · MCP stdin line cap — Each MCP stdin line is capped at MAX_FRAME before rmcp reads it; an over-long line is refused whole and ends the session, never dispatched in part.
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.

## authentication-authorization · ## Authentication & Authorization
decision/explicit-mode-before-write · Mode set explicitly before any byte — A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded. · except: The trust record writer replaces Claude's config file keeping that file's own mode.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/hyprctl-read-deadline · hyprctl child bounded by a deadline — The hyprctl child runs with null stdin and the named environment variables removed, under a 5 s deadline at which it is killed and reaped; a deadline kill or a non-zero exit reads as unreachable. · guard: read_past_the_deadline_kills_and_reaps_its_child · provisional
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate, and the Claude transcript path shape check in the Claude adapter crate, which the hook and the wrapper both call.

## data-protection · ## Data Protection
decision/hyprland-recorder-string-allowlist · Hyprland recorder keeps strings by a closed allowlist — viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag); every other string becomes the placeholder, a non-JSON reply stays only as ok, then the shared scrub runs and an unclean set is refused whole. · why: Hyprland 0.56.2 carries titles and user text under keys no deny-list foresees (lastwindowtitle, nested workspace.name, tags). · guard: record_replaces_every_string_off_the_allowlist; planted_hyprland_lastwindowtitle_is_rejected
decision/hyprland-reply-text-dropped · Hyprland reply text dropped at the parse — Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error. · guard: contract_hyprland_canary_reaches_no_line_and_no_error
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a home, drive-letter or user-folder path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home, drive-letter or user-folder path, the user word or an email-shaped token survives.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving home, drive-letter or user-folder path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
contract/verify-refusal-codes/unclean-fixture · verify's refusal codes · unclean-fixture — One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.

## dependency-security · ## Dependency Security
decision/c-building-crates-banned · C-building crates banned — Crates that build C are banned from the dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/crates-io-only · crates.io as the only dependency source — cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only. · why: keeps CVE-2026-5222's sparse-registry class out of reach
decision/exact-dependency-pins · Exact pins for third-party crates — Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly. · except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
decision/exact-third-party-pins · Exact third-party version pins — The workspace pins every third-party dependency version exactly.
decision/no-c-building-crates · No C-building crates — Crates that build C code are banned in the root dependency graph by cargo-deny. · guard: scripts/deny-probes.sh
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/nutype-input-validation · nutype for input validation — Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · why: arch rejected garde and validator
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value

## secret-management · ## Secret Management
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.

## error-handling · ## Error Handling
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
decision/every-error-writer-named · Every product error writer named — Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census. · guard: product_error_writers_are_each_named

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/no-serde-unbounded-depth · No unbounded JSON depth — serde_json's unbounded_depth feature is never enabled. · guard: deny.toml ban
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
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
