@row_add n=1 row=decision/wheel-starts-with-driver area=established-decisions rec=1,2
name: The wheel starts with the driver
rule: The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key.
@row_add n=2 row=decision/send-never-queued area=established-decisions rec=1,2
name: A send mid-turn or mid-send is refused, never queued
rule: A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
@row_add n=3 row=decision/nutype-newtypes area=established-decisions rec=3,4
name: nutype newtypes for validation
rule: Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
@row_add n=4 row=decision/plugin-compiled-into-binary area=established-decisions rec=5,6
name: Plugin compiled into the binary
rule: viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start.
why: The binary and the plugin can no longer drift apart.
@row_add n=5 row=decision/pinned-exe-copy area=established-decisions rec=5,6
name: Pinned exe copy keyed by version and content hash
rule: At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls.
why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
@row_add n=6 row=decision/plugin-only-in-wrapped-sessions area=established-decisions rec=7,8
name: Plugin loads only into wrapped sessions
rule: The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`.
why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola.
except: `viola verify`'s transient unwrapped children, all gone when verify ends.
@row_add n=7 row=decision/prompt-text-from-stdin-or-file area=established-decisions rec=9,10
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument.
@row_add n=8 row=absence/mobile-view area=established-decisions rec=11,12
capability: A mobile or phone view
@row_add n=9 row=contract/channel-methods/send area=standard-contracts rec=14,15,13
boundary: channel methods
item: send
promise: A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
@row_add n=10 row=contract/channel-methods/wait area=standard-contracts rec=16,15,13
boundary: channel methods
item: wait
promise: A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
@row_add n=11 row=contract/channel-methods/last area=standard-contracts rec=17,15,13
boundary: channel methods
item: last
promise: A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
@row_add n=12 row=contract/channel-methods/answer area=standard-contracts rec=18,15,13
boundary: channel methods
item: answer
promise: A request: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind.
@row_add n=13 row=contract/channel-methods/pause area=standard-contracts rec=19,15,13
boundary: channel methods
item: pause
promise: A request: `pause` `{from?}` answers `{wheel:"human"}`, the wheel moving to `human` / `manual-pause`.
@row_add n=14 row=contract/channel-methods/release area=standard-contracts rec=20,15,13
boundary: channel methods
item: release
promise: A request: `release` `{budget?:bool, from?}` answers `{wheel:<holder>, budget_paused:bool, override_until?}`.
@row_add n=15 row=contract/channel-methods/link area=standard-contracts rec=21,15,13
boundary: channel methods
item: link
promise: A request: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper.
