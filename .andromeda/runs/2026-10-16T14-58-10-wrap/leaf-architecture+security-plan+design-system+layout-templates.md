group: architecture+security-plan+design-system+layout-templates
read: .claude/rules/frontend.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:3   class: tree
  leaf-text: «  - "crates/viola-ui/assets/**"»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:76 «└── viola-e2e/»
  replace-with: «  - "crates/viola-ui/**"»

- leaf: .claude/rules/frontend.md:9   class: stale
  leaf-text: «Path-scoped rules for viola's two design surfaces: the `viola ui` strip-bay page (web-spa) and human CLI output (cli). Authoritative sources: `.andromeda/design-system.md` and `.andromeda/layout-templates.md`; output encoding from `.andromeda/security-plan.md` §API Security.»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «Path-scoped rules for viola's one built design surface, human CLI output (cli); the web-spa frontend is deferred to 0.2.0, its technology is not chosen, and 0.1.0 renders no page. Authoritative sources: `.andromeda/design-system.md` and `.andromeda/layout-templates.md`.»

- leaf: .claude/rules/frontend.md:11   class: retired
  leaf-text: «## Framework (web-spa)»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «## Web-spa (deferred to 0.2.0)»

- leaf: .claude/rules/frontend.md:12   class: retired
  leaf-text: «**Framework:** React + TypeScript (founder ruling 2026-09-30), a built bundle embedded in `viola`; no component library. The bundler, versions, embedding, CSP needs and npm gates are OPEN, owned by the route's frontend-toolchain entry (Epoch 8's head). Until it lands: no JS build step, and no tsconfig, package manifest or bundler config under `crates/viola-ui/` (architecture Build system guard).»
  source: .andromeda/architecture.md:106 «the frontend (deferred to 0.2.0; its technology is not chosen)»
  replace-with: «**Frontend:** deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page. No tsconfig, package manifest or bundler config ever covers `crates/viola-ui/`; a CI-runnable guard lists any such file (architecture Build system guard).»

- leaf: .claude/rules/frontend.md:13   class: retired
  leaf-text: «**Component model:** plain `viola-*` components over native HTML — no Shadow DOM, no component-scoped or runtime-injected styles in v1. The Lit-era light-DOM custom-element form is retired; how each `viola-*` name maps onto React's output (element, class, data attribute) is OPEN, same owner. The rendered-DOM requirements stand.»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:14   class: retired
  leaf-text: «**Styling:** one hand-written `/assets/app.css` with `@layer tokens, base, components, states, motion`, native nesting, custom properties. State only in `data-*` attributes (`data-rb`, `data-dialog`, `data-liveness`, `data-wrapped`, `data-live`).»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:15   class: retired
  leaf-text: «**Routes:** one page `/`; loads `/api/sessions` + `/api/links`, then tails SSE `/api/events`. No client routes, no query-string state, no history entries.»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:17   class: retired
  leaf-text: «## Output encoding (CSP `default-src 'none'; script-src 'self'; style-src 'self'; … require-trusted-types-for 'script'`)»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:18   class: retired
  leaf-text: «Every event field and unwrapped session name is a text binding (JSX text children). NEVER `innerHTML`, `dangerouslySetInnerHTML`, `style="…"` (React's `style` prop included), inline `<script>`, inline handlers, `eval`, `@font-face`, CDN assets or Markdown rendering.»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:19   class: retired
  leaf-text: «Never display, echo or log the GUI token, launch URL, `?t=`, `.url` path or a cookie value — the 401 strip included — and never render `viola_home`.»
  source: .andromeda/security-plan.md:389 «Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:20   class: retired
  leaf-text: «`require-trusted-types-for 'script'` stands; lit-html's built-in policy no longer satisfies it, and what does under React is OPEN (frontend-toolchain entry). Relaxing any CSP directive is a boundary widening the founder rules live.»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:22   class: omission
  leaf-text: «## Tokens (the Token Test)»
  source: .andromeda/design-system.md:584 «A 0.2.0 frontend design input; 0.1.0 renders no page.»
  replace-with: «## Tokens (the Token Test; a 0.2.0 frontend design input, 0.1.0 renders no page)»

- leaf: .claude/rules/frontend.md:26   class: omission
  leaf-text: «the Linux DejaVu render is what CI checks.»
  source: .andromeda/design-system.md:271 «and the DejaVu families the CI image provides) stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text.»
  replace-with: «the Linux DejaVu render is what CI checks, until the route entry "Remove the browser leg" removes that browser leg.»

- leaf: .claude/rules/frontend.md:28   class: omission
  leaf-text: «## Motion (expression 0.3 on the page)»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «## Motion (expression 0.3 on the 0.2.0 page; 0.1.0 renders no page)»

- leaf: .claude/rules/frontend.md:29   class: stale
  leaf-text: «Exactly two motions: the cock (`translateX(--cock-offset)` over `--cock-dur`, into pending only) and the `--fade-dur` `@starting-style` fade on `data-live` tape lines/markers. Readback fill, strike, stale dimming, return into line, jumps and text swaps are instant. `prefers-reduced-motion` zeroes both durations; the offset stays.»
  source: .andromeda/design-system.md:41 «Exactly two motions, each played once on a state change. (1) The cocked-strip `translate` (12px / 160ms ease-out). (2) A new tape line or new transfer marker fades in (opacity, ≤120ms).»
  replace-with: «Exactly two motions, each played once on a state change: the cocked-strip `translate` (12px over `--cock-dur`, 160ms ease-out) and a new tape line or transfer marker fading in (opacity over `--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and the return of a strip into line are instant; no hover effects except link underline and focus rings.»

- leaf: .claude/rules/frontend.md:30   class: retired
  leaf-text: «Banned: spinners, pulses, blink, shimmer, skeletons, looped/staggered animation, hover colour transitions, "Loading…", toasts, auto-dismissing notices, command palette, shortcuts.»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line)»

- leaf: .claude/rules/frontend.md:33   class: retired
  leaf-text: «`viola-session-row` strip (six fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot by name, `unknown` for a missing reading, `n/a` absent-by-contract, never blank); `viola-readback` (the signature: `open` / `read` / `refused` / `unconfirmable`, identical drawing in tape and marker); `viola-transfer`; `viola-event-feed` (native `<details>` lines, 2000-line cap, follow only at bottom); `viola-atis`.»
  source: .andromeda/layout-templates.md:310 «The six captions, words and slot order are fixed, and a future page reuses them.»
  replace-with: «The web components are 0.2.0 frontend design; 0.1.0 renders no page. The six captions NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, their words and slot order are fixed, and a future page reuses them; the readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`.»

- leaf: .claude/rules/frontend.md:38   class: retired
  leaf-text: «errors, the `viola ui` launch block.»
  source: .andromeda/design-system.md:418 «each refusal's `hint:` line directly after it, and errors (`error: …`).»
  replace-with: «errors.»
