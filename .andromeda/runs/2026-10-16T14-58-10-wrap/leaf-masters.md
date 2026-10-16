group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:7   class: retired
  leaf-text: «holds a one-driver wheel the human can take at any moment, and shows the sessions and their links on a minimal view-only loopback page.»
  source: .andromeda/architecture.md:428 «a minimal view-only frontend, deferred to 0.2.0; its technology is not chosen.»
  replace-with: «and holds a one-driver wheel the human can take at any moment; a minimal view-only frontend is deferred to 0.2.0 (its technology is not chosen).»

- leaf: CLAUDE.md:9   class: stale
  leaf-text: «Tokio only in `mcp`/`ui`»
  source: .andromeda/architecture.md:443 «A sync std-thread core; Tokio 1.53.1 only in `mcp`; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.»
  replace-with: «Tokio only in `mcp`»

- leaf: CLAUDE.md:13   class: retired
  leaf-text: «`-mcp`, `-ui`)»
  source: .andromeda/architecture.md:96 «Members: `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, plus the test-only `viola-e2e`»
  replace-with: «`-mcp`)»

- leaf: CLAUDE.md:16   class: retired
  leaf-text: «the ts code-graph plane via its tracked `tsconfig.json`); `a11y/sr-pass/` — manual screen-reader pass records»
  source: .andromeda/architecture.md:35 «stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text.»
  replace-with: «the ts code-graph plane via its tracked `tsconfig.json`; the browser leg stands until the route entry "Remove the browser leg" removes it)»

- leaf: CLAUDE.md:24   class: retired
  leaf-text: «, tailing, liveness, strict-modes»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «tempfile (the one `persist` helper), thiserror, tracing;»
  replace-with: «, liveness, strict-modes»

- leaf: CLAUDE.md:27   class: retired
  leaf-text: «- **`viola-ui`** — axum 0.8.9 GET routes + SSE on 127.0.0.1, Host allowlist, cookie gate, embedded React + TypeScript page (its toolchain lands with the frontend-toolchain chunk) (Tokio)»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «- **frontend** — deferred to 0.2.0; its technology is not chosen (no `viola-ui` workspace crate in 0.1.0)»

- leaf: CLAUDE.md:36   class: retired
  leaf-text: «NEVER-log floor: the GUI token, launch URL, `?t=`, `Cookie` and R8-stripped `CLAUDE*` values reach no log»
  source: .andromeda/security-plan.md:389 «Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.»
  replace-with: «NEVER-log floor: R8-stripped `CLAUDE*` values reach no log»

- leaf: CLAUDE.md:37   class: retired
  leaf-text: «(CLI `--json`, MCP `isError`, Problem Details, channel `error.data`)»
  source: .andromeda/security-plan.md:452 «**External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.»
  replace-with: «(CLI `--json`, MCP `isError`, channel `error.data`)»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «Tokio only in `viola-mcp` / `viola-ui`»
  source: .andromeda/architecture.md:411 «Only `viola-mcp` may build a runtime.»
  replace-with: «Tokio only in `viola-mcp`»

- leaf: CLAUDE.md:52   class: retired
  leaf-text: «| Wire contracts (channel frames, event line, snapshots, GUI HTTP, SSE) |»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «| Wire contracts (channel frames, event line, snapshots, the `list` payload) |»

- leaf: CLAUDE.md:55   class: retired
  leaf-text: «| IPC / home access control, input validation, GUI cookie |»
  source: .andromeda/security-plan.md:168 «**Auth approach:** OS-user ambient trust, enforced per OS, on the IPC endpoints and `~/.viola/`.»
  replace-with: «| IPC / home access control, input validation |»

- leaf: CLAUDE.md:57   class: stale
  leaf-text: «| Design tokens (8 hex values, type, spacing, motion) | `.andromeda/design-system.md` §Color Palette / §Typography / §Surface: web-spa Tokens |»
  source: .andromeda/design-system.md:374 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «| Design tokens (8 hex values, type, spacing, motion: a 0.2.0 frontend design input; 0.1.0 renders no page) | `.andromeda/design-system.md` §Color Palette / §Typography / §Surface: cli Tokens |»

- leaf: CLAUDE.md:58   class: stale
  leaf-text: «| Web and CLI layouts |»
  source: .andromeda/layout-templates.md:17 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «| CLI layouts (the web surface deferred to 0.2.0) |»

- leaf: CLAUDE.md:62   class: retired
  leaf-text: «| A11y harness · per-SC map · ARIA catalog |»
  source: .andromeda/registries/contracts/a11y-plan/wcag-criteria-mapping.md:3 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «| A11y harness · cli / tui landmarks and path scenarios (the frontend deferred to 0.2.0) |»

- leaf: CLAUDE.md:85   class: stale
  leaf-text: «(`pty · channel · state · agent-claude · mcp · ui` around `core`)»
  source: .andromeda/architecture.md:430 «compiler-enforced internal crates (`pty · channel · state · agent-claude · mcp` around `core`)»
  replace-with: «(`pty · channel · state · agent-claude · mcp` around `core`)»
