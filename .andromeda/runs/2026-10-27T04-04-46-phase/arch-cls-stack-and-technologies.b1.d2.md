# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the six new fuzz targets build on the fuzz/ workspace nightly pin, and the new Rust code stays on the pinned 1.98.1 toolchain at the 1.96 floor
stack-and-technologies#1.2 · yes · Tokio containment: the run, hook, send and channel-server paths stay std threads with blocking I/O, and Tokio stays inside viola-mcp
stack-and-technologies#1.3 · yes · the argv shapes the chunk bounds (send and answer text read only from stdin or --file, the hidden hook --capture, --answers and --shell arms) are clap derive declarations of these subcommands
stack-and-technologies#1.4 · no · the PTY backend (sideloaded ConPTY or openpty) is not what the pump.rs passthrough re-read or the feed-panic degrade checks, and the ConPTY sideloading and vendored audit are Not needed
stack-and-technologies#1.5 · yes · the vt100 feed on run's feed thread, handed a copy of the pump output, is what the screen.rs catch_unwind re-read and the vt100_feed fuzz target act on
stack-and-technologies#1.6 · yes · the interprocess local_socket streams carry the channel frame bound (one line of at most MAX_FRAME with its LF counted, -32600 and close) on the sync server and both clients, and the Tokio client sits behind its feature gate
stack-and-technologies#1.7 · yes · the events.ndjson reader, the snapshot read and read_budget are the ndjson and atomically replaced snapshot stores this entity names, and the chunk adds no database
stack-and-technologies#1.8 · yes · the readers of viola's own formats (events, snapshot, budget, stamps) skip and count unknown kinds and fields, carry v and never use deny_unknown_fields, and process-log lines carry no v
stack-and-technologies#1.9 · yes · read_budget takes a shared std File::lock on the budget .lock sibling, which the entity says is opened for write; the strict-modes check and the no-clobber placement are Not needed
stack-and-technologies#1.10 · yes · the closed-shape validator of the pinned version-hash key with its 16-hex SHA-256 prefix binds; the pinned-copy re-hash and the ConPTY pins are Not needed
stack-and-technologies#1.11 · yes · serde_json at its default recursion depth on every parse of external bytes, with unbounded_depth absent from the product graph
stack-and-technologies#1.12 · yes · the new parse-rejected process-log lines go through tracing and obs_event! under #[instrument], and veil's Redact keeps a frame's params out of any Debug render
stack-and-technologies#1.13 · yes · nutype is the mechanism of ViolaName (charset and length, validated before every instance join), Percent (0 to 100) and DialogHold (60 to 3600) that the chunk's readers feed
stack-and-technologies#1.14 · no · the chunk adds no timestamp writer or ts reader check, and the RFC 3339 millisecond form is the writers' rule that the events reader does not re-check
stack-and-technologies#1.15 · yes · the chunk's new typed refusals and parse errors use thiserror per crate, anyhow stays at the bin edge, and serde source chains stay in the owner-only detail file
stack-and-technologies#1.16 · yes · the per-wrapper JSON-RPC 2.0 over ndjson framing is what the chunk's one-line MAX_FRAME bound and -32600 close act on, and no broker is added
stack-and-technologies#1.17 · yes · the bounded claude agents --json read (take of MAX_FRAME + 1, 5 s kill, a failed read leaves status unknown and is never an error) binds; the sysinfo liveness and the join are Not needed
stack-and-technologies#1.18 · yes · the rmcp stdio server behind viola mcp binds: its stdin framing, and the send, wait, last, answer and list tools whose arguments the server re-validates (invalid-params, unknown tool -32602)
stack-and-technologies#1.19 · no · no model API is called or added, and the task's Not needed names the AI/ML rows; the claude spawns the chunk bounds are CLI reads, not model calls
stack-and-technologies#1.20 · no · a mobile framework is N/A and mobile is Not needed in the task's scope
stack-and-technologies#1.21 · no · the deployment is cargo install with the plugin compiled in and written out by viola run; the chunk changes no plugin file, and hosting is Not needed
stack-and-technologies#1.22 · yes · the chunk's new fuzz targets run in the CI fuzz jobs on the fuzz/ toolchain, and its unit witnesses run in the three-OS test matrix and the pre-push ubuntu suites
stack-and-technologies#1.23 · yes · cargo-deny's tokio ban holds the Tokio containment, cargo-fuzz 0.13.2 with the libfuzzer-sys and arbitrary pins runs the new fuzz targets, and the clippy and G1/G3 ripgrep gates check the new code and log lines
stack-and-technologies#1.24 · no · distribution and signed releases are v1.x, and the chunk has no release or distribution scope
stack-and-technologies#2.1 · no · the peer-pid identity read and the identity check before a client's first frame are Not needed (owned by Unix endpoint and home hardening), and the chunk's client edits are the frame bound and the paste rule
