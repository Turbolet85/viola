group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 95 · «- 2026-10-16: A test fixture's synthetic session id has the »
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:15   class: retired
  leaf-text: «+ `cargo test --doc`; Playwright 1.63.0 headless Chromium (all three CI OSes and the native Linux `pre-push`, `run --browser` only; under the pinned Node v24.21.0) for the page»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:31 «3. None in 0.1.0: the browser step left with the browser leg, and the mutation step keeps its number 4.»
  replace-with: «+ `cargo test --doc`»

- leaf: .claude/docs/tests-summary.md:17   class: retired
  leaf-text: «`ui` stays `null`, and `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:3 «This is the `agent-run status` document. Its nested `list` member is the verbatim `viola list --json` document (see Section 1, Status endpoint shape).»
  replace-with: «`pid` and `uptime_ms` are `null`»

- leaf: .claude/docs/tests-summary.md:17   class: retired
  leaf-text: «`list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «`list`, `pid` and `uptime_ms` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`.»
  replace-with: «`list`, `pid` and `uptime_ms` are `null`»

- leaf: .claude/docs/tests-summary.md:43   class: retired
  leaf-text: «nextest `retries = 0`, Playwright `retries: 0`»
  source: .andromeda/test-plan.md:1114 «The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget.»
  replace-with: «nextest `retries = 0`»

- leaf: .claude/docs/tests-summary.md:49   class: retired
  leaf-text: «`#[ignore]` / `test.skip` as a parking place»
  source: .andromeda/test-plan.md:1114 «`#[ignore]` is not allowed as a parking place.»
  replace-with: «`#[ignore]` as a parking place»

- leaf: .claude/rules/testing.md:6   class: tree
  leaf-text: «  - "e2e-web/tests/**"»
  source: .andromeda/test-plan.md:150 «3. none in 0.1.0: the browser suite left with the browser leg, and the mutation step keeps its number 4»
  replace-with: «»

- leaf: .claude/rules/testing.md:20   class: retired
  leaf-text: «(rmcp client) + Playwright 1.63.0 specs `e2e-web/tests/<bay-layout-type>.spec.ts` (the pipe stub `pipe-reachability.spec.ts`, titles `pipe:`, is the one exception) on all three CI OSes; the a11y lint runs in the ubuntu leg of the `test` job.»
  source: .andromeda/test-plan.md:1022 «| E2E | `agent-run run --e2e` on all 3 OSes | matrix per OS | rust-cache |»
  replace-with: «(rmcp client) on all three CI OSes.»

- leaf: .claude/rules/testing.md:24   class: retired
  leaf-text: «naming its label); Playwright `test('<layout type>: <expected state>')` with the scenario id in the title.»
  source: .andromeda/test-plan.md:411 «Critical-path scenarios: `path<N>_<slug>`, for example `path2_send_confirms_with_cl1_events`.»
  replace-with: «naming its label).»

- leaf: .claude/rules/testing.md:28   class: retired
  leaf-text: «`viola wait --after`, a status field or an auto-waiting locator. nextest `retries = 0`, Playwright `retries: 0`; no `#[ignore]` / `test.skip` / `test.fixme` as a parking place.»
  source: .andromeda/test-plan.md:1158 «NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.»
  replace-with: «`viola wait --after` or a status field. nextest `retries = 0`; no `#[ignore]` as a parking place.»

- leaf: .claude/rules/testing.md:46   class: retired
  leaf-text: «`run --unit` / `run --integration` · **browser (every OS, only when named — never under `--all`):** `run --browser` · **one spec:** `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts --grep "<title>"` (from the repo root `--config` is required)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:18 «Command body: `scripts/agent-run.sh run [--unit|--integration|--e2e|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>]»
  replace-with: «`run --unit` / `run --integration`»

- leaf: .claude/rules/verification-harness.md:24   class: stale
  leaf-text: «PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin <argv>`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:130 «every child `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root»
  replace-with: «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>`»

- leaf: .claude/rules/verification-harness.md:24   class: stale
  leaf-text: «Its `tools` stage checks `node` against ci.yml's `NODE_PIN_VERSION` (detail `node`), and `linux-tests` runs `run --coverage`, `run --browser`, then `gate --require coverage,doctest,playwright` (the document's `linux{run,browser,gate}`).»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:131 «`linux-tests` (`run --coverage`, then `gate --require coverage,doctest`), the last stage: `ok:true` when it is green.»
  replace-with: «Its `linux-tests` stage runs `run --coverage`, then `gate --require coverage,doctest` (the document's `linux{run,gate}`).»

- leaf: .claude/rules/verification-harness.md:33   class: retired
  leaf-text: «`viola list --json` alone; `ui` and `api_sessions_equal_list` are `null`; exit 0»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:3 «This is the `agent-run status` document. Its nested `list` member is the verbatim `viola list --json` document (see Section 1, Status endpoint shape).»
  replace-with: «`viola list --json` alone; `pid` and `uptime_ms` are `null`; exit 0»

- leaf: .claude/rules/verification-harness.md:33   class: retired
  leaf-text: «`list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`, and `state`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «`list`, `pid` and `uptime_ms` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`.»
  replace-with: «`list`, `pid` and `uptime_ms` are `null`, and `state`»

- leaf: .claude/rules/verification-harness.md:34   class: retired
  leaf-text: «(`endpoint_gone` reported from the wrapper channel on; `port_free` and `url_file_removed` stay `null`)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:76 «Verification: reported in `{"cleaned":[…],"processes_gone":true,"endpoint_gone":true,"home_removed":true,"killed":[<names force-killed>]}`.»
  replace-with: «(`endpoint_gone` reported from the wrapper channel on)»

- leaf: .claude/rules/verification-harness.md:44   class: retired
  leaf-text: «then doctest), `run --fuzz-replay` (Linux only) and `run --browser` (every OS, only when named: `npm ci` — `npm.cmd` on Windows, fixed argv, no shell — then a `node -e` Chromium probe, then `node node_modules/@playwright/test/cli.js test`, suite `playwright` from `pw.json` `stats`, the stale reports deleted first; a failed `npm ci` or absent Chromium is `reason:"browser-missing"`) drive»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:31 «3. None in 0.1.0: the browser step left with the browser leg, and the mutation step keeps its number 4.»
  replace-with: «then doctest) and `run --fuzz-replay` (Linux only) drive»

- leaf: .claude/rules/verification-harness.md:44   class: retired
  leaf-text: «never nest `cargo llvm-cov`, `cargo fuzz` or Playwright inside nextest»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:31 «3. None in 0.1.0: the browser step left with the browser leg, and the mutation step keeps its number 4.»
  replace-with: «never nest `cargo llvm-cov` or `cargo fuzz` inside nextest»
