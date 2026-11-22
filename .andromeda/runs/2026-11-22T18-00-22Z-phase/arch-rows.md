# architecture · the rows that bind 2026-11-22-epoch-4-cleanup — picked 196 of 983 (96 by every draw) · rule any · pin 3998

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker; automation is refused with a typed reason instead of competing with the human.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.

## stack-and-technologies · ## Stack and Technologies
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/cargo-deny-dependency-policy · cargo-deny dependency policy — cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/vt100-screen-model · vt100 as screen model — vt100 gives pre-send readiness and modal detection on run's feed thread, the screen of verify's interactive probe runs, and the screen the test-only live rig reads of its child.
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/msrv-floor · Declared MSRV floor 1.96 — The workspace declares rust-version 1.96 and CI builds against that floor without touching the toolchain pin. · why: the security toolchain floor · guard: msrv CI job
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures and the rig-only first-screen record of whole scrubbed rows.

## established-decisions · ## Established Decisions
decision/atomic-bracketed-send · Atomic bracketed-paste send — A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after. · except: A paste not finished within the compiled paste bound is cut: the writer stops after its in-flight chunk and writes the closing paste marker with no CR.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/confirm-window-compiled · Confirmation window compiled — The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp. · except: A /compact send waiting for its post-condition has its own 40 s window.
decision/every-send-confirmed · Every send is confirmed — A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted. · why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate. · except: A ledger-listed local command is confirmed by its own measured post-condition, such as a stdout echo in the session transcript, or answered ok unconfirmable where none can be read.
decision/identity-floor-ledger-row · Identity-floor ledger row and its readings — The identity-floor relied ledger row passes when every CLAUDE name the CLI adds to a SessionStart hook is an identity-floor name or a known non-identity name, and fails on an unknown name or an unread or off-shape env-names file; kept names are dropped before the check.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/paste-bound · Paste cut at a compiled paste bound — A send's paste not finished within the compiled 5 s paste bound, counted from the call, is cut: the writer stops after its in-flight chunk of at most 4096 bytes and writes the closing paste marker with no Enter; a paste still queued at the deadline is cut with nothing written. · why: So a paste never blocks past its bound, even while queued behind human keys.
decision/paste-cut-mark · Cut mark refuses sends until the human acts — A cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor; a human key read, a prompt-submitted of any origin, a session-start or a session-end clears it; a release with no key or a turn-ended leaves it.
decision/r8-identity-floor · R8 parent-identity floor — The identity-floor CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/screen-signatures-compiled · Screen signatures as compiled literals — Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row is a standing modal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char. · why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read. · except: A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/send-time-bound · Send time bound — send blocks at most the readiness gate's maximum wait plus the compiled 5 s paste bound plus the window; on the MCP surface the MCP call deadline bounds the driver's wait, never the send itself.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/trust-record-ledger-row · Trust-record ledger row and its probe — The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box. · why: Ratified as a widening of verify's child set.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, one agents --json read inside Run B and one print-mode sender, verify-sender, inside Run B's cross-session turn, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-own-trust-record · Verify writes and removes its own trust record — viola verify writes a trust record for its own temp folder only into the Claude config file its child reads, never creating an absent file, and removes that one record after. · why: Ratified as the one bounded exception to viola writing nothing into Claude's config.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks. · except: The test-only live rig in viola-e2e also uses vt100 as the screen model it reads its child's trust dialog, selector and input box from.
contract/events-ndjson/send-confirmed-order · events.ndjson · send-confirmed order — The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts; a send confirmed by a stdout echo has no such line and its submitted_at is the instant the echo was read.
contract/not-delivered-details/no-prompt-submitted · not-delivered refusal details · no-prompt-submitted — A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
contract/not-delivered-details/paste-cut · not-delivered refusal details · paste-cut — A send whose paste was cut at its bound, with that send's cursor, or refused by a standing cut mark, with no cursor, is refused `not-delivered` with this detail, exit 13.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait → `human-typing` and `turn-running` again → `paste-cut` of a standing cut mark → `paste-cut` of a paste cut at its bound.
contract/send-results/unconfirmable · send results · unconfirmable — A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
decision/browser-tools-prompt-ledger-row · Browser-tools-prompt ledger row and its readings — The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/clear-confirmed-by-new-session · /clear confirmed by a new session — /clear on a verified CLI is confirmed by a session-start of cause clear whose agent session id differs from the remembered one, and by no prompt-submitted.
decision/compact-confirm-window · /compact confirmation window — A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency. · provisional
decision/compact-confirmed-by-same-session · /compact confirmed by the same session — /compact on a verified CLI is confirmed by a session-start of cause compact whose agent session id equals the remembered one, and by no prompt-submitted.
decision/cross-session-prefix-ledger-row · Cross-session-prefix ledger row and its readings — The cross-session-prefix relied ledger row reads pass only when a print-mode sender's message to verify's Run B, started with the crossSessionInbound accept settings, arrives opening with the CLI's cross-session harness prefix, else fail, never on the sender's words; no runtime dir runs no turn.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/local-command-context-probe · Local command context ledger row and its probe — The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window; verify probes it by Run B's eighth paste, /context, and a failing read of it is fail, never absent. · why: Ratified as a widening of verify's pastes.
decision/local-command-echo-confirmation · Local command confirmed by its stdout echo — A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content, and the read is bounded by MAX_FRAME. · why: Ratified; the read opens no symlink, only a regular file owned by the own euid.
decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails. · why: Ratified as a widening of verify's pastes.
decision/local-command-post-conditions · Local commands listed with delivery post-conditions — The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands. · why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
decision/local-commands-by-ledger · Local commands classified by the ledger — A send is a local command only when its typed text is exactly a name on the ledger's local-command list, or that name, one space and a non-empty rest not starting with a space, never by a leading slash; any other slash text, skills included, is an ordinary send. · why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
decision/no-automatic-reverify-of-red · No automatic verify of an earlier red version — An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that did not verify is never verified again automatically. · why: So a red version does not loop.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/permission-request-question-ledger-row · Permission-request-question ledger row and its probe — The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option; it passes only when that request is raised and the answer reaches the tool's answers.
decision/plugin-only-in-wrapped-sessions · Plugin loads only into wrapped sessions — The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`. · why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola. · except: `viola verify`'s transient unwrapped children, all gone when verify ends.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/send-waits-out-paste-hint · Send waits out the paste hint — On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed. · why: Ratified ruling.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content. · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-transcripts-accepted-residual · Verify's CLI transcripts are an accepted residual — The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
contract/channel-results/cursor · channel results · cursor — Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
contract/not-delivered-details/input-not-ready · not-delivered refusal details · input-not-ready — A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a standing modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-refusal-codes/interrupted · viola verify process-exit refusal codes · interrupted — Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
contract/viola-verify-refusal-codes/version-unreadable · viola verify process-exit refusal codes · version-unreadable — Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.

## conventions · ## Conventions
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/ok-payload/unconfirmable · ok payload · unconfirmable — unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured, or is a stdout echo with no readable session transcript, or for any listed local command on an unverified CLI.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
contract/send-ok-payload/unconfirmable · send ok payload · unconfirmable — Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
contract/send/paste-failure-after-issue · send refusals · paste failing after send-issued — A paste that fails after send-issued is refused as input-not-ready with that send's cursor, except a paste cut at its bound, refused as paste-cut with that send's cursor.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
contract/channel-response/error · channel response · error — Reserved for protocol faults: `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}`.
contract/channel-response/ok · channel response · ok — Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/cli-exit-codes/14 · CLI exit codes · 14 — The `unknown` refusal.
contract/normalised-event-kinds/send-confirmed · normalised event kinds · send-confirmed — `send-confirmed` is a normalised event kind.
contract/normalised-event-kinds/send-issued · normalised event kinds · send-issued — `send-issued` is a normalised event kind.
contract/normalised-event-kinds/send-refused · normalised event kinds · send-refused — `send-refused` is a normalised event kind.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.

## standard-contracts · ## Standard Contracts
decision/agents-listing-never-errors · Agents listing degrades, never errors — When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
contract/board/agents-read-failure · The board (CLI list and MCP list) · agents read failure — If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
contract/channel-methods/hook-event-driver-origin · channel methods · hook.event driver origin — A `prompt-submitted` that is the in-flight `send`'s typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with `origin:"driver"`, its text kept as submitted, and only then settles that send.
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
contract/event-data/send-confirmed · event data · `send-confirmed` — `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/hook-event/driver-relabel · channel hook.event · driver relabel — A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted, and only then settles that send.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/env-probe · ledger stamps envelope · measured.env_probe — Holds read, added, unknown, other_count and unshaped from the identity-floor probe, names only, with every kept name removed.
contract/ledger-stamps/first-screen · ledger stamps envelope · measured.first_screen — For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, browser-tools, box, none or exited; no screen text.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/list-payload/agents-read-failure · list payload · agents read failure — A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
contract/list-payload/status · list payload · status — A failed `claude agents --json` read leaves every `status` `unknown`.
contract/trust-writer-words/removed · Trust writer code words · removed — The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
contract/trust-writer-words/written · Trust writer code words · written — The trust record was written into the Claude config file.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/agents-listing/status · agents listing · `status` — `claude agents --json` adds `status` (idle/busy) to the listing.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`; a finished or paused turn is for the driver to tell, and a parked `wait` never polls the file.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records, an activity record and a harness-origin prompt-submitted never wake, also after a task-free turn end.
contract/event-data/prompt-submitted-origin · event data per kind · prompt-submitted origin — `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`; the wrapper sets `driver` only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake `harness`.
contract/event-data/prompt-submitted-text · event data per kind · prompt-submitted text — `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed.
contract/event-data/send-issued · event data · `send-issued` — `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
contract/event-prompt-submitted/origin · prompt-submitted event data · origin — driver, human or harness; the hook files only human or harness; the wrapper sets driver only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake harness.
contract/event-prompt-submitted/text · prompt-submitted event data · text — The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/list-item/status · list payload item · status — idle, busy or unknown.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/trust-writer-words/key-present · Trust writer code words · key-present — The config already holds a record under the key; it is left as found and nothing is written. Only viola verify's writer gives it; a start reads it as lost.
contract/wrapper-channel-frames/ok-result · wrapper channel frames · ok result — A success is answered as `result` holding `{"ok": <payload>}`.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.

## occupied-resources · ## Occupied Resources
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/mutants-own-target-dir · Mutation runs use their own relative target dir — Mutation testing builds in its own target dir, passed to cargo-mutants as a relative path, so each copied tree builds every test binary in its own copy. · why: a copied default target keeps the original tree's binary paths, so root integration tests would drive the unmutated binary
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/harness-environment/agent-run-keep-failed · test harness environment · AGENT_RUN_KEEP_FAILED — The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
contract/harness-environment/agent-run-keep-homes · test harness environment · AGENT_RUN_KEEP_HOMES — The value 1 keeps the test and session homes for the CI gates.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording. · except: The hidden rig-only --record-first-screens keeps a failed verify's first screens as whole scrubbed rows, kept locally only.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/verify-run-e-resumes-run-b-first-session · Verify's Run E resumes Run B's first session — Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the six runs execute in order A to F.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list and viola revive do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.

## inherited-defaults · ## Inherited Defaults
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## build-system · key of ## Infrastructure Patterns
decision/workspace-dependency-pins · Third-party versions pinned in one place — Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
decision/msrv · Minimum supported Rust version 1.96 — The workspace declares rust-version 1.96, and dependency resolution prefers versions compatible with it.
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.

## project-directory-structure · key of ## Infrastructure Patterns
decision/cargo-deny-bans · cargo-deny supply-chain policy — cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features. · guard: deny-probes negative probe per ban with a clean control

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
