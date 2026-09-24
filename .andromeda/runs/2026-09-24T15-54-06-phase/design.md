# design extract

## Relevance
Partial. This chunk renders no UI and changes no tokens. The design plan applies only where the chunk names web-spa artifacts in the architecture tree (`crates/viola-ui/assets/`, `e2e-web/`), where it decides the TypeScript plane, and where it wires CI that will later host the headless GUI checks.

## Constraints
- design-system §Surface: web-spa → Toolkit / Framework requires Lit 3.3.3 as vendored ESM, embedded via `include_bytes!`, with no JS build step. The TS-plane decision (scope item 6) must not add a TS or JS build step, a `tsconfig.json` or a bundler that covers `crates/viola-ui/assets/`. TypeScript stays test-side, in `e2e-web/`.
- design-system §Surface: web-spa → Tokens and §Platform-Specific Notes → Constructable stylesheets require `/assets/app.css`, served via `include_str!`, to be the single stylesheet in v1. The architecture tree entry for `crates/viola-ui/assets/` should name `index.html` and `app.css`, plus the vendored Lit ESM, as that directory's contents. Whether architecture.md's tree already describes the assets directory this way is research's question.
- design-system §Typography → Loading, §Iconography → Library and §Platform-Specific Notes → CSP require no web fonts, no `@font-face`, no icon font or CDN icon set, and no inline SVG. The tree must not add font, icon or generated-asset directories under `crates/viola-ui/assets/`.
- design-system §Typography → "Assertions hold on the Linux fallback" and §Platform-Specific Notes → Fonts ("Linux is the CI render") require the CI image that runs the headless GUI checks to provide the DejaVu families (`fonts-dejavu-core` and `fonts-dejavu-extra`). A missing family counts as a CI setup failure. That job arrives with `e2e-web/` in a later chunk. This chunk should not wire CI in a way that rules out an ubuntu headless job with those packages.
- design-system §Anti-Patterns → Per-Surface Bans (web-spa) bans CDN assets. If the release build (scope item 4) embeds viola-ui assets, it embeds only files committed to the repo and fetches nothing at build time. This applies only if viola-ui is already a workspace member at HEAD, which is research's question.

## Patterns to follow
- Assets embedded in the binary with `include_bytes!` / `include_str!` (per design-system §Surface: web-spa → Toolkit / Framework and the header comment of the §Tokens code block). The release-build check should expect assets to be compiled into the binary, not shipped as loose files next to it.
- Headless-browser verification is the default for the web page (per design-system §Surface: web-spa → Platform). This supports placing the TS plane in `e2e-web/` (Playwright specs and fixtures) as test tooling, not product code.
- A single `@layer tokens, base, components, states, motion` stylesheet with no preprocessor (per design-system §Surface: web-spa → Tokens). Record `app.css` as hand-written source in the tree, not as build output.

## Anti-patterns to avoid
- Adding a JS/TS build step, an npm-built bundle or a `tsconfig.json` that covers viola-ui page sources (per design-system §Surface: web-spa → Toolkit / Framework, "no JS build step").
- Listing or wiring any fetched font, icon or CDN asset for the page (per design-system §Anti-Patterns → Per-Surface Bans, web-spa: "NEVER use … `@font-face`, CDN assets …").

## Contract bindings
- design ↔ tests: the headless GUI checks run on ubuntu, and they render and assert the DejaVu fallback stacks, including the resolved family (design-system §Typography → Assertions hold on the Linux fallback). This ties to the test plan's `e2e-web/` placement, which is the TS plane this chunk decides.
- design ↔ a11y: `crates/viola-ui/assets/index.html` and `e2e-web/*` are the a11y artifacts the chunk adds to the tree. Design's semantic anatomy of the page (§Design Decisions Log, overseer fix passes 2 and 3: Y1 and Z9) lives in those files, but the a11y plan owns their listing.
- design ↔ architecture: the §Project directory structure tree entry for `crates/viola-ui/assets/` has to match the web-spa toolkit facts above (Lit ESM vendored and embedded, a single `app.css`, no build output).

## Acceptance criteria contributions
- (design) The architecture tree's `crates/viola-ui/assets/` entry names only committed, hand-written or vendored files (`index.html`, `app.css`, vendored Lit ESM), with no font, icon or build-output directory (per design-system §Surface: web-spa → Toolkit / Framework and §Iconography → Library).
- (design) The recorded TS-plane decision limits TypeScript to `e2e-web/` and introduces no JS/TS build step or `tsconfig.json` covering viola-ui page sources (per design-system §Surface: web-spa → Toolkit / Framework).
- (design) If this chunk records CI placement for the future headless GUI job, the record says it runs on ubuntu with `fonts-dejavu-core` and `fonts-dejavu-extra` installed (per design-system §Typography → Assertions hold on the Linux fallback).

## Relevant amendment history
(none). The sidecar `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist, which is normal on a fresh project. One entry in the plan's own Design Decisions Log is nearby: "Phase 4.5 review round 1" (2026-09-24) added per-OS font fallback stacks because CI's headless GUI checks run on ubuntu. That is where the DejaVu CI-image requirement above comes from.
