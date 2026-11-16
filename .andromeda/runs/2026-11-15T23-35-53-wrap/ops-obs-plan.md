@row_set n=1 row=contract/diag-line-fields/hook-events cell=promise seen=3 detector=D-obs-instrumentation
value: hook_event (one of ten closed hook names), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
basis: fct-vkofdjrk, fct-q6nhp2v3
because: The diag-line schema's hook_event enum gains stop-failure and now holds ten values, so the row's closed count of nine is false.
@row_add n=1 row=contract/turn-ended-event-data/cause area=log-format-json-schema fact=F1 detector=none
boundary: the turn-ended event line's data
item: cause
promise: Present only on a turn the CLI ended on an API error, holding one closed code word, api-error; absent on a turn ended by Stop.
basis: fct-vkofdjrk, fct-xbqeduib
because: The turn-ended event line's data gains a cause key on an API-error end only, a closed word, a promise at the event-line boundary no row of the area states.
@row_add n=2 row=decision/api-error-text-never-recorded area=8-pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: API-error text never recorded
rule: An API-error turn end's error text, error details and assistant message never reach an event line, the snapshot, a process-log line or a detail file; only the closed cause word crosses.
guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot
basis: fct-xbqeduib, fct-gyihyjgp, fct-d5pz7chk
because: The new StopFailure path drops the payload's error text, details and assistant message at the parse, stricter than the detail-file allowance the area's content rows give, so a row of its own must hold it.
@row_add n=3 row=decision/api-error-class-held-in-agent-crate area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: API-error class held inside the Claude agent crate
rule: The CLI's API-error class is computed inside the Claude agent crate, and no class word reaches event data, an event, a snapshot or a log line.
guard: api_error_class_maps_every_cli_error_word
state: provisional
basis: fct-xbqeduib, fct-eptmbfsh, fct-d5pz7chk, fct-tf4ifap3
because: The chunk adds a classification of the CLI's error word that stays inside the crate, applied ahead of the founder's ratification, and no row of the area says it.
@row_add n=4 row=decision/turn-end-cause-revalidated area=error-capture-reporting fact=F4 detector=none
name: Hook turn end appended only with a closed cause
rule: The wrapper appends a hook turn-ended only when its cause is absent or a word of the closed turn-end cause set; any other value is not appended.
guard: run_hook_event_turn_ended_with_a_cause_outside_the_closed_set_is_not_appended
basis: fct-xbqeduib, fct-gyihyjgp, fct-d5pz7chk
because: The wrapper's hook.event re-validation gains a closed check of the turn-ended cause, a rule the area's peer-reply screening row does not cover.
