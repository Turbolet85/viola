@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: the viola-pty spawn/write/wait/kill span form is what G1 bare #[instrument] checks, and the portable-pty seam is bumped
@answer n=2 entity=telemetry-strategy#1.2 v=yes
why: the process-start and process-exit lines of the PTY seam (the item 1 row, the claude-child spawn and the handle-wait exit of item 6) are the lifecycle lines this rule covers
@answer n=3 entity=telemetry-strategy#1.3 v=yes
why: the handle-wait thread catch_unwind, the zero panic-line error budget of item 8, and the fixed-Display errors item 3 may fold all fall under this rule
@answer n=4 entity=telemetry-strategy#1.4 v=no
why: the chunk adds no metric, instrument or grouping key (not_needed item 1) and names no perf gate in scope
@answer n=5 entity=telemetry-strategy#1.5 v=yes
why: scope items 6 and 11 name the no-payload-in-process-log behaviour (paste as text_bytes only; no send text in a home-level file) for the live session diagnostics
@answer n=6 entity=telemetry-strategy#2.1 v=yes
why: G2 and the zero panic-line budget of item 8 count the single-line panic record, and the detail-file drop before upload (item 11) rules over where its payload lives
@answer n=7 entity=telemetry-strategy#3.1 v=yes
why: the task prose re-measures the channel-handle no-leak property on Linux, which is the Vector 1 IPC surface this trigger covers
@answer n=8 entity=telemetry-strategy#3.2 v=yes
why: item 6 names paste as text_bytes only on the PTY seam, which is the Vector 2 paste surface
@answer n=9 entity=telemetry-strategy#3.3 v=yes
why: item 14 names the socket-dir-refused wording and the hook-decision line the hook's connect writes, which this Vector 4 trigger covers
@answer n=10 entity=telemetry-strategy#3.4 v=no
why: the chunk names no MCP tool, mcp code path or frame verb, so the Vector 5 MCP surface is untouched
@answer n=11 entity=telemetry-strategy#3.5 v=yes
why: item 3 may fold the fixed-Display scrubbing layer that the §7 fixed messages carry to the CLI
@answer n=12 entity=telemetry-strategy#3.6 v=yes
why: items 9 and 11 name the secret-scan-before-upload and pre-upload drop obligations of the filesystem trigger, which is the scope's own obligation
@answer n=13 entity=telemetry-strategy#3.7 v=yes
why: item 6 names process-start subject claude-child on spawn, the Vector 8 child-spawn subject value
@answer n=14 entity=telemetry-strategy#3.8 v=yes
why: item 5 names the supply-chain trigger itself: a Tokio-free graph, no C-building crates, and cargo deny passing
@answer n=15 entity=telemetry-strategy#3.9 v=no
why: the chunk adds no channel method, verb or cross-surface call (not_needed item 3), so corr and conn propagation is untouched
@answer n=16 entity=telemetry-strategy#3.10 v=no
why: no perf budget row or hyperfine gate is named in scope
@answer n=17 entity=telemetry-strategy#3.11 v=yes
why: item 6 names exit_source kill-fallback on the kill fallback and the process-exit from the handle-wait thread, which this chaos trigger covers
@answer n=18 entity=telemetry-strategy#3.12 v=yes
why: item 13 names the ubuntu-latest-only matrix and the remnant removal must leave no OS-specific exporter, artifact or gate branch
@answer n=19 entity=telemetry-strategy#3.13 v=yes
why: item 8 names the zero error budget (0 panic lines, 0 secret-scan hits, 0 schema failures) that this SLO trigger covers
@answer n=20 entity=telemetry-strategy#3.14 v=no
why: no rotation, retention or file is added, and events.ndjson retention is unchanged (not_needed item 6)
@answer n=21 entity=telemetry-strategy#3.15 v=yes
why: G4 schema conformance against schemas/diag-line.v1.json and schemas/diag-detail.v1.json is named in item 7, and this trigger states the schema verbatim
@answer n=22 entity=telemetry-strategy#3.16 v=yes
why: item 14 names the socket-dir-refused wording and its detail code, which the Founder Direction 4 exit-code and detail-catalog rule carries
@answer n=23 entity=telemetry-strategy#3.17 v=no
why: the chunk adds no counter and measures no installed-CLI parse-rejected or skipped count
@answer n=24 entity=telemetry-strategy#3.18 v=yes
why: items 10 and 12 read the live sessions through logs only, which is the no-screen-scraping evidence method this trigger rules over
