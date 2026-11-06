@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the msrv job checks the 1.96 floor, so the new portable-pty release must build on 1.96 and CI's seven jobs stay green
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the Tokio containment keystone binds through the new portable-pty graph, which must keep Tokio out of the sync crates and the channel's normal graph
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap's pin and feature set are untouched by the move (not-needed item 12), and the chunk adds no colour or terminal dependency
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: this row is the portable-pty pin the move changes, and pty_backend() must still read openpty behind the pty seam
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: the new portable-pty graph must add no windows-sys edge, so the Linux-only OS-binding crate set this row states must hold
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: vt100 and its readiness gate are not changed (not-needed item 10), and no chunk check reads the screen model
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: the Linux IPC endpoint's socket directory, 0600 chmod and owner check are what the chunk measures for the no-leak property and rewords in the security plan
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the chunk writes no ndjson log and no snapshot, and the snapshot and log writers are not-needed items 4 and 11
@answer n=9 entity=stack-and-technologies#1.9 v=no standing=S5
why: it states the process-log line format convention (no v key, version in the schema filename) that every new log line owes, and the chunk adds no log line
@answer n=10 entity=stack-and-technologies#1.10 v=yes
why: the per-user 0700 socket directory and the strict-modes read rule (item 19, PROVISIONAL) are created and checked through this libc path
@answer n=11 entity=stack-and-technologies#1.11 v=yes
why: the pinned copy key bin/<version>-<hash> (first 16 hex of the SHA-256) is the Deployment decision named in item 8
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the serde_json pins and parse behaviour are untouched (not-needed item 12), and its deny feature ban is not one the scope names, so it is a re-check every change owes
@answer n=13 entity=stack-and-technologies#1.13 v=no standing=S5
why: its one-line JSON log format and diagnostics location are the obs-owned convention S5 names, and the chunk adds no log line
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: the DialogHold newtype (whole seconds 60 to 3600) is the bound the live dialog-hold measure near the 3600 s ceiling reads (item 7)
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: chrono's pin and its RFC 3339 timestamp format are not-needed item 9 and are untouched by the move
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: PtyError is the ratified hand-written exception and the per-crate thiserror census is re-worded under the Error Handling decision (item 5)
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: it names the per-wrapper socket topology, whose endpoint rules are carried by row 1.7, and the chunk does not change that topology
@answer n=18 entity=stack-and-technologies#1.18 v=yes
why: the pid plus start-time gone rule and the sysinfo liveness signal are what the killed-wrapper and session-liveness leg (item 17) rests on
@answer n=19 entity=stack-and-technologies#1.19 v=yes
why: the MCP tools and the rmcp server feature set are what the live driver session drives (item 14) and what the release graph check reads
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the AI/ML row is N/A in v1 and unreached (not-needed item 13)
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Mobile row is N/A in v1 and unreached (not-needed item 13)
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: container and deployment are cargo install and local only, and the portable-pty move and the removals do not reach them
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: CI's seven ubuntu-latest jobs, the msrv and supply-chain jobs included, must stay green at the wrap (item 20)
@answer n=24 entity=stack-and-technologies#1.24 v=yes
why: the fmt, clippy and cargo deny gates are re-run, and the syn cfg reader behind run --mutants host exclusion reads the windows key the chunk removes (item 13)
@answer n=25 entity=stack-and-technologies#1.25 v=no
why: dist and self_update are v1.x release tooling outside v1 scope, and the release check is witnessed through rows 1.23 and 1.24
