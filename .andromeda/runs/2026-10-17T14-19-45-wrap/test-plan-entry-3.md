
## 2026-10-17-mcp-server-for-drivers — harness keys and the time mechanism
**Section:** §3 → Bootstrap phases (derive for route / setup-project); §3 → 5-command implementation (`harness_session`); §7 Test Data & Fixtures (Time-sensitive data); §8 Mocking & Stubbing Discipline (Time mocking)
**Change:**
- Bootstrap phases: the `ci` profile's 20 s kill line holds twenty-one verify-driven binaries (the named list on that filter line, `mcp_tools` joined; its cases read 0.01–5.9 s; `path6_budget_harness` not on it) — was twenty; `viola-e2e` takes rmcp `>=3.4.1, <3.5` with `client` only and tokio `=1.53.1` `rt` + `process` — was `client` + `transport-child-process` and tokio `test-util`.
- 5-command implementation: was "`harness_session` … with `stamp:true`"; now `stamp: !opts.unstamped`, `SessionOptions::unstamped` default false (the in-test form of `boot --unstamped`; Path 7's MCP leg boots an unstamped `builder` alone).
- §7 and §8: no manifest names tokio `test-util` (as landed: `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`); `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout`, a unit case — so no `start_paused` test exists.
**Why:** the counted list is the filter line's names (the count's rule); the unstamped option serves the Path 7 MCP leg; the time qualifier keeps a planned mechanism from reading as built.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
