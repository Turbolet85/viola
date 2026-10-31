# Classified — obs-plan · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · the session adds no surface entry and no process role; its cli stderr change alters what a surface prints, not the count of 5 entries across 4 roles
obs-scope-summary#1.2 · no · the new tests/cross_exit_causes.rs is a root test binary, not a crate, and no dependency was added or bumped under Cargo.toml
obs-scope-summary#1.3 · no · no must-trace path or edge flow was added or removed; the run and verify texts changed content, not the count of 6 and 4
obs-scope-summary#1.4 · no · no trigger was added or removed; Vector 7 and Founder Direction 4 changed their reading, not the count of 18
obs-scope-summary#2.1 · yes · stale · the integration-test setups moved into reached tests/support/start.rs, and the report does not show the test-home `<home>/diagnostics/` reading still holds there
obs-scope-summary#2.2 · no · the session changes MCP result bodies in result.rs and tools.rs, not mcp.ndjson or the stderr fallback, and the mcp catch site stays silent
obs-scope-summary#2.3 · no · no event value or field was added, so the event enum extensions are not reached
obs-scope-summary#2.4 · no · hook is untouched: src/cmd/hook.rs is not in the diff, so hook-invoked corr timing stands
obs-scope-summary#3.1 · no · no dependency was added or bumped, so the clap, vt100, thiserror and anyhow stack entries hold
obs-scope-summary#3.2 · no · the cli crate stays instrumentable: the catch site still logs its chain and line, and under --json adds only a stdout document
obs-scope-summary#3.3 · no · anyhow stays only in this crate and the full chain still goes to diagnostics/ only; under --json the catch site prints a fixed internal-error document
obs-scope-summary#4.1 · no · the viola-core source entry is unchanged; the session adds a type to viola-core and no dependency
obs-scope-summary#4.2 · no · viola-core stays pure with no line of its own, so not-instrumentable holds
obs-scope-summary#4.3 · yes · add-to · viola-core also supplies `UnreachableCause`, the closed set of exit 21's five cause words (F11), with no serde path, so it joins the kebab-case values listed here
obs-scope-summary#5.1 · no · no viola-pty file is in the diff and no dependency was bumped, so the portable-pty source entry holds
obs-scope-summary#5.2 · no · no viola-pty file is in the diff, so the seam stays instrumentable
obs-scope-summary#5.3 · no · no viola-pty file is in the diff, and the PTY byte stream is still never logged
obs-scope-summary#6.1 · no · viola-channel is not reached and the IPC method list is unmoved; the channel wire and its logging are untouched
obs-scope-summary#6.2 · no · nothing in viola-channel changed, so instrumentable stands
obs-scope-summary#6.3 · no · no dependency was added or bumped, so the Tokio ban on this graph holds
obs-scope-summary#7.1 · no · no dependency was added or bumped, so the ndjson, atomic-write-file and sysinfo source entry holds
obs-scope-summary#7.2 · no · viola-state gains without_endpoint, a reader that opens nothing and writes no line, so it stays instrumentable
obs-scope-summary#7.3 · no · no Tokio dependency was added, so the sync-only rule for viola-state holds
obs-scope-summary#8.1 · no · no dependency was added or bumped, so the serde_path_to_error source entry holds
obs-scope-summary#8.2 · no · no dependency moved and the report shows no new logging path, so instrumentable stands
