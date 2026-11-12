@row_add n=1 row=contract/mcp-tool-results/success area=conventions rec=1
boundary: MCP tool results
item: success
promise: A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`.
@row_add n=2 row=contract/mcp-tool-results/is-error-true area=conventions rec=1
boundary: MCP tool results
item: isError true
promise: A refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`.
@row_add n=3 row=contract/mcp-json-rpc-errors/32602 area=conventions rec=1
boundary: MCP JSON-RPC errors
item: -32602
promise: An unknown tool name is a JSON-RPC error `-32602`; JSON-RPC errors are only the MCP connection's own faults.
@row_add n=4 row=contract/mcp-json-rpc-errors/32603 area=conventions rec=1
boundary: MCP JSON-RPC errors
item: -32603
promise: An internal handler fault is a JSON-RPC error `-32603`; JSON-RPC errors are only the MCP connection's own faults.
@row_add n=5 row=decision/one-error-enum-per-crate area=conventions rec=2
name: One error enum per crate
rule: Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages.
except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
@row_add n=6 row=decision/nutype-errors-fold-into-crate-enum area=conventions rec=2
name: nutype validation errors fold into the crate enum
rule: A nutype validation error is folded into the owning crate's error enum.
@row_add n=7 row=decision/anyhow-only-in-root-bin area=conventions rec=2
name: anyhow only in the root bin crate
rule: `anyhow` appears only in the root bin crate.
@row_none n=1 rec=3 type=S
@row_add n=8 row=decision/crate-naming area=conventions rec=4
name: Crate naming
rule: Crates are named `viola-<area>`, kebab-case.
@row_add n=9 row=decision/standard-rust-naming area=conventions rec=4
name: Standard Rust naming
rule: Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
@row_add n=10 row=decision/json-field-names-snake-case area=conventions rec=5
name: JSON field names in snake_case
rule: JSON field names are snake_case in every viola format.
except: `last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them.
@row_add n=11 row=decision/wire-enum-values-kebab-case area=conventions rec=6
name: Enum string values on the wire in kebab-case
rule: Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
@row_add n=12 row=contract/normalised-event-kinds/session-start area=conventions rec=7
boundary: normalised event kinds
item: session-start
promise: `session-start` is a normalised event kind.
@row_add n=13 row=contract/normalised-event-kinds/turn-ended area=conventions rec=7
boundary: normalised event kinds
item: turn-ended
promise: `turn-ended` is a normalised event kind.
@row_add n=14 row=contract/normalised-event-kinds/prompt-submitted area=conventions rec=7
boundary: normalised event kinds
item: prompt-submitted
promise: `prompt-submitted` is a normalised event kind.
@row_add n=15 row=contract/normalised-event-kinds/question area=conventions rec=7
boundary: normalised event kinds
item: question
promise: `question` is a normalised event kind.
@row_add n=16 row=contract/normalised-event-kinds/permission area=conventions rec=7
boundary: normalised event kinds
item: permission
promise: `permission` is a normalised event kind.
@row_add n=17 row=contract/normalised-event-kinds/plan area=conventions rec=7
boundary: normalised event kinds
item: plan
promise: `plan` is a normalised event kind.
@row_add n=18 row=contract/normalised-event-kinds/session-end area=conventions rec=7
boundary: normalised event kinds
item: session-end
promise: `session-end` is a normalised event kind.
@row_add n=19 row=contract/normalised-event-kinds/activity area=conventions rec=7
boundary: normalised event kinds
item: activity
promise: `activity` is a normalised event kind.
@row_add n=20 row=contract/normalised-event-kinds/link area=conventions rec=7
boundary: normalised event kinds
item: link
promise: `link` is a normalised event kind.
@row_add n=21 row=contract/normalised-event-kinds/unlink area=conventions rec=7
boundary: normalised event kinds
item: unlink
promise: `unlink` is a normalised event kind.
@row_add n=22 row=contract/normalised-event-kinds/wheel area=conventions rec=7
boundary: normalised event kinds
item: wheel
promise: `wheel` is a normalised event kind.
@row_add n=23 row=contract/normalised-event-kinds/budget-gate area=conventions rec=7
boundary: normalised event kinds
item: budget-gate
promise: `budget-gate` is a normalised event kind.
@row_add n=24 row=contract/normalised-event-kinds/send-issued area=conventions rec=7
boundary: normalised event kinds
item: send-issued
promise: `send-issued` is a normalised event kind.
@row_add n=25 row=contract/normalised-event-kinds/send-confirmed area=conventions rec=7
boundary: normalised event kinds
item: send-confirmed
promise: `send-confirmed` is a normalised event kind.
@row_add n=26 row=contract/normalised-event-kinds/send-refused area=conventions rec=7
boundary: normalised event kinds
item: send-refused
promise: `send-refused` is a normalised event kind.
@row_add n=27 row=contract/normalised-event-kinds/dialog-expired area=conventions rec=7
boundary: normalised event kinds
item: dialog-expired
promise: `dialog-expired` is a normalised event kind.
@row_add n=28 row=decision/event-kinds-agent-neutral area=conventions rec=7
name: Event kinds added only in viola-core
rule: A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
@row_add n=29 row=contract/hook-to-kind-map/sessionstart area=conventions rec=8
boundary: Hook to kind map
item: SessionStart
promise: SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
@row_add n=30 row=contract/hook-to-kind-map/userpromptsubmit area=conventions rec=8
boundary: Hook to kind map
item: UserPromptSubmit
promise: UserPromptSubmit maps to `prompt-submitted`.
@row_add n=31 row=contract/hook-to-kind-map/pretooluse-askuserquestion area=conventions rec=8
boundary: Hook to kind map
item: PreToolUse AskUserQuestion
promise: PreToolUse `AskUserQuestion` maps to `question`.
@row_add n=32 row=contract/hook-to-kind-map/pretooluse-exitplanmode area=conventions rec=8
boundary: Hook to kind map
item: PreToolUse ExitPlanMode
promise: PreToolUse `ExitPlanMode` maps to `plan`.
@row_add n=33 row=contract/hook-to-kind-map/permissionrequest area=conventions rec=8
boundary: Hook to kind map
item: PermissionRequest
promise: PermissionRequest maps to `permission`, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
@row_add n=34 row=contract/hook-to-kind-map/stop area=conventions rec=8
boundary: Hook to kind map
item: Stop
promise: Stop maps to `turn-ended`.
@row_add n=35 row=contract/hook-to-kind-map/sessionend area=conventions rec=8
boundary: Hook to kind map
item: SessionEnd
promise: SessionEnd maps to `session-end`.
@row_add n=36 row=contract/hook-to-kind-map/notification area=conventions rec=8
boundary: Hook to kind map
item: Notification
promise: Notification maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=37 row=contract/hook-to-kind-map/posttooluse area=conventions rec=8
boundary: Hook to kind map
item: PostToolUse
promise: PostToolUse maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=38 row=contract/hook-to-kind-map/posttoolusefailure area=conventions rec=8
boundary: Hook to kind map
item: PostToolUseFailure
promise: PostToolUseFailure maps to `activity`, which is log-only and never wakes `wait`.
@row_add n=39 row=contract/hook-to-kind-map/hook-statusline area=conventions rec=8
boundary: Hook to kind map
item: hook statusline
promise: `hook statusline` emits no event.
@row_add n=40 row=decision/env-var-viola-prefix area=conventions rec=9
name: Environment variables carry the VIOLA_ prefix
rule: Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE.
except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM` and `COLORTERM`, read never set, only to decide human output's SGR bytes.
@row_add n=41 row=decision/harness-env-var-prefix area=conventions rec=9
name: Test-harness variables carry the AGENT_RUN_ prefix
rule: Test-harness-only environment variables use the `AGENT_RUN_` prefix and are never read by the `viola` binary.
@row_add n=42 row=decision/fake-agent-test-seams area=conventions rec=9
name: Fake-agent test seam variables
rule: The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds.
why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
@row_add n=43 row=decision/on-disk-file-naming area=conventions rec=10
name: On-disk file naming
rule: On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
@row_add n=44 row=decision/cli-flag-and-subcommand-style area=conventions rec=11
name: CLI flag and subcommand style
rule: CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
@row_add n=45 row=contract/cli-verbs/send area=conventions rec=11
boundary: CLI verbs
item: send
promise: `viola send <target>` takes its text from stdin or `--file`.
@row_add n=46 row=contract/cli-verbs/wait area=conventions rec=11
boundary: CLI verbs
item: wait
promise: `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
@row_add n=47 row=contract/cli-verbs/last area=conventions rec=11
boundary: CLI verbs
item: last
promise: `viola last <target>`.
@row_add n=48 row=contract/cli-verbs/answer area=conventions rec=11
boundary: CLI verbs
item: answer
promise: `viola answer <target> <dialog_id>` takes the `response` object as JSON from stdin or `--file`.
@row_add n=49 row=contract/cli-verbs/pause area=conventions rec=11
boundary: CLI verbs
item: pause
promise: `viola pause <target>`.
@row_add n=50 row=contract/cli-verbs/release area=conventions rec=11
boundary: CLI verbs
item: release
promise: `viola release <target> [--budget]`.
@row_add n=51 row=contract/cli-verbs/link area=conventions rec=11
boundary: CLI verbs
item: link
promise: `viola link <driver> <driven>`.
@row_add n=52 row=contract/cli-verbs/unlink area=conventions rec=11
boundary: CLI verbs
item: unlink
promise: `viola unlink <driver> <driven>`.
@row_add n=53 row=contract/cli-verbs/revive area=conventions rec=11
boundary: CLI verbs
item: revive
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_add n=54 row=contract/cli-verbs/from area=conventions rec=11
boundary: CLI verbs
item: from
promise: Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
@row_none n=2 rec=12 type=S
@row_add n=55 row=absence/database area=conventions rec=13
capability: A database
@row_add n=56 row=decision/viola-name-instance-key area=conventions rec=13
name: ViolaName as the instance key
rule: The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype.
why: No database, so no UUID or serial keys.
@row_add n=57 row=decision/endpoint-name-stable-hash area=conventions rec=14
name: Endpoint name from a stable hand-written hash
rule: The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden.
why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
@row_add n=58 row=decision/timestamp-field-names area=conventions rec=15
name: Timestamp field names
rule: Every event and frame carries its instant in the field `ts`; other instants use `*_at` fields.
@row_add n=59 row=decision/timestamp-format area=conventions rec=15
name: Timestamp format
rule: Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
@row_add n=60 row=decision/absent-optional-fields-omitted area=conventions rec=16
name: Absent optional fields omitted on write
rule: viola omits an absent optional field on write rather than writing `null`.
@row_add n=61 row=decision/missing-equals-null-on-read area=conventions rec=16
name: Missing and null read the same
rule: Readers treat a missing field and a `null` field the same.
@row_add n=62 row=decision/unparseable-external-values-unknown area=conventions rec=17
name: Unparseable external values become unknown
rule: An unparseable external value becomes the string `"unknown"` in viola's own records, never an error, covering `resets_at`, `used_percentage` and a missing statusline window.
@row_add n=63 row=decision/percent-newtype area=conventions rec=18
name: Percentages as a 0 to 100 newtype
rule: A percentage is the `Percent` newtype, 0 to 100, serialised as a JSON number.
except: An external budget reading's `used_percentage` is a reading kept only from 0 to 100 and otherwise the word `"unknown"`.
@row_add n=64 row=decision/ndjson-line-discipline area=conventions rec=19
name: ndjson line discipline
rule: Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
@row_add n=65 row=absence/graphical-interface area=standard-contracts rec=20
capability: A web or graphical surface
@row_add n=66 row=decision/list-without-pagination area=standard-contracts rec=21
name: list without pagination
rule: The `list` payload has no pagination; lists are returned whole.
why: Lists are personal-scale.
@row_add n=67 row=contract/list-payload/skipped area=standard-contracts rec=21
boundary: list payload
item: skipped
promise: `skipped` counts records a reader could not interpret, so the board can show them.
@row_add n=68 row=contract/list-payload/items area=standard-contracts rec=22,21
boundary: list payload
item: items
promise: Each `items` entry is `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
@row_add n=69 row=contract/list-payload/liveness area=standard-contracts rec=22,21
boundary: list payload
item: liveness
promise: An entry's `liveness` is `live` or `stale`; gone instances are omitted.
@row_add n=70 row=contract/list-payload/wrapped area=standard-contracts rec=22,21
boundary: list payload
item: wrapped
promise: `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`.
@row_add n=71 row=contract/list-payload/budget area=standard-contracts rec=22,21
boundary: list payload
item: budget
promise: The payload carries a top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`.
@row_none n=3 rec=23 type=S
@row_add n=72 row=contract/event-line/shape area=standard-contracts rec=24,23
boundary: ndjson event line
item: shape
promise: Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
@row_add n=73 row=contract/event-line/source area=standard-contracts rec=25,23
boundary: ndjson event line
item: source
promise: `source` is one of `hook` · `wrapper` · `cli`.
@row_add n=74 row=contract/event-line/data area=standard-contracts rec=25,23
boundary: ndjson event line
item: data
promise: `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
@row_add n=75 row=contract/snapshot-envelope/shape area=standard-contracts rec=26
boundary: snapshot envelope
item: shape
promise: A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
@row_add n=76 row=contract/snapshot-envelope/unreadable-snapshot area=standard-contracts rec=26
boundary: snapshot envelope
item: unreadable snapshot
promise: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
@row_none n=4 rec=27 type=4
relied: A replayed snapshot carries no endpoint, pid, start time or statusline command, so its readers treat the instance as unreachable and the collision check judges liveness from the heartbeat alone.
@row_add n=77 row=contract/ledger-stamps/writer area=standard-contracts rec=28
boundary: ledger stamps envelope
item: writer
promise: `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`.
@row_add n=78 row=contract/ledger-stamps/content area=standard-contracts rec=28
boundary: ledger stamps envelope
item: content
promise: Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
@row_add n=79 row=contract/ledger-stamps/verified-version area=standard-contracts rec=28
boundary: ledger stamps envelope
item: verified version
promise: A CLI version is verified only when every relied ledger row reads `pass`.
@row_add n=80 row=contract/ledger-stamps/malformed-envelope area=standard-contracts rec=28
boundary: ledger stamps envelope
item: malformed envelope
promise: A malformed envelope makes `run`'s gate record `cli_verified:false`.
@row_add n=81 row=contract/instance-snapshot/data area=standard-contracts rec=29
boundary: instance snapshot
item: data
promise: `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
@row_add n=82 row=contract/instance-snapshot/writer area=standard-contracts rec=29
boundary: instance snapshot
item: writer
promise: `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
@row_add n=83 row=contract/instance-snapshot/started-at area=standard-contracts rec=29
boundary: instance snapshot
item: started_at
promise: `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
@row_add n=84 row=contract/instance-snapshot/endpoint area=standard-contracts rec=29
boundary: instance snapshot
item: endpoint
promise: `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
@row_add n=85 row=contract/instance-snapshot/spawn-order area=standard-contracts rec=29
boundary: instance snapshot
item: spawn order
promise: The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
@row_add n=86 row=contract/instance-snapshot/pid-and-started-at area=standard-contracts rec=30,29
boundary: instance snapshot
item: pid and started_at
promise: `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
@row_add n=87 row=contract/instance-snapshot/child-pid area=standard-contracts rec=31,29
boundary: instance snapshot
item: child_pid
promise: `child_pid` is the pid of the spawned agent child and exists only to join `claude agents --json` rows to this instance.
@row_add n=88 row=contract/instance-snapshot/agent-session-id area=standard-contracts rec=31,29
boundary: instance snapshot
item: agent_session_id
promise: `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
@row_add n=89 row=contract/instance-snapshot/cwd area=standard-contracts rec=32,29
boundary: instance snapshot
item: cwd
promise: `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it only as the revived child's spawn directory.
@row_add n=90 row=decision/spawn-directory-confined-to-snapshot area=standard-contracts rec=32,29
name: Spawn directory confined to the snapshot
rule: The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout.
@row_add n=91 row=contract/instance-snapshot/statusline-command area=standard-contracts rec=33,29
boundary: instance snapshot
item: statusline_command
promise: `statusline_command` is the user's statusline command that `run` read from the named source at start, omitted when the source gave none; no event carries it, and `hook statusline` reads it and never changes the snapshot.
@row_add n=92 row=contract/instance-snapshot/cli-version area=standard-contracts rec=34,29
boundary: instance snapshot
item: cli_version
promise: `cli_version` comes from `run`'s version gate.
@row_add n=93 row=contract/instance-snapshot/cli-verified area=standard-contracts rec=34,29
boundary: instance snapshot
item: cli_verified
promise: `cli_verified` comes from `run`'s version gate.
@row_add n=94 row=contract/instance-snapshot/pending-dialog area=standard-contracts rec=35,29
boundary: instance snapshot
item: pending_dialog
promise: `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
@row_add n=95 row=contract/list-payload/dialog-pending area=standard-contracts rec=35
boundary: list payload
item: dialog_pending
promise: `dialog_pending` comes only from the snapshot's `pending_dialog`, so a reader that falls back to replay reports `dialog_pending: false`.
@row_add n=96 row=contract/instance-snapshot/links area=standard-contracts rec=36,29
boundary: instance snapshot
item: links
promise: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, revived included, and a failed append leaves it as it was.
@row_none n=5 rec=37 type=S
@row_add n=97 row=contract/wrapper-channel-frames/envelope area=standard-contracts rec=38,37
boundary: wrapper channel frames
item: envelope
promise: Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
@row_add n=98 row=contract/wrapper-channel-frames/ok-result area=standard-contracts rec=38,37
boundary: wrapper channel frames
item: ok result
promise: A success is answered as `result` holding `{"ok": <payload>}`.
@row_add n=99 row=contract/wrapper-channel-frames/refusal-result area=standard-contracts rec=38,37
boundary: wrapper channel frames
item: refusal result
promise: A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
@row_add n=100 row=contract/wrapper-channel-frames/unsupported-protocol-version area=standard-contracts rec=38,37
boundary: wrapper channel frames
item: unsupported protocol version
promise: An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
@row_add n=101 row=contract/wrapper-channel-frames/max-frame area=standard-contracts rec=39
boundary: wrapper channel frames
item: MAX_FRAME
promise: A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.
@row_add n=102 row=contract/wrapper-channel-frames/id-less-frames area=standard-contracts rec=39
boundary: wrapper channel frames
item: id-less frames
promise: Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
@row_add n=103 row=contract/wrapper-channel-frames/conn area=standard-contracts rec=39
boundary: wrapper channel frames
item: conn
promise: A client may add `conn` to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
@row_add n=104 row=contract/channel-methods/v-and-sender area=standard-contracts rec=40
boundary: channel methods
item: v and sender
promise: Every channel method's `params` carries `v` and `sender`.
@row_add n=105 row=contract/channel-methods/from area=standard-contracts rec=40
boundary: channel methods
item: from
promise: `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
@row_add n=106 row=contract/channel-methods/send area=standard-contracts rec=41,40
boundary: channel methods
item: send
promise: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
@row_add n=107 row=contract/channel-methods/send-budget-paused area=standard-contracts rec=41,40
boundary: channel methods
item: send budget-paused
promise: A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.
@row_add n=108 row=contract/channel-methods/send-from-link area=standard-contracts rec=41,40
boundary: channel methods
item: send from link
promise: A `send` whose `from` names another instance records the link before `send-issued`.
@row_add n=109 row=contract/channel-methods/wait area=standard-contracts rec=42,40
boundary: channel methods
item: wait
promise: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
@row_add n=110 row=contract/channel-methods/wait-pending-dialog area=standard-contracts rec=42,40
boundary: channel methods
item: wait pending dialog
promise: For `wait`, a pending dialog comes first unless `after` is past its line.
@row_add n=111 row=contract/channel-methods/wait-wake-kinds area=standard-contracts rec=42,40
boundary: channel methods
item: wait wake kinds
promise: Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file.
@row_add n=112 row=contract/channel-methods/last area=standard-contracts rec=43,40
boundary: channel methods
item: last
promise: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
@row_add n=113 row=contract/channel-methods/last-after-seen-line area=standard-contracts rec=43,40
boundary: channel methods
item: last after a seen line
promise: A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
@row_add n=114 row=contract/channel-methods/answer area=standard-contracts rec=44,40
boundary: channel methods
item: answer
promise: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind.
@row_add n=115 row=contract/answer-response/question area=standard-contracts rec=44
boundary: answer response
item: question
promise: A `question` response is `{answers:{<question>:<text>}, annotations?}`.
@row_add n=116 row=contract/answer-response/permission area=standard-contracts rec=44
boundary: answer response
item: permission
promise: A `permission` response is `{behavior:"allow"|"deny", message?}`, with no suggestion slot.
@row_add n=117 row=decision/permission-answer-without-suggestions area=standard-contracts rec=44
name: Permission answers without suggestions
rule: A permission answer has no suggestion slot, so an offered `permission_suggestions` entry cannot be picked.
why: A deliberate v1 limit.
@row_add n=118 row=contract/answer-response/plan area=standard-contracts rec=44
boundary: answer response
item: plan
promise: A `plan` response is `{behavior:"approve"|"revise", message?}`.
@row_add n=119 row=contract/channel-methods/answer-from-link area=standard-contracts rec=44,40
boundary: channel methods
item: answer from link
promise: An `answer` whose `from` names another instance records the link once every refusal has passed and its kind matched, before the answer is taken; a link that cannot be recorded answers `-32603` and the dialog stays held; others record nothing.
@row_add n=120 row=contract/channel-methods/pause area=standard-contracts rec=45,40
boundary: channel methods
item: pause
promise: `pause` `{from?}` answers `{wheel:"human"}`, the wheel moving to `human` / `manual-pause`.
@row_add n=121 row=contract/channel-methods/release area=standard-contracts rec=46,40
boundary: channel methods
item: release
promise: `release` `{budget?:bool, from?}` answers `{wheel:<holder>, budget_paused:bool, override_until?}`.
@row_add n=122 row=contract/channel-methods/release-from-driver area=standard-contracts rec=46,40
boundary: channel methods
item: release from driver
promise: A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
@row_add n=123 row=contract/channel-methods/release-type-check area=standard-contracts rec=46,40
boundary: channel methods
item: release type check
promise: A `release` whose `from` is of a non-string type, or whose `budget` is not a bool (`null` included), is `-32602` with `data: null`; this check comes first.
@row_add n=124 row=contract/channel-methods/release-budget-true area=standard-contracts rec=46,40
boundary: channel methods
item: release budget true
promise: `budget:true` leaves the wheel and lifts the gate: `{wheel, budget_paused:false, override_until:<the crossed window's resets_at>}` when a pause was lifted, `{wheel, budget_paused:false}` and nothing recorded when none stood.
@row_add n=125 row=contract/channel-methods/plain-release area=standard-contracts rec=46,40
boundary: channel methods
item: plain release
promise: A `release` with no `budget` or `budget:false` lifts no pause and answers `budget_paused` as the gate's last recorded state; a `release` on a driver-held wheel changes nothing.
@row_add n=126 row=contract/channel-methods/wheel-move-durability area=standard-contracts rec=47,40
boundary: channel methods
item: wheel move durability
promise: `pause` and `release` are answered only after the move's `wheel` record has landed, `-32603` if it did not.
@row_add n=127 row=contract/channel-methods/budget-release-durability area=standard-contracts rec=47,40
boundary: channel methods
item: budget release durability
promise: A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed; else `-32603` and the pause stands.
@row_add n=128 row=contract/channel-methods/link area=standard-contracts rec=48,40
boundary: channel methods
item: link
promise: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper.
@row_add n=129 row=contract/channel-methods/unlink area=standard-contracts rec=48,40
boundary: channel methods
item: unlink
promise: `unlink` `{driver, driven}` answers `{}`, sent to the driven instance's wrapper.
@row_add n=130 row=contract/channel-methods/link-params-check area=standard-contracts rec=48,40
boundary: channel methods
item: link params check
promise: For `link` and `unlink`, a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
@row_add n=131 row=contract/channel-methods/hook-event area=standard-contracts rec=49,40
boundary: channel methods
item: hook.event
promise: `hook.event` is sent id-less by `hook`, a notification with no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
@row_add n=132 row=contract/channel-methods/hook-event-driver-origin area=standard-contracts rec=49,40
boundary: channel methods
item: hook.event driver origin
promise: A `prompt-submitted` equal to the in-flight `send`'s typed text is appended with `origin:"driver"`, and only then settles that send.
@row_add n=133 row=contract/channel-methods/hook-dialog area=standard-contracts rec=50,40
boundary: channel methods
item: hook.dialog
promise: `hook.dialog` `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
@row_add n=134 row=contract/channel-methods/hook-dialog-unverified-cli area=standard-contracts rec=50,40
boundary: channel methods
item: hook.dialog unverified CLI
promise: On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
@row_add n=135 row=contract/channel-methods/hook-dialog-continuation area=standard-contracts rec=50,40
boundary: channel methods
item: hook.dialog continuation
promise: A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise, otherwise `null`.
@row_add n=136 row=decision/list-reads-state-directly area=standard-contracts rec=51
name: list reads state without a wrapper
rule: `list` is not a channel method: the calling process reads the instances and budget files itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
@row_add n=137 row=contract/list-payload/ok area=standard-contracts rec=51
boundary: list payload
item: ok
promise: The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, budget, skipped}`.
@row_add n=138 row=contract/list-payload/status area=standard-contracts rec=51
boundary: list payload
item: status
promise: A failed `claude agents --json` read leaves every `status` `unknown`.
@row_add n=139 row=decision/upstream-text-is-content area=standard-contracts rec=52
name: Upstream text is content
rule: Upstream text carried in event `data` is content, never commands.
@row_add n=140 row=contract/event-data/session-start area=standard-contracts rec=53,52
boundary: event data per kind
item: session-start
promise: `session-start` data is `{cause, agent_session_id}`.
@row_add n=141 row=contract/event-data/prompt-submitted-text area=standard-contracts rec=54,52
boundary: event data per kind
item: prompt-submitted text
promise: `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed.
@row_add n=142 row=contract/event-data/prompt-submitted-origin area=standard-contracts rec=54,52
boundary: event data per kind
item: prompt-submitted origin
promise: `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_add n=143 row=contract/event-data/turn-ended-last-assistant-message area=standard-contracts rec=55,52
boundary: event data per kind
item: turn-ended last_assistant_message
promise: `turn-ended` data is `{last_assistant_message, background_tasks}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
@row_add n=144 row=contract/event-data/turn-ended-background-tasks area=standard-contracts rec=55,52
boundary: event data per kind
item: turn-ended background_tasks
promise: `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
