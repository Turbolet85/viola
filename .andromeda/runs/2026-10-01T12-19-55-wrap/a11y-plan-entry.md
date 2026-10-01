
## 2026-10-01-t12-19-55-wrap — a11y lint and component semantics: React + TypeScript
**Section:** §2 Strategy → the Robust row; §4 ARIA Patterns → the landmark row and the Session strip row; §4 → the web-spa tooling row; §9 CI Integration → Lint; §10 → the lint-stage budget line; §11 Anti-Patterns (the two role NEVERs); §3 → a11y testing tool pick; §3 → Bootstrap phases; §3 → Focus management test harness; §3 → Structured violation JSON schema
**Change:**
- The lint is eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y 5.1.1), over the JSX/TSX sources in `crates/viola-ui/`; its version is OPEN, owned by the route's frontend-toolchain entry. The `check_source` value is `eslint-jsx-a11y` (was `eslint-lit-a11y`).
- `<header>` is rendered by the `viola-atis` component (was "in light DOM by `<viola-atis>`").
- Session strip renders native `<tr>`/`<td>`; any element left between `<tbody>` and the `<tr>` has no role and `display: contents`; `data-*` state stays on the component's outermost rendered element.
- NEVER a row/table/cell role on a wrapper element of a `viola-*` component; NEVER a role set imperatively (`setAttribute` from an effect or ref) — roles belong in the JSX (was the Lit host / `connectedCallback` / `html` template form).
- The html-validate config declares any `viola-*` elements the rendered page carries; APG patterns are rendered by the React components; native focus is in the DOM they render.
**Why:** founder ruling of 2026-09-30, relayed by the overseer.
**Kept:** §1's Lit mentions (:130, :153), the verbatim upstream copy; every rendered-DOM assertion.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/
