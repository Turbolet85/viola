# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · rust-version 1.96, the rust-toolchain pin and edition 2024 hold; the report moves no crate, dependency or config key and Cargo.toml gains only [[test]] tables
stack-and-technologies#1.2 · yes · stale · src/cmd/run.rs and src/cmd/client.rs are reached whole and the report does not show their std-thread model holds; Tokio stays only in viola-mcp (no dependency moved)
stack-and-technologies#1.3 · no · clap features are unchanged (no dependency moved) and the report adds, removes and renames no CLI verb
stack-and-technologies#1.4 · no · portable-pty, windows-sys and libc are unchanged and the report leaves [PTY] and the ConPTY sideload untouched
stack-and-technologies#1.5 · no · vt100 0.16.2 is unchanged and dialog.rs product rules are unmoved (test additions only); the Screen::rows() reads in verify are not among the session's changes
stack-and-technologies#1.6 · yes · stale · crates/viola-channel/src/client.rs and server.rs are reached whole and the report does not show the Windows pipe open (client::open_handle) or the server DACL holds on them
stack-and-technologies#1.7 · no · the session adds no store and viola-state was not edited; the channel-response line is a process-log line, not a database record
stack-and-technologies#1.8 · no · the schema is still named schemas/diag-line.v1.json (edited in place), so the version-in-filename and no-v rule hold
stack-and-technologies#1.9 · no · viola-state was not edited and no dependency moved, so the replace_private and place_private_new primitives hold
stack-and-technologies#1.10 · no · sha2 =0.11.0 and the [profile.dev.package.sha2] config key are unchanged; no dependency or config key moved
stack-and-technologies#1.11 · no · serde, serde_json (preserve_order) and serde_path_to_error are unchanged and no feature moved; the drift reports still form and only their text is kept out of documents
stack-and-technologies#1.12 · no · tracing and veil are unchanged (no dependency moved) and the line format and levels stay owned by the obs plan, which the session does not edit
stack-and-technologies#1.13 · no · nutype 0.8.0 is unchanged and the session adds is_code_word as a plain function, not a newtype; DialogHold's product rules are unmoved
stack-and-technologies#1.14 · no · chrono 0.4.45 is unchanged and no timestamp rule moved
stack-and-technologies#1.15 · no · thiserror 2.0.20 and anyhow 1.0.104 are unchanged (no dependency moved), PtyError stays hand-written, and AgentError stays a per-crate typed error
stack-and-technologies#1.16 · no · the stack has no broker and the session adds no socket or daemon
stack-and-technologies#1.17 · no · sysinfo 0.39.6 and claude agents --json are unchanged; the report moves no heartbeat, pid or start-time rule
stack-and-technologies#1.18 · no · rmcp 3.4.1, the five tools send, wait, last, answer and list, and the viola-e2e client-only rule are unchanged; the report moves no tool or dependency
stack-and-technologies#1.19 · no · viola calls no model API and the session adds none
stack-and-technologies#1.20 · no · a phone view is out of scope and the session adds nothing of the kind
stack-and-technologies#1.21 · no · cargo install --path, the include_str! plugin and the local-only deployment are unchanged; no build or distribution rule moved
stack-and-technologies#1.22 · no · the CI matrix, toolchain installs and action pins are unchanged; .github/workflows/ci.yml was only read and no CI step moved
stack-and-technologies#1.23 · no · the lint, audit and fuzz tool pins are unchanged; no dependency moved and no harness verb moved, so run --perf, --coverage and --mutants hold
stack-and-technologies#1.24 · no · the release stack is out of v1 scope and the session touches no release rule
stack-and-technologies#2.1 · no · the peer_creds pid check before the first frame is a pid rule the report shows unmoved (no heartbeat, pid or start-time rule moved)
