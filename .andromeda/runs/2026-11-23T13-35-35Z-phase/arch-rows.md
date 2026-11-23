# architecture · the rows that bind 2026-11-23-seam-requests-for-viola-open — picked 57 of 1008 (41 by every draw) · rule any · pin 4223

## design-philosophy · ## Design Philosophy
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it

## established-decisions · ## Established Decisions
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/window-features-target-hyprland · Window features target Hyprland — The window features target the Hyprland compositor.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/no-cargo-pup · cargo_pup not taken — Module boundaries are enforced by crate separation, not by cargo_pup. · why: cargo_pup needs nightly and was rejected.

## conventions · ## Conventions
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`. · except: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.

## standard-contracts · ## Standard Contracts
decision/hyprctl-direct-spawn · hyprctl spawned directly with a closed argv — hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is the Lua form hl.dsp.focus whose only inserted value is a closed window address or an i32 workspace id, never reply text. · why: Hyprland 0.56.2 runs a dispatch's arguments as Lua, so only the measured hl.dsp.focus form is accepted.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
contract/hyprland-seam-errors/malformed · Hyprland seam errors · malformed — The reply could not be parsed for its request; the fixed message hyprctl reply not understood interpolates nothing.
contract/hyprland-seam-errors/oversize · Hyprland seam errors · oversize — The reply passed the frame cap and was refused whole before the exit was read; the fixed message hyprctl reply over the frame cap interpolates nothing.
contract/hyprland-seam-errors/refused · Hyprland seam errors · refused — hyprctl refused a dispatch; the fixed message hyprctl refused the dispatch interpolates nothing.
contract/hyprland-seam-errors/unreachable · Hyprland seam errors · unreachable — hyprctl could not start, exited non-zero or was killed at its deadline; the fixed message hyprctl did not answer interpolates nothing.
contract/hyprland-seam-errors/unsupported · Hyprland seam errors · unsupported — No hyprctl program was given, so nothing is spawned and no line is written; the fixed message hyprctl not found interpolates nothing.
contract/wrapper-channel-frames/max-frame · wrapper channel frames · MAX_FRAME — A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.

## occupied-resources · ## Occupied Resources
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness, viola-mcp-tap and viola-live-rig are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds a home, drive-letter or user-folder path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.

## project-intent · ## Project Intent
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.

## inherited-defaults · ## Inherited Defaults
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.

## build-system · key of ## Infrastructure Patterns
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth

## ci-cd-approach · key of ## Infrastructure Patterns
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on viola-core alone among viola crates, never on viola-agent-claude, viola-state or viola-channel. · guard: the cargo tree check of viola-hyprland and the per-sync-crate tokio ban

## project-directory-structure · key of ## Infrastructure Patterns
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job
decision/anyhow-at-bin-edge-only · anyhow at the binary edge only — anyhow error handling is confined to the edge of the viola binary.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command
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
name/binary/viola-hypr-tap — viola-hypr-tap
name/binary/viola-live-rig — viola-live-rig
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
name/crate/viola-hyprland — viola-hyprland
name/crate/viola-mcp — viola-mcp
name/crate/viola-pty — viola-pty
name/crate/viola-state — viola-state
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES
name/other/agent-run-rig — AGENT_RUN_RIG
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/harness-tied-test-child — HARNESS_TIED_TEST_CHILD
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
name/other/hook-stop — hook stop
name/other/hook-stop-failure — hook stop-failure
name/other/hook-user-prompt-submit — hook user-prompt-submit
name/other/ledger-verify-lock — ledger/verify.lock
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD
name/other/mcp-tool-answer — answer
name/other/mcp-tool-last — last
name/other/mcp-tool-list — list
name/other/mcp-tool-send — send
name/other/mcp-tool-wait — wait
name/other/root-tied-test-child — ROOT_TIED_TEST_CHILD
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
