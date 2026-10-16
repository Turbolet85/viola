
## 2026-10-16-remove-the-browser-leg — the Browser e2e row and the leg's occupied resources leave
**Section:** §Stack and Technologies (CI/CD, Browser e2e, Code quality rows); §Occupied Resources (CI workflow data lines, Test-side install sites, `target/nextest/ci/junit.xml`, `target/npm-audit/audit.json`, the `e2e-web/` outputs, `target/run-archive/<n>/`)
**Change:**
- The Browser e2e (test-side only) row is gone: Node v24.21.0 with its `NODE_PIN_*` pin and `scripts/install-node.sh`, `@playwright/test` 1.63.0 and its Chromium, `e2e-web/` with `npm ci` and `scripts/npm-audit.sh`, `run --browser` and the pre-push browser stage. The stack names no test-side Node, npm or browser tool.
- CI/CD row: the pre-push runs every child through `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, and its `tools` stage checks `cc`, the channel and ci.yml's `test`-job pins (was also `NODE_PIN_VERSION`). Code quality row: `jq`'s consumers are G2, `scripts/release-check.sh` and `scripts/orphans-check.sh`.
- Occupied Resources: the `NODE_PIN_*` workflow data lines, the Node and Playwright install sites (`$RUNNER_TEMP/node`, `~/.local/viola-node/`, `~/.cache/ms-playwright/`), `target/npm-audit/` and the `e2e-web/` outputs leave the registry. `junit-<os>` carries `target/nextest/ci/junit.xml` alone; the run archive holds `junit-{nextest-unit,nextest-integration,coverage}.xml` and `outcomes.json`.
**Why:** This chunk removed the browser test leg (founder ruling 2026-10-16, item 3), as the row's own stand-until clause asked of this entry's wrap. No resource was added.
**Kept:** The npm-shim → `claude.exe` resolution text, the product's own reading of the CLI's npm shim.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/
