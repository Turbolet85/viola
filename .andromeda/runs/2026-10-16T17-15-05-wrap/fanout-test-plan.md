proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`run` loses `--browser` from its command body and the never-default sentence, and step 3 (Browser: the `npm ci` / Chromium probe / Playwright CLI spawn, the test-scoped sessions, the cross-runner rule, the non-DOM rule and the `browser-missing` paragraph) is removed, the mutation step keeping its number 4 (cited as `run` step 4 throughout); Output format lists the nine `suite` values `nextest-unit|nextest-integration|doctest|nextest-e2e|mutants|coverage|perf|fuzz-replay|local-live` and is built with no `e2e-web/pw.json`; `archived` holds the `junit-{nextest-unit,nextest-integration,coverage}.xml` and `outcomes.json` sources only (no playwright arm, no `pw-junit.xml`); `skipped` names no Playwright `test.skip` / `test.fixme`; Test selection drops the one-browser-test `npx … playwright test` line'
    sidecar: '5-command `run`: the browser step 3, `--browser`, the `playwright` suite, its archive source and its test-selection line leave (chunk 2026-10-16-remove-the-browser-leg); mutation stays step 4'
    rationale: 'report Changes → Symbols / APIs (harness `run`: the `--browser` selector, `Selection::browser`, the `browser` module and the `browser-missing` refusal are gone; `archive_sources` archives the three nextest/coverage JUnit files and `outcomes.json` only) and Files (`crates/viola-e2e/src/harness/run/browser.rs` deleted); the key file still specifies the browser step as built'
    basis: 'crates/viola-e2e/src/harness/run.rs (report Files: modified; `crates/viola-e2e/src/harness/run/browser.rs` deleted)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`gate`: `--require` takes only the eight closed `suite` values other than `local-live` (`nextest-unit · nextest-integration · doctest · nextest-e2e · mutants · coverage · perf · fuzz-replay`), `playwright` included among the refused values (usage `unknown-suite`); Inputs drop the `e2e-web/pw-junit.xml` → `junit-playwright.xml` copy (JUnit for the four `nextest-unit · nextest-integration · nextest-e2e · coverage`); the skip breach loses its `playwright` parenthetical; the CI sentence reads `coverage,doctest` in the per-OS `test` job'
    sidecar: '5-command `gate`: eight closed suites, four JUnit inputs, `--require playwright` is usage `unknown-suite`; CI `test` job gates `coverage,doctest`'
    rationale: 'report Changes → Symbols / APIs (harness `gate`: `SUITES` is the eight, `JUNIT` the four, `gate --require playwright` usage exit 2 `unknown-suite`, gate entry 8) and Counts (closed `suite` values 9 → 8; suites with a JUnit file 5 → 4; CI `test` job `gate --require coverage,doctest`)'
    basis: 'crates/viola-e2e/src/harness/gate.rs:416-421 (`parse_require_refuses_playwright`, report New text)'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'Closed enums: `run` `suite` is the nine values in `run` Output format and `gate` `--require` takes the other eight; the `run` `reason` list drops `browser-missing` (and its browser-verdict-reachability clause); `secret-scan` hit `class` is `claude-stripped`, `cookie`, `content-canary`, `mode`, `unreadable` (no `token-query`, no `gui-token`); `pre-push` `detail` drops `node` and the clause on an unreadable `run --browser` document'
    sidecar: 'Closed enums: suites 10/9 → 9/8, `browser-missing` and the `token-query` / `gui-token` classes and the pre-push `node` detail removed'
    rationale: 'report Changes → Symbols / APIs (run reason values lose `browser-missing`; secret-scan `token-query` / `?t=`, `gui-token` and `viola_<port>=` gone; pre-push `node_pin` / `NODE_BIN` gone, `tools` checks no `node`) and Counts (closed `suite` values 9 → 8)'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 Test Harness Contract
    change: 'Exit codes: the `unknown-suite` usage detail reads "a `gate --require` value outside the eight suites"'
    sidecar: '§3 Exit codes: `unknown-suite` names the eight gate suites'
    rationale: 'report Changes → Counts / qualifiers moved (closed `suite` values 9 → 8, `gate::SUITES`); the §3 body still says nine'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 Test Harness Contract
    change: 'Session record: `target/agent-run/<session>/session.json` is `{v, session, home, instances, supervisor_pid, supervisor_started_at}` (no `ui`, no `cookie_file`; an older record carrying both still parses, `viola-e2e` holds no `deny_unknown_fields`); the parallel-sessions clause drops "one per Playwright test, per §3 `run` step 3"'
    sidecar: '§3 Session record: `ui` and `cookie_file` leave the record; no Playwright session clause'
    rationale: 'report Changes → Symbols / APIs (`SessionRecord` drops `ui` and `cookie_file`: `{v, session, home, instances, supervisor_pid, supervisor_started_at}`); site test-plan.md:449 (report Expected amendments)'
    basis: 'test-plan.md:449'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → Status endpoint shape
    change: 'the `agent-run status` document holds the keys `v, cmd, ok, state, pid, uptime_ms, last_error, list, instances` — no `"ui": null` and no `"api_sessions_equal_list": null`; the lead sentence drops "`ui` stays `null`" and the interim sentence names only `list`, `pid` and `uptime_ms` as `null`'
    sidecar: 'Status endpoint shape: `ui` and `api_sessions_equal_list` removed'
    rationale: 'report Changes → Symbols / APIs (harness `status`: the document drops `"ui": null` and `"api_sessions_equal_list": null`, measured by the smoke entry `status --session p-rbl-smoke` with `lacks api_sessions_equal_list`); obs-plan §3 reads this shape, so the change is one-sided until applied here'
    basis: 'crates/viola-e2e/src/harness/status.rs:26 (report New text)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`status` Polled fields: the bullet "`ui` and `api_sessions_equal_list`: `null`" is removed'
    sidecar: '5-command `status`: the two null keys leave the polled fields'
    rationale: 'report Changes → Symbols / APIs (harness `status` keys); duplicate of the Status endpoint shape claim'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`cleanup`: Verification reports `{"cleaned":[…],"processes_gone":…,"endpoint_gone":…,"home_removed":…,"killed":[…]}` with no `port_free` or `url_file_removed` and no "stay `null`" clause; step 5 removes the home with its `viola-session-*` parent and `session.json` (no cookie file), and in CI `session.json` alone'
    sidecar: '5-command `cleanup`: `port_free`, `url_file_removed` and the cookie-file removal leave'
    rationale: 'report Changes → Symbols / APIs (harness `cleanup`: the report drops `"port_free": null` and `"url_file_removed": null`, keys `v, cmd, ok, cleaned, processes_gone, endpoint_gone, home_removed, killed`; `SessionRecord` drops `cookie_file`), measured by the smoke entry with `lacks port_free`'
    basis: 'crates/viola-e2e/src/harness/cleanup.rs:70-71 (report New text)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §7 Test Data & Fixtures → Test data lifecycle
    change: 'Per-session: `cleanup` removes the harness home, `session.json` and the supervisor (no cookie file)'
    sidecar: '§7 Test data lifecycle: no cookie file in the per-session cleanup'
    rationale: 'report Changes → Symbols / APIs (`SessionRecord` drops `cookie_file`); duplicate of the cleanup claim'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`pre-push`: every child runs under `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` (no `<home>/.local/viola-node/bin`); the `tools` stage checks the passwd home, `cc`, the `rust-toolchain.toml` channel and the `test`-job `tool:` line pins only (no `node --version` / `NODE_PIN_VERSION`); `linux-tests` runs `run --coverage` then `gate --require coverage,doctest`; the document is `{…,"stage",linux{run,gate}}` (no `browser`); a green pass makes nine harness-runner calls'
    sidecar: '5-command `pre-push`: PATH 5 → 4 elements, no Node check, `linux-tests` = `run --coverage` + `gate --require coverage,doctest`, `linux` sections 3 → 2'
    rationale: 'report Changes → Symbols / APIs (harness `pre-push`: `NODE_BIN` and `node_pin` gone, PATH element left, `linux` holds `run` and `gate` only) and Counts (PATH elements 5 → 4, `linux` sections 3 → 2, nine runner calls pinned by `pre_push_every_native_call_carries_only_home_and_path`)'
    basis: 'crates/viola-e2e/src/harness/pre_push/linux.rs (report Files: modified)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`secret-scan` Classes: `claude-stripped` (the twelve fixed `CLAUDE*` canaries), `cookie` (the case-insensitive `cookie:` header form only), `content-canary`, `mode`, `unreadable` — `token-query` (`?t=`), `gui-token` (`ui/*.url`) and the `viola_<digits>=` cookie-pair form are removed; the Scope sentence keeps the `chunk.diff` exclusion with its reason restated without `?t=` (repository source text that hits the scan''s own patterns whenever it touches them)'
    sidecar: '5-command `secret-scan`: the three GUI forms leave; `CRITICAL` 13 → 12 needles'
    rationale: 'report Changes → Symbols / APIs (harness `secret-scan`: `CRITICAL` holds the twelve canaries, `token-query` / `?t=` left, `gui-token` and `viola_<port>=` gone, `cookie:` header kept) and Counts (`CRITICAL` needles 13 → 12); pinned by `secret_scan_ignores_the_removed_gui_forms`'
    basis: 'crates/viola-e2e/src/harness/secret_scan.rs (report Files: modified)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → Log format
    change: 'Constraints: no line may contain a `Cookie` header or any stripped `CLAUDE*` value (the GUI token and `?t=` clauses removed), enforced by the §6 secret scan'
    sidecar: 'Log format Constraints: GUI token and `?t=` leave the forbidden list'
    rationale: 'report Changes → Symbols / APIs (secret-scan GUI forms gone); the Log format key is kept equal to obs-plan §3 (report Expected amendments: "Log format (kept equal to obs-plan''s copy)")'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §6 E2E Test Strategy → Non-path suites (Error sanitization and secret scan)
    change: 'the third check reads: every `diagnostics/*` file and log contains no `Cookie` header and no stripped-var values (no token read from `ui/<port>.url`, no `?t=`)'
    sidecar: '§6 Error sanitization: no GUI token or `?t=` check'
    rationale: 'report Changes → Symbols / APIs (secret-scan `gui-token` and `token-query` gone); duplicate occurrence of the secret-scan class claim'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §1 Test Scope Summary → Test harness requirements (Log format)
    change: 'Logs must never contain the `Cookie` header or stripped `CLAUDE*` values (the token and `?t=` removed)'
    sidecar: '§1 Log format requirement: no token / `?t=` clause'
    rationale: 'report Changes → Symbols / APIs (secret-scan GUI forms gone); duplicate occurrence'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §1 Test Scope Summary → Coverage triggers (error sanitization)
    change: 'the grep of all logs and `diagnostics/` must show no `Cookie` or stripped `CLAUDE*` values (token and `?t=` removed)'
    sidecar: '§1 error-sanitization trigger: no token / `?t=` clause'
    rationale: 'report Changes → Symbols / APIs (secret-scan GUI forms gone); duplicate occurrence'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → Log format
    change: 'Process logs write to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson` and the reserved-vocabulary parenthetical is removed; the `corr` rules read `process-start`, `process-exit`, `panic`: null and `state-recovered`, `parse-rejected`: null (no `http-request`, `liveness-changed`, `sse-opened`, `sse-closed`); `process` is `run|hook|mcp|cli`'
    sidecar: 'Log format: the `ui` process, `ui-<port>` role file and the four web events leave (19 → 15 events, 5 → 4 processes); obs-plan §3 copy moves with it'
    rationale: 'report Changes → Symbols / APIs (product obs: `ObsEvent::{HttpRequest, LivenessChanged, SseOpened, SseClosed}` and `ObsProcess::Ui` removed; role files `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`), Schema / config (`diag-line.v1.json` process enum the four, role-file list, `$defs.event` the fifteen) and Counts (19 → 15, 5 → 4); the key file itself says the vocabulary is "removed with the code by Remove the browser leg"'
    basis: 'schemas/diag-line.v1.json:5 (report New text)'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: '`logs` command body: `[--process run|hook|mcp|cli]`'
    sidecar: '5-command `logs`: `ui` leaves the `--process` values'
    rationale: 'report Changes → Symbols / APIs (`ObsProcess::ALL` is the four `run · hook · mcp · cli`); duplicate occurrence of the process-vocabulary claim'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §1 Test Scope Summary → Test harness requirements (`logs`)
    change: '`logs` prints the structured per-process logs of `run` / `hook` / `mcp` / `cli` in the §3 log format'
    sidecar: '§1 `logs` requirement: `ui` replaced by `cli`'
    rationale: 'report Changes → Symbols / APIs (`ObsProcess::Ui` removed; the four processes); duplicate occurrence'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §5 Integration Test Strategy → Boundary types covered (Module ↔ DB)
    change: 'the concurrent-append check names a colliding second `run` on the live `run-<name>` file (no `ui` / `ui-<port>`)'
    sidecar: '§5 Module ↔ DB: no `ui-<port>` append case'
    rationale: 'report Changes → Symbols / APIs (`role_file_name` / `open_role_file` lose `port`; no `ui-<port>` role file); site test-plan.md:555 (report Expected amendments)'
    basis: 'test-plan.md:555'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'Internal harness subcommands: the `ui-restart --session <id>` bullet is removed (neither shim forwards it; the forwarded internal list is `supervise, gate, schema-check, secret-scan, pre-push`)'
    sidecar: '5-command: `ui-restart` leaves the internal subcommands'
    rationale: 'report Changes → Symbols / APIs (the shims: `ui-restart` is no longer forwarded) and Harness / gate surface'
    basis: 'scripts/agent-run.sh:37 (report New text)'
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy → Test pyramid
    change: 'E2E row: the critical paths and expansion scenarios run against harness-booted `viola run … -- viola-fake-agent` through the outer PTY (portable-pty) and the rmcp client; no Playwright, Chromium or Node'
    sidecar: '§2 E2E row: Playwright 1.63.0 / headless Chromium / Node v24.21.0 leave the runner set'
    rationale: 'report Changes → Dependencies (the npm graph with `@playwright/test` 1.63.0 and the Node v24.21.0 pin removed), Dev-tool versions (no tree path uses them), Files (`e2e-web/` deleted whole); the chunk runs no browser runner'
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy → Agent-runnable invariants
    change: 'drop "Playwright writes `pw.json` and `pw-junit.xml`", "Playwright runs only in headless test-runner mode" and "or Playwright auto-waiting locators"'
    sidecar: '§2 invariants: Playwright clauses removed'
    rationale: 'report Changes → Dependencies / Files (Playwright and `e2e-web/` gone); duplicate occurrence of the browser-runner claim'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy → Test directory + naming conventions
    change: 'remove the Browser suite directory (`e2e-web/tests/*.spec.ts`), the Playwright spec file-naming bullet (with the `pipe-reachability.spec.ts` exception) and the Playwright test-function naming bullet'
    sidecar: '§2 naming: browser suite directory and Playwright naming rules removed'
    rationale: 'report Changes → Files (`e2e-web/` deleted, `tests/pipe-reachability.spec.ts` among its six files); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §1 Test Scope Summary → Test harness requirements (`run`)
    change: 'the `run` list drops item 3 (the headless-browser suite selected by `run --browser`); the mutation item keeps its number 4'
    sidecar: '§1 `run` requirement: the browser suite item removed'
    rationale: 'report Changes → Symbols / APIs (harness `run`: `--browser` and the `browser` module gone); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §1 Test Scope Summary → Coverage triggers (multi-os-compat)
    change: 'drop the sentence "The headless-browser suite runs on all three runners."'
    sidecar: '§1 multi-os-compat: no browser suite leg'
    rationale: 'report Changes → Harness / gate surface (`ci.yml` `test` job loses the Chromium and Browser suite steps); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §6 E2E Test Strategy → Non-path suites (Security sweep)
    change: 'drop the runner-split reason "because no nextest test drives Chromium (§3 `run` step 3)"; the sweep lists its nextest homes only'
    sidecar: '§6 Security sweep: no Chromium runner split'
    rationale: 'report Changes → Symbols / APIs (`run` step 3 removed); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent
    change: 'drop "A Rust scenario and its Playwright test load the same file" and "and a Playwright spec (§3 `run` step 3)": a Rust test releases gated steps through `--control`'
    sidecar: '§7 Fake agent: Playwright consumers removed'
    rationale: 'report Changes → Files (`e2e-web/` deleted); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → 5-command implementation
    change: '`boot` `--statusline-echo`: drop "and its `path6` test in `bay-steady-state.spec.ts`" (used by Path 6)'
    sidecar: '5-command `boot`: no Playwright spec reference'
    rationale: 'report Changes → Files (`e2e-web/` deleted); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → 5-command implementation
    change: 'remove the Browser-side controls bullet (the `e2e-web` Playwright fixtures: Hold the tape, Unknown routes after render)'
    sidecar: '5-command: Browser-side controls removed'
    rationale: 'report Changes → Files (`e2e-web/` deleted) and Symbols / APIs (no browser arm); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → Log format
    change: 'remove the Harness-side a11y rows bullet (the a11y Playwright fixture, `e2e-web/test-results/a11y/*.ndjson`, `e2e-web/schemas/a11y-row.v1.json`, the `process` const `"ui"`)'
    sidecar: 'Log format: Harness-side a11y rows removed with the browser leg'
    rationale: 'report Changes → Files (`e2e-web/` deleted) and Expected amendments (a11y-plan: the browser a11y leg''s text leaves; Log format kept equal to obs-plan''s copy)'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: 'test-runner-install: remove the Node side bullet (`e2e-web/package.json`, `@playwright/test@1.63.0`, `e2e-web/tsconfig.json`, `e2e-web/playwright.config.ts` and its five settings)'
    sidecar: 'Bootstrap test-runner-install: Node side removed'
    rationale: 'report Changes → Files (`e2e-web/` six files deleted), Dependencies (npm graph removed) and Counts (tracked `tsconfig.json` 1 → 0)'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: 'ci-tool-install: remove the Node v24.21.0 / `npm ci` / `playwright install chromium` / pre-push Node bullet, and drop `scripts/npm-audit.sh` (`supply-chain`, nightly `npm-advisories`) from the runner-`jq` script list'
    sidecar: 'Bootstrap ci-tool-install: Node, npm and `npm-audit.sh` removed'
    rationale: 'report Changes → Files (`scripts/install-node.sh`, `scripts/npm-audit.sh` deleted), Harness / gate surface (`NODE_PIN_*`, `Node (pinned)`, npm steps, `npm-advisories` job removed)'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration
    change: 'Caches: rust-cache is the only cache, in `ci.yml` only (the npm / Playwright-browser / Node sentence removed)'
    sidecar: '§9 Caches: npm, browsers and Node removed'
    rationale: 'report Changes → Harness / gate surface (Node and browser steps removed); site test-plan.md:1015 (report Expected amendments)'
    basis: 'test-plan.md:1015'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration
    change: 'remove the paragraph "The browser test leg named here … stands until the route entry Remove the browser leg removes it; that entry''s wrap amends this text." — this is that wrap'
    sidecar: '§9: the stand-until browser-leg paragraph removed'
    rationale: 'the paragraph names this chunk as the one that removes the leg; report Changes → Harness / gate surface and Files'
    basis: 'test-plan.md:1017'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Pipeline structure
    change: 'Lint row: drop the parenthetical on the a11y lint needing `e2e-web/node_modules` in the ubuntu `test` leg'
    sidecar: '§9 Lint row: a11y lint parenthetical removed'
    rationale: 'report Changes → Files (`e2e-web/` deleted) and Harness / gate surface (no npm step); site test-plan.md:1023'
    basis: 'test-plan.md:1023'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Pipeline structure
    change: 'E2E row: `agent-run run --e2e` on all 3 OSes; the `run --browser` leg (Node (pinned), `npm ci`, Chromium install, the shim run, the a11y-lint ordering) is removed; Cache column rust-cache'
    sidecar: '§9 E2E row: the browser leg removed'
    rationale: 'report Changes → Harness / gate surface (`test` job loses `Node (pinned)`, `Browser suite dependencies (npm ci)`, `Chromium (with system deps)`, `Chromium`, `Browser suite (pwsh shim)`, `Browser suite (sh shim)`); site test-plan.md:1027'
    basis: 'test-plan.md:1027'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Pipeline structure
    change: 'Coverage report row: the per-OS test job gates `coverage,doctest`'
    sidecar: '§9 Coverage report row: gate list `coverage,doctest`'
    rationale: 'report Changes → Harness / gate surface (`Gate verdict` runs `gate --require coverage,doctest`); site test-plan.md:1028'
    basis: 'test-plan.md:1028'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration
    change: 'tool-install paragraph: the runner-`jq` script list is G2, `orphans-check.sh` and `release-check.sh` (three scripts; `npm-audit.sh` removed); the Node v24.21.0 / `NODE_PIN_*` / Chromium sentence is removed; the pre-push `tools` stage checks `cc`, the `rust-toolchain.toml` channel and the `test`-job `tool:` pins only (no Node at `~/.local/viola-node`, no `node` detail)'
    sidecar: '§9 tool install: Node, Chromium and `npm-audit.sh` removed; pre-push checks no Node'
    rationale: 'report Changes → Files (`scripts/install-node.sh`, `scripts/npm-audit.sh` deleted), Harness / gate surface (workflow `env:` `NODE_PIN_*` removed), Symbols / APIs (pre-push `node_pin` gone); site test-plan.md:1036'
    basis: 'test-plan.md:1036'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Matrix builds
    change: 'drop "The Playwright browser suite runs on all three legs of the `test` job."'
    sidecar: '§9 Matrix: no browser suite leg'
    rationale: 'report Changes → Harness / gate surface; site test-plan.md:1041'
    basis: 'test-plan.md:1041'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Test report format
    change: '`junit-${{ matrix.os }}` uploads `target/nextest/ci/junit.xml` alone (one path; no `junit-playwright.xml`)'
    sidecar: '§9 Test report format: `junit-<os>` one path'
    rationale: 'report Changes → Harness / gate surface (`Upload JUnit` carries one path) and Counts (`junit-<os>` uploads `target/nextest/ci/junit.xml` alone); site test-plan.md:1048'
    basis: 'test-plan.md:1048'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Test report format
    change: 'remove the list item "`e2e-web/pw.json` and `pw-junit.xml`"'
    sidecar: '§9 Test report format: Playwright reports removed'
    rationale: 'report Changes → Files (`e2e-web/` deleted) and Schema / config (`.gitignore` `e2e-web/pw.json`, `pw-junit.xml` block removed); site test-plan.md:1052'
    basis: 'test-plan.md:1052'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Build failure conditions
    change: 'the first condition reads "Any unit / integration / E2E / contract / property test fails" (no browser), and the Lint errors condition drops the eslint-plugin-jsx-a11y / html-validate finding in the ubuntu a11y lint'
    sidecar: '§9 Build failure conditions: browser test and a11y lint removed'
    rationale: 'report Changes → Harness / gate surface (browser steps removed) and Files (`e2e-web/` deleted); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §10 Quality Gates & Coverage Targets
    change: 'Zero-flakiness budget: nextest `retries = 0` (the Playwright `retries: 0` clause removed); `#[ignore]` is not allowed as a parking place (no `test.skip`)'
    sidecar: '§10 Zero-flakiness: Playwright retries removed'
    rationale: 'report Changes → Files (`e2e-web/playwright.config.ts` deleted); site test-plan.md:1120'
    basis: 'test-plan.md:1120'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §11 Test Anti-Patterns → CI
    change: 'the cache ban reads "NEVER add a cache beyond Swatinem/rust-cache in `ci.yml`." (the npm / Playwright browsers sentence removed)'
    sidecar: '§11 CI: cache ban no longer names npm and browsers'
    rationale: 'report Changes → Harness / gate surface; site test-plan.md:1189'
    basis: 'test-plan.md:1189'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §11 Test Anti-Patterns → CI
    change: 'remove the ban "NEVER let `run --browser` or `viola-harness gate --require …playwright` pass …" (`--require playwright` is now a usage error)'
    sidecar: '§11 CI: the playwright-suite ban removed'
    rationale: 'report Changes → Symbols / APIs (`gate --require playwright` is usage exit 2 `unknown-suite`; `--browser` gone); site test-plan.md:1190'
    basis: 'test-plan.md:1190'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §11 Test Anti-Patterns → Universal (agent-driven specific)
    change: 'the recorder ban drops its "(use as headless runner only)" parenthetical, which names Playwright as a runner'
    sidecar: '§11 Universal: no Playwright runner parenthetical'
    rationale: 'report Changes → Dependencies (`@playwright/test` removed); duplicate occurrence of the browser-runner claim'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Pipeline structure
    change: 'Supply-chain row: `bash scripts/deny-probes.sh` last line `deny-probes: 12/12 banned, control clean`; the `Node (pinned)` + `scripts/npm-audit.sh` npm lockfile audit sentence and the weekly `npm-advisories` job are removed (weekly is `nightly.yml` job `advisories` alone)'
    sidecar: '§9 Supply-chain: npm audit and `npm-advisories` removed; deny probes 13 → 12'
    rationale: 'report Changes → Harness / gate surface (`supply-chain` loses `Node (pinned)` and `npm lockfile audit`; `nightly.yml` `npm-advisories` removed), Schema / config (`deny.toml` axum `http2` ban removed) and Counts (deny ban probes 13 → 12, measured `deny-probes: 12/12 banned, control clean`); site test-plan.md:1024'
    basis: 'test-plan.md:1024'
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy → Trigger → layer map
    change: 'Supply chain (V9) row: the Supply-chain stage lists cargo deny incl. the `fuzz/Cargo.lock` audit, the sole-root tokio ban, ban probes and zizmor (the `e2e-web` npm lockfile audit removed); weekly advisories cover the root and fuzz lockfiles only (no `npm-advisories`)'
    sidecar: '§2 V9 row: npm lockfile audit removed'
    rationale: 'report Changes → Harness / gate surface and Files (`scripts/npm-audit.sh` deleted); duplicate occurrence'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Build failure conditions
    change: 'Supply-chain errors: `scripts/deny-probes.sh` not ending `deny-probes: 12/12 banned, control clean`'
    sidecar: '§9 Build failure: deny probes 12/12'
    rationale: 'report Changes → Counts (deny ban probes 13 → 12, measured gate entry 14); site test-plan.md:1066'
    basis: 'test-plan.md:1066'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: 'test-runner-install: remove the `[test-groups] fixed-port` bullet and its `test(/default_port/)` override (no test holds `default_port`)'
    sidecar: 'Bootstrap test-runner-install: the nextest `fixed-port` group removed'
    rationale: 'report Changes → Schema / config (`.config/nextest.toml`: the `[test-groups] fixed-port` group and its override removed; no test held `default_port`)'
  - detector: D-tests-framework
    severity: warning
    section: §3 → Test data bootstrap
    change: 'Per-test isolation: remove the bullet on tests touching the default port 47319 in the `fixed-port` group'
    sidecar: 'Test data bootstrap: `fixed-port` isolation bullet removed'
    rationale: 'report Changes → Schema / config (`fixed-port` group removed); site reg:test-plan/test-data-bootstrap.md:16 (report Expected amendments)'
    basis: '.andromeda/registries/contracts/test-plan/test-data-bootstrap.md:16'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §5 Integration Test Strategy → Setup / teardown lifecycle
    change: 'the last sentence reads "Nothing is shared between tests." (the `fixed-port` group for port 47319 removed)'
    sidecar: '§5 lifecycle: no `fixed-port` exception'
    rationale: 'report Changes → Schema / config (`fixed-port` group removed); site test-plan.md:561'
    basis: 'test-plan.md:561'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §9 CI Integration → Pipeline structure
    change: 'Integration row Parallel column: "yes" (the `fixed-port` exception removed)'
    sidecar: '§9 Integration row: no `fixed-port` group'
    rationale: 'report Changes → Schema / config (`fixed-port` group removed); site test-plan.md:1026'
    basis: 'test-plan.md:1026'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §11 Test Anti-Patterns → CI
    change: 'the serialisation ban reads "NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home; the fuzz stages are single jobs, by design (§9)" — no `fixed-port` group named'
    sidecar: '§11 CI: serialisation ban names no `fixed-port` group'
    rationale: 'report Changes → Schema / config (`fixed-port` group removed); site test-plan.md:1188'
    basis: 'test-plan.md:1188'
    dependent-of: D-tests-framework
note: 'D-tests-coverage holds (the chunk adds no surface; its two narrowed paths carry unit tests `parse_require_refuses_playwright` and `secret_scan_ignores_the_removed_gui_forms` plus gate probes). D-tests-obs-harness is bilateral: the Log format and Status endpoint shape proposals must land with obs-plan §3''s copies (Log format JSON schema, Log file location `ui-<port>`) in the same apply, or the pair is one-sided. §1 line 138 quotes Founder Direction 3 verbatim ("run/hook/mcp/ui") and is left as a quote. Sites not stated by the report (§1, §2, §6, §7, §11 Universal, the 5-command and bootstrap key files) carry no line number.'
