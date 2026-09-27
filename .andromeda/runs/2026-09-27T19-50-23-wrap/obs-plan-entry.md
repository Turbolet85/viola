
## 2026-09-27-browser-verdict-reachability — the browser suite in the test job, `junit-playwright.xml` in `junit-<os>`
**Section:** §9 (the artifact table's JUnit row; Step order and conditions, steps 1, 3 and 4)
**Change:**
- The JUnit artifact row names the browser suite's JUnit too: Playwright's `e2e-web/pw-junit.xml`, copied by the harness to `target/agent-run/artifacts/junit-playwright.xml`; `junit-<os>` carries both files (was nextest's alone).
- Step 1 includes the browser suite (`Node (pinned)`, `npm ci`, the Chromium install, `run --browser`, after the harness lifecycle); the G2 → G4 → capture → scan → uploads → gate order is unchanged.
- Step 3: the secret scan's `target/agent-run/*` scope covers `junit-playwright.xml`, which can hold a failing spec's output.
**Why:** the browser pipe joined each OS's `test` job (chunk 2026-09-27-browser-verdict-reachability); its JUnit rides the scan-gated upload.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/
