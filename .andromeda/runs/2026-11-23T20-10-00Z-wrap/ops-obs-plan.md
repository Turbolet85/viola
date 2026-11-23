@row_set n=1 row=contract/diag-subject/hyprctl-probe cell=promise seen=1 fact=F1 detector=D-obs-instrumentation
value: Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn, for every request of its closed set, queries and dispatches alike; a new request adds no subject word.
basis: fct-lhh72hqo
because: The seam's closed set gains a query and a dispatch observed by the standing hyprctl-probe spawn pair alone, so the row must say the one subject covers every request and a new one adds none.
@row_set n=2 row=contract/parse-rejected/hyprctl-json cell=promise seen=1 fact=F1 detector=D-obs-instrumentation
value: A hyprctl reply that is oversize or malformed gives one parse-rejected line with parser hyprctl-json, written by the Hyprland seam and carrying codes only; every request of the seam's closed set shares that parser word, and a new request adds no parser or detail word.
basis: fct-lhh72hqo
because: The new monitors reply read is instrumented by the standing parse-rejected hyprctl-json word, so the row must say the one parser word serves every request and a new one adds none.
@row_add n=1 row=decision/monitors-reply-read-without-text area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Monitors reply read without text
rule: A Hyprland monitors reply is read as numbers and booleans only; no monitor text (name, description, make, model, serial) reaches any type, so no log line or record can carry one.
basis: fct-y4jmpjio
because: The chunk adds a compositor monitors reply read whose type holds no text field and drops reply text at the parse, a PII control the scrubbing section does not yet hold.
