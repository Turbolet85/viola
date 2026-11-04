@answer n=1 entity=obs-scope-summary#1.1 v=no
why: summary count of telemetry surfaces with no rule of its own for the chunk's code or tests

@answer n=2 entity=obs-scope-summary#1.2 v=no
why: summary count of instrumentable crates with no rule of its own

@answer n=3 entity=obs-scope-summary#1.3 v=no
why: summary count of must-trace paths and edge flows with no rule of its own

@answer n=4 entity=obs-scope-summary#1.4 v=no
why: summary count of triggers, and the rules it indexes are carried by their own entities

@answer n=5 entity=obs-scope-summary#2.1 v=yes
why: the diagnostics root fork (tests versus instances diagnostics) decides where the per-OS log file location, modes, G2 home-level scope and secret scan look

@answer n=6 entity=obs-scope-summary#2.2 v=yes
why: mcp writes mcp.ndjson and falls back to stderr JSON only when that file cannot be opened, which the per-OS mcp log-location and mode check on Unix must act on

@answer n=7 entity=obs-scope-summary#2.3 v=no
why: Phase 3 event enum additions (release-from-driver and chaos signals) are not emitted, read or validated by the parity chunk, and the task settles them as not reopened

@answer n=8 entity=obs-scope-summary#2.4 v=no
why: hook-invoked corr timing is a settled Phase 3 gap the task marks not needed, and the chunk changes no corr assignment

@answer n=9 entity=obs-scope-summary#3.1 v=no
why: source citation for the CLI crate with no rule of its own, the rules sit in 3.2 and 3.3

@answer n=10 entity=obs-scope-summary#3.2 v=yes
why: the chunk adds run wrapper, revive arm and refusal code in this instrumentable crate, so its new log lines and spans owe instrumentation

@answer n=11 entity=obs-scope-summary#3.3 v=yes
why: the Unix refusals must write the fixed two-line stderr pair through human::refuse, with the full error chain going only to diagnostics

@answer n=12 entity=obs-scope-summary#4.1 v=no
why: source citation for viola_core with no rule of its own

@answer n=13 entity=obs-scope-summary#4.2 v=no
why: not-instrumentable flag for a pure type crate that adds no emission the chunk must act on, and the word rules it carries are in 4.3

@answer n=14 entity=obs-scope-summary#4.3 v=yes
why: the UnreachableCause closed word set includes server-verify-failed and socket-dir-refused, which the Unix details and exit 21 documents must write from this set

@answer n=15 entity=obs-scope-summary#5.1 v=no
why: source citation for the portable-pty stack with no rule of its own

@answer n=16 entity=obs-scope-summary#5.2 v=yes
why: the viola-pty seam is instrumentable and the chunk adds its openpty backend with PTY-seam logging at the seam only

@answer n=17 entity=obs-scope-summary#5.3 v=yes
why: the openpty OS API stays boundary-only and the PTY byte stream is never logged, which the openpty seam code must keep

@answer n=18 entity=obs-scope-summary#6.1 v=no
why: source citation for the interprocess crate and its IPC method list with no rule of its own

@answer n=19 entity=obs-scope-summary#6.2 v=yes
why: the Unix channel endpoint telemetry the chunk adds is instrumentable in this crate

@answer n=20 entity=obs-scope-summary#6.3 v=yes
why: Tokio is forbidden in this crate's graph, so the added channel telemetry must be sync

@answer n=21 entity=obs-scope-summary#7.1 v=no
why: source citation for the ndjson and snapshot writer stack with no rule of its own

@answer n=22 entity=obs-scope-summary#7.2 v=yes
why: the state-recovered lines (snapshot-replayed, torn-line-healed) keep the same detail words on every OS, and this crate is instrumentable

@answer n=23 entity=obs-scope-summary#7.3 v=yes
why: Tokio is banned in this crate, so the chunk's events here must be sync

@answer n=24 entity=obs-scope-summary#8.1 v=no
why: source citation for the serde_path_to_error drift reporting with no rule of its own

@answer n=25 entity=obs-scope-summary#8.2 v=yes
why: the drift-report crate is where G4 schema conformance is re-measured, so its instrumentability binds the chunk's new code there
