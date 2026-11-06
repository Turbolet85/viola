@answer n=1 entity=obs-scope-summary#16.3 v=no
why: static configuration reason; the chunk does not touch the config parse or the viola-state skipped-key logging
@answer n=2 entity=obs-scope-summary#17.1 v=no
why: CI/CD platform source line only; the chunk adds no telemetry to CI, which is not product runtime
@answer n=3 entity=obs-scope-summary#17.2 v=no
why: not-instrumentable classification of the CI/CD platform; the chunk adds no CI instrumentation
@answer n=4 entity=obs-scope-summary#17.3 v=no
why: fixtures classed as scrub-required data and excluded from sections 2 to 4; the chunk adds no fixture and states no synthetic-fixture rule itself
@answer n=5 entity=obs-scope-summary#18.1 v=yes
why: CLI dispatch spans, the JSON logger and the per-process panic hook bind the plain_drive edit in src/cmd/mod.rs and the G1 and G2 gates with the zero panic-line budget
@answer n=6 entity=obs-scope-summary#18.2 v=yes
why: CLI output is static fixed-column stdout and stderr with no TUI or pager, which the plain_drive edit and tests/cli_output_plain.rs must keep
@answer n=7 entity=obs-scope-summary#18.3 v=yes
why: the file-sink-only exporter binds through the multi-platform trigger (one file sink and one line schema on Linux, no OS-specific exporter left after the remnant removal)
@answer n=8 entity=obs-scope-summary#18.4 v=no
why: service identity values are unchanged, and the chunk adds no short-lived verb, process or version change
@answer n=9 entity=obs-scope-summary#18.5 v=yes
why: fixed stderr strings, the --json internal-error document in src/cmd/mod.rs and the socket-dir-refused code bind the re-worded socket-directory refusal pair (item 14) and the plain_drive edit
@answer n=10 entity=obs-scope-summary#19.1 v=yes
why: tracing spans on the viola-pty seam operations and the sync, Tokio-free instrumentation bind the portable-pty bump and the seam wrapper it moves
@answer n=11 entity=obs-scope-summary#19.2 v=no
why: frontend hooks are N/A and the 0.2.0 frontend is out of scope; the terminal rules that bind sit in 19.3 and 19.5
@answer n=12 entity=obs-scope-summary#19.3 v=yes
why: the live viola run must log to diagnostics/run-<name>.ndjson and write nothing to the terminal, which is the evidence item 10 reads
@answer n=13 entity=obs-scope-summary#19.4 v=yes
why: the process run and instance ViolaName fields are what the live run's log lines carry for the evidence read through agent-run logs and jq
@answer n=14 entity=obs-scope-summary#19.5 v=yes
why: the run panic hook replaced before spawn and the child's process-exit taken from the handle wait, never reader EOF, bind the live-run terminal rule and the item 6 exit path
@answer n=15 entity=obs-scope-summary#20.1 v=yes
why: hook-invoked and hook-decision lines and the single panic line bind the live dialog hold scenario (item 12) and the zero panic-line budget
@answer n=16 entity=obs-scope-summary#20.2 v=no
why: frontend hooks are N/A and the hook has no frontend surface the chunk touches
@answer n=17 entity=obs-scope-summary#20.3 v=yes
why: the hook-<name>.ndjson file sink, with stderr forbidden and stdout limited to the decision body, is where the live dialog read's evidence lines are written
@answer n=18 entity=obs-scope-summary#20.4 v=yes
why: the process hook identity and the silent no-op for an absent instance bind the live dialog session, whose evidence exists only when its instance resolves
@answer n=19 entity=obs-scope-summary#20.5 v=no
why: append atomicity and the latency budget are the hook's own timed path, which the chunk's perf.rs EXE_SUFFIX joins do not touch
@answer n=20 entity=obs-scope-summary#21.1 v=no
why: the chunk adds no channel method, verb or cross-surface call, so trace-context and corr propagation are not touched
@answer n=21 entity=obs-scope-summary#21.2 v=no
why: frontend hooks are N/A and out of scope
@answer n=22 entity=obs-scope-summary#21.3 v=no
why: the exporter is each side's own log file, which the chunk does not change
@answer n=23 entity=obs-scope-summary#21.4 v=no
why: each side's process identity and the peer version are unchanged by the chunk
@answer n=24 entity=obs-scope-summary#21.5 v=yes
why: the peer_admitted drop that writes no line is the mutation audit's one survivor by construction, which item 15 keeps with its dated note and untestable zone
@answer n=25 entity=obs-scope-summary#22.1 v=no
why: MCP tool-call spans wrap the viola-channel client, which the chunk does not change
