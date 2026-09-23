# Tooling Context — viola

_Extracted by the `/andromeda-design` orchestrator from `.andromeda/architecture.md` and `.andromeda/security-plan.md`. Values are quoted or closely paraphrased from those files; nothing here is a design decision._

## From architecture.md (Project Intent)

- **product_type:** Hybrid local developer tool made of three parts: (1) a native cross-platform CLI binary (`viola`, Rust); (2) a Claude Code plugin (hooks + stdio MCP server) that calls that binary; (3) a minimal view-only local web GUI served by the same binary (`viola ui`).
- **audience:** The founder (a single developer) running v1 on their own Claude subscription; later (v1.x) public distribution to individual Claude Code subscribers. The main automated callers are LLM driver sessions (MCP + CLI `--json`). Development style: agent-driven — the founder's projects verify UI through a headless browser by default (brief §3.4).
- **platforms:** Windows (live-supported, first target), macOS and Linux (CI-tested from the first commit). UI surfaces detected by design Setup: `web-spa` (the `viola ui` page: `/` + `/assets/*` embedded in the binary, fed by `/api/*` JSON and SSE `/api/events`) and `cli` (clap subcommands with human text by default and `--json` for agents; `viola run` passes the wrapped `claude` TUI through unchanged and writes nothing else to the terminal while the child runs).
- **scale_intent:** Personal v1 (no accounts, no hosting, GUI on 127.0.0.1 only). v1.x adds a GUI "brake" (two POST routes: pause, unlink) and then public distribution (signed releases, winget/Homebrew/Scoop). A phone / remote view comes later — "the same web page behind authentication".

## From architecture.md (Stack)

- **backend_framework:** Rust stable (MSRV 1.89, edition 2024). The GUI server is axum 0.8.9 (feature `sse`) + tower-http 0.7.1, inside the `viola-ui` crate, with Tokio 1.53.1 confined to `ui`/`mcp`. Real-time: SSE via axum `Sse::keep_alive`, fed by notify 8.2.0 tailing ndjson logs. No WebSocket, no polling endpoints, no database.
- **mobile_framework:** N/A (phone view is a later version: the same web page behind authentication).
- **Build / distribution facts relevant to frontend tooling:** installed with `cargo install --path .`; every asset under `/assets/*` is served from `include_bytes!`/`include_str!` embedded data (never from the filesystem). CI runs on GitHub Actions on windows-2025 / macos-latest / ubuntu-latest and currently contains only Rust steps (fmt, clippy, check, deny, tests against a fake agent, release build). Arch names no Node/JS toolchain anywhere in the build.
- **GUI data contract:** view-only GET routes `/api/info`, `/api/sessions` (session rows: name, wrapped, liveness live|stale, status idle|busy|unknown, wheel human|driver, budget_paused, dialog_pending, cli_version, cli_verified; plus a top-level budget reading `{five_hour, seven_day, read_at, paused}`), `/api/links` (`{driver, driven, since}`), SSE `/api/events` (event kinds: session-start, turn-ended, prompt-submitted, question, permission, plan, session-end, activity, link, unlink, wheel, budget-gate; `data:` is the raw ndjson event line), and `skipped` counts of records a reader could not interpret. Lists are personal-scale and returned whole (no pagination). The page shows session rows, liveness, idle/busy, wheel holder, pending dialog, budget reading and its age, the link set, skipped counts, and a live event feed. v1 is view-only: no control accepts input.

## From security-plan.md

- **security_tier:** Minimal (0), with targeted elevations for the local privilege boundary — including GUI output encoding, GUI cross-user and cross-origin readability, and v1.x per-launch brake auth.
- **auth_approach:** No user accounts. The GUI uses a per-launch secret: `viola ui` prints `http://127.0.0.1:<port>/?t=<token>` once; `GET /?t=<token>` sets an `HttpOnly; SameSite=Strict` cookie `viola_<port>` and 303-redirects to `/`. `/api/*` and SSE require that cookie; `/`, `/assets/*`, `/health`, `/ready` are ungated. The frontend JS never sees the token (no `Authorization` header; `EventSource` sends the cookie same-origin).
- **Frontend constraints the security plan imposes (binding on any tooling choice):**
  - CSP on every response: `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`. Consequences: no inline `<script>`, no inline event handlers, no `eval`/`new Function`, no inline `<style>` or `style="…"` attributes, no runtime-injected `<style>` tags; no `font-src` directive, so it falls back to `default-src 'none'` (no web fonts, not even self-hosted, unless the security plan is amended); Trusted Types enforced for script sinks.
  - Output encoding: every event field (prompts, `last_assistant_message`, plan text, tool `input`, question text, unwrapped session names) is rendered with `textContent` or the framework's text interpolation. Never `innerHTML`, `v-html`, `dangerouslySetInnerHTML`. Assistant Markdown is never rendered to HTML.
  - All JS is served from `/assets/*`, embedded in the binary.
