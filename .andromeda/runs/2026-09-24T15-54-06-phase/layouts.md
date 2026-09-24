# layouts extract

## Relevance
Partial, and only at the edges. This chunk builds no surface, region, component, focus stop or breakpoint. The layouts plan touches it in two places. First, the architecture tree now names `crates/viola-ui/assets/`. Second, the TypeScript-plane decision and the dependency-policy wording must agree with the tooling context the layouts plan states for each surface.

## Constraints
- The web-spa page is built from vendored Lit 3.3.3 ESM, embedded via `include_bytes!`, with no JS build step. There is a single hand-written stylesheet, `/assets/app.css`, and no component library (per layout-templates §Surface: web-spa → Tooling context). The `crates/viola-ui/assets/` entry in the architecture tree therefore needs to cover `index.html`, `app.css` and the vendored Lit ESM. It should not add any bundler output, `package.json` or `tsconfig.json` under viola-ui. Whether the architecture tree already lists these files is research's question.
- The page's own sources are plain light-DOM `viola-*` elements with no TS compile step (per layout-templates §Surface: web-spa → Tooling context). So the TypeScript plane decision should only ever cover `e2e-web/`, never the viola-ui page.
- The cli surface uses clap 4.6.7 (derive), std `IsTerminal` and windows-sys 0.61.2 `SetConsoleMode`. Styling is a small hand-written SGR module in the `viola` bin, with no colour or table crate (per layout-templates §Surface: cli → Tooling context). If this chunk rewords the architecture's dependency policy, that wording should not admit a colour or table crate for the `viola` bin. The architecture plan is the authority on dependencies; whether its current policy already excludes them is research's question.
- The `viola` bin is the only human-facing cli surface. `viola hook` / `viola mcp` have no human surface, and `viola run` prints nothing (per layout-templates §Surface: cli → Component — Header / banner). A release build that drops test-only binaries (`viola-fake-agent`, `viola-harness`) takes nothing away from any laid-out surface. The `viola` bin itself must stay in the release output.

## Patterns to follow
- Treat the viola-ui asset set as fixed and build-free: one route `/`, one stylesheet, vendored ESM (per layout-templates §Surface: web-spa → Tooling context and §IA notes → Global model). List it in the tree as-is and don't introduce a build pipeline.
- The page's creation is deferred to the Web UI chunks (per layout-templates §Decisions Log → Notable surface-specific deferrals). This chunk only names the paths, which matches the scope's out-of-scope boundary.

## Anti-patterns to avoid
- Adding a TS or JS build step, a `tsconfig.json`, or a bundler config under `crates/viola-ui/` to justify a TypeScript plane. The layouts plan fixes the page as "no JS build step" (per layout-templates §Surface: web-spa → Tooling context).
- Adding a colour or table crate to the cli dependency set (per layout-templates §Surface: cli → Tooling context).

## Contract bindings
- layouts ↔ arch: the architecture's §Project directory structure entry for `crates/viola-ui/assets/` and its dependency policy should agree with the layouts tooling contexts for web-spa and cli.
- layouts ↔ tests/a11y: `e2e-web/`, with `playwright.config.ts`, `fixtures/a11y.ts` and `tests/*.spec.ts`, is the only home for TypeScript. The page it drives is the build-free Lit bay (per layout-templates §Surface: web-spa → Tooling context).

## Acceptance criteria contributions
- The architecture tree's `crates/viola-ui/assets/` entry names `index.html` and `app.css` as the single stylesheet, plus the vendored Lit ESM. It lists no JS build output or TS config under viola-ui (per layout-templates §Surface: web-spa → Tooling context).
- The recorded TypeScript-plane decision limits TS sources to `e2e-web/` and states that the viola-ui page has no TS or JS build step (per layout-templates §Surface: web-spa → Tooling context).
- If the dependency-policy wording is changed, it admits no colour or table crate for the `viola` bin (per layout-templates §Surface: cli → Tooling context).

## Relevant amendment history
(none). The sidecar `layout-templates-amendments.md` does not exist, so the history is empty. The plan's inline §Decisions Log has three fix passes: T3/T4/T5 (cli hints and escaping), Y4 (focus-ring naming) and Z10 (announcement scope). None of them touch the architecture tree, the release build or the code-graph planes.
