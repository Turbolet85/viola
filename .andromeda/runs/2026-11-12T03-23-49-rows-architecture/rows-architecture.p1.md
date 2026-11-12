@row_add n=1 row=decision/mechanism-not-policy area=design-philosophy rec=1,8
name: Mechanism, not policy: a general bridge
rule: viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb.
why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
@row_add n=2 row=decision/send-confirmed-after-the-fact area=design-philosophy rec=2,14
name: Every send confirmed after the fact
rule: A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash.
why: A turn boundary per the hooks does not prove the input box is ready.
except: A ledger-listed local command whose post-condition is none, not yet measured, or whose CLI is unverified returns `ok` with `confirmed: false` and detail `unconfirmable`.
@row_add n=3 row=decision/capability-ledger area=design-philosophy rec=2,26
name: Version-stamped capability ledger
rule: Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_add n=4 row=decision/unverified-cli-transport-only area=design-philosophy rec=2,26
name: Transport-only degrade on an unverified CLI
rule: On a CLI build nobody has verified, viola still types, runs the wheel and emits events, but withholds dialog answers with `unverified-cli`, and its readiness gate is partial.
@row_add n=5 row=decision/human-always-wins-wheel area=design-philosophy rec=3
name: The human always wins the wheel
rule: Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
@row_add n=6 row=decision/no-daemon-disk-is-shared-truth area=design-philosophy rec=4
name: No daemon; the disk is the shared truth
rule: Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
@row_add n=7 row=decision/linux-only area=design-philosophy rec=5,7
name: Linux only
rule: viola targets Linux only: the core on any current distribution, built and CI-tested on `ubuntu-latest`, and the window features on Hyprland.
why: The Windows and macOS support is cut, its code recoverable at the tag `multi-os-final`.
@row_add n=8 row=decision/no-tmux-or-shell-dependency area=design-philosophy rec=5
name: No dependence on tmux or a shell
rule: No design choice may depend on tmux or a shell.
@row_add n=9 row=overview/stack-and-technologies area=stack-and-technologies rec=6
text: viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.
@row_add n=10 row=decision/personal-v1-no-accounts area=established-decisions rec=7
name: Personal v1 without accounts
rule: v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively.
why: The later public version for individual Claude Code subscribers must not need a rewrite.
@row_add n=11 row=decision/claude-first-no-adapter-trait area=established-decisions rec=9
name: Claude-first, one agent crate, no adapter trait
rule: All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events.
why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
@row_add n=12 row=decision/sync-core-tokio-only-in-mcp area=established-decisions rec=10,33
name: Hybrid sync core, Tokio only in mcp
rule: The core is synchronous on std threads, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`.
why: portable-pty's handles block so the pump needs threads anyway; `hook` must not pay runtime startup; rmcp and interprocess-async require Tokio.
guard: sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates in CI
@row_add n=13 row=decision/portable-pty-behind-pty-seam area=established-decisions rec=11
name: portable-pty pinned behind viola's pty seam
rule: portable-pty, pinned exactly at `=0.9.0`, is what drives the terminal, used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill).
why: The workspace pins every third-party version exactly; 0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family.
@row_add n=14 row=decision/child-exit-on-process-handle area=established-decisions rec=11
name: Child exit detected on the process handle
rule: Child exit is detected on the process handle, never on EOF.
@row_add n=15 row=decision/human-key-never-held-behind-resize area=established-decisions rec=11
name: Human key never held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize.
@row_add n=16 row=decision/no-da1-answer area=established-decisions rec=11
name: viola never answers DA1
rule: viola never answers a DA1 query itself.
@row_none n=1 rec=12 type=9
@row_add n=17 row=decision/vt100-readiness-gate area=established-decisions rec=13
name: vt100 as a pre-send readiness gate only
rule: The vt100 screen model serves only as a pre-send gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits up to its maximum for the version-stamped input-box signature; the passthrough never waits on it.
why: CLI-native modals bypass every hook, and the screen lags the hooks.
@row_add n=18 row=contract/not-delivered-details/input-not-ready area=established-decisions rec=13
boundary: not-delivered refusal details
item: input-not-ready
promise: A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
@row_add n=19 row=contract/not-delivered-details/no-prompt-submitted area=established-decisions rec=14
boundary: not-delivered refusal details
item: no-prompt-submitted
promise: A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
@row_add n=20 row=decision/send-text-line-endings area=established-decisions rec=15
name: Send text line-ending normalisation
rule: A `send` types its text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters.
@row_add n=21 row=contract/not-delivered-details/empty-text area=established-decisions rec=15
boundary: not-delivered refusal details
item: empty-text
promise: A send whose typed text is left empty is refused `not-delivered` with this detail, nothing issued.
@row_add n=22 row=decision/ndjson-logs-and-atomic-snapshots area=established-decisions rec=16
name: ndjson append logs plus atomic JSON snapshots
rule: State is kept in ndjson append logs and atomically replaced JSON snapshots, locked with std `File::lock`; no database.
why: The trail stays human-readable and tail-able, with no C in the build and no daemon; SQLite and redb were rejected.
@row_add n=23 row=decision/separate-lock-files area=established-decisions rec=16
name: Locks on separate .lock files
rule: File locks are taken on separate `.lock` files, never on the data files themselves.
why: A layout chosen while Windows (where append plus exclusive lock fails) was a target, and kept.
@row_add n=24 row=decision/torn-last-line-healed area=established-decisions rec=16
name: Torn log line healed by the next append
rule: Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_add n=25 row=decision/tempfile-snapshot-writer area=established-decisions rec=17
name: tempfile as the snapshot writer
rule: Every atomic file write goes through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp-plus-rename code, and a failed write leaves the target as it was.
why: atomic-write-file's BSD-3-Clause licence is outside the deny allow list, and tempfile was already locked.
@row_add n=26 row=decision/private-mode-before-write area=established-decisions rec=17
name: File mode set before any byte is written
rule: A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
@row_add n=27 row=decision/pinned-copy-never-replaced area=established-decisions rec=17
name: Pinned copy never replaced
rule: The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
@row_add n=28 row=decision/interprocess-local-socket-channel area=established-decisions rec=18
name: interprocess local sockets behind the channel seam
rule: interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_add n=29 row=decision/channel-blocks-never-polls area=established-decisions rec=18
name: Nothing polls the channel
rule: `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
@row_add n=30 row=decision/identity-by-instance area=established-decisions rec=18,41
name: Identity by instance, no database keys
rule: Instances are identified by `ViolaName` and endpoints by a short hash of the instance; there are no database keys.
why: Identity is by instance, never by location (R5).
@row_add n=31 row=decision/hand-rolled-json-rpc-over-ndjson area=established-decisions rec=19
name: JSON-RPC 2.0 over ndjson, hand-rolled with serde
rule: The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one.
why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
@row_add n=32 row=decision/self-describing-frames-no-handshake area=established-decisions rec=19
name: Self-describing frames, no handshake
rule: Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake.
why: Mixed binary versions are a measured fact, so every frame self-describes.
@row_add n=33 row=decision/list-is-caller-side-read area=established-decisions rec=19,48
name: list is a caller-side read
rule: `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
@row_add n=34 row=decision/rmcp-stdio-mcp-server area=established-decisions rec=20
name: rmcp stdio MCP server
rule: rmcp, its minor pinned, is what serves MCP, over stdio.
why: The official SDK.
@row_add n=35 row=contract/mcp-tools/send area=established-decisions rec=20
boundary: MCP tools
item: send
promise: Offered as an MCP tool.
@row_add n=36 row=contract/mcp-tools/wait area=established-decisions rec=20
boundary: MCP tools
item: wait
promise: Offered as an MCP tool.
@row_add n=37 row=contract/mcp-tools/last area=established-decisions rec=20
boundary: MCP tools
item: last
promise: Offered as an MCP tool.
@row_add n=38 row=contract/mcp-tools/answer area=established-decisions rec=20
boundary: MCP tools
item: answer
promise: Offered as an MCP tool.
@row_add n=39 row=contract/mcp-tools/list area=established-decisions rec=20
boundary: MCP tools
item: list
promise: Offered as an MCP tool.
@row_add n=40 row=contract/mcp-tool-result/refusal area=established-decisions rec=20
boundary: MCP tool results
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_add n=41 row=decision/release-cli-only area=established-decisions rec=20
name: release is CLI-only
rule: `release` is never an MCP tool; only the CLI offers it.
why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
@row_add n=42 row=decision/pause-link-unlink-cli-only area=established-decisions rec=20
name: pause, link and unlink are CLI-only
rule: `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools.
why: They are human controls over the wheel and the link topology.
@row_add n=43 row=decision/mcp-stdin-line-cap area=established-decisions rec=20
name: MCP stdin read under a line cap
rule: The MCP server's stdin is read under a `MAX_FRAME` line cap, newline counted; a longer line is refused whole, never truncated or skipped, and ends the session with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it.
why: rmcp's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp.
@row_add n=44 row=decision/exec-form-hooks area=established-decisions rec=21
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`.
why: Exec form skips shell startup.
@row_add n=45 row=contract/hook-tiers/sync-with-decision area=established-decisions rec=21
boundary: hook tiers
item: sync with a decision
promise: PreToolUse `AskUserQuestion|ExitPlanMode` and PermissionRequest run synchronously and may return a decision.
@row_add n=46 row=contract/hook-tiers/sync-spine area=established-decisions rec=21
boundary: hook tiers
item: sync spine
promise: SessionStart, UserPromptSubmit and Stop run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
@row_add n=47 row=contract/hook-tiers/async area=established-decisions rec=21
boundary: hook tiers
item: async
promise: Every other hook runs asynchronously, printing nothing.
@row_add n=48 row=decision/lost-endpoint-loses-event area=established-decisions rec=21
name: A lost endpoint loses the event, not the turn
rule: When a hook cannot reach its endpoint, the hook event is lost and the turn goes on.
@row_add n=49 row=decision/hooks-fail-open area=established-decisions rec=22,55
name: Hooks fail open
rule: `viola hook` always exits 0, including on parse errors, channel failures and caught panics; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr.
why: Exit 2 blocks Claude Code, and a viola bug must never block the session.
@row_add n=50 row=decision/dialog-hold-bound area=established-decisions rec=22
name: Dialog hold bounds a sync dialog hook
rule: A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60 to 3600 s, default 60), after which the dialog renders for the human.
state: provisional
@row_add n=51 row=decision/wheel-starts-with-driver area=established-decisions rec=23
name: The wheel starts with the driver
rule: The wheel starts with `driver` at every start.
@row_add n=52 row=decision/wheel-byte-source-claim area=established-decisions rec=23
name: Byte-source claim of the wheel
rule: Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts.
why: `viola run` owns the real stdin, so it knows byte provenance.
@row_add n=53 row=decision/send-never-queued area=established-decisions rec=23
name: A send mid-turn is refused, never queued
rule: A `send` mid-turn or mid-send is refused, never queued.
@row_add n=54 row=decision/explicit-wheel-release area=established-decisions rec=23
name: Explicit wheel release
rule: The wheel returns from the human to the driver only through `viola release`.
@row_add n=55 row=decision/link-recording area=established-decisions rec=24
name: How a driver-to-driven link is recorded
rule: A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
@row_add n=56 row=decision/links-are-log-events area=established-decisions rec=24
name: Links are events in the log
rule: Links are events in the log, and the snapshot's link set is derived by replaying them.
why: The log stays the single source of truth for the link set.
@row_add n=57 row=decision/budget-soft-stop area=established-decisions rec=25
name: Budget soft stop at the turn boundary
rule: Once a configured threshold is crossed (defaults `five_hour` at 90 %, `seven_day` at 85 %), automated `send` is refused `budget-paused`; human keys are never blocked and a running turn finishes.
why: The per-model weekly window is hidden, so thresholds are set low.
@row_add n=58 row=decision/budget-pause-lift area=established-decisions rec=25
name: Budget pause lifts on reset or release
rule: A budget pause lifts when `resets_at` passes or on `viola release --budget`.
@row_add n=59 row=decision/budget-gate-fails-open area=established-decisions rec=25
name: Budget gate never polls and fails open
rule: The budget gate never polls and fails open.
@row_add n=60 row=decision/global-statusline-untouched area=established-decisions rec=25
name: Global statusline config untouched
rule: The user's global statusline config is never touched.
@row_add n=61 row=decision/verify-never-types-into-dialog area=established-decisions rec=26
name: viola verify never answers a CLI dialog
rule: No `viola verify` run writes a byte into a CLI dialog.
@row_add n=62 row=absence/graphical-frontend area=established-decisions rec=27,29
capability: A web or graphical frontend, page or graphical control surface
@row_add n=63 row=decision/terminal-fleet-console area=established-decisions rec=27,29
name: Terminal fleet console as the one view
rule: The one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland.
@row_add n=64 row=decision/liveness-heartbeat-and-pid area=established-decisions rec=28
name: Heartbeat-first session liveness
rule: The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it.
why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
@row_add n=65 row=decision/no-start-over-live-instance area=established-decisions rec=28
name: No start over a live or stale instance
rule: A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
@row_add n=66 row=decision/revive-in-place area=established-decisions rec=28,4
name: Revive resumes in place through the start path
rule: A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
@row_add n=67 row=decision/chrono-utc-millisecond-timestamps area=established-decisions rec=30
name: chrono timestamps, RFC 3339 UTC, milliseconds
rule: chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision.
why: chrono is already in the tree through rmcp.
@row_add n=68 row=decision/tolerant-resets-at area=established-decisions rec=30
name: External resets_at parsed tolerantly
rule: An external `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error.
why: External clocks must never break parsing.
@row_add n=69 row=decision/tolerant-external-payloads area=established-decisions rec=31
name: External payloads parsed tolerantly
rule: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options, the raw value kept for the fixture recorder and serde_path_to_error for drift reports.
why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
@row_add n=70 row=decision/versioned-tolerant-own-formats area=established-decisions rec=31
name: Own formats versioned-tolerant
rule: Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields.
why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
@row_add n=71 row=decision/nutype-newtypes area=established-decisions rec=31
name: nutype for validated newtypes
rule: nutype newtypes (`ViolaName`, `Percent`, `DialogHold`) do validation; no garde or validator.
@row_add n=72 row=decision/thiserror-per-crate-anyhow-in-bin area=established-decisions rec=32
name: thiserror per crate, anyhow only in the bin
rule: Each workspace crate has one thiserror error enum; anyhow is used only in the `viola` bin.
why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr.
except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
@row_add n=73 row=decision/flat-cargo-workspace area=established-decisions rec=33
name: Flat Cargo workspace with the bin at the root
rule: One Cargo workspace with a flat `crates/` layout, the root `Cargo.toml` being both the `viola` bin package and the workspace; `fuzz/` is its own workspace.
why: The compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.
@row_add n=74 row=decision/plugin-compiled-into-binary area=established-decisions rec=34
name: Plugin compiled into the binary
rule: viola is installed by `cargo install --path .`, with the plugin compiled into the binary.
why: The binary and the plugin can no longer drift apart.
@row_add n=75 row=decision/pinned-exe-copy area=established-decisions rec=34
name: Pinned exe copy keyed by version and hash
rule: At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, and rewrites the plugin for that copy, loaded with `--plugin-dir`.
why: The content hash keys the copy because the version alone collides across rebuilds.
@row_add n=76 row=decision/no-path-lookup area=established-decisions rec=34,35
name: Hooks and MCP never search PATH
rule: Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
@row_add n=77 row=decision/plugin-only-in-wrapped-sessions area=established-decisions rec=35
name: Plugin hooks only in wrapped sessions
rule: The plugin with its hooks loads only into wrapped sessions via `--plugin-dir`.
why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola.
except: `viola verify`'s transient unwrapped children.
@row_add n=78 row=decision/optional-plugin-install area=established-decisions rec=35
name: Optional MCP-only plugin install
rule: `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver, and the installed plugin carries only `.mcp.json` and no hooks.
@row_add n=79 row=decision/cli-output-conventions area=established-decisions rec=36
name: Human text by default, --json for agents
rule: The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal.
why: Agents need machine-parseable refusals.
@row_add n=80 row=decision/prompt-text-from-stdin-or-file area=established-decisions rec=36
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument.
@row_add n=81 row=decision/ci-against-fake-agent area=established-decisions rec=37
name: CI on ubuntu-latest against the fake agent
rule: GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests.
why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
@row_add n=82 row=decision/fake-agent-after-double-dash area=established-decisions rec=37
name: Fake agent named after --
rule: The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
@row_add n=83 row=decision/child-program-resolution area=established-decisions rec=37,9
name: viola resolves the child program itself
rule: viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
@row_add n=84 row=decision/test-isolated-viola-home area=established-decisions rec=37
name: Each test in its own viola home
rule: Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
@row_add n=85 row=decision/no-hosting area=established-decisions rec=38,7
name: No hosting
rule: viola has no hosting; Docker, serverless and PaaS do not apply.
why: v1 is local-only.
@row_add n=86 row=absence/mobile-view area=established-decisions rec=39
capability: A mobile or phone view
@row_add n=87 row=decision/no-model-api-calls area=established-decisions rec=40
name: No model API calls
rule: The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI.
why: D2, subscription only through the unmodified `claude` CLI.
@row_add n=88 row=decision/viola-name-prefix area=established-decisions rec=42
name: viola prefix for names
rule: The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`).
why: One name everywhere removes the need to translate between prototype and product.
except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
@row_none n=2 rec=43 type=9
@row_none n=3 rec=44 type=S
@row_add n=89 row=decision/four-interfaces-one-binary area=conventions rec=45
name: Four interfaces from one binary
rule: The binary exposes four interfaces: CLI verbs, MCP tools over stdio, the hook stdin/stdout contract, and the wrapper channel; there is no REST write surface, GraphQL, gRPC or tRPC.
@row_add n=90 row=contract/protocol-version/v area=conventions rec=46
boundary: protocol versioning
item: v
promise: An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
@row_add n=91 row=decision/protocol-version-bump-rule area=conventions rec=46
name: When the protocol version bumps
rule: Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.
@row_add n=92 row=contract/channel-params/sender area=conventions rec=47
boundary: channel params
item: sender
promise: Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
@row_add n=93 row=contract/snapshot/writer area=conventions rec=47
boundary: snapshot
item: writer
promise: Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
@row_add n=94 row=contract/channel-methods/send area=conventions rec=48
boundary: channel methods
item: send
promise: A request, carrying an integer id monotonic per connection.
@row_add n=95 row=contract/channel-methods/wait area=conventions rec=48
boundary: channel methods
item: wait
promise: A request, carrying an integer id monotonic per connection.
@row_add n=96 row=contract/channel-methods/last area=conventions rec=48
boundary: channel methods
item: last
promise: A request, carrying an integer id monotonic per connection.
@row_add n=97 row=contract/channel-methods/answer area=conventions rec=48
boundary: channel methods
item: answer
promise: A request, carrying an integer id monotonic per connection.
@row_add n=98 row=contract/channel-methods/pause area=conventions rec=48
boundary: channel methods
item: pause
promise: A request, carrying an integer id monotonic per connection.
@row_add n=99 row=contract/channel-methods/release area=conventions rec=48
boundary: channel methods
item: release
promise: A request, carrying an integer id monotonic per connection.
@row_add n=100 row=contract/channel-methods/link area=conventions rec=48
boundary: channel methods
item: link
promise: A request, carrying an integer id monotonic per connection.
@row_add n=101 row=contract/channel-methods/unlink area=conventions rec=48
boundary: channel methods
item: unlink
promise: A request, carrying an integer id monotonic per connection.
@row_add n=102 row=contract/channel-methods/hook-dialog area=conventions rec=48
boundary: channel methods
item: hook.dialog
promise: A request, carrying an integer id monotonic per connection.
@row_add n=103 row=contract/channel-methods/hook-event area=conventions rec=48
boundary: channel methods
item: hook.event
promise: A notification, carrying no id.
@row_none n=4 rec=49 type=S
@row_add n=104 row=contract/channel-response/refusal area=conventions rec=50,19
boundary: channel response
item: refusal
promise: A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
@row_add n=105 row=contract/channel-response/ok area=conventions rec=51
boundary: channel response
item: ok
promise: Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
@row_add n=106 row=contract/channel-response/error area=conventions rec=52,19
boundary: channel response
item: error
promise: Reserved for protocol faults: `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}`.
@row_add n=107 row=contract/channel-fault-codes/-32700 area=conventions rec=52
boundary: channel protocol fault codes
item: -32700
promise: Parse error.
@row_add n=108 row=contract/channel-fault-codes/-32600 area=conventions rec=52
boundary: channel protocol fault codes
item: -32600
promise: Invalid request.
@row_add n=109 row=contract/channel-fault-codes/-32601 area=conventions rec=52
boundary: channel protocol fault codes
item: -32601
promise: Unknown method.
@row_add n=110 row=contract/channel-fault-codes/-32602 area=conventions rec=52,46
boundary: channel protocol fault codes
item: -32602
promise: Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
@row_add n=111 row=contract/channel-fault-codes/-32603 area=conventions rec=52
boundary: channel protocol fault codes
item: -32603
promise: Internal error.
@row_add n=112 row=contract/refusal-reason/human-typing area=conventions rec=53
boundary: RefusalReason
item: human-typing
promise: A refusal reason, serialised kebab-case, its detail from a closed set.
@row_add n=113 row=contract/refusal-reason/budget-paused area=conventions rec=53
boundary: RefusalReason
item: budget-paused
promise: A refusal reason, serialised kebab-case, its detail from a closed set.
@row_add n=114 row=contract/refusal-reason/unverified-cli area=conventions rec=53
boundary: RefusalReason
item: unverified-cli
promise: A refusal reason, serialised kebab-case, its detail from a closed set.
@row_add n=115 row=contract/refusal-reason/not-delivered area=conventions rec=53
boundary: RefusalReason
item: not-delivered
promise: A refusal reason, serialised kebab-case, its detail from a closed set.
@row_add n=116 row=contract/refusal-reason/unknown area=conventions rec=53
boundary: RefusalReason
item: `unknown` refusal reason
promise: The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
@row_add n=117 row=decision/refusal-details-closed-set area=conventions rec=53
name: Refusal details from a closed set
rule: Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
@row_add n=118 row=decision/first-refusal-only area=conventions rec=53
name: Only the first refusal returned
rule: Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
@row_add n=119 row=decision/control-characters-refused area=conventions rec=53
name: Control characters refused, never stripped
rule: Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
@row_add n=120 row=contract/send-ok-payload/unconfirmable area=conventions rec=53,2
boundary: send ok payload
item: unconfirmable
promise: Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
@row_add n=121 row=contract/cli-exit-codes/0 area=conventions rec=54
boundary: CLI exit codes
item: 0
promise: ok.
@row_add n=122 row=contract/cli-exit-codes/1 area=conventions rec=54
boundary: CLI exit codes
item: 1
promise: Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them.
@row_add n=123 row=contract/cli-exit-codes/2 area=conventions rec=54
boundary: CLI exit codes
item: 2
promise: Usage error.
@row_add n=124 row=contract/cli-exit-codes/10 area=conventions rec=54
boundary: CLI exit codes
item: 10
promise: Refused `human-typing`.
@row_add n=125 row=contract/cli-exit-codes/11 area=conventions rec=54
boundary: CLI exit codes
item: 11
promise: Refused `budget-paused`.
@row_add n=126 row=contract/cli-exit-codes/12 area=conventions rec=54
boundary: CLI exit codes
item: 12
promise: Refused `unverified-cli`.
@row_add n=127 row=contract/cli-exit-codes/13 area=conventions rec=54
boundary: CLI exit codes
item: 13
promise: Refused `not-delivered`.
@row_add n=128 row=contract/cli-exit-codes/14 area=conventions rec=54
boundary: CLI exit codes
item: 14
promise: Refused `unknown`.
@row_add n=129 row=contract/cli-exit-codes/20 area=conventions rec=54
boundary: CLI exit codes
item: 20
promise: Protocol fault from the wrapper.
@row_add n=130 row=contract/cli-exit-codes/21 area=conventions rec=54
boundary: CLI exit codes
item: 21
promise: Instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a CLI frame verb, or a refused socket directory of a frame verb, `viola link` or `viola unlink`.
@row_add n=131 row=contract/viola-hook-exit-codes/0 area=conventions rec=55
boundary: viola hook exit codes
item: 0
promise: Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
