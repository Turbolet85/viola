# architecture · the rows that bind 2026-11-28-the-fleet-on-the-founder-s-host — picked 491 of 1050 (297 by every draw) · rule any · pin 5476

## design-philosophy · ## Design Philosophy
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker; automation is refused with a typed reason instead of competing with the human.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.

## stack-and-technologies · ## Stack and Technologies
decision/content-hash-sha256 · Pure-Rust SHA-256 content hash — The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/server-start-time-read-through-sysinfo · Server start time is read from the OS through sysinfo — The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame. · why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
decision/unicode-width-column-widths · unicode-width for terminal column widths — unicode-width is a direct, exactly pinned dependency of the root bin, used for terminal column widths of the console's editing line. · why: already in the lock through vt100, so no crate is added · provisional
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/append-one-write-per-line · Log appends one write per line — Log appends use std append mode with one write per line. · why: crash-safe multi-process writes with no C code
decision/clients-read-server-pid-euid-from-peer-creds · Clients read server pid and euid from peer credentials — Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame. · why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
decision/pinned-copy-placed-once · Pinned copy placed once, never replaced — The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.

## established-decisions · ## Established Decisions
decision/adopt-refuses-live-session · Adopt refuses a session a live claude holds — viola adopt resumes under viola, with an operator-given name, a plain claude session named by its session id, and refuses with session-live, exit 1, when a claude agents --json row carries that id; a failed read passes.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals. · except: viola console, the terminal fleet console's own surface, takes no --json and draws its own screen.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals. · except: viola console, the terminal fleet console's own surface, takes no --json and draws its own screen.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/every-send-confirmed · Every send is confirmed — A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted. · why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate. · except: A ledger-listed local command is confirmed by its own measured post-condition, such as a stdout echo in the session transcript, or answered ok unconfirmable where none can be read.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/focus-has-no-driver-form · Focus has no driver-facing form — viola focus is offered on the CLI only: no MCP tool raises a window, and adding one is a further boundary widening of its own.
decision/human-wheel-renders-dialogs · Human-held wheel renders dialogs at once — While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once. · why: A dialog held by the hook is never hidden from the human for the rest of the hold.
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/interprocess-local-socket-channel · interprocess local sockets behind the channel seam — interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/interprocess-local-socket-ipc · interprocess local sockets behind the channel seam — IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
decision/ipc-interprocess-local-socket · interprocess local_socket for IPC — Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/launch-pair-handed-not-verified · Launch pair handed, never verified — Every start, run, revive and adopt, and every switch, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on. · why: Whether a session started or resumed with the pair runs on it is unmeasured on the real CLI.
decision/lineage-label-only · Lineage is a label only — Lineage is a label only: no wheel, link, send or answer path reads the declared parent or role; viola up reads the parent only to order its re-opens. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run`, `viola revive`, `viola adopt` or `viola open` whose `ViolaName` is `live` or `stale` refuses to start.
decision/one-pending-dialog · At most one pending dialog per instance — At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated, and keeps the parent, the role and the model and effort pair its real snapshot records.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/revive-reads-snapshot-once · Revive reads the snapshot once — A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/revive-resumes-in-place · Revive resumes a gone instance in place — viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and under the hidden --origin the recorded workspace, and its first record is wheel with holder driver and cause start, as every start's. · why: Ratified: a revived start keeps the wheel at driver.
decision/run-refuses-held-name · A start refuses a held name — viola run, viola revive and viola adopt refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/self-describing-frames-no-handshake · Self-describing frames, no handshake — Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. · why: Mixed binary versions are a measured fact, so every frame self-describes.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs, while another `send` is in flight, or while a switch stands, from its request until the new child's `session-start`, is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/session-end-budget-never-raised · SessionEnd budget never raised — The 1.5 s SessionEnd budget is never raised.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/start-verifies-new-cli-version · Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record and the claude child, and gates on the final verdict; the key is off by default.
decision/submitted-form-prompt-match · Prompt match in the submitted form after agent normalisation — The typed text matches prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and frame and reverses tag escaping, when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF. · why: The CLI trims only a trailing run at submit: it keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run of spaces and an inner LF.
decision/terminal-fleet-console · Terminal fleet console as the one view — The one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland.
decision/up-parents-first · up re-opens parents before children — viola up re-opens its candidates one at a time, parents before children; a candidate whose parent names no other candidate is a root, and a parent cycle is taken once.
decision/up-selection-rule · up re-opens only instances gone before the boot — viola up skips a live or stale instance and takes a gone one only when its heartbeat is older than the host boot time and, unless it was named, its events log has no session-end after its last session-start; an unnamed instance ended on purpose stays down.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
decision/wait-blocks-nothing-polls · wait blocks, nothing polls — wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
decision/window-features-target-hyprland · Window features target Hyprland — The window features target the Hyprland compositor.
contract/channel-last/last-assistant-message · channel last · last_assistant_message — last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
contract/channel-results/cursor · channel results · cursor — Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
contract/channel-wait/after · channel wait · after — after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
contract/channel-wait/driver-relevant-events · channel wait · driver-relevant events — wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
contract/channel-wait/pending-dialog · channel wait · pending dialog — A dialog pending when wait is called without after, or with after at or before its event line, is returned at once; with after past that line wait blocks for the next event.
contract/channel-wait/timeout-ms · channel wait · timeout_ms — On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
contract/events-ndjson/send-confirmed-order · events.ndjson · send-confirmed order — The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts; a send confirmed by a stdout echo has no such line and its submitted_at is the instant the echo was read.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/session-end · Hook tiers · SessionEnd — Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/human-hint-lines/input-not-ready · Human hint lines · input-not-ready — Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a standing modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
contract/human-hint-lines/paste-cut · Human hint lines · paste-cut — Reads "<name> did not take the whole text in time; a human must look at the session before the next send".
contract/not-delivered-details/no-prompt-submitted · not-delivered refusal details · no-prompt-submitted — A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait → `human-typing` and `turn-running` again → `paste-cut` of a standing cut mark → `paste-cut` of a paste cut at its bound.
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a standing modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/send-results/unconfirmable · send results · unconfirmable — A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
contract/viola-revive-json-error-codes/already-live · viola revive --json error codes · already-live — The name is held by a live instance.
contract/viola-revive-json-error-codes/cwd-missing · viola revive --json error codes · cwd-missing — The recorded directory is missing.
contract/viola-revive-json-error-codes/internal-error · viola revive --json error codes · internal-error — A claude not on PATH or an anyhow error of the start.
contract/viola-revive-json-error-codes/no-session · viola revive --json error codes · no-session — No logged session to revive.
contract/viola-revive-json-error-codes/pinned-hash-mismatch · viola revive --json error codes · pinned-hash-mismatch — The pinned copy no longer matches its hash.
contract/viola-revive-json-error-codes/session-live · viola revive --json error codes · session-live — The logged session is held by a live claude.
contract/viola-revive-json-error-codes/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The name is held by a stale instance.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — A revive whose recorded cwd names no existing directory is refused cwd-missing.
decision/atomic-bracketed-send · Atomic bracketed-paste send — A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after. · except: A paste not finished within the compiled paste bound is cut: the writer stops after its in-flight chunk and writes the closing paste marker with no CR.
decision/byte-source-wheel-claim · Byte-source wheel claim — Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`. · why: `viola run` owns the real stdin, so it knows byte provenance exactly.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/confirm-window-compiled · Confirmation window compiled — The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp. · except: A /compact send waiting for its post-condition has its own 40 s window.
decision/exclusive-endpoint-bind · Exclusive endpoint bind — A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name. · why: So two concurrent starts cannot both win.
decision/frame-client-verifies-server · Frame clients verify the serving wrapper — Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
decision/frame-version-safety-net · Per-frame version as second safety net — Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/human-controls-cli-only · Human wheel and link controls are CLI-only — `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface. · why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
decision/human-interrupt-moves-wheel · Human interrupt moves the wheel — A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state. · why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/no-live-endpoint-dead-or-unwrapped · No live endpoint reads instance-dead or unwrapped-name — Without a live endpoint, whether the wrapper is stale, gone or replayed, a client reads an existing instance directory as instance-dead and anything else as an unwrapped name. · why: The CLI frame verbs and the MCP frame tools take their exit 21 detail and hint from it, and the cross exit-cause tests pin both words.
decision/pause-link-unlink-cli-only · pause, link and unlink are CLI-only — `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools. · why: They are human controls over the wheel and the link topology.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/pinned-copy-never-replaced · Pinned copy never replaced — The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
decision/prompt-normalise-order · Classify before unwrapping and un-escaping — Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /. · why: So a typed pair is never unwrapped and delivery matching compares plain text.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or --file — Prompt text comes from stdin or `--file`, never from a leading-slash argument.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/release-cli-only · release is CLI-only — `release` is never an MCP tool; only the CLI offers it. · why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/socket-dir-check-at-channel-callers · Socket-directory check at the channel's callers — The socket-directory check stands at the channel seam's callers, not inside the channel client's connect. · why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
decision/socket-dir-refused-covers-create-and-check · Socket-dir create failure and refusal read the same — A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
decision/switch-cli-only · Switch is CLI-only — viola switch is offered on the CLI only: no MCP tool switches a session's model or effort.
decision/switch-in-place · Model and effort switched in place — A running wrapped session switches its model and effort in place: the wrapper ends only the child its own PTY handle holds, reaps it within RESPAWN_REAP_BOUND, appends `launch`, and resumes the same session id on the new pair under the same wrapper.
decision/switch-writes-no-user-settings · Switch writes nothing into the user's settings — A switch never writes to the user's own config or settings and passes no --settings; the switched pair lives only in viola's own record, which a later revive replays.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/torn-last-line-healed · Torn log line healed by the next append — Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/verify-server-before-first-frame · Server verified before the first frame — Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison. · why: A squatted or rogue server is never sent a frame.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks. · except: The test-only live rig in viola-e2e also uses vt100 as the screen model it reads its child's trust dialog, selector and input box from.
decision/wheel-in-wrapper-memory · Wheel decided in wrapper memory — The wheel is decided in the wrapper's memory and mirrored to a snapshot.
contract/channel-release/release-from-driver · Wrapper channel release method · release-from-driver — A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
contract/dialog-events/dialog-id · dialog events · dialog_id — Each dialog event carries a wrapper-assigned integer dialog_id, monotonic per instance and never repeated across wrapper restarts; answer names its target by it.
contract/event-line/dialog-expired · The event line · dialog-expired — Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
contract/not-delivered-details/input-not-ready · not-delivered refusal details · input-not-ready — A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
contract/viola-revive-json-error-codes/socket-dir-refused · viola revive --json error codes · socket-dir-refused — The socket directory was refused or could not be created.
contract/viola-revive-json-error-codes/squatted-name · viola revive --json error codes · squatted-name — The endpoint name is already taken at bind.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.

## conventions · ## Conventions
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/silent-roles-at-internal-error-catch · Run, revive, adopt, mcp and hook silent at the catch site — Run, revive's start, adopt's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-response/ok · channel response · ok — Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-exit-21/socket-dir-refused · CLI exit 21 · socket-dir-refused — On send, wait, last, answer, pause, release, link and unlink a refused socket directory exits 21 and writes one process-exit line at WARN with exit_code 21, detail socket-dir-refused and during connect, and no channel-request.
contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause, release or focus exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error; the start, revive-preflight, open, focus, up, verify and console refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
contract/cli-exit-codes/10 · CLI exit codes · 10 — The `human-typing` refusal.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/cli-verbs/focus · CLI verbs · focus — `viola focus <name> [--json]` takes one ViolaName and `--json` only.
contract/cli-verbs/open · CLI verbs · open — `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.
contract/cli-verbs/revive · CLI verbs · revive — `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.
contract/cli-verbs/send · CLI verbs · send — `viola send <target>` takes its text from stdin or `--file`.
contract/cli-verbs/up · CLI verbs · up — `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken; CLI-only, no MCP tool and no channel method.
contract/cli-verbs/wait · CLI verbs · wait — `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
contract/hook-to-kind-map/sessionend · Hook to kind map · SessionEnd — SessionEnd maps to `session-end`.
contract/hook-to-kind-map/sessionstart · Hook to kind map · SessionStart — SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
contract/hook-to-kind-map/stopfailure · Hook to kind map · StopFailure — StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
contract/model-word/format · Model word · format — A model word is 1 to 64 bytes, a lower-case letter first, then lower-case letters, digits, dots or hyphens.
contract/normalised-event-kinds/launch · normalised event kinds · launch — `launch` is a normalised event kind.
contract/normalised-event-kinds/lineage · normalised event kinds · lineage — `lineage` is a normalised event kind.
contract/normalised-event-kinds/prompt-submitted · normalised event kinds · prompt-submitted — `prompt-submitted` is a normalised event kind.
contract/normalised-event-kinds/send-confirmed · normalised event kinds · send-confirmed — `send-confirmed` is a normalised event kind.
contract/normalised-event-kinds/send-issued · normalised event kinds · send-issued — `send-issued` is a normalised event kind.
contract/normalised-event-kinds/session-end · normalised event kinds · session-end — `session-end` is a normalised event kind.
contract/normalised-event-kinds/session-start · normalised event kinds · session-start — `session-start` is a normalised event kind.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/normalised-event-kinds/wheel · normalised event kinds · wheel — `wheel` is a normalised event kind.
contract/normalised-event-kinds/window · normalised event kinds · window — `window` is a normalised event kind, appended by the wrapper, log-only, that never wakes `wait`.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut, dialog-pending, no-session, child-not-ended, resume-failed.
contract/refusal-reason/unknown · RefusalReason · `unknown` refusal reason — The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/send-ok-payload/unconfirmable · send ok payload · unconfirmable — Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
contract/snapshot/writer · snapshot · writer — Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
contract/viola-up-exit-codes/0 · viola up exit codes · 0 — Every line reads live, already-live, stale-heartbeat or would-open.
contract/viola-up-exit-codes/1 · viola up exit codes · 1 — Any other line; or a refusal with a document under --json: boot-time-unread, strict-modes-failed, pinned-hash-mismatch or terminal-not-found.
contract/viola-up-exit-codes/21 · viola up exit codes · 21 — The board's strict refusal (instance-unreachable, strict-modes-failed), with no line printed.
decision/connect-failure-dead-closed-stream-lost · Failed connect is instance-dead, lost stream endpoint-lost — A failed connect reads as the instance-dead cause and a closed or broken stream as the endpoint-lost cause, on the CLI and in the MCP tools alike. · why: The exit 21 detail words, the process-exit during field and the MCP instance-unreachable detail all rest on the split.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/endpoint-name-stable-hash · Endpoint name from a stable hand-written hash — The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden. · why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/event-kinds-agent-neutral · Event kinds added only in viola-core — A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
contract/channel-fault-codes/-32602 · channel protocol fault codes · -32602 — Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
contract/channel-methods/hook-dialog · channel methods · hook.dialog — A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}, transcript?}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/channel-params/sender · channel params · sender — Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/14 · CLI exit codes · 14 — The `unknown` refusal.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Protocol fault from the wrapper, a -32602 from link or unlink included; the link self-pair is refused by the wrapper only, never client-side.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/cli-verbs/switch · CLI verbs · switch — `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2; prints `<name>  switched  <model> <effort>` on stdout and exits 0, 10, 12, 13, 14, 20 or 21.
contract/client-server-verify/check-order · Client server verification · check order — The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
contract/client-server-verify/server-verify-failed · Client server verification · server-verify-failed — A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/exit-21-human-hints/placement · Exit 21 human hints · placement — Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
contract/hook-to-kind-map/notification · Hook to kind map · Notification — Notification maps to `activity`, which is log-only and never wakes `wait`.
contract/hook-to-kind-map/posttooluse · Hook to kind map · PostToolUse — PostToolUse maps to `activity`, which is log-only and never wakes `wait`.
contract/hook-to-kind-map/posttoolusefailure · Hook to kind map · PostToolUseFailure — PostToolUseFailure maps to `activity`, which is log-only and never wakes `wait`.
contract/hook-to-kind-map/stop · Hook to kind map · Stop — Stop maps to `turn-ended`.
contract/hook-to-kind-map/userpromptsubmit · Hook to kind map · UserPromptSubmit — UserPromptSubmit maps to `prompt-submitted`.
contract/mcp-frame-tools/server-verify · MCP frame tools · server-verify — A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.
contract/mcp-frame-tools/socket-dir-refused · MCP frame tools · socket-dir-refused — A send, wait, last or answer meeting a refused socket directory returns the instance-unreachable isError result with detail socket-dir-refused, no frame written and no log line.
contract/mcp-frame-tools/strict-modes-failed · MCP frame tools · strict-modes-failed — A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
contract/normalised-event-kinds/send-refused · normalised event kinds · send-refused — `send-refused` is a normalised event kind.
contract/ok-payload/unconfirmable · ok payload · unconfirmable — unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured, or is a stdout echo with no readable session transcript, or for any listed local command on an unverified CLI.
contract/paste-text/control-characters · Paste text · control characters — LF, CR and TAB pass; every other C0, DEL and C1 is refused as not-delivered control-character, never stripped; the CLI checks before any frame and the wrapper checks again.
contract/paste-text/empty · Paste text · empty text — A text empty after CR LF and lone CR become LF and trailing CR and LF are dropped is refused as not-delivered empty-text, by the CLI and again by the wrapper; one also holding a refused character is control-character.
contract/peer-reply/detail · A peer's reply · detail — A string detail that is no code word is read as null; a non-string detail makes the reply no known reply.
contract/peer-reply/refusal · A peer's reply · refusal — A refusal that is no code word is read as unknown.
contract/refusal-order/answer · Refusal order · answer — control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.
contract/refusal-order/first-only · Refusal order · several refusals — When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.
contract/refusal-reason/unknown-catch-all · RefusalReason · unknown catch-all — The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.
contract/send/paste-failure-after-issue · send refusals · paste failing after send-issued — A paste that fails after send-issued is refused as input-not-ready with that send's cursor, except a paste cut at its bound, refused as paste-cut with that send's cursor.
contract/unreachable-cause/cause-set · Unreachable cause · cause set — UnreachableCause is a closed set of seven (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused, CallDeadline as call-deadline), no serde path, no RefusalReason; the six exit 21's, written by both clients, call-deadline by the MCP frame tools alone.

## standard-contracts · ## Standard Contracts
decision/agents-listing-never-errors · Agents listing degrades, never errors — When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
decision/hyprctl-direct-spawn · hyprctl spawned directly with a closed argv — hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is one Lua element, hl.dsp.focus or hl.dsp.window.move, whose only inserted values are a closed window address or an i32 workspace id, never reply text. · why: Hyprland 0.56.2 runs a dispatch's arguments as Lua, so only the measured hl.dsp.focus and hl.dsp.window.move forms are accepted.
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
decision/list-without-pagination · list without pagination — The `list` payload has no pagination; lists are returned whole. · why: Lists are personal-scale.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal. · except: viola adopt --json's not-resumed end prints its error document after the child released the terminal; nothing of run or revive.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/option-text-never-in-human-line · Upstream option text kept out of human lines — Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
decision/revive-refused-without-cwd · Revive over a replayed snapshot refused — A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage, launch and window from the log; a field no line gave is absent, never a default.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/up-writes-no-state · up writes no state of its own — viola up writes no snapshot, event line or stamp, leaving each to the revive it spawns; its lines, hint and --json document hold instance names and closed words only, never a window address, pid or workspace; under --dry-run it spawns nothing.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/agents-listing/status · agents listing · `status` — `claude agents --json` adds `status` (idle/busy) to the listing.
contract/agents-listing/unwrapped-sessions · agents listing · unwrapped sessions — Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/board/agents-read-failure · The board (CLI list and MCP list) · agents read failure — If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
contract/board/status · The board (CLI list and MCP list) · status — Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows.
contract/board/wrapped-merge · The board (CLI list and MCP list) · wrapped instance join — An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/channel-methods/hook-event-driver-origin · channel methods · hook.event driver origin — A `prompt-submitted` that is the in-flight `send`'s typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with `origin:"driver"`, its text kept as submitted, and only then settles that send.
contract/channel-methods/last · channel methods · last — A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
contract/channel-methods/last-after-seen-line · channel methods · last after a seen line — A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
contract/channel-methods/wait · channel methods · wait — A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
contract/channel-methods/wait-pending-dialog · channel methods · wait pending dialog — For `wait`, a pending dialog comes first unless `after` is past its line.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`; a finished or paused turn is for the driver to tell, and a parked `wait` never polls the file.
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
contract/channel-wait/result · channel wait method · result — {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
contract/channel-wait/start-offset · channel wait method · start offset — The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records, an activity record and a harness-origin prompt-submitted never wake, also after a task-free turn end.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/event-data/launch · event data · `launch` — `{model, effort}`, both strings, with `source: wrapper`.
contract/event-data/lineage · event data · `lineage` — `{parent, role}`, each a string or `null`, with `source: wrapper`.
contract/event-data/prompt-submitted-origin · event data per kind · prompt-submitted origin — `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`; the wrapper sets `driver` only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake `harness`.
contract/event-data/session-end · event data · `session-end` — `{}`.
contract/event-data/session-start · event data per kind · session-start — `session-start` data is `{cause, agent_session_id}`.
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
contract/event-data/window · event data · `window` — `{address, workspace, placement, origin}`, with `source: wrapper`; placement and origin are closed words.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/event-prompt-submitted/origin · prompt-submitted event data · origin — driver, human or harness; the hook files only human or harness; the wrapper sets driver only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake harness.
contract/hook-event/driver-relabel · channel hook.event · driver relabel — A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted, and only then settles that send.
contract/hyprland-seam-errors/malformed · Hyprland seam errors · malformed — The reply could not be parsed for its request; the fixed message hyprctl reply not understood interpolates nothing.
contract/hyprland-seam-errors/oversize · Hyprland seam errors · oversize — The reply passed the frame cap and was refused whole before the exit was read; the fixed message hyprctl reply over the frame cap interpolates nothing.
contract/hyprland-seam-errors/refused · Hyprland seam errors · refused — hyprctl refused a dispatch; the fixed message hyprctl refused the dispatch interpolates nothing.
contract/hyprland-seam-errors/unreachable · Hyprland seam errors · unreachable — hyprctl could not start, exited non-zero or was killed at its deadline; the fixed message hyprctl did not answer interpolates nothing.
contract/hyprland-seam-errors/unsupported · Hyprland seam errors · unsupported — No hyprctl program was given, so nothing is spawned and no line is written; the fixed message hyprctl not found interpolates nothing.
contract/instance-snapshot/agent-session-id · instance snapshot · agent_session_id — `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its readers are `viola revive`, which uses it as the revived child's spawn directory and its trust record's folder, and `viola up`, opening each terminal there.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?, lineage?, launch?, window?}`.
contract/instance-snapshot/launch · instance snapshot · launch — `launch` is `{model, effort}`, the pair the start or the latest switch handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
contract/instance-snapshot/lineage · instance snapshot · lineage — `lineage` is `{parent?, role?}`, the declared parent and role the start recorded; a replay rebuilds it from the log, the last `lineage` line replacing the held value.
contract/instance-snapshot/pending-dialog · instance snapshot · pending_dialog — `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/instance-snapshot/window · instance snapshot · window — `window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a later `launch` line clearing it.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/list-item/liveness · list payload item · liveness — live or stale; gone instances are omitted.
contract/list-item/parent · list payload item · parent — Present only when the recorded parent parses as an instance name other than the item's own.
contract/list-item/status · list payload item · status — idle, busy or unknown.
contract/list-item/wheel · list payload item · wheel — human or driver; absent when the session is unwrapped.
contract/list-item/wrapped · list payload item · wrapped — false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, cli_version or cli_verified.
contract/list-payload/agents-read-failure · list payload · agents read failure — A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window?, origin?}`.
contract/list-payload/liveness · list payload · liveness — An entry's `liveness` is `live` or `stale`; gone instances are omitted.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, links?, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/list-payload/status · list payload · status — A failed `claude agents --json` read leaves every `status` `unknown`.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
contract/session-liveness/heartbeat · session liveness · heartbeat — The wrapper touches `<instance dir>/heartbeat` every 1 s.
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`.
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`, `window`; the replay writes no file.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-channel-frames/ok-result · wrapper channel frames · ok result — A success is answered as `result` holding `{"ok": <payload>}`.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
contract/wrapper-event-appends/launch · wrapper event appends · `launch` — The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn; it is log-only and never wakes `wait`.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.
contract/wrapper-event-appends/window · wrapper event appends · `window` — The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window; it is log-only and never wakes `wait`.
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
decision/dialog-logged-exactly-once · Every dialog logged exactly once — Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once. · except: A matched continuation appends nothing, being the same dialog.
decision/link-set-follows-log · Link set follows the log — The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
decision/server-identity-check-before-frame · Server identity checked before the first frame — Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
decision/wrapper-channel-json-rpc · JSON-RPC over ndjson for the wrapper channel — The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets.
contract/answer-refusals/unknown-dialog · answer refusals · `unknown-dialog` — A late `answer`, to a dialog already answered or after its hold expired, is refused `unknown-dialog`.
contract/channel-methods/from · channel methods · from — `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
contract/channel-methods/release-from-driver · channel methods · release from driver — A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
contract/channel-methods/switch · channel methods · switch — A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed; a bad or missing half is -32602, data null.
contract/channel-methods/v-and-sender · channel methods · v and sender — Every channel method's `params` carries `v` and `sender`.
contract/channel-wait/params · channel wait method · after and timeout_ms — Each absent or a non-negative integer fitting u64; null, a negative, a fraction or a string is -32602; unknown fields are ignored.
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document.
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
contract/event-data/dialog-expired · event data · `dialog-expired` — `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text.
contract/event-data/permission · event data · `permission` — `{dialog_id, tool, input}`; `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload.
contract/event-data/plan · event data · `plan` — `{dialog_id, plan}`, with the plan as text.
contract/event-data/question · event data · `question` — `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`; each `question` string is a key in `answer`'s `answers`.
contract/event-data/send-confirmed · event data · `send-confirmed` — `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
contract/event-data/send-issued · event data · `send-issued` — `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/event-data/wheel · event data · `wheel` — `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
contract/event-prompt-submitted/text · prompt-submitted event data · text — The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it.
contract/hook-dialog/data · `hook.dialog` · `data` — `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`.
contract/hook-dialog/dialog-hooks · `hook.dialog` · dialog hooks — Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}; after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/human-error-line/wrapper-fault · The human error line · wrapper fault — A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — `child_pid` is the pid of the current agent child, rewritten when a switch respawns it, and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/links · instance snapshot · links — `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/list-payload/wrapped · list payload · wrapped — `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`.
contract/mcp-list/refusal · MCP list · refusal — The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal.
contract/viola-open-exit-codes/0 · viola open exit codes · 0 — The session went live: the human line `<name>  live  <placement>`, or under --json `{"v":1,"ok":{"name","parent","window","placement"}}`, window `{address, workspace}` or null.
contract/viola-open-exit-codes/1 · viola open exit codes · 1 — A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each under --json the `{"v":1,"error":"<code>","detail":null}` document.
contract/wrapper-channel-frames/envelope · wrapper channel frames · envelope — Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
contract/wrapper-event-appends/wheel · wrapper event appends · `wheel` — The wrapper appends `wheel` with `source: wrapper` once at start and on every change.

## occupied-resources · ## Occupied Resources
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads. · except: viola adopt's fixed not-resumed hint names claude --resume <id> as words for the human to type, never as an argument viola passes.
decision/connect-to-recorded-endpoint · Clients connect to the recorded endpoint — The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path. · why: Processes spawned inside the session may run with a different environment from the wrapper.
decision/default-launch-pair · Compiled default launch pair — The compiled default model and effort pair is one named constant, DEFAULT_PAIR, in the Claude agent crate, and the effort words are the five of claude --help; no environment variable sets the pair.
decision/fake-agent-pump-delay-seam · FAKE_AGENT_PUMP_DELAY_MS test seam — FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction. · why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: mcp's start and plugin install keep the write rule on the home.
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types.
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/recorded-pair-on-child-argv · Recorded launch pair on the child's command line — A model and effort pair read from the instance's record may be handed on the revived claude child's command line, each word re-parsed to its closed shape first. · why: A ratified crossing, beside the logged session id on the child's command line.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found. · except: A home that fails the strict-modes check while viola console runs, after its log is open, ends the console with one process-exit line of detail strict-modes-failed.
decision/revive-pair-from-record · Revive hands the recorded pair — Each half of the model and effort pair a revive hands is its --model or --effort override, else the recorded half while it still parses, else the start's default.
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home · except: A home that fails the strict-modes check while viola console runs, after its log is open, ends the console with exit 1, the home pair and one process-exit line of detail strict-modes-failed.
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness, viola-mcp-tap and viola-live-rig are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
contract/child-argv/fork-session · flags passed to the child · `--fork-session` — A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
contract/child-argv/launch-pair · flags passed to the child · launch pair — `--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived or adopted start, and a switch's respawn, launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
contract/claude-child-argv/fork-session · the claude child's argv · --fork-session — --fork-session follows --resume when --fork is given; the words after -- come last.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it; an adopted start adds --resume with the operator-typed id, in the session-id shape; a switch's respawn adds --resume with the switched id.
contract/diag-line/state-recovered · diagnostics line · state-recovered — A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm and viola adopt's start included.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/process-exit-detail/boot-time-unread · process-exit detail enum · boot-time-unread — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola up: its exit 1 when the host boot time cannot be read.
contract/process-exit-detail/not-live · process-exit detail enum · not-live — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json.
contract/process-exit-detail/terminal-not-found · process-exit detail enum · terminal-not-found — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/session-id/shape · Claude session id · shape — 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
contract/viola-cli-arguments/revive · viola CLI arguments · revive — viola revive takes <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>], or <name> --list, each with [--json].
contract/viola-cli-arguments/revive-origin · viola CLI arguments · revive --origin — viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
contract/viola-cli-arguments/run-effort · viola CLI arguments · run --effort — viola run --effort <EFFORT> names the effort handed to the child, one of the five effort words; any other value is a usage error, exit 2; with none given the effort is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-model · viola CLI arguments · run --model — viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-origin · viola CLI arguments · run --origin — viola run --origin <WORD> is a hidden flag naming the window's origin, an origin word; any other word is a usage error, exit 2.
contract/viola-cli-arguments/run-parent · viola CLI arguments · run --parent — viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2, before anything is read; with no --parent the inherited VIOLA_NAME is taken as parent only, dropped when it equals the own name.
contract/viola-cli-arguments/run-place · viola CLI arguments · run --place — viola run --place <N> is a hidden flag naming the workspace the window is moved to; --place without --origin is a usage error, exit 2.
contract/viola-list-exit-codes/21 · viola list exit codes · 21 — The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
contract/viola-revive-exit-codes/0 · viola revive exit codes · 0 — The revived child's wrapper ended as viola run's does.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
decision/cleanup-reads-refused-or-missing-socket-gone · Cleanup reads a refused or missing socket as gone — Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper. · why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/socket-dir-owner-only · Socket in a per-user owner-only directory — The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
decision/socket-in-per-user-dir · Endpoint socket in a per-user 0700 directory — The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
decision/stale-socket-file-left · Stale socket file left in place — A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: A choice: a harmless file in a 0700 directory.
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/channel-response-detail/invalid-params · channel-response detail, a closed enum · invalid-params — Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/config-json/default-effort · config.json · default_effort — One of the five effort words, default the effort half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
contract/config-json/default-model · config.json · default_model — A string of the model word shape, default the model half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
contract/frame-verb-exit-codes/21 · send, wait, last, answer, pause and release exit codes · 21 — The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
contract/frame-verb-exit-detail/endpoint-lost · CLI frame verb process-exit detail · endpoint-lost — Exit 21 with during call, for a wrapper lost with the call open.
contract/frame-verb-exit-detail/instance-dead · CLI frame verb process-exit detail · instance-dead — Exit 21 with during connect, at WARN.
contract/frame-verb-exit-detail/server-verify-failed · CLI frame verb process-exit detail · server-verify-failed — Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run's, revive's and adopt's start write it with exit_code 1.
contract/frame-verb-exit-detail/unwrapped-name · CLI frame verb process-exit detail · unwrapped-name — Exit 21 with during connect.
contract/frame-verb-exit-detail/wrapper-fault · CLI frame verb process-exit detail · wrapper-fault — Exit 20.
contract/process-exit-detail/unreachable · process-exit detail enum · unreachable — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/unsupported · process-exit detail enum · unsupported — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/window-not-found · process-exit detail enum · window-not-found — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — A refusal code verify writes under a parsed VIOLA_NAME.
contract/viola-cli-arguments/list · viola CLI arguments · list — viola list takes [--json] and nothing else.
contract/viola-list-exit-codes/0 · viola list exit codes · 0 — The board was printed in one stdout write.
contract/viola-revive-exit-codes/2 · viola revive exit codes · 2 — A clap usage error: a malformed --id, --model or --effort, or a model or effort flag in the words after the dashes, included.
contract/viola-revive-json/list · viola revive --json output · --list — viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/channel-params/sender-version · Channel params · sender version — Every channel params object carries the sender's CARGO_PKG_VERSION.
contract/channel-params/v · Channel params · v — Every channel params object carries the format version v.
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list, viola revive and viola console do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp, cli or console ndjson).
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing

## project-intent · ## Project Intent
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools. · except: viola console, the human's terminal fleet console, takes no --json.
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/clap-for-cli · clap parses the CLI — clap is what parses the CLI.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
contract/channel/framing · Channel · framing — The channel speaks JSON-RPC 2.0 over ndjson.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## build-system · key of ## Infrastructure Patterns
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/workspace-dependency-pins · Third-party versions pinned in one place — Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on viola-core alone among viola crates, never on viola-agent-claude, viola-state or viola-channel. · guard: the cargo tree check of viola-hyprland and the per-sync-crate tokio ban
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at.
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job
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
name/command/console — console
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
name/command/switch — switch
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
