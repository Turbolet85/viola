### Screen reader test pattern

- **Automated proxy (gating):** @guidepup/virtual-screen-reader 0.33.0 runs in the same Playwright Chromium.
  - **Loading:** `page.route('**/assets/__a11y/vsr.js', …)` fulfils the package's `browser.js`, and `page.evaluate(() => import('/assets/__a11y/vsr.js'))` loads it from a same-origin path, which satisfies `script-src 'self'`.
  - **Run:** `virtual.start({ container: document.body })`, trigger the state, then assert `spokenPhraseLog()`.
  - **Checks:**
    - the four design/overseer announcements plus the post-render rack error strips (Section 7) appear once each;
    - appending tape lines produces zero live phrases (`log-announced`);
    - the reading order follows the four regions.
  - **CSP guard:** the shared fixture in `e2e-web/fixtures/a11y.ts` records `securitypolicyviolation` events and Trusted Types console errors in every `@a11y` test, not only the VSR specs. It fails the test at teardown with one row per event: `violation_type:"csp-console"`, `severity:"serious"`, the `check_source` of the check running when the event fired, and a `selector` naming the violated directive, never the blocked URI (it can carry `?t=`). This is the runtime evidence for the Section 1 `[resolved: …]` claim that axe injection reaches no Trusted Types sink, and it feeds the Section 10 CSP / Trusted Types failure condition. The first VSR spec's clean-console assertion remains the D-A11Y-11 check on the VSR import.
  - **Fallback:** if TT rejects the import, a `page.addInitScript` MutationObserver recorder scoped to `[role=status],[role=log],[aria-live]` replaces it (Decisions Log).
  - **DOM assertions alongside the proxy:** `role="status"` `toHaveText(...)` per trigger, and `toHaveAccessibleName` per readback word cell.
- **Per-surface test spec (supplemental, founder-owned):**
  - web-spa: NVDA 2026.2 + Edge/Chrome (Windows, live target); VoiceOver + Safari (macOS); Orca (GNOME 50 cycle) + Firefox/Chromium (Linux).
  - cli: NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal.
  - tui: NVDA + Windows Terminal. The wrapped `claude` TUI must read the same as unwrapped (Must-Work O1).
- **Manual pass spec format:** one JSON row per step in `a11y/sr-pass/<date>-<at>.json`, shaped like ARIA-AT rows. Example: `{path:"P2", step, at:"nvda-2026.2", browser:"edge", surface:"web-spa", expected:"send to builder unable, not-delivered, …", heard, result:"PASS"|"FAIL"}`. Record the Orca version per pass.
- **Supplemental to automated:** the SR pass is NEVER the sole verification. Axe and html-validate cover SC 4.1.2 programmatically, and VSR plus the status-text assertions cover SC 4.1.3. The SR pass verifies real-AT announcement quality. `@guidepup/guidepup` 0.34.0 + `@guidepup/playwright` 0.19.1 (real NVDA/VoiceOver) are optional and founder-local only, not in the CI gate (see the Decisions Log).
- **Real-AT automation stays out of CI:** adding `@guidepup/guidepup` / `@guidepup/playwright` to CI requires a tests-harness change, because the a11y verdict is judged on the ubuntu leg only (the browser suite itself runs on all three OS legs) and Guidepup has no Orca support.
