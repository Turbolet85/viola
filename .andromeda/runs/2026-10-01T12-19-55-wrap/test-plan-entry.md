
## 2026-10-01-t12-19-55-wrap — web page entity and a11y lint: React + TypeScript
**Section:** §1 Test Scope Summary → the web page entity; §6 E2E Test Strategy → Playwright locator conventions; §9 CI Integration → E2E row (the a11y lint); §9 → Lint errors; §3 → 5-command implementation (Embedded-asset freshness)
**Change:**
- The web page entity was Lit 3.3.3 light-DOM `viola-*` elements; now React + TypeScript `viola-*` components in a built bundle, its toolchain OPEN (owned by the route's frontend-toolchain entry).
- The a11y lint runs eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y 5.1.1) over the `crates/viola-ui/` frontend sources; its version and source glob are OPEN.
- Component names scope text matches; the selector form of those names and of the `viola-*[data-*]` selectors follows the component mapping, OPEN.
- Embedded-asset freshness: no JS build step until the frontend-toolchain entry; how the bundle build joins step 1 is OPEN, owned by that entry.
**Why:** founder ruling of 2026-09-30, relayed by the overseer.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/
