@carried n=1 label=obs-plan/metric-coverage fact=fct-wl54ld47
why: the G2 zero-panics gate must read green on ubuntu-latest and macos-latest beside windows-2025, so the panic count stays 0 on every OS the full fake-agent suite runs on.
quote: | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |
@carried n=2 label=obs-plan/metric-coverage fact=fct-dhdxvyfp
why: the per-OS perf job the chunk re-reads green on both Unix OSes must gate the hook latency rows on the hyperfine max, the same statistic on every OS.
quote: | Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |
@carried n=2 run=2 label=obs-plan fact=fct-kvmjumn6
quote: There is no in-process metrics pipeline. Every metric is derived from the JSON-per-line logs or from hyperfine output
@carried n=3 label=obs-plan/metric-coverage fact=fct-l3f7hrdu
why: the chunk re-asserts one diagnostics schema on every OS, so a CLI wait refused by server verification (on macOS an euid mismatch alone) or by its socket directory must log the same single exit-21 process-exit line with the same closed detail word on Linux and macOS as on Windows, and MCP must log none.
quote: The CLI's server-verification refusal (Linux, Windows and macOS, where it means an euid mismatch alone) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, and its socket-directory refusal one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN there, each counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for either.
@carried n=3 run=2 label=obs-plan/metric-coverage fact=fct-iwloyhyc
quote: | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
@carried n=3 run=3 label=obs-plan fact=fct-5mkprlwu
quote: grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`.
@carried n=4 label=obs-plan/metric-coverage fact=fct-tinnk5cr
why: strict-modes-failed is a Unix-reached refusal under the one-schema re-assertion, so the Unix fake-agent suite must read a strict-modes-refused CLI or MCP wait as writing no line at all on both Unix OSes, its exit or isError code its only record.
quote: A CLI `wait` refused by the instance strict-modes check (`client::refuse_untrusted`: exit 21, the instance pair or the `instance-unreachable` document) writes no line at all, neither a `process-exit` nor a `channel-request`, so neither `viola.wait.outcome` nor the Counter row's `detail` grouping counts it; MCP's `wait` tool refused by its `check_instance` (`instance-unreachable`, `detail: "strict-modes-failed"`) writes none either. The exit code or the `isError` code is its only record.
@binding n=1
**metric-coverage Counter and refusal rows ↔ span-trace-coverage wait / last scenario**: the server-verify-failed and socket-dir-refused CLI refusals must each be one exit-21 `process-exit` at WARN with `during:"connect"` and no `channel-request` line on ubuntu-latest, macos-latest and windows-2025 alike, with macOS comparing an euid alone, so the Counter row's closed `detail` grouping counts them the same on every OS (obs-plan/metric-coverage fct-l3f7hrdu) (obs-plan/metric-coverage fct-iwloyhyc) (obs-plan fct-q67chfay).
@binding n=2
**metric-coverage Panics row ↔ slo-invariants-telemetry-budgets error budget and ci-integration G2**: `viola.panic` must be 0 as G2 counts it, over home-level role files only with the fake-agent seam's forced panic its one exemption, measured by the in-job gate exit code on each of the three OSes (obs-plan/metric-coverage fct-wl54ld47) (obs-plan fct-zze2ezbr) (obs-plan fct-eodqccd6).
@binding n=3
**metric-coverage hook latency row ↔ slo-invariants-telemetry-budgets performance budgets**: the per-OS perf job must gate the five timed hook rows on the hyperfine `max` on both Unix OSes, and obs overhead on hook holds only with `VIOLA_NAME` set and a home under `target/e2e-home/` (obs-plan/metric-coverage fct-dhdxvyfp) (obs-plan fct-onnnq5uj).
@binding n=4
**metric-coverage strict-modes no-line row ↔ error-capture-reporting MCP rejects**: a strict-modes-refused wait must write no process-log line on any OS, CLI or MCP, its exit code or `isError` code its only record (obs-plan/metric-coverage fct-tinnk5cr) (obs-plan fct-pqz5mgd5).
