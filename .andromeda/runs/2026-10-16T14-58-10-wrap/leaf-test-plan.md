group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 96 · «- 2026-10-16: A test fixture's synthetic session id has the »
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:9   class: stale
  leaf-text: «eight agent-driven surfaces across three required OSes»
  source: .andromeda/test-plan.md:23 «Test-scope Sec 2 has 6 agent-driven surface entries»
  replace-with: «six agent-driven surfaces across three required OSes»

- leaf: .claude/docs/tests-summary.md:17   class: stale
  leaf-text: «`agent-run status` aggregates `viola list --json`, `GET /ready` and cookie-gated `/api/sessions` (`state: ready|degraded|down`, `api_sessions_equal_list`); interim, while `status` runs no `viola list` (its harness reading owed to "End-to-end path binary and harness legs") and `viola ui` is unbuilt,»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:3 «Its nested `list` member is the verbatim `viola list --json` document»
  replace-with: «`agent-run status` aggregates `viola list --json` alone (`state: ready|degraded|down`; `ui` stays `null`, and `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`); interim, while `status` runs no `viola list` (its harness reading owed to "End-to-end path binary and harness legs"),»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «MCP / SSE / web halves owed to `:118` / `:147` / `:155`; readback `open → read`»
  source: .andromeda/test-plan.md:652 «the MCP `send` step and its verification are owed to `:118`.»
  replace-with: «the MCP `send` step and its verification owed to `:118`»

- leaf: .claude/docs/tests-summary.md:27   class: stale
  leaf-text: «the MCP / `/api/sessions` / page-STATUS steps owed to `:118` / `:145` / `:155`»
  source: .andromeda/test-plan.md:673 «The MCP `wait` / `last` steps and their verification are owed to `:118`.»
  replace-with: «the MCP `wait` / `last` steps owed to `:118`»

- leaf: .claude/docs/tests-summary.md:28   class: stale
  leaf-text: «the MCP / api / web steps owed to their surface chunks»
  source: .andromeda/test-plan.md:693 «The MCP `answer` step is owed to its surface chunk.»
  replace-with: «the MCP `answer` step owed to its surface chunk»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «the MCP `send` step owed to `:118`, the Playwright WHEEL cell to `:155`»
  source: .andromeda/test-plan.md:720 «The MCP `send` step and its refusal checks are owed to `:118`.»
  replace-with: «the MCP `send` step and its refusal checks owed to `:118`»

- leaf: .claude/docs/tests-summary.md:30   class: retired
  leaf-text: «"End-to-end path binary and harness legs", the web signals to Epoch 8, and the MCP form of the refusal»
  source: .andromeda/test-plan.md:767 «the MCP form of the refusal to "MCP server for drivers".»
  replace-with: «"End-to-end path binary and harness legs", and the MCP form of the refusal»

- leaf: .claude/docs/tests-summary.md:32   class: stale
  leaf-text: «unwrapped hooks no-op · R8 env strip · link/unlink markers · SSE `Last-Event-ID` resume · snapshot corruption → replay»
  source: .andromeda/test-plan.md:823 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «unwrapped hooks no-op · R8 env strip · link/unlink · E4 (SSE `Last-Event-ID` resume) deferred with the frontend to 0.2.0 · snapshot corruption → replay»

- leaf: .claude/docs/tests-summary.md:33   class: stale
  leaf-text: «cross-surface parity, bay layout states, chaos, property (7 parsers), contract»
  source: .andromeda/test-plan.md:375 «proptest 1.11.0 over all 6 parser surfaces named by the property-test trigger»
  replace-with: «cross-surface parity, chaos, property (6 parsers), contract»

- leaf: .claude/docs/tests-summary.md:50   class: stale
  leaf-text: «verdicts come from events, payloads, receipts and DOM attributes.»
  source: .andromeda/test-plan.md:1165 «Verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (R7).»
  replace-with: «verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.»

- leaf: .claude/docs/tests-summary.md:61   class: retired
  leaf-text: «- Arch requests pending: `viola ui --port 0`. (The spine deadline is `viola_core::SPINE_DEADLINE`; the confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` = 10 s, validated per CLI version by the `confirm-window` ledger row.)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:15 «- Timeout: 20 s per instance after the build.»
  replace-with: «- The spine deadline is `viola_core::SPINE_DEADLINE`; the confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` = 10 s, validated per CLI version by the `confirm-window` ledger row.»

- leaf: .claude/rules/testing.md:19   class: retired
  leaf-text: «with assert_cmd 2.2.2, trycmd 1.2.1, axum-test 21.1.0, jsonschema 0.57.0.»
  source: .andromeda/test-plan.md:373 «Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite»
  replace-with: «with assert_cmd 2.2.2, trycmd 1.2.1, jsonschema 0.57.0.»

- leaf: .claude/rules/testing.md:20   class: stale
  leaf-text: «`crates/viola-e2e/tests/{path,mcp,http,sse,cross}_<topic>.rs` (rmcp client, reqwest, eventsource-client)»
  source: .andromeda/test-plan.md:398 «This covers the rmcp client, cross-surface parity and critical-path scenarios, as `tests/{path,mcp,cross}_<topic>.rs`.»
  replace-with: «`crates/viola-e2e/tests/{path,mcp,cross}_<topic>.rs` (rmcp client)»

- leaf: .claude/rules/testing.md:20   class: retired
  leaf-text: «; render, contrast and a11y verdicts are judged on the ubuntu leg.»
  source: .andromeda/test-plan.md:1023 «(The a11y lint needs `e2e-web/node_modules`, so it runs in the ubuntu leg of the `test` job after its `npm ci`; see the E2E row.)»
  replace-with: «; the a11y lint runs in the ubuntu leg of the `test` job.»

- leaf: .claude/rules/testing.md:28   class: stale
  leaf-text: «`viola wait --after`, an SSE `id:`, a status field or an auto-waiting locator.»
  source: .andromeda/test-plan.md:1164 «Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.»
  replace-with: «`viola wait --after`, a status field or an auto-waiting locator.»

- leaf: .claude/rules/testing.md:39   class: stale
  leaf-text: «the fake-agent receipt, exit codes and DOM attributes.»
  source: .andromeda/test-plan.md:1165 «Verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (R7).»
  replace-with: «the fake-agent receipt and exit codes.»

- leaf: .claude/rules/testing.md:43   class: retired
  leaf-text: «- Web selectors: roles + `data-*` + exact text; never CSS classes (`.band`, `.rb`), inline `style` or xpath; never `bypassCSP` or `toHaveScreenshot`.»
  source: .andromeda/test-plan.md:823 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/rules/verification-harness.md:23   class: stale
  leaf-text: «`supervise`, `ui-restart`, `gate`,»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:101 «- **`ui-restart --session <id>`**: the shims forward it unchanged; `viola-harness` implements no such subcommand.»
  replace-with: «`supervise`, `gate`,»

- leaf: .claude/rules/verification-harness.md:28   class: retired
  leaf-text: «(master writer kept until the child exits, one drain thread per master) → optional `viola ui` → token exchange (303 + `HttpOnly; SameSite=Strict`, cookie stored 0600, never printed).»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:9 «5. Spawn the supervisor `viola-harness supervise --session <id>` as an ordinary child process»
  replace-with: «(master writer kept until the child exits, one drain thread per master).»

- leaf: .claude/rules/verification-harness.md:29   class: retired
  leaf-text: «Still to join: UI `/health` ok and `/ready` `"ready"`. Failure reasons are the closed list in test-plan §3.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:14 «`{"v":1,"cmd":"boot","ok":false,"reason":"build-failed"|"verify-failed"|"run-exited"|"readiness-timeout",»
  replace-with: «Failure reasons are the closed list in test-plan §3 (`build-failed`, `verify-failed`, `run-exited`, `readiness-timeout`).»

- leaf: .claude/rules/verification-harness.md:30   class: retired
  leaf-text: «- After UI readiness, byte-compare every embedded `/assets/*` with the repo file (`stale-embedded-assets`).»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:14 «`{"v":1,"cmd":"boot","ok":false,"reason":"build-failed"|"verify-failed"|"run-exited"|"readiness-timeout",»
  replace-with: «»

- leaf: .claude/rules/verification-harness.md:34   class: stale
  leaf-text: «- `status`: `viola list --json` + `/ready` + cookie-gated `/api/info` and `/api/sessions`; `api_sessions_equal_list` deep-equals `.ok.items`; exit 0 only for `state:"ready"`. Interim, while `status` runs no `viola list` (owed to "End-to-end path binary and harness legs") and `viola ui` is unbuilt:»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:68 «- `ui` and `api_sessions_equal_list`: `null`»
  replace-with: «- `status`: `viola list --json` alone; `ui` and `api_sessions_equal_list` are `null`; exit 0 only for `state:"ready"`. Interim, while `status` runs no `viola list` (owed to "End-to-end path binary and harness legs"):»

- leaf: .claude/rules/verification-harness.md:35   class: stale
  leaf-text: «`port_free` and `url_file_removed` `null` until their surfaces exist); verify endpoint gone (CLI exit 21, socket path absent), port free, `ui/<port>.url` removed.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:82 «`endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` stay `null`.»
  replace-with: «`port_free` and `url_file_removed` stay `null`); verify endpoint gone (CLI exit 21, socket path absent).»
