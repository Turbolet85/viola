@row_add n=1 row=decision/stop-session-crons-count-only area=input-validation fact=F1 detector=D-security-input
name: Stop session crons carried as a count only
rule: Stop's session_crons is carried only as its length, scheduled_wakes on turn-ended, no element read or kept; null when absent, null or not an array, a non-array also writing one drift entry to the diagnostics detail file; null on an API-error turn end.
why: Ratified as a boundary widening: one number crosses beside the background-task count, no content.
guard: hook_a_stop_holding_scheduled_wakes_carries_their_count_and_no_content
basis: fct-wkxfnidr, fct-q5e2q7d3, fct-cp6xiw3d, fct-e2rsvlwc
because: A new external-input field crosses the hook boundary as a length of ignored elements with a drift entry for a non-array, the sibling of stop-background-tasks-count-only the section does not yet hold; the founder ratified the widening (ruling B), so the row stands with no provisional state.
