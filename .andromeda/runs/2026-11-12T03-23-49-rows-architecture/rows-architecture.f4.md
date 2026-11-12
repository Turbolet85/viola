@row_add n=1 row=contract/channel-methods/unlink area=conventions rec=1,2,3
boundary: channel methods
item: unlink
promise: A request with an integer JSON-RPC id, monotonic per connection; `unlink` `{driver, driven}` answers `{}`, sent to the driven instance's wrapper.
@row_add n=2 row=contract/channel-methods/hook-dialog area=conventions rec=1,4,3
boundary: channel methods
item: hook.dialog
promise: A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
@row_add n=3 row=contract/channel-methods/hook-event area=conventions rec=1,5,3
boundary: channel methods
item: hook.event
promise: A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
@row_add n=4 row=contract/refusal-reason/human-typing area=conventions rec=6,7
boundary: RefusalReason
item: human-typing
promise: Serialised kebab-case; its detail is null or manual-pause.
@row_add n=5 row=contract/refusal-reason/budget-paused area=conventions rec=6,7
boundary: RefusalReason
item: budget-paused
promise: Serialised kebab-case; its detail is five-hour or seven-day.
@row_add n=6 row=contract/refusal-reason/unverified-cli area=conventions rec=6,7
boundary: RefusalReason
item: unverified-cli
promise: Serialised kebab-case; its detail is null.
@row_add n=7 row=contract/refusal-reason/not-delivered area=conventions rec=6,7
boundary: RefusalReason
item: not-delivered
promise: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_add n=8 row=contract/cli-exit-codes/0 area=conventions rec=8,9
boundary: CLI exit codes
item: 0
promise: ok.
@row_add n=9 row=contract/cli-exit-codes/1 area=conventions rec=8,9,10,11,12
boundary: CLI exit codes
item: 1
promise: Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
@row_add n=10 row=contract/cli-exit-codes/2 area=conventions rec=8,9
boundary: CLI exit codes
item: 2
promise: Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
