# The perf arm's hyperfine argv (plan Implementation notes, "Command shape under `-N`")

`run --perf` runs hyperfine once per row with the current dir at the repository root, `VIOLA_NAME=builder` and
`VIOLA_DIR=<perf home>/instances/builder` set on the hyperfine process (the benchmarked `viola hook` inherits them).
The exact argv, pinned by `perf_times_every_row_on_the_perf_build_and_cleans_up` in
`crates/viola-e2e/src/harness/run/perf.rs`:

```
hyperfine -N --warmup 3 --runs 30 --input target/agent-run/<session>/payload-<hook>.json --export-json target/agent-run/artifacts/perf-<hook>.json "target/perf/release/viola[.exe] hook <hook>"
```

`<session>` is `perf-<harness pid>`; `<hook>` is each of `session-start`, `user-prompt-submit`, `stop`,
`session-end`. The command string holds no space inside the executable path, so `-N`'s whitespace split yields
exactly `[target/perf/release/viola.exe, hook, <hook>]`, and every path is repo-relative, so the export carries no
host path.

## The host reading (Windows, implement gate 15, 2026-09-28)

hyperfine 1.20.0 printed `Benchmark 1: target/perf/release/viola.exe hook <hook>` for each row, 30 runs each:

| row | mean ± σ | min … max |
|---|---|---|
| session-start | 19.9 ms ± 23.7 ms | 9.1 ms … 73.0 ms |
| user-prompt-submit | 22.0 ms ± 25.4 ms | 9.1 ms … 72.8 ms |
| stop | 26.1 ms ± 28.4 ms | 8.9 ms … 73.1 ms |
| session-end | 30.3 ms ± 30.2 ms | 8.9 ms … 73.7 ms |

`gate --require perf` (gate 16) read all four rows present and each `max` below 1.0 s: `{"ok":true,"breaches":[]}`.
