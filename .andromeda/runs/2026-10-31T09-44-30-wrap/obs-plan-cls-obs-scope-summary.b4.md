# Classified — obs-plan · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · yes · add-to · the oversize-line clause gains exit 1, since viola mcp exits 1, not 11, on a stdin line past MAX_FRAME before initialize
obs-scope-summary#22.4 · no · the identity still holds on the changed mcp file, as the logger that stamps process and instance is not reached and no env var such as VIOLA_NAME moved
obs-scope-summary#22.5 · no · the codes-only isError claim still holds on the changed result.rs and is now only witnessed
obs-scope-summary#23.1 · no · the init order and the no-exporter, no-dependency stance still hold, as main.rs and obs.rs are not reached and no dependency was added
obs-scope-summary#24.1 · no · the logger stack and sink rules still hold, as obs.rs is not reached and no sink or config key moved
obs-scope-summary#25.0 · yes · add-to · the channel-response entry gains a closed detail on the three -32602 faults beside its corr, while the required fields do not move
obs-scope-summary#26.1 · no · the diagnostics paths, file modes and retention still hold, as obs.rs is not reached and no config key or env var moved
obs-scope-summary#27.1 · no · the run and channel-bind surface chain still holds, as no role, span or exporter was added
obs-scope-summary#27.2 · no · the run spans and start events still hold on the changed run files, as no span or event value was added
obs-scope-summary#27.3 · no · the run exit-1 detail codes still hold on the changed run files, as the two refusal messages are unchanged and only the retired Refusal type moved
obs-scope-summary#27.4 · no · the source citation still holds, as no cited source moved
obs-scope-summary#28.1 · no · the send surface chain still holds, as no role, span or exporter was added
obs-scope-summary#28.2 · no · the send spans and event order still hold on the changed files, as no span or event value was added and the detail word is a field, not an event
obs-scope-summary#28.3 · yes · add-to · the channel-response lines on the send path gain closed -32602 detail words that the five send-refused words do not cover, while the required fields do not move
obs-scope-summary#28.4 · no · the source citation still holds, as no cited source moved
obs-scope-summary#29.1 · no · the wait surface chain still holds, as no role, span or exporter was added
obs-scope-summary#29.2 · no · the wait spans and channel-request and channel-response pair still hold on the changed files, as no span or event value was added
obs-scope-summary#29.3 · yes · add-to · the wait path's channel-response gains a closed -32602 detail beside its corr and outcome, while the required fields do not move
obs-scope-summary#29.4 · no · the source citation still holds, as no cited source moved
obs-scope-summary#30.1 · no · the dialog surface chain still holds, as no role, span or exporter was added
obs-scope-summary#30.2 · no · the dialog spans and events still hold on the changed dialog.rs, as no span or event value was added
obs-scope-summary#30.3 · no · the dialog refusal and detail codes and their exits still hold on the changed dialog.rs, as dialog lines carry the dialog_id corr and no channel-response detail
obs-scope-summary#30.4 · no · the source citation still holds, as no cited source moved
obs-scope-summary#31.1 · no · the wheel and release surface chain still holds, as no role, span or exporter was added
obs-scope-summary#31.2 · yes · add-to · release-from-driver also becomes a channel-response detail word, while its own logged event and exit codes stand
