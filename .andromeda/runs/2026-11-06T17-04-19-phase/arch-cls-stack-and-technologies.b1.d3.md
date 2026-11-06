@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the msrv job's 1.96 floor is one of CI's seven jobs the chunk re-runs, and the moved portable-pty release must build at that floor
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the no-Tokio containment of the hot path binds the new portable-pty graph through the Tokio containment keystone
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap's feature set is not moved, and item 12 keeps clap's pin out of the chunk
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: this row is the portable-pty =0.8.1 pin the chunk moves, and its openpty and host-tty behaviour is what the PTY decision carries
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: the Linux-only build's OS-binding rule (libc alone, no windows-sys in the graph) must hold after the Windows remnants go and the pty graph moves
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: vt100 is used by run and verify's probes, which the chunk does not change or measure
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: the Unix socket endpoint, its chmod 0600 and the directory check are the IPC endpoint the chunk measures for channel-handle no-leak
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: no database exists and the chunk touches no ndjson or snapshot store
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the record versioning and process-log schema-filename rule is not changed by the chunk, which adds no log line or record kind
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: state-file primitives and the crash-safe writers are not changed (item 11 not_needed)
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the sha2 pin and the 16-hex bin key are untouched by the chunk, and Measurement B is a founder item, not chunk code
@answer n=12 entity=stack-and-technologies#1.12 v=yes
why: the cargo deny check re-run witnesses deny.toml's serde_json unbounded_depth feature ban, which the moved graph must keep
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: tracing and the log line format are owned by the obs plan, and the chunk's obs row edit does not depend on this row's rule
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: DialogHold's whole-second 60 to 3600 range is the dialog-hold ceiling the chunk reads live near 3600 s
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: chrono is not moved and timestamps are not changed (item 9 not_needed)
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: the one-enum-per-crate rule with PtyError as the ratified exception is what the chunk folds or ratifies, and the thiserror derive census is re-measured
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the no-broker rule is not changed, and the channel-handle endpoint rule is carried by stack row 1.7
@answer n=18 entity=stack-and-technologies#1.18 v=yes
why: the pid plus start-time liveness rule is the one the killed-wrapper leg and the heartbeat reading (item 17) stand beside
@answer n=19 entity=stack-and-technologies#1.19 v=yes
why: the MCP tools send, wait, last, answer and list are the surfaces the live driver session reaches, and the rmcp feature set is the Tokio containment the chunk re-runs
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the AI/ML row is N/A in v1 and unreached (item 13 not_needed)
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Mobile row is N/A in v1 and unreached (item 13 not_needed)
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: the deployment shape (cargo install, plugin via include_str) is not changed by the chunk, and the release check's refusal rule is not this row's
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: CI's ubuntu-latest matrix and its jobs are the seven jobs the chunk re-runs green, and the pre-push gate runs the same suites
@answer n=24 entity=stack-and-technologies#1.24 v=yes
why: clippy with -D warnings binds the chunk's code edits, and syn and proc-macro2 are the cfg reader behind the run --mutants windows host key the chunk edits
@answer n=25 entity=stack-and-technologies#1.25 v=no
why: the release row is v1.x, not v1, and the chunk does not put distribution in scope
