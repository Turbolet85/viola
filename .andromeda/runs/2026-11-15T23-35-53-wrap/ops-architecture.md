@row_set n=1 row=contract/event-data/turn-ended-last-assistant-message cell=promise seen=1 detector=D-arch-resources
value: `turn-ended` data is `{last_assistant_message, background_tasks, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
basis: fct-vkofdjrk
because: The turn-ended data gains a third key, cause, so the row's statement of the data shape no longer holds whole.
@row_set n=2 row=decision/capability-ledger-gate cell=except seen=1 fact=F7 detector=D-arch-decisions
value: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered, provisionally, with no ledger row and no verify probe.
basis: fct-klhffiim
because: StopFailure is relied on with no ledger row and no verify probe, a named provisional exception to the single-gate rule.
@row_set n=3 row=decision/cli-capability-ledger cell=except seen=1 detector=D-arch-decisions
value: StopFailure is registered, provisionally, with no ledger row and no verify probe.
basis: fct-klhffiim
because: The rule that every relied CLI behaviour is a ledger row now has StopFailure as a named exception.
@row_set n=4 row=decision/capability-ledger cell=except seen=1 detector=D-arch-decisions
value: StopFailure is registered, provisionally, with no ledger row and no verify probe.
basis: fct-klhffiim
because: The design rule that every relied CLI behaviour is a ledger row now has StopFailure as a named exception.
@row_set n=5 row=contract/hook-tiers/ordering-spine cell=promise seen=1 detector=none
value: SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
basis: fct-xbqeduib
because: StopFailure is registered in Stop's tier with the 5 s timeout, so the spine's member list is incomplete without it.
@row_set n=6 row=contract/hook-tiers/sync-spine cell=promise seen=1 detector=none
value: SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
basis: fct-xbqeduib
because: StopFailure joins Stop's synchronous tier, so the sync spine's member list is incomplete without it.
@row_add n=1 row=contract/hook-to-kind-map/stopfailure area=conventions fact=F1 detector=D-arch-resources
boundary: Hook to kind map
item: StopFailure
promise: StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`.
basis: fct-xbqeduib
because: A tenth hook event now maps to a normalised kind, and the hook-to-kind map holds one row per hook.
@row_add n=2 row=contract/turn-end-cause/cause-set area=conventions fact=F2 detector=D-arch-resources
boundary: Turn-end cause
item: cause set
promise: `turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, with one member `api-error` and no Claude-specific name; a `turn-ended` cause is one of its words.
basis: fct-xbqeduib
because: A new closed cause set lands in viola-core and bounds what a turn-ended cause may be.
@row_add n=3 row=contract/event-data/turn-ended-cause area=standard-contracts fact=F3 detector=D-arch-resources
boundary: event data per kind
item: turn-ended cause
promise: `cause` is present only on an API-error turn end, as `api-error`, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`, applied provisionally ahead of ratification.
basis: fct-vkofdjrk
because: The turn-ended event data gains a cause key that crosses outward to drivers, a widened boundary.
@row_add n=4 row=contract/hook-event/turn-ended-cause area=standard-contracts fact=F4 detector=D-arch-resources
boundary: channel hook.event
item: turn-ended cause
promise: A `turn-ended` whose data holds `cause` is appended only when `cause` is a TurnEndCause word; with `cause` absent it is appended unchanged by this check; any other value is not appended.
basis: fct-xbqeduib
because: The wrapper's hook.event re-validation gains a closed check on the new cause key.
@row_add n=5 row=decision/stop-failure-text-never-carried area=standard-contracts fact=F5 detector=none
name: StopFailure payload text never carried
rule: A StopFailure payload's error text, error_details and last_assistant_message are never copied into event data, an event line, the snapshot or a diagnostics file.
guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot
basis: fct-xbqeduib
because: The new hook carries upstream error text that the chunk deliberately drops at the parse, a rule future work must keep.
@row_add n=6 row=decision/api-error-class-in-agent-crate area=crate-dependency-direction fact=F6 detector=none
name: API-error classes held inside viola-agent-claude
rule: The CLI's StopFailure error word is classified into nine API-error classes only inside viola-agent-claude; normalisation does not use it and no class reaches event data, a snapshot or a log line.
guard: api_error_class_maps_every_cli_error_word
basis: fct-xbqeduib
because: A Claude-specific classification lands and is confined to the agent crate, a boundary later work must respect.
@row_add n=7 row=name/other/hook-stop-failure area=occupied-resources fact=F8 detector=D-arch-resources
class: other
name: hook stop-failure
basis: fct-xbqeduib
because: The hook event name stop-failure is newly registered, the tenth hook event, and so is occupied.
@row_add n=8 row=decision/api-error-turn-end-clears-turn area=standard-contracts fact=F9 detector=none
name: API-error turn end clears the running turn
rule: An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted.
guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
basis: fct-d5pz7chk
because: A turn the CLI ends on an API error now ends the running turn like a Stop, a behaviour drivers rely on.
