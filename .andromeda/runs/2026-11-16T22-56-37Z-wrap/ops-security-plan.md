@row_set n=1 row=decision/api-error-turn-end-text-dropped cell=rule seen=4 detector=none
value: StopFailure's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message, background_tasks and scheduled_wakes and one nine-word cause set word, crossing into events.ndjson, wait --json and MCP wait.
basis: fct-wkxfnidr, fct-mwx3vsm6, fct-e2rsvlwc
because: The StopFailure arm now writes scheduled_wakes null, so the enumeration of what an API-error end's turn-ended data holds gains that key and the row's «holds only» would otherwise be false.
@row_add n=1 row=decision/stop-session-crons-count-only area=input-validation fact=F1 detector=D-security-input
name: Stop session crons carried as a count only
rule: Stop's session_crons is carried only as its length, scheduled_wakes on turn-ended, no element read or kept; null when absent, null or not an array, a non-array also writing one drift entry to the diagnostics detail file; null on an API-error turn end.
why: A ratified boundary widening: one number crosses, no content.
guard: hook_a_stop_holding_scheduled_wakes_carries_their_count_and_no_content
state: provisional
basis: fct-wkxfnidr, fct-q5e2q7d3, fct-cp6xiw3d, fct-e2rsvlwc
because: A new external-input field crosses the hook boundary, validated as a length of ignored elements with a drift entry for a non-array, and the plan names it a sibling of stop-background-tasks-count-only that the section does not yet hold.
@row_add n=2 row=decision/scheduled-wake-count-reach area=data-protection fact=F2 detector=none
name: Scheduled-wake count reaches only the turn-end hops
rule: The scheduled-wake count reaches only the hops the background-task count reaches: the turn-ended line in events.ndjson, CLI wait --json and MCP wait; last, list, the snapshot and the replay do not carry it.
why: A ratified boundary widening held to one number beside the background-task count, no content.
state: provisional
basis: fct-wkxfnidr, fct-s2jm54rg
because: The chunk widens which data crosses out of the hook to a new count, and the plan of where it may and may not travel is a data-protection decision no row of the section states.
