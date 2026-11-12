@row_add n=1 row=contract/event-data/question area=standard-contracts rec=1
boundary: event data
item: `question`
promise: `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`; each `question` string is a key in `answer`'s `answers`.
@row_add n=2 row=contract/event-data/question-option-details area=standard-contracts rec=1
boundary: event data
item: `question` option_details
promise: `option_details` is parallel to `options`, one per option, each field the option's string or `null`; a missing or non-string value never refuses the dialog.
@row_add n=3 row=decision/option-text-never-in-human-line area=standard-contracts rec=1
name: Upstream option text kept out of human lines
rule: Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
@row_add n=4 row=contract/event-data/permission area=standard-contracts rec=2
boundary: event data
item: `permission`
promise: `{dialog_id, tool, input}`; `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload.
@row_add n=5 row=contract/event-data/plan area=standard-contracts rec=3
boundary: event data
item: `plan`
promise: `{dialog_id, plan}`, with the plan as text.
@row_add n=6 row=contract/event-data/session-end area=standard-contracts rec=4
boundary: event data
item: `session-end`
promise: `{}`.
@row_add n=7 row=contract/event-data/activity area=standard-contracts rec=5
boundary: event data
item: `activity`
promise: `{}`, plus `tool` when the hook names one.
@row_add n=8 row=contract/event-data/link area=standard-contracts rec=6
boundary: event data
item: `link`
promise: `{driver, driven}`.
@row_add n=9 row=contract/event-data/unlink area=standard-contracts rec=6
boundary: event data
item: `unlink`
promise: `{driver, driven}`.
@row_add n=10 row=contract/event-data/wheel area=standard-contracts rec=7
boundary: event data
item: `wheel`
promise: `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
@row_add n=11 row=contract/event-data/budget-gate area=standard-contracts rec=8
boundary: event data
item: `budget-gate`
promise: `{paused, window?, override_until?}` for the instance's own gate, with `window` `five-hour` or `seven-day`.
@row_add n=12 row=contract/event-data/send-issued area=standard-contracts rec=9
boundary: event data
item: `send-issued`
promise: `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
@row_add n=13 row=contract/event-data/send-confirmed area=standard-contracts rec=10
boundary: event data
item: `send-confirmed`
promise: `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
@row_add n=14 row=contract/event-data/send-refused area=standard-contracts rec=11
boundary: event data
item: `send-refused`
promise: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
@row_add n=15 row=contract/event-data/dialog-expired area=standard-contracts rec=12,13
boundary: event data
item: `dialog-expired`
promise: `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text.
@row_add n=16 row=decision/wrapper-events-log-only area=standard-contracts rec=13
name: Wrapper-appended events are log-only
rule: Event kinds the wrapper appends (`wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
why: They are how the audit trail and a replay learn the wheel holder, the budget gate, each send's outcome and the link set.
except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
@row_add n=17 row=contract/wrapper-event-appends/wheel area=standard-contracts rec=13
boundary: wrapper event appends
item: `wheel`
promise: The wrapper appends `wheel` with `source: wrapper` once at start and on every change.
@row_add n=18 row=contract/wrapper-event-appends/budget-gate area=standard-contracts rec=13
boundary: wrapper event appends
item: `budget-gate`
promise: The wrapper appends `budget-gate` with `source: wrapper` once at start and on every change.
@row_add n=19 row=contract/wrapper-event-appends/send-records area=standard-contracts rec=13
boundary: wrapper event appends
item: `send-issued` / `send-confirmed` / `send-refused`
promise: The wrapper appends the send records with `source: wrapper` for each send.
@row_add n=20 row=contract/wrapper-event-appends/link area=standard-contracts rec=13
boundary: wrapper event appends
item: `link`
promise: The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call.
@row_add n=21 row=contract/wrapper-event-appends/unlink area=standard-contracts rec=13
boundary: wrapper event appends
item: `unlink`
promise: The driven instance's wrapper appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes.
@row_add n=22 row=contract/hook-dialog/data area=standard-contracts rec=14
boundary: `hook.dialog`
item: `data`
promise: `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`.
@row_add n=23 row=contract/hook-dialog/dialog-hooks area=standard-contracts rec=14
boundary: `hook.dialog`
item: dialog hooks
promise: Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
@row_add n=24 row=decision/dialog-logged-exactly-once area=standard-contracts rec=14
name: Every dialog logged exactly once
rule: Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once.
except: A matched continuation appends nothing, being the same dialog.
@row_add n=25 row=contract/answer-refusals/unknown-dialog area=standard-contracts rec=14
boundary: answer refusals
item: `unknown-dialog`
promise: A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`.
@row_add n=26 row=contract/cli-json-output/result area=standard-contracts rec=15
boundary: CLI `--json` output
item: channel result
promise: Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
@row_add n=27 row=contract/cli-json-output/error area=standard-contracts rec=15
boundary: CLI `--json` output
item: `error`
promise: Outcomes that have no channel `result` print an `error` document.
@row_add n=28 row=contract/cli-json-output/instance-unreachable area=standard-contracts rec=16
boundary: CLI `--json` output
item: `instance-unreachable`
promise: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
@row_add n=29 row=contract/mcp-refusal-result/instance-unreachable area=standard-contracts rec=16
boundary: MCP refusal result
item: `instance-unreachable`
promise: The MCP `isError` result for an unreachable instance is the CLI `instance-unreachable` document less `v`, its text `refused: instance-unreachable`.
@row_add n=30 row=contract/cli-json-output/wrapper-fault area=standard-contracts rec=17
boundary: CLI `--json` output
item: `wrapper-fault`
promise: `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null.
@row_add n=31 row=contract/cli-json-output/internal-error area=standard-contracts rec=18
boundary: CLI `--json` output
item: `internal-error`
promise: `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_add n=32 row=contract/cli-json-output/revive-refusal area=standard-contracts rec=19
boundary: CLI `--json` output
item: `viola revive --json` refusal
promise: A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
@row_add n=33 row=contract/cli-json-output/revive-refusal-codes area=standard-contracts rec=19
boundary: CLI `--json` output
item: `viola revive --json` refusal codes
promise: `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
@row_add n=34 row=contract/cli-json-output/revive-list area=standard-contracts rec=19
boundary: CLI `--json` output
item: `viola revive <name> --list --json`
promise: Prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order with exit 0, or the `no-session` document with exit 1 on an empty chain.
@row_add n=35 row=decision/no-document-after-child-held-terminal area=standard-contracts rec=19
name: No JSON document once the child held the terminal
rule: An error after the child has held the terminal prints no JSON document.
why: The child has held the terminal.
@row_add n=36 row=contract/cli-json-output/plugin-install area=standard-contracts rec=20
boundary: CLI `--json` output
item: `viola plugin install --json`
promise: Prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with exit 0, no path.
@row_add n=37 row=contract/cli-json-output/plugin-install-strict-modes-refused area=standard-contracts rec=20
boundary: CLI `--json` output
item: `viola plugin install --json` strict-modes refusal
promise: A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21.
@row_none n=1 rec=21 type=S
@row_add n=38 row=contract/mcp-refusal-result/shape area=standard-contracts rec=22,21
boundary: MCP refusal result
item: result shape
promise: `{"content":[{"type":"text","text":"refused: <refusal>"}],"isError":true,"structuredContent":{"refusal":"<refusal>","detail":…}}`, e.g. `budget-paused` with detail `five-hour`.
@row_add n=39 row=decision/hook-output-one-crate area=standard-contracts rec=23
name: Hook output produced by one crate
rule: The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
@row_add n=40 row=contract/hook-contract/decision area=standard-contracts rec=24
boundary: hook contract
item: a decision
promise: A decision is exit 0 with a `hookSpecificOutput` body.
@row_add n=41 row=contract/hook-contract/pre-tool-use area=standard-contracts rec=24
boundary: hook contract
item: PreToolUse
promise: PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`.
@row_add n=42 row=contract/hook-contract/permission-request area=standard-contracts rec=24
boundary: hook contract
item: PermissionRequest
promise: PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
@row_add n=43 row=contract/hook-contract/no-decision area=standard-contracts rec=25
boundary: hook contract
item: no decision
promise: No decision is exit 0 with an empty stdout.
@row_add n=44 row=contract/hook-contract/async-tier area=standard-contracts rec=25
boundary: hook contract
item: async-tier hooks
promise: Async-tier hooks always print nothing.
@row_add n=45 row=decision/statusline-hook-exempt area=standard-contracts rec=26
name: Statusline wrapper exempt from hook output rules
rule: `viola hook statusline` records the `rate_limits` reading, prints the user's statusline command's stdout unchanged, and always exits 0 with empty stderr.
why: It is the statusline wrapper, not a Claude Code hook, so the hook decision rules do not apply.
@row_add n=46 row=decision/no-shell-out area=standard-contracts rec=26
name: No shell-out
rule: Viola runs no command through a shell.
except: The user's own statusline command, read at start from one named source, run through `/bin/sh -c` with the same stdin.
@row_none n=2 rec=27 type=S
@row_add n=47 row=contract/session-liveness/heartbeat area=standard-contracts rec=28
boundary: session liveness
item: heartbeat
promise: The wrapper touches `<instance dir>/heartbeat` every 1 s.
@row_add n=48 row=contract/session-liveness/gone area=standard-contracts rec=28
boundary: session liveness
item: `gone`
promise: A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
@row_add n=49 row=contract/session-liveness/live area=standard-contracts rec=28
boundary: session liveness
item: `live`
promise: A session whose process is running and whose beat is at most 5 s old is `live`.
@row_add n=50 row=contract/session-liveness/stale area=standard-contracts rec=28
boundary: session liveness
item: `stale`
promise: A session whose process is running and whose beat is older than 5 s or absent is `stale`.
@row_add n=51 row=contract/agents-listing/status area=standard-contracts rec=29
boundary: agents listing
item: `status`
promise: `claude agents --json` adds `status` (idle/busy) to the listing.
@row_add n=52 row=contract/agents-listing/unwrapped-sessions area=standard-contracts rec=29
boundary: agents listing
item: unwrapped sessions
promise: Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_add n=53 row=decision/agents-listing-never-errors area=standard-contracts rec=29
name: Agents listing degrades, never errors
rule: When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
@row_none n=3 rec=30 type=S
@row_add n=54 row=decision/wrapper-channel-json-rpc area=standard-contracts rec=31
name: JSON-RPC over ndjson for the wrapper channel
rule: The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets.
@row_add n=55 row=decision/mcp-over-stdio area=standard-contracts rec=32,35
name: MCP over stdio only
rule: MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only.
@row_add n=56 row=absence/websocket area=standard-contracts rec=33
capability: WebSocket endpoints (v1)
@row_add n=57 row=absence/polling-endpoints area=standard-contracts rec=33
capability: Polling endpoints (v1)
@row_none n=4 rec=34 type=S
@row_add n=58 row=absence/listening-network-port area=occupied-resources rec=35
capability: A listening network port of any kind: no database port, no broker port, no listening TCP port
@row_none n=5 rec=36 type=S
@row_add n=59 row=name/other/viola-test-chan-pid-label area=occupied-resources rec=37
class: other
name: viola-test-chan-<pid>-<label>
@row_add n=60 row=decision/test-endpoints-outside-namespace area=occupied-resources rec=37
name: Test channel endpoints outside the product namespace
rule: Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`.
@row_add n=61 row=name/other/viola-h12-sock area=occupied-resources rec=38
class: other
name: viola-<h12>.sock
@row_add n=62 row=decision/socket-dir-owner-only area=occupied-resources rec=38
name: Socket in a per-user owner-only directory
rule: The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
@row_add n=63 row=decision/socket-file-owner-only area=occupied-resources rec=38
name: Socket file owner-only
rule: The socket is chmod 0600 after the bind.
@row_add n=64 row=decision/same-user-peers-only area=occupied-resources rec=38
name: Same-user peers only
rule: Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame.
@row_add n=65 row=decision/connect-to-recorded-endpoint area=occupied-resources rec=38
name: Connect to the recorded endpoint
rule: Every other process connects to the `endpoint` recorded in the instance snapshot.
@row_add n=66 row=decision/no-abstract-namespace-sockets area=occupied-resources rec=38
name: No abstract-namespace sockets
rule: Linux abstract-namespace socket names are never used.
why: They carry no filesystem permissions and any local user could connect.
@row_none n=6 rec=39 type=S
@row_add n=67 row=name/binary/viola area=occupied-resources rec=40
class: binary
name: viola
@row_add n=68 row=name/binary/viola-fake-agent area=occupied-resources rec=41
class: binary
name: viola-fake-agent
@row_add n=69 row=name/binary/viola-harness area=occupied-resources rec=41,55
class: binary
name: viola-harness
@row_add n=70 row=name/binary/viola-mcp-tap area=occupied-resources rec=41,55
class: binary
name: viola-mcp-tap
@row_add n=71 row=decision/test-binaries-never-released area=occupied-resources rec=41
name: Test-only binaries never released
rule: Test-only binaries (`viola-fake-agent`, `viola-harness`, `viola-mcp-tap`) are never in a release build.
@row_add n=72 row=name/command/run area=occupied-resources rec=42
class: command
name: run
@row_add n=73 row=name/command/revive area=occupied-resources rec=42
class: command
name: revive
@row_add n=74 row=name/command/send area=occupied-resources rec=42
class: command
name: send
@row_add n=75 row=name/command/wait area=occupied-resources rec=42
class: command
name: wait
@row_add n=76 row=name/command/last area=occupied-resources rec=42
class: command
name: last
@row_add n=77 row=name/command/list area=occupied-resources rec=42
class: command
name: list
@row_add n=78 row=name/command/answer area=occupied-resources rec=42
class: command
name: answer
@row_add n=79 row=name/command/hook area=occupied-resources rec=42
class: command
name: hook
@row_add n=80 row=name/command/mcp area=occupied-resources rec=42
class: command
name: mcp
@row_add n=81 row=name/command/verify area=occupied-resources rec=42
class: command
name: verify
@row_add n=82 row=name/command/pause area=occupied-resources rec=42
class: command
name: pause
@row_add n=83 row=name/command/release area=occupied-resources rec=42
class: command
name: release
@row_add n=84 row=name/command/link area=occupied-resources rec=42
class: command
name: link
@row_add n=85 row=name/command/unlink area=occupied-resources rec=42
class: command
name: unlink
@row_add n=86 row=name/command/plugin-install area=occupied-resources rec=42
class: command
name: plugin install
@row_add n=87 row=decision/probe-capture-arms-inert area=occupied-resources rec=43
name: Probe capture arms are inert
rule: The hidden hook `--capture` arms take an absolute, existing directory only, read no `VIOLA_*` variable, start no observability, open no channel, and always exit 0 with empty stderr.
@row_add n=88 row=name/other/hook-session-start area=occupied-resources rec=44
class: other
name: hook session-start
@row_add n=89 row=name/other/hook-user-prompt-submit area=occupied-resources rec=44
class: other
name: hook user-prompt-submit
@row_add n=90 row=name/other/hook-pre-tool-use area=occupied-resources rec=44
class: other
name: hook pre-tool-use
@row_add n=91 row=name/other/hook-permission-request area=occupied-resources rec=44
class: other
name: hook permission-request
@row_add n=92 row=name/other/hook-stop area=occupied-resources rec=44
class: other
name: hook stop
@row_add n=93 row=name/other/hook-session-end area=occupied-resources rec=44
class: other
name: hook session-end
@row_add n=94 row=name/other/hook-notification area=occupied-resources rec=44
class: other
name: hook notification
@row_add n=95 row=name/other/hook-post-tool-use area=occupied-resources rec=44
class: other
name: hook post-tool-use
@row_add n=96 row=name/other/hook-post-tool-use-failure area=occupied-resources rec=44
class: other
name: hook post-tool-use-failure
@row_add n=97 row=name/other/hook-statusline area=occupied-resources rec=44
class: other
name: hook statusline
@row_none n=7 rec=45 type=S
@row_none n=8 rec=46 type=S
@row_add n=98 row=name/other/viola area=occupied-resources rec=47,48
class: other
name: viola
@row_add n=99 row=name/server/viola area=occupied-resources rec=47,48
class: server
name: viola
@row_add n=100 row=name/other/mcp-plugin-viola-viola-send area=occupied-resources rec=47
class: other
name: mcp__plugin_viola_viola__send
@row_add n=101 row=name/other/mcp-plugin-viola-viola-wait area=occupied-resources rec=47
class: other
name: mcp__plugin_viola_viola__wait
@row_add n=102 row=name/other/mcp-plugin-viola-viola-last area=occupied-resources rec=47
class: other
name: mcp__plugin_viola_viola__last
@row_add n=103 row=name/other/mcp-plugin-viola-viola-answer area=occupied-resources rec=47
class: other
name: mcp__plugin_viola_viola__answer
@row_add n=104 row=name/other/mcp-plugin-viola-viola-list area=occupied-resources rec=47
class: other
name: mcp__plugin_viola_viola__list
@row_add n=105 row=name/other/viola-at-viola area=occupied-resources rec=48
class: other
name: viola@viola
@row_add n=106 row=decision/plugin-carries-no-hooks area=occupied-resources rec=48
name: Installed plugin carries no hooks
rule: The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks.
@row_none n=9 rec=49 type=5
@row_add n=107 row=contract/child-argv/plugin-dir area=occupied-resources rec=50
boundary: flags passed to the child
item: `--plugin-dir`
promise: `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
@row_add n=108 row=contract/child-argv/settings area=occupied-resources rec=50
boundary: flags passed to the child
item: `--settings`
promise: `--settings <instance dir>/settings.json` follows when the start wrote a settings override.
@row_add n=109 row=contract/child-argv/caller-arguments area=occupied-resources rec=50
boundary: flags passed to the child
item: caller's arguments
promise: The caller's arguments follow the viola flags.
@row_add n=110 row=contract/child-argv/resume area=occupied-resources rec=50
boundary: flags passed to the child
item: `--resume`
promise: A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
@row_add n=111 row=contract/child-argv/fork-session area=occupied-resources rec=50
boundary: flags passed to the child
item: `--fork-session`
promise: A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
@row_add n=112 row=name/other/viola-verify-probe area=occupied-resources rec=51
class: other
name: viola-verify-probe
@row_add n=113 row=contract/settings-override/status-line area=occupied-resources rec=52
boundary: per-session settings override
item: `statusLine`
promise: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn.
@row_add n=114 row=contract/settings-override/plain-path-only area=occupied-resources rec=52
boundary: per-session settings override
item: plain-character pinned path
promise: The override is written only for a pinned path of plain characters; otherwise no override is written and no `--settings` is passed.
@row_none n=10 rec=53 type=S
@row_add n=115 row=name/crate/viola area=occupied-resources rec=54
class: crate
name: viola
@row_add n=116 row=name/crate/viola-core area=occupied-resources rec=54
class: crate
name: viola-core
@row_add n=117 row=name/crate/viola-pty area=occupied-resources rec=54
class: crate
name: viola-pty
@row_add n=118 row=name/crate/viola-channel area=occupied-resources rec=54
class: crate
name: viola-channel
@row_add n=119 row=name/crate/viola-state area=occupied-resources rec=54
class: crate
name: viola-state
@row_add n=120 row=name/crate/viola-agent-claude area=occupied-resources rec=54
class: crate
name: viola-agent-claude
@row_add n=121 row=name/crate/viola-mcp area=occupied-resources rec=54
class: crate
name: viola-mcp
@row_add n=122 row=name/crate/viola-e2e area=occupied-resources rec=55
class: crate
name: viola-e2e
@row_add n=123 row=decision/e2e-noop-fake-agent-feature area=occupied-resources rec=55
name: No-op fake-agent feature in the e2e crate
rule: The test-only `viola-e2e` crate declares a no-op `fake-agent` feature.
why: So package-scoped `--features fake-agent` invocations resolve.
