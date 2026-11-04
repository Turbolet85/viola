@answer n=1 entity=obs-scope-summary#1.1 v=no
why: a count of telemetry surface entries restates the summary and carries no rule the chunk's code or tests must honour
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: a count of instrumentable crates; the rules sit on the per-crate entities
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: a count of must-trace paths and edge flows; no rule the chunk must honour
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: a count of telemetry triggers; the multi-OS compat trigger is carried by the chunk's own needs, not by this summary
@answer n=5 entity=obs-scope-summary#2.1 v=yes
why: the diagnostics/ root decides where the chunk's per-role log files, the secret scan and the per-OS diag upload reach, and the scope leaves that root open
@answer n=6 entity=obs-scope-summary#2.2 v=yes
why: mcp always writes mcp.ndjson, the home-level log location the chunk re-asserts for mcp on every OS
@answer n=7 entity=obs-scope-summary#2.3 v=no
why: the event enum extensions are deferred (release-from-driver is Phase 3 and parity reopens none), and the chunk adds no event word
@answer n=8 entity=obs-scope-summary#2.4 v=no
why: hook-invoked corr timing is a settled Phase 3 alignment gap that parity reopens none of
@answer n=9 entity=obs-scope-summary#3.1 v=no
why: a source citation for the CLI crate that states no rule
@answer n=10 entity=obs-scope-summary#3.2 v=yes
why: the CLI crate is instrumentable, and the chunk's run, revive and pre-child refusal lines are logged from it
@answer n=11 entity=obs-scope-summary#3.3 v=yes
why: the full error chain stays in diagnostics and a serde-sourced chain must become a fixed message before stderr, which the chunk's fixed two-line refusal pair must honour
@answer n=12 entity=obs-scope-summary#4.1 v=no
why: a source citation for viola_core that states no rule
@answer n=13 entity=obs-scope-summary#4.2 v=yes
why: viola_core is not-instrumentable, so the exit-21 cause words the chunk's clients write produce no line from this crate
@answer n=14 entity=obs-scope-summary#4.3 v=yes
why: the UnreachableCause words include server-verify-failed, socket-dir-refused and strict-modes-failed, which are Unix-reached details, and a refused strict-modes check writes no line
@answer n=15 entity=obs-scope-summary#5.1 v=no
why: a source citation naming portable-pty that states no rule for the chunk to honour
@answer n=16 entity=obs-scope-summary#5.2 v=yes
why: the viola-pty seam is instrumented at the seam only, which the chunk's openpty backend and PTY-seam logging must honour
@answer n=17 entity=obs-scope-summary#5.3 v=yes
why: the openpty OS API is boundary-only and the PTY byte stream is never logged
@answer n=18 entity=obs-scope-summary#6.1 v=no
why: a source citation for the interprocess stack entry that states no rule
@answer n=19 entity=obs-scope-summary#6.2 v=yes
why: the IPC crate is instrumentable, and the chunk's channel-request and channel-response endpoint telemetry is logged from it
@answer n=20 entity=obs-scope-summary#6.3 v=yes
why: instrumentation in this crate must be sync with no Tokio, which the chunk's channel telemetry code must honour
@answer n=21 entity=obs-scope-summary#7.1 v=no
why: a source citation for the ndjson, atomic-write-file and sysinfo stack entries that states no rule
@answer n=22 entity=obs-scope-summary#7.2 v=yes
why: the ndjson and atomic-write sink is instrumentable, and the chunk's same file sink on every OS and its snapshot writes run through it
@answer n=23 entity=obs-scope-summary#7.3 v=yes
why: this sink must be sync with no Tokio, which the chunk's file sink on every OS must honour
@answer n=24 entity=obs-scope-summary#8.1 v=no
why: a source citation for the serde_path_to_error drift reports that states no rule
@answer n=25 entity=obs-scope-summary#8.2 v=yes
why: this serde-path reader crate is instrumentable, and the state-recovered lines (snapshot-replayed, torn-line-healed) the chunk keeps identical on every OS are read through it
