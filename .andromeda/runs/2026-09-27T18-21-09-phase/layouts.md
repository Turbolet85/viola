# layouts extract

## No domain coverage
The chunk builds only the Playwright pipe (a static stub page, one trivially true assertion, harness/CI/WSL wiring) and its scope puts `viola ui`, the Lit page and every bay spec out of bounds, so no web-spa surface, wireframe, component, focus order or cli output structure from layout-templates §Surface: web-spa / §Surface: cli is created or modified; the stub page is a test fixture, not a layout surface (a stub that imitates the bay's `viola-*` elements or its single `/` route shape would pre-empt Epoch 8, per layout-templates §Surface: web-spa §IA notes, and is research/P4's to avoid, not this domain's to specify).
