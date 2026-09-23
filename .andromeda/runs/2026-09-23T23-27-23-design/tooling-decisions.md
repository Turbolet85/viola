# UI Tooling Decisions — viola

## Context (from architecture.md + security-plan.md)

- Product type: Hybrid local developer tool — a native cross-platform CLI binary (`viola`, Rust), a Claude Code plugin (hooks + stdio MCP server) that calls it, and a minimal view-only local web GUI (`viola ui`) served by the same binary.
- Audience: The founder (a single developer) on their own Claude subscription in v1; individual Claude Code subscribers after v1.x public distribution. Main automated callers are LLM driver sessions (MCP + CLI `--json`). Development style: agent-driven; UI verified through a headless browser by default.
- Platforms: Windows (live-supported, first target), macOS and Linux (CI-tested). Detected UI surfaces: `web-spa` (the `viola ui` page on 127.0.0.1, `/` + embedded `/assets/*`, fed by `/api/*` JSON and SSE `/api/events`) and `cli` (clap subcommands, human text by default, `--json` for agents; `viola run` passes the wrapped `claude` TUI through unchanged).
- Scale intent: personal — v1 personal (no accounts, no hosting, loopback only); v1.x adds a two-route GUI brake (pause, unlink) and public distribution; a phone/remote view later, as the same page behind authentication.
- Backend framework: Rust stable (MSRV 1.89, edition 2024); GUI served by axum 0.8.9 (`sse`) + tower-http 0.7.1 in `viola-ui`, Tokio 1.53.1 confined to `ui`/`mcp`; SSE fed by notify 8.2.0 tailing ndjson logs. Assets served from `include_bytes!`/`include_str!`; build is `cargo install --path .`; CI has only Rust steps.
- Mobile framework: N/A (phone view is a later version: the same web page behind authentication).
- Security tier: Minimal (0), with targeted elevations for the local privilege boundary (incl. GUI output encoding, GUI cross-user/cross-origin readability, v1.x per-launch brake auth). Binding frontend constraints: CSP `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'` (no inline script/style, no `style="…"` attributes, no web fonts since `font-src` falls back to `'none'`); every event field rendered as text only (never `innerHTML` / `unsafeHTML`; assistant Markdown never rendered to HTML); all JS served from embedded `/assets/*`.

## Family Selected

Web Components / Lit (build-optional) — the research's recommended family, kept by the user; components refined to none (see Q3).

## Q1: Frontend framework

- **Research recommended:** Lit 3.3.3
- **User response:** accepted (within the refine path: "Lit 3.3.3 + hand-written CSS, NO component library")
- **Final answer:** Lit 3.3.3 (vendored ESM, embedded via `include_bytes!`, no JS build step)
- **Reasoning:** Lit's built-in `lit-html` Trusted Types policy meets `require-trusted-types-for 'script'`, its `${}` text bindings meet the security plan's text-only output-encoding rule, and it ships as a vendored ESM file, so arch's Rust-only `cargo install` build and the Rust-only 3-OS CI need no Node step.

## Q2: CSS tooling

- **Research recommended:** hand-written modern CSS (native nesting, `@layer`, custom properties) served as `/assets/app.css`, plus Lit 3.3.3 `static styles`, using Web Awesome 3.13.0 design tokens
- **User response:** picked — "hand-written CSS" (no Web Awesome tokens, since no Web Awesome)
- **Final answer:** Hand-written modern CSS — no CSS tool (native nesting, `@layer`, custom properties; external stylesheet under `/assets/*`; Lit 3.3.3 `static styles` / `adoptedStyleSheets` remain available for component-scoped rules). Design tokens are viola's own custom properties, defined by this design system.
- **Reasoning:** Needs no tool in a Rust-only build and is `style-src 'self'`-clean by construction (no runtime `<style>` injection, no `style="…"` attributes); the minimal view-only page (D5) has a small enough rule set to hand-author. System font stacks only (no `font-src`).

## Q3: Component library

- **Research recommended:** Web Awesome 3.13.0 Free (MIT), with a local icon library and a system-font theme
- **User response:** custom: "NO component library (plain custom viola-* elements)" — rationale: "D5 asks for a minimal, view-only page. A whole component library adds embedded bytes, vendored code to audit (cargo-deny cannot see JS) and friction with the security plan's CSP + require-trusted-types-for. Lit alone keeps no build step and supports Trusted Types."
- **Final answer:** build-from-scratch — plain custom `viola-*` Lit 3.3.3 elements (e.g. `<viola-session-row>`, `<viola-event-feed>`) over semantic HTML (the research's "No library" option within the Lit family)
- **Reasoning:** D5's minimal view-only page needs no form widgets in v1; skipping a library keeps the embedded bytes small, leaves no vendored third-party JS outside cargo-deny's reach, and avoids CDN icon / injected-style friction with the CSP and Trusted Types. Status indicators (liveness, idle/busy, wheel holder, budget windows, reading age, pending dialog) become viola's own elements styled by this design system.

## Decisions Log

`2026-09-23` — Initial UI tooling decisions by `/andromeda-design` Phase 1

- Framework: Lit 3.3.3 (vendored ESM, no build step)
- CSS: hand-written modern CSS, no tool (viola's own custom-property tokens; Lit `static styles` available)
- Components: build-from-scratch — plain custom `viola-*` Lit elements
- Notes: User refined within the recommended Lit family: dropped Web Awesome 3.13.0 (and its design tokens) in favour of no component library, citing D5 minimality, embedded-byte cost, JS vendored code that cargo-deny cannot audit, and CSP / Trusted Types friction (Web Awesome's default CDN icon set is blocked by `img-src`/`connect-src 'self'`). No JS build step is introduced, so no cross-lane arch follow-up on build tooling is needed. Research caveats carried forward: ban `unsafeHTML`/`unsafeSVG`; system font stacks only (no `font-src`); confirm constructable stylesheets under the strict `style-src` in the headless-browser check; pin the vendored Lit version.
