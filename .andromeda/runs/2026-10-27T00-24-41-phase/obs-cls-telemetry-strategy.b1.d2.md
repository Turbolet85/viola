# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · yes · the pin step sits inside the must-trace run-start to child-spawn path and the chunk adds viola-state instrumentation at a crate seam, so the span coverage binds (its #[instrument] form is S3's)
telemetry-strategy#1.2 · yes · the pin write's typed fault becomes one closed-enum log line, verify's pin emits process-start and process-exit, and the race witness reads the merged logs through agent-run logs
telemetry-strategy#1.3 · yes · the chunk adds a new exit-1 cause on the absent-copy write with a distinct detail code on process-exit, StateError keeps its fixed Display, and panic = unwind must hold under any profile key the measurement changes
telemetry-strategy#1.4 · no · the chunk adds no duration sample, in-process instrument or hyperfine perf gate, and its race counts are test outcomes read from status and logs, not jq-derived metrics
telemetry-strategy#1.5 · no · the chunk writes no events.ndjson line, rotates and truncates nothing, and the race witness reads events.ndjson offsets only
telemetry-strategy#2.1 · yes · a build change the measurement motivates (path remap, linker flag, profile key) must keep panic_location in the src/cmd/hook/seam.rs:<digits> form that G2's single exemption compares as a whole string
telemetry-strategy#3.1 · no · the chunk adds or changes no channel method, frame or allow-list, so the IPC vector is not reached
telemetry-strategy#3.2 · no · no send path changes, so the paste-text vector and its send-refused side are not reached
telemetry-strategy#3.3 · yes · a viola hook spawn of the pinned path that fails must keep the hook fail-open rules: exit 0, empty stderr, hook-<name>.ndjson only, and a silent no-op on a refused or absent instance
telemetry-strategy#3.4 · no · the viola mcp surface (mcp.ndjson, its stderr fallback, isError codes) is not needed, and the plugin/ rewrite's effect on an mcp reader adds no line here
telemetry-strategy#3.5 · yes · the start refusal stays the fixed two-line human::refuse pair under the §7 fixed messages, and any refusal site the fix adds or removes moves the run 7 and verify 6 site counts
telemetry-strategy#3.6 · yes · the absent-copy write is the filesystem vector: the pinned-hash-mismatch line shape and level stay, the strict-modes-failed and sideload_fallback values stay, and any new closed code is recorded in the §6 detail catalog
telemetry-strategy#3.7 · yes · the pin step sits inside Scenario 1, so the process-start subject values stay as they are and no pinned_bin path, endpoint or companion hash may appear in a span or line
telemetry-strategy#3.8 · yes · viola-state instrumentation must stay sync and Tokio-free under the cargo-deny sync-crate ban over its graph, with the §3 logging feature set unchanged
telemetry-strategy#3.9 · no · the channel corr + conn join and hook-invoked corr timing are not needed, since no channel method or dialog path changes
telemetry-strategy#3.10 · no · the chunk names no budget and its release-build comparison is a build measurement, not a §5 perf gate or §10 budget row
telemetry-strategy#3.11 · no · the chunk adds no kill, stale-heartbeat or chaos class, so its new exit-1 is a natural refusal carried by the existing detail field (#1.3, #3.6)
telemetry-strategy#3.12 · yes · the absent-copy write runs on Windows, macOS and Linux, the exe's Windows arm may take pin_companions' ERROR_SHARING_VIOLATION wait, and the 0700 and DACL rules plus Scenario 1's endpoint_kind and pty_backend values apply
telemetry-strategy#3.13 · no · standing S5 · new test output counts toward the zero budgets, 0 unlogged panics (G2)
telemetry-strategy#3.14 · no · the chunk rotates and truncates no log and moves no log file location, so the no-rotation rule is untouched
telemetry-strategy#3.15 · yes · the chunk's new closed detail code and its WARN or ERROR pin-write line must be recorded in the §6 catalog and pass the verbatim schema, which the scope names
telemetry-strategy#3.16 · yes · verify's home-tree check runs before its log and a refused tree writes no line, which the chunk keeps, and the absent-copy exit carries its detail code only after the log opens
telemetry-strategy#3.17 · no · the chunk parses no installed-CLI output and changes no ledger stamp or capability row, so its race counts are harness outcomes, not CLI measurements
telemetry-strategy#3.18 · yes · the race witness must wait on hook events and events.ndjson offsets, never on sleeps or screen content, which the scope states
