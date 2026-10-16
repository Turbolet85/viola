group: architecture+design-system
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:12   class: retired
  leaf-text: «- **Frontend:** deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page. No tsconfig, package manifest or bundler config ever covers `crates/viola-ui/`; a CI-runnable guard lists any such file (architecture Build system guard).»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «**ts:** no input in 0.1.0. Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json`»
  replace-with: «- **Frontend:** deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page.»

- leaf: .claude/rules/frontend.md:18   class: retired
  leaf-text: «the Linux DejaVu render is what CI checks, until the route entry "Remove the browser leg" removes that browser leg.»
  source: .andromeda/design-system.md:269 «0.1.0 renders no page, so no CI step renders these stacks or provides their families; how a 0.2.0 frontend checks these assertions is that version's to decide»
  replace-with: «every contrast and render assertion must hold with the Linux DejaVu families resolved (the resolved family, not only the declared stack); 0.1.0 renders no page, so no CI step renders these stacks.»
