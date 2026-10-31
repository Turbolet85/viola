# obs-plan · picked accounting · group 1 of 4

## Picked
obs-scope-summary#2.3 · not touched · the line names `event` enum extensions, and the report's only schema edit closes `channel-response`'s `detail` at `schemas/diag-line.v1.json:42`, so no event value moves
obs-scope-summary#2.4 · not touched · `hook-invoked`'s `corr` timing: the report moves no hook line, and `src/cmd/hook/statusline.rs` gains only test lines
obs-scope-summary#4.3 · not touched · `viola-core` is reached, but `is_code_word` is a pure predicate over `&str`, so the crate still does no I/O and emits nothing
obs-scope-summary#12.3 · not touched · the report has `src/run/agents.rs` reading `resolve_program` through `let Ok(..) else`, unchanged, so spawn, exit and parse-outcome instrumentation holds
obs-scope-summary#13.3 · not touched · `src/cmd/hook/statusline.rs` is reached with cap-driving test cases only, and the report adds no line, so the duration-and-status rule holds
obs-scope-summary#14.3 · not touched · `src/cmd/client.rs` is reached in `reply_of` only, no public signature moved and no line added, so the call-site and `server-verify-failed` rule holds
obs-scope-summary#15.3 · not touched · the store scrub rules hold, the report's witnesses reading error bodies clean of absolute paths, and no store reader is changed
obs-scope-summary#18.4 · not touched · `Cargo.toml` gains `[[test]]` tables, no version-census row applies and no config key or env var moved, so service identity holds
obs-scope-summary#19.3 · not touched · `src/run/mod.rs` moves `resolve_program`'s error type only, with outputs unchanged, so `run`'s file sink and terminal rule hold
obs-scope-summary#19.4 · not touched · `run`'s `process` and `instance` identity is not moved by the wrapper's new return type
obs-scope-summary#20.3 · not touched · `viola hook statusline`'s exporter holds, its reached file gaining test lines only and no output moved
obs-scope-summary#20.4 · not touched · the hook's identity and instance-check order hold, the report moving no hook path outside the statusline's tests
obs-scope-summary#20.5 · not touched · the `max` figures are anchored to their named 2026-10-26 gate file, the report's `run --perf` moved no bound, and no `run --perf` row times the statusline
obs-scope-summary#21.4 · not touched · the channel crates are reached, yet the report states the wire form unchanged and `v` 1, so each side's `process` and `params.sender` hold
obs-scope-summary#22.3 · not touched · the report measures both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}`, so the clause holds; the exit it does not state is F6, another category's
obs-scope-summary#25.0 · not touched · the report's `detail` is additive and optional on one event, the wire form unchanged, so the mirrored required fields and `corr` table hold
obs-scope-summary#28.3 · proposed · its `detail` list for the send path omits the closed words `channel-response` now carries on `-32602`
obs-scope-summary#29.3 · not touched · the list names no `detail` or error field, so `channel-response`'s closed word on a fault falsifies nothing, and no result body reaches the line
obs-scope-summary#31.2 · proposed · `release-from-driver` is now also a `channel-response` `detail` word on this path, beside the event the line names
obs-scope-summary#33.1 · proposed · the scrub rule's «only» field list omits the `detail` word `channel-response` now writes
telemetry-strategy#1.2 · proposed · «every … process lifecycle» does not hold for `mcp`, the report measuring no self `process-exit` on that role
telemetry-strategy#1.3 · proposed · `detail` codes on `process-exit` do not reach `mcp`, whose role the report measures writing none, though its catch site stands
telemetry-strategy#3.15 · not touched · the row maps a trigger to §3 and §6, both standing, and neither heading nor mapping moved
