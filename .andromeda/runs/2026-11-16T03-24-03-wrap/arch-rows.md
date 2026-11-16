# architecture · the rows 2026-11-16-wait-wake-condition touches — picked 21 of 943 · rule any · pin 1764

## standard-contracts · ## Standard Contracts
decision/agents-listing-never-errors · Agents listing degrades, never errors — When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error. · r1
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release · r1
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison. · r1
decision/dialog-logged-exactly-once · Every dialog logged exactly once — Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once. · except: A matched continuation appends nothing, being the same dialog. · r1
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass. · r1
decision/hook-fails-open-on-replayed-snapshot · Hook fails open on a replayed snapshot — viola hook reads a replayed or absent snapshot as unreachable and fails open. · r1
decision/hook-output-one-crate · Hook output produced by one crate — The Claude Code hook contract is produced only by the `viola-agent-claude` crate. · r1
decision/hyprctl-direct-spawn · hyprctl spawned directly with a closed argv — hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is the Lua form hl.dsp.focus whose only inserted value is a closed window address or an i32 workspace id, never reply text. · why: Hyprland 0.56.2 runs a dispatch's arguments as Lua, so only the measured hl.dsp.focus form is accepted. · r1
decision/link-set-changes-only-on-link-calls · Link set changes only on implicit links and link calls — The wrapper's link set changes only on an implicit link from a send or an answer and on a served link or unlink call, with each change appended to the instance log first and then mirrored to the snapshot. · why: Restoring the set at start replays only link and unlink lines, so the snapshot and the log agree only while no other path changes the set. · r1
decision/link-set-follows-log · Link set follows the log — The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails. · r1
decision/list-needs-no-wrapper · list is not a channel method — list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper. · r1
decision/list-reads-state-directly · list reads state without a wrapper — `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper. · r2
decision/list-without-pagination · list without pagination — The `list` payload has no pagination; lists are returned whole. · why: Lists are personal-scale. · r1
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line. · r1
decision/mcp-list-reads-agents · MCP list reads claude agents on each call — The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call. · why: A ratified widening. · r1
decision/mcp-over-stdio · MCP over stdio only — MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only. · r1
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal. · r1
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly. · r2
decision/option-text-never-in-human-line · Upstream option text kept out of human lines — Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line. · r1
decision/permission-answer-without-suggestions · Permission answers without suggestions — A permission answer has no suggestion slot, so an offered `permission_suggestions` entry cannot be picked. · why: A deliberate v1 limit. · r1
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged. · r1
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live. · r1
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale. · r1
decision/revive-refused-without-cwd · Revive over a replayed snapshot refused — A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing. · r1
decision/server-identity-check-before-frame · Server identity checked before the first frame — Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame. · r1
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default. · r2
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout. · r1
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole. · r1
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten. · r1
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands. · r1
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified. · r1
decision/wrapper-channel-json-rpc · JSON-RPC over ndjson for the wrapper channel — The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets. · r1
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`. · r3 · add-to · F2
contract/agents-listing/status · agents listing · `status` — `claude agents --json` adds `status` (idle/busy) to the listing. · r1
contract/agents-listing/unwrapped-sessions · agents listing · unwrapped sessions — Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately. · r1
contract/answer-refusals/unknown-dialog · answer refusals · `unknown-dialog` — A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`. · r1
contract/answer-response/permission · answer response · permission — A `permission` response is `{behavior:"allow"|"deny", message?}`, with no suggestion slot. · r1
contract/answer-response/plan · answer response · plan — A `plan` response is `{behavior:"approve"|"revise", message?}`. · r1
contract/answer-response/question · answer response · question — A `question` response is `{answers:{<question>:<text>}, annotations?}`. · r1
contract/board/agents-read-failure · The board (CLI list and MCP list) · agents read failure — If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error. · r1
contract/board/status · The board (CLI list and MCP list) · status — Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows. · r1
contract/board/wrapped-merge · The board (CLI list and MCP list) · wrapped instance join — An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately. · r1
contract/channel-32602-cause/invalid-params · channel -32602 causes · invalid-params — One of the three causes a -32602 fault reads. · r1
contract/channel-32602-cause/release-from-driver · channel -32602 causes · release-from-driver — One of the three causes a -32602 fault reads. · r1
contract/channel-32602-cause/unsupported-version · channel -32602 causes · unsupported-version — One of the three causes a -32602 fault reads. · r1
contract/channel-link-unlink/answer-order · channel link and unlink methods · answer order — Each answers only after its event line and the snapshot write. · r1
contract/channel-link-unlink/error-32602 · channel link and unlink methods · -32602 — A link or unlink answered -32602 writes no line. · r1
contract/channel-link-unlink/error-32603 · channel link and unlink methods · -32603 — A failed record or remove of the link answers -32603. · r1
contract/channel-link-unlink/from · channel link and unlink methods · from — link and unlink read no from; a from of any shape changes nothing. · r1
contract/channel-link/already-linked · channel link method · already_linked — A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false. · r1
contract/channel-methods/answer · channel methods · answer — A request: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind. · r1
contract/channel-methods/answer-from-link · channel methods · answer from link — An `answer` whose `from` names another instance records the link once every refusal has passed and its kind matched, before the answer is taken; a link that cannot be recorded answers `-32603` and the dialog stays held; others record nothing. · r1
contract/channel-methods/from · channel methods · from — `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`. · r1
contract/channel-methods/hook-dialog-continuation · channel methods · hook.dialog continuation — A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise, otherwise `null`. · r1
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once. · r1
contract/channel-methods/hook-event-driver-origin · channel methods · hook.event driver origin — A `prompt-submitted` equal to the in-flight `send`'s typed text is appended with `origin:"driver"`, and only then settles that send. · r1
contract/channel-methods/last · channel methods · last — A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`. · r1
contract/channel-methods/last-after-seen-line · channel methods · last after a seen line — A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start. · r1
contract/channel-methods/link · channel methods · link — A request: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper. · r1
contract/channel-methods/link-params-check · channel methods · link params check — For `link` and `unlink`, a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`. · r1
contract/channel-methods/pause · channel methods · pause — A request: `pause` `{from?}` answers `{wheel:"human"}`, the wheel moving to `human` / `manual-pause`. · r1
contract/channel-methods/plain-release · channel methods · plain release — A `release` on a driver-held wheel changes nothing. · r2
contract/channel-methods/release · channel methods · release — A request: `release` `{from?}` answers `{wheel:<holder>}`. · r2
contract/channel-methods/release-from-driver · channel methods · release from driver — A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first. · r1
contract/channel-methods/release-type-check · channel methods · release type check — A `release` whose `from` is of a non-string type, or that carries a `budget` key of any value (`null` included), is `-32602` `invalid-params` with `data: null`, no wheel moved and no record; only the string-`from` refusal comes before it. · r2
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste. · r1
contract/channel-methods/send-from-link · channel methods · send from link — A `send` whose `from` names another instance records the link before `send-issued`. · r1
contract/channel-methods/v-and-sender · channel methods · v and sender — Every channel method's `params` carries `v` and `sender`. · r1
contract/channel-methods/wait · channel methods · wait — A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call. · r1
contract/channel-methods/wait-pending-dialog · channel methods · wait pending dialog — For `wait`, a pending dialog comes first unless `after` is past its line. · r1
contract/channel-methods/wait-wake-kinds · channel methods · wait wake kinds — Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file. · r1 · add-to · F1
contract/channel-methods/wheel-move-durability · channel methods · wheel move durability — `pause` and `release` are answered only after the move's `wheel` record has landed, `-32603` if it did not. · r1
contract/channel-send/from · channel send method · from — A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing. · r1
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste. · r1
contract/channel-unlink/result · channel unlink method · result — Answers {} whether or not the set held the pair; an absent pair appends nothing and leaves the set unchanged. · r1
contract/channel-wait/params · channel wait method · after and timeout_ms — Each absent or a non-negative integer fitting u64; null, a negative, a fraction or a string is -32602; unknown fields are ignored. · r1
contract/channel-wait/result · channel wait method · result — {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}. · r1 · add-to · F1
contract/channel-wait/start-offset · channel wait method · start offset — The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once. · r1
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake. · r1 · add-to · F2
contract/cli-exit-21-document/endpoint-lost · exit-21 CLI document · endpoint-lost — One of six closed detail causes; v 1, error instance-unreachable. · r1
contract/cli-exit-21-document/instance-dead · exit-21 CLI document · instance-dead — One of six closed detail causes; v 1, error instance-unreachable. · r1
contract/cli-exit-21-document/server-verify-failed · exit-21 CLI document · server-verify-failed — One of six closed detail causes; v 1, error instance-unreachable. · r1
contract/cli-exit-21-document/socket-dir-refused · exit-21 CLI document · socket-dir-refused — A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN. · r1
contract/cli-exit-21-document/strict-modes-failed · exit-21 CLI document · strict-modes-failed — A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line. · r1
contract/cli-exit-21-document/unwrapped-name · exit-21 CLI document · unwrapped-name — One of six closed detail causes; v 1, error instance-unreachable. · r1
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document. · r1
contract/cli-json-output/instance-unreachable · CLI `--json` output · `instance-unreachable` — `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`. · r1
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent. · r1
contract/cli-json-output/plugin-install · CLI `--json` output · `viola plugin install --json` — Prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with exit 0, no path. · r1
contract/cli-json-output/plugin-install-strict-modes-refused · CLI `--json` output · `viola plugin install --json` strict-modes refusal — A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21. · r1
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code. · r1
contract/cli-json-output/revive-list · CLI `--json` output · `viola revive <name> --list --json` — Prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order with exit 0, or the `no-session` document with exit 1 on an empty chain. · r1
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty. · r1
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`. · r1
contract/cli-json-output/wrapper-fault · CLI `--json` output · `wrapper-fault` — `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null. · r1
contract/cli-link-json/ok · viola link and unlink --json output · ok — link prints {"v":1,"ok":{"since":…,"already_linked":…}} and unlink prints {"v":1,"ok":{}}, each with exit 0. · r1
contract/event-data/activity · event data · `activity` — `{}`, plus `tool` when the hook names one. · r1 · add-to · F2
contract/event-data/dialog-expired · event data · `dialog-expired` — `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text. · r1
contract/event-data/link · event data · `link` — `{driver, driven}`. · r1
contract/event-data/permission · event data · `permission` — `{dialog_id, tool, input}`; `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload. · r1
contract/event-data/plan · event data · `plan` — `{dialog_id, plan}`, with the plan as text. · r1
contract/event-data/prompt-submitted-origin · event data per kind · prompt-submitted origin — `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt. · r1 · add-to · F2
contract/event-data/prompt-submitted-text · event data per kind · prompt-submitted text — `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed. · r1
contract/event-data/question · event data · `question` — `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`; each `question` string is a key in `answer`'s `answers`. · r1
contract/event-data/question-option-details · event data · `question` option_details — `option_details` is parallel to `options`, one per option, each field the option's string or `null`; a missing or non-string value never refuses the dialog. · r1
contract/event-data/send-confirmed · event data · `send-confirmed` — `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record. · r1
contract/event-data/send-issued · event data · `send-issued` — `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor. · r1
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`. · r1
contract/event-data/session-end · event data · `session-end` — `{}`. · r1
contract/event-data/session-start · event data per kind · session-start — `session-start` data is `{cause, agent_session_id}`. · r1
contract/event-data/turn-ended-background-tasks · event data per kind · turn-ended background_tasks — `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array. · r1 · add-to · F1
contract/event-data/turn-ended-cause · event data per kind · turn-ended cause — `cause` is present only on an API-error turn end, as `api-error`, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`, applied provisionally ahead of ratification. · r1
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from. · r2 · add-to · F3
contract/event-data/unlink · event data · `unlink` — `{driver, driven}`. · r1
contract/event-data/wheel · event data · `wheel` — `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`. · r1
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload. · r1
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`. · r1
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`. · r1
contract/event-prompt-submitted/origin · prompt-submitted event data · origin — driver, human or harness; the hook files only human or harness, and driver is set only by the wrapper's relabel of the in-flight send's own prompt. · r1 · add-to · F2
contract/event-prompt-submitted/text · prompt-submitted event data · text — The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it. · r1
contract/fault-document/invalid-params · The fault document on --json and MCP · other -32602 — Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null. · r1
contract/fault-document/known-code-message · The fault document on --json and MCP · message of a known fault code — For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault. · r1
contract/fault-document/release-from-driver · The fault document on --json and MCP · -32602 release-from-driver — A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body. · r1
contract/fault-document/unsupported-version · The fault document on --json and MCP · -32602 unsupported version — A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values. · r1
contract/fault-document/wrapper-fault · The fault document on --json and MCP · wrapper fault — A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word. · r1
contract/hook-contract/async-tier · hook contract · async-tier hooks — Async-tier hooks always print nothing. · r1
contract/hook-contract/decision · hook contract · a decision — A decision is exit 0 with a `hookSpecificOutput` body. · r1
contract/hook-contract/no-decision · hook contract · no decision — No decision is exit 0 with an empty stdout. · r1
contract/hook-contract/permission-request · hook contract · PermissionRequest — PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`. · r1
contract/hook-contract/pre-tool-use · hook contract · PreToolUse — PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`. · r1
contract/hook-dialog/data · `hook.dialog` · `data` — `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`. · r1
contract/hook-dialog/dialog-hooks · `hook.dialog` · dialog hooks — Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog. · r1
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}. · r1
contract/hook-event/driver-relabel · channel hook.event · driver relabel — A prompt-submitted whose text equals the in-flight send's typed text is appended with origin driver and only then settles that send. · r1
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603. · r1
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}. · r1
contract/human-error-line/wrapper-fault · The human error line · wrapper fault — A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`. · r1
contract/hyprland-seam-errors/malformed · Hyprland seam errors · malformed — The reply could not be parsed for its request; the fixed message hyprctl reply not understood interpolates nothing. · r1
contract/hyprland-seam-errors/oversize · Hyprland seam errors · oversize — The reply passed the frame cap and was refused whole before the exit was read; the fixed message hyprctl reply over the frame cap interpolates nothing. · r1
contract/hyprland-seam-errors/refused · Hyprland seam errors · refused — hyprctl refused a dispatch; the fixed message hyprctl refused the dispatch interpolates nothing. · r1
contract/hyprland-seam-errors/unreachable · Hyprland seam errors · unreachable — hyprctl could not start, exited non-zero or was killed at its deadline; the fixed message hyprctl did not answer interpolates nothing. · r1
contract/hyprland-seam-errors/unsupported · Hyprland seam errors · unsupported — No hyprctl program was given, so nothing is spawned and no line is written; the fixed message hyprctl not found interpolates nothing. · r1
contract/instance-snapshot/agent-session-id · instance snapshot · agent_session_id — `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance. · r1
contract/instance-snapshot/child-pid · instance snapshot · child_pid — `child_pid` is the pid of the spawned agent child and exists only to join `claude agents --json` rows to this instance. · r1
contract/instance-snapshot/cli-verified · instance snapshot · cli_verified — `cli_verified` comes from `run`'s version gate. · r1
contract/instance-snapshot/cli-version · instance snapshot · cli_version — `cli_version` comes from `run`'s version gate. · r1
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it only as the revived child's spawn directory. · r1
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`. · r3
contract/instance-snapshot/endpoint · instance snapshot · endpoint — `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it. · r1
contract/instance-snapshot/links · instance snapshot · links — `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was. · r1
contract/instance-snapshot/pending-dialog · instance snapshot · pending_dialog — `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log. · r1
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame. · r1
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it. · r1
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`. · r1
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`. · r1
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none. · r1
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`. · r1
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it. · r2
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail". · r1
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`. · r1
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version. · r1
contract/list-item/liveness · list payload item · liveness — live or stale; gone instances are omitted. · r1
contract/list-item/status · list payload item · status — idle, busy or unknown. · r1
contract/list-item/wheel · list payload item · wheel — human or driver; absent when the session is unwrapped. · r1
contract/list-item/wrapped · list payload item · wrapped — false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, cli_version or cli_verified. · r2
contract/list-payload/agents-read-failure · list payload · agents read failure — A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item. · r1
contract/list-payload/dialog-pending · list payload · dialog_pending — `dialog_pending` comes only from the snapshot's `pending_dialog`, so a reader that falls back to replay reports `dialog_pending: false`. · r1
contract/list-payload/items · list payload · items — Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?}`. · r2
contract/list-payload/liveness · list payload · liveness — An entry's `liveness` is `live` or `stale`; gone instances are omitted. · r1
contract/list-payload/ok · list payload · ok — The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper. · r2
contract/list-payload/skipped · list payload · skipped — `skipped` counts records a reader could not interpret, so the board can show them. · r1
contract/list-payload/status · list payload · status — A failed `claude agents --json` read leaves every `status` `unknown`. · r1
contract/list-payload/wrapped · list payload · wrapped — `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`. · r1
contract/mcp-list/refusal · MCP list · refusal — The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed. · r1
contract/mcp-refusal-result/instance-unreachable · MCP refusal result · `instance-unreachable` — The MCP `isError` result for an unreachable instance is the CLI `instance-unreachable` document less `v`, its text `refused: instance-unreachable`. · r1
contract/mcp-refusal-result/shape · MCP refusal result · result shape — `{"content":[{"type":"text","text":"refused: human-typing"}],"isError":true,"structuredContent":{"refusal":"human-typing","detail":null}}` · r2
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged. · r1
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned. · r1
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal. · r1
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned. · r1
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age. · r1
contract/session-liveness/heartbeat · session liveness · heartbeat — The wrapper touches `<instance dir>/heartbeat` every 1 s. · r1
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`. · r1
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`. · r1
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`. · r1
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`; the replay writes no file. · r2
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson. · r1
contract/wrapper-channel-frames/conn · wrapper channel frames · conn — A client may add `conn` to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity. · r1
contract/wrapper-channel-frames/envelope · wrapper channel frames · envelope — Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`. · r1
contract/wrapper-channel-frames/id-less-frames · wrapper channel frames · id-less frames — Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`. · r1
contract/wrapper-channel-frames/max-frame · wrapper channel frames · MAX_FRAME — A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed. · r1
contract/wrapper-channel-frames/ok-result · wrapper channel frames · ok result — A success is answered as `result` holding `{"ok": <payload>}`. · r1
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`. · r1
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`. · r1
contract/wrapper-event-appends/link · wrapper event appends · `link` — The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call. · r1
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send. · r1
contract/wrapper-event-appends/unlink · wrapper event appends · `unlink` — The driven instance's wrapper appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes. · r1
contract/wrapper-event-appends/wheel · wrapper event appends · `wheel` — The wrapper appends `wheel` with `source: wrapper` once at start and on every change. · r1
absence/graphical-interface — A web or graphical surface · r1
absence/polling-endpoints — Polling endpoints (v1) · r1
absence/websocket — WebSocket endpoints (v1) · r1

## occupied-resources · ## Occupied Resources
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer. · r1
decision/chaos-home-outside-gates · Chaos panic home outside the obs gates — The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself. · why: the one named carve-out to the test-data rule · guard: chaos_feed_panic · r1
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says. · r1
decision/chunk-diff-secret-scan-skip · chunk.diff is the one secret-scan skip — The mutation run's chunk.diff is the one file secret-scan skips, by exact path, and the harness artifact upload excludes it. · r1
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name. · r1
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads. · r2
decision/cleanup-reads-refused-or-missing-socket-gone · Cleanup reads a refused or missing socket as gone — Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper. · why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing. · r1
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files. · r1
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read · r1
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded. · r2
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value makes the dialog hold zero or unbounded · r2
decision/config-keys-match-schema · Config keys equal the config schema — The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties. · guard: config_keys_are_the_config_schema_properties · r1
decision/connect-to-recorded-endpoint · Clients connect to the recorded endpoint — The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path. · why: Processes spawned inside the session may run with a different environment from the wrapper. · r1
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget · r1 · add-to · F4
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line. · r1
decision/e2e-home-headroom-floor · e2e-home backing headroom floor — A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key. · why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB · r1 · add-to · F4
decision/e2e-home-owner-only-headroom · End-to-end home base checked before use — The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free. · r1 · add-to · F4
decision/e2e-noop-fake-agent-feature · No-op fake-agent feature in the e2e crate — The test-only `viola-e2e` crate declares a no-op `fake-agent` feature. · why: So package-scoped `--features fake-agent` invocations resolve. · r1
decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates. · r1 · add-to · F4
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · r1
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · r1
decision/events-log-never-rotated · Events log never rotated — v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into it · r1
decision/events-log-never-truncated · Events log never rotated or truncated — The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid. · why: wait's after and send's cursor are byte offsets into the file. · r1
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client. · r1
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction. · why: A ratified test seam, ratified together with the exact-path exemption for its panic lines. · r1
decision/fake-agent-pump-delay-seam · FAKE_AGENT_PUMP_DELAY_MS test seam — FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction. · why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel. · r1
decision/harness-env-unread-by-product · Harness variables never read by viola — AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED and CI are test-harness variables that viola itself never reads. · r1
decision/hook-missing-instance-dir-exits-silently · Hook with a missing instance folder exits 0 silently — A hook whose VIOLA_DIR names an instance folder that does not exist fails the canonical-directory step, exits 0 and writes nothing: no role file, no stdin read, no frame and no event line. · why: Claude Code runs the plugin's hooks in sessions viola does not wrap; the hook fail-open contract and its no-instance-directory test rely on it. · r1
decision/identity-floor-eleven-names · Identity floor of eleven names — The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID. · why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited. · r1
decision/linked-e2e-home-base-strict · Linked e2e-home base must be a private real directory — A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path. · r1 · add-to · F4
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: mcp's start and plugin install keep the write rule on the home. · r2
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types. · r1
decision/mcp-subcommand-hidden · Hidden mcp subcommand — The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home. · why: The plugin runs it, never a person. · r1
decision/mutants-own-target-dir · Mutation runs use their own relative target dir — Mutation testing builds in its own target dir, passed to cargo-mutants as a relative path, so each copied tree builds every test binary in its own copy. · why: a copied default target keeps the original tree's binary paths, so root integration tests would drive the unmutated binary · r1
decision/mutation-scratch-nocow · Mutation scratch directory is NOCOW — On the Linux dev host every mutation run's TMPDIR is a sibling directory outside the repository marked NOCOW, an environment fact and not harness code. · why: /tmp is a quota tmpfs too small for the copied tree, and a COW btrfs reflink copy drops the fake agent's exec bit; NOCOW makes the copy fall back to a mode-keeping copy · r1
decision/no-abstract-namespace-sockets · No abstract-namespace sockets — Linux abstract-namespace socket names are never used. · why: They carry no filesystem permissions and any local user could connect. · r1
decision/no-after-every-test-error-body-helper · No after-every-test error-body helper — Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built. · r1
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls · r1
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI · r1
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair. · r1
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found. · r1
decision/plugin-carries-no-hooks · Installed plugin carries no hooks — The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks. · r1
decision/plugin-env-vars-unused · Plugin-provided environment variables unused — viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR. · r1
decision/plugin-install-rewrites-marketplace-whole · Plugin install rewrites the marketplace tree whole — The plugin marketplace tree is rewritten whole on every plugin install, any other entry there is left as found, and viola never reads the tree back. · why: A rerun re-points the MCP server at the current exe and home, the rerun test expects the same tree, and only its folders need the strict-modes check. · r1
decision/probe-capture-arms-inert · Probe capture arms are inert — The hidden hook `--capture` arm takes an absolute, existing directory only, reads no `VIOLA_*` variable, starts no observability, opens no channel, and always exits 0 with empty stderr. · r2
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files. · r1
decision/profraw-census-dev-host-only · Profraw census is a dev-host witness only — The profraw census script is a dev-host witness for the corrupt-profile mechanism, and no CI job, pre-push stage or harness command runs either of its forms. · r1
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene · r1
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found. · r1
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages. · r1
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support. · r2 · stale
decision/run-archive-never-uploaded · Per-run archive never uploaded — The per-run archive of JUnit files and mutation outcomes stays local, outside the agent-run artifacts, and is never uploaded. · why: The mutation outcomes file carries absolute argv paths. · r1
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined. · r1
decision/same-user-peers-only · Same-user peers only — Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame. · r1
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording. · r2
decision/secret-scan-skips-only-chunk-diff · Secret scan skips only the chunk diff — secret-scan covers every file except the harness's chunk.diff, skipped by exact path, and the harness upload excludes that file. · r1
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home. · r1
decision/socket-dir-owner-only · Socket in a per-user owner-only directory — The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed. · r1
decision/socket-file-owner-only · Socket file owner-only — The socket is chmod 0600 after the bind. · r1
decision/socket-in-per-user-dir · Endpoint socket in a per-user 0700 directory — The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found. · r1
decision/stale-socket-file-left · Stale socket file left in place — A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: A choice: a harmless file in a 0700 directory. · r1
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing. · r1 · add-to · F4
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770. · r1
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home · r1
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit. · r1
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature. · r1
decision/test-endpoints-outside-namespace · Test channel endpoints outside the product namespace — Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`. · r1
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence · r1
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep · r1 · contradict
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name. · r1 · contradict
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction. · r1
decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified · r1 · add-to · F4
decision/test-sockets-under-short-tmp-root · Test channel sockets under a short /tmp root — Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint socket is not under that root. · why: So the socket path length bound holds whatever the length of TMPDIR. · r1
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it · r1
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads · r1
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole when it passes, fails or is stopped by Ctrl-C or TERM, and only the folders that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand. · r1
decision/verify-run-e-resumes-run-b-first-session · Verify's Run E resumes Run B's first session — Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the five runs execute in order A to E. · r1
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false. · r1 · add-to · F4
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line. · r1
decision/viola-channel-tokio-free-by-default · viola-channel tokio-free without its feature — The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client. · guard: channel-tokio-free.sh · r1
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing. · r1
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence. · r1
contract/channel-response-detail/invalid-params · channel-response detail, a closed enum · invalid-params — Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data. · r1
contract/channel-response-detail/release-from-driver · channel-response detail, a closed enum · release-from-driver — Written at WARN on both the server's and the client's line. · r1
contract/channel-response-detail/unsupported-version · channel-response detail, a closed enum · unsupported-version — Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name. · r1
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags. · r1
contract/child-argv/fork-session · flags passed to the child · `--fork-session` — A revived start adds `--fork-session` after `--resume <id>` under `--fork`. · r1
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start. · r1
contract/child-argv/resume · flags passed to the child · `--resume` — A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn. · r1
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder. · r1
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it. · r1
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home. · r1
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing. · r1
contract/claude-child-argv/fork-session · the claude child's argv · --fork-session — --fork-session follows --resume when --fork is given; the words after -- come last. · r1
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start. · r1
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it. · r1
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. · r1
contract/config-json/diagnostics-level · config.json · diagnostics_level — "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect. · r1
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded. · r1
contract/config-json/v · config.json · v — config.json carries v, which must be 1; the file is read with a MAX_FRAME cap. · r1
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600. · r1
contract/diag-line/state-recovered · diagnostics line · state-recovered — A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay. · r1
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1. · r1
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1. · r1
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install. · r1
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines. · r1
contract/diagnostics-role-files/mcp · home-level diagnostics role files · mcp.ndjson — viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes. · r1
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included. · r1
contract/e2e-home-refusal/headroom-message · e2e-home base refusal · headroom message — A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path. · r1
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson. · r1
contract/frame-verb-exit-codes/21 · send, wait, last, answer, pause and release exit codes · 21 — The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout. · r1
contract/frame-verb-exit-detail/endpoint-lost · CLI frame verb process-exit detail · endpoint-lost — Exit 21 with during call, for a wrapper lost with the call open. · r1
contract/frame-verb-exit-detail/instance-dead · CLI frame verb process-exit detail · instance-dead — Exit 21 with during connect, at WARN. · r1
contract/frame-verb-exit-detail/server-verify-failed · CLI frame verb process-exit detail · server-verify-failed — Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line. · r1
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1. · r1
contract/frame-verb-exit-detail/unwrapped-name · CLI frame verb process-exit detail · unwrapped-name — Exit 21 with during connect. · r1
contract/frame-verb-exit-detail/wrapper-fault · CLI frame verb process-exit detail · wrapper-fault — Exit 20. · r1
contract/harness-environment/agent-run-chunk-base · test harness environment · AGENT_RUN_CHUNK_BASE — Overrides the diff base of viola-harness run --mutants; without it the harness derives the base from git history. · r1
contract/harness-environment/agent-run-keep-failed · test harness environment · AGENT_RUN_KEEP_FAILED — The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command. · r1
contract/harness-environment/agent-run-keep-homes · test harness environment · AGENT_RUN_KEEP_HOMES — The value 1 keeps the test and session homes for the CI gates. · r1
contract/hook-capture-arm/exit · viola hook --capture arm · exit status — Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened. · r1
contract/hook-capture-arm/payload-file · viola hook --capture arm · payload file — Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder. · r1
contract/hook-decision-detail/socket-dir-refused · hook-decision detail · socket-dir-refused — One hook-decision line at WARN on a refused socket folder, with decision_emitted false. · r1
contract/human-output-colour-env/colorterm · Human output colour environment · COLORTERM — truecolor or 24bit picks truecolor. · r1
contract/human-output-colour-env/no-color · Human output colour environment · NO_COLOR — Present, an empty value included, switches colour off. · r1
contract/human-output-colour-env/term · Human output colour environment · TERM — Exactly dumb switches colour off; a value containing 256color picks 256 colours. · r1
contract/instance-unreachable-detail/server-verify-failed · the instance-unreachable --json document · server-verify-failed — The detail when the server runs under another euid. · r1
contract/instance-unreachable-detail/socket-dir-refused · the instance-unreachable --json document · socket-dir-refused — The detail when the recorded socket folder was refused, past the instance check. · r1
contract/instance-unreachable-detail/strict-modes-failed · the instance-unreachable --json document · strict-modes-failed — The detail when the instance check refused the instance. · r1
contract/process-exit-detail/stale-heartbeat · process-exit detail enum · stale-heartbeat — A word of the process-exit detail enum in schemas/diag-line.v1.json. · r1
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only. · r1
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only. · r1
contract/profraw-census-exit-codes/0 · profraw census exit codes · 0 — Every run passed and, in the default form, the profile count equals the run count with no third or short profile; in the case form, no profile was refused. · r1
contract/profraw-census-exit-codes/1 · profraw census exit codes · 1 — Any other count. · r1
contract/profraw-census-exit-codes/2 · profraw census exit codes · 2 — A usage error, no instrumented binary, or (case form) no llvm-profdata. · r1
contract/session-id/shape · Claude session id · shape — 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere. · r1
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path. · r1
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token. · r1
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path. · r1
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content. · r1
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam. · r1
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code. · r1
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME. · r1
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — A refusal code verify writes under a parsed VIOLA_NAME. · r1
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line. · r1
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code. · r1
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME. · r1
contract/viola-cli-arguments/link-unlink · viola CLI arguments · link and unlink — viola link and viola unlink each take <driver> <driven> [--json], the verb's instance being the driven one. · r1
contract/viola-cli-arguments/list · viola CLI arguments · list — viola list takes [--json] and nothing else. · r1
contract/viola-cli-arguments/plugin-install · viola CLI arguments · plugin install — viola plugin install takes [--json] and nothing else; viola plugin alone is a usage error. · r1
contract/viola-cli-arguments/revive · viola CLI arguments · revive — viola revive takes <name> [--id <ID>] [--fork] [-- <child args>], or <name> --list, each with [--json]. · r1
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude. · r1
contract/viola-link-exit-codes/0 · viola link and unlink exit codes · 0 — One result line, or under --json {"v":1,"ok":<ok>}, link's ok holding since and already_linked and unlink's ok empty. · r1
contract/viola-link-exit-codes/1 · viola link and unlink exit codes · 1 — A refusal reply, or an ok of another shape. · r1
contract/viola-link-exit-codes/20 · viola link and unlink exit codes · 20 — A wrapper error, a -32602 included, such as driver equal to driven. · r1
contract/viola-link-exit-codes/21 · viola link and unlink exit codes · 21 — The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid. · r1
contract/viola-list-exit-codes/0 · viola list exit codes · 0 — The board was printed in one stdout write. · r1
contract/viola-list-exit-codes/1 · viola list exit codes · 1 — An internal error, through the cli catch site. · r1
contract/viola-list-exit-codes/2 · viola list exit codes · 2 — A clap usage error. · r1
contract/viola-list-exit-codes/21 · viola list exit codes · 21 — The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused. · r1
contract/viola-mcp-exit-codes/0 · viola mcp exit codes · 0 — The session ended by stdin closing, or by a stdin line past MAX_FRAME, once initialized; both streams empty. · r1
contract/viola-mcp-exit-codes/1 · viola mcp exit codes · 1 — The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file. · r1
contract/viola-plugin-install-exit-codes/0 · viola plugin install exit codes · 0 — The result was printed in one stdout write. · r1
contract/viola-plugin-install-exit-codes/1 · viola plugin install exit codes · 1 — An internal error, through the cli catch site. · r1
contract/viola-plugin-install-exit-codes/2 · viola plugin install exit codes · 2 — A clap usage error. · r1
contract/viola-plugin-install-exit-codes/21 · viola plugin install exit codes · 21 — The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened. · r1
contract/viola-revive-exit-codes/0 · viola revive exit codes · 0 — The revived child's wrapper ended as viola run's does. · r1
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight. · r1
contract/viola-revive-exit-codes/2 · viola revive exit codes · 2 — A clap usage error, a malformed --id included. · r1
contract/viola-revive-json/list · viola revive --json output · --list — viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1. · r1
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind. · r1
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced. · r1
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed. · r1
absence/listening-network-port — A listening network port of any kind: no database port, no broker port, no listening TCP port · r1
name/binary/viola — viola · r1
name/binary/viola-fake-agent — viola-fake-agent · r1
name/binary/viola-harness — viola-harness · r1
name/binary/viola-hypr-tap — viola-hypr-tap · r1
name/binary/viola-mcp-tap — viola-mcp-tap · r1
name/command/answer — answer · r1
name/command/hook — hook · r1
name/command/last — last · r1
name/command/link — link · r1
name/command/list — list · r1
name/command/mcp — mcp · r1
name/command/pause — pause · r1
name/command/plugin-install — plugin install · r1
name/command/release — release · r1
name/command/revive — revive · r1
name/command/run — run · r1
name/command/send — send · r1
name/command/unlink — unlink · r1
name/command/verify — verify · r1
name/command/wait — wait · r1
name/crate/viola — viola · r1
name/crate/viola-agent-claude — viola-agent-claude · r1
name/crate/viola-channel — viola-channel · r1
name/crate/viola-core — viola-core · r1
name/crate/viola-e2e — viola-e2e · r1
name/crate/viola-fuzz — viola-fuzz · r1
name/crate/viola-hyprland — viola-hyprland · r1
name/crate/viola-mcp — viola-mcp · r1
name/crate/viola-pty — viola-pty · r1
name/crate/viola-state — viola-state · r1
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE · r1
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED · r1
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES · r1
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC · r1
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS · r1
name/other/hook-notification — hook notification · r1
name/other/hook-permission-request — hook permission-request · r1
name/other/hook-post-tool-use — hook post-tool-use · r1
name/other/hook-post-tool-use-failure — hook post-tool-use-failure · r1
name/other/hook-pre-tool-use — hook pre-tool-use · r1
name/other/hook-session-end — hook session-end · r1
name/other/hook-session-start — hook session-start · r1
name/other/hook-stop — hook stop · r1
name/other/hook-stop-failure — hook stop-failure · r1
name/other/hook-user-prompt-submit — hook user-prompt-submit · r1
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD · r1
name/other/mcp-tool-answer — answer · r1
name/other/mcp-tool-last — last · r1
name/other/mcp-tool-list — list · r1
name/other/mcp-tool-send — send · r1
name/other/mcp-tool-wait — wait · r1
name/other/viola — viola · r1
name/other/viola-at-viola — viola@viola · r1
name/other/viola-bin — VIOLA_BIN · r1
name/other/viola-dir — VIOLA_DIR · r1
name/other/viola-h12-sock — viola-<h12>.sock · r1
name/other/viola-name — VIOLA_NAME · r1
name/other/viola-test-chan-pid-label — viola-test-chan-<pid>-<label> · r1
name/other/viola-verify-probe — viola-verify-probe · r1
name/server/viola — viola · r1

## infrastructure-patterns · ## Infrastructure Patterns

## build-system · key of ## Infrastructure Patterns
decision/banned-features · Banned dependency features — The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned. · guard: cargo deny check · r1
decision/channel-tokio-only-behind-feature · viola-channel tokio only behind its feature — viola-channel's normal dependency graph without its tokio feature holds no tokio; the Tokio client exists only behind that feature. · why: As its own root viola-channel false-fails the sole-root ban once viola-mcp enables the feature, so a tree check replaces it. · guard: scripts/channel-tokio-free.sh · r1
decision/crates-io-only · Sources from crates.io only — Every dependency comes from crates.io only. · guard: cargo deny check · r1
decision/deny-bans-proven · Every dependency ban proven live — Every dependency ban is proven live: a throwaway project per ban must fail with that ban's own diagnostic, and a clean control must pass. · guard: scripts/deny-probes.sh · r1
decision/deny-single-target · Dependency policy for one target triple — The dependency policy is evaluated for the one target triple x86_64-unknown-linux-gnu. · r1
decision/dual-licence · MIT OR Apache-2.0 licence — The project is licensed MIT OR Apache-2.0, set once for the workspace and inherited by every member, with both licence texts at the root and the README naming both. · r1
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied. · r1
decision/fuzz-lockfile-audited · Fuzz lockfile audited separately — The fuzz workspace's own lockfile is audited separately for advisories and sources. · guard: CI supply-chain step Fuzz lockfile audit; nightly advisories job · r1
decision/install-from-source · Install with cargo install — viola is installed with cargo install from the repository root. · r1
decision/licence-allowlist · Licence allowlist — Dependency licences allowed are exactly MIT, Apache-2.0, Zlib and Unicode-3.0. · why: The 0BSD exception was ratified. · guard: cargo deny check · except: 0BSD only through the per-crate exception for doctest-file, never through the allowlist. · r1
decision/lint-bans-proven · Every lint ban proven to fire — Every lint ban is proven to fire and every control proven to pass. · guard: scripts/lint-probes.sh · r1
decision/module-graph-review-not-gate · Module dependency graph is a review, not a gate — The acyclic module dependency graph stays an on-demand review, never a gate. · why: cargo-modules reports every type and its own inherent method as a cycle, whatever the filters. · r1
decision/msrv · Minimum supported Rust version 1.96 — The workspace declares rust-version 1.96, and dependency resolution prefers versions compatible with it. · r1
decision/no-c-building-crates · No C-building crates — C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph. · guard: cargo deny check · except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace. · r1
decision/no-crates-io-publishing · Nothing published to crates.io — Every workspace member, the root viola bin included, is unpublished; nothing goes to crates.io in v1. · r1
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh · r1
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow. · r1
decision/no-telemetry-crates · No telemetry crates — Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned. · guard: cargo deny check · r1
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh · r1
decision/rmcp-features-exact · rmcp with exactly server and transport-io — rmcp is present in viola's release graph with default features off, and the features requested of it are exactly server and transport-io. · guard: scripts/rmcp-features.sh · r1
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth · r1
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it. · r1
decision/sync-crates-tokio-free · Sync crates are tokio-free — Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root. · why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification. · guard: deny-sync.toml per crate in scripts/sync-crates.txt · r1
decision/workspace-dependency-pins · Third-party versions pinned in one place — Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it. · r1
contract/cli-json/internal-error · CLI --json output · internal-error — Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty. · r1
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`. · r1
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write. · r1

## ci-cd-approach · key of ## Infrastructure Patterns
decision/actions-pinned-by-sha · Actions pinned by commit SHA — Every action used in a workflow is pinned by full commit SHA with a version comment. · guard: zizmor · r1
decision/ci-tools-pinned · CI tools at pinned versions — Every tool CI installs is version-pinned and checkout keeps no credentials; a release asset installed by hand is checked against its published sha256. · r1
decision/coverage-floors · Coverage floors — The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest. · guard: gate --require coverage,doctest · r1
decision/event-payload-via-env · Event payload only through env — Event-payload values reach a workflow step only through environment variables. · guard: zizmor · r1
decision/fuzz-replay-ci · Fuzz corpus replayed in CI — A CI job replays the fuzz corpus on every push and pull request. · guard: gate --require fuzz-replay · r1
decision/gate-steps-fail-closed · Gate steps fail closed — Every CI gate step runs fail-closed in bash. · r1
decision/github-actions-ci · GitHub Actions CI — CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow. · r1
decision/local-pre-push-gate · Local pre-push gate — A local pre-push gate precedes every operator push and a red pre-push stops the pass; CI's run on the pushed sha stays the verdict of record. · r1
decision/msrv-ci · MSRV checked in CI — A CI job checks the workspace and runs the unit tests on Rust 1.96. · guard: gate --require nextest-unit · r1
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit. · r1
decision/no-workflow-concurrency · No workflow concurrency groups — No workflow has a concurrency block. · why: A concurrency group cancels pending runs, which would drop a cancelled run's always() gate and upload chain. · r1
decision/perf-gate · Perf runs in its own job — Performance runs only in the perf job, gated on its perf verdict; the test job and the pre-push carry no perf step. · guard: gate --require perf · r1
decision/pre-push-clean-environment · Pre-push children in a clean environment — The pre-push runs natively in the working tree, every child with an emptied environment holding only HOME from the passwd entry and a fixed PATH; the harness's own HOME is never read. · r1
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs · r1
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh · r1
decision/release-via-dist · Release workflow generated by dist — The v1.x release path is a dist-generated release workflow with cargo-auditable. · r1
decision/sync-crates-compile-without-tokio · Sync crates compile without tokio — Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails. · r1
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures. · r1 · stale
decision/uploads-gated-on-secret-scan · Uploads gated on the secret scan — CI artifacts are uploaded only when the secret scan succeeds; on a scan failure only the secret-scan report is uploaded. · guard: secret-scan step · r1
decision/weekly-advisories · Weekly advisory audit — A weekly job audits advisories over both the root and the fuzz lockfile. · why: The advisory database moves without code changes. · r1
decision/weekly-fuzz · Weekly fuzzing — A weekly job fuzzes every target for a bounded time and uploads the fuzz artifacts on failure. · r1
decision/workflow-permissions · Least-privilege workflow permissions — Every workflow sets empty permissions at the top and read-only contents per job. · guard: zizmor · r1
contract/pre-push/linux-only · viola-harness pre-push · pre-push-linux-only — On a host other than Linux the pre-push refuses `pre-push-linux-only` with exit 2. · r1
contract/pre-push/passwd-home · viola-harness pre-push · tool-missing passwd-home — A failed passwd probe or a non-absolute home field is `tool-missing` `passwd-home` at stage `tools`. · r1
absence/deploy-stage — A deploy stage · r1

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure. · r2
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · r1
decision/core-depends-on-no-viola-crate · viola-core depends on no viola crate — viola-core depends on no other viola crate. · r1
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it. · r1
decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on viola-core alone among viola crates, never on viola-agent-claude, viola-state or viola-channel. · guard: the cargo tree check of viola-hyprland and the per-sync-crate tokio ban · r1
decision/mcp-dependencies · viola-mcp takes channel with tokio — viola-mcp depends on viola-channel with its tokio feature and on tokio, and takes no serde. · r1
decision/pty-knows-no-agent · viola-pty depends on no viola crate and knows no agent — viola-pty depends on no other viola crate, knows no agent, and takes no thiserror. · r1
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events. · r3

## deployment-model · key of ## Infrastructure Patterns
decision/local-only-deployment · Local-only deployment — viola is deployed local-only. · r1
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory. · r1
decision/single-binary-on-path · Single binary on PATH — viola ships as a single binary installed on PATH. · r1
absence/container-or-serverless-deployment — Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless) · r1

## project-directory-structure · key of ## Infrastructure Patterns
decision/anyhow-at-bin-edge-only · anyhow at the binary edge only — anyhow error handling is confined to the edge of the viola binary. · r1
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid, started_at or links. · r2
decision/cargo-deny-bans · cargo-deny supply-chain policy — cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features. · guard: deny-probes negative probe per ban with a clean control · r1
decision/channel-json-rpc-local-sockets · JSON-RPC over local sockets for the channel — The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets. · r1
decision/channel-tokio-behind-feature · Channel Tokio client behind a feature — viola-channel's normal dependency graph carries no tokio; the Tokio client exists only behind its tokio feature. · guard: channel-tokio-free check in CI supply-chain · r1
decision/exact-toolchain-pin · Exact Rust toolchain pin — The project builds on one exactly pinned stable Rust toolchain with rustfmt and clippy, never a floating channel. · except: The separate fuzz workspace uses its own pinned nightly toolchain. · r1
decision/fuzz-separate-workspace · Fuzzing in a separate workspace — Fuzz targets live in a separate cargo-fuzz workspace with its own lockfile, excluded from the root workspace, with committed synthetic seeds. · r1
decision/no-tokio-in-sync-crates · No tokio in sync crates — Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root. · guard: cargo-deny tokio ban over the sync-crate list · r1
decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes · r1
decision/pty-seam-portable-pty · portable-pty is the PTY seam — The PTY seam is built over portable-pty, with host raw mode through libc termios. · r1
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job · r1
decision/rmcp-exact-features · rmcp features exactly server and transport-io — viola's release graph requests rmcp with exactly the features server and transport-io. · guard: rmcp-features check in the CI lint job · r1
decision/snapshots-check-mode-only · Insta snapshots in check mode only — The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots. · r1
decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module. · r1
