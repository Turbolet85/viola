group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:16   class: retired
  leaf-text: «- `e2e-web/` — the Playwright browser suite (`run --browser` on all three CI OSes; test-side only, under the sha256-pinned Node v24.21.0; axe and the a11y lint land with the a11y chunks; the ts code-graph plane via its tracked `tsconfig.json`; the browser leg stands until the route entry "Remove the browser leg" removes it)»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json` and `scripts/code-graph.py` detects no ts plane.»
  replace-with: «»
