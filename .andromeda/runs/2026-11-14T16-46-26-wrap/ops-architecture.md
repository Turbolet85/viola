@row_add n=1 row=absence/usage-budget-gate area=design-philosophy fact=F1 detector=none
capability: A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage
basis: fct-vwne6ov4
because: The budget gate and governor were removed whole, so the project no longer has the capability and the masters must say so.
@row_add n=2 row=decision/retired-exit-codes-never-reused area=conventions fact=F3 detector=none
name: Retired exit codes never reused
rule: A retired CLI exit code is never reused or renumbered: its number stays unassigned.
basis: fct-vwne6ov4
because: Exit 11 left with budget-paused and was retired, never renumbered or reused, which binds every later exit-code assignment.
@row_add n=3 row=decision/older-release-shapes-as-fixtures area=occupied-resources fact=F5 detector=none
name: Older-release shapes kept as per-version fixtures
rule: Older-release own-format shapes and retired words live as committed fixtures under a per-version fixtures folder, outside the identifier-probed source trees, and the fixture hygiene walk covers them.
why: An identifier probe that forbids a word and a compatibility case that must spell it cannot share a tree.
guard: contract_fixture_hygiene
basis: fct-tdrwno6x
because: The 0.1.0 snapshot, log and retired words went to fixtures/viola-0.1.0/, outside the probed trees and walked by the hygiene check.
@row_add n=4 row=decision/wrapper-appended-kinds-log-only area=standard-contracts detector=none
name: Wrapper-appended event kinds are log-only
rule: Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set.
except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
basis: fct-xxks7iqj
because: The budget-gate kind was removed from the wrapper's appends, so the row's kind list and its reason both lose the gate.
@row_set n=1 row=decision/nutype-domain-newtypes cell=rule seen=1 detector=none
value: Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).
basis: fct-xxks7iqj
because: Percent was removed from viola-core with the budget thresholds, so it is no longer a domain newtype.
@row_set n=2 row=decision/nutype-newtypes cell=rule seen=1 detector=none
value: Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.
basis: fct-xxks7iqj
because: Percent was removed from viola-core with the budget thresholds, so it is no longer a validated newtype.
@row_set n=3 row=decision/permission-request-question-stays-null cell=why seen=1 detector=none
value: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.
basis: fct-velgct6n
because: The relied ledger rows went from 24 to 23 with the statusline-rate-limits row removed, so a new row is now the 24th.
@row_set n=4 row=decision/run-start-order cell=rule seen=1 detector=none
value: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child.
basis: fct-xxks7iqj
because: The start no longer evaluates a budget gate, so that step leaves the start order.
@row_set n=5 row=decision/test-isolated-viola-home cell=rule seen=1 detector=none
value: Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
basis: fct-vwne6ov4
because: budget.json no longer exists, so tests no longer share or isolate it.
@row_set n=6 row=contract/send-refusal/order cell=promise seen=1 detector=none
value: `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait.
basis: fct-xxks7iqj
because: The budget rung left the send refusal order.
@row_set n=7 row=decision/json-field-names-snake-case cell=except seen=1 detector=none
value: `last_assistant_message` stays as Claude emits it.
basis: fct-vwne6ov4
because: The rate-limits reading and its resets_at and used_percentage fields are gone from every viola format.
@row_set n=8 row=decision/protocol-version-bump-rule cell=except seen=1 detector=none
value: Removing the budget gate's fields and event kind kept every own-format `v` at 1 (snapshot, channel params, event line, list document); readers skip the removed keys and kind.
basis: fct-qlalaysz
because: The founder ruled that every own-format v stays 1 for this removal, written as an exception to the bump rule.
@row_set n=9 row=decision/unparseable-external-values-unknown cell=rule seen=2 detector=none
value: An unparseable external value becomes the string `"unknown"` in viola's own records, never an error.
basis: fct-vwne6ov4
because: resets_at, used_percentage and the statusline window are no longer read, so the row no longer covers them.
@row_set n=10 row=contract/cli-exit-codes/11 cell=promise seen=1 detector=none
value: Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
basis: fct-velgct6n
because: budget-paused is gone and nothing maps to exit 11, which stays retired and is never reused.
@row_set n=11 row=contract/cli-verbs/release cell=promise seen=1 detector=none
value: `viola release <target>`; a `--budget` flag is a usage error (exit 2).
basis: fct-tjydxlay
because: release's --budget flag was removed and clap now refuses it with exit 2.
@row_set n=12 row=contract/mcp-tool-results/success cell=promise seen=1 detector=none
value: A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, skipped}` payload.
basis: fct-xxks7iqj
because: The list ok payload lost its budget member.
@row_set n=13 row=contract/refusal-order/send cell=promise seen=1 detector=none
value: control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
basis: fct-xxks7iqj
because: The budget rung left the send refusal order.
@row_set n=14 row=decision/list-reads-state-directly cell=rule seen=1 detector=none
value: `list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
basis: fct-xxks7iqj
because: list no longer reads a budget file or thresholds.
@row_set n=15 row=decision/snapshot-replay-log-fields-only cell=rule seen=1 detector=none
value: A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default.
basis: fct-xxks7iqj
because: The replay now recovers links, agent_session_id and wheel alone, the budget gate fields removed.
@row_set n=16 row=decision/statusline-hook-exempt cell=rule seen=1 detector=none
value: `viola hook statusline` prints the user's statusline command's stdout unchanged and always exits 0 with empty stderr.
basis: fct-xxks7iqj
because: The statusline wrapper no longer reads or records rate_limits.
@row_set n=17 row=contract/channel-methods/release-type-check cell=promise seen=1 fact=F2 detector=none
value: A `release` whose `from` is of a non-string type, or that carries a `budget` key of any value (`null` included), is `-32602` `invalid-params` with `data: null`, no wheel moved and no record; only the string-`from` refusal comes before it.
basis: fct-xxks7iqj
because: Any budget key on release is now refused invalid-params instead of being read, the string-from refusal still first.
@row_set n=18 row=contract/channel-methods/plain-release cell=promise seen=1 detector=none
value: A `release` on a driver-held wheel changes nothing.
basis: fct-xxks7iqj
because: release has no budget param or budget_paused answer any more, so only the driver-held clause stays true.
@row_set n=19 row=contract/channel-methods/release cell=promise seen=1 detector=none
value: A request: `release` `{from?}` answers `{wheel:<holder>}`.
basis: fct-velgct6n
because: release params went from {budget?, from?} to {from?} and the reply to {wheel}.
@row_set n=20 row=contract/hook-statusline/capture cell=promise seen=1 detector=none
value: With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no command run, nothing printed; without it the pass-through is unchanged.
basis: fct-xxks7iqj
because: The statusline arm writes no budget file in any mode.
@row_set n=21 row=contract/hook-statusline/refused-check cell=promise seen=1 detector=none
value: A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed and the home's mode left as found.
basis: fct-xxks7iqj
because: budget.json no longer exists for the arm to leave untouched.
@row_set n=22 row=contract/instance-snapshot/data cell=promise seen=1 detector=none
value: `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
basis: fct-xxks7iqj
because: The snapshot no longer holds budget_paused or budget_override_until.
@row_set n=23 row=contract/list-item/wrapped cell=promise seen=1 detector=none
value: false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, cli_version or cli_verified.
basis: fct-xxks7iqj
because: The list item lost its budget_paused field.
@row_set n=24 row=contract/list-payload/items cell=promise seen=1 detector=none
value: Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?}`.
basis: fct-xxks7iqj
because: The list item lost its budget_paused field.
@row_set n=25 row=contract/list-payload/ok cell=promise seen=1 detector=none
value: The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.
basis: fct-velgct6n
because: The list payload went from {items, budget, skipped} to {items, skipped}.
@row_set n=26 row=contract/mcp-refusal-result/shape cell=promise seen=1 detector=none
value: `{"content":[{"type":"text","text":"refused: human-typing"}],"isError":true,"structuredContent":{"refusal":"human-typing","detail":null}}`
basis: fct-vwne6ov4
because: budget-paused and its MCP sample were retired, so the shape is shown on a refusal that stands.
@row_set n=27 row=contract/snapshot-envelope/unreadable-snapshot cell=promise seen=1 detector=none
value: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`; the replay writes no file.
basis: fct-xxks7iqj
because: The replay now recovers links, agent_session_id and wheel alone.
@row_set n=28 row=decision/config-bad-value-falls-back cell=rule seen=1 detector=none
value: An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded.
basis: fct-xalfd6ij
because: The budget thresholds left config.json, so no value concerns a budget gate.
@row_set n=29 row=decision/config-falls-back-to-defaults cell=why seen=1 detector=none
value: so no value makes the dialog hold zero or unbounded
basis: fct-xalfd6ij
because: The budget thresholds left config.json, so no value concerns a budget gate.
@row_set n=30 row=decision/list-home-read-rule cell=except seen=1 detector=none
value: mcp's start and plugin install keep the write rule on the home.
basis: fct-xxks7iqj
because: list makes no budget read any more.
@row_set n=31 row=decision/refused-statusline-check-no-trace cell=rule seen=1 detector=none
value: A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run and nothing printed.
basis: fct-xxks7iqj
because: budget.json no longer exists for the arm to leave untouched.
@row_set n=32 row=decision/agent-claude-pure cell=rule seen=1 detector=none
value: viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin.
basis: fct-vwne6ov4
because: The budget file and its writer were removed.
@row_set n=33 row=decision/root-bin-consumes-normalised-events cell=rule seen=2 detector=none
value: The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
basis: fct-xxks7iqj
because: The budget governor was removed from the root bin.
@row_set n=34 row=decision/mixed-version-tolerance cell=rule seen=1 fact=F4 detector=none
value: Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
basis: fct-fke6bmz4
because: A budget-era snapshot and log read whole with their retired keys and lines skipped, and a stamp holding the retired row verifies on the relied rows.
@row_retire n=1 row=decision/budget-default-thresholds seen=1 detector=none
why: The budget thresholds were removed with the gate.
basis: fct-vwne6ov4
because: five_hour and seven_day and their defaults were retired whole.
@row_retire n=2 row=decision/budget-gate-fail-open seen=1 detector=none
why: The budget gate no longer exists.
basis: fct-vwne6ov4
because: The gate and governor were retired whole.
@row_retire n=3 row=decision/budget-gate-fails-open seen=1 detector=none
why: The budget gate no longer exists.
basis: fct-vwne6ov4
because: The gate and governor were retired whole.
@row_retire n=4 row=decision/budget-gate-never-polls seen=1 detector=none
why: The budget gate is no longer evaluated anywhere.
basis: fct-xxks7iqj
because: No start, send or turn-end evaluation of a budget gate remains.
@row_retire n=5 row=decision/budget-governor-soft-stop seen=1 detector=none
why: The budget governor no longer exists.
basis: fct-vwne6ov4
because: The gate and governor were retired whole.
@row_retire n=6 row=decision/budget-pause-gates-only-send seen=1 detector=none
why: budget-paused no longer exists.
basis: fct-vwne6ov4
because: The budget-paused refusal was retired.
@row_retire n=7 row=decision/budget-pause-lift seen=1 detector=none
why: No budget pause exists to lift.
basis: fct-vwne6ov4
because: The pause lift and release --budget were retired.
@row_retire n=8 row=decision/budget-readings-last-writer-wins seen=1 detector=none
why: No budget reading is kept.
basis: fct-vwne6ov4
because: budget.json and its single-writer decisions were retired.
@row_retire n=9 row=decision/budget-soft-stop seen=1 detector=none
why: The budget governor no longer exists.
basis: fct-vwne6ov4
because: The gate, governor and thresholds were retired whole.
@row_retire n=10 row=decision/only-send-reads-budget-gate seen=1 detector=none
why: No verb reads a budget gate, and its Path 6 guard was deleted.
basis: fct-vwne6ov4
because: The gate and the Path 6 suite were retired.
@row_retire n=11 row=decision/release-budget-per-instance seen=1 detector=none
why: release --budget no longer exists.
basis: fct-vwne6ov4
because: The flag, the budget param and override_until were retired.
@row_retire n=12 row=decision/tolerant-resets-at seen=1 detector=none
why: No external resets_at is read any more.
basis: fct-xxks7iqj
because: The rate_limits parse that read resets_at was removed.
@row_retire n=13 row=contract/normalised-event-kinds/budget-gate seen=1 detector=none
why: budget-gate is no longer a normalised event kind.
basis: fct-xxks7iqj
because: EventKind::BudgetGate was removed from viola-core.
@row_retire n=14 row=contract/refusal-reason/budget-paused seen=1 detector=none
why: budget-paused is no longer a RefusalReason.
basis: fct-xxks7iqj
because: RefusalReason::BudgetPaused was removed; an older wrapper's word reads unknown.
@row_retire n=15 row=decision/wrapper-events-log-only seen=1 heir=decision/wrapper-appended-kinds-log-only detector=none
why: Its kind list and its reason both name the removed budget-gate kind.
basis: fct-xxks7iqj
because: Two cells name the removed kind, so the row is replaced by an heir without it.
@row_retire n=16 row=contract/channel-methods/budget-release-durability seen=1 detector=none
why: A release takes no budget key and nothing records a budget lift.
basis: fct-xxks7iqj
because: Any budget key on release is now refused invalid-params with no record.
@row_retire n=17 row=contract/channel-methods/release-budget-true seen=1 detector=none
why: budget:true is refused invalid-params and lifts nothing.
basis: fct-xxks7iqj
because: Any budget key on release is now refused invalid-params.
@row_retire n=18 row=contract/channel-methods/send-budget-paused seen=1 detector=none
why: No budget gate refuses a send.
basis: fct-xxks7iqj
because: The send path's budget rung was removed.
@row_retire n=19 row=contract/channel-send/budget-paused seen=1 detector=none
why: No budget gate refuses a send.
basis: fct-xxks7iqj
because: The send path's budget rung was removed.
@row_retire n=20 row=contract/event-data/budget-gate seen=1 detector=none
why: The budget-gate kind and its data keys no longer exist.
basis: fct-xxks7iqj
because: EventKind::BudgetGate and its paused, window and override_until keys were removed.
@row_retire n=21 row=contract/list-item/budget-paused seen=1 detector=none
why: The list item has no budget_paused field.
basis: fct-xxks7iqj
because: board::Item.budget_paused was removed.
@row_retire n=22 row=contract/list-payload/budget seen=1 detector=none
why: The list payload has no budget member.
basis: fct-velgct6n
because: The list payload went from {items, budget, skipped} to {items, skipped}.
@row_retire n=23 row=contract/wrapper-event-appends/budget-gate seen=1 detector=none
why: The wrapper appends no budget-gate line.
basis: fct-xxks7iqj
because: A start writes one wheel record then session-start, and no budget-gate kind remains.
@row_retire n=24 row=decision/budget-json-single-writer seen=1 detector=none
why: budget.json is no longer written or read.
basis: fct-vwne6ov4
because: budget.json, its writer, readers and single-writer decisions were retired.
@row_retire n=25 row=decision/shared-budget-reading seen=1 detector=none
why: No budget reading is kept.
basis: fct-vwne6ov4
because: budget.json, its writer, readers and single-writer decisions were retired.
@row_retire n=26 row=contract/budget-json/five-hour seen=1 detector=none
why: budget.json no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields were retired.
@row_retire n=27 row=contract/budget-json/read-at seen=1 detector=none
why: budget.json no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields were retired.
@row_retire n=28 row=contract/budget-json/seven-day seen=1 detector=none
why: budget.json no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields were retired.
@row_retire n=29 row=contract/budget-json/v seen=1 detector=none
why: budget.json no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields were retired.
@row_retire n=30 row=contract/config-json/five-hour seen=1 detector=none
why: five_hour is no longer a config.json key.
basis: fct-xalfd6ij
because: schemas/config.v1.json dropped five_hour and CONFIG_KEYS no longer holds it.
@row_retire n=31 row=contract/config-json/seven-day seen=1 detector=none
why: seven_day is no longer a config.json key.
basis: fct-xalfd6ij
because: schemas/config.v1.json dropped seven_day and CONFIG_KEYS no longer holds it.
