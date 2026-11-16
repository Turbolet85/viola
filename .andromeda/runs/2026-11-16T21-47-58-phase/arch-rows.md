# architecture · the rows that bind 2026-11-16-scheduled-wake-count — picked 154 of 947 (73 by every draw) · rule any · pin 2090

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.

## stack-and-technologies · ## Stack and Technologies
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/frame-params-redacted · Frame params never rendered in Debug — A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
decision/mcp-server-rmcp · rmcp is the MCP server — The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.

## established-decisions · ## Established Decisions
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/harness-prompt-prefixes · Harness prompts classified by raw prefix — A prompt whose raw start matches a compiled harness prefix (<agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message) is filed harness; an escaped tag is typed text and never classifies harness. · why: Ratified; the escaped cross-session prefix rests on a relayed measurement that the CLI's own injection arrives escaped. · except: <\cross-session-message is a harness prefix, so a human who types that tag at a prompt's start is filed harness too.
decision/harness-prompts-not-human · Harness prompts are not a human take — A prompt whose raw start is one of the compiled harness prefixes is filed as a harness turn and never moves the wheel. · except: A human who types such a tag at a prompt's start is filed `harness`, an accepted side effect.
decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arm, called only by `viola verify`'s probes, reads no `VIOLA_*` and opens no channel, still exiting 0 with empty stderr.
decision/human-prompt-confirms-wheel-take · Human prompt confirms the wheel take — An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it.
decision/list-is-caller-side-read · list is a caller-side read — `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/prompt-normalise-order · Classify before unwrapping and un-escaping — Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /. · why: So a typed pair is never unwrapped and delivery matching compares plain text.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
decision/wait-blocks-nothing-polls · wait blocks, nothing polls — wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
contract/channel-last/last-assistant-message · channel last · last_assistant_message — last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
contract/channel-results/cursor · channel results · cursor — Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
contract/channel-wait/after · channel wait · after — after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
contract/channel-wait/driver-relevant-events · channel wait · driver-relevant events — wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
contract/channel-wait/pending-dialog · channel wait · pending dialog — A dialog pending when wait is called without after, or with after at or before its event line, is returned at once; with after past that line wait blocks for the next event.
contract/channel-wait/timeout-ms · channel wait · timeout_ms — On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
decision/byte-source-wheel-claim · Byte-source wheel claim — Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`. · why: `viola run` owns the real stdin, so it knows byte provenance exactly.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/frame-version-safety-net · Per-frame version as second safety net — Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
decision/human-wheel-renders-dialogs · Human-held wheel renders dialogs at once — While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once. · why: A dialog held by the hook is never hidden from the human for the rest of the hold.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/self-describing-frames-no-handshake · Self-describing frames, no handshake — Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. · why: Mixed binary versions are a measured fact, so every frame self-describes.
decision/spine-deadline · Spine deadline of 1 s — Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event. · guard: the perf gate (test-plan §10)
decision/spine-hook-loses-event-not-turn · Spine hook loses the event, never delays the turn — A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost. · why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
decision/submitted-form-prompt-match · Prompt match in the submitted form after agent normalisation — The typed text matches prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and frame and reverses tag escaping, when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF. · why: The CLI trims whitespace at submit; the trim set is an assumption not yet read live, and trimming at either end is held as the candidate.
decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
decision/torn-last-line-healed · Torn log line healed by the next append — Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
contract/hook-exit/2 · Hook exit and body · exit 2 — Never produced.
contract/mcp-tools/list · MCP tools · list — One of the five MCP tools; takes `{}` and shows the driver the sessions.
contract/mcp-tools/wait · MCP tools · wait — One of the five MCP tools; takes `{target, after?, timeout_ms?}`.

## conventions · ## Conventions
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/event-kinds-agent-neutral · Event kinds added only in viola-core — A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/standard-rust-naming · Standard Rust naming — Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}, transcript?}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/hook-to-kind-map/stop · Hook to kind map · Stop — Stop maps to `turn-ended`.
contract/hook-to-kind-map/stopfailure · Hook to kind map · StopFailure — StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
contract/mcp-tool-results/success · MCP tool results · success — A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, skipped}` payload.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
contract/channel-response/ok · channel response · ok — Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
contract/cli-exit-codes/10 · CLI exit codes · 10 — The `human-typing` refusal.
contract/cli-verbs/last · CLI verbs · last — `viola last <target>`.
contract/cli-verbs/wait · CLI verbs · wait — `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
contract/hook-to-kind-map/userpromptsubmit · Hook to kind map · UserPromptSubmit — UserPromptSubmit maps to `prompt-submitted`.
contract/mcp-json-rpc-errors/32603 · MCP JSON-RPC errors · -32603 — An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is `-32603` `internal error` with `data: null`.
contract/normalised-event-kinds/prompt-submitted · normalised event kinds · prompt-submitted — `prompt-submitted` is a normalised event kind.
contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.

## standard-contracts · ## Standard Contracts
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default.
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
contract/channel-methods/last · channel methods · last — A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
contract/channel-methods/wait · channel methods · wait — A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks`; telling a finished turn from a paused one is the driver's, and a parked `wait` never polls the file.
contract/channel-wait/result · channel wait method · result — {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records, an activity record and a harness-origin prompt-submitted never wake, also after a task-free turn end.
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
contract/event-data/turn-ended-background-tasks · event data per kind · turn-ended background_tasks — `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, cause?}` and carries no scheduled-wake count, so a driver cannot learn from the turn end that the CLI has a wake scheduled; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`; the replay writes no file.
decision/hook-output-one-crate · Hook output produced by one crate — The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
contract/channel-methods/hook-event-driver-origin · channel methods · hook.event driver origin — A `prompt-submitted` that is the in-flight `send`'s typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with `origin:"driver"`, its text kept as submitted, and only then settles that send.
contract/channel-methods/last-after-seen-line · channel methods · last after a seen line — A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
contract/channel-methods/wait-pending-dialog · channel methods · wait pending dialog — For `wait`, a pending dialog comes first unless `after` is past its line.
contract/channel-wait/start-offset · channel wait method · start offset — The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
contract/event-data/prompt-submitted-origin · event data per kind · prompt-submitted origin — `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.
contract/event-data/wheel · event data · `wheel` — `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-prompt-submitted/origin · prompt-submitted event data · origin — driver, human or harness; the hook files only human or harness, and driver is set only by the wrapper's relabel of the in-flight send's own prompt.
contract/hook-contract/no-decision · hook contract · no decision — No decision is exit 0 with an empty stdout.
contract/hook-event/driver-relabel · channel hook.event · driver relabel — A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted, and only then settles that send.
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}, transcript?}.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail".
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?}`.
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.

## occupied-resources · ## Occupied Resources
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/test-sockets-under-short-tmp-root · Test channel sockets under a short /tmp root — Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint socket is not under that root. · why: So the socket path length bound holds whatever the length of TMPDIR.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/channel-response-detail/unsupported-version · channel-response detail, a closed enum · unsupported-version — Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/diagnostics-role-files/mcp · home-level diagnostics role files · mcp.ndjson — viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.

## build-system · key of ## Infrastructure Patterns
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth

## ci-cd-approach · key of ## Infrastructure Patterns
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.

## project-directory-structure · key of ## Infrastructure Patterns
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
name/other/hook-stop-failure — hook stop-failure
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
