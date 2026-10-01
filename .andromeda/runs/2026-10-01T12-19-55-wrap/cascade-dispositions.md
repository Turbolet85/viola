# Cascade dispositions — 2026-10-01T12-19-55-wrap (0-pending, operator-requested adaptation)

Amendments of this pass (no-op path, "operator ratification" door — never drift-derived):
- **B1** — the web front's toolkit moves from Lit to React + TypeScript (founder ruling 2026-09-30, relayed by the
  overseer; the overseer approved the apply form at this wrap). Masters state the toolkit; every measured detail
  (bundler and versions, embedding, CSP, Trusted Types, npm supply chain, the jsx-a11y version, the component→DOM
  mapping) is marked OPEN and owned by the route's new frontend-toolchain entry (Epoch 8's head). No control is
  retired: the Build-system no-bundler guard, every CSP directive and every output-encoding ban stand (Lit API names
  in the bans are replaced by their React sink counterparts, a narrowing).
- **D** — security-plan's arch amendment 1 (v1 GUI cookie) folded into architecture [GUI Control Scope], with the
  two GUI URNs of amendment 3 (§Conventions → GUI HTTP errors) and amendment 5 (`ui/<port>.url`, §Occupied
  Resources → Filesystem) that the cookie mechanism entails (overseer, founder-delegated: "amend arch now").
  Amendment 3's `control-character` detail is NOT folded: no code carries it yet and the route's "Confirmed send"
  entry names it.

## The search
Patterns: `.andromeda/runs/2026-10-01T12-19-55-wrap/cascade-patterns.toml` (18 patterns, every control fired; the
first form's `gui-ungated` never fired at 95c1a9b5 and was replaced by the controlled `no-token-any`). Runs:
`cascade-sweep.txt` (before the leaves), `cascade-sweep-after.txt` (after). The phrasings swept beyond the
toolkit's name: light DOM, `createRenderRoot`, "no (JS) build step", vendored ESM, `static styles`, `styleMap`,
`unsafeHTML`/`unsafeSVG`, custom element(s), `connectedCallback`, `${}` bindings, `html` templates; for D, "no token
or CSRF", every "no token / no CSRF / no cookie", the 401 URN and the `.url` file.

## Master rows (pre-leaf sweep)
- design-system :376, :379, :528, :555, :597, :359, :668-670, :678, :868 — amended (B1).
- layout-templates :18 — amended (B1).
- test-plan :67, :707, :1144, :1182 — amended (B1).
- obs-plan :587 — amended (B1; the "needs a bundler" clause dropped, the GET-only reason kept).
- obs-plan :93, :208 — no change: obs §1 is a verbatim scope copy (playbook "Verbatim scope copy").
- a11y-plan :413, :497, :512, :526, :894, :946, :974, :975 — amended (B1).
- a11y-plan :130, :153 — no change: a11y §1 is a verbatim upstream copy (playbook "Verbatim upstream copy (other
  masters)").
- architecture :426 — amended (B1, the lint report's plugin name). :94, :150-156 (URNs), :386+ (Filesystem) —
  amended (D).
- security-plan :393-396 (Bootstrap phases, auth-scaffolding-baseline) — amended (D: the fold recorded where the
  amendments were sequenced).
- security-plan :84, :144 ("no token and no CSRF protection") — no change: Threat Model Summary is the verbatim copy
  of threat-assessment.md (playbook "Verbatim upstream copy (other masters)"); §Authentication & Authorization wins
  and already states the cookie.
- registries: architecture/build-system.md :15 (guard kept, the ruling and its owner added) ·
  architecture/project-directory-structure.md :43-45, :84 · test-plan/5-command-implementation.md :12 ·
  a11y-plan/a11y-testing-tool-pick.md :10 · a11y-plan/bootstrap-phases-derive-for-route-setup-project.md :9, :11, :13,
  :20 · a11y-plan/focus-management-test-harness.md :4 · a11y-plan/structured-violation-json-schema.md :20 (the
  `check_source` value `eslint-lit-a11y` → `eslint-jsx-a11y`; no obs-plan or code site carries the value — grep over
  the repo, run dirs and archives excluded) — amended (B1). `registry.py check --all`: 0 defects.
- Standing true claims sharing a token (no change): every `ui/<port>.url` and `urn:viola:problem:unauthorized` row
  in security / test / obs / a11y / design; every "no token … in markup / logs / shared" row (secrets hygiene, not the
  retired GUI claim); `lit` as the strip STATE (the `case-variants` count; design-system :85-:116 etc.).

## Leaves (re-derived)
- CLAUDE.md :27 (GENERATED modules) — re-derived (B1).
- `.claude/rules/frontend.md` :12, :13, :18, the new :20 Trusted Types line, :32 (component names without tag
  brackets) — re-derived (B1).
- `.claude/rules/a11y.md` :15, :16 — re-derived (B1).
- `.claude/docs/a11y-summary.md` :16, :35, :37 — re-derived (B1).
- `.claude/docs/services/viola-ui.md` :14, :24, :33 — re-derived (B1).
- `.claude/docs/stack.md` :54, :57 — re-derived (B1).
- `.claude/docs/security-summary.md` :44 — re-derived (D: the fold state of amendments 1, 3's URNs, 5).
- `.claude/docs/conventions.md` :35 and `.claude/rules/api.md` :38-:39 — no change: already state the URNs and the
  cookie gate (the leaves were ahead of arch).
- design-summary / tests-summary / obs-summary / testing.md / observability.md — no row in either sweep.

## Post-leaf sweep
Every remaining row is (a) this pass's own text naming the retired form ("the Lit-era … is retired"), (b) a verbatim
section above, or (c) a "no JS build step until the frontend-toolchain entry" statement — the guard standing by
design. No curation-home or judgment-base row in either sweep.

## Not looked for
The route's own wording (B2 edited `:129`/`:133` by hand); `viola-0.1.0/intent.md:118` and verification-matrix v1-08's
`observed_gap` (dated route-time observations, left as history by the operator's default at this wrap).
