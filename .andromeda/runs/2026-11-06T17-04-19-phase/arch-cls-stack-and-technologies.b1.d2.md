@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the msrv job in the chunk's CI re-run holds the 1.96 floor, which the moved portable-pty graph must still meet
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the Tokio containment keystone binds the new portable-pty graph, which must keep Tokio built only inside mcp
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap pin and feature set are not moved by the pty change and the CLI parser is not in the chunk's scope
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: the PTY row names the portable-pty =0.8.1 pin this chunk moves and the openpty backend it must keep
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: the moved portable-pty graph must keep no crate depending on windows-sys and libc as the one OS-binding crate
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the vt100 pin and its screen-reading roles are untouched, since the pty move does not change readiness or screen handling
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: the Linux IPC endpoint in scope (socket directory, chmod 0600, peer check) is the endpoint the no-leak measurement runs on
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: there is no database and no state store the chunk changes
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: ORM and migrations are N/A and no record format changes
@answer n=10 entity=stack-and-technologies#1.10 v=yes
why: the strict-modes check and private-dir creation through libc are the socket-directory and strict-modes read rules the scope names
@answer n=11 entity=stack-and-technologies#1.11 v=yes
why: the scope names the first 16 hex of the content SHA-256 key of bin/<version>-<hash>/, which this row states with its sha2 pin
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: serde pins and the unbounded_depth ban are untouched, and the chunk's cargo deny check is scoped to advisories and the ignore list
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: logging and veil redaction are not changed and no frame params or log lines move
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: DialogHold bounds the 60–3600 s dialog hold that the chunk measures near the ceiling
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: chrono RFC 3339 timestamps are not touched by the chunk
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: the Error Handling echo names PtyError as the hand-written ratified exception in the one-enum-per-crate rule, which the chunk re-words
@answer n=17 entity=stack-and-technologies#1.17 v=yes
why: the per-wrapper local sockets are the channel endpoints whose absence from the wrapped child's fds the chunk measures
@answer n=18 entity=stack-and-technologies#1.18 v=yes
why: the pid plus process start time liveness signal is the gone rule and killed-wrapper leg the scope names
@answer n=19 entity=stack-and-technologies#1.19 v=yes
why: the MCP tool set send, wait, last, answer and list is one of the driving surfaces the live overseer session uses
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: AI/ML serving is N/A and viola calls no model API
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the mobile framework is N/A in v1 and unreached
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: cargo install and plugin embedding are not changed, and the pinned-copy key the scope names is carried by 1.11
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: CI on ubuntu-latest only and the local pre-push gate are the seven-job green the chunk re-runs
@answer n=24 entity=stack-and-technologies#1.24 v=yes
why: the chunk re-runs fmt, clippy, cargo deny and coverage, and the syn cfg reader behind the run --mutants host table it edits
@answer n=25 entity=stack-and-technologies#1.25 v=no
why: dist and self_update are release tooling for later versions, outside v1 scope
