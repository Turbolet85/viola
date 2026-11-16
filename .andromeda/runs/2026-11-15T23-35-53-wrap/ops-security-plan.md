@row_set n=1 row=decision/hook-unknown-event-silent-exit cell=rule seen=2 detector=none
value: The hook event argument maps into a closed set of ten events and no other word; an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing read or written anywhere.
basis: fct-q6nhp2v3
because: StopFailure joins the registered hook events, so the closed set the row counts moves from nine to ten.
@row_set n=2 row=contract/channel-frames/hook-event-revalidation cell=promise seen=1 fact=F1 detector=D-security-input
value: kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; a turn-ended cause, when present, is a string in the closed turn-end cause set (api-error alone); an invalid event is not appended.
basis: fct-xbqeduib
because: The wrapper's hook.event re-validation now also holds a turn-ended cause to the closed turn-end cause set before appending, a new check at this channel boundary.
@row_set n=3 row=decision/unverified-cli-gate cell=except seen=1 fact=F4 detector=none
value: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set; this exception is provisional.
basis: fct-klhffiim
because: StopFailure is registered as the named exception to the unverified-CLI capability gate, provisional on founder fork F1, and the gate row names no exception.
@row_add n=1 row=decision/api-error-turn-end-text-dropped area=data-protection fact=F2 detector=none
name: API-error turn end carries no error text
rule: The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and the closed cause word api-error, which crosses into events.ndjson, wait --json and MCP wait.
why: A boundary widening held to one closed word, so the upstream error text crosses into nothing.
guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot
state: provisional
basis: fct-xbqeduib
because: The chunk adds the turn-ended cause word crossing outward and drops the payload's error text at the parse, a data-protection decision no row of the section states.
@row_add n=2 row=decision/api-error-classes-adapter-only area=data-protection fact=F3 detector=none
name: API-error classes kept inside the Claude adapter
rule: The CLI's StopFailure error word is classified into nine closed API-error classes inside the Claude adapter crate only; no class is written to event data, an event line, the snapshot or a log line.
state: provisional
basis: fct-eptmbfsh
because: The chunk adds an API-error classification that is held inside the adapter crate and crosses no output, a data-protection decision no row of the section states.
