# architecture · the rows that bind 2026-11-14-hyprland-seam — picked 123 of 987 (80 by every draw) · rule any · pin 1006

## design-philosophy · ## Design Philosophy
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/cargo-deny-dependency-policy · cargo-deny dependency policy — cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/libc-only-os-binding · libc is the one OS-binding crate — libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
decision/wrapper-ipc-local-socket · interprocess local sockets for wrapper IPC — Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.

## established-decisions · ## Established Decisions
decision/anyhow-only-in-bin · anyhow only in the viola binary — anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter. · why: anyhow's context chains are only useful where a person reads them.
decision/cargo-workspace-flat-crates · Cargo workspace with flat crates — The project is a Cargo workspace with a flat crates layout, the root manifest being both the viola bin package and the workspace; each product crate is created by the first piece of work that consumes it. · why: The compiler enforces that viola-pty cannot see viola-agent-claude; cargo install --path . requires the bin at the root.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/flat-cargo-workspace · Flat Cargo workspace with the bin at the root — One Cargo workspace with a flat `crates/` layout, the root `Cargo.toml` being both the `viola` bin package and the workspace; `fuzz/` is its own workspace. · why: The compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/socket-dir-check-verdicts · Socket-directory check verdicts, never chmods — The socket-directory check passes an absent path and never chmods, so a refused directory stays as found. It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path. · why: A start makes an absent directory itself, and the negative tests for start, frame verbs and hooks expect a loosened or linked directory left as found.
decision/sync-core-tokio-only-in-mcp · Hybrid sync core, Tokio only in mcp — The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`. · why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio. · guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest` · except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/tokio-only-in-mcp-crate · tokio only in the MCP crate — Among the product crates only viola-mcp enables tokio; viola-channel lists it optional behind its tokio feature, and no other product member's normal graph reaches tokio. · guard: scripts/channel-tokio-free.sh
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/window-features-target-hyprland · Window features target Hyprland — The window features target the Hyprland compositor.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/mcp-stdin-line-cap · MCP stdin read under a line cap — The MCP server's stdin is read under a `MAX_FRAME` line cap, newline counted; a longer line is refused whole, never truncated or skipped, and ends the session with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it. · why: rmcp's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp.
decision/no-cargo-pup · cargo_pup not taken — Module boundaries are enforced by crate separation, not by cargo_pup. · why: cargo_pup needs nightly and was rejected.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/terminal-fleet-console · Terminal fleet console as the one view — The one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland.

## conventions · ## Conventions
decision/anyhow-only-in-root-bin · anyhow only in the root bin crate — `anyhow` appears only in the root bin crate.
decision/crate-naming · Crate naming — Crates are named `viola-<area>`, kebab-case.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error, covering `resets_at`, `used_percentage` and a missing statusline window.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.

## standard-contracts · ## Standard Contracts
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly. · except: `hook statusline` runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin.
decision/statusline-only-shell-out · Statusline command is the only shell-out — The user's statusline command is the only shell-out in viola: one argument of `/bin/sh -c`, with the same stdin, environment and working directory unchanged, stderr discarded, bounded by a deadline. · why: The command is a user-written shell string; a ratified boundary widening. · guard: ledger row statusline-shell
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
contract/wrapper-channel-frames/max-frame · wrapper channel frames · MAX_FRAME — A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.

## occupied-resources · ## Occupied Resources
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace socket names are never used. · why: They carry no filesystem permissions and any local user could connect.
decision/no-after-every-test-error-body-helper · No after-every-test error-body helper — Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 7 s WITHIN bound of the watch support.
decision/socket-dir-owner-only · Socket in a per-user owner-only directory — The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
decision/socket-in-per-user-dir · Endpoint socket in a per-user 0700 directory — The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/viola-channel-tokio-free-by-default · viola-channel tokio-free without its feature — The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client. · guard: channel-tokio-free.sh
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/same-user-peers-only · Same-user peers only — Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/test-endpoints-outside-namespace · Test channel endpoints outside the product namespace — Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`.
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
decision/same-user-endpoint-trust · Local endpoint trusts the same OS user only — v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/serde-json-unbounded-depth-banned · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in the dependency policy. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.

## inherited-defaults · ## Inherited Defaults
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.

## build-system · key of ## Infrastructure Patterns
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth
decision/sync-crates-tokio-free · Sync crates are tokio-free — Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root. · why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification. · guard: deny-sync.toml per crate in scripts/sync-crates.txt
decision/banned-features · Banned dependency features — The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned. · guard: cargo deny check
decision/no-c-building-crates · No C-building crates — C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph. · guard: cargo deny check · except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace.
decision/no-telemetry-crates · No telemetry crates — Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned. · guard: cargo deny check

## ci-cd-approach · key of ## Infrastructure Patterns
decision/github-actions-ci · GitHub Actions CI — CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow.
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh
decision/sync-crates-compile-without-tokio · Sync crates compile without tokio — Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it.

## project-directory-structure · key of ## Infrastructure Patterns
decision/anyhow-at-bin-edge-only · anyhow at the binary edge only — anyhow error handling is confined to the edge of the viola binary.
decision/no-tokio-in-sync-crates · No tokio in sync crates — Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root. · guard: cargo-deny tokio ban over the sync-crate list
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job
decision/cargo-deny-bans · cargo-deny supply-chain policy — cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features. · guard: deny-probes negative probe per ban with a clean control

# architecture · its absences, names and overviews, whole

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
absence/windows-and-macos-support — Support for Windows or macOS

## conventions · ## Conventions
absence/database — A database

## standard-contracts · ## Standard Contracts
absence/graphical-interface — A web or graphical surface
absence/polling-endpoints — Polling endpoints (v1)
absence/websocket — WebSocket endpoints (v1)

## occupied-resources · ## Occupied Resources
absence/listening-network-port — A listening network port of any kind: no database port, no broker port, no listening TCP port
name/binary/viola — viola
name/binary/viola-fake-agent — viola-fake-agent
name/binary/viola-harness — viola-harness
name/binary/viola-mcp-tap — viola-mcp-tap
name/command/answer — answer
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait
name/crate/viola — viola
name/crate/viola-agent-claude — viola-agent-claude
name/crate/viola-channel — viola-channel
name/crate/viola-core — viola-core
name/crate/viola-e2e — viola-e2e
name/crate/viola-fuzz — viola-fuzz
name/crate/viola-mcp — viola-mcp
name/crate/viola-pty — viola-pty
name/crate/viola-state — viola-state
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
name/other/hook-statusline — hook statusline
name/other/hook-stop — hook stop
name/other/hook-user-prompt-submit — hook user-prompt-submit
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD
name/other/mcp-tool-answer — answer
name/other/mcp-tool-last — last
name/other/mcp-tool-list — list
name/other/mcp-tool-send — send
name/other/mcp-tool-wait — wait
name/other/viola — viola
name/other/viola-at-viola — viola@viola
name/other/viola-bin — VIOLA_BIN
name/other/viola-dir — VIOLA_DIR
name/other/viola-h12-sock — viola-<h12>.sock
name/other/viola-name — VIOLA_NAME
name/other/viola-test-chan-pid-label — viola-test-chan-<pid>-<label>
name/other/viola-verify-probe — viola-verify-probe
name/server/viola — viola

## cross-cutting-patterns · ## Cross-cutting Patterns
absence/secrets — Secrets of any kind in v1 configuration

## project-intent · ## Project Intent
absence/web-or-graphical-frontend — A web or graphical frontend
overview/project-intent — viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.

## inherited-defaults · ## Inherited Defaults
absence/containers-and-hosting — Containers or hosting of any kind

## ci-cd-approach · key of ## Infrastructure Patterns
absence/deploy-stage — A deploy stage

## deployment-model · key of ## Infrastructure Patterns
absence/container-or-serverless-deployment — Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
