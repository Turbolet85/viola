
## 2026-10-01-t12-19-55-wrap — web-spa tooling context: React + TypeScript
**Section:** Surface: web-spa → Tooling context
**Change:** Framework was Lit 3.3.3 (vendored ESM, no JS build step) with light-DOM `viola-*` Lit elements; now React + TypeScript, a built bundle embedded in `viola`, with the bundler, versions and embedding OPEN (owned by the route's frontend-toolchain entry) and the page built from plain `viola-*` components whose mapping onto React's output is OPEN. The no-`style` rule names React's `style` prop; `dangerouslySetInnerHTML` replaces `styleMap` / `unsafeHTML` in it.
**Why:** founder ruling of 2026-09-30, relayed by the overseer (0.2.0's node-graph view does not fit a build-less Lit page).
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/
