group: a11y-plan
read: .claude/docs/a11y-summary.md · 63 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 40 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:35   class: omission
  leaf-text: «**a11y-tooling-install** — devDependencies (colorjs.io, tabbable, virtual-screen-reader, html-validate, eslint-plugin-jsx-a11y + eslint)»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:8 «Add `@axe-core/playwright@4.13.0` to `e2e-web/package.json`. Tests declared only `@playwright/test@1.63.0` when the browser pipe landed (chunk 2026-09-27-browser-verdict-reachability); the axe pin is this phase's.»
  replace-with: «**a11y-tooling-install** — `@axe-core/playwright@4.13.0` into `e2e-web/package.json` (the axe pin is this phase's), devDependencies (colorjs.io, tabbable, virtual-screen-reader, html-validate, eslint-plugin-jsx-a11y + eslint)»

- leaf: .claude/docs/a11y-summary.md:31   class: omission
  leaf-text: «- **P6 — CLI board, wait/last, verify, launch line** — cli output discipline»
  source: .andromeda/a11y-plan.md:592 «`viola list` emits no SGR yet (chunk 2026-10-11-the-board-viola-list), so its three spans have no check until the colour layer lands ("CLI output discipline");»
  replace-with: «- **P6 — CLI board, wait/last, verify, launch line** — cli output discipline; as built (chunk 2026-10-11-the-board-viola-list) `viola list` prints words only with no SGR (the colour layer and the `--help` group labels owed to "CLI output discipline", the `-- UNWRAPPED - READ-ONLY --` separator to "Unwrapped sessions on the board"), and its caption order, board, empty line and refusal pairs are pinned by literals (`tests/cli_list.rs`, `src/human.rs`), not trycmd»
