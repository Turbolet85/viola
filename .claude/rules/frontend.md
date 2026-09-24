---
paths:
  - "crates/viola-ui/assets/**"
  - "src/cmd/**"
---

# Frontend Rules

Path-scoped rules for viola's two design surfaces: the `viola ui` strip-bay page (web-spa) and human CLI output (cli). Authoritative sources: `.andromeda/design-system.md` and `.andromeda/layout-templates.md`; output encoding from `.andromeda/security-plan.md` §API Security.

## Framework (web-spa)
- **Framework:** Lit 3.3.3, vendored ESM, embedded via `include_bytes!`; no JS build step, no component library.
- **Component model:** plain light-DOM `viola-*` elements (`createRenderRoot(){ return this; }`) over native HTML — no Shadow DOM, no Lit `static styles` in v1.
- **Styling:** one hand-written `/assets/app.css` with `@layer tokens, base, components, states, motion`, native nesting, custom properties. State only in `data-*` attributes (`data-rb`, `data-dialog`, `data-liveness`, `data-wrapped`, `data-live`).
- **Routes:** one page `/`; loads `/api/sessions` + `/api/links`, then tails SSE `/api/events`. No client routes, no query-string state, no history entries.

## Output encoding (CSP `default-src 'none'; script-src 'self'; style-src 'self'; … require-trusted-types-for 'script'`)
- Every event field and unwrapped session name is a `${}` text binding. NEVER `innerHTML`, `unsafeHTML`, `unsafeSVG`, `styleMap`, `style="…"`, inline `<script>`, inline handlers, `eval`, `@font-face`, CDN assets or Markdown rendering.
- Never display, echo or log the GUI token, launch URL, `?t=`, `.url` path or a cookie value — the 401 strip included — and never render `viola_home`.

## Tokens (the Token Test)
- Exactly eight hex values (`--c-*`), read only where a role alias is assigned; components use role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`). Every length is a token or a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`; only the `760px` / `1024px` breakpoints are literal.
- Amber means `dialog_pending` only; blue means handoff only. Lamp-off, amber and blue text never sit on holder `#2A2E33` (they fail AA there).
- Borders-only depth: no `box-shadow`, `text-shadow`, glow or `drop-shadow`; radius 0 everywhere; dark only.
- Fonts: installed stacks only (`--font-label` Bahnschrift…, `--font-field` Cascadia Mono…); the Linux DejaVu render is what CI checks.

## Motion (expression 0.3 on the page)
- Exactly two motions: the cock (`translateX(--cock-offset)` over `--cock-dur`, into pending only) and the `--fade-dur` `@starting-style` fade on `data-live` tape lines/markers. Readback fill, strike, stale dimming, return into line, jumps and text swaps are instant. `prefers-reduced-motion` zeroes both durations; the offset stays.
- Banned: spinners, pulses, blink, shimmer, skeletons, looped/staggered animation, hover colour transitions, "Loading…", toasts, auto-dismissing notices, command palette, shortcuts.

## Components
- `<viola-session-row>` strip (six fixed fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, stable slot by name, `unknown` for a missing reading, `n/a` absent-by-contract, never blank); `<viola-readback>` (the signature: `open` / `read` / `refused` / `unconfirmable`, identical drawing in tape and marker); `<viola-transfer>`; `<viola-event-feed>` (native `<details>` lines, 2000-line cap, follow only at bottom); `<viola-atis>`.
- Nothing is shown as delivered until read back; `turn-ended` never fills a box.

## CLI surface (`src/cmd/**`)
- Colour decision order (first match → no colour): `--json` · `viola run` · `viola hook`/`mcp` · `NO_COLOR` · `TERM=dumb` · non-TTY stdout · Windows VT enable failed. SGR only for amber `DIALOG`, dim `stale` rows, bold NAME — always beside its word; never on `unable` / `fail`.
- stdout = results; stderr = `waiting:`, the send issue line, refusals `unable  <name>  <reason>  <detail>` + one `hint:` line, errors, the `viola ui` launch block. ASCII only (`->`, `[RB]` / `[  ]` / `[/ ]`, ` - `); no spinner, no redraw, no stack traces, no "done".
- `list` rows escape C0/C1 including `\n`/`\t`; `wait`/`last` text escapes C0/C1 but keeps `\n`/`\t`. No hint ever names `viola release`; no hint for `unknown`, `wrapper fault` or `internal error`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
