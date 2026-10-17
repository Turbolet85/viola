group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 96 · «- 2026-10-16: An oracle literal for a list or a count the pl»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:17   class: retired
  leaf-text: «`agent-run status` aggregates `viola list --json` alone (`state: ready|degraded|down`; `pid` and `uptime_ms` are `null`); interim, while `status` runs no `viola list` (its harness reading owed to "End-to-end path binary and harness legs"), `list`, `pid` and `uptime_ms` are `null` and `state` is `ready` when every recorded wrapper is alive, else `degraded`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «`state` is `ready` when `list` exits 0 and every booted instance is an item `wrapped:true, liveness:"live"` (an unwrapped item is ignored; a session that booted none reads `ready`).»
  replace-with: «`agent-run status` runs `<bin dir>/viola --home <home> list --json` (the bin dir and session `bin/` from `supervise.json`, the session `PATH`, `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed) and aggregates it (`state: ready|degraded|down`; `pid` and `uptime_ms` are `null`): `ready` when `list` exits 0 and every booted instance is an item `wrapped:true, liveness:"live"`, `degraded` when one is `stale` or absent or the document holds no `items`, `down` when `list` exits non-zero (`list-exit-<code>`), gives no exit code, cannot be spawned or has no `supervise.json` (`list-exit-none`), or exits 0 with no JSON object (`list-exit-0`)»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «Still owed: `boot --statusline-echo`, the harness-session form and the two-instance leg to the route entry "End-to-end path binary and harness legs", and the MCP form of the refusal to "MCP server for drivers".»
  source: .andromeda/test-plan.md:765 «The harness-session form and step 7's two-instance leg landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `path6_two_instance_release_over_a_harness_session`»
  replace-with: «`boot --statusline-echo`, the harness-session form and step 7's two-instance leg landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `path6_two_instance_release_over_a_harness_session` (`crates/viola-e2e/tests/path6_budget_harness.rs`). Still owed: the MCP form of the refusal to "MCP server for drivers".»

- leaf: .claude/rules/testing.md:20   class: stale
  leaf-text: «**E2E:** `crates/viola-e2e/tests/{path,mcp,cross}_<topic>.rs` (rmcp client) on all three CI OSes.»
  source: .andromeda/test-plan.md:398 «as `tests/{path<N>,mcp,cross}_<topic>.rs` (a numbered Path binary `path<N>_<topic>.rs`, e.g. `path6_budget_harness.rs`). The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync,»
  replace-with: «**E2E:** `crates/viola-e2e/tests/{path<N>,mcp,cross}_<topic>.rs` (rmcp client; a numbered Path binary, e.g. `path6_budget_harness.rs`) on all three CI OSes, plus the root package's numbered sync Path binaries (`tests/path1_run_start.rs`, `[[test]]` `required-features = ["fake-agent"]`); both report in the E2E layer (`run --e2e`, suite `nextest-e2e`), whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`.»

- leaf: .claude/rules/testing.md:45   class: retired
  leaf-text: «(a whole root test file: `binary(<stem>)`; `--e2e` is a usage error, exit 2, until the first E2E binary lands)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:53 «an E2E-prefixed binary through `run --e2e --filter 'binary(path1_run_start)'` (suite `nextest-e2e`), which `run --integration` never selects»
  replace-with: «(a whole root test file: `binary(<stem>)`; an E2E-prefixed binary through `run --e2e --filter 'binary(path1_run_start)'`, suite `nextest-e2e`, which `run --integration` never selects)»

- leaf: .claude/rules/verification-harness.md:28   class: omission
  leaf-text: «unless `--unstamped` → `viola-harness supervise` as an ordinary child»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «A stale file is removed before step 4 and the value written (`replace_private`, 0600) after step 4 and the statusline plant, before `supervise.json`»
  replace-with: «unless `--unstamped` → under `--statusline-echo` (clap-conflicts with `--unstamped`) `<home>/statusline-source.json` planted at 0600 with the command `<bin dir>/viola-fake-agent[.exe] statusline-echo <home>/fake/statusline.marker` (no env var, no `budget.json`) → the `--agents-mode recorded|oversize|malformed` value (default `recorded`; a stale file removed before verify) written to the `agents-mode` file beside `bin/claude[.exe]`, after verify because Run B reads `claude agents --json` through that copy, and recorded as `agents_mode` in `session.json` and `supervise.json` → `viola-harness supervise` as an ordinary child»

- leaf: .claude/rules/verification-harness.md:30   class: stale
  leaf-text: «tests use the `harness_session` fixture or the rstest chain»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:26 «`viola_e2e::harness::session::harness_session(test, label, opts) -> HarnessSession`, a plain function and drop guard (`viola-e2e` carries no `[dev-dependencies]`, so never an rstest fixture)»
  replace-with: «tests use `viola_e2e::harness::session::harness_session(test, label, opts) -> HarnessSession` (a plain function and drop guard, never an rstest fixture; the library `boot` spawns the `supervise` process from the given binary dir) or the rstest chain»

- leaf: .claude/rules/verification-harness.md:33   class: retired
  leaf-text: «`status`: `viola list --json` alone; `pid` and `uptime_ms` are `null`; exit 0 only for `state:"ready"`. Interim, while `status` runs no `viola list` (owed to "End-to-end path binary and harness legs"): `list`, `pid` and `uptime_ms` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:59 «`<bin dir>/viola --home <home> list --json`, the bin dir and the session `bin/` read from `supervise.json` (`session.json` records neither), with `PATH` the session path»
  replace-with: «`status`: `<bin dir>/viola --home <home> list --json` alone (the bin dir and session `bin/` from `supervise.json`, `PATH` the session path, `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed, stdout through `take(MAX_FRAME + 1)`; with no `supervise.json` it spawns nothing); `pid` and `uptime_ms` are `null`; `ready` when `list` exits 0 and every booted instance is `wrapped:true, liveness:"live"`, `degraded` when one is `stale` or absent, `down` on a non-zero exit (`list-exit-<code>`) or no exit, spawn or `supervise.json` (`list-exit-none`); exit 0 only for `state:"ready"`.»

- leaf: .claude/rules/verification-harness.md:42   class: omission
  leaf-text: «The `viola_e2e::fixtures` copy lands with its first E2E consumer.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:27 «Path 1's flow is not one, landing on the root chain as `tests/path1_run_start.rs`»
  replace-with: «The `viola_e2e::fixtures` copy lands with its first E2E consumer; Path 1's flow is not one, landing on the root chain as `tests/path1_run_start.rs`.»
