@row_add n=1 row=contract/parse-rejected-detail/unknown-value area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: parse-rejected detail
item: unknown-value
promise: hook-stdin: a hook stdin field held a value outside its closed set, first the StopFailure error word; the line carries the code and count only.
basis: fct-wnfbdzgw
because: The diag-line schema's parse-rejected detail enum gains unknown-value, and the boundary's closed list holds one row an item, so the new word owes its row.
@row_add n=2 row=contract/parse-rejected/hook-stdin-unknown-value area=log-coverage fact=F2 detector=D-obs-instrumentation
boundary: parse-rejected diagnostic line
item: parser hook-stdin, detail unknown-value
promise: A StopFailure error word outside the CLI's 13 writes one WARN parse-rejected line with parser hook-stdin, detail unknown-value and count 1, carrying the code and count only, and the event is still delivered as an api-error turn end; a known word writes no line.
basis: fct-xw6ru4by
because: The hook now writes a codes-only parse-rejected line for an outside StopFailure error word, a new line other readers of the role file rely on.
@row_add n=3 row=decision/stopfailure-error-text-never-logged area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: StopFailure error text never logged
rule: The CLI's StopFailure error text and error_details are read only to classify the turn-end cause and reach no log line, detail line or event; only the closed cause word crosses out of the adapter.
basis: fct-pmkk6ij2
because: The hook now classifies the StopFailure error word into a cause word, and no standing row of the section says its error text stays out of every log and event.
@row_set n=1 row=decision/parse-failures-unknown-counter cell=except seen=2 detector=D-obs-instrumentation
value: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
basis: fct-pqfgw3kz
because: An outside StopFailure error word now surfaces as the cause api-error with one parse-rejected unknown-value line, a second exception to the unknown value the rule names.
@row_set n=2 row=decision/parse-failures-surface-as-unknown cell=except seen=2 detector=D-obs-instrumentation
value: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
basis: fct-xw6ru4by
because: An outside StopFailure error word now surfaces as the cause api-error with one parse-rejected unknown-value line, a second exception to the unknown value the rule names.
