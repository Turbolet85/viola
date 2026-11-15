# architecture · the rows that bind 2026-11-15-live-round-on-the-trimmed-product — picked 291 of 938 (169 by every draw) · rule any · pin 1297

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/content-hash-sha256 · Pure-Rust SHA-256 content hash — The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
decision/e2e-no-rmcp-child-transport · E2E owns the viola mcp child — The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit. · why: transport-child-process kills the server 3 s after a close
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).
decision/pinned-copy-placed-once · Pinned copy placed once, never replaced — The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/plugin-embedded-in-binary · Claude Code plugin compiled into the binary — The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.
decision/portable-pty-seam · portable-pty behind viola's own pty seam — The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
decision/vt100-screen-model · vt100 as screen model — vt100 gives pre-send readiness and modal detection on run's feed thread and the screen of verify's interactive probe runs.
decision/mcp-server-rmcp · rmcp is the MCP server — The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
decision/preserve-key-order · Printed documents keep declared key order — serde_json's preserve_order feature is on, so a printed document keeps its declared key order.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key

## established-decisions · ## Established Decisions
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/dialog-hold · Dialog hold per instance — A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start. · provisional
decision/dialog-hold-bound · Dialog hold bounds a sync dialog hook — A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60 to 3600 s, default 60), after which the dialog renders for the human. · provisional
decision/exclusive-endpoint-bind · Exclusive endpoint bind — A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name. · why: So two concurrent starts cannot both win.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/hooks-and-mcp-never-search-path · Hooks and MCP never search PATH — Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup. · why: PATH-found hooks run a different binary from the wrapper after a rebuild.
decision/human-controls-cli-only · Human wheel and link controls are CLI-only — `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface. · why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
decision/human-key-never-held-behind-resize · Human key never held behind a resize — viola never holds, queues or re-orders a human key behind a resize.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/no-key-held-behind-resize · No human key held behind a resize — viola never holds, queues or re-orders a human key behind a resize; a test that writes a key after a resize waits for the child to observe the new size first.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
decision/no-typing-into-trust-dialogs · viola types into no trust dialog — viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
decision/passthrough-never-waits-on-model · Passthrough never waits on the screen model — The human's bytes are written first and the screen model gets a copy through a bounded queue it may drop from, so the passthrough never waits on the model.
decision/pause-link-unlink-cli-only · pause, link and unlink are CLI-only — `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools. · why: They are human controls over the wheel and the link topology.
decision/pinned-copy-never-replaced · Pinned copy never replaced — The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/plugin-compiled-into-binary · Plugin compiled into the binary — viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start. · why: The binary and the plugin can no longer drift apart.
decision/plugin-only-in-wrapped-sessions · Plugin loads only into wrapped sessions — The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`. · why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola. · except: `viola verify`'s transient unwrapped children, all gone when verify ends.
decision/portable-pty-behind-pty-seam · portable-pty pinned behind viola's pty seam — portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill). · why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
decision/r8-identity-floor · R8 parent-identity floor — The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/release-cli-only · release is CLI-only — `release` is never an MCP tool; only the CLI offers it. · why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
decision/run-refuses-held-name · A start refuses a held name — viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/screen-signatures-compiled · Screen signatures as compiled literals — Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char. · why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-never-types-into-dialog · viola verify never answers a CLI dialog — No `viola verify` run writes a byte into a CLI dialog.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-types-nothing-into-dialogs · Verify never types into a CLI dialog — No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key. · why: Ratified rulings.
decision/viola-ends-the-dialog-wait · Viola, never Claude Code, ends the dialog wait — The dialog hook bounds its reply read at the hold ceiling plus 5 s, and the dialog hooks' `timeout` in `hooks.json` sits above that by a pinned margin. · why: Viola ends the wait rather than Claude Code, and the rendered plugin stays byte-identical across starts.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks.
decision/wheel-classifier-only-observes · Wheel classifier only observes — The stdin classifier returns every byte whole and unchanged, and a read holding an editing key moves the wheel before it returns.
contract/child-exit-record/exit-source · Child exit record · exit_source — The child's exit is recorded with exit_source handle-wait or kill-fallback.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/dialog-hold-end/null-reply · Dialog hold end · null reply — At the hold's end the wrapper replies `null`, the hook exits 0 with no body, the dialog renders for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
contract/event-line/dialog-expired · The event line · dialog-expired — Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
contract/hook-exit/0-no-body · Hook exit and body · exit 0 with no body — No decision: the dialog renders for the human.
contract/hook-tiers/sync-with-decision · Hook tiers · sync with a decision — PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest, run sync and may reply with a decision.
contract/mcp-tools/list · MCP tools · list — One of the five MCP tools; takes `{}` and shows the driver the sessions.
contract/plugin-mcp-json/viola · The plugin's .mcp.json · viola — Registers the server viola with command the pinned copy's absolute path and args ["mcp"].
contract/viola-revive-json-error-codes/already-live · viola revive --json error codes · already-live — The name is held by a live instance.
contract/viola-revive-json-error-codes/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The name is held by a stale instance.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
decision/anyhow-only-in-bin · anyhow only in the viola binary — anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter. · why: anyhow's context chains are only useful where a person reads them.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/exec-form-command-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments. · why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
decision/exec-form-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`. · why: Exec form skips shell startup.
decision/hashed-endpoint-names · Hashed endpoint names — Endpoint names are a short hash of the instance, never the raw instance name and path.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arm, called only by `viola verify`'s probes, reads no `VIOLA_*` and opens no channel, still exiting 0 with empty stderr.
decision/human-wheel-renders-dialogs · Human-held wheel renders dialogs at once — While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once. · why: A dialog held by the hook is never hidden from the human for the rest of the hold.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/no-da1-answer · viola never answers DA1 — viola never answers a DA1 query itself.
decision/no-model-api-calls · No model API calls — The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI. · why: D2, subscription only through the unmodified `claude` CLI.
decision/no-path-lookup · Hooks and MCP never search PATH — Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.
decision/one-pending-dialog · At most one pending dialog per instance — At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
decision/permission-request-question-stays-null · A question first raised by PermissionRequest stays null — A question first raised by PermissionRequest stays null to the human. · why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.
decision/pinned-copy-no-clobber-placement · Pinned copy placed no-clobber with the handle closed — The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched. · why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
contract/dialog-events/dialog-id · dialog events · dialog_id — Each dialog event carries a wrapper-assigned integer dialog_id, monotonic per instance and never repeated across wrapper restarts; answer names its target by it.
contract/mcp-tools/answer · MCP tools · answer — One of the five MCP tools; takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
contract/mcp-tools/last · MCP tools · last — One of the five MCP tools; takes `{target}`.
contract/mcp-tools/send · MCP tools · send — One of the five MCP tools; takes `{target, text}`, `target` a `ViolaName`.
contract/mcp-tools/wait · MCP tools · wait — One of the five MCP tools; takes `{target, after?, timeout_ms?}`.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail.

## conventions · ## Conventions
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
decision/mcp-tools-list-held-to-recorded-exchange · MCP tools/list held to a recorded claude exchange — The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order. · guard: mcp_recorded_exchange (viola-e2e)
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
contract/channel-methods/hook-dialog · channel methods · hook.dialog — A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/mcp-list-results/cachescope · MCP list results · cacheScope — The tools/list and server/discover results each carry cacheScope private, because claude drops every tool of a list result without it.
contract/mcp-list-results/ttlms · MCP list results · ttlMs — The tools/list and server/discover results each carry ttlMs 0, because claude drops every tool of a list result without it.
contract/normalised-event-kinds/dialog-expired · normalised event kinds · dialog-expired — `dialog-expired` is a normalised event kind.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
contract/verify/no-json · viola verify · --json — viola verify takes no --json.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/cli-exit-codes/11 · CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/hook-to-kind-map/permissionrequest · Hook to kind map · PermissionRequest — PermissionRequest maps to `permission`, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
contract/hook-to-kind-map/pretooluse-askuserquestion · Hook to kind map · PreToolUse AskUserQuestion — PreToolUse `AskUserQuestion` maps to `question`.
contract/hook-to-kind-map/pretooluse-exitplanmode · Hook to kind map · PreToolUse ExitPlanMode — PreToolUse `ExitPlanMode` maps to `plan`.
contract/normalised-event-kinds/question · normalised event kinds · question — `question` is a normalised event kind.
contract/normalised-event-kinds/session-end · normalised event kinds · session-end — `session-end` is a normalised event kind.
contract/normalised-event-kinds/session-start · normalised event kinds · session-start — `session-start` is a normalised event kind.
contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/verify-process-exit/interrupted · viola verify process-exit detail · interrupted — The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
contract/verify-process-exit/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — The code verify's process-exit carries when the pinned copy fails its re-hash.
contract/verify-process-exit/program-not-found · viola verify process-exit detail · program-not-found — The code verify's process-exit carries when the claude CLI is not found.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-process-exit/version-unreadable · viola verify process-exit detail · version-unreadable — The code verify's process-exit carries when the CLI version cannot be read.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.

## standard-contracts · ## Standard Contracts
decision/dialog-logged-exactly-once · Every dialog logged exactly once — Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once. · except: A matched continuation appends nothing, being the same dialog.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/mcp-over-stdio · MCP over stdio only — MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/event-data/dialog-expired · event data · `dialog-expired` — `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text.
contract/instance-snapshot/cli-verified · instance snapshot · cli_verified — `cli_verified` comes from `run`'s version gate.
contract/instance-snapshot/cli-version · instance snapshot · cli_version — `cli_version` comes from `run`'s version gate.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/instance-snapshot/pending-dialog · instance snapshot · pending_dialog — `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail".
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`.
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
decision/link-set-follows-log · Link set follows the log — The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/hook-contract/no-decision · hook contract · no decision — No decision is exit 0 with an empty stdout.
contract/hook-dialog/dialog-hooks · `hook.dialog` · dialog hooks — Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
contract/session-liveness/heartbeat · session liveness · heartbeat — The wrapper touches `<instance dir>/heartbeat` every 1 s.

## occupied-resources · ## Occupied Resources
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/cleanup-reads-refused-or-missing-socket-gone · Cleanup reads a refused or missing socket as gone — Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper. · why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing.
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value makes the dialog hold zero or unbounded
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
decision/e2e-home-headroom-floor · e2e-home backing headroom floor — A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key. · why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
decision/e2e-home-owner-only-headroom · End-to-end home base checked before use — The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
decision/e2e-noop-fake-agent-feature · No-op fake-agent feature in the e2e crate — The test-only `viola-e2e` crate declares a no-op `fake-agent` feature. · why: So package-scoped `--features fake-agent` invocations resolve.
decision/identity-floor-eleven-names · Identity floor of eleven names — The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID. · why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
decision/linked-e2e-home-base-strict · Linked e2e-home base must be a private real directory — A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/stale-socket-file-left · Stale socket file left in place — A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: A choice: a harmless file in a 0700 directory.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
contract/config-json/v · config.json · v — config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
contract/e2e-home-refusal/headroom-message · e2e-home base refusal · headroom message — A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
contract/harness-environment/agent-run-keep-homes · test harness environment · AGENT_RUN_KEEP_HOMES — The value 1 keeps the test and session homes for the CI gates.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value.
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/mcp-subcommand-hidden · Hidden mcp subcommand — The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home. · why: The plugin runs it, never a person.
decision/plugin-carries-no-hooks · Installed plugin carries no hooks — The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks.
decision/plugin-env-vars-unused · Plugin-provided environment variables unused — viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
decision/plugin-install-rewrites-marketplace-whole · Plugin install rewrites the marketplace tree whole — The plugin marketplace tree is rewritten whole on every plugin install, any other entry there is left as found, and viola never reads the tree back. · why: A rerun re-points the MCP server at the current exe and home, the rerun test expects the same tree, and only its folders need the strict-modes check.
decision/profraw-census-dev-host-only · Profraw census is a dev-host witness only — The profraw census script is a dev-host witness for the corrupt-profile mechanism, and no CI job, pre-push stage or harness command runs either of its forms.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/verify-run-e-resumes-run-b-first-session · Verify's Run E resumes Run B's first session — Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the five runs execute in order A to E.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/harness-environment/agent-run-keep-failed · test harness environment · AGENT_RUN_KEEP_FAILED — The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
contract/channel-params/v · Channel params · v — Every channel params object carries the format version v.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
contract/mcp-output/stdout · MCP server output · stdout and stderr — viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.

## project-intent · ## Project Intent
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
decision/local-cargo-install · Local install with embedded plugin — viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## build-system · key of ## Infrastructure Patterns
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
decision/gate-steps-fail-closed · Gate steps fail closed — Every CI gate step runs fail-closed in bash.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/pty-seam-portable-pty · portable-pty is the PTY seam — The PTY seam is built over portable-pty, with host raw mode through libc termios.

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
