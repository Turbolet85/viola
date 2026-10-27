# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the toolchain pin, the 1.96 floor and the edition are unchanged, and no dependency was added or bumped
stack-and-technologies#1.2 · no · the new AsyncRead reader sits inside viola-mcp, the one Tokio crate, so the std-thread CLI path and Tokio containment hold as a re-read
stack-and-technologies#1.3 · no · the subcommand set and clap's feature set are unchanged, and mcp is already listed; only its end of session on a stdin cut changed
stack-and-technologies#1.4 · no · the PTY seam and its portable-pty, ConPTY and openpty hosting are unchanged; the chaos test adds a passthrough check, not a PTY change
stack-and-technologies#1.5 · no · the feed thread is still handed a copy of the pump's output and run still reads no content; the panicked and oversize reading is carried by established-decisions
stack-and-technologies#1.6 · no · no endpoint or channel transport changed, and the stdin cap sits on stdio rather than the local socket
stack-and-technologies#1.7 · no · still no database, and the one new mcp.ndjson line is an ndjson append like the rest
stack-and-technologies#1.8 · no · process-log lines still carry no v and their version still lives in schemas/diag-line.v1.json, which holds the added parser value inside the same v1 file
stack-and-technologies#1.9 · no · replace_private's mode before write, sync_all and persist are re-read unchanged; the fs.rs change bounds only the fallback compare read in replace_private_shared, which this row does not describe
stack-and-technologies#1.10 · no · the sha2 content hash and the 64 KiB re-hash of a pinned copy are untouched; the fs.rs change bounds a byte compare, which hashes nothing
stack-and-technologies#1.11 · yes · add-to · F4: the serde_json entry gains deny.toml's [[bans.features]] ban on unbounded_depth, probed by the deny-probes row serde_json-unbounded-depth
stack-and-technologies#1.12 · no · the line format and levels stay owned by the obs plan, and the new mcp.ndjson WARN line is one more line in that format
stack-and-technologies#1.13 · no · no newtype was added or changed, and the stdin cap uses the existing MAX_FRAME constant
stack-and-technologies#1.14 · no · the RFC 3339 millisecond timestamp format and chrono are unchanged, and the new mcp.ndjson line is an ordinary obs line
stack-and-technologies#1.15 · no · thiserror per crate, anyhow at the bin edge and ServeError are unchanged, and the cut reads as end of stream rather than a new error type
stack-and-technologies#1.16 · no · no channel, socket or broker changed; the cut is on the stdio JSON-RPC stream, not a per-wrapper local socket
stack-and-technologies#1.17 · no · sysinfo and the claude agents read are unchanged, and no liveness code path changed
stack-and-technologies#1.18 · no · rmcp stays at server and transport-io, viola-e2e still takes rmcp client alone and owns the viola mcp child, and the new mcp_tools case reuses that child
stack-and-technologies#1.19 · no · viola still calls no model API and the session touches no model path
stack-and-technologies#1.20 · no · a phone view stays a later version and the session touches nothing of it
stack-and-technologies#1.21 · no · cargo install --path . and the include_str! plugin files are unchanged, and no plugin/ file is reached
stack-and-technologies#1.22 · no · the matrix, toolchain installs and pinned actions are unchanged, and the new contract binary runs under the existing run --e2e with no added job
stack-and-technologies#1.23 · no · the row names feature bans as a category and the changed deny.toml still holds that category, so the serde_json ban is one more feature ban carried by 1.11; no tool, version or gate changed
stack-and-technologies#1.24 · no · the release tools are not touched and nothing in the session moves the release path
stack-and-technologies#2.1 · no · the sync and Tokio clients keep their pid read and pre-frame identity check, and lib.rs changes only the stdin wiring and the cut's exit path
