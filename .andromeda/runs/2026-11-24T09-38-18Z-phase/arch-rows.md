# architecture · the rows that bind 2026-11-24-viola-adopt — picked 339 of 1037 (254 by every draw) · rule any · pin 4656

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/append-one-write-per-line · Log appends one write per line — Log appends use std append mode with one write per line. · why: crash-safe multi-process writes with no C code
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/content-hash-sha256 · Pure-Rust SHA-256 content hash — The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).
decision/pinned-copy-placed-once · Pinned copy placed once, never replaced — The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/portable-pty-seam · portable-pty behind viola's own pty seam — The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/server-start-time-read-through-sysinfo · Server start time is read from the OS through sysinfo — The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame. · why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/wrapper-ipc-local-socket · interprocess local sockets for wrapper IPC — Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/clap-cli-parser · clap as CLI parser, without colour — clap (derive) parses the CLI with its default features minus color. · why: help and usage output stays plain and no colour or terminal crate is in the graph
decision/clients-read-server-pid-euid-from-peer-creds · Clients read server pid and euid from peer credentials — Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame. · why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/no-database · No database: ndjson logs and JSON snapshots — State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
decision/no-message-broker · No message broker — Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.
decision/plugin-embedded-in-binary · Claude Code plugin compiled into the binary — The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.
decision/preserve-key-order · Printed documents keep declared key order — serde_json's preserve_order feature is on, so a printed document keeps its declared key order.

## established-decisions · ## Established Decisions
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/exclusive-endpoint-bind · Exclusive endpoint bind — A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name. · why: So two concurrent starts cannot both win.
decision/hashed-endpoint-names · Hashed endpoint names — Endpoint names are a short hash of the instance, never the raw instance name and path.
decision/hooks-and-mcp-never-search-path · Hooks and MCP never search PATH — Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup. · why: PATH-found hooks run a different binary from the wrapper after a rebuild.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/launch-pair-handed-not-verified · Launch pair handed, never verified — Every start, run and revive, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on. · why: Whether a session started or resumed with the pair runs on it is unmeasured on the real CLI.
decision/lineage-label-only · Lineage is a label only — Lineage is a label only: no wheel, link, send or answer path reads the declared parent or role; viola up reads the parent only to order its re-opens. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run`, `viola revive` or `viola open` whose `ViolaName` is `live` or `stale` refuses to start.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.
decision/pinned-copy-never-replaced · Pinned copy never replaced — The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
decision/pinned-copy-no-clobber-placement · Pinned copy placed no-clobber with the handle closed — The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched. · why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/plugin-compiled-into-binary · Plugin compiled into the binary — viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start. · why: The binary and the plugin can no longer drift apart.
decision/plugin-only-in-wrapped-sessions · Plugin loads only into wrapped sessions — The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`. · why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola. · except: `viola verify`'s transient unwrapped children, all gone when verify ends.
decision/r8-identity-floor · R8 parent-identity floor — The identity-floor CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/run-refuses-held-name · A start refuses a held name — viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/socket-dir-check-at-channel-callers · Socket-directory check at the channel's callers — The socket-directory check stands at the channel seam's callers, not inside the channel client's connect. · why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
decision/socket-dir-check-verdicts · Socket-directory check verdicts, never chmods — The socket-directory check passes an absent path and never chmods, so a refused directory stays as found. It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path. · why: A start makes an absent directory itself, and the negative tests for start, frame verbs and hooks expect a loosened or linked directory left as found.
decision/socket-dir-refused-covers-create-and-check · Socket-dir create failure and refusal read the same — A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/start-verifies-new-cli-version · Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record and the claude child, and gates on the final verdict; the key is off by default.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/viola-revive-json-error-codes/already-live · viola revive --json error codes · already-live — The name is held by a live instance.
contract/viola-revive-json-error-codes/cwd-missing · viola revive --json error codes · cwd-missing — The recorded directory is missing.
contract/viola-revive-json-error-codes/internal-error · viola revive --json error codes · internal-error — A claude not on PATH or an anyhow error of the start.
contract/viola-revive-json-error-codes/pinned-hash-mismatch · viola revive --json error codes · pinned-hash-mismatch — The pinned copy no longer matches its hash.
contract/viola-revive-json-error-codes/session-live · viola revive --json error codes · session-live — The logged session is held by a live claude.
contract/viola-revive-json-error-codes/socket-dir-refused · viola revive --json error codes · socket-dir-refused — The socket directory was refused or could not be created.
contract/viola-revive-json-error-codes/squatted-name · viola revive --json error codes · squatted-name — The endpoint name is already taken at bind.
contract/viola-revive-json-error-codes/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The name is held by a stale instance.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — A revive whose recorded cwd names no existing directory is refused cwd-missing.
decision/append-heals-torn-line · The next append heals a torn line — Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/frame-client-verifies-server · Frame clients verify the serving wrapper — Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
decision/interprocess-local-socket-channel · interprocess local sockets behind the channel seam — interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/interprocess-local-socket-ipc · interprocess local sockets behind the channel seam — IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
decision/ipc-interprocess-local-socket · interprocess local_socket for IPC — Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/no-automatic-reverify-of-red · No automatic verify of an earlier red version — An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that did not verify is never verified again automatically. · why: So a red version does not loop.
decision/no-path-lookup · Hooks and MCP never search PATH — Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/portable-pty-behind-pty-seam · portable-pty pinned behind viola's pty seam — portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill). · why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
decision/private-mode-before-write · File mode set before any byte is written — A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
decision/separate-lock-files · Locks on separate .lock files — State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked. · why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
decision/torn-last-line-healed · Torn log line healed by the next append — Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/verify-server-before-first-frame · Server verified before the first frame — Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison. · why: A squatted or rogue server is never sent a frame.
decision/window-features-target-hyprland · Window features target Hyprland — The window features target the Hyprland compositor.

## conventions · ## Conventions
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
decision/endpoint-name-stable-hash · Endpoint name from a stable hand-written hash — The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden. · why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`. · except: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/model-word/format · Model word · format — A model word is 1 to 64 bytes, a lower-case letter first, then lower-case letters, digits, dots or hyphens.
contract/normalised-event-kinds/launch · normalised event kinds · launch — `launch` is a normalised event kind.
contract/normalised-event-kinds/lineage · normalised event kinds · lineage — `lineage` is a normalised event kind.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/snapshot/writer · snapshot · writer — Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/mcp-tools-list-held-to-recorded-exchange · MCP tools/list held to a recorded claude exchange — The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order. · guard: mcp_recorded_exchange (viola-e2e)
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/retired-exit-codes-never-reused · Retired exit codes never reused — A retired CLI exit code is never reused or renumbered: its number stays unassigned.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
contract/mcp-frame-tools/server-verify · MCP frame tools · server-verify — A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.

## standard-contracts · ## Standard Contracts
decision/agents-listing-never-errors · Agents listing degrades, never errors — When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage, launch and window from the log; a field no line gave is absent, never a default.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/agents-listing/unwrapped-sessions · agents listing · unwrapped sessions — Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/board/agents-read-failure · The board (CLI list and MCP list) · agents read failure — If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
contract/board/wrapped-merge · The board (CLI list and MCP list) · wrapped instance join — An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/event-data/launch · event data · `launch` — `{model, effort}`, both strings, with `source: wrapper`.
contract/event-data/lineage · event data · `lineage` — `{parent, role}`, each a string or `null`, with `source: wrapper`.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/instance-snapshot/agent-session-id · instance snapshot · agent_session_id — `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — `child_pid` is the pid of the spawned agent child and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/cli-verified · instance snapshot · cli_verified — `cli_verified` is the start's final verdict: run's version gate, its stamps read again after the start's automatic verify where one ran.
contract/instance-snapshot/cli-version · instance snapshot · cli_version — `cli_version` comes from `run`'s version gate.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its readers are `viola revive`, which uses it as the revived child's spawn directory and its trust record's folder, and `viola up`, opening each terminal there.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?, lineage?, launch?, window?}`.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
contract/instance-snapshot/launch · instance snapshot · launch — `launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
contract/instance-snapshot/lineage · instance snapshot · lineage — `lineage` is `{parent?, role?}`, the declared parent and role the start recorded; a replay rebuilds it from the log, the last `lineage` line replacing the held value.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/list-item/effort · list payload item · effort — Present only when the recorded effort re-parses as a code word at the board; follows model.
contract/list-item/liveness · list payload item · liveness — live or stale; gone instances are omitted.
contract/list-item/model · list payload item · model — Present only when the recorded model re-parses as a model word at the board; follows role.
contract/list-item/parent · list payload item · parent — Present only when the recorded parent parses as an instance name other than the item's own.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window?, origin?}`.
contract/list-payload/liveness · list payload · liveness — An entry's `liveness` is `live` or `stale`; gone instances are omitted.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned.
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
contract/session-liveness/heartbeat · session liveness · heartbeat — The wrapper touches `<instance dir>/heartbeat` every 1 s.
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`.
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`, `window`; the replay writes no file.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-event-appends/launch · wrapper event appends · `launch` — The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`; it is log-only and never wakes `wait`.
decision/hyprctl-direct-spawn · hyprctl spawned directly with a closed argv — hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is one Lua element, hl.dsp.focus or hl.dsp.window.move, whose only inserted values are a closed window address or an i32 workspace id, never reply text. · why: Hyprland 0.56.2 runs a dispatch's arguments as Lua, so only the measured hl.dsp.focus and hl.dsp.window.move forms are accepted.
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/instance-snapshot/links · instance snapshot · links — `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
contract/instance-snapshot/window · instance snapshot · window — `window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a later `launch` line clearing it.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/list-item/wrapped · list payload item · wrapped — false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, cli_version or cli_verified.
contract/list-payload/agents-read-failure · list payload · agents read failure — A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, links?, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/list-payload/status · list payload · status — A failed `claude agents --json` read leaves every `status` `unknown`.
contract/list-payload/wrapped · list payload · wrapped — `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`.
contract/trust-writer-words/key-present · Trust writer code words · key-present — The config already holds a record under the key; it is left as found and nothing is written. Only viola verify's writer gives it; a start reads it as lost.
contract/trust-writer-words/removed · Trust writer code words · removed — The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
contract/viola-open-exit-codes/1 · viola open exit codes · 1 — A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each under --json the `{"v":1,"error":"<code>","detail":null}` document.
contract/viola-open-exit-codes/2 · viola open exit codes · 2 — A usage error: a clap error, any word after -- but claude alone, or a parent equal to the opened name.

## occupied-resources · ## Occupied Resources
decision/automatic-verify-attempt-lock · Automatic verify under its own attempt lock — A start verifies only holding ledger/verify.lock, owner-only, try-locked after the stamps strict-modes check, never the stamps lock; a held lock means no verify and an unverified start. Under it the start takes another start's verdict only when the entry changed after its own gate read it. · guard: verify_window_two_starts_at_once_run_one_verify
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value makes the dialog hold zero or unbounded
decision/connect-to-recorded-endpoint · Clients connect to the recorded endpoint — The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path. · why: Processes spawned inside the session may run with a different environment from the wrapper.
decision/default-launch-pair · Compiled default launch pair — The compiled default model and effort pair is one named constant, DEFAULT_PAIR, in the Claude agent crate, and the effort words are the five of claude --help; no environment variable sets the pair.
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types.
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/revive-pair-from-record · Revive hands the recorded pair — Each half of the model and effort pair a revive hands is its --model or --effort override, else the recorded half while it still parses, else the start's default. · provisional
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/socket-dir-owner-only · Socket in a per-user owner-only directory — The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
decision/socket-in-per-user-dir · Endpoint socket in a per-user 0700 directory — The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/child-argv/launch-pair · flags passed to the child · launch pair — `--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/config-json/default-effort · config.json · default_effort — One of the five effort words, default the effort half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
contract/config-json/default-model · config.json · default_model — A string of the model word shape, default the model half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
contract/config-json/verify-new-cli · config.json · verify_new_cli — A JSON boolean, default false, turning on the start's automatic verify; any other value reads false with one parse-rejected config-json malformed line, and v must be 1.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/session-id/shape · Claude session id · shape — 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
contract/viola-cli-arguments/run-effort · viola CLI arguments · run --effort — viola run --effort <EFFORT> names the effort handed to the child, one of the five effort words; any other value is a usage error, exit 2; with none given the effort is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-model · viola CLI arguments · run --model — viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-parent · viola CLI arguments · run --parent — viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2, before anything is read; with no --parent the inherited VIOLA_NAME is taken as parent only, dropped when it equals the own name.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/events-log-never-rotated · Events log never rotated — v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into it
decision/events-log-never-truncated · Events log never rotated or truncated — The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into the file.
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: mcp's start and plugin install keep the write rule on the home.
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace socket names are never used. · why: They carry no filesystem permissions and any local user could connect.
decision/plugin-env-vars-unused · Plugin-provided environment variables unused — viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary and reads no value of those, CLAUDE_ENV_FILE or CLAUDE_PROJECT_DIR; the four are known non-identity hook names, read by name only by the identity-floor ledger check.
decision/recorded-pair-on-child-argv · Recorded launch pair on the child's command line — A model and effort pair read from the instance's record may be handed on the revived claude child's command line, each word re-parsed to its closed shape first. · why: A ratified crossing, beside the logged session id on the child's command line.
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
decision/socket-file-owner-only · Socket file owner-only — The socket is chmod 0600 after the bind.
decision/stale-socket-file-left · Stale socket file left in place — A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: A choice: a harmless file in a 0700 directory.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
contract/config-json/v · config.json · v — config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/process-exit-detail/boot-time-unread · process-exit detail enum · boot-time-unread — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola up: its exit 1 when the host boot time cannot be read.
contract/process-exit-detail/malformed · process-exit detail enum · malformed — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/not-live · process-exit detail enum · not-live — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/process-exit-detail/oversize · process-exit detail enum · oversize — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/refused · process-exit detail enum · refused — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json.
contract/process-exit-detail/terminal-not-found · process-exit detail enum · terminal-not-found — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/process-exit-detail/unreachable · process-exit detail enum · unreachable — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/unsupported · process-exit detail enum · unsupported — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/process-exit-detail/window-not-found · process-exit detail enum · window-not-found — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola focus.
contract/viola-revive-exit-codes/0 · viola revive exit codes · 0 — The revived child's wrapper ended as viola run's does.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list and viola revive do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/same-user-endpoint-trust · Local endpoint trusts the same OS user only — v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/config-json/unknown-keys · config.json · unknown keys — config.json is parsed tolerantly: unknown keys are skipped and counted.
contract/exit-21-human-hints/socket-dir · Exit 21 human hints · socket-dir — The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/hashed-short-endpoint-names · Hashed short endpoint names — Endpoint names are hashed and short.

## project-intent · ## Project Intent
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/clap-for-cli · clap parses the CLI — clap is what parses the CLI.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/interprocess-local-sockets · interprocess local sockets for IPC — IPC uses interprocess local sockets, one endpoint per viola run.
decision/local-cargo-install · Local install with embedded plugin — viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.

## build-system · key of ## Infrastructure Patterns
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
contract/cli-json/internal-error · CLI --json output · internal-error — Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at.

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
