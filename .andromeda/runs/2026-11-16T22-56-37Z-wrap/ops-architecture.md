@row_set n=1 row=contract/event-data/turn-ended-last-assistant-message cell=promise seen=3 detector=D-arch-resources
value: `turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
basis: fct-wkxfnidr, fct-mwx3vsm6
because: The turn-ended data key set grew from three to four keys with `scheduled_wakes`, so the row's «carries no scheduled-wake count» is now false.
@row_set n=2 row=contract/hook-to-kind-map/stopfailure cell=promise seen=2 detector=none
value: StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
basis: fct-wkxfnidr, fct-mwx3vsm6
because: The StopFailure arm now writes `scheduled_wakes` null, so the row's exact data enumeration must name it.
@row_set n=3 row=contract/channel-methods/wait-wake-kinds cell=promise seen=2 fact=F2 detector=none
value: Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`; a finished or paused turn is for the driver to tell, and a parked `wait` never polls the file.
basis: fct-5zoszjnq, fct-e2rsvlwc
because: A scheduled wake is counted on the turn end but never a wake condition, so the wake rule names it beside `background_tasks`.
@row_add n=1 row=contract/event-data/turn-ended-scheduled-wakes area=standard-contracts fact=F1 detector=D-arch-resources
boundary: event data per kind
item: turn-ended scheduled_wakes
promise: `scheduled_wakes` counts the session's scheduled wakes: the Stop payload's `session_crons` array length with no element read or carried, `null` when the key is absent, `null` or not an array, and on an API-error turn end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait` only.
basis: fct-wkxfnidr, fct-q5e2q7d3
because: The chunk adds the `scheduled_wakes` key to turn-ended data, a new event field the Standard Contracts must register beside `background_tasks`.
