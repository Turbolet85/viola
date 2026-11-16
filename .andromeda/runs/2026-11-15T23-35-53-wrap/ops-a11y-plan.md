@row_add n=1 row=decision/api-error-turn-end-closed-cause area=cognitive-accessibility fact=F1 detector=none
name: API-error turn end reported by a closed cause word
rule: A turn the agent CLI ends on an API error is reported as turn-ended carrying one closed cause code word, api-error; the vendor's error text never reaches any output, and finer error classes stay inside the adapter.
guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot
state: provisional
basis: fct-xbqeduib
because: The chunk maps the stop-failure hook to turn-ended with the closed cause api-error and drops the vendor error text from every event line, snapshot and diagnostics file, provisional on founder fork F2, and no standing row of the section says so.
