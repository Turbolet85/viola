# design extract

## Relevance
partial — the chunk renders only a static stub page, not the `viola ui` bay, so tokens, bay components and motion do not apply; what applies is the design plan's CI-render and font-resolution mandates, which the three-OS Playwright move and the WSL leg touch.

## Constraints
- The product surface this pipe will later verify is the `viola ui` page (Lit 3.3.3 vendored ESM, `/assets/app.css`, light DOM) per design-system §Surface: web-spa (Platform / Toolkit). The stub page is none of these. It must not seed `/assets/app.css`, a `viola-*` element or any partial bay markup (the Lit page is out of scope per scope.md §Boundaries).
- "Linux is the CI render": design-system §Surface: web-spa → Platform-Specific Notes (Fonts) and §Typography ("Assertions hold on the Linux fallback") require the headless GUI contrast and render assertions to be judged on ubuntu with DejaVu Sans Condensed + DejaVu Sans Mono resolved. Running Playwright on three CI OSes (founder ruling W125) must not make the Windows or macOS render an authority for those future assertions. The test-plan wrap amendment for the three-OS step should keep design render assertions ubuntu-judged, or else flag the design plan for a matching amendment.
- The CI image must provide the DejaVu families (Debian/Ubuntu `fonts-dejavu-core` for Sans Mono, `fonts-dejavu-extra` for the Condensed cut), and a missing family is a CI setup failure, not a design fallback, per design-system §Typography. This chunk's stub makes no render assertion, so it does not have to install them now. Whether the ubuntu CI E2E job and `scripts/wsl-provision.sh` already provide them is research's question. If this chunk adds them, they go in as CI pins (the WSL leg installs only CI's pins).
- No web fonts, `@font-face`, `local()` or CDN assets, per design-system §Typography (Loading) and §Anti-Patterns → Per-Surface Bans (web-spa). These bind the product page. The stub should follow them too so the pipe needs no network fetch and fixes no pattern the product forbids.
- Chromium only in this chunk. design-system §Surface: web-spa (Platform) names Chromium, Firefox and Safari 17.5+ as the supported engines, and §Platform-Specific Notes (`@starting-style` row) names their floors. Adding more engines is a later a11y/GUI-harness decision, not this chunk's.

## Patterns to follow
- Keep the stub page unstyled plain semantic HTML (one element for the trivially true assertion). An unstyled stub cannot violate the token test (design-system §Self-Validation Protocol → 4. Token Test) or add a ninth hex value.
- Keep the stub outside the embedded UI assets (`include_str!` / `include_bytes!` of `/assets/*`) per design-system §Surface: web-spa (Toolkit), so the real page's single-stylesheet rule (§Platform-Specific Notes, Constructable stylesheets) stays untouched.

## Anti-patterns to avoid
- The stub must not use inline `<script>`, `style="…"`, inline handlers, `innerHTML`, `@font-face` or CDN assets. This mirrors design-system §Anti-Patterns → Per-Surface Bans (web-spa, the CSP bullet). The pipe should not normalise a page shape the product CSP (`script-src 'self'`, `style-src 'self'`, `font-src 'none'`) rejects.
- Do not name any Universal-Bans face (Inter, Roboto, Arial, Helvetica, system-ui, `-apple-system`, …) in the stub or the Playwright config (for example as a `font-family` override). Per design-system §Anti-Patterns → Universal Bans.

## Contract bindings
- design ↔ tests: design-system §Typography / §Platform-Specific Notes ("Linux is the CI render", the resolved-family check, DejaVu in the CI image) binds to test-plan §Toolchain ("headless (ubuntu only)") and §3 `run` step 3 (`browser-linux-only`). The W125 three-OS change amends the test-plan side. The design side's ubuntu-render authority must survive that amendment, or be amended with it at wrap.
- design ↔ a11y: the future contrast assertions (design-system §Color Palette contrast rules, measured on the Linux fallback) bind to a11y SC 1.4.3, which runs on this pipe in Epoch 8. This chunk only makes that binding reachable. It asserts nothing.
- design ↔ security: the WSL/CI font packages, if added, are CI pins installed by `scripts/wsl-provision.sh` (security.md, the WSL distro installs only CI's pins), never host values.

## Acceptance criteria contributions
- (design) The stub page carries no inline style/script, `@font-face`, web font or CDN asset, and no hex colour or font-family outside the design tokens. An unstyled page passes. (per design-system §Anti-Patterns → Per-Surface Bans (web-spa) and §Self-Validation Protocol → 4. Token Test)
- (design) No product UI artifact is introduced: no `/assets/app.css`, no `viola-*` element, no Lit vendoring. (per design-system §Surface: web-spa (Toolkit / Framework))
- (design) The chunk's test-plan amendment for Playwright on three OSes states that future contrast and render assertions stay judged on the ubuntu leg with the DejaVu families resolved, or it records a paired design-system amendment. (per design-system §Typography "Assertions hold on the Linux fallback" and §Surface: web-spa → Platform-Specific Notes, Fonts)
