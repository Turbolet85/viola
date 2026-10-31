# architecture · Standard Contracts
Read whole. The master's broad view is .andromeda/architecture.md; this file holds its Standard Contracts detail.

## Standard Contracts

<!-- detail: values · contract-list-payload -->

- `items` entry: `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
  - `budget_paused` is the instance's own gate from its snapshot. It is `false` after a `release --budget` override, and it is absent when the session is unwrapped. The payload's `budget.paused` only says that the newest reading crosses a threshold.
  - `wrapped: false` marks a read-only unwrapped row from `claude agents --json`. Its `name` is the session identifier that `claude agents --json` reports (its `sessionId`; the join is ledger row `agents-join`). It is not a `ViolaName` and is never a valid `target`. Its `liveness` is `live` and `dialog_pending` is `false`, and `wheel`, `budget_paused`, `cli_version` and `cli_verified` are absent.
  - `liveness` is `live` or `stale`. Gone instances are omitted.
  - `status` is `idle`, `busy` or `unknown`.
  - `wheel` is `human` or `driver`, and is absent when the session is unwrapped.
  - The payload also carries a top-level `budget`: `{five_hour, seven_day, read_at, paused}`, or `"unknown"`. `five_hour` and `seven_day` are each `{used_percentage, resets_at}`, with either field `"unknown"` when unparseable and the whole window `"unknown"` when the statusline omits it, because the two windows reset independently and a budget pause lifts at the crossed window's own `resets_at`. `budget.json` holds the same per-window reading plus `read_at`.

<!-- /detail -->

<!-- detail: values · contract-snapshot-envelope -->

**Snapshot envelope** (atomically replaced JSON)
```json
{"v":1,"written_at":"2026-09-23T19:43:58.123Z","writer":"0.1.0","data":{}}
```
A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`. Replay recovers only log-derived fields: `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`. Fields no event carries (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) are treated as absent. Liveness then rests on the heartbeat alone, so the instance is `live` or gone and never `stale`. The endpoint is rebuilt as described under IPC endpoints, and `hook statusline` prints nothing. As landed the fallback is library code in `viola-state` with six product readers, each after the instance's strict-modes check has passed: `viola revive`'s preflight takes it in its second reading, the logged session (chunk 2026-10-10-viola-revive; second of four since chunk 2026-10-16-unwrapped-sessions-on-the-board added `session-live`, the strict-modes check ahead of the preflight since chunk 2026-10-10-budget-governor), and `viola list`'s `viola_state::board::read` (chunk 2026-10-11-the-board-viola-list), which gives a `list` payload item for a replayed snapshot only when the beat is at most `STALE_AFTER` (5 s) old: `live`, never `stale`, `dialog_pending: false`, no CLI field, `wheel` / `budget_paused` only where the log gave one; an absent snapshot or a failed replay gives no item. The other four are `viola_state::liveness::live_endpoint` (Session liveness, below); `viola hook`'s `connect`, which reads a replayed or absent snapshot as unreachable, so the hook fails open; `hook statusline`'s `serve`, which finds no `statusline_command` in a replayed snapshot; and `run`'s collision check, which takes its caller's one read and judges a replayed snapshot by the heartbeat alone (`already-live` when the beat is at most `STALE_AFTER` old, else free; an absent snapshot or a failed read is free). `viola revive`'s preflight hands its one read to both its collision check and its recorded-directory reading. `snapshot::read_snapshot_classified` tells four cases apart through the same `MAX_FRAME` bound: the snapshot, no file, an unreadable one, and an unsupported one with the `v` it saw. It reads the envelope's `v` first: above 1 is unsupported whatever else the envelope holds, 1 must then parse whole, and anything else (a `v` of 0 included) is unreadable.
 `replay::read_snapshot_or_replay` replays the log for an unreadable or unsupported snapshot, in one pass, and logs one `state-recovered` line; an absent snapshot is neither replayed nor logged, and the replay writes no file. It recovers `wheel` and the budget gate from their last line and `agent_session_id` from the last `session-start` that carries one; a field no line gave is absent, never a default; `links` is recovered in the same pass through the public rule `viola_state::replay::apply_link_line` (chunk 2026-10-10-session-links): a `link` line whose `data.driver` and `data.driven` are strings adds `{driver, driven, since}` with `since` that line's `ts` unless the set holds the pair, an `unlink` line removes the pair, and a line of another kind or with a name that is no string changes nothing. A `link` line with no string `ts` gets `since: "unknown"`, the form `session_chain` takes for a missing `ts`; no writer of this build makes such a line, and the shape stands on the operator's word, the founder not having ruled on it. `read_snapshot` keeps its results and has no product caller (the rule: `read_snapshot(` under `src/`, outside test modules, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`). A replayed snapshot gives no `cwd`, so a revive over one is refused `cwd-missing`. Beside the replay, `replay::session_chain` reads the logged sessions for revive in one pass over `events.ndjson` through `events::read_from`: one link per `session-start` line whose `agent_session_id` is a string, with the pass's skip counts. It writes and logs nothing.

<!-- /detail -->

<!-- detail: values · contract-ledger-stamps -->

**Ledger stamps envelope** (`ledger/stamps.json`, atomically replaced JSON, written only by `viola verify` through `viola_state::stamps::update_stamps`)
```json
{"v":1,"written_at":"2026-09-28T15:14:02.311Z","writer":"verify","data":{"versions":{"2.1.283":{"verified_at":"2026-09-28T15:14:02.311Z","rows":{"shim-resolution":"pass"},"measured":{"largest_hook_payload":{"SessionStart":412}}}}}}
```
`writer` is the literal `"verify"`, not a binary version. `rows` maps each ledger row id to `"pass"` or `"fail"`; `measured` holds the probe's readings: `largest_hook_payload` per spine event, in bytes, and `typed_probe` `{ready_settle_ms, turn_settle_ms, prompt_latency_ms, max_turn_gap_ms, compact_latency_ms}` from verify's interactive runs (`compact_latency_ms` from Run B's `/compact` paste to the instant its answer was read; additive, no `v` bump), and `dialog_probe` `{parallel_both_before_first_post}` (bool or null, from Run C's captures; additive, no `v` bump). `run` reads no field of `measured`. A merge replaces only its own version's entry and keeps unknown versions and unknown top-level and `data` fields; bytes of the wrong shape are replaced whole. A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`), so an all-`pass` stamp of the eighteen pre-2026-10-13-owed-ledger-rows ids, of the twenty-one pre-2026-10-14-resume-and-compact-ledger-rows ids, of the twenty-two pre-2026-10-16-unwrapped-sessions-on-the-board ids or of the twenty-three pre-2026-10-18-compact-post-condition-ledger-row ids reads unverified on every OS, so do one of the eighteen and `resume-session` and one of the eighteen, `resume-session` and `agents-join` on Windows, and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only. A non-`v:1` or wrong-shaped envelope is `AgentError::StampsMalformed` to a reader (`ledger::verified` returns `Result<bool, AgentError>`), and `run`'s gate then records `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}` line. `run`'s gate reads the file through `viola_state::stamps::read_stamps_strict`: a strict-modes refusal is the same outcome, its detail `strict-modes-failed`.

<!-- /detail -->

<!-- detail: values · contract-ledger-stamps -->

- `viola_state::stamps::update_stamps` runs `strict::check_stamps` on the home before it creates `ledger/` and again under its lock before its capped read; a refusal reads and writes nothing and returns `StampsError::StrictModes`. Its error is `StampsError` (`StrictModes` · `State`), each variant with a fixed message.

<!-- /detail -->

<!-- detail: values · contract-instance-snapshot -->

- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.
- The set changes on an implicit link and on a served `link` / `unlink` call: `LinkSet::link` adds the pair through `record`, and `LinkSet::unlink` removes it through `remove`.
- A `link` of a pair the set holds appends nothing and answers that pair's `since` with `already_linked: true`; a new pair answers its new `link` line's `ts` as `since`, with `already_linked: false`.
- `unlink` answers `{}` whether or not the set held the pair; an absent pair appends nothing and leaves the set as it was.
- A `link` or `unlink` answered `-32602` (its causes: §Standard Contracts, Channel methods) writes no line, and a failed record or remove is `-32603`.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, compares the connected stream's peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and stays a recorded gap.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- Before that comparison each client runs the instance's strict-modes check, and a refused check writes no frame and no line: `client::refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the hook's `trusted` for `hook.event` and `hook.dialog`, and `strict::check_instance` in the MCP frame tools. Its one snapshot read is `replay::read_snapshot_or_replay`, and only a real snapshot goes on to the comparison.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `send` `{text, from?}` → `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`. `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste. A `send` reads the wrapper's budget gate once at its arrival ([Budget Governor]); a closed gate is the refusal `budget-paused` with detail `five-hour` or `seven-day`, before `send-issued`, with no cursor, nothing typed and no flight reserved, and a gate change that cannot be recorded there answers `-32603`. Since chunk 2026-10-10-session-links a `send` whose `from` is a name other than the wrapper's own records the link `from → this instance` ([Links]) after the second wheel and running-turn reads and directly before `send-issued`: a new pair's `link` line stands directly before that send's `send-issued`, whose `cursor` is still the offset its own line starts at, and a pair the set holds adds nothing. A link that cannot be recorded answers `-32603` with nothing issued and nothing typed. A send refused at an earlier rung, one with no `from` and one whose `from` is the wrapper's own name record nothing.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `wait` `{after?, timeout_ms?, from?}` → `{event:<ndjson event line>, cursor}`, where `cursor` is the end offset of that event's line, or `{timed_out:true}`. `after` and `timeout_ms` are each absent or a non-negative integer that fits `u64` (`null`, a negative, a fraction or a string is `-32602`, `data: null`); unknown fields are ignored. The start offset is `after`, else `events.ndjson`'s end read at the call; a line counts only when it STARTS at or after it. A pending dialog comes first: an `after`-less `wait`, or one whose `after` is at or before the pending dialog's line, returns that dialog at once. Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake (`viola_core::EventKind::WAIT_WAKE`); the deadline is `now + timeout_ms` through `Instant::checked_add` (an unrepresentable instant is no deadline). The wake (`src/run/wait.rs` `WaitFeed`): one `Mutex` (a generation counter + the newest turn) and one `Condvar`; `WaitFeed::appending` runs each hook line's append inside the lock, then bumps the generation, keeps a `turn-ended` as the newest turn and wakes the parked calls. Hook lines signal, and of the wrapper's own records only `dialog-expired`, appended through `WaitFeed::appending` (the wrapper's `send-*`, `link`, `unlink` and start records never wake). A parked `wait` reads the generation before each scan, re-scans only when it moves, and its 20 ms `wait_timeout` re-reads the injected clock against the deadline, never the file; it holds its connection's worker thread until a wake, its deadline or the wrapper's exit.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `link` and `unlink` read no `from`: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` reads `unsupported-version`, `invalid-params` or `release-from-driver` for the three `-32602` faults and `None` for the other four.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `hook.event`, sent id-less by `hook` once its instance check (`trusted`) and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot or a failed server check sends nothing and the hook fails open): `{ts, event:{kind, data}}`. The wrapper re-validates the event (`kind` is one of the five hook kinds `session-start` · `prompt-submitted` · `turn-ended` · `session-end` · `activity`; `data` is an object; for `prompt-submitted`, `text` is a string and `origin` is `harness` · `human`) and appends `{v, ts:<append time>, instance, kind, source:"hook", data}` to `events.ndjson`. An invalid event is not appended, and an append failure is `-32603`. A `prompt-submitted` whose `text` exactly equals the in-flight `send`'s typed text (the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters; any `origin` the hook filed) is appended with `origin:"driver"` instead, and only then settles that send ([Delivery Confirmation]). The one in-flight send no `prompt-submitted` claims is a listed local command waiting for its post-condition: a `session-start` whose `cause` is `clear` and whose `agent_session_id` is a string differing from the remembered one is appended unchanged and only then settles it.

<!-- /detail -->

<!-- detail: values · contract-channel-methods -->

- `list` is not a channel method. The calling process reads `instances/*/` (heartbeat and snapshot) and `claude agents --json` itself, so `list` needs no live wrapper. Its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, with the item, `budget` and `skipped` shapes of the `list` payload above. As landed (chunk 2026-10-11-the-board-viola-list), CLI `list` reads `<home>/instances/` (`viola_state::board::scan`, then `board::read`) and `budget.json` (through `read_budget_stamped`) and contacts no wrapper. Since chunk 2026-10-16-unwrapped-sessions-on-the-board it then makes one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from its current directory) and joins it by `viola_state::board::join`: a row matching a wrapped item's `child_pid` or `agent_session_id` sets that item's `status` to `idle` or `busy`; every unjoined row is an unwrapped item (`wrapped: false`, its name the row's `sessionId`, `live`, `dialog_pending: false`, no `wheel` / `budget_paused` / `cli_version` / `cli_verified`), sorted by name after the wrapped items. A failed, oversize or malformed read leaves the board as `board::read` gave it (every `status` `unknown`, no unwrapped item). The join field is ledger row `agents-join`. MCP `list` (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read → document, the provider `viola mcp` hands `viola_mcp::serve`) and returns the document's `ok`, or `instance-unreachable` for the board's refusal; CLI `list` keeps its own order (scan, its log opened, read, document).

<!-- /detail -->

<!-- detail: values · contract-event-data -->

- `prompt-submitted`: `{text, origin}`. `text` is the submitted prompt as it was typed: `viola-agent-claude` removes any `<pasted_content id=…>` long-paste wrapper together with the frame the CLI writes around it (two newlines before the open tag, one after the close tag, and a second after it when text follows that is not the next pair's own frame) and then reverses tag escaping before emitting the event, so for a wrapped paste `text` is the pasted text, and delivery matching compares it directly with the in-flight send's typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters ([Delivery Confirmation]). For a `send` whose text ended in CR or LF characters, or held a CR or a CR LF inside, `text` is the text as typed, not as sent. The CLI drops a pasted text's last newline before the hook, wrapped or not (as measured on 2.1.287, chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host, `evidence/scratch-session.md`), so `text` for a pasted text that ended in a newline is one byte short of it. No `send` relies on this: `send` never types a text that ends in a newline. `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.

<!-- /detail -->

<!-- detail: values · contract-cli-json-output -->

- `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21), with stderr empty and no line in any log.

<!-- /detail -->

<!-- detail: values · contract-cli-json-output -->

- `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.
- `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`; for each of the five codes `message` is the fault's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with `data` null; every other known code reads `data` null.
- A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP, and the human line reads `error: wrapper fault  <code>`.

<!-- /detail -->

<!-- detail: values · contract-hook-statusline -->

- `viola hook statusline` is exempt from both rules, because it is the statusline wrapper, not a Claude Code hook: a hidden arm of `hook`, taken on the word `statusline` before the event is mapped, so `HookEvent` keeps its nine events. It records the `rate_limits` reading to `budget.json` and runs the user's own statusline command with the same stdin. With `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through ([Hook Contract]): no instance check, no `budget.json` write, no command run, nothing printed; without `--capture` the pass-through is unchanged.
  - The source rule. `run` reads that command at start from one named source, which viola only reads: `<viola home>/statusline-source.json` (the Claude settings shape) whenever it exists; else, only in the default home, the user's `<user home>/.claude/settings.json`; else none. So a session in a home named by `--home` does not show the user's own status line until `statusline-source.json` is planted there. The reader is `viola_agent_claude::statusline::user_command`: a command only from a `statusLine` of type `command`, non-empty, with no NUL. `run` records it in the instance snapshot as `statusline_command`, and `hook statusline` reads it from there through `VIOLA_DIR`. Two files deciding a command viola later runs is a boundary widening the founder ratified, choosing the file in the viola home (live, 2026-10-10, relayed verbatim by the operator).
  - The arm's order, as landed (chunk 2026-10-10-statusline-pass-through): it canonicalises the instance directory and runs the instance check (`canonical` and `trusted` in `src/cmd/hook.rs`, shared with the event and dialog paths; `trusted` runs `viola_state::strict::check_instance`), opens its log, reads stdin through `take(MAX_FRAME + 1)`, writes `budget.json` when the payload holds a `rate_limits` object, reads the snapshot through `replay::read_snapshot_or_replay` (a replayed one holds no `statusline_command`, so no command runs), then runs the command. The check comes before the log because opening the log sets an existing home to 0700, and a check made after that passes a home at 0770 (as measured at that chunk's `evidence/red-green.md` §3). A refused check ends the arm at once with exit 0, before anything under the home is read or opened (chunk 2026-10-10-budget-governor): no `config.json` read, no role file opened, no `hook-invoked` or `hook-decision` line, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found. Before that chunk the arm opened its log after a refused check, which narrowed the home, so the refusal lasted one invocation (as measured at that chunk's `evidence/red-green.md` (b)).
  - The shell-out. The command is a user-written shell string, so it runs as one argument of `/bin/sh -c` (`statusline::shell_argv`) with the same stdin, the arm's environment and working directory unchanged and its stderr discarded, bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`). This is the only shell-out in viola, a boundary widening the founder ratified (live, 2026-10-10, relayed verbatim by the operator). The shell is the ledger row `statusline-shell` (probed through Run B's `--shell "$0"` word, relied on Unix only, since chunk 2026-10-13-owed-ledger-rows; §Cross-cutting Patterns, Capability ledger). On Windows `shell_argv` returns `None` for every input and no command runs; the Windows shell-out is owed to the working-route entry "Windows-only live measurements" (the founder's word on the cut, same day and relay).
  - It prints that command's stdout unchanged, read through `take(MAX_FRAME)`. It prints nothing when the command fails or passes the bound, when no command is recorded, and on oversize or malformed stdin. It always exits 0 with empty stderr.

<!-- /detail -->

<!-- detail: values · contract-session-liveness -->

- The server identity read is `viola_state::liveness`'s. `live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot (a replayed or absent one, or a failed read, gives `None`), so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check. The closed decision enum `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares the snapshot's `pid` and `started_at` (whole seconds, read through its RFC 3339 form) with the observed peer pid and that process's OS start time; with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap. The joined `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` reads the observed start time through `process_start_time` (a zombie reads none) and returns `server_verdict`'s verdict.

<!-- /detail -->

<!-- detail: values · contract-session-liveness -->

- `claude agents --json` adds `status` (idle/busy) and lists unwrapped sessions as read-only rows. As landed (chunk 2026-10-16-unwrapped-sessions-on-the-board) it is spawned directly by CLI `list`, by `viola revive`'s preflight and by `viola verify`'s Run B (and, since chunk 2026-10-17-mcp-server-for-drivers, by the `mcp` process's `list` tool, below), through the root bin's `run::agents` (`read_logged`: the R8 strip, stdout through `take(MAX_FRAME + 1)` by `version_gate::run_capped`, killed at 5 s, one `agents-probe` spawn pair under an open log) and parsed by the pure `viola_agent_claude::agents::parse` into rows or a closed `AgentsFailure` (`oversize` · `malformed`); `list` and `revive` resolve `claude` from their current directory with the same npm-shim → `claude.exe` resolution `run` uses (`run::resolve_program`). The `mcp` process's `list` tool makes the same one read on each call (chunk 2026-10-17-mcp-server-for-drivers; the widening the founder's own answer ratified, that chunk's `inputs#I2` Q1): its provider is `list::board_document` (scan → read with the agents read and join → document), run on a blocking thread, its `agents-probe` pair in `mcp.ndjson`, the board's refusal returned as `instance-unreachable`. If it is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error. A row that matches a wrapped instance's `child_pid` or `agent_session_id` in its snapshot is merged into that instance's row, never listed separately. Both fields are exposed (`pid`, `sessionId`; as measured on 2.1.287), and the join is ledger row `agents-join`, relied on every OS.

<!-- /detail -->
