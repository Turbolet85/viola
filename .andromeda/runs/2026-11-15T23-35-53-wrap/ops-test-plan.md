@row_set n=1 row=contract/fake-script/format cell=promise seen=1 detector=D-tests-count-literals
value: A script is {"v":1,"steps":[{"event","variant","gate"}]} over the ten registered hook events, validated by the fake-script v1 schema.
basis: fct-q6nhp2v3, fct-vkofdjrk
because: The registered hook events moved 9 to 10 with StopFailure, and the fake-script schema's event enum gained StopFailure, so «nine» is a stale count literal.
@row_set n=2 row=contract/event-data/turn-ended cell=promise seen=1 detector=D-tests-count-literals
value: data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only and absent on a Stop.
basis: fct-xbqeduib, fct-vkofdjrk, fct-q6nhp2v3
because: The turn-ended data keys moved 2 to 3 with cause, present on an API-error end only, so the row's key list is incomplete.
@row_set n=3 row=contract/viola-send/turn-running cell=promise seen=1 fact=F3 detector=none
value: While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.
basis: fct-d5pz7chk, fct-gyihyjgp
because: An API-error turn end now clears the running turn with no pause or release, so the row's accepted-after-turn-ended promise must name that end too.
@row_set n=4 row=decision/hook-payloads-recorded-by-verify cell=except seen=1 fact=F4 detector=none
value: StopFailure, provisionally, is registered with no recorded payload, no ledger row and no verify probe; its tests use a synthetic StopFailure payload.
basis: fct-klhffiim, fct-o6lv4y7d, fct-gglhmaiy
because: StopFailure joined the registered hooks without a recorded fixture, ledger row or verify probe (PROVISIONAL F1), a named exception to recording every hook payload by verify.
@row_add n=1 row=contract/turn-end-cause/api-error area=integration-test-strategy fact=F1 detector=none
boundary: turn-end causes
item: api-error
promise: The one word of the closed turn-ended cause set; the wrapper does not append a hook turn-ended whose cause is outside the set, and a turn-ended with cause absent is still appended.
basis: fct-xbqeduib, fct-gyihyjgp, fct-d5pz7chk
because: The chunk adds a closed turn-end cause set and the wrapper's re-validation of it on the hook.event channel input, a contract no row states.
@row_add n=2 row=decision/api-error-text-crosses-into-nothing area=e2e-test-strategy fact=F2 detector=none
name: API-error text crosses into nothing
rule: An API-error turn end's error text reaches no events.ndjson line, snapshot.json or diagnostics file, held by an absence probe that plants a value in the hook payload.
guard: hook_stop_failure_error_text_reaches_no_event_line_or_snapshot
basis: fct-gyihyjgp, fct-d5pz7chk, fct-xbqeduib
because: The new StopFailure hook carries the CLI's error text, and the chunk rules and probes that the text is dropped at the parse, a decision no row of the area states.
@row_add n=3 row=decision/send-cases-copy-recorded-fixtures area=test-data-fixtures fact=F5 detector=none
name: Send-after-turn cases copy the recorded fixtures
rule: A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it.
why: Synthetic payloads alone hold no screen rows, so the fake agent draws no input box and the send reads input-not-ready.
basis: fct-dmzhnpda, fct-gglhmaiy
because: The chunk measured that a purely synthetic fixtures root fails any send after the first turn, a convention future send cases must follow.
@row_add n=4 row=decision/api-error-classes-held-in-adapter area=unit-test-strategy fact=F6 detector=none
name: API error classes held inside the agent adapter
rule: The CLI's API error words are classified into nine classes inside the agent adapter alone; no class reaches event data, an event, a snapshot or a log line, and a unit table maps every CLI error word.
guard: api_error_class_maps_every_cli_error_word
state: provisional
basis: fct-xbqeduib, fct-eptmbfsh, fct-d5pz7chk
because: The chunk adds the API error classification and keeps it inside the adapter (founder fork F2, provisional), a decision no row of the area states.
