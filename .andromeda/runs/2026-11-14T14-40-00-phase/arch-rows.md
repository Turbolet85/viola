# architecture · the rows that bind 2026-11-14-budget-gate-removed — picked 180 of 994 (119 by every draw) · rule any · pin 1142

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.

## stack-and-technologies · ## Stack and Technologies
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), Percent (0 to 100), DialogHold (whole seconds 60 to 3600).
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.

## established-decisions · ## Established Decisions
decision/budget-default-thresholds · Budget default thresholds — The defaults are `five_hour` at or above 90 % and `seven_day` at or above 85 %, read once at the wrapper's start, and no value switches the gate off.
decision/budget-gate-fail-open · Budget gate fails open — No reading, an unknown figure, a passed or unparseable `resets_at`, an unreadable `budget.json` or a bad threshold leaves the gate open or at its default.
decision/budget-gate-fails-open · Budget gate never polls and fails open — The budget gate never polls and fails open.
decision/budget-gate-never-polls · Budget gate evaluated at three points — The wrapper evaluates the budget gate only at start, when a `send` arrives and at each `turn-ended`, never by polling, so a snapshot can read paused after the reset until the next point.
decision/budget-governor-soft-stop · Budget governor soft stop — Once a configured threshold is crossed, automated `send` is refused `budget-paused`; human keys are never blocked, a running turn finishes, and the pause lifts when `resets_at` passes or on `viola release --budget`. · why: The per-model weekly window is hidden, so thresholds are set low and the governor stays visible and conservative.
decision/budget-pause-gates-only-send · Budget pause gates only send — `budget-paused` gates only `send`, never `hook.dialog` or `answer`. · why: A running turn finishes.
decision/budget-pause-lift · Budget pause lifts on reset or release — A budget pause lifts when `resets_at` passes or on `viola release --budget`.
decision/budget-readings-last-writer-wins · Budget readings last-writer-wins — Budget readings are timestamped and last-writer-wins under File::lock.
decision/budget-soft-stop · Budget soft stop at the turn boundary — Once a configured threshold is crossed (defaults `five_hour` at 90 %, `seven_day` at 85 %), automated `send` is refused `budget-paused`; human keys are never blocked and a running turn finishes. · why: The per-model weekly window is hidden, so thresholds are set low.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/driver-release-deterred-not-enforced · Release from a driver deterred, not enforced — A `release` whose params carry a string `from` is refused, which deters a driver; a driver can still run CLI `viola release` outside a wrapped session, because v1 trusts every process of the same OS user. · why: `from` is self-reported, so the refusal deters rather than enforces.
decision/human-controls-cli-only · Human wheel and link controls are CLI-only — `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface. · why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
decision/only-send-reads-budget-gate · Only send reads the budget gate — `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the budget gate. · guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/release-budget-per-instance · Budget release per instance — `release --budget` lifts the pause for one instance only, until the crossed window's `resets_at`, and leaves the wheel; a plain `release` never lifts a budget pause, and an override never outlives its wrapper.
decision/release-cli-only · release is CLI-only — `release` is never an MCP tool; only the CLI offers it. · why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, evaluates the budget gate, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/tolerant-resets-at · External resets_at parsed tolerantly — An external `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error. · why: External clocks must never break parsing.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/channel-release/release-from-driver · Wrapper channel release method · release-from-driver — A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
contract/list-json/skipped · viola list --json · skipped — skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
contract/mcp-tools/list · MCP tools · list — One of the five MCP tools; takes `{}` and shows the driver the sessions.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail.
decision/byte-source-wheel-claim · Byte-source wheel claim — Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`. · why: `viola run` owns the real stdin, so it knows byte provenance exactly.
decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
decision/wheel-returns-only-through-release · Wheel returns only through CLI release — CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
contract/channel-wait/driver-relevant-events · channel wait · driver-relevant events — wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
contract/mcp-tool-result/refusal · MCP tool results · refusal — A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
contract/mcp-tools/refusal · MCP tools · refusal — A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
contract/send-refusals/empty-text · send refusals · empty-text — A send whose typed text is empty is refused not-delivered / empty-text (exit 13) and never issued: nothing reserved, typed or logged as a send, and no wheel read.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.

## conventions · ## Conventions
decision/event-kinds-agent-neutral · Event kinds added only in viola-core — A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/mcp-tools-list-held-to-recorded-exchange · MCP tools/list held to a recorded claude exchange — The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order. · guard: mcp_recorded_exchange (viola-e2e)
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error, covering `resets_at`, `used_percentage` and a missing statusline window. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error.
contract/channel-fault-codes/-32602 · channel protocol fault codes · -32602 — Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/cli-exit-codes/11 · CLI exit codes · 11 — The `budget-paused` refusal.
contract/cli-verbs/release · CLI verbs · release — `viola release <target> [--budget]`.
contract/mcp-tool-results/success · MCP tool results · success — A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, budget, skipped}` payload.
contract/normalised-event-kinds/budget-gate · normalised event kinds · budget-gate — `budget-gate` is a normalised event kind.
contract/refusal-order/first-only · Refusal order · several refusals — When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, budget-paused, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
contract/refusal-reason/budget-paused · RefusalReason · budget-paused — Serialised kebab-case; its detail is five-hour or seven-day.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-verbs/from · CLI verbs · from — Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — `hook statusline` emits no event.
contract/refusal-reason/unknown · RefusalReason · `unknown` refusal reason — The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
contract/refusal-reason/unknown-catch-all · RefusalReason · unknown catch-all — The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.

## standard-contracts · ## Standard Contracts
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances and budget files itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
decision/statusline-hook-exempt · Statusline wrapper exempt from hook output rules — `viola hook statusline` records the `rate_limits` reading, prints the user's statusline command's stdout unchanged, and always exits 0 with empty stderr. · why: It is the statusline wrapper, not a Claude Code hook, so the hook decision rules do not apply.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, the budget gate, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/channel-methods/budget-release-durability · channel methods · budget release durability — A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed; else `-32603` and the pause stands.
contract/channel-methods/plain-release · channel methods · plain release — A `release` with no `budget` or `budget:false` lifts no pause and answers `budget_paused` as the gate's last recorded state; a `release` on a driver-held wheel changes nothing.
contract/channel-methods/release · channel methods · release — A request: `release` `{budget?:bool, from?}` answers `{wheel:<holder>, budget_paused:bool, override_until?}`.
contract/channel-methods/release-budget-true · channel methods · release budget true — `budget:true` leaves the wheel and lifts the gate: `{wheel, budget_paused:false, override_until:<the crossed window's resets_at>}` when a pause was lifted, `{wheel, budget_paused:false}` and nothing recorded when none stood.
contract/channel-methods/release-type-check · channel methods · release type check — A `release` whose `from` is of a non-string type, or whose `budget` is not a bool (`null` included), is `-32602` with `data: null`; this check comes first.
contract/channel-methods/send-budget-paused · channel methods · send budget-paused — A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.
contract/channel-send/budget-paused · channel send method · budget-paused — A closed budget gate read at arrival refuses with budget-paused, detail five-hour or seven-day, before send-issued, with no cursor, nothing typed and no flight reserved.
contract/event-data/budget-gate · event data · `budget-gate` — `{paused, window?, override_until?}` for the instance's own gate, with `window` `five-hour` or `seven-day`.
contract/hook-statusline/capture · viola hook statusline · --capture <DIR> — With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed; without it the pass-through is unchanged.
contract/hook-statusline/refused-check · viola hook statusline · refused instance check — A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail".
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`.
contract/list-item/budget-paused · list payload item · budget_paused — The instance's own budget gate from its snapshot; false after a release --budget override, absent for an unwrapped session.
contract/list-item/wrapped · list payload item · wrapped — false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, budget_paused, cli_version or cli_verified.
contract/list-payload/budget · list payload · budget — A top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`; each window is `{used_percentage, resets_at}`, a field `"unknown"` when unparseable and a window `"unknown"` when omitted; `paused` only says the newest reading crosses a threshold.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, budget, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/mcp-refusal-result/shape · MCP refusal result · result shape — `{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,"structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}`
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-event-appends/budget-gate · wrapper event appends · `budget-gate` — The wrapper appends `budget-gate` with `source: wrapper` once at start and on every change.
contract/channel-methods/release-from-driver · channel methods · release from driver — A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
contract/channel-methods/send-from-link · channel methods · send from link — A `send` whose `from` names another instance records the link before `send-issued`.
contract/channel-methods/wheel-move-durability · channel methods · wheel move durability — `pause` and `release` are answered only after the move's `wheel` record has landed, `-32603` if it did not.
contract/channel-send/from · channel send method · from — A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
contract/hook-statusline/exit · viola hook statusline · exit status — Always exits 0 with empty stderr.
contract/hook-statusline/stdout · viola hook statusline · stdout — Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
contract/list-payload/skipped · list payload · skipped — `skipped` counts records a reader could not interpret, so the board can show them.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.

## occupied-resources · ## Occupied Resources
decision/budget-json-single-writer · budget.json written only by the statusline hook — budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline and only when the payload holds a rate_limits object.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value switches the budget gate off or makes the dialog hold zero or unbounded.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value switches the budget gate off or makes the dialog hold zero or unbounded
decision/config-keys-match-schema · Config keys equal the config schema — The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties. · guard: config_keys_are_the_config_schema_properties
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: The budget read, mcp's start and plugin install keep the write rule on the home.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-statusline-check-no-trace · Refused statusline check leaves no trace — A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched. · why: Ratified as the founder's own answer.
decision/shared-budget-reading · One shared budget reading — The newest rate_limits reading across wrapped sessions is kept timestamped, last writer wins, replaced whole under a lock, written only by hook statusline; every failed read of it counts as no reading.
contract/budget-json/five-hour · budget.json · five_hour — The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
contract/budget-json/read-at · budget.json · read_at — An RFC 3339 UTC timestamp with milliseconds; read as "unknown" when absent or no string.
contract/budget-json/seven-day · budget.json · seven_day — The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
contract/budget-json/v · budget.json · v — Must be 1, or the file gives no reading.
contract/config-json/five-hour · config.json · five_hour — The five-hour budget threshold, a JSON whole number from 0 to 100, default 90; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
contract/config-json/seven-day · config.json · seven_day — The seven-day budget threshold, a JSON whole number from 0 to 100, default 85; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 7 s WITHIN bound of the watch support.
contract/channel-response-detail/invalid-params · channel-response detail, a closed enum · invalid-params — Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
contract/channel-response-detail/release-from-driver · channel-response detail, a closed enum · release-from-driver — Written at WARN on both the server's and the client's line.
contract/diag-line/state-recovered · diagnostics line · state-recovered — A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/frame-verb-exit-codes/21 · send, wait, last, answer, pause and release exit codes · 21 — The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
contract/config-json/unknown-keys · config.json · unknown keys — config.json is parsed tolerantly: unknown keys are skipped and counted.
decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/channel-params/sender-version · Channel params · sender version — Every channel params object carries the sender's CARGO_PKG_VERSION.
contract/channel-params/v · Channel params · v — Every channel params object carries the format version v.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
contract/hook-statusline-output/stdout · Hook statusline output · stdout — The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.

## project-intent · ## Project Intent
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.

## build-system · key of ## Infrastructure Patterns
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
decision/lint-bans-proven · Every lint ban proven to fire — Every lint ban is proven to fire and every control proven to pass. · guard: scripts/lint-probes.sh
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/coverage-floors · Coverage floors — The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest. · guard: gate --require coverage,doctest
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
decision/fuzz-replay-ci · Fuzz corpus replayed in CI — A CI job replays the fuzz corpus on every push and pull request. · guard: gate --require fuzz-replay
decision/gate-steps-fail-closed · Gate steps fail closed — Every CI gate step runs fail-closed in bash.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.

## project-directory-structure · key of ## Infrastructure Patterns
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, statusline_command, pid, started_at or links.
decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes

# architecture · its absences, names and overviews, whole

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
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
name/other/hook-statusline — hook statusline
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
