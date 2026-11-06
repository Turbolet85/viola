@answer n=1 entity=obs-scope-summary#1.1 v=no
why: a count of telemetry surface entries the chunk neither adds to nor re-measures (its re-measured facts are the process-start row, the thiserror census, the enum divergences and the channel-handle property)
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: a count of instrumentable crates the chunk does not change
@answer n=3 entity=obs-scope-summary#1.3 v=yes
why: the Linux-only OS target is what the chunk makes true by removing the Windows remnants and the macOS/Windows CI and tests
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: a count of must-trace paths and edge flows the chunk does not touch
@answer n=5 entity=obs-scope-summary#1.5 v=no
why: an inventory count of the 18 triggers, which the chunk does not add to or remove; the Linux-compat trigger's rule binds through need 13, not this count line
@answer n=6 entity=obs-scope-summary#2.1 v=yes
why: the open fork on the diagnostics/ root decides where the live viola run writes diagnostics/run-<name>.ndjson and whether any upload or home-level file rule applies (needs 10 and 11)
@answer n=7 entity=obs-scope-summary#2.2 v=no
why: describes mcp's stderr fallback for an already-landed change; the chunk touches no mcp code and states no print prohibition
@answer n=8 entity=obs-scope-summary#2.3 v=no
why: the event enum extensions (release-from-driver, chaos signals) are not added or changed by the chunk
@answer n=9 entity=obs-scope-summary#2.4 v=yes
why: the hook-invoked corr timing relative to dialog_id is the evidence ordering the live dialog-hold read must check (need 12)
@answer n=10 entity=obs-scope-summary#3.1 v=no
why: source provenance line for the error-boundary crate, not a rule
@answer n=11 entity=obs-scope-summary#3.2 v=no
why: a bare instrumentable classification of the CLI crate that no chunk change must act on; the chunk adds no log lines there
@answer n=12 entity=obs-scope-summary#3.3 v=yes
why: the fixed-message rule before stderr governs the fixed-Display scrubbing layer that a typed-error fold (need 3) changes, and the chunk's plain_drive edit sits in the dispatch boundary
@answer n=13 entity=obs-scope-summary#4.1 v=no
why: source provenance line for the type crate, not a rule
@answer n=14 entity=obs-scope-summary#4.2 v=no
why: a bare not-instrumentable classification of the type crate with no rule the chunk must honour
@answer n=15 entity=obs-scope-summary#4.3 v=yes
why: its six-word UnreachableCause closed set includes socket-dir-refused, which the chunk's ratify-or-re-word of the socket-dir-refusal pair (need 14) may change, and the one-enum fold may touch the same crate
@answer n=16 entity=obs-scope-summary#5.1 v=no
why: source provenance line for the PTY seam, not a rule
@answer n=17 entity=obs-scope-summary#5.2 v=yes
why: the seam is classified instrumentable, and the portable-pty bump must keep its wrapper emitting process-start and process-exit (need 6)
@answer n=18 entity=obs-scope-summary#5.3 v=yes
why: openpty is boundary-only and the PTY byte stream is never logged, which the bump and the paste-as-text_bytes-only rule (need 6) and the openpty-alone pty_backend value (need 1) must honour
@answer n=19 entity=obs-scope-summary#6.1 v=no
why: source provenance line for the IPC crate, not a rule
@answer n=20 entity=obs-scope-summary#6.2 v=no
why: a bare instrumentable classification of the IPC crate that the chunk's code does not need to act on
@answer n=21 entity=obs-scope-summary#6.3 v=yes
why: the Tokio-free sync-crate graph rule is what the portable-pty bump must hold (need 5) and what cargo deny's Tokio ban checks (need 8)
@answer n=22 entity=obs-scope-summary#7.1 v=no
why: source provenance line for the file-sink crate, not a rule
@answer n=23 entity=obs-scope-summary#7.2 v=no
why: a bare instrumentable classification of the sink crate that the chunk does not change
@answer n=24 entity=obs-scope-summary#7.3 v=yes
why: the Tokio-free sync-crate rule for this graph is what the portable-pty bump must hold (need 5) and what cargo deny checks (need 8)
@answer n=25 entity=obs-scope-summary#8.1 v=no
why: source provenance line for the drift-report crate, not a rule
