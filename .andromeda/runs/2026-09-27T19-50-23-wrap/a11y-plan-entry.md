
## 2026-09-27-browser-verdict-reachability — the browser pipe on three OSes, the a11y verdict ubuntu-judged, axe pin deferred
**Section:** §3 (CI integration Runner; the a11y harness Command; Bootstrap phases `a11y-tooling-install`) · §9 (the layer table's Unit/integration and E2E rows; Pipeline integration; the ubuntu-only sentence) · §11 (CI anti-pattern) · §12 (D-A11Y-12)
**Change:**
- `run --browser` runs the locked Playwright CLI (`node node_modules/@playwright/test/cli.js test` after `npm ci`; was `npx --prefix e2e-web playwright test`) on all three legs of the `test` job (was the ubuntu leg only, `browser-linux-only` elsewhere); the a11y verdict stays judged on the ubuntu leg only. `browser-missing` covers a failed `npm ci` as well as a missing Chromium.
- The test job's gate is `coverage,doctest,playwright` (was `coverage,doctest` / `playwright`).
- `a11y-tooling-install` adds `@axe-core/playwright@4.13.0` itself (was "already declared by tests"): tests declared only `@playwright/test@1.63.0`.
- §11 bans judging the a11y verdict on Windows or macOS (was: running `--browser` there). D-A11Y-12's reason is the ubuntu-judged verdict, not an ubuntu-only `--browser`.
**Why:** founder ruling W125 put the browser pipe on all three CI OSes before the a11y harness lands on it (chunk 2026-09-27-browser-verdict-reachability); the axe pin was deferred to Epoch 8.
**Kept:** §1 (the verbatim a11y-scope copy) and the §12 key-decision history ("already declared") stand as written.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/
