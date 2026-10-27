# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the new code must build on the 1.96 MSRV floor under the exact 1.98.1 pin that both reproducible-build release builds use
stack-and-technologies#1.2 · yes · the fix on the run and verify paths uses std threads and blocking I/O only, Tokio stays inside mcp
stack-and-technologies#1.3 · no · the chunk adds no subcommand, flag, feature or dependency to clap, and run and verify keep their dispatch
stack-and-technologies#1.4 · yes · the Windows companion reading rests on the sideloaded conpty.dll's absolute-path load and the inbox-ConPTY fail-open fallback that the scope names
stack-and-technologies#1.5 · no · the vt100 screen model is untouched, since run's feed thread and verify's probe screens are not edited
stack-and-technologies#1.6 · no · the chunk sends no frame and changes no endpoint, so the pin race is upstream of the channel
stack-and-technologies#1.7 · no · the pinned copy is a binary, not an ndjson log or JSON snapshot, and the chunk adds no authoritative state
stack-and-technologies#1.8 · no · standing S4 · a new process-log line keeps no v and its version is the schema filename
stack-and-technologies#1.9 · yes · replace_private_shared and replace_private are the helper the absent-copy write goes through (mode before write, sync_all, persist), and any guard is a std File::lock on a separate .lock sibling
stack-and-technologies#1.10 · yes · the content hash keys bin/ and plugin/ <version>-<hash> and re-hashes an existing pinned copy before every reuse, the rule the chunk re-asserts
stack-and-technologies#1.11 · no · the chunk prints no document and adds no record whose declared key order a reader depends on
stack-and-technologies#1.12 · no · standing S4 · a new log line goes only through obs_event! to the role's codes-only process log
stack-and-technologies#1.13 · no · the pin key joins the CLI version and the content hash, never an instance name, and the chunk reads no newtype
stack-and-technologies#1.14 · no · the chunk writes no timestamp, and verify's edit is the pin_exe call only
stack-and-technologies#1.15 · yes · the absent-copy write's new failure mode is a variant of viola-state's per-crate thiserror enum with a fixed message, and anyhow stays at the bin edge
stack-and-technologies#1.16 · no · no broker is used and the chunk adds no socket or frame
stack-and-technologies#1.17 · no · pid and start-time liveness is untouched, and the pin guard is a file lock that reads no liveness
stack-and-technologies#1.18 · no · no MCP tool or result shape changes, and the plugin's .mcp.json is a path write, not an rmcp contract
stack-and-technologies#1.19 · no · viola calls no model API, so no serving rule is met by the chunk
stack-and-technologies#1.20 · no · a phone view is a later version, outside this chunk
stack-and-technologies#1.21 · yes · run writes the plugin from its compiled-in files at every start, which is the per-start rewrite the chunk edits and reads for a concurrent hooks.json reader
stack-and-technologies#1.22 · yes · the scope reads Windows behaviour on the windows-2025 CI runner, and the local pre-push gate runs the Linux test suites
stack-and-technologies#1.23 · yes · release-check.sh, whose release build the measurement runs, refuses with tool-missing when jq is absent, and the new code must pass clippy -D warnings and the print bans
stack-and-technologies#1.24 · no · dist and cargo-auditable are v1.x release tooling, outside v1
stack-and-technologies#2.1 · no · the peer-pid identity check guards frame clients, and the chunk sends no frame
