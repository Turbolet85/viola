# architecture · the rows that bind 2026-11-24-switching-a-session-s-model-and-effort — picked 402 of 1040 (275 by every draw) · rule any · pin 4716

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker; automation is refused with a typed reason instead of competing with the human.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.

## stack-and-technologies · ## Stack and Technologies
decision/append-one-write-per-line · Log appends one write per line — Log appends use std append mode with one write per line. · why: crash-safe multi-process writes with no C code
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/clap-cli-parser · clap as CLI parser, without colour — clap (derive) parses the CLI with its default features minus color. · why: help and usage output stays plain and no colour or terminal crate is in the graph
decision/clients-read-server-pid-euid-from-peer-creds · Clients read server pid and euid from peer credentials — Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame. · why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/frame-params-redacted · Frame params never rendered in Debug — A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
decision/no-database · No database: ndjson logs and JSON snapshots — State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).
decision/preserve-key-order · Printed documents keep declared key order — serde_json's preserve_order feature is on, so a printed document keeps its declared key order.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/server-start-time-read-through-sysinfo · Server start time is read from the OS through sysinfo — The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame. · why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/wrapper-ipc-local-socket · interprocess local sockets for wrapper IPC — Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/no-message-broker · No message broker — Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.

## established-decisions · ## Established Decisions
decision/byte-source-wheel-claim · Byte-source wheel claim — Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`. · why: `viola run` owns the real stdin, so it knows byte provenance exactly.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/dialog-hold · Dialog hold per instance — A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/exclusive-endpoint-bind · Exclusive endpoint bind — A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name. · why: So two concurrent starts cannot both win.
decision/frame-client-verifies-server · Frame clients verify the serving wrapper — Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
decision/frame-version-safety-net · Per-frame version as second safety net — Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/hand-rolled-json-rpc-over-ndjson · JSON-RPC 2.0 over ndjson, hand-rolled with serde — The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one. · why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
decision/hashed-endpoint-names · Hashed endpoint names — Endpoint names are a short hash of the instance, never the raw instance name and path.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/launch-pair-handed-not-verified · Launch pair handed, never verified — Every start, run, revive and adopt, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on. · why: Whether a session started or resumed with the pair runs on it is unmeasured on the real CLI.
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/local-command-post-conditions · Local commands listed with delivery post-conditions — The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands. · why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/no-live-endpoint-dead-or-unwrapped · No live endpoint reads instance-dead or unwrapped-name — Without a live endpoint, whether the wrapper is stale, gone or replayed, a client reads an existing instance directory as instance-dead and anything else as an unwrapped name. · why: The CLI frame verbs and the MCP frame tools take their exit 21 detail and hint from it, and the cross exit-cause tests pin both words.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run`, `viola revive`, `viola adopt` or `viola open` whose `ViolaName` is `live` or `stale` refuses to start.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.
decision/one-pending-dialog · At most one pending dialog per instance — At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated, and keeps the parent, the role and the model and effort pair its real snapshot records.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/revive-resumes-in-place · Revive resumes a gone instance in place — viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and under the hidden --origin the recorded workspace, and its first record is wheel with holder driver and cause start, as every start's. · why: Ratified: a revived start keeps the wheel at driver.
decision/run-refuses-held-name · A start refuses a held name — viola run, viola revive and viola adopt refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/self-describing-frames-no-handshake · Self-describing frames, no handshake — Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. · why: Mixed binary versions are a measured fact, so every frame self-describes.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/socket-dir-check-at-channel-callers · Socket-directory check at the channel's callers — The socket-directory check stands at the channel seam's callers, not inside the channel client's connect. · why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
decision/socket-dir-check-verdicts · Socket-directory check verdicts, never chmods — The socket-directory check passes an absent path and never chmods, so a refused directory stays as found. It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path. · why: A start makes an absent directory itself, and the negative tests for start, frame verbs and hooks expect a loosened or linked directory left as found.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/torn-last-line-healed · Torn log line healed by the next append — Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/verify-server-before-first-frame · Server verified before the first frame — Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison. · why: A squatted or rogue server is never sent a frame.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
decision/wheel-in-wrapper-memory · Wheel decided in wrapper memory — The wheel is decided in the wrapper's memory and mirrored to a snapshot.
decision/wheel-returns-only-through-release · Wheel returns only through CLI release — CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/session-end · Hook tiers · SessionEnd — Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/viola-revive-json-error-codes/already-live · viola revive --json error codes · already-live — The name is held by a live instance.
contract/viola-revive-json-error-codes/internal-error · viola revive --json error codes · internal-error — A claude not on PATH or an anyhow error of the start.
contract/viola-revive-json-error-codes/session-live · viola revive --json error codes · session-live — The logged session is held by a live claude.
contract/viola-revive-json-error-codes/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The name is held by a stale instance.
decision/adopt-refuses-live-session · Adopt refuses a session a live claude holds — viola adopt resumes under viola, with an operator-given name, a plain claude session named by its session id, and refuses with session-live, exit 1, when a claude agents --json row carries that id; a failed read passes. · provisional
decision/append-heals-torn-line · The next append heals a torn line — Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/explicit-wheel-release · Explicit wheel release — The wheel returns from the human to the driver only through `viola release`.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/human-interrupt-moves-wheel · Human interrupt moves the wheel — A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state. · why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
decision/human-prompt-confirms-wheel-take · Human prompt confirms the wheel take — An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it. · except: On a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is a fired wake: relabelled harness, wheel kept.
decision/human-wheel-renders-dialogs · Human-held wheel renders dialogs at once — While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once. · why: A dialog held by the hook is never hidden from the human for the rest of the hold.
decision/interprocess-local-socket-channel · interprocess local sockets behind the channel seam — interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/interprocess-local-socket-ipc · interprocess local sockets behind the channel seam — IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
decision/ipc-interprocess-local-socket · interprocess local_socket for IPC — Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/local-commands-by-ledger · Local commands classified by the ledger — A send is a local command only when its typed text is exactly a name on the ledger's local-command list, or that name, one space and a non-empty rest not starting with a space, never by a leading slash; any other slash text, skills included, is an ordinary send. · why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
decision/paste-cut-mark · Cut mark refuses sends until the human acts — A cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor; a human key read, a prompt-submitted of any origin, a session-start or a session-end clears it; a release with no key or a turn-ended leaves it.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/private-mode-before-write · File mode set before any byte is written — A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
decision/revive-reads-snapshot-once · Revive reads the snapshot once — A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/separate-lock-files · Locks on separate .lock files — State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked. · why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
decision/socket-dir-refused-covers-create-and-check · Socket-dir create failure and refusal read the same — A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
decision/spine-hook-loses-event-not-turn · Spine hook loses the event, never delays the turn — A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost. · why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/wheel-byte-source-claim · Byte-source claim of the wheel — Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts. · why: `viola run` owns the real stdin, so it knows byte provenance.
decision/wheel-classifier-only-observes · Wheel classifier only observes — The stdin classifier returns every byte whole and unchanged, and a read holding an editing key moves the wheel before it returns.
decision/wheel-starts-with-driver · The wheel starts with the driver — The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key, a key the human types into the folder-trust dialog before the session starts included.
contract/child-exit-record/exit-source · Child exit record · exit_source — The child's exit is recorded with exit_source handle-wait or kill-fallback.
contract/event-line/dialog-expired · The event line · dialog-expired — Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
contract/event-line/link · The event line · link — The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
contract/event-line/unlink · The event line · unlink — `viola unlink` appends an `unlink` line with `source: wrapper` and `{driver, driven}`.
contract/events-ndjson/send-confirmed-order · events.ndjson · send-confirmed order — The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts; a send confirmed by a stdout echo has no such line and its submitted_at is the instant the echo was read.
contract/human-hint-lines/input-not-ready · Human hint lines · input-not-ready — Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a standing modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
contract/human-hint-lines/paste-cut · Human hint lines · paste-cut — Reads "<name> did not take the whole text in time; a human must look at the session before the next send".
contract/send-refusal/manual-pause · Send refusal details · manual-pause — After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait → `human-typing` and `turn-running` again → `paste-cut` of a standing cut mark → `paste-cut` of a paste cut at its bound.
contract/viola-revive-json-error-codes/cwd-missing · viola revive --json error codes · cwd-missing — The recorded directory is missing.
contract/viola-revive-json-error-codes/no-session · viola revive --json error codes · no-session — No logged session to revive.
contract/viola-revive-json-error-codes/pinned-hash-mismatch · viola revive --json error codes · pinned-hash-mismatch — The pinned copy no longer matches its hash.
contract/viola-revive-json-error-codes/socket-dir-refused · viola revive --json error codes · socket-dir-refused — The socket directory was refused or could not be created.
contract/viola-revive-json-error-codes/squatted-name · viola revive --json error codes · squatted-name — The endpoint name is already taken at bind.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — A revive whose recorded cwd names no existing directory is refused cwd-missing.

## conventions · ## Conventions
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
decision/connect-failure-dead-closed-stream-lost · Failed connect is instance-dead, lost stream endpoint-lost — A failed connect reads as the instance-dead cause and a closed or broken stream as the endpoint-lost cause, on the CLI and in the MCP tools alike. · why: The exit 21 detail words, the process-exit during field and the MCP instance-unreachable detail all rest on the split.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/event-kinds-agent-neutral · Event kinds added only in viola-core — A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/retired-exit-codes-never-reused · Retired exit codes never reused — A retired CLI exit code is never reused or renumbered: its number stays unassigned.
decision/silent-roles-at-internal-error-catch · Run, revive, adopt, mcp and hook silent at the catch site — Run, revive's start, adopt's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-fault-codes/-32602 · channel protocol fault codes · -32602 — Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
contract/channel-params/sender · channel params · sender — Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
contract/channel-response/error · channel response · error — Reserved for protocol faults: `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}`.
contract/channel-response/ok · channel response · ok — Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-exit-21/socket-dir-refused · CLI exit 21 · socket-dir-refused — On send, wait, last, answer, pause, release, link and unlink a refused socket directory exits 21 and writes one process-exit line at WARN with exit_code 21, detail socket-dir-refused and during connect, and no channel-request.
contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause, release or focus exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
contract/cli-exit-codes/10 · CLI exit codes · 10 — The `human-typing` refusal.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Protocol fault from the wrapper, a -32602 from link or unlink included; the link self-pair is refused by the wrapper only, never client-side.
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/cli-verbs/revive · CLI verbs · revive — `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.
contract/client-server-verify/check-order · Client server verification · check order — The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
contract/client-server-verify/server-verify-failed · Client server verification · server-verify-failed — A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/exit-21-human-hints/placement · Exit 21 human hints · placement — Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
contract/hook-to-kind-map/sessionend · Hook to kind map · SessionEnd — SessionEnd maps to `session-end`.
contract/hook-to-kind-map/sessionstart · Hook to kind map · SessionStart — SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
contract/mcp-tool-results/instance-unreachable · MCP tool results · instance-unreachable — An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused, or call-deadline, MCP's alone.
contract/model-word/format · Model word · format — A model word is 1 to 64 bytes, a lower-case letter first, then lower-case letters, digits, dots or hyphens.
contract/normalised-event-kinds/launch · normalised event kinds · launch — `launch` is a normalised event kind.
contract/normalised-event-kinds/session-end · normalised event kinds · session-end — `session-end` is a normalised event kind.
contract/normalised-event-kinds/session-start · normalised event kinds · session-start — `session-start` is a normalised event kind.
contract/normalised-event-kinds/wheel · normalised event kinds · wheel — `wheel` is a normalised event kind.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/refusal-order/first-only · Refusal order · several refusals — When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/snapshot/writer · snapshot · writer — Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
contract/unreachable-cause/cause-set · Unreachable cause · cause set — UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no RefusalReason; the six exit 21's, written by both clients, call-deadline by the MCP frame tools alone.
decision/endpoint-name-stable-hash · Endpoint name from a stable hand-written hash — The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden. · why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
decision/four-interfaces-one-binary · Four interfaces from one binary — The binary exposes four interfaces: CLI verbs, MCP tools over stdio, the hook stdin/stdout contract, and the wrapper channel; there is no REST write surface, GraphQL, gRPC or tRPC.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
contract/channel-fault-codes/-32601 · channel protocol fault codes · -32601 — Unknown method.
contract/channel-fault-codes/-32603 · channel protocol fault codes · -32603 — Internal error.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/cli-exit-codes/14 · CLI exit codes · 14 — The `unknown` refusal.
contract/cli-verbs/up · CLI verbs · up — `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken; CLI-only, no MCP tool and no channel method.
contract/normalised-event-kinds/activity · normalised event kinds · activity — `activity` is a normalised event kind.
contract/normalised-event-kinds/dialog-expired · normalised event kinds · dialog-expired — `dialog-expired` is a normalised event kind.
contract/normalised-event-kinds/lineage · normalised event kinds · lineage — `lineage` is a normalised event kind.
contract/normalised-event-kinds/link · normalised event kinds · link — `link` is a normalised event kind.
contract/normalised-event-kinds/permission · normalised event kinds · permission — `permission` is a normalised event kind.
contract/normalised-event-kinds/plan · normalised event kinds · plan — `plan` is a normalised event kind.
contract/normalised-event-kinds/prompt-submitted · normalised event kinds · prompt-submitted — `prompt-submitted` is a normalised event kind.
contract/normalised-event-kinds/question · normalised event kinds · question — `question` is a normalised event kind.
contract/normalised-event-kinds/send-confirmed · normalised event kinds · send-confirmed — `send-confirmed` is a normalised event kind.
contract/normalised-event-kinds/send-issued · normalised event kinds · send-issued — `send-issued` is a normalised event kind.
contract/normalised-event-kinds/send-refused · normalised event kinds · send-refused — `send-refused` is a normalised event kind.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/normalised-event-kinds/unlink · normalised event kinds · unlink — `unlink` is a normalised event kind.
contract/normalised-event-kinds/window · normalised event kinds · window — `window` is a normalised event kind, appended by the wrapper, log-only, that never wakes `wait`.
contract/ok-payload/unconfirmable · ok payload · unconfirmable — unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured, or is a stdout echo with no readable session transcript, or for any listed local command on an unverified CLI.
contract/peer-reply/detail · A peer's reply · detail — A string detail that is no code word is read as null; a non-string detail makes the reply no known reply.
contract/peer-reply/refusal · A peer's reply · refusal — A refusal that is no code word is read as unknown.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
contract/refusal-reason/unknown · RefusalReason · `unknown` refusal reason — The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
contract/refusal-reason/unknown-catch-all · RefusalReason · unknown catch-all — The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.

## standard-contracts · ## Standard Contracts
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal.
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
decision/server-identity-check-before-frame · Server identity checked before the first frame — Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage, launch and window from the log; a field no line gave is absent, never a default.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
decision/wrapper-channel-json-rpc · JSON-RPC over ndjson for the wrapper channel — The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/channel-32602-cause/invalid-params · channel -32602 causes · invalid-params — One of the three causes a -32602 fault reads.
contract/channel-32602-cause/unsupported-version · channel -32602 causes · unsupported-version — One of the three causes a -32602 fault reads.
contract/channel-methods/v-and-sender · channel methods · v and sender — Every channel method's `params` carries `v` and `sender`.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`; a finished or paused turn is for the driver to tell, and a parked `wait` never polls the file.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records, an activity record and a harness-origin prompt-submitted never wake, also after a task-free turn end.
contract/cli-exit-21-document/endpoint-lost · exit-21 CLI document · endpoint-lost — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/instance-dead · exit-21 CLI document · instance-dead — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/server-verify-failed · exit-21 CLI document · server-verify-failed — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/socket-dir-refused · exit-21 CLI document · socket-dir-refused — A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
contract/cli-exit-21-document/strict-modes-failed · exit-21 CLI document · strict-modes-failed — A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
contract/cli-exit-21-document/unwrapped-name · exit-21 CLI document · unwrapped-name — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document.
contract/cli-json-output/instance-unreachable · CLI `--json` output · `instance-unreachable` — `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/cli-json-output/wrapper-fault · CLI `--json` output · `wrapper-fault` — `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null.
contract/event-data/launch · event data · `launch` — `{model, effort}`, both strings, with `source: wrapper`.
contract/event-data/session-end · event data · `session-end` — `{}`.
contract/event-data/session-start · event data per kind · session-start — `session-start` data is `{cause, agent_session_id}`.
contract/event-data/wheel · event data · `wheel` — `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/fault-document/invalid-params · The fault document on --json and MCP · other -32602 — Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null.
contract/fault-document/known-code-message · The fault document on --json and MCP · message of a known fault code — For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault.
contract/fault-document/unsupported-version · The fault document on --json and MCP · -32602 unsupported version — A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values.
contract/fault-document/wrapper-fault · The fault document on --json and MCP · wrapper fault — A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word.
contract/human-error-line/wrapper-fault · The human error line · wrapper fault — A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?, lineage?, launch?, window?}`.
contract/instance-snapshot/launch · instance snapshot · launch — `launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
contract/instance-snapshot/pending-dialog · instance snapshot · pending_dialog — `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/window · instance snapshot · window — `window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a later `launch` line clearing it.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/list-item/effort · list payload item · effort — Present only when the recorded effort re-parses as a code word at the board; follows model.
contract/list-item/model · list payload item · model — Present only when the recorded model re-parses as a model word at the board; follows role.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window?, origin?}`.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`.
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`, `window`; the replay writes no file.
contract/wrapper-channel-frames/conn · wrapper channel frames · conn — A client may add `conn` to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
contract/wrapper-channel-frames/envelope · wrapper channel frames · envelope — Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
contract/wrapper-channel-frames/id-less-frames · wrapper channel frames · id-less frames — Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
contract/wrapper-channel-frames/max-frame · wrapper channel frames · MAX_FRAME — A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.
contract/wrapper-channel-frames/ok-result · wrapper channel frames · ok result — A success is answered as `result` holding `{"ok": <payload>}`.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
contract/wrapper-event-appends/launch · wrapper event appends · `launch` — The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`; it is log-only and never wakes `wait`.
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/revive-refused-without-cwd · Revive over a replayed snapshot refused — A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
contract/channel-methods/from · channel methods · from — `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
contract/fault-document/release-from-driver · The fault document on --json and MCP · -32602 release-from-driver — A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its readers are `viola revive`, which uses it as the revived child's spawn directory and its trust record's folder, and `viola up`, opening each terminal there.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/list-item/wheel · list payload item · wheel — human or driver; absent when the session is unwrapped.
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal.
contract/session-liveness/heartbeat · session liveness · heartbeat — The wrapper touches `<instance dir>/heartbeat` every 1 s.
contract/wrapper-event-appends/wheel · wrapper event appends · `wheel` — The wrapper appends `wheel` with `source: wrapper` once at start and on every change.

## occupied-resources · ## Occupied Resources
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/connect-to-recorded-endpoint · Clients connect to the recorded endpoint — The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path. · why: Processes spawned inside the session may run with a different environment from the wrapper.
decision/default-launch-pair · Compiled default launch pair — The compiled default model and effort pair is one named constant, DEFAULT_PAIR, in the Claude agent crate, and the effort words are the five of claude --help; no environment variable sets the pair.
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/recorded-pair-on-child-argv · Recorded launch pair on the child's command line — A model and effort pair read from the instance's record may be handed on the revived claude child's command line, each word re-parsed to its closed shape first. · why: A ratified crossing, beside the logged session id on the child's command line.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/revive-pair-from-record · Revive hands the recorded pair — Each half of the model and effort pair a revive hands is its --model or --effort override, else the recorded half while it still parses, else the start's default. · provisional
decision/same-user-peers-only · Same-user peers only — Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/socket-dir-owner-only · Socket in a per-user owner-only directory — The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
decision/socket-in-per-user-dir · Endpoint socket in a per-user 0700 directory — The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
contract/channel-response-detail/invalid-params · channel-response detail, a closed enum · invalid-params — Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
contract/channel-response-detail/unsupported-version · channel-response detail, a closed enum · unsupported-version — Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/launch-pair · flags passed to the child · launch pair — `--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived or adopted start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it; an adopted start adds --resume with the operator-typed id, in the session-id shape.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
contract/frame-verb-exit-codes/21 · send, wait, last, answer, pause and release exit codes · 21 — The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
contract/frame-verb-exit-detail/endpoint-lost · CLI frame verb process-exit detail · endpoint-lost — Exit 21 with during call, for a wrapper lost with the call open.
contract/frame-verb-exit-detail/instance-dead · CLI frame verb process-exit detail · instance-dead — Exit 21 with during connect, at WARN.
contract/frame-verb-exit-detail/server-verify-failed · CLI frame verb process-exit detail · server-verify-failed — Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run's, revive's and adopt's start write it with exit_code 1.
contract/frame-verb-exit-detail/unwrapped-name · CLI frame verb process-exit detail · unwrapped-name — Exit 21 with during connect.
contract/frame-verb-exit-detail/wrapper-fault · CLI frame verb process-exit detail · wrapper-fault — Exit 20.
contract/instance-unreachable-detail/server-verify-failed · the instance-unreachable --json document · server-verify-failed — The detail when the server runs under another euid.
contract/instance-unreachable-detail/socket-dir-refused · the instance-unreachable --json document · socket-dir-refused — The detail when the recorded socket folder was refused, past the instance check.
contract/instance-unreachable-detail/strict-modes-failed · the instance-unreachable --json document · strict-modes-failed — The detail when the instance check refused the instance.
contract/viola-cli-arguments/revive · viola CLI arguments · revive — viola revive takes <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>], or <name> --list, each with [--json].
contract/viola-cli-arguments/revive-origin · viola CLI arguments · revive --origin — viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
contract/viola-cli-arguments/run-effort · viola CLI arguments · run --effort — viola run --effort <EFFORT> names the effort handed to the child, one of the five effort words; any other value is a usage error, exit 2; with none given the effort is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-model · viola CLI arguments · run --model — viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: mcp's start and plugin install keep the write rule on the home.
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types.
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace socket names are never used. · why: They carry no filesystem permissions and any local user could connect.
decision/no-after-every-test-error-body-helper · No after-every-test error-body helper — Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/stale-socket-file-left · Stale socket file left in place — A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: A choice: a harmless file in a 0700 directory.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
contract/child-argv/fork-session · flags passed to the child · `--fork-session` — A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/claude-child-argv/fork-session · the claude child's argv · --fork-session — --fork-session follows --resume when --fork is given; the words after -- come last.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
contract/diag-line/state-recovered · diagnostics line · state-recovered — A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm and viola adopt's start included.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/process-exit-detail/not-live · process-exit detail enum · not-live — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json.
contract/viola-link-exit-codes/21 · viola link and unlink exit codes · 21 — The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.
contract/viola-revive-exit-codes/0 · viola revive exit codes · 0 — The revived child's wrapper ended as viola run's does.
contract/viola-revive-exit-codes/2 · viola revive exit codes · 2 — A clap usage error: a malformed --id, --model or --effort, or a model or effort flag in the words after the dashes, included.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/hashed-short-endpoint-names · Hashed short endpoint names — Endpoint names are hashed and short.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
contract/channel-errors/32602-newer-request · Channel errors · -32602 for a newer request — A request newer than the wrapper supports gets a -32602 error naming the wrapper's version.
contract/channel-params/sender-version · Channel params · sender version — Every channel params object carries the sender's CARGO_PKG_VERSION.
contract/channel-params/v · Channel params · v — Every channel params object carries the format version v.
contract/exit-21-human-hints/socket-dir · Exit 21 human hints · socket-dir — The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list and viola revive do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/same-user-endpoint-trust · Local endpoint trusts the same OS user only — v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.

## project-intent · ## Project Intent
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/clap-for-cli · clap parses the CLI — clap is what parses the CLI.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/interprocess-local-sockets · interprocess local sockets for IPC — IPC uses interprocess local sockets, one endpoint per viola run.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.
contract/channel/framing · Channel · framing — The channel speaks JSON-RPC 2.0 over ndjson.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.

## build-system · key of ## Infrastructure Patterns
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
contract/cli-json/internal-error · CLI --json output · internal-error — Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/core-depends-on-no-viola-crate · viola-core depends on no viola crate — viola-core depends on no other viola crate.

## project-directory-structure · key of ## Infrastructure Patterns
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at.
decision/channel-json-rpc-local-sockets · JSON-RPC over local sockets for the channel — The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets.

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
name/command/adopt — adopt
name/command/answer — answer
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/up — up
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
name/other/foot-spawns-ndjson — foot-spawns.ndjson
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
