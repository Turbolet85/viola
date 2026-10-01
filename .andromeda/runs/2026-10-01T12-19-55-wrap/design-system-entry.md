
## 2026-10-01-t12-19-55-wrap — web-spa toolkit: React + TypeScript, measured details open
**Section:** Iconography → Library; Surface: web-spa → Toolkit / Framework; Surface: web-spa → state attributes; Component Patterns → session strip Semantics; Component Patterns → event tape Expanded body; Platform-Specific Notes → CSP; Platform-Specific Notes → Constructable stylesheets; Anti-Patterns (the output-encoding NEVER)
**Change:**
- Toolkit was Lit 3.3.3, vendored ESM, no JS build step; now React + TypeScript, a built bundle embedded in `viola`. The bundler and its version, the React version, the embedding, the CSP the bundle needs and the npm gates are OPEN, owned by the route's frontend-toolchain entry (Epoch 8's head). Until it lands the page has no JS build step.
- The Lit-era host form (light-DOM custom elements via `createRenderRoot`, role-less `display: contents` hosts, Lit `static styles`) is retired; how each `viola-*` name maps onto React's output is OPEN, and the rendered-DOM requirements stand. Styling stays `/assets/app.css` alone: no Shadow DOM, no component-scoped or runtime-injected styles.
- Session strip: renders a native `<tr>` with `<td>` cells; any element left between `<tbody>` and the `<tr>` is role-less with `display: contents`; no role set by script.
- Text bindings are JSX text children (was Lit `${}`).
- CSP: `require-trusted-types-for 'script'` stands; "lit-html's built-in policy satisfies it" is retired, and what satisfies it under React is OPEN.
- Bans name React sinks: `dangerouslySetInnerHTML`, `innerHTML`, `style="…"` with React's `style` prop (were `unsafeHTML`, `unsafeSVG`, `styleMap`).
**Why:** founder ruling of 2026-09-30, relayed by the overseer: 0.2.0 renders the session hierarchy as a node graph, which a build-less Lit page does not fit. Standing rule: relaxing any CSP directive or a ban is a boundary widening the founder rules live at the frontend-toolchain entry.
**Kept:** every CSP directive and every ban (Lit API names swapped for their React sink counterparts, a narrowing); `viola-*` names and selectors as component names.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/
