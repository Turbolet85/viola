# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S1 · span and log discipline: `#[instrument(skip_all, fields(..))]`, events only through `obs_event!`, no field outside the Section 6 catalog
telemetry-strategy#1.2 · yes · the closed `event` enum, which the Needs keep untouched (no wire or log-line change)
telemetry-strategy#1.3 · no · the panic hook and catch sites sit in code the chunk does not edit, and the scope names no change to panic capture
telemetry-strategy#1.4 · no · metrics are jq-derived, and the chunk adds no metric, instrument or exporter and runs no hyperfine gate
telemetry-strategy#1.5 · no · the arch-owned `events.ndjson` audit stream (never rotated) is written, rotated and copied by no code the chunk edits
telemetry-strategy#2.1 · no · the zero-unlogged-panics invariant and its exemptions are Not-needed, since no hook, run or heartbeat code changes
telemetry-strategy#3.1 · no · Vector 1 (IPC) is a Not-needed trigger: the channel surface and its field allow-list are not edited
telemetry-strategy#3.2 · no · Vector 2 (paste text) is a Not-needed trigger: no `send` paste or validation path is touched
telemetry-strategy#3.3 · no · Vector 4 (hook stdin) is a Not-needed trigger: no hook code or hook log sink is edited
telemetry-strategy#3.4 · no · Vector 5 (MCP rejects) is a Not-needed trigger: no viola-mcp reject or §4 mcp-surface row is edited
telemetry-strategy#3.5 · no · Vector 6 (CLI/env) is a Not-needed trigger: no env read, env stripping or fixed message is touched
telemetry-strategy#3.6 · no · Vector 7 (filesystem) is a Not-needed trigger: no log-file mode, state-recovery or strict-modes path is touched, and its §6 codes sit in the catalog #3.16 carries
telemetry-strategy#3.7 · no · Vector 8 (child spawn) is a Not-needed trigger: no viola-pty spawn path or spawn subject is touched
telemetry-strategy#3.8 · yes · the Vector 9 supply-chain trigger: obs dependencies stay Tokio-free in the sync crates, add no C-building crate and pass cargo deny
telemetry-strategy#3.9 · no · corr + conn propagation runs over channel frames and Scenarios 2/4, none of which the bump or the scope edits
telemetry-strategy#3.10 · no · the hyperfine budgets are Not-needed: no hook, run or heartbeat code changes and no perf gate is in the chunk's re-run list
telemetry-strategy#3.11 · no · the chaos fields `exit_source` and `during` sit on recovery and exit paths the chunk does not touch
telemetry-strategy#3.12 · no · per-OS log-file modes, the concurrent-append check and per-OS artifacts sit in code and CI jobs the chunk does not edit
telemetry-strategy#3.13 · no · the error budget (zero-unlogged-panics) is Not-needed, and no hook, run or heartbeat code changes
telemetry-strategy#3.14 · no · audit-log retention (no rotation) is over `events.ndjson`, which the chunk neither writes nor rotates
telemetry-strategy#3.15 · yes · the §3 schema verbatim, which the Needs keep untouched (`schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`)
telemetry-strategy#3.16 · yes · the §6 detail code catalog, which the Needs keep untouched (no code added or changed)
telemetry-strategy#3.17 · no · the installed-CLI measured trigger covers CLI parse counters and parse codes the chunk neither re-measures nor emits, and the catalog is carried by #3.16
telemetry-strategy#3.18 · no · the no-screen-scraping rule is over the vt100 gate outcome in viola-agent-claude, which the chunk does not touch
