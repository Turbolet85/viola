# tests extract

## Relevance
relevant: the chunk builds the browser test runner, turns on a harness `run` selector, and adds a `gate`-read suite, a CI job and a pre-push leg. The folded 10 s wait recorder is test-support work under the mutation gate.

## Constraints
- **Browser step shape (per test-plan §3 `run` step 3).** The step is `npm ci --prefix e2e-web`, then `npx --prefix e2e-web playwright test`. A missing Chromium or a failed `npm ci` is a failure (`suite:"playwright"`, `failed` ≥ 1, `reason:"browser-missing"`), never a skip. The same step's OS clause conflicts with the entry's "three CI OSes": on Windows and macOS, `--all` omits the step and `--browser` exits 2 with `browser-linux-only`. Other clauses carry the same ubuntu-only premise: §6 Drivers table (web-spa "headless (ubuntu only)"), §9 E2E row, §9 Matrix builds, §12 Decisions Log (web-spa driver and Browser caching rows), §11 Test Strategy and §2 Scope (line ~396, "The headless-browser suite runs on ubuntu"). The founder ruling outranks them, and the fix is a wrap amendment to each clause, not a phase edit. Two things are research's question: which OS arms `run --browser` has today, and whether it is still a usage error. §3 Exit codes requires that "a flag or selector whose surface is not built yet … is a usage error until its chunk lands, never a vacuous pass".
- **Suite and gate contract (per test-plan §3 `run` Output format and §3 `gate`).**
  - The `playwright` suite in `run-summary.json` is built from `e2e-web/pw.json`.
  - `e2e-web/pw-junit.xml` is copied as `junit-playwright.xml`.
  - `gate --require playwright` breaches when the suite is absent, when `failed > 0`, or when `skipped > 0` ("skips are never allowed for `playwright`").
  - `skipped` counts Playwright `test.skip` / `test.fixme`.
  - §3 `gate` Inputs lets only a fresh file count, never an earlier run's (source deleted before the invocation, missing after it = `artifact-missing`). Whether `run` applies that stale-file rule to `pw.json` / `pw-junit.xml` is research's question.
- **Node side pins and configuration (per test-plan §3 Bootstrap phases `test-runner-install`, Node side).**
  - `e2e-web/package.json` pins `@playwright/test@1.63.0`. The plan pins `@axe-core/playwright@4.13.0` in the same line, so leaving axe to Epoch 8 is a deviation to record.
  - `playwright.config.ts` holds: `use: { headless: true }`, a single `chromium` project, `retries: 0`, `forbidOnly: true`, no `webServer`, `globalSetup` limited to boot step 1, and `reporter: [['json',{outputFile:'pw.json'}],['junit',{outputFile:'pw-junit.xml'}]]`.
- **CI job rules (per test-plan §9 Pipeline structure (E2E and Coverage report rows), §9 tool-install paragraph, §12 Browser caching, and §3 `gate`).**
  - Use the runner-image Node, with no `setup-node`.
  - No npm or Playwright browser cache. rust-cache is the only cache allowed.
  - Install Chromium with `npx --prefix e2e-web playwright install --with-deps chromium` (§3 Bootstrap `ci-tool-install`).
  - The job's last step is `viola-harness gate --require` set to exactly the suites that job ran (`playwright`).
  - Every `uses:` is SHA-pinned, with `permissions: {}` at the top level and `contents: read` per job.
  - The `pw.json` and `pw-junit.xml` artifacts are uploaded (§9 Test report format). `--with-deps` is an apt step, so it has no counterpart on the windows and macOS runners. What each non-ubuntu leg installs is P3/P4's question.
- **WSL pre-push leg (per test-plan §9 tool-install paragraph and §3 Internal harness subcommands `pre-push`).**
  - `scripts/wsl-provision.sh` installs only the pins it parses from `ci.yml`, so ci.yml stays the one version source.
  - Every WSL call is `env -i`, so no host value crosses.
  - A browser step inside `pre-push` changes closed enums: a new `stage` value, and new `tools` `detail` values (Node, Playwright or Chromium), or a new `tool-pin-mismatch` subject. Each needs a Decisions Log entry (§3 Closed enums: "A new value needs a Decisions Log entry").
  - Whether the browser step joins the `linux-tests` stage (today `run --coverage` then `gate --require coverage,doctest`) or gets its own stage is P4's call.
- **The folded 10 s waits (per test-plan §3 Bootstrap `test-runner-install` `[profile.mutants]`).** The kill is `slow-timeout = { period = "5s", terminate-after = 2 }` = 10 s, and "every in-test wait a root-package mutant can reach, and the 10 s kill, stay below 20 s". §3 `run` step 2 (the `booted_wrapper` wait, "bounded at 10 s") states the current bound as plan text, so lowering it to 7 s is a wrap amendment to that clause. §10 Zero-flakiness budget applies to the watched test (`retries = 0`; a red stays open until its root cause is fixed).
- **Naming (per test-plan §2 Test organisation, File naming and Test function naming).** Specs are named `<bay-layout-type>.spec.ts` and titles `test('<layout type>: <expected state>')`. A stub spec has no bay layout type, so P4 must choose a stub name that no Epoch 8 spec will take, and record the name as a deviation.

## Patterns to follow
- **Harness output (per test-plan §3 Exit codes and `run` Output format).** Every refusal is a typed arm in the one-JSON-document-plus-exit-code discipline: exit 0 / 1 / 2, `reason` from a closed enum, suites merged by `suite` into `run-summary.json`. `browser-missing` is a failure arm (exit 1), not a tool-missing refusal.
- **Test selection (per test-plan §3 `run` Test selection).** One browser test is `npx --prefix e2e-web playwright test --grep "<title text>"`. The stub's title must be selectable this way.
- **Output location (per test-plan §2 Agent-runnable invariants).** Playwright output is machine-parseable and lands only in `pw.json` / `pw-junit.xml`. It runs only in headless test-runner mode.
- **The 10 s fold (per test-plan §3 `run` step 2).** The fold follows the root `booted_wrapper` exit-aware wait: it fails at once when `child.try_wait()` reports that the wrapper exited first, and the bound stays under the mutants kill. The recorder only adds evidence; it changes no assertion.

## Anti-patterns to avoid
- **Serving the stub page.** Two of the scope's three options collide with plan clauses:
  - §3 Internal harness subcommands (Browser-side controls): "`route.fulfill` and `route.abort` stay banned".
  - §8 Mocking libraries (Fake servers row): "no HTTP fake servers".
  - §11 Universal: "The only socket allowed is 127.0.0.1 `viola ui`".

  A Playwright `route` fulfilment and a fixture server each break one of these. A `file://` page breaks none of them, and it keeps the "no `webServer`" shape (§6 Drivers table) for Epoch 8.
- **Faking a green (per test-plan §11 CI, §11 Quality and §10 Zero-flakiness budget).**
  - NEVER let `run --all`, `run --browser` or `gate` pass on ubuntu with `playwright` missing or skipped.
  - NEVER `test.skip` a spec per OS.
  - NEVER add a retry budget or cache npm or browsers.
  - Never raise a wait bound past the mutants kill to make a watch go quiet.
- **Browser-test bans (per test-plan §11 E2E and §11 Universal).** No `bypassCSP`, no `toHaveScreenshot`, no codegen or UI mode, no `sleep`-based synchronisation, no test-owned timer verdict.

## Contract bindings
- **tests ↔ security.**
  - The committed `e2e-web` npm lockfile sits outside `cargo deny`'s graph. Whether an npm audit/sources gate joins the `supply-chain` job is security-plan's CI rows, not test-plan's.
  - The WSL provisioning follows security.md: CI's pins only, `env -i`.
  - Any `ci.yml` change is judged by zizmor (test-plan §9 Supply-chain row, §11 CI SHA-pin ban).
- **tests ↔ obs.**
  - The browser artifacts upload next to obs-plan §9's scan-gated uploads (test-plan §9 Test report format).
  - Whether `pw.json` / `pw-junit.xml` on the new non-ubuntu legs go behind the per-OS secret scan is P3/P4's question.
- **tests ↔ a11y (Epoch 8 builds on this pipe).**
  - The a11y lint runs in the browser job after `npm ci --prefix e2e-web` and before `run --browser` (test-plan §9 E2E row, Decisions Log Z6).
  - `a11y` is not a `gate` suite value (§3 `gate`: `unknown-suite`).
  - The pipe must leave room for that step ordering and for the deferred `@axe-core/playwright` pin.
- **tests ↔ arch (code graph).** The tracked `e2e-web/tsconfig.json` is the TypeScript plane's marker (CLAUDE.md Key directories). The refresh and health check 11 behaviour is arch/tooling's, and P3 reads it.

## Acceptance criteria contributions
- On a Linux leg (CI ubuntu and the WSL `pre-push`), `scripts/agent-run.sh run --browser` exits 0 and `run-summary.json` holds `{"suite":"playwright","passed":≥1,"failed":0,"skipped":0}`. Then `viola-harness gate --require playwright` exits 0 with `"breaches":[]`, and `junit-playwright.xml` is present in `target/agent-run/artifacts/` (per test-plan §3 `run` step 3, §3 `run` Output format, §3 `gate`).
- Negative arms, each asserted by a harness test:
  - With Chromium absent or `npm ci` failing, `run --browser` exits 1 with `suite:"playwright"`, `failed` ≥ 1 and `reason:"browser-missing"`.
  - A skipped stub makes `gate --require playwright` report `{"gate":"suite-skipped","suite":"playwright"}`.
  - A missing `playwright` suite reports `suite-missing`.

  (per test-plan §3 `run` step 3, §3 `gate`, §11 CI)
- `e2e-web/playwright.config.ts` holds `retries: 0`, `forbidOnly: true`, `headless: true`, a single `chromium` project, no `webServer`, and the `pw.json` + `pw-junit.xml` reporters. Every CI browser job:
  - has no npm or browser cache and no `setup-node`;
  - has SHA-pinned `uses:`;
  - ends with `gate --require playwright`.

  (per test-plan §3 Bootstrap phases `test-runner-install`, §9 Pipeline structure, §12 Browser caching)
- The 8 folded waits read 7 s, below the 10 s `mutants` kill, and none is raised. Their reports go to a known file that a passing test removes. The chunk's mutation verdict is green: `missed == 0`, `timeout == 0`, `unviable <= caught` (per test-plan §3 Bootstrap phases `test-runner-install` `[profile.mutants]`, §10 Mutation gate).
