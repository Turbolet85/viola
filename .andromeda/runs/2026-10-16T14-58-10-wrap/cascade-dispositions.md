# Cascade dispositions — the 2026-10-16T14-58-10 0-pending wrap (CLI-only 0.1.0, the frontend to 0.2.0)

total (11 patterns) · 588 rows over 51 files · per class · new 14/9 · standing 276/25 · leaf 297/25 · curation 0/0 · base 1/1
(copied from the sweep listing, `cascade.py sweep --patterns-file cascade-patterns.toml`, run after every master and
key-file edit of this pass had landed; the trail is `cascade-no-marker.json`)

## The search
Eleven patterns, each over all seven masters, every `.andromeda/registries/**` file, the three curation homes, the two
judgment bases and the leaves, each control fired on the pre-pass text: `viola-ui` (`viola ui`, `viola-ui`, `viola_ui`,
`` `ui` ``, `ui-<port>`, `ui/<port>.url`), `gui`, `sse` (SSE, `/api/events`, EventSource, keep-alive, `Last-Event-ID`,
`sse-*`), `http-routes` (`/api/*`, `/health`, `/ready`, `http-request`, axum, tower-http, TraceLayer), `gui-auth` (cookie,
launch URL/line/file, `?t=`, per-launch token, getrandom, constant_time_eq, Host allowlist), `port` (47319, GUI port,
`--port`), `web-spa` (web-spa, strip bay, `<viola-*>`, the page, web strip/racks), `problem` (Problem Details, URNs,
RFC 9457, UiError), `web-a11y` (axe, WCAG, ARIA, screen reader, sr-pass, forced colours, reduced motion),
`browser-leg` (Playwright, Chromium, e2e-web, npm, Node, NODE_PIN, install-node, npm-audit, run --browser),
`frontend-tech` (React, TypeScript, Lit, frontend-toolchain, Epoch 8). Every product-pattern master row (131) was read
at its window; the `browser-leg` and `web-a11y` rows were read by class.

## Master and registry rows (new 14 · standing 276)
- **Amended this pass (new 14):** the reserved marks and the browser-leg marker sentences this pass wrote — no change.
- **Kept by the operator's first answer (browser test leg true to the tree; `browser-leg` 119 standing and the
  `web-a11y` rows naming axe / Playwright tooling):** the Node pin, npm and the npm audit, `e2e-web/`, Playwright,
  Chromium, the CI browser job, the pre-push browser stage, `run --browser`, the ts plane, and the a11y lint stage that
  runs over them. Owned by "Remove the browser leg" — no change.
- **Kept by the operator's second answer (closed obs vocabulary true to the tree):** the `ui` process value, `ui-<port>`
  role-file name, `http-request`, `liveness-changed`, `sse-opened` / `sse-closed`, the `last-event-id` parser value, the
  `viola_ui` / axum / tower_http / hyper log targets (`src/obs.rs`, `schemas/diag-line.v1.json`), each enum or catalogue
  site marked reserved once — no change.
- **Kept as test code in the tree:** the secret-scan classes `token-query` / `gui-token` / `cookie` and every never-log
  or scan line naming the GUI token, `Cookie` or `?t=` (`secret_scan.rs`); `status.rs` `ui` / `api_sessions_equal_list`
  and `cleanup.rs` `port_free` / `url_file_removed` nulls; the `cookie_file` session field; the `fixed-port` nextest
  group and its 47319 note (`.config/nextest.toml`); `deny.toml`'s `axum` `http2` ban; the shims' `ui-restart`
  forwarding; the Browser-side controls section of the 5-command key (browser test text) — no change, owned by
  "Remove the browser leg" (its third CARRY).
- **Kept by the founder's ruling (Brand Identity and the concept stay; the page's purpose is the 0.2.0 starting
  intent):** design-system Brand Identity (L3–L51: strip bay, `<viola-atis>`, the web-spa expression row, L45's motion
  note), the architecture GUI Control Scope intent sentence, test-plan's creator-excerpt caption agreement, CLI-board ↔
  future-page agreement lines — no change. Named for the founder: Brand L41 / L45 cite Motion's values block and L16 /
  L21 name web-spa components, both now cut; Brand stays untouched by his word.
- **Kept by the operator's third answer (design inputs for 0.2.0):** design-system Contrast rules, Surface Scale, Text
  Hierarchy, Semantic Colors and the domain-status rows (incl. `TAPE stopped · viola ui not answering`, the access
  strip, `/api/sessions` omission, `/ready` / `/health` not visualised), Border Progression, Typography, Spacing, Depth,
  Radius, Iconography, the Motion heading's expression note, the Signature Test places — each section marked «A 0.2.0
  frontend design input; 0.1.0 renders no page.» — no change.
- **Stubs and their headings:** `## Surface: web-spa` (layout-templates, design-system), `[Real-time Strategy]`,
  `[GUI Control Scope]`, `**GUI contracts.**`, the obs `GUI view` scenario heading, test-plan E4's heading — no change.
- **Amended here, not by a planner (3):** test-plan.md:1027, a11y-plan.md:325 and the a11y bootstrap key L9 cited "the
  route's frontend-toolchain entry", which left 0.1.0 for `.andromeda/residuals.md` — re-pointed
  (`edits-test-plan-cross.json`, `edits-a11y-plan-cross.json`, `edits-a11y-plan--bootstrap-cross.json`).
- **Bound pair (test-plan §3 ↔ obs-plan §3, Log format):** test-plan's key L5 gained the reserved mark; obs-plan's key
  L8 took the same line byte for byte and obs-plan.md L225 the same mark (`edits-obs-plan--bind-*.json`). The obs master
  copy lagged test-plan before this pass (no `cli-<name>`, no detail sentence) — named, not this pass's to widen; the
  test-plan lines missing from the obs key copy read 2 at HEAD and 2 after.

## Judgment bases (base 1)
- `.andromeda/drift-base.md:113` (`D-a11y-surface`): conditional — a chunk that adds an interactive UI element is drift
  against the a11y-plan §5–§7 stubs. Still true; no change, no rule proposed.

## Curation homes (curation 0)
- None.

## Leaf rows of the sweep (leaf 297 rows / 25 files)
Every leaf row joins step 3's set through `cascade.py leaves` (pass mode); see `## Leaves` below.

## Leaves
Mode: pass (no epoch closes at a 0-pending wrap; the operator asked for no whole mode). Set: 14 groups · 28 leaves ·
`total · pass · 14 group(s) · 28 leaves · 308054 B · unmapped 2`. One comparer per group in one batch inside one
snapshot pair (before `tree fc341e02… index 36247384… head 3fc2e61c…`; after `tree b351b49f…`, index and head
unchanged; `git diff-tree` printed exactly the 14 `A` rows of `leaf-{group}.md`). Apply totals, copied from the tool:
`total · blocks 230 · applied 155 · preserved 13 · reshaped 50 · held 12`.

Per group — findings · applied by the tool · by hand · declined:
- architecture-1 — 9 · 9 · 0 · 0
- architecture-2 — 36 · 13 · 3 (`viola-state.md` L29 symlink line, `stack.md` L11 / L26 rows deleted) · 0; the other 20
  were the `services/viola-ui.md` findings, written as one whole-content stub (the crate is not in the workspace; the
  file is kept because CLAUDE.md's services list names it — its removal is named for the operator)
- architecture-3 — 1 · 1 · 0 · 0
- architecture+security-plan (`rules/api.md`) — 11 · 5 · 5 (two headings, three GUI bullets deleted) · 1 (L4 frontmatter
  glob `crates/viola-ui/src/**`: matches no file, harmless)
- architecture+security-plan+design-system+layout-templates (`rules/frontend.md`) — 18 · 6 · 11 (eight «(delete the
  line)» findings deleted rather than written as text, three headings) · 1 (L3 frontmatter glob)
- masters (`CLAUDE.md`) — 15 · 15 · 0 · 0
- architecture+test-plan+obs-plan (`docs/commands.md`) — 11 · 9 · 2 (two fenced filter lines) · 0
- security-plan — 15 · 8 · 7 (four bullets deleted, the amendment list L44, three `security.md` lines deleted) · 0
- security-plan+obs-plan (`rules/ci.md`) — 0
- design-system+layout-templates (`docs/design-summary.md`) — 31 · 24 · 7 (five web layout bullets and the reduce-motion
  bullet deleted, two headings) · 0
- test-plan — 23 · 21 · 2 (`testing.md` L43, `verification-harness.md` L30 deleted) · 0
- test-plan+obs-plan (`rules/observability.md`) — 7 · 7 · 0 · 0
- obs-plan (`docs/obs-summary.md`) — 13 · 13 · 0 · 0
- a11y-plan — 40 · 24 · 16 (twelve page bullets and the announcement heading deleted, three lines replaced) · 0

Every source side was read against the edited masters at its cited line before the write; each finding's replace-with
was read whole in the review (`leafrev`), the `(delete …)` replace-withs routed to a hand deletion. Hand writes:
`handleaf-*.json` (guarded line edits, every untouched line read back identical) and the `viola-ui.md` stub; two
doubled blank lines the deletions left (`frontend.md`, `a11y.md`) closed by anchored Edits. Unmapped: `host-linux.md`
(the host's own leaf, no comparer) and `testing-src.md` (read: no web statement).

Web-only rule files named for the operator (the relay's item 5): `.claude/rules/frontend.md` (its web half is now the
deferral line; its cli half stands), `.claude/rules/a11y.md` (paths scope `crates/viola-ui/assets/**` and `e2e-web/`
only; its content is now the deferral, the terminal rules and the browser-leg harness), and the doc
`.claude/docs/services/viola-ui.md` (a stub for a crate that does not exist).
