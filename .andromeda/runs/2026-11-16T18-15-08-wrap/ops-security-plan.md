@row_set n=1 row=contract/channel-frames/hook-event-revalidation cell=promise seen=2 detector=D-security-input
value: kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; a turn-ended cause, when present, is a string in the closed nine-word turn-end cause set; an invalid event is not appended.
basis: fct-xw6ru4by
because: The turn-end cause set the wrapper re-checks grew from api-error alone to nine closed words, so the promise naming one word is false.
@row_set n=2 row=decision/api-error-turn-end-text-dropped cell=rule seen=1 detector=none
value: The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and one closed nine-word cause set word, which crosses into events.ndjson, wait --json and MCP wait.
basis: fct-pqfgw3kz
because: The cause word an API-error turn end carries is now one of nine closed words rather than api-error alone, while the error text still crosses nothing.
@row_retire n=1 row=decision/api-error-classes-adapter-only seen=1 heir=decision/api-error-cause-classified-in-adapter detector=none
why: The classification no longer stays inside the adapter: it crosses as one of the nine closed turn-end cause words, ratified, so the provisional rule that no class is written out is false.
basis: fct-vldmp3ur
because: ApiErrorClass was retired and the classified word now crosses into event data as the cause, while the classifier itself stays in the adapter.
@row_add n=1 row=decision/api-error-cause-classified-in-adapter area=data-protection detector=none
name: API-error cause classified inside the Claude adapter
rule: The CLI's StopFailure error word is classified into one of nine closed turn-end cause words inside the Claude adapter crate only; only that cause word crosses out of the adapter, never the error word or any error text.
why: Ratified; the classification crosses as one closed word so that no upstream text does.
basis: fct-rexe2a5m
because: The nine cause words crossing out of the adapter stand as ratified rows, replacing the provisional rule that kept every class inside the adapter.
@row_add n=2 row=contract/hook-stdin/unknown-value area=input-validation fact=F1 detector=D-security-input
boundary: hook stdin
item: unknown-value
promise: A StopFailure error value outside the CLI's closed word list is delivered with cause api-error and noted by exactly one WARN parse-rejected line, parser hook-stdin detail unknown-value count 1, carrying the code and count only, never the value.
basis: fct-xw6ru4by
because: The hook now marks an outside StopFailure error value and logs it codes-only while still delivering the event, a new promise at the hook stdin boundary.
@row_none n=1 fact=F2 type=9
what: The live reading of the CLI's StopFailure error words against real payloads is an owed item, carried by the route on the recorded fixture set.
