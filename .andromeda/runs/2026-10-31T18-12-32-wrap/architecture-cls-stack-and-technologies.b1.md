# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the toolchain pins, rust-version 1.96 and edition 2024 stand; the Cargo.toml change is one cross_ [[test]] stanza and rust-toolchain.toml is untouched
stack-and-technologies#1.2 · no · the std-thread and Tokio-in-mcp split is unchanged; no thread or async code moved and no tokio dependency changed
stack-and-technologies#1.3 · no · the clap configuration and the subcommand list stand; json_asked is a catch-site argv read in main.rs and adds no subcommand or parser feature
stack-and-technologies#1.4 · no · viola-pty, the ConPTY and openpty hosting and the DLL-search setup are outside the change; main.rs changes only the catch site
stack-and-technologies#1.5 · no · run still reads no content and verify still alone reads Screen::rows() for its --record fixtures; the verify.rs change is exit log codes and Ended returns
stack-and-technologies#1.6 · no · the endpoint open, SQOS adoption, socket chmod and per-user directory are unchanged; the session changes only the CLI's no-endpoint cause mapping
stack-and-technologies#1.7 · no · no writer, lock or append path changed; the new process-exit codes are words inside an existing ndjson log line
stack-and-technologies#1.8 · no · the schema filename diag-line.v1.json and its v1 stand; only the process-exit detail enum grew, which this row does not list
stack-and-technologies#1.9 · no · replace_private, persist_noclobber, File::lock and the libc and windows-sys strict-modes check are unchanged; without_endpoint opens and writes nothing
stack-and-technologies#1.10 · no · the pinned-copy re-hash before reuse is unchanged; pinned-hash-mismatch only gains a printed document, not a different hash path
stack-and-technologies#1.11 · no · the serde and serde_json pins and preserve_order stand; the exit-21 document's key order fits the stated rule
stack-and-technologies#1.12 · no · the tracing setup and the one-line JSON diagnostics format, owned by the obs plan, are unchanged; the new process-exit detail words are values, not the line format
stack-and-technologies#1.13 · no · ViolaName, Percent and DialogHold are untouched; the new causes are plain enums, not nutype values
stack-and-technologies#1.14 · no · the chrono use is unchanged and the session adds no timestamp path
stack-and-technologies#1.15 · no · thiserror per crate and anyhow at the bin edge stand; internal_error_document sits at main.rs's catch site and the crate error enums gain nothing
stack-and-technologies#1.16 · no · per-wrapper JSON-RPC over ndjson is unchanged; JSON-RPC errors and channel frames are unmoved
stack-and-technologies#1.17 · no · the heartbeat stays the primary signal and the sysinfo and claude agents enrichment is not changed; liveness.rs adds without_endpoint, which reads the directory only
stack-and-technologies#1.18 · no · the rmcp features, the tool list and the spec version stand; the session changes the MCP isError body, which this row does not describe
stack-and-technologies#1.19 · no · no model API is added and the session changes no call toward the claude CLI
stack-and-technologies#1.20 · no · N/A row; the session adds no phone view
stack-and-technologies#1.21 · no · cargo install and the include_str! plugin embedding are unchanged; plugin.rs changes a refused write, not the plugin's embedding or its write-out by viola run
stack-and-technologies#1.22 · no · the matrix, the SHA-pinned actions and the toolchain and CI tool pins read as they stood; this row does not name nextest or a check count
stack-and-technologies#1.23 · no · the lint, deny, fuzz and gate tools are unchanged; the print bans and the orphans gate stand and scripts/ is untouched
stack-and-technologies#1.24 · no · the release row is deferred to v1.x and no release path changed
stack-and-technologies#2.1 · no · both clients still read the peer pid before the first frame, none on macOS as stated; a failed server check now reads server-verify-failed, which this row does not state
