@row_add n=1 row=contract/exit-21-human-hints/placement area=conventions rec=1
boundary: Exit 21 human hints
item: placement
promise: Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
@row_add n=2 row=contract/cli-exit-21/strict-modes-failed area=conventions rec=2
boundary: CLI exit 21
item: strict-modes-failed
promise: A refused strict-modes check of send, wait, last, answer, pause or release exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
@row_none n=1 rec=3 type=4
relied: A failed connect reads as an instance-dead cause and a closed or broken stream as an endpoint-lost cause, which the exit 21 detail words rest on.
@row_add n=3 row=contract/cli-exit-21/socket-dir-refused area=conventions rec=4
boundary: CLI exit 21
item: socket-dir-refused
promise: On send, wait, last, answer, pause, release, link and unlink a refused socket directory exits 21 and writes one process-exit line at WARN with exit_code 21, detail socket-dir-refused and during connect, and no channel-request.
@row_add n=4 row=contract/mcp-tool-results/success area=conventions rec=5
boundary: MCP tool results
item: success
promise: A success is a tool result with isError false and the channel ok payload as structuredContent; for list, the caller-side {items, budget, skipped} payload.
@row_add n=5 row=contract/mcp-tool-results/refusal area=conventions rec=5
boundary: MCP tool results
item: refusal
promise: A refusal is a tool result with isError true and the channel's refusal and detail in structuredContent.
@row_add n=6 row=contract/mcp-tool-results/instance-unreachable area=conventions rec=5
boundary: MCP tool results
item: instance-unreachable
promise: An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_add n=7 row=contract/mcp-tool-results/wrapper-fault area=conventions rec=5
boundary: MCP tool results
item: wrapper-fault
promise: A wrapper protocol fault is an isError true result {"error":"wrapper-fault","detail":…} carrying the sanitised JSON-RPC fault {code,message,data}: a fixed message, data a closed object or null.
@row_add n=8 row=contract/mcp-tool-results/invalid-params area=conventions rec=5
boundary: MCP tool results
item: invalid-params
promise: A tool argument failing re-validation is an isError true result {"error":"invalid-params","detail":"<field>"}, the field one of target, text, after, timeout_ms, dialog_id, response, with no frame written.
@row_add n=9 row=contract/mcp-tool-results/is-error-text area=conventions rec=5
boundary: MCP tool results
item: isError text
promise: Every isError result's text is refused: <code>.
@row_add n=10 row=contract/mcp-json-rpc-errors/32602 area=conventions rec=5
boundary: MCP JSON-RPC errors
item: -32602
promise: An unknown tool name is the JSON-RPC error -32602.
@row_add n=11 row=contract/mcp-json-rpc-errors/32603 area=conventions rec=5
boundary: MCP JSON-RPC errors
item: -32603
promise: An internal handler fault (any other channel error, a reply of no known shape, a wait or last refusal, a panicked call) is -32603 internal error with data null.
@row_add n=12 row=contract/mcp-list-results/ttlms area=conventions rec=5
boundary: MCP list results
item: ttlMs
promise: The tools/list and server/discover results each carry ttlMs 0, because claude drops every tool of a list result without it.
@row_add n=13 row=contract/mcp-list-results/cachescope area=conventions rec=5
boundary: MCP list results
item: cacheScope
promise: The tools/list and server/discover results each carry cacheScope private, because claude drops every tool of a list result without it.
@row_add n=14 row=decision/mcp-tools-list-held-to-recorded-exchange area=conventions rec=5
name: MCP tools/list held to a recorded claude exchange
rule: The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order.
guard: mcp_recorded_exchange (viola-e2e)
@row_add n=15 row=contract/mcp-frame-tools/server-verify area=conventions rec=6
boundary: MCP frame tools
item: server-verify
promise: A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.
@row_add n=16 row=contract/mcp-frame-tools/socket-dir-refused area=conventions rec=7
boundary: MCP frame tools
item: socket-dir-refused
promise: A send, wait, last or answer meeting a refused socket directory returns the instance-unreachable isError result with detail socket-dir-refused, no frame written and no log line.
@row_add n=17 row=contract/mcp-frame-tools/strict-modes-failed area=conventions rec=8
boundary: MCP frame tools
item: strict-modes-failed
promise: A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
@row_add n=18 row=decision/config-precedence area=cross-cutting-patterns rec=9
name: Config precedence
rule: CLI flags override the home's config.json, which overrides built-in defaults.
@row_add n=19 row=decision/viola-home-resolution area=cross-cutting-patterns rec=9
name: Viola home resolution
rule: The viola home is the --home flag, else the instance home named by a valid VIOLA_NAME and VIOLA_DIR pair, else the user home's .viola; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it.
why: The home is the isolation point for tests: each test runs in its own home.
@row_add n=20 row=decision/statusline-source-follows-home area=cross-cutting-patterns rec=9
name: Statusline source follows the home
rule: The statusline source is the home's statusline-source.json whenever it exists; only the default home falls back to the user's Claude settings, so a home named by --home or reached through VIOLA_DIR never reads the user's settings.
@row_add n=21 row=decision/hook-exits-on-untrusted-instance area=cross-cutting-patterns rec=9
name: Hook exits at once on an untrusted instance
rule: viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append.
except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
@row_add n=22 row=contract/config-json/v area=cross-cutting-patterns rec=9
boundary: config.json
item: v
promise: config.json carries a version field v.
@row_add n=23 row=contract/config-json/unknown-keys area=cross-cutting-patterns rec=9
boundary: config.json
item: unknown keys
promise: config.json is parsed tolerantly: unknown keys are skipped and counted.
@row_add n=24 row=contract/config-json/five-hour area=cross-cutting-patterns rec=9
boundary: config.json
item: five_hour
promise: The five-hour budget threshold, a Percent, default 90.
@row_add n=25 row=contract/config-json/seven-day area=cross-cutting-patterns rec=9
boundary: config.json
item: seven_day
promise: The seven-day budget threshold, a Percent, default 85.
@row_add n=26 row=contract/config-json/diagnostics-level area=cross-cutting-patterns rec=9
boundary: config.json
item: diagnostics_level
promise: The process-log level, info by default or debug.
@row_add n=27 row=contract/config-json/claude-env-keep area=cross-cutting-patterns rec=9
boundary: config.json
item: claude_env_keep
promise: The pass-list of CLAUDE* environment variable names kept for the child, default empty.
@row_add n=28 row=contract/config-json/dialog-hold-s area=cross-cutting-patterns rec=9
boundary: config.json
item: dialog_hold_s
promise: The dialog hold in whole seconds, 60 to 3600, default 60.
@row_add n=29 row=decision/diagnostics-level-sole-log-level area=cross-cutting-patterns rec=9
name: diagnostics_level as the sole log level source
rule: The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
@row_add n=30 row=decision/no-environment-configuration area=cross-cutting-patterns rec=9
name: Environment variables are no configuration channel
rule: No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does.
except: NO_COLOR, TERM and COLORTERM, read by viola's own colour layer for list's human mode; two fake-agent test seams compiled only under the fake-agent feature.
@row_add n=31 row=decision/clap-without-color-feature area=cross-cutting-patterns rec=9
name: clap built without its color feature
rule: clap is built without its color feature, so viola's own colour layer is the only reader of colour environment variables.
why: The feature's anstream and anstyle-query route read CLICOLOR, CLICOLOR_FORCE, NO_COLOR, TERM, COLORTERM and CI.
@row_add n=32 row=absence/secrets area=cross-cutting-patterns rec=9
capability: Secrets of any kind in configuration
@row_add n=33 row=decision/capability-ledger-gate area=cross-cutting-patterns rec=10
name: Capability ledger as the single gate for CLI behaviour
rule: Any new dependence on an undocumented claude behaviour is added as a capability ledger row with a viola verify probe that checks a post-condition, never hard-coded as an unconditional assumption.
except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
@row_add n=34 row=decision/agents-join-read-ungated area=cross-cutting-patterns rec=10
name: Agents join read not gated on a stamp
rule: viola list and viola revive do not gate their claude agents --json read on a ledger stamp.
why: The read only enriches and its failure is never an error.
@row_add n=35 row=decision/bounded-readers area=cross-cutting-patterns rec=11
name: Bounded readers at every boundary
rule: Every product reader of external bytes is bounded in the same statement or named as an exception; none reads an external byte source to its end unbounded.
guard: contract_bounded_readers: product_readers_are_each_bounded_or_named
except: The own-executable reads, which are no external input.
@row_add n=36 row=decision/serde-json-unbounded-depth-banned area=cross-cutting-patterns rec=11
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in the dependency policy.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_add n=37 row=decision/sanitised-error-surfaces area=cross-cutting-patterns rec=12
name: Sanitised error surfaces
rule: CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value.
guard: contract_error_bodies: product_error_writers_are_each_named
except: A code word outside the closed list stays visible by design.
@row_add n=38 row=decision/thiserror-messages-interpolate-nothing area=cross-cutting-patterns rec=12
name: thiserror messages interpolate nothing
rule: A thiserror error message interpolates no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=39 row=contract/exit-21-human-hints/socket-dir area=cross-cutting-patterns rec=12
boundary: Exit 21 human hints
item: socket-dir
promise: The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
@row_add n=40 row=decision/os-gated-tests-named area=cross-cutting-patterns rec=13
name: OS-gated tests named with their reason
rule: Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands.
guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
@row_add n=41 row=decision/no-other-os-test-gates area=cross-cutting-patterns rec=13
name: No test gate naming another OS
rule: A test gate whose predicate names windows or macOS is a fault even when the gates table names it.
guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
@row_add n=42 row=decision/sync-core-tokio-only-in-mcp area=established-decisions rec=14
name: Hybrid sync core, Tokio only in mcp
rule: The core is synchronous and Tokio is confined to the mcp subcommand.
why: portable-pty's handles block, so the pump needs threads anyway; hook spawns per event and must not pay runtime startup; rmcp and interprocess-async require Tokio.
guard: cargo deny tokio ban (deny-sync.toml) per listed sync crate, and cargo check of the sync crates in CI
except: viola-channel's optional tokio feature, enabled by viola-mcp; its graph without the feature is proven tokio-free.
@row_add n=43 row=decision/portable-pty-behind-pty-seam area=established-decisions rec=15
name: portable-pty pinned behind the pty seam
rule: portable-pty, pinned exactly at 0.9.0, is the PTY library, used only behind viola's own pty seam (spawn, read, write, resize, wait, kill).
why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no pin-utils, so RUSTSEC-2026-0441 does not reach the build.
@row_add n=44 row=decision/child-exit-on-process-handle area=established-decisions rec=15
name: Child exit detected on the process handle
rule: The child's exit is detected on the process handle, never on EOF.
@row_add n=45 row=decision/child-cwd-explicit area=established-decisions rec=15
name: Child cwd set explicitly
rule: The child's cwd is set explicitly to viola's current directory.
except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
@row_add n=46 row=decision/revive-program-resolution area=established-decisions rec=15
name: Revive resolves the program from its own PATH
rule: Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola.
why: Ratified as a boundary widening.
@row_add n=47 row=contract/viola-revive-refusals/cwd-missing area=established-decisions rec=15
boundary: viola revive refusals
item: cwd-missing
promise: A revive whose recorded cwd names no existing directory is refused cwd-missing.
@row_add n=48 row=contract/child-exit-record/exit-source area=established-decisions rec=15
boundary: Child exit record
item: exit_source
promise: The child's exit is recorded with exit_source handle-wait or kill-fallback.
@row_add n=49 row=decision/no-key-held-behind-resize area=established-decisions rec=15
name: No human key held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize; a test that writes a key after a resize waits for the child to observe the new size first.
@row_add n=50 row=decision/vt100-readiness-gate area=established-decisions rec=16
name: vt100 screen model as a pre-send readiness gate only
rule: vt100 0.16.2 is used only as a pre-send readiness gate: after turn-ended it waits for a quiet screen, requires the absence of every known modal signature and waits up to its maximum for the input-box signature.
why: CLI-native modals bypass every hook and the screen lags the hooks.
@row_add n=51 row=decision/passthrough-never-waits-on-model area=established-decisions rec=16
name: Passthrough never waits on the screen model
rule: The human's bytes are written first and the screen model gets a copy through a bounded queue it may drop from, so the passthrough never waits on the model.
@row_add n=52 row=decision/poisoned-screen-model area=established-decisions rec=16
name: Poisoned screen model reads not ready
rule: A caught vt100 panic or a dropped feed message poisons the screen model until the host size changes, and a poisoned model reads input-not-ready.
why: A dropped byte desynchronises the screen.
@row_add n=53 row=contract/parse-rejected-event/vt100-feed area=established-decisions rec=16
boundary: parse-rejected event
item: vt100-feed
promise: One parse-rejected line with parser vt100-feed per poisoning episode, its detail panicked for a caught panic and oversize for a dropped message.
@row_add n=54 row=decision/gate-timings-compiled area=established-decisions rec=16
name: Readiness gate timings compiled
rule: The quiet period (300 ms) and the maximum wait (8.5 s) are compiled built-ins, validated per CLI version by the stamped quiet-period ledger row.
why: The maximum wait covers the CLI's 8.0 s paste hint.
@row_add n=55 row=decision/screen-signatures-compiled area=established-decisions rec=16
name: Screen signatures as compiled literals
rule: Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when a row holds an input-box literal and no row holds a modal literal.
why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
@row_add n=56 row=decision/partial-gate-on-unverified-cli area=established-decisions rec=16
name: Partial readiness gate on an unverified CLI
rule: On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
@row_add n=57 row=contract/send-results/input-not-ready area=established-decisions rec=16
boundary: send results
item: input-not-ready
promise: A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
@row_add n=58 row=decision/screen-rows-read-only-by-verify area=established-decisions rec=16
name: Screen row text read only by verify
rule: The screen model has one row-text exit, and only viola verify reads it.
@row_add n=59 row=decision/no-typing-into-trust-dialogs area=established-decisions rec=16
name: viola types into no trust dialog
rule: viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
@row_add n=60 row=decision/every-send-confirmed area=established-decisions rec=17
name: Every send is confirmed
rule: A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted.
why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate.
@row_add n=61 row=decision/confirm-window-compiled area=established-decisions rec=17
name: Confirmation window compiled
rule: The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp.
except: A /compact send waiting for its post-condition has its own 40 s window.
@row_add n=62 row=decision/compact-confirm-window area=established-decisions rec=17
name: /compact confirmation window
rule: A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency.
state: provisional
@row_add n=63 row=decision/send-time-bound area=established-decisions rec=17
name: Send time bound
rule: send blocks at most the readiness gate's maximum wait plus the paste plus the window, and on the MCP surface that total stays below the MCP client's tool-call timeout.
@row_add n=64 row=decision/exact-prompt-match area=established-decisions rec=17
name: Exact prompt match after agent normalisation
rule: The typed text is matched exactly against prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and its frame and reverses tag escaping, so the wrapper-side matcher never sees a Claude-specific shape.
@row_add n=65 row=decision/typed-text-normalisation area=established-decisions rec=17
name: Typed text normalisation
rule: send types its text with every CR LF pair and every other CR as one LF and without trailing CR and LF characters, nothing else changed, and uses that typed text for classification, matching, text_bytes and the paste.
why: The CLI drops a pasted text's last newline and submits a CR as LF, so a text typed with them failed the exact match.
@row_add n=66 row=contract/send-refusals/empty-text area=established-decisions rec=17
boundary: send refusals
item: empty-text
promise: A send whose typed text is empty is refused not-delivered / empty-text (exit 13) and never issued: nothing reserved, typed or logged as a send, and no wheel read.
@row_add n=67 row=contract/events-ndjson/send-confirmed-order area=established-decisions rec=17
boundary: events.ndjson
item: send-confirmed order
promise: The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts.
@row_add n=68 row=decision/local-commands-by-ledger area=established-decisions rec=17
name: Local commands classified by the ledger
rule: A send is a local command only by exact equality of its typed text with the ledger's local-command list, never by a leading slash; any other slash text, skills included, is an ordinary send.
why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
@row_add n=69 row=contract/send-results/unconfirmable area=established-decisions rec=17
boundary: send results
item: unconfirmable
promise: A listed local command whose post-condition is none, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
@row_add n=70 row=decision/clear-confirmed-by-new-session area=established-decisions rec=17
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed by a session-start of cause clear whose agent session id differs from the remembered one, and by no prompt-submitted.
@row_add n=71 row=decision/compact-confirmed-by-same-session area=established-decisions rec=17
name: /compact confirmed by the same session
rule: /compact on a verified CLI is confirmed by a session-start of cause compact whose agent session id equals the remembered one, and by no prompt-submitted.
@row_add n=72 row=decision/ndjson-logs-and-atomic-snapshots area=established-decisions rec=18
name: ndjson append logs and atomic JSON snapshots
rule: State is ndjson append logs plus atomic JSON snapshots, with std File::lock on separate lock files; the log is the source of truth.
why: The audit trail stays human-readable and tail-able, with no C in the build and no daemon; SQLite and redb were rejected.
@row_add n=73 row=decision/append-heals-torn-line area=established-decisions rec=18
name: The next append heals a torn line
rule: Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_add n=74 row=decision/reader-counts-skipped-lines area=established-decisions rec=18
name: Event reader counts what it skips
rule: The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
@row_add n=75 row=contract/list-json/skipped area=established-decisions rec=18
boundary: viola list --json
item: skipped
promise: skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
@row_add n=76 row=decision/separate-lock-files area=established-decisions rec=18
name: Lock files separate from logs
rule: Lock files are separate from logs, so no file is both appended to and exclusively locked.
why: Chosen while Windows, where that pair fails, was a target, and kept.
@row_add n=77 row=decision/wheel-in-wrapper-memory area=established-decisions rec=18
name: Wheel decided in wrapper memory
rule: The wheel is decided in the wrapper's memory and mirrored to a snapshot.
@row_add n=78 row=decision/budget-readings-last-writer-wins area=established-decisions rec=18
name: Budget readings last-writer-wins
rule: Budget readings are timestamped and last-writer-wins under File::lock.
@row_add n=79 row=decision/sqlite-index-upgrade-path area=established-decisions rec=18
name: SQLite index as upgrade path
rule: If a real query need appears, a derived, rebuildable SQLite index sits beside the authoritative log, which stays the source of truth.
@row_add n=80 row=absence/daemon area=established-decisions rec=18,23
capability: A daemon
@row_add n=81 row=decision/tempfile-snapshot-writer area=established-decisions rec=19
name: tempfile as the snapshot writer
rule: Snapshot and private file writes go through tempfile's persist and persist_noclobber via shared state helpers; no call site carries its own temp-and-rename code.
why: atomic-write-file's BSD-3-Clause licence is outside the deny.toml allow list; tempfile was already locked, MIT OR Apache-2.0.
@row_add n=82 row=decision/pinned-copy-no-clobber-placement area=established-decisions rec=19,20,22
name: Pinned copy placed no-clobber with the handle closed
rule: The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched.
why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
@row_none n=2 rec=21 type=4
@row_add n=83 row=decision/interprocess-local-socket-ipc area=established-decisions rec=23
name: interprocess local sockets behind the channel seam
rule: IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
@row_add n=84 row=decision/wait-blocks-nothing-polls area=established-decisions rec=23
name: wait blocks, nothing polls
rule: wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
@row_add n=85 row=contract/channel-wait/driver-relevant-events area=established-decisions rec=23
boundary: channel wait
item: driver-relevant events
promise: wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
@row_add n=86 row=contract/channel-wait/after area=established-decisions rec=23
boundary: channel wait
item: after
promise: after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
@row_add n=87 row=contract/channel-wait/timeout-ms area=established-decisions rec=23
boundary: channel wait
item: timeout_ms
promise: On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
@row_add n=88 row=contract/channel-wait/pending-dialog area=established-decisions rec=23
boundary: channel wait
item: pending dialog
promise: A dialog pending when wait is called without after, or with after at or before its event line, is returned at once; with after past that line wait blocks for the next event.
@row_add n=89 row=contract/channel-results/cursor area=established-decisions rec=23
boundary: channel results
item: cursor
promise: Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
@row_add n=90 row=contract/dialog-events/dialog-id area=established-decisions rec=23
boundary: dialog events
item: dialog_id
promise: Each dialog event carries a wrapper-assigned integer dialog_id, monotonic per instance and never repeated across wrapper restarts; answer names its target by it.
@row_add n=91 row=decision/one-pending-dialog area=established-decisions rec=23
name: At most one pending dialog per instance
rule: At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
@row_add n=92 row=contract/channel-answer/unknown-dialog area=established-decisions rec=23
boundary: channel answer
item: unknown-dialog
promise: An answer for a dialog that was not held pending is refused not-delivered with detail unknown-dialog.
@row_add n=93 row=contract/channel-last/last-assistant-message area=established-decisions rec=23
boundary: channel last
item: last_assistant_message
promise: last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
@row_add n=94 row=decision/hashed-endpoint-names area=established-decisions rec=23
name: Hashed endpoint names
rule: Endpoint names are a short hash of the instance, never the raw instance name and path.
@row_add n=95 row=decision/socket-dir-check-at-channel-callers area=established-decisions rec=24
name: Socket-directory check at the channel's callers
rule: The socket-directory check stands at the channel seam's callers, not inside the channel client's connect.
why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
@row_add n=96 row=decision/verify-server-before-first-frame area=established-decisions rec=25,28
name: Server verified before the first frame
rule: Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison.
why: A squatted or rogue server is never sent a frame.
@row_add n=97 row=decision/peer-euid-admission area=established-decisions rec=26,27
name: Peer admitted only on a matching euid
rule: A channel peer is admitted only when its euid is read and equals the own user's; an unreadable euid is refused, and the wrapper drops a refused stream before any frame, writing nothing, with no env, flag or feature seam.
@row_none n=3 rec=29 type=4
relied: The socket-directory check passes an absent path, refuses a symlink, a non-directory or an unreadable path, and never chmods, so a refused directory stays as found.
@row_add n=98 row=decision/snapshots-read-through-replay area=established-decisions rec=30,31
name: Snapshots read through the replay
rule: Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
