# obs · picked accounting · group 1 of 5

## Picked
obs-scope-summary#2.1 · not in scope · the diagnostics-root reconciliation is settled by D-08 / D-22 and the chunk opens no new log file
obs-scope-summary#3.2 · carried · the root bin's `run` and `verify` pin call sites the chunk edits are its instrumentable error-capture boundary
obs-scope-summary#3.3 · carried · a pin-write error reaching `run` / `verify` dispatch keeps a fixed stderr message, its full chain to `diagnostics/` only
obs-scope-summary#4.3 · not in scope · `viola-core` is not reached; a new `detail` value it might supply is catalogued under standing S2
obs-scope-summary#5.3 · not in scope · the chunk changes no PTY seam operation and logs no PTY byte stream
obs-scope-summary#7.2 · carried · `viola-state`'s `pin.rs` (and `fs.rs`) are edited, an instrumentable crate
obs-scope-summary#7.3 · carried · any instrumentation the pin fix adds in `viola-state` stays sync and Tokio-free
obs-scope-summary#10.2 · carried · the wrapped child is boundary-only, and the two-sided witness drives it
obs-scope-summary#10.3 · carried · the pin step precedes the child's `process-start`, and the witness reads the failure at viola's boundary, never the child's transcripts
obs-scope-summary#11.2 · carried · the hook invocation the race breaks is a boundary-only surface
obs-scope-summary#11.3 · carried · the failed hook spawn is observed only through hook-boundary telemetry and the missing `session-start`
obs-scope-summary#14.2 · carried · the absent-copy write's rename is an OS-API call site the chunk edits
obs-scope-summary#14.3 · carried · instrumentation of the pin write is at viola's rename / lock call sites, and the pinned-exe mismatch telemetry it names stays as is
obs-scope-summary#15.2 · carried · `bin/<key>/` and `snapshot.json`'s `pinned_bin` are home stores reached through `viola-state` call sites
obs-scope-summary#15.3 · carried · `pinned_bin` stays scrubbed from external output, and the witness reads `events.ndjson` without copying its payload into process lines
obs-scope-summary#18.1 · not in scope · standing S1 · each new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`
obs-scope-summary#18.2 · not in scope · the chunk adds no frontend surface
obs-scope-summary#18.3 · carried · `viola verify`'s `pin_exe` call is edited, and its logging order behind the home-tree check must hold
obs-scope-summary#18.4 · carried · `service.version` is the version half of the pinned key, and the reproducible-build measurement and any build change it leads to must keep it compile-time
obs-scope-summary#18.5 · carried · a new exit-1 cause on the absent-copy write takes a distinct `detail` code, refusal lines fixed with no path or pid
obs-scope-summary#19.1 · carried · the pin write is a `viola-state` write inside `run`'s start sequence, instrumented sync
obs-scope-summary#19.3 · carried · any line the pin fix adds in `run` goes to `run-<name>.ndjson` only, never the terminal
obs-scope-summary#19.4 · not in scope · `process="run"` / `instance` come from the unchanged logger init, a re-check every change owes
obs-scope-summary#19.5 · not in scope · the panic hook, heartbeat and handle-wait exit detection are untouched by the absent-copy write's concurrency
