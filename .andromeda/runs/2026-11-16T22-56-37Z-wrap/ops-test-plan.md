@row_set n=1 row=contract/event-data/turn-ended cell=promise seen=3 detector=D-tests-count-literals
value: data holds the keys last_assistant_message, background_tasks, scheduled_wakes and cause; cause is present on an API-error turn end only, absent on a Stop, one of nine words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
basis: fct-mwx3vsm6
because: The turn-ended data key set moved from three keys to four with scheduled_wakes added, so the row's key list is stale.
@row_add n=1 row=contract/dialog-data/scheduled-wakes area=unit-test-strategy fact=F2 detector=D-tests-coverage
boundary: dialog and Stop event data
item: scheduled_wakes
promise: scheduled_wakes carries only the length of the Stop payload's session_crons array, no element read or kept, or null when absent or not an array; a non-array writes one drift entry expecting array to the instance detail file only, as background_tasks does.
basis: fct-wkxfnidr
because: The Stop hook now reads session_crons by length alone into scheduled_wakes, a sibling of the background_tasks promise this boundary already states.
@row_add n=2 row=contract/turn-ended/scheduled-wakes area=e2e-test-strategy fact=F1 detector=D-tests-coverage
boundary: turn-ended event data
item: scheduled_wakes
promise: The Stop payload's session_crons array length, or null when the key is absent, null or not an array and on an API-error end; it reaches events.ndjson, CLI wait --json and MCP wait only, never last, list, the snapshot or the replay.
basis: fct-wkxfnidr
because: turn-ended data gains the scheduled-wake count beside background_tasks, a new item at this boundary with its own reach.
