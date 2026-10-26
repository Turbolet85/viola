# Classified — obs-plan · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · no surface, process role or product part added or removed and the session edits only the tokio pin and its lock entry
obs-scope-summary#1.2 · no · the crate count stands because Cargo.toml moved at line 242 only and no crate was added or removed
obs-scope-summary#1.3 · no · no span, instrument site or code path changed, so the must-trace paths and edge flows stand
obs-scope-summary#1.4 · no · the trigger count is unchanged and the supply-chain trigger was re-read and holds on the bumped lock, cargo deny exiting 0 with no ignore widened
obs-scope-summary#2.1 · no · the diagnostics root lives in source and test code the session did not change
obs-scope-summary#2.2 · no · the mcp stderr logging is source the session did not change, and no logger, sink or event changed
obs-scope-summary#2.3 · no · no event enum variant changed since the session edits only the tokio pin and its lock entry
obs-scope-summary#2.4 · no · the hook-invoked corr timing is hook source the session did not change and no hook changed
obs-scope-summary#3.1 · no · the lock delta is the tokio entry alone and Cargo.toml moved at line 242 only, so the clap 4.6.7, vt100 0.16.2, thiserror and anyhow versions hold on the changed files
obs-scope-summary#3.2 · no · instrumentability holds because no instrument site or source changed and the lock delta adds no dependency
obs-scope-summary#3.3 · no · anyhow stays in this crate alone because the session added no dependency and the lock delta adds no dependency edge
obs-scope-summary#4.1 · no · the cited architecture excerpt is not a reached file and the crate map it cites is unchanged
obs-scope-summary#4.2 · no · the not-instrumentable status holds because this pure type crate's source and dependencies are unchanged on the changed Cargo.toml and lock
obs-scope-summary#4.3 · no · the no-I/O, no-runtime reading holds because the session changed no source of this pure type crate and the lock delta is the tokio entry alone
obs-scope-summary#5.1 · no · the portable-pty =0.8.1 pin and its Stack entry hold because Cargo.toml moved at line 242 only and the lock delta is the tokio entry alone
obs-scope-summary#5.2 · no · instrumentability holds because the seam and its OS API boundary are unchanged and no crate gained or lost tokio
obs-scope-summary#5.3 · no · the boundary-only ConPTY and openpty reading holds because no seam code changed and the vendored ConPTY files are outside the reached files
obs-scope-summary#6.1 · no · interprocess 2.4.4 and the IPC method set hold because the lock delta is the tokio entry alone and no channel method changed
obs-scope-summary#6.2 · no · instrumentability holds because the sync client and server are unchanged and no crate gained or lost tokio
obs-scope-summary#6.3 · no · the sync-only containment holds on the bumped lock because the session re-measured it, channel-tokio-free reads ok, and it is unmoved
obs-scope-summary#7.1 · no · ndjson, atomic-write-file and sysinfo 0.39.6 hold because the lock delta is the tokio entry alone
obs-scope-summary#7.2 · no · instrumentability holds because no source changed and the sync-only containment it rests on is unmoved
obs-scope-summary#7.3 · no · the Tokio ban holds on the bumped lock because the session re-measured the containment and it is unmoved
obs-scope-summary#8.1 · no · serde_path_to_error and its drift-report source hold because the lock delta is the tokio entry alone
obs-scope-summary#8.2 · no · instrumentability holds because no source changed and no crate gained or lost tokio
