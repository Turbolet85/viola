# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the 1.96 floor and 1.98.1 pin bound the chunk's new directory, owner-mode and euid code, which must compile on the declared MSRV
stack-and-technologies#1.2 · yes · the std-threads-and-blocking rule binds the new euid and directory checks on run, hook, send and channel-server paths; Tokio stays only in mcp
stack-and-technologies#1.3 · no · clap dispatches the named verbs unchanged; the chunk adds no verb, flag or parser config
stack-and-technologies#1.4 · no · the PTY, ConPTY and host-tty libc raw-mode pins are untouched; the chunk's uid reads are carried by rows 1.6 and 1.9
stack-and-technologies#1.5 · no · vt100 readiness on run's feed thread is PTY-seam work the chunk does not touch
stack-and-technologies#1.6 · yes · interprocess 2.4.4 local_socket carries the peer euid read and the socket chmod 0600 after bind, and libc gives the uid for the socket directory
stack-and-technologies#1.7 · yes · the instance snapshot's recorded endpoint, and the rule that each check runs before any log opens because opening a log narrows an existing home
stack-and-technologies#1.8 · yes · a replayed snapshot carries no endpoint, so the instance reads unreachable; the new endpoint field and any refusal line follow the v-skip and schema-filename rules
stack-and-technologies#1.9 · yes · the private directory create and verify through libc Unix modes and owner, and the .lock arbiter the leftover-socket takeover runs beside
stack-and-technologies#1.10 · no · sha2 content hashing of the exe and pinned copies; the chunk hashes nothing
stack-and-technologies#1.11 · yes · the new refusal `--json` documents printed through serde_json keep their declared key order, v first, via unreachable_document and refused_document
stack-and-technologies#1.12 · no · standing S4 · one-line JSON process logs into diagnostics/, one line per write inside the schemas, is the form S4 fixes for each new log line
stack-and-technologies#1.13 · no · nutype domain newtypes (ViolaName, Percent, DialogHold) are not touched; the socket name is not a ViolaName
stack-and-technologies#1.14 · yes · the server check compares the peer's OS start time with the snapshot's started_at, which is read in its RFC 3339 UTC millisecond form
stack-and-technologies#1.15 · yes · one thiserror enum per crate for the new refusals, closed values extended in viola-core, anyhow only at the root bin
stack-and-technologies#1.16 · yes · the euid refusal comes before the first JSON-RPC frame on each per-wrapper local socket, so it adds no method or kind
stack-and-technologies#1.17 · yes · sysinfo's pid and process start time is the source of the OS start time the server-verify compares against started_at
stack-and-technologies#1.18 · yes · the MCP frame tools' refusal reaches the client as an isError tool result through rmcp, the same document less v; the rmcp feature set is unchanged
stack-and-technologies#1.19 · no · viola calls no model API; nothing in the chunk touches it
stack-and-technologies#1.20 · no · no phone view is in scope
stack-and-technologies#1.21 · no · cargo install distribution and the plugin written out by run are unchanged; the chunk adds no distribution
stack-and-technologies#1.22 · yes · the macOS carve-out and the §6 macOS exit-cause case are OS-only checks that must run on the macos-latest runner; the Linux-only pre-push gate does not stand in for them
stack-and-technologies#1.23 · yes · cargo-deny's sole-root tokio ban and the deny.toml allow list bind any dependency the chunk adds; clippy's print bans bind the new refusal output
stack-and-technologies#1.24 · no · v1.x release tooling (dist, cargo-auditable, self_update) is deferred; distribution is out of scope
stack-and-technologies#2.1 · yes · the client-side peer identity read through StreamCommon::peer_creds() is what each frame-writing client checks before its first frame; macOS yields none, the named carve-out
