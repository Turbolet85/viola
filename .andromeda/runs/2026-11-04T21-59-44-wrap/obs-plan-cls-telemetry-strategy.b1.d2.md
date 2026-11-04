@answer n=1 entity=telemetry-strategy#1.1 v=no
why: no span, field or emitting call site changed, and the session is test-only with product code unchanged
@answer n=2 entity=telemetry-strategy#1.2 v=no
why: no event enum, required field or emitting boundary changed, so the structured-log rows stand
@answer n=3 entity=telemetry-strategy#1.3 v=no
why: product code unchanged, so the catch_unwind sites and the handle-wait thread catch stay as written
@answer n=4 entity=telemetry-strategy#1.4 v=no
why: no metric, counter or perf gate changed, and no gate command or step moved
@answer n=5 entity=telemetry-strategy#1.5 v=no
why: the events.ndjson audit writers are untouched and no product file changed
@answer n=6 entity=telemetry-strategy#2.1 v=no
why: the panic hook and its bounded exemption list are untouched and no src file changed
@answer n=7 entity=telemetry-strategy#3.1 v=no
why: no IPC method, channel field or allow-list changed
@answer n=8 entity=telemetry-strategy#3.2 v=no
why: no paste-text path or send-refused side changed
@answer n=9 entity=telemetry-strategy#3.3 v=no
why: the hook server-verify behaviour is unchanged and the session's macOS carve-out count names wait, last and MCP, not this hook line
@answer n=10 entity=telemetry-strategy#3.4 v=no
why: the MCP surface and its channel fields are unchanged
@answer n=11 entity=telemetry-strategy#3.5 v=no
why: CLI and env handling are unchanged and no env var changed
@answer n=12 entity=telemetry-strategy#3.6 v=no
why: the strict-modes checks and log file modes are unchanged, and the witness adds no new open or exemption
@answer n=13 entity=telemetry-strategy#3.7 v=no
why: the child spawn subject values and parse-rejected paths are unchanged
@answer n=14 entity=telemetry-strategy#3.8 v=no
why: no Cargo.toml or Cargo.lock moved and the cargo-deny gates are unchanged
@answer n=15 entity=telemetry-strategy#3.9 v=no
why: the corr and conn propagation and Scenarios 2 and 4 are unchanged
@answer n=16 entity=telemetry-strategy#3.10 v=no
why: the hyperfine budgets and perf gates are unchanged
@answer n=17 entity=telemetry-strategy#3.11 v=yes mode=stale
why: the new integration witness freezes a live wrapper and back-dates its heartbeat, and the wrapper SIGKILL now proves the child's kill-fallback exit on every OS
@answer n=18 entity=telemetry-strategy#3.12 v=yes mode=stale
why: the session measured the per-OS JUnit sets and widened Unix coverage (ubuntu-not-macos 48), so the three-OS reading moved
@answer n=19 entity=telemetry-strategy#3.13 v=no
why: the panic and SLO invariants and the zero-unlogged-panics budget are unchanged
@answer n=20 entity=telemetry-strategy#3.14 v=no
why: the log file location and the no-rotation rule are unchanged
@answer n=21 entity=telemetry-strategy#3.15 v=no
why: the log schema stays verbatim and no schema file moved
@answer n=22 entity=telemetry-strategy#3.16 v=no
why: the strict-modes refusal and the detail code catalog are unchanged and no exit code moved
@answer n=23 entity=telemetry-strategy#3.17 v=no
why: the installed-CLI counters and the parse-rejected and skipped details are unchanged
@answer n=24 entity=telemetry-strategy#3.18 v=no
why: the new witness reads list's terminal stream escape sequences rather than a scraped screen, and no screen-scraping rule changed
