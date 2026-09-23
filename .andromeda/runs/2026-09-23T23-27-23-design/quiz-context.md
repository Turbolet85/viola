# Quiz Context — viola

_Extracted by the `/andromeda-design` orchestrator from `.andromeda/architecture.md`, `.andromeda/security-plan.md`, `tooling-decisions.md`, and the creator brief (`.andromeda/input.md` + `refs/`). Constant across all Phase 2 sub-agent spawns. Facts only — no design decisions._

## Project Intent (architecture.md)

- **product_name:** viola (binary `viola` / `viola.exe`; all identifiers use the `viola` prefix)
- **product_type:** Hybrid local developer tool: (1) a native cross-platform CLI binary (Rust); (2) a Claude Code plugin (hooks + stdio MCP server) that calls that binary; (3) a minimal view-only local web GUI served by the same binary.
- **core_functionality:** Lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a pseudo-terminal, types into the driven session at turn boundaries, answers its dialogs (questions, permissions, plan approvals) through hooks, and holds a one-driver "wheel" the human can take at any moment. Everything is logged as an ndjson audit trail on disk.
- **audience:** The founder — a single developer who today is the manual "transport and operator" between two Claude Code sessions (an overseer that verifies a build, and a builder that runs the Andromeda pipeline). Later (v1.x): individual Claude Code subscribers via public distribution. The main automated callers are LLM driver sessions (MCP + CLI `--json`).
- **platforms / detected surfaces:** Windows first (live-supported), macOS + Linux CI-tested. Surfaces: `web-spa` — the `viola ui` page on 127.0.0.1 (session rows, links, budget, live event feed over SSE; view-only in v1); `cli` — clap subcommands (`run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`), human text by default, `--json` + typed exit codes for agents; `viola run` passes the wrapped `claude` TUI through unchanged and prints nothing else while the child runs.
- **scale_intent:** personal (v1: no accounts, no hosting, loopback only). v1.x: a GUI "brake" (pause, unlink) and public distribution. Later: a phone/remote view — the same web page behind authentication.
- **growth_model:** Modular monolith — one binary, compiler-enforced crates (`pty · channel · state · agent-claude · mcp · ui` around `core`), no daemon. New GUI views arrive as GET routes.
- **development_style:** agent-driven. The founder's projects verify UI through a headless browser by default.

## Design philosophy (architecture.md — informs brand, verbatim-close)

- **Mechanism, not policy** — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer.
- **Measured, never assumed** — every relied-on CLI behaviour is a row in a version-stamped capability ledger; unverified builds degrade to transport-only; every send is confirmed after the fact, never presumed.
- **The human always wins the wheel** — human keystrokes are never blocked; automation is refused with a typed reason (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`) instead of competing with the human.
- **No daemon; the disk is the shared truth** — ndjson logs + atomic snapshots survive a crash on either side.
- **Cross-platform from the first commit, Windows first.**

## What the GUI shows (architecture.md — GUI Control Scope + Standard Contracts)

Each session row (wrapped, or read-only unwrapped): name, liveness (`live` · `stale`), status (`idle` · `busy` · `unknown`), wheel holder (`human` · `driver`), `budget_paused`, `dialog_pending`, CLI version + verified flag. Also: the budget reading (`five_hour` / `seven_day` used % + `resets_at`, or `unknown`) and its age; the link set (`driver → driven`, `since`); `skipped` counts (records a reader could not interpret); a live event feed (kinds: session-start, prompt-submitted, turn-ended, question, permission, plan, session-end, activity, link, unlink, wheel, budget-gate). v1 is view-only: no control accepts input.

## Security (security-plan.md)

- **security_tier:** Minimal (0), with targeted elevations for the local privilege boundary (GUI output encoding, GUI cross-user/cross-origin readability, v1.x per-launch brake auth, IPC endpoint access control).
- **auth_approach:** No accounts. Per-launch secret for the GUI: a one-time `?t=<token>` launch URL exchanged for an HttpOnly SameSite=Strict cookie; no login screen.
- **Visual constraints from the CSP:** no web fonts (no `font-src` → system font stacks only); no inline styles / `style="…"` attributes; no external images or CDN assets; all event text rendered as plain text (assistant Markdown is never rendered to HTML).

## Tooling (tooling-decisions.md — Phase 1 final)

- **family_chosen:** Web Components / Lit (build-optional)
- **frontend_framework:** Lit 3.3.3 (vendored ESM, no JS build step)
- **css_tool:** Hand-written modern CSS, no tool (native nesting, `@layer`, custom properties; Lit `static styles` available)
- **component_library:** none — build-from-scratch plain custom `viola-*` Lit elements over semantic HTML
- **mobile_framework:** N/A

## Creator brief — aesthetic / tone touchpoints (input.md + refs/, full-fidelity extracts)

- **The founder's own words (Russian, translated in the brief):** "a mechanism that lets the overseer drive the builder itself, on the subscription, never the API"; "link agents in two terminals through our own buffer, easy, **no dancing with a tambourine**" (i.e. no hacks, no ritual); "a **minimal GUI** so it's visible which sessions are active and which of them are **linked**".
- **D5 DECIDED — a minimal GUI:** the active sessions and the links between them. Brief §3.2: "a feed of recent events. Read-only first; take-the-wheel, pause and unlink come next."
- **Core metaphors in the brief's own language:** the **wheel** (one driver at a time; the human typing takes the wheel); the **bridge** ("a general bridge between agents running in terminals"); "a tmux reduced to what agents need"; "our own buffer"; **authority flows one way, events the other** (R1); "identity by instance, never by location" (R5); "keys only for what must be typed" (R7).
- **Prior-art reference the brief itself names as closest to viola's page:** MulmoTerminal's grid — "one cell per session, coloured **working / done / needs you**"; "viola adds the links between sessions."
- **Negative anchors the creator stated (avoid):** "**Overclaiming** — the GUI shows only what the hooks and the wrapper observed"; "Blanket approval" (auto-yes defaults that remove the human checkpoint); screen-parsing as the primary channel; "fragile shared state".
- **Why a web page, not a TUI:** "no GUI toolkit on any platform, and an agent can verify it through a headless browser"; the alternative on the table was a terminal UI (ratatui) — "simpler, but not viewable from a phone".
- **Naming constraint (legal page, verbatim):** "the Claude Code name and logo may not be part of the product's own name or logo."
- **Name:** "Viola" — the project folder name chosen by the founder (D1); no stated rationale in the brief.
- **Posture:** "ordinary, individual usage" of the subscription — a visible, conservative budget governor ships in v1.
