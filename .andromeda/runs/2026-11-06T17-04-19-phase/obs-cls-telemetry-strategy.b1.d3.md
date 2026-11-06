@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: the `#[instrument(skip_all, name = ...)]` declaration form is what the G1 bare-`#[instrument]` gate re-runs at the wrap
@answer n=2 entity=telemetry-strategy#1.2 v=yes
why: the every-process-lifecycle-is-a-closed-event-enum-line rule is what the re-measured `process-start` log-coverage row and the seam's process-start/process-exit lines (needs 1, 6) must hold
@answer n=3 entity=telemetry-strategy#1.3 v=yes
why: the catch_unwind sites in the run PTY pump and handle-wait threads, the fixed-Display thiserror errors and the panic record are what the portable-pty bump and the error-census fold (needs 2, 3, 8) must keep
@answer n=4 entity=telemetry-strategy#1.4 v=no
why: metrics are derived by jq and perf gates run on hyperfine; the chunk adds no metric (not_needed 1) and re-measures no perf value
@answer n=5 entity=telemetry-strategy#1.5 v=no
why: `events.ndjson` audit stream and its no-rotation rule are untouched; retention is unchanged (not_needed 6)
@answer n=6 entity=telemetry-strategy#2.1 v=yes
why: one panic record per panic after init step 4 is what the zero error budget (0 panic lines, G2 zero panics) in needs 8 counts
@answer n=7 entity=telemetry-strategy#3.1 v=yes
why: the re-measured channel-handle no-leak property on Linux is the IPC vector's check, witnessed by the §6 channel field allow-list
@answer n=8 entity=telemetry-strategy#3.2 v=yes
why: paste as `text_bytes` only at the PTY seam wrapper (needs 6) is the paste-text vector this row covers
@answer n=9 entity=telemetry-strategy#3.3 v=yes
why: the `hook-decision{detail:"socket-dir-refused"}` WARN line is the socket-directory refusal code whose wording needs 14 ratifies or re-words
@answer n=10 entity=telemetry-strategy#3.4 v=no
why: the MCP surface and its §6 fields are not named by the scope; McpError appears only as a fold-or-ratify candidate and the MCP log path is not touched
@answer n=11 entity=telemetry-strategy#3.5 v=yes
why: the §7 fixed messages are the fixed-Display scrubbing layer that the one-enum fold (needs 3) changes
@answer n=12 entity=telemetry-strategy#3.6 v=yes
why: the secret-scan-before-upload gate on `junit-<os>` and `diag-<os>` uploads is named by needs 9 and 11 and re-run by needs 7
@answer n=13 entity=telemetry-strategy#3.7 v=yes
why: `process-start{subject:"claude-child"}` on spawn through the bumped PTY seam is named by needs 6
@answer n=14 entity=telemetry-strategy#3.8 v=yes
why: the supply-chain trigger (Tokio-free, C-crate-free, cargo deny gates on the new portable-pty graph) is needs 5 and the §9 cargo-deny gates
@answer n=15 entity=telemetry-strategy#3.9 v=no
why: the chunk adds no channel method, verb or cross-surface call and touches no corr/conn join (not_needed 3)
@answer n=16 entity=telemetry-strategy#3.10 v=no
why: perf-budget instruments and the §10 budget table are not re-measured or changed by the chunk
@answer n=17 entity=telemetry-strategy#3.11 v=yes
why: the `exit_source:"kill-fallback"` process-exit path the killed-wrapper reading rests on is named by needs 6
@answer n=18 entity=telemetry-strategy#3.12 v=yes
why: the `[ubuntu-latest]` matrix, one file sink and one line schema on Linux, and the `matrix.os`-form artifact names are named by needs 13 and needs 7
@answer n=19 entity=telemetry-strategy#3.13 v=yes
why: the zero-unlogged-panics error budget is the zero error budget (0 panic lines) that needs 8 must clear
@answer n=20 entity=telemetry-strategy#3.14 v=no
why: the chunk changes no log file location or rotation; retention is unchanged and none is added (not_needed 6)
@answer n=21 entity=telemetry-strategy#3.15 v=yes
why: the §3 diag-line schema stated verbatim is what G4 schema conformance against `schemas/diag-line.v1.json` re-asserts at the wrap (needs 7)
@answer n=22 entity=telemetry-strategy#3.16 v=yes
why: the strict-modes and `socket-dir-refused` exit codes (21 and 1) and the logged detail code are the start refusals whose wording needs 14 ratifies or re-words
@answer n=23 entity=telemetry-strategy#3.17 v=no
why: the §5 counters and the parse-rejected/skipped installed-CLI measurement are not named by the scope and the chunk changes none
@answer n=24 entity=telemetry-strategy#3.18 v=yes
why: the no-screen-scraping rule (evidence read through logs, never human review) is what the live-session evidence reading of needs 10 must honour
