# Classified — obs-plan · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the deny ban and probe row are CI gate config, so the arch source line still holds
obs-scope-summary#17.2 · no · CI/CD stays not-instrumentable; the session adds no runtime instrumentation
obs-scope-summary#17.3 · no · CI/CD stays not product runtime; no fixture is among the reached files
obs-scope-summary#18.1 · no · the tracing facade and logger pin are unchanged (report §1: no dependency added or bumped)
obs-scope-summary#18.2 · no · frontend stays N/A; no frontend file is among the reached files
obs-scope-summary#18.3 · no · no CLI verb's sink, refusal or log-before-check order changed; the new line comes from the mcp process
obs-scope-summary#18.4 · no · the CLI service identity is unchanged (report §1)
obs-scope-summary#18.5 · no · the mcp cut's exits 0 and 11 are already in the listed set (no exit or detail code was added)
obs-scope-summary#19.1 · no · start-sequence span names and steps are unchanged (report §4); the replace compare read adds no event, field or detail
obs-scope-summary#19.2 · no · frontend stays N/A; no frontend file is among the reached files
obs-scope-summary#19.3 · no · no run code is among the reached files, so the run sink and terminal rule hold
obs-scope-summary#19.4 · no · the run service identity is unchanged (report §1)
obs-scope-summary#19.5 · no · no run code is among the reached files, so the panic-hook rule holds
obs-scope-summary#20.1 · no · no hook path changed (report §10), so the hook spans and lines hold
obs-scope-summary#20.2 · no · frontend stays N/A; no frontend file is among the reached files
obs-scope-summary#20.3 · no · no hook path changed (report §10), so the hook sink and its stdout and stderr rules hold
obs-scope-summary#20.4 · no · no hook path changed (report §10), so the hook identity and instance-check order hold
obs-scope-summary#20.5 · no · no hook path or perf row changed (report §10); the only fs.rs change is the replace compare read
obs-scope-summary#21.1 · no · viola-channel is not among the reached files, so its spans and corr field are unchanged
obs-scope-summary#21.2 · no · frontend stays N/A; no frontend file is among the reached files
obs-scope-summary#21.3 · no · mcp.ndjson stays the mcp side's log file (report §1: log file location re-read and unmoved)
obs-scope-summary#21.4 · no · the service identity is unchanged (report §1), so each side still logs its own process
obs-scope-summary#21.5 · no · no error type or mapping changed (report §7 and §11)
obs-scope-summary#22.1 · no · tool-call spans and the logger are unchanged (report §4 and §2); the stdin cut writes a log line, not a span
obs-scope-summary#22.2 · no · frontend stays N/A; no frontend file is among the reached files
