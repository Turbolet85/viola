@row_add n=1 row=decision/mechanism-not-policy area=design-philosophy rec=fct-v73h4for@1,fct-pdkjglgn@1
name: Mechanism, not policy: a general bridge
rule: viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb.
why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
@row_add n=2 row=decision/send-confirmed-after-the-fact area=design-philosophy rec=fct-24sxrnlw@1,fct-i4ogppls@1
name: Every send confirmed after the fact
rule: A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash.
why: A turn boundary per the hooks does not prove the input box is ready.
except: A ledger-listed local command whose post-condition is none, not yet measured, or whose CLI is unverified returns `ok` with `confirmed: false` and detail `unconfirmable`.
@row_add n=3 row=decision/capability-ledger area=design-philosophy rec=fct-24sxrnlw@1,fct-7vnaqpcq@1
name: Version-stamped capability ledger
rule: Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_add n=4 row=decision/human-always-wins-wheel area=design-philosophy rec=fct-5pxykib3@1
name: The human always wins the wheel
rule: Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
@row_add n=5 row=decision/no-daemon-disk-is-shared-truth area=design-philosophy rec=fct-efluvsb5@1
name: No daemon; the disk is the shared truth
rule: Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
@row_add n=6 row=decision/no-tmux-or-shell-dependency area=design-philosophy rec=fct-igbvtfv6@2
name: No dependence on tmux or a shell
rule: No design choice may depend on tmux or a shell.
@row_add n=7 row=overview/stack-and-technologies area=stack-and-technologies rec=fct-zegjs5rw@3
text: viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.
@row_add n=8 row=decision/personal-v1-no-accounts area=established-decisions rec=fct-hcu7z4ke@2
name: Personal v1 without accounts
rule: v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively.
why: The later public version for individual Claude Code subscribers must not need a rewrite.
@row_add n=9 row=decision/claude-first-no-adapter-trait area=established-decisions rec=fct-7tdo3mcl@2
name: Claude-first, one agent crate, no adapter trait
rule: All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events.
why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
@row_add n=10 row=decision/human-key-never-held-behind-resize area=established-decisions rec=fct-omwrdnlc@3
name: Human key never held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize.
@row_add n=11 row=decision/no-da1-answer area=established-decisions rec=fct-omwrdnlc@3
name: viola never answers DA1
rule: viola never answers a DA1 query itself.
@row_none n=1 rec=fct-bq65okf4@3 type=9
@row_add n=12 row=contract/not-delivered-details/input-not-ready area=established-decisions rec=fct-hsjl3wtp@1
boundary: not-delivered refusal details
item: input-not-ready
promise: A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
@row_add n=13 row=contract/not-delivered-details/no-prompt-submitted area=established-decisions rec=fct-i4ogppls@1
boundary: not-delivered refusal details
item: no-prompt-submitted
promise: A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
@row_add n=14 row=decision/send-text-line-endings area=established-decisions rec=fct-qevnbocy@1
name: Send text line-ending normalisation
rule: A `send` types its text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters.
@row_add n=15 row=contract/not-delivered-details/empty-text area=established-decisions rec=fct-qevnbocy@1
boundary: not-delivered refusal details
item: empty-text
promise: A send whose typed text is left empty is refused `not-delivered` with this detail, nothing issued.
@row_add n=16 row=decision/torn-last-line-healed area=established-decisions rec=fct-73drmylz@2
name: Torn log line healed by the next append
rule: Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_add n=17 row=decision/private-mode-before-write area=established-decisions rec=fct-2sdvn2pr@1
name: File mode set before any byte is written
rule: A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
@row_add n=18 row=decision/pinned-copy-never-replaced area=established-decisions rec=fct-2sdvn2pr@1
name: Pinned copy never replaced
rule: The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
@row_add n=19 row=decision/interprocess-local-socket-channel area=established-decisions rec=fct-w2igrcse@2
name: interprocess local sockets behind the channel seam
rule: interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_add n=20 row=decision/channel-blocks-never-polls area=established-decisions rec=fct-w2igrcse@2
name: Nothing polls the channel
rule: `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
@row_add n=21 row=decision/hand-rolled-json-rpc-over-ndjson area=established-decisions rec=fct-lhug4cke@1
name: JSON-RPC 2.0 over ndjson, hand-rolled with serde
rule: The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one.
why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
@row_add n=22 row=decision/self-describing-frames-no-handshake area=established-decisions rec=fct-lhug4cke@1
name: Self-describing frames, no handshake
rule: Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake.
why: Mixed binary versions are a measured fact, so every frame self-describes.
@row_add n=23 row=decision/list-is-caller-side-read area=established-decisions rec=fct-lhug4cke@1,fct-4ghn4t2o@1
name: list is a caller-side read
rule: `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
@row_add n=24 row=decision/rmcp-stdio-mcp-server area=established-decisions rec=fct-tkwxvt3j@1
name: rmcp stdio MCP server
rule: rmcp, its minor pinned, is what serves MCP, over stdio.
why: The official SDK.
@row_add n=25 row=contract/mcp-tool-result/refusal area=established-decisions rec=fct-tkwxvt3j@1
boundary: MCP tool results
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_add n=26 row=decision/release-cli-only area=established-decisions rec=fct-tkwxvt3j@1
name: release is CLI-only
rule: `release` is never an MCP tool; only the CLI offers it.
why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
@row_add n=27 row=decision/pause-link-unlink-cli-only area=established-decisions rec=fct-tkwxvt3j@1
name: pause, link and unlink are CLI-only
rule: `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools.
why: They are human controls over the wheel and the link topology.
@row_add n=28 row=decision/mcp-stdin-line-cap area=established-decisions rec=fct-tkwxvt3j@1
name: MCP stdin read under a line cap
rule: The MCP server's stdin is read under a `MAX_FRAME` line cap, newline counted; a longer line is refused whole, never truncated or skipped, and ends the session with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it.
why: rmcp's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp.
@row_add n=29 row=decision/exec-form-hooks area=established-decisions rec=fct-zhr3wx2i@2
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`.
why: Exec form skips shell startup.
@row_add n=30 row=contract/hook-tiers/sync-spine area=established-decisions rec=fct-zhr3wx2i@2
boundary: hook tiers
item: sync spine
promise: SessionStart, UserPromptSubmit and Stop run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
@row_add n=31 row=decision/lost-endpoint-loses-event area=established-decisions rec=fct-zhr3wx2i@2
name: A lost endpoint loses the event, not the turn
rule: When a hook cannot reach its endpoint, the hook event is lost and the turn goes on.
@row_add n=32 row=decision/dialog-hold-bound area=established-decisions rec=fct-4nt6qtyp@3
name: Dialog hold bounds a sync dialog hook
rule: A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60 to 3600 s, default 60), after which the dialog renders for the human.
state: provisional
@row_add n=33 row=decision/wheel-byte-source-claim area=established-decisions rec=fct-dl246x3d@1
name: Byte-source claim of the wheel
rule: Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts.
why: `viola run` owns the real stdin, so it knows byte provenance.
@row_add n=34 row=decision/explicit-wheel-release area=established-decisions rec=fct-dl246x3d@1
name: Explicit wheel release
rule: The wheel returns from the human to the driver only through `viola release`.
@row_add n=35 row=decision/link-recording area=established-decisions rec=fct-3iyhqr2o@1
name: How a driver-to-driven link is recorded
rule: A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
@row_add n=36 row=decision/links-are-log-events area=established-decisions rec=fct-3iyhqr2o@1
name: Links are events in the log
rule: Links are events in the log, and the snapshot's link set is derived by replaying them.
why: The log stays the single source of truth for the link set.
@row_add n=37 row=decision/budget-soft-stop area=established-decisions rec=fct-3hy3yfb4@1
name: Budget soft stop at the turn boundary
rule: Once a configured threshold is crossed (defaults `five_hour` at 90 %, `seven_day` at 85 %), automated `send` is refused `budget-paused`; human keys are never blocked and a running turn finishes.
why: The per-model weekly window is hidden, so thresholds are set low.
@row_add n=38 row=decision/budget-pause-lift area=established-decisions rec=fct-3hy3yfb4@1
name: Budget pause lifts on reset or release
rule: A budget pause lifts when `resets_at` passes or on `viola release --budget`.
@row_add n=39 row=decision/budget-gate-fails-open area=established-decisions rec=fct-3hy3yfb4@1
name: Budget gate never polls and fails open
rule: The budget gate never polls and fails open.
@row_add n=40 row=decision/global-statusline-untouched area=established-decisions rec=fct-3hy3yfb4@1
name: Global statusline config untouched
rule: The user's global statusline config is never touched.
@row_add n=41 row=decision/verify-never-types-into-dialog area=established-decisions rec=fct-7vnaqpcq@1
name: viola verify never answers a CLI dialog
rule: No `viola verify` run writes a byte into a CLI dialog.
@row_add n=42 row=absence/graphical-frontend area=established-decisions rec=fct-jn2hd4ac@2,fct-7wwoxbip@2
capability: A web or graphical frontend, page or graphical control surface
@row_add n=43 row=decision/terminal-fleet-console area=established-decisions rec=fct-jn2hd4ac@2,fct-7wwoxbip@2
name: Terminal fleet console as the one view
rule: The one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland.
@row_add n=44 row=decision/liveness-heartbeat-and-pid area=established-decisions rec=fct-aos27ydg@1
name: Heartbeat-first session liveness
rule: The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it.
why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
@row_add n=45 row=decision/no-start-over-live-instance area=established-decisions rec=fct-aos27ydg@1
name: No start over a live or stale instance
rule: A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
@row_add n=46 row=decision/revive-in-place area=established-decisions rec=fct-aos27ydg@1,fct-efluvsb5@1
name: Revive resumes in place through the start path
rule: A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
@row_add n=47 row=decision/chrono-utc-millisecond-timestamps area=established-decisions rec=fct-qx4of5bv@1
name: chrono timestamps, RFC 3339 UTC, milliseconds
rule: chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision.
why: chrono is already in the tree through rmcp.
@row_add n=48 row=decision/tolerant-resets-at area=established-decisions rec=fct-qx4of5bv@1
name: External resets_at parsed tolerantly
rule: An external `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error.
why: External clocks must never break parsing.
@row_add n=49 row=decision/tolerant-external-payloads area=established-decisions rec=fct-caebuvjq@1
name: External payloads parsed tolerantly
rule: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options, the raw value kept for the fixture recorder and serde_path_to_error for drift reports.
why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
@row_add n=50 row=decision/versioned-tolerant-own-formats area=established-decisions rec=fct-caebuvjq@1
name: Own formats versioned-tolerant
rule: Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields.
why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
@row_add n=51 row=decision/thiserror-per-crate-anyhow-in-bin area=established-decisions rec=fct-sha7o35z@2
name: thiserror per crate, anyhow only in the bin
rule: Each workspace crate has one thiserror error enum; anyhow is used only in the `viola` bin.
why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr.
except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
@row_add n=52 row=decision/flat-cargo-workspace area=established-decisions rec=fct-brnw2t5e@1
name: Flat Cargo workspace with the bin at the root
rule: One Cargo workspace with a flat `crates/` layout, the root `Cargo.toml` being both the `viola` bin package and the workspace; `fuzz/` is its own workspace.
why: The compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.
@row_add n=53 row=decision/no-path-lookup area=established-decisions rec=fct-c3wssf3l@2,fct-le5sc2tk@1
name: Hooks and MCP never search PATH
rule: Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
@row_add n=54 row=decision/optional-plugin-install area=established-decisions rec=fct-le5sc2tk@1
name: Optional MCP-only plugin install
rule: `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver, and the installed plugin carries only `.mcp.json` and no hooks.
@row_add n=55 row=decision/cli-output-conventions area=established-decisions rec=fct-nvft5tqc@2
name: Human text by default, --json for agents
rule: The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal.
why: Agents need machine-parseable refusals.
@row_add n=56 row=decision/ci-against-fake-agent area=established-decisions rec=fct-bvzxxcy7@2
name: CI on ubuntu-latest against the fake agent
rule: GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests.
why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
@row_add n=57 row=decision/fake-agent-after-double-dash area=established-decisions rec=fct-bvzxxcy7@2
name: Fake agent named after --
rule: The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
@row_add n=58 row=decision/child-program-resolution area=established-decisions rec=fct-bvzxxcy7@2,fct-7tdo3mcl@2
name: viola resolves the child program itself
rule: viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
@row_add n=59 row=decision/test-isolated-viola-home area=established-decisions rec=fct-bvzxxcy7@2
name: Each test in its own viola home
rule: Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
@row_add n=60 row=decision/no-hosting area=established-decisions rec=fct-mg4wkqys@1,fct-hcu7z4ke@2
name: No hosting
rule: viola has no hosting; Docker, serverless and PaaS do not apply.
why: v1 is local-only.
@row_add n=61 row=decision/no-model-api-calls area=established-decisions rec=fct-bdlakgwt@1
name: No model API calls
rule: The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI.
why: D2, subscription only through the unmodified `claude` CLI.
@row_add n=62 row=decision/viola-name-prefix area=established-decisions rec=fct-5ohkl26i@1
name: viola prefix for names
rule: The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`).
why: One name everywhere removes the need to translate between prototype and product.
except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
@row_none n=2 rec=fct-jqic6r22@2 type=9
@row_none n=3 rec=fct-gevx5agt@1 type=S
@row_add n=63 row=decision/four-interfaces-one-binary area=conventions rec=fct-ynzkvjxk@1
name: Four interfaces from one binary
rule: The binary exposes four interfaces: CLI verbs, MCP tools over stdio, the hook stdin/stdout contract, and the wrapper channel; there is no REST write surface, GraphQL, gRPC or tRPC.
@row_add n=64 row=contract/protocol-version/v area=conventions rec=fct-hhwkhtaq@1
boundary: protocol versioning
item: v
promise: An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
@row_add n=65 row=decision/protocol-version-bump-rule area=conventions rec=fct-hhwkhtaq@1
name: When the protocol version bumps
rule: Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.
@row_add n=66 row=contract/channel-params/sender area=conventions rec=fct-gajhyfaa@1
boundary: channel params
item: sender
promise: Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
@row_add n=67 row=contract/snapshot/writer area=conventions rec=fct-gajhyfaa@1
boundary: snapshot
item: writer
promise: Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
@row_none n=4 rec=fct-cdd57vln@1 type=S
@row_add n=68 row=contract/channel-response/refusal area=conventions rec=fct-wfq6mlyt@1,fct-lhug4cke@1
boundary: channel response
item: refusal
promise: A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
@row_add n=69 row=contract/channel-response/ok area=conventions rec=fct-5z4gbncz@1
boundary: channel response
item: ok
promise: Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
@row_add n=70 row=contract/channel-response/error area=conventions rec=fct-h4oc7sqh@1,fct-lhug4cke@1
boundary: channel response
item: error
promise: Reserved for protocol faults: `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}`.
@row_add n=71 row=contract/channel-fault-codes/-32700 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32700
promise: Parse error.
@row_add n=72 row=contract/channel-fault-codes/-32600 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32600
promise: Invalid request.
@row_add n=73 row=contract/channel-fault-codes/-32601 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32601
promise: Unknown method.
@row_add n=74 row=contract/channel-fault-codes/-32602 area=conventions rec=fct-h4oc7sqh@1,fct-hhwkhtaq@1
boundary: channel protocol fault codes
item: -32602
promise: Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
@row_add n=75 row=contract/channel-fault-codes/-32603 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32603
promise: Internal error.
@row_add n=76 row=contract/refusal-reason/unknown area=conventions rec=fct-xscdpgpr@1
boundary: RefusalReason
item: `unknown` refusal reason
promise: The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
@row_add n=77 row=decision/refusal-details-closed-set area=conventions rec=fct-xscdpgpr@1
name: Refusal details from a closed set
rule: Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
@row_add n=78 row=decision/first-refusal-only area=conventions rec=fct-xscdpgpr@1
name: Only the first refusal returned
rule: Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
@row_add n=79 row=decision/control-characters-refused area=conventions rec=fct-xscdpgpr@1
name: Control characters refused, never stripped
rule: Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
@row_add n=80 row=contract/send-ok-payload/unconfirmable area=conventions rec=fct-xscdpgpr@1,fct-24sxrnlw@1
boundary: send ok payload
item: unconfirmable
promise: Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
@row_add n=81 row=contract/viola-hook-exit-codes/0 area=conventions rec=fct-nppkdemz@1
boundary: viola hook exit codes
item: 0
promise: Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
@row_add n=82 row=contract/mcp-tool-results/is-error-true area=conventions rec=fct-5nrjpmo2@1
boundary: MCP tool results
item: isError true
promise: A refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`.
@row_add n=83 row=decision/one-error-enum-per-crate area=conventions rec=fct-nhsg4k7g@2
name: One error enum per crate
rule: Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages.
except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
@row_add n=84 row=decision/nutype-errors-fold-into-crate-enum area=conventions rec=fct-nhsg4k7g@2
name: nutype validation errors fold into the crate enum
rule: A nutype validation error is folded into the owning crate's error enum.
@row_add n=85 row=decision/anyhow-only-in-root-bin area=conventions rec=fct-nhsg4k7g@2
name: anyhow only in the root bin crate
rule: `anyhow` appears only in the root bin crate.
@row_none n=5 rec=fct-oizocdnn@1 type=S
@row_add n=86 row=decision/crate-naming area=conventions rec=fct-md27i7yu@1
name: Crate naming
rule: Crates are named `viola-<area>`, kebab-case.
@row_add n=87 row=decision/standard-rust-naming area=conventions rec=fct-md27i7yu@1
name: Standard Rust naming
rule: Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
@row_add n=88 row=decision/json-field-names-snake-case area=conventions rec=fct-osx5axgl@1
name: JSON field names in snake_case
rule: JSON field names are snake_case in every viola format.
except: `last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them.
@row_add n=89 row=decision/wire-enum-values-kebab-case area=conventions rec=fct-vnzcvfvg@1
name: Enum string values on the wire in kebab-case
rule: Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
@row_add n=90 row=contract/normalised-event-kinds/session-start area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: session-start
promise: `session-start` is a normalised event kind.
@row_add n=91 row=contract/normalised-event-kinds/turn-ended area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: turn-ended
promise: `turn-ended` is a normalised event kind.
@row_add n=92 row=contract/normalised-event-kinds/prompt-submitted area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: prompt-submitted
promise: `prompt-submitted` is a normalised event kind.
@row_add n=93 row=contract/normalised-event-kinds/question area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: question
promise: `question` is a normalised event kind.
@row_add n=94 row=contract/normalised-event-kinds/permission area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: permission
promise: `permission` is a normalised event kind.
@row_add n=95 row=contract/normalised-event-kinds/plan area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: plan
promise: `plan` is a normalised event kind.
@row_add n=96 row=contract/normalised-event-kinds/session-end area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: session-end
promise: `session-end` is a normalised event kind.
@row_add n=97 row=contract/normalised-event-kinds/activity area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: activity
promise: `activity` is a normalised event kind.
@row_add n=98 row=contract/normalised-event-kinds/link area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: link
promise: `link` is a normalised event kind.
@row_add n=99 row=contract/normalised-event-kinds/unlink area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: unlink
promise: `unlink` is a normalised event kind.
@row_add n=100 row=contract/normalised-event-kinds/wheel area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: wheel
promise: `wheel` is a normalised event kind.
@row_add n=101 row=contract/normalised-event-kinds/budget-gate area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: budget-gate
promise: `budget-gate` is a normalised event kind.
@row_add n=102 row=contract/normalised-event-kinds/send-issued area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-issued
promise: `send-issued` is a normalised event kind.
@row_add n=103 row=contract/normalised-event-kinds/send-confirmed area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-confirmed
promise: `send-confirmed` is a normalised event kind.
@row_add n=104 row=contract/normalised-event-kinds/send-refused area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-refused
promise: `send-refused` is a normalised event kind.
@row_add n=105 row=contract/normalised-event-kinds/dialog-expired area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: dialog-expired
promise: `dialog-expired` is a normalised event kind.
@row_add n=106 row=decision/event-kinds-agent-neutral area=conventions rec=fct-5pojuecq@1
name: Event kinds added only in viola-core
rule: A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
@row_add n=107 row=contract/hook-to-kind-map/sessionstart area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: SessionStart
promise: SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
@row_add n=108 row=contract/hook-to-kind-map/userpromptsubmit area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: UserPromptSubmit
promise: UserPromptSubmit maps to `prompt-submitted`.
@row_add n=109 row=contract/hook-to-kind-map/pretooluse-askuserquestion area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PreToolUse AskUserQuestion
promise: PreToolUse `AskUserQuestion` maps to `question`.
@row_add n=110 row=contract/hook-to-kind-map/pretooluse-exitplanmode area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PreToolUse ExitPlanMode
promise: PreToolUse `ExitPlanMode` maps to `plan`.
@row_add n=111 row=contract/hook-to-kind-map/permissionrequest area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PermissionRequest
promise: PermissionRequest maps to `permission`, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
@row_add n=112 row=contract/hook-to-kind-map/stop area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: Stop
promise: Stop maps to `turn-ended`.
@row_add n=113 row=contract/hook-to-kind-map/sessionend area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: SessionEnd
promise: SessionEnd maps to `session-end`.
@row_add n=114 row=contract/hook-to-kind-map/notification area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: Notification
promise: Notification maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=115 row=contract/hook-to-kind-map/posttooluse area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PostToolUse
promise: PostToolUse maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=116 row=contract/hook-to-kind-map/posttoolusefailure area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PostToolUseFailure
promise: PostToolUseFailure maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=117 row=contract/hook-to-kind-map/hook-statusline area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: hook statusline
promise: `hook statusline` emits no event.
@row_add n=118 row=decision/env-var-viola-prefix area=conventions rec=fct-qqxg3ika@1
name: Environment variables carry the VIOLA_ prefix
rule: Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE.
except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM` and `COLORTERM`, read never set, only to decide human output's SGR bytes.
@row_add n=119 row=decision/harness-env-var-prefix area=conventions rec=fct-qqxg3ika@1
name: Test-harness variables carry the AGENT_RUN_ prefix
rule: Test-harness-only environment variables use the `AGENT_RUN_` prefix and are never read by the `viola` binary.
@row_add n=120 row=decision/fake-agent-test-seams area=conventions rec=fct-qqxg3ika@1
name: Fake-agent test seam variables
rule: The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds.
why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
@row_add n=121 row=decision/on-disk-file-naming area=conventions rec=fct-2ejf4aiq@1
name: On-disk file naming
rule: On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
@row_add n=122 row=decision/cli-flag-and-subcommand-style area=conventions rec=fct-7ikxzg5k@1
name: CLI flag and subcommand style
rule: CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
@row_add n=123 row=contract/cli-verbs/send area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: send
promise: `viola send <target>` takes its text from stdin or `--file`.
@row_add n=124 row=contract/cli-verbs/wait area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: wait
promise: `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
@row_add n=125 row=contract/cli-verbs/last area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: last
promise: `viola last <target>`.
@row_add n=126 row=contract/cli-verbs/answer area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: answer
promise: `viola answer <target> <dialog_id>` takes the `response` object as JSON from stdin or `--file`.
@row_add n=127 row=contract/cli-verbs/pause area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: pause
promise: `viola pause <target>`.
@row_add n=128 row=contract/cli-verbs/release area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: release
promise: `viola release <target> [--budget]`.
@row_add n=129 row=contract/cli-verbs/link area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: link
promise: `viola link <driver> <driven>`.
@row_add n=130 row=contract/cli-verbs/unlink area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: unlink
promise: `viola unlink <driver> <driven>`.
@row_add n=131 row=contract/cli-verbs/revive area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: revive
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_add n=132 row=contract/cli-verbs/from area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: from
promise: Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
@row_none n=6 rec=fct-lh3i55lk@1 type=S
@row_add n=133 row=decision/viola-name-instance-key area=conventions rec=fct-tnxtyezf@1
name: ViolaName as the instance key
rule: The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype.
why: No database, so no UUID or serial keys.
@row_add n=134 row=decision/endpoint-name-stable-hash area=conventions rec=fct-zcqzutrw@1
name: Endpoint name from a stable hand-written hash
rule: The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden.
why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
@row_add n=135 row=decision/timestamp-field-names area=conventions rec=fct-u2cu7pdd@1
name: Timestamp field names
rule: Every event and frame carries its instant in the field `ts`; other instants use `*_at` fields.
@row_add n=136 row=decision/timestamp-format area=conventions rec=fct-u2cu7pdd@1
name: Timestamp format
rule: Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
@row_add n=137 row=decision/absent-optional-fields-omitted area=conventions rec=fct-u5xw5irn@1
name: Absent optional fields omitted on write
rule: viola omits an absent optional field on write rather than writing `null`.
@row_add n=138 row=decision/missing-equals-null-on-read area=conventions rec=fct-u5xw5irn@1
name: Missing and null read the same
rule: Readers treat a missing field and a `null` field the same.
@row_add n=139 row=decision/unparseable-external-values-unknown area=conventions rec=fct-iskzo3uv@1
name: Unparseable external values become unknown
rule: An unparseable external value becomes the string `"unknown"` in viola's own records, never an error, covering `resets_at`, `used_percentage` and a missing statusline window.
@row_add n=140 row=decision/percent-newtype area=conventions rec=fct-xomyjm5b@1
name: Percentages as a 0 to 100 newtype
rule: A percentage is the `Percent` newtype, 0 to 100, serialised as a JSON number.
except: An external budget reading's `used_percentage` is a reading kept only from 0 to 100 and otherwise the word `"unknown"`.
@row_add n=141 row=decision/ndjson-line-discipline area=conventions rec=fct-2ajbpybj@1
name: ndjson line discipline
rule: Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
@row_add n=142 row=absence/graphical-interface area=standard-contracts rec=fct-ygzlcqsl@2
capability: A web or graphical surface
@row_add n=143 row=decision/list-without-pagination area=standard-contracts rec=fct-oy7hacpg@1
name: list without pagination
rule: The `list` payload has no pagination; lists are returned whole.
why: Lists are personal-scale.
@row_add n=144 row=contract/list-payload/skipped area=standard-contracts rec=fct-oy7hacpg@1
boundary: list payload
item: skipped
promise: `skipped` counts records a reader could not interpret, so the board can show them.
@row_add n=145 row=contract/list-payload/items area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: items
promise: Each `items` entry is `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
@row_add n=146 row=contract/list-payload/liveness area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: liveness
promise: An entry's `liveness` is `live` or `stale`; gone instances are omitted.
@row_add n=147 row=contract/list-payload/wrapped area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: wrapped
promise: `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`.
@row_add n=148 row=contract/event-line/shape area=standard-contracts rec=fct-jzfs52nf@1,fct-anjjkm64@1
boundary: ndjson event line
item: shape
promise: Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
@row_add n=149 row=contract/event-line/source area=standard-contracts rec=fct-lzgtihuf@1,fct-anjjkm64@1
boundary: ndjson event line
item: source
promise: `source` is one of `hook` · `wrapper` · `cli`.
@row_add n=150 row=contract/event-line/data area=standard-contracts rec=fct-lzgtihuf@1,fct-anjjkm64@1
boundary: ndjson event line
item: data
promise: `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
@row_add n=151 row=contract/snapshot-envelope/unreadable-snapshot area=standard-contracts rec=fct-vufggymy@1
boundary: snapshot envelope
item: unreadable snapshot
promise: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
@row_none n=7 rec=fct-y6frgi47@1 type=4
relied: A replayed snapshot carries no endpoint, pid, start time or statusline command, so its readers treat the instance as unreachable and the collision check judges liveness from the heartbeat alone.
@row_add n=152 row=contract/ledger-stamps/content area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: content
promise: Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
@row_add n=153 row=contract/ledger-stamps/verified-version area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: verified version
promise: A CLI version is verified only when every relied ledger row reads `pass`.
@row_add n=154 row=contract/ledger-stamps/malformed-envelope area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: malformed envelope
promise: A malformed envelope makes `run`'s gate record `cli_verified:false`.
@row_add n=155 row=contract/instance-snapshot/data area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: data
promise: `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
@row_add n=156 row=contract/instance-snapshot/writer area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: writer
promise: `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
@row_add n=157 row=contract/instance-snapshot/started-at area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: started_at
promise: `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
@row_add n=158 row=contract/instance-snapshot/endpoint area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: endpoint
promise: `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
@row_add n=159 row=contract/instance-snapshot/spawn-order area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: spawn order
promise: The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
@row_add n=160 row=contract/instance-snapshot/pid-and-started-at area=standard-contracts rec=fct-ld365mzq@2,fct-q5tbsmkw@1
boundary: instance snapshot
item: pid and started_at
promise: `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
@row_add n=161 row=contract/instance-snapshot/child-pid area=standard-contracts rec=fct-n4ricmno@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: child_pid
promise: `child_pid` is the pid of the spawned agent child and exists only to join `claude agents --json` rows to this instance.
@row_add n=162 row=contract/instance-snapshot/agent-session-id area=standard-contracts rec=fct-n4ricmno@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: agent_session_id
promise: `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
@row_add n=163 row=contract/instance-snapshot/cwd area=standard-contracts rec=fct-27cqwcgd@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cwd
promise: `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it only as the revived child's spawn directory.
@row_add n=164 row=decision/spawn-directory-confined-to-snapshot area=standard-contracts rec=fct-27cqwcgd@1,fct-q5tbsmkw@1
name: Spawn directory confined to the snapshot
rule: The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout.
@row_add n=165 row=contract/instance-snapshot/statusline-command area=standard-contracts rec=fct-st4h3pj7@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: statusline_command
promise: `statusline_command` is the user's statusline command that `run` read from the named source at start, omitted when the source gave none; no event carries it, and `hook statusline` reads it and never changes the snapshot.
@row_add n=166 row=contract/instance-snapshot/cli-version area=standard-contracts rec=fct-gbttz3aj@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cli_version
promise: `cli_version` comes from `run`'s version gate.
@row_add n=167 row=contract/instance-snapshot/cli-verified area=standard-contracts rec=fct-gbttz3aj@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cli_verified
promise: `cli_verified` comes from `run`'s version gate.
@row_add n=168 row=contract/instance-snapshot/pending-dialog area=standard-contracts rec=fct-yqoy7mew@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: pending_dialog
promise: `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
@row_add n=169 row=contract/list-payload/dialog-pending area=standard-contracts rec=fct-yqoy7mew@1
boundary: list payload
item: dialog_pending
promise: `dialog_pending` comes only from the snapshot's `pending_dialog`, so a reader that falls back to replay reports `dialog_pending: false`.
@row_add n=170 row=contract/wrapper-channel-frames/envelope area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: envelope
promise: Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
@row_add n=171 row=contract/wrapper-channel-frames/ok-result area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: ok result
promise: A success is answered as `result` holding `{"ok": <payload>}`.
@row_add n=172 row=contract/wrapper-channel-frames/refusal-result area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: refusal result
promise: A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
@row_add n=173 row=contract/wrapper-channel-frames/unsupported-protocol-version area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: unsupported protocol version
promise: An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
@row_add n=174 row=contract/wrapper-channel-frames/max-frame area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: MAX_FRAME
promise: A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.
@row_add n=175 row=contract/wrapper-channel-frames/id-less-frames area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: id-less frames
promise: Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
@row_add n=176 row=contract/wrapper-channel-frames/conn area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: conn
promise: A client may add `conn` to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
@row_add n=177 row=contract/channel-methods/v-and-sender area=standard-contracts rec=fct-vi22dxmq@1
boundary: channel methods
item: v and sender
promise: Every channel method's `params` carries `v` and `sender`.
@row_add n=178 row=contract/channel-methods/from area=standard-contracts rec=fct-vi22dxmq@1
boundary: channel methods
item: from
promise: `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
@row_add n=179 row=contract/channel-methods/send-budget-paused area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1
boundary: channel methods
item: send budget-paused
promise: A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.
@row_add n=180 row=contract/channel-methods/send-from-link area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1
boundary: channel methods
item: send from link
promise: A `send` whose `from` names another instance records the link before `send-issued`.
@row_add n=181 row=contract/channel-methods/wait-pending-dialog area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1
boundary: channel methods
item: wait pending dialog
promise: For `wait`, a pending dialog comes first unless `after` is past its line.
@row_add n=182 row=contract/channel-methods/wait-wake-kinds area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1
boundary: channel methods
item: wait wake kinds
promise: Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file.
@row_add n=183 row=contract/channel-methods/last-after-seen-line area=standard-contracts rec=fct-5chyftpi@1,fct-vi22dxmq@1
boundary: channel methods
item: last after a seen line
promise: A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
@row_add n=184 row=contract/answer-response/question area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: question
promise: A `question` response is `{answers:{<question>:<text>}, annotations?}`.
@row_add n=185 row=contract/answer-response/permission area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: permission
promise: A `permission` response is `{behavior:"allow"|"deny", message?}`, with no suggestion slot.
@row_add n=186 row=decision/permission-answer-without-suggestions area=standard-contracts rec=fct-er6xfhrr@1
name: Permission answers without suggestions
rule: A permission answer has no suggestion slot, so an offered `permission_suggestions` entry cannot be picked.
why: A deliberate v1 limit.
@row_add n=187 row=contract/answer-response/plan area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: plan
promise: A `plan` response is `{behavior:"approve"|"revise", message?}`.
@row_add n=188 row=contract/channel-methods/answer-from-link area=standard-contracts rec=fct-er6xfhrr@1,fct-vi22dxmq@1
boundary: channel methods
item: answer from link
promise: An `answer` whose `from` names another instance records the link once every refusal has passed and its kind matched, before the answer is taken; a link that cannot be recorded answers `-32603` and the dialog stays held; others record nothing.
@row_add n=189 row=contract/channel-methods/release-from-driver area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release from driver
promise: A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
@row_add n=190 row=contract/channel-methods/release-type-check area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release type check
promise: A `release` whose `from` is of a non-string type, or whose `budget` is not a bool (`null` included), is `-32602` with `data: null`; this check comes first.
@row_add n=191 row=contract/channel-methods/release-budget-true area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release budget true
promise: `budget:true` leaves the wheel and lifts the gate: `{wheel, budget_paused:false, override_until:<the crossed window's resets_at>}` when a pause was lifted, `{wheel, budget_paused:false}` and nothing recorded when none stood.
@row_add n=192 row=contract/channel-methods/plain-release area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: plain release
promise: A `release` with no `budget` or `budget:false` lifts no pause and answers `budget_paused` as the gate's last recorded state; a `release` on a driver-held wheel changes nothing.
@row_add n=193 row=contract/channel-methods/wheel-move-durability area=standard-contracts rec=fct-ufpqhup6@1,fct-vi22dxmq@1
boundary: channel methods
item: wheel move durability
promise: `pause` and `release` are answered only after the move's `wheel` record has landed, `-32603` if it did not.
@row_add n=194 row=contract/channel-methods/budget-release-durability area=standard-contracts rec=fct-ufpqhup6@1,fct-vi22dxmq@1
boundary: channel methods
item: budget release durability
promise: A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed; else `-32603` and the pause stands.
@row_add n=195 row=contract/channel-methods/link-params-check area=standard-contracts rec=fct-ax5zoxh3@1,fct-vi22dxmq@1
boundary: channel methods
item: link params check
promise: For `link` and `unlink`, a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
@row_add n=196 row=contract/channel-methods/hook-event-driver-origin area=standard-contracts rec=fct-nf7gdkwl@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.event driver origin
promise: A `prompt-submitted` equal to the in-flight `send`'s typed text is appended with `origin:"driver"`, and only then settles that send.
@row_add n=197 row=contract/channel-methods/hook-dialog-unverified-cli area=standard-contracts rec=fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog unverified CLI
promise: On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
@row_add n=198 row=contract/channel-methods/hook-dialog-continuation area=standard-contracts rec=fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog continuation
promise: A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise, otherwise `null`.
@row_add n=199 row=decision/list-reads-state-directly area=standard-contracts rec=fct-cqamt3xw@1
name: list reads state without a wrapper
rule: `list` is not a channel method: the calling process reads the instances and budget files itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
@row_add n=200 row=contract/list-payload/status area=standard-contracts rec=fct-cqamt3xw@1
boundary: list payload
item: status
promise: A failed `claude agents --json` read leaves every `status` `unknown`.
@row_add n=201 row=decision/upstream-text-is-content area=standard-contracts rec=fct-ijx4lsqj@1
name: Upstream text is content
rule: Upstream text carried in event `data` is content, never commands.
@row_add n=202 row=contract/event-data/session-start area=standard-contracts rec=fct-6kdnuk7n@1,fct-ijx4lsqj@1
boundary: event data per kind
item: session-start
promise: `session-start` data is `{cause, agent_session_id}`.
@row_add n=203 row=contract/event-data/prompt-submitted-text area=standard-contracts rec=fct-bjew567e@1,fct-ijx4lsqj@1
boundary: event data per kind
item: prompt-submitted text
promise: `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed.
@row_add n=204 row=contract/event-data/prompt-submitted-origin area=standard-contracts rec=fct-bjew567e@1,fct-ijx4lsqj@1
boundary: event data per kind
item: prompt-submitted origin
promise: `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_add n=205 row=contract/event-data/turn-ended-last-assistant-message area=standard-contracts rec=fct-nh2dddb3@1,fct-ijx4lsqj@1
boundary: event data per kind
item: turn-ended last_assistant_message
promise: `turn-ended` data is `{last_assistant_message, background_tasks}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
@row_add n=206 row=contract/event-data/turn-ended-background-tasks area=standard-contracts rec=fct-nh2dddb3@1,fct-ijx4lsqj@1
boundary: event data per kind
item: turn-ended background_tasks
promise: `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
@row_add n=207 row=contract/event-data/question area=standard-contracts rec=fct-7tqstti5@1
boundary: event data
item: `question`
promise: `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`; each `question` string is a key in `answer`'s `answers`.
@row_add n=208 row=contract/event-data/question-option-details area=standard-contracts rec=fct-7tqstti5@1
boundary: event data
item: `question` option_details
promise: `option_details` is parallel to `options`, one per option, each field the option's string or `null`; a missing or non-string value never refuses the dialog.
@row_add n=209 row=decision/option-text-never-in-human-line area=standard-contracts rec=fct-7tqstti5@1
name: Upstream option text kept out of human lines
rule: Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
@row_add n=210 row=contract/event-data/permission area=standard-contracts rec=fct-6tq6etq5@1
boundary: event data
item: `permission`
promise: `{dialog_id, tool, input}`; `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload.
@row_add n=211 row=contract/event-data/plan area=standard-contracts rec=fct-kjgokxdm@1
boundary: event data
item: `plan`
promise: `{dialog_id, plan}`, with the plan as text.
@row_add n=212 row=contract/event-data/session-end area=standard-contracts rec=fct-qnym535j@1
boundary: event data
item: `session-end`
promise: `{}`.
@row_add n=213 row=contract/event-data/activity area=standard-contracts rec=fct-dyfp6yle@1
boundary: event data
item: `activity`
promise: `{}`, plus `tool` when the hook names one.
@row_add n=214 row=contract/event-data/link area=standard-contracts rec=fct-sbsvt5hx@1
boundary: event data
item: `link`
promise: `{driver, driven}`.
@row_add n=215 row=contract/event-data/unlink area=standard-contracts rec=fct-sbsvt5hx@1
boundary: event data
item: `unlink`
promise: `{driver, driven}`.
@row_add n=216 row=contract/event-data/wheel area=standard-contracts rec=fct-7nqw7hm2@1
boundary: event data
item: `wheel`
promise: `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
@row_add n=217 row=contract/event-data/budget-gate area=standard-contracts rec=fct-yvpb6dvx@1
boundary: event data
item: `budget-gate`
promise: `{paused, window?, override_until?}` for the instance's own gate, with `window` `five-hour` or `seven-day`.
@row_add n=218 row=contract/event-data/send-issued area=standard-contracts rec=fct-3rjugric@1
boundary: event data
item: `send-issued`
promise: `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
@row_add n=219 row=contract/event-data/send-confirmed area=standard-contracts rec=fct-fpzowl4z@1
boundary: event data
item: `send-confirmed`
promise: `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
@row_add n=220 row=contract/event-data/send-refused area=standard-contracts rec=fct-u7zh3gf3@1
boundary: event data
item: `send-refused`
promise: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
@row_add n=221 row=contract/event-data/dialog-expired area=standard-contracts rec=fct-xpezbo7u@1,fct-575nxi3k@1
boundary: event data
item: `dialog-expired`
promise: `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text.
@row_add n=222 row=decision/wrapper-events-log-only area=standard-contracts rec=fct-575nxi3k@1
name: Wrapper-appended events are log-only
rule: Event kinds the wrapper appends (`wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
why: They are how the audit trail and a replay learn the wheel holder, the budget gate, each send's outcome and the link set.
except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
@row_add n=223 row=contract/wrapper-event-appends/wheel area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `wheel`
promise: The wrapper appends `wheel` with `source: wrapper` once at start and on every change.
@row_add n=224 row=contract/wrapper-event-appends/budget-gate area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `budget-gate`
promise: The wrapper appends `budget-gate` with `source: wrapper` once at start and on every change.
@row_add n=225 row=contract/wrapper-event-appends/send-records area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `send-issued` / `send-confirmed` / `send-refused`
promise: The wrapper appends the send records with `source: wrapper` for each send.
@row_add n=226 row=contract/wrapper-event-appends/link area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `link`
promise: The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call.
@row_add n=227 row=contract/wrapper-event-appends/unlink area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `unlink`
promise: The driven instance's wrapper appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes.
@row_add n=228 row=contract/hook-dialog/data area=standard-contracts rec=fct-zis5czrj@1
boundary: `hook.dialog`
item: `data`
promise: `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`.
@row_add n=229 row=contract/hook-dialog/dialog-hooks area=standard-contracts rec=fct-zis5czrj@1
boundary: `hook.dialog`
item: dialog hooks
promise: Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
@row_add n=230 row=decision/dialog-logged-exactly-once area=standard-contracts rec=fct-zis5czrj@1
name: Every dialog logged exactly once
rule: Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once.
except: A matched continuation appends nothing, being the same dialog.
@row_add n=231 row=contract/answer-refusals/unknown-dialog area=standard-contracts rec=fct-zis5czrj@1
boundary: answer refusals
item: `unknown-dialog`
promise: A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`.
@row_add n=232 row=contract/cli-json-output/result area=standard-contracts rec=fct-sgn7a3bf@1
boundary: CLI `--json` output
item: channel result
promise: Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
@row_add n=233 row=contract/cli-json-output/error area=standard-contracts rec=fct-sgn7a3bf@1
boundary: CLI `--json` output
item: `error`
promise: Outcomes that have no channel `result` print an `error` document.
@row_add n=234 row=contract/cli-json-output/instance-unreachable area=standard-contracts rec=fct-ksmkl4jd@1
boundary: CLI `--json` output
item: `instance-unreachable`
promise: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
@row_add n=235 row=contract/mcp-refusal-result/instance-unreachable area=standard-contracts rec=fct-ksmkl4jd@1
boundary: MCP refusal result
item: `instance-unreachable`
promise: The MCP `isError` result for an unreachable instance is the CLI `instance-unreachable` document less `v`, its text `refused: instance-unreachable`.
@row_add n=236 row=contract/cli-json-output/wrapper-fault area=standard-contracts rec=fct-cuelorpk@1
boundary: CLI `--json` output
item: `wrapper-fault`
promise: `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null.
@row_add n=237 row=contract/cli-json-output/internal-error area=standard-contracts rec=fct-cp7zw4em@1
boundary: CLI `--json` output
item: `internal-error`
promise: `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_add n=238 row=contract/cli-json-output/revive-refusal area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive --json` refusal
promise: A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
@row_add n=239 row=contract/cli-json-output/revive-refusal-codes area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive --json` refusal codes
promise: `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
@row_add n=240 row=contract/cli-json-output/revive-list area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive <name> --list --json`
promise: Prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order with exit 0, or the `no-session` document with exit 1 on an empty chain.
@row_add n=241 row=decision/no-document-after-child-held-terminal area=standard-contracts rec=fct-54jxzo44@2
name: No JSON document once the child held the terminal
rule: An error after the child has held the terminal prints no JSON document.
why: The child has held the terminal.
@row_add n=242 row=contract/cli-json-output/plugin-install area=standard-contracts rec=fct-2xyi4o3s@1
boundary: CLI `--json` output
item: `viola plugin install --json`
promise: Prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with exit 0, no path.
@row_add n=243 row=contract/cli-json-output/plugin-install-strict-modes-refused area=standard-contracts rec=fct-2xyi4o3s@1
boundary: CLI `--json` output
item: `viola plugin install --json` strict-modes refusal
promise: A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21.
@row_add n=244 row=decision/hook-output-one-crate area=standard-contracts rec=fct-scvnkltz@1
name: Hook output produced by one crate
rule: The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
@row_add n=245 row=contract/hook-contract/decision area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: a decision
promise: A decision is exit 0 with a `hookSpecificOutput` body.
@row_add n=246 row=contract/hook-contract/pre-tool-use area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: PreToolUse
promise: PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`.
@row_add n=247 row=contract/hook-contract/permission-request area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: PermissionRequest
promise: PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
@row_add n=248 row=contract/hook-contract/no-decision area=standard-contracts rec=fct-2vjklnmm@1
boundary: hook contract
item: no decision
promise: No decision is exit 0 with an empty stdout.
@row_add n=249 row=contract/hook-contract/async-tier area=standard-contracts rec=fct-2vjklnmm@1
boundary: hook contract
item: async-tier hooks
promise: Async-tier hooks always print nothing.
@row_add n=250 row=decision/statusline-hook-exempt area=standard-contracts rec=fct-horlp4ay@2
name: Statusline wrapper exempt from hook output rules
rule: `viola hook statusline` records the `rate_limits` reading, prints the user's statusline command's stdout unchanged, and always exits 0 with empty stderr.
why: It is the statusline wrapper, not a Claude Code hook, so the hook decision rules do not apply.
@row_none n=8 rec=fct-2fgobtbl@1 type=S
@row_add n=251 row=contract/session-liveness/heartbeat area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: heartbeat
promise: The wrapper touches `<instance dir>/heartbeat` every 1 s.
@row_add n=252 row=contract/session-liveness/gone area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `gone`
promise: A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
@row_add n=253 row=contract/session-liveness/live area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `live`
promise: A session whose process is running and whose beat is at most 5 s old is `live`.
@row_add n=254 row=contract/session-liveness/stale area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `stale`
promise: A session whose process is running and whose beat is older than 5 s or absent is `stale`.
@row_add n=255 row=contract/agents-listing/status area=standard-contracts rec=fct-l2ty3vtu@1
boundary: agents listing
item: `status`
promise: `claude agents --json` adds `status` (idle/busy) to the listing.
@row_add n=256 row=contract/agents-listing/unwrapped-sessions area=standard-contracts rec=fct-l2ty3vtu@1
boundary: agents listing
item: unwrapped sessions
promise: Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_add n=257 row=decision/agents-listing-never-errors area=standard-contracts rec=fct-l2ty3vtu@1
name: Agents listing degrades, never errors
rule: When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
@row_none n=9 rec=fct-fxxhpbnl@1 type=S
@row_add n=258 row=decision/wrapper-channel-json-rpc area=standard-contracts rec=fct-mxxjcuwn@1
name: JSON-RPC over ndjson for the wrapper channel
rule: The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets.
@row_add n=259 row=decision/mcp-over-stdio area=standard-contracts rec=fct-uy5hviac@1,fct-kq7qpqyi@1
name: MCP over stdio only
rule: MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only.
@row_add n=260 row=absence/websocket area=standard-contracts rec=fct-4yre6p6e@1
capability: WebSocket endpoints (v1)
@row_add n=261 row=absence/polling-endpoints area=standard-contracts rec=fct-4yre6p6e@1
capability: Polling endpoints (v1)
@row_none n=10 rec=fct-tp7yy7x7@1 type=S
@row_add n=262 row=absence/listening-network-port area=occupied-resources rec=fct-kq7qpqyi@1
capability: A listening network port of any kind: no database port, no broker port, no listening TCP port
@row_none n=11 rec=fct-4khdjfgb@1 type=S
@row_add n=263 row=name/other/viola-test-chan-pid-label area=occupied-resources rec=fct-s2vyyjop@2
class: other
name: viola-test-chan-<pid>-<label>
@row_add n=264 row=decision/test-endpoints-outside-namespace area=occupied-resources rec=fct-s2vyyjop@2
name: Test channel endpoints outside the product namespace
rule: Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`.
@row_add n=265 row=name/other/viola-h12-sock area=occupied-resources rec=fct-wular7qm@2
class: other
name: viola-<h12>.sock
@row_add n=266 row=decision/socket-dir-owner-only area=occupied-resources rec=fct-wular7qm@2
name: Socket in a per-user owner-only directory
rule: The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
@row_add n=267 row=decision/socket-file-owner-only area=occupied-resources rec=fct-wular7qm@2
name: Socket file owner-only
rule: The socket is chmod 0600 after the bind.
@row_add n=268 row=decision/same-user-peers-only area=occupied-resources rec=fct-wular7qm@2
name: Same-user peers only
rule: Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame.
@row_none n=12 rec=fct-mxi5smga@1 type=S
@row_add n=269 row=decision/probe-capture-arms-inert area=occupied-resources rec=fct-cz2fjgdx@1
name: Probe capture arms are inert
rule: The hidden hook `--capture` arms take an absolute, existing directory only, read no `VIOLA_*` variable, start no observability, open no channel, and always exit 0 with empty stderr.
@row_add n=270 row=name/other/hook-session-start area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook session-start
@row_add n=271 row=name/other/hook-user-prompt-submit area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook user-prompt-submit
@row_add n=272 row=name/other/hook-pre-tool-use area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook pre-tool-use
@row_add n=273 row=name/other/hook-permission-request area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook permission-request
@row_add n=274 row=name/other/hook-stop area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook stop
@row_add n=275 row=name/other/hook-session-end area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook session-end
@row_add n=276 row=name/other/hook-notification area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook notification
@row_add n=277 row=name/other/hook-post-tool-use area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook post-tool-use
@row_add n=278 row=name/other/hook-post-tool-use-failure area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook post-tool-use-failure
@row_add n=279 row=name/other/hook-statusline area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook statusline
@row_none n=13 rec=fct-qslc37ld@1 type=S
@row_none n=14 rec=fct-bsiojb2t@1 type=S
@row_add n=280 row=name/other/viola area=occupied-resources rec=fct-frmh2fi2@1,fct-vw4eap6t@1
class: other
name: viola
@row_add n=281 row=name/server/viola area=occupied-resources rec=fct-frmh2fi2@1,fct-vw4eap6t@1
class: server
name: viola
@row_add n=282 row=name/other/viola-at-viola area=occupied-resources rec=fct-vw4eap6t@1
class: other
name: viola@viola
@row_add n=283 row=decision/plugin-carries-no-hooks area=occupied-resources rec=fct-vw4eap6t@1
name: Installed plugin carries no hooks
rule: The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks.
@row_none n=15 rec=fct-h57b2ldw@1 type=5
@row_add n=284 row=contract/child-argv/plugin-dir area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--plugin-dir`
promise: `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
@row_add n=285 row=contract/child-argv/settings area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--settings`
promise: `--settings <instance dir>/settings.json` follows when the start wrote a settings override.
@row_add n=286 row=contract/child-argv/caller-arguments area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: caller's arguments
promise: The caller's arguments follow the viola flags.
@row_add n=287 row=contract/child-argv/resume area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--resume`
promise: A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
@row_add n=288 row=contract/child-argv/fork-session area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--fork-session`
promise: A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
@row_add n=289 row=name/other/viola-verify-probe area=occupied-resources rec=fct-zlrgexly@1
class: other
name: viola-verify-probe
@row_add n=290 row=contract/settings-override/status-line area=occupied-resources rec=fct-klpb7auf@2
boundary: per-session settings override
item: `statusLine`
promise: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn.
@row_add n=291 row=contract/settings-override/plain-path-only area=occupied-resources rec=fct-klpb7auf@2
boundary: per-session settings override
item: plain-character pinned path
promise: The override is written only for a pinned path of plain characters; otherwise no override is written and no `--settings` is passed.
@row_none n=16 rec=fct-7clomlnw@1 type=S
@row_add n=292 row=decision/e2e-noop-fake-agent-feature area=occupied-resources rec=fct-j2leq5fi@1
name: No-op fake-agent feature in the e2e crate
rule: The test-only `viola-e2e` crate declares a no-op `fake-agent` feature.
why: So package-scoped `--features fake-agent` invocations resolve.
@row_add n=293 row=decision/fake-agent-feature-on-every-member area=occupied-resources rec=fct-26g75mkn@1
name: A fake-agent feature on every member crate
rule: The sync crates, viola-mcp and viola-e2e each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any member resolves.
why: so a package-scoped --features fake-agent build (cargo-mutants) resolves
except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
@row_add n=294 row=decision/viola-channel-tokio-free-by-default area=occupied-resources rec=fct-26g75mkn@1
name: viola-channel tokio-free without its feature
rule: The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client.
guard: channel-tokio-free.sh
@row_add n=295 row=decision/test-support-never-in-product area=occupied-resources rec=fct-4whhx2hw@2
name: Test-support feature kept out of product builds
rule: The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it.
guard: release-check.sh refuses a release artifact built with it
@row_add n=296 row=name/crate/viola-fuzz area=occupied-resources rec=fct-3fwjuai5@1,fct-7f6dkl55@1
class: crate
name: viola-fuzz
@row_none n=17 rec=fct-a232k7eb@1 type=S
@row_add n=297 row=name/other/viola-name area=occupied-resources rec=fct-tvaxd6b5@1
class: other
name: VIOLA_NAME
@row_add n=298 row=contract/child-environment/viola-name area=occupied-resources rec=fct-tvaxd6b5@1
boundary: The wrapped child's environment
item: VIOLA_NAME
promise: Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
@row_add n=299 row=name/other/viola-dir area=occupied-resources rec=fct-esr7jyhf@1
class: other
name: VIOLA_DIR
@row_add n=300 row=name/other/viola-bin area=occupied-resources rec=fct-7qo4h5jo@2
class: other
name: VIOLA_BIN
@row_add n=301 row=contract/child-environment/viola-bin area=occupied-resources rec=fct-7qo4h5jo@2
boundary: The wrapped child's environment
item: VIOLA_BIN
promise: Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
@row_add n=302 row=contract/child-environment/path area=occupied-resources rec=fct-fz733ta4@1
boundary: The wrapped child's environment
item: PATH
promise: Prefixed by viola run with the pinned copy's folder.
@row_add n=303 row=decision/claude-env-stripped-from-child area=occupied-resources rec=fct-jujxlicd@2
name: Inherited CLAUDE names stripped from the child
rule: Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says.
except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
@row_add n=304 row=decision/env-strip-reads-names-only area=occupied-resources rec=fct-jujxlicd@2
name: Environment strip reads names only
rule: The child-environment strip and its logs handle variable names only and never read a value.
@row_add n=305 row=decision/plugin-env-vars-unused area=occupied-resources rec=fct-55fxxi35@1
name: Plugin-provided environment variables unused
rule: viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
@row_add n=306 row=name/other/agent-run-chunk-base area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_CHUNK_BASE
@row_add n=307 row=name/other/agent-run-keep-homes area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_KEEP_HOMES
@row_add n=308 row=name/other/agent-run-keep-failed area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_KEEP_FAILED
@row_add n=309 row=name/other/fake-agent-pump-delay-ms area=occupied-resources rec=fct-4roqlggd@1
class: other
name: FAKE_AGENT_PUMP_DELAY_MS
@row_add n=310 row=name/other/fake-agent-hook-panic area=occupied-resources rec=fct-4roqlggd@1
class: other
name: FAKE_AGENT_HOOK_PANIC
@row_add n=311 row=decision/test-seams-not-configuration area=occupied-resources rec=fct-4roqlggd@1
name: Test seams are not a configuration channel
rule: Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
@row_add n=312 row=name/other/mcp-serve-test-child area=occupied-resources rec=fct-rw5zqusn@1
class: other
name: MCP_SERVE_TEST_CHILD
@row_add n=313 row=decision/viola-env-namespace-product-only area=occupied-resources rec=fct-rw5zqusn@1
name: VIOLA_ environment namespace is product plumbing
rule: Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
@row_add n=314 row=contract/human-output-colour-env/no-color area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: NO_COLOR
promise: Present, an empty value included, switches colour off.
@row_add n=315 row=contract/human-output-colour-env/term area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: TERM
promise: Exactly dumb switches colour off; a value containing 256color picks 256 colours.
@row_add n=316 row=contract/human-output-colour-env/colorterm area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: COLORTERM
promise: truecolor or 24bit picks truecolor.
@row_add n=317 row=decision/colour-env-decides-only-sgr area=occupied-resources rec=fct-lak6evbp@1
name: Colour variables decide only SGR output
rule: Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read.
why: ratified: the colour variables are to be read
@row_none n=18 rec=fct-rtcscax5@1 type=S
@row_add n=318 row=decision/config-falls-back-to-defaults area=occupied-resources rec=fct-i4wacyrp@2
name: Unusable configuration falls back to defaults
rule: config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line.
why: so no value switches the budget gate off or makes the dialog hold zero or unbounded
@row_add n=319 row=decision/pinned-binary-copy area=occupied-resources rec=fct-hnee7xdr@2
name: Pinned copy of the binary
rule: viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
@row_add n=320 row=decision/no-deletion-of-pinned-copies area=occupied-resources rec=fct-hnee7xdr@2
name: Pinned copies and plugin folders never deleted
rule: v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses.
why: so any later cleanup can skip copies that a live or stale instance still calls
@row_add n=321 row=decision/strict-modes-check-before-log area=occupied-resources rec=fct-ikjy6gp2@1,fct-cicfj3bq@2
name: Strict-modes check before any log
rule: Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child.
why: opening a log narrows an existing home
@row_add n=322 row=decision/run-pins-checked-bytes area=occupied-resources rec=fct-ikjy6gp2@1
name: Run pins the bytes it checked
rule: viola run pins the very executable bytes its start check examined.
@row_none n=19 rec=fct-t7ncsx4g@1 type=5
@row_none n=20 rec=fct-xcjb7ry2@1 type=4
relied: The plugin marketplace tree is rewritten whole on every plugin install, any other entry is left as found, and viola never reads the tree back.
@row_add n=323 row=decision/process-logs-hold-codes-only area=occupied-resources rec=fct-5jecrew3@2
name: Home-level process logs hold codes only
rule: Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
@row_add n=324 row=decision/diagnostics-open-checks-modes area=occupied-resources rec=fct-5jecrew3@2
name: Diagnostics files checked before open
rule: Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
@row_add n=325 row=contract/diagnostics-lines/process-log-line area=occupied-resources rec=fct-5jecrew3@2,fct-gn6wx4l2@1
boundary: Diagnostics lines
item: home-level process-log line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
@row_add n=326 row=contract/diagnostics-lines/detail-line area=occupied-resources rec=fct-5jecrew3@2,fct-gn6wx4l2@1
boundary: Diagnostics lines
item: instance detail line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
@row_add n=327 row=decision/events-log-never-rotated area=occupied-resources rec=fct-cicfj3bq@2
name: Events log never rotated
rule: v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into it
@row_add n=328 row=contract/events-ndjson/byte-offsets area=occupied-resources rec=fct-cicfj3bq@2
boundary: events.ndjson
item: byte offsets
promise: The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
@row_add n=329 row=decision/shared-budget-reading area=occupied-resources rec=fct-ljlz2wdv@1
name: One shared budget reading
rule: The newest rate_limits reading across wrapped sessions is kept timestamped, last writer wins, replaced whole under a lock, written only by hook statusline; every failed read of it counts as no reading.
@row_add n=330 row=decision/statusline-source-precedence area=occupied-resources rec=fct-tbyc7us6@1,fct-t3ics2st@1
name: Source of the user's statusline command
rule: When the home holds statusline-source.json it is the source of the user's statusline command; otherwise, only in the default home, the statusLine command of the user's Claude settings is taken; viola writes neither and reads both under the frame bound.
@row_add n=331 row=decision/verify-sole-stamps-writer area=occupied-resources rec=fct-vevvxa42@1
name: verify is the one stamps writer
rule: viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
@row_add n=332 row=decision/verify-probe-dirs-drop-guarded area=occupied-resources rec=fct-ns4etpk3@1,fct-dut3tvmp@1
name: verify probe dirs removed by drop guard
rule: Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing.
why: its captures are transient, content-bearing raw hook payloads
@row_none n=21 rec=fct-75zp6bfs@1 type=5
@row_add n=333 row=decision/test-evidence-survives-kill area=occupied-resources rec=fct-ut3pz3ml@1,fct-w4pd7id3@1
name: Test evidence survives a killed test
rule: Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them.
why: so a test the runner kills keeps its evidence
@row_add n=334 row=decision/chaos-home-outside-gates area=occupied-resources rec=fct-swegdoif@2
name: Chaos panic home outside the obs gates
rule: The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself.
why: the one named carve-out to the test-data rule
guard: chaos_feed_panic
@row_none n=22 rec=fct-cny66nid@1 type=5
@row_none n=23 rec=fct-th4sqjku@1 type=S
@row_none n=24 rec=fct-cqhnweqz@1 type=5
@row_none n=25 rec=fct-rhfeygrr@1 type=5
@row_none n=26 rec=fct-2nq4afki@1 type=5
@row_none n=27 rec=fct-42ufdodu@1 type=5
@row_none n=28 rec=fct-5uugfnul@1 type=5
@row_none n=29 rec=fct-uutjz2rg@1 type=5
@row_none n=30 rec=fct-xscn73uj@1 type=5
@row_add n=335 row=decision/config-keys-match-schema area=occupied-resources rec=fct-6e4hf2ai@1
name: Config keys equal the config schema
rule: The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties.
guard: config_keys_are_the_config_schema_properties
@row_none n=31 rec=fct-n6dovmau@1 type=5
@row_add n=336 row=decision/secret-scan-skips-only-chunk-diff area=occupied-resources rec=fct-6edq3mfh@1
name: Secret scan skips only the chunk diff
rule: secret-scan covers every file except the harness's chunk.diff, skipped by exact path, and the harness upload excludes that file.
@row_none n=32 rec=fct-44on672u@1 type=5
@row_add n=337 row=decision/test-homes-cleaned-by-owner area=occupied-resources rec=fct-e4sufzw6@1
name: Test homes cleaned by owner process
rule: Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
@row_none n=33 rec=fct-5q3bjmyd@2 type=5
@row_add n=338 row=decision/mutants-own-target-dir area=occupied-resources rec=fct-4jytw3f7@1
name: Mutation runs use their own relative target dir
rule: Mutation testing builds in its own target dir, passed to cargo-mutants as a relative path, so each copied tree builds every test binary in its own copy.
why: a copied default target keeps the original tree's binary paths, so root integration tests would drive the unmutated binary
@row_none n=34 rec=fct-mcr4cbe6@1 type=5
@row_none n=35 rec=fct-4u2vf5lk@2 type=5
@row_none n=36 rec=fct-nu56jv33@1 type=5
@row_none n=37 rec=fct-gwc5vubx@1 type=6
@row_none n=38 rec=fct-eesxehzs@2 type=5
@row_none n=39 rec=fct-fslyjpzg@1 type=5
@row_none n=40 rec=fct-37dicsq3@1 type=5
@row_none n=41 rec=fct-5gglmp64@1 type=5
@row_none n=42 rec=fct-ziwerfk3@1 type=5
@row_none n=43 rec=fct-io6ijhtt@1 type=5
@row_none n=44 rec=fct-lftr3wgo@1 type=5
@row_none n=45 rec=fct-xzimrtmj@1 type=5
@row_add n=339 row=decision/run-archive-never-uploaded area=occupied-resources rec=fct-q5vc67hz@1
name: Per-run archive never uploaded
rule: The per-run archive of JUnit files and mutation outcomes stays local, outside the agent-run artifacts, and is never uploaded.
why: The mutation outcomes file carries absolute argv paths.
@row_none n=46 rec=fct-7wuxnmfc@2 type=5
@row_none n=47 rec=fct-zuwbep4y@1 type=5
@row_none n=48 rec=fct-o2wbgrws@2 type=S
@row_add n=340 row=decision/no-env-config-channel area=cross-cutting-patterns rec=fct-dkudywpi@1
name: Environment variables are no configuration channel
rule: Environment variables are not a configuration channel; a dependency or feature that reads one breaks this rule like a direct read does.
@row_add n=341 row=decision/agent-driven-development area=cross-cutting-patterns rec=fct-iki7xwfk@1,fct-eiwmnbsy@1
name: Agent-driven development style
rule: The project's development style is agent-driven.
@row_add n=342 row=decision/hook-fails-open area=cross-cutting-patterns rec=fct-7h5nww77@1,fct-vf75c7sp@2
name: Hook fails open toward the human
rule: Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
@row_add n=343 row=decision/unconfirmed-delivery-not-delivered area=cross-cutting-patterns rec=fct-7h5nww77@1
name: Unconfirmed delivery reported as not-delivered
rule: Any failure to confirm delivery is reported as not-delivered and never retried blindly.
except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.
@row_add n=344 row=decision/mixed-version-tolerance area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-tkfup5lp@1
name: Mixed-version tolerance
rule: Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
@row_add n=345 row=contract/channel-params/v area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-jtewz5y6@1
boundary: Channel params
item: v
promise: Every channel params object carries the format version v.
@row_add n=346 row=contract/channel-params/sender-version area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-jtewz5y6@1
boundary: Channel params
item: sender version
promise: Every channel params object carries the sender's CARGO_PKG_VERSION.
@row_add n=347 row=contract/channel-errors/32602-newer-request area=cross-cutting-patterns rec=fct-74jt2s34@1
boundary: Channel errors
item: -32602 for a newer request
promise: A request newer than the wrapper supports gets a -32602 error naming the wrapper's version.
@row_add n=348 row=decision/tokio-containment area=cross-cutting-patterns rec=fct-me2myqzf@1,fct-xclilru3@1
name: Tokio containment
rule: Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
@row_add n=349 row=decision/crash-safe-disk-writes area=cross-cutting-patterns rec=fct-4xccofxh@1
name: Crash-safe disk writes
rule: Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
@row_add n=350 row=contract/hook-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: Hook output
item: stdout and stderr
promise: viola hook writes only its decision body to stdout and nothing to stderr.
@row_add n=351 row=contract/hook-statusline-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: Hook statusline output
item: stdout
promise: The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.
@row_add n=352 row=decision/run-terminal-child-only area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Run writes only the child's output to the terminal
rule: While the child runs, viola run writes nothing to the terminal except the child's own output.
why: The terminal carries the child's screen.
@row_add n=353 row=decision/codes-only-process-logs area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Codes-only process logs per role
rule: Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
@row_add n=354 row=decision/content-detail-per-instance area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Content-bearing detail only in the instance
rule: Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
@row_add n=355 row=contract/mcp-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: MCP server output
item: stdout and stderr
promise: viola mcp writes only MCP frames to stdout and logs to its diagnostics file; it writes JSON to stderr only when that file cannot be opened.
@row_add n=356 row=decision/cli-verbs-human-stderr area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Short-lived CLI verbs may use stderr
rule: Short-lived CLI verbs may use stderr for their human output.
@row_add n=357 row=decision/plugin-paths-forward-slashes area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: Forward slashes in plugin paths
rule: Every path written into plugin files uses forward slashes.
@row_add n=358 row=decision/hashed-short-endpoint-names area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: Hashed short endpoint names
rule: Endpoint names are hashed and short.
@row_add n=359 row=decision/os-gated-tests-name-reason area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: OS-gated tests name their reason
rule: Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=360 row=decision/ci-linux-fake-agent area=cross-cutting-patterns rec=fct-fa2klbw5@3,fct-f53f3qjo@2
name: CI on Linux against the fake agent
rule: CI runs on GitHub Actions ubuntu-latest against the fake agent.
@row_add n=361 row=decision/untrusted-upstream-text area=cross-cutting-patterns rec=fct-3bclffxa@1
name: Upstream text is untrusted
rule: A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
@row_add n=362 row=decision/same-user-endpoint-trust area=cross-cutting-patterns rec=fct-ao5mmzh3@2
name: Local endpoint trusts the same OS user only
rule: v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
@row_add n=363 row=overview/project-intent area=project-intent rec=fct-iigxhka7@3,fct-hr7nlfjh@1,fct-4dkeuwnr@1
text: viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary, a Claude Code plugin of hooks and a stdio MCP server that calls that binary, and a keyboard-driven terminal fleet console. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.
@row_add n=364 row=absence/web-or-graphical-frontend area=project-intent rec=fct-iigxhka7@3
capability: A web or graphical frontend
@row_add n=365 row=decision/wrap-unmodified-claude area=project-intent rec=fct-hr7nlfjh@1
name: Wrap the unmodified claude CLI
rule: viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.
@row_add n=366 row=decision/modular-monolith-no-daemon area=project-intent rec=fct-4dkeuwnr@1
name: Modular monolith, no daemon
rule: The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
@row_add n=367 row=decision/new-capabilities-as-verbs area=project-intent rec=fct-jkmwg2b6@1
name: New capabilities as subcommands or channel verbs
rule: New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.
@row_add n=368 row=decision/second-agent-sibling-crate area=project-intent rec=fct-jkmwg2b6@1
name: A second driven agent as a sibling crate
rule: A second driven agent arrives as a sibling viola-agent crate, at which point the adapter trait is extracted into viola-core.
@row_none n=49 rec=fct-kmtpkt4i@1 type=0
what: a statement that the repository is a single product with no template patterns
@row_add n=369 row=decision/rust-toolchain-pinned area=inherited-defaults rec=fct-xclilru3@1
name: Pinned Rust toolchain
rule: The Rust toolchain is pinned by rust-toolchain.toml, with a declared MSRV and edition 2024.
@row_add n=370 row=decision/rmcp-for-mcp area=inherited-defaults rec=fct-xclilru3@1
name: rmcp serves MCP
rule: rmcp is what serves MCP.
@row_add n=371 row=decision/clap-for-cli area=inherited-defaults rec=fct-xclilru3@1
name: clap parses the CLI
rule: clap is what parses the CLI.
@row_add n=372 row=decision/ndjson-logs-and-json-snapshots area=inherited-defaults rec=fct-gfhsqoso@1
name: State in ndjson logs and JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
@row_add n=373 row=decision/one-shared-persist-helper area=inherited-defaults rec=fct-gfhsqoso@1
name: Snapshots persisted through one shared helper
rule: Atomic JSON snapshots are written with tempfile persist through one shared helper.
@row_add n=374 row=decision/interprocess-local-sockets area=inherited-defaults rec=fct-jtewz5y6@1
name: interprocess local sockets for IPC
rule: IPC uses interprocess local sockets, one endpoint per viola run.
@row_add n=375 row=contract/channel/framing area=inherited-defaults rec=fct-jtewz5y6@1
boundary: Channel
item: framing
promise: The channel speaks JSON-RPC 2.0 over ndjson.
@row_add n=376 row=contract/channel-result/refusal area=inherited-defaults rec=fct-v6b65upn@1
boundary: Channel result
item: refusal
promise: A refusal is a typed RefusalReason in result.refusal.
@row_add n=377 row=decision/anyhow-only-root-bin area=inherited-defaults rec=fct-vf75c7sp@2
name: anyhow only in the root binary
rule: anyhow is used only in the root binary; CLI exit codes are typed.
@row_add n=378 row=decision/tolerant-serde-external area=inherited-defaults rec=fct-tkfup5lp@1
name: Tolerant serde for external payloads
rule: External payloads are deserialised tolerantly.
@row_add n=379 row=contract/timestamps/format area=inherited-defaults rec=fct-2d7pt7rb@1
boundary: Timestamps
item: format
promise: Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
@row_add n=380 row=decision/claude-shapes-isolated area=inherited-defaults rec=fct-jo2qqab5@1
name: Claude-specific shapes isolated
rule: Claude-specific shapes live only in viola-agent-claude.
@row_add n=381 row=decision/local-cargo-install area=inherited-defaults rec=fct-ll43vwdo@1
name: Local install with embedded plugin
rule: viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.
@row_add n=382 row=absence/containers-and-hosting area=inherited-defaults rec=fct-ll43vwdo@1
capability: Containers or hosting of any kind
@row_add n=383 row=decision/no-crates-io-publish area=inherited-defaults rec=fct-khy4oaxt@1
name: Nothing published to crates.io
rule: Every workspace crate, the root bin included, sets publish false; nothing is published to crates.io in v1.
@row_add n=384 row=decision/dual-mit-apache-license area=inherited-defaults rec=fct-khy4oaxt@1
name: MIT OR Apache-2.0 license
rule: Every workspace crate inherits the license MIT OR Apache-2.0 from the workspace package.
@row_none n=50 rec=fct-pcwm7hb5@1 type=0
what: a note that the project had no existing scopes
@row_add n=385 row=contract/refusal-reason/unknown-catch-all area=conventions rec=fct-5uakjii2@1
boundary: RefusalReason
item: unknown catch-all
promise: The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
@row_add n=386 row=contract/refusal-reason/detail area=conventions rec=fct-5uakjii2@1
boundary: RefusalReason
item: detail
promise: A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
@row_add n=387 row=contract/refusal-order/first-only area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: several refusals
promise: When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
@row_add n=388 row=contract/refusal-order/send area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: send
promise: control-character, empty-text, human-typing, budget-paused, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
@row_add n=389 row=contract/refusal-order/answer area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: answer
promise: control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.
@row_add n=390 row=contract/paste-text/control-characters area=conventions rec=fct-5uakjii2@1
boundary: Paste text
item: control characters
promise: LF, CR and TAB pass; every other C0, DEL and C1 is refused as not-delivered control-character, never stripped; the CLI checks before any frame and the wrapper checks again.
@row_add n=391 row=contract/paste-text/empty area=conventions rec=fct-5uakjii2@1
boundary: Paste text
item: empty text
promise: A text empty after CR LF and lone CR become LF and trailing CR and LF are dropped is refused as not-delivered empty-text, by the CLI and again by the wrapper; one also holding a refused character is control-character.
@row_add n=392 row=contract/send/paste-failure-after-issue area=conventions rec=fct-5uakjii2@1
boundary: send refusals
item: paste failing after send-issued
promise: A paste that fails after send-issued is refused as input-not-ready with that send's cursor.
@row_add n=393 row=contract/ok-payload/unconfirmable area=conventions rec=fct-5uakjii2@1
boundary: ok payload
item: unconfirmable
promise: unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured.
@row_add n=394 row=contract/code-word/format area=conventions rec=fct-7qxhyabf@1
boundary: Code word
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_add n=395 row=contract/peer-reply/refusal area=conventions rec=fct-6tvyxpxd@1
boundary: A peer's reply
item: refusal
promise: A refusal that is no code word is read as unknown.
@row_add n=396 row=contract/peer-reply/detail area=conventions rec=fct-6tvyxpxd@1
boundary: A peer's reply
item: detail
promise: A string detail that is no code word is read as null; a non-string detail makes the reply no known reply.
@row_add n=397 row=contract/unreachable-cause/cause-set area=conventions rec=fct-jpo3rq7m@1
boundary: Unreachable cause
item: cause set
promise: Exit 21 has a closed cause set (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused) written by both clients, with no serde path; it is no RefusalReason.
@row_add n=398 row=contract/cli-json-error-document/shape area=conventions rec=fct-vuikun3q@2
boundary: CLI --json error document
item: shape
promise: One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
@row_add n=399 row=contract/cli-internal-error/output area=conventions rec=fct-vuikun3q@2
boundary: CLI internal error output
item: internal error line
promise: A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
@row_none n=51 rec=fct-q7tym3q2@1 type=4
relied: run, revive's start, mcp and hook stay silent at the internal-error catch site.
@row_add n=400 row=contract/run-start-refusals/socket-dir-refused area=conventions rec=fct-2y3oelpa@1
boundary: viola run and revive start refusals
item: socket-dir-refused
promise: A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
@row_add n=401 row=contract/run-start-refusals/home-tree area=conventions rec=fct-czy74f6t@1
boundary: viola run and revive start refusals
item: home tree
promise: A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
@row_add n=402 row=contract/verify-refusals/home-tree area=conventions rec=fct-zvpgaziz@1
boundary: viola verify refusals
item: home tree
promise: A refused home tree, or a strict-modes refusal while measuring, exits 1 with the home-state unable and hint pair on stderr, and no step line, summary or log line.
@row_add n=403 row=contract/verify-process-exit/program-not-found area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: program-not-found
promise: The code verify's process-exit carries when the claude CLI is not found.
@row_add n=404 row=contract/verify-process-exit/pinned-hash-mismatch area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: The code verify's process-exit carries when the pinned copy fails its re-hash.
@row_add n=405 row=contract/verify-process-exit/version-unreadable area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: version-unreadable
promise: The code verify's process-exit carries when the CLI version cannot be read.
@row_add n=406 row=contract/verify-process-exit/strict-modes-failed area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: The code verify's process-exit carries on a strict-modes refusal.
@row_add n=407 row=contract/verify-process-exit/unclean-fixture area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: unclean-fixture
promise: The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
@row_add n=408 row=contract/verify-process-exit/interrupted area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: interrupted
promise: The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
@row_add n=409 row=contract/verify/no-json area=conventions rec=fct-tg3zks37@2
boundary: viola verify
item: --json
promise: viola verify takes no --json.
@row_add n=410 row=contract/client-server-verify/server-verify-failed area=conventions rec=fct-7s455w3m@2
boundary: Client server verification
item: server-verify-failed
promise: A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
@row_add n=411 row=contract/client-server-verify/check-order area=conventions rec=fct-7s455w3m@2
boundary: Client server verification
item: check order
promise: The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
@row_add n=412 row=contract/exit-21-human-hints/placement area=conventions rec=fct-jatimdcm@1
boundary: Exit 21 human hints
item: placement
promise: Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
@row_add n=413 row=contract/cli-exit-21/strict-modes-failed area=conventions rec=fct-u4uyh6oz@1
boundary: CLI exit 21
item: strict-modes-failed
promise: A refused strict-modes check of send, wait, last, answer, pause or release exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
@row_none n=52 rec=fct-dhyimyqh@1 type=4
relied: A failed connect reads as an instance-dead cause and a closed or broken stream as an endpoint-lost cause, which the exit 21 detail words rest on.
@row_add n=414 row=contract/cli-exit-21/socket-dir-refused area=conventions rec=fct-wm75dacy@1
boundary: CLI exit 21
item: socket-dir-refused
promise: On send, wait, last, answer, pause, release, link and unlink a refused socket directory exits 21 and writes one process-exit line at WARN with exit_code 21, detail socket-dir-refused and during connect, and no channel-request.
@row_add n=415 row=contract/mcp-tool-results/refusal area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: refusal
promise: A refusal is a tool result with isError true and the channel's refusal and detail in structuredContent.
@row_add n=416 row=contract/mcp-tool-results/instance-unreachable area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: instance-unreachable
promise: An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_add n=417 row=contract/mcp-tool-results/wrapper-fault area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: wrapper-fault
promise: A wrapper protocol fault is an isError true result {"error":"wrapper-fault","detail":…} carrying the sanitised JSON-RPC fault {code,message,data}: a fixed message, data a closed object or null.
@row_add n=418 row=contract/mcp-tool-results/invalid-params area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: invalid-params
promise: A tool argument failing re-validation is an isError true result {"error":"invalid-params","detail":"<field>"}, the field one of target, text, after, timeout_ms, dialog_id, response, with no frame written.
@row_add n=419 row=contract/mcp-tool-results/is-error-text area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: isError text
promise: Every isError result's text is refused: <code>.
@row_add n=420 row=contract/mcp-list-results/ttlms area=conventions rec=fct-4bh4742d@1
boundary: MCP list results
item: ttlMs
promise: The tools/list and server/discover results each carry ttlMs 0, because claude drops every tool of a list result without it.
@row_add n=421 row=contract/mcp-list-results/cachescope area=conventions rec=fct-4bh4742d@1
boundary: MCP list results
item: cacheScope
promise: The tools/list and server/discover results each carry cacheScope private, because claude drops every tool of a list result without it.
@row_add n=422 row=decision/mcp-tools-list-held-to-recorded-exchange area=conventions rec=fct-4bh4742d@1
name: MCP tools/list held to a recorded claude exchange
rule: The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order.
guard: mcp_recorded_exchange (viola-e2e)
@row_add n=423 row=contract/mcp-frame-tools/server-verify area=conventions rec=fct-gvna3tj2@2
boundary: MCP frame tools
item: server-verify
promise: A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.
@row_add n=424 row=contract/mcp-frame-tools/socket-dir-refused area=conventions rec=fct-qfrfkv27@1
boundary: MCP frame tools
item: socket-dir-refused
promise: A send, wait, last or answer meeting a refused socket directory returns the instance-unreachable isError result with detail socket-dir-refused, no frame written and no log line.
@row_add n=425 row=contract/mcp-frame-tools/strict-modes-failed area=conventions rec=fct-pfqpxroc@1
boundary: MCP frame tools
item: strict-modes-failed
promise: A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
@row_add n=426 row=decision/statusline-source-follows-home area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: Statusline source follows the home
rule: The statusline source is the home's statusline-source.json whenever it exists; only the default home falls back to the user's Claude settings, so a home named by --home or reached through VIOLA_DIR never reads the user's settings.
@row_add n=427 row=decision/hook-exits-on-untrusted-instance area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: Hook exits at once on an untrusted instance
rule: viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append.
except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
@row_add n=428 row=contract/config-json/unknown-keys area=cross-cutting-patterns rec=fct-k6htnrfj@3
boundary: config.json
item: unknown keys
promise: config.json is parsed tolerantly: unknown keys are skipped and counted.
@row_add n=429 row=decision/diagnostics-level-sole-log-level area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: diagnostics_level as the sole log level source
rule: The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
@row_add n=430 row=decision/no-environment-configuration area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: Environment variables are no configuration channel
rule: No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does.
except: NO_COLOR, TERM and COLORTERM, read by viola's own colour layer for list's human mode; two fake-agent test seams compiled only under the fake-agent feature.
@row_add n=431 row=decision/clap-without-color-feature area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: clap built without its color feature
rule: clap is built without its color feature, so viola's own colour layer is the only reader of colour environment variables.
why: The feature's anstream and anstyle-query route read CLICOLOR, CLICOLOR_FORCE, NO_COLOR, TERM, COLORTERM and CI.
@row_add n=432 row=decision/agents-join-read-ungated area=cross-cutting-patterns rec=fct-wfge3rsz@3
name: Agents join read not gated on a stamp
rule: viola list and viola revive do not gate their claude agents --json read on a ledger stamp.
why: The read only enriches and its failure is never an error.
@row_add n=433 row=decision/serde-json-unbounded-depth-banned area=cross-cutting-patterns rec=fct-qip7sr3o@1
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in the dependency policy.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_add n=434 row=decision/sanitised-error-surfaces area=cross-cutting-patterns rec=fct-7ok4646z@1
name: Sanitised error surfaces
rule: CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value.
guard: contract_error_bodies: product_error_writers_are_each_named
except: A code word outside the closed list stays visible by design.
@row_add n=435 row=decision/thiserror-messages-interpolate-nothing area=cross-cutting-patterns rec=fct-7ok4646z@1
name: thiserror messages interpolate nothing
rule: A thiserror error message interpolates no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=436 row=contract/exit-21-human-hints/socket-dir area=cross-cutting-patterns rec=fct-7ok4646z@1
boundary: Exit 21 human hints
item: socket-dir
promise: The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
@row_add n=437 row=decision/os-gated-tests-named area=cross-cutting-patterns rec=fct-xxlqtlbc@2
name: OS-gated tests named with their reason
rule: Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands.
guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
@row_add n=438 row=decision/no-other-os-test-gates area=cross-cutting-patterns rec=fct-xxlqtlbc@2
name: No test gate naming another OS
rule: A test gate whose predicate names windows or macOS is a fault even when the gates table names it.
guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
@row_add n=439 row=decision/child-cwd-explicit area=established-decisions rec=fct-ad44nn2e@4
name: Child cwd set explicitly
rule: The child's cwd is set explicitly to viola's current directory.
except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
@row_add n=440 row=decision/revive-program-resolution area=established-decisions rec=fct-ad44nn2e@4
name: Revive resolves the program from its own PATH
rule: Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola.
why: Ratified as a boundary widening.
@row_add n=441 row=contract/viola-revive-refusals/cwd-missing area=established-decisions rec=fct-ad44nn2e@4
boundary: viola revive refusals
item: cwd-missing
promise: A revive whose recorded cwd names no existing directory is refused cwd-missing.
@row_add n=442 row=contract/child-exit-record/exit-source area=established-decisions rec=fct-ad44nn2e@4
boundary: Child exit record
item: exit_source
promise: The child's exit is recorded with exit_source handle-wait or kill-fallback.
@row_add n=443 row=decision/no-key-held-behind-resize area=established-decisions rec=fct-ad44nn2e@4
name: No human key held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize; a test that writes a key after a resize waits for the child to observe the new size first.
@row_add n=444 row=decision/passthrough-never-waits-on-model area=established-decisions rec=fct-gollksbj@1
name: Passthrough never waits on the screen model
rule: The human's bytes are written first and the screen model gets a copy through a bounded queue it may drop from, so the passthrough never waits on the model.
@row_add n=445 row=decision/poisoned-screen-model area=established-decisions rec=fct-gollksbj@1
name: Poisoned screen model reads not ready
rule: A caught vt100 panic or a dropped feed message poisons the screen model until the host size changes, and a poisoned model reads input-not-ready.
why: A dropped byte desynchronises the screen.
@row_add n=446 row=contract/parse-rejected-event/vt100-feed area=established-decisions rec=fct-gollksbj@1
boundary: parse-rejected event
item: vt100-feed
promise: One parse-rejected line with parser vt100-feed per poisoning episode, its detail panicked for a caught panic and oversize for a dropped message.
@row_add n=447 row=decision/gate-timings-compiled area=established-decisions rec=fct-gollksbj@1
name: Readiness gate timings compiled
rule: The quiet period (300 ms) and the maximum wait (8.5 s) are compiled built-ins, validated per CLI version by the stamped quiet-period ledger row.
why: The maximum wait covers the CLI's 8.0 s paste hint.
@row_add n=448 row=decision/screen-signatures-compiled area=established-decisions rec=fct-gollksbj@1
name: Screen signatures as compiled literals
rule: Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when a row holds an input-box literal and no row holds a modal literal.
why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
@row_add n=449 row=decision/partial-gate-on-unverified-cli area=established-decisions rec=fct-gollksbj@1
name: Partial readiness gate on an unverified CLI
rule: On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
@row_add n=450 row=contract/send-results/input-not-ready area=established-decisions rec=fct-gollksbj@1
boundary: send results
item: input-not-ready
promise: A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
@row_add n=451 row=decision/screen-rows-read-only-by-verify area=established-decisions rec=fct-gollksbj@1
name: Screen row text read only by verify
rule: The screen model has one row-text exit, and only viola verify reads it.
@row_add n=452 row=decision/no-typing-into-trust-dialogs area=established-decisions rec=fct-gollksbj@1
name: viola types into no trust dialog
rule: viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
@row_add n=453 row=decision/every-send-confirmed area=established-decisions rec=fct-va43dnf2@2
name: Every send is confirmed
rule: A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted.
why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate.
@row_add n=454 row=decision/confirm-window-compiled area=established-decisions rec=fct-va43dnf2@2
name: Confirmation window compiled
rule: The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp.
except: A /compact send waiting for its post-condition has its own 40 s window.
@row_add n=455 row=decision/compact-confirm-window area=established-decisions rec=fct-va43dnf2@2
name: /compact confirmation window
rule: A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency.
state: provisional
@row_add n=456 row=decision/send-time-bound area=established-decisions rec=fct-va43dnf2@2
name: Send time bound
rule: send blocks at most the readiness gate's maximum wait plus the paste plus the window, and on the MCP surface that total stays below the MCP client's tool-call timeout.
@row_add n=457 row=decision/exact-prompt-match area=established-decisions rec=fct-va43dnf2@2
name: Exact prompt match after agent normalisation
rule: The typed text is matched exactly against prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and its frame and reverses tag escaping, so the wrapper-side matcher never sees a Claude-specific shape.
@row_add n=458 row=decision/typed-text-normalisation area=established-decisions rec=fct-va43dnf2@2
name: Typed text normalisation
rule: send types its text with every CR LF pair and every other CR as one LF and without trailing CR and LF characters, nothing else changed, and uses that typed text for classification, matching, text_bytes and the paste.
why: The CLI drops a pasted text's last newline and submits a CR as LF, so a text typed with them failed the exact match.
@row_add n=459 row=contract/send-refusals/empty-text area=established-decisions rec=fct-va43dnf2@2
boundary: send refusals
item: empty-text
promise: A send whose typed text is empty is refused not-delivered / empty-text (exit 13) and never issued: nothing reserved, typed or logged as a send, and no wheel read.
@row_add n=460 row=contract/events-ndjson/send-confirmed-order area=established-decisions rec=fct-va43dnf2@2
boundary: events.ndjson
item: send-confirmed order
promise: The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts.
@row_add n=461 row=decision/local-commands-by-ledger area=established-decisions rec=fct-va43dnf2@2
name: Local commands classified by the ledger
rule: A send is a local command only by exact equality of its typed text with the ledger's local-command list, never by a leading slash; any other slash text, skills included, is an ordinary send.
why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
@row_add n=462 row=contract/send-results/unconfirmable area=established-decisions rec=fct-va43dnf2@2
boundary: send results
item: unconfirmable
promise: A listed local command whose post-condition is none, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
@row_add n=463 row=decision/clear-confirmed-by-new-session area=established-decisions rec=fct-va43dnf2@2
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed by a session-start of cause clear whose agent session id differs from the remembered one, and by no prompt-submitted.
@row_add n=464 row=decision/compact-confirmed-by-same-session area=established-decisions rec=fct-va43dnf2@2
name: /compact confirmed by the same session
rule: /compact on a verified CLI is confirmed by a session-start of cause compact whose agent session id equals the remembered one, and by no prompt-submitted.
@row_add n=465 row=decision/append-heals-torn-line area=established-decisions rec=fct-ak4xjyyx@3
name: The next append heals a torn line
rule: Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_add n=466 row=decision/reader-counts-skipped-lines area=established-decisions rec=fct-ak4xjyyx@3
name: Event reader counts what it skips
rule: The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
@row_add n=467 row=contract/list-json/skipped area=established-decisions rec=fct-ak4xjyyx@3
boundary: viola list --json
item: skipped
promise: skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
@row_add n=468 row=decision/wheel-in-wrapper-memory area=established-decisions rec=fct-ak4xjyyx@3
name: Wheel decided in wrapper memory
rule: The wheel is decided in the wrapper's memory and mirrored to a snapshot.
@row_add n=469 row=decision/budget-readings-last-writer-wins area=established-decisions rec=fct-ak4xjyyx@3
name: Budget readings last-writer-wins
rule: Budget readings are timestamped and last-writer-wins under File::lock.
@row_add n=470 row=decision/sqlite-index-upgrade-path area=established-decisions rec=fct-ak4xjyyx@3
name: SQLite index as upgrade path
rule: If a real query need appears, a derived, rebuildable SQLite index sits beside the authoritative log, which stays the source of truth.
@row_add n=471 row=absence/daemon area=established-decisions rec=fct-ak4xjyyx@3,fct-wfmyj5kz@2
capability: A daemon
@row_add n=472 row=decision/pinned-copy-no-clobber-placement area=established-decisions rec=fct-kfz3cseb@2,fct-wbroquss@1,fct-fwlvhl64@1
name: Pinned copy placed no-clobber with the handle closed
rule: The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched.
why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
@row_none n=53 rec=fct-k7elqwqc@2 type=4
@row_add n=473 row=decision/interprocess-local-socket-ipc area=established-decisions rec=fct-wfmyj5kz@2
name: interprocess local sockets behind the channel seam
rule: IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
@row_add n=474 row=decision/wait-blocks-nothing-polls area=established-decisions rec=fct-wfmyj5kz@2
name: wait blocks, nothing polls
rule: wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
@row_add n=475 row=contract/channel-wait/driver-relevant-events area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: driver-relevant events
promise: wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
@row_add n=476 row=contract/channel-wait/after area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: after
promise: after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
@row_add n=477 row=contract/channel-wait/timeout-ms area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: timeout_ms
promise: On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
@row_add n=478 row=contract/channel-wait/pending-dialog area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: pending dialog
promise: A dialog pending when wait is called without after, or with after at or before its event line, is returned at once; with after past that line wait blocks for the next event.
@row_add n=479 row=contract/channel-results/cursor area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel results
item: cursor
promise: Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
@row_add n=480 row=contract/dialog-events/dialog-id area=established-decisions rec=fct-wfmyj5kz@2
boundary: dialog events
item: dialog_id
promise: Each dialog event carries a wrapper-assigned integer dialog_id, monotonic per instance and never repeated across wrapper restarts; answer names its target by it.
@row_add n=481 row=decision/one-pending-dialog area=established-decisions rec=fct-wfmyj5kz@2
name: At most one pending dialog per instance
rule: At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
@row_add n=482 row=contract/channel-answer/unknown-dialog area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel answer
item: unknown-dialog
promise: An answer for a dialog that was not held pending is refused not-delivered with detail unknown-dialog.
@row_add n=483 row=contract/channel-last/last-assistant-message area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel last
item: last_assistant_message
promise: last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
@row_add n=484 row=decision/hashed-endpoint-names area=established-decisions rec=fct-wfmyj5kz@2
name: Hashed endpoint names
rule: Endpoint names are a short hash of the instance, never the raw instance name and path.
@row_add n=485 row=decision/socket-dir-check-at-channel-callers area=established-decisions rec=fct-lbhmyin2@1
name: Socket-directory check at the channel's callers
rule: The socket-directory check stands at the channel seam's callers, not inside the channel client's connect.
why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
@row_add n=486 row=decision/verify-server-before-first-frame area=established-decisions rec=fct-rrvv5rrr@2,fct-y7p4rnnj@2
name: Server verified before the first frame
rule: Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison.
why: A squatted or rogue server is never sent a frame.
@row_add n=487 row=decision/peer-euid-admission area=established-decisions rec=fct-wqjpkbtb@2,fct-ddp4mvl7@1
name: Peer admitted only on a matching euid
rule: A channel peer is admitted only when its euid is read and equals the own user's; an unreadable euid is refused, and the wrapper drops a refused stream before any frame, writing nothing, with no env, flag or feature seam.
@row_none n=54 rec=fct-kbitf2mh@2 type=4
relied: The socket-directory check passes an absent path, refuses a symlink, a non-directory or an unreadable path, and never chmods, so a refused directory stays as found.
@row_add n=488 row=decision/snapshots-read-through-replay area=established-decisions rec=fct-cskfkcjl@1,fct-ldjewu6a@1
name: Snapshots read through the replay
rule: Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
@row_add n=489 row=decision/mcp-server-on-rmcp area=established-decisions rec=fct-xewwdlh6@1
name: MCP server on rmcp over stdio
rule: The MCP server viola serves to drivers is built on the rmcp SDK over stdio, pinned to one minor version.
why: rmcp is the official SDK and tracks the MCP spec; the pinned minor limits churn from its near-weekly releases.
@row_add n=490 row=contract/mcp-tools/refusal area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tools
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_add n=491 row=contract/mcp-tools/from area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tools
item: from
promise: The MCP server adds `from` from its own `VIOLA_NAME` only when it has a name of its own.
@row_add n=492 row=decision/human-controls-cli-only area=established-decisions rec=fct-xewwdlh6@1
name: Human wheel and link controls are CLI-only
rule: `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface.
why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
@row_add n=493 row=decision/driver-release-deterred-not-enforced area=established-decisions rec=fct-xewwdlh6@1
name: Release from a driver deterred, not enforced
rule: A `release` whose params carry a string `from` is refused, which deters a driver; a driver can still run CLI `viola release` outside a wrapped session, because v1 trusts every process of the same OS user.
why: `from` is self-reported, so the refusal deters rather than enforces.
@row_add n=494 row=contract/channel-release/release-from-driver area=established-decisions rec=fct-xewwdlh6@1
boundary: Wrapper channel release method
item: release-from-driver
promise: A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
@row_add n=495 row=decision/mcp-wait-cap area=established-decisions rec=fct-xewwdlh6@1
name: MCP wait capped at 50 s
rule: MCP `wait` sends `timeout_ms` as the smaller of the given value and 50 000 ms, and 50 000 ms when none is given.
why: The cap sits below the Claude CLI's stdio tool-call default, so the tool call never times out on the client first.
@row_add n=496 row=contract/mcp-tool-errors/invalid-params area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tool errors
item: invalid-params
promise: An argument object that breaks the tool's schema gives the `invalid-params` result with a closed field `detail` and no frame.
@row_add n=497 row=contract/mcp-tool-errors/-32603 area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tool errors
item: -32603
promise: An internal handler fault is the JSON-RPC error `-32603` `internal error` with `data: null`.
@row_add n=498 row=decision/mcp-stdin-oversize-refused-whole area=established-decisions rec=fct-cvcawwtk@1
name: Oversize MCP stdin line refused whole
rule: An MCP stdin line longer than the frame cap reaches rmcp as no byte and ends the session as at a closed stdin; the line is never truncated and never skipped.
why: A truncated line could dispatch a padded valid request, and a skipped one leaves the peer unanswered with no signal; follows the wrapper channel's close-on-oversize rule.
@row_none n=55 rec=fct-kan7g7fj@1 type=4
relied: Every MCP frame tool refuses a home or target instance directory failing the strict-mode check with the `instance-unreachable` result and detail `strict-modes-failed`, sending no frame.
@row_none n=56 rec=fct-au3pdba5@1 type=4
relied: Every unreachable cause a frame tool meets surfaces as the `instance-unreachable` result with no frame and no line.
@row_add n=499 row=contract/mcp-stdin-oversize/parse-rejected area=established-decisions rec=fct-dfjcw6cl@1,fct-u62xcdo4@1
boundary: MCP stdin oversize process log
item: parse-rejected
promise: A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
@row_add n=500 row=contract/viola-mcp-exit-codes/0-oversize-after-initialize area=established-decisions rec=fct-dfjcw6cl@1
boundary: viola mcp exit codes
item: 0 after an oversize line once initialized
promise: An oversize line after `initialize` completed ends `viola mcp` with exit 0, as at a closed stdin.
@row_add n=501 row=contract/viola-mcp-exit-codes/1-oversize-before-initialize area=established-decisions rec=fct-dfjcw6cl@1,fct-u62xcdo4@1
boundary: viola mcp exit codes
item: 1 on an oversize line before initialize
promise: An oversize line before `initialize` completes ends `viola mcp` with exit 1 and both streams empty.
@row_add n=502 row=decision/mcp-stdin-bound-in-own-reader area=established-decisions rec=fct-75j5r4u3@1
name: MCP stdin bound in viola's own reader
rule: The MCP stdin line bound sits in the reader viola hands rmcp, never in rmcp's codec, and a cut reads as end of stream, never as an I/O error.
why: rmcp's stdio transport reads each line unbounded and its codec bounds only the writer side; end of stream keeps rmcp from writing anything of its own.
@row_add n=503 row=decision/exec-form-command-hooks area=established-decisions rec=fct-xbwprhwr@2
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments.
why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
@row_add n=504 row=contract/hook-tiers/ordering-spine area=established-decisions rec=fct-xbwprhwr@2
boundary: Hook tiers
item: sync with no reply body (the ordering spine)
promise: SessionStart, UserPromptSubmit and Stop each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
@row_add n=505 row=contract/hook-tiers/session-end area=established-decisions rec=fct-xbwprhwr@2
boundary: Hook tiers
item: SessionEnd
promise: Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key, and `run` writes `session-end` from the process exit if no hook arrived.
@row_add n=506 row=decision/spine-deadline area=established-decisions rec=fct-xbwprhwr@2
name: Spine deadline of 1 s
rule: Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event.
guard: the perf gate (test-plan §10)
@row_add n=507 row=decision/hook-connect-deadline area=established-decisions rec=fct-xbwprhwr@2
name: Hook connect deadline
rule: The connect deadline a hook passes to the channel client is 750 ms, kept below the spine deadline.
guard: a unit test asserting it below the spine deadline
state: provisional
@row_add n=508 row=decision/spine-hook-loses-event-not-turn area=established-decisions rec=fct-xbwprhwr@2
name: Spine hook loses the event, never delays the turn
rule: A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost.
why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
@row_add n=509 row=decision/session-end-budget-never-raised area=established-decisions rec=fct-xbwprhwr@2
name: SessionEnd budget never raised
rule: The 1.5 s SessionEnd budget is never raised.
@row_add n=510 row=decision/hook-tier-map-verified area=established-decisions rec=fct-xbwprhwr@2
name: Hook tier map is a verified ledger row
rule: The hook tier map is a ledger row that `viola verify` re-stamps.
guard: viola verify
@row_add n=511 row=contract/hook-diagnostics/socket-dir-refused area=established-decisions rec=fct-5ktqti5y@1,fct-qoiufq4r@1
boundary: Hook diagnostics
item: socket-dir-refused
promise: A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_add n=512 row=contract/hook-diagnostics/server-verify-failed area=established-decisions rec=fct-5ktqti5y@1,fct-qoiufq4r@1
boundary: Hook diagnostics
item: server-verify-failed
promise: A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
@row_add n=513 row=contract/hook-exit/0-with-body area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 0 with a JSON body
promise: A decision.
@row_add n=514 row=contract/hook-exit/0-no-body area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 0 with no body
promise: No decision: the dialog renders for the human.
@row_add n=515 row=contract/hook-exit/2 area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 2
promise: Never produced.
@row_add n=516 row=contract/probe-capture/payload-file area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook probe capture directory
item: `<PascalEvent>.<k>.json`
promise: Each captured payload is claimed exclusively at 0600 under the first free `k`, which counts across events, so names sort in arrival order and two captures never share a name.
@row_add n=517 row=contract/probe-capture/statusline-shell-file area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook probe capture directory
item: `StatusLine.<k>.shell`
promise: Holds exactly `bin-sh` when the `--shell` word equals `/bin/sh` byte for byte, else `other`; the word itself is written nowhere.
@row_add n=518 row=decision/dialog-hold area=established-decisions rec=fct-wpbp4hte@2
name: Dialog hold per instance
rule: A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start.
state: provisional
@row_add n=519 row=decision/viola-ends-the-dialog-wait area=established-decisions rec=fct-wpbp4hte@2
name: Viola, never Claude Code, ends the dialog wait
rule: The dialog hook bounds its reply read at the hold ceiling plus 5 s, and the dialog hooks' `timeout` in `hooks.json` sits above that by a pinned margin.
why: Viola ends the wait rather than Claude Code, and the rendered plugin stays byte-identical across starts.
@row_add n=520 row=contract/dialog-hold-end/null-reply area=established-decisions rec=fct-wpbp4hte@2
boundary: Dialog hold end
item: null reply
promise: At the hold's end the wrapper replies `null`, the hook exits 0 with no body, the dialog renders for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
@row_add n=521 row=decision/hook-never-writes-stderr area=established-decisions rec=fct-wpbp4hte@2
name: Hook logs only to diagnostics
rule: A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
@row_add n=522 row=contract/event-line/dialog-expired area=established-decisions rec=fct-ohekhpaf@1
boundary: The event line
item: dialog-expired
promise: Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
@row_none n=57 rec=fct-l7zodwoq@1 type=8
@row_none n=58 rec=fct-vwfyd52k@1 type=4
relied: A dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
@row_none n=59 rec=fct-4qhayrrs@1 type=4
relied: A hook whose instance check refuses or finds no instance exits 0 before reading config, stdin or writing any line.
@row_none n=60 rec=fct-vxml3b56@3 type=9
@row_add n=523 row=decision/byte-source-wheel-claim area=established-decisions rec=fct-4coxmu76@2
name: Byte-source wheel claim
rule: Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`.
why: `viola run` owns the real stdin, so it knows byte provenance exactly.
@row_add n=524 row=decision/closed-non-editing-byte-list area=established-decisions rec=fct-4coxmu76@2
name: Closed list of non-editing bytes
rule: Only a closed list of focus reports, mouse reports and terminal replies, each by exact grammar, never takes the wheel; every other byte does, and a reply no terminal was measured to write is not added on a guess.
why: Terminal replies read as typing took the wheel from a live driver with no key typed.
@row_add n=525 row=decision/wheel-classifier-only-observes area=established-decisions rec=fct-4coxmu76@2
name: Wheel classifier only observes
rule: The stdin classifier returns every byte whole and unchanged, and a read holding an editing key moves the wheel before it returns.
@row_add n=526 row=decision/atomic-bracketed-send area=established-decisions rec=fct-4coxmu76@2
name: Atomic bracketed-paste send
rule: A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after.
@row_add n=527 row=decision/human-prompt-confirms-wheel-take area=established-decisions rec=fct-4coxmu76@2
name: Human prompt confirms the wheel take
rule: An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it.
@row_add n=528 row=decision/harness-prompts-not-human area=established-decisions rec=fct-4coxmu76@2
name: Harness prompts are not a human take
rule: A prompt whose raw start is one of the compiled harness prefixes is filed as a harness turn and never moves the wheel.
except: A human who types such a tag at a prompt's start is filed `harness`, an accepted side effect.
@row_add n=529 row=decision/human-interrupt-moves-wheel area=established-decisions rec=fct-4coxmu76@2
name: Human interrupt moves the wheel
rule: A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state.
why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
@row_add n=530 row=decision/wheel-returns-only-through-release area=established-decisions rec=fct-4coxmu76@2
name: Wheel returns only through CLI release
rule: CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
@row_add n=531 row=contract/send-refusal/manual-pause area=established-decisions rec=fct-4coxmu76@2
boundary: Send refusal details
item: manual-pause
promise: After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.
@row_add n=532 row=decision/human-wheel-renders-dialogs area=established-decisions rec=fct-4coxmu76@2
name: Human-held wheel renders dialogs at once
rule: While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once.
why: A dialog held by the hook is never hidden from the human for the rest of the hold.
@row_add n=533 row=decision/budget-pause-gates-only-send area=established-decisions rec=fct-4coxmu76@2
name: Budget pause gates only send
rule: `budget-paused` gates only `send`, never `hook.dialog` or `answer`.
why: A running turn finishes.
@row_add n=534 row=decision/links-recorded-as-log-events area=established-decisions rec=fct-anqliuvh@1
name: Links recorded as log events
rule: A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events.
why: The log stays the single source of truth for the link set as for everything else.
@row_add n=535 row=contract/event-line/link area=established-decisions rec=fct-anqliuvh@1
boundary: The event line
item: link
promise: The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
@row_add n=536 row=contract/event-line/unlink area=established-decisions rec=fct-anqliuvh@1
boundary: The event line
item: unlink
promise: `viola unlink` appends an `unlink` line with `source: wrapper` and `{driver, driven}`.
@row_add n=537 row=contract/channel-link/-32603 area=established-decisions rec=fct-anqliuvh@1
boundary: Wrapper channel link recording
item: -32603
promise: A link that cannot be recorded is `-32603`, with nothing issued or the answer not taken.
@row_add n=538 row=decision/instance-check-before-any-read area=established-decisions rec=fct-2y2sv475@1
name: Instance check before any read
rule: The client verbs check the home and target instance directories before any role file, log or snapshot read.
why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
@row_add n=539 row=contract/process-log/link-channel-lines area=established-decisions rec=fct-fmdsf7l4@1
boundary: Wrapper process log
item: link and unlink channel lines
promise: The `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
@row_none n=61 rec=fct-b46ywspz@1 type=4
@row_add n=540 row=decision/budget-governor-soft-stop area=established-decisions rec=fct-ikuc6e4y@2
name: Budget governor soft stop
rule: Once a configured threshold is crossed, automated `send` is refused `budget-paused`; human keys are never blocked, a running turn finishes, and the pause lifts when `resets_at` passes or on `viola release --budget`.
why: The per-model weekly window is hidden, so thresholds are set low and the governor stays visible and conservative.
@row_add n=541 row=decision/budget-default-thresholds area=established-decisions rec=fct-ikuc6e4y@2
name: Budget default thresholds
rule: The defaults are `five_hour` at or above 90 % and `seven_day` at or above 85 %, read once at the wrapper's start, and no value switches the gate off.
@row_add n=542 row=decision/release-budget-per-instance area=established-decisions rec=fct-ikuc6e4y@2
name: Budget release per instance
rule: `release --budget` lifts the pause for one instance only, until the crossed window's `resets_at`, and leaves the wheel; a plain `release` never lifts a budget pause, and an override never outlives its wrapper.
@row_add n=543 row=decision/budget-gate-never-polls area=established-decisions rec=fct-ikuc6e4y@2
name: Budget gate evaluated at three points
rule: The wrapper evaluates the budget gate only at start, when a `send` arrives and at each `turn-ended`, never by polling, so a snapshot can read paused after the reset until the next point.
@row_add n=544 row=decision/budget-gate-fail-open area=established-decisions rec=fct-ikuc6e4y@2
name: Budget gate fails open
rule: No reading, an unknown figure, a passed or unparseable `resets_at`, an unreadable `budget.json` or a bad threshold leaves the gate open or at its default.
@row_add n=545 row=contract/send-refusal/order area=established-decisions rec=fct-ikuc6e4y@2
boundary: Send refusal details
item: order
promise: `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait.
@row_add n=546 row=decision/only-send-reads-budget-gate area=established-decisions rec=fct-ikuc6e4y@2
name: Only send reads the budget gate
rule: `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the budget gate.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=547 row=decision/statusline-wrapped-per-session area=established-decisions rec=fct-ikuc6e4y@2
name: Statusline wrapped only per viola session
rule: The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that runs the user's own statusline command unchanged.
why: The user's global statusline config is never touched.
@row_none n=62 rec=fct-y6kczlfs@1 type=9
@row_add n=548 row=decision/cli-capability-ledger area=established-decisions rec=fct-qwpklckh@2
name: Per-behaviour CLI capability ledger
rule: Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_add n=549 row=contract/claude-cli-version-output/first-line area=established-decisions rec=fct-qwpklckh@2
boundary: Claude CLI --version output
item: first line
promise: Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
@row_add n=550 row=decision/exit-plan-mode-allow-with-updated-input area=established-decisions rec=fct-qwpklckh@2
name: ExitPlanMode answered only through PreToolUse
rule: An ExitPlanMode plan is approved only through PreToolUse as allow with updatedInput set to the tool's own input unchanged (a non-object input gives {}), and revised through deny with a message.
why: A bare allow left the plan dialog up on the live CLI.
@row_add n=551 row=decision/r8-identity-floor area=established-decisions rec=fct-qwpklckh@2
name: R8 parent-identity floor
rule: The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
@row_add n=552 row=decision/harness-prompt-prefixes area=established-decisions rec=fct-qwpklckh@2
name: Harness prompts classified by raw prefix
rule: A prompt whose raw start matches a compiled harness prefix (<agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message) is filed harness; an escaped tag is typed text and never classifies harness.
why: Ratified; the escaped cross-session prefix rests on a relayed measurement that the CLI's own injection arrives escaped.
except: <\cross-session-message is a harness prefix, so a human who types that tag at a prompt's start is filed harness too.
@row_add n=553 row=decision/prompt-normalise-order area=established-decisions rec=fct-qwpklckh@2
name: Classify before unwrapping and un-escaping
rule: Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /.
why: So a typed pair is never unwrapped and delivery matching compares plain text.
@row_add n=554 row=decision/no-probe-for-unrelied-shapes area=established-decisions rec=fct-qwpklckh@2
name: No probe for a shape no send relies on
rule: A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only.
why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
@row_add n=555 row=decision/send-never-types-trailing-newline area=established-decisions rec=fct-qwpklckh@2
name: Send never types a trailing newline
rule: send never types a text that ends in a newline.
why: A pasted text's own last newline never reaches the hook.
@row_add n=556 row=decision/local-command-post-conditions area=established-decisions rec=fct-qwpklckh@2
name: Local commands listed with delivery post-conditions
rule: The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands.
why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
@row_add n=557 row=decision/send-waits-out-paste-hint area=established-decisions rec=fct-qwpklckh@2
name: Send waits out the paste hint
rule: On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed.
why: Ratified ruling.
@row_add n=558 row=contract/human-hint-lines/input-not-ready area=established-decisions rec=fct-qwpklckh@2
boundary: Human hint lines
item: input-not-ready
promise: Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
@row_add n=559 row=decision/verify-box-wait-for-literal area=established-decisions rec=fct-qwpklckh@2
name: Verify waits for an input-box literal after added turns
rule: After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding a compiled literal, a poisoned screen, or the probe deadline; a quiet screen with no literal keeps waiting.
why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
@row_add n=560 row=decision/verify-types-nothing-into-dialogs area=established-decisions rec=fct-qwpklckh@2
name: Verify never types into a CLI dialog
rule: No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key.
why: Ratified rulings.
@row_add n=561 row=decision/verify-plan-files-in-run-dir area=established-decisions rec=fct-qwpklckh@2
name: Plan-mode probe keeps plan files in its own dir
rule: The plan-mode verify run points plansDirectory at a 0700 plans dir inside its own 0700 run dir.
why: So the plan file lands inside the run's own dir and none under the user's own plans directory.
@row_add n=562 row=contract/viola-verify-output/step-line area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify stdout
item: step line
promise: One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
@row_add n=563 row=contract/viola-verify-output/summary-line area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify stdout
item: summary line
promise: stamped <ver>  N pass  M fail.
@row_add n=564 row=decision/verify-record-only-clean area=established-decisions rec=fct-qwpklckh@2
name: Fixtures recorded only from a clean verify
rule: viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
@row_add n=565 row=decision/real-cli-probe-local-only area=established-decisions rec=fct-qwpklckh@2
name: Real CLI probes run only locally
rule: The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
@row_none n=63 rec=fct-u3cn6cai@1 type=9
@row_none n=64 rec=fct-vag5jbae@1 type=4
relied: A strict-modes refusal of the home tree or of the stamps makes viola verify exit 1 with the home pair, reading and writing nothing.
@row_add n=566 row=contract/viola-verify-refusal-codes/program-not-found area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: program-not-found
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
@row_add n=567 row=contract/viola-verify-refusal-codes/pinned-hash-mismatch area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: pinned-hash-mismatch
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
@row_add n=568 row=contract/viola-verify-refusal-codes/version-unreadable area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: version-unreadable
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.
@row_add n=569 row=contract/viola-verify-refusal-codes/strict-modes-failed area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: strict-modes-failed
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
@row_add n=570 row=contract/viola-verify-refusal-codes/unclean-fixture area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: unclean-fixture
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
@row_add n=571 row=contract/viola-verify-refusal-codes/interrupted area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: interrupted
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
@row_add n=572 row=decision/session-liveness-heartbeat-and-start-time area=established-decisions rec=fct-44n3w6o5@2
name: Heartbeat primary, pid plus start time decide gone
rule: The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows.
why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
@row_add n=573 row=decision/run-refuses-held-name area=established-decisions rec=fct-44n3w6o5@2
name: A start refuses a held name
rule: viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
@row_add n=574 row=decision/revive-refuses-live-session area=established-decisions rec=fct-44n3w6o5@2
name: Revive refuses a session a live claude holds
rule: viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open).
why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
@row_add n=575 row=decision/revive-resumes-in-place area=established-decisions rec=fct-44n3w6o5@2
name: Revive resumes a gone instance in place
rule: viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch, and its first record is wheel with holder driver and cause start, as every start's.
why: Ratified: a revived start keeps the wheel at driver.
@row_add n=576 row=decision/run-start-order area=established-decisions rec=fct-44n3w6o5@2
name: Child spawned last in the start order
rule: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, evaluates the budget gate, writes the first snapshot, heartbeat and start events, and only then spawns the child.
why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
@row_add n=577 row=decision/exclusive-endpoint-bind area=established-decisions rec=fct-44n3w6o5@2
name: Exclusive endpoint bind
rule: A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name.
why: So two concurrent starts cannot both win.
@row_none n=65 rec=fct-2iyz7ner@1 type=4
relied: A frame verb against a stale instance reads instance-dead, since no live endpoint is given for a stale wrapper.
@row_add n=578 row=decision/frame-client-verifies-server area=established-decisions rec=fct-uzcv7hop@2,fct-uqyfijk4@2
name: Frame clients verify the serving wrapper
rule: Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
@row_none n=66 rec=fct-e4yo5lcc@1 type=4
relied: Without a live endpoint a client reads an existing instance directory as instance-dead and anything else as an unwrapped name.
@row_none n=67 rec=fct-zgxijmsq@1 type=4
relied: Over a replayed snapshot the start's collision check decides from the heartbeat alone, refusing already-live within the stale bound.
@row_none n=68 rec=fct-5kgumnrt@1 type=4
relied: A revive makes one snapshot replay and logs one state-recovered line.
@row_none n=69 rec=fct-wwpgfv5r@1 type=4
relied: Every start refuses a strict-modes home-tree failure with exit 1 before any log, line or child.
@row_none n=70 rec=fct-mefdhndu@1 type=4
relied: A failed socket-dir create and a refused check both read socket-dir-refused with exit 1, and a bind failure does not.
@row_add n=579 row=decision/pty-error-hand-written area=established-decisions rec=fct-xqkfyxga@3
name: Hand-written PTY error type
rule: The PTY crate's error has a hand-written Display and Error with fixed messages, keeping the portable-pty anyhow error only as a boxed source.
why: The seam's normal dependencies are exactly portable-pty, tracing and libc; an exception to the thiserror rule.
@row_add n=580 row=decision/anyhow-only-in-bin area=established-decisions rec=fct-xqkfyxga@3
name: anyhow only in the viola binary
rule: anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter.
why: anyhow's context chains are only useful where a person reads them.
@row_add n=581 row=decision/error-chain-to-diagnostics-only area=established-decisions rec=fct-xqkfyxga@3
name: Error chains reach only the post-mortem detail file
rule: An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr.
why: The owner's post-mortem is the one place a person reads the chain.
@row_none n=71 rec=fct-5gkabtaw@2 type=4
relied: Program resolution has one refusal, ProgramNotFound, with a fixed message.
@row_none n=72 rec=fct-gctodeyz@2 type=5
@row_add n=582 row=decision/cargo-workspace-flat-crates area=established-decisions rec=fct-yjcz4wzl@1
name: Cargo workspace with flat crates
rule: The project is a Cargo workspace with a flat crates layout, the root manifest being both the viola bin package and the workspace; each product crate is created by the first piece of work that consumes it.
why: The compiler enforces that viola-pty cannot see viola-agent-claude; cargo install --path . requires the bin at the root.
@row_add n=583 row=decision/fuzz-own-workspace area=established-decisions rec=fct-yjcz4wzl@1
name: Fuzz targets in their own workspace
rule: The cargo-fuzz targets are their own cargo workspace, excluded from the root workspace.
why: So their nightly toolchain and C++-building libfuzzer-sys never enter the root graph.
@row_add n=584 row=decision/tokio-only-in-mcp-crate area=established-decisions rec=fct-yjcz4wzl@1
name: tokio only in the MCP crate
rule: Among the product crates only viola-mcp enables tokio; viola-channel lists it optional behind its tokio feature, and no other product member's normal graph reaches tokio.
guard: scripts/channel-tokio-free.sh
@row_add n=585 row=decision/no-cargo-pup area=established-decisions rec=fct-yjcz4wzl@1
name: cargo_pup not taken
rule: Module boundaries are enforced by crate separation, not by cargo_pup.
why: cargo_pup needs nightly and was rejected.
@row_add n=586 row=decision/pinned-copy-rehash area=established-decisions rec=fct-23yj6ah3@2
name: Pinned copy re-hashed before reuse
rule: An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
@row_add n=587 row=decision/hooks-and-mcp-never-search-path area=established-decisions rec=fct-23yj6ah3@2
name: Hooks and MCP never search PATH
rule: Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup.
why: PATH-found hooks run a different binary from the wrapper after a rebuild.
@row_add n=588 row=decision/frame-version-safety-net area=established-decisions rec=fct-23yj6ah3@2
name: Per-frame version as second safety net
rule: Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
@row_add n=589 row=contract/plugin-mcp-json/viola area=established-decisions rec=fct-23yj6ah3@2
boundary: The plugin's .mcp.json
item: viola
promise: Registers the server viola with command the pinned copy's absolute path and args ["mcp"].
@row_none n=73 rec=fct-5ijnay4v@1 type=4
relied: A start pins the very bytes it read once and keyed for its home-tree check.
@row_none n=74 rec=fct-wtlwpexy@2 type=0
what: a measured note that one source tree can build two different binaries, recorded as known with no action
@row_add n=590 row=decision/installed-plugin-mcp-only area=established-decisions rec=fct-4jk25cex@2
name: Installed plugin carries MCP only
rule: viola plugin install is optional, for the MCP verbs in an unwrapped driver; the installed plugin carries only .mcp.json and no hooks, its command the absolute path of the installing exe.
why: So a wrapped session that also has it installed never runs two hook processes for one event.
@row_add n=591 row=decision/plugin-precedence-no-ledger-row area=established-decisions rec=fct-4jk25cex@2
name: Plugin precedence is no ledger row
rule: Plugin precedence between the installed and the --plugin-dir plugin is not a capability-ledger row.
why: No send relies on it, the ruled limit.
@row_add n=592 row=decision/verify-transcripts-accepted-residual area=established-decisions rec=fct-4jk25cex@2
name: Verify's CLI transcripts are an accepted residual
rule: The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
@row_add n=593 row=decision/cli-human-text-json-exit-codes area=established-decisions rec=fct-onw4pywk@3
name: Human text by default, --json for agents
rule: CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged.
why: Agents need machine-parseable refusals.
@row_add n=594 row=contract/viola-revive-json/list-document area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json
item: --list document
promise: <name> --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_add n=595 row=contract/viola-revive-json-error-codes/strict-modes-failed area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: strict-modes-failed
promise: A strict-modes check refused the start.
@row_add n=596 row=contract/viola-revive-json-error-codes/already-live area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: already-live
promise: The name is held by a live instance.
@row_add n=597 row=contract/viola-revive-json-error-codes/no-session area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: no-session
promise: No logged session to revive.
@row_add n=598 row=contract/viola-revive-json-error-codes/session-live area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: session-live
promise: The logged session is held by a live claude.
@row_add n=599 row=contract/viola-revive-json-error-codes/cwd-missing area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: cwd-missing
promise: The recorded directory is missing.
@row_add n=600 row=contract/viola-revive-json-error-codes/stale-heartbeat area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The name is held by a stale instance.
@row_add n=601 row=contract/viola-revive-json-error-codes/pinned-hash-mismatch area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: pinned-hash-mismatch
promise: The pinned copy no longer matches its hash.
@row_add n=602 row=contract/viola-revive-json-error-codes/squatted-name area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: squatted-name
promise: The endpoint name is already taken at bind.
@row_add n=603 row=contract/viola-revive-json-error-codes/socket-dir-refused area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: socket-dir-refused
promise: The socket directory was refused or could not be created.
@row_add n=604 row=contract/viola-revive-json-error-codes/internal-error area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: internal-error
promise: A claude not on PATH or an anyhow error of the start.
@row_add n=605 row=absence/windows-and-macos-support area=established-decisions rec=fct-hss5qy6y@1
capability: Support for Windows or macOS
@row_add n=606 row=decision/window-features-target-hyprland area=established-decisions rec=fct-hss5qy6y@1
name: Window features target Hyprland
rule: The window features target the Hyprland compositor.
@row_add n=607 row=decision/socket-in-per-user-dir area=occupied-resources rec=fct-nm6raycf@2
name: Endpoint socket in a per-user 0700 directory
rule: The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
@row_add n=608 row=decision/stale-socket-file-left area=occupied-resources rec=fct-uufx66yt@1
name: Stale socket file left in place
rule: A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: A choice: a harmless file in a 0700 directory.
@row_add n=609 row=decision/mcp-subcommand-hidden area=occupied-resources rec=fct-mnor3ks3@2
name: Hidden mcp subcommand
rule: The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home.
why: The plugin runs it, never a person.
@row_add n=610 row=contract/viola-mcp-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola mcp exit codes
item: 0
promise: The session ended by stdin closing, or by a stdin line past MAX_FRAME, once initialized; both streams empty.
@row_add n=611 row=contract/viola-mcp-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola mcp exit codes
item: 1
promise: The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file.
@row_add n=612 row=contract/viola-cli-arguments/verify area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: verify
promise: viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude.
@row_add n=613 row=contract/viola-cli-arguments/list area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: list
promise: viola list takes [--json] and nothing else.
@row_add n=614 row=contract/viola-cli-arguments/plugin-install area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: plugin install
promise: viola plugin install takes [--json] and nothing else; viola plugin alone is a usage error.
@row_add n=615 row=contract/viola-cli-arguments/revive area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: revive
promise: viola revive takes <name> [--id <ID>] [--fork] [-- <child args>], or <name> --list, each with [--json].
@row_add n=616 row=contract/viola-list-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 0
promise: The board was printed in one stdout write.
@row_add n=617 row=contract/viola-list-exit-codes/21 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 21
promise: The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
@row_add n=618 row=contract/viola-list-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_add n=619 row=contract/viola-list-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 2
promise: A clap usage error.
@row_add n=620 row=contract/viola-plugin-install-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 0
promise: The result was printed in one stdout write.
@row_add n=621 row=contract/viola-plugin-install-exit-codes/21 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 21
promise: The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened.
@row_add n=622 row=contract/viola-plugin-install-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_add n=623 row=contract/viola-plugin-install-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 2
promise: A clap usage error.
@row_add n=624 row=contract/viola-revive-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 0
promise: The revived child's wrapper ended as viola run's does.
@row_add n=625 row=contract/viola-revive-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 1
promise: A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
@row_add n=626 row=contract/viola-revive-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_add n=627 row=contract/viola-revive-json/list area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive --json output
item: --list
promise: viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1.
@row_add n=628 row=contract/viola-cli-arguments/link-unlink area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola CLI arguments
item: link and unlink
promise: viola link and viola unlink each take <driver> <driven> [--json], the verb's instance being the driven one.
@row_add n=629 row=contract/viola-link-exit-codes/0 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 0
promise: One result line, or under --json {"v":1,"ok":<ok>}, link's ok holding since and already_linked and unlink's ok empty.
@row_add n=630 row=contract/viola-link-exit-codes/21 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 21
promise: The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.
@row_add n=631 row=contract/viola-link-exit-codes/20 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 20
promise: A wrapper error, a -32602 included, such as driver equal to driven.
@row_add n=632 row=contract/viola-link-exit-codes/1 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 1
promise: A refusal reply, or an ok of another shape.
@row_add n=633 row=contract/frame-verb-exit-codes/21 area=occupied-resources rec=fct-l6ysfimx@2
boundary: send, wait, last, answer, pause and release exit codes
item: 21
promise: The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
@row_add n=634 row=contract/instance-unreachable-detail/strict-modes-failed area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: strict-modes-failed
promise: The detail when the instance check refused the instance.
@row_add n=635 row=contract/instance-unreachable-detail/socket-dir-refused area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: socket-dir-refused
promise: The detail when the recorded socket folder was refused, past the instance check.
@row_add n=636 row=contract/instance-unreachable-detail/server-verify-failed area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: server-verify-failed
promise: The detail when the server runs under another euid.
@row_add n=637 row=contract/viola-verify-exit-codes/1 area=occupied-resources rec=fct-l6ysfimx@2
boundary: viola verify exit codes
item: 1
promise: The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
@row_add n=638 row=contract/viola-run-start-exit-codes/1 area=occupied-resources rec=fct-l6ysfimx@2
boundary: viola run and revive start exit codes
item: 1
promise: The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
@row_add n=639 row=contract/hook-capture-arm/exit area=occupied-resources rec=fct-yoyjutua@1
boundary: viola hook --capture arm
item: exit status
promise: Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
@row_add n=640 row=contract/hook-capture-arm/payload-file area=occupied-resources rec=fct-yoyjutua@1
boundary: viola hook --capture arm
item: payload file
promise: Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
@row_add n=641 row=contract/statusline-capture-shell-file/verdict area=occupied-resources rec=fct-yoyjutua@1
boundary: StatusLine.<k>.shell capture file
item: verdict
promise: Holds exactly bin-sh or other; a missing shell word is other, and the word is written nowhere.
@row_add n=642 row=contract/claude-child-argv/plugin-dir area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --plugin-dir
promise: --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
@row_add n=643 row=contract/claude-child-argv/settings area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --settings
promise: --settings <instance dir>/settings.json follows when the start wrote a settings override; the caller's arguments come after.
@row_add n=644 row=contract/claude-child-argv/resume area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --resume
promise: A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
@row_add n=645 row=contract/claude-child-argv/fork-session area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --fork-session
promise: --fork-session follows --resume when --fork is given; the words after -- come last.
@row_add n=646 row=contract/session-id/shape area=occupied-resources rec=fct-p6lni4fy@2
boundary: Claude session id
item: shape
promise: 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
@row_add n=647 row=decision/logged-session-id-on-child-argv area=occupied-resources rec=fct-p6lni4fy@2
name: Logged session id on the child's command line
rule: A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn.
why: A ratified crossing, chosen over allowing only an id the human types.
@row_add n=648 row=decision/claude-flag-literals-in-agent-crate area=occupied-resources rec=fct-p6lni4fy@2
name: Claude flag literals in the agent crate
rule: The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that run and verify read.
@row_none n=75 rec=fct-nbf2wdvs@2 type=4
relied: verify's Run E resumes the session id taken from Run B's first SessionStart, so Run E depends on Run B.
@row_add n=649 row=decision/per-session-settings-override area=occupied-resources rec=fct-ehpssyzz@2
name: Per-session settings override for the status line
rule: Every start, a revived one included, rewrites the instance's settings.json whole at 0600 before the spawn, so the child's status line is viola's wrapper.
why: A code-bearing file handed to the CLI at every start is a ratified boundary widening.
@row_add n=650 row=decision/settings-override-safe-path-only area=occupied-resources rec=fct-ehpssyzz@2
name: Settings override only for a safe pinned path
rule: The settings override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings is passed.
@row_add n=651 row=contract/instance-settings-json/statusline area=occupied-resources rec=fct-ehpssyzz@2
boundary: instance settings.json
item: statusLine
promise: {"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}, the pinned path absolute.
@row_add n=652 row=decision/session-home-from-viola-dir area=occupied-resources rec=fct-7uan73fu@1
name: Session home resolved from VIOLA_DIR
rule: Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing.
why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
@row_none n=76 rec=fct-2oafusjr@1 type=4
relied: A hook whose VIOLA_DIR names an instance folder that does not exist exits 0 and writes nothing.
@row_add n=653 row=decision/child-env-claude-strip area=occupied-resources rec=fct-lb4pv4em@2
name: CLAUDE names stripped from the child's environment
rule: Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it.
except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
@row_add n=654 row=decision/identity-floor-eleven-names area=occupied-resources rec=fct-lb4pv4em@2
name: Identity floor of eleven names
rule: The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID.
why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
@row_add n=655 row=decision/env-strip-names-only area=occupied-resources rec=fct-lb4pv4em@2
name: Environment strip reads names only
rule: The environment strip and the logs read variable names only, never a value.
@row_add n=656 row=contract/process-start-claude-child/env-stripped-known area=occupied-resources rec=fct-lb4pv4em@2
boundary: process-start line of the claude child
item: env_stripped_known
promise: Names the CLAUDE* variables stripped from the child's environment, names only.
@row_add n=657 row=contract/process-start-claude-child/env-kept area=occupied-resources rec=fct-lb4pv4em@2
boundary: process-start line of the claude child
item: env_kept
promise: Names the CLAUDE* variables kept through claude_env_keep, names only.
@row_add n=658 row=decision/harness-env-unread-by-product area=occupied-resources rec=fct-5lqu6sil@2
name: Harness variables never read by viola
rule: AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED and CI are test-harness variables that viola itself never reads.
@row_add n=659 row=contract/harness-environment/agent-run-chunk-base area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_CHUNK_BASE
promise: Overrides the diff base of viola-harness run --mutants; without it the harness derives the base from git history.
@row_add n=660 row=contract/harness-environment/agent-run-keep-homes area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_KEEP_HOMES
promise: The value 1 keeps the test and session homes for the CI gates.
@row_add n=661 row=contract/harness-environment/agent-run-keep-failed area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_KEEP_FAILED
promise: The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
@row_add n=662 row=decision/fake-agent-hook-panic-seam area=occupied-resources rec=fct-2hatag6l@1
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction.
why: A ratified test seam, ratified together with the exact-path exemption for its panic lines.
@row_add n=663 row=decision/config-bad-value-falls-back area=occupied-resources rec=fct-3jytrhfq@2
name: Bad config values fall back to defaults
rule: An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value switches the budget gate off or makes the dialog hold zero or unbounded.
@row_add n=664 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=occupied-resources rec=fct-6u55z4cp@2
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
@row_add n=665 row=contract/diagnostics-role-files/run-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: run-<name>.ndjson
promise: The wrapper's codes-only process lines, viola revive's start arm included.
@row_add n=666 row=contract/diagnostics-role-files/hook-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: hook-<name>.ndjson
promise: The hook's hook-invoked, hook-decision and parse-rejected lines.
@row_add n=667 row=contract/diagnostics-role-files/mcp area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: mcp.ndjson
promise: viola mcp's codes-only lines, with no self process-exit line.
@row_add n=668 row=contract/diagnostics-role-files/cli-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: cli-<name>.ndjson
promise: The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.
@row_add n=669 row=contract/diag-line/format area=occupied-resources rec=fct-6oxorhgk@1
boundary: diagnostics line
item: format
promise: Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
@row_add n=670 row=decision/codes-only-role-logs area=occupied-resources rec=fct-6oxorhgk@1
name: Codes-only role logs
rule: Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
@row_add n=671 row=decision/refused-check-writes-no-line area=occupied-resources rec=fct-6oxorhgk@1,fct-7uywwkoa@2
name: Refused check writes no line
rule: A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
@row_add n=672 row=contract/frame-verb-exit-detail/instance-dead area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: instance-dead
promise: Exit 21 with during connect, at WARN.
@row_add n=673 row=contract/frame-verb-exit-detail/server-verify-failed area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: server-verify-failed
promise: Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
@row_add n=674 row=contract/frame-verb-exit-detail/unwrapped-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: unwrapped-name
promise: Exit 21 with during connect.
@row_add n=675 row=contract/frame-verb-exit-detail/endpoint-lost area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: endpoint-lost
promise: Exit 21 with during call, for a wrapper lost with the call open.
@row_add n=676 row=contract/frame-verb-exit-detail/wrapper-fault area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: wrapper-fault
promise: Exit 20.
@row_add n=677 row=contract/frame-verb-exit-detail/socket-dir-refused area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: socket-dir-refused
promise: Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
@row_add n=678 row=contract/process-exit-detail/stale-heartbeat area=occupied-resources rec=fct-6oxorhgk@1
boundary: process-exit detail enum
item: stale-heartbeat
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json.
@row_add n=679 row=contract/hook-decision-detail/socket-dir-refused area=occupied-resources rec=fct-6oxorhgk@1
boundary: hook-decision detail
item: socket-dir-refused
promise: One hook-decision line at WARN on a refused socket folder, with decision_emitted false.
@row_add n=680 row=contract/channel-response-detail/unsupported-version area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: unsupported-version
promise: Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
@row_add n=681 row=contract/channel-response-detail/invalid-params area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: invalid-params
promise: Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
@row_add n=682 row=contract/channel-response-detail/release-from-driver area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: release-from-driver
promise: Written at WARN on both the server's and the client's line.
@row_add n=683 row=contract/verify-exit-detail/program-not-found area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: program-not-found
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=684 row=contract/verify-exit-detail/pinned-hash-mismatch area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=685 row=contract/verify-exit-detail/version-unreadable area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: version-unreadable
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=686 row=contract/verify-exit-detail/strict-modes-failed area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
@row_add n=687 row=contract/verify-exit-detail/unclean-fixture area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: unclean-fixture
promise: A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
@row_add n=688 row=contract/verify-exit-detail/interrupted area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: interrupted
promise: The detail of a stopped run; a failing ledger row is no refusal and carries no code.
@row_add n=689 row=contract/diag-line/state-recovered area=occupied-resources rec=fct-bmahq2ma@1
boundary: diagnostics line
item: state-recovered
promise: A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
@row_none n=77 rec=fct-wdtionxy@2 type=4
relied: A refused diagnostics folder or file silences the log rather than failing the process, and a detail line is dropped.
@row_add n=690 row=decision/events-log-never-truncated area=occupied-resources rec=fct-hn66ngvt@2
name: Events log never rotated or truncated
rule: The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into the file.
@row_add n=691 row=decision/strict-check-before-log-open area=occupied-resources rec=fct-hn66ngvt@2,fct-7uywwkoa@2
name: Strict-modes check before any log opens
rule: Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read.
why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
@row_add n=692 row=decision/board-refused-whole area=occupied-resources rec=fct-hn66ngvt@2
name: Whole board refused on one failed check
rule: viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line.
why: Ratified as the founder's own answer.
@row_add n=693 row=decision/refused-statusline-check-no-trace area=occupied-resources rec=fct-hn66ngvt@2
name: Refused statusline check leaves no trace
rule: A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched.
why: Ratified as the founder's own answer.
@row_add n=694 row=decision/strict-read-rule area=occupied-resources rec=fct-kekl3p46@2
name: Strict-modes read rule
rule: The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
@row_none n=78 rec=fct-vs5zumck@2 type=4
relied: run's and revive's starts and verify check the home tree, refusing at home or instance level, before using it.
@row_none n=79 rec=fct-sbmskf35@2 type=4
relied: A path under a regular file is a strict-modes refusal: verify exits 1 with the home pair, the frame verbs exit 21 with the instance pair.
@row_add n=695 row=decision/list-home-read-rule area=occupied-resources rec=fct-isxdwj6d@2,fct-5oqvk7j6@1
name: viola list holds the home to the read rule
rule: viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board.
except: The budget read, mcp's start and plugin install keep the write rule on the home.
@row_add n=696 row=contract/budget-json/v area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: v
promise: Must be 1, or the file gives no reading.
@row_add n=697 row=contract/budget-json/five-hour area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: five_hour
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_add n=698 row=contract/budget-json/seven-day area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: seven_day
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_add n=699 row=contract/budget-json/read-at area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: read_at
promise: An RFC 3339 UTC timestamp with milliseconds; read as "unknown" when absent or no string.
@row_add n=700 row=decision/budget-json-single-writer area=occupied-resources rec=fct-hgzk5gxy@1
name: budget.json written only by the statusline hook
rule: budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline and only when the payload holds a rate_limits object.
@row_add n=701 row=decision/budget-gate-reads-hook-written-file area=occupied-resources rec=fct-hgzk5gxy@1
name: Budget gate refuses on a hook-written file
rule: The wrapper may refuse a driver on budget.json, read behind the strict-modes check on the home and the file and failing open: every failure is no reading, with no log line and nothing created.
why: A ratified boundary widening, ratified as built.
@row_add n=702 row=decision/stamps-single-writer area=occupied-resources rec=fct-jznqbn54@2
name: Ledger stamps have one writer
rule: ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
@row_add n=703 row=decision/version-gate-strict-stamps-read area=occupied-resources rec=fct-jznqbn54@2
name: Version gate reads stamps strictly
rule: run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
@row_add n=704 row=decision/verify-removes-only-own-dirs area=occupied-resources rec=fct-i7o6tqmg@2,fct-c3nhxk2n@2
name: verify removes only the probe folders it created
rule: viola verify removes its probe folders whole when it passes, fails or is stopped by Ctrl-C or TERM, and only the folders that run created, never by a pattern or a listing.
except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
@row_add n=705 row=decision/root-waits-share-one-bound area=occupied-resources rec=fct-jv5gqhbl@4
name: Root test waits share one bound
rule: Root test waits on a child share the one 7 s WITHIN bound of the watch support.
@row_add n=706 row=decision/wait-report-kept-on-failure area=occupied-resources rec=fct-jv5gqhbl@4
name: Wait report kept on failure
rule: A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes.
why: A test the runner kills keeps its evidence.
@row_none n=80 rec=fct-flit34us@1 type=4
relied: Harness cleanup reads ok after a force-killed wrapper, since a refused or missing socket reads gone and no leftover file is removed.
@row_add n=707 row=decision/no-after-every-test-error-body-helper area=occupied-resources rec=fct-mimmga3r@1
name: No after-every-test error-body helper
rule: Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
@row_none n=81 rec=fct-w6vpbrjt@2 type=4
relied: Root tests freeze a wrapper with SIGSTOP through one shared guard so that it reads stale, resuming it on drop.
@row_none n=82 rec=fct-bh2gc2qq@1 type=5
@row_add n=708 row=decision/e2e-home-owner-only-headroom area=occupied-resources rec=fct-5pbk7sp3@1
name: End-to-end home base checked before use
rule: The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
@row_none n=83 rec=fct-5v4ir6ih@2 type=5
@row_add n=709 row=decision/screen-fixtures-signature-rows-only area=occupied-resources rec=fct-3fjm2cf6@2
name: Screen fixtures keep only signature rows
rule: A recorded screen fixture keeps only the rows that hold a compiled signature literal; every other row is an empty string.
@row_add n=710 row=contract/unclean-fixture-refusal/message area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: message
promise: An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
@row_add n=711 row=contract/unclean-fixture-refusal/home-path area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: home-path
promise: The code given when a recording holds a home path.
@row_add n=712 row=contract/unclean-fixture-refusal/absolute-path area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: absolute-path
promise: The code given when a recording holds an absolute path.
@row_add n=713 row=contract/unclean-fixture-refusal/username area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: username
promise: The code given when a recording holds the username, including one split across a screen row seam.
@row_add n=714 row=contract/unclean-fixture-refusal/email area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: email
promise: The code given when a recording holds an email-shaped token.
@row_add n=715 row=decision/fixture-keeps-own-repository-remote area=occupied-resources rec=fct-3fjm2cf6@2
name: Fixture keeps this repository's own remote
rule: A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed.
why: ratified
@row_add n=716 row=decision/chunk-diff-secret-scan-skip area=occupied-resources rec=fct-fs7kfa27@2
name: chunk.diff is the one secret-scan skip
rule: The mutation run's chunk.diff is the one file secret-scan skips, by exact path, and the harness artifact upload excludes it.
@row_add n=717 row=decision/test-home-owner-sweep area=occupied-resources rec=fct-xib5fpty@2
name: Test homes removed by owner record only
rule: A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last.
why: a removal that stops part-way keeps the record for a later sweep
@row_add n=718 row=decision/linked-e2e-home-base-strict area=occupied-resources rec=fct-xib5fpty@2
name: Linked e2e-home base must be a private real directory
rule: A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
@row_add n=719 row=decision/e2e-home-headroom-floor area=occupied-resources rec=fct-xib5fpty@2
name: e2e-home backing headroom floor
rule: A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key.
why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
@row_add n=720 row=contract/e2e-home-refusal/headroom-message area=occupied-resources rec=fct-xib5fpty@2
boundary: e2e-home base refusal
item: headroom message
promise: A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
@row_add n=721 row=decision/dev-host-e2e-home-on-tmpfs area=occupied-resources rec=fct-xib5fpty@2
name: Dev-host test homes on a tmpfs link
rule: On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk.
why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
@row_add n=722 row=decision/test-side-removal-outside-workdir area=occupied-resources rec=fct-xib5fpty@2
name: Test-side removals outside the working directory
rule: Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made.
why: a boundary widening, ratified
@row_add n=723 row=decision/profraw-census-dev-host-only area=occupied-resources rec=fct-mtiut4fw@1
name: Profraw census is a dev-host witness only
rule: The profraw census script is a dev-host witness for the corrupt-profile mechanism, and no CI job, pre-push stage or harness command runs either of its forms.
@row_add n=724 row=contract/profraw-census-exit-codes/0 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 0
promise: Every run passed and, in the default form, the profile count equals the run count with no third or short profile; in the case form, no profile was refused.
@row_add n=725 row=contract/profraw-census-exit-codes/1 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 1
promise: Any other count.
@row_add n=726 row=contract/profraw-census-exit-codes/2 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 2
promise: A usage error, no instrumented binary, or (case form) no llvm-profdata.
@row_add n=727 row=decision/mutation-scratch-nocow area=occupied-resources rec=fct-xtqblyzi@2
name: Mutation scratch directory is NOCOW
rule: On the Linux dev host every mutation run's TMPDIR is a sibling directory outside the repository marked NOCOW, an environment fact and not harness code.
why: /tmp is a quota tmpfs too small for the copied tree, and a COW btrfs reflink copy drops the fake agent's exec bit; NOCOW makes the copy fall back to a mode-keeping copy
@row_add n=728 row=decision/rust-toolchain-pin area=stack-and-technologies rec=fct-nuruygpl@2
name: Rust toolchain pinned exactly
rule: Rust is the language of one native binary, pinned exactly by rust-toolchain.toml with rustfmt and clippy, edition 2024.
@row_add n=729 row=decision/msrv-floor area=stack-and-technologies rec=fct-nuruygpl@2
name: Declared MSRV floor 1.96
rule: The workspace declares rust-version 1.96 and CI builds against that floor without touching the toolchain pin.
why: the security toolchain floor
guard: msrv CI job
@row_add n=730 row=decision/fuzz-nightly-toolchain area=stack-and-technologies rec=fct-nuruygpl@2
name: Fuzz workspace alone on nightly
rule: Only the separate fuzz workspace builds on a pinned nightly toolchain.
@row_add n=731 row=decision/hybrid-concurrency area=stack-and-technologies rec=fct-7z3oulbd@1
name: Std threads everywhere, Tokio only inside mcp
rule: Every CLI verb runs on std threads; Tokio is built only inside mcp.
why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
@row_add n=732 row=decision/clap-cli-parser area=stack-and-technologies rec=fct-rd72s73l@1
name: clap as CLI parser, without colour
rule: clap (derive) parses the CLI with its default features minus color.
why: help and usage output stays plain and no colour or terminal crate is in the graph
@row_add n=733 row=decision/portable-pty-seam area=stack-and-technologies rec=fct-kxiazeyf@3
name: portable-pty behind viola's own pty seam
rule: The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
@row_add n=734 row=decision/libc-only-os-binding area=stack-and-technologies rec=fct-iw73rjlh@1
name: libc is the one OS-binding crate
rule: libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
@row_add n=735 row=decision/vt100-screen-model area=stack-and-technologies rec=fct-jucr2aui@1
name: vt100 as screen model
rule: vt100 gives pre-send readiness and modal detection on run's feed thread and the screen of verify's interactive probe runs.
@row_add n=736 row=decision/run-never-reads-screen-content area=stack-and-technologies rec=fct-jucr2aui@1
name: run never reads screen content
rule: run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
@row_add n=737 row=decision/wrapper-ipc-local-socket area=stack-and-technologies rec=fct-jsy3xdhu@1
name: interprocess local sockets for wrapper IPC
rule: Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.
@row_add n=738 row=decision/socket-dir-strict-check area=stack-and-technologies rec=fct-jsy3xdhu@1,fct-rvenzu5e@1
name: Socket directory checked before bind and connect
rule: The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
@row_add n=739 row=decision/no-database area=stack-and-technologies rec=fct-qzebh4lx@1
name: No database: ndjson logs and JSON snapshots
rule: State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
@row_add n=740 row=decision/versioned-records-no-migrations area=stack-and-technologies rec=fct-rzlme45f@1
name: Versioned records instead of migrations
rule: Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log.
except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
@row_add n=741 row=decision/atomic-replaced-state-files area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Replaced files through one atomic helper
rule: Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt.
why: crash-safe multi-process writes with no C code
@row_add n=742 row=decision/pinned-copy-placed-once area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Pinned copy placed once, never replaced
rule: The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
@row_add n=743 row=decision/append-one-write-per-line area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Log appends one write per line
rule: Log appends use std append mode with one write per line.
why: crash-safe multi-process writes with no C code
@row_add n=744 row=decision/content-hash-sha256 area=stack-and-technologies rec=fct-itjxddea@2
name: Pure-Rust SHA-256 content hash
rule: The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
@row_add n=745 row=decision/pinned-copy-rehash-before-reuse area=stack-and-technologies rec=fct-itjxddea@2
name: Pinned copy re-hashed before every reuse
rule: An existing pinned copy is re-hashed before every reuse.
@row_add n=746 row=decision/sha2-optimized-in-dev area=stack-and-technologies rec=fct-itjxddea@2
name: sha2 optimised in dev builds
rule: The dev profile builds sha2 at opt-level 3.
why: dev and test builds hash the debug exe at release speed
@row_add n=747 row=decision/serde-serialization area=stack-and-technologies rec=fct-f6zprgcc@1
name: serde for frames, logs and snapshots
rule: serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
@row_add n=748 row=decision/preserve-key-order area=stack-and-technologies rec=fct-f6zprgcc@1
name: Printed documents keep declared key order
rule: serde_json's preserve_order feature is on, so a printed document keeps its declared key order.
@row_add n=749 row=decision/no-unbounded-json-depth area=stack-and-technologies rec=fct-f6zprgcc@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned.
why: every parse of external bytes keeps serde_json's default recursion limit
guard: deny.toml
@row_add n=750 row=decision/tracing-process-logs area=stack-and-technologies rec=fct-zse22tqm@1
name: tracing for one-line JSON process logs
rule: Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
@row_add n=751 row=decision/frame-params-redacted area=stack-and-technologies rec=fct-zse22tqm@1
name: Frame params never rendered in Debug
rule: A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
@row_add n=752 row=decision/nutype-domain-newtypes area=stack-and-technologies rec=fct-dy7nnp4c@1
name: nutype domain newtypes
rule: Bounded domain values are nutype newtypes: ViolaName (charset and length cap), Percent (0 to 100), DialogHold (whole seconds 60 to 3600).
@row_add n=753 row=decision/chrono-timestamps area=stack-and-technologies rec=fct-eaidxga3@1
name: chrono RFC 3339 UTC timestamps
rule: Timestamps are RFC 3339 UTC with millisecond precision, through chrono.
why: already in the tree through rmcp
@row_add n=754 row=decision/typed-errors-per-crate area=stack-and-technologies rec=fct-r7pka372@1
name: thiserror per crate, anyhow at the bin edge
rule: Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only.
except: viola-pty's error type is hand-written
@row_add n=755 row=decision/error-context-detail-file-only area=stack-and-technologies rec=fct-r7pka372@1
name: Error context chains only in the detail file
rule: Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
@row_add n=756 row=decision/no-message-broker area=stack-and-technologies rec=fct-cy7u4li4@1
name: No message broker
rule: Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.
@row_add n=757 row=decision/process-liveness-signals area=stack-and-technologies rec=fct-cnnrnbfw@1
name: Heartbeat first, sysinfo and claude agents enrich
rule: The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
@row_add n=758 row=decision/rmcp-release-features area=stack-and-technologies rec=fct-qeiflw5v@1
name: rmcp release features exactly server and transport-io
rule: The release graph's rmcp features are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=759 row=decision/e2e-no-rmcp-child-transport area=stack-and-technologies rec=fct-qeiflw5v@1
name: E2E owns the viola mcp child
rule: The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit.
why: transport-child-process kills the server 3 s after a close
@row_add n=760 row=absence/model-api-calls area=stack-and-technologies rec=fct-24lf33ny@1
capability: Calling any model API; viola drives the unmodified claude CLI on the user's subscription
@row_add n=761 row=decision/local-install-no-hosting area=stack-and-technologies rec=fct-5oigicbx@1
name: Local-only install, no hosting
rule: v1 is installed locally with cargo install and has no container, hosting or deployment.
@row_add n=762 row=decision/ci-ubuntu-fake-agent area=stack-and-technologies rec=fct-sb4r6c4a@2
name: CI on ubuntu against the fake agent
rule: CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
@row_add n=763 row=decision/pre-push-filter-not-verdict area=stack-and-technologies rec=fct-sb4r6c4a@2
name: Local pre-push gate is a filter, not the verdict
rule: The local pre-push gate runs the ubuntu test job's suites natively in the working tree under a cleared environment, refuses off Linux and installs nothing; it is a pre-push filter, never the verdict of record.
@row_add n=764 row=decision/clippy-print-and-macro-bans area=stack-and-technologies rec=fct-lj7tqt64@2
name: clippy deny warnings with print and dbg bans
rule: clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
@row_add n=765 row=decision/cargo-deny-dependency-policy area=stack-and-technologies rec=fct-lj7tqt64@2
name: cargo-deny dependency policy
rule: cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
@row_add n=766 row=decision/workflow-lint-zizmor area=stack-and-technologies rec=fct-lj7tqt64@2
name: GitHub workflows linted by zizmor
rule: GitHub workflows are linted by zizmor.
@row_add n=767 row=decision/coverage-floors-gate area=stack-and-technologies rec=fct-lj7tqt64@2
name: Coverage floors enforced by the harness gate
rule: Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
@row_add n=768 row=decision/fuzz-workspace-deny-exemption area=stack-and-technologies rec=fct-lj7tqt64@2
name: Fuzz workspace outside the cargo-deny graph
rule: Fuzzing runs in CI only, in a separate fuzz workspace with its own lockfile outside the root cargo-deny graph.
why: a test-only exemption
@row_add n=769 row=decision/syn-test-only area=stack-and-technologies rec=fct-lj7tqt64@2
name: syn and proc-macro2 test-only
rule: syn and proc-macro2 are direct dependencies of the test-only E2E crate alone; no product crate depends on either.
@row_add n=770 row=decision/release-tooling-later area=stack-and-technologies rec=fct-iro7xkpv@1
name: Release tooling deferred past v1
rule: Public signed releases and installers come with dist and cargo-auditable, and self_update later, once distribution is in scope, not in v1.
@row_none n=84 rec=fct-vp2lxvsd@2 type=4
relied: Both channel clients read the serving process's pid and euid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame.
@row_none n=85 rec=fct-e6lqqdby@1 type=4
relied: The serving process's start time read through sysinfo is the observed side of the server check each frame-writing client makes before its first frame.
@row_add n=771 row=contract/list-item/budget-paused area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: budget_paused
promise: The instance's own budget gate from its snapshot; false after a release --budget override, absent for an unwrapped session.
@row_add n=772 row=contract/list-item/wrapped area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: wrapped
promise: false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, budget_paused, cli_version or cli_verified.
@row_add n=773 row=contract/list-item/liveness area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: liveness
promise: live or stale; gone instances are omitted.
@row_add n=774 row=contract/list-item/status area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: status
promise: idle, busy or unknown.
@row_add n=775 row=contract/list-item/wheel area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: wheel
promise: human or driver; absent when the session is unwrapped.
@row_add n=776 row=contract/snapshot-envelope/v area=standard-contracts rec=fct-uve5rstc@1
boundary: snapshot envelope
item: v
promise: Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
@row_add n=777 row=decision/snapshot-replay-log-fields-only area=standard-contracts rec=fct-uve5rstc@1
name: Snapshot replay recovers only log-derived fields
rule: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
@row_add n=778 row=decision/replayed-snapshot-liveness area=standard-contracts rec=fct-uve5rstc@1
name: Replayed snapshot liveness rests on the heartbeat
rule: For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
@row_add n=779 row=decision/replay-writes-no-file area=standard-contracts rec=fct-uve5rstc@1
name: Snapshot replay writes no file
rule: Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
@row_add n=780 row=decision/hook-fails-open-on-replayed-snapshot area=standard-contracts rec=fct-uve5rstc@1
name: Hook fails open on a replayed snapshot
rule: viola hook reads a replayed or absent snapshot as unreachable and fails open.
@row_add n=781 row=decision/revive-refused-without-cwd area=standard-contracts rec=fct-uve5rstc@1
name: Revive over a replayed snapshot refused
rule: A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
@row_add n=782 row=contract/ledger-stamps/rows area=standard-contracts rec=fct-62qgonvf@2
boundary: ledger stamps envelope
item: rows
promise: Maps each ledger row id to "pass" or "fail".
@row_add n=783 row=contract/ledger-stamps/measured area=standard-contracts rec=fct-62qgonvf@2
boundary: ledger stamps envelope
item: measured
promise: The probe's readings (largest hook payload per spine event, typed_probe timings, dialog_probe); additions need no v bump, and run reads no field of it.
@row_add n=784 row=decision/stamps-merge-own-version-only area=standard-contracts rec=fct-62qgonvf@2
name: Stamps merge replaces only its own version
rule: A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
@row_add n=785 row=decision/version-verified-all-relied-rows area=standard-contracts rec=fct-62qgonvf@2
name: A CLI version is verified only when every relied row passes
rule: A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
@row_none n=86 rec=fct-ekks4dfz@1 type=4
relied: The stamps writer runs the strict-modes check before creating the ledger directory and again under its lock, and a refusal reads and writes nothing.
@row_add n=786 row=decision/link-set-follows-log area=standard-contracts rec=fct-vgnixbh3@1
name: Link set follows the log
rule: The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
@row_none n=87 rec=fct-fr2pme2q@1 type=4
relied: The link set changes only on an implicit link and on a served link or unlink call.
@row_add n=787 row=contract/channel-unlink/result area=standard-contracts rec=fct-rkas3xta@1
boundary: channel unlink method
item: result
promise: Answers {} whether or not the set held the pair; an absent pair appends nothing and leaves the set unchanged.
@row_add n=788 row=contract/channel-link-unlink/error-32602 area=standard-contracts rec=fct-ooli5yny@1
boundary: channel link and unlink methods
item: -32602
promise: A link or unlink answered -32602 writes no line.
@row_add n=789 row=contract/channel-link-unlink/error-32603 area=standard-contracts rec=fct-ooli5yny@1
boundary: channel link and unlink methods
item: -32603
promise: A failed record or remove of the link answers -32603.
@row_add n=790 row=decision/server-identity-check-before-frame area=standard-contracts rec=fct-2tmeycxu@2
name: Server identity checked before the first frame
rule: Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
@row_add n=791 row=decision/client-strict-modes-before-frame area=standard-contracts rec=fct-rvenzu5e@1
name: Strict-modes check before any client frame
rule: Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
@row_add n=792 row=contract/channel-send/result area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: result
promise: {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
@row_add n=793 row=contract/channel-send/budget-paused area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: budget-paused
promise: A closed budget gate read at arrival refuses with budget-paused, detail five-hour or seven-day, before send-issued, with no cursor, nothing typed and no flight reserved.
@row_add n=794 row=contract/channel-send/from area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: from
promise: A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
@row_add n=795 row=contract/channel-wait/result area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: result
promise: {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
@row_add n=796 row=contract/channel-wait/params area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: after and timeout_ms
promise: Each absent or a non-negative integer fitting u64; null, a negative, a fraction or a string is -32602; unknown fields are ignored.
@row_add n=797 row=contract/channel-wait/start-offset area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: start offset
promise: The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
@row_add n=798 row=contract/channel-wait/wake-kinds area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: wake kinds
promise: Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
@row_add n=799 row=contract/channel-link-unlink/from area=standard-contracts rec=fct-463usxdx@1
boundary: channel link and unlink methods
item: from
promise: link and unlink read no from; a from of any shape changes nothing.
@row_add n=800 row=contract/channel-link-unlink/answer-order area=standard-contracts rec=fct-463usxdx@1
boundary: channel link and unlink methods
item: answer order
promise: Each answers only after its event line and the snapshot write.
@row_add n=801 row=contract/channel-32602-cause/unsupported-version area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: unsupported-version
promise: One of the three causes a -32602 fault reads.
@row_add n=802 row=contract/channel-32602-cause/invalid-params area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: invalid-params
promise: One of the three causes a -32602 fault reads.
@row_add n=803 row=contract/channel-32602-cause/release-from-driver area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: release-from-driver
promise: One of the three causes a -32602 fault reads.
@row_add n=804 row=contract/hook-event/notification area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: notification
promise: Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
@row_add n=805 row=contract/hook-event/kind area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: kind
promise: One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
@row_add n=806 row=contract/hook-event/appended-line area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: appended line
promise: A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
@row_add n=807 row=contract/hook-event/driver-relabel area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: driver relabel
promise: A prompt-submitted whose text equals the in-flight send's typed text is appended with origin driver and only then settles that send.
@row_add n=808 row=decision/list-needs-no-wrapper area=standard-contracts rec=fct-smoe4pw5@1
name: list is not a channel method
rule: list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
@row_add n=809 row=contract/list-payload/agents-read-failure area=standard-contracts rec=fct-smoe4pw5@1
boundary: list payload
item: agents read failure
promise: A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
@row_add n=810 row=contract/event-prompt-submitted/text area=standard-contracts rec=fct-kylkhqhb@1
boundary: prompt-submitted event data
item: text
promise: The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it.
@row_add n=811 row=contract/event-prompt-submitted/origin area=standard-contracts rec=fct-kylkhqhb@1
boundary: prompt-submitted event data
item: origin
promise: driver, human or harness; the hook files only human or harness, and driver is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_add n=812 row=contract/cli-link-json/ok area=standard-contracts rec=fct-ubupynfq@1
boundary: viola link and unlink --json output
item: ok
promise: link prints {"v":1,"ok":{"since":…,"already_linked":…}} and unlink prints {"v":1,"ok":{}}, each with exit 0.
@row_add n=813 row=contract/cli-exit-21-document/strict-modes-failed area=standard-contracts rec=fct-dt7d6rx2@1,fct-ubupynfq@1,fct-qybkldvd@1
boundary: exit-21 CLI document
item: strict-modes-failed
promise: A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
@row_add n=814 row=contract/cli-exit-21-document/instance-dead area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: instance-dead
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=815 row=contract/cli-exit-21-document/unwrapped-name area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: unwrapped-name
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=816 row=contract/cli-exit-21-document/endpoint-lost area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: endpoint-lost
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=817 row=contract/cli-exit-21-document/server-verify-failed area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: server-verify-failed
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=818 row=contract/cli-exit-21-document/socket-dir-refused area=standard-contracts rec=fct-dt7d6rx2@1,fct-ubupynfq@1,fct-qybkldvd@1
boundary: exit-21 CLI document
item: socket-dir-refused
promise: A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
@row_none n=88 rec=fct-ng4l52uh@1 type=4
relied: Every exit-21 CLI document is written by one writer.
@row_add n=819 row=contract/fault-document/known-code-message area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: message of a known fault code
promise: For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault.
@row_add n=820 row=contract/fault-document/unsupported-version area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: -32602 unsupported version
promise: A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values.
@row_add n=821 row=contract/fault-document/release-from-driver area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: -32602 release-from-driver
promise: A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body.
@row_add n=822 row=contract/fault-document/invalid-params area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: other -32602
promise: Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null.
@row_add n=823 row=contract/fault-document/wrapper-fault area=standard-contracts rec=fct-pavvapld@1
boundary: The fault document on --json and MCP
item: wrapper fault
promise: A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word.
@row_add n=824 row=contract/human-error-line/wrapper-fault area=standard-contracts rec=fct-pavvapld@1
boundary: The human error line
item: wrapper fault
promise: A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
@row_add n=825 row=contract/revive-json/refusal-document area=standard-contracts rec=fct-zxepdepr@2
boundary: viola revive --json
item: start refusal document
promise: The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
@row_add n=826 row=contract/revive-json/program-not-found area=standard-contracts rec=fct-zxepdepr@2
boundary: viola revive --json
item: program not found
promise: A program not found prints the `internal-error` document, before any child is spawned.
@row_add n=827 row=contract/revive-json/exit-1-document area=standard-contracts rec=fct-cz7o3ttv@1
boundary: viola revive --json
item: exit-1 document
promise: Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
@row_add n=828 row=contract/revive-json/pump-error area=standard-contracts rec=fct-cz7o3ttv@1
boundary: viola revive --json
item: error after the child holds the terminal
promise: An error while pumping the child prints no document, because the child has held the terminal.
@row_add n=829 row=decision/statusline-wrapper-not-a-hook area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline wrapper is not a hook event
rule: `viola hook statusline` is a hidden arm of `hook` taken on the word `statusline` before the event is mapped, so the hook event set keeps its nine events and the arm is exempt from the hook rules.
why: The statusline arm is the statusline wrapper, not a Claude Code hook.
@row_add n=830 row=contract/hook-statusline/capture area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: --capture <DIR>
promise: With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed; without it the pass-through is unchanged.
@row_add n=831 row=decision/statusline-source-in-viola-home area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command source
rule: The user's statusline command is read at start from one named source that viola only reads: the statusline source file in the viola home whenever it exists, else only in the default home the user's Claude settings, else none.
why: Two files deciding a command viola later runs is a ratified boundary widening, choosing the file in the viola home.
@row_add n=832 row=contract/statusline-source/command area=standard-contracts rec=fct-p4s6aosk@2
boundary: The statusline source settings
item: statusLine command
promise: A command is taken only from a `statusLine` of type `command`, non-empty, with no NUL.
@row_add n=833 row=decision/statusline-check-before-log area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline instance check before the log
rule: The statusline arm runs the instance check before it opens its log or reads anything under the home.
why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
@row_add n=834 row=contract/hook-statusline/refused-check area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: refused instance check
promise: A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
@row_add n=835 row=decision/statusline-only-shell-out area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command is the only shell-out
rule: The user's statusline command is the only shell-out in viola: one argument of `/bin/sh -c`, with the same stdin, environment and working directory unchanged, stderr discarded, bounded by a deadline.
why: The command is a user-written shell string; a ratified boundary widening.
guard: ledger row statusline-shell
@row_add n=836 row=decision/statusline-deadline area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command deadline
rule: The statusline command is bounded by a 5 s deadline.
state: provisional
@row_add n=837 row=contract/hook-statusline/stdout area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: stdout
promise: Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
@row_add n=838 row=contract/hook-statusline/exit area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: exit status
promise: Always exits 0 with empty stderr.
@row_add n=839 row=contract/board/status area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: status
promise: Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows.
@row_add n=840 row=contract/board/agents-read-failure area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: agents read failure
promise: If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
@row_add n=841 row=contract/board/wrapped-merge area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: wrapped instance join
promise: An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_add n=842 row=decision/mcp-list-reads-agents area=standard-contracts rec=fct-yj2flshm@2
name: MCP list reads claude agents on each call
rule: The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call.
why: A ratified widening.
@row_add n=843 row=decision/workspace-dependency-pins area=build-system rec=fct-j6b6zix5@2
name: Third-party versions pinned in one place
rule: Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
@row_add n=844 row=decision/msrv area=build-system rec=fct-j6b6zix5@2
name: Minimum supported Rust version 1.96
rule: The workspace declares rust-version 1.96, and dependency resolution prefers versions compatible with it.
@row_add n=845 row=decision/fmt-and-clippy-gate area=build-system rec=fct-jbq7vmcz@2,fct-jykzrth4@2
name: Formatting and clippy with warnings denied
rule: Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
@row_add n=846 row=decision/no-print-macros area=build-system rec=fct-jbq7vmcz@2
name: No print macros
rule: The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them.
guard: tests/contract_lints.rs
except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
@row_add n=847 row=decision/single-human-writer area=build-system rec=fct-jbq7vmcz@2
name: One human writer in the root bin
rule: The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
@row_add n=848 row=contract/human-stderr/refusal area=build-system rec=fct-jbq7vmcz@2
boundary: Human stderr
item: refusal
promise: A refusal writes the two fixed `unable:` / `hint:` lines as one write.
@row_add n=849 row=contract/human-stderr/internal-error area=build-system rec=fct-jbq7vmcz@2
boundary: Human stderr
item: internal error
promise: An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
@row_add n=850 row=contract/cli-json/internal-error area=build-system rec=fct-jbq7vmcz@2
boundary: CLI --json output
item: internal-error
promise: Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
@row_add n=851 row=decision/obs-event-only-emitter area=build-system rec=fct-jbq7vmcz@2
name: obs_event! is the only event emitter
rule: The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep.
why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep.
guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
@row_add n=852 row=decision/lint-bans-proven area=build-system rec=fct-jbq7vmcz@2
name: Every lint ban proven to fire
rule: Every lint ban is proven to fire and every control proven to pass.
guard: scripts/lint-probes.sh
@row_add n=853 row=decision/licence-allowlist area=build-system rec=fct-cotc6dxt@2
name: Licence allowlist
rule: Dependency licences allowed are exactly MIT, Apache-2.0, Zlib and Unicode-3.0.
why: The 0BSD exception was ratified.
guard: cargo deny check
except: 0BSD only through the per-crate exception for doctest-file, never through the allowlist.
@row_add n=854 row=decision/crates-io-only area=build-system rec=fct-cotc6dxt@2
name: Sources from crates.io only
rule: Every dependency comes from crates.io only.
guard: cargo deny check
@row_add n=855 row=decision/no-c-building-crates area=build-system rec=fct-cotc6dxt@2
name: No C-building crates
rule: C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph.
guard: cargo deny check
except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace.
@row_add n=856 row=decision/no-telemetry-crates area=build-system rec=fct-cotc6dxt@2
name: No telemetry crates
rule: Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned.
guard: cargo deny check
@row_add n=857 row=decision/banned-features area=build-system rec=fct-cotc6dxt@2
name: Banned dependency features
rule: The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned.
guard: cargo deny check
@row_add n=858 row=decision/serde-json-recursion-limit area=build-system rec=fct-cotc6dxt@2
name: serde_json keeps its recursion limit
rule: The serde_json unbounded_depth feature is banned.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: probe row serde_json-unbounded-depth
@row_add n=859 row=decision/deny-single-target area=build-system rec=fct-cotc6dxt@2
name: Dependency policy for one target triple
rule: The dependency policy is evaluated for the one target triple x86_64-unknown-linux-gnu.
@row_add n=860 row=decision/sync-crates-tokio-free area=build-system rec=fct-cotc6dxt@2
name: Sync crates are tokio-free
rule: Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root.
why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification.
guard: deny-sync.toml per crate in scripts/sync-crates.txt
@row_add n=861 row=decision/channel-tokio-only-behind-feature area=build-system rec=fct-cotc6dxt@2,fct-vavh762f@2
name: viola-channel tokio only behind its feature
rule: viola-channel's normal dependency graph without its tokio feature holds no tokio; the Tokio client exists only behind that feature.
why: As its own root viola-channel false-fails the sole-root ban once viola-mcp enables the feature, so a tree check replaces it.
guard: scripts/channel-tokio-free.sh
@row_add n=862 row=decision/deny-bans-proven area=build-system rec=fct-cotc6dxt@2
name: Every dependency ban proven live
rule: Every dependency ban is proven live: a throwaway project per ban must fail with that ban's own diagnostic, and a clean control must pass.
guard: scripts/deny-probes.sh
@row_add n=863 row=decision/fuzz-lockfile-audited area=build-system rec=fct-cotc6dxt@2
name: Fuzz lockfile audited separately
rule: The fuzz workspace's own lockfile is audited separately for advisories and sources.
guard: CI supply-chain step Fuzz lockfile audit; nightly advisories job
@row_add n=864 row=decision/no-orphan-modules area=build-system rec=fct-ecerp7e3@2
name: No orphan modules
rule: No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans.
guard: scripts/orphans-check.sh
@row_add n=865 row=decision/module-graph-review-not-gate area=build-system rec=fct-ecerp7e3@2
name: Module dependency graph is a review, not a gate
rule: The acyclic module dependency graph stays an on-demand review, never a gate.
why: cargo-modules reports every type and its own inherent method as a cycle, whatever the filters.
@row_add n=866 row=decision/rmcp-features-exact area=build-system rec=fct-ecerp7e3@2,fct-u4dur7ij@1
name: rmcp with exactly server and transport-io
rule: rmcp is present in viola's release graph with default features off, and the features requested of it are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=867 row=decision/install-from-source area=build-system rec=fct-lfogagcm@1
name: Install with cargo install
rule: viola is installed with cargo install from the repository root.
@row_add n=868 row=decision/no-crates-io-publishing area=build-system rec=fct-p4rq22po@1
name: Nothing published to crates.io
rule: Every workspace member, the root viola bin included, is unpublished; nothing goes to crates.io in v1.
@row_add n=869 row=decision/github-actions-ci area=ci-cd-approach rec=fct-vf6cfvel@2
name: GitHub Actions CI
rule: CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow.
@row_add n=870 row=decision/weekly-advisories area=ci-cd-approach rec=fct-vf6cfvel@2
name: Weekly advisory audit
rule: A weekly job audits advisories over both the root and the fuzz lockfile.
why: The advisory database moves without code changes.
@row_add n=871 row=decision/weekly-fuzz area=ci-cd-approach rec=fct-vf6cfvel@2
name: Weekly fuzzing
rule: A weekly job fuzzes every target for a bounded time and uploads the fuzz artifacts on failure.
@row_add n=872 row=decision/no-workflow-concurrency area=ci-cd-approach rec=fct-vf6cfvel@2
name: No workflow concurrency groups
rule: No workflow has a concurrency block.
why: A concurrency group cancels pending runs, which would drop a cancelled run's always() gate and upload chain.
@row_add n=873 row=decision/workflow-permissions area=ci-cd-approach rec=fct-27xbiznz@1
name: Least-privilege workflow permissions
rule: Every workflow sets empty permissions at the top and read-only contents per job.
guard: zizmor
@row_add n=874 row=decision/actions-pinned-by-sha area=ci-cd-approach rec=fct-27xbiznz@1
name: Actions pinned by commit SHA
rule: Every action used in a workflow is pinned by full commit SHA with a version comment.
guard: zizmor
@row_add n=875 row=decision/event-payload-via-env area=ci-cd-approach rec=fct-27xbiznz@1
name: Event payload only through env
rule: Event-payload values reach a workflow step only through environment variables.
guard: zizmor
@row_add n=876 row=decision/ci-tools-pinned area=ci-cd-approach rec=fct-uy6tohyo@1
name: CI tools at pinned versions
rule: Every tool CI installs is version-pinned and checkout keeps no credentials; a release asset installed by hand is checked against its published sha256.
@row_add n=877 row=decision/coverage-floors area=ci-cd-approach rec=fct-3nc6guvb@3
name: Coverage floors
rule: The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest.
guard: gate --require coverage,doctest
@row_add n=878 row=decision/uploads-gated-on-secret-scan area=ci-cd-approach rec=fct-3nc6guvb@3
name: Uploads gated on the secret scan
rule: CI artifacts are uploaded only when the secret scan succeeds; on a scan failure only the secret-scan report is uploaded.
guard: secret-scan step
@row_add n=879 row=decision/perf-gate area=ci-cd-approach rec=fct-3nc6guvb@3
name: Perf runs in its own job
rule: Performance runs only in the perf job, gated on its perf verdict; the test job and the pre-push carry no perf step.
guard: gate --require perf
@row_add n=880 row=decision/mutation-at-epoch-boundary area=ci-cd-approach rec=fct-3nc6guvb@3
name: Mutation testing at the epoch boundary
rule: CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
@row_add n=881 row=decision/msrv-ci area=ci-cd-approach rec=fct-3nc6guvb@3
name: MSRV checked in CI
rule: A CI job checks the workspace and runs the unit tests on Rust 1.96.
guard: gate --require nextest-unit
@row_add n=882 row=decision/fuzz-replay-ci area=ci-cd-approach rec=fct-3nc6guvb@3
name: Fuzz corpus replayed in CI
rule: A CI job replays the fuzz corpus on every push and pull request.
guard: gate --require fuzz-replay
@row_add n=883 row=decision/gate-steps-fail-closed area=ci-cd-approach rec=fct-3nc6guvb@3
name: Gate steps fail closed
rule: Every CI gate step runs fail-closed in bash.
@row_add n=884 row=decision/sync-crates-compile-without-tokio area=ci-cd-approach rec=fct-jykzrth4@2
name: Sync crates compile without tokio
rule: Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.
@row_add n=885 row=decision/tests-against-fake-agent area=ci-cd-approach rec=fct-jykzrth4@2
name: Tests run against the fake agent
rule: The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
@row_add n=886 row=decision/release-build-viola-only area=ci-cd-approach rec=fct-jykzrth4@2
name: Release build yields viola only
rule: The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent.
why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables.
guard: scripts/release-check.sh
@row_add n=887 row=decision/real-claude-local-only area=ci-cd-approach rec=fct-oqzfmod4@2
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally.
guard: tests/cli_verify.rs
@row_add n=888 row=decision/local-pre-push-gate area=ci-cd-approach rec=fct-ni3gbuvj@2
name: Local pre-push gate
rule: A local pre-push gate precedes every operator push and a red pre-push stops the pass; CI's run on the pushed sha stays the verdict of record.
@row_add n=889 row=decision/pre-push-clean-environment area=ci-cd-approach rec=fct-ni3gbuvj@2
name: Pre-push children in a clean environment
rule: The pre-push runs natively in the working tree, every child with an emptied environment holding only HOME from the passwd entry and a fixed PATH; the harness's own HOME is never read.
@row_add n=890 row=contract/pre-push/linux-only area=ci-cd-approach rec=fct-ni3gbuvj@2
boundary: viola-harness pre-push
item: pre-push-linux-only
promise: On a host other than Linux the pre-push refuses `pre-push-linux-only` with exit 2.
@row_add n=891 row=contract/pre-push/passwd-home area=ci-cd-approach rec=fct-ni3gbuvj@2
boundary: viola-harness pre-push
item: tool-missing passwd-home
promise: A failed passwd probe or a non-absolute home field is `tool-missing` `passwd-home` at stage `tools`.
@row_add n=892 row=absence/deploy-stage area=ci-cd-approach rec=fct-32j43dt2@1
capability: A deploy stage
@row_add n=893 row=decision/release-via-dist area=ci-cd-approach rec=fct-32j43dt2@1
name: Release workflow generated by dist
rule: The v1.x release path is a dist-generated release workflow with cargo-auditable.
@row_add n=894 row=decision/core-depends-on-no-viola-crate area=crate-dependency-direction rec=fct-pqkdzm6m@1
name: viola-core depends on no viola crate
rule: viola-core depends on no other viola crate.
@row_add n=895 row=decision/pty-knows-no-agent area=crate-dependency-direction rec=fct-ggzishfk@2
name: viola-pty depends on no viola crate and knows no agent
rule: viola-pty depends on no other viola crate, knows no agent, and takes no thiserror.
@row_add n=896 row=decision/claude-shapes-in-one-crate area=crate-dependency-direction rec=fct-sfkaxxdn@2
name: Claude payload shapes in one crate
rule: viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
@row_add n=897 row=decision/agent-claude-pure area=crate-dependency-direction rec=fct-sfkaxxdn@2
name: viola-agent-claude does no file I/O
rule: viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin.
why: So the crate stays pure.
@row_add n=898 row=decision/mcp-dependencies area=crate-dependency-direction rec=fct-u4dur7ij@1
name: viola-mcp takes channel with tokio
rule: viola-mcp depends on viola-channel with its tokio feature and on tokio, and takes no serde.
@row_add n=899 row=decision/e2e-test-only area=crate-dependency-direction rec=fct-tr3aduln@1
name: viola-e2e is test-only
rule: viola-e2e is test-only, and no product crate depends on it.
@row_add n=900 row=decision/root-bin-consumes-normalised-events area=crate-dependency-direction rec=fct-p5s5gpab@2
name: Root bin consumes only normalised events
rule: The root bin depends on all members and holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.
@row_add n=901 row=decision/local-only-deployment area=deployment-model rec=fct-hupns7gt@1
name: Local-only deployment
rule: viola is deployed local-only.
@row_add n=902 row=absence/container-or-serverless-deployment area=deployment-model rec=fct-hupns7gt@1
capability: Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
@row_add n=903 row=decision/single-binary-on-path area=deployment-model rec=fct-4jixev2q@2
name: Single binary on PATH
rule: viola ships as a single binary installed on PATH.
@row_add n=904 row=decision/run-pins-self-copy area=deployment-model rec=fct-4jixev2q@2
name: run pins a copy of itself and its plugin
rule: viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.
@row_none n=89 rec=fct-a6y4blve@2 type=4
relied: A frame-writing client runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order.
@row_none n=90 rec=fct-4yfg5mra@1 type=4
relied: A frame verb against a stale or dead instance reads instance-dead, while a name with no instance directory reads as unwrapped.
@row_none n=91 rec=fct-ikwybol4@2 type=4
relied: A peer whose euid cannot be read is refused.
@row_none n=92 rec=fct-ntix5ygh@1 type=S
@row_none n=93 rec=fct-nhzfyd7i@1 type=6
@row_none n=94 rec=fct-2nyzhkqj@2 type=5
@row_none n=95 rec=fct-v4imvdl4@1 type=S
@row_none n=96 rec=fct-sgev6rlz@1 type=S
@row_none n=97 rec=fct-a4ctqcf6@2 type=5
@row_none n=98 rec=fct-x3nhuj7r@1 type=5
@row_none n=99 rec=fct-panweniz@1 type=S
@row_none n=100 rec=fct-sxdyevwi@1 type=S
@row_none n=101 rec=fct-uzytp74j@1 type=S
@row_none n=102 rec=fct-5iwrwosa@1 type=S
@row_add n=905 row=decision/exact-toolchain-pin area=project-directory-structure rec=fct-irhgihut@5
name: Exact Rust toolchain pin
rule: The project builds on one exactly pinned stable Rust toolchain with rustfmt and clippy, never a floating channel.
except: The separate fuzz workspace uses its own pinned nightly toolchain.
@row_add n=906 row=decision/no-tracing-level-macros area=project-directory-structure rec=fct-irhgihut@5
name: Tracing level macros banned
rule: The tracing info, warn, error, debug and trace macros are disallowed in the code.
guard: clippy disallowed-macros, checked by the lint probes
@row_add n=907 row=decision/anyhow-at-bin-edge-only area=project-directory-structure rec=fct-irhgihut@5
name: anyhow at the binary edge only
rule: anyhow error handling is confined to the edge of the viola binary.
@row_add n=908 row=decision/cargo-deny-bans area=project-directory-structure rec=fct-irhgihut@5
name: cargo-deny supply-chain policy
rule: cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features.
guard: deny-probes negative probe per ban with a clean control
@row_add n=909 row=decision/no-tokio-in-sync-crates area=project-directory-structure rec=fct-irhgihut@5
name: No tokio in sync crates
rule: Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root.
guard: cargo-deny tokio ban over the sync-crate list
@row_add n=910 row=decision/channel-tokio-behind-feature area=project-directory-structure rec=fct-irhgihut@5
name: Channel Tokio client behind a feature
rule: viola-channel's normal dependency graph carries no tokio; the Tokio client exists only behind its tokio feature.
guard: channel-tokio-free check in CI supply-chain
@row_add n=911 row=decision/rmcp-exact-features area=project-directory-structure rec=fct-irhgihut@5
name: rmcp features exactly server and transport-io
rule: viola's release graph requests rmcp with exactly the features server and transport-io.
guard: rmcp-features check in the CI lint job
@row_add n=912 row=decision/release-carries-viola-only area=project-directory-structure rec=fct-irhgihut@5
name: Release build carries viola only
rule: The release build carries the viola binary only and no test-only feature.
guard: release-check in the CI release job
@row_add n=913 row=decision/fuzz-separate-workspace area=project-directory-structure rec=fct-irhgihut@5
name: Fuzzing in a separate workspace
rule: Fuzz targets live in a separate cargo-fuzz workspace with its own lockfile, excluded from the root workspace, with committed synthetic seeds.
@row_add n=914 row=decision/board-read-model-shape area=project-directory-structure rec=fct-irhgihut@5
name: Board read model is sync and Claude-free
rule: The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, statusline_command, pid, started_at or links.
@row_add n=915 row=decision/zero-panics-gate area=project-directory-structure rec=fct-irhgihut@5
name: Zero panic lines in end-to-end runs
rule: No panic role line may appear under the end-to-end home, checked fail-closed.
guard: obs G2 zero-panics gate
except: A panic whose location is the hook seam module.
@row_add n=916 row=decision/snapshots-check-mode-only area=project-directory-structure rec=fct-irhgihut@5
name: Insta snapshots in check mode only
rule: Root integration test snapshots are only checked by tests, never written by them.
@row_add n=917 row=decision/channel-json-rpc-local-sockets area=project-directory-structure rec=fct-irhgihut@5
name: JSON-RPC over local sockets for the channel
rule: The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets.
@row_add n=918 row=decision/pty-seam-portable-pty area=project-directory-structure rec=fct-irhgihut@5
name: portable-pty is the PTY seam
rule: The PTY seam is built over portable-pty, with host raw mode through libc termios.
@row_add n=919 row=decision/unverified-cli-transport-only area=established-decisions rec=fct-7vnaqpcq@1,fct-24sxrnlw@1,fct-qwpklckh@2
name: Transport-only degrade on an unverified CLI
rule: On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_add n=920 row=decision/linux-only area=established-decisions rec=fct-hcu7z4ke@2,fct-hss5qy6y@1,fct-igbvtfv6@2,fct-fa2klbw5@3,fct-qndsom6a@3
name: Linux only
rule: viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree.
why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
@row_add n=921 row=decision/sync-core-tokio-only-in-mcp area=established-decisions rec=fct-l5jpezvw@2,fct-brnw2t5e@1,fct-gmzv3mtg@2
name: Hybrid sync core, Tokio only in mcp
rule: The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`.
why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio.
guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest`
except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.
@row_add n=922 row=decision/portable-pty-behind-pty-seam area=established-decisions rec=fct-omwrdnlc@3,fct-ad44nn2e@4
name: portable-pty pinned behind viola's pty seam
rule: portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill).
why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
@row_add n=923 row=decision/child-exit-on-process-handle area=established-decisions rec=fct-omwrdnlc@3,fct-ad44nn2e@4
name: Child exit detected on the process handle
rule: Child exit is detected on the process handle, never on EOF.
@row_add n=924 row=decision/vt100-readiness-gate area=established-decisions rec=fct-hsjl3wtp@1,fct-gollksbj@1
name: vt100 as a pre-send readiness gate only
rule: vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model.
why: CLI-native modals bypass every hook, and the screen lags the hooks.
@row_add n=925 row=decision/ndjson-logs-and-atomic-snapshots area=established-decisions rec=fct-73drmylz@2,fct-ak4xjyyx@3
name: ndjson append logs plus atomic JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth.
why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
@row_add n=926 row=decision/separate-lock-files area=established-decisions rec=fct-73drmylz@2,fct-ak4xjyyx@3,fct-s7lo3z2q@2
name: Locks on separate .lock files
rule: State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked.
why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
@row_add n=927 row=decision/tempfile-snapshot-writer area=established-decisions rec=fct-2sdvn2pr@1,fct-kfz3cseb@2
name: tempfile as the snapshot writer
rule: Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced.
why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
@row_add n=928 row=decision/ipc-interprocess-local-socket area=established-decisions rec=fct-w2igrcse@2
name: interprocess local_socket for IPC
rule: Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_add n=929 row=decision/identity-by-instance area=established-decisions rec=fct-qwajajfk@1,fct-hag5uwzu@1
name: Identity by instance, never by location
rule: Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys.
why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
@row_add n=930 row=contract/mcp-tools/send area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: send
promise: One of the five MCP tools; takes `{target, text}`, `target` a `ViolaName`.
@row_add n=931 row=contract/mcp-tools/wait area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: wait
promise: One of the five MCP tools; takes `{target, after?, timeout_ms?}`.
@row_add n=932 row=contract/mcp-tools/last area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: last
promise: One of the five MCP tools; takes `{target}`.
@row_add n=933 row=contract/mcp-tools/answer area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: answer
promise: One of the five MCP tools; takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
@row_add n=934 row=contract/mcp-tools/list area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: list
promise: One of the five MCP tools; takes `{}` and shows the driver the sessions.
@row_add n=935 row=contract/hook-tiers/sync-with-decision area=established-decisions rec=fct-zhr3wx2i@2,fct-xbwprhwr@2
boundary: Hook tiers
item: sync with a decision
promise: PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest, run sync and may reply with a decision.
@row_add n=936 row=contract/hook-tiers/async area=established-decisions rec=fct-zhr3wx2i@2,fct-xbwprhwr@2
boundary: Hook tiers
item: async
promise: Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
@row_add n=937 row=decision/hooks-fail-open area=established-decisions rec=fct-4nt6qtyp@3,fct-nppkdemz@1,fct-wpbp4hte@2
name: Hooks fail open
rule: `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr.
why: Exit 2 blocks Claude Code, and a viola bug must never block the session.
except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
@row_add n=938 row=decision/wheel-starts-with-driver area=established-decisions rec=fct-dl246x3d@1,fct-4coxmu76@2
name: The wheel starts with the driver
rule: The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key.
@row_add n=939 row=decision/send-never-queued area=established-decisions rec=fct-dl246x3d@1,fct-4coxmu76@2
name: A send mid-turn or mid-send is refused, never queued
rule: A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
@row_add n=940 row=decision/nutype-newtypes area=established-decisions rec=fct-caebuvjq@1,fct-tkfup5lp@1
name: nutype newtypes for validation
rule: Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
@row_add n=941 row=decision/plugin-compiled-into-binary area=established-decisions rec=fct-c3wssf3l@2,fct-23yj6ah3@2
name: Plugin compiled into the binary
rule: viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start.
why: The binary and the plugin can no longer drift apart.
@row_add n=942 row=decision/pinned-exe-copy area=established-decisions rec=fct-c3wssf3l@2,fct-23yj6ah3@2
name: Pinned exe copy keyed by version and content hash
rule: At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls.
why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
@row_add n=943 row=decision/plugin-only-in-wrapped-sessions area=established-decisions rec=fct-le5sc2tk@1,fct-4jk25cex@2
name: Plugin loads only into wrapped sessions
rule: The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`.
why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola.
except: `viola verify`'s transient unwrapped children, all gone when verify ends.
@row_add n=944 row=decision/prompt-text-from-stdin-or-file area=established-decisions rec=fct-nvft5tqc@2,fct-onw4pywk@3
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument.
@row_add n=945 row=absence/mobile-view area=established-decisions rec=fct-6zkbklpd@1,fct-cmvs3cxm@1
capability: A mobile or phone view
@row_add n=946 row=contract/channel-methods/send area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: send
promise: A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
@row_add n=947 row=contract/channel-methods/wait area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: wait
promise: A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
@row_add n=948 row=contract/channel-methods/last area=standard-contracts rec=fct-5chyftpi@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: last
promise: A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
@row_add n=949 row=contract/channel-methods/answer area=standard-contracts rec=fct-er6xfhrr@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: answer
promise: A request: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind.
@row_add n=950 row=contract/channel-methods/pause area=standard-contracts rec=fct-7jswn64g@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: pause
promise: A request: `pause` `{from?}` answers `{wheel:"human"}`, the wheel moving to `human` / `manual-pause`.
@row_add n=951 row=contract/channel-methods/release area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: release
promise: A request: `release` `{budget?:bool, from?}` answers `{wheel:<holder>, budget_paused:bool, override_until?}`.
@row_add n=952 row=contract/channel-methods/link area=standard-contracts rec=fct-ax5zoxh3@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: link
promise: A request: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper.
@row_add n=953 row=contract/channel-methods/unlink area=conventions rec=fct-4ghn4t2o@1,fct-ax5zoxh3@1,fct-vi22dxmq@1
boundary: channel methods
item: unlink
promise: A request with an integer JSON-RPC id, monotonic per connection; `unlink` `{driver, driven}` answers `{}`, sent to the driven instance's wrapper.
@row_add n=954 row=contract/channel-methods/hook-dialog area=conventions rec=fct-4ghn4t2o@1,fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog
promise: A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
@row_add n=955 row=contract/channel-methods/hook-event area=conventions rec=fct-4ghn4t2o@1,fct-nf7gdkwl@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.event
promise: A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
@row_add n=956 row=contract/refusal-reason/human-typing area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: human-typing
promise: Serialised kebab-case; its detail is null or manual-pause.
@row_add n=957 row=contract/refusal-reason/budget-paused area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: budget-paused
promise: Serialised kebab-case; its detail is five-hour or seven-day.
@row_add n=958 row=contract/refusal-reason/unverified-cli area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: unverified-cli
promise: Serialised kebab-case; its detail is null.
@row_add n=959 row=contract/refusal-reason/not-delivered area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: not-delivered
promise: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_add n=960 row=contract/cli-exit-codes/0 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 0
promise: ok.
@row_add n=961 row=contract/cli-exit-codes/1 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2,fct-2y3oelpa@1,fct-czy74f6t@1,fct-zvpgaziz@1
boundary: CLI exit codes
item: 1
promise: Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
@row_add n=962 row=contract/cli-exit-codes/2 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 2
promise: Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
@row_add n=963 row=contract/cli-exit-codes/10 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 10
promise: The `human-typing` refusal.
@row_add n=964 row=contract/cli-exit-codes/11 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 11
promise: The `budget-paused` refusal.
@row_add n=965 row=contract/cli-exit-codes/12 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 12
promise: The `unverified-cli` refusal.
@row_add n=966 row=contract/cli-exit-codes/13 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 13
promise: The `not-delivered` refusal.
@row_add n=967 row=contract/cli-exit-codes/14 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 14
promise: The `unknown` refusal.
@row_add n=968 row=contract/cli-exit-codes/20 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 20
promise: Protocol fault from the wrapper, a -32602 from link or unlink included; the link self-pair is refused by the wrapper only, never client-side.
@row_add n=969 row=contract/cli-exit-codes/21 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2,fct-7s455w3m@2,fct-2y2sv475@1,fct-uqyfijk4@2
boundary: CLI exit codes
item: 21
promise: Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
@row_add n=970 row=contract/mcp-tool-results/success area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP tool results
item: success
promise: A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, budget, skipped}` payload.
@row_add n=971 row=contract/mcp-json-rpc-errors/32602 area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP JSON-RPC errors
item: -32602
promise: An unknown tool name is the JSON-RPC error `-32602`; JSON-RPC errors are only the MCP connection's own faults.
@row_add n=972 row=contract/mcp-json-rpc-errors/32603 area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP JSON-RPC errors
item: -32603
promise: An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is `-32603` `internal error` with `data: null`.
@row_add n=973 row=absence/database area=conventions rec=fct-tnxtyezf@1,fct-gfhsqoso@1
capability: A database
@row_add n=974 row=contract/list-payload/budget area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1,fct-dtnnvvl3@1
boundary: list payload
item: budget
promise: A top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`; each window is `{used_percentage, resets_at}`, a field `"unknown"` when unparseable and a window `"unknown"` when omitted; `paused` only says the newest reading crosses a threshold.
@row_add n=975 row=contract/snapshot-envelope/shape area=standard-contracts rec=fct-vufggymy@1,fct-uve5rstc@1
boundary: snapshot envelope
item: shape
promise: A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
@row_add n=976 row=contract/ledger-stamps/writer area=standard-contracts rec=fct-scthzguh@1,fct-62qgonvf@2
boundary: ledger stamps envelope
item: writer
promise: `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
@row_add n=977 row=contract/instance-snapshot/links area=standard-contracts rec=fct-hv3qoyp2@1,fct-q5tbsmkw@1,fct-vgnixbh3@1
boundary: instance snapshot
item: links
promise: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
@row_add n=978 row=contract/list-payload/ok area=standard-contracts rec=fct-cqamt3xw@1,fct-smoe4pw5@1
boundary: list payload
item: ok
promise: The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, budget, skipped}`; `list` is not a channel method and needs no live wrapper.
@row_add n=979 row=contract/mcp-refusal-result/shape area=standard-contracts rec=fct-2ry3h2lh@1,fct-lcysncyb@1
boundary: MCP refusal result
item: result shape
promise: `{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,"structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}`
@row_add n=980 row=decision/no-shell-out area=standard-contracts rec=fct-horlp4ay@2,fct-fa2klbw5@3
name: No shelling out
rule: Nothing shells out through `sh` or `bash`; child processes are spawned directly.
except: `hook statusline` runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin.
@row_add n=981 row=decision/connect-to-recorded-endpoint area=occupied-resources rec=fct-wular7qm@2,fct-nm6raycf@2
name: Clients connect to the recorded endpoint
rule: The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path.
why: Processes spawned inside the session may run with a different environment from the wrapper.
@row_add n=982 row=decision/no-abstract-namespace-sockets area=occupied-resources rec=fct-wular7qm@2,fct-nm6raycf@2
name: No abstract-namespace sockets
rule: Linux abstract-namespace socket names are never used.
why: They carry no filesystem permissions and any local user could connect.
@row_add n=983 row=name/binary/viola area=occupied-resources rec=fct-lvi3drxp@2,fct-irhgihut@5
class: binary
name: viola
@row_add n=984 row=name/binary/viola-fake-agent area=occupied-resources rec=fct-v2uqfwgj@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-fake-agent
@row_add n=985 row=name/binary/viola-harness area=occupied-resources rec=fct-v2uqfwgj@1,fct-j2leq5fi@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-harness
@row_add n=986 row=name/binary/viola-mcp-tap area=occupied-resources rec=fct-v2uqfwgj@1,fct-j2leq5fi@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-mcp-tap
@row_add n=987 row=decision/test-binaries-never-released area=occupied-resources rec=fct-v2uqfwgj@1,fct-bzckmn3n@2
name: Test-only binaries kept out of release builds
rule: viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
@row_add n=988 row=name/command/run area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: run
@row_add n=989 row=name/command/revive area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: revive
@row_add n=990 row=name/command/send area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: send
@row_add n=991 row=name/command/wait area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: wait
@row_add n=992 row=name/command/last area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: last
@row_add n=993 row=name/command/list area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: list
@row_add n=994 row=name/command/answer area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: answer
@row_add n=995 row=name/command/hook area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: hook
@row_add n=996 row=name/command/mcp area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: mcp
@row_add n=997 row=name/command/verify area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: verify
@row_add n=998 row=name/command/pause area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: pause
@row_add n=999 row=name/command/release area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: release
@row_add n=1000 row=name/command/link area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: link
@row_add n=1001 row=name/command/unlink area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: unlink
@row_add n=1002 row=name/command/plugin-install area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: plugin install
@row_add n=1003 row=name/other/mcp-tool-send area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: send
@row_add n=1004 row=name/other/mcp-tool-wait area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: wait
@row_add n=1005 row=name/other/mcp-tool-last area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: last
@row_add n=1006 row=name/other/mcp-tool-answer area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: answer
@row_add n=1007 row=name/other/mcp-tool-list area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: list
@row_add n=1008 row=name/crate/viola area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1
class: crate
name: viola
@row_add n=1009 row=name/crate/viola-core area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-core
@row_add n=1010 row=name/crate/viola-pty area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-pty
@row_add n=1011 row=name/crate/viola-channel area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-channel
@row_add n=1012 row=name/crate/viola-state area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-state
@row_add n=1013 row=name/crate/viola-agent-claude area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-agent-claude
@row_add n=1014 row=name/crate/viola-mcp area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-mcp
@row_add n=1015 row=name/crate/viola-e2e area=occupied-resources rec=fct-j2leq5fi@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-e2e
@row_add n=1016 row=contract/child-environment/viola-dir area=occupied-resources rec=fct-esr7jyhf@1,fct-7uan73fu@1
boundary: The wrapped child's environment
item: VIOLA_DIR
promise: Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
@row_add n=1017 row=decision/no-live-claude-in-ci area=occupied-resources rec=fct-mkgpzodl@1,fct-5lqu6sil@2
name: No live claude in CI
rule: When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables.
why: so the real claude never runs in CI
@row_add n=1018 row=contract/config-json/v area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: v
promise: config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
@row_add n=1019 row=contract/config-json/five-hour area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: five_hour
promise: The five-hour budget threshold, a JSON whole number from 0 to 100, default 90; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
@row_add n=1020 row=contract/config-json/seven-day area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: seven_day
promise: The seven-day budget threshold, a JSON whole number from 0 to 100, default 85; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
@row_add n=1021 row=contract/config-json/diagnostics-level area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: diagnostics_level
promise: "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.
@row_add n=1022 row=contract/config-json/claude-env-keep area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: claude_env_keep
promise: The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect.
@row_add n=1023 row=contract/config-json/dialog-hold-s area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
@row_add n=1024 row=decision/recorded-fixtures-scrubbed area=occupied-resources rec=fct-kyb4p5xt@1,fct-3fjm2cf6@2
name: Recorded fixtures scrubbed and hygiene-walked
rule: Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content.
guard: contract_fixture_hygiene
@row_add n=1025 row=decision/config-precedence area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
name: Configuration precedence
rule: CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
@row_add n=1026 row=decision/viola-home-resolution area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
name: Viola home resolution order
rule: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it.
why: The home is the isolation point for tests: each test runs in its own home.
@row_add n=1027 row=absence/secrets area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
capability: Secrets of any kind in v1 configuration
@row_add n=1028 row=decision/capability-ledger-gate area=cross-cutting-patterns rec=fct-unnwkesq@1,fct-jkmwg2b6@1,fct-jo2qqab5@1,fct-wfge3rsz@3
name: Capability ledger as the single gate for CLI behaviour
rule: Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption.
except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
why: A ruled limit sets the exception.
@row_add n=1029 row=decision/bounded-readers area=cross-cutting-patterns rec=fct-tpj3dxid@1,fct-qip7sr3o@1
name: Bounded readers at every boundary
rule: Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded.
except: The own-executable reads, which are no external input.
guard: contract_bounded_readers: product_readers_are_each_bounded_or_named
@row_add n=1030 row=decision/thiserror-one-enum-per-crate area=established-decisions rec=fct-xqkfyxga@3,fct-vf75c7sp@2
name: thiserror, one error enum per workspace crate
rule: Each workspace crate has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError.
except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
@row_add n=1031 row=contract/channel-link/already-linked area=standard-contracts rec=fct-rma7vozl@1,fct-anqliuvh@1
boundary: channel link method
item: already_linked
promise: A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
@row_add n=1032 row=decision/verify-child-set area=established-decisions rec=fct-qwpklckh@2
name: Verify's bounded child set
rule: viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.
why: Each widening of the child set was ratified.
@row_add n=1033 row=contract/viola-verify-exit-codes/1-relied-row-failed area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify exit codes
item: 1
promise: A relied ledger row failed; the failing row's stamp is still written.
@row_add n=1034 row=decision/permission-request-question-stays-null area=established-decisions rec=fct-qwpklckh@2
name: A question first raised by PermissionRequest stays null
rule: A question first raised by PermissionRequest stays null to the human.
why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 25th row, its viola verify probe and a re-stamp of every stamped version.
@row_add n=1035 row=contract/viola-revive-json/error-document area=established-decisions rec=fct-onw4pywk@3,fct-mnor3ks3@2
boundary: viola revive --json
item: error document
promise: Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
@row_add n=1036 row=decision/fake-agent-pump-delay-seam area=occupied-resources rec=fct-2hatag6l@1
name: FAKE_AGENT_PUMP_DELAY_MS test seam
rule: FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction.
why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.
@row_add n=1037 row=decision/end-to-end-homes-kept-after-each-job area=occupied-resources rec=fct-x2qzlbqt@1,fct-xib5fpty@2
name: Homes kept after each job's end-to-end run
rule: Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out.
why: The homes are kept in CI for the obs gates.
@row_add n=1038 row=decision/mcp-server-rmcp area=stack-and-technologies rec=fct-qeiflw5v@1,fct-irhgihut@5
name: rmcp is the MCP server
rule: The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
@row_add n=1039 row=decision/plugin-embedded-in-binary area=stack-and-technologies rec=fct-5oigicbx@1,fct-irhgihut@5
name: Claude Code plugin compiled into the binary
rule: The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.
@row_add n=1040 row=decision/malformed-stamps-unverified area=standard-contracts rec=fct-62qgonvf@2
name: Malformed or untrusted stamps read unverified
rule: A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
@row_add n=1041 row=contract/mcp-list/refusal area=standard-contracts rec=fct-smoe4pw5@1,fct-yj2flshm@2
boundary: MCP list
item: refusal
promise: The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
@row_add n=1042 row=decision/dual-licence area=build-system rec=fct-duju5u6d@1,fct-irhgihut@5
name: MIT OR Apache-2.0 licence
rule: The project is licensed MIT OR Apache-2.0, set once for the workspace and inherited by every member, with both licence texts at the root and the README naming both.
