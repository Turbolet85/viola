
## 2026-09-29-verify-stamped-test-homes-and-harness — run --local-live specified; its suite, reason and codes closed
**Section:** §3 `run` (`--local-live` bullet; Output format); §3 `gate` (`--require`); §3 Closed enums; §12 Test Decisions Log (2026-09-29)
**Change:**
- `--local-live`: under `CI` (presence only, passed to `run_with` as `ci`) it is `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite; otherwise, after the selected suites, `cargo build --workspace --features viola/fake-agent` into `target/harness`, then one `<harness bins>/viola --home target/e2e-home/viola-live-<pid>/home verify` against the real `claude`, both through the runner seam. Suite `local-live` passes 1 on exit 0 with each of the six literal row ids naming one `  pass` line and a `… 0 fail` summary; else it fails 1 with `build`, `verify-exit-<n>`, `verify-exit-none` or `row-missing`. It claims no H2 measurement.
- Output format `suite` gains `local-live`; `gate --require` takes the nine other values; Closed enums add suite `local-live` and its four codes, `run` reason `live-in-ci`, and the `boot` readiness missing codes list with `<name>:events`.
**Why:** the plan left the live run's binary unstated, so it builds first, as `--perf` does, and a verify that yields no exit code is named; a new closed value takes a Decisions Log entry.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/
