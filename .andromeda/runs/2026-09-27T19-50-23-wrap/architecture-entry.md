
## 2026-09-27-browser-verdict-reachability — the browser pipe: Node pin, npm audit, install sites, root-watch reports
**Section:** §Stack and Technologies (CI/CD row; new Browser e2e row; Code quality `jq`) · §Occupied Resources (Environment variables; Filesystem; Repository) · §Infrastructure Patterns → Project directory structure · §Infrastructure Patterns → CI/CD approach
**Change:**
- A test-side Browser e2e row: Node v24.21.0 (the official build, sha256-pinned by ci.yml's `NODE_PIN_*`, `scripts/install-node.sh`; never runner-image Node or a setup action) + `@playwright/test` 1.63.0 (3 locked packages) and its Chromium; `npm ci` and the Playwright CLI spawned directly; axe lands with the a11y chunks.
- WSL provisioning adds the pinned Node and Chromium; Chromium's system libraries are an operator-only root install (`--install-deps`).
- Registered: the `NODE_PIN_*` workflow data lines (never read from the environment by viola or viola-harness); `<temp dir>/viola-root-watch/`; the install sites `$RUNNER_TEMP/node`, `~/.local/viola-node/`, `~/.cache/viola-provision/e2e-web/`, `~/.cache/ms-playwright/`; `target/npm-audit/audit.json`; `e2e-web/node_modules/`, `pw.json`, `pw-junit.xml`, `test-results/` (written now, was "land with the Web UI chunks"); `junit-<os>` carries 2 paths; run-archive holds the playwright JUnit.
- Tree: `scripts/install-node.sh`, `scripts/npm-audit.sh`, `e2e-web/package-lock.json`, `stub/pipe.html`; `package.json` pins `@playwright/test` only (was also `@axe-core/playwright`); workflow comments.
- CI/CD: `nightly.yml` runs three jobs (was two; + `npm-advisories`); Node via `install-node.sh`; the `test` job runs the browser suite and gates `coverage,doctest,playwright` (was `coverage,doctest`); `supply-chain` adds the npm lockfile audit; pre-push's Linux leg checks `node` and runs `run --browser` + the playwright gate.
**Why:** founder ruling W125 (the pipe proven before any feature needs it); P4 operator forks 1–3; the root launch ratified live by the overseer, operator-only.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/
