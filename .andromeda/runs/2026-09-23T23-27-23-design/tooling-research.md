## Framework Families

These points apply to every family below. They all come from the project profile.
- **Trusted Types:** the CSP sets `require-trusted-types-for 'script'` with **no `trusted-types` allowlist**. That means any framework's named policy (`lit-html`, `svelte-trusted-html`, Vue's built-in one) can be created without amending the plan. If security-plan.md ever adds a `trusted-types` allowlist, those policy names must go on it.
- **Styles:** `style-src 'self'` with no nonce rules out anything that puts `<style>` tags or `style="…"` attributes into the page at runtime. That excludes CSS-in-JS such as Emotion and styled-components, and so MUI, Mantine's CSS-variable injector and PrimeVue's styled mode. Setting styles from JS through `el.style.x = …` (the CSSOM) is still allowed.
- **Fonts:** there is no `font-src`, so the page can only use system font stacks. No web fonts and no icon fonts.
- **Build tooling:** arch.md names no Node or JS toolchain, and CI runs only Rust steps. Any family that needs a compile step adds a JS build to CI, or requires committing built `/assets/*`. Whichever it is, the design orchestrator should flag it to the user as a cross-lane item for arch.
- **Page size:** the page is personal-scale. It shows session rows, a link set, budget readings with their age, skipped counts and a live SSE feed, with no pagination and no inputs in v1. The JS bundle is embedded via `include_bytes!` in every `cargo install`, so bundle size adds to binary size.

### Family: Web Components / Lit (build-optional)

**Frameworks:**
- Lit 3.3.3 — stable 3.x line (latest publish around May 2026, no Lit 4 announced). About 5–6 KB gzipped. It has a built-in `lit-html` Trusted Types policy, and text bindings are escaped by default. It can ship as one vendored ESM file with no build step.
- Vanilla TypeScript 7.0.2 + DOM APIs (`textContent`, `EventSource`), bundled with the esbuild 0.28.2 native binary — the smallest option and needs no Node. Every render path is hand-written, which is manageable at this page's size but gives no reactivity.
- Web Awesome 3.13.0 used directly as custom elements in plain HTML — a Lit-based library that can be the whole UI layer. Needs a small amount of glue JS for the SSE feed.

**Recommended CSS approaches:**
- Hand-written modern CSS (native nesting, `@layer`, custom properties) served as `/assets/app.css` — no tool at all, which matches the Rust-only build, and it is `style-src 'self'`-clean by construction.
- Lit 3.3.3 `static styles` (constructable stylesheets / `adoptedStyleSheets`) — scoped per component, with no `<style>` injection in modern browsers. Caveat: confirm in the founder's headless-browser check that the strict `style-src` does not block them.
- Pico CSS 2.1.1 (classless) — styles semantic `<table>`/`<article>` in light DOM with a system font stack. Useful if the page stays mostly plain HTML.

**Recommended component libraries:**
- Web Awesome 3.13.0 (Free core, MIT) — 50+ components, including `wa-badge`/`wa-tag` for liveness, idle/busy and wheel holder, `wa-progress-bar` for the `five_hour`/`seven_day` budget, `wa-relative-time` for the budget reading's age, and `wa-callout` for a pending dialog. Caveats: its default icon set loads from Font Awesome's CDN, which `connect-src`/`img-src 'self'` blocks, so a local icon library under `/assets` must be registered. Also pick a theme with a system font stack.
- Shoelace 2.x — the MIT predecessor of Web Awesome and now in maintenance. Only worth it if a frozen API is preferred over Web Awesome's still-changing 3.x line.
- No library: plain `<table>` plus custom `<viola-session-row>` / `<viola-event-feed>` Lit 3.3.3 elements. A v1 view with no inputs needs no form widgets.

**Recommended coherent stack:**
- Framework: Lit 3.3.3
- CSS: hand-written modern CSS plus Lit 3.3.3 `static styles`, using Web Awesome 3.13.0 design tokens
- Components: Web Awesome 3.13.0 Free (local icon library, system-font theme)
- Reasoning: Lit is the only family here that meets the security plan's Trusted Types and output-encoding rules without extra code, since the policy is built in and `${}` bindings are text by default (`unsafeHTML` is banned by lint/review). It can also ship as vendored ESM files embedded with `include_bytes!`, so the Rust-only CI across windows-2025/macOS/Linux needs no Node step. Web Awesome's badge, progress-bar and relative-time elements map directly onto the `/api/sessions` fields and budget reading. The phone view planned for later is the same page behind auth, and responsive web components carry over to it unchanged.

### Family: Svelte

**Frameworks:**
- Svelte 5.57.0 (runes), compiled with Vite 8.x (Rolldown) as a plain SPA. Since svelte@5.51.0 (Feb 2026, PR #16271) it wraps its internal `innerHTML` in a `svelte-trusted-html` policy. It compiles to small, direct DOM code with no virtual DOM.
- SvelteKit 2.70.3 (3.0 is at `next.25`) — adds routing and SSR adapters that are pointless here, because axum already serves `/`, `/assets/*` and `/api/*`. Plain Svelte is enough.

**Recommended CSS approaches:**
- Svelte 5.57.0 component-scoped `<style>` compiled into an external CSS file (`css: 'external'`) — scoped styles with nothing injected at runtime. Never use `css: 'injected'`, which the CSP blocks.
- Tailwind CSS 4.3.2 — the output is a static CSS file. It has a standalone CLI binary, but the Svelte compile step still needs a JS runtime.
- Caveat: static `style="…"` attributes in Svelte templates are parsed from a template string and blocked by the CSP. Use classes or `style:` directives, which go through the CSSOM.

**Recommended component libraries:**
- Bits UI 2.x (headless, Svelte 5 runes-native) — gives accessible building blocks for the v1.x brake confirmation dialog without shipping extra styling.
- shadcn-svelte 1.x (Bits UI 2.x + Tailwind 4, 50 components, copied into the repo) — you own the source, so every template can be checked for `{@html}` and inline `style`.

**Recommended coherent stack:**
- Framework: Svelte 5.57.0 (plain Vite 8.x SPA, no SvelteKit)
- CSS: Svelte scoped CSS (external) + Tailwind CSS 4.3.2
- Components: shadcn-svelte 1.x on Bits UI 2.x
- Reasoning: Svelte's compiled output keeps the embedded bundle small, and its Trusted Types support has shipped since 5.51, so the security plan's CSP holds as long as `{@html}` is banned. The cost is that it adds a Node + Vite 8.x compile to a CI that currently has only Rust steps, or requires committing built assets. That burden is disproportionate for a single view-only page used by one developer in v1.

### Family: React / Preact

**Frameworks:**
- Preact 10.29.7 + htm 3.1.1 — about 4 KB. htm uses tagged templates (no `eval`/`new Function`), so it runs with no build as vendored ESM. It creates DOM with `createElement` rather than `innerHTML`. Preact 11 is still an RC, so stay on 10.x.
- React 19.2.7 (19.3.0 released 2026-09-09) — react-dom does not write templates through `innerHTML`, so Trusted Types matter only if `dangerouslySetInnerHTML` is used. Needs a JSX build (Vite 8.x). Upgrade past 19.2.2 because of CVE-2025-55182. The runtime is about 45 KB gzipped, which is heavy to embed for one page.

**Recommended CSS approaches:**
- Tailwind CSS 4.3.2 via the standalone CLI binary (no Node) — static file, CSP-clean.
- CSS Modules via Vite 8.x — scoped classes extracted to a file.
- Exclude Emotion, styled-components and MUI, because they inject `<style>` at runtime and the CSP has no nonce.

**Recommended component libraries:**
- shadcn/ui via shadcn CLI 4.21.0 (Base UI 1.6 default since July 2026, Tailwind 4) — copied into the repo so it can be checked. Caveat: avoid components that render their own `<style>` element (for example Radix ScrollArea).
- Base UI 1.6.0 (headless, by the MUI team) — positions elements through the CSSOM, which the CSP allows. Useful for v1.x brake dialogs.

**Recommended coherent stack:**
- Framework: Preact 10.29.7 + htm 3.1.1 (no build)
- CSS: Tailwind CSS 4.3.2 (standalone CLI)
- Components: none in v1. Add Base UI 1.6.0 only if the page moves to React 19.2.7 for v1.x.
- Reasoning: Preact + htm keeps a JSX-like model with no Node step, so the Rust-only CI and `cargo install --path .` stay untouched. The Tailwind standalone binary can run as a CI step without Node. The React/shadcn ecosystem only pays off if the GUI grows well past a view-only page, and the current scale intent (personal v1, a two-route brake in v1.x) does not call for that.

### Family: Vue

**Frameworks:**
- Vue 3.5.41 — Trusted Types support built in since 3.5. Must use the runtime-only build with SFCs precompiled by Vite 8.x, because the in-browser template compiler uses `new Function` and the CSP blocks it.
- Vue 3.6 (RC, Vapor mode) — not stable yet, so avoid it for a release planned for winget/Homebrew/Scoop.
- petite-vue 0.4.1 — excluded, because it evaluates expressions with `new Function`.

**Recommended CSS approaches:**
- SFC `<style scoped>` extracted by Vite 8.x to a CSS file.
- Tailwind CSS 4.3.2.

**Recommended component libraries:**
- Reka UI 2.10.5 (headless, formerly Radix Vue) — no injected styling.
- PrimeVue 4.5.5, **unstyled mode only** — styled mode injects `<style>` at runtime, which `style-src 'self'` blocks.

**Recommended coherent stack:**
- Framework: Vue 3.5.41 (runtime-only, SFCs built with Vite 8.x)
- CSS: scoped SFC styles + Tailwind CSS 4.3.2
- Components: Reka UI 2.10.5
- Reasoning: Vue 3.5's automatic Trusted Types handling meets the CSP, and text interpolation meets the output-encoding rule once `v-html` is banned. Like Svelte, it requires a JS build in a Rust-only toolchain, and the runtime is heavier to embed than Lit or Svelte for one page.

### Family: Solid

**Frameworks:**
- Solid 1.9.x (stable) — fine-grained reactivity with a small runtime. Its compiled templates are cloned from `innerHTML` strings, and no built-in Trusted Types policy was found. The app would have to register a default policy, which weakens the security plan's Trusted Types guard.
- Solid 2.0 (RC since Aug 2026, new Rust/OXC compiler) — pre-stable.

**Recommended CSS approaches:**
- Tailwind CSS 4.3.2 — static file.
- CSS Modules via vite-plugin-solid 2.11.14 on Vite 8.x.

**Recommended component libraries:**
- Kobalte 0.13.x (headless) — pre-1.0.
- Corvu 0.x (headless primitives) — small component set.

**Recommended coherent stack:**
- Framework: Solid 1.9.x
- CSS: Tailwind CSS 4.3.2
- Components: Kobalte 0.13.x
- Reasoning: Solid runs fast, but this page is personal-scale and returns whole lists, so the speed buys nothing. It is the only family where the security plan's `require-trusted-types-for 'script'` needs a hand-written default policy, and it adds a JS build plus pre-1.0 component libraries. It is not recommended for this project.

## Overall Recommendation

**Recommended family:** Web Components / Lit (build-optional)
**Recommended stack:** Framework Lit 3.3.3 + CSS hand-written modern CSS with Lit 3.3.3 `static styles` and Web Awesome 3.13.0 tokens (no CSS tool) + Components Web Awesome 3.13.0 Free (MIT)

**Reasoning:** Lit is the only family that clears every frontend constraint in the security plan without extra work, which matters because the tier is Minimal but raised for GUI output encoding. Its `lit-html` Trusted Types policy satisfies `require-trusted-types-for 'script'`, `${}` bindings satisfy the `textContent`-only rule, and nothing is injected into `<style>` or `style="…"`. It is also the only family that fits arch's Rust-only build (axum 0.8.9 serving `include_bytes!` assets, CI with only Rust steps on windows-2025, macOS and Linux) without adding Node, since Lit and Web Awesome can be vendored as ESM files. Svelte 5.57 is the runner-up if the founder accepts a Vite 8.x build step. Web Awesome covers the view-only data contract closely: badges for liveness, status and wheel, a progress bar for the budget, relative-time for `read_at` age, and callouts for `dialog_pending`. The same page can later sit behind authentication for the phone view without changes.

Caveats for public distribution in v1.x:
- Register a local Web Awesome icon library. The default loads from a CDN, which the CSP blocks.
- Use a system-font theme, because there is no `font-src`.
- Web Awesome 3.x is still releasing often (3.12 → 3.13 within weeks), so pin its version.
- Confirm in the founder's default headless-browser verification that `adoptedStyleSheets` are not blocked by the strict `style-src`. Playwright can see into the open shadow roots.
- Ban `unsafeHTML`/`unsafeSVG` by lint or review.

**Research basis:** WebSearch/WebFetch, 2026-09-23:
- sveltejs/svelte PR #16271 (merged 2026-02-13, svelte@5.51.0, `svelte-trusted-html` policy)
- svelte.dev "What's new in Svelte: September 2026" (Svelte 5.57.0, SvelteKit 2.70.3 / 3.0.0-next.25)
- lit.dev release notes + npm `lit` (3.3.3; built-in `lit-html` policy)
- blog.vuejs.org "Announcing Vue 3.5" (Trusted Types) + endoflife/eosl (3.5.41, Aug 2026)
- Vue 3.6 Vapor RC articles (2026)
- react.dev/versions + newreleases.io (React 19.3.0 on 2026-09-09, 19.2.7)
- npm/openreplay "What's New in Preact for 2026" (10.29.7, 11.0.0-rc)
- solidjs.com "Solid 2.0 RC: The Big Reveal" (Aug 2026)
- tailwindcss.com standalone CLI + releases (4.3.2, 2026-06-29)
- ui.shadcn.com changelog "July 2026 – Base UI as the Default" (Base UI 1.6.0) and "March 2026 – shadcn/cli v4" + npm `shadcn` (4.21.0)
- npm @awesome.me/webawesome (3.13.0) + blog.fontawesome.com "Web Awesome Is Here" (MIT core)
- reka-ui releases (2.10.5), PrimeVue releases (4.5.5), shadcn-svelte docs (Svelte 5 / Tailwind 4), picocss.com v2 docs
- devblogs.microsoft.com "Announcing TypeScript 7.0" + microsoft/TypeScript releases (7.0.2)
- npm `esbuild` (0.28.2, Aug 2026)
- vite.dev "Vite 8.0 is out!" (stable 2026-03-12, Rolldown)
- npm `vite-plugin-solid` (2.11.14)

These points come from training data (June 2026) and were not confirmed by a 2026 search:
- Solid 1.9 has no built-in Trusted Types policy.
- `adoptedStyleSheets` behave under a strict `style-src`.
- Radix ScrollArea renders its own `<style>` element.
- Mantine and PrimeVue inject `<style>` at runtime.
- The version lines for Kobalte 0.13.x, Corvu 0.x, htm 3.1.1, Pico 2.1.1, Bits UI 2.x, shadcn-svelte 1.x and petite-vue 0.4.1.
