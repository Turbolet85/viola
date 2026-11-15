# architecture · the rows that bind 2026-11-15-api-error-turn-end — picked 279 of 939 (164 by every draw) · rule any · pin 1610

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.

## stack-and-technologies · ## Stack and Technologies
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/e2e-no-rmcp-child-transport · E2E owns the viola mcp child — The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit. · why: transport-child-process kills the server 3 s after a close
decision/frame-params-redacted · Frame params never rendered in Debug — A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/plugin-embedded-in-binary · Claude Code plugin compiled into the binary — The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.

## established-decisions · ## Established Decisions
decision/anyhow-only-in-bin · anyhow only in the viola binary — anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter. · why: anyhow's context chains are only useful where a person reads them.
decision/byte-source-wheel-claim · Byte-source wheel claim — Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`. · why: `viola run` owns the real stdin, so it knows byte provenance exactly.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/exec-form-command-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments. · why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
decision/exec-form-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`. · why: Exec form skips shell startup.
decision/explicit-wheel-release · Explicit wheel release — The wheel returns from the human to the driver only through `viola release`.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/hook-connect-deadline · Hook connect deadline — The connect deadline a hook passes to the channel client is 750 ms, kept below the spine deadline. · guard: a unit test asserting it below the spine deadline · provisional
decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
decision/hook-without-trusted-instance-exits-silently · Hook with no trusted instance exits 0 untouched — A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file. · why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
decision/hooks-and-mcp-never-search-path · Hooks and MCP never search PATH — Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup. · why: PATH-found hooks run a different binary from the wrapper after a rebuild.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arm, called only by `viola verify`'s probes, reads no `VIOLA_*` and opens no channel, still exiting 0 with empty stderr.
decision/human-interrupt-moves-wheel · Human interrupt moves the wheel — A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state. · why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/ipc-interprocess-local-socket · interprocess local_socket for IPC — Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/lost-endpoint-loses-event · A lost endpoint loses the event, not the turn — When a hook cannot reach its endpoint, the hook event is lost and the turn goes on.
decision/no-path-lookup · Hooks and MCP never search PATH — Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/plugin-compiled-into-binary · Plugin compiled into the binary — viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start. · why: The binary and the plugin can no longer drift apart.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/spine-deadline · Spine deadline of 1 s — Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event. · guard: the perf gate (test-plan §10)
decision/spine-hook-loses-event-not-turn · Spine hook loses the event, never delays the turn — A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost. · why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
decision/verify-server-before-first-frame · Server verified before the first frame — Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison. · why: A squatted or rogue server is never sent a frame.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/wait-blocks-nothing-polls · wait blocks, nothing polls — wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
decision/wheel-in-wrapper-memory · Wheel decided in wrapper memory — The wheel is decided in the wrapper's memory and mirrored to a snapshot.
contract/channel-last/last-assistant-message · channel last · last_assistant_message — last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
contract/channel-wait/driver-relevant-events · channel wait · driver-relevant events — wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
contract/hook-diagnostics/server-verify-failed · Hook diagnostics · server-verify-failed — A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
contract/hook-diagnostics/socket-dir-refused · Hook diagnostics · socket-dir-refused — A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
contract/hook-exit/0-no-body · Hook exit and body · exit 0 with no body — No decision: the dialog renders for the human.
contract/hook-exit/2 · Hook exit and body · exit 2 — Never produced.
contract/hook-tiers/async · Hook tiers · async — Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit and Stop each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit and Stop run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/list-json/skipped · viola list --json · skipped — skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
contract/mcp-tools/list · MCP tools · list — One of the five MCP tools; takes `{}` and shows the driver the sessions.
contract/probe-capture/payload-file · Hook probe capture directory · `<PascalEvent>.<k>.json` — Each captured payload is claimed exclusively at 0600 under the first free `k`, which counts across events, so names sort in arrival order and two captures never share a name.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/frame-client-verifies-server · Frame clients verify the serving wrapper — Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
decision/frame-version-safety-net · Per-frame version as second safety net — Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
decision/hand-rolled-json-rpc-over-ndjson · JSON-RPC 2.0 over ndjson, hand-rolled with serde — The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one. · why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
decision/interprocess-local-socket-channel · interprocess local sockets behind the channel seam — interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/interprocess-local-socket-ipc · interprocess local sockets behind the channel seam — IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/permission-request-question-stays-null · A question first raised by PermissionRequest stays null — A question first raised by PermissionRequest stays null to the human. · why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/self-describing-frames-no-handshake · Self-describing frames, no handshake — Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. · why: Mixed binary versions are a measured fact, so every frame self-describes.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/socket-dir-check-at-channel-callers · Socket-directory check at the channel's callers — The socket-directory check stands at the channel seam's callers, not inside the channel client's connect. · why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-never-types-into-dialog · viola verify never answers a CLI dialog — No `viola verify` run writes a byte into a CLI dialog.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-types-nothing-into-dialogs · Verify never types into a CLI dialog — No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key. · why: Ratified rulings.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks.
decision/wheel-byte-source-claim · Byte-source claim of the wheel — Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts. · why: `viola run` owns the real stdin, so it knows byte provenance.
decision/wheel-returns-only-through-release · Wheel returns only through CLI release — CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
contract/channel-results/cursor · channel results · cursor — Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
contract/channel-wait/after · channel wait · after — after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
contract/channel-wait/timeout-ms · channel wait · timeout_ms — On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
contract/hook-exit/0-with-body · Hook exit and body · exit 0 with a JSON body — A decision.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail.
contract/viola-verify-refusal-codes/strict-modes-failed · viola verify process-exit refusal codes · strict-modes-failed — Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.

## conventions · ## Conventions
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/anyhow-only-in-root-bin · anyhow only in the root bin crate — `anyhow` appears only in the root bin crate.
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
decision/event-kinds-agent-neutral · Event kinds added only in viola-core — A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/cli-verbs/wait · CLI verbs · wait — `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
contract/hook-to-kind-map/stop · Hook to kind map · Stop — Stop maps to `turn-ended`.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
contract/channel-params/sender · channel params · sender — Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
contract/channel-response/ok · channel response · ok — Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-verbs/pause · CLI verbs · pause — `viola pause <target>`.
contract/cli-verbs/release · CLI verbs · release — `viola release <target>`; a `--budget` flag is a usage error (exit 2).
contract/cli-verbs/send · CLI verbs · send — `viola send <target>` takes its text from stdin or `--file`.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/mcp-tool-results/success · MCP tool results · success — A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, skipped}` payload.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
contract/verify/no-json · viola verify · --json — viola verify takes no --json.

## standard-contracts · ## Standard Contracts
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
decision/hook-fails-open-on-replayed-snapshot · Hook fails open on a replayed snapshot — viola hook reads a replayed or absent snapshot as unreachable and fails open.
decision/hook-output-one-crate · Hook output produced by one crate — The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
decision/mcp-list-reads-agents · MCP list reads claude agents on each call — The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call. · why: A ratified widening.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default.
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/agents-listing/status · agents listing · `status` — `claude agents --json` adds `status` (idle/busy) to the listing.
contract/agents-listing/unwrapped-sessions · agents listing · unwrapped sessions — Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/board/status · The board (CLI list and MCP list) · status — Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows.
contract/board/wrapped-merge · The board (CLI list and MCP list) · wrapped instance join — An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/channel-methods/last · channel methods · last — A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
contract/channel-methods/last-after-seen-line · channel methods · last after a seen line — A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
contract/channel-methods/wait · channel methods · wait — A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file.
contract/channel-wait/result · channel wait method · result — {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
contract/event-data/turn-ended-background-tasks · event data per kind · turn-ended background_tasks — `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/hook-contract/async-tier · hook contract · async-tier hooks — Async-tier hooks always print nothing.
contract/hook-contract/no-decision · hook contract · no decision — No decision is exit 0 with an empty stdout.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail".
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`.
contract/list-item/status · list payload item · status — idle, busy or unknown.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?}`.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`; the replay writes no file.
contract/wrapper-channel-frames/envelope · wrapper channel frames · envelope — Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
contract/wrapper-channel-frames/id-less-frames · wrapper channel frames · id-less frames — Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
contract/wrapper-channel-frames/ok-result · wrapper channel frames · ok result — A success is answered as `result` holding `{"ok": <payload>}`.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/server-identity-check-before-frame · Server identity checked before the first frame — Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
contract/channel-methods/v-and-sender · channel methods · v and sender — Every channel method's `params` carries `v` and `sender`.
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
contract/channel-wait/start-offset · channel wait method · start offset — The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/list-item/wheel · list payload item · wheel — human or driver; absent when the session is unwrapped.
contract/list-payload/agents-read-failure · list payload · agents read failure — A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
contract/list-payload/skipped · list payload · skipped — `skipped` counts records a reader could not interpret, so the board can show them.
contract/list-payload/status · list payload · status — A failed `claude agents --json` read leaves every `status` `unknown`.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
contract/wrapper-event-appends/wheel · wrapper event appends · `wheel` — The wrapper appends `wheel` with `source: wrapper` once at start and on every change.

## occupied-resources · ## Occupied Resources
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/hook-missing-instance-dir-exits-silently · Hook with a missing instance folder exits 0 silently — A hook whose VIOLA_DIR names an instance folder that does not exist fails the canonical-directory step, exits 0 and writes nothing: no role file, no stdin read, no frame and no event line. · why: Claude Code runs the plugin's hooks in sessions viola does not wrap; the hook fail-open contract and its no-instance-directory test rely on it.
decision/plugin-env-vars-unused · Plugin-provided environment variables unused — viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole when it passes, fails or is stopped by Ctrl-C or TERM, and only the folders that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/hook-capture-arm/payload-file · viola hook --capture arm · payload file — Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/connect-to-recorded-endpoint · Clients connect to the recorded endpoint — The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path. · why: Processes spawned inside the session may run with a different environment from the wrapper.
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/events-log-never-rotated · Events log never rotated — v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into it
decision/events-log-never-truncated · Events log never rotated or truncated — The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into the file.
decision/no-after-every-test-error-body-helper · No after-every-test error-body helper — Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/channel-response-detail/unsupported-version · channel-response detail, a closed enum · unsupported-version — Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/viola-cli-arguments/list · viola CLI arguments · list — viola list takes [--json] and nothing else.
contract/viola-list-exit-codes/0 · viola list exit codes · 0 — The board was printed in one stdout write.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/hook-exits-on-untrusted-instance · Hook exits at once on an untrusted instance — viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append. · except: The --capture arm, which reads no VIOLA_* variable, runs ahead of the check.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/plugin-paths-forward-slashes · Forward slashes in plugin paths — Every path written into plugin files uses forward slashes.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
decision/local-cargo-install · Local install with embedded plugin — viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.

## build-system · key of ## Infrastructure Patterns
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/sync-crates-compile-without-tokio · Sync crates compile without tokio — Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/core-depends-on-no-viola-crate · viola-core depends on no viola crate — viola-core depends on no other viola crate.

## project-directory-structure · key of ## Infrastructure Patterns
decision/anyhow-at-bin-edge-only · anyhow at the binary edge only — anyhow error handling is confined to the edge of the viola binary.
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid, started_at or links.

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
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
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
